# TRD-ARCH-03B.2 — Risk Reservation & Economic Finality Design Resolution

**Mode:** DESIGN / DOCUMENTATION / DOCUMENTATION-ONLY VALIDATION. **NOT an implementation authorization.**  
**Date:** 2026-11-15. **Approval authority:** user only.  
**Prerequisite:** TRD-ARCH-03B.1.2 has been completed and committed, with its report and exact prompt archive present.

## 0. Mission and hard boundaries

Prepare an evidence-grounded, reviewable **risk contract decision package** focused on `XC-02-PRED` and its dependencies: atomic risk reservations, partial fills, reservation adjustments, economic finality, late fills/busts/corrections, budget insufficiency, valuation and rounding, and the conditions for *considering* eventual release. Deliver independent decision questions with 2–3 alternatives, consequences, recommendation and dependencies. **Do not ratify your own recommendations.** The output is a design proposal, not an executable risk policy, migration, implementation, canonical schema or permission to trade.

**Strict prohibitions:** No changes to `crates/*/src`, `apps/*/src`, database migrations, Cargo dependencies or runtime configuration. No actual order/risk state machine, ledger posting, reservation/release code, broker calls, dispatch, APIs, event/command schemas, canonical registry or deployment. Do not edit the 12 architecture source files, source manifest, ADFR JSON/CSV, prior decision records, or historical proposal snapshots. Do not enable live trading, risk release, broker terminal acceptance, automated economic posting or modify/replace. Do not invent IBKR evidence or claim deadlock, reconciliation or DB integration tests have passed.

Preserve: `SKELETON_NOT_CANONICAL`, `canonical_schemas_published=false`, `part_b=NO-GO`, `live_trading_authorized=false`, `broker_uniqueness_evidence=EVIDENCE_PENDING`, all 78 ADFR `verification=NOT_VERIFIED`, `ORDER_STATE_MACHINE_IMPLEMENTED=false`, `STATE_TRANSITIONS_IMPLEMENTED=false`, no approved `OrderClosure` enum or transition edge. CR-B01 precedence over conflicting/less restrictive DET-07 remains binding. `XC-02-PRED`, `XC-03-MATRIX`, `XC-01`, `XC-07-ROUND`, `XC-08-IBKR`, `XC-05-LOCK`, `XC-06`, `XC-09-RESET` and other unresolved findings must not be silently closed.

## 1. Preflight: repository evidence, stop conditions and authority

