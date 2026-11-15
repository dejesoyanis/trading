# TRD-ARCH-03A.2 report

**Date:** 2026-11-15  
**Mode:** documentation and documentation-only validation. No trading behavior was added.  
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
| BASELINE_HEAD | `a27156785a4711628f324a11c4c2944dbbabe8a0` |
| HEAD subject | `TRD-ARCH-03A: Define Contract Status Model and Verification Semantics` |
| Previous commit | `c771113d2b4b6d2262dd353cf9167679d0bbe32a` `TRD-ARCH-SYNC-02: Ratify Design Baselines and Reconcile ADFR` |
| rustc | 1.99.1 (58cf8f9c3 2026-10-13) |
| cargo | 1.99.1 (5f94df478 2026-08-27) |
| Destructive git | none. Nothing was discarded, reset, or stashed. |

Predecessor reports were read and were not overwritten. SMQ-01 through SMQ-05 were open questions in `contracts/status-model/approval-requests.md`. They were not already recorded under `contracts/decisions/`. The only prior decision file was `ARCH-DEC-01-bis-03-2026-11-15.md`. No naming collision existed for `ARCH-DEC-04-bis-08-2026-11-15.md`.

Registry flags at baseline were `SKELETON_NOT_CANONICAL`, `canonical_schemas_published=false`, `part_b=NO-GO`, `live_trading_authorized=false`, `broker_uniqueness_evidence=EVIDENCE_PENDING`. Registry SHA-256 `7abed2af5363726403175c6524bd7a4d05d36b8735ad48442fc26941f8d98197` (44332 bytes) is unchanged.

## Files inspected

| Requested item | Path used |
| --- | --- |
| TRD-ARCH-SYNC-01 report | `docs/architecture/TRD-ARCH-SYNC-01-report.md` |
| TRD-ARCH-SYNC-02 report | `docs/architecture/TRD-ARCH-SYNC-02-report.md` |
| TRD-ARCH-03A report | `docs/architecture/TRD-ARCH-03A-report.md` |
| ARCH-DEC-01..03 | `contracts/decisions/ARCH-DEC-01-bis-03-2026-11-15.md` |
| Approval requests | `contracts/status-model/approval-requests.md` |
| Status model | `contracts/status-model/contract-status-model-v1.0.md` |
| Legacy mapping | `contracts/status-model/legacy-status-mapping.md` |
| Status draft | `contracts/status-model/status-dimensions-draft-v0.json` |
| Traceability index | `contracts/traceability-index.md` |
| Unresolved decisions | `contracts/domain/unresolved-decisions.md` |
| Source manifest | `contracts/source-manifest.json` |
| ADFR JSON, CSV, README | `contracts/sources/ADFR-Consolidated-v1.0.json`, `contracts/sources/ADFR-Consolidated-v1.0.csv`, `contracts/sources/ADFR-README.md` |
| CR-B10 | `contracts/sources/CR-B10-Domain-Glossary-and-Schema-Registry-v1.0.md` |
| DET-01 | `contracts/sources/DET-01-Order-Workflow-and-Finality-Contract-v1.0.md` |
| DET-02 | `contracts/sources/DET-02-Risk-Contract-v1.0.md` |
| DET-07 | No separate file. Section `## DET-07 — Submission & Execution Control Contract` in `contracts/sources/DET-03-bis-DET-08-Architecture-Contracts-v1.0.md`, including `### Kritische Dispatch-Sequenz` and `**Grenze des Epoch-Fencing:**`. |
| CR-B01 | No separate file. Section `# 1. CR-B01 — Writer-Fencing, Broker-Session und Failover` in `contracts/sources/P0-Resolution-Contracts-CR-B01-bis-CR-B04-v1.0.md`, including `## 0` and `## 1.1`. |
| Registry | `contracts/registry/registry.json` and `contracts/registry/README.md` |
| Existing tests | `crates/trading-domain/tests/adfr_reconciliation.rs` and `crates/trading-domain/tests/registry_skeleton.rs` |

No requested file was missing. No content was invented for an absent file. P1 text remains at `contracts/sources/P1-Resolution-Contracts-CR-B05-bis-CR-B09-v1.0.md`. The glossary index is `contracts/domain/glossary.md`.

## Decisions

The record is [ARCH-DEC-04-bis-08-2026-11-15.md](../../contracts/decisions/ARCH-DEC-04-bis-08-2026-11-15.md). Authority is the explicit user decision dated 2026-11-15 in the TRD-ARCH-03A.2 instruction. No signature artifact was supplied, and none was invented. No evidence URI was invented.

