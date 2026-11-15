# TRD-ARCH-03A report

**Date:** 2026-11-15  
**Mode:** documentation and documentation-only tests. No trading behavior was added.  
**Live trading:** NOT AUTHORIZED  
**Canonical schemas:** not published  
**Remote CI:** remote CI was not run

`VERDICT: PASS`

`PUSHED: NO`. The commit SHA is the commit that adds this report. It is named in the Cursor output because this file cannot contain the hash of the commit that introduces it. The gates below are the basis for the verdict.

## Baseline

| Item | Value |
| --- | --- |
| Branch | `main` |
| Worktree before edits | clean. `git status --short` printed nothing. |
| BASELINE_HEAD | `c771113d2b4b6d2262dd353cf9167679d0bbe32a` |
| HEAD subject | `TRD-ARCH-SYNC-02: Ratify Design Baselines and Reconcile ADFR` |
| rustc | 1.99.1 (58cf8f9c3 2026-10-13) |
| cargo | 1.99.1 (5f94df478 2026-08-27) |
| Destructive git | none. Nothing was discarded, reset, or stashed. |

Predecessor reports were read and were not overwritten. Registry flags at baseline were `SKELETON_NOT_CANONICAL`, `canonical_schemas_published=false`, `part_b=NO-GO`, `live_trading_authorized=false`, `broker_uniqueness_evidence=EVIDENCE_PENDING`. Registry SHA-256 `7abed2af5363726403175c6524bd7a4d05d36b8735ad48442fc26941f8d98197` (44332 bytes) is unchanged.

## Files inspected

| Requested item | Path used |
| --- | --- |
| TRD-ARCH-SYNC-01 report | `docs/architecture/TRD-ARCH-SYNC-01-report.md` |
| TRD-ARCH-SYNC-02 report | `docs/architecture/TRD-ARCH-SYNC-02-report.md` |
| ARCH-DEC-01..03 | `contracts/decisions/ARCH-DEC-01-bis-03-2026-11-15.md` |
| Source manifest | `contracts/source-manifest.json` |
| Traceability index | `contracts/traceability-index.md` |
| Unresolved decisions | `contracts/domain/unresolved-decisions.md` |
| Glossary | `contracts/domain/glossary.md` |
| Registry | `contracts/registry/registry.json` and `contracts/registry/README.md` |
| ADFR JSON, CSV, README | `contracts/sources/ADFR-Consolidated-v1.0.json`, `contracts/sources/ADFR-Consolidated-v1.0.csv`, `contracts/sources/ADFR-README.md` |
| CR-B10 | `contracts/sources/CR-B10-Domain-Glossary-and-Schema-Registry-v1.0.md` |
| DET-01 | `contracts/sources/DET-01-Order-Workflow-and-Finality-Contract-v1.0.md` |
| DET-02 | `contracts/sources/DET-02-Risk-Contract-v1.0.md` |
| DET-07 | No separate file. Section `## DET-07 — Submission & Execution Control Contract` in `contracts/sources/DET-03-bis-DET-08-Architecture-Contracts-v1.0.md`. |
| CR-B01 | No separate file. Section `# 1. CR-B01 — Writer-Fencing, Broker-Session und Failover` in `contracts/sources/P0-Resolution-Contracts-CR-B01-bis-CR-B04-v1.0.md`. |
| Architecture consolidation | `contracts/sources/Architecture-Consolidation-and-Acceptance-v1.0.md` |
| Existing tests | `crates/trading-domain/tests/adfr_reconciliation.rs` and `crates/trading-domain/tests/registry_skeleton.rs` |

No separate contract-validation script was found outside those two tests. None was invented. P1 text was read at `contracts/sources/P1-Resolution-Contracts-CR-B05-bis-CR-B09-v1.0.md`. The roadmap was read at `contracts/sources/Implementation-Roadmap-and-Cursor-Prompt-Plan-v1.0.md`.

## Source hashes

Before any edit, and again after the documentation edits, all 12 files in `contracts/source-manifest.json` matched that manifest's current `sha256` and `bytes`. Mismatches: 0. `package_sha256` remains the TRD-ARCH-SYNC-01 intake hash. It differs from the current hash only for the three files whose `placement` is `AMENDED_BY_ARCH_DEC_02`.

