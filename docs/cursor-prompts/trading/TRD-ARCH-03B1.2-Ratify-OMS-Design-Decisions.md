# TRD-ARCH-03B.1.2 — Ratify OMS Design Decisions (Documentation Only)

**Date:** 2026-11-15  
**Authority:** Explicit user ratification of OMSQ-01 through OMSQ-12 in the conversation on 2026-11-15  
**Mode:** Documentation, traceability, and documentation-only validation. **NO runtime implementation.**  
**Branch:** Current repository branch; do not switch branches without asking.  
**Live trading:** NOT AUTHORIZED. **Canonical schemas:** NOT PUBLISHED. **Part B:** NO-GO.

## 0. Mission and absolute boundaries

Record the twelve explicitly ratified design choices from TRD-ARCH-03B.1 in dated decision records, and reconcile **documentation interpretations only**. The approval is limited to the exact policy choices below. Do not treat a choice of model *strategy* or acceptance *bar* as approval of a concrete enum, state-transition edge, database schema, executable predicate, broker rule, or implementation.

**Never:** edit historical source files or source-manifest hashes; rewrite ADFR JSON/CSV or their verification values; edit `contracts/registry/registry.json` or relax its safety flags; create/publish canonical schema or wire contract; edit `crates/*/src`, `apps/*/src`, migrations, broker adapters, production config, Cargo dependencies; enable broker dispatch, risk release, OMS transitions, order closure writers, auto-adjudication, live trading, or unsafe failover; execute live orders; push to remote. Do not invent external evidence, test results, signatures, user approvals, broker uniqueness guarantees, IBKR completeness guarantees, or executed tests. If a requirement conflicts with these boundaries, stop and report the blocker.

Keep `ORDER_STATE_MACHINE_IMPLEMENTED=false`, `STATE_TRANSITIONS_IMPLEMENTED=false`, `live_trading_authorized=false`, `canonical_schemas_published=false`, `part_b=NO-GO`, `broker_uniqueness_evidence=EVIDENCE_PENDING`, and registry `SKELETON_NOT_CANONICAL` (use exact current representations; do not introduce new flags merely to restate them). All 78 ADFR `verification` fields remain `NOT_VERIFIED`; the one supersession remains `ADFR-CROSS-005 -> ADFR-CR-B10-001` and 77 entries remain non-superseded.

## 1. Preflight and provenance

1. Record branch, clean/dirty worktree, `git rev-parse HEAD`, subject, toolchain versions, and existing changes. **If dirty, stop and ask**; do not reset, stash, discard, or overwrite user work.
2. Read predecessor reports: `docs/architecture/TRD-ARCH-SYNC-01-report.md`, `TRD-ARCH-SYNC-02-report.md`, `TRD-ARCH-03A-report.md`, `TRD-ARCH-03A.2-report.md`, and `TRD-ARCH-03B.1-report.md`.
3. Read `contracts/decisions/ARCH-DEC-01-bis-03-2026-11-15.md`, `ARCH-DEC-04-bis-08-2026-11-15.md`; `contracts/design-proposals/TRD-ARCH-03B1-approval-requests.md`, its semantics, transition coverage, vocabulary, conflict matrix and proposal index; `contracts/status-model/*`; `contracts/traceability-index.md`; `contracts/domain/unresolved-decisions.md`; `contracts/source-manifest.json`; `contracts/registry/registry.json`; ADFR JSON/CSV and README; existing documentation-only tests. Follow actual repo paths; report missing sources rather than inventing substitutes.
4. Verify all 12 manifest-current source hashes/byte counts, preserve original `package_sha256` and `package_bytes`; capture registry and ADFR SHA-256 and count/ID/order parity. Confirm no existing record already uses the intended new decision IDs. Preserve `reconstructed_draft`, `SOURCE_REVIEW_PENDING`, `reviewed=false`, `approved=false` for DET-01/02.
5. Inspect the CR-B01/DET-07 precedence, DET-01 Q01–Q10 exceptions, XC-03-MATRIX, XC-01, XC-02-PRED, XC-04, XC-05-LOCK, XC-06, XC-07-ROUND, XC-08-IBKR, XC-09-RESET, XC-10-ACTION, XC-11. Do not silently resolve them.

## 2. Exact user-ratified choices — binding *design policy only*

Use the following verbatim choices and IDs in the decision record. The original proposal questions and their unchosen alternatives must remain historically intact. Record the user ratification date **2026-11-15**; do not invent signature or evidence URI.

