# Cursor Execution Prompt — TRD-ARCH-03B.3
## Broker Identity, Snapshot Completeness & Unknown-Send Recovery — DESIGN PROPOSALS ONLY

**Date:** 2026-11-15  
**Authority:** User explicitly approved starting the next *design-proposal* milestone. This is NOT authorization to implement, ratify proposed choices, change canonical contracts, dispatch broker requests, or enable trading.  
**Expected result:** A reviewable, source-anchored set of technical alternatives and independent decision questions for subsequent explicit user ratification.

---

## 0. Non-negotiable execution protocol

1. Work in the existing Rust trading-platform repository. Do not assume its path; locate the repository root using `git rev-parse --show-toplevel` and verify `Cargo.toml`, `contracts/`, `docs/`.
2. Preflight: `git status --short`, `git branch --show-current`, `git rev-parse HEAD`, `git log -1 --format=%s`, `git status -sb`. If dirty, **STOP** and report; never stash/reset/clean/discard user work.
3. Verify predecessor `TRD-ARCH-03B.2.2` report, archived prompt, and `contracts/decisions/ARCH-DEC-21-bis-34-2026-11-15.md` exist in HEAD. If not, STOP and explain. Capture exact baseline SHA and hashes; do not assume a SHA from this prompt.
4. Archive this exact prompt under `docs/cursor-prompts/trading/TRD-ARCH-03B3-Broker-Identity-and-Snapshot-Completeness-Design-Resolution.md` without overwriting an unrelated file. If already present, check exact content and report.
5. Complete ONLY the documentation/design-proposal work below. Do not silently ratify alternatives or close open findings.
6. Execute the validation suite and write a dated final report. Make **exactly one local Git commit** containing this milestone's authorized changes, with subject `TRD-ARCH-03B.3: Draft Broker Identity and Snapshot Completeness Decisions`. **DO NOT PUSH.** If gates fail, report `BLOCKED`/`FAIL` and do not claim success; do not commit a false PASS. Never amend prior commits or use destructive Git operations.

## 1. Binding decisions and hierarchy

Read the actual repository sources; do not reconstruct missing files from memory. In particular inspect:

- `contracts/source-manifest.json`, all 12 manifest-listed sources, source integrity metadata and provenance, especially DET-01/DET-02 `RECONSTRUCTED_DRAFT` with exceptions.
- `contracts/decisions/ARCH-DEC-01-bis-03-2026-11-15.md`, `ARCH-DEC-04-bis-08-2026-11-15.md`, `ARCH-DEC-09-bis-20-2026-11-15.md`, `ARCH-DEC-21-bis-34-2026-11-15.md`.
- `docs/architecture/TRD-ARCH-SYNC-01-report.md`, `TRD-ARCH-SYNC-02-report.md`, `TRD-ARCH-03A-report.md`, `TRD-ARCH-03A.2-report.md`, `TRD-ARCH-03B.1-report.md`, `TRD-ARCH-03B.1.2-report.md`, `TRD-ARCH-03B.2-report.md`, `TRD-ARCH-03B.2.2-report.md` if present.
- `contracts/design-proposals/TRD-ARCH-03B1-*`, `TRD-ARCH-03B12-resolution-index.json`, `TRD-ARCH-03B2-*`, `TRD-ARCH-03B22-resolution-index.json` and existing proposal/decision traceability.
- DET-03 through DET-10, CR-B01 through CR-B10, broker/IBKR/readiness/reconciliation references, `contracts/domain/unresolved-decisions.md`, `contracts/traceability-index.md`, `contracts/domain/glossary.md`, `contracts/registry/registry.json`, ADFR JSON and CSV.
- Existing Rust broker identity, order identity, evidence, connection-state and readiness *shells* for descriptive alignment only. **Do not edit production sources.**

Preserve all binding policies, especially:

