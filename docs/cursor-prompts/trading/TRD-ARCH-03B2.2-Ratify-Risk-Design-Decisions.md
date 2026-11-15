# Cursor Execution Prompt — TRD-ARCH-03B.2.2: Ratify Risk Design Decisions

**Date of user authorization:** 2026-11-15  
**Execution mode:** Documentation, traceability, documentation-only tests. **No runtime or trading implementation.**  
**Expected commit subject:** `TRD-ARCH-03B.2.2: Ratify Risk Design Decisions`  
**Git:** Exactly one new local commit at the end, **NO PUSH**.

## 0. Authority, scope and non-negotiable restrictions

The user has **explicitly and independently ratified** exactly fourteen options for the proposal `TRD-ARCH-03B.2`, enumerated in section 2. These are binding **DESIGN POLICY ONLY**, not executable contracts or evidence of implementation. Treat the ratification message dated 2026-11-15 as the authority; do not invent a signature, evidence URI, verification, source transcript or independent approval.

**Do not implement, approve, infer or enable:**
- Economic posting, a compensating financial ledger entry, reservation-to-exposure transfer, partial-fill accounting, economic-finality predicate, risk-reservation release, margin/netting/stress arithmetic, valuation/FX conversion, financial rounding, or new risk evaluation.
- OMS transitions, transition matrix edges, `OrderClosure` enum/tokens, broker terminal observations, broker uniqueness, completeness proofs, broker submissions, retries, cancels, modify, implicit cancel-replace, risk-reducing broker orders, failover or live trading.
- Canonical/wire schemas, database DDL or migrations, runtime API, adapter, feature flag, credential, deployment, CI claim, authority role, simulated second approver, audit table name, lock order, isolation level or a production dependency.
- Do not modify `crates/*/src`, `apps/*/src`, Cargo manifests or lockfiles, source manifest, its 12 source files, ADFR JSON/CSV, registry, existing ARCH-DEC-01..20 records, or the prior 03B.1/03B.1.2 proposal and resolution snapshots.
- `part_b=NO-GO`, `live_trading_authorized=false`, `canonical_schemas_published=false`, `broker_uniqueness_evidence=EVIDENCE_PENDING`, `XC-02-PRED=OPEN`. No evidence promotion. Existing `ORDER_STATE_MACHINE_IMPLEMENTED`, `STATE_TRANSITIONS_IMPLEMENTED`, `RISK_EVALUATION_IMPLEMENTED`, `FINANCIAL_ROUNDING_IMPLEMENTED` remain false.

**Critical interpretation:** ARCH-DEC-13 (OMSQ-05 B) quarantines **every proposed economic effect**, even a uniquely matched fill; ARCH-DEC-14 (OMSQ-06 A) approves **no executable economic-finality predicate** and forbids reservation release. This task does not supersede either rule. A future evidence reference, version field, test obligation, or review record is **not** a release predicate, an approval to post, a schema, or proof that a test ran.

## 1. Preflight — stop on inconsistency

1. Inspect `git status --short`, branch, `git rev-parse HEAD`, latest commit subject, toolchain versions. Require a clean baseline. Do not reset, clean, stash, or discard user changes. If dirty or predecessor missing, stop with `BLOCKED` and explain; do not commit.
2. Read, not merely grep, the relevant contracts and historical decisions:
   - `docs/architecture/TRD-ARCH-03B.2-report.md`, `docs/architecture/TRD-ARCH-03B.1.2-report.md`, `docs/architecture/TRD-ARCH-SYNC-01-report.md`, `docs/architecture/TRD-ARCH-SYNC-02-report.md`.
   - `contracts/design-proposals/TRD-ARCH-03B2-approval-requests.md` (all RISKQ-01..14 alternatives and binding-rule citations), `TRD-ARCH-03B2-risk-semantics-and-invariants.md`, `TRD-ARCH-03B2-economic-finality-and-release-coverage-draft.md`, `TRD-ARCH-03B2-reservation-and-correction-options.md`, `TRD-ARCH-03B2-dependency-and-conflict-matrix.md`, `TRD-ARCH-03B2-proposal-index.json`.
   - `contracts/decisions/ARCH-DEC-01-bis-03-2026-11-15.md`, `ARCH-DEC-04-bis-08-2026-11-15.md`, `ARCH-DEC-09-bis-20-2026-11-15.md`; `contracts/domain/unresolved-decisions.md`, `contracts/domain/glossary.md`, `contracts/traceability-index.md`, `contracts/status-model/`.
   - `contracts/source-manifest.json`, both `contracts/sources/ADFR-Consolidated-v1.0.*`, `contracts/registry/registry.json`, and relevant existing documentation-only tests.
