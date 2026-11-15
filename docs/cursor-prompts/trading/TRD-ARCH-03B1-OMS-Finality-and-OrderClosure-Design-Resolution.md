# TRD-ARCH-03B.1 — OMS Finality, OrderClosure & Transition-Matrix Design Resolution

**Mode:** DESIGN / DOCUMENTATION / DOCUMENTATION-ONLY TESTS. **Not** an implementation authorization.
**Date:** 2026-11-15. **Owner of approvals:** User, never Cursor.
**Prerequisite:** TRD-ARCH-03A.2 reported PASS; confirm actual Git state and committed report before proceeding.

## 0. Objective and non-negotiable boundaries

Produce an evidence-grounded, reviewable **decision package** for the remaining OMS finality and transition questions, primarily `XC-03-MATRIX`, without inventing user approval. The result is a rigorous proposal for subsequent user decisions, **not** a ratified transition matrix, `OrderClosure` vocabulary, schema, executable state machine, or permission to submit orders.

**Prohibited:** editing `crates/*/src`, `apps/*/src`, migrations, runtime flags, API/command/event/wire schemas, canonical registry, ADFR historical JSON/CSV, the 12 architecture source files, source manifest, live-trading settings, broker dispatch or risk postings. No canonical enum, DDL, new runtime dependency, trading behavior, CI push, or deployment. Keep `SKELETON_NOT_CANONICAL`, `canonical_schemas_published=false`, `part_b=NO-GO`, `live_trading_authorized=false`, `broker_uniqueness_evidence=EVIDENCE_PENDING`. CR-B01 precedence over DET-07 remains in force. Never infer broker-send failure from a timeout, nor economic finality from a cancel ACK or an incomplete snapshot.

## 1. Preflight and source hierarchy

1. Confirm branch, `git status --short` clean, baseline HEAD, preceding TRD-ARCH-03A.2 commit subject/SHA, report, prompt archive. **STOP if dirty or predecessor report/commit missing**; do not stash/reset/discard.
2. Read actual repository files, not just this prompt: `docs/architecture/TRD-ARCH-SYNC-01-report.md`, `TRD-ARCH-SYNC-02-report.md`, `TRD-ARCH-03A-report.md`, `TRD-ARCH-03A.2-report.md`; `contracts/decisions/ARCH-DEC-01-bis-03-2026-11-15.md`, `ARCH-DEC-04-bis-08-2026-11-15.md`; `contracts/sources/DET-01-Order-Workflow-and-Finality-Contract-v1.0.md`, `DET-02-Risk-Contract-v1.0.md`, `DET-03-bis-DET-08-Architecture-Contracts-v1.0.md` (especially DET-03/04/06/07/08), `DET-09-bis-DET-10-und-Cross-Contract-Review-v1.0.md`, `P0-Resolution-Contracts-CR-B01-bis-CR-B04-v1.0.md`, `P1-Resolution-Contracts-CR-B05-bis-CR-B09-v1.0.md`, `CR-B10-Domain-Glossary-and-Schema-Registry-v1.0.md`, `Architecture-Consolidation-and-Acceptance-v1.0.md`; `contracts/domain/unresolved-decisions.md`, `contracts/domain/glossary.md`, `contracts/traceability-index.md`, `contracts/status-model/*`, `contracts/source-manifest.json`, ADFR JSON/CSV, `contracts/registry/registry.json`, relevant documentation tests.
3. Validate the 12 current manifest hashes/byte lengths; preserve distinct intake `package_sha256`/`package_bytes`. Verify 78 ADFR IDs in JSON/CSV, identical core fields and order, 78 `NOT_VERIFIED`, one supersession `ADFR-CROSS-005 -> ADFR-CR-B10-001`, 77 not superseded; unchanged registry hash and flags. Record exact baseline hashes. **STOP on discrepancies.**
4. Respect authority: explicit ARCH-DEC-01..08 and confirmed DET-01 Q01..Q10 principles are binding *within their recorded scope*. DET-01/02 remain reconstructed provenance with open details. Treat examples, reference models, and proposed tokens as non-ratified. For contradictions, identify source, clause, consequence and unresolved decision; do not silently choose. ARCH-DEC-04 explicitly rejects implicit precedence among `OPEN P0`, `PROPOSED_FOR_ACCEPTANCE`, `DESIGN_RESOLVED`.

## 2. Deliverable A — OMS semantic decomposition

Create `contracts/design-proposals/TRD-ARCH-03B1-OMS-semantics-and-invariants.md` with traceable, explicitly labeled **BINDING_EXISTING**, **PROPOSED**, **OPEN**, **DEPENDENCY**, **UNVERIFIED** sections:

- Independent dimensions: command/workflow state; broker lifecycle and raw evidence; execution ledger/economic projection; risk reservation reference; historical closure reason/evidence. Distinguish workflow closure, broker terminal observation, economic finality, risk-release eligibility, and dispatch outcome. Do not collapse dimensions.
- Distinguish `OrderVersion` (monotone aggregate), `WriterEpoch` (dispatch fencing), opaque broker identifiers, persisted command/event/evidence IDs; no implied cross-domain equivalence.
- Historical closure immutability vs late fill, duplicate fill, partial fill, bust, correction, cancel/fill race; append-only evidence and corrections; idempotent economic effect; incident/halt when reservation/budget insufficient.
- Commands vs observations vs derived projections vs administrative actions. For each, specify trusted evidence prerequisites, replay/idempotency expectations, allowed conceptual effects and what remains prohibited or unknown.
- Conservative trust boundary: unknown aliases, incomplete snapshots, missing broker uniqueness evidence, stale versions, unknown submission outcomes, conflicting evidence, no silent re-open, no blind resend, no risk release based on request/timeout alone.
- Transactional constraints: local PostgreSQL unit-of-work for order/command/evidence/audit/outbox and applicable risk/ledger effects; no broker/NATS call inside transaction; CAS plus locks subject to unresolved global lock order. Do not invent DDL or claim atomicity is implemented.
- Explicit dependencies on `XC-01`, `XC-02-PRED`, `XC-04`, `XC-05-LOCK`, `XC-06`, `XC-08-IBKR`, `XC-09-RESET`, `XC-10-ACTION`; describe why an OMS-only answer cannot close them.

## 3. Deliverable B — Candidate transition coverage, NOT an approved matrix

Create `contracts/design-proposals/TRD-ARCH-03B1-transition-coverage-draft.md`:

1. Inventory **only** workflow/state labels actually present in the repository, with file/section citations and confidence. Do not manufacture a definitive enum or serialize any candidate labels as canonical.
2. Define a conceptual transition-record template: `source_dimension`, `current_label`, `trigger_kind`, `command_or_evidence`, `trust_and_authorization_guards`, `expected_version`, `risk_precondition`, `candidate_next_label`, `ledger_effect`, `reservation_effect`, `audit_and_outbox_effect`, `replay_behavior`, `rejection_or_quarantine`, `recovery_behavior`, `source_reference`, `approval_status`.
3. Build a **candidate coverage matrix** of scenario families, not asserted permitted edges: create, validate, risk reserve, submit preparation, dispatch started, broker ACK/reject, partial/full fill, cancel request/confirm, modify request/confirm, timeout/outcome unknown, disconnect/restart, reconcile, duplicate/conflicting evidence, late fill after closure, bust/correction, manual intervention. Mark each row `PROPOSED_REQUIRES_USER_DECISION` or `BLOCKED_BY_DEPENDENCY` and identify exact missing guards. Where labels are unavailable use `UNSPECIFIED` rather than inventing them.
4. Provide explicit negative/forbidden transitions and invariants: `CancelRequested != Cancelled`, `ACK != executed`, `timeout != unsent`, `workflow_closed != economic_final`, `snapshot_missing != nonexistence`, `same key + different payload => conflict`, `unknown broker identity => quarantine`, `closed workflow cannot be silently reopened`, `late fills do not vanish`, `unverified economic finality cannot release risk`.
5. Propose deterministic state/model-based and replay test scenarios; **do not run or claim OMS functional tests**. Map to DET-01 OMS-T01..T12 and any CR-B10 B10-Txx gates, preserving `TESTS_NOT_EXECUTED`.

## 4. Deliverable C — OrderClosure reason vocabulary decision request

Create `contracts/design-proposals/TRD-ARCH-03B1-OrderClosure-vocabulary-options.md`:

- Extract all existing closure-related terms, if any, and provenance. Explicitly identify that the authoritative complete `closure_reason` vocabulary and permitted edges are still absent.
- Provide **at least 3 well-defined design alternatives** (e.g., minimal reason families with typed evidence, richer discriminated reasons, reason + independent terminality evidence). Do not assert that these example architectures are already approved.
- For each alternative: semantics of closure vs economic finality; evidence requirements; late fills/corrections; cancel/fill races; unknown outcome; persistence/audit implications; compatibility/migration risk; validation burden; dependency on risk and broker evidence; safety/fail-closed properties; advantages/disadvantages.
- Recommend one **for user review**, clearly label recommendation as non-binding. Include concrete counterexamples showing why a single `Closed/Cancelled/Filled` flag is insufficient.
- Avoid final Rust enum, DB schema, wire payload, and auto-transitions.

## 5. Deliverable D — numbered user decisions and dependency map

Create `contracts/design-proposals/TRD-ARCH-03B1-approval-requests.md`. Use stable question IDs `OMSQ-01` onward, with **one independently decidable issue per question**. Cover at minimum:

1. Whether historical closure is recorded once or versioned through explicit append-only corrections (without reopening workflow).
2. Closure-reason modeling strategy and minimum evidence.
3. Whether an unknown broker send outcome can ever close a workflow, and under what evidence/incident handling.
4. Cancel/replace/modify versus fill-race semantics and guards.
5. Late fill/bust/correction ledger effects and closure preservation.
6. Required evidence predicate for economic finality (mark cross-contract with DET-02; do not pre-approve risk release).
7. Transition-matrix coverage/negative-edge policy and versioning.
8. Replay/conflict resolution and manual adjudication authority.
9. Preconditions for accepting broker terminal observations (identity, completeness, provenance).
10. Concurrency/transaction/audit boundaries, with dependencies on lock order and outbox.