- ARCH-DEC-03: CR-B01 single-writer/epoch-fencing precedence. An unknown in-flight send or a prior writer that may still send prevents automatic takeover; lease expiry alone does not prove safety. No automatic second send.
- ARCH-DEC-09..20: one historical closure record, separate economic facts, fail-closed unknown sends, broker terminal observations are not accepted as authoritative terminal, all economic effects quarantined, no economic-finality predicate or risk release, identical-payload replay is not authorization, no canonical lock order/isolation, distinct transactional audit record without table name, modify and implicit cancel-replace blocked.
- ARCH-DEC-21..34: hard-deny conjunction shape only; one reservation identity and all-or-nothing multi-scope design; missing/stale price, liquidity, FX prevents reservation; partial fill/cancel does not reduce holds; late facts quarantined; no broker risk-reduction; no finality/release proof adopted; valuation citation obligations only; future concurrency/crash tests required but not executed; distinct unnamed audit; no invented approver; immutable future policy versions and negative tests.
- `SKELETON_NOT_CANONICAL`, `canonical_schemas_published=false`, `part_b=NO-GO`, `live_trading_authorized=false`, `broker_uniqueness_evidence=EVIDENCE_PENDING`; all 78 historical ADFR IDs `NOT_VERIFIED`, exactly one supersession `ADFR-CROSS-005 -> ADFR-CR-B10-001`.
- No precedence among disputed status words (XC-11). `DESIGN_RESOLVED` does not imply verified/implemented. Documentation tests are not broker evidence.

If a source conflicts with a binding user decision, cite both, preserve the conflict and do not silently choose the source. Never invent IBKR/API behavior, completeness guarantees, orderId/permId uniqueness, sequence numbers, execution IDs, broker terminality, timeout guarantees, or evidence URLs. External evidence not actually obtained must be labeled `EXTERNAL_EVIDENCE_PENDING`.

## 2. Required investigation and scope

Prepare a **non-runtime, non-wire** architecture proposal covering at least:

### A. Broker identity and matching (XC-01)
- Distinguish platform `OrderId` UUID, command/idempotency key, `RiskReservationId`, broker/account/client/session context, broker orderId, permId, execution/fill identity and correlation identifiers. Preserve their different semantics.
- Analyze scoped identity matching across reconnects, account/client changes, session boundaries, duplicate/reused broker IDs, missing permId, late assignment of permId, execution duplicates, correction/bust chains and out-of-order evidence.
- Define **candidate** evidence classes and matching confidence/ambiguity states; distinguish observed evidence from inference. A match, even unique-looking, is not permission to post or release.
- Compare at least three identity strategies (e.g. strict scoped mapping; evidence-graph/candidate sets; optimistic mapping **contrast only**). Describe failure modes, idempotency, audit and manual adjudication dependencies.
- No schema, uniqueness index, migration or canonical broker identity tuple is approved here.

### B. Broker snapshot completeness (XC-08-IBKR)
- Distinguish snapshot coverage by account, client/session, order lifecycle, open orders, executions, corrections, time interval, reconnect boundary, event ordering and authoritative source. Do not assume one broker response is a complete global snapshot.
- Compare incremental stream vs snapshot vs reconciliation evidence; identify absent, delayed, duplicated, contradictory, partially covered and out-of-order observations.
- Draft a *non-executable* completeness proof-obligation model and 3+ options. An empty response, timeout, cancel label, filled label, disconnect, or lease expiry is not proof of non-execution or economic finality.
- Record exact missing external broker evidence and a future verification plan (official API docs, reproducible test cases, environment, broker versions, timestamps, negative cases); mark all external cases `NOT_EXECUTED` / `EXTERNAL_EVIDENCE_PENDING` unless actually proven. Do not fabricate citations.

### C. Unknown sends, recovery and fencing (XC-04)
- Analyze send-before/after-DB-commit crash windows, lost ACK, broker accepted but local unknown, delayed permId, reconnect, previous writer still active, epoch changes, and conflicting broker observations.
- Maintain single-writer dispatch and epoch-fencing as binding *design*, without claiming operational proof. Explicitly distinguish DB lease/epoch from real-world broker acceptance.
- No automatic retry/resubmission of uncertain sends; no failover solely because lease expired; no blind reconciliation-based cancellation; no broker calls or network access.
- Compare recovery decision strategies; make manual escalation and proof obligations explicit without inventing roles or authorization.

