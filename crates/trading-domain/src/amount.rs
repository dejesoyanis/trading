//! Decimal, money, price, and quantity shells.
//!
//! Authoritative amounts use an `i128` coefficient and an explicit scale.
//! There is no `f32` or `f64` field. Differing currency, instrument, definition
//! version, or scale is rejected. Rescaling, rounding, and foreign exchange
//! return [`DecimalError::NotImplemented`] instead of an invented result.
//!
//! Equal mathematical values with different scales are not equal: `1.0` stored
//! as coefficient `10` scale `1` is not coefficient `100` scale `2`.
//!
//! The JSON object below is a draft shell. It is not the M0-02B canonical
//! envelope. Coefficients and scales are strings so a JSON number cannot
//! arrive as a float.

use std::fmt;
use std::str::FromStr;

use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::error::{DecimalError, FinancialGap, MoneyError, PriceError, QuantityError, TokenError};
use crate::parse::{TokenClass, parse_canonical_i128, parse_canonical_u64, parse_token};
use crate::version::InstrumentDefinitionVersion;

/// Largest scale that still fits in the decimal digit budget of `i128`.
///
/// This is a storage bound, not a tick size or a market rounding rule.
pub const MAX_DECIMAL_SCALE: u8 = 38;

const CURRENCY_MAX: usize = 16;
const INSTRUMENT_MAX: usize = 64;

/// Signed decimal coefficient with an explicit scale.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DecimalAmount {
    coefficient: i128,
    scale: u8,
}

impl DecimalAmount {
    /// Builds a decimal when `scale` is within [`MAX_DECIMAL_SCALE`].
    pub const fn try_new(coefficient: i128, scale: u8) -> Result<Self, DecimalError> {
        if scale > MAX_DECIMAL_SCALE {
            return Err(DecimalError::ScaleAboveBound);
        }
        Ok(Self { coefficient, scale })
    }

    /// Returns the signed coefficient.
    pub const fn coefficient(self) -> i128 {
        self.coefficient
    }

    /// Returns the number of decimal digits represented by the coefficient.
    pub const fn scale(self) -> u8 {
        self.scale
    }

    /// Adds coefficients when the scales are identical.
    pub const fn checked_add(self, other: Self) -> Result<Self, DecimalError> {
        if self.scale != other.scale {
            return Err(DecimalError::ScaleMismatch);
        }
        match self.coefficient.checked_add(other.coefficient) {
            Some(coefficient) => Ok(Self {
                coefficient,
                scale: self.scale,
            }),
            None => Err(DecimalError::Overflow),
        }
    }

    /// Subtracts coefficients when the scales are identical.
    pub const fn checked_sub(self, other: Self) -> Result<Self, DecimalError> {
        if self.scale != other.scale {
            return Err(DecimalError::ScaleMismatch);
        }
        match self.coefficient.checked_sub(other.coefficient) {
            Some(coefficient) => Ok(Self {
                coefficient,
                scale: self.scale,
            }),
            None => Err(DecimalError::Overflow),
        }
    }

    /// Negates the coefficient, including the overflow of `i128::MIN`.
    pub const fn checked_neg(self) -> Result<Self, DecimalError> {
        match self.coefficient.checked_neg() {
            Some(coefficient) => Ok(Self {
                coefficient,
                scale: self.scale,
            }),
            None => Err(DecimalError::Overflow),
        }
    }

    /// Scale changes are not implemented, even when the scale is unchanged.
    pub const fn rescale(self, _target_scale: u8) -> Result<Self, DecimalError> {
        Err(DecimalError::NotImplemented(FinancialGap::Rescale))
    }
}

impl Serialize for DecimalAmount {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("coefficient", &self.coefficient.to_string())?;
        map.serialize_entry("scale", &self.scale.to_string())?;
        map.end()
    }
}

