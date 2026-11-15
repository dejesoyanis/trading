# TRD-ARCH-03A.2: Ratify SMQ-01..SMQ-05 — Status Interpretation Decisions

**Mode:** DOCUMENTATION + DOCUMENTATION-ONLY VALIDATION. **No trading implementation.**
**Predecessor:** TRD-ARCH-03A (Cursor reported PASS; not independently audited here).
**Authority:** Explicit user decisions dated **2026-11-15**, quoted in §2. This prompt authorizes recording those decisions, **not** adopting the unratified multidimensional enum, changing ADFR decisions, or publishing schemas.

## 0. Immutable safety boundaries

- Live trading remains `NOT AUTHORIZED`; `live_trading_authorized=false`, `canonical_schemas_published=false`, `part_b=NO-GO`, `broker_uniqueness_evidence=EVIDENCE_PENDING`; `SKELETON_NOT_CANONICAL` unchanged.
- No edits to `crates/*/src`, `apps/*/src`, migrations, HTTP routes, trading runtime flags, order/risk state machines, broker adapters, execution/dispatch, leases/failover, audit persistence, or financial rounding.
- **Do not edit** `contracts/registry/registry.json`, `contracts/sources/ADFR-Consolidated-v1.0.json`, `contracts/sources/ADFR-Consolidated-v1.0.csv`, any of the 12 `contracts/source-manifest.json` source files, or their manifest hashes/byte counts. Do not rewrite historical `status`, `verification`, `lifecycle`, provenance, dates, or source text to make conflicting labels agree.
- Do not introduce/ratify a closed lifecycle enum, new runtime or wire types, schema paths, canonical fixtures, or any `registry.yaml`, `events/v1`, `commands/v1`, `api/v1`, `sql/v1` artifacts.
- No external broker calls, database migrations, credential use, live/paper trading, deployment, or push. Fail closed on ambiguities.
- Documentation-only tests may be updated narrowly in existing test locations. No production dependencies. Preserve existing tests and safety assertions.

## 1. Preflight: inspect before writing

1. Confirm `main` or report actual branch, clean `git status --short`, baseline HEAD, previous commit, Rust toolchain. If dirty, **STOP**, do not reset/stash/discard.
2. Read actual `docs/architecture/TRD-ARCH-SYNC-01-report.md`, `TRD-ARCH-SYNC-02-report.md`, `TRD-ARCH-03A-report.md`; `contracts/decisions/ARCH-DEC-01-bis-03-2026-11-15.md`; `contracts/status-model/approval-requests.md`, `contract-status-model-v1.0.md`, `legacy-status-mapping.md`, `status-dimensions-draft-v0.json`; `contracts/traceability-index.md`, `contracts/domain/unresolved-decisions.md`; `contracts/source-manifest.json`, ADFR JSON/CSV/README, CR-B10, DET-01/02, DET-07 section, CR-B01 section, `contracts/registry/registry.json`, relevant documentation tests. Record missing files; do not invent their content.
3. Before edits, validate all 12 current manifest hashes and byte counts; separately preserve intake `package_sha256`/`package_bytes`. Record baseline registry SHA-256 and all five safety indicators. Validate ADFR: JSON and CSV exactly 78 identical historical IDs in same order, 78 `NOT_VERIFIED`, exactly one `SUPERSEDED` (`ADFR-CROSS-005`), successor `ADFR-CR-B10-001`, 77 `NOT_SUPERSEDED`, no cycles. **STOP** on unexplained discrepancies.
4. Preserve prior ratifications ARCH-DEC-01..03 and existing unresolved XC findings. Check if SMQ decisions already recorded; avoid duplicates and conflicts.

## 2. Exact user-ratified decisions (2026-11-15)

Record the following verbatim decision semantics in a **new dated decision record**, e.g. `contracts/decisions/ARCH-DEC-04-bis-08-2026-11-15.md` (check for naming collisions first):

