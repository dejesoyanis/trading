# TRD-ARCH-03B.2 report

**Date:** 2026-11-15  
**Mode:** design, documentation, and documentation-only validation. No trading behavior was added.  
**Live trading:** NOT AUTHORIZED  
**Canonical schemas:** not published  
**Economic-finality predicate:** not approved  
**Economic posting:** not approved  
**Reservation release:** forbidden  
**OMS transitions:** none approved  
**Remote CI:** remote CI was not run

`VERDICT: PASS`

`PUSHED: NO`. The commit SHA is the commit that adds this report. It is named in the Cursor output because this file cannot contain the hash of the commit that introduces it. The gates below are the basis for the verdict.

## Baseline

| Item | Value |
| --- | --- |
| Branch | `main` |
| Worktree before edits | clean. `git status --short` printed nothing. |
| BASELINE_HEAD | `f2b61d7b465b3a815ccfbeab26511b23939f8a0e` |
| HEAD subject | `TRD-ARCH-03B.1.2: Ratify OMS Design Decisions` |
| Predecessor report | `docs/architecture/TRD-ARCH-03B.1.2-report.md` present in that commit |
| Predecessor prompt | `docs/cursor-prompts/trading/TRD-ARCH-03B1.2-Ratify-OMS-Design-Decisions.md` present in that commit |
| rustc | 1.99.1 (58cf8f9c3 2026-10-13) |
| cargo | 1.99.1 (5f94df478 2026-08-27) |
| git | 2.56.0.windows.3 |
| Destructive git | none. Nothing was discarded, reset, or stashed. |

The predecessor commit `f2b61d7b465b3a815ccfbeab26511b23939f8a0e` lists both the report and the prompt archive in `git log -1 --name-only`. The branch was ahead of `origin/main` by that one commit. This task does not push.

Registry flags at baseline were `SKELETON_NOT_CANONICAL`, `canonical_schemas_published=false`, `part_b=NO-GO`, `live_trading_authorized=false`, `broker_uniqueness_evidence=EVIDENCE_PENDING`. Registry SHA-256 `7abed2af5363726403175c6524bd7a4d05d36b8735ad48442fc26941f8d98197` (44332 bytes) is unchanged.

## Files inspected

| Requested item | Path used |
| --- | --- |
| TRD-ARCH-SYNC-01 report | `docs/architecture/TRD-ARCH-SYNC-01-report.md`, including the XC-01 through XC-09 sections used as finding text |
| TRD-ARCH-SYNC-02 report | `docs/architecture/TRD-ARCH-SYNC-02-report.md` |
| TRD-ARCH-03A report | `docs/architecture/TRD-ARCH-03A-report.md` located. Not edited. |
| TRD-ARCH-03A.2 report | `docs/architecture/TRD-ARCH-03A.2-report.md` located. Not edited. |
| TRD-ARCH-03B.1 report | `docs/architecture/TRD-ARCH-03B.1-report.md` located. Not edited. |
| TRD-ARCH-03B.1.2 report | `docs/architecture/TRD-ARCH-03B.1.2-report.md` |
| ARCH-DEC-01..03 | `contracts/decisions/ARCH-DEC-01-bis-03-2026-11-15.md`, including the DET-02 exception list |
| ARCH-DEC-04..08 | `contracts/decisions/ARCH-DEC-04-bis-08-2026-11-15.md` located and not edited. Status-word precedence stays absent. |
| ARCH-DEC-09..20 | `contracts/decisions/ARCH-DEC-09-bis-20-2026-11-15.md` |
| DET-01, DET-02 | the two reconstructed source files, read for the risk and finality sections |
| DET-03..08 | `contracts/sources/DET-03-bis-DET-08-Architecture-Contracts-v1.0.md`, DET-07 and DET-08 by heading |
| DET-09..10 | `contracts/sources/DET-09-bis-DET-10-und-Cross-Contract-Review-v1.0.md` |
| CR-B01..04 | `contracts/sources/P0-Resolution-Contracts-CR-B01-bis-CR-B04-v1.0.md`, CR-B02, CR-B03, and CR-B04 by heading. CR-B01 precedence is the ARCH-DEC-03 rule already in force. |
| CR-B05..09 | `contracts/sources/P1-Resolution-Contracts-CR-B05-bis-CR-B09-v1.0.md` |
| CR-B10 | `contracts/sources/CR-B10-Domain-Glossary-and-Schema-Registry-v1.0.md` |
| Architecture consolidation | `contracts/sources/Architecture-Consolidation-and-Acceptance-v1.0.md` |
| OMS proposal package | the TRD-ARCH-03B1 notes, proposal index, and resolution index. Not edited. |
| Status model | `contracts/status-model/` located. Not edited. |
| Traceability, unresolved decisions, glossary | the three files under `contracts/` |
| Manifest, ADFR, registry, tests | `contracts/source-manifest.json`, both ADFR files, `contracts/registry/registry.json`, `crates/trading-domain/tests/adfr_reconciliation.rs`, `crates/trading-domain/tests/registry_skeleton.rs` |

