# Unresolved domain decisions

**Status:** DESIGN_RESOLVED / IMPLEMENTATION_PENDING. The shells below are still in force. They are not production-proven.  
**Milestone:** M0-02A provisional shells. M0-02B-a is the registry skeleton only. M0-02B-b canonical schemas are not published.  
**Date:** 2026-11-14  
**Resolution:** [M0-02A-UD-01-bis-UD-09-Architecture-Resolution-v1.0.md](../sources/M0-02A-UD-01-bis-UD-09-Architecture-Resolution-v1.0.md)  
**Index:** [traceability-index.md](../traceability-index.md)

M0-02A needed Rust types whose full representation is not fixed by CR-B10. Where a shell was required, it fails closed and is named here. Do not treat a shell as production-proven. DET-01 through DET-10 and CR-B01 through CR-B09 were not in the repository when the shells were written. They were still not supplied with the resolution, so no transition or policy text from those documents was reviewed. If a later copy of those documents uses different names or scopes, that text overrides these shells and the vocabulary must be revised. Do not silently keep the shell.

The resolution refines CR-B10's explicitly provisional details. It does not replace a DET invariant. Implementation of each decision below is pending. Broker uniqueness rules stay **EVIDENCE_PENDING**.

## UD-01 — Two types named CorrelationId

**Status:** DESIGN_RESOLVED / IMPLEMENTATION_PENDING

### Shell still in force

`trading_common::CorrelationId` is the existing HTTP request handle. It is process-local and is not a distributed trace. CR-B10 `CorrelationId` is the domain id that links commands and events.

M0-02A keeps both types. There is no conversion. Renaming either one would change a foundation type or invent a name the contract does not use. A later ADR should rename one of them before any API publishes both on the same document.

### Resolution

Keep `trading_domain::CorrelationId` as the domain identity. Rename the HTTP type to `HttpRequestId` in a separately scoped refactor before either type is exposed in a public schema. Preserve HTTP behavior through an explicitly documented compatibility adapter if one is required. Do not convert or equate the ids. Domain correlation is not a distributed span. `trading_common::CorrelationId` may stay temporarily, marked deprecated and internal, until API tests and consumers migrate. That rename is not in this change. The registry lists both qualified names with `public_schema` false, and lists `trading_common::HttpRequestId` as not implemented.

## UD-02 — Broker alias scope, including IBKR

**Status:** DESIGN_RESOLVED / IMPLEMENTATION_PENDING  
**Broker uniqueness:** EVIDENCE_PENDING

### Shell still in force

CR-B10 says a broker order alias includes broker, account, client or session as appropriate, and the external id kind. It also says IBKR identity and session isolation are not yet evidenced.

The shell records:

- broker, account, and an environment label
- either no session id (`account_only`) or one internal `BrokerSessionId`
- an `UnverifiedSessionScope` value on every scope
- separate types for order alias and permanent alias

A separate client id is not modeled. Equality is structural equality of the recorded scope. It is not a uniqueness proof and it is not a merge. `session_scope` on the draft JSON shell must be the text `unverified`; `verified` is rejected. No identity resolver is implemented.

### Resolution

Keep `UnverifiedSessionScope` as a fail-closed shell. A future `BrokerIdentityContext` is to carry broker, account, environment, broker client id, connection or session identity, adapter session epoch, external id kind, and provenance or evidence id. Not every field belongs in every identity key. Kind-specific uniqueness for `orderId`, `permId`, and `execId` needs evidence-backed verified scopes and a separately approved rule table. `permId` is not assumed globally unique. Matching raw numeric aliases across clients or sessions are not equal. A broker `orderId` is a scoped alias, not an order primary key. Unknown or missing required scope quarantines; it does not auto-merge. External source identifiers stay raw immutable evidence. Client id is a different type from internal `BrokerSessionId`. Do not turn `unverified` into `verified` by renaming a field. `BrokerIdentityContext` is not implemented. Verified scopes are not claimed.

## UD-03 — Decimal representation, scale, and rounding

**Status:** DESIGN_RESOLVED / IMPLEMENTATION_PENDING

### Shell still in force

CR-B10 requires checked decimal arithmetic and an explicit rounding policy. It does not name the coefficient width, the scale bound, or the rounding mode.

The shell stores a signed `i128` coefficient and a scale from 0 through 38. Thirty-eight is the decimal digit budget of `i128`, not a tick size. Values at different scales are not normalized: coefficient `10` at scale `1` is not equal to coefficient `100` at scale `2`. Addition and subtraction require the same scale. `rescale` always returns `NotImplemented`, including when the scale is unchanged. There is no `f32` or `f64` field.

