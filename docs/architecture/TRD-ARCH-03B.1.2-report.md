# TRD-ARCH-03B.1.2 report

**Date:** 2026-11-15  
**Mode:** documentation, traceability, and documentation-only validation. No trading behavior was added.  
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
| BASELINE_HEAD | `8a7502835176cf3ddc4c14f9f87f0c2092921c9b` |
| HEAD subject | `TRD-ARCH-03B.1: Draft OMS Finality and OrderClosure Decisions` |
| Predecessor report | `docs/architecture/TRD-ARCH-03B.1-report.md` present, verdict PASS in that file |
| rustc | 1.99.1 (58cf8f9c3 2026-10-13) |
| cargo | 1.99.1 (5f94df478 2026-08-27) |
| git | 2.56.0.windows.3 |
| Destructive git | none. Nothing was discarded, reset, or stashed. |

ARCH-DEC-09 through ARCH-DEC-20 were unused before this record. No naming collision existed for `ARCH-DEC-09-bis-20-2026-11-15.md`.

Registry flags at baseline were `SKELETON_NOT_CANONICAL`, `canonical_schemas_published=false`, `part_b=NO-GO`, `live_trading_authorized=false`, `broker_uniqueness_evidence=EVIDENCE_PENDING`. Registry SHA-256 `7abed2af5363726403175c6524bd7a4d05d36b8735ad48442fc26941f8d98197` (44332 bytes) is unchanged.

## Files inspected

| Requested item | Path used |
| --- | --- |
| TRD-ARCH-SYNC-01 report | `docs/architecture/TRD-ARCH-SYNC-01-report.md` |
| TRD-ARCH-SYNC-02 report | `docs/architecture/TRD-ARCH-SYNC-02-report.md` |
| TRD-ARCH-03A report | `docs/architecture/TRD-ARCH-03A-report.md` |
| TRD-ARCH-03A.2 report | `docs/architecture/TRD-ARCH-03A.2-report.md` |
| TRD-ARCH-03B.1 report | `docs/architecture/TRD-ARCH-03B.1-report.md` |
| ARCH-DEC-01..03 | `contracts/decisions/ARCH-DEC-01-bis-03-2026-11-15.md` |
| ARCH-DEC-04..08 | `contracts/decisions/ARCH-DEC-04-bis-08-2026-11-15.md` |
| Approval requests | `contracts/design-proposals/TRD-ARCH-03B1-approval-requests.md` |
| Semantics, coverage, vocabulary | the three TRD-ARCH-03B1 proposal notes under `contracts/design-proposals/` |
| Conflict matrix and proposal index | `TRD-ARCH-03B1-dependency-and-conflict-matrix.md`, `TRD-ARCH-03B1-proposal-index.json` |
| Status model | `contracts/status-model/contract-status-model-v1.0.md`, `approval-requests.md`, `legacy-status-mapping.md`, `status-dimensions-draft-v0.json` |
| Traceability index | `contracts/traceability-index.md` |
| Unresolved decisions | `contracts/domain/unresolved-decisions.md` |
| Glossary | `contracts/domain/glossary.md` |
| Source manifest | `contracts/source-manifest.json` |
| ADFR | `contracts/sources/ADFR-Consolidated-v1.0.json`, `contracts/sources/ADFR-Consolidated-v1.0.csv`, `contracts/sources/ADFR-README.md` |
| Registry | `contracts/registry/registry.json` |
| Existing tests | `crates/trading-domain/tests/adfr_reconciliation.rs`, `crates/trading-domain/tests/registry_skeleton.rs` |

No requested file was missing. DET-07 and CR-B01 remain sections inside the combined source files already indexed by the predecessor reports. No content was invented for an absent file. The status-model files were read and were not edited. Their SMQ decisions stay ARCH-DEC-04 through ARCH-DEC-08.

## Decisions

The record is [ARCH-DEC-09-bis-20-2026-11-15.md](../../contracts/decisions/ARCH-DEC-09-bis-20-2026-11-15.md). Authority is the explicit user ratification dated 2026-11-15 in the TRD-ARCH-03B.1.2 instruction. No signature artifact was supplied, and none was invented. No evidence URI was invented. Each row is `RATIFIED_DESIGN_POLICY_ONLY`, `NOT_IMPLEMENTED`, and `NOT_VERIFIED`. Those words describe this record. They are not a canonical status enum and they are not written onto an ADFR row.

