//! PostgreSQL integration for the trading platform.
//!
//! The crate can open a connection, apply the infrastructure-metadata
//! migration, and read that metadata back. It does not create order, risk,
//! execution, or broker-identity tables.

mod database;
mod endpoint;
mod error;
mod redact;

pub use database::Database;
pub use endpoint::{DatabaseEndpoint, database_endpoint};
pub use error::{DatabaseUrlProblem, PersistenceError};
pub use redact::redact_connection_secrets;
