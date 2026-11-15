# TRD-ARCH-03B.4 report

**Date:** 2026-11-15  
**Mode:** documentation-only evidence and verification planning. No trading behavior was added. The plan was not executed.  
**Live trading:** NOT AUTHORIZED  
**Canonical schemas:** not published  
**Broker uniqueness:** `EVIDENCE_PENDING`  
**External broker evidence:** `EXTERNAL_EVIDENCE_PENDING`  
**External cases:** `NOT_EXECUTED`  
**Economic posting:** not approved  
**Reservation release:** forbidden  
**Economic-finality predicate:** not approved  
**OMS transitions:** none approved  
**Broker dispatch:** not approved  
**Broker connection:** not authorized  
**Remote CI:** remote CI was not run

`VERDICT: PASS`

`PUSHED: NO`. The commit SHA is the commit that adds this report. It is named in the Cursor output because this file cannot contain the hash of the commit that introduces it. The gates below are the basis for the verdict.

## Baseline

| Item | Value |
| --- | --- |
| Branch | `main` |
| Worktree before edits | clean. `git status --short` printed nothing. |
| BASELINE_HEAD | `c20ce5fb2686bd4038342819f364a04ba56b0e4d` |
| HEAD subject | `TRD-ARCH-03B.3.2: Ratify Broker Design Decisions` |
| Predecessor report | `docs/architecture/TRD-ARCH-03B.3.2-report.md` present in that commit |
| Predecessor prompt | `docs/cursor-prompts/trading/TRD-ARCH-03B3.2-Ratify-Broker-Design-Decisions.md` present in that commit |
| Decision record | `contracts/decisions/ARCH-DEC-35-bis-48-2026-11-15.md` present |
| rustc | 1.99.1 (58cf8f9c3 2026-10-13) |
| cargo | 1.99.1 (5f94df478 2026-08-27) |
| git | 2.56.0.windows.3 |
| Destructive git | none. Nothing was discarded, reset, or stashed. |

ARCH-DEC-35 through ARCH-DEC-48 were already assigned by the predecessor. This task did not add a decision identifier and did not edit those files. ARCH-DEC-49 stays unused. No identifier was renumbered.

The choice mapping in the predecessor record is 13 times B and 1 time A. BROKERQ-07 is the only A. Each row is `RATIFIED_DESIGN_POLICY_ONLY`, `NOT_IMPLEMENTED`, and `NOT_VERIFIED`. This plan does not implement or verify them.

Registry flags at baseline were `SKELETON_NOT_CANONICAL`, `canonical_schemas_published=false`, `part_b=NO-GO`, `live_trading_authorized=false`, `broker_uniqueness_evidence=EVIDENCE_PENDING`. Registry SHA-256 `7abed2af5363726403175c6524bd7a4d05d36b8735ad48442fc26941f8d98197` (44332 bytes) matches the previously reported value and is unchanged.

No `contracts/verification-plans/` directory existed. The plan is placed there. It is not registered in `contracts/registry/registry.json`.

## Authority and scope

The authority for this milestone is the user instruction dated 2026-11-15, archived at `docs/cursor-prompts/trading/TRD-ARCH-03B4-IBKR-Evidence-and-Verification-Planning.md`. The archive starts with one identifying sentence and then the executed instruction. No signature artifact was supplied, and none is invented. No evidence URI is recorded. The instruction authorizes documentation-only planning. It does not authorize a broker connection, a read, an order, a paper session, or a test run.

IBKREVQ-01 through IBKREVQ-14 stay `USER_DECISION_PENDING` and `NOT_RATIFIED`. A recommendation in that file is not an approval and is not testing authority. Option C on every question is `CONFLICTS_WITH_BINDING_POLICY`. No option was selected.

Stages E0 through E4 have checkpoint STOP. `PROOF_OBLIGATION_OWNER_PLACEHOLDER` is not an operator role.

## Counts

| Item | Count |
| --- | --- |
| Claims | 27 (`IBKR-CLM-01` through `IBKR-CLM-27`) |
| Scenarios | 43 (`IBKR-SCN-A01`..`A14`, `B01`..`B11`, `C01`..`C12`, `D01`..`D06`) |
| Questions | 14 (`IBKREVQ-01` through `IBKREVQ-14`) |