| Decision | Question | Outcome | Alternative | User wording |
| --- | --- | --- | --- | --- |
| ARCH-DEC-04 | SMQ-01 | DECIDED | 1 | Keine Priorität; historische Statusbegriffe unverändert lassen |
| ARCH-DEC-05 | SMQ-02 | DECIDED | 1 | Kein neues Enum und keine zusätzlichen Lifecycle-Zuordnungen |
| ARCH-DEC-06 | SMQ-03 | DECIDED | 1 | Nur konkret getestete Teilkomponenten als lokal getestet behandeln |
| ARCH-DEC-07 | SMQ-04 | DECIDED | 1 | Datierter Nutzerentscheid plus geeigneter Evidenznachweis |
| ARCH-DEC-08 | SMQ-05 | DECIDED | 1 | Bestehende Manifestangaben beibehalten |

These are interpretation and evidence rules. They do not approve an open technical contract.

**Exclusions.** No winning status among `OPEN P0`, `PROPOSED_FOR_ACCEPTANCE`, and `DESIGN_RESOLVED`. No default precedence. Those words are not treated as equivalent. Historical `P0`, `P1`, and `P2` are not a decision-lifecycle state. The closed lifecycle enum is not ratified. `SUPERSEDED` is not withdrawn or rejected. `IMPLEMENTED_AND_LOCALLY_TESTED` stays scoped to the named type-shell part. ARCH-DEC-07 is a procedural gate and does not promote any current `verification` value. No evidence record was created. Manifest placement and provenance fields were not relabeled. `IMPLEMENTED`, `VERIFIED`, `APPROVED`, `ORIGINAL`, `REJECTED`, and `WITHDRAWN` were not assigned to historical records because of these decisions. `DESIGN_BASELINE_RATIFIED_WITH_EXCEPTIONS` for DET-01 and DET-02 remains the ARCH-DEC-01 label. Their Q01–Q10 verification cells remain OPEN. ARCH-DEC-01, ARCH-DEC-02, and ARCH-DEC-03 were not reopened.

The original SMQ questions stay in [approval-requests.md](../../contracts/status-model/approval-requests.md) as historical text, including each Alternative 1. A dated resolution index was appended. The questions were not rewritten as if they had already been approved when they were asked.

## Source and ADFR integrity

Before any edit, all 12 files in `contracts/source-manifest.json` matched that manifest's current `sha256` and `bytes`. Mismatches: 0. `package_sha256` and `package_bytes` remain the TRD-ARCH-SYNC-01 intake values. Current and package hashes differ only for the three files whose `placement` is `AMENDED_BY_ARCH_DEC_02`. DET-01 and DET-02 stay `reconstructed_draft` true, `SOURCE_REVIEW_PENDING`, `reviewed` false, and `approved` false. The other ten package files stay `reconstructed_draft` false, `reviewed` false, and `approved` false. None was labeled `original` or `unknown`.

`contracts/source-manifest.json` was not edited. The 12 manifest source files were not edited. `contracts/sources/ADFR-README.md` is not one of those 12 files.

| Item | Value, unchanged from baseline |
| --- | --- |
| JSON count | 78 |
| CSV count | 78 |
| Unique IDs, same order | 78 |
| `verification` | 78 `NOT_VERIFIED` |
| Superseded | 1: `ADFR-CROSS-005`, `superseded_by` `ADFR-CR-B10-001`, `status` still `PROPOSED_FOR_ACCEPTANCE` |
| Successor | `ADFR-CR-B10-001`, `supersedes` `ADFR-CROSS-005`, lifecycle `NOT_SUPERSEDED`, `verification` `NOT_VERIFIED` |
| Not superseded | 77 |
| Cycles | none. One directed edge. |
| JSON SHA-256 | `272a6dabdb411701a25dd708577dbb498c7a7cd2754d83c612e85454a178a1d5` (29880 bytes) |
| CSV SHA-256 | `b5577302650e3d497465b93063d149313ec1197a38ddad3a9e7a6f223757ec32` (13138 bytes) |
| JSON root `status` | `ARCHITECTURE_DRAFT_NOT_IMPLEMENTED` |

## Mapping and lifecycle

[contract-status-model-v1.0.md](../../contracts/status-model/contract-status-model-v1.0.md) now separates ratified interpretation constraints (ARCH-DEC-04 through ARCH-DEC-08) from token sets that stay `PROPOSED_NOT_RATIFIED`. The file is not a canonical schema.

[legacy-status-mapping.md](../../contracts/status-model/legacy-status-mapping.md) still has 41 phrase-and-context rows and the original literals. Cross-source status precedence for `OPEN P0`, `OPEN P1`, `OPEN P2`, `PROPOSED_FOR_ACCEPTANCE`, and `DESIGN_RESOLVED` stays `REQUIRES_USER_DECISION`. ARCH-DEC-04 is why that mapping stays unresolved: no precedence was chosen, and the words are not equivalent. `IMPLEMENTED_AND_LOCALLY_TESTED` is now `DEFINITIVE` for the scoped type-shell reading in ARCH-DEC-06. That change does not write the phrase onto an ADFR row.