| File | Current equals package | Current hash | Bytes |
| --- | --- | --- | --- |
| Nine unchanged sources, including DET-01 and DET-02 | yes | manifest `sha256` | manifest `bytes` |
| `ADFR-Consolidated-v1.0.json` | no | `272a6dabdb411701a25dd708577dbb498c7a7cd2754d83c612e85454a178a1d5` | 29880 |
| `ADFR-Consolidated-v1.0.csv` | no | `b5577302650e3d497465b93063d149313ec1197a38ddad3a9e7a6f223757ec32` | 13138 |
| `Architecture-Consolidation-and-Acceptance-v1.0.md` | no | `9f1fdd36e0a45343762a2d608b7b71dccaa45f82e41f7adbddbf1560e18bd9a1` | 5770 |

Package hashes for those three amended files stay `4c587b94f668e34e9c088650bbcc4d86ee51c90cbd3602281ce2b0e93931452c` (21514 bytes), `242e4a1205ee1a050218fd52b6597af02e9badd13ac842e01df70116ca4dda13` (11831 bytes), and `334bcfcba5015534184d65c556d0cd2d308238bd76bf61429c2bfe6a877bfd67` (3676 bytes). DET-01 and DET-02 bytes remain the package bytes. `reconstructed_draft` remains true. `SOURCE_REVIEW_PENDING` remains. `approved` remains false.

## Status model

The vocabulary is [contract-status-model-v1.0.md](../../contracts/status-model/contract-status-model-v1.0.md). It is an interpretation layer. It does not replace ADFR `status`, `verification`, `lifecycle`, `supersedes`, or `superseded_by`.

**Already in force, cited rather than extended.**

| Topic | In force |
| --- | --- |
| ARCH-DEC-01 | DET-01 and DET-02 are `DESIGN_BASELINE_RATIFIED_WITH_EXCEPTIONS`. Provenance stays `RECONSTRUCTED_DRAFT`. Q01–Q10 verification cells stay `OPEN`. The twenty `CONFIRMED_DECISION` rows are not relabeled and are not marked verified. |
| ARCH-DEC-02 | 78 historical IDs. `ADFR-CROSS-005` lifecycle `SUPERSEDED`, successor `ADFR-CR-B10-001`. 77 `NOT_SUPERSEDED`. Every `verification` value stays `NOT_VERIFIED`. Status-word precedence was not decided. |
| ARCH-DEC-03 | CR-B01 has design precedence over conflicting or less restrictive DET-07 text. Automatic takeover is not authorized. Fencing is not implemented. IBKR behavior is not demonstrated. |
| Safety | `live_trading_authorized` false, `canonical_schemas_published` false, `part_b` `NO-GO`, broker uniqueness `EVIDENCE_PENDING`, dispatch readiness `NOT_AUTHORIZED`. |

**Proposed and not ratified.** Closed sets for `original` / `reconstructed` / `external_evidence` / `unknown`, for `proposed` / `open` / `ratified_with_exceptions` / `superseded` / `rejected` / `withdrawn`, for `not_implemented` / `partial` / `implemented`, and for `not_verified` / `evidence_pending` / `tested_locally` / `externally_verified` / `failed_or_blocked` are labeled `PROPOSED_NOT_RATIFIED`. None of those proposed tokens is assigned to a row. `rejected` and `withdrawn` have no register row. Questions are SMQ-01 through SMQ-05 in [approval-requests.md](../../contracts/status-model/approval-requests.md). This task does not approve them.

## Mapping

[legacy-status-mapping.md](../../contracts/status-model/legacy-status-mapping.md) records 41 phrase-and-context rows. The required literals are present: `OPEN P0`, `OPEN P1`, `PROPOSED_FOR_ACCEPTANCE`, `CONFIRMED_DECISION`, `DESIGN_RESOLVED`, `DESIGN_BASELINE_RATIFIED_WITH_EXCEPTIONS`, `RECONSTRUCTED_DRAFT`, `NOT_VERIFIED`, `EVIDENCE_PENDING`, `SUPERSEDED`, `NOT_SUPERSEDED`, `NOT_IMPLEMENTED`, `SKELETON_NOT_CANONICAL`, and `NO-GO`.

`OPEN P0`, `OPEN P1`, `OPEN P2`, `PROPOSED_FOR_ACCEPTANCE`, and `DESIGN_RESOLVED` are `REQUIRES_USER_DECISION` where a crosswalk would pick a winner. `NOT_VERIFIED` is `DEFINITIVE` and does not mean verified. `NO_CONFLICT_FOUND` does not permit an unresolved predicate or transition. `DESIGN_RESOLVED` does not mean verified, implemented, or live-authorized. The DET-09/10 cell `OPEN P2` on `ADFR-CROSS-005` was not rewritten when the register lifecycle became `SUPERSEDED`.

