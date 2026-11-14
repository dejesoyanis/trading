use std::future::Future;
use std::net::SocketAddr;
use std::time::Duration;

use tokio::net::TcpListener;
use tokio::sync::oneshot;
use tokio::task::{JoinError, JoinHandle};
use trading_application::Application;
use trading_persistence::{Database, database_endpoint};

use crate::config::Config;
use crate::error::AppError;
use crate::health::{AppState, router};
use crate::logging::log_starting;

/// How long a requested shutdown waits before the server task is aborted.
///
/// When this elapses, [`RunningServer::shutdown`] aborts the task and returns
/// [`AppError::ShutdownTimeout`]. Process shutdown is not a trading halt.
pub const GRACEFUL_SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(10);

/// How long to wait for an aborted task to observe cancellation.
const ABORT_JOIN_TIMEOUT: Duration = Duration::from_millis(200);

/// Handle for a running API process.
///
/// [`Self::shutdown`] is the graceful path. [`Drop`] is only a best-effort
/// fallback: it signals the task and aborts it without waiting. Drop must not
/// block, and it does not guarantee that in-flight requests finish.
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

    /// Asks the server to stop and waits until the task finishes or `timeout` elapses.
    ///
    /// On timeout the task is aborted. The abort is given a short additional
    /// moment to observe cancellation. If it is still running, the handle is
    /// dropped and [`AppError::ShutdownTimeout`] is returned.
    ///
    /// # Errors
    ///
    /// Returns [`AppError::ShutdownTimeout`] when the task does not finish in
    /// time. Returns the task's own error when the server fails while stopping.
    pub async fn shutdown(mut self, timeout: Duration) -> Result<(), AppError> {
        if let Some(sender) = self.shutdown.take() {
            let _ = sender.send(());
        }
        let Some(task) = self.task.take() else {
            return Ok(());
        };
        finish_shutdown(task, timeout).await
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

/// Serves until Ctrl+C or, on Unix, SIGTERM, then shuts down within
/// [`GRACEFUL_SHUTDOWN_TIMEOUT`].
///
/// If the HTTP task stops before a signal, this returns an error and the
/// process should exit nonzero. This is process lifecycle, not a trading halt.
///
/// # Errors
///
/// Returns [`AppError`] when signal installation fails, the server stops
/// unexpectedly, or graceful shutdown times out.
pub async fn run_until_shutdown(server: RunningServer) -> Result<(), AppError> {
    drive(server, shutdown_signal(), GRACEFUL_SHUTDOWN_TIMEOUT).await
}

/// Starts the HTTP server and performs one read-only readiness check.
///
/// A database outage or an unmigrated schema is logged and does not prevent
/// the process from serving `GET /health/live`. This function does not migrate
/// and does not submit orders. HTTP readiness is not Trading Readiness and
/// grants no submission authority.
///
/// # Errors
///
/// Returns [`AppError`] when the database URL is rejected or the HTTP listener
/// cannot bind. An unreachable or unmigrated database is not a startup error.
pub async fn start(config: Config) -> Result<RunningServer, AppError> {
    let endpoint = database_endpoint(config.database_url().expose())?;
    let application = Application::new(config.app_env());
    log_starting(&application, &config, &endpoint);

    let database = Database::connect(config.database_url().expose())?;
    let readiness = database.check_ready().await;
    let state = AppState::new(database, endpoint.to_string());
    state.record_result(&readiness);

    let listener = TcpListener::bind(config.http_bind())
        .await
        .map_err(AppError::Bind)?;
    let local_addr = listener.local_addr().map_err(AppError::Bind)?;
    tracing::info!(address = %local_addr, "http server listening");

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

async fn drive(
    mut server: RunningServer,
    signal: impl Future<Output = Result<(), AppError>>,
    timeout: Duration,
) -> Result<(), AppError> {
    tokio::pin!(signal);
    let signaled = {
        let task = server.task.as_mut().ok_or(AppError::ServerStopped)?;
        tokio::select! {
            result = task => {
                tracing::error!("http server stopped before a shutdown signal");
                return unexpected_exit(result);
            }
            received = &mut signal => received,
        }
    };
    signaled?;
    tracing::info!("shutdown signal received");
    server.shutdown(timeout).await
}

async fn shutdown_signal() -> Result<(), AppError> {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{SignalKind, signal};
        let mut terminate = signal(SignalKind::terminate()).map_err(AppError::Signal)?;
        tokio::select! {
            result = tokio::signal::ctrl_c() => result.map_err(AppError::Signal),
            _ = terminate.recv() => Ok(()),
        }
    }
    #[cfg(not(unix))]
    {
        tokio::signal::ctrl_c().await.map_err(AppError::Signal)
    }
}

async fn finish_shutdown(
    mut task: JoinHandle<Result<(), AppError>>,
    timeout: Duration,
) -> Result<(), AppError> {
    tokio::select! {
        result = &mut task => match result {
            Ok(result) => result,
            Err(error) => Err(AppError::ServerTask(error)),
        },
        _ = tokio::time::sleep(timeout) => {
            task.abort();
            let _ = tokio::time::timeout(ABORT_JOIN_TIMEOUT, task).await;
            Err(AppError::ShutdownTimeout)
        }
    }
}

fn unexpected_exit(result: Result<Result<(), AppError>, JoinError>) -> Result<(), AppError> {
    match result {
        Ok(Ok(())) => Err(AppError::ServerStopped),
        Ok(Err(error)) => Err(error),
        Err(error) => Err(AppError::ServerTask(error)),
    }
}

#[cfg(test)]
mod tests {
    use std::future::pending;
    use std::io;
    use std::time::{Duration, Instant};

    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::{TcpListener, TcpStream};
    use tokio::sync::oneshot;
    use tokio::time::timeout;

    use super::{RunningServer, drive, start};
    use crate::config::Config;
    use crate::error::AppError;

    const SECRET_URL: &str = "postgres://trading:super-secret-password@127.0.0.1:1/trading";

    fn test_config() -> Config {
        Config::from_pairs([
            ("APP_ENV", "test"),
            ("HTTP_BIND_ADDRESS", "127.0.0.1:0"),
            ("DATABASE_URL", SECRET_URL),
        ])
        .expect("test configuration")
    }

    fn placeholder_address() -> std::net::SocketAddr {
        std::net::SocketAddr::from(([127, 0, 0, 1], 0))
    }

    async fn port_one_is_closed() {
        if TcpStream::connect("127.0.0.1:1").await.is_ok() {
            panic!("127.0.0.1:1 accepted a connection; the startup test will not use it");
        }
    }

    async fn http_request(addr: std::net::SocketAddr, method: &str, path: &str) -> String {
        http_request_within(addr, method, path, Duration::from_secs(2)).await
    }

    async fn http_request_within(
        addr: std::net::SocketAddr,
        method: &str,
        path: &str,
        limit: Duration,
    ) -> String {
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
        match timeout(limit, stream.read_to_end(&mut body)).await {
            Ok(Ok(_)) => {}
            Ok(Err(error)) => panic!("response read failed: {error}"),
            Err(_) => panic!("timed out waiting for the http response"),
        }
        if body.is_empty() {
            panic!("http response was empty");
        }
        let response =
            String::from_utf8(body).unwrap_or_else(|_| panic!("http response was not utf-8"));
        if status_code(&response) == 0 {
            panic!("http response did not contain a status line");
        }
        response
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
        server
            .shutdown(Duration::from_secs(2))
            .await
            .expect("server should shut down");
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
            assert!(!response.contains("postgres://"));
            assert!(!response.contains("database_unreachable"));
        }

        server
            .shutdown(Duration::from_secs(2))
            .await
            .expect("server should shut down");
    }

    #[tokio::test]
    #[should_panic(expected = "timed out waiting for the http response")]
    async fn http_request_fails_when_the_response_does_not_finish() {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("test listener");
        let address = listener.local_addr().expect("listener address");
        tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let _ = socket.write_all(b"HTTP/1.1 200").await;
            pending::<()>().await;
        });
        let _response =
            http_request_within(address, "GET", "/health/live", Duration::from_millis(200)).await;
    }

    #[tokio::test]
    async fn graceful_shutdown_finishes_when_the_task_stops() {
        let (sender, receiver) = oneshot::channel();
        let task = tokio::spawn(async move {
            receiver.await.map_err(|_| AppError::ServerStopped)?;
            Ok(())
        });
        let server = RunningServer {
            local_addr: placeholder_address(),
            shutdown: Some(sender),
            task: Some(task),
        };
        let started = Instant::now();
        drive(server, async { Ok(()) }, Duration::from_secs(2))
            .await
            .expect("graceful shutdown");
        assert!(started.elapsed() < Duration::from_secs(2));
    }

    #[tokio::test]
    async fn shutdown_timeout_aborts_a_task_that_ignores_the_signal() {
        let (sender, receiver) = oneshot::channel::<()>();
        let task = tokio::spawn(async move {
            let _hold = receiver;
            pending::<Result<(), AppError>>().await
        });
        let server = RunningServer {
            local_addr: placeholder_address(),
            shutdown: Some(sender),
            task: Some(task),
        };
        let started = Instant::now();
        let error = drive(server, async { Ok(()) }, Duration::from_millis(200))
            .await
            .expect_err("timeout");
        assert!(matches!(error, AppError::ShutdownTimeout), "{error}");
        assert!(started.elapsed() < Duration::from_secs(2));
    }

    #[tokio::test]
    async fn unexpected_task_completion_is_reported() {
        let (sender, _receiver) = oneshot::channel();
        let task =
            tokio::spawn(async { Err(AppError::Serve(io::Error::other("listener closed"))) });
        let server = RunningServer {
            local_addr: placeholder_address(),
            shutdown: Some(sender),
            task: Some(task),
        };
        let started = Instant::now();
        let error = drive(
            server,
            pending::<Result<(), AppError>>(),
            Duration::from_secs(5),
        )
        .await
        .expect_err("unexpected exit");
        assert!(matches!(error, AppError::Serve(_)), "{error}");
        assert!(started.elapsed() < Duration::from_secs(2));
    }

    #[tokio::test]
    async fn unexpected_clean_exit_fails_without_waiting_for_the_timeout() {
        let (sender, _receiver) = oneshot::channel();
        let task = tokio::spawn(async { Ok(()) });
        let server = RunningServer {
            local_addr: placeholder_address(),
            shutdown: Some(sender),
            task: Some(task),
        };
        let started = Instant::now();
        let error = drive(
            server,
            pending::<Result<(), AppError>>(),
            Duration::from_secs(5),
        )
        .await
        .expect_err("clean unexpected exit");
        assert!(matches!(error, AppError::ServerStopped), "{error}");
        assert!(started.elapsed() < Duration::from_secs(2));
    }

    #[tokio::test]
    async fn drop_aborts_without_blocking() {
        let (sender, receiver) = oneshot::channel::<()>();
        let (started_tx, started_rx) = oneshot::channel();
        let (done_tx, done_rx) = oneshot::channel();
        let task = tokio::spawn(async move {
            struct Notify(Option<oneshot::Sender<()>>);
            impl Drop for Notify {
                fn drop(&mut self) {
                    if let Some(sender) = self.0.take() {
                        let _ = sender.send(());
                    }
                }
            }
            let _notify = Notify(Some(done_tx));
            let _ = started_tx.send(());
            let _hold = receiver;
            pending::<Result<(), AppError>>().await
        });
        let server = RunningServer {
            local_addr: placeholder_address(),
            shutdown: Some(sender),
            task: Some(task),
        };
        timeout(Duration::from_secs(1), started_rx)
            .await
            .expect("task should start")
            .expect("start notification");
        let started = Instant::now();
        drop(server);
        assert!(started.elapsed() < Duration::from_millis(200));
        timeout(Duration::from_secs(1), done_rx)
            .await
            .expect("drop should abort the server task")
            .expect("abort notification");
    }
}