impl<'de> Deserialize<'de> for DecimalAmount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Body {
            coefficient: String,
            scale: String,
        }

        let body = Body::deserialize(deserializer)?;
        decimal_from_text(&body.coefficient, &body.scale).map_err(serde::de::Error::custom)
    }
}

fn decimal_from_text(coefficient: &str, scale: &str) -> Result<DecimalAmount, DecimalError> {
    let coefficient =
        parse_canonical_i128(coefficient).map_err(|_| DecimalError::InvalidCoefficient)?;
    let scale = parse_canonical_u64(scale).map_err(|_| DecimalError::InvalidScale)?;
    let scale = u8::try_from(scale).map_err(|_| DecimalError::ScaleAboveBound)?;
    DecimalAmount::try_new(coefficient, scale)
}

/// Currency code.
///
/// Owner: shared value type. Uppercase ASCII, 1 to 16 letters. This is not an
/// ISO 4217 registry check. Lowercase is rejected rather than folded.
#[must_use]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct CurrencyCode {
    bytes: [u8; CURRENCY_MAX],
    len: u8,
}

impl CurrencyCode {
    /// Validates and stores a currency code.
    pub fn new(value: &str) -> Result<Self, TokenError> {
        let token = parse_token(value, 1, CURRENCY_MAX, TokenClass::UpperAscii)?;
        let mut bytes = [0_u8; CURRENCY_MAX];
        bytes[..token.len()].copy_from_slice(token.as_bytes());
        Ok(Self {
            bytes,
            len: u8::try_from(token.len()).expect("currency length fits in u8"),
        })
    }

    /// Returns the code.
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.bytes[..usize::from(self.len)])
            .expect("currency code is uppercase ascii")
    }
}

impl fmt::Display for CurrencyCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl fmt::Debug for CurrencyCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("CurrencyCode")
            .field(&self.as_str())
            .finish()
    }
}

impl FromStr for CurrencyCode {
    type Err = TokenError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::new(text)
    }
}

/// Instrument identifier.
///
/// Owner: reference data. Printable ASCII, 1 to 64 bytes, with no case
/// folding. This is not a symbology check and it is not [`CurrencyCode`].
#[must_use]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InstrumentId(String);

impl InstrumentId {
    /// Validates and stores an instrument id.
    pub fn new(value: &str) -> Result<Self, TokenError> {
        parse_token(value, 1, INSTRUMENT_MAX, TokenClass::PrintableAscii).map(Self)
    }

    /// Returns the instrument id.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for InstrumentId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for InstrumentId {
    type Err = TokenError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::new(text)
    }
}

/// Currency-scoped amount.
///
/// Owner: shared value type. This is not a [`Price`]: it has no instrument.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Money {
    amount: DecimalAmount,
    currency: CurrencyCode,
}

impl Money {
    /// Pairs an amount with a currency.
    pub const fn new(amount: DecimalAmount, currency: CurrencyCode) -> Self {
        Self { amount, currency }
    }

    /// Returns the decimal amount.
    pub const fn amount(self) -> DecimalAmount {
        self.amount
    }

    /// Returns the currency.
    pub const fn currency(self) -> CurrencyCode {
        self.currency
    }

    /// Adds amounts of the same currency and scale.
    pub fn checked_add(self, other: Self) -> Result<Self, MoneyError> {
        if self.currency != other.currency {
            return Err(MoneyError::CurrencyMismatch);
        }
        match self.amount.checked_add(other.amount) {
            Ok(amount) => Ok(Self {
                amount,
                currency: self.currency,
            }),
            Err(error) => Err(MoneyError::Decimal(error)),
        }
    }

    /// Subtracts amounts of the same currency and scale.
    pub fn checked_sub(self, other: Self) -> Result<Self, MoneyError> {
        if self.currency != other.currency {
            return Err(MoneyError::CurrencyMismatch);
        }
        match self.amount.checked_sub(other.amount) {
            Ok(amount) => Ok(Self {
                amount,
                currency: self.currency,
            }),
            Err(error) => Err(MoneyError::Decimal(error)),
        }
    }

