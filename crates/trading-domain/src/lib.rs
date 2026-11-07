//! Foundation for future domain types and invariants.
//!
//! DET-01 through DET-10 are conceptually approved and still lack
//! implementation-level specifications. Those specifications are not invented
//! here. The order state machine is not implemented.

/// The order state machine is not implemented in this milestone.
pub const ORDER_STATE_MACHINE_IMPLEMENTED: bool = false;

#[cfg(test)]
mod tests {
    use super::ORDER_STATE_MACHINE_IMPLEMENTED;

    #[test]
    fn order_state_machine_is_not_implemented() {
        const { assert!(!ORDER_STATE_MACHINE_IMPLEMENTED) };
    }
}
