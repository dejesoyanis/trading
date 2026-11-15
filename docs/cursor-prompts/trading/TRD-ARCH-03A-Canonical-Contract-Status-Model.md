# TRD-ARCH-03A: Canonical Contract Status Model & Verification Semantics

**Mode:** DOCUMENTATION + DOCUMENTATION-ONLY TESTS. **No trading implementation.**
**Predecessor:** TRD-ARCH-SYNC-02 (reported PASS, not independently audited).
**Objective:** Establish an unambiguous, machine-checkable vocabulary for architecture decision lifecycle, design ratification, implementation state, and evidence-based verification without conflating these dimensions or silently changing existing decisions.

## 0. Non-negotiable safety and scope

- Live trading remains `NOT AUTHORIZED`; canonical wire schemas remain unpublished; `part_b=NO-GO`; broker uniqueness remains `EVIDENCE_PENDING`.
- Do **not** edit trading runtime (`crates/*/src`, `apps/*/src`), migrations, broker adapters, risk evaluation, OMS transitions, dispatch, failover, readiness, HTTP routes, secrets, or live/paper broker connections.
- Do **not** implement `OrderClosure`, `IncidentState`, rounding, release predicates, fencing, snapshot completeness, cancel/modify or other XC findings.
- Do not rewrite historical primary-source text to make statuses agree. Preserve original words, dates, provenance, hashes, and source revisions. Add separate interpretations and explicit mappings.
- Documentation tests may be added under `tests/` or existing test locations only if necessary; changes to Cargo metadata/dependencies require a justification and must not introduce production dependencies.
- Do not push. One commit after all gates pass; archive this exact prompt in `docs/cursor-prompts/trading/`.

## 1. Preflight and evidence capture

1. Confirm clean worktree, branch, HEAD, `git status --short`, Rust toolchain and baseline flags. If dirty, stop and report; do not stash/reset/discard.
2. Read **actual files**, not summaries: `docs/architecture/TRD-ARCH-SYNC-01-report.md`, TRD-ARCH-SYNC-02 report (locate by filename), `contracts/decisions/ARCH-DEC-01-bis-03-2026-11-15.md`, `contracts/source-manifest.json`, `contracts/traceability-index.md`, `contracts/domain/unresolved-decisions.md`, `contracts/domain/glossary.md`, `contracts/registry/registry.json`, `contracts/sources/ADFR-Consolidated-v1.0.json`, `.csv`, `contracts/sources/ADFR-README.md`, CR-B10, DET-01/02, DET-07, CR-B01, and architecture consolidation. Locate each by exact path or documented equivalent; record absences, do not fabricate.
3. Verify source-manifest current hashes and bytes against actual files. Distinguish original ZIP/package hashes from current amended-file hashes. Capture JSON/CSV counts, ID sets, order, lifecycle, supersession graph and statuses. Stop on unexplained mismatch.
4. Read existing registry tests and any contract validation scripts before adding a parallel validator.

## 2. Define a multidimensional status contract

Produce `contracts/status-model/contract-status-model-v1.0.md` (or existing canonical documentation location if one already exists), with a **normative glossary** and **explicit separation** of:

- **Source provenance:** original, reconstructed, external evidence, unknown. Provenance is not approval.
- **Decision/design lifecycle:** proposed, open, ratified with exceptions, superseded, rejected/withdrawn where supported; document exact permitted transitions and required human evidence. Do not retroactively assign a state without authority.
- **Implementation state:** not implemented, partial, implemented (only with scoped evidence); independent of design approval.
- **Verification state:** not verified, evidence pending, tested locally, externally verified, failed/blocked as appropriate; specify evidence URI, method, environment, timestamp, scope and limitations. Passing local unit tests does not imply broker/DB correctness.
- **Safety/authorization state:** separate gate for production/live trading, canonical schema publication, broker identity uniqueness and dispatch readiness. All remain closed.
- **Severity/priority:** `P0`/`P1` is priority, not lifecycle. A source string such as `OPEN P0` is a composite historical label, not a synonym for `PROPOSED_FOR_ACCEPTANCE`.
- **Supersession:** historical records retained, one-way reference, acyclic links, successor existence, not automatic acceptance. Preserve 78 historical IDs, exactly one superseded (`ADFR-CROSS-005`), successor `ADFR-CR-B10-001`, 77 not superseded.

For every dimension define: purpose, allowed values, meanings, valid transitions, forbidden implications, evidence requirements, who may approve, and compatibility with existing `status`, `verification`, `lifecycle`, `supersedes`, `superseded_by` fields. Prefer a **non-destructive interpretation layer** rather than replacing fields in the existing ADFR schema. If the existing sources do not authorize a complete enum/transition set, label that part `PROPOSED_NOT_RATIFIED` and request user approval; do not claim it canonical.

## 3. Build a source-grounded mapping matrix

Create `contracts/status-model/legacy-status-mapping.md` listing each distinct status phrase encountered (at minimum `OPEN P0`, `OPEN P1`, `PROPOSED_FOR_ACCEPTANCE`, `CONFIRMED_DECISION`, `DESIGN_RESOLVED`, `DESIGN_BASELINE_RATIFIED_WITH_EXCEPTIONS`, `RECONSTRUCTED_DRAFT`, `NOT_VERIFIED`, `EVIDENCE_PENDING`, `SUPERSEDED`, `NOT_SUPERSEDED`, `NOT_IMPLEMENTED`, `SKELETON_NOT_CANONICAL`, `NO-GO`). For each: source location, historical literal, dimension(s), safe interpretation, whether mapping is definitive or provisional, and **what it does not imply**. Mark ambiguous or conflicting mappings `REQUIRES_USER_DECISION`; never resolve by arbitrary precedence.

