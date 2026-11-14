//! Canonical parsers for identifier and numeric shells.
//!
//! Accepted text round-trips through `Display`. Alternate spellings fail.

use uuid::Uuid;

use crate::error::{IdParseError, IntegerParseError, TokenError};

const I128_MIN_ABS: &str = "170141183460469231731687303715884105728";

#[derive(Clone, Copy)]
pub(crate) enum TokenClass {
    UpperAscii,
    PrintableAscii,
}

pub(crate) fn parse_token(
    value: &str,
    minimum: usize,
    maximum: usize,
    class: TokenClass,
) -> Result<String, TokenError> {
    if value.is_empty() {
        return Err(TokenError::Empty);
    }
    if value.len() < minimum {
        return Err(TokenError::TooShort);
    }
    if value.len() > maximum {
        return Err(TokenError::TooLong);
    }
    let valid = match class {
        TokenClass::UpperAscii => value.bytes().all(|byte| byte.is_ascii_uppercase()),
        TokenClass::PrintableAscii => value.bytes().all(|byte| (0x21..=0x7E).contains(&byte)),
    };
    if !valid {
        return Err(TokenError::InvalidCharacter);
    }
    Ok(value.to_owned())
}

pub(crate) fn parse_canonical_uuid(
    text: &str,
    type_name: &'static str,
) -> Result<Uuid, IdParseError> {
    if text.len() != 36 {
        return Err(IdParseError::new(type_name));
    }
    let Ok(value) = Uuid::parse_str(text) else {
        return Err(IdParseError::new(type_name));
    };
    if value.hyphenated().to_string() != text {
        return Err(IdParseError::new(type_name));
    }
    Ok(value)
}

pub(crate) fn parse_canonical_i128(text: &str) -> Result<i128, IntegerParseError> {
    let (negative, digits) = match text.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, text),
    };
    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(IntegerParseError);
    }
    if digits.len() > 1 && digits.starts_with('0') {
        return Err(IntegerParseError);
    }
    if negative && digits == "0" {
        return Err(IntegerParseError);
    }
    if negative && digits == I128_MIN_ABS {
        return Ok(i128::MIN);
    }

    let mut magnitude: i128 = 0;
    for byte in digits.bytes() {
        let digit = i128::from(byte - b'0');
        magnitude = magnitude
            .checked_mul(10)
            .and_then(|value| value.checked_add(digit))
            .ok_or(IntegerParseError)?;
    }
    if negative {
        magnitude.checked_neg().ok_or(IntegerParseError)
    } else {
        Ok(magnitude)
    }
}

pub(crate) fn parse_canonical_i64(text: &str) -> Result<i64, IntegerParseError> {
    let value = parse_canonical_i128(text)?;
    i64::try_from(value).map_err(|_| IntegerParseError)
}

pub(crate) fn parse_canonical_u64(text: &str) -> Result<u64, IntegerParseError> {
    if text.starts_with('-') {
        return Err(IntegerParseError);
    }
    let value = parse_canonical_i128(text)?;
    u64::try_from(value).map_err(|_| IntegerParseError)
}
