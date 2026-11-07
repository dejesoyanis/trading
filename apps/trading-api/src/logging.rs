use tracing_subscriber::EnvFilter;
use trading_application::Application;
use trading_common::error_chain;
use trading_persistence::{DatabaseEndpoint, redact_connection_secrets};

use crate::config::{Config, LogFormat, LogLevel};
use crate::error::AppError;

/// Installs the process-wide tracing subscriber.
///
/// # Errors
///
/// Returns [`AppError::Tracing`] when a subscriber is already installed.
pub fn init_tracing(level: LogLevel, format: LogFormat) -> Result<(), AppError> {
    let filter = EnvFilter::new(level.as_str());
    let result = match format {
        LogFormat::Json => tracing_subscriber::fmt()
            .json()
            .with_env_filter(filter)
            .with_current_span(true)
            .with_ansi(false)
            .try_init(),
        LogFormat::Text => tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_target(true)
            .with_ansi(false)
            .try_init(),
    };
    result.map_err(|error| AppError::Tracing(error.to_string()))
}

/// Logs process startup without the database password.
pub(crate) fn log_starting(
    application: &Application,
    config: &Config,
    endpoint: &DatabaseEndpoint,
) {
    tracing::info!(
        app_env = application.environment().as_str(),
        http_bind = %config.http_bind(),
        log_level = %config.log_level(),
        log_format = %config.log_format(),
        postgres = %endpoint,
        "starting trading-api"
    );
}

/// Logs a PostgreSQL failure after removing the connection secret.
pub(crate) fn log_postgres_unavailable(
    endpoint: &DatabaseEndpoint,
    database_url: &str,
    error: &(dyn std::error::Error + 'static),
) {
    let message = redact_connection_secrets(&error_chain(error), database_url);
    tracing::error!(
        postgres = %endpoint,
        error = %message,
        "postgres is not ready"
    );
}

#[cfg(test)]
mod tests {
    use std::io::{self, Write};
    use std::sync::{Arc, Mutex};

    use tracing::Level;
    use trading_application::{Application, DeploymentEnvironment};
    use trading_persistence::database_endpoint;

    use super::{log_postgres_unavailable, log_starting};
    use crate::config::Config;

    const SECRET_URL: &str = "postgres://trading:super-secret-password@127.0.0.1:5432/trading";

    #[derive(Clone)]
    struct BufferWriter(Arc<Mutex<Vec<u8>>>);

    impl Write for BufferWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            let mut guard = self
                .0
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            guard.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[derive(Debug)]
    struct LeakyError(String);

    impl std::fmt::Display for LeakyError {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str(&self.0)
        }
    }

    impl std::error::Error for LeakyError {}

    fn capture_log(record: impl FnOnce()) -> String {
        let buffer = Arc::new(Mutex::new(Vec::new()));
        let writer = BufferWriter(Arc::clone(&buffer));
        let subscriber = tracing_subscriber::fmt()
            .with_max_level(Level::INFO)
            .with_ansi(false)
            .without_time()
            .with_writer(move || writer.clone())
            .finish();
        tracing::subscriber::with_default(subscriber, record);
        let bytes = buffer
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone();
        String::from_utf8(bytes).expect("log text")
    }

    #[test]
    fn startup_log_omits_the_database_password() {
        let config = Config::from_pairs([("DATABASE_URL", SECRET_URL)]).expect("config");
        let endpoint = database_endpoint(SECRET_URL).expect("endpoint");
        let application = Application::new(DeploymentEnvironment::Local);
        let text = capture_log(|| log_starting(&application, &config, &endpoint));

        assert!(text.contains("starting trading-api"));
        assert!(text.contains("127.0.0.1"));
        assert!(!text.contains("super-secret-password"));
        assert!(!text.contains(SECRET_URL));
    }

    #[test]
    fn postgres_error_log_redacts_a_leaked_url() {
        let endpoint = database_endpoint(SECRET_URL).expect("endpoint");
        let error = LeakyError(format!("connection failed for {SECRET_URL}"));
        let text = capture_log(|| log_postgres_unavailable(&endpoint, SECRET_URL, &error));

        assert!(text.contains("postgres is not ready"));
        assert!(text.contains("[redacted]"));
        assert!(!text.contains("super-secret-password"));
        assert!(!text.contains(SECRET_URL));
    }
}