- **ARCH-DEC-04 / SMQ-01, Alternative 1:** `Keine Priorität; historische Statusbegriffe unverändert lassen`. No winning status between `OPEN P0`, `PROPOSED_FOR_ACCEPTANCE`, and `DESIGN_RESOLVED`; preserve source-specific words and ambiguity. The remaining XC-11 status-word conflict **stays OPEN**. Historical priority P0/P1/P2 is not a decision-lifecycle state.
- **ARCH-DEC-05 / SMQ-02, Alternative 1:** `Kein neues Enum und keine zusätzlichen Lifecycle-Zuordnungen`. Do not ratify proposed `proposed/open/ratified_with_exceptions/superseded/rejected/withdrawn` closed enum. Preserve only existing lifecycle facts `SUPERSEDED` and `NOT_SUPERSEDED`; do not conflate superseded with withdrawn or rejected.
- **ARCH-DEC-06 / SMQ-03, Alternative 1:** `Nur konkret getestete Teilkomponenten als lokal getestet behandeln`. `IMPLEMENTED_AND_LOCALLY_TESTED` is scoped evidence for explicitly named type-shell functionality, not whole UD, not external broker/DB verification, not remote CI, and not ADFR verification. Preserve all 78 `NOT_VERIFIED` entries and `EVIDENCE_PENDING` broker uniqueness.
- **ARCH-DEC-07 / SMQ-04, Alternative 1:** `Datierter Nutzerentscheid plus geeigneter Evidenznachweis`. Before any ADFR verification promotion, require a dated explicit user approval and evidence record containing URI or stable artifact reference, method, environment, timestamp, exact claim/scope, and limitations; evidence must be appropriate to the claim. Ignored tests, unrun CI, `DESIGN_RESOLVED`, and isolated local tests are not external proof. **This is a procedural gate, not approval of any present verification promotion.**
- **ARCH-DEC-08 / SMQ-05, Alternative 1:** `Bestehende Manifestangaben beibehalten`. Preserve `placement`, intake/current hashes, `reconstructed_draft`, `reviewed`, `approved`, and `source_text_status`. Do not label other ten package files `original` or `unknown` or claim original Q&A provenance; DET-01/02 retain reconstructed provenance and baseline ratification with exceptions.

**Important:** These are approvals of conservative *interpretation and evidence rules only*. Do not approve or implement any open technical contract. Cite original user approval in the record; do not fabricate signatures or evidence URIs. Record source, alternatives rejected, rationale, consequences, exclusions, and effective date. A ratified decision may coexist with unratified proposed status-dimension tokens.

## 3. Documentation updates, minimal and non-destructive

1. Add the dated record under `contracts/decisions/`; link each SMQ ID to ARCH-DEC-04..08 with an explicit `DECIDED` outcome, chosen Alternative 1, and original references. Preserve the original questions in `contracts/status-model/approval-requests.md` as historical text; add a clearly dated **resolution index/addendum** instead of rewriting questions to pretend they were already approved.
2. Update `contracts/status-model/contract-status-model-v1.0.md`: distinguish **ratified interpretation constraints** from still `PROPOSED_NOT_RATIFIED` token sets and machine-readable model. Do not promote entire status model to canonical. Explain XC-11 remains OPEN, and no default precedence is established.
3. Update `contracts/status-model/legacy-status-mapping.md` with references to the decisions, retaining all 41 original phrase/context rows and all original literals; retain `REQUIRES_USER_DECISION` or equivalent **unresolved mapping** for cross-source status precedence. A decision to have *no precedence* is not a decision that historical statuses are equivalent.
4. Update `contracts/status-model/status-dimensions-draft-v0.json` **only if needed** to reference dated decisions and explicitly preserve `PROPOSED_NOT_RATIFIED`, `NON_RUNTIME_NON_WIRE_DRAFT`, no canonical enum and all false authorization flags. If no change is necessary, state why. Never introduce runtime schema or change registry.
5. Update `contracts/traceability-index.md`, `contracts/domain/unresolved-decisions.md`, and relevant README indexes minimally to link decision record and reflect SMQ-01..05 as decided while leaving underlying XC/UD issues OPEN. Do not change `contracts/source-manifest.json` if no source file was changed (expected: no change).
6. Do not assign `IMPLEMENTED`, `VERIFIED`, `APPROVED`, `ORIGINAL`, `REJECTED`, or `WITHDRAWN` to historical records solely due to these decisions. `DESIGN_BASELINE_RATIFIED_WITH_EXCEPTIONS` for DET-01/02 remains as before; their Q01–Q10 verification cells remain OPEN.

## 4. Documentation-only regression tests

Prefer extending `crates/trading-domain/tests/adfr_reconciliation.rs` and/or `registry_skeleton.rs` rather than creating a duplicate framework. Test, with robust semantic checks where possible:

