//! Separate UTC timestamp shells.
//!
//! [`OccurredAt`] is a source-event time, [`ReceivedAt`] is an ingestion time,
//! and [`PersistedAt`] is a database commit time. They do not convert into
//! each other. Ordering inside one type orders the stored microseconds only.
//! It does not order domain events, and it does not order broker messages.
//!
//! The storage resolution is microseconds since the Unix epoch. Nanoseconds
//! are rejected even when they divide evenly by 1_000, so this crate does not
//! truncate. RFC 3339 parsing is not implemented. Zero is an instant, not a
//! missing timestamp; absence belongs in `Option`.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::error::{IntegerParseError, TimeError, TimeGap};
use crate::parse::parse_canonical_i64;

macro_rules! instant {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[must_use]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(i64);

        impl $name {
            /// Stores microseconds since the Unix epoch.
            ///
            /// The full `i64` range is accepted. Zero is `1970-01-01T00:00:00Z`
            /// at this resolution, not a missing value.
            pub const fn from_unix_microseconds(value: i64) -> Self {
                Self(value)
            }

            /// Returns the stored microseconds.
            pub const fn unix_microseconds(self) -> i64 {
                self.0
            }

            /// Nanosecond conversion is not implemented.
            ///
            /// Exact multiples of 1_000 are also rejected. This function does
            /// not truncate.
            pub const fn from_unix_nanoseconds(_value: i128) -> Result<Self, TimeError> {
                Err(TimeError::NotImplemented(TimeGap::NanosecondConversion))
            }

            /// RFC 3339 parsing is not implemented.
            pub fn from_rfc3339(_text: &str) -> Result<Self, TimeError> {
                Err(TimeError::NotImplemented(TimeGap::Rfc3339))
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
                parse_canonical_i64(text).map(Self)
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(&self.0.to_string())
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
    };
}

instant! {
    /// Source-event time in UTC microseconds.
    ///
    /// Owner: shared. Missing source time is `Option<OccurredAt>`, not a
    /// sentinel inside this type. Clock quality is not claimed.
    OccurredAt
}

instant! {
    /// Ingestion time in UTC microseconds.
    ///
    /// Owner: shared. Distinct from [`OccurredAt`] and [`PersistedAt`].
    ReceivedAt
}

instant! {
    /// Database commit time in UTC microseconds.
    ///
    /// Owner: shared. Distinct from [`OccurredAt`] and [`ReceivedAt`].
    /// Recording the commit is not implemented here.
    PersistedAt
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{OccurredAt, PersistedAt, ReceivedAt};
    use crate::error::{TimeError, TimeGap};

    #[test]
    fn microseconds_round_trip_and_zero_is_present() {
        for value in [0, -1, 1, i64::MIN, i64::MAX] {
            let occurred = OccurredAt::from_unix_microseconds(value);
            assert_eq!(occurred.unix_microseconds(), value);
            assert_eq!(
                OccurredAt::from_str(&occurred.to_string()).unwrap(),
                occurred
            );
            let json = serde_json::to_string(&occurred).unwrap();
            assert!(json.starts_with('"'));
            let decoded: OccurredAt = serde_json::from_str(&json).unwrap();
            assert_eq!(decoded, occurred);
        }
        assert_eq!(OccurredAt::from_unix_microseconds(0).unix_microseconds(), 0);
    }

    #[test]
    fn non_canonical_and_finer_clocks_are_rejected() {
        for text in ["01", "-0", "+1", "1.0", "1e2", "", "2026-11-14T12:00:00Z"] {
            assert!(OccurredAt::from_str(text).is_err(), "{text}");
            assert!(ReceivedAt::from_str(text).is_err(), "{text}");
            assert!(PersistedAt::from_str(text).is_err(), "{text}");
        }
        assert!(serde_json::from_str::<ReceivedAt>("0").is_err());
        assert!(serde_json::from_str::<ReceivedAt>("\"1.5\"").is_err());
        assert_eq!(
            OccurredAt::from_unix_nanoseconds(1_000),
            Err(TimeError::NotImplemented(TimeGap::NanosecondConversion))
        );
        assert_eq!(
            PersistedAt::from_rfc3339("2026-11-14T12:00:00Z"),
            Err(TimeError::NotImplemented(TimeGap::Rfc3339))
        );
    }
}
