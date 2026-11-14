//! Draft domain vocabulary for the trading platform.
//!
//! The types in this crate are shells from CR-B10. They do not submit orders,
//! evaluate risk, resolve broker identities, or transition states. DET-01
//! through DET-10 were not in this repository and their transition tables are
//! not invented here.
//!
//! `trading_common::HttpRequestId` is the HTTP request identifier. The
//! [`CorrelationId`] in this crate is the domain correlation id. Neither crate
//! converts one into the other. The HTTP header remains `x-correlation-id`.
//!
//! UUID generation is version 4. Bit order is not event order. Timestamp order
//! is not event order.
//!
//! # Compile-time boundaries
//!
//! ```
//! use trading_domain::OrderId;
//!
//! let order = OrderId::from_uuid(uuid::Uuid::nil());
//! assert_eq!(order.as_uuid(), uuid::Uuid::nil());
//! ```
//!
//! ```compile_fail,E0308
//! use trading_domain::{CommandId, OrderId};
//! let id = uuid::Uuid::nil();
//! let _same = OrderId::from_uuid(id) == CommandId::from_uuid(id);
//! ```
//!
//! ```compile_fail,E0308
//! use trading_domain::{CausationId, CorrelationId};
//! let id = uuid::Uuid::nil();
//! let _same = CorrelationId::from_uuid(id) == CausationId::from_uuid(id);
//! ```
//!
//! ```compile_fail,E0308
//! use trading_common::HttpRequestId;
//! use trading_domain::CorrelationId;
//! let http = HttpRequestId::generate();
//! let domain = CorrelationId::from_uuid(uuid::Uuid::nil());
//! let _same = http == domain;
//! ```
//!
//! ```compile_fail,E0308
//! use trading_common::HttpRequestId;
//! use trading_domain::CorrelationId;
//! fn accepts(id: CorrelationId) -> CorrelationId {
//!     id
//! }
//! accepts(HttpRequestId::generate());
//! ```
//!
//! ```compile_fail,E0308
//! use trading_domain::{OrderVersion, WriterEpoch};
//! let _same = OrderVersion::new(1) == WriterEpoch::new(1);
//! ```
//!
//! ```compile_fail,E0599
//! use trading_domain::SchemaVersion;
//! let _next = SchemaVersion::new(1).checked_next();
//! ```
//!
//! ```compile_fail,E0308
//! use trading_domain::{OrderWorkflowState, RiskReservationState};
//! fn accepts(state: RiskReservationState) -> RiskReservationState {
//!     state
//! }
//! accepts(OrderWorkflowState::Closed);
//! ```
//!
//! ```compile_fail,E0599
//! use trading_domain::OrderWorkflowState;
//! let _missing = OrderWorkflowState::Released;
//! ```
//!
//! ```compile_fail,E0599
//! use trading_domain::RiskReservationState;
//! let _missing = RiskReservationState::Closed;
//! ```
//!
//! ```compile_fail,E0599
//! use trading_domain::RiskReservationState;
//! let _missing = RiskReservationState::Adjusted;
//! ```
//!
//! ```compile_fail,E0599
//! use trading_domain::TradingReadinessState;
//! let _state = TradingReadinessState::default();
//! ```
//!
//! ```compile_fail,E0599
//! use trading_domain::RiskDecision;
//! let _decision = RiskDecision::default();
//! ```
//!
//! ```compile_fail,E0599
//! use trading_domain::DispatchOutcome;
//! let _outcome = DispatchOutcome::default();
//! ```
//!
//! ```compile_fail,E0308
//! use trading_domain::DecimalAmount;
//! let _amount = DecimalAmount::try_new(1.5_f64, 0);
//! ```
//!
//! ```compile_fail,E0369
//! use trading_domain::{CurrencyCode, DecimalAmount, Money};
//! let amount = DecimalAmount::try_new(1, 0).unwrap();
//! let money = Money::new(amount, CurrencyCode::new("USD").unwrap());
//! let _sum = money + money;
//! ```
//!
//! ```compile_fail,E0308
//! use trading_domain::{OccurredAt, PersistedAt};
//! fn accepts(time: PersistedAt) -> PersistedAt {
//!     time
//! }
//! accepts(OccurredAt::from_unix_microseconds(0));
//! ```
//!
//! ```compile_fail,E0308
//! use trading_domain::{BrokerOrderAlias, BrokerPermIdAlias};
//! fn accepts(alias: BrokerPermIdAlias) -> BrokerPermIdAlias {
//!     alias
//! }
//! let account = trading_domain::BrokerAccountKey::new("IBKR", "U1", "label").unwrap();
//! let scope = trading_domain::BrokerAliasScope::account_only(trading_domain::UnverifiedSessionScope);
//! let external = trading_domain::ExternalBrokerId::new("1001").unwrap();
//! accepts(BrokerOrderAlias::new(account, scope, external));
//! ```
//!
//! ```compile_fail,E0308
//! use trading_domain::{ExecutionIdentity, QuarantinedExecutionIdentity};
//! fn accepts(identity: ExecutionIdentity) -> ExecutionIdentity {
//!     identity
//! }
//! let account = trading_domain::BrokerAccountKey::new("IBKR", "U1", "label").unwrap();
//! let scope = trading_domain::BrokerAliasScope::account_only(trading_domain::UnverifiedSessionScope);
//! let external = trading_domain::ExternalBrokerId::new("EX").unwrap();
//! let recorded = ExecutionIdentity::new(account, scope, external);
//! accepts(recorded.quarantine());
//! ```
//!
//! ```compile_fail,E0308
//! use trading_domain::{BrokerClientId, BrokerSessionId};
//! fn accepts(id: BrokerSessionId) -> BrokerSessionId {
//!     id
//! }
//! accepts(BrokerClientId::new("client-1").unwrap());
//! ```
//!
//! ```compile_fail,E0599
//! use trading_domain::BrokerIdentityContext;
//! let _verified = BrokerIdentityContext::verified();
//! ```
//!
//! ```compile_fail,E0599
//! use trading_domain::BrokerIdentityContext;
//! let _merged = BrokerIdentityContext::merge();
//! ```
//!
//! ```compile_fail,E0308
//! use trading_domain::{AdapterSessionEpoch, WriterEpoch};
//! let _same = AdapterSessionEpoch::new(1) == WriterEpoch::new(1);
//! ```
//!
//! ```compile_fail,E0599
//! use trading_domain::AdapterSessionEpoch;
//! let _next = AdapterSessionEpoch::new(1).checked_next();
//! ```
//!
//! ```compile_fail,E0369
//! use trading_domain::RiskPolicyVersion;
//! let left = RiskPolicyVersion::new("v1").unwrap();
//! let right = RiskPolicyVersion::new("v2").unwrap();
//! let _order = left < right;
//! ```
//!
//! ```compile_fail,E0599
//! use trading_domain::OccurredAt;
//! let _quality = OccurredAt::from_unix_microseconds(0).clock_quality();
//! ```
//!
//! ```compile_fail,E0432
//! use trading_domain::OrderClosure;
//! let _closure = OrderClosure::new();
//! ```

