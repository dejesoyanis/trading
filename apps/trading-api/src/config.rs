use std::collections::HashMap;
use std::fmt;
use std::net::SocketAddr;
use std::str::FromStr;

use trading_application::DeploymentEnvironment;
use trading_common::SecretString;
use trading_persistence::{DatabaseUrlProblem, database_endpoint};

use crate::error::ConfigError;

/// Severity threshold for process logs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    /// Error events only.
    Error,
    /// Warnings and errors.
    Warn,
    /// Operational events, warnings, and errors.
    Info,
    /// Diagnostic detail.
    Debug,
    /// Very verbose diagnostic detail.
    Trace,
}

impl LogLevel {
    /// Returns the `tracing` filter directive for this level.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warn => "warn",
            Self::Info => "info",
            Self::Debug => "debug",
            Self::Trace => "trace",
        }
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Output encoding for process logs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogFormat {
    /// Human-readable text with structured fields.
    Text,
    /// One JSON object per event.
    Json,
}

impl LogFormat {
    /// Returns the configuration spelling of this format.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Json => "json",
        }
    }
}

impl fmt::Display for LogFormat {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Validated process configuration.
///
/// The database URL is stored as a [`SecretString`]. Debug output redacts it.
#[derive(Clone)]
pub struct Config {
    app_env: DeploymentEnvironment,
    http_bind: SocketAddr,
    log_level: LogLevel,
    log_format: LogFormat,
    database_url: SecretString,
}

impl Config {
    /// Loads configuration from the process environment.
    ///
    /// Keys are matched without regard to case. An empty value is invalid.
    /// Absent non-secret settings use the local development defaults.
    /// `DATABASE_URL` has no default because it contains a credential.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError`] when a present value is invalid or the database
    /// URL is missing. Error text does not include the database URL.
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_pairs(std::env::vars())
    }

