# TRD-ARCH-03B.3 report

**Date:** 2026-11-15  
**Mode:** documentation and documentation-only tests. No trading behavior was added.  
**Live trading:** NOT AUTHORIZED  
**Canonical schemas:** not published  
**Canonical broker identity tuple:** not approved  
**Economic-finality predicate:** not approved  
**Economic posting:** not approved  
**Reservation release:** forbidden  
**OMS transitions:** none approved  
**Broker dispatch:** not approved  
**Remote CI:** remote CI was not run

`VERDICT: PASS`

`PUSHED: NO`. The commit SHA is the commit that adds this report. It is named in the Cursor output because this file cannot contain the hash of the commit that introduces it. The gates below are the basis for the verdict.

## Baseline

| Item | Value |
| --- | --- |
| Branch | `main` |
| Worktree before edits | clean. `git status --short` printed nothing. |
| BASELINE_HEAD | `9fd8c58e74dd104af542d099a6544813be2cfc43` |
| HEAD subject | `TRD-ARCH-03B.2.2: Ratify Risk Design Decisions` |
| Predecessor report | `docs/architecture/TRD-ARCH-03B.2.2-report.md` present in that commit. SHA-256 `bb93ec2f75bce5cd7eaf986615e84d3693e374c38c448e3aab9bc2a696868bb3`. |
| Predecessor prompt | `docs/cursor-prompts/trading/TRD-ARCH-03B2.2-Ratify-Risk-Design-Decisions.md` present in that commit. SHA-256 `ebe33b3aeb087f2c72bf1fe78a51f1a1cd4b8639f5fa838ed2a636f8ef9f1703`. |
| ARCH-DEC-21..34 | `contracts/decisions/ARCH-DEC-21-bis-34-2026-11-15.md` present in that commit. SHA-256 `a3afc65a0e1a965ac85a9015acc60da858be0ee57ffe104098f2cf56407e5b07` (31894 bytes). |
| rustc | 1.99.1 (58cf8f9c3 2026-10-13) |
| cargo | 1.99.1 (5f94df478 2026-08-27) |
| git | 2.56.0.windows.3 |
| Destructive git | none. Nothing was discarded, reset, or stashed. |

`Cargo.toml`, `contracts/`, and `docs/` are present at the repository root `C:/Users/mifur/dev/cursor/trading`.

Registry flags at baseline were `SKELETON_NOT_CANONICAL`, `canonical_schemas_published=false`, `part_b=NO-GO`, `live_trading_authorized=false`, `broker_uniqueness_evidence=EVIDENCE_PENDING`. Registry SHA-256 `7abed2af5363726403175c6524bd7a4d05d36b8735ad48442fc26941f8d98197` (44332 bytes) is unchanged.

## Authority

The authority is the user instruction dated 2026-11-15 to draft design proposals only. It is archived at `docs/cursor-prompts/trading/TRD-ARCH-03B3-Broker-Identity-and-Snapshot-Completeness-Design-Resolution.md`. That file was not already present. No BROKERQ alternative is ratified. No signature artifact was supplied, and none is invented. No evidence URI is recorded. No IBKR document was retrieved.

## Proposal contents

BROKERQ-01 through BROKERQ-14 are `USER_DECISION_PENDING` and `NOT_RATIFIED`. Count: 14. Each has alternatives A, B, and C. Each C that would book, release, resubmit, overwrite, or treat a lease or an empty snapshot as proof is `CONFLICTS_WITH_BINDING_POLICY`.

Coverage rows: 30. Each row is `EVIDENCE_RETAINED`, `BOOKING_PROHIBITED`, `RELEASE_PROHIBITED`, `RESUBMIT_PROHIBITED`, `BROKER_TERMINAL_ACCEPTANCE_PROHIBITED`, and `ORACLE_NOT_EXECUTED`. Required themes are present: reconnect, missing executions, empty snapshot, reused id, late permId, unknown send, prior writer, split brain, partial fill, cancel race, late fill, correction/bust, duplicate, stale snapshot, incomplete time window, and out-of-order updates.

Identity approaches: `strict_scoped_observation`, `evidence_graph_candidate_sets`, and `optimistic_global_id_merge` (`CONFLICTS_WITH_BINDING_POLICY`). Completeness approaches: `fail_closed_partial_unknown`, `named_scope_proof_obligations`, and `single_response_as_global_complete` (`CONFLICTS_WITH_BINDING_POLICY`). Recovery approaches: `manual_escalation_no_resubmit`, `evidence_rich_manual_recovery`, and `automatic_retry_or_lease_failover` (`CONFLICTS_WITH_BINDING_POLICY`). The non-binding preferences are the three fail-closed baselines. A preference is not an approval.

