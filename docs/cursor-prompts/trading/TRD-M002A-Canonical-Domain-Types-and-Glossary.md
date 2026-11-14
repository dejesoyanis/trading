# TRD-M002A: Canonical Domain Types & Glossary

**Status:** READY_FOR_CURSOR_EXECUTION (requires user to paste into Cursor)  
**Scope:** M0-02A only · **Trading:** DISABLED · **Date:** 2026-11-14

## Mission
Implement the **minimal, type-safe Rust domain vocabulary** of the private Trading Platform, based on the supplied `CR-B10-Domain-Glossary-and-Schema-Registry-v1.0.md`. Preserve the current foundation and create no broker, OMS, Risk, or order execution behavior. Do not infer the repository state from earlier reports.

## Authoritative documents and conflict handling
Read, in order: (1) actual current repository source and tests, (2) user-provided `CR-B10-Domain-Glossary-and-Schema-Registry-v1.0.md`, (3) DET-01..DET-10 and CR-B01..B09 contracts if supplied, (4) `Implementation-Roadmap-and-Cursor-Prompt-Plan-v1.0.md`, (5) M0-01A foundation hardening report if present. The contract specifies **draft** enum names, not a completed transition model. If a conflict or missing authoritative document prevents safe implementation, STOP, report the exact conflict and propose a versioned ADR; do not silently choose semantics.

## Mandatory preflight (before any edits)
1. Show `git status --short`, branch, recent commit, workspace members, Rust toolchain, current CI and tests; identify pre-existing changes and never overwrite them.
2. Inspect actual `Cargo.toml`, `Cargo.lock`, `crates/trading-common`, other reserved domain crates, `apps/trading-api`, current migrations, `docs/` and CI workflows. Verify whether M0-01A was implemented; its existence as a prompt/report does **not** prove execution.
3. Run `cargo fmt --all -- --check`, `cargo check --workspace --locked`, `cargo test --workspace --locked` where environment permits; record actual failures and prerequisites. Never claim PostgreSQL/Docker integration tests passed unless run.
4. Publish a short implementation plan, proposed files and dependencies. Use existing crate boundaries wherever possible.

## Deliverables
### A. Strongly typed identifiers
- Implement owner-scoped UUID newtypes where appropriate: `OrderId`, `CommandId`, `EventId`, `CorrelationId`, `CausationId`, `BrokerObservationId`, `BrokerSessionId`, `SubmissionAttemptId`, `RiskReservationId`, `RiskEvaluationId`, `ReconciliationRunId`, `IncidentId`, `MarketObservationId`, `MarketSnapshotId`, `ValuationSnapshotId`, `AuditRecordId`, `ExecutionCorrectionId`.
- Add distinct numeric/version newtypes `OrderVersion`, `WriterEpoch`, `SchemaVersion`, `RiskPolicyVersion`, `NormalizerVersion`, `InstrumentDefinitionVersion`, `ValuationModelVersion` where their representation can be specified without guessing. `OrderVersion` and `WriterEpoch` must reject invalid overflows; no assumption that time or UUID ordering proves event order.
- Define structured identity *shapes* for `BrokerAccountKey`, `BrokerOrderAlias`, `BrokerPermIdAlias`, and `ExecutionIdentity` with explicit scope. Avoid assuming IBKR global uniqueness; if precise session/client scope is uncertain, model it explicitly and mark unverified. No identity resolver implementation.
- No implicit conversion between distinct identifiers; use private fields, explicit constructors, `TryFrom` where needed, `Display`/`FromStr` only when unambiguous. Never use a broker ID as an internal OrderId.

### B. Independent domain states
- Define distinct enums from CR-B10 §2 for `OrderWorkflowState`, `BrokerLifecycleState`, `ExecutionState`, `RiskReservationState`, `TradingReadinessState`, `ReconciliationRunState`, `RiskDecision`, `DispatchOutcome`.
- Do **not** implement transitions, policy evaluation, readiness decisions, or external broker actions. Do not equate workflow `Closed` with economic finality; do not equate HTTP readiness with trading readiness.
- Unknown externally decoded safety-critical variants must return a typed error, never silently default to `Ready`, `Allow`, `Released` or `ConfirmedAccepted`.
- `RiskReservationAdjusted` is a future event concept, not a reservation state.