    /// Foreign exchange is not implemented, including a same-currency call.
    pub const fn convert(self, _target: CurrencyCode) -> Result<Self, MoneyError> {
        Err(MoneyError::NotImplemented(FinancialGap::CurrencyConversion))
    }
}

impl Serialize for Money {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("coefficient", &self.amount.coefficient.to_string())?;
        map.serialize_entry("scale", &self.amount.scale.to_string())?;
        map.serialize_entry("currency", self.currency.as_str())?;
        map.end()
    }
}

impl<'de> Deserialize<'de> for Money {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Body {
            coefficient: String,
            scale: String,
            currency: String,
        }

        let body = Body::deserialize(deserializer)?;
        let amount =
            decimal_from_text(&body.coefficient, &body.scale).map_err(serde::de::Error::custom)?;
        let currency = CurrencyCode::new(&body.currency).map_err(serde::de::Error::custom)?;
        Ok(Self::new(amount, currency))
    }
}

/// Instrument-scoped quote.
///
/// Owner: shared value type, with reference-data identity. Addition requires
/// the same quote currency, instrument, definition version, and scale.
#[must_use]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Price {
    amount: DecimalAmount,
    quote_currency: CurrencyCode,
    instrument_id: InstrumentId,
    definition_version: InstrumentDefinitionVersion,
}

impl Price {
    /// Pairs a quote with its instrument definition.
    pub const fn new(
        amount: DecimalAmount,
        quote_currency: CurrencyCode,
        instrument_id: InstrumentId,
        definition_version: InstrumentDefinitionVersion,
    ) -> Self {
        Self {
            amount,
            quote_currency,
            instrument_id,
            definition_version,
        }
    }

    /// Returns the decimal amount.
    pub const fn amount(&self) -> DecimalAmount {
        self.amount
    }

    /// Returns the quote currency.
    pub const fn quote_currency(&self) -> CurrencyCode {
        self.quote_currency
    }

    /// Returns the instrument id.
    pub fn instrument_id(&self) -> &InstrumentId {
        &self.instrument_id
    }

    /// Returns the pinned instrument-definition version.
    pub fn definition_version(&self) -> &InstrumentDefinitionVersion {
        &self.definition_version
    }

    /// Adds quotes that share currency, instrument, definition version, and scale.
    ///
    /// Mismatches are reported in that order.
    pub fn checked_add(&self, other: &Self) -> Result<Self, PriceError> {
        if self.quote_currency != other.quote_currency {
            return Err(PriceError::CurrencyMismatch);
        }
        if self.instrument_id != other.instrument_id {
            return Err(PriceError::InstrumentMismatch);
        }
        if self.definition_version != other.definition_version {
            return Err(PriceError::DefinitionVersionMismatch);
        }
        let amount = self
            .amount
            .checked_add(other.amount)
            .map_err(PriceError::Decimal)?;
        Ok(Self {
            amount,
            quote_currency: self.quote_currency,
            instrument_id: self.instrument_id.clone(),
            definition_version: self.definition_version.clone(),
        })
    }

    /// Currency conversion is not implemented.
    pub fn convert_currency(self, _target: CurrencyCode) -> Result<Self, PriceError> {
        Err(PriceError::NotImplemented(FinancialGap::CurrencyConversion))
    }
}

impl Serialize for Price {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("coefficient", &self.amount.coefficient.to_string())?;
        map.serialize_entry("scale", &self.amount.scale.to_string())?;
        map.serialize_entry("quote_currency", self.quote_currency.as_str())?;
        map.serialize_entry("instrument_id", self.instrument_id.as_str())?;
        map.serialize_entry("definition_version", self.definition_version.as_str())?;
        map.end()
    }
}

impl<'de> Deserialize<'de> for Price {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Body {
            coefficient: String,
            scale: String,
            quote_currency: String,
            instrument_id: String,
            definition_version: String,
        }

