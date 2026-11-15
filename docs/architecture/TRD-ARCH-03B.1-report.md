# TRD-ARCH-03B.1 report

**Date:** 2026-11-15  
**Mode:** design and documentation, plus documentation-only tests. No trading behavior was added.  
**Live trading:** NOT AUTHORIZED  
**Canonical schemas:** not published  
**OMS transitions:** none approved  
**OrderClosure vocabulary:** none approved  
**Remote CI:** remote CI was not run

`VERDICT: PASS`

`PUSHED: NO`. The commit SHA is the commit that adds this report. It is named in the Cursor output because this file cannot contain the hash of the commit that introduces it. The gates below are the basis for the verdict.

## Baseline

| Item | Value |
| --- | --- |
| Branch | `main` |
| Worktree before edits | clean. `git status --short` printed nothing. |
| BASELINE_HEAD | `7537d628a75dafb8aa751e90ce0f26463bd4b69b` |
| HEAD subject | `TRD-ARCH-03A.2: Ratify Status Interpretation Decisions` |
| Previous commit | `a27156785a4711628f324a11c4c2944dbbabe8a0` `TRD-ARCH-03A: Define Contract Status Model and Verification Semantics` |
| Predecessor report | `docs/architecture/TRD-ARCH-03A.2-report.md` present, verdict PASS in that file |
| Predecessor prompt | `docs/cursor-prompts/trading/TRD-ARCH-03A2-Ratify-Status-Interpretation-Decisions.md` present |
| rustc | 1.99.1 (58cf8f9c3 2026-10-13) |
| cargo | 1.99.1 (5f94df478 2026-08-27) |
| Destructive git | none. Nothing was discarded, reset, or stashed. |

Registry flags at baseline were `SKELETON_NOT_CANONICAL`, `canonical_schemas_published=false`, `part_b=NO-GO`, `live_trading_authorized=false`, `broker_uniqueness_evidence=EVIDENCE_PENDING`. Registry SHA-256 `7abed2af5363726403175c6524bd7a4d05d36b8735ad48442fc26941f8d98197` (44332 bytes) is unchanged.

## Files inspected

| Requested item | Path used |
| --- | --- |
| TRD-ARCH-SYNC-01 report | `docs/architecture/TRD-ARCH-SYNC-01-report.md` |
| TRD-ARCH-SYNC-02 report | `docs/architecture/TRD-ARCH-SYNC-02-report.md` |
| TRD-ARCH-03A report | `docs/architecture/TRD-ARCH-03A-report.md` |
| TRD-ARCH-03A.2 report | `docs/architecture/TRD-ARCH-03A.2-report.md` |
| ARCH-DEC-01..03 | `contracts/decisions/ARCH-DEC-01-bis-03-2026-11-15.md` |
| ARCH-DEC-04..08 | `contracts/decisions/ARCH-DEC-04-bis-08-2026-11-15.md` |
| DET-01 | `contracts/sources/DET-01-Order-Workflow-and-Finality-Contract-v1.0.md` |
| DET-02 | `contracts/sources/DET-02-Risk-Contract-v1.0.md` |
| DET-03..08 | `contracts/sources/DET-03-bis-DET-08-Architecture-Contracts-v1.0.md`, including DET-03, DET-04, DET-06, DET-07 (`### Kritische Dispatch-Sequenz`, `**Grenze des Epoch-Fencing:**`), and DET-08 |
| DET-09..10 | `contracts/sources/DET-09-bis-DET-10-und-Cross-Contract-Review-v1.0.md` |
| CR-B01..04 | `contracts/sources/P0-Resolution-Contracts-CR-B01-bis-CR-B04-v1.0.md` |
| CR-B05..09 | `contracts/sources/P1-Resolution-Contracts-CR-B05-bis-CR-B09-v1.0.md` |
| CR-B10 | `contracts/sources/CR-B10-Domain-Glossary-and-Schema-Registry-v1.0.md` |
| Consolidation | `contracts/sources/Architecture-Consolidation-and-Acceptance-v1.0.md` |
| Traceability index | `contracts/traceability-index.md` |
| Unresolved decisions | `contracts/domain/unresolved-decisions.md` |
| Glossary | `contracts/domain/glossary.md` |
| Status model | `contracts/status-model/contract-status-model-v1.0.md`, `approval-requests.md`, `legacy-status-mapping.md`, `status-dimensions-draft-v0.json` |
| Source manifest | `contracts/source-manifest.json` |
| ADFR | `contracts/sources/ADFR-Consolidated-v1.0.json`, `contracts/sources/ADFR-Consolidated-v1.0.csv`, `contracts/sources/ADFR-README.md` |
| Registry | `contracts/registry/registry.json`, `contracts/registry/README.md` |
| Draft labels | `crates/trading-domain/src/state.rs`, `version.rs`, `identity.rs` (read only) |
| Existing tests | `crates/trading-domain/tests/adfr_reconciliation.rs`, `crates/trading-domain/tests/registry_skeleton.rs` |