    /// Loads configuration from explicit key-value pairs.
    ///
    /// This is the deterministic constructor used by tests and by [`Self::from_env`].
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError`] when a present value is invalid or the database
    /// URL is missing.
    pub fn from_pairs<I, K, V>(pairs: I) -> Result<Self, ConfigError>
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let mut values = HashMap::<String, String>::new();
        for (key, value) in pairs {
            values.insert(key.as_ref().to_ascii_uppercase(), value.as_ref().to_owned());
        }
        Self::from_map(&values)
    }

    /// Deployment environment. This does not select a trading mode.
    pub fn app_env(&self) -> DeploymentEnvironment {
        self.app_env
    }

    /// Address on which the HTTP server listens.
    pub fn http_bind(&self) -> SocketAddr {
        self.http_bind
    }

    /// Configured log threshold.
    pub fn log_level(&self) -> LogLevel {
        self.log_level
    }

    /// Configured log encoding.
    pub fn log_format(&self) -> LogFormat {
        self.log_format
    }

    /// Database URL. Display and debug forms are redacted.
    pub fn database_url(&self) -> &SecretString {
        &self.database_url
    }

    fn from_map(values: &HashMap<String, String>) -> Result<Self, ConfigError> {
        let app_env = match lookup(values, "APP_ENV")? {
            None => DeploymentEnvironment::Local,
            Some(value) => parse_app_env(value)?,
        };
        let http_bind = match lookup(values, "HTTP_BIND_ADDRESS")? {
            None => SocketAddr::from(([127, 0, 0, 1], 8080)),
            Some(value) => SocketAddr::from_str(value).map_err(|_| ConfigError::InvalidBind {
                value: value.to_owned(),
            })?,
        };
        let log_level = match lookup(values, "LOG_LEVEL")? {
            None => LogLevel::Info,
            Some(value) => parse_log_level(value)?,
        };
        let log_format = match lookup(values, "LOG_FORMAT")? {
            None => LogFormat::Text,
            Some(value) => parse_log_format(value)?,
        };
        let database_url =
            lookup(values, "DATABASE_URL")?.ok_or(ConfigError::DatabaseUrlRequired)?;
        let problem = database_endpoint(database_url)
            .err()
            .map(|error| match error {
                trading_persistence::PersistenceError::InvalidUrl(problem) => problem,
                _ => DatabaseUrlProblem::Rejected,
            });
        if let Some(problem) = problem {
            return Err(ConfigError::InvalidDatabaseUrl { problem });
        }

        Ok(Self {
            app_env,
            http_bind,
            log_level,
            log_format,
            database_url: SecretString::new(database_url),
        })
    }
}

impl fmt::Debug for Config {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Config")
            .field("app_env", &self.app_env)
            .field("http_bind", &self.http_bind)
            .field("log_level", &self.log_level)
            .field("log_format", &self.log_format)
            .field("database_url", &self.database_url)
            .finish()
    }
}

/// Loads a `.env` file from the working directory when one exists.
///
/// Existing process variables win. A missing file is not an error. Parse
/// failures are reported without the file contents.
///
/// # Errors
///
/// Returns [`ConfigError::EnvFile`] when `.env` exists and cannot be read.
pub fn load_local_env() -> Result<(), ConfigError> {
    match dotenvy::dotenv() {
        Ok(_path) => Ok(()),
        Err(error) if error.not_found() => Ok(()),
        Err(_error) => Err(ConfigError::EnvFile),
    }
}

fn lookup<'a>(
    values: &'a HashMap<String, String>,
    key: &'static str,
) -> Result<Option<&'a str>, ConfigError> {
    match values.get(key) {
        None => Ok(None),
        Some(value) if value.trim().is_empty() => Err(ConfigError::Empty { field: key }),
        Some(value) => Ok(Some(value.trim())),
    }
}

fn parse_app_env(value: &str) -> Result<DeploymentEnvironment, ConfigError> {
    match value.to_ascii_lowercase().as_str() {
        "local" => Ok(DeploymentEnvironment::Local),
        "test" => Ok(DeploymentEnvironment::Test),
        "production" => Ok(DeploymentEnvironment::Production),
        "live" | "paper" => Err(ConfigError::TradingModeUnsupported {
            value: value.to_owned(),
        }),
        _ => Err(ConfigError::InvalidAppEnv {
            value: value.to_owned(),
        }),
    }
}

fn parse_log_level(value: &str) -> Result<LogLevel, ConfigError> {
    match value.to_ascii_lowercase().as_str() {
        "error" => Ok(LogLevel::Error),
        "warn" => Ok(LogLevel::Warn),
        "info" => Ok(LogLevel::Info),
        "debug" => Ok(LogLevel::Debug),
        "trace" => Ok(LogLevel::Trace),
        _ => Err(ConfigError::InvalidLogLevel {
            value: value.to_owned(),
        }),
    }
}

fn parse_log_format(value: &str) -> Result<LogFormat, ConfigError> {
    match value.to_ascii_lowercase().as_str() {
        "text" => Ok(LogFormat::Text),
        "json" => Ok(LogFormat::Json),
        _ => Err(ConfigError::InvalidLogFormat {
            value: value.to_owned(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;

    use trading_application::DeploymentEnvironment;

    use super::{Config, LogFormat, LogLevel};
    use crate::error::ConfigError;

    const SECRET_URL: &str = "postgres://trading:super-secret-password@127.0.0.1:5432/trading";

    #[test]
    fn defaults_apply_when_only_the_database_url_is_set() {
        let config = Config::from_pairs([("DATABASE_URL", SECRET_URL)]).expect("config");
        assert_eq!(config.app_env(), DeploymentEnvironment::Local);
        assert_eq!(
            config.http_bind(),
            "127.0.0.1:8080".parse::<SocketAddr>().expect("address")
        );
        assert_eq!(config.log_level(), LogLevel::Info);
        assert_eq!(config.log_format(), LogFormat::Text);
    }

    #[test]
    fn accepts_explicit_non_trading_values() {
        let config = Config::from_pairs([
            ("app_env", "production"),
            ("http_bind_address", "127.0.0.1:9090"),
            ("log_level", "DEBUG"),
            ("log_format", "json"),
            ("database_url", SECRET_URL),
        ])
        .expect("config");
        assert_eq!(config.app_env(), DeploymentEnvironment::Production);
        assert_eq!(config.log_level(), LogLevel::Debug);
        assert_eq!(config.log_format(), LogFormat::Json);
        assert_eq!(config.http_bind().port(), 9090);
    }

    #[test]
    fn debug_redacts_the_database_url() {
        let config = Config::from_pairs([("DATABASE_URL", SECRET_URL)]).expect("config");
        let debug = format!("{config:?}");
        assert_eq!(config.database_url().expose(), SECRET_URL);
        assert!(!debug.contains("super-secret-password"));
        assert!(!debug.contains(SECRET_URL));
    }

    #[test]
    fn rejects_invalid_values_without_echoing_the_database_url() {
        let cases = [
            (vec![("DATABASE_URL", "")], "DATABASE_URL must not be empty"),
            (vec![], "DATABASE_URL is required"),
            (
                vec![
                    ("DATABASE_URL", SECRET_URL),
                    ("HTTP_BIND_ADDRESS", "localhost:8080"),
                ],
                "HTTP_BIND_ADDRESS=localhost:8080",
            ),
            (
                vec![("DATABASE_URL", SECRET_URL), ("LOG_LEVEL", "verbose")],
                "LOG_LEVEL=verbose",
            ),
            (
                vec![("DATABASE_URL", SECRET_URL), ("LOG_FORMAT", "yaml")],
                "LOG_FORMAT=yaml",
            ),
            (
                vec![("DATABASE_URL", SECRET_URL), ("APP_ENV", "staging")],
                "APP_ENV=staging",
            ),
            (
                vec![(
                    "DATABASE_URL",
                    "http://trading:super-secret-password@127.0.0.1/trading",
                )],
                "scheme must be postgres or postgresql",
            ),
        ];

        for (pairs, expected) in cases {
            let error = Config::from_pairs(pairs).expect_err("config should fail");
            let message = error.to_string();
            assert!(message.contains(expected), "{message}");
            assert!(!message.contains("super-secret-password"), "{message}");
        }
    }

    #[test]
    fn rejects_paper_and_live_trading_modes() {
        for value in ["live", "paper", "LIVE", "Paper"] {
            let error = Config::from_pairs([("DATABASE_URL", SECRET_URL), ("APP_ENV", value)])
                .expect_err("trading mode");
            match error {
                ConfigError::TradingModeUnsupported { .. } => {}
                other => panic!("unexpected error: {other}"),
            }
            let message = error.to_string();
            assert!(
                message.contains("Live trading is not supported"),
                "{message}"
            );
            assert!(!message.contains("super-secret-password"));
        }
    }
}