3. Verify the predecessor report is present and that the 14 original RISKQ questions are all `USER_DECISION_PENDING` / `NOT_RATIFIED` **as historical creation-time text**. Their recommendations are not user approvals. Preserve their original alternatives, recommendations, and status in the historical section. Add a **separate dated resolution index/addendum**; do not retrospectively rewrite the original request.
4. Check `ARCH-DEC-21` through `ARCH-DEC-34` are all unused. If **any** collision, stop and report; never silently renumber or overwrite.
5. Record pre-edit SHA-256 and byte counts for protected artifacts, including the manifest, its 12 listed source files, registry, ADFR JSON/CSV, earlier decision files, and prior proposal/resolution index. Validate all 12 manifest hashes and byte counts. Verify ADFR JSON/CSV ordered ID parity: 78 unique IDs, 78 `NOT_VERIFIED`, exactly one superseded `ADFR-CROSS-005` by `ADFR-CR-B10-001`. Stop on discrepancy.
6. Preserve the previously reported registry SHA-256 `7abed2af5363726403175c6524bd7a4d05d36b8735ad48442fc26941f8d98197` unless preflight proves the repository legitimately differs; do not normalize or repair it in this task. Report mismatch and stop.

## 2. Exactly ratified user choices (do not substitute recommendations)

Create `contracts/decisions/ARCH-DEC-21-bis-34-2026-11-15.md` with 14 separately identifiable decisions, precise original question anchors, date, exact option and user wording, provenance, consequences, explicit prohibitions, dependencies, and open XC findings. Use identifiers in this exact mapping **only after the collision check**:

| Decision | Question | Option | Explicit user wording / binding interpretation |
|---|---|---|---|
| ARCH-DEC-21 | RISKQ-01 | **B** | **Zusätzlich harte Ablehnungen als Konjunktionsregel festlegen.** Keep six scope names (global, account, strategy, instrument, sector, risk factor); any hard deny blocks, soft allow never overrides hard deny. This is **composition shape only**, no metric catalogue, numeric algebra, netting/margin/stress rule, executable budget check. |
| ARCH-DEC-22 | RISKQ-02 | **A** | **Eine Reservierungs-ID, Idempotenz und atomare Multi-Scope-Reservierung.** One reservation identity, explicit order association, one command key; identical key+payload replays stored result, differing payload conflicts without second reservation; multi-scope all-or-nothing. No table, lock order, isolation or writer. |
| ARCH-DEC-23 | RISKQ-03 | **A** | **Keine Reservierung bei fehlenden oder veralteten Daten.** Missing/stale/unknown/incomparable price, liquidity, FX or required valuation input => no reservation and no ALLOW. No fallback price, silent 1:1 FX, invented staleness threshold, or new scope halt. |
| ARCH-DEC-24 | RISKQ-04 | **A** | **Fill erfassen; gesamte Reservierung halten; keine Umbuchung.** Retain fill as evidence, preserve entire original reservation, no reservation-to-exposure conversion, no residual release. Economic effects remain quarantined. |
| ARCH-DEC-25 | RISKQ-05 | **A** | **Restverpflichtung bleibt unverändert.** Preserve cancel request, broker cancel observation, and fills as separate facts; incomplete snapshot cannot shrink or release residual obligation. No cancel dispatch or terminal edge. |
| ARCH-DEC-26 | RISKQ-06 | **A** | **Alle Fakten behalten und wirtschaftliche Effekte quarantänisieren.** Retain late fills, duplicates, busts, corrections; quarantine every proposed economic effect, including uniquely matched events; original closure immutable; **do not draft compensating ledger line as ratified design**. |
| ARCH-DEC-27 | RISKQ-07 | **A** | **Neue risikosteigernde Aktionen blockieren; keine Broker-Aktion.** Deny new risk-increasing commands on insufficient budget; for evidence of already-held limit breach, require scoped block/incident policy at design level; no broker cancel/reduce, no reset authority, no incident enum. Do not imply the runtime block is implemented. |
| ARCH-DEC-28 | RISKQ-08 | **A** | **Keine Nachweisliste als verbindlich übernehmen.** No candidate evidence checklist is ratified as a sufficiency or acceptance bar; economic finality undefined; 21-row coverage draft stays unratified. |
| ARCH-DEC-29 | RISKQ-09 | **A** | **Keinen Freigabenachweis als ausreichend anerkennen.** No release operation or release proof accepted; reservation release remains forbidden. No implicit checklist from unselected B. |
| ARCH-DEC-30 | RISKQ-10 | **B** | **Künftige Bewertungs- und Policy-Referenzen versionierbar machen.** Future valuation evidence should be able to cite instrument-policy version, market snapshot, FX snapshot, pricing method, rounding-policy version, base currency and as-of time. **Citation obligations only**, not concrete fields/schema, valuation algorithm, FX rate, rounding mode, quantum, or freshness number. |
| ARCH-DEC-31 | RISKQ-11 | **B** | **Zusätzlich verpflichtende zukünftige Concurrency- und Crash-Tests.** Any later proposed protocol must demonstrate multi-scope budget contention, order-version CAS, crash before/after commit, conflict/retry behavior through actual tests. No lock order or isolation level adopted; tests not executed. Retry remains local only, never broker resend. |
| ARCH-DEC-32 | RISKQ-12 | **A** | **Separaten transaktionalen Auditdatensatz verlangen, ohne Tabellennamen.** Future reservation mutation requires distinct authoritative audit record in the same local transaction; if audit cannot write, mutation fails. Outbox/export not authority. Retain reconciliation evidence while booking blocked. No DDL/table name/hash chain. |
| ARCH-DEC-33 | RISKQ-13 | **A** | **Keine Rollen oder Freigabewege einführen; Quarantäne bleibt.** No adjudication role, writer, workflow, fake second approver or posting. If future two-person rule requires a second person and unavailable, deny. Manual review alone never posts. |
| ARCH-DEC-34 | RISKQ-14 | **B** | **Zusätzlich unveränderliche Policy-Versionen und negative Tests verlangen.** Future risk policy changes require a new immutable version ID; historical evaluations preserve their cited version. Require future negative/property/crash/replay tests, marked `NOT_EXECUTED` until genuinely run. Recovery neither releases risk nor resubmits broker orders; identical-payload replay returns stored result only. No version migration or code. |