        let body = Body::deserialize(deserializer)?;
        let amount =
            decimal_from_text(&body.coefficient, &body.scale).map_err(serde::de::Error::custom)?;
        let quote_currency =
            CurrencyCode::new(&body.quote_currency).map_err(serde::de::Error::custom)?;
        let instrument_id =
            InstrumentId::new(&body.instrument_id).map_err(serde::de::Error::custom)?;
        let definition_version = InstrumentDefinitionVersion::new(&body.definition_version)
            .map_err(serde::de::Error::custom)?;
        Ok(Self::new(
            amount,
            quote_currency,
            instrument_id,
            definition_version,
        ))
    }
}

/// Instrument-scoped quantity.
///
/// Owner: shared value type, with reference-data identity. Sign is not
/// restricted; no order side is implied.
#[must_use]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Quantity {
    amount: DecimalAmount,
    instrument_id: InstrumentId,
    definition_version: InstrumentDefinitionVersion,
}

impl Quantity {
    /// Pairs a quantity with its instrument definition.
    pub const fn new(
        amount: DecimalAmount,
        instrument_id: InstrumentId,
        definition_version: InstrumentDefinitionVersion,
    ) -> Self {
        Self {
            amount,
            instrument_id,
            definition_version,
        }
    }

    /// Returns the decimal amount.
    pub const fn amount(&self) -> DecimalAmount {
        self.amount
    }

    /// Returns the instrument id.
    pub fn instrument_id(&self) -> &InstrumentId {
        &self.instrument_id
    }

    /// Returns the pinned instrument-definition version.
    pub fn definition_version(&self) -> &InstrumentDefinitionVersion {
        &self.definition_version
    }

    /// Adds quantities that share instrument, definition version, and scale.
    pub fn checked_add(&self, other: &Self) -> Result<Self, QuantityError> {
        if self.instrument_id != other.instrument_id {
            return Err(QuantityError::InstrumentMismatch);
        }
        if self.definition_version != other.definition_version {
            return Err(QuantityError::DefinitionVersionMismatch);
        }
        let amount = self
            .amount
            .checked_add(other.amount)
            .map_err(QuantityError::Decimal)?;
        Ok(Self {
            amount,
            instrument_id: self.instrument_id.clone(),
            definition_version: self.definition_version.clone(),
        })
    }

    /// Instrument conversion is not implemented, even for the same instrument.
    pub fn convert_instrument(
        &self,
        _instrument_id: &InstrumentId,
        _definition_version: &InstrumentDefinitionVersion,
    ) -> Result<Self, QuantityError> {
        Err(QuantityError::NotImplemented(
            FinancialGap::InstrumentConversion,
        ))
    }
}

impl Serialize for Quantity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("coefficient", &self.amount.coefficient.to_string())?;
        map.serialize_entry("scale", &self.amount.scale.to_string())?;
        map.serialize_entry("instrument_id", self.instrument_id.as_str())?;
        map.serialize_entry("definition_version", self.definition_version.as_str())?;
        map.end()
    }
}