Preserved conflicts: DET-03-D04 and CR-B04 §4.3 step 4 would book a single `MATCHED` result; ARCH-DEC-13 and ARCH-DEC-26 quarantine that effect. DET-08-D04 and CR-B03 §3.2 describe economic consideration and a compensating line; ARCH-DEC-26 does not adopt the line. DET-06-D03 and CR-B09 P1-B09-D02 discuss cancel during a halt; ARCH-DEC-20 does not authorize it. DET-07 text that is less restrictive than CR-B01 stays subordinate under ARCH-DEC-03. `DESIGN_RESOLVED` stays a source word without verification priority (ARCH-DEC-04, XC-11).

No IBKR uniqueness, sequence number, timeout guarantee, terminality, or completeness criterion is asserted. External cases are `NOT_EXECUTED` and `EXTERNAL_EVIDENCE_PENDING`. No URL was invented.

## Open XC map

| Finding | Status after this proposal |
| --- | --- |
| XC-01 | OPEN. `EVIDENCE_PENDING`. |
| XC-02-PRED | OPEN. No predicate was adopted. |
| XC-02-ALIGN | Principle only. |
| XC-03-BOOK | Principle only. Automatic booking stays forbidden by ARCH-DEC-13. |
| XC-03-MATRIX | OPEN. No edge. |
| XC-04 | Design precedence remains `RESOLVED_BY_ARCH_DEC_03`. The remainder stays OPEN. CR-B01 precedence stays in force. |
| XC-05-OUTBOX | Principle only. |
| XC-05-LOCK | OPEN. No lock order. |
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

ARCH-DEC-13 and ARCH-DEC-14 stay binding. This package does not supersede them. The economic-finality predicate and the OMS transition matrix stay unresolved. Naming them as downstream work does not authorize that work.

## Prohibitions

No identity resolver, uniqueness index, canonical tuple, schema, or migration is approved. Economic posting is not approved. An economic-finality predicate is not approved. Reservation release is forbidden. OMS transitions are not approved. Broker dispatch is not authorized. No retry, takeover, reconciliation cancel, modify, implicit cancel-replace, or risk-reducing broker order is authorized. Live trading is not authorized. Documentation tests do not prove broker behavior. The ignored PostgreSQL test was not run against a database. Ignored is not passed. **NO IMPLEMENTATION.** **NO VERIFICATION.**

## Source and ADFR integrity

Before any edit, all 12 files in `contracts/source-manifest.json` matched that manifest's current `sha256` and `bytes`. Mismatches: 0. The same check was repeated after the edits. Mismatches: 0. `package_sha256` and `package_bytes` remain the TRD-ARCH-SYNC-01 intake values. DET-01 and DET-02 stay `reconstructed_draft` true, `SOURCE_REVIEW_PENDING`, `reviewed` false, and `approved` false. `contracts/source-manifest.json` was not edited. The 12 manifest source files were not edited. `contracts/registry/registry.json` was not edited. The ADFR JSON and CSV were not edited. ARCH-DEC-01 through ARCH-DEC-34 were not edited. Earlier proposal indexes and resolution indexes were not edited. Historical OMSQ and RISKQ question text was not edited.

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
| Prior OMS proposal index SHA-256 | `96dc34c61db04bc0d4eb0576a98f787fddb5632816b903985b1163993afc73cc` (3654 bytes) |
| Prior OMS resolution index SHA-256 | `e4bd482d2b8b971612e241e069009e86810bfde3cf499784725e6d3b5019ca00` (6179 bytes) |
| Historical risk proposal index SHA-256 | `6520b2d64ebcdd00793872ac97a618c9c635e0283d3cf479f7926fc83dac62e6` (7035 bytes) |
| Risk resolution index SHA-256 | `5a09d0daedef4bd9faebfd2ab1f22844fefc8bdb6ac1f906222aea94adb54718` (10530 bytes) |
| ARCH-DEC-01..03 SHA-256 | `9f946806667a3700cb45d5033e509355aeefc60d2216262647f2dbbae034e30a` (9761 bytes) |
| ARCH-DEC-04..08 SHA-256 | `af2cd4b5c35912fe97d3a8ea36c459259aabf0a7b7d54657fdb7b7bce6149ddc` (13679 bytes) |
| ARCH-DEC-09..20 SHA-256 | `ddaffca4b9d1d69c430a3241db14460e49adf1e8d54a33ac9f0f07b5b957a7a2` (24820 bytes) |
| ARCH-DEC-21..34 SHA-256 | `a3afc65a0e1a965ac85a9015acc60da858be0ee57ffe104098f2cf56407e5b07` (31894 bytes) |

## Non-implementation