1. Confirm branch, clean `git status --short`, baseline HEAD, latest commit subject/SHA, and presence of `docs/architecture/TRD-ARCH-03B.1.2-report.md` and `docs/cursor-prompts/trading/TRD-ARCH-03B1.2-Ratify-OMS-Design-Decisions.md`. Confirm the preceding commit exists and includes the report and prompt archive. **STOP** on dirty worktree, missing commit/report, or conflicting recorded decisions. Do not reset/stash/discard changes or infer a commit from report text.
2. Read the actual source files (never rely solely on this prompt): `docs/architecture/TRD-ARCH-SYNC-01-report.md`, `TRD-ARCH-SYNC-02-report.md`, `TRD-ARCH-03A-report.md`, `TRD-ARCH-03A.2-report.md`, `TRD-ARCH-03B.1-report.md`, `TRD-ARCH-03B.1.2-report.md`; all `contracts/decisions/ARCH-DEC-01-bis-03-2026-11-15.md`, `ARCH-DEC-04-bis-08-2026-11-15.md`, `ARCH-DEC-09-bis-20-2026-11-15.md`; DET-01, DET-02, DET-03..08, DET-09..10, CR-B01..04, CR-B05..09, CR-B10 and Architecture Consolidation source documents under `contracts/sources/`; `contracts/design-proposals/TRD-ARCH-03B1-*` including approval requests, semantics, dependency matrix and resolution index; `contracts/status-model/*`; `contracts/domain/unresolved-decisions.md`, glossary, traceability index, `contracts/source-manifest.json`, ADFR JSON/CSV, registry JSON and existing documentation tests. DET-07 and CR-B01 may be sections of combined files: locate by heading, not imagined individual paths.
3. Verify manifest current hashes/byte lengths for all 12 sources; preserve distinct package intake hashes. Check JSON/CSV ADFR parity: exactly 78 unique ordered IDs, all `NOT_VERIFIED`, one supersession `ADFR-CROSS-005 -> ADFR-CR-B10-001`, 77 `NOT_SUPERSEDED`. Record SHA-256/byte counts of manifest, ADFR JSON/CSV, registry and prior resolution index. Verify all five registry safety flags. **STOP** on discrepancies.
4. Explicitly separate `BINDING_EXISTING`, `RECONSTRUCTED_BASELINE_WITH_EXCEPTIONS`, `REFERENCE_PROPOSAL`, `EXTERNAL_EVIDENCE_PENDING`, `NEW_PROPOSAL`, `USER_DECISION_PENDING`. DET-01 and DET-02 reconstructed provenance and open Q01–Q10 verification cells remain unchanged. ARCH-DEC-04 gives no implicit priority to historical status words. Do not elevate any reference equation, posting sequence, `READ COMMITTED` default, lock order, table name or test case into a ratified contract.
5. **Binding OMS policy from ARCH-DEC-09..20:** immutable single historical closure record; reason-family plus independent finality dimensions as *strategy only*; unknown send cannot close workflow; cancel/fill evidence retained separately; **all proposed economic effects quarantined pending authorized manual review, including single consistent identity matches**; **no economic-finality predicate and no risk release**; future transition matrix needs full coverage/negative edges/version; contradictory replay quarantined; no authoritative terminal broker observations; lock order/isolation unadopted; distinct transactional audit record required but unnamed; modify/implicit cancel-replace blocked. This task must not weaken these policies or construe a proposed future algorithm as permission to post.

## 2. Deliverable A — Risk semantics, invariants and dimensional boundaries

Create `contracts/design-proposals/TRD-ARCH-03B2-risk-semantics-and-invariants.md`:

- Define separate conceptual dimensions for **risk budget**, **exposure**, **reservation**, **execution evidence**, **economic ledger/projection**, **valuation snapshot**, **workflow closure**, **broker evidence completeness**, **risk incident/halt**, and **authorization**. Distinguish risk *reservation*, *consumption*, *reallocation*, *compensating adjustment* and *release*; these are not synonyms and no operations are approved.
- Inventory existing DET-02/CR-B02/CR-B03/CR-B04/CR-B07/CR-B09 and OMS decision invariants with exact file/section anchors and provenance. Identify all missing definitions explicitly. Separate hierarchical scopes (global/account/strategy/instrument/sector/factor), shared budget constraints and reservation ownership from proposed formulas.
- Model the conceptual lifecycle of an order's **risk obligation** under submission uncertainty, partial fill, cancellation request/observation, late fill after workflow closure, execution bust/correction, duplicate evidence, valuation staleness, broker disconnect, restart and reconciliation. Use scenario descriptions, **not** approved state edges or DDL.
- Record fail-closed principles: no risk release on timeout, cancel request, workflow closure, broker ACK or incomplete snapshot; no second submission under uncertain send; no silent exposure netting; no negative available-budget allowance by accident; ambiguous identity quarantined; duplicates have at most one eventual economic effect; keep all evidence even when booking is blocked; preserve audit and source lineage.
- Address local PostgreSQL atomic unit-of-work and outbox at **design level**: command/order/risk reservation/evidence/audit/outbox consistency, idempotency, CAS, targeted locks, retries only of local DB unit, no broker/NATS call in transaction. Do not assume an adopted global lock order, isolation level, audit table or tested concurrency behavior.
- Identify explicit conflicts or ambiguity between existing contracts, especially `XC-02-PRED`, `XC-03-MATRIX`, `XC-01`, `XC-05-LOCK`, `XC-06`, `XC-07-ROUND`, `XC-08-IBKR`, `XC-09-RESET`, `XC-10-ACTION`, `XC-04`. Mark what ARCH-03B.2 cannot decide without ARCH-03B.3 broker evidence.

