//! Technical primitives shared by the trading platform.
//!
//! This crate is not a general utility collection. It holds the process
//! primitives used by the foundation: redacted secrets, an HTTP request
//! identifier, and error-chain rendering. Domain identifiers live in
//! `trading-domain`. [`HttpRequestId`] is not that crate's correlation id.

mod correlation;
mod error_chain;
mod secret;

pub use correlation::HttpRequestId;
pub use error_chain::error_chain;
pub use secret::SecretString;