No file under `crates/*/src` or `apps/*/src` was edited. No route, adapter, migration, credential, runtime flag, order or risk state machine, broker call, lease, audit write, or financial rounding was added. `Cargo.toml` and `Cargo.lock` were not changed. `ORDER_STATE_MACHINE_IMPLEMENTED`, `STATE_TRANSITIONS_IMPLEMENTED`, `IDENTITY_RESOLVER_IMPLEMENTED`, `RISK_EVALUATION_IMPLEMENTED`, `FINANCIAL_ROUNDING_IMPLEMENTED`, and `BROKER_DISPATCH_IMPLEMENTED` stay false. No external live or paper broker request was made.

`TRD-ARCH-03B3-proposal-index.json` is `NON_RUNTIME_NON_WIRE_PROPOSAL_INDEX` and `NOT_RATIFIED`. It is not `registry.json` and not a wire schema. Its trading, posting, release, transition, dispatch, and verification flags are false. `canonical_broker_identity_tuple_approved` and `uniqueness_index_approved` are false. `closes_xc_findings` is false.

The registry JSON allowlist in `crates/trading-domain/tests/registry_skeleton.rs` gained one path, `contracts/design-proposals/TRD-ARCH-03B3-proposal-index.json`, so the new non-wire index is recognized and still forbidden from publishing a schema.

## Changed files

| Path | SHA-256 | Bytes |
| --- | --- | --- |
| `contracts/design-proposals/README.md` | `b365047d8748bb34020eda67847caa4eca2305ff030ab73a45ff9b45d19d05f4` | 7354 |
| `contracts/design-proposals/TRD-ARCH-03B3-approval-requests.md` | `0ae902d63e138833545b85fe23efa745ef20f9a1f1a07d4790af76374290975f` | 31525 |
| `contracts/design-proposals/TRD-ARCH-03B3-broker-identity-semantics-and-invariants.md` | `51f098c9b4e4caa0f9d204496c8976c113139e2ff6a74a8654637202637a3583` | 18980 |
| `contracts/design-proposals/TRD-ARCH-03B3-dependency-and-conflict-matrix.md` | `479ee5cf92d9f09ba80d5fd780d85218ae61c8746d268c85c7b13e14b40020c7` | 7916 |
| `contracts/design-proposals/TRD-ARCH-03B3-identity-snapshot-and-recovery-options.md` | `5939cb69b1fc198d961904e1f4c9371afe2d7810e33c5ff168734b220ec6e2c8` | 13127 |
| `contracts/design-proposals/TRD-ARCH-03B3-proposal-index.json` | `e808dcf8a2339845b4ef49085275b7aad9094862086830d57d13a415841703ce` | 13593 |
| `contracts/design-proposals/TRD-ARCH-03B3-snapshot-completeness-and-reconciliation-coverage-draft.md` | `5252d0d5a6864273e065bde7cfd0d2d0ecb75b70f7d43334b9ba4c967b515173` | 17455 |
| `contracts/domain/glossary.md` | `d536bbb51914257dc9bd3a5d175dc3f8c1cee9466c2fa62b9195b9e27d5aaf7a` | 17932 |
| `contracts/domain/unresolved-decisions.md` | `614c68c46968276f9138a13be28f612aec61377a61c477a55296c4907e72ffe4` | 24240 |
| `contracts/traceability-index.md` | `a3aef3aa2b3a66e823b6fc2c6593e6d54d979fdf94606cdf290776dc2d2093bd` | 27548 |
| `crates/trading-domain/tests/adfr_reconciliation.rs` | `490a77bd61d4f80be0cc881b660c3092b87df40ced8f5482587398d56deac96c` | 134868 |
| `crates/trading-domain/tests/registry_skeleton.rs` | `9024624111a223eac14f3358e78fe9ce840dd103216fcff773979e19b53ec1a9` | 25846 |
| `docs/architecture/README.md` | `d115db98a9f0211827388ba426bfeb1fd692d3c945d15c6ef0a84238c4b9744c` | 14448 |
| `docs/cursor-prompts/trading/TRD-ARCH-03B3-Broker-Identity-and-Snapshot-Completeness-Design-Resolution.md` | `0c36291f4e9ba3d95da776eea50b225e346140e8187c77f31b2b910ba2eb12bb` | 16455 |
| `tests/README.md` | `17c3c393a4df2f577099cc82c67eb50c3f65e421778eaa846295561a9aef2437` | 1562 |

This report's own hash is not listed. No Cargo metadata change required a dependency justification.

## Local checks

Remote CI was not run. The ignored PostgreSQL test was not executed against a database. Ignored is not passed.

`cargo fmt --all -- --check` was run once before formatting and exited 1. The diff was layout in `crates/trading-domain/tests/adfr_reconciliation.rs` only. `cargo fmt --all` was then applied. A second `cargo fmt --all -- --check` exited 0.

