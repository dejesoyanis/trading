//! Reserved boundary for the future deterministic risk engine.
//!
//! No risk policy, risk check, or risk reservation is implemented.

/// The risk engine is not implemented in this milestone.
pub const RISK_ENGINE_IMPLEMENTED: bool = false;

#[cfg(test)]
mod tests {
    use super::RISK_ENGINE_IMPLEMENTED;

    #[test]
    fn risk_engine_is_not_implemented() {
        const { assert!(!RISK_ENGINE_IMPLEMENTED) };
    }
}
