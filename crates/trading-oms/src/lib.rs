//! Reserved boundary for the future order management system.
//!
//! No order type, order store, or order-processing function is implemented.
//! Those behaviors wait for approved implementation-level contracts.

/// Order processing is not implemented in this milestone.
pub const ORDER_PROCESSING_IMPLEMENTED: bool = false;

#[cfg(test)]
mod tests {
    use super::ORDER_PROCESSING_IMPLEMENTED;

    #[test]
    fn order_processing_is_not_implemented() {
        const { assert!(!ORDER_PROCESSING_IMPLEMENTED) };
    }
}