[status-dimensions-draft-v0.json](../../contracts/status-model/status-dimensions-draft-v0.json) was updated because the draft had to cite the dated decisions. It still has `document_role` `NON_RUNTIME_NON_WIRE_DRAFT`, `interpretation_layer_ratification` `PROPOSED_NOT_RATIFIED`, `live_trading_authorized` false, `canonical_schemas_published` false, `part_b` `NO-GO`, and `broker_uniqueness_evidence` `EVIDENCE_PENDING`. `interpretation_constraints_do_not_ratify_token_sets` is true. The proposed lifecycle tokens, including `rejected` and `withdrawn`, stay unassigned. No canonical enum was added. `registry.yaml`, `events/v1`, `commands/v1`, `api/v1`, and `sql/v1` were not created.

Lifecycle stored values remain `SUPERSEDED` and `NOT_SUPERSEDED` only. Supersession semantics are unchanged.

## Unresolved items

Findings that stay open: XC-01, XC-02-PRED, XC-03-MATRIX, XC-05-LOCK, XC-06, XC-07-ROUND, XC-08-IBKR, XC-09-RESET, XC-10-ACTION. XC-11 status-word disagreement stays OPEN. XC-04 stays OPEN apart from design precedence, including broker-session exclusivity, lease and heartbeat, dispatch scope, in-flight send evidence, recovery tests, and IBKR behavior.

Sub-findings already closed by a prior user decision, and not reopened: XC-11 population and supersession (`RESOLVED_BY_ARCH_DEC_02`); XC-04 design-reading precedence (`RESOLVED_BY_ARCH_DEC_03`).

UD-01 through UD-09 status lines were not rewritten. DET-01 and DET-02 exceptions, including Q01–Q10 verification cells, stay OPEN. `NO_CONFLICT_FOUND` rows from TRD-ARCH-SYNC-01 were not closed by this task.

## Safety

`contracts/registry/registry.json` is byte-identical to the baseline. No file under `crates/*/src` or `apps/*/src` was edited. No route, adapter, migration, credential, runtime flag, order or risk state machine, broker call, lease, audit write, or financial rounding was added. `Cargo.toml` and `Cargo.lock` were not changed. No production dependency was added. B10-T01 through B10-T12 are not passed. No runtime behavior changed.

## Changed files

| Path | SHA-256 | Bytes |
| --- | --- | --- |
| `contracts/decisions/ARCH-DEC-04-bis-08-2026-11-15.md` | `af2cd4b5c35912fe97d3a8ea36c459259aabf0a7b7d54657fdb7b7bce6149ddc` | 13679 |
| `contracts/domain/glossary.md` | `9953869e3dda89852e38a158a0af61aeb0407f1314199f273bd14af9c6ccc3ec` | 16633 |
| `contracts/domain/unresolved-decisions.md` | `09a4b8768a32e8e2c34df6aec396d8cd3710ca96228e0cf78ab102da39430997` | 18291 |
| `contracts/registry/README.md` | `a1de93503b1fe3ae5408a54c9e5826ce207e52e9d57f41aea5c7165991f1768a` | 2879 |
| `contracts/sources/ADFR-README.md` | `ae7a81b1510c12aa1c0ec4731a03b7b5c16521eaa47886acb8bb5cd3b0f77251` | 4367 |
| `contracts/status-model/approval-requests.md` | `384e7db094a5266c3da720ace2d38a46b76a8bcb3ec0cae9e3ada68df7b46adc` | 10322 |
| `contracts/status-model/contract-status-model-v1.0.md` | `090c663b9bfcd6b1340204d4498c6be0a85c9d657e260a4f5c423ff8585d02cd` | 28168 |
| `contracts/status-model/legacy-status-mapping.md` | `efed8f9dfd4dbf99ab48528f2741b566d8b5d95e32c3879a5034847a945f75d1` | 25709 |
| `contracts/status-model/status-dimensions-draft-v0.json` | `2c86da4bf3d1263764501a485703ea770a3fca9d9281ce85713c17432a19754f` | 33171 |
| `contracts/traceability-index.md` | `cdafaf9ef89cec7d97c7d54275ac7fc292dc2a9f9a66479d0f225e9cbc880950` | 20131 |
| `crates/trading-domain/tests/adfr_reconciliation.rs` | `4a1fd9d616d79828aa54ffca278464d2d4931b54e44419fabf5433cafed6d276` | 49449 |
| `docs/architecture/README.md` | `3429afdeed0eaef462fe554c82a3ded22e0b8ae14000655946012767e01cbefd` | 12062 |
| `docs/cursor-prompts/trading/TRD-ARCH-03A2-Ratify-Status-Interpretation-Decisions.md` | `656a70b1d2710a89a200d5a031cc22696c9fe20843d75ffdd66dc775bb57c511` | 13089 |
| `tests/README.md` | `657216333e8d59e39e1d1d96ea36a09c43eb73c9f3662088af4124f0152bb712` | 1044 |