No requested file was missing. DET-07 and CR-B01 are sections inside the combined source files, not separate paths. No content was invented for an absent file.

## Source and ADFR integrity

Before any edit, all 12 files in `contracts/source-manifest.json` matched that manifest's current `sha256` and `bytes`. Mismatches: 0. `package_sha256` and `package_bytes` remain the TRD-ARCH-SYNC-01 intake values. Current and package hashes differ only for the three files whose `placement` is `AMENDED_BY_ARCH_DEC_02`. Those files were not edited. DET-01 and DET-02 stay `reconstructed_draft` true, `SOURCE_REVIEW_PENDING`, `reviewed` false, and `approved` false.

`contracts/source-manifest.json` was not edited. The 12 manifest source files were not edited. `contracts/registry/registry.json` was not edited.

| Item | Value, unchanged from baseline |
| --- | --- |
| JSON count | 78 |
| CSV count | 78 |
| Unique IDs, same order | 78 |
| `verification` | 78 `NOT_VERIFIED` |
| Superseded | 1: `ADFR-CROSS-005`, `superseded_by` `ADFR-CR-B10-001` |
| Not superseded | 77 |
| JSON SHA-256 | `272a6dabdb411701a25dd708577dbb498c7a7cd2754d83c612e85454a178a1d5` (29880 bytes) |
| CSV SHA-256 | `b5577302650e3d497465b93063d149313ec1197a38ddad3a9e7a6f223757ec32` (13138 bytes) |
| JSON root `status` | `ARCHITECTURE_DRAFT_NOT_IMPLEMENTED` |
| Registry SHA-256 | `7abed2af5363726403175c6524bd7a4d05d36b8735ad48442fc26941f8d98197` (44332 bytes) |

## Binding principles and proposals

Binding, and not widened by this package: ARCH-DEC-01 through ARCH-DEC-08. DET-01 Q01–Q10 are baseline principles with verification cells still OPEN. The `OrderClosure` vocabulary, allowed edges, and reference DDL stay inside the ARCH-DEC-01 exceptions. ARCH-DEC-03 keeps CR-B01 ahead of conflicting or less restrictive DET-07 text. ARCH-DEC-04 still assigns no precedence among `OPEN P0`, `PROPOSED_FOR_ACCEPTANCE`, and `DESIGN_RESOLVED`.

`NO_CONFLICT_FOUND` rows from TRD-ARCH-SYNC-01 stay principle-only. They do not authorize an edge or a release. Reference attempt phases, incident states, and command class names stay reference text. Draft labels in `state.rs` stay draft labels.

This package proposes a dimension split, a coverage template, three vocabulary strategies, and OMSQ-01 through OMSQ-12. Every question is `USER_DECISION_PENDING`. Recommendations are marked not to be approvals. `XC-03-MATRIX` stays OPEN.

OMSQ-04 and OMSQ-12 are split so cancel/fill retention is not a vote on modify. OMSQ-10 and OMSQ-11 are split so the lock-order proposal is not a vote on the audit artifact. The task's tenth topic contained both of those pairs.

## Deliverables