Each question MUST include: exact question, source references, already binding principles, what remains open, 2–3 mutually distinguishable alternatives (including conservative fail-closed option), consequence table (safety, complexity, recoverability, auditability), recommended alternative with rationale, required external evidence, and explicit `USER_DECISION_PENDING`. **Do not present recommendations as approvals.** If 10 questions cannot be made independent, split further and explain.

Create `contracts/design-proposals/TRD-ARCH-03B1-dependency-and-conflict-matrix.md` mapping each proposed resolution to `XC-03-MATRIX` and other open XC findings. Mark `XC-03-MATRIX` **OPEN**, not resolved. State which questions require subsequent ARCH-03B.2 risk and ARCH-03B.3 broker decisions. No user decision or broker evidence may be fabricated.

## 6. Documentation integration and tests

- Update only documentation indexes (`contracts/traceability-index.md`, `contracts/domain/unresolved-decisions.md`, `docs/architecture/README.md`, optionally relevant `contracts/design-proposals/README.md`) to link the proposal package and retain all existing open statuses. Do not edit historical ADFR, manifest, source package, dated decision records, status ratification, or canonical registry.
- Add focused **documentation-only** tests only if necessary, preferably extending existing `crates/trading-domain/tests/adfr_reconciliation.rs` or `registry_skeleton.rs` outside `src/`. Tests should assert that proposal docs are explicitly non-ratified, user questions remain pending, forbidden promotions absent, all 78 ADFR entries unchanged, source manifest hashes valid, registry unchanged and safety flags closed. Avoid brittle text-only checks where structured validation is possible. No Cargo dependencies unless justified as test-only.
- Run and record exact exit codes: `cargo fmt --all -- --check`; `cargo check --workspace --locked`; `cargo test --workspace --locked`; `cargo clippy --workspace --all-targets --locked -- -D warnings`. Report ignored PostgreSQL test as ignored, not passed. Remote CI not run unless actually run. If a check fails, report initial failure, fix in scope, rerun and show both outcomes.

## 7. Archive, report, Git

Archive this **exact prompt** in `docs/cursor-prompts/trading/TRD-ARCH-03B1-OMS-Finality-and-OrderClosure-Design-Resolution.md`.

Write `docs/architecture/TRD-ARCH-03B.1-report.md` with: preflight branch/HEAD/clean worktree, predecessor commit verification, exact sources inspected and missing sources, hash/ADFR/registry checks, existing binding principles vs proposals, complete deliverable index, numbered decision requests, open XC matrix, explicit non-implementation/non-authorization, changed file list with hashes, test commands/results and skipped tests, deviations, 12 acceptance gates, commit/push status. The report cannot contain the SHA of the commit that introduces itself; output SHA in Cursor's final response.

One local commit only **after all gates pass**: `TRD-ARCH-03B.1: Draft OMS Finality and OrderClosure Decisions`. **Do not push.** If blocked, do not label PASS and do not commit as completed.

## 8. Acceptance gates

- **A03B1-G01:** Clean baseline, predecessor commit/report confirmed, 12/12 source hashes and registry baseline validated.
- **A03B1-G02:** All binding decisions distinguished from reconstructed/reference proposals; no fabricated authority.
- **A03B1-G03:** OMS independent dimensions and finality semantics documented, with explicit fail-closed invariants.
- **A03B1-G04:** Candidate transition coverage includes positive/negative scenario families, missing guards and dependencies; no approved transition edges or enum.
- **A03B1-G05:** OrderClosure alternatives compare evidence, late fills, corrections, cancel races and risk implications; no canonical vocabulary published.
- **A03B1-G06:** At least ten independently reviewable OMSQ decisions, each with alternatives, tradeoffs, recommendation, provenance, pending status.
- **A03B1-G07:** `XC-03-MATRIX` and all other unresolved safety-critical XC items remain OPEN; CR-B01 precedence maintained.
- **A03B1-G08:** 78 ADFR historical entries, 78 `NOT_VERIFIED`, one supersession, manifest and source bytes unchanged.
- **A03B1-G09:** Registry byte-identical, all safety flags unchanged; no runtime/schema/DB/broker implementation.
- **A03B1-G10:** Traceability and docs indexes updated without changing prior user decisions or historical statuses.
- **A03B1-G11:** Documentation checks and four local Cargo commands pass; skipped/unrun tests correctly identified.
- **A03B1-G12:** Exact prompt archived, report complete, exactly one local commit after gates, no push; SHA reported externally.

## 9. Required final Cursor output

Report `VERDICT: PASS/PARTIAL/FAIL`, gate table, baseline HEAD, final commit SHA (if committed), `PUSHED: NO`, exact file paths, `OMSQ-01..N` list, unresolved blockers, and explicitly: **`NO OMS TRANSITIONS APPROVED; NO ORDERCLOSURE VOCABULARY APPROVED; LIVE TRADING NOT AUTHORIZED.`**
