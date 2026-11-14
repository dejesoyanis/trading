//! Separate UTC timestamp shells.
//!
//! [`OccurredAt`] is a source-event time, [`ReceivedAt`] is an ingestion time,
//! and [`PersistedAt`] is a database commit time. They do not convert into
//! each other. Ordering inside one type orders the stored microseconds only.
//! It does not order domain events, and it does not order broker messages.
//!
//! The storage resolution is microseconds since the Unix epoch. Nanoseconds
//! are rejected even when they divide evenly by 1_000, so this crate does not
//! truncate. [`OccurredAt::from_rfc3339`] is an opt-in codec. It is not
//! [`std::str::FromStr`] and it is not the draft JSON shape, which remains a
//! decimal integer string. Zero is an instant, not a missing timestamp;
//! absence belongs in `Option`.
//!
//! `ClockQuality` and `TimestampProvenance` are not stored on these types.
//! The resolution names that metadata and does not list variants, so this
//! crate does not invent a vocabulary or infer one from the timestamp.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::error::{IntegerParseError, TimeError, TimeGap};
use crate::parse::parse_canonical_i64;

const MICROS_PER_SECOND: i64 = 1_000_000;
const MICROS_PER_MINUTE: i64 = 60 * MICROS_PER_SECOND;
const MICROS_PER_HOUR: i64 = 60 * MICROS_PER_MINUTE;
const MICROS_PER_DAY: i64 = 24 * MICROS_PER_HOUR;
const MAX_RFC3339_LEN: usize = 64;
const MAX_FRACTION_DIGITS: usize = 12;

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

            /// Parses an RFC 3339 timestamp into Unix microseconds.
            ///
            /// The offset is required. `Z` and `±HH:MM` are accepted and
            /// normalized to UTC. Fractional seconds may be omitted or may
            /// contain up to six digits. Extra digits are accepted only when
            /// every one of them is `0`. A nonzero digit after the microsecond
            /// is [`TimeError::SubMicrosecondPrecision`] and is not truncated.
            ///
            /// This codec is opt-in. [`FromStr`], [`Display`], and serde still
            /// use the decimal microsecond string.
            pub fn from_rfc3339(text: &str) -> Result<Self, TimeError> {
                parse_rfc3339_micros(text).map(Self)
            }

            /// Formats this instant as UTC RFC 3339 with exactly six fractional digits.
            ///
            /// The result always ends in `Z`. It is not the draft JSON encoding.
            #[must_use]
            pub fn to_rfc3339_utc(self) -> String {
                format_rfc3339_micros(self.0)
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
    /// The persistence boundary does not assign this value, and the value is
    /// not proof of an external broker order.
    PersistedAt
}

fn parse_rfc3339_micros(text: &str) -> Result<i64, TimeError> {
    if text.is_empty()
        || text.len() > MAX_RFC3339_LEN
        || text.as_bytes().iter().any(u8::is_ascii_whitespace)
    {
        return Err(TimeError::Malformed);
    }
    let (year, rest) = parse_year(text)?;
    let rest = expect_byte(rest, b'-')?;
    let (month, rest) = parse_fixed_u32(rest, 2)?;
    let rest = expect_byte(rest, b'-')?;
    let (day, rest) = parse_fixed_u32(rest, 2)?;
    let rest = expect_byte(rest, b'T')?;
    let (hour, rest) = parse_fixed_u32(rest, 2)?;
    let rest = expect_byte(rest, b':')?;
    let (minute, rest) = parse_fixed_u32(rest, 2)?;
    let rest = expect_byte(rest, b':')?;
    let (second, rest) = parse_fixed_u32(rest, 2)?;
    if hour > 23 || minute > 59 || second > 59 {
        return Err(TimeError::Malformed);
    }
    let (micro, rest) = parse_fraction(rest)?;
    let offset = parse_offset(rest)?;
    let days = civil_days(year, month, day)?;
    let time_us = i64::from(hour) * MICROS_PER_HOUR
        + i64::from(minute) * MICROS_PER_MINUTE
        + i64::from(second) * MICROS_PER_SECOND
        + i64::from(micro);
    compose_micros(days, time_us, offset)
}

/// Combines a civil day count with a time of day.
///
/// The day product is computed in `i128` because `i64::MIN`'s Euclidean day
/// quotient times the length of a day is below `i64::MIN`, while the sum with
/// the positive remainder is a representable microsecond. The narrowing is
/// checked. It does not truncate.
fn compose_micros(days: i64, time_us: i64, offset_us: i64) -> Result<i64, TimeError> {
    let wide =
        i128::from(days) * i128::from(MICROS_PER_DAY) + i128::from(time_us) - i128::from(offset_us);
    i64::try_from(wide).map_err(|_| TimeError::Overflow)
}