**Status wording:** use clear local record metadata such as `RATIFIED_DESIGN_POLICY_ONLY`, `NOT_IMPLEMENTED`, `NOT_VERIFIED`; do **not** present these as canonical status enums or overwrite the ADFR verification column. Source provenance of reconstructed DET-01/DET-02 stays `RECONSTRUCTED_DRAFT`, `SOURCE_REVIEW_PENDING`, `reviewed=false`, `approved=false`; design baseline with exceptions stays binding under ARCH-DEC-01, but no false verification promotion.

**Consistency rules:**
- RISKQ-01 B's logical conjunction applies only to hard-deny composition; it is not a universal mathematical aggregation or a numeric approval predicate.
- RISKQ-02 A is an identity/atomicity **design requirement**, not permission to create a reservation or to commit partial scope effects.
- RISKQ-04 A + RISKQ-05 A + RISKQ-06 A prohibit converting, shrinking or releasing holds even after a fill, cancel observation or unique broker match.
- RISKQ-08 A + RISKQ-09 A **do not define any economic-finality predicate** and cannot be interpreted as permitting release when evidence arrives.
- RISKQ-10 B's seven references are not a canonical valuation schema, numeric policy or rounding approval.
- RISKQ-11 B requires future tests but **does not claim** existing tests prove serializability/deadlock freedom; the ignored PostgreSQL test is not a pass.
- RISKQ-12 A repeats existing ARCH-DEC-19 audit authority and does not resolve `XC-06` table-name dispute.
- RISKQ-13 A does not grant manual adjudication/posting authority, and cannot supersede ARCH-DEC-13.
- RISKQ-14 B pins future immutable policy lineage, not a live migration, new schema, or replay authorization beyond identical-payload idempotence.

## 3. Documentation and traceability deliverables