mod amount;
mod error;
mod identifier;
mod identity;
mod parse;
mod state;
mod time;
mod version;

pub use amount::{
    CurrencyCode, DecimalAmount, InstrumentId, MAX_DECIMAL_SCALE, Money, Price, Quantity,
    RoundingPolicy,
};
pub use error::{
    DecimalError, FinancialGap, IdParseError, IdentityParseError, IntegerParseError, MoneyError,
    PriceError, QuantityError, TimeError, TimeGap, TokenError, UnknownState, VersionOverflow,
};
pub use identifier::{
    AuditRecordId, BrokerObservationId, BrokerSessionId, CausationId, CommandId, CorrelationId,
    EventId, ExecutionCorrectionId, IncidentId, MarketObservationId, MarketSnapshotId, OrderId,
    ReconciliationRunId, RiskEvaluationId, RiskReservationId, SubmissionAttemptId,
    ValuationSnapshotId,
};
pub use identity::{
    AdapterSessionEpoch, BrokerAccountCode, BrokerAccountKey, BrokerAliasScope, BrokerClientId,
    BrokerEnvironmentLabel, BrokerIdentityContext, BrokerName, BrokerOrderAlias, BrokerPermIdAlias,
    ExecutionIdentity, ExternalAliasKind, ExternalBrokerId, IdentityEvidenceId,
    QuarantinedExecutionIdentity, UnverifiedSessionScope,
};
pub use state::{
    BrokerLifecycleState, DispatchOutcome, ExecutionState, OrderWorkflowState,
    ReconciliationRunState, RiskDecision, RiskReservationState, TradingReadinessState,
};
pub use time::{OccurredAt, PersistedAt, ReceivedAt};
pub use version::{
    InstrumentDefinitionVersion, NormalizerVersion, OrderVersion, RiskPolicyVersion, SchemaVersion,
    ValuationModelVersion, WriterEpoch,
};

/// The order state machine is not implemented.
pub const ORDER_STATE_MACHINE_IMPLEMENTED: bool = false;

/// State transitions are not implemented.
pub const STATE_TRANSITIONS_IMPLEMENTED: bool = false;

/// Broker-identity resolution is not implemented.
pub const IDENTITY_RESOLVER_IMPLEMENTED: bool = false;

/// Decimal rounding policy is not implemented.
pub const FINANCIAL_ROUNDING_IMPLEMENTED: bool = false;

/// Broker dispatch is not implemented.
pub const BROKER_DISPATCH_IMPLEMENTED: bool = false;

/// Risk evaluation is not implemented.
pub const RISK_EVALUATION_IMPLEMENTED: bool = false;

#[cfg(test)]
mod tests {
    use super::{
        BROKER_DISPATCH_IMPLEMENTED, FINANCIAL_ROUNDING_IMPLEMENTED, IDENTITY_RESOLVER_IMPLEMENTED,
        ORDER_STATE_MACHINE_IMPLEMENTED, RISK_EVALUATION_IMPLEMENTED,
        STATE_TRANSITIONS_IMPLEMENTED,
    };

    #[test]
    fn order_state_machine_is_not_implemented() {
        const { assert!(!ORDER_STATE_MACHINE_IMPLEMENTED) };
    }

    #[test]
    fn behavior_flags_stay_closed() {
        const { assert!(!STATE_TRANSITIONS_IMPLEMENTED) };
        const { assert!(!IDENTITY_RESOLVER_IMPLEMENTED) };
        const { assert!(!FINANCIAL_ROUNDING_IMPLEMENTED) };
        const { assert!(!BROKER_DISPATCH_IMPLEMENTED) };
        const { assert!(!RISK_EVALUATION_IMPLEMENTED) };
    }

    #[test]
    fn http_request_id_is_a_different_type_from_domain_correlation() {
        let http = std::any::type_name::<trading_common::HttpRequestId>();
        let domain = std::any::type_name::<crate::CorrelationId>();
        assert!(http.ends_with("HttpRequestId"));
        assert!(domain.ends_with("CorrelationId"));
        assert_ne!(http, domain);
    }
}