No requested file was missing. DET-07 and CR-B01 remain sections inside the combined source files. No content was invented for an absent file. No IBKR evidence file was added.

## Binding policy and proposed semantics

ARCH-DEC-09 through ARCH-DEC-20 stay `BINDING_EXISTING`. This package does not weaken them and does not record a new user decision. RISKQ-01 through RISKQ-14 are `USER_DECISION_PENDING`. Recommendations are not approvals.

| Policy | Effect on this package |
| --- | --- |
| ARCH-DEC-13, OMSQ-05 B | Every proposed economic effect stays quarantined, including a single consistent identity match. |
| ARCH-DEC-14, OMSQ-06 A | No executable economic-finality predicate. Reservation release is forbidden. The coverage draft is not that predicate. |
| ARCH-DEC-09, ARCH-DEC-10 | One immutable closure record. Reason family versus terminality is strategy only. |
| ARCH-DEC-11, ARCH-DEC-03 | Unknown send does not close the workflow and does not authorize a second submission. CR-B01 precedence stays in force. |
| ARCH-DEC-12, ARCH-DEC-17 | Cancel and fill facts stay separate. No broker observation is authoritative terminal. |
| ARCH-DEC-16 | Identical-payload replay returns a stored result and creates no new economic effect. |
| ARCH-DEC-18, ARCH-DEC-19 | Lock order and isolation stay unadopted. The audit record stays unnamed. |
| ARCH-DEC-20 | Modify and implicit cancel-replace stay blocked. No risk-reducing broker action is authorized. |
| ARCH-DEC-01 | DET-01 and DET-02 stay `RECONSTRUCTED_DRAFT` and `DESIGN_BASELINE_RATIFIED_WITH_EXCEPTIONS`. Q01–Q10 verification cells stay OPEN. The reference inequality is not a netting rule. |

`XC-02-PRED` stays OPEN. A principle that closure is not release (XC-02-ALIGN) is not an executable predicate.

## Deliverable index

| File | Role |
| --- | --- |
| `contracts/design-proposals/TRD-ARCH-03B2-risk-semantics-and-invariants.md` | Dimensions, invariant inventory, lifecycle scenarios, fail-closed rules, design-level unit of work |
| `contracts/design-proposals/TRD-ARCH-03B2-economic-finality-and-release-coverage-draft.md` | Evidence categories and 21 coverage rows. Booking and release prohibited on every row |
| `contracts/design-proposals/TRD-ARCH-03B2-reservation-and-correction-options.md` | Three comparison architectures. Non-binding preference is the conservative hold |
| `contracts/design-proposals/TRD-ARCH-03B2-approval-requests.md` | RISKQ-01 through RISKQ-14 |
| `contracts/design-proposals/TRD-ARCH-03B2-dependency-and-conflict-matrix.md` | Findings, handoffs, blockers |
| `contracts/design-proposals/TRD-ARCH-03B2-proposal-index.json` | Non-runtime index for documentation tests |

## RISKQ status

Every row is `USER_DECISION_PENDING`. The non-binding recommendation is not a decision. Alternative C on each question is marked `CONFLICTS_WITH_BINDING_POLICY` and is not available for ratification without an explicit superseding decision. This package does not supersede ARCH-DEC-09 through ARCH-DEC-20.