1. Create the single dated decision record `contracts/decisions/ARCH-DEC-21-bis-34-2026-11-15.md`. Include a table from all 14 RISKQ IDs to all 14 ARCH-DEC IDs, original user wording, option, scope, constraints, open findings, and explicit **NO IMPLEMENTATION / NO VERIFICATION** statements. Quote source anchors accurately; do not invent evidence or signatures.
2. Append a clearly separated **2026-11-15 resolution addendum** to `contracts/design-proposals/TRD-ARCH-03B2-approval-requests.md`, retaining all original questions, alternatives, comparison tables and historical `USER_DECISION_PENDING` lines. Link each question to the new decision. Keep `TRD-ARCH-03B2-proposal-index.json` unchanged as its historical `NOT_RATIFIED` creation-time snapshot.
3. Create `contracts/design-proposals/TRD-ARCH-03B22-resolution-index.json` (or a consistent documentation-only filename, clearly declared). It is `NON_RUNTIME_NON_WIRE_RESOLUTION_INDEX`, not a canonical schema. Include ratification date, source approval document, 14 decision IDs, 14 exact option choices, previous decision dependencies, and explicit flags: `implementation_authorized=false`, `risk_release_approved=false`, `economic_posting_approved=false`, `economic_finality_predicate_approved=false`, `oms_transitions_approved=false`, `broker_dispatch_approved=false`, `canonical_schemas_published=false`, `live_trading_authorized=false`, `closes_xc_findings=false`. Do not invent a signature URI.
4. Update only necessary navigation/traceability documents: `contracts/design-proposals/README.md`, `contracts/traceability-index.md`, `contracts/domain/unresolved-decisions.md`, `docs/architecture/README.md`, optionally `contracts/domain/glossary.md` if required. Keep the historic original question text and prior ARCH-DEC files byte-identical. If any design proposal text is updated, only append an explicitly labeled resolution reference; do not rewrite original unratified comparison as if previously binding.
5. Preserve open status of `XC-01`, `XC-02-PRED`, `XC-03-MATRIX`, remaining `XC-04`, `XC-05-LOCK`, `XC-06`, `XC-07-ROUND`, `XC-08-IBKR`, `XC-09-RESET`, `XC-10-ACTION`, and status-word remainder of `XC-11`. Respect previously resolved subfindings and CR-B01 precedence from ARCH-DEC-03. Preserve `XC-02-ALIGN`, `XC-03-BOOK`, `XC-05-OUTBOX`, `XC-07-FX`, `XC-08-RULE`, `XC-09-SOLO`, `XC-10-RACE` as **principles, not executable proofs**.
6. Archive **this exact executed prompt** under `docs/cursor-prompts/trading/TRD-ARCH-03B2.2-Ratify-Risk-Design-Decisions.md`. If the supplied prompt is already present in the repository, verify contents rather than duplicating.

## 4. Documentation-only regression tests

Extend the existing **documentation-only** Rust tests in `crates/trading-domain/tests/adfr_reconciliation.rs` and, if necessary, `crates/trading-domain/tests/registry_skeleton.rs` and their explicit allowlists. Do not change production code. Tests should be semantic/structured, not fragile substring counts, and verify:

- Exactly 14 distinct decision mappings `ARCH-DEC-21..34` ↔ `RISKQ-01..14`; options precisely `B,A,A,A,A,A,A,A,A,B,B,A,A,B`, each with ratification date and source anchor.
- New resolution index is explicitly non-runtime/non-wire and **cannot** be confused with the historical proposal index, registry or canonical schema. Original RISKQ alternative text and historical `NOT_RATIFIED` snapshot unchanged.
- `RISKQ-01 B` does not adopt arithmetic; `RISKQ-10 B` does not adopt rounding/FX; `RISKQ-11 B` does not adopt locks/isolation or mark tests executed; `RISKQ-14 B` does not authorize a migration/release/resubmit.
- `ARCH-DEC-13` quarantine and `ARCH-DEC-14` no-predicate/no-release remain binding. No booking, conversion, release, transition, broker dispatch or live-trading flag turns true.
- 78 ADFR unique IDs, ordered JSON/CSV parity, 78 `NOT_VERIFIED`, exactly one known supersession, 12/12 manifest source hashes and sizes; all protected hashes identical to preflight.
- Existing historical decision files ARCH-DEC-01..20, registry, source manifest, sources and ADFR files remain byte-identical; no source or canonical registry publication.

Do not use a test name or a passing documentation assertion to imply financial safety, transaction atomicity, broker behavior, deadlock freedom or external verification.

## 5. Local validation commands

Run from the repository root, recording commands, exit codes, failures and corrections:

```powershell
cargo fmt --all -- --check
cargo check --workspace --locked
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
```