| Path | Role |
| --- | --- |
| `contracts/design-proposals/README.md` | Package index. `NOT_RATIFIED`. |
| `contracts/design-proposals/TRD-ARCH-03B1-OMS-semantics-and-invariants.md` | Dimensions, identifiers, trust boundary, invariants. |
| `contracts/design-proposals/TRD-ARCH-03B1-transition-coverage-draft.md` | Label inventory and scenario families. No approved edge. |
| `contracts/design-proposals/TRD-ARCH-03B1-OrderClosure-vocabulary-options.md` | Three strategies. Non-binding recommendation is alternative C. |
| `contracts/design-proposals/TRD-ARCH-03B1-approval-requests.md` | OMSQ-01..OMSQ-12, each `USER_DECISION_PENDING`. |
| `contracts/design-proposals/TRD-ARCH-03B1-dependency-and-conflict-matrix.md` | XC map. `XC-03-MATRIX` OPEN. ARCH-03B.2 and ARCH-03B.3 not started. |
| `contracts/design-proposals/TRD-ARCH-03B1-proposal-index.json` | Non-runtime index for the documentation test. Not a registry and not a wire schema. |

## Decision requests

| Id | Question in short | Status |
| --- | --- | --- |
| OMSQ-01 | Closure row written once, or one original row plus an append-only correction that does not reopen the workflow | `USER_DECISION_PENDING` |
| OMSQ-02 | Vocabulary strategy: coarse families, rich reasons, or family plus independent terminality slots | `USER_DECISION_PENDING` |
| OMSQ-03 | Whether an unknown send outcome can close a workflow | `USER_DECISION_PENDING` |
| OMSQ-04 | Facts retained in a cancel/fill race, and the guards before closure or risk movement | `USER_DECISION_PENDING` |
| OMSQ-05 | Ledger effect of late fill, bust, and correction, with closure preserved | `USER_DECISION_PENDING` |
| OMSQ-06 | Evidence bar before economic finality can be considered. Release is not approved. | `USER_DECISION_PENDING` |
| OMSQ-07 | Coverage, negative-edge, and version bar for any future matrix | `USER_DECISION_PENDING` |
| OMSQ-08 | Who may adjudicate a replay or evidence conflict | `USER_DECISION_PENDING` |
| OMSQ-09 | Identity, completeness, and provenance before a broker observation is terminal | `USER_DECISION_PENDING` |
| OMSQ-10 | Whether the CR-B02 lock order stays an unadopted proposal | `USER_DECISION_PENDING` |
| OMSQ-11 | Which artifact is the in-transaction audit authority | `USER_DECISION_PENDING` |
| OMSQ-12 | Whether modify or cancel-replace may exist before broker and risk-delta evidence | `USER_DECISION_PENDING` |

## Open XC matrix

Still OPEN: XC-01, XC-02-PRED, XC-03-MATRIX, XC-05-LOCK, XC-06, XC-07-ROUND, XC-08-IBKR, XC-09-RESET, XC-10-ACTION. XC-11 status-word disagreement stays OPEN. XC-04 stays OPEN apart from design precedence (`RESOLVED_BY_ARCH_DEC_03`). XC-11 population and supersession stays `RESOLVED_BY_ARCH_DEC_02`.

Principle-only rows that stay `NO_CONFLICT_FOUND` and were not turned into implementations: XC-02-ALIGN, XC-03-BOOK, XC-05-OUTBOX, XC-07-FX, XC-08-RULE, XC-09-SOLO, XC-10-RACE.

ARCH-03B.2 would still be required for risk release, rebooking, and OMSQ-05/OMSQ-06. ARCH-03B.3 would still be required for broker identity, terminal observations, unknown-send evidence, and modify. Neither was started. No broker evidence was added.

## Non-implementation

No file under `crates/*/src` or `apps/*/src` was edited. No route, adapter, migration, credential, runtime flag, order or risk state machine, broker call, lease, audit write, or financial rounding was added. `Cargo.toml` and `Cargo.lock` were not changed. No production dependency was added. `ORDER_STATE_MACHINE_IMPLEMENTED` and `STATE_TRANSITIONS_IMPLEMENTED` stay false. `OrderClosure` stays absent. B10-T01 through B10-T12 and OMS-T01 through OMS-T12 are not passed. UD-01 through UD-09 status lines were not rewritten.