| ID | Subject | Non-binding recommendation |
| --- | --- | --- |
| RISKQ-01 | Hierarchical scope shape, no executable math | A. Keep the baseline scope names and deny-overrides. Adopt no algebra. |
| RISKQ-02 | Reservation identity, idempotency, cross-scope contention | A. One reservation id, all-or-nothing scopes, no lock order. |
| RISKQ-03 | Missing price, liquidity, or FX | A. No reservation and no `ALLOW`. |
| RISKQ-04 | Partial fill versus residual obligation | A. Record the fill. Do not convert and do not release. |
| RISKQ-05 | Cancel request, cancel observation, residual | A. Keep the facts. Residual stays. |
| RISKQ-06 | Late fill, duplicate, bust, correction | A. Retain and quarantine. Draft no compensating line. |
| RISKQ-07 | Insufficient budget, halt, reduction | A. Deny or block new risk-increasing actions. No broker reduce. |
| RISKQ-08 | Future evidence bar under ARCH-DEC-14 | A. Adopt no list as a decision. Finality stays undefined. |
| RISKQ-09 | Future release proofs | A. Accept no proof as sufficient. Release stays forbidden. |
| RISKQ-10 | Valuation, FX, rounding | A. Choose no mode, rate, or threshold. |
| RISKQ-11 | Isolation, locks, CAS, retry | A. Leave the protocol unadopted. Local retry only, as already bound. |
| RISKQ-12 | Audit authority | A. Restate the unnamed distinct record. No table. |
| RISKQ-13 | Manual adjudication | A. Add no role and no simulated approver. |
| RISKQ-14 | Recovery, replay, policy versions | A. No release and no resubmit on recovery. Tests stay `NOT_EXECUTED`. |

Count: 14. Identifiers: RISKQ-01, RISKQ-02, RISKQ-03, RISKQ-04, RISKQ-05, RISKQ-06, RISKQ-07, RISKQ-08, RISKQ-09, RISKQ-10, RISKQ-11, RISKQ-12, RISKQ-13, RISKQ-14.

## Open XC map

| Finding | Status after this package |
| --- | --- |
| XC-01 | OPEN. `EVIDENCE_PENDING`. |
| XC-02-PRED | OPEN. |
| XC-02-ALIGN | Principle only. |
| XC-03-BOOK | Principle only. Automatic booking stays forbidden by ARCH-DEC-13. |
| XC-03-MATRIX | OPEN. No edge. |
| XC-04 | Design precedence remains `RESOLVED_BY_ARCH_DEC_03`. The remainder stays OPEN. |
| XC-05-OUTBOX | Principle only. |
| XC-05-LOCK | OPEN. Deadlock tests were not run. |
| XC-06 | OPEN. No table name. |
| XC-07-FX | Principle only. |
| XC-07-ROUND | OPEN. No rounding mode. |
| XC-08-RULE | Principle only. |
| XC-08-IBKR | OPEN. No completeness proof. |
| XC-09-SOLO | Principle only. |
| XC-09-RESET | OPEN. |
| XC-10-RACE | Principle only. |
| XC-10-ACTION | OPEN. |
| XC-11 | Status words stay OPEN. Population and supersession stay `RESOLVED_BY_ARCH_DEC_02`. |

ARCH-03B.3 is the handoff for broker identity and snapshot completeness. It is not started. Later OMS matrix work remains the handoff for transition edges. It is not started.

## Prohibitions

Risk posting is not approved. An economic-finality predicate is not approved. Reservation release is forbidden. OMS transitions are not approved. Canonical schemas are not published. Broker dispatch is not authorized. Live trading is not authorized. No modify or replace path was enabled. Documentation tests do not prove financial safety. The ignored PostgreSQL test was not run against a database. Ignored is not passed.

## Source and ADFR integrity

Before any edit, all 12 files in `contracts/source-manifest.json` matched that manifest's current `sha256` and `bytes`. Mismatches: 0. `package_sha256` and `package_bytes` remain the TRD-ARCH-SYNC-01 intake values. DET-01 and DET-02 stay `reconstructed_draft` true, `SOURCE_REVIEW_PENDING`, `reviewed` false, and `approved` false.

`contracts/source-manifest.json` was not edited. The 12 manifest source files were not edited. `contracts/registry/registry.json` was not edited. The ADFR JSON and CSV were not edited. ARCH-DEC-01 through ARCH-DEC-20 were not edited. The TRD-ARCH-03B.1 proposal files and `TRD-ARCH-03B12-resolution-index.json` were not edited.

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
| Prior proposal index SHA-256 | `96dc34c61db04bc0d4eb0576a98f787fddb5632816b903985b1163993afc73cc` (3654 bytes) |
| Prior resolution index SHA-256 | `e4bd482d2b8b971612e241e069009e86810bfde3cf499784725e6d3b5019ca00` (6179 bytes) |

