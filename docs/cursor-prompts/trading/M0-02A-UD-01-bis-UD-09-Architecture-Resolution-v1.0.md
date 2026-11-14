# M0-02A — Architecture Resolution UD-01..UD-09

**Date:** 2026-11-14  
**Status:** DESIGN_RESOLVED; IMPLEMENTATION_NOT_VERIFIED; BROKER_SEMANTICS_UNVERIFIED; LIVE_NOT_AUTHORIZED  
**Basis:** Cursor implementation report, domain glossary, unresolved decisions, CR-B10, DET-03..08.  
**Priority:** Where these decisions refine CR-B10's explicitly provisional details, apply this document. DET contracts remain authoritative for their domain invariants. No runtime code has been inspected or changed.

## Acceptance assessment

M0-02A is **provisionally accepted as a fail-closed domain shell**, not as production-complete domain behavior. Cursor reports 83 library tests and 17 doctests passing; PostgreSQL integration remains ignored; CI was not run remotely. M0-02B cannot publish current provisional JSON shapes unchanged.

## UD-01 — Correlation identity

**Decision:** Preserve `trading_domain::CorrelationId` as the cross-command/event domain identity. Rename the HTTP-local type to `HttpRequestId` in a separately scoped refactor **before exposing both types in any public schema**. Preserve existing HTTP request behavior and serialization via an explicitly documented compatibility adapter if required. Do not implicitly convert or equate IDs. Trace propagation is a later concern; domain correlation is not a distributed span. Existing `trading_common::CorrelationId` may remain temporarily, marked deprecated/internal, until API tests and consumers migrate.

**Gate:** Compile-time non-interchangeability; API response/header regression tests; no duplicate exported `CorrelationId` in shared API documentation.

## UD-02 — Broker aliases and IBKR scope

**Decision:** The current `UnverifiedSessionScope` model is retained as a fail-closed shell only. Define a future `BrokerIdentityContext` containing broker, account, environment, broker client ID (explicit typed value when available), connection/session identity, adapter session epoch, external ID kind, and provenance/evidence ID. Not every field belongs in every identity key: define **kind-specific uniqueness rules** for orderId, permId and execId, with evidence-backed verified scopes. Never assume `permId` is globally unique, and never equate matching raw numeric aliases across clients/sessions. Broker-assigned `orderId` is a scoped alias, not an order primary key. Unknown/missing required scope => quarantine, no auto-merge. External source identifiers remain raw immutable evidence.

**Gate:** Model client ID separately from internal `BrokerSessionId`; add ambiguous/collision fixtures; preserve raw observations; verified scopes require integration evidence and a separately approved rule table. Do not change the shell's `unverified` behavior into `verified` just by renaming a field.

## UD-03 — Decimal arithmetic and rounding

**Decision:** Keep signed `i128` coefficient + scale `0..=38` as a bounded storage primitive, not an assertion that all coefficients/scales are valid for every operation. Reject overflow and any intermediate multiplication overflow. Numeric **equality** must be mathematical for amounts with different scales when used in economic comparisons, via checked normalization/comparison; structural representation equality may remain a distinct explicit operation. Never use scale as tick size. `Money`, `Price`, `Quantity` must pin currency/instrument and definition versions. Default arithmetic: exact only; no implicit rescale, FX or rounding. Explicit `RoundingPolicy` is an enum with `RejectInexact` (default), `TowardZero`, `AwayFromZero`, `HalfEven`, `Floor`, `Ceiling`; a rounding mode is permitted only when a caller supplies a validated target quantum/scale and a business-operation policy, with the result carrying provenance. Financial decisions should use operation-specific allowed modes, not a universal default. Any division, conversion or instrument tick rounding without a validated policy fails closed.

**Gate:** property tests for equivalent decimals (1.0 == 1.00 numerically), negatives, halfway ties, zero, overflow, tick quantum and forbidden implicit rounding. Existing `rescale=NotImplemented` remains until tested policy implementation exists.

## UD-04 — Version identities

**Decision:** `OrderVersion`, `WriterEpoch` and `SchemaVersion` remain separate `u64` newtypes; overflow fails closed. `WriterEpoch` must be DB-allocated/transactionally fenced at runtime, never inferred from an in-process counter. `SchemaVersion` denotes **major wire contract version**; additive evolution must be explicitly compatible and fixture-tested, not inferred from numeric comparison. `RiskPolicyVersion`, `NormalizerVersion`, `InstrumentDefinitionVersion`, `ValuationModelVersion` remain immutable opaque ASCII identifiers, with exact equality and no implicit ordering. Registry metadata separately declares lineage and compatibility. `PolicyVersion` in generic envelope documentation means `RiskPolicyVersion` when referring to risk; do not silently alias other policy domains.

**Gate:** distinct types; no arithmetic/order on opaque versions; schema compatibility validator; epoch fencing integration test deferred to dispatcher milestone.

## UD-05 — Timestamp semantics