fn format_rfc3339_micros(micros: i64) -> String {
    let days = micros.div_euclid(MICROS_PER_DAY);
    let within = micros.rem_euclid(MICROS_PER_DAY);
    let seconds = within / MICROS_PER_SECOND;
    let micro = u32::try_from(within % MICROS_PER_SECOND).expect("microseconds fit u32");
    let hour = u32::try_from(seconds / 3600).expect("hour fits u32");
    let minute = u32::try_from((seconds % 3600) / 60).expect("minute fits u32");
    let second = u32::try_from(seconds % 60).expect("second fits u32");
    let (year, month, day) = civil_from_days(days).expect("unix microsecond day has a civil date");
    format!(
        "{}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}.{micro:06}Z",
        format_year(year)
    )
}

fn format_year(year: i32) -> String {
    if (0..=9999).contains(&year) {
        format!("{year:04}")
    } else if year > 0 {
        format!("+{year}")
    } else {
        format!("-{:04}", year.unsigned_abs())
    }
}

fn parse_year(text: &str) -> Result<(i32, &str), TimeError> {
    let Some(first) = text.as_bytes().first().copied() else {
        return Err(TimeError::Malformed);
    };
    match first {
        b'0'..=b'9' => {
            let (value, rest) = parse_fixed_u32(text, 4)?;
            Ok((i32::try_from(value).map_err(|_| TimeError::Overflow)?, rest))
        }
        b'+' | b'-' => {
            let rest = &text[1..];
            let digit_len = rest.bytes().take_while(u8::is_ascii_digit).count();
            if !(4..=7).contains(&digit_len) {
                return Err(TimeError::Malformed);
            }
            let (digits, tail) = rest.split_at(digit_len);
            let magnitude: i32 = digits.parse().map_err(|_| TimeError::Overflow)?;
            let year = if first == b'-' {
                magnitude.checked_neg().ok_or(TimeError::Overflow)?
            } else {
                magnitude
            };
            Ok((year, tail))
        }
        _ => Err(TimeError::Malformed),
    }
}

fn parse_fraction(text: &str) -> Result<(u32, &str), TimeError> {
    let Some(rest) = text.strip_prefix('.') else {
        return Ok((0, text));
    };
    let digit_len = rest.bytes().take_while(u8::is_ascii_digit).count();
    if digit_len == 0 || digit_len > MAX_FRACTION_DIGITS {
        return Err(TimeError::Malformed);
    }
    let (digits, tail) = rest.split_at(digit_len);
    let digits = digits.as_bytes();
    if digits[6.min(digit_len)..].iter().any(|byte| *byte != b'0') {
        return Err(TimeError::SubMicrosecondPrecision);
    }
    let mut micro = 0_u32;
    let significant = 6.min(digit_len);
    for (index, byte) in digits[..significant].iter().enumerate() {
        let digit = u32::from(byte - b'0');
        let places = 5 - index;
        micro += digit * 10_u32.pow(u32::try_from(places).expect("place fits u32"));
    }
    Ok((micro, tail))
}

fn parse_offset(text: &str) -> Result<i64, TimeError> {
    if text == "Z" {
        return Ok(0);
    }
    let Some(sign) = text.as_bytes().first().copied() else {
        return Err(TimeError::Malformed);
    };
    let signed = match sign {
        b'+' => 1_i64,
        b'-' => -1,
        _ => return Err(TimeError::Malformed),
    };
    let rest = &text[1..];
    let (hour, rest) = parse_fixed_u32(rest, 2)?;
    let rest = expect_byte(rest, b':')?;
    let (minute, rest) = parse_fixed_u32(rest, 2)?;
    if !rest.is_empty() || hour > 23 || minute > 59 {
        return Err(TimeError::Malformed);
    }
    let magnitude = i64::from(hour) * MICROS_PER_HOUR + i64::from(minute) * MICROS_PER_MINUTE;
    Ok(signed * magnitude)
}

fn parse_fixed_u32(text: &str, width: usize) -> Result<(u32, &str), TimeError> {
    let Some((head, tail)) = text.split_at_checked(width) else {
        return Err(TimeError::Malformed);
    };
    if !head.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(TimeError::Malformed);
    }
    let value = head.parse::<u32>().map_err(|_| TimeError::Malformed)?;
    Ok((value, tail))
}

fn expect_byte(text: &str, expected: u8) -> Result<&str, TimeError> {
    let Some(rest) = text.strip_prefix(expected as char) else {
        return Err(TimeError::Malformed);
    };
    Ok(rest)
}