| Decision | Question | Option | User wording |
| --- | --- | --- | --- |
| ARCH-DEC-09 | OMSQ-01 | A | Ein ursprünglicher Abschlussdatensatz, spätere Fakten separat |
| ARCH-DEC-10 | OMSQ-02 | C | Grundfamilie plus unabhängige Finalitätsfelder |
| ARCH-DEC-11 | OMSQ-03 | A | Keine Schliessung ohne neue autorisierte Aktion und positive Evidenz |
| ARCH-DEC-12 | OMSQ-04 | A | Alle Fakten behalten; Closure und Risk nur bei ausreichender Evidenz |
| ARCH-DEC-13 | OMSQ-05 | B | Alle wirtschaftlichen Effekte zunächst manuell prüfen |
| ARCH-DEC-14 | OMSQ-06 | A | Kein Finalitätsprädikat akzeptieren; Risk Release verboten |
| ARCH-DEC-15 | OMSQ-07 | A | Vollständige Szenarien, Negativfälle und eigene Version verlangen |
| ARCH-DEC-16 | OMSQ-08 | A | Quarantäne und strukturierte autorisierte Entscheidung |
| ARCH-DEC-17 | OMSQ-09 | A | Keine terminale Brokerbeobachtung akzeptieren |
| ARCH-DEC-18 | OMSQ-10 | A | Sperrreihenfolge und Isolation bleiben unbestätigter Vorschlag |
| ARCH-DEC-19 | OMSQ-11 | A | Separaten transaktionalen Auditdatensatz verlangen |
| ARCH-DEC-20 | OMSQ-12 | A | Modify und implizites Cancel-Replace blockieren |

The original OMSQ questions, alternatives, and recommendations stay in the approval-request file as historical text. A dated resolution index was appended. The questions were not rewritten as if they had already been approved when they were asked. The proposal index JSON stays the creation-time `NOT_RATIFIED` snapshot.

## Decision-to-XC matrix

A policy choice does not close the finding in the right-hand column.

| Decision | What the policy binds | What stays open |
| --- | --- | --- |
| ARCH-DEC-09 | One immutable historical closure record. Later facts sit outside it. No closure-shaped correction and no reopening. | No writer and no schema. `XC-03-MATRIX` stays OPEN. |
| ARCH-DEC-10 | Modeling strategy only: reason family separate from broker, economic, and evidence terminality. | No token, enum, field, terminality semantics, or edge. `XC-03-MATRIX` stays OPEN. |
| ARCH-DEC-11 | Unknown send does not close the workflow. No automatic retry or takeover. | XC-04 remainder and XC-08-IBKR stay OPEN. ARCH-DEC-03 precedence stays in force. |
| ARCH-DEC-12 | Cancel request, cancel observation, and every fill stay separate facts. | No risk movement. The predicate is not satisfied. XC-01, XC-02-PRED, XC-08-IBKR, and XC-10-ACTION stay OPEN. |
| ARCH-DEC-13 | Every proposed economic effect is quarantined, including a single match. Original closure stays. | No posting algorithm. XC-01, XC-02-PRED, and XC-07-ROUND stay OPEN. |
| ARCH-DEC-14 | No executable economic-finality predicate and no reservation release. Checklist B is not adopted. | XC-02-PRED stays OPEN. |
| ARCH-DEC-15 | A future matrix needs full scenario families, forbidden negative edges, missing guards, and its own version. | No matrix version and no edge. `XC-03-MATRIX` stays OPEN. |
| ARCH-DEC-16 | Identical-payload replay may return a stored result. Every other conflict stays quarantined. OMSQ-05 B blocks single-match posting. | No role and no write path. XC-09-RESET stays OPEN. |
| ARCH-DEC-17 | No broker observation is accepted as authoritative terminal. | No acceptance rule and no terminal edge. XC-01 and XC-08-IBKR stay OPEN. `EVIDENCE_PENDING` stays. |
| ARCH-DEC-18 | CR-B02 lock order and `READ COMMITTED` stay proposals. A future retry is local only. | No canonical lock protocol or isolation level. XC-05-LOCK stays OPEN. Deadlock tests were not run. |
| ARCH-DEC-19 | An OMS mutation needs a distinct in-transaction audit record and fails if that record cannot be written. | No table name, DDL, hash chain, or retention policy. XC-06 stays OPEN. |
| ARCH-DEC-20 | No modify and no implicit cancel-replace. | No broker dispatch and no action gate. Cancel during halt is not authorized. XC-10-ACTION stays OPEN. |

XC-11 population and supersession stays `RESOLVED_BY_ARCH_DEC_02`. The status-word remainder stays OPEN under ARCH-DEC-04. XC-04 design precedence stays `RESOLVED_BY_ARCH_DEC_03`. The rest of XC-04 stays OPEN. CR-B01 precedence over conflicting or less restrictive DET-07 text stays in force. DET-01 and DET-02 Q01–Q10 verification cells stay OPEN. UD-01 through UD-09 status lines were not rewritten.

## Source and ADFR integrity