The other three commands were run from the repository root after that formatting. None failed. Clippy did not fail, so there was no second clippy correction.

| Command | Exit code |
| --- | --- |
| `cargo fmt --all -- --check` | 0 on the run after formatting. The first run exited 1. |
| `cargo check --workspace --locked` | 0 |
| `cargo test --workspace --locked` | 0 |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | 0 |

`cargo test` result: 135 passed, 0 failed, 1 ignored. The ignored test is `migration_is_explicit_and_readiness_is_read_only` in `crates/trading-persistence/tests/connectivity.rs`, because `TRADING_ALLOW_DATABASE_INTEGRATION_TESTS=1` and `TRADING_TEST_DATABASE_URL` were not set. It was not run against a database. `adfr_reconciliation` passed 15 tests, including `broker_identity_proposals_stay_unratified_and_dispatch_stays_forbidden`. `registry_skeleton_stays_unpublished` passed.

These documentation tests do not prove financial safety, reconciliation, or broker behavior. They do not execute P0-B04-T01 through T10 or B07-T01 through T07.

## Deviations

`contracts/design-proposals/README.md`, `contracts/domain/unresolved-decisions.md`, `contracts/traceability-index.md`, `docs/architecture/README.md`, `contracts/domain/glossary.md`, and `tests/README.md` received pointer text for the new unratified package. Historical OMSQ and RISKQ question text was not rewritten. In the design-proposal and architecture indexes, the sentence that ARCH-03B.3 had not been started by the earlier package was put in the past tense so the new section is not contradicted. The TRD-ARCH-03B.2.2 paragraphs in `unresolved-decisions.md` and `traceability-index.md` still contain the original "ARCH-03B.3 is not started" sentence, and the following section says that sentence belongs to the earlier reading.

The registry JSON allowlist gained the one new proposal index. No other allowlist change was required.

No evidence file was created. `contracts/source-manifest.json`, `contracts/registry/registry.json`, the 12 package source files, the ADFR JSON and CSV, the decision records, and the earlier proposal indexes were not edited.

## Gates

| Gate | Result |
| --- | --- |
| A03B3-G01 | PASS. Clean baseline `9fd8c58e74dd104af542d099a6544813be2cfc43` on `main`. Predecessor report, predecessor prompt, and ARCH-DEC-21..34 are in that commit. Tool versions are recorded above. |
| A03B3-G02 | PASS. 12/12 manifest `sha256` and `bytes` matched before the edits and again after them. Provenance and package intake fields were not modified. |
| A03B3-G03 | PASS. 78 JSON and CSV ids, 78 `NOT_VERIFIED`, one supersession. Registry hash and safety flags unchanged. ARCH-DEC-01..34 files unchanged. |
| A03B3-G04 | PASS. ARCH-DEC-01..34 retained. CR-B01 precedence explicit. No fabricated approval, evidence URI, or status promotion. |
| A03B3-G05 | PASS. Identity namespaces, ambiguity labels, and source conflicts are in the semantics note. No uniqueness proof and no canonical tuple. |
| A03B3-G06 | PASS. 30 coverage rows. No invented completeness guarantee. Economic and dispatch prohibitions are on every row. External evidence is pending. |
| A03B3-G07 | PASS. Crash windows, lost acknowledgement, reconnect, prior writer, and split brain are covered. No automatic takeover, resend, cancel, or dispatch. |
| A03B3-G08 | PASS. 14 independent BROKERQ items, each with A/B/C, impact, a non-binding recommendation, dependencies, and `USER_DECISION_PENDING`. |
| A03B3-G09 | PASS. XC-01 and XC-08-IBKR stay OPEN. XC-04 precedence is noted and the operational remainder stays OPEN. XC-02-PRED and the other blockers stay OPEN. |
| A03B3-G10 | PASS. No production source, schema, migration, dependency, broker call, booking, release, transition, or live-trading change. |
| A03B3-G11 | PASS on the run after formatting. The first `cargo fmt --check` exit 1 is recorded above. 135 passed, 0 failed, 1 ignored. The ignored PostgreSQL test was not run. Remote CI was not run. |
| A03B3-G12 | PASS when this report and the prompt archive are in the one commit named in the Cursor output. Nothing was pushed. |

## Next step

Not started. BROKERQ-01 through BROKERQ-14 are not ratified. A later ratification milestone is not authorized by this package. The economic-finality predicate, the OMS transition matrix, an `OrderClosure` enum, economic posting, risk release, broker terminal acceptance, broker dispatch, and live trading are not authorized and are not implemented. `XC-01`, `XC-02-PRED`, and `XC-08-IBKR` stay OPEN.
