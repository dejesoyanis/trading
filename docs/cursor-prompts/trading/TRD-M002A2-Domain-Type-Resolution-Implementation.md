# TRD-M002A.2 — Domain Type Resolution Implementation

**Status:** Implementation prompt; user-authorized development, NOT live-trading authorization  
**Milestone:** M0-02A.2 (between M0-02A and M0-02B)  
**Target commit subject:** `TRD-M002A.2: Implement Resolved Domain Type Contracts`  
**Baseline reported by Cursor:** `d0398bcc8710a36659bcf1d0240cb43112e61f2c` (verify locally; do not assume current HEAD)  
**Required source:** `contracts/sources/M0-02A-UD-01-bis-UD-09-Architecture-Resolution-v1.0.md` (locate actual path), CR-B10 draft, `contracts/domain/unresolved-decisions.md`, `contracts/traceability-index.md`.

## 0. Operating contract

You are implementing a narrowly scoped, safety-critical Rust domain type refactor for a private trading platform. **Inspect the actual repository first.** Do not implement OMS transitions, broker submission, risk evaluation, execution, financial valuation, database migrations or live trading. Do not publish canonical schemas or golden bytes. Fail closed on ambiguous, unsupported or unverified semantics. Do not infer an absent DET contract from a summary. Preserve compatibility of existing HTTP health/readiness behavior.

Treat UD-01..09 as `DESIGN_RESOLVED / IMPLEMENTATION_PENDING`, not as already implemented. Broker identity verification remains `EVIDENCE_PENDING`. If required source documents are absent, record the absence and implement only work not depending on them; do not invent contents. If source conflict is found, STOP the affected subtask, report the exact conflicting passages and continue independent safe subtasks.

## 1. Preflight and source provenance

1. Print branch, `git status --short`, HEAD SHA and subject, toolchain, workspace crates, source paths, CI status and test baseline. Never overwrite unrelated changes. If tree is dirty, inspect and report; stop before touching affected files.
2. Verify the recorded commit `d0398bcc...` is in history or explicitly report divergence. Read the resolution document **in the repository**, not from the prompt summary alone.
3. Inventory the existing `trading_common::CorrelationId`, `trading_domain::CorrelationId`, `DecimalAmount`, `Money`, `Price`, `Quantity`, time types, alias scope types, state enums, serde implementations, registry skeleton and tests. Produce a change map with public API implications.
4. Check `contracts/traceability-index.md`. DET-01..10 / CR-B01..09 marked `NOT_PROVIDED / NOT_REVIEWED` must stay that way unless the actual source is supplied and reviewed. A file merely named in a resolution is not proof of availability.

## 2. Implementation workstreams

### A — UD-01: distinguish HTTP request and domain correlation

- Introduce `trading_common::HttpRequestId` as the HTTP-local request identifier; keep `trading_domain::CorrelationId` unchanged and distinct.
- Migrate internal HTTP call sites and tests to `HttpRequestId`. Maintain the existing HTTP header/body wire format and behavior. If compatibility requires an old name, use a narrowly scoped, documented, deprecated alias only; **do not export both ambiguously in public schema docs**.
- Do not add implicit conversions to domain correlation or pretend HTTP request ID is a distributed span.
- Add compile-time non-interchangeability tests and regression tests for request ID header/serialization.

### B — UD-03: checked numeric decimal semantics

- Preserve the bounded `i128` coefficient and scale `0..=38` storage primitive; reject invalid construction and overflow.
- Introduce an explicit numeric equality/comparison API that recognizes mathematically equivalent values such as `1.0` and `1.00`, including signed values, **without overflow**. Choose a checked normalization/comparison algorithm; do not use floats. If deriving `Eq`/`Hash` on numeric semantics, ensure hash equivalence; otherwise retain representation equality under an explicitly named API and expose separate `numeric_eq` / `numeric_cmp` with documented semantics. Never silently change existing structural equality without tests and impact review.
- Add the `RoundingPolicy` enum: `RejectInexact`, `TowardZero`, `AwayFromZero`, `HalfEven`, `Floor`, `Ceiling`. This is a **policy vocabulary**, not permission to round arbitrary economic values.
- Implement only exact, validated operations whose invariants can be proven and tested. `rescale`, division, FX, tick quantization and conversion **must continue returning explicit `NotImplemented`/typed errors** unless a target quantum, operation-specific authorization, provenance and complete tests exist. Do not make an implicit default rounding mode in economic operations.
- Keep `Money`, `Price`, `Quantity` currency/instrument/definition-version boundaries. No floating-point authoritative values.
- Add unit and property-based tests for equivalent scales, zero, negatives, `i128::MIN/MAX`, scale bounds, overflow, half-even ties **if rounding is implemented**, and cross-currency/instrument rejection. Avoid asserting unsupported tick behavior.

### C — UD-05: timestamp wire conversions

- Preserve `OccurredAt`, `ReceivedAt`, `PersistedAt` as separate newtypes over signed Unix microseconds.
- Add explicit, **opt-in** RFC 3339 UTC wire codec with exactly six fractional digits. Accept explicit timezone offsets; normalize to UTC; reject malformed timestamps, overflow, invalid dates and nonzero sub-microsecond precision. Do not silently truncate. Do not globally replace draft serde JSON shapes until M0-02B's envelope is approved.
- Represent absent source timestamps explicitly only where relevant (do not conflate zero with missing). `ClockQuality`/`TimestampProvenance` may be added as non-authoritative metadata vocabulary if supported by the resolution; do not invent broker clock guarantees.
- Tests: pre-1970 timestamps, leap days, offsets crossing midnight, `1970-01-01T00:00:00.000000Z`, boundary overflow, fractional precision and round-trip stability. No inference of event precedence from timestamps.