## 3. Deliverable B — Economic-finality evidence and release *candidate* analysis

Create `contracts/design-proposals/TRD-ARCH-03B2-economic-finality-and-release-coverage-draft.md`:

1. Explicitly state **CURRENT BINDING POLICY: NO EXECUTABLE ECONOMIC-FINALITY PREDICATE; RISK RELEASE FORBIDDEN** (ARCH-DEC-14). Any future checklist is a proposal only, not an adopted predicate or evidence of sufficiency.
2. Compare possible future evidence categories: scoped broker/execution identity; snapshot dataset/window/connection completeness and provenance; raw events; append-only fill/bust/correction chain; versioned valuation/FX; reservation ledger reconciliation; authorization/incident state; audit record and CAS version. Mark each `REQUIRED_BY_EXISTING_PRINCIPLE`, `CANDIDATE_ONLY`, or `EVIDENCE_PENDING` with sources. Never claim IBKR snapshot completeness or identity uniqueness.
3. Define a **candidate coverage template**, not executable logic: scenario, evidence received/missing, identity confidence, completeness status, ledger ambiguity, valuation version, budget scope, candidate reservation effect, prohibited effect, incident/quarantine path, recovery/replay, approval status, dependencies and test oracle.
4. Include at minimum: initial reserve, rejected pre-dispatch command, `MayHaveSent`, ACK without fills, partial fill, complete fill, cancel requested, broker cancel observation, cancel/fill race, disconnect/incomplete snapshot, duplicate fill, late fill, bust, correction, insufficient budget, stale price/FX, cross-scope shared budget contention, CAS conflict, restart/reconciliation, manual adjudication, emergency halt. For every row distinguish **record evidence** from **book economics** from **release budget**; booking/release remain prohibited by current policy.
5. Document why tempting shortcuts are unsafe: `Cancelled => release`, `Filled => economic finality`, `Closed => no future fill`, `timeout => not sent`, `broker snapshot empty => no order`, `same nominal quantity => same execution`, `risk reduction => safe without broker semantics`. Provide adversarial counterexamples and model-based/property-based/replay test proposals; label all such functional tests `NOT_EXECUTED`.
6. Do not specify a final financial rounding rule, valuation policy, stress metric, transaction lock order or broker-completeness proof. These remain separate findings.

## 4. Deliverable C — Reservation, partial-fill and correction policy alternatives

Create `contracts/design-proposals/TRD-ARCH-03B2-reservation-and-correction-options.md` with at least three well-distinguished approaches to **future** reservation accounting, including (i) conservative hold-until-evidenced finality, (ii) explicit reservation-to-exposure conversion with independently audited residual obligation, (iii) proposed staged allocation/adjustment subject to strict proofs. These are comparison architectures, not allowed operations. Explain:

- Ledger double-entry or equivalent conservation concepts where justified by sources, without inventing executable posting formulas; exact arithmetic and FX/rounding are OPEN.
- Partial-fill attribution and residual unfilled obligation; late fill after historical closure; bust/reversal/correction chains; duplicate detection; insufficient budget/negative exposure; multi-scope hierarchical contention and over-reservation; recovery and crash consistency.
- Required evidence and guards, idempotency and audit implications, worst-case failure modes, operational burden, migration/compatibility, tests, and dependencies on ARCH-03B.3.
- The existing OMSQ-05 B quarantine policy and OMSQ-06 A no-predicate/no-release policy must remain binding **regardless of preferred future alternative**.
- Recommend only for user consideration, not as approval.

## 5. Deliverable D — Independent risk decision requests

Create `contracts/design-proposals/TRD-ARCH-03B2-approval-requests.md` with stable IDs `RISKQ-01` onward, **at least 12 independently decidable questions**, each `USER_DECISION_PENDING`. Each question MUST contain: precise question, existing binding rule(s), sources with file/section, exact unresolved aspect, 2–3 genuinely distinct alternatives (including fail-closed), consequence table (safety, complexity, recoverability, auditability), non-binding recommendation with reason, evidence needed, explicit dependencies, and what the decision **would not authorize**.

