//! Technical primitives shared by the trading platform.
//!
//! This crate is not a general utility collection. It holds the process
//! primitives used by the foundation: redacted secrets, correlation
//! identifiers, and error-chain rendering.

mod correlation;
mod error_chain;
mod secret;

pub use correlation::CorrelationId;
pub use error_chain::error_chain;
pub use secret::SecretString;