## ADFR invariants

| Item | Value |
| --- | --- |
| JSON count | 78 |
| CSV count | 78 |
| Unique IDs, same order | 78 |
| `status` counts | 20 `CONFIRMED_DECISION`, 57 `PROPOSED_FOR_ACCEPTANCE`, 1 `DESIGN_RESOLVED` |
| `verification` | 78 `NOT_VERIFIED` |
| Superseded | 1: `ADFR-CROSS-005`, `superseded_by` `ADFR-CR-B10-001`, `status` still `PROPOSED_FOR_ACCEPTANCE` |
| Successor | `ADFR-CR-B10-001`, `supersedes` `ADFR-CROSS-005`, lifecycle `NOT_SUPERSEDED`, `verification` `NOT_VERIFIED` |
| Not superseded | 77 |
| Cycles | none. One directed edge. |
| JSON and CSV bytes | not edited |

## Unresolved items

Approval requests, not decisions: SMQ-01, SMQ-02, SMQ-03, SMQ-04, SMQ-05.

Findings that stay open: XC-01, XC-02-PRED, XC-03-MATRIX, XC-05-LOCK, XC-06, XC-07-ROUND, XC-08-IBKR, XC-09-RESET, XC-10-ACTION. XC-11 status-word disagreement stays OPEN. XC-04 stays OPEN apart from design precedence.

Sub-findings already closed by a prior user decision, and not reopened: XC-11 population and supersession (`RESOLVED_BY_ARCH_DEC_02`); XC-04 design-reading precedence (`RESOLVED_BY_ARCH_DEC_03`).

`NO_CONFLICT_FOUND` rows left as TRD-ARCH-SYNC-01 recorded them: XC-02-ALIGN, XC-03-BOOK, XC-05-OUTBOX, XC-07-FX, XC-08-RULE, XC-09-SOLO, XC-10-RACE. UD-01 through UD-09 statuses were not rewritten.

## Machine-readable draft

[status-dimensions-draft-v0.json](../../contracts/status-model/status-dimensions-draft-v0.json) has `document_role` `NON_RUNTIME_NON_WIRE_DRAFT`, version `0.1-draft`, and `interpretation_layer_ratification` `PROPOSED_NOT_RATIFIED`. It sets `live_trading_authorized` false, `canonical_schemas_published` false, `part_b` `NO-GO`, and `closes_xc_findings` false. It does not copy ADFR decision bodies. It is not `registry.json`. `registry.yaml`, `events/v1`, `commands/v1`, `api/v1`, and `sql/v1` were not created.

## Safety

`contracts/registry/registry.json` is byte-identical to the baseline. No file under `crates/*/src` or `apps/*/src` was edited. No route, adapter, migration, credential, or runtime flag was added. `Cargo.toml` and `Cargo.lock` were not changed. No production dependency was added. B10-T01 through B10-T12 are not passed. No runtime behavior changed.

## Changed files

| Path | SHA-256 | Bytes |
| --- | --- | --- |
| `contracts/status-model/contract-status-model-v1.0.md` | `510fdccf045bc57d639cff4f113ea87f940c13d979a78065af0c412bf2e6799d` | 24120 |
| `contracts/status-model/legacy-status-mapping.md` | `5a3fba8c85eb85ad6fce611f793cb6c5f16eeac11fc49948c8b104bd767a02df` | 23275 |
| `contracts/status-model/status-dimensions-draft-v0.json` | `75700ce98bd79ab0b810b8bae58de14fa53795c9644e88226280ccf2c651c606` | 29131 |
| `contracts/status-model/approval-requests.md` | `0d88bde8f37a8a703f782b884a9672619d1e45bc7ef8106a989049114f796310` | 8173 |
| `contracts/domain/glossary.md` | `b2d4f25ce6fe76553c878c65b2bca220f7d7626a48ddda981f4eff2191350a43` | 16434 |
| `contracts/domain/unresolved-decisions.md` | `7f5ac54cf4461bd06544aeaf26b3159d98f6496cd3b7b46ca0f7ac3c89c10bb6` | 17557 |
| `contracts/registry/README.md` | `6468bcf3ce22227d5aa2004d9adac4626b23c1d0c19bf262a6684b6a8f10223e` | 2762 |
| `contracts/sources/ADFR-README.md` | `2c1f580b0afab94b31172aeae0a1f1126d243f090aa84c0e94d4d82cd58ca34a` | 4129 |
| `contracts/traceability-index.md` | `86ac5939d6cb53efc11ba0f4e915091d76affbc1146fe2c1f40e95a1b891c356` | 19189 |
| `crates/trading-domain/tests/adfr_reconciliation.rs` | `1cc72202dc6ac237f7891a08583371e68cc1be6c7624da96d8ed213ac353137e` | 39134 |
| `crates/trading-domain/tests/registry_skeleton.rs` | `2452e7b756cb77741e76feaf143124806984e4c06548b174cf613458fb5fde22` | 15799 |
| `docs/architecture/README.md` | `762ebc2677db05a86f0dcdb2dcbb36b4e27b6df42ed3d3a50c9ae140f5a4a7c8` | 11555 |
| `tests/README.md` | `3ca345baa6a3a0f7dbceab759ec4e746ee351011da75e1b1845662524f66da68` | 892 |
| `docs/cursor-prompts/trading/TRD-ARCH-03A-Canonical-Contract-Status-Model.md` | `836dc20b626065ae1fef21c8cd99502ce75cac343d997fd65c0628933c4f5df5` | 11249 |

