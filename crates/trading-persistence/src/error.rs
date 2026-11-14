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

/// Why a read-only readiness check failed.
///
/// These variants are safe to log. They do not include driver text or secrets.
/// HTTP readiness is not Trading Readiness and grants no submission authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadinessProblem {
    /// A connection could not be established.
    Unreachable,
    /// The check exceeded its time bound, or PostgreSQL canceled the statement.
    TimedOut,
    /// The infrastructure migration has not been applied.
    SchemaMissing,
    /// Migration history or infrastructure metadata does not match this build.
    SchemaIncompatible,
    /// The role could not authenticate or could not read the infrastructure schema.
    AccessDenied,
}

impl ReadinessProblem {
    /// Stable, non-sensitive reason code for logs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unreachable => "database_unreachable",
            Self::TimedOut => "database_timeout",
            Self::SchemaMissing => "schema_missing",
            Self::SchemaIncompatible => "schema_incompatible",
            Self::AccessDenied => "database_access_denied",
        }
    }
}

impl fmt::Display for ReadinessProblem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Unreachable => "the database is unreachable",
            Self::TimedOut => "the database check timed out",
            Self::SchemaMissing => "the infrastructure schema is missing",
            Self::SchemaIncompatible => "the infrastructure schema is incompatible",
            Self::AccessDenied => "database access was denied",
        })
    }
}

/// Failure from the PostgreSQL foundation.
///
/// [`Display`] and [`Debug`] do not include a raw driver error. [`Self::public_reason`]
/// is the routine log field. [`Self::redacted_detail`] is for the migration command
/// and has connection URLs removed; it is not written on health probes.
#[derive(Error)]
pub enum PersistenceError {
    /// The connection URL cannot be used.
    #[error("postgres connection url is invalid: {0}")]
    InvalidUrl(DatabaseUrlProblem),

    /// An infrastructure migration could not be applied.
    ///
    /// `detail` has already been passed through URL redaction.
    #[error("postgres migration failed")]
    Migrate {
        /// Redacted diagnostic text. Do not treat it as a secret-free proof;
        /// callers that still know the URL should redact that URL as well.
        detail: String,
    },

    /// Readiness failed. The process must not repair the schema from this error.
    #[error("postgres is not ready: {0}")]
    NotReady(ReadinessProblem),
}

impl PersistenceError {
    /// Short reason that is safe for routine logs.
    pub const fn public_reason(&self) -> &'static str {
        match self {
            Self::InvalidUrl(_) => "invalid_database_url",
            Self::Migrate { .. } => "migration_failed",
            Self::NotReady(problem) => problem.as_str(),
        }
    }

    /// Class used by readiness logs. Migration and URL failures are not schema states.
    pub const fn readiness_problem(&self) -> ReadinessProblem {
        match self {
            Self::NotReady(problem) => *problem,
            Self::InvalidUrl(_) | Self::Migrate { .. } => ReadinessProblem::Unreachable,
        }
    }

    /// Diagnostic text for an operator. Connection URLs have been stripped from
    /// migration failures. Health probes do not log this.
    pub fn redacted_detail(&self) -> String {
        match self {
            Self::InvalidUrl(problem) => problem.to_string(),
            Self::Migrate { detail } => detail.clone(),
            Self::NotReady(problem) => problem.to_string(),
        }
    }
}

impl fmt::Debug for PersistenceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidUrl(problem) => {
                formatter.debug_tuple("InvalidUrl").field(problem).finish()
            }
            Self::Migrate { detail } => formatter
                .debug_struct("Migrate")
                .field("detail", detail)
                .finish(),
            Self::NotReady(problem) => formatter.debug_tuple("NotReady").field(problem).finish(),
        }
    }
}