fn is_leap(year: i32) -> bool {
    let year = i64::from(year);
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn days_in_month(year: i32, month: u32) -> Option<u32> {
    const DAYS: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let base = *DAYS.get(usize::try_from(month.checked_sub(1)?).ok()?)?;
    if month == 2 && is_leap(year) {
        Some(29)
    } else {
        Some(base)
    }
}

fn civil_days(year: i32, month: u32, day: u32) -> Result<i64, TimeError> {
    let allowed = days_in_month(year, month).ok_or(TimeError::InvalidDate)?;
    if day == 0 || day > allowed {
        return Err(TimeError::InvalidDate);
    }
    let days = days_from_civil(year, month, day).ok_or(TimeError::Overflow)?;
    match civil_from_days(days) {
        Some((got_year, got_month, got_day))
            if got_year == year && got_month == month && got_day == day =>
        {
            Ok(days)
        }
        Some(_) => Err(TimeError::InvalidDate),
        None => Err(TimeError::Overflow),
    }
}

fn days_from_civil(year: i32, month: u32, day: u32) -> Option<i64> {
    let mut year = i64::from(year);
    if month <= 2 {
        year = year.checked_sub(1)?;
    }
    let era = if year >= 0 {
        year / 400
    } else {
        year.checked_sub(399)? / 400
    };
    let yoe = u32::try_from(year.checked_sub(era.checked_mul(400)?)?).ok()?;
    let month_prime = if month > 2 { month - 3 } else { month + 9 };
    let doy = (153 * month_prime + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era.checked_mul(146_097)?
        .checked_add(i64::from(doe))?
        .checked_sub(719_468)
}

fn civil_from_days(mut days: i64) -> Option<(i32, u32, u32)> {
    days = days.checked_add(719_468)?;
    let era = if days >= 0 {
        days / 146_097
    } else {
        days.checked_sub(146_096)? / 146_097
    };
    let doe = u32::try_from(days.checked_sub(era.checked_mul(146_097)?)?).ok()?;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let mut year = i64::from(yoe).checked_add(era.checked_mul(400)?)?;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let month_prime = (5 * doy + 2) / 153;
    let day = doy - (153 * month_prime + 2) / 5 + 1;
    let month = if month_prime < 10 {
        month_prime + 3
    } else {
        month_prime - 9
    };
    if month <= 2 {
        year = year.checked_add(1)?;
    }
    let year = i32::try_from(year).ok()?;
    Some((year, month, day))
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
        let missing: Option<OccurredAt> = None;
        assert_ne!(missing, Some(OccurredAt::from_unix_microseconds(0)));
        assert_eq!(std::mem::size_of::<OccurredAt>(), 8);
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
        assert!(serde_json::from_str::<OccurredAt>("\"1970-01-01T00:00:00.000000Z\"").is_err());
        assert_eq!(
            OccurredAt::from_unix_nanoseconds(1_000),
            Err(TimeError::NotImplemented(TimeGap::NanosecondConversion))
        );
        assert_eq!(
            OccurredAt::from_unix_nanoseconds(1),
            Err(TimeError::NotImplemented(TimeGap::NanosecondConversion))
        );
    }

    #[test]
    fn rfc3339_utc_codec_round_trips_and_rejects_inexact_fractions() {
        let epoch = OccurredAt::from_rfc3339("1970-01-01T00:00:00.000000Z").unwrap();
        assert_eq!(epoch.unix_microseconds(), 0);
        assert_eq!(epoch.to_rfc3339_utc(), "1970-01-01T00:00:00.000000Z");
        assert_eq!(epoch.to_string(), "0");

        let before = OccurredAt::from_rfc3339("1969-12-31T23:59:59.999999Z").unwrap();
        assert_eq!(before.unix_microseconds(), -1);
        assert_eq!(before.to_rfc3339_utc(), "1969-12-31T23:59:59.999999Z");
        assert!(before < epoch);

        let one_micro = OccurredAt::from_rfc3339("1970-01-01T00:00:00.000001Z").unwrap();
        assert_eq!(one_micro.unix_microseconds(), 1);
        let tenth = OccurredAt::from_rfc3339("1970-01-01T00:00:00.1Z").unwrap();
        assert_eq!(tenth.unix_microseconds(), 100_000);
        let padded = OccurredAt::from_rfc3339("1970-01-01T00:00:00.123456000Z").unwrap();
        assert_eq!(padded.unix_microseconds(), 123_456);
        assert_eq!(
            OccurredAt::from_rfc3339("1970-01-01T00:00:00.1234567Z"),
            Err(TimeError::SubMicrosecondPrecision)
        );
        assert_eq!(
            OccurredAt::from_rfc3339("1970-01-01T00:00:00.0000001Z"),
            Err(TimeError::SubMicrosecondPrecision)
        );

        let from_offset = OccurredAt::from_rfc3339("1970-01-01T00:30:00.000000+01:00").unwrap();
        assert_eq!(
            from_offset,
            OccurredAt::from_rfc3339("1969-12-31T23:30:00.000000Z").unwrap()
        );
        assert_eq!(from_offset.to_rfc3339_utc(), "1969-12-31T23:30:00.000000Z");
        let across = OccurredAt::from_rfc3339("1969-12-31T23:30:00.000000-01:00").unwrap();
        assert_eq!(across.to_rfc3339_utc(), "1970-01-01T00:30:00.000000Z");
        assert_eq!(
            OccurredAt::from_rfc3339("1970-01-01T01:00:00.000000+01:00")
                .unwrap()
                .unix_microseconds(),
            0
        );

        let leap = OccurredAt::from_rfc3339("2024-02-29T12:00:00.000000Z").unwrap();
        assert_eq!(leap.to_rfc3339_utc(), "2024-02-29T12:00:00.000000Z");
        assert_eq!(
            OccurredAt::from_rfc3339("2000-02-29T00:00:00Z")
                .unwrap()
                .to_rfc3339_utc(),
            "2000-02-29T00:00:00.000000Z"
        );
        assert_eq!(
            OccurredAt::from_rfc3339("1900-02-29T00:00:00Z"),
            Err(TimeError::InvalidDate)
        );
        assert_eq!(
            OccurredAt::from_rfc3339("2023-02-29T00:00:00Z"),
            Err(TimeError::InvalidDate)
        );
        assert_eq!(
            OccurredAt::from_rfc3339("2024-02-30T00:00:00Z"),
            Err(TimeError::InvalidDate)
        );
        assert_eq!(
            OccurredAt::from_rfc3339("2024-13-01T00:00:00Z"),
            Err(TimeError::InvalidDate)
        );
        assert_eq!(
            OccurredAt::from_rfc3339("2024-04-31T00:00:00.000000Z"),
            Err(TimeError::InvalidDate)
        );
        assert_eq!(
            OccurredAt::from_rfc3339("2024-00-01T00:00:00Z"),
            Err(TimeError::InvalidDate)
        );

        for text in [
            "",
            "1970-01-01T00:00:00.000000",
            "1970-01-01 00:00:00.000000Z",
            "1970-01-01T00:00:00.000000z",
            "1970-01-01T00:00:00.000000+0100",
            "1970-01-01T24:00:00.000000Z",
            "1970-01-01T00:00:60.000000Z",
            "not-a-timestamp",
        ] {
            assert!(
                matches!(OccurredAt::from_rfc3339(text), Err(TimeError::Malformed)),
                "{text}"
            );
        }

        for value in [0, -1, 1, -3_600_000_000, 86_400_000_000, i64::MIN, i64::MAX] {
            let occurred = OccurredAt::from_unix_microseconds(value);
            let received = ReceivedAt::from_unix_microseconds(value);
            let persisted = PersistedAt::from_unix_microseconds(value);
            assert_eq!(
                OccurredAt::from_rfc3339(&occurred.to_rfc3339_utc()).unwrap(),
                occurred
            );
            assert_eq!(
                ReceivedAt::from_rfc3339(&received.to_rfc3339_utc())
                    .unwrap()
                    .unix_microseconds(),
                value
            );
            assert_eq!(
                PersistedAt::from_rfc3339(&persisted.to_rfc3339_utc())
                    .unwrap()
                    .unix_microseconds(),
                value
            );
        }

        let max_text = OccurredAt::from_unix_microseconds(i64::MAX).to_rfc3339_utc();
        let shifted_max = format!("{}-00:01", max_text.trim_end_matches('Z'));
        assert_eq!(
            OccurredAt::from_rfc3339(&shifted_max),
            Err(TimeError::Overflow)
        );
        let min_text = OccurredAt::from_unix_microseconds(i64::MIN).to_rfc3339_utc();
        let shifted_min = format!("{}+00:01", min_text.trim_end_matches('Z'));
        assert_eq!(
            OccurredAt::from_rfc3339(&shifted_min),
            Err(TimeError::Overflow)
        );
        assert_eq!(
            OccurredAt::from_rfc3339("+999999-01-01T00:00:00.000000Z"),
            Err(TimeError::Overflow)
        );
    }
}