## Changed files

| Path | SHA-256 | Bytes |
| --- | --- | --- |
| `contracts/design-proposals/README.md` | `f69093dab4cc11255c0c6b3a5e80724ab1911fc55873f9546efa9a886dc2e7ea` | 1506 |
| `contracts/design-proposals/TRD-ARCH-03B1-OMS-semantics-and-invariants.md` | `80148a2ad3f7100dd4c27e3245f21996eb9929806b6b18df2f3a97a4dd7b530e` | 25741 |
| `contracts/design-proposals/TRD-ARCH-03B1-transition-coverage-draft.md` | `8ab7185c1a066ab15271692d5876448f538ef532ef20f64be9167368525b1b9d` | 18724 |
| `contracts/design-proposals/TRD-ARCH-03B1-OrderClosure-vocabulary-options.md` | `efcc9e2ced885700f8cd93657a7ba2aa4158725dd9d2612c77398512353780d7` | 16243 |
| `contracts/design-proposals/TRD-ARCH-03B1-approval-requests.md` | `8034db25a7749826e4898864e2557afab366ab0c390339b90f9f46e0be4fd7bb` | 33649 |
| `contracts/design-proposals/TRD-ARCH-03B1-dependency-and-conflict-matrix.md` | `ec6df5e73679c9866743e89c86d68a5fb2a92d33355fb035d1fab1219e3ecad1` | 8226 |
| `contracts/design-proposals/TRD-ARCH-03B1-proposal-index.json` | `96dc34c61db04bc0d4eb0576a98f787fddb5632816b903985b1163993afc73cc` | 3654 |
| `contracts/domain/unresolved-decisions.md` | `65d201254a5ae2eddef04afc893557984a86c948dd8f306e4ce1814a3f9ac773` | 18801 |
| `contracts/traceability-index.md` | `c845ff1661c98ef08baf363f208af8a43b6ae82608a9d096472bc10c6b07ed2a` | 20800 |
| `crates/trading-domain/tests/adfr_reconciliation.rs` | `f48ba4f6fd83110e2cb20157a9747053e6e70bf0110ab2c6e74b2ef9881d5d98` | 58559 |
| `crates/trading-domain/tests/registry_skeleton.rs` | `13f2fb5d0f980c8f871e9c67c7188bb90fbe09cae1eb6cfead4449ea92dafacb` | 16641 |
| `docs/architecture/README.md` | `ddeda9b6bdd0c82592931cc7725d11c7800de4873761d9d0015475d01d219ad1` | 12459 |
| `docs/cursor-prompts/trading/TRD-ARCH-03B1-OMS-Finality-and-OrderClosure-Design-Resolution.md` | `d3cdaf41764907250bca5d6b9d4fb955765e05d41883a9f5ad4b50c2e6ee0f27` | 14899 |

This report's own hash is not listed. No Cargo metadata change required a dependency justification.

## Local checks

Remote CI was not run. The ignored PostgreSQL test was not executed against a database. Ignored is not passed.

`cargo fmt --all -- --check` was run once before formatting and exited 1. The diff was layout in `crates/trading-domain/tests/adfr_reconciliation.rs` only. `cargo fmt --all` was then applied.

The first combined run after that formatting, from the repository root:

| Command | Exit code |
| --- | --- |
| `cargo fmt --all -- --check` | 0 |
| `cargo check --workspace --locked` | 0 |
| `cargo test --workspace --locked` | 101 |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | 0 |

The test failure was `registry_skeleton_stays_unpublished`. It rejected `contracts/design-proposals/TRD-ARCH-03B1-proposal-index.json` because the allowlist named only the registry, the manifest, the ADFR extract, and the status draft. The allowlist was extended to that one index, and the test now requires the index to say `NOT_RATIFIED`, `xc_03_matrix` `OPEN`, vocabulary and edges `NOT_APPROVED`, and the two safety flags false.