This report's own hash is not listed. No Cargo metadata change required a dependency justification.

## Local checks

Remote CI was not run. The ignored PostgreSQL test was not executed against a database.

The first `cargo test --workspace --locked` exited 101. `registry_skeleton_stays_unpublished` rejected `contracts/status-model/status-dimensions-draft-v0.json` because the allowlist named only the registry, the manifest, and the ADFR extract. The correction adds that draft path and asserts `NON_RUNTIME_NON_WIRE_DRAFT`, `live_trading_authorized` false, and `canonical_schemas_published` not true. Forbidden schema directories stay rejected. The four commands were then run again.

| Command | Exit code on the rerun |
| --- | --- |
| `cargo fmt --all -- --check` | 0 |
| `cargo check --workspace --locked` | 0 |
| `cargo test --workspace --locked` | 0 |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | 0 |

`cargo test` result on that rerun: 129 passed, 0 failed, 1 ignored. The ignored test is `migration_is_explicit_and_readiness_is_read_only` in `crates/trading-persistence/tests/connectivity.rs`, because `TRADING_ALLOW_DATABASE_INTEGRATION_TESTS=1` and `TRADING_TEST_DATABASE_URL` were not set. It was not run against a database. Ignored is not passed. `adfr_reconciliation` passed 9 tests. `registry_skeleton_stays_unpublished` passed.

## Gates

| Gate | Result |
| --- | --- |
| A03A-G01 | PASS. Clean baseline. 12/12 current manifest hashes match. Package hashes kept. DET-01/02 provenance unchanged. |
| A03A-G02 | PASS. Provenance, lifecycle, implementation, verification, severity, authorization, and supersession are separate. |
| A03A-G03 | PASS. Legacy phrases are mapped with source and `REQUIRES_USER_DECISION` where the words conflict. |
| A03A-G04 | PASS. Design baseline with exceptions. Provenance stays reconstructed. Q01–Q10 stay open. |
| A03A-G05 | PASS. CR-B01 design precedence stands. Takeover is not authorized. |
| A03A-G06 | PASS. 78 IDs, 77 not superseded, one supersession. JSON and CSV were not edited. |
| A03A-G07 | PASS. The draft is labeled non-runtime and non-wire. |
| A03A-G08 | PASS. Documentation tests cover the invariants and passed on the rerun. |
| A03A-G09 | PASS. Open XC items remain open. SMQ-01..SMQ-05 are requests, not decisions. |
| A03A-G10 | PASS. Registry bytes and the five safety flags are unchanged. |
| A03A-G11 | PASS on the rerun. One PostgreSQL test stayed ignored and was not run. Remote CI was not run. |
| A03A-G12 | PASS when this report and the prompt archive are in the one commit named in the Cursor output. Nothing was pushed. |

## Deviations

The registry JSON allowlist in `registry_skeleton.rs` gained one documentation path. That was required for the existing unpublished-schema test to accept the draft file. The test still rejects live-trading and canonical-schema flags in the draft. No ADFR, manifest, registry, or historical source byte was edited to make status words agree.

## Next step

Not started. ARCH-03B was not started. SMQ-01 through SMQ-05 remain for the user. Until a later decision says otherwise, `OPEN P0`, `PROPOSED_FOR_ACCEPTANCE`, and `DESIGN_RESOLVED` stay unreconciled, and live trading stays unauthorized.
