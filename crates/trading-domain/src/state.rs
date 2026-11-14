//! Draft state labels.
//!
//! CR-B10 section 2 names these enums. This module does not transition them,
//! compare them across types, or treat declaration order as a lifecycle.
//! `RiskReservationAdjusted` is an event name in that contract, not a variant.
//!
//! Unknown text fails closed. [`BrokerLifecycleState::Unknown`] is itself a
//! known label; garbage does not become `Unknown`, `Ready`, `Allow`,
//! `Released`, or `ConfirmedAccepted`.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::error::UnknownState;

macro_rules! state_enum {
    (
        test_round_trip = $round_trip:ident;
        test_rejects = $rejects:ident;
        $(#[$enum_meta:meta])*
        $name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident
            ),+ $(,)?
        }
    ) => {
        $(#[$enum_meta])*
        #[must_use]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum $name {
            $(
                $(#[$variant_meta])*
                $variant,
            )+
        }

        impl $name {
            /// Returns the draft label spelled exactly as the variant.
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => stringify!($variant),)+
                }
            }
        }

        impl TryFrom<&str> for $name {
            type Error = UnknownState;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                match value {
                    $(stringify!($variant) => Ok(Self::$variant),)+
                    _ => Err(UnknownState::new(stringify!($name))),
                }
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?;
                Self::try_from(value.as_str()).map_err(serde::de::Error::custom)
            }
        }

        #[cfg(test)]
        #[test]
        fn $round_trip() {
            let samples = [$( $name::$variant ),+];
            for state in samples {
                assert_eq!($name::try_from(state.as_str()).unwrap(), state);
                let json = serde_json::to_string(&state).unwrap();
                assert_eq!(json, format!("\"{}\"", state.as_str()));
                let decoded: $name = serde_json::from_str(&json).unwrap();
                assert_eq!(decoded, state);
            }
        }

        #[cfg(test)]
        #[test]
        fn $rejects() {
            for label in [
                "",
                "RECONCILIATION_REQUIRED",
                "ADJUSTED",
                "RiskReservationAdjusted",
                "Canceled",
                "ready",
                "READY",
            ] {
                assert!($name::try_from(label).is_err(), "{label}");
                let json = format!("\"{label}\"");
                let decoded = serde_json::from_str::<$name>(&json);
                assert!(decoded.is_err(), "{label}");
            }
            assert!(serde_json::from_str::<$name>("null").is_err());
            assert!(serde_json::from_str::<$name>("0").is_err());
            assert!(serde_json::from_str::<$name>("{}").is_err());
            assert!(serde_json::from_str::<$name>("true").is_err());
        }
    };
}

state_enum! {
    test_round_trip = order_workflow_round_trip;
    test_rejects = order_workflow_rejects_unknown_labels;
    /// Internal order-workflow label.
    ///
    /// Owner: OMS. [`Closed`](Self::Closed) is workflow closure. It is not
    /// economic finality, not [`RiskReservationState::Released`], and not
    /// [`ExecutionState::Filled`].
    OrderWorkflowState {
        /// The order has not entered validation.
        Draft,
        /// Validation has been requested. No validator runs here.
        Validating,
        /// Approval is outstanding. No approver runs here.
        AwaitingApproval,
        /// Approval was recorded. Submission is not implied.
        Approved,
        /// Submission is pending. No broker call is implied.
        SubmissionPending,
        /// A submission attempt is in flight. No broker call is made here.
        SubmissionInFlight,
        /// A submission was recorded. This is not broker acceptance.
        Submitted,
        /// Cancellation was requested. The broker is not contacted here.
        CancelRequested,
        /// This workflow requires reconciliation.
        ///
        /// This is not [`RiskReservationState::ReconciliationRequired`] and
        /// not [`ReconciliationRunState::Blocked`].
        ReconciliationRequired,
        /// Local workflow rejection. Not [`BrokerLifecycleState::Rejected`].
        Rejected,
        /// Internal workflow closure. Not economic finality.
        Closed,
    }
}

state_enum! {
    test_round_trip = broker_lifecycle_round_trip;
    test_rejects = broker_lifecycle_rejects_unknown_labels;
    /// Broker-normalized status observation.
    ///
    /// Owner: broker adapter. This is not execution truth and not
    /// [`OrderWorkflowState`]. [`Unknown`](Self::Unknown) is a recorded label.
    /// Text that is not a variant does not become `Unknown`.
    BrokerLifecycleState {
        /// No broker status has been recorded.
        NotObserved,
        /// The adapter recorded a pending submission status.
        PendingSubmit,
        /// The adapter recorded broker acceptance.
        ///
        /// This is not [`DispatchOutcome::ConfirmedAccepted`] and not
        /// [`RiskDecision::Allow`].
        Accepted,
        /// The adapter recorded a working status.
        Working,
        /// The adapter recorded a pending cancel status.
        PendingCancel,
        /// The adapter recorded broker cancellation.
        ///
        /// The contract spelling is `Cancelled`. `Canceled` is rejected.
        /// Recorded cancellation does not erase executions or release risk.
        Cancelled,
        /// The adapter recorded broker rejection.
        Rejected,
        /// The adapter recorded expiry.
        Expired,
        /// The adapter recorded that the external status was unknown.
        Unknown,
    }
}