### D — UD-02: explicit broker client context, still unverified

- Introduce a **distinct** broker client identifier type, not an alias for internal `BrokerSessionId`. Represent the resolution's `BrokerIdentityContext` components (broker, account, environment, client ID if available, connection/session, adapter session epoch, alias kind and evidence/provenance) with validated fields and documented optionality.
- Retain `UnverifiedSessionScope` and `SKELETON_NOT_CANONICAL`. Do **not** create a verified scope constructor, identity resolver, automatic alias merge or broker uniqueness guarantee. Do not claim that all fields are always part of one key: per-kind scope rules require broker evidence.
- Add negative/collision tests: identical external raw IDs across accounts, environments, clients and sessions; missing client context; order alias vs perm alias; ambiguous evidence. Reject any attempt to deserialize `verified` or promote quarantined identities.
- Preserve raw source IDs and evidence provenance. No IBKR network calls.

### E — UD-04, UD-06, UD-07, UD-08, UD-09: maintain boundaries

- Keep `OrderVersion`, `WriterEpoch`, `SchemaVersion` distinct, overflow-checked. Opaque policy/normalizer/instrument/valuation versions remain non-orderable. No DB epoch allocation in this milestone.
- Do not implement state transition edges, defaults, or permissive unknown-state decoding. Do not invent `IncidentState` variants.
- `OrderClosure`: **only** add a safe internal skeleton if DET-01's exact reason vocabulary and evidence model are available in the repository and cross-reviewed. Otherwise keep the type deferred and document the dependency; do not invent a reason enum or publish a wire shape.
- Current serde payloads remain draft. Keep registry status `SKELETON_NOT_CANONICAL`; no event/command schema directories, canonical serializer, golden bytes or compatibility claim.
- Retain UUIDv4 generation; typed parsing is not minting/provenance validation. Test nil UUID rejection at **trusted command boundaries** where such boundaries actually exist; do not globally reject historical UUID bits without a contract.

## 3. Required safety invariants

- Different domain identifiers cannot be implicitly interchanged.
- Numeric decimal comparison is mathematical, overflow-safe and independent of representation; economic operations never round implicitly.
- Unknown state labels cannot become permissive states.
- Broker alias equality does not prove identity or broker uniqueness; ambiguity remains quarantined.
- Wire timestamp codec never truncates sub-microsecond information or assumes event ordering.
- No canonical schema is published while unresolved dependent contracts or tests remain.
- No order send, risk decision, OMS transition, new migration or trading capability is introduced.

## 4. Test strategy and acceptance gates

| Gate | Acceptance criterion |
| --- | --- |
| A01 | Baseline provenance, clean-tree safety and dependency map documented |
| A02 | HTTP identifier rename compiles; HTTP behavior and request ID formatting unchanged |
| A03 | Compile-fail tests prevent HTTP/domain correlation interchange |
| A04 | Numeric decimal equivalence, ordering and overflow tests pass without floats |
| A05 | RoundingPolicy vocabulary exists; forbidden rounding/rescale/FX still fail closed |
| A06 | UTC six-digit RFC 3339 codec passes positive, negative and edge-case fixtures |
| A07 | Client ID and internal broker session ID are distinct; ambiguous aliases remain unverified |
| A08 | No invented transitions, closure reasons, incident states, broker uniqueness or schema publication |
| A09 | Registry skeleton still rejects canonical schema publication and verified identity claims |
| A10 | Existing API health/readiness, persistence unit tests and workspace checks pass |
| A11 | All ignored integration tests, remote CI and unverified evidence are explicitly reported |
| A12 | Exact prompt archived; report created; commit subject matches; final SHA reported after commit |

Run and report actual exit codes:

```bash
cargo fmt --all -- --check
cargo check --workspace --locked
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
```

If introducing a property-test dependency, justify the lockfile delta and deterministic seed/reproducibility. Do not silently enable integration tests against an unknown database. If PostgreSQL integration is skipped, explicitly say **ignored, not passed**. Remote GitHub Actions are **not run** unless actually executed.

## 5. Documentation, traceability and reporting

Update `contracts/domain/glossary.md`, `contracts/domain/unresolved-decisions.md`, `contracts/traceability-index.md`, and `contracts/registry/registry.json` **only as necessary**, preserving evidence/status distinctions. For each UD-01..09, record `DESIGN_RESOLVED`, `IMPLEMENTED_AND_LOCALLY_TESTED` or `IMPLEMENTATION_PENDING` individually; `EVIDENCE_PENDING` stays on broker uniqueness. Do not mark a UD fully implemented when only a subset is delivered.

Create `docs/implementation/TRD-M002A2-report.md` (or existing convention) with: baseline SHA, source inventory, implementation strategy, changed files, precise behavioral changes, test commands and counts, skipped tests, unresolved gaps, security review, deviations, per-gate PASS/PARTIAL/FAIL, and next-step recommendation. Archive this **exact prompt** as `docs/cursor-prompts/trading/TRD-M002A2-Domain-Type-Resolution-Implementation.md` before committing.

If any gate fails, report **PARTIAL/FAIL**, do not claim milestone acceptance, and do not publish schemas. Preserve unrelated changes. Make **one commit** for this accepted prompt with subject:

`TRD-M002A.2: Implement Resolved Domain Type Contracts`

Report actual `git rev-parse HEAD` **after** commit; do not insert self-referential SHA into committed report. Do not push without explicit permission.

## 6. Handoff to M0-02B

M0-02B canonical publication remains **NO-GO** until type adjustments are locally verified, the relevant authoritative DET/CR sources are supplied and reconciled, and canonical wire/compatibility gates are separately implemented. A passing M0-02A.2 is not a live-trading authorization.
