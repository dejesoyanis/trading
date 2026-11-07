//! Reserved boundary for a future Interactive Brokers adapter.
//!
//! This crate does not connect to Interactive Brokers and does not contain
//! broker credentials. It does not depend on the trading-broker crate because
//! that crate does not yet define a contract to implement.

/// An Interactive Brokers connection is not implemented in this milestone.
pub const IBKR_CONNECTION_IMPLEMENTED: bool = false;

#[cfg(test)]
mod tests {
    use super::IBKR_CONNECTION_IMPLEMENTED;

    #[test]
    fn ibkr_connection_is_not_implemented() {
        const { assert!(!IBKR_CONNECTION_IMPLEMENTED) };
    }
}