Every claim is `HYPOTHESIS`, `EXTERNAL_EVIDENCE_PENDING`, `NOT_EXECUTED`, and `NOT_AUTHORIZED`. Every scenario is `NOT_EXECUTED` and `EXTERNAL_EVIDENCE_PENDING`. No uniqueness assertion is made. No API operation name is asserted. Research topics are marked `UNVERIFIED_RESEARCH_TOPIC`.

## Open XC map

| Finding | Status after this plan |
| --- | --- |
| XC-01 | OPEN. `EVIDENCE_PENDING`. Not closed. |
| XC-02-PRED | OPEN. No predicate was adopted. Broker evidence would not close it. |
| XC-02-ALIGN | Principle only. |
| XC-03-BOOK | Principle only. Automatic booking stays forbidden by ARCH-DEC-13. |
| XC-03-MATRIX | OPEN. No edge. |
| XC-04 | Design precedence remains `RESOLVED_BY_ARCH_DEC_03`. The operational remainder stays OPEN. CR-B01 precedence stays in force. |
| XC-05-OUTBOX | Principle only. |
| XC-05-LOCK | OPEN. No lock order. |
| XC-06 | OPEN. No table name. |
| XC-07-FX | Principle only. |
| XC-07-ROUND | OPEN. No rounding mode. |
| XC-08-RULE | Principle only. |
| XC-08-IBKR | OPEN. `EXTERNAL_EVIDENCE_PENDING`. No completeness proof. |
| XC-09-SOLO | Principle only. |
| XC-09-RESET | OPEN. No reset right. |
| XC-10-RACE | Principle only. |
| XC-10-ACTION | OPEN. No broker action. |
| XC-11 | Status words stay OPEN. Population and supersession stay `RESOLVED_BY_ARCH_DEC_02`. |

The dependency matrix states what a later package would have to contain before anyone could consider closing `XC-01`, `XC-08-IBKR`, or the operational remainder of `XC-04`. Those lists are entry conditions. They are not closures.

The DET-03-D04 and CR-B04 §4.3 sentence that a single match books stays subordinate to ARCH-DEC-13 and ARCH-DEC-26. No compensating ledger line is approved. CR-B01 keeps design precedence over conflicting or less restrictive DET-07 text.

## Prohibitions

No broker login, connection, API request, TWS or IB Gateway startup, account access, paper order, or live order was performed. No uniqueness guarantee, global alias key, canonical identity tuple, UNIQUE index, identity resolver, schema, DDL, or migration is approved. No economic posting, compensating or reversal line, reservation conversion, shrink, or release is approved. No economic-finality predicate, OMS transition, terminal acceptance, cancel, modify, replace, broker dispatch, retry, resubmit, or automatic takeover is approved. No completeness token is ratified. An empty response is not a negative proof. A local capture gap is not an IBKR sequence. Documentation tests do not prove broker behavior. The ignored PostgreSQL test was not run against a database. Ignored is not passed. **NO IMPLEMENTATION.** **NO VERIFICATION.**

## Source and ADFR integrity

Before any edit, all 12 files in `contracts/source-manifest.json` matched that manifest's current `sha256` and `bytes`. Mismatches: 0. The same check is asserted again in `ibkr_evidence_verification_plan_stays_unexecuted`. `package_sha256` and `package_bytes` remain the TRD-ARCH-SYNC-01 intake values. DET-01 and DET-02 stay `reconstructed_draft` true, `SOURCE_REVIEW_PENDING`, `reviewed` false, and `approved` false.

`contracts/source-manifest.json` was not edited. The 12 manifest source files were not edited. `contracts/registry/registry.json` was not edited. The ADFR JSON and CSV were not edited. ARCH-DEC-01 through ARCH-DEC-48 were not edited. The historical proposal and resolution indexes were not edited.

