//! Scoped broker and execution identity.
//!
//! Broker order ids are not internal order ids, and they are not globally
//! unique. IBKR client and session scope is unverified. Every alias records
//! that fact. This module does not resolve, merge, or submit identities.
//!
//! The JSON object is a draft shell, not the M0-02B canonical envelope.
//! `session_scope` must be `unverified`. A document cannot claim `verified`.

use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;

use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize, Serializer};

use crate::error::{IdentityParseError, TokenError};
use crate::identifier::BrokerSessionId;
use crate::parse::{TokenClass, parse_token};

const NAME_MAX: usize = 32;
const ACCOUNT_MAX: usize = 64;
const ENVIRONMENT_MAX: usize = 32;
const EXTERNAL_ID_MAX: usize = 128;

const ORDER_ALIAS_KIND: &str = "broker_order";
const PERM_ALIAS_KIND: &str = "broker_perm_id";
const EXECUTION_KIND: &str = "execution";
const QUARANTINED_KIND: &str = "quarantined_execution";
const AMBIGUITY_NONE: &str = "none_recorded";
const AMBIGUITY_QUARANTINED: &str = "quarantined";
const SESSION_UNVERIFIED: &str = "unverified";
const SCOPE_ACCOUNT_ONLY: &str = "account_only";
const SCOPE_ACCOUNT_AND_SESSION: &str = "account_and_session";

const ALIAS_FIELDS: &[&str] = &[
    "alias_kind",
    "broker",
    "account",
    "environment",
    "scope_kind",
    "session_scope",
    "session_id",
    "external_id",
];

const EXECUTION_FIELDS: &[&str] = &[
    "identity_kind",
    "broker",
    "account",
    "environment",
    "scope_kind",
    "session_scope",
    "session_id",
    "broker_execution_id",
    "ambiguity",
];