Cover independently at least:

1. Hierarchical risk-budget scope and compositional deny-overrides precedence (policy shape vs executable budget math).
2. Reservation ownership, identity, idempotency and concurrent cross-scope contention.
3. Initial reservation prerequisites and failure behavior when price/liquidity/FX inputs are missing.
4. Partial-fill treatment: reservation vs realized exposure vs residual obligation, without economic posting authorization.
5. Cancel request/cancel observation and residual risk obligations under incomplete broker evidence.
6. Late fill after closure, duplicate fill, bust and correction: evidence retention and quarantine under ARCH-DEC-13.
7. Insufficient budget, incident/halt severity and authorization to reduce risk (not broker action approval).
8. Candidate evidence bar for **future** economic finality, explicitly subordinate to ARCH-DEC-14 and not adopting an executable predicate.
9. Conditions and proof obligations for *future* reservation release; **current release stays forbidden**.
10. Valuation, FX, precision and rounding governance, versioning and stale-data fail-closed policy; no rounding policy adopted.
11. Transaction isolation, lock ordering, CAS and crash/retry boundaries, without adopting CR-B02 lock order or `READ COMMITTED`.
12. Audit authority and reconciliation evidence for reservation changes, without naming a table or enabling posting.
13. Manual adjudication/role approvals and second-person requirements for quarantined economic effects; no fake second approver.
14. Recovery, deterministic replay, test coverage and versioned risk-policy changes, with negative tests and external broker dependencies.

Split further if necessary for independence. **Never** offer an alternative that contradicts binding ARCH-DEC-09..20 as if it were presently permissible; if a rejected/unsafe concept is included for contrast, explicitly mark `CONFLICTS_WITH_BINDING_POLICY`, not an available ratification without an explicit superseding decision. In particular, do not propose enabling automatic posting, release, broker terminality, or live trading as an outcome of these questions. Choices may establish future design constraints while leaving implementation blocked.

## 6. Deliverable E — Dependency, conflict and traceability matrix

Create `contracts/design-proposals/TRD-ARCH-03B2-dependency-and-conflict-matrix.md` mapping each `RISKQ` to source clauses, ARCH-DEC-01..20, ADFR IDs (without editing them), `XC-02-PRED` and other open findings. Clearly mark `XC-02-PRED` OPEN; distinguish principles already aligned from executable rules unapproved. State explicit handoffs to ARCH-03B.3 for broker identity/completeness and to later OMS matrix work for state transitions. Identify blockers for future financial arithmetic, concurrency tests, reconciliation and risk release. Do not invent an ADFR status or claim resolution.

Optionally create a structured `TRD-ARCH-03B2-proposal-index.json` solely for documentation tests, explicitly `NON_RUNTIME_NON_WIRE_PROPOSAL_INDEX`, `NOT_RATIFIED`, with `risk_release_approved=false`, `economic_posting_approved=false`, `economic_finality_predicate_approved=false`, `live_trading_authorized=false`, `canonical_schemas_published=false`, and unresolved XC flags. If created, add a narrow explicit test allowlist and prove it is not a schema/registry. Never modify existing historical proposal/resolution indexes.

## 7. Documentation integration and tests

- Update only necessary documentation navigation and open-issues indexes, e.g. `contracts/design-proposals/README.md`, `contracts/traceability-index.md`, `contracts/domain/unresolved-decisions.md`, `docs/architecture/README.md`, and `tests/README.md` if applicable. Preserve prior ratified records and source history byte-for-byte.
- Add **documentation-only** tests in existing `crates/trading-domain/tests/adfr_reconciliation.rs` / `registry_skeleton.rs` as justified, never under `src/`. Assert: 12 source manifest hashes match; 78 ADFR JSON/CSV IDs and `NOT_VERIFIED` status; one supersession; registry bytes/safety flags unchanged; prior ARCH-DEC-09..20 policy retained; new proposals `NOT_RATIFIED`; `XC-02-PRED` OPEN; no risk-release/posting/finality approval; no new canonical schema; no production changes. Prefer structured checks over brittle text matching. Do not claim that these documentation tests prove actual financial safety.
- Run from repo root, record exit codes: `cargo fmt --all -- --check`, `cargo check --workspace --locked`, `cargo test --workspace --locked`, `cargo clippy --workspace --all-targets --locked -- -D warnings`. If any initially fails, document first failure and correction, rerun all and show both outcomes. The ignored PostgreSQL integration test is **ignored**, not passed; remote CI is **not run** unless evidenced.