| Question | Choice | Exact user decision | Scope and exclusion |
|---|---|---|---|
| OMSQ-01 | **A** | Ein ursprünglicher Abschlussdatensatz, spätere Fakten separat | One immutable historical closure record; later facts in separate append-only ledger/evidence/reconciliation artifacts. No closure-shaped correction record and no reopening. No writer/schema authorized. |
| OMSQ-02 | **C** | Grundfamilie plus unabhängige Finalitätsfelder | Adopt *modeling strategy* only: reason family separate from broker/economic/evidence terminality dimensions. **No token strings, enum, field schema, terminality semantics, or edge approved.** |
| OMSQ-03 | **A** | Keine Schliessung ohne neue autorisierte Aktion und positive Evidenz | Unknown send cannot close workflow; remain in reconciliation/quarantine as appropriate. Later closure requires separately authorized action and positive evidence. No automatic retry or takeover. |
| OMSQ-04 | **A** | Alle Fakten behalten; Closure und Risk nur bei ausreichender Evidenz | Preserve cancel request, cancel observation and every fill as separate facts. No closure or risk movement until identity, completeness, and economic predicate are sufficient. **The predicate is not approved by OMSQ-06; consequently no risk release is enabled.** |
| OMSQ-05 | **B** | Alle wirtschaftlichen Effekte zunächst manuell prüfen | Retain all late/duplicate/bust/correction evidence; quarantine **all proposed economic effects**, even apparently single matches, pending **authorized** manual adjudication. Manual review is not itself authorization to post; no posting algorithm, compensation arithmetic, risk release, or bypass approved. Preserve original closure. |
| OMSQ-06 | **A** | Kein Finalitätsprädikat akzeptieren; Risk Release verboten | No executable economic-finality predicate. No reservation release. Evidence checklist B remains an unratified future candidate, not silently adopted. |
| OMSQ-07 | **A** | Vollständige Szenarien, Negativfälle und eigene Version verlangen | Future transition matrix must cover all scenario families, explicitly forbidden negative edges, missing guards and an independent version. **No current matrix version or transition edge approved.** |
| OMSQ-08 | **A** | Quarantäne und strukturierte autorisierte Entscheidung | Only idempotent identical-payload replay is automatically safe as already bounded; all other conflicts quarantined until an authorized manual record with evidence IDs, actor, expected `OrderVersion`, reason. If independent second approver is required but unavailable, deny. **No adjudicator role or write path approved.** Read with OMSQ-05 B: no automatic economic posting for even a single match. |
| OMSQ-09 | **A** | Keine terminale Brokerbeobachtung akzeptieren | No broker observation currently accepted as authoritative terminal. Preserve raw payload and unverified mapping without downstream terminal effects; identity/completeness evidence pending. No acceptance rule or terminal edge approved. |
| OMSQ-10 | **A** | Sperrreihenfolge und Isolation bleiben unbestätigter Vorschlag | CR-B02 lock order and `READ COMMITTED` stay proposals. No canonical lock protocol or isolation level. Future conflict retries local DB unit only, never broker resend. Deadlock/serialization tests pending. |
| OMSQ-11 | **A** | Separaten transaktionalen Auditdatensatz verlangen | OMS mutation requires distinct in-transaction audit record, not outbox/export, and must fail if it cannot be written. No authoritative table name, DDL, hash-chain, retention/signing/backup policy approved. |
| OMSQ-12 | **A** | Modify und implizites Cancel-Replace blockieren | No modify and no implicit cancel-replace. Any future replacement is a **new order** only after separately evidenced finality and a fresh risk decision; until then reject command. No broker dispatch or action gate authorized. |

### Cross-decision consistency rules