macro_rules! token_type {
    ($(#[$meta:meta])* $name:ident, $max:expr) => {
        $(#[$meta])*
        #[must_use]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        pub struct $name(String);

        impl $name {
            /// Validates and stores the token.
            pub fn new(value: &str) -> Result<Self, TokenError> {
                parse_token(value, 1, $max, TokenClass::PrintableAscii).map(Self)
            }

            /// Returns the token.
            pub fn as_str(&self) -> &str {
                &self.0
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
    };
}

token_type! {
    /// Broker name.
    ///
    /// Owner: broker integration. This is not an adapter selector and it does
    /// not open a connection. Printable ASCII, 1 to 32 bytes.
    BrokerName,
    NAME_MAX
}

token_type! {
    /// Broker account code.
    ///
    /// Owner: broker integration. Printable ASCII, 1 to 64 bytes. Not an
    /// internal [`crate::OrderId`].
    BrokerAccountCode,
    ACCOUNT_MAX
}

token_type! {
    /// Broker environment label.
    ///
    /// Owner: broker integration. This is not the process deployment
    /// environment and it does not authorize paper or live trading.
    /// Printable ASCII, 1 to 32 bytes.
    BrokerEnvironmentLabel,
    ENVIRONMENT_MAX
}

token_type! {
    /// External broker identifier text.
    ///
    /// Owner: broker identity. The text is only meaningful inside an explicit
    /// scope. Printable ASCII, 1 to 128 bytes. No broker grammar is checked.
    ExternalBrokerId,
    EXTERNAL_ID_MAX
}

/// Marker that the broker session or client dimension has not been verified.
///
/// IBKR session isolation and client-id scope are unverified. Constructing a
/// scope requires this value so the gap stays visible at the call site.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UnverifiedSessionScope;

/// How much session scope was supplied with a broker id.
///
/// Equality is structural. It is not evidence that the broker treats the
/// external id as unique inside that scope.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BrokerAliasScope {
    /// No session id was supplied.
    AccountOnly {
        /// Explicit unverified-scope marker.
        unverified: UnverifiedSessionScope,
    },
    /// An internal session id was supplied.
    ///
    /// Whether the broker itself scopes the external id by this session is
    /// unverified. A separate client id is not modeled.
    AccountAndSession {
        /// Internal session identifier.
        session_id: BrokerSessionId,
        /// Explicit unverified-scope marker.
        unverified: UnverifiedSessionScope,
    },
}

impl BrokerAliasScope {
    /// Records an account-scoped id whose session dimension is absent.
    pub const fn account_only(unverified: UnverifiedSessionScope) -> Self {
        Self::AccountOnly { unverified }
    }

    /// Records an internal session id without claiming the broker agrees.
    pub const fn account_and_session(
        session_id: BrokerSessionId,
        unverified: UnverifiedSessionScope,
    ) -> Self {
        Self::AccountAndSession {
            session_id,
            unverified,
        }
    }

    /// Returns the supplied internal session id, if one was recorded.
    pub const fn session_id(self) -> Option<BrokerSessionId> {
        match self {
            Self::AccountOnly { .. } => None,
            Self::AccountAndSession { session_id, .. } => Some(session_id),
        }
    }

    /// Returns the unverified-scope marker stored with this scope.
    pub const fn unverified(self) -> UnverifiedSessionScope {
        match self {
            Self::AccountOnly { unverified } | Self::AccountAndSession { unverified, .. } => {
                unverified
            }
        }
    }
}

/// Broker, account, and environment key.
///
/// Owner: broker integration. All three parts participate in equality.
#[must_use]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BrokerAccountKey {
    broker: BrokerName,
    account: BrokerAccountCode,
    environment: BrokerEnvironmentLabel,
}

impl BrokerAccountKey {
    /// Builds a key from the three scope parts.
    pub fn new(broker: &str, account: &str, environment: &str) -> Result<Self, TokenError> {
        Ok(Self {
            broker: BrokerName::new(broker)?,
            account: BrokerAccountCode::new(account)?,
            environment: BrokerEnvironmentLabel::new(environment)?,
        })
    }

    /// Returns the broker name.
    pub fn broker(&self) -> &BrokerName {
        &self.broker
    }

    /// Returns the account code.
    pub fn account(&self) -> &BrokerAccountCode {
        &self.account
    }

    /// Returns the environment label.
    ///
    /// The label does not authorize trading.
    pub fn environment(&self) -> &BrokerEnvironmentLabel {
        &self.environment
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct ScopedExternalId {
    account: BrokerAccountKey,
    scope: BrokerAliasScope,
    external_id: ExternalBrokerId,
}

impl ScopedExternalId {
    fn new(
        account: BrokerAccountKey,
        scope: BrokerAliasScope,
        external_id: ExternalBrokerId,
    ) -> Self {
        Self {
            account,
            scope,
            external_id,
        }
    }
}

/// Broker order alias.
///
/// Owner: broker identity. Distinct from [`crate::OrderId`] and from
/// [`BrokerPermIdAlias`]. Same external text in a different account or session
/// scope is not the same alias.
#[must_use]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BrokerOrderAlias(ScopedExternalId);

/// Permanent broker alias.
///
/// Owner: broker identity. Global uniqueness is not assumed. Distinct from
/// [`BrokerOrderAlias`].
#[must_use]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BrokerPermIdAlias(ScopedExternalId);

fn alias_new(
    account: BrokerAccountKey,
    scope: BrokerAliasScope,
    external_id: ExternalBrokerId,
) -> ScopedExternalId {
    ScopedExternalId::new(account, scope, external_id)
}

impl BrokerOrderAlias {
    /// Records a broker order id inside an explicit scope.
    pub fn new(
        account: BrokerAccountKey,
        scope: BrokerAliasScope,
        external_id: ExternalBrokerId,
    ) -> Self {
        Self(alias_new(account, scope, external_id))
    }

    /// Returns the account key.
    pub fn account(&self) -> &BrokerAccountKey {
        &self.0.account
    }

    /// Returns the session scope.
    pub fn scope(&self) -> BrokerAliasScope {
        self.0.scope
    }

    /// Returns the external id text.
    pub fn external_id(&self) -> &ExternalBrokerId {
        &self.0.external_id
    }
}

impl BrokerPermIdAlias {
    /// Records a permanent broker id inside an explicit scope.
    pub fn new(
        account: BrokerAccountKey,
        scope: BrokerAliasScope,
        external_id: ExternalBrokerId,
    ) -> Self {
        Self(alias_new(account, scope, external_id))
    }

    /// Returns the account key.
    pub fn account(&self) -> &BrokerAccountKey {
        &self.0.account
    }

    /// Returns the session scope.
    pub fn scope(&self) -> BrokerAliasScope {
        self.0.scope
    }

    /// Returns the external id text.
    pub fn external_id(&self) -> &ExternalBrokerId {
        &self.0.external_id
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct ExecutionKey {
    account: BrokerAccountKey,
    scope: BrokerAliasScope,
    broker_execution_id: ExternalBrokerId,
}

/// Execution identity whose ambiguity was not flagged.
///
/// Owner: execution ledger. `none_recorded` is not a uniqueness proof. This
/// type is not [`QuarantinedExecutionIdentity`], and there is no resolver.
#[must_use]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExecutionIdentity(ExecutionKey);

/// Execution identity held back because its scope was ambiguous.
///
/// Owner: execution ledger. Quarantine is not cleared by cloning the key into
/// an [`ExecutionIdentity`]. No such conversion is provided.
#[must_use]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct QuarantinedExecutionIdentity(ExecutionKey);

impl ExecutionIdentity {
    /// Records a scoped broker execution id without a uniqueness claim.
    pub fn new(
        account: BrokerAccountKey,
        scope: BrokerAliasScope,
        broker_execution_id: ExternalBrokerId,
    ) -> Self {
        Self(ExecutionKey {
            account,
            scope,
            broker_execution_id,
        })
    }

    /// Moves the key into quarantine.
    ///
    /// The consumed value can no longer be used as an unflagged identity.
    pub fn quarantine(self) -> QuarantinedExecutionIdentity {
        QuarantinedExecutionIdentity(self.0)
    }

    /// Returns the account key.
    pub fn account(&self) -> &BrokerAccountKey {
        &self.0.account
    }

    /// Returns the session scope.
    pub fn scope(&self) -> BrokerAliasScope {
        self.0.scope
    }

    /// Returns the broker execution id.
    pub fn broker_execution_id(&self) -> &ExternalBrokerId {
        &self.0.broker_execution_id
    }
}

impl QuarantinedExecutionIdentity {
    /// Returns the account key.
    pub fn account(&self) -> &BrokerAccountKey {
        &self.0.account
    }

    /// Returns the session scope.
    pub fn scope(&self) -> BrokerAliasScope {
        self.0.scope
    }

    /// Returns the broker execution id.
    pub fn broker_execution_id(&self) -> &ExternalBrokerId {
        &self.0.broker_execution_id
    }
}

struct Fields(BTreeMap<String, String>);

impl Fields {
    fn take(&mut self, key: &'static str) -> Result<String, IdentityParseError> {
        self.0
            .remove(key)
            .ok_or(IdentityParseError::MissingField(key))
    }

    fn contains(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    fn reject_unknown(&self) -> Result<(), IdentityParseError> {
        if self.0.is_empty() {
            Ok(())
        } else {
            Err(IdentityParseError::UnknownField)
        }
    }
}

fn deserialize_string_map<'de, D>(deserializer: D) -> Result<BTreeMap<String, String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct MapVisitor;

    impl<'de> Visitor<'de> for MapVisitor {
        type Value = BTreeMap<String, String>;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("an object of strings")
        }

        fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut fields = BTreeMap::new();
            while let Some(key) = access.next_key::<String>()? {
                if fields.contains_key(&key) {
                    return Err(de::Error::custom(IdentityParseError::DuplicateField));
                }
                let value = access.next_value::<String>()?;
                fields.insert(key, value);
            }
            Ok(fields)
        }
    }

    deserializer.deserialize_map(MapVisitor)
}

fn reject_disallowed(
    fields: &BTreeMap<String, String>,
    allowed: &[&str],
) -> Result<(), IdentityParseError> {
    if fields.keys().any(|key| !allowed.contains(&key.as_str())) {
        Err(IdentityParseError::UnknownField)
    } else {
        Ok(())
    }
}

fn parse_scope(fields: &mut Fields) -> Result<BrokerAliasScope, IdentityParseError> {
    let scope_kind = fields.take("scope_kind")?;
    let session_scope = fields.take("session_scope")?;
    if session_scope != SESSION_UNVERIFIED {
        return Err(IdentityParseError::SessionScopeNotUnverified);
    }
    match scope_kind.as_str() {
        SCOPE_ACCOUNT_ONLY => {
            if fields.contains("session_id") {
                return Err(IdentityParseError::UnexpectedSession);
            }
            Ok(BrokerAliasScope::account_only(UnverifiedSessionScope))
        }
        SCOPE_ACCOUNT_AND_SESSION => {
            if !fields.contains("session_id") {
                return Err(IdentityParseError::MissingSession);
            }
            let session_id = fields.take("session_id")?;
            let session_id = BrokerSessionId::from_str(&session_id)
                .map_err(IdentityParseError::InvalidIdentifier)?;
            Ok(BrokerAliasScope::account_and_session(
                session_id,
                UnverifiedSessionScope,
            ))
        }
        _ => Err(IdentityParseError::ScopeKindMismatch),
    }
}

fn parse_account(fields: &mut Fields) -> Result<BrokerAccountKey, IdentityParseError> {
    let broker = fields.take("broker")?;
    let account = fields.take("account")?;
    let environment = fields.take("environment")?;
    BrokerAccountKey::new(&broker, &account, &environment).map_err(IdentityParseError::InvalidToken)
}

fn parse_alias(
    mut fields: Fields,
    expected_kind: &str,
) -> Result<ScopedExternalId, IdentityParseError> {
    let alias_kind = fields.take("alias_kind")?;
    if alias_kind != expected_kind {
        return Err(IdentityParseError::AliasKindMismatch);
    }
    let account = parse_account(&mut fields)?;
    let scope = parse_scope(&mut fields)?;
    let external_id = fields.take("external_id")?;
    fields.reject_unknown()?;
    let external_id =
        ExternalBrokerId::new(&external_id).map_err(IdentityParseError::InvalidToken)?;
    Ok(ScopedExternalId::new(account, scope, external_id))
}

fn parse_execution(
    mut fields: Fields,
    expected_kind: &str,
    expected_ambiguity: &str,
) -> Result<ExecutionKey, IdentityParseError> {
    let identity_kind = fields.take("identity_kind")?;
    if identity_kind != expected_kind {
        return Err(IdentityParseError::IdentityKindMismatch);
    }
    let ambiguity = fields.take("ambiguity")?;
    if ambiguity != expected_ambiguity {
        return Err(IdentityParseError::AmbiguityMismatch);
    }
    let account = parse_account(&mut fields)?;
    let scope = parse_scope(&mut fields)?;
    let broker_execution_id = fields.take("broker_execution_id")?;
    fields.reject_unknown()?;
    let broker_execution_id =
        ExternalBrokerId::new(&broker_execution_id).map_err(IdentityParseError::InvalidToken)?;
    Ok(ExecutionKey {
        account,
        scope,
        broker_execution_id,
    })
}

fn load_fields<'de, D>(deserializer: D, allowed: &[&str]) -> Result<Fields, D::Error>
where
    D: Deserializer<'de>,
{
    let fields = deserialize_string_map(deserializer)?;
    reject_disallowed(&fields, allowed).map_err(de::Error::custom)?;
    Ok(Fields(fields))
}

fn serialize_scope<M>(map: &mut M, scope: BrokerAliasScope) -> Result<(), M::Error>
where
    M: SerializeMap,
{
    match scope {
        BrokerAliasScope::AccountOnly { .. } => {
            map.serialize_entry("scope_kind", SCOPE_ACCOUNT_ONLY)?;
            map.serialize_entry("session_scope", SESSION_UNVERIFIED)?;
        }
        BrokerAliasScope::AccountAndSession { session_id, .. } => {
            map.serialize_entry("scope_kind", SCOPE_ACCOUNT_AND_SESSION)?;
            map.serialize_entry("session_scope", SESSION_UNVERIFIED)?;
            map.serialize_entry("session_id", &session_id.to_string())?;
        }
    }
    Ok(())
}

fn serialize_account<M>(map: &mut M, account: &BrokerAccountKey) -> Result<(), M::Error>
where
    M: SerializeMap,
{
    map.serialize_entry("broker", account.broker.as_str())?;
    map.serialize_entry("account", account.account.as_str())?;
    map.serialize_entry("environment", account.environment.as_str())?;
    Ok(())
}

fn serialize_alias<S>(
    alias_kind: &str,
    value: &ScopedExternalId,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let field_count = match value.scope {
        BrokerAliasScope::AccountOnly { .. } => 7,
        BrokerAliasScope::AccountAndSession { .. } => 8,
    };
    let mut map = serializer.serialize_map(Some(field_count))?;
    map.serialize_entry("alias_kind", alias_kind)?;
    serialize_account(&mut map, &value.account)?;
    serialize_scope(&mut map, value.scope)?;
    map.serialize_entry("external_id", value.external_id.as_str())?;
    map.end()
}

fn serialize_execution<S>(
    identity_kind: &str,
    ambiguity: &str,
    value: &ExecutionKey,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let field_count = match value.scope {
        BrokerAliasScope::AccountOnly { .. } => 8,
        BrokerAliasScope::AccountAndSession { .. } => 9,
    };
    let mut map = serializer.serialize_map(Some(field_count))?;
    map.serialize_entry("identity_kind", identity_kind)?;
    serialize_account(&mut map, &value.account)?;
    serialize_scope(&mut map, value.scope)?;
    map.serialize_entry("broker_execution_id", value.broker_execution_id.as_str())?;
    map.serialize_entry("ambiguity", ambiguity)?;
    map.end()
}

impl Serialize for BrokerOrderAlias {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_alias(ORDER_ALIAS_KIND, &self.0, serializer)
    }
}

impl<'de> Deserialize<'de> for BrokerOrderAlias {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let fields = load_fields(deserializer, ALIAS_FIELDS)?;
        parse_alias(fields, ORDER_ALIAS_KIND)
            .map(Self)
            .map_err(de::Error::custom)
    }
}

impl Serialize for BrokerPermIdAlias {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_alias(PERM_ALIAS_KIND, &self.0, serializer)
    }
}

impl<'de> Deserialize<'de> for BrokerPermIdAlias {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let fields = load_fields(deserializer, ALIAS_FIELDS)?;
        parse_alias(fields, PERM_ALIAS_KIND)
            .map(Self)
            .map_err(de::Error::custom)
    }
}