Currency codes are uppercase ASCII, 1 to 16 letters. That is not an ISO 4217 registry check. Lowercase is rejected rather than case-folded. Foreign exchange always returns `NotImplemented`.

Instrument ids are printable ASCII, 1 to 64 bytes, with no case folding and no symbology check.

### Resolution

Keep the signed `i128` coefficient and scale `0..=38` as a bounded storage primitive. That bound does not make every coefficient or scale valid for every operation. Overflow, including intermediate multiplication overflow, is rejected. Economic comparison must use mathematical equality across scales, through checked normalization or comparison. Structural representation equality may remain a separate explicit operation. The shell still uses structural, scale-sensitive equality. Numeric equality is not implemented. Scale is not a tick size. `Money`, `Price`, and `Quantity` pin currency or instrument and definition versions. Default arithmetic stays exact: no implicit rescale, foreign exchange, or rounding. `RoundingPolicy` is decided as `RejectInexact` (default), `TowardZero`, `AwayFromZero`, `HalfEven`, `Floor`, and `Ceiling`. A mode is allowed only when the caller supplies a validated target quantum or scale and a business-operation policy, and the result carries provenance. Financial decisions use operation-specific allowed modes. Division, conversion, or instrument-tick rounding without a validated policy fails closed. `rescale` stays `NotImplemented` until a tested policy implementation exists. `RoundingPolicy` is not implemented.

## UD-04 — Version text versus version integers

**Status:** DESIGN_RESOLVED / IMPLEMENTATION_PENDING

### Shell still in force

`OrderVersion` is a `u64` because CR-B10 section 1 says so. `SchemaVersion` is a `u64` because the only envelope example is a JSON integer. It has no successor method. A larger integer is not a compatibility claim.

`WriterEpoch` is a `u64` because M0-02A requires overflow rejection for a monotonic local counter. The width is provisional. It is not a broker fence.

`RiskPolicyVersion`, `NormalizerVersion`, `InstrumentDefinitionVersion`, and `ValuationModelVersion` are opaque printable ASCII tokens, 1 to 128 bytes. CR-B10 does not say whether they are integers or semantic versions. They do not order. `PolicyVersion` in CR-B10 section 3 is not a second Rust type; the section 1 name `RiskPolicyVersion` is used. If those names diverge later, add a type rather than overloading this one.

### Resolution

`OrderVersion`, `WriterEpoch`, and `SchemaVersion` stay separate `u64` newtypes. Overflow fails closed. At runtime, `WriterEpoch` must be allocated by the database and transactionally fenced. It must not be inferred from an in-process counter. The shell's `checked_next` is that local counter and is not the fence. `SchemaVersion` denotes the major wire-contract version. Additive evolution must be explicitly compatible and fixture-tested. Numeric comparison is not compatibility. The four opaque versions stay immutable ASCII identifiers with exact equality and no implicit ordering. Registry metadata is where lineage and compatibility are declared. In a risk envelope, `PolicyVersion` means `RiskPolicyVersion`. Do not alias other policy domains onto it. Epoch fencing integration is deferred to the dispatcher milestone. No schema-compatibility validator for published wire schemas exists yet. The M0-02B-a skeleton validator does not perform that job.

## UD-05 — Timestamp resolution

**Status:** DESIGN_RESOLVED / IMPLEMENTATION_PENDING

### Shell still in force

`OccurredAt`, `ReceivedAt`, and `PersistedAt` store `i64` microseconds since the Unix epoch. The resolution is a storage choice. Nanosecond values are rejected even when they are exact multiples of 1_000, so the shell does not truncate. RFC 3339 parsing is not implemented. Zero is an instant, not a missing timestamp. Clock quality is not represented. Ordering inside one type orders stored microseconds only. It does not order events or broker messages. Calendar logic is absent.

### Resolution

Signed `i64` Unix microseconds stay the in-memory precision. The decided event wire encoding is normalized RFC 3339 UTC with exactly six fractional digits. Parsing accepts only explicit timezone offsets, converts with checked range and precision, and rejects nonzero sub-microsecond precision. Persist integer microseconds where the database constraints support it. An absent source timestamp is `Option<OccurredAt>` only where a contract allows it. `ClockQuality` and `TimestampProvenance` are separate metadata and are not inferred from the timestamp. `PersistedAt` is assigned by the persistence boundary and is not proof of an external broker order. The persistence boundary does not assign it yet. Zero is a real timestamp. RFC 3339 stays `NotImplemented` until tests pass. Timestamp order is not broker precedence. `ClockQuality` and `TimestampProvenance` are not implemented.