- **OMSQ-05 B overrides any appearance of automatic economic booking in OMSQ-08 A**: its “single consistent identity match” clause cannot be used to post automatically under the ratified OMSQ-05 B. Identical-payload replay may return stored results idempotently, not create a new economic effect.
- **OMSQ-04 A does not authorize risk release**: OMSQ-06 A leaves the economic-finality predicate undefined, so risk release remains forbidden. Do not confuse an evidence requirement with its satisfaction.
- **OMSQ-01 A + OMSQ-02 C**: closure is an immutable historical fact, not broker/economic finality. The independent finality slots are conceptual, not approved runtime fields. Later corrections belong outside the closure record.
- **OMSQ-03 A + OMSQ-09 A + ARCH-DEC-03**: timeout, empty snapshot, expired lease, broker label, or restart is not proof of non-send, broker terminality, or safe failover. Never auto-resubmit or auto-takeover.
- **OMSQ-07 A** is an acceptance bar, not a matrix ratification. `XC-03-MATRIX` remains OPEN.
- **OMSQ-08 A** does not choose roles, reset policy, two-person exception, or halt-version CAS. If authorization evidence is absent, remain quarantined.
- **OMSQ-10 A + OMSQ-11 A**: transaction/audit principle may be binding while lock order and actual audit table remain unresolved. No database implementation.
- **OMSQ-12 A** does not authorize cancel during halt or risk-reducing actions; XC-10-ACTION remains OPEN.
- Preserve ARCH-DEC-01..08, DET-01/02 exceptions, ARCH-DEC-04 historical status non-precedence and all previously OPEN XC findings. A design choice does not create external verification.

## 3. Deliverables and record numbering

Create a **new dated** decision record under `contracts/decisions/`, e.g. `ARCH-DEC-09-bis-20-2026-11-15.md`, **only if** the IDs ARCH-DEC-09..20 are unused. If any collision, stop and report it; do not renumber without explicit approval. Assign one decision ID per question in order: ARCH-DEC-09=OMSQ-01 through ARCH-DEC-20=OMSQ-12. For each include date, authority (user message), exact German wording, selected option, source question path, binding scope, explicit non-approvals, prerequisites, unresolved XC/UD references, and verification state (`NOT_VERIFIED` unless source explicitly provides narrower existing evidence). Label record as **RATIFIED_DESIGN_POLICY_ONLY / NOT_IMPLEMENTED / NOT_VERIFIED**, or use equivalent documentary prose without introducing an unratified canonical enum.

Update **only** necessary documentation, e.g.:
- `contracts/design-proposals/TRD-ARCH-03B1-approval-requests.md`: retain original proposal text/alternatives and historical `USER_DECISION_PENDING` framing as of creation; append a **dated resolution index** pointing to the new decision record. Do not rewrite original proposals as if approved at proposal time.
- `contracts/design-proposals/README.md`, dependency/conflict matrix, proposal index (if necessary): distinguish proposal package's original role from subsequent user ratification; proposal index stays `NON_RUNTIME_NON_WIRE_PROPOSAL_INDEX` or exact existing equivalent, never canonical.
- `contracts/traceability-index.md`, `contracts/domain/unresolved-decisions.md`, `docs/architecture/README.md`, glossary only where useful: record approved **policy choices** and unchanged technical blockers. Do not close XC-03-MATRIX, XC-01, XC-02-PRED, XC-04 remainder, XC-05-LOCK, XC-06, XC-07-ROUND, XC-08-IBKR, XC-09-RESET, XC-10-ACTION or XC-11 status disagreement.
- Documentation-only tests in `crates/trading-domain/tests/` where appropriate; never touch `src`.
- Archive this exact prompt under `docs/cursor-prompts/trading/TRD-ARCH-03B1.2-Ratify-OMS-Design-Decisions.md` and create `docs/architecture/TRD-ARCH-03B.1.2-report.md`.

Do not alter the original 12 source files, `contracts/source-manifest.json`, ADFR JSON/CSV, `contracts/registry/registry.json`, earlier ARCH-DEC files, production code, migrations, dependencies, or any canonical schema. Do not add a new status enum to make this task easier.

## 4. Documentation-only regression tests

Add focused deterministic checks for:
1. All twelve question IDs, exactly one dated decision each, correct mapping ARCH-DEC-09..20 and choices `A,C,A,A,B,A,A,A,A,A,A,A`.
2. The exact selected German wording and scope/exclusions, including OMSQ-05 B / OMSQ-08 A compatibility, OMSQ-04 A / OMSQ-06 A release block, OMSQ-02 C strategy-only, OMSQ-09 A no terminal acceptance.
3. No approved `OrderClosure` enum/token list, no transition edges, no approved matrix version, no finality predicate, no auto economic posting, no modify/replace dispatch.
4. All 78 ADFR IDs and order, JSON/CSV parity, 78 `NOT_VERIFIED`, 1 supersession, 77 not superseded; no promotion or lifecycle rewriting.
5. All 12 source-manifest hashes/byte counts match, package hashes preserved, DET-01/02 reconstructed provenance intact.
6. Registry byte-for-byte SHA-256 and safety flags unchanged; any documentation JSON is non-runtime/non-wire, uncanonical, no new canonical schema publication.
7. All critical XC findings remain open with correct partial precedences; no external evidence falsely claimed.
8. No production source, migration, broker behavior, dependencies, or risk-release implementation.