## Non-implementation

No file under `crates/*/src` or `apps/*/src` was edited. No route, adapter, migration, credential, runtime flag, order or risk state machine, broker call, lease, audit write, or financial rounding was added. `Cargo.toml` and `Cargo.lock` were not changed. No production dependency was added. `ORDER_STATE_MACHINE_IMPLEMENTED` and `STATE_TRANSITIONS_IMPLEMENTED` stay false. `OrderClosure` stays absent. `RISK_EVALUATION_IMPLEMENTED` and `FINANCIAL_ROUNDING_IMPLEMENTED` stay false. No economic posting, risk release, broker terminal acceptance, modify or replace dispatch, or live trading was enabled. ARCH-03B.3 was not started.

## Changed files

| Path | SHA-256 | Bytes |
| --- | --- | --- |
| `contracts/design-proposals/README.md` | `44fee8eea31f27f5d29e5bd46de2d1905e91abc0f88e042970af503896243e7d` | 4318 |
| `contracts/design-proposals/TRD-ARCH-03B2-approval-requests.md` | `2d32dd7381a83745a9477b5b21905872fdbf45985a65fe2a810f6b4e0d4b4e71` | 39350 |
| `contracts/design-proposals/TRD-ARCH-03B2-dependency-and-conflict-matrix.md` | `5c67b0aba6340a2a2c8e849e3c5075186d91be9f88b9356c8816331ed81f3473` | 9310 |
| `contracts/design-proposals/TRD-ARCH-03B2-economic-finality-and-release-coverage-draft.md` | `067d2f0f9b743d9f86706a5c317f6ad4f22f17b7ec5f94ee4bd0a7c5d4f7cb83` | 20069 |
| `contracts/design-proposals/TRD-ARCH-03B2-proposal-index.json` | `6520b2d64ebcdd00793872ac97a618c9c635e0283d3cf479f7926fc83dac62e6` | 7035 |
| `contracts/design-proposals/TRD-ARCH-03B2-reservation-and-correction-options.md` | `eeb3e631f843cd1a204583a8d293fba696f94fc4d0667ccca70b5ab1fc58e925` | 13718 |
| `contracts/design-proposals/TRD-ARCH-03B2-risk-semantics-and-invariants.md` | `4da327a9126e1a0bfc8eb296176c2b0deb1c89949da6850091cd3bb918e0cfdc` | 26145 |
| `contracts/domain/glossary.md` | `faa30def66512340cfd25ae9f64053924a05e9c199b9fba25a36ccc95f10536c` | 17175 |
| `contracts/domain/unresolved-decisions.md` | `82ba21c791d865654f1dbe48d3a6bd8efd977cfb0f71163a408ed09566b378f0` | 21291 |
| `contracts/traceability-index.md` | `5409d0aa28dd5ba0a03d418fd8415bcc8de7a187b7ea70cfeb969b0bfde126a9` | 23838 |
| `crates/trading-domain/tests/adfr_reconciliation.rs` | `09a86252bb319b27f87906027a7101f0c3a2035bf50ead3de336640a7fedbee4` | 90146 |
| `crates/trading-domain/tests/registry_skeleton.rs` | `f80266d9d9b4981e7af02bdb18b9f6d59f841a6705e3e75c54410f9966d73212` | 20711 |
| `docs/architecture/README.md` | `0063e85d16a6f3a704e67f46f6b612bcec990bf2b7c8cba2b06871669c18202e` | 13388 |
| `docs/cursor-prompts/trading/TRD-ARCH-03B2-Risk-Reservation-and-Economic-Finality-Design-Resolution.md` | `2a04dbc9c8103f328d0fabf7980cdb2ea50b0b2bd08b86d5f1206f9e907fa7fd` | 20602 |
| `tests/README.md` | `2ba6adb8e95b6b909137fdd7d38418654a256f060a84f252e45cdd9c9ef2d826` | 1277 |

This report's own hash is not listed. No Cargo metadata change required a dependency justification.

## Local checks

Remote CI was not run. The ignored PostgreSQL test was not executed against a database. Ignored is not passed.

`cargo fmt --all -- --check` was run once before formatting and exited 1. The diff was layout in `crates/trading-domain/tests/adfr_reconciliation.rs`. `cargo fmt --all` was then applied.

