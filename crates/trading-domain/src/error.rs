//! Typed failures for draft domain shells.
//!
//! Error text names the failed rule. It does not echo the rejected input.

/// A financial policy this crate deliberately does not perform.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FinancialGap {
    /// Changing decimal scale, including any rounding.
    Rescale,
    /// Converting an amount from one currency to another.
    CurrencyConversion,
    /// Replacing the instrument on a quantity or price.
    InstrumentConversion,
}

/// A time conversion this crate deliberately does not perform.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeGap {
    /// Converting nanoseconds, including exact multiples of 1_000.
    NanosecondConversion,
    /// Parsing or formatting a civil or RFC 3339 timestamp.
    Rfc3339,
}

/// A decoded label is not one of the known variants of a state enum.
///
/// The raw text is not stored. Unknown safety-critical labels fail closed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[error("unknown {type_name} value")]
pub struct UnknownState {
    type_name: &'static str,
}

impl UnknownState {
    pub(crate) const fn new(type_name: &'static str) -> Self {
        Self { type_name }
    }

    /// Returns the state type that rejected the value.
    pub const fn type_name(self) -> &'static str {
        self.type_name
    }
}

/// A text value is not the canonical hyphenated UUID for an identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[error("invalid canonical uuid for {type_name}")]
pub struct IdParseError {
    type_name: &'static str,
}

impl IdParseError {
    pub(crate) const fn new(type_name: &'static str) -> Self {
        Self { type_name }
    }

    /// Returns the identifier type that rejected the text.
    pub const fn type_name(self) -> &'static str {
        self.type_name
    }
}

/// A text value is not a canonical base-10 integer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[error("invalid canonical integer")]
pub struct IntegerParseError;

/// A bounded token failed validation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum TokenError {
    /// The token is empty.
    #[error("token is empty")]
    Empty,
    /// The token is shorter than the storage minimum.
    #[error("token is too short")]
    TooShort,
    /// The token is longer than the storage maximum.
    #[error("token is too long")]
    TooLong,
    /// The token contains a character outside its allowed set.
    #[error("token contains an invalid character")]
    InvalidCharacter,
}

/// [`crate::OrderVersion`] or [`crate::WriterEpoch`] could not advance by one.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum VersionOverflow {
    /// [`crate::OrderVersion`] is already `u64::MAX`.
    #[error("order version cannot advance past u64::MAX")]
    OrderVersion,
    /// [`crate::WriterEpoch`] is already `u64::MAX`.
    #[error("writer epoch cannot advance past u64::MAX")]
    WriterEpoch,
}

/// Checked decimal construction or arithmetic failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum DecimalError {
    /// The scale is above the storage bound.
    #[error("decimal scale exceeds the storage bound")]
    ScaleAboveBound,
    /// The scales differ. No rescaling or rounding is applied.
    #[error("decimal scales differ and rescaling is not implemented")]
    ScaleMismatch,
    /// The `i128` coefficient cannot represent the result.
    #[error("decimal coefficient overflow")]
    Overflow,
    /// Coefficient text is not a canonical integer.
    #[error("decimal coefficient is not a canonical integer")]
    InvalidCoefficient,
    /// Scale text is not a canonical integer.
    #[error("decimal scale is not a canonical integer")]
    InvalidScale,
    /// The operation would require a policy this crate does not implement.
    #[error("decimal operation is not implemented")]
    NotImplemented(FinancialGap),
}

/// [`crate::Money`] arithmetic or conversion failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum MoneyError {
    /// The currencies differ. No foreign-exchange conversion is applied.
    #[error("currency mismatch")]
    CurrencyMismatch,
    /// Currency conversion or another money policy is not implemented.
    #[error("money operation is not implemented")]
    NotImplemented(FinancialGap),
    /// The decimal shell rejected the coefficients.
    #[error(transparent)]
    Decimal(DecimalError),
}

/// [`crate::Price`] arithmetic failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum PriceError {
    /// The quote currencies differ.
    #[error("quote currency mismatch")]
    CurrencyMismatch,
    /// The instruments differ.
    #[error("instrument mismatch")]
    InstrumentMismatch,
    /// The instrument definition versions differ.
    #[error("instrument definition version mismatch")]
    DefinitionVersionMismatch,
    /// A price policy such as conversion is not implemented.
    #[error("price operation is not implemented")]
    NotImplemented(FinancialGap),
    /// The decimal shell rejected the coefficients.
    #[error(transparent)]
    Decimal(DecimalError),
}

/// [`crate::Quantity`] arithmetic failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum QuantityError {
    /// The instruments differ.
    #[error("instrument mismatch")]
    InstrumentMismatch,
    /// The instrument definition versions differ.
    #[error("instrument definition version mismatch")]
    DefinitionVersionMismatch,
    /// Changing instrument is not implemented.
    #[error("quantity operation is not implemented")]
    NotImplemented(FinancialGap),
    /// The decimal shell rejected the coefficients.
    #[error(transparent)]
    Decimal(DecimalError),
}

/// A timestamp shell rejected an operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum TimeError {
    /// The conversion or calendar operation is not implemented.
    #[error("time operation is not implemented")]
    NotImplemented(TimeGap),
}

/// A broker alias or execution identity document was rejected.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum IdentityParseError {
    /// The object contains a field this shell does not accept.
    #[error("identity document contains an unknown field")]
    UnknownField,
    /// A required field is absent.
    #[error("identity document is missing field {0}")]
    MissingField(&'static str),
    /// The same field appears more than once.
    #[error("identity document contains a duplicate field")]
    DuplicateField,
    /// A session id was supplied for an account-only scope.
    #[error("account-only scope cannot include a session id")]
    UnexpectedSession,
    /// An account-and-session scope did not include a session id.
    #[error("account-and-session scope requires a session id")]
    MissingSession,
    /// The session scope was not the only accepted label, `unverified`.
    #[error("session scope must be unverified")]
    SessionScopeNotUnverified,
    /// The alias kind does not match the destination type.
    #[error("alias kind does not match the destination type")]
    AliasKindMismatch,
    /// The scope kind is not one of the two draft scopes.
    #[error("scope kind is not recognized")]
    ScopeKindMismatch,
    /// The ambiguity label does not match the destination type.
    #[error("execution ambiguity does not match the destination type")]
    AmbiguityMismatch,
    /// The identity kind does not match the destination type.
    #[error("execution identity kind does not match the destination type")]
    IdentityKindMismatch,
    /// A token field failed validation.
    #[error(transparent)]
    InvalidToken(TokenError),
    /// A session id was not a canonical UUID.
    #[error(transparent)]
    InvalidIdentifier(IdParseError),
}