### C. Money, quantities, time and provenance
- Define safe typed shells or narrow constructors for `Money`, `Price`, `Quantity`, `InstrumentId`, `CurrencyCode`, `OccurredAt`, `ReceivedAt`, `PersistedAt`. Use checked decimal arithmetic and explicit currency/instrument/version association; no f32/f64 for authoritative financial amounts.
- Do not invent tick sizes, FX conversion, rounding policy, trading calendars or valuation logic. Where necessary expose validated metadata and `NotImplemented`/typed error, rather than fake correctness.
- Keep serialization of authoritative values explicit and deterministic where supported; no promises about canonical JSON bytes before M0-02B.

### D. Documentation
- Add `contracts/domain/glossary.md` covering every type: owner, meaning, invariants, and distinction from neighboring types. Add a traceability table to CR-B10 sections and DET contracts.
- Document unresolved design decisions separately, especially broker alias scoping, exact decimal representation and state transitions. Do not mark draft semantics as production-proven.

## Implementation strategy
1. Implement IDs and scoped identity types in the existing common/domain crate with minimal dependencies.
2. Implement state enums in owning modules or clearly separated domain submodules; avoid a giant shared enum or cyclic crate dependencies.
3. Implement safe financial/time wrappers with tests and without business logic.
4. Add documentation, rustdoc, compile-fail tests where practical, and unit tests.
5. Run quality checks; review diff for scope creep and credential leakage.

## Acceptance gates
- A01: All defined identifiers are non-interchangeable at compile time; negative compile tests or documented equivalent prove this.
- A02: Scoped broker aliases with equal raw IDs in different accounts/sessions are not equal when scopes differ.
- A03: Unknown safety-critical states fail closed on deserialization.
- A04: `Closed` and `Released` are distinct type families; no implicit transition is encoded.
- A05: Currency and instrument mismatches reject invalid arithmetic/combination; checked overflow tests.
- A06: No authoritative money/quantity/price representation uses floating-point.
- A07: Glossary has ownership and traceability for all introduced types.
- A08: `cargo fmt`, `cargo check --workspace --locked`, `cargo test --workspace --locked`, and `cargo clippy --workspace --all-targets --locked -- -D warnings` pass, or failures are honestly reported; no hidden skips.
- A09: No broker network access, order submission, migration side effects or live/paper trading logic introduced.
- A10: Existing health/readiness and infrastructure behavior preserved; tests remain green.

**Deferred to M0-02B:** B10-T01..T04 and T06/T11/T12 registry/compatibility gates; **deferred to M0-03+** B10-T07/T08/T10 economic replay and state transition tests. Never claim all B10 tests passed in M002A.

## Constraints
No live/paper broker calls, credentials, automatic pushes, destructive Git commands, DB schema changes unless absolutely necessary (stop and request approval), unrelated refactors, new trading features, agent/ML logic, or bypassing CI. Do not change contract semantics to make tests pass. Do not fabricate successful checks.

## Required report
Write `docs/reports/TRD-M002A-implementation-report.md` containing: repository preflight and base SHA; changes/files; key type decisions; test commands and exact outcomes; tests not run; mapping A01..A10; deviations/ADRs; security assessment; remaining M002B prerequisites; final commit SHA. Archive this entire prompt verbatim at `docs/cursor-prompts/trading/TRD-M002A-Canonical-Domain-Types-and-Glossary.md` before committing. Commit once with title **`TRD-M002A: Canonical Domain Types & Glossary`** after successful checks. If mandatory checks fail, do not present the milestone as complete; preserve work safely and report blockers. Never push without explicit approval.