impl Serialize for ExecutionIdentity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_execution(EXECUTION_KIND, AMBIGUITY_NONE, &self.0, serializer)
    }
}

impl<'de> Deserialize<'de> for ExecutionIdentity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let fields = load_fields(deserializer, EXECUTION_FIELDS)?;
        parse_execution(fields, EXECUTION_KIND, AMBIGUITY_NONE)
            .map(Self)
            .map_err(de::Error::custom)
    }
}

impl Serialize for QuarantinedExecutionIdentity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_execution(QUARANTINED_KIND, AMBIGUITY_QUARANTINED, &self.0, serializer)
    }
}

impl<'de> Deserialize<'de> for QuarantinedExecutionIdentity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let fields = load_fields(deserializer, EXECUTION_FIELDS)?;
        parse_execution(fields, QUARANTINED_KIND, AMBIGUITY_QUARANTINED)
            .map(Self)
            .map_err(de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{
        BrokerAccountKey, BrokerAliasScope, BrokerEnvironmentLabel, BrokerOrderAlias,
        BrokerPermIdAlias, ExecutionIdentity, ExternalBrokerId, QuarantinedExecutionIdentity,
        UnverifiedSessionScope,
    };
    use crate::BROKER_DISPATCH_IMPLEMENTED;
    use crate::identifier::BrokerSessionId;

    fn account(environment: &str) -> BrokerAccountKey {
        BrokerAccountKey::new("IBKR", "U1", environment).unwrap()
    }

    fn external(value: &str) -> ExternalBrokerId {
        ExternalBrokerId::new(value).unwrap()
    }

    fn session(last: char) -> BrokerSessionId {
        let text = format!("aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeee{last}");
        BrokerSessionId::from_str(&text).unwrap()
    }

    #[test]
    fn equal_raw_ids_in_different_scopes_are_not_equal() {
        let raw = external("1001");
        let marker = UnverifiedSessionScope;
        let first_account = account("paper-label");
        let other_account = BrokerAccountKey::new("IBKR", "U2", "paper-label").unwrap();
        let other_broker = BrokerAccountKey::new("OTHER", "U1", "paper-label").unwrap();
        let other_environment = account("live");
        let session_a = session('1');
        let session_b = session('2');

        let account_only = BrokerOrderAlias::new(
            first_account.clone(),
            BrokerAliasScope::account_only(marker),
            raw.clone(),
        );
        assert_ne!(
            account_only,
            BrokerOrderAlias::new(
                other_account,
                BrokerAliasScope::account_only(marker),
                raw.clone(),
            )
        );
        assert_ne!(
            account_only,
            BrokerOrderAlias::new(
                other_broker,
                BrokerAliasScope::account_only(marker),
                raw.clone(),
            )
        );
        assert_ne!(
            account_only,
            BrokerOrderAlias::new(
                other_environment,
                BrokerAliasScope::account_only(marker),
                raw.clone(),
            )
        );
        let with_session = BrokerOrderAlias::new(
            first_account.clone(),
            BrokerAliasScope::account_and_session(session_a, marker),
            raw.clone(),
        );
        assert_ne!(account_only, with_session);
        assert_ne!(
            with_session,
            BrokerOrderAlias::new(
                first_account.clone(),
                BrokerAliasScope::account_and_session(session_b, marker),
                raw.clone(),
            )
        );
        assert_eq!(
            with_session,
            BrokerOrderAlias::new(
                first_account,
                BrokerAliasScope::account_and_session(session_a, marker),
                raw,
            )
        );
        assert!(with_session.scope().session_id().is_some());
        assert_eq!(account_only.scope().unverified(), marker);
    }

    #[test]
    fn permanent_aliases_use_the_same_scope_rule() {
        let raw = external("PERM");
        let marker = UnverifiedSessionScope;
        let left = BrokerPermIdAlias::new(
            account("paper-label"),
            BrokerAliasScope::account_only(marker),
            raw.clone(),
        );
        let right =
            BrokerPermIdAlias::new(account("live"), BrokerAliasScope::account_only(marker), raw);
        assert_ne!(left, right);
    }

    #[test]
    fn environment_labels_do_not_authorize_trading() {
        let live = BrokerEnvironmentLabel::new("live").unwrap();
        let paper = BrokerEnvironmentLabel::new("paper").unwrap();
        assert_eq!(live.as_str(), "live");
        assert_eq!(paper.as_str(), "paper");
        assert_ne!(live, paper);
        const { assert!(!BROKER_DISPATCH_IMPLEMENTED) };
    }

    #[test]
    fn quarantine_preserves_the_key_and_the_wire_kind() {
        let identity = ExecutionIdentity::new(
            account("paper-label"),
            BrokerAliasScope::account_only(UnverifiedSessionScope),
            external("EXEC-1"),
        );
        let quarantined = identity.clone().quarantine();
        assert_eq!(
            identity.broker_execution_id(),
            quarantined.broker_execution_id()
        );
        assert_eq!(identity.account(), quarantined.account());
        let recorded_json = serde_json::to_string(&identity).unwrap();
        let quarantined_json = serde_json::to_string(&quarantined).unwrap();
        assert!(recorded_json.contains("\"ambiguity\":\"none_recorded\""));
        assert!(quarantined_json.contains("\"ambiguity\":\"quarantined\""));
        assert!(serde_json::from_str::<ExecutionIdentity>(&quarantined_json).is_err());
        assert!(serde_json::from_str::<QuarantinedExecutionIdentity>(&recorded_json).is_err());
        let restored: ExecutionIdentity = serde_json::from_str(&recorded_json).unwrap();
        assert_eq!(restored, identity);
    }

    #[test]
    fn alias_documents_fail_closed_on_scope_and_kind() {
        let alias = BrokerOrderAlias::new(
            account("paper-label"),
            BrokerAliasScope::account_and_session(session('3'), UnverifiedSessionScope),
            external("1001"),
        );
        let json = serde_json::to_value(&alias).unwrap();
        assert_eq!(json.get("alias_kind").unwrap(), "broker_order");
        assert_eq!(json.get("session_scope").unwrap(), "unverified");
        assert_eq!(json.get("external_id").unwrap(), "1001");
        let restored: BrokerOrderAlias = serde_json::from_value(json.clone()).unwrap();
        assert_eq!(restored, alias);
        assert!(serde_json::from_value::<BrokerPermIdAlias>(json).is_err());

        let mut verified = serde_json::to_value(&alias).unwrap();
        verified["session_scope"] = serde_json::json!("verified");
        assert!(serde_json::from_value::<BrokerOrderAlias>(verified).is_err());

        let mut global = serde_json::to_value(&alias).unwrap();
        global["scope_kind"] = serde_json::json!("global");
        assert!(serde_json::from_value::<BrokerOrderAlias>(global).is_err());

        let mut extra_session = serde_json::to_value(BrokerOrderAlias::new(
            account("paper-label"),
            BrokerAliasScope::account_only(UnverifiedSessionScope),
            external("1001"),
        ))
        .unwrap();
        extra_session["session_id"] = serde_json::json!("00000000-0000-0000-0000-000000000003");
        assert!(serde_json::from_value::<BrokerOrderAlias>(extra_session).is_err());

        let mut missing_session = serde_json::to_value(&alias).unwrap();
        let object = missing_session.as_object_mut().unwrap();
        object.remove("session_id");
        assert!(serde_json::from_value::<BrokerOrderAlias>(missing_session).is_err());

        let mut uppercase_session = serde_json::to_value(&alias).unwrap();
        let uppercase = uppercase_session["session_id"]
            .as_str()
            .unwrap()
            .to_ascii_uppercase();
        uppercase_session["session_id"] = serde_json::Value::String(uppercase);
        assert!(serde_json::from_value::<BrokerOrderAlias>(uppercase_session).is_err());
    }
}