Before any edit, all 12 files in `contracts/source-manifest.json` matched that manifest's current `sha256` and `bytes`. Mismatches: 0. `package_sha256` and `package_bytes` remain the TRD-ARCH-SYNC-01 intake values. DET-01 and DET-02 stay `reconstructed_draft` true, `SOURCE_REVIEW_PENDING`, `reviewed` false, and `approved` false.

`contracts/source-manifest.json` was not edited. The 12 manifest source files were not edited. `contracts/registry/registry.json` was not edited. The ADFR JSON and CSV were not edited. ARCH-DEC-01 through ARCH-DEC-08 were not edited.

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
| Manifest SHA-256 | `82b205d27172ad234d46cfa2a59f0f4785128dd7a6282dea1aab32b412fc2808` (17945 bytes) |
| Proposal index SHA-256 | `96dc34c61db04bc0d4eb0576a98f787fddb5632816b903985b1163993afc73cc` (3654 bytes) |

## Non-implementation

No file under `crates/*/src` or `apps/*/src` was edited. No route, adapter, migration, credential, runtime flag, order or risk state machine, broker call, lease, audit write, or financial rounding was added. `Cargo.toml` and `Cargo.lock` were not changed. No production dependency was added. `ORDER_STATE_MACHINE_IMPLEMENTED` and `STATE_TRANSITIONS_IMPLEMENTED` stay false. `OrderClosure` stays absent. No economic posting, risk release, broker terminal acceptance, modify or replace dispatch, or live trading was enabled. ARCH-03B.2 and ARCH-03B.3 were not started.

## Changed files

| Path | SHA-256 | Bytes |
| --- | --- | --- |
| `contracts/decisions/ARCH-DEC-09-bis-20-2026-11-15.md` | `ddaffca4b9d1d69c430a3241db14460e49adf1e8d54a33ac9f0f07b5b957a7a2` | 24820 |
| `contracts/design-proposals/README.md` | `7214c67a21fe37583b571f44fdf9bcf04337a5521abb8033061878bc003951ec` | 2866 |
| `contracts/design-proposals/TRD-ARCH-03B1-approval-requests.md` | `3412b82299b8d31ea6a56fe3ec1b219830690633f35d6ea9c1a3fefde5b72011` | 36374 |
| `contracts/design-proposals/TRD-ARCH-03B1-dependency-and-conflict-matrix.md` | `94be3ca9163b36f32cbe96e2c6cf7042e61633d625e8061f8c15ff0469bda706` | 9802 |
| `contracts/design-proposals/TRD-ARCH-03B12-resolution-index.json` | `e4bd482d2b8b971612e241e069009e86810bfde3cf499784725e6d3b5019ca00` | 6179 |
| `contracts/domain/glossary.md` | `46e018a5602e42002c8e313e7ec8516b346b9827250dccd9c69be8774bfd23ed` | 16922 |
| `contracts/domain/unresolved-decisions.md` | `163b17ff5d84ef6fd886307b7e934e1d0013cfbc4538b3c0edb6e1b0a0e90114` | 20743 |
| `contracts/traceability-index.md` | `8d76dcfbb1006396a34d1c1e00f4182a6bd9ec86722f8111f3b054fc6e451ae3` | 23115 |
| `crates/trading-domain/tests/adfr_reconciliation.rs` | `b456df6e0c4243467b0113de4f04a8ad3a19c047bb36ddd0234bea2d0d4ea65c` | 77028 |
| `crates/trading-domain/tests/registry_skeleton.rs` | `da026f8c31018203d1b23a12c5e4fcb5b136b7768aa26760ac05e0c739e40589` | 18946 |
| `docs/architecture/README.md` | `de3007a5fbdf5bb7b60d9a0df0668cefa6678ddc2c84f8fae737123496b5ee6b` | 13043 |
| `docs/cursor-prompts/trading/TRD-ARCH-03B1.2-Ratify-OMS-Design-Decisions.md` | `7b7dc1e06e22aeab66c16cad90a2e54d9a4b7ed6ae312d6d417e07345a031926` | 17289 |
| `tests/README.md` | `e4f484139f7d60c04a6ff0c5b9c8b92b3426363e226f218e628ecb4496272b66` | 1153 |

This report's own hash is not listed. No Cargo metadata change required a dependency justification.

## Local checks

Remote CI was not run. The ignored PostgreSQL test was not executed against a database. Ignored is not passed.

`cargo fmt --all -- --check` was run once before formatting and exited 1. The diff was layout in `crates/trading-domain/tests/adfr_reconciliation.rs` and `crates/trading-domain/tests/registry_skeleton.rs`. `cargo fmt --all` was then applied.

The first combined run after that formatting, from the repository root:

| Command | Exit code |
| --- | --- |
| `cargo fmt --all -- --check` | 0 |
| `cargo check --workspace --locked` | 0 |
| `cargo test --workspace --locked` | 0 |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | 101 |

The clippy failure was `useless_format` in `oms_design_policy_09_through_20_stays_unimplemented`. The assertion now uses `decision.contains(finding)`.

The four commands were run again after that test edit. None failed.

| Command | Exit code |
| --- | --- |
| `cargo fmt --all -- --check` | 0 |
| `cargo check --workspace --locked` | 0 |
| `cargo test --workspace --locked` | 0 |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | 0 |

`cargo test` result on the passing rerun: 132 passed, 0 failed, 1 ignored. The ignored test is `migration_is_explicit_and_readiness_is_read_only` in `crates/trading-persistence/tests/connectivity.rs`, because `TRADING_ALLOW_DATABASE_INTEGRATION_TESTS=1` and `TRADING_TEST_DATABASE_URL` were not set. It was not run against a database. `adfr_reconciliation` passed 12 tests, including `oms_finality_proposals_stay_unratified_and_registers_stay_closed` and `oms_design_policy_09_through_20_stays_unimplemented`. `registry_skeleton_stays_unpublished` passed.

## Deviations

`TRD-ARCH-03B12-resolution-index.json` is an extra documentation file. The task asked for structured checks rather than brittle prose checks. The JSON is `NON_RUNTIME_NON_WIRE_RESOLUTION_INDEX`. It is not `registry.json` and not a canonical schema. The registry-skeleton allowlist names it and checks the safety flags, including `order_closure_enum_approved` false, `risk_release_approved` false, `implementation_authorized` false, `closes_xc_findings` false, and both trading and schema flags false. The historical proposal index was not edited. It remains `NON_RUNTIME_NON_WIRE_PROPOSAL_INDEX` and `NOT_RATIFIED`.

No evidence file was created. `contracts/source-manifest.json`, `contracts/registry/registry.json`, the 12 package source files, the ADFR JSON and CSV, and the earlier decision records were not edited.

## Gates

| Gate | Result |
| --- | --- |
| A03B12-G01 | PASS. Clean baseline. Predecessor commit `8a7502835176cf3ddc4c14f9f87f0c2092921c9b` and its report confirmed. 12/12 current manifest hashes match. Registry baseline recorded. |
| A03B12-G02 | PASS. ARCH-DEC-09 through ARCH-DEC-20 record the twelve user wordings and options A, C, A, A, B, A, A, A, A, A, A, A. No signature and no evidence URI were invented. |
| A03B12-G03 | PASS. Original proposal questions stay `USER_DECISION_PENDING` in the historical section. Prior decision files, the 12 sources, the manifest, and the proposal index bytes are unchanged. |
| A03B12-G04 | PASS. 78 IDs, same order, JSON/CSV parity, 78 `NOT_VERIFIED`, one supersession `ADFR-CROSS-005` to `ADFR-CR-B10-001`, 77 not superseded. ADFR bytes unchanged. |
| A03B12-G05 | PASS. OMSQ-01 A and OMSQ-02 C are recorded as policy only. No writer, enum, token list, field schema, or terminality claim. |
| A03B12-G06 | PASS. OMSQ-03 A, OMSQ-04 A, OMSQ-09 A, and OMSQ-12 A are recorded. No terminal acceptance, resubmit, modify, or replace authorization. |
| A03B12-G07 | PASS. OMSQ-05 B, OMSQ-06 A, and OMSQ-08 A are consistent. No automatic economic effect, no finality predicate, and no risk release. |
| A03B12-G08 | PASS. OMSQ-07 A, OMSQ-10 A, and OMSQ-11 A are recorded. No approved edge, lock order, isolation level, or audit table. |
| A03B12-G09 | PASS. Traceability and unresolved findings cite the policy record and leave the named XC items open. CR-B01 precedence stays in force. |
| A03B12-G10 | PASS. Registry bytes and the safety flags are unchanged. No production code, schema, migration, broker, dependency, or runtime change. |
| A03B12-G11 | PASS on the rerun. The first `cargo fmt --check` exit 1 and the first `cargo clippy` exit 101 are recorded above. One PostgreSQL test stayed ignored and was not run. Remote CI was not run. |
| A03B12-G12 | PASS when this report and the prompt archive are in the one commit named in the Cursor output. Nothing was pushed. |

## Next step

Not started. ARCH-03B.2 and ARCH-03B.3 were not started. The OMS transition matrix, an `OrderClosure` enum, economic posting, risk release, broker terminal acceptance, deployment, and live trading are not authorized and are not implemented. `XC-03-MATRIX` stays OPEN.