Prefer structured and narrowly scoped checks over brittle prose assertions. If an existing allowlist rejects a new documentation-only JSON, adjust only that exact allowlist with explicit safety assertions; do not weaken the safety gate. Do not mark ignored tests as passed.

## 5. Acceptance gates (all twelve must be reported PASS/FAIL with evidence)

- **A03B12-G01 — Baseline:** clean worktree, predecessor commit/report, manifest and registry checks captured.
- **A03B12-G02 — Authority:** twelve exact user choices recorded under dated, collision-free ARCH-DEC-09..20, with no fabricated approval or evidence.
- **A03B12-G03 — Historical integrity:** original proposals, prior decisions, 12 sources and manifest unchanged.
- **A03B12-G04 — ADFR integrity:** 78/78 JSON/CSV, 78 `NOT_VERIFIED`, exact supersession, no changes.
- **A03B12-G05 — Closure constraints:** OMSQ-01 A and OMSQ-02 C recorded without writer, enum, token or terminality claim.
- **A03B12-G06 — Broker/unknown constraints:** OMSQ-03 A, 04 A, 09 A, 12 A recorded; no terminal acceptance, resubmit, modify or replace authorization.
- **A03B12-G07 — Economic safety:** OMSQ-05 B, 06 A, 08 A recorded consistently; no automatic economic effect, no finality predicate or risk release.
- **A03B12-G08 — Matrix/transaction/audit:** OMSQ-07 A, 10 A, 11 A recorded; no approved edge, lock order, isolation or audit table.
- **A03B12-G09 — Traceability:** references and unresolved findings updated without false closure; CR-B01 precedence retained.
- **A03B12-G10 — Safety/non-implementation:** registry bytes and flags unchanged; no production code, schema, migration, broker, dependency or runtime changes.
- **A03B12-G11 — Validation:** run `cargo fmt --all -- --check`, `cargo check --workspace --locked`, `cargo test --workspace --locked`, `cargo clippy --workspace --all-targets --locked -- -D warnings`; capture exact results, failures/fixes/reruns; ignored DB test and unrun remote CI stated honestly.
- **A03B12-G12 — Report/commit:** archive prompt and report; one local commit after passing checks; **NO PUSH**. If any gate fails, report FAIL/PARTIAL rather than falsely PASS.

## 6. Execution order and report

1. Preflight and baseline evidence.
2. Write new decision record without editing earlier records.
3. Add dated resolution pointers and update documentation traceability, retaining original proposal history.
4. Add/run documentation-only regression tests; run the four required Cargo commands. Correct only in-scope defects, then rerun. Capture tests passed/failed/ignored and whether DB integration/remote CI ran.
5. Verify diff and file hashes; explicitly list unchanged safety artifacts and every modified file. Confirm no prohibited file changes.
6. Write `docs/architecture/TRD-ARCH-03B.1.2-report.md` with baseline HEAD, exact decisions, decision-to-XC matrix, before/after hashes/counts, all 12 gate results, test command exit codes, deviations, blocked items, no-implementation declaration, and next step. Report verdict PASS only if all gates genuinely pass.
7. Archive this prompt exactly and create **one local commit** with subject:

   `TRD-ARCH-03B.1.2: Ratify OMS Design Decisions`

   Include the report and archived prompt in that commit. Verify final `git status --short`, `git log -1 --format='%H %s'`; print commit SHA in Cursor's final output (do not fabricate it inside the report). **Do not push.** If commit cannot be made, report why rather than claim success.

## 7. Expected final Cursor response

Give a compact but precise summary: `VERDICT`, `COMMIT_SHA`, `PUSHED: NO`, exact ARCH-DEC-09..20 mapping, changed paths, hash/ADFR integrity, four Cargo command outcomes, test pass/fail/ignored, all 12 gate statuses, open XC blockers, and a clear statement that **ARCH-03B.2/03B.3, OMS transition matrix, OrderClosure enum, economic posting, risk release, broker terminal acceptance, deployment, and live trading are NOT authorized or implemented**.

**Do not proceed to ARCH-03B.2 or ARCH-03B.3 in this task.**