impl<'de> Deserialize<'de> for Quantity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Body {
            coefficient: String,
            scale: String,
            instrument_id: String,
            definition_version: String,
        }

        let body = Body::deserialize(deserializer)?;
        let amount =
            decimal_from_text(&body.coefficient, &body.scale).map_err(serde::de::Error::custom)?;
        let instrument_id =
            InstrumentId::new(&body.instrument_id).map_err(serde::de::Error::custom)?;
        let definition_version = InstrumentDefinitionVersion::new(&body.definition_version)
            .map_err(serde::de::Error::custom)?;
        Ok(Self::new(amount, instrument_id, definition_version))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CurrencyCode, DecimalAmount, InstrumentId, MAX_DECIMAL_SCALE, Money, Price, Quantity,
    };
    use crate::error::{DecimalError, FinancialGap, MoneyError, PriceError, QuantityError};
    use crate::version::InstrumentDefinitionVersion;

    fn usd(coefficient: i128, scale: u8) -> Money {
        Money::new(
            DecimalAmount::try_new(coefficient, scale).unwrap(),
            CurrencyCode::new("USD").unwrap(),
        )
    }

    fn version(text: &str) -> InstrumentDefinitionVersion {
        InstrumentDefinitionVersion::new(text).unwrap()
    }

    fn price(
        coefficient: i128,
        scale: u8,
        currency: &str,
        instrument: &str,
        definition: &str,
    ) -> Price {
        Price::new(
            DecimalAmount::try_new(coefficient, scale).unwrap(),
            CurrencyCode::new(currency).unwrap(),
            InstrumentId::new(instrument).unwrap(),
            version(definition),
        )
    }

    fn quantity(coefficient: i128, scale: u8, instrument: &str, definition: &str) -> Quantity {
        Quantity::new(
            DecimalAmount::try_new(coefficient, scale).unwrap(),
            InstrumentId::new(instrument).unwrap(),
            version(definition),
        )
    }

    #[test]
    fn coefficients_are_integers_and_scales_do_not_normalize() {
        let amount = DecimalAmount::try_new(-125, 2).unwrap();
        let coefficient: i128 = amount.coefficient();
        let scale: u8 = amount.scale();
        assert_eq!(coefficient, -125);
        assert_eq!(scale, 2);
        assert_ne!(
            DecimalAmount::try_new(10, 1).unwrap(),
            DecimalAmount::try_new(100, 2).unwrap()
        );
        assert_eq!(
            DecimalAmount::try_new(1, MAX_DECIMAL_SCALE + 1),
            Err(DecimalError::ScaleAboveBound)
        );
    }

    #[test]
    fn arithmetic_checks_scale_and_overflow() {
        let left = DecimalAmount::try_new(100, 2).unwrap();
        let right = DecimalAmount::try_new(25, 2).unwrap();
        assert_eq!(left.checked_add(right).unwrap().coefficient(), 125);
        assert_eq!(left.checked_sub(right).unwrap().coefficient(), 75);
        assert_eq!(
            left.checked_add(DecimalAmount::try_new(1, 3).unwrap()),
            Err(DecimalError::ScaleMismatch)
        );
        assert_eq!(
            DecimalAmount::try_new(i128::MAX, 0)
                .unwrap()
                .checked_add(DecimalAmount::try_new(1, 0).unwrap()),
            Err(DecimalError::Overflow)
        );
        assert_eq!(
            DecimalAmount::try_new(i128::MIN, 0).unwrap().checked_neg(),
            Err(DecimalError::Overflow)
        );
        assert_eq!(
            left.rescale(left.scale()),
            Err(DecimalError::NotImplemented(FinancialGap::Rescale))
        );
    }

    #[test]
    fn money_rejects_currency_mismatch_and_conversion() {
        let sum = usd(1, 0).checked_add(usd(2, 0)).unwrap();
        assert_eq!(sum.amount().coefficient(), 3);
        assert_eq!(sum.currency().as_str(), "USD");
        let eur = Money::new(sum.amount(), CurrencyCode::new("EUR").unwrap());
        assert_eq!(
            usd(1, 0).checked_add(eur),
            Err(MoneyError::CurrencyMismatch)
        );
        assert_eq!(
            usd(1, 0).convert(CurrencyCode::new("USD").unwrap()),
            Err(MoneyError::NotImplemented(FinancialGap::CurrencyConversion))
        );
        assert!(CurrencyCode::new("usd").is_err());
        assert!(CurrencyCode::new("US1").is_err());
        assert!(CurrencyCode::new("").is_err());
        let error = CurrencyCode::new("usd").unwrap_err();
        assert!(!error.to_string().contains("usd"));
    }

    #[test]
    fn price_and_quantity_reject_identity_mismatches() {
        let aapl = price(100, 2, "USD", "AAPL", "def-1");
        assert_eq!(aapl.checked_add(&aapl).unwrap().amount().coefficient(), 200);
        assert_eq!(
            aapl.checked_add(&price(100, 2, "EUR", "AAPL", "def-1")),
            Err(PriceError::CurrencyMismatch)
        );
        assert_eq!(
            aapl.checked_add(&price(100, 2, "USD", "MSFT", "def-1")),
            Err(PriceError::InstrumentMismatch)
        );
        assert_eq!(
            aapl.checked_add(&price(100, 2, "USD", "AAPL", "def-2")),
            Err(PriceError::DefinitionVersionMismatch)
        );
        assert_eq!(
            aapl.checked_add(&price(1, 3, "USD", "AAPL", "def-1")),
            Err(PriceError::Decimal(DecimalError::ScaleMismatch))
        );
        assert_eq!(
            aapl.convert_currency(CurrencyCode::new("USD").unwrap()),
            Err(PriceError::NotImplemented(FinancialGap::CurrencyConversion))
        );

        let shares = quantity(-5, 0, "AAPL", "def-1");
        assert_eq!(shares.amount().coefficient(), -5);
        assert_eq!(
            shares
                .checked_add(&quantity(1, 0, "AAPL", "def-1"))
                .unwrap()
                .amount()
                .coefficient(),
            -4
        );
        assert_eq!(
            shares.checked_add(&quantity(1, 0, "MSFT", "def-1")),
            Err(QuantityError::InstrumentMismatch)
        );
        assert_eq!(
            shares.checked_add(&quantity(1, 0, "AAPL", "def-2")),
            Err(QuantityError::DefinitionVersionMismatch)
        );
        assert_eq!(
            shares.convert_instrument(shares.instrument_id(), shares.definition_version()),
            Err(QuantityError::NotImplemented(
                FinancialGap::InstrumentConversion
            ))
        );
        assert_ne!(
            InstrumentId::new("AAPL").unwrap(),
            InstrumentId::new("aapl").unwrap()
        );
    }

    #[test]
    fn json_amounts_reject_floats_and_unknown_fields() {
        let money = usd(-125, 2);
        let value = serde_json::to_value(money).unwrap();
        assert!(value.get("coefficient").unwrap().is_string());
        assert!(value.get("scale").unwrap().is_string());
        assert_eq!(value.get("currency").unwrap(), "USD");
        let decoded: Money = serde_json::from_value(value).unwrap();
        assert_eq!(decoded, money);

        assert!(
            serde_json::from_str::<Money>(r#"{"coefficient":"1.5","scale":"0","currency":"USD"}"#)
                .is_err()
        );
        assert!(
            serde_json::from_str::<Money>(r#"{"coefficient":1,"scale":"0","currency":"USD"}"#)
                .is_err()
        );
        assert!(
            serde_json::from_str::<DecimalAmount>(r#"{"coefficient":"1","scale":"0","extra":"1"}"#)
                .is_err()
        );
        assert!(
            serde_json::from_str::<DecimalAmount>(
                r#"{"coefficient":"-170141183460469231731687303715884105728","scale":"0"}"#
            )
            .unwrap()
            .coefficient()
                == i128::MIN
        );
        assert_eq!(
            serde_json::from_str::<DecimalAmount>(
                r#"{"coefficient":"170141183460469231731687303715884105727","scale":"0"}"#
            )
            .unwrap()
            .coefficient(),
            i128::MAX
        );
        assert!(
            serde_json::from_str::<Money>(r#"{"coefficient":"01","scale":"0","currency":"USD"}"#)
                .is_err()
        );
        assert!(
            serde_json::from_str::<Money>(r#"{"coefficient":"1","scale":"0","currency":"usd"}"#)
                .is_err()
        );
    }
}
