use std::fmt;

use thiserror::Error;

/// Why a PostgreSQL URL was rejected.
///
/// The reason never includes the URL or its password.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseUrlProblem {
    /// The value is not a URL.
    Malformed,
    /// The scheme is neither `postgres` nor `postgresql`.
    UnsupportedScheme,
    /// The URL does not name a TCP host.
    MissingHost,
    /// The URL parsed, and the PostgreSQL driver still rejected it.
    Rejected,
}

impl fmt::Display for DatabaseUrlProblem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Malformed => "it is not a valid URL",
            Self::UnsupportedScheme => "the scheme must be postgres or postgresql",
            Self::MissingHost => "it must include a host",
            Self::Rejected => "the postgres driver rejected it",
        })
    }
}

/// Failure from the PostgreSQL foundation.
#[derive(Debug, Error)]
pub enum PersistenceError {
    /// The connection URL cannot be used.
    #[error("postgres connection url is invalid: {0}")]
    InvalidUrl(DatabaseUrlProblem),

    /// An infrastructure migration could not be applied.
    #[error("postgres migration failed")]
    Migrate(#[source] sqlx::migrate::MigrateError),

    /// A connectivity or metadata query could not be completed.
    #[error("postgres query failed")]
    Query(#[source] sqlx::Error),
}