### D. Cross-contract safety
- Preserve XC-02-PRED OPEN (economic finality), XC-03-MATRIX OPEN (OMS transition matrix), XC-05-LOCK OPEN (locking/isolation), XC-06 OPEN (audit artifact), XC-07-ROUND OPEN, XC-08-IBKR OPEN, XC-09-RESET OPEN, XC-10-ACTION OPEN, XC-11 OPEN. Note existing CR-B01 precedence but operational XC-04 still open.
- Explain how evidence is retained append-only, provenance/correlation is preserved, conflicting facts quarantined, duplicates idempotently observed, and replay cannot create an economic effect or a second send.
- Show how partial fills, cancel requests/observations, late fills, corrections and busts affect **evidence tracking only**. No ledger posting, reservation conversion or release.
- Maintain no implicit cancel-replace, modify, risk-reducing order or broker terminal acceptance.

## 3. Required artifacts

Create these paths under `contracts/design-proposals/` (names exact unless already occupied; stop rather than overwrite unrelated content):

1. `TRD-ARCH-03B3-broker-identity-semantics-and-invariants.md`: identity namespaces, trust boundaries, evidence provenance, ambiguity, uniqueness proof obligations, source anchors, failure modes and explicit non-authorizations.
2. `TRD-ARCH-03B3-snapshot-completeness-and-reconciliation-coverage-draft.md`: coverage model and **at least 24** scenario rows, including reconnect, missing executions, empty snapshot, reused ID, late permId, unknown send, prior writer, split brain, partial fill, cancel race, late fill, correction/bust, duplicate, stale snapshot, incomplete time window, and out-of-order updates. Each row must explicitly mark `EVIDENCE_RETAINED`, `BOOKING_PROHIBITED`, `RELEASE_PROHIBITED`, `RESUBMIT_PROHIBITED`, `BROKER_TERMINAL_ACCEPTANCE_PROHIBITED`, and `ORACLE_NOT_EXECUTED` unless independently justified by a previously binding restriction (never loosen restrictions).
3. `TRD-ARCH-03B3-identity-snapshot-and-recovery-options.md`: at least three clearly labeled approaches per major axis (identity, completeness, recovery), including safe fail-closed baseline, future evidence-rich candidate, and a conflict/unsafe contrast. Explain safety, complexity, recoverability, auditability, testability, and blockers.
4. `TRD-ARCH-03B3-approval-requests.md`: **at least 12 independent numbered questions `BROKERQ-01..`**, each with A/B/C (or more) meaningful alternatives, source anchors, binding constraints, open unknowns, dependency/impact matrix, non-binding recommendation, and evidence requirements. Mark `USER_DECISION_PENDING` and `NOT_RATIFIED` throughout. Any alternative conflicting with binding policy must be `CONFLICTS_WITH_BINDING_POLICY` and cannot be ratified without explicit supersession. Include questions addressing identity tuple, ambiguity, missing permId, duplicate executions, correction chain, snapshot scope, completeness proof, gap detection, reconnect, unknown send, epoch fencing, manual escalation, evidence authority and future verification.
5. `TRD-ARCH-03B3-dependency-and-conflict-matrix.md`: XC-by-XC map, old/new design decisions, upstream/downstream blockers, handoff to economic-finality and OMS transition work without authorizing it.
6. `TRD-ARCH-03B3-proposal-index.json`: clearly `NON_RUNTIME_NON_WIRE_PROPOSAL_INDEX`, `NOT_RATIFIED`, no canonical schema, all trading/posting/release/transition/dispatch/verification flags false. JSON for documentation checks only.

Update navigation and unresolved-decision pointers in `contracts/design-proposals/README.md`, `contracts/domain/unresolved-decisions.md`, `contracts/traceability-index.md`, `docs/architecture/README.md`, and `tests/README.md` **only as needed**, keeping prior ratifications intact. Do not edit the earlier proposal indexes or resolution indexes. Do not edit historical approval question texts. Do not rewrite the original sources.

## 4. Acceptance gates — A03B3-G01..G12