Explicitly handle:
- XC-11 remaining status-word disagreement without altering original source statements.
- ARCH-DEC-01 DET-01/02 binding design baseline **with open exceptions**, while provenance remains reconstructed and Q01–Q10 verification remains open.
- ARCH-DEC-03 CR-B01 precedence for dispatch/failover **design only**, without implementing fencing or proving IBKR semantics.
- `DESIGN_RESOLVED` ≠ `VERIFIED` ≠ `IMPLEMENTED` ≠ `LIVE_AUTHORIZED`.
- `NO_CONFLICT_FOUND` on a principle ≠ permission to implement unresolved predicates or transitions.

## 4. Machine-readable companion (proposal only)

Create a small, explicitly **non-runtime, non-wire, draft** schema/example for the status dimensions in `contracts/status-model/`, with version, source references and validation rules. Reuse existing ADFR data rather than duplicating authoritative decisions. Do not mutate `contracts/registry/registry.json` or publish `registry.yaml`, `events/v1`, `commands/v1`, `api/v1`, `sql/v1` or canonical fixtures. Do not introduce new unapproved values as if ratified.

Define invariants: source hashes intact; 78 unique historical IDs and matching JSON/CSV order/core fields; exactly one supersession with existing successor; no cycles; `NOT_VERIFIED` never maps to verified; `RECONSTRUCTED_DRAFT` remains provenance for DET-01/02; no `live_trading_authorized=true`; no canonical schema publication; no automatic closure of XC findings.

## 5. Tests and validation

Implement or extend **documentation-only** tests for the above invariants, preferably using existing test tooling. Tests must fail on: mismatched JSON/CSV IDs; missing historical ID; broken supersession; unsupported approval promotion; loss of reconstructed provenance; accidental registry flag flip; invalid source-manifest current hash; accidental claim of implementation or verification. Preserve existing tests; avoid brittle full-document hashes where a semantic check is better, but retain provenance verification where required.

Run and report exact results:

```sh
cargo fmt --all -- --check
cargo check --workspace --locked
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
```

Report ignored database tests and remote CI as **not run**, never as passed. If a command fails, fix only in scope, rerun, and report the failure and correction.

## 6. Decision request and deliverables

Prepare `contracts/status-model/approval-requests.md` with numbered, minimal **decision questions** where semantics cannot be ratified from existing ARCH-DEC-01..03. Include 2–3 alternatives and a conservative recommendation per question, consequences, and exact source references. **Do not self-approve.**

Update `contracts/traceability-index.md`, `contracts/domain/unresolved-decisions.md`, and relevant documentation indexes with links and precise statuses. Preserve XC-01, XC-02-PRED, XC-03-MATRIX, XC-04 remaining evidence, XC-05-LOCK, XC-06, XC-07-ROUND, XC-08-IBKR, XC-09-RESET, XC-10-ACTION and remaining XC-11 status issue as open unless explicitly resolved by existing user decision; this prompt grants no new approval.

Create `docs/architecture/TRD-ARCH-03A-report.md` containing:
- baseline HEAD/branch/worktree; exact files inspected; source hash results;
- status model and mapping table summary; explicit **ratified vs proposed** distinctions;
- ADFR counts and supersession invariants;
- every unresolved decision and its exact ID;
- files changed and justification for any Cargo/test dependencies;
- commands, exits, test counts, ignored tests, CI status;
- unchanged safety flags and explicit statement no runtime behavior changed;
- acceptance-gate results, deviations, commit SHA from CLI output (report cannot self-contain its introducing commit SHA).

## 7. Acceptance gates

- **A03A-G01:** Clean baseline and verified source-manifest current hashes; no source provenance lost.
- **A03A-G02:** Distinct lifecycle, provenance, implementation, verification, severity and authorization dimensions documented.
- **A03A-G03:** Every legacy status phrase mapped with source and ambiguity indicated, no silent equivalence.
- **A03A-G04:** DET-01/02 ratification with exceptions preserved, provenance unchanged.
- **A03A-G05:** CR-B01 design precedence preserved; unsafe takeover never authorized.
- **A03A-G06:** 78 historical ADFR IDs; 77 not superseded; exactly one supersession; JSON/CSV unchanged unless an independently justified, explicitly approved correction is necessary (default: no edit).
- **A03A-G07:** Draft machine-readable status model cannot be mistaken for runtime or canonical wire schema.
- **A03A-G08:** Documentation tests cover regression invariants and pass.
- **A03A-G09:** All unresolved XC items retained; unratified choices recorded as approval requests, not decisions.
- **A03A-G10:** `contracts/registry/registry.json` byte-identical; `SKELETON_NOT_CANONICAL`, `part_b=NO-GO`, `canonical_schemas_published=false`, `broker_uniqueness_evidence=EVIDENCE_PENDING`, `live_trading_authorized=false`.
- **A03A-G11:** Four Cargo checks pass; skipped integration/CI clearly identified.
- **A03A-G12:** Prompt archived, complete report, one commit, no push.

## 8. Commit discipline

After all gates pass, archive this prompt exactly as executed and create **one commit** titled:

`TRD-ARCH-03A: Define Contract Status Model and Verification Semantics`

If any gate cannot pass, report `PARTIAL`/`NO-GO` and do **not** claim PASS; preserve work without destructive Git operations. Do not push. Return the report, changed-file list, commit SHA and remaining user approval questions. **Do not start ARCH-03B automatically.**