The four commands were run from the repository root after that formatting. None failed. Clippy did not fail on this run, so there was no second clippy correction.

| Command | Exit code |
| --- | --- |
| `cargo fmt --all -- --check` | 0 |
| `cargo check --workspace --locked` | 0 |
| `cargo test --workspace --locked` | 0 |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | 0 |

`cargo test` result: 133 passed, 0 failed, 1 ignored. The ignored test is `migration_is_explicit_and_readiness_is_read_only` in `crates/trading-persistence/tests/connectivity.rs`, because `TRADING_ALLOW_DATABASE_INTEGRATION_TESTS=1` and `TRADING_TEST_DATABASE_URL` were not set. It was not run against a database. `adfr_reconciliation` passed 13 tests, including `risk_reservation_proposals_stay_unratified_and_release_stays_forbidden` and `oms_design_policy_09_through_20_stays_unimplemented`. `registry_skeleton_stays_unpublished` passed.

These documentation tests do not prove financial safety, deadlock freedom, reconciliation, or broker behavior.

## Deviations

`TRD-ARCH-03B2-proposal-index.json` is the optional documentation index the task allowed. It is `NON_RUNTIME_NON_WIRE_PROPOSAL_INDEX` and `NOT_RATIFIED`. It is not `registry.json` and not a canonical schema. The registry-skeleton allowlist names it and checks `risk_release_approved` false, `economic_posting_approved` false, `economic_finality_predicate_approved` false, `closes_xc_findings` false, and both trading and schema flags false. The historical proposal index and the resolution index were not edited.

`contracts/domain/glossary.md` received one paragraph stating that this package does not approve release, posting, or a predicate. Type rows were not rewritten.

No evidence file was created. `contracts/source-manifest.json`, `contracts/registry/registry.json`, the 12 package source files, the ADFR JSON and CSV, and the earlier decision records were not edited.

## Gates

| Gate | Result |
| --- | --- |
| A03B2-G01 | PASS. Clean baseline. Predecessor commit `f2b61d7b465b3a815ccfbeab26511b23939f8a0e` includes the 03B.1.2 report and prompt archive. 12/12 current manifest hashes match. ADFR and registry baselines recorded. |
| A03B2-G02 | PASS. ARCH-DEC-01 through ARCH-DEC-20 stay in force. DET-01 and DET-02 exceptions stay open. OMSQ-05 B and OMSQ-06 A are restated as binding. No user decision was fabricated. |
| A03B2-G03 | PASS. Dimensions, hierarchy, identity, invariants, and trust boundaries are in the semantics note with source anchors. Missing definitions are listed. |
| A03B2-G04 | PASS. Twenty-one coverage rows keep record=`RETAIN`, book=`PROHIBITED`, release=`PROHIBITED`, oracle=`NOT_EXECUTED`. No predicate and no release are approved. |
| A03B2-G05 | PASS. Three comparison architectures record failure modes. No arithmetic and no posting are promoted to runtime. |
| A03B2-G06 | PASS. Fourteen independent RISKQ questions, each `USER_DECISION_PENDING`, with alternatives, consequence tables, and a non-binding recommendation. |
| A03B2-G07 | PASS. `XC-02-PRED` stays OPEN. ARCH-03B.3 and the OMS matrix handoffs are explicit. |
| A03B2-G08 | PASS. 78 IDs, JSON/CSV parity, 78 `NOT_VERIFIED`, one supersession, 12 manifest sources. Manifest and prior decision records unchanged. |
| A03B2-G09 | PASS. Registry bytes and safety flags unchanged. No production source, schema, migration, broker, booking, release, or live-trading change. |
| A03B2-G10 | PASS. Navigation and the unresolved-decision index point at the unratified package. Historical decision records and the 03B.1 snapshot were not rewritten. |
| A03B2-G11 | PASS on the run after formatting. The first `cargo fmt --check` exit 1 is recorded above. One PostgreSQL test stayed ignored and was not run. Remote CI was not run. |
| A03B2-G12 | PASS when this report and the prompt archive are in the one commit named in the Cursor output. Nothing was pushed. |

## Next step

Not started. ARCH-03B.3 was not started. The OMS transition matrix, an `OrderClosure` enum, economic posting, risk release, an economic-finality predicate, broker terminal acceptance, deployment, and live trading are not authorized and are not implemented. `XC-02-PRED` stays OPEN.
