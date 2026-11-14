//! PostgreSQL integration for the trading platform.
//!
//! Connecting does not migrate. [`Database::migrate`] applies the
//! infrastructure migration. [`Database::check_ready`] only reads connectivity
//! and schema compatibility. The crate does not create order, risk, execution,
//! or broker-identity tables.

mod database;
mod endpoint;
mod error;
mod readiness;
mod redact;

pub use database::Database;
pub use endpoint::{DatabaseEndpoint, database_endpoint};
pub use error::{DatabaseUrlProblem, PersistenceError, ReadinessProblem};
pub use readiness::{
    ACQUIRE_TIMEOUT, INFRASTRUCTURE_METADATA_KEY, INFRASTRUCTURE_METADATA_VALUE,
    MIGRATION_STATEMENT_TIMEOUT, MIGRATION_TIMEOUT, READINESS_STATEMENT_TIMEOUT, READINESS_TIMEOUT,
};
pub use redact::{redact_connection_secrets, redact_database_urls, redact_error};