state_enum! {
    test_round_trip = execution_round_trip;
    test_rejects = execution_rejects_unknown_labels;
    /// Execution-ledger fill label.
    ///
    /// Owner: execution ledger. CR-B10 section 1 does not give this enum its
    /// own row; section 2 defines it. It is not workflow closure.
    ExecutionState {
        /// No fill is recorded.
        Unfilled,
        /// A partial fill is recorded. This is not finality.
        PartiallyFilled,
        /// A fill label. Not workflow closure and not risk release.
        Filled,
        /// A dispute is recorded. This crate does not resolve it.
        Disputed,
    }
}

state_enum! {
    test_round_trip = risk_reservation_round_trip;
    test_rejects = risk_reservation_rejects_unknown_labels;
    /// Risk-reservation lifecycle label.
    ///
    /// Owner: risk. `RiskReservationAdjusted` is not a variant. An adjusted
    /// reservation is not modeled here. [`Released`](Self::Released) is not
    /// [`OrderWorkflowState::Closed`].
    RiskReservationState {
        /// The reservation is active.
        Active,
        /// This reservation requires reconciliation.
        ///
        /// Not [`OrderWorkflowState::ReconciliationRequired`] and not
        /// [`ReconciliationRunState::Blocked`].
        ReconciliationRequired,
        /// The reservation is released. Not workflow closure.
        Released,
    }
}

state_enum! {
    test_round_trip = trading_readiness_round_trip;
    test_rejects = trading_readiness_rejects_unknown_labels;
    /// Operations trading-readiness label.
    ///
    /// Owner: operations. This is not HTTP readiness and grants no submission
    /// authority. Unknown text does not become [`Ready`](Self::Ready).
    TradingReadinessState {
        /// Trading is not ready.
        NotReady,
        /// Initialization is in progress. No initializer runs here.
        Initializing,
        /// Synchronization is in progress. No sync runs here.
        Synchronizing,
        /// The readiness label is ready.
        ///
        /// This does not authorize an order and it is not an HTTP status.
        Ready,
        /// Readiness is degraded.
        Degraded,
        /// Trading is halted. This is not a process shutdown.
        Halted,
        /// Recovery is in progress. No recovery runs here.
        Recovering,
        /// A manual reset is the recorded label. No reset runs here.
        ManualReset,
    }
}

state_enum! {
    test_round_trip = reconciliation_run_round_trip;
    test_rejects = reconciliation_run_rejects_unknown_labels;
    /// Reconciliation-run label.
    ///
    /// Owner: recovery. [`Blocked`](Self::Blocked) is not the workflow or
    /// reservation label `ReconciliationRequired`.
    ReconciliationRunState {
        /// The run record has been created. No run is executed here.
        Created,
        /// Collection is the recorded label.
        Collecting,
        /// Comparison is the recorded label.
        Comparing,
        /// The run is blocked.
        Blocked,
        /// The run is resolved. This is not economic finality.
        Resolved,
        /// The run failed. This crate does not retry it.
        Failed,
    }
}

state_enum! {
    test_round_trip = risk_decision_round_trip;
    test_rejects = risk_decision_rejects_unknown_labels;
    /// Risk decision label.
    ///
    /// Owner: risk. [`Allow`](Self::Allow) is necessary and not sufficient for
    /// dispatch. Unknown text does not become `Allow`. No policy runs here.
    RiskDecision {
        /// The recorded decision is allow. Dispatch is not performed.
        Allow,
        /// The recorded decision is deny.
        Deny,
        /// The recorded decision is indeterminate. This is not allow.
        Indeterminate,
    }
}

state_enum! {
    test_round_trip = dispatch_outcome_round_trip;
    test_rejects = dispatch_outcome_rejects_unknown_labels;
    /// Dispatch outcome label.
    ///
    /// Owner: dispatch. Unknown text does not become
    /// [`ConfirmedAccepted`](Self::ConfirmedAccepted). No order is sent here.
    DispatchOutcome {
        /// No send was attempted.
        NotAttempted,
        /// A send may have left the process. Reconciliation is not run here.
        MayHaveSent,
        /// Acceptance was confirmed. This is not a deserialization default.
        ConfirmedAccepted,
        /// Rejection was confirmed.
        ConfirmedRejected,
        /// This dispatch outcome requires reconciliation.
        ///
        /// Not [`OrderWorkflowState::ReconciliationRequired`].
        ReconciliationRequired,
    }
}

