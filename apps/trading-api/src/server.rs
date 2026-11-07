use std::net::SocketAddr;

use tokio::net::TcpListener;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use trading_application::Application;
use trading_persistence::{Database, database_endpoint};

use crate::config::Config;
use crate::error::AppError;
use crate::health::{AppState, router};
use crate::logging::{log_postgres_unavailable, log_starting};

/// Handle for a running API process.
pub struct RunningServer {
    local_addr: SocketAddr,
    shutdown: Option<oneshot::Sender<()>>,
    task: Option<JoinHandle<Result<(), AppError>>>,
}

impl RunningServer {
    /// Address the listener bound, including an ephemeral port when requested.
    pub fn local_addr(&self) -> SocketAddr {
        self.local_addr
    }

    /// Asks the server to stop and waits until the listener task finishes.
    ///
    /// # Errors
    ///
    /// Returns [`AppError`] when the server task failed or stopped unexpectedly.
    pub async fn shutdown(mut self) -> Result<(), AppError> {
        if let Some(sender) = self.shutdown.take() {
            let _ = sender.send(());
        }
        let Some(task) = self.task.take() else {
            return Ok(());
        };
        match task.await {
            Ok(result) => result,
            Err(error) => Err(AppError::ServerTask(error)),
        }
    }
}

impl Drop for RunningServer {
    fn drop(&mut self) {
        if let Some(sender) = self.shutdown.take() {
            let _ = sender.send(());
        }
        if let Some(task) = self.task.take() {
            task.abort();
        }
    }
}

/// Starts the HTTP server and attempts the PostgreSQL readiness check.
///
/// A database outage is logged and does not prevent the process from serving
/// `GET /health/live`. Readiness reports that dependency separately. This
/// function does not submit orders.
///
/// # Errors
///
/// Returns [`AppError`] when the database URL is rejected or the HTTP listener
/// cannot bind. An unreachable database is not an error.
pub async fn start(config: Config) -> Result<RunningServer, AppError> {
    let endpoint = database_endpoint(config.database_url().expose())?;
    let application = Application::new(config.app_env());
    log_starting(&application, &config, &endpoint);

    let database = Database::connect(config.database_url().expose())?;
    match database.check_ready().await {
        Ok(()) => {
            tracing::info!(postgres = %endpoint, "postgres connection verified");
        }
        Err(error) => {
            log_postgres_unavailable(&endpoint, config.database_url().expose(), &error);
        }
    }

    let listener = TcpListener::bind(config.http_bind())
        .await
        .map_err(AppError::Bind)?;
    let local_addr = listener.local_addr().map_err(AppError::Bind)?;
    tracing::info!(address = %local_addr, "http server listening");

    let state = AppState::new(
        database,
        config.database_url().clone(),
        endpoint.to_string(),
    );
    let app = router(state);
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
    let task = tokio::spawn(async move {
        let result = axum::serve(listener, app)
            .with_graceful_shutdown(async move {
                let _ = shutdown_rx.await;
            })
            .await
            .map_err(AppError::Serve);
        tracing::info!("trading-api shut down");
        result
    });

    Ok(RunningServer {
        local_addr,
        shutdown: Some(shutdown_tx),
        task: Some(task),
    })
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpStream;
    use tokio::time::timeout;

    use super::start;
    use crate::config::Config;

    const SECRET_URL: &str = "postgres://trading:super-secret-password@127.0.0.1:1/trading";

    fn test_config() -> Config {
        Config::from_pairs([
            ("APP_ENV", "test"),
            ("HTTP_BIND_ADDRESS", "127.0.0.1:0"),
            ("DATABASE_URL", SECRET_URL),
        ])
        .expect("test configuration")
    }

    async fn port_one_is_closed() {
        if TcpStream::connect("127.0.0.1:1").await.is_ok() {
            panic!("127.0.0.1:1 accepted a connection; the startup test will not use it");
        }
    }

    async fn http_request(addr: std::net::SocketAddr, method: &str, path: &str) -> String {
        let mut stream = TcpStream::connect(addr)
            .await
            .expect("connection to the test server");
        let request =
            format!("{method} {path} HTTP/1.1\r\nHost: {addr}\r\nConnection: close\r\n\r\n");
        stream
            .write_all(request.as_bytes())
            .await
            .expect("request write");
        stream.flush().await.expect("request flush");
        let mut body = Vec::new();
        match timeout(Duration::from_secs(2), stream.read_to_end(&mut body)).await {
            Ok(Ok(_)) => {}
            Ok(Err(error)) => panic!("response read failed: {error}"),
            Err(_) => {}
        }
        String::from_utf8_lossy(&body).into_owned()
    }

    fn status_code(response: &str) -> u16 {
        response
            .split_whitespace()
            .nth(1)
            .and_then(|value| value.parse().ok())
            .unwrap_or(0)
    }

    #[tokio::test]
    async fn application_starts_on_loopback_and_shuts_down() {
        port_one_is_closed().await;
        let server = start(test_config()).await.expect("server should start");
        assert!(server.local_addr().ip().is_loopback());
        server.shutdown().await.expect("server should shut down");
    }

    #[tokio::test]
    async fn health_endpoints_report_process_and_database_state() {
        port_one_is_closed().await;
        let server = start(test_config()).await.expect("server should start");
        let address = server.local_addr();

        let live = http_request(address, "GET", "/health/live").await;
        assert_eq!(status_code(&live), 200, "{live}");
        assert!(live.contains("\"status\":\"ok\""));
        assert!(live.to_ascii_lowercase().contains("x-correlation-id"));

        let ready = http_request(address, "GET", "/health/ready").await;
        assert_eq!(status_code(&ready), 503, "{ready}");
        assert!(ready.contains("\"status\":\"not_ready\""));
        assert!(ready.contains("\"postgres\":\"unavailable\""));

        let orders = http_request(address, "GET", "/orders").await;
        assert_eq!(status_code(&orders), 404, "{orders}");

        let post = http_request(address, "POST", "/health/live").await;
        assert_eq!(status_code(&post), 405, "{post}");

        for response in [&live, &ready, &orders, &post] {
            assert!(!response.contains("super-secret-password"));
            assert!(!response.contains(SECRET_URL));
        }

        server.shutdown().await.expect("server should shut down");
    }
}