**Decision:** Retain signed `i64` Unix microseconds as canonical in-memory timestamp precision. Explicit wire encoding for events is normalized RFC 3339 UTC with exactly six fractional digits; parse only explicit timezone offsets and convert with checked range/precision, rejecting nonzero sub-microsecond precision. Persist integer microseconds where DB constraints support it. Represent absent source timestamp separately (`Option<OccurredAt>` only where contract allows), and model `ClockQuality` / `TimestampProvenance` as separate metadata, never inferred from the timestamp. `PersistedAt` is assigned by the persistence boundary and is not proof of external broker order. Zero is a real timestamp.

**Gate:** round trips, leap/calendar edge cases, negative epochs, timezone offsets, precision rejection, no timestamp-based broker precedence. Current RFC 3339 `NotImplemented` remains until tests pass.

## UD-06 — State transitions

**Decision:** Freeze enum labels as **draft registry candidates**, not executable transition rules. Do not implement edges or guards until DET-01 transition table, DET-02 reservation transitions, DET-06 readiness gates and DET-07 dispatch outcomes are supplied in repository and cross-reviewed. Unknown wire state fails closed. Do not implement `Ord`/`Default`. `RiskReservationAdjusted` remains an event, not a state. Existing `Cancelled` spelling is retained for draft compatibility; the broker adapter must normalize raw vendor spelling with raw evidence preserved.

**Gate:** explicit transition matrices, illegal transition tests, late-fill replay and model/property tests at M0-03+.

## UD-07 — Missing types

**Decision:** Add `OrderClosure` as an immutable **record** with closure reason, closing `EventId`, closing `OrderVersion`, `OccurredAt`/recorded time semantics and audit reference; do not invent the closed reason enum until DET-01 reason vocabulary is present. Add `IncidentState` only after DET-10 incident transition model is present. `ExecutionState` ownership belongs to execution ledger; add missing glossary ownership row. No fabricated state variants.

**Gate:** type skeleton may exist with a non-instantiable/explicitly incomplete reason representation; never expose incomplete `OrderClosure` as a canonical wire schema.

## UD-08 — Canonical JSON and envelopes

**Decision:** Current serde shapes are **internal draft**. M0-02B defines canonical event/command wire schemas with fixed field names, explicit string encoding for arbitrary-precision coefficient values, stable enum labels, `deny_unknown_fields` for safety-critical messages, bounded payload size, explicit UTC timestamp normalization, and deterministic key ordering / numeric formatting. Canonical bytes must be specified by a documented serializer profile and golden fixtures; do not promise canonical bytes from ordinary `serde_json` map iteration. The event envelope uses CR-B10 §4, with separately documented nullability, schema name, major version and causal identifiers. A payload hash uses an explicitly named hash algorithm and domain separator; no hidden conversion of draft fields.

**Gate:** golden-byte tests, unknown-state tests, round-trip compatibility fixtures, previous-schema decoding/migration tests and reproducibility across clean builds.

## UD-09 — UUID versions

**Decision:** Retain UUIDv4 generation for M0-02A and initial M0-02B. UUIDv7 is optional future ADR, not a migration prerequisite. Accepting arbitrary UUID bits from external input is acceptable only for **typed parsing**, not for validating that an identifier was minted internally; enforce non-nil and permitted provenance at relevant command trust boundaries. UUID sorting never determines event order. No mass migration or rewrite of historical IDs.

**Gate:** uniqueness/provenance boundary tests, non-interchangeability, no reliance on UUID ordering.

## Source-of-truth and delivery requirements

1. Add DET-01..10, CR-B01..B10, this resolution, ADFR and traceability index under `docs/architecture/` or `contracts/` before publishing a schema. Preserve source names, version and draft status; do not claim unprovided contracts have been reviewed.
2. Update `contracts/domain/unresolved-decisions.md` to mark UD-01..UD-09 **DESIGN_RESOLVED / IMPLEMENTATION_PENDING**, preserving the original shell limitations and adding references to this document. For broker uniqueness rules retain **EVIDENCE_PENDING**.
3. Split M0-02B into (a) document/registry skeleton and validator, (b) canonical schemas/fixtures after the needed type adjustments. Never silently serialize provisional aliases as production.
4. Run `cargo fmt --all -- --check`, `cargo check --workspace --locked`, `cargo test --workspace --locked`, `cargo clippy --workspace --all-targets --locked -- -D warnings`; report ignored DB tests and remote CI separately.
5. Preserve fail-closed behavior; no OMS transitions, risk decisions, live trading, broker dispatch, migrations or API behavior changes beyond an explicitly scoped HTTP identifier refactor.
6. Make one commit per independently accepted Cursor prompt; archive the exact prompt in `docs/cursor-prompts/trading/`. Include actual commit SHA in the **post-commit response**, not in a file included in the same commit.

## Readiness / acceptance

**M0-02A:** provisional PASS for shell boundary only, based on reported tests. **M0-02B:** conditional GO for registry scaffolding and explicit type refactoring; NO-GO for publishing broker identity uniqueness claims, transition matrices or financial rounding semantics until their tests/evidence are available. **Live trading:** NOT AUTHORIZED.
