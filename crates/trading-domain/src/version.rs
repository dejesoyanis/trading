//! Independent version and fencing counters.
//!
//! Numeric order of [`OrderVersion`] is the aggregate counter, not proof that
//! timestamps or UUIDs are ordered. [`SchemaVersion`] has no successor method:
//! a larger integer is not a compatibility claim.

use std::fmt;
use std::str::FromStr;

use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::{Serialize, Serializer};

use crate::error::{IntegerParseError, TokenError, VersionOverflow};
use crate::parse::{TokenClass, parse_canonical_u64, parse_token};

const VERSION_TOKEN_MAX: usize = 128;

/// Opaque version text whose numeric or semantic-version form is not specified.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct OpaqueVersion(String);

impl OpaqueVersion {
    fn new(value: &str) -> Result<Self, TokenError> {
        parse_token(value, 1, VERSION_TOKEN_MAX, TokenClass::PrintableAscii).map(Self)
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

fn deserialize_exact_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    struct ExactU64;

    impl<'de> Visitor<'de> for ExactU64 {
        type Value = u64;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("an unsigned integer")
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value)
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            u64::try_from(value).map_err(|_| E::custom(IntegerParseError))
        }
    }

    deserializer.deserialize_any(ExactU64)
}

macro_rules! counter_version {
    (
        $(#[$meta:meta])*
        $name:ident,
        $overflow:ident
    ) => {
        $(#[$meta])*
        #[must_use]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(u64);

        impl $name {
            /// Wraps a counter value. Zero is allowed; no initial value is implied.
            pub const fn new(value: u64) -> Self {
                Self(value)
            }

            /// Returns the counter.
            pub const fn get(self) -> u64 {
                self.0
            }

            /// Returns the next counter, or an overflow error at `u64::MAX`.
            ///
            /// The original value is unchanged when this returns an error.
            pub const fn checked_next(self) -> Result<Self, VersionOverflow> {
                match self.0.checked_add(1) {
                    Some(value) => Ok(Self(value)),
                    None => Err(VersionOverflow::$overflow),
                }
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "{}", self.0)
            }
        }

        impl FromStr for $name {
            type Err = IntegerParseError;

            fn from_str(text: &str) -> Result<Self, Self::Err> {
                parse_canonical_u64(text).map(Self)
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_u64(self.0)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserialize_exact_u64(deserializer).map(Self)
            }
        }
    };
}

macro_rules! schema_version {
    (
        $(#[$meta:meta])*
        $name:ident
    ) => {
        $(#[$meta])*
        #[must_use]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct $name(u64);

        impl $name {
            /// Wraps a schema integer. Ordering and compatibility are not implied.
            pub const fn new(value: u64) -> Self {
                Self(value)
            }

            /// Returns the integer.
            pub const fn get(self) -> u64 {
                self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "{}", self.0)
            }
        }

        impl FromStr for $name {
            type Err = IntegerParseError;

            fn from_str(text: &str) -> Result<Self, Self::Err> {
                parse_canonical_u64(text).map(Self)
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_u64(self.0)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserialize_exact_u64(deserializer).map(Self)
            }
        }
    };
}

macro_rules! opaque_versions {
    ($(
        $(#[$meta:meta])*
        $name:ident
    ),+ $(,)?) => {
        $(
            $(#[$meta])*
            #[must_use]
            #[derive(Clone, Debug, PartialEq, Eq, Hash)]
            pub struct $name(OpaqueVersion);

            impl $name {
                /// Stores version text without interpreting its scheme.
                pub fn new(value: &str) -> Result<Self, TokenError> {
                    OpaqueVersion::new(value).map(Self)
                }

                /// Returns the stored text.
                pub fn as_str(&self) -> &str {
                    self.0.as_str()
                }
            }

            impl fmt::Display for $name {
                fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                    formatter.write_str(self.as_str())
                }
            }

            impl FromStr for $name {
                type Err = TokenError;

                fn from_str(text: &str) -> Result<Self, Self::Err> {
                    Self::new(text)
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
                    Self::new(&value).map_err(serde::de::Error::custom)
                }
            }
        )+

        #[cfg(test)]
        mod generated_opaque_version_tests {
            use std::any::TypeId;

            use super::*;

            #[test]
            fn opaque_version_types_are_unique() {
                let type_ids = [$(TypeId::of::<$name>()),+];
                for (index, type_id) in type_ids.iter().enumerate() {
                    for other in type_ids.iter().skip(index + 1) {
                        assert_ne!(type_id, other);
                    }
                }
            }
        }
    };
}

counter_version! {
    /// Monotonic order-aggregate counter.
    ///
    /// Owner: OMS. [`Ord`] compares the counter. It does not order events by
    /// time or by UUID. Overflow fails instead of wrapping.
    OrderVersion,
    OrderVersion
}

counter_version! {
    /// Local database fencing counter.
    ///
    /// Owner: dispatch. This is not an external broker fence. Overflow fails
    /// instead of wrapping. The `u64` width is a provisional storage choice.
    WriterEpoch,
    WriterEpoch
}

schema_version! {
    /// Event or schema document integer.
    ///
    /// Owner: shared infrastructure. This is not an aggregate version. A
    /// larger value is not treated as a compatible successor.
    SchemaVersion
}

opaque_versions! {
    /// Published risk-policy version.
    ///
    /// Owner: risk. The text form is provisional. CR-B10 section 3 also says
    /// `PolicyVersion`; this crate uses one type and does not treat that alias
    /// as a second identifier.
    RiskPolicyVersion,
    /// Broker-normalizer version.
    ///
    /// Owner: broker adapter. Ordering between versions is not defined.
    NormalizerVersion,
    /// Instrument-definition version pinned by a price or quantity.
    ///
    /// Owner: reference data. This is not [`SchemaVersion`].
    InstrumentDefinitionVersion,
    /// Valuation-model version.
    ///
    /// Owner: data and risk. Valuation is not implemented here.
    ValuationModelVersion,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{
        InstrumentDefinitionVersion, NormalizerVersion, OrderVersion, RiskPolicyVersion,
        SchemaVersion, ValuationModelVersion, WriterEpoch,
    };
    use crate::error::VersionOverflow;

    #[test]
    fn counters_advance_until_the_last_value_and_do_not_wrap() {
        assert_eq!(OrderVersion::new(0).checked_next().unwrap().get(), 1);
        assert_eq!(
            OrderVersion::new(u64::MAX).checked_next(),
            Err(VersionOverflow::OrderVersion)
        );
        assert_eq!(OrderVersion::new(u64::MAX).get(), u64::MAX);
        assert_eq!(
            WriterEpoch::new(u64::MAX).checked_next(),
            Err(VersionOverflow::WriterEpoch)
        );
        assert!(OrderVersion::new(2) > OrderVersion::new(1));
    }

    #[test]
    fn numeric_versions_reject_non_canonical_integers() {
        assert_eq!(OrderVersion::from_str("0").unwrap().get(), 0);
        assert_eq!(
            OrderVersion::from_str(&u64::MAX.to_string()).unwrap().get(),
            u64::MAX
        );
        for text in ["01", "-1", "+1", "1.0", "1e2", "", " 1"] {
            assert!(OrderVersion::from_str(text).is_err(), "{text}");
            assert!(WriterEpoch::from_str(text).is_err(), "{text}");
            assert!(SchemaVersion::from_str(text).is_err(), "{text}");
        }
        assert!(serde_json::from_str::<OrderVersion>("1.0").is_err());
        assert!(serde_json::from_str::<OrderVersion>("-1").is_err());
        assert!(serde_json::from_str::<SchemaVersion>("\"1\"").is_err());
        assert_eq!(serde_json::from_str::<SchemaVersion>("1").unwrap().get(), 1);
        let encoded = serde_json::to_string(&WriterEpoch::new(7)).unwrap();
        assert_eq!(encoded, "7");
    }

    #[test]
    fn opaque_versions_do_not_normalize_text() {
        let policy = RiskPolicyVersion::new("v1").unwrap();
        let normalizer = NormalizerVersion::new("v1").unwrap();
        assert_eq!(policy.as_str(), normalizer.as_str());
        assert_ne!(
            RiskPolicyVersion::new("v1").unwrap(),
            RiskPolicyVersion::new("V1").unwrap()
        );
        assert!(InstrumentDefinitionVersion::new("").is_err());
        assert!(InstrumentDefinitionVersion::new("a b").is_err());
        assert!(ValuationModelVersion::new("ok").is_ok());
        assert!(serde_json::from_str::<RiskPolicyVersion>("1").is_err());
        let json = serde_json::to_string(&policy).unwrap();
        assert_eq!(json, "\"v1\"");
        assert!(serde_json::from_str::<NormalizerVersion>(&json).is_ok());
    }
}
