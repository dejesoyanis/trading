//! Reserved boundary for future broker-neutral types.
//!
//! No order-submission trait or other operational broker port is defined.
//! A trait is not introduced ahead of an approved contract.

/// A broker order-submission interface is not implemented in this milestone.
pub const ORDER_SUBMISSION_INTERFACE_IMPLEMENTED: bool = false;

#[cfg(test)]
mod tests {
    use super::ORDER_SUBMISSION_INTERFACE_IMPLEMENTED;

    #[test]
    fn order_submission_interface_is_not_implemented() {
        const { assert!(!ORDER_SUBMISSION_INTERFACE_IMPLEMENTED) };
    }
}
