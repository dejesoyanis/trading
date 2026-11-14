//! Owner-scoped UUID identifiers.
//!
//! Each identifier is a distinct type. Equal bits do not make two types the
//! same id, and UUID order is not event order. Generation uses UUID version 4.
//! Version 7 was not adopted. Parsed text must be the lowercase hyphenated
//! form produced by [`std::fmt::Display`].

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

use crate::error::IdParseError;
use crate::parse::parse_canonical_uuid;

macro_rules! define_ids {
    ($(
        $(#[$meta:meta])*
        $name:ident
    ),+ $(,)?) => {
        $(
            $(#[$meta])*
            #[must_use]
            #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
            pub struct $name(Uuid);

            impl $name {
                /// Generates a random UUID version-4 value.
                ///
                /// The result is opaque. Its timestamp bits, if any future
                /// constructor uses them, are not an event clock.
                pub fn generate() -> Self {
                    Self(Uuid::new_v4())
                }

                /// Wraps a UUID without inspecting its version or time.
                ///
                /// Copying bits into another identifier type requires that
                /// type's own constructor. The copy does not link the ids.
                pub const fn from_uuid(value: Uuid) -> Self {
                    Self(value)
                }

                /// Returns the wrapped UUID bits.
                pub const fn as_uuid(self) -> Uuid {
                    self.0
                }
            }

            impl fmt::Display for $name {
                fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                    fmt::Display::fmt(&self.0.hyphenated(), formatter)
                }
            }

            impl FromStr for $name {
                type Err = IdParseError;

                fn from_str(text: &str) -> Result<Self, Self::Err> {
                    parse_canonical_uuid(text, stringify!($name)).map(Self)
                }
            }

            impl TryFrom<&str> for $name {
                type Error = IdParseError;

                fn try_from(value: &str) -> Result<Self, Self::Error> {
                    Self::from_str(value)
                }
            }

            impl Serialize for $name {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    serializer.serialize_str(&self.to_string())
                }
            }

            impl<'de> Deserialize<'de> for $name {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    let value = String::deserialize(deserializer)?;
                    Self::from_str(&value).map_err(serde::de::Error::custom)
                }
            }
        )+

        #[cfg(test)]
        mod generated_id_tests {
            use std::any::TypeId;
            use std::str::FromStr;

            use super::*;

            #[test]
            fn identifier_types_are_unique_and_round_trip() {
                let type_ids = [$(TypeId::of::<$name>()),+];
                for (index, type_id) in type_ids.iter().enumerate() {
                    for other in type_ids.iter().skip(index + 1) {
                        assert_ne!(type_id, other);
                    }
                }

                $(
                    let generated = $name::generate();
                    let text = generated.to_string();
                    let parsed = $name::from_str(&text).unwrap();
                    assert_eq!(parsed, generated);
                    assert_eq!(
                        generated.as_uuid().get_version(),
                        Some(uuid::Version::Random)
                    );
                    let json = serde_json::to_string(&generated).unwrap();
                    assert_eq!(json, format!("\"{text}\""));
                    let decoded: $name = serde_json::from_str(&json).unwrap();
                    assert_eq!(decoded, generated);
                )+
            }
        }
    };
}

define_ids! {
    /// Stable internal order identifier.
    ///
    /// Owner: OMS. Never derived from a broker id.
    OrderId,
    /// Command identifier.
    ///
    /// Owner: OMS. Payload-hash equality is not checked here.
    CommandId,
    /// Immutable domain-event identifier.
    ///
    /// Owner: shared infrastructure, assigned by the producer.
    EventId,
    /// Domain correlation identifier for commands and events.
    ///
    /// Owner: shared. This is not the HTTP request handle in `trading-common`.
    /// No conversion between those types is implemented.
    CorrelationId,
    /// Domain causation identifier.
    ///
    /// Owner: shared. Equal bits do not make this a [`CorrelationId`].
    CausationId,
    /// Raw broker-observation identifier.
    ///
    /// Owner: broker integration. The observation body is not stored here.
    BrokerObservationId,
    /// Internal broker-session identifier.
    ///
    /// Owner: broker integration. This does not prove how the broker scopes
    /// its own ids.
    BrokerSessionId,
    /// Dispatch submission-attempt identifier.
    ///
    /// Owner: dispatch. Persisting the attempt is not implemented here.
    SubmissionAttemptId,
    /// Risk-reservation identifier.
    ///
    /// Owner: risk. Distinct from [`OrderId`]. Association is not stored here.
    RiskReservationId,
    /// Risk-evaluation identifier.
    ///
    /// Owner: risk. Rule inputs are not stored here.
    RiskEvaluationId,
    /// Reconciliation-run identifier.
    ///
    /// Owner: recovery.
    ReconciliationRunId,
    /// Incident identifier.
    ///
    /// Owner: operations. Incident lifecycle is not defined in this milestone.
    IncidentId,
    /// Market-observation identifier.
    ///
    /// Owner: market data.
    MarketObservationId,
    /// Market-snapshot identifier.
    ///
    /// Owner: market data. Distinct from [`MarketObservationId`].
    MarketSnapshotId,
    /// Valuation-snapshot identifier.
    ///
    /// Owner: data and risk. Valuation math is not implemented here.
    ValuationSnapshotId,
    /// Append-only audit-record identifier.
    ///
    /// Owner: audit.
    AuditRecordId,
    /// Append-only execution-correction identifier.
    ///
    /// Owner: execution ledger. The referenced execution is not stored here.
    ExecutionCorrectionId,
}

#[cfg(test)]
mod tests {
    use std::any::TypeId;
    use std::str::FromStr;

    use super::{CommandId, OrderId};

    #[test]
    fn canonical_text_is_lowercase_hyphenated_and_rejects_alternates() {
        let nil = OrderId::from_uuid(uuid::Uuid::nil());
        assert_eq!(nil.to_string(), "00000000-0000-0000-0000-000000000000");
        assert_eq!(OrderId::from_str(&nil.to_string()).unwrap(), nil);

        let letters = OrderId::from_uuid(uuid::Uuid::from_u128(
            0xaaaa_bbbb_cccc_dddd_eeee_ffff_0000_1111,
        ));
        let canonical = letters.to_string();
        assert!(canonical.contains('a'));
        let uppercase = canonical.to_ascii_uppercase();
        let error = OrderId::from_str(&uppercase).unwrap_err();
        assert_eq!(error.type_name(), "OrderId");
        assert!(!error.to_string().contains(&uppercase));
        assert!(OrderId::from_str("00000000000000000000000000000000").is_err());
        assert!(OrderId::from_str("urn:uuid:00000000-0000-0000-0000-000000000000").is_err());
        assert!(serde_json::from_str::<OrderId>("0").is_err());
        assert!(serde_json::from_str::<OrderId>("null").is_err());
    }

    #[test]
    fn the_same_bits_remain_different_identifier_types() {
        let text = "00000000-0000-0000-0000-000000000000";
        let order = OrderId::from_str(text).unwrap();
        let command = CommandId::from_str(text).unwrap();
        assert_eq!(order.as_uuid(), command.as_uuid());
        assert_ne!(TypeId::of::<OrderId>(), TypeId::of::<CommandId>());
    }

    #[test]
    fn from_uuid_accepts_any_bits_without_reading_a_clock() {
        let arbitrary = uuid::Uuid::from_u128(0x0123_4567_89ab_cdef_0123_4567_89ab_cdef);
        let order = OrderId::from_uuid(arbitrary);
        assert_eq!(order.as_uuid(), arbitrary);
    }
}