#[cfg(test)]
mod safety_tests {
    use super::{
        BrokerLifecycleState, DispatchOutcome, ExecutionState, OrderWorkflowState,
        ReconciliationRunState, RiskDecision, RiskReservationState, TradingReadinessState,
    };

    #[test]
    fn safety_critical_garbage_does_not_decode_to_the_permissive_labels() {
        for label in ["ReadyNow", "READY", "NotReadyEnough", "true", "1", "Allow"] {
            assert!(TradingReadinessState::try_from(label).is_err(), "{label}");
            let json = format!("\"{label}\"");
            assert!(serde_json::from_str::<TradingReadinessState>(&json).is_err());
        }
        assert_eq!(
            TradingReadinessState::try_from("Ready").unwrap(),
            TradingReadinessState::Ready
        );

        for label in ["Allowing", "ALLOW", "Ready", "Indeterminateish"] {
            assert!(RiskDecision::try_from(label).is_err(), "{label}");
        }
        assert_eq!(
            RiskDecision::try_from("Allow").unwrap(),
            RiskDecision::Allow
        );
        assert_eq!(
            RiskDecision::try_from("Indeterminate").unwrap(),
            RiskDecision::Indeterminate
        );

        for label in ["Release", "RELEASED", "Closed", "Adjusted", "Activeish"] {
            assert!(RiskReservationState::try_from(label).is_err(), "{label}");
        }
        assert_eq!(
            RiskReservationState::try_from("Released").unwrap(),
            RiskReservationState::Released
        );

        for label in ["Accepted", "Confirmed", "ConfirmedAccept", "MayHaveSentt"] {
            assert!(DispatchOutcome::try_from(label).is_err(), "{label}");
            let json = format!("\"{label}\"");
            assert!(serde_json::from_str::<DispatchOutcome>(&json).is_err());
        }
        assert_eq!(
            DispatchOutcome::try_from("ConfirmedAccepted").unwrap(),
            DispatchOutcome::ConfirmedAccepted
        );
    }

    #[test]
    fn broker_unknown_is_a_real_label_and_garbage_is_not_rewritten_to_it() {
        assert_eq!(
            BrokerLifecycleState::try_from("Unknown").unwrap(),
            BrokerLifecycleState::Unknown
        );
        assert!(BrokerLifecycleState::try_from("Unrecognized").is_err());
        assert!(BrokerLifecycleState::try_from("Canceled").is_err());
        assert_eq!(
            BrokerLifecycleState::try_from("Cancelled").unwrap(),
            BrokerLifecycleState::Cancelled
        );
        assert_eq!(
            BrokerLifecycleState::try_from("PendingCancel").unwrap(),
            BrokerLifecycleState::PendingCancel
        );
        let error = BrokerLifecycleState::try_from("Unrecognized").unwrap_err();
        assert_eq!(error.to_string(), "unknown BrokerLifecycleState value");
        assert!(!error.to_string().contains("Unrecognized"));
    }

    #[test]
    fn reconciliation_required_is_not_a_shared_label() {
        assert_eq!(
            OrderWorkflowState::try_from("ReconciliationRequired").unwrap(),
            OrderWorkflowState::ReconciliationRequired
        );
        assert_eq!(
            RiskReservationState::try_from("ReconciliationRequired").unwrap(),
            RiskReservationState::ReconciliationRequired
        );
        assert_eq!(
            DispatchOutcome::try_from("ReconciliationRequired").unwrap(),
            DispatchOutcome::ReconciliationRequired
        );
        assert!(ReconciliationRunState::try_from("ReconciliationRequired").is_err());
        assert_eq!(
            ReconciliationRunState::try_from("Blocked").unwrap(),
            ReconciliationRunState::Blocked
        );
        assert!(TradingReadinessState::try_from("ReconciliationRequired").is_err());
        assert!(ExecutionState::try_from("ReconciliationRequired").is_err());
        assert!(RiskDecision::try_from("ReconciliationRequired").is_err());
        assert!(BrokerLifecycleState::try_from("ReconciliationRequired").is_err());
    }

    #[test]
    fn reservation_states_do_not_include_an_adjusted_variant() {
        for state in [
            RiskReservationState::Active,
            RiskReservationState::ReconciliationRequired,
            RiskReservationState::Released,
        ] {
            assert!(!state.as_str().to_ascii_uppercase().contains("ADJUST"));
        }
    }

    #[test]
    fn closed_released_and_filled_are_different_labels() {
        assert_ne!(
            OrderWorkflowState::Closed.as_str(),
            RiskReservationState::Released.as_str()
        );
        assert_ne!(
            OrderWorkflowState::Closed.as_str(),
            ExecutionState::Filled.as_str()
        );
        assert!(OrderWorkflowState::try_from("Released").is_err());
        assert!(OrderWorkflowState::try_from("Filled").is_err());
        assert!(RiskReservationState::try_from("Closed").is_err());
        assert!(ExecutionState::try_from("Closed").is_err());
    }
}
