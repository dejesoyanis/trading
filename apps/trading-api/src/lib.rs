//! Trading API process.
//!
//! The process exposes liveness and a read-only PostgreSQL readiness check.
//! `trading-api migrate` applies infrastructure migrations and exits. Starting
//! the server does not migrate. Readiness is not authorization to trade.

mod command;
mod config;
mod error;
mod health;
mod logging;
mod server;

pub use command::{Command, parse_args};
pub use config::{Config, LogFormat, LogLevel, load_local_env};
pub use error::AppError;
pub use logging::init_tracing;
pub use server::{GRACEFUL_SHUTDOWN_TIMEOUT, RunningServer, run_until_shutdown, start};