| Item | Value, unchanged from baseline |
| --- | --- |
| JSON count | 78 |
| CSV count | 78 |
| `verification` | 78 `NOT_VERIFIED` |
| Superseded | 1: `ADFR-CROSS-005`, `superseded_by` `ADFR-CR-B10-001` |
| Not superseded | 77 |
| JSON root `status` | `ARCHITECTURE_DRAFT_NOT_IMPLEMENTED` |
| JSON SHA-256 | `272a6dabdb411701a25dd708577dbb498c7a7cd2754d83c612e85454a178a1d5` (29880 bytes) |
| CSV SHA-256 | `b5577302650e3d497465b93063d149313ec1197a38ddad3a9e7a6f223757ec32` (13138 bytes) |
| Registry SHA-256 | `7abed2af5363726403175c6524bd7a4d05d36b8735ad48442fc26941f8d98197` (44332 bytes) |
| Manifest SHA-256 | `82b205d27172ad234d46cfa2a59f0f4785128dd7a6282dea1aab32b412fc2808` (17945 bytes) |
| ARCH-DEC-35..48 SHA-256 | `5c16bc15b2a67fac16f476437d998de834ce823972f40f584647bf607e26b4f5` (48595 bytes) |
| Historical broker proposal index SHA-256 | `e808dcf8a2339845b4ef49085275b7aad9094862086830d57d13a415841703ce` (13593 bytes) |
| Broker resolution index SHA-256 | `e962623370a87fc7e93acc0c90b92728ccfae4678d4247fade0d5f2259e5a6a9` (14376 bytes) |

## Non-implementation

No file under `crates/*/src` or `apps/*/src` was edited. No route, adapter, migration, credential, runtime flag, order or risk state machine, broker call, lease, audit write, or financial rounding was added. `Cargo.toml` and `Cargo.lock` were not changed. Their hashes stay `5843ca50c51c0c316bf716c7e3412e4d1be1d9a3870d278c6ef0ef09cd186d3f` (1473 bytes) and `76fd3cefad9eef856ec2d784b1ae373d53ba25a5030a77be00a77d9b8adb80f0` (50294 bytes). No production dependency was added. `ORDER_STATE_MACHINE_IMPLEMENTED`, `STATE_TRANSITIONS_IMPLEMENTED`, `IDENTITY_RESOLVER_IMPLEMENTED`, `BROKER_DISPATCH_IMPLEMENTED`, `RISK_EVALUATION_IMPLEMENTED`, and `FINANCIAL_ROUNDING_IMPLEMENTED` stay false.

`TRD-ARCH-03B4-plan-index.json` is `NON_RUNTIME_NON_WIRE_VERIFICATION_PLAN_INDEX`. It is not `registry.json` and not a wire schema. It is allowlisted beside the other non-wire indexes so the existing contracts-JSON guard accepts it as documentation. It is not registered as a schema. Its safety flags include `implementation_authorized=false`, `testing_authorized=false`, `broker_connection_authorized=false`, `order_send_authorized=false`, `read_only_broker_access_authorized=false`, `paper_trading_authorized=false`, `uniqueness_guaranteed=false`, `completeness_proof_approved=false`, `completeness_token_ratified=false`, `economic_posting_approved=false`, `risk_release_approved=false`, `economic_finality_predicate_approved=false`, `oms_transitions_approved=false`, `broker_dispatch_approved=false`, `live_trading_authorized=false`, `closes_xc_findings=false`, and `conflict_closure=false`. `tests_executed` is false. That flag means external broker and coverage tests. It does not describe the documentation regression tests in this commit. Those documentation tests ran, and they are not broker evidence. Stages E0 through E4 stay unauthorized. `evidence_uri` and `signature_artifact` are null.

## Changed files