- **G01 — Preflight and predecessor:** clean worktree, exact baseline SHA, archived predecessor and ARCH-DEC-21..34 found; record tool versions and current branch.
- **G02 — Source integrity:** validate all 12 source files against manifest `sha256` and `bytes` before/after; preserve reconstructed provenance and package intake metadata; never modify sources/manifest.
- **G03 — ADFR/registry integrity:** exactly 78 JSON/CSV matching IDs/order, 78 `NOT_VERIFIED`, exactly one supersession, registry SHA unchanged and all safety flags unchanged; preserve historical decision files.
- **G04 — Binding-policy traceability:** all ARCH-DEC-01..34 retained; CR-B01 precedence explicit; no fabricated approvals, evidence, authority or status promotions.
- **G05 — Identity:** all identity namespaces and ambiguity cases covered with source anchors; no unproven uniqueness or canonical tuple.
- **G06 — Snapshot:** 24+ coverage rows, no invented completeness guarantees, all economic and dispatch prohibitions explicit, external evidence clearly pending.
- **G07 — Unknown send/fencing:** crash windows, lost ACK, reconnect, prior writer and split-brain included; no automatic takeover/resend/cancel/dispatch.
- **G08 — Decision questions:** 12+ independent `BROKERQ` items with A/B/C, impact, recommendation, dependencies and pending user ratification.
- **G09 — Open XC map:** XC-01 and XC-08-IBKR remain OPEN; XC-04 precedence noted but operational portion OPEN; XC-02-PRED and other blockers preserved.
- **G10 — No implementation:** no `crates/*/src`, `apps/*/src`, Cargo metadata, migrations, schemas, broker calls, flags, runtime, trading, risk release, economic posting or OMS transitions modified; no external live/paper broker requests.
- **G11 — Tests and tooling:** documentation-only regression checks for proposal index and all forbidden flags, plus `cargo fmt --all -- --check`, `cargo check --workspace --locked`, `cargo test --workspace --locked`, `cargo clippy --workspace --all-targets --locked -- -D warnings`; report first failures and reruns. Explicitly distinguish ignored DB tests from passes; remote CI not run unless actually run. No production test assertions asserting unverified broker behavior.
- **G12 — Report and commit:** write `docs/architecture/TRD-ARCH-03B.3-report.md` with gate-by-gate PASS/FAIL, changed files, relevant hashes, preflight baseline, test counts, ignored tests, outstanding unknowns, source anchors, deviations, explicit no-push, and the exact one-commit result. Commit subject as specified. Do not claim that the report contains its own final commit SHA; print the SHA after committing in Cursor output.

## 5. Validation implementation boundaries

Documentation-only tests may extend existing `crates/trading-domain/tests/adfr_reconciliation.rs` and `registry_skeleton.rs` only to enforce metadata, source protection, no ratification, forbidden flags and traceability. Keep tests deterministic and local. If existing allowlists need extension, make the smallest scoped update and explain it. Do not introduce executable broker reconciliation, API mocks that assert real IBKR guarantees, financial posting, reservation release, new domain types, runtime logic or migrations.

Report `PASS` only if all gates are supported by concrete local evidence. If a missing source or conflict prevents safe completion, stop and report `BLOCKED`, list what is needed, and do not fabricate content.

## 6. Required final Cursor response

Give a concise German summary plus:

- `VERDICT: PASS | FAIL | BLOCKED`
- baseline HEAD, new HEAD SHA, branch, clean/dirty worktree
- precise artifact paths and number of `BROKERQ` questions and coverage rows
- 12 gates, checks and counts, ignored integration tests and remote CI status
- all open XC findings, unverified broker assumptions, and evidence gaps
- `IMPLEMENTATION_AUTHORIZED: false`, `BROKER_DISPATCH_AUTHORIZED: false`, `ECONOMIC_POSTING_AUTHORIZED: false`, `RISK_RELEASE_AUTHORIZED: false`, `ECONOMIC_FINALITY_PREDICATE_APPROVED: false`, `LIVE_TRADING_AUTHORIZED: false`, `PUSHED: NO`
- one local commit subject and SHA; no push.

**STOP at design proposals. The user will separately review BROKERQ decisions and authorize any subsequent ratification milestone.**