- All five decision IDs present once, dated, mapped to correct SMQ and Alternative 1; no contradictory approval or implicit promotion.
- Original `approval-requests.md` alternatives remain traceable; interpretation layer explicitly refuses precedence and refuses closed lifecycle enum.
- ADFR JSON/CSV still 78/78, 78 `NOT_VERIFIED`, one superseded and 77 not superseded, same ID order and core fields; no changed bytes relative to baseline.
- All 12 source hashes/bytes match manifest; current vs package provenance unchanged; DET-01/02 remain reconstructed and unverified as sources.
- Registry SHA-256 unchanged, safety flags remain closed; proposed status-dimensions file is non-runtime/non-wire and not ratified.
- Local test claims do not promote ADFR verification or broker/DB/CI evidence. Evidence gate requires the listed fields and dated user decision, but **do not create fictitious evidence records**.
- Open XC-01, XC-02-PRED, XC-03-MATRIX, XC-04 remaining evidence, XC-05-LOCK, XC-06, XC-07-ROUND, XC-08-IBKR, XC-09-RESET, XC-10-ACTION, and XC-11 status-word remainder stay OPEN.

If the tests are already sufficient, do not add redundant code. Any test-only Rust/Cargo edit must be justified, remain outside `src/`, and not add production dependencies.

Run from repository root, report commands and exact exit codes:

```sh
cargo fmt --all -- --check
cargo check --workspace --locked
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
```

An ignored PostgreSQL integration test is **not passed**; remote CI is **not run**. On any failure, fix only within scope, rerun, and report both failure and rerun. Do not hide a failing gate.

## 5. Deliverables, report, commit

- Dated decision record `contracts/decisions/ARCH-DEC-04-bis-08-2026-11-15.md` (or explain safe alternative path if existing).
- Status-model decision references, non-destructive resolution index, updated traceability and documentation indexes.
- Focused documentation-only tests (if necessary), with explicit assertions that all safety gates stay closed.
- Archive this **exact prompt** as `docs/cursor-prompts/trading/TRD-ARCH-03A2-Ratify-Status-Interpretation-Decisions.md`.
- `docs/architecture/TRD-ARCH-03A.2-report.md` containing: baseline branch/HEAD/worktree; exact sources inspected; decisions and explicit exclusions; source and ADFR integrity; mapping and lifecycle impact; unresolved XC status; changed paths and hashes; tests and ignored/unrun checks; gate table; deviations; commit/push status. Do not claim report contains its own introducing commit SHA: provide commit SHA in Cursor's terminal output.
- Exactly **one local Git commit** after all gates pass, subject `TRD-ARCH-03A.2: Ratify Status Interpretation Decisions`. Do not push. If gates fail, do not commit as PASS; report blockers.

## 6. Acceptance gates

- **A03A2-G01:** Clean preflight, all source-manifest hashes valid, baseline registry/ADFR integrity recorded.
- **A03A2-G02:** SMQ-01..05 recorded with exact approved Alternative 1 and dated provenance; no extra user decisions invented.
- **A03A2-G03:** No historical status precedence chosen; XC-11 status-word conflict remains OPEN.
- **A03A2-G04:** No closed lifecycle enum ratified; supersession semantics unchanged.
- **A03A2-G05:** Scoped local testing claims only; all ADFR verification entries unchanged.
- **A03A2-G06:** Dated user approval plus appropriately scoped evidence required for any future verification promotion; no present promotion.
- **A03A2-G07:** Source provenance/manifest placement unchanged; no false `original` or `unknown` classifications.
- **A03A2-G08:** Original 78 ADFR IDs, order, JSON/CSV parity, 1 supersession and 12 source hashes preserved.
- **A03A2-G09:** All unresolved technical XC items and DET-01/02 exceptions remain OPEN; no design-to-runtime escalation.
- **A03A2-G10:** Registry bytes and all safety flags unchanged; no runtime, migrations, schemas or broker activity.
- **A03A2-G11:** Documentation tests and four Cargo checks pass; ignored DB test/remote CI honestly recorded.
- **A03A2-G12:** Report, exact archived prompt, and exactly one local commit; commit SHA reported in Cursor output, `PUSHED: NO`.

## 7. Stop conditions

STOP and return `BLOCKED` without silently correcting source data if: dirty worktree; hash mismatch; ADFR discrepancy; existing decision contradicts user-approved alternatives; impossible-to-preserve source provenance; a required change would modify trading runtime, historical sources, ADFR or registry; or tests cannot pass within documentation-only scope. Present exact evidence and smallest safe next action. No speculative repair.

**Next milestone is not authorized by this prompt:** ARCH-03B safety-critical contract design, schema publication, implementation, deployment, and live trading require separate review and approval.