| Path | SHA-256 | Bytes |
| --- | --- | --- |
| `contracts/verification-plans/README.md` | `1bf13a42107928ed7c1ed6b76a885c1c32d39909c26058f6d0693ada590ce8f6` | 1756 |
| `contracts/verification-plans/TRD-ARCH-03B4-ibkr-evidence-inventory.md` | `3d25e0e52282a6a35649431ad315ec672bb1f7c8580e70b47378557795f89fc9` | 52371 |
| `contracts/verification-plans/TRD-ARCH-03B4-ibkr-verification-protocol.md` | `70eea6d1ab6047c56baed77f5ee42631c7ed4da348a2c4387796334a1335530c` | 8773 |
| `contracts/verification-plans/TRD-ARCH-03B4-ibkr-test-matrix.md` | `d8dc4acc1ee3a2a987f25ac9d184d2e762ae0222202f358c4bc4483595166c45` | 49876 |
| `contracts/verification-plans/TRD-ARCH-03B4-ibkr-evidence-capture-and-provenance.md` | `9468b6f25949565c8f677155ed045df97dd317d8144da520e24251f6f33bce7b` | 6110 |
| `contracts/verification-plans/TRD-ARCH-03B4-dependency-and-conflict-matrix.md` | `2f3775d00fd3996503a86548799415ee2e9cb130d9ff0d789baeefaca0ed60e4` | 8083 |
| `contracts/verification-plans/TRD-ARCH-03B4-approval-requests.md` | `7e5046f52d84e31496acb9938ab046b11ba4baf671a8ecd0703561ead2beaa63` | 29485 |
| `contracts/verification-plans/TRD-ARCH-03B4-plan-index.json` | `d7e62abc81d541ad08725c754d634e0792ea7f0d60165dab34b5fedb364a15f4` | 23386 |
| `docs/cursor-prompts/trading/TRD-ARCH-03B4-IBKR-Evidence-and-Verification-Planning.md` | `fe8d8e2ee69b0e97a2df165ce32c84542f798021dede100a777857244d9559e4` | 16330 |
| `contracts/design-proposals/README.md` | `e352968b932cbfc2b0c7cbc3342b9077e751a50591acac3f83675ff4837aec25` | 9605 |
| `contracts/domain/glossary.md` | `8129337d06f57e3c1ce62b4402c139cdd1ce3937d10192b32bf1c48e8fb582ed` | 18825 |
| `contracts/domain/unresolved-decisions.md` | `0efb8f6bb236ac739c38eac7a1d7d62c93eeeb24604b017aa1dcf5dd573123b0` | 27854 |
| `contracts/traceability-index.md` | `5c81478cc547b6d4761f1115600be24640cb7320363529b042f1bd3afee1c02f` | 31532 |
| `docs/architecture/README.md` | `4ac93f1616d030c78ec083226583b827be1092b0f3232304608679bc7ecc5c93` | 15544 |
| `tests/README.md` | `c2d44de77ad7e419648b4c002de3b058a64c4c76083cd79918cf3d8b04e68bc6` | 1925 |
| `crates/trading-domain/tests/adfr_reconciliation.rs` | `74868fda58c3b68109573e1f7370b34e91e7e006a8ac8779c723e5523ab6c49b` | 185802 |
| `crates/trading-domain/tests/registry_skeleton.rs` | `88c79aa79cc410bdf6781cd8ca096654539b3216b555bb33af2f9ea510c1e3ca` | 33279 |

This report's own hash is not listed. No Cargo metadata change required a dependency justification.

## Local checks

Remote CI was not run. No broker test was run. No network request was sent. The ignored PostgreSQL test was not executed against a database. Ignored is not passed. No extra document-integrity script exists outside the Cargo tests. Those Cargo tests are the document checks.

`cargo fmt --all` was applied once to the new test sources before the check below. It did not need a second application. `cargo fmt --all -- --check` then exited 0.

The four commands below were run from the repository root on the tree that includes this report. None failed.

| Command | Exit code |
| --- | --- |
| `cargo fmt --all -- --check` | 0 |
| `cargo check --workspace --locked` | 0 |
| `cargo test --workspace --locked` | 0 |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | 0 |

`cargo test` result: 137 passed, 0 failed, 1 ignored. The ignored test is `migration_is_explicit_and_readiness_is_read_only` in `crates/trading-persistence/tests/connectivity.rs`, because `TRADING_ALLOW_DATABASE_INTEGRATION_TESTS=1` and `TRADING_TEST_DATABASE_URL` were not set. It was not run against a database. `adfr_reconciliation` passed 17 tests, including `ibkr_evidence_verification_plan_stays_unexecuted` and `broker_design_policy_35_through_48_stays_unimplemented`. `registry_skeleton_stays_unpublished` passed.

These documentation tests do not prove broker behavior, reconciliation, crash freedom, or economic safety. A passing assertion in `ibkr_evidence_verification_plan_stays_unexecuted` is not IBKR evidence.

