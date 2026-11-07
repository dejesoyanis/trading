//! Trading API process.
//!
//! The process exposes liveness and PostgreSQL readiness. It does not expose
//! trading commands, and readiness is not authorization to trade.

mod config;
mod error;
mod health;
mod logging;
mod server;

pub use config::{Config, LogFormat, LogLevel, load_local_env};
pub use error::AppError;
pub use logging::init_tracing;
pub use server::{RunningServer, start};