The four commands were run again after that test edit. None failed.

| Command | Exit code |
| --- | --- |
| `cargo fmt --all -- --check` | 0 |
| `cargo check --workspace --locked` | 0 |
| `cargo test --workspace --locked` | 0 |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | 0 |

`cargo test` result on the passing run: 131 passed, 0 failed, 1 ignored. The ignored test is `migration_is_explicit_and_readiness_is_read_only` in `crates/trading-persistence/tests/connectivity.rs`, because `TRADING_ALLOW_DATABASE_INTEGRATION_TESTS=1` and `TRADING_TEST_DATABASE_URL` were not set. It was not run against a database. `adfr_reconciliation` passed 11 tests, including `oms_finality_proposals_stay_unratified_and_registers_stay_closed`. `registry_skeleton_stays_unpublished` passed.

## Deviations

OMSQ-10 in the task list covered both lock order and audit authority. Those are XC-05-LOCK and XC-06. They are OMSQ-10 and OMSQ-11. Cancel/fill-race semantics and modify/replace authorization are OMSQ-04 and OMSQ-12. The split is explained in the approval-request file. Twelve questions remain independently pending.

`TRD-ARCH-03B1-proposal-index.json` is an extra documentation file. Section 6 asked for structured checks rather than brittle prose checks. The JSON is `NON_RUNTIME_NON_WIRE_PROPOSAL_INDEX` and `NOT_RATIFIED`. It is not `registry.json`. The registry-skeleton allowlist names it and checks those flags. No canonical schema directory was created.

No evidence file was created. `contracts/source-manifest.json`, `contracts/registry/registry.json`, the 12 package source files, and `contracts/decisions/` were not edited.

## Gates

| Gate | Result |
| --- | --- |
| A03B1-G01 | PASS. Clean baseline. Predecessor commit `7537d628a75dafb8aa751e90ce0f26463bd4b69b` and its report confirmed. 12/12 current manifest hashes match. Registry baseline recorded. |
| A03B1-G02 | PASS. ARCH-DEC-01..08 and the DET-01 Q01–Q10 principles are separated from reference text and from this package's proposals. No decision was invented. |
| A03B1-G03 | PASS. Independent dimensions, identifier non-equivalence, and fail-closed invariants are in the semantics note. |
| A03B1-G04 | PASS. Coverage families name missing guards and are `PROPOSED_REQUIRES_USER_DECISION` or `BLOCKED_BY_DEPENDENCY`. No edge and no new enum. |
| A03B1-G05 | PASS. Three vocabulary alternatives compare evidence, late fills, corrections, cancel races, and risk. No vocabulary is published as canonical. |
| A03B1-G06 | PASS. OMSQ-01 through OMSQ-12 each have alternatives, a consequence table, a non-binding recommendation, sources, and `USER_DECISION_PENDING`. |
| A03B1-G07 | PASS. `XC-03-MATRIX` and the other unresolved safety-critical XC items stay OPEN. CR-B01 precedence stays in force. |
| A03B1-G08 | PASS. 78 IDs, 78 `NOT_VERIFIED`, one supersession, manifest and source bytes unchanged. |
| A03B1-G09 | PASS. Registry bytes and the five safety flags are unchanged. No runtime, schema, database, or broker implementation. |
| A03B1-G10 | PASS. Traceability, unresolved decisions, and the architecture README link the package. Prior decision records and historical statuses were not edited. |
| A03B1-G11 | PASS. The documentation test and the four Cargo commands passed on the rerun. The first `cargo test` exit 101 and the first `cargo fmt --check` exit 1 are recorded above. One PostgreSQL test stayed ignored and was not run. Remote CI was not run. |
| A03B1-G12 | PASS when this report and the prompt archive are in the one commit named in the Cursor output. Nothing was pushed. |

## Next step

Not started. ARCH-03B.2 and ARCH-03B.3 were not started. Schema publication, implementation, deployment, and live trading are not authorized. `XC-03-MATRIX` stays OPEN. No OMS transition is approved. No `OrderClosure` vocabulary is approved.