## Deviations

`contracts/verification-plans/` is new. The repository had no verification-plan directory. The plan index is allowlisted in `crates/trading-domain/tests/registry_skeleton.rs` as non-wire JSON. It is not an entry in `contracts/registry/registry.json`.

Pointer paragraphs were added to the design-proposals README, the glossary, unresolved decisions, the traceability index, the architecture README, and `tests/README.md`. Historical proposal text was not rewritten.

The prompt archive has one identifying sentence before the executed instruction, in the same pattern as the TRD-ARCH-03B.3.2 archive. That sentence does not add a decision.

Two early runs of the new documentation test failed because the capture note lacked `EXTERNAL_EVIDENCE_PENDING` and two recommendation sentences lacked the phrase "This recommendation is not an approval". Those sentences were corrected in this same commit before the recorded passing run. No broker test was involved.

`cargo fmt --all` was run once before `--check`. The check exited 0. No unrelated files were reformatted.

No evidence payload was created. No URL was invented. `contracts/source-manifest.json`, `contracts/registry/registry.json`, the 12 package source files, the ADFR JSON and CSV, and ARCH-DEC-01 through ARCH-DEC-48 were not edited.

## Gates

| Gate | Result |
| --- | --- |
| A03B4-G01 | PASS. Clean baseline. Predecessor commit `c20ce5fb2686bd4038342819f364a04ba56b0e4d` includes the 03B.3.2 report, the archived prompt, and ARCH-DEC-35..48. No conflicting identifier was introduced. |
| A03B4-G02 | PASS. 12/12 manifest sources matched before edits and match again after edits. Provenance files were not modified. |
| A03B4-G03 | PASS. 78 ADFR rows, JSON/CSV parity, 78 `NOT_VERIFIED`, one supersession `ADFR-CROSS-005` to `ADFR-CR-B10-001`. Registry hash and safety flags unchanged. |
| A03B4-G04 | PASS. ARCH-DEC-01..48 files are byte-identical. CR-B01 precedence stays in force. No decision was ratified and no ADFR verification cell was promoted. |
| A03B4-G05 | PASS. 27 claims cover `orderId`, `permId`, `execId`, status versus execution versus attempt, and the related ambiguities. Uniqueness stays unclaimed. |
| A03B4-G06 | PASS. Capture completeness and broker completeness are separate. No completeness token is ratified. An empty response is not a negative proof. |
| A03B4-G07 | PASS. Unknown-send, crash, fencing, and failover rows stay `NOT_EXECUTED` and authorize no retry or takeover. |
| A03B4-G08 | PASS. 43 scenario rows, each with an oracle limit, evidence status `EXTERNAL_EVIDENCE_PENDING`, and `NOT_EXECUTED`. |
| A03B4-G09 | PASS. 14 `IBKREVQ` questions, each `USER_DECISION_PENDING` and `NOT_RATIFIED`, with A/B/C and option C marked `CONFLICTS_WITH_BINDING_POLICY`. |
| A03B4-G10 | PASS. No production source, schema, migration, dependency, credential, broker, network, booking, release, OMS, or live-trading change. Safety flags stay false. |
| A03B4-G11 | PASS. The four local Cargo commands exited 0. 137 passed, 0 failed, 1 ignored. The ignored PostgreSQL test was not run. Remote CI was not run. No broker test was run. |
| A03B4-G12 | PASS when this report and the prompt archive are in the one commit named in the Cursor output. Subject: `TRD-ARCH-03B.4: Plan IBKR Evidence and Verification`. Nothing was pushed. |

## Next step

Not started. IBKREVQ-01 through IBKREVQ-14 are not ratified. TRD-ARCH-03B.4.2 is not started. No IBKR integration is started. Stages E0 through E4 remain STOP. External broker identity and snapshot evidence remain `NOT_EXECUTED`. The economic-finality predicate remains unresolved. The OMS transition matrix, an `OrderClosure` enum, economic posting, risk release, broker dispatch, deployment, and live trading are not authorized and are not implemented. `XC-01`, `XC-02-PRED`, `XC-08-IBKR`, and the operational remainder of `XC-04` stay OPEN.