This report's own hash is not listed. No Cargo metadata change required a dependency justification.

## Local checks

Remote CI was not run. The ignored PostgreSQL test was not executed against a database. Ignored is not passed.

The four commands were run once from the repository root after the documentation edits. None failed, so there was no rerun.

| Command | Exit code |
| --- | --- |
| `cargo fmt --all -- --check` | 0 |
| `cargo check --workspace --locked` | 0 |
| `cargo test --workspace --locked` | 0 |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | 0 |

`cargo test` result: 130 passed, 0 failed, 1 ignored. The ignored test is `migration_is_explicit_and_readiness_is_read_only` in `crates/trading-persistence/tests/connectivity.rs`, because `TRADING_ALLOW_DATABASE_INTEGRATION_TESTS=1` and `TRADING_TEST_DATABASE_URL` were not set. It was not run against a database. `adfr_reconciliation` passed 10 tests, including `interpretation_decisions_04_through_08_stay_conservative`. `registry_skeleton_stays_unpublished` passed.

## Gates

| Gate | Result |
| --- | --- |
| A03A2-G01 | PASS. Clean baseline. 12/12 current manifest hashes match. Package hashes kept. Registry and ADFR baseline recorded. |
| A03A2-G02 | PASS. SMQ-01..05 are DECIDED as Alternative 1 on 2026-11-15, with the user wording quoted. No extra decision was invented. |
| A03A2-G03 | PASS. No historical status precedence. XC-11 status-word disagreement stays OPEN. The words are not equivalent. |
| A03A2-G04 | PASS. The closed lifecycle enum is not ratified. Supersession stays one edge, `ADFR-CROSS-005` to `ADFR-CR-B10-001`. |
| A03A2-G05 | PASS. Local testing claims stay scoped. All 78 ADFR `verification` values stay `NOT_VERIFIED`. Broker uniqueness stays `EVIDENCE_PENDING`. |
| A03A2-G06 | PASS. ARCH-DEC-07 requires a dated user approval and an evidence record with URI or stable artifact reference, method, environment, timestamp, exact claim and scope, and limitations. No present promotion and no evidence URI. |
| A03A2-G07 | PASS. Manifest placement and provenance fields are unchanged. No `original` or `unknown` classification was added. |
| A03A2-G08 | PASS. 78 IDs, same order, JSON/CSV parity, one supersession, 77 not superseded, and the 12 source hashes are unchanged. |
| A03A2-G09 | PASS. The named XC items and DET-01/02 exceptions stay OPEN. No design-to-runtime escalation. |
| A03A2-G10 | PASS. Registry bytes and the five safety flags are unchanged. No runtime, migration, schema, or broker activity. |
| A03A2-G11 | PASS. Documentation tests and the four Cargo checks passed. One PostgreSQL test stayed ignored and was not run. Remote CI was not run. |
| A03A2-G12 | PASS when this report and the prompt archive are in the one commit named in the Cursor output. Nothing was pushed. |

## Deviations

`status-dimensions-draft-v0.json` was edited so the draft cites ARCH-DEC-04 through ARCH-DEC-08. The edit was required. The draft stays non-runtime, non-wire, and `PROPOSED_NOT_RATIFIED` for its token sets. The new field `interpretation_constraints_do_not_ratify_token_sets` is true.

`IMPLEMENTED_AND_LOCALLY_TESTED` moved from `REQUIRES_USER_DECISION` to `DEFINITIVE` for the scoped reading ARCH-DEC-06 fixed. The historical literal is unchanged. The other 40 phrase-and-context rows keep their previous mapping status. Cross-source status precedence stays `REQUIRES_USER_DECISION`.

`contracts/sources/ADFR-README.md` was edited to point at ARCH-DEC-04 and ARCH-DEC-07. That README is not a manifest source file. The ADFR JSON and CSV bytes were not edited.

No evidence file was created. `contracts/source-manifest.json`, `contracts/registry/registry.json`, and the 12 package source files were not edited.

## Next step

Not started. ARCH-03B was not started. Schema publication, implementation, deployment, and live trading are not authorized. Until a later decision says otherwise, `OPEN P0`, `PROPOSED_FOR_ACCEPTANCE`, and `DESIGN_RESOLVED` stay unreconciled, every ADFR `verification` value stays `NOT_VERIFIED`, and live trading stays unauthorized.
