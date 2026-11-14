use tracing_subscriber::EnvFilter;
use trading_application::Application;
use trading_persistence::{DatabaseEndpoint, ReadinessProblem};

use crate::config::{Config, LogFormat, LogLevel};
use crate::error::AppError;

/// Installs the process-wide tracing subscriber.
///
/// # Errors
///
/// Returns [`AppError::Tracing`] when a subscriber is already installed.
pub fn init_tracing(level: LogLevel, format: LogFormat) -> Result<(), AppError> {
    // Driver targets are disabled so a connection error cannot reach the log
    // through sqlx. Application logs record classified reasons instead.
    let directives = format!("{level},sqlx=off,sqlx_core=off,sqlx_postgres=off");
    let filter =
        EnvFilter::try_new(directives).map_err(|error| AppError::Tracing(error.to_string()))?;
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

/// Last readiness result recorded for transition logging.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ReadinessSnapshot {
    /// The schema check succeeded.
    Ready,
    /// The schema check failed for a classified reason.
    NotReady(ReadinessProblem),
}

/// Logs a readiness result when it differs from the previous one.
///
/// Repeated probes with the same result produce no log line.
pub(crate) fn log_readiness_transition(
    endpoint: &str,
    previous: &mut Option<ReadinessSnapshot>,
    next: ReadinessSnapshot,
) {
    if *previous == Some(next) {
        return;
    }
    *previous = Some(next);
    match next {
        ReadinessSnapshot::Ready => {
            tracing::info!(postgres = %endpoint, "readiness check passed");
        }
        ReadinessSnapshot::NotReady(problem) => {
            tracing::warn!(
                postgres = %endpoint,
                reason = problem.as_str(),
                "readiness check failed"
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::{self, Write};
    use std::sync::{Arc, Mutex};

    use tracing::Level;
    use trading_application::{Application, DeploymentEnvironment};
    use trading_persistence::database_endpoint;

    use trading_persistence::ReadinessProblem;

    use super::{ReadinessSnapshot, log_readiness_transition, log_starting};
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
    fn readiness_failures_log_once_per_transition_and_omit_secrets() {
        let endpoint = database_endpoint(SECRET_URL).expect("endpoint").to_string();
        let failure = ReadinessSnapshot::NotReady(ReadinessProblem::Unreachable);
        let text = capture_log(|| {
            let mut previous = None;
            log_readiness_transition(&endpoint, &mut previous, failure);
            log_readiness_transition(&endpoint, &mut previous, failure);
            log_readiness_transition(&endpoint, &mut previous, ReadinessSnapshot::Ready);
        });

        assert_eq!(text.matches("readiness check failed").count(), 1);
        assert_eq!(text.matches("readiness check passed").count(), 1);
        assert!(text.contains("database_unreachable"));
        assert!(!text.contains("super-secret-password"));
        assert!(!text.contains(SECRET_URL));
        assert!(!text.contains("postgres://trading:"));
    }
}
