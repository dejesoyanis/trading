use thiserror::Error;
use trading_persistence::{DatabaseUrlProblem, PersistenceError};

/// Configuration failure whose text is safe to print.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// `.env` exists and could not be parsed. The file contents are omitted.
    #[error("failed to read .env")]
    EnvFile,

    /// A provided setting was empty.
    #[error("{field} must not be empty")]
    Empty {
        /// Environment variable name.
        field: &'static str,
    },

    /// `APP_ENV` was not a deployment environment and was not a trading mode.
    #[error(
        "APP_ENV={value} is not valid. Expected local, test, or production. APP_ENV is a deployment environment, not a trading mode"
    )]
    InvalidAppEnv {
        /// Value that was rejected.
        value: String,
    },

    /// `APP_ENV` attempted to select paper or live trading.
    #[error(
        "APP_ENV={value} is not valid. APP_ENV is a deployment environment (local, test, production), not a trading mode. Live trading is not supported"
    )]
    TradingModeUnsupported {
        /// Value that was rejected.
        value: String,
    },

    /// `HTTP_BIND_ADDRESS` was not an IP socket address.
    #[error(
        "HTTP_BIND_ADDRESS={value} is not a valid socket address. Use an IP address and port, for example 127.0.0.1:8080"
    )]
    InvalidBind {
        /// Value that was rejected.
        value: String,
    },

    /// `LOG_LEVEL` was not a known severity.
    #[error("LOG_LEVEL={value} is not valid. Expected error, warn, info, debug, or trace")]
    InvalidLogLevel {
        /// Value that was rejected.
        value: String,
    },

    /// `LOG_FORMAT` was neither `text` nor `json`.
    #[error("LOG_FORMAT={value} is not valid. Expected text or json")]
    InvalidLogFormat {
        /// Value that was rejected.
        value: String,
    },

    /// `DATABASE_URL` was absent.
    #[error("DATABASE_URL is required. Copy .env.example to .env for local development")]
    DatabaseUrlRequired,

    /// `DATABASE_URL` was present and could not be used.
    #[error("DATABASE_URL is invalid: {problem}")]
    InvalidDatabaseUrl {
        /// Non-sensitive reason the URL was rejected.
        problem: DatabaseUrlProblem,
    },
}

/// Failure that stops or is returned by the API process.
#[derive(Debug, Error)]
pub enum AppError {
    /// Process configuration is missing or invalid.
    #[error(transparent)]
    Config(#[from] ConfigError),

    /// Tracing could not be installed.
    #[error("failed to initialize tracing: {0}")]
    Tracing(String),

    /// PostgreSQL configuration was rejected before startup completed.
    #[error(transparent)]
    Database(#[from] PersistenceError),

    /// The HTTP listener could not bind.
    #[error("failed to bind the http listener")]
    Bind(#[source] std::io::Error),

    /// The HTTP server stopped because of an I/O error.
    #[error("http server failed")]
    Serve(#[source] std::io::Error),

    /// The process could not observe Ctrl-C or, on Unix, SIGTERM.
    #[error("failed to listen for a shutdown signal")]
    Signal(#[source] std::io::Error),

    /// The server task panicked or was cancelled outside shutdown.
    #[error("http server task stopped unexpectedly")]
    ServerTask(#[source] tokio::task::JoinError),

    /// The HTTP server finished before a shutdown signal arrived.
    #[error("http server stopped before a shutdown signal")]
    ServerStopped,

    /// Graceful shutdown exceeded its bound and the server task was aborted.
    #[error("http server did not finish shutting down before the timeout")]
    ShutdownTimeout,

    /// The process was started with an unknown command.
    #[error(
        "usage: trading-api [migrate]. migrate applies infrastructure migrations and exits. Starting trading-api does not migrate"
    )]
    Usage,
}
