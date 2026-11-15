# TRD-ARCH-SYNC-01 — Architecture Source Integration & Cross-Contract Validation

**Mode:** documentation-only, fail-closed, no functional changes. **Source package:** `TRD-ARCH-SYNC-01-Source-Package-v1.0.zip` (provided separately). **Do not use internet content as replacement for project source contracts.**

Archived from the Cursor task that produced this integration. The archive is the instruction. It is not evidence that the instruction was approved as a trading design, and it is not authorization to trade.

## Objective

Integrate the provided architecture source documents into the existing Rust Trading Platform repository with verifiable provenance, traceability, and an explicit conflict report. Do not claim that a document's presence means it was reviewed, approved, tested, or implemented. The user has not authorized trading, canonical wire publication, or behavioral changes.

## Preflight (stop on failure)

1. Confirm repository, branch, clean worktree, HEAD and latest TRD-M002A.2 commit. If dirty, report and STOP; never discard changes.
2. Inventory `contracts/sources`, `contracts/traceability-index.md`, `contracts/registry`, `docs/architecture`, existing prompts/reports, and existing architecture decisions. Record hashes and versions.
3. Locate the supplied ZIP. Verify every file against the included SHA-256 manifest. If missing or mismatched, STOP. Do not fabricate absent sources.
4. Compare duplicate sources already present in repository (`CR-B10`, `M0-02A`): preserve existing originals and compare bytes/content. Never overwrite on conflict; report as CONFLICT_REQUIRES_REVIEW.

## Scope

A. Place the 12 supplied source documents under `contracts/sources/` using stable original filenames. For duplicates with identical hashes, reuse existing files. For different hashes, retain both versions in an explicit review area or STOP; never silently replace canonical/previous content. Include ADFR JSON and CSV as source artifacts; validate JSON parses and CSV can be read, but do not claim semantic equivalence unless checked.

B. Produce `contracts/source-manifest.json` with source filename, SHA-256, provenance, status, referenced DET/CR IDs, source type, and review date if actually reviewed. Distinguish `PRESENT`, `RECONSTRUCTED_DRAFT`, `REVIEWED`, `APPROVED`, `IMPLEMENTED`, `EVIDENCE_PENDING` as separate dimensions, not a single status. DET-01/02 are reconstructed drafts (`SOURCE_REVIEW_PENDING`); they are not the missing original Q&A transcripts.

C. Update `contracts/traceability-index.md` for DET-01..DET-10, CR-B01..CR-B10, ADFR. The combined DET-03..08 and DET-09..10 documents cover multiple IDs; map each ID to the precise heading/section and document. Preserve historical `NOT_PROVIDED` information as a prior snapshot, then mark `PRESENT / NOT_REVIEWED` until substantive cross-review is complete. Do not claim the 77 ADFR entries are correct solely because the register parses.

D. Perform a read-only cross-contract review: compare identity scope, risk reservation finality, OMS late fills, dispatch epoch fencing, outbox/audit transaction boundaries, market data/FX/rounding, broker snapshot completeness, incident/readiness/reset, cancel/modify. Record each finding with `ID`, conflicting source and exact section, severity P0/P1/P2, rationale, proposed resolution, status (`OPEN`/`NO_CONFLICT_FOUND`), and whether a design decision needs user approval. Never silently resolve conflicts by inventing a rule. Document missing source evidence and unverified IBKR semantics.

E. Check CR-B10 domain names against current Rust types and registry skeleton; create a **gap matrix only**. Do not implement types, change serde, publish schema files, change runtime flags, or mark schema as canonical. Retain `SKELETON_NOT_CANONICAL`, `part_b=NO-GO`, `EVIDENCE_PENDING`, `live_trading_authorized=false`.

F. Add documentation `docs/architecture/TRD-ARCH-SYNC-01-report.md`: baseline SHA, inventory, file hashes, coverage by ID, unresolved conflicts, verified vs unverified claims, tests, deviations, changed files, next-step recommendation. Archive this prompt at `docs/cursor-prompts/trading/TRD-ARCH-SYNC-01-Architecture-Source-Integration.md`.

## Mandatory safety constraints

- Documentation and metadata only. No database migrations, HTTP changes, new trading endpoints, order state transitions, risk evaluations, broker connections, dispatch, credentials, live flags, or canonical event/command schemas.
- Do not promote DET-01/02 reconstructed drafts to user-ratified primary source texts. Reference designs and vocabulary gaps remain open.
- Existing `M0-02A` resolution and registry status remain unchanged unless a documented metadata correction is strictly necessary; do not publish golden bytes or compatibility fixtures.
- ADFR count must be computed from the actual JSON and compared with the stated 77; mismatches become findings, not silent edits.
- Any uncertainty affecting financial safety stays fail-closed. No automatic approval of unresolved cross-contract conflicts.

## Acceptance gates

- A01: Clean baseline, documented HEAD, no destructive Git operations.
- A02: ZIP and manifest SHA-256 verified for all source files.
- A03: All DET-01..10 and CR-B01..10 map to present documents with section-level references, and ADFR is present.
- A04: DET-01/02 flagged reconstructed draft; all unreviewed documents distinguish presence from approval.
- A05: Existing duplicates compared; no silent overwrites or false provenance.
- A06: Cross-contract review covers the ten domains in section D, with explicit OPEN findings where evidence is missing.
- A07: Rust-vs-contract gap matrix produced; no code changes.
- A08: Registry remains skeleton; no published canonical schema, verified uniqueness, or trading authorization.
- A09: `cargo fmt --all -- --check`, `cargo check --workspace --locked`, `cargo test --workspace --locked`, `cargo clippy --workspace --all-targets --locked -- -D warnings` executed and exit codes recorded; DB integration ignored must be reported as ignored; remote CI not run unless actually run.
- A10: Full report, source manifest, updated index and prompt archive committed; report includes exact diff summary and skipped checks.

## Execution / commit policy

Execute in phases: preflight → integrate & hash → traceability → cross-review → verify → report → commit. One commit only after passing applicable gates. Commit subject exactly `TRD-ARCH-SYNC-01: Integrate Architecture Sources and Traceability`. Do not push. If any required gate fails, do not claim PASS; produce blocker report and stop before commit unless documenting the failure itself is explicitly approved. Report resulting commit SHA in final Cursor response (not by modifying the already committed report).

## Supplied package path

`C:\Users\mifur\Downloads\TRD-ARCH-SYNC-01-Source-Package-v1.0.zip`