## UD-06 — State transitions

**Status:** DESIGN_RESOLVED / IMPLEMENTATION_PENDING

### Shell still in force

CR-B10 section 2 gives draft variant names and says the transition matrix is a separate acceptance item. DET-01 was not supplied. M0-02A defines the enums and does not define edges, guards, or events. `RiskReservationAdjusted` is not a variant. `BrokerLifecycleState::Unknown` is a known label; unrecognized text does not become `Unknown`, `Ready`, `Allow`, `Released`, or `ConfirmedAccepted`.

Declaration order is not lifecycle order. The enums do not implement `Ord` or `Default`.

### Resolution

The enum labels are frozen as draft registry candidates. They are not executable transition rules. Do not add edges or guards until DET-01, DET-02, DET-06, and DET-07 are in the repository and cross-reviewed. Those bodies are not in the repository and were not reviewed. Unknown wire state fails closed. Do not implement `Ord` or `Default`. `RiskReservationAdjusted` stays an event, not a state. The draft spelling `Cancelled` stays. A broker adapter must normalize raw vendor spelling and preserve the raw evidence. Transition matrices are not implemented.

## UD-07 — Types named by CR-B10 and not implemented

**Status:** DESIGN_RESOLVED / IMPLEMENTATION_PENDING

### Shell still in force

`OrderClosure` needs a reason vocabulary that CR-B10 does not list. `IncidentState` is named in section 1 and has no variant list. Neither type was invented. `ExecutionState` is implemented from section 2 even though section 1 has no row for it; the glossary marks that ownership as taken from section 2.

### Resolution

`OrderClosure` is to be an immutable record with closure reason, closing `EventId`, closing `OrderVersion`, occurred and recorded time, and an audit reference. Do not invent the closure-reason enum until DET-01 supplies that vocabulary. DET-01 was not supplied. A non-instantiable skeleton may exist later. This change does not add the Rust type, and it does not expose `OrderClosure` as a canonical wire schema. `IncidentState` waits until DET-10 is present and reviewed. DET-10 was not supplied. `ExecutionState` is owned by the execution ledger. The glossary ownership row records that assignment. No state variants were fabricated.

## UD-08 — Canonical JSON

**Status:** DESIGN_RESOLVED / IMPLEMENTATION_PENDING

### Shell still in force

Serde implementations exist so unknown state text and float amounts can be rejected in tests. Field names and enum spellings are draft. They are not the M0-02B canonical envelope, and identical bytes are not promised across serializer versions. Enum labels use the CR-B10 PascalCase variant names. `RECONCILIATION_REQUIRED` is rejected. `Canceled` is rejected; the broker label is `Cancelled`.

### Resolution

Current serde shapes are an internal draft. M0-02B-b is the milestone that defines canonical event and command wire schemas: fixed field names, string encoding for arbitrary-precision coefficients, stable enum labels, `deny_unknown_fields` on safety-critical messages, bounded payload size, explicit UTC timestamp normalization, and deterministic key ordering and numeric formatting. Canonical bytes require a documented serializer profile and golden fixtures. Ordinary `serde_json` map iteration is not that profile. The event envelope follows CR-B10 §4, with nullability, schema name, major version, and causal identifiers documented separately. A payload hash needs an explicitly named algorithm and domain separator. Draft fields are not silently converted into that hash. M0-02B-a publishes the registry skeleton only. It does not publish those schemas. Golden-byte tests are not implemented.

## UD-09 — UUID version

**Status:** DESIGN_RESOLVED / IMPLEMENTATION_PENDING

### Shell still in force

CR-B10 proposes UUID version 7 and allows version 4. M0-02A generates version 4, matching the existing workspace dependency. Constructors accept any UUID bits. The version nibble and any timestamp bits are not interpreted. UUID order is not event order.

### Resolution

Keep UUIDv4 generation for M0-02A and for the initial M0-02B schemas. UUIDv7 is an optional future ADR, not a migration prerequisite. Accepting arbitrary UUID bits from external input is allowed for typed parsing only. It does not prove that an identifier was minted internally. Non-nil checks and permitted provenance belong at the relevant command trust boundaries. Those checks are not implemented. UUID sorting is not event order. Historical ids are not rewritten.