If formatting fails on newly edited test code, run `cargo fmt --all` **only after inspecting its diff**, then rerun the complete four-command suite. Do not format unrelated files. Record the ignored database test explicitly as **IGNORED / NOT EXECUTED**, never passed. Remote CI **NOT RUN** unless actually run and evidenced. Validate JSON parsing, 14 mappings, all safety flags, source-manifest SHA-256 and byte counts, and pre/post protected hashes using an auditable local script or test. Record what was actually executed; do not fabricate test results.

## 6. Acceptance gates — all 12 must PASS, otherwise BLOCKED/PARTIAL

| Gate | Required evidence |
|---|---|
| A03B22-G01 | Clean baseline, predecessor present, no ARCH-DEC ID collision; preflight source and registry integrity. |
| A03B22-G02 | Exactly 14 user-ratified choices mapped to ARCH-DEC-21..34; date and source authority correct, no invented approval. |
| A03B22-G03 | Historical RISKQ requests and proposal index unchanged; dated resolution addendum/index clearly separated. |
| A03B22-G04 | 78 ADFR IDs and 78 `NOT_VERIFIED`, one supersession, 12/12 manifest sources, byte-for-byte protected files. |
| A03B22-G05 | RISKQ-01 B and RISKQ-02 A recorded as policy shape/atomicity only; no budget math or reservation writer. |
| A03B22-G06 | RISKQ-03..07 choices recorded; missing data, partial fill, cancel, corrections, breach stay fail-closed, no economic effect. |
| A03B22-G07 | RISKQ-08 A, 09 A, 10 B: no finality predicate or release; only future valuation evidence references. |
| A03B22-G08 | RISKQ-11 B, 12 A, 13 A, 14 B: future tests, audit, no roles, immutable versions; no protocol/table/migration. |
| A03B22-G09 | Traceability, open XC findings and precedence intact; ARCH-DEC-01..20 binding rules unweakened. |
| A03B22-G10 | No production source, DDL, schema, dependency, broker or runtime changes; all safety flags remain disabled. |
| A03B22-G11 | Final fmt/check/test/clippy pass, documentation validation pass; ignored integration test and remote CI honestly labeled. |
| A03B22-G12 | Prompt archive and full report included in exactly one local Git commit; no push; clean post-commit worktree. |

## 7. Final report, commit and no push

Create `docs/architecture/TRD-ARCH-03B.2.2-report.md` **before** committing. Include:
- `VERDICT: PASS | PARTIAL | BLOCKED` (PASS only if every gate actually passes), `PUSHED: NO`, date, branch, baseline HEAD, predecessor report and prompt paths, tool versions, and final commit subject.
- Fourteen-row decision mapping with exact choices and status `RATIFIED_DESIGN_POLICY_ONLY / NOT_IMPLEMENTED / NOT_VERIFIED`; source citations and open-XC crosswalk.
- File-by-file changed paths, hashes and byte counts, protected pre/post hashes, 12 manifest checks, 78 ADFR parity, registry flags.
- All executed commands and exit codes, passed/failed/ignored counts, database-test environment status, remote CI status; initial failures and fixes, if any.
- Twelve gate results, deviations, limitations, no evidence fabrication, no trading authorization.
- Explain that the report cannot contain the SHA of its **own introducing commit** without a self-reference; capture the actual SHA in the Cursor terminal output after commit and report it to the user.

Stage **only** the intended documentation, documentation-only tests, prompt archive and report. Inspect `git diff --check`, `git diff --stat`, `git diff --cached --name-only` and staged diff for protected paths and unexpected changes. If any gate fails, stop and **do not make a success commit**; provide a blocked report and request instruction. Otherwise create exactly one local commit:

```powershell
git commit -m "TRD-ARCH-03B.2.2: Ratify Risk Design Decisions"
```

After commit run `git rev-parse HEAD`, `git log -1 --format="%h %s"`, `git status --short`, `git show --stat --oneline HEAD`; verify clean worktree and all required files are in that commit. **Never run `git push`.**

## 8. Cursor final response format

Return concise German summary with `VERDICT`, 12/12 gate count, 14 ratified mappings, tests passed/failed/ignored, remote CI status, all prohibitions, changed-file summary, actual **new commit SHA**, `PUSHED: NO`, and link/path to `docs/architecture/TRD-ARCH-03B.2.2-report.md`. Explicitly state **ARCH-03B.3 NOT STARTED** and that broker identity/snapshot completeness and economic-finality predicate remain unresolved.