## 8. Archive, report and commit

1. Archive this **exact prompt** as `docs/cursor-prompts/trading/TRD-ARCH-03B2-Risk-Reservation-and-Economic-Finality-Design-Resolution.md`.
2. Create `docs/architecture/TRD-ARCH-03B.2-report.md` including: date, mode, baseline HEAD/branch/clean state, predecessor commit verification, sources inspected/missing, manifest/ADFR/registry hashes and parity, binding vs proposed semantics, deliverable index, every RISKQ and its status, open XC map, risk release/booking prohibitions, changed-file hashes, test commands and results (including initial failures), skipped DB/remote CI, deviations, all 12 gates, commit and push status. Do not put the SHA of the commit containing the report inside the report; print actual SHA in Cursor's final response.
3. Exactly **one local commit after all gates pass** with subject: `TRD-ARCH-03B.2: Draft Risk Reservation and Economic Finality Decisions`. **No push.** No commit labeled PASS if blocked. Do not perform destructive Git operations.

## 9. Acceptance gates (A03B2-G01..G12)

- **G01:** Clean baseline, verified committed TRD-ARCH-03B.1.2 predecessor and report, 12/12 source hashes, ADFR/registry baseline recorded; no preflight mismatch.
- **G02:** ARCH-DEC-01..20 and DET-01/02 reconstructed exceptions respected; binding OMS quarantine/no-release rules explicit; no fabricated user decision.
- **G03:** Risk dimensions, hierarchy, identity, invariants and trust boundaries documented with exact source traceability and unresolved details.
- **G04:** Economic-finality evidence/coverage scenarios include missing/contradictory evidence and explicit forbidden effects; no predicate or release approved.
- **G05:** Reservation/partial-fill/correction alternatives compare failure modes and evidence; no arithmetic or economic posting promoted to runtime.
- **G06:** At least 12 independent `RISKQ` questions with alternatives, consequence tables, recommendations and `USER_DECISION_PENDING`; no hidden bundled vote.
- **G07:** Dependency matrix preserves `XC-02-PRED` and relevant other findings OPEN; ARCH-03B.3 broker dependencies and OMS transition dependencies explicit.
- **G08:** 78 ADFR entries and JSON/CSV parity, one supersession, 12 manifest sources, manifest and prior decision records remain unchanged.
- **G09:** Registry byte-identical; safety flags unchanged; no production source, migrations, dependencies, schema, broker activity, booking, risk release or live-trading enablement.
- **G10:** Documentation navigation and unresolved-decision indexes updated without rewriting historical decisions, status meanings or prior proposal snapshots.
- **G11:** Documentation-only regression tests and all four Cargo checks pass in final run; any failures and ignored/remote tests reported honestly.
- **G12:** Exact prompt archived, comprehensive report produced, exactly one local commit with prescribed subject, SHA shown in Cursor output, `PUSHED: NO`.

## 10. Cursor final response

Report `VERDICT: PASS` **only if** G01..G12 are all supported. Include baseline and new commit SHA, changed-file list, RISKQ count and identifiers, open blockers, final check results, ignored tests, confirmation of no push, and clear statement: **risk posting, economic finality predicate, reservation release, OMS transitions, canonical schemas, broker dispatch and live trading remain unauthorized.** If blocked, output `VERDICT: BLOCKED` with exact gate and evidence; do not silently downgrade safeguards or claim completion.
