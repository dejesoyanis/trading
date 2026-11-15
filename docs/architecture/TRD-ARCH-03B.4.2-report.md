# TRD-ARCH-03B.4.2 report

**Date:** 2026-11-15  
**Mode:** dated documentation, traceability, and documentation-only regression tests. No trading behavior was added.  
**Live trading:** NOT AUTHORIZED  
**Canonical schemas:** not published  
**Broker connection:** not authorized  
**External broker tests:** `external_broker_tests_executed=false`  
**External evidence:** `EXTERNAL_EVIDENCE_PENDING`  
**External cases:** `NOT_EXECUTED`  
**Stages E0 through E4:** STOP  
**Economic posting:** not approved  
**Reservation release:** forbidden  
**Economic-finality predicate:** not approved  
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
| BASELINE_HEAD | `4bbfd6748103b806619a4c0ac762a12c427595d1` |
| HEAD subject | `TRD-ARCH-03B.4: Plan IBKR Evidence and Verification` |
| Predecessor report | `docs/architecture/TRD-ARCH-03B.4-report.md` present in that commit |
| Predecessor prompt | `docs/cursor-prompts/trading/TRD-ARCH-03B4-IBKR-Evidence-and-Verification-Planning.md` present in that commit |
| rustc | 1.99.1 (58cf8f9c3 2026-10-13) |
| cargo | 1.99.1 (5f94df478 2026-08-27) |
| git | 2.56.0.windows.3 |
| Destructive git | none. Nothing was discarded, reset, or stashed. |

ARCH-DEC-49 through ARCH-DEC-62 were unused as decision identifiers before this task. No decision file, resolution index, or registry entry assigned them. The only mentions were forward guards asserting that no decision filename contained `ARCH-DEC-49`. Those guards now name `ARCH-DEC-63`. No identifier was renumbered. No existing decision file was overwritten.

Registry flags at baseline were `SKELETON_NOT_CANONICAL`, `canonical_schemas_published=false`, `part_b=NO-GO`, `live_trading_authorized=false`, `broker_uniqueness_evidence=EVIDENCE_PENDING`. Registry SHA-256 `7abed2af5363726403175c6524bd7a4d05d36b8735ad48442fc26941f8d98197` (44332 bytes) matches the previously reported value and is unchanged.

The decision filename `contracts/decisions/ARCH-DEC-49-bis-62-2026-11-15.md` follows the existing `ARCH-DEC-35-bis-48-2026-11-15.md` convention.

## Authority

The authority is the user ratification dated 2026-11-15 in the TRD-ARCH-03B.4.2 instruction, archived at `docs/cursor-prompts/trading/TRD-ARCH-03B4.2-Ratify-IBKR-Verification-Strategy.md`. No signature artifact was supplied, and none is invented. No evidence URI is recorded. No source transcript is invented. No independent approval is invented. Recommendations in the historical IBKREVQ text are not this ratification. The user choice matched the non-binding recommendation letter on every question. The recommendation was still not the approval. Option C stays `CONFLICTS_WITH_BINDING_POLICY` for every question.

Record labels `RATIFIED_DESIGN_POLICY_ONLY`, `NOT_IMPLEMENTED`, `NOT_VERIFIED`, and `NON_RUNTIME_NON_WIRE` describe the new decision file and the resolution index. They are not a canonical status enum and they do not change an ADFR `verification` cell.

The archived prompt is the executed instruction. One sentence at the top of that file identifies it as the instruction and states that no signature or evidence URI is attached. It does not add a decision.

## Decision mapping

Each row is `RATIFIED_DESIGN_POLICY_ONLY` / `NOT_IMPLEMENTED` / `NOT_VERIFIED`. The source anchor is the historical heading in `contracts/verification-plans/TRD-ARCH-03B4-approval-requests.md`. The decision record is `contracts/decisions/ARCH-DEC-49-bis-62-2026-11-15.md`.

| Decision | Question | Option |
| --- | --- | --- |
| ARCH-DEC-49 | IBKREVQ-01 | B |
| ARCH-DEC-50 | IBKREVQ-02 | A |
| ARCH-DEC-51 | IBKREVQ-03 | A |
| ARCH-DEC-52 | IBKREVQ-04 | B |
| ARCH-DEC-53 | IBKREVQ-05 | B |
| ARCH-DEC-54 | IBKREVQ-06 | B |
| ARCH-DEC-55 | IBKREVQ-07 | A |
| ARCH-DEC-56 | IBKREVQ-08 | A |
| ARCH-DEC-57 | IBKREVQ-09 | B |
| ARCH-DEC-58 | IBKREVQ-10 | A |
| ARCH-DEC-59 | IBKREVQ-11 | A |
| ARCH-DEC-60 | IBKREVQ-12 | A |
| ARCH-DEC-61 | IBKREVQ-13 | A |
| ARCH-DEC-62 | IBKREVQ-14 | B |

Options in order: B, A, A, B, B, B, A, A, B, A, A, A, A, B. Count: 8 times A and 6 times B.

IBKREVQ-01 B requires an obtained versioned primary text and a second obtained copy of the same version before a later E4 citation. The second copy is not an empirical broker confirmation. No URL or version is invented. IBKREVQ-02 A pins one product, API client, platform, OS, and account-mode tuple per future package. IBKREVQ-03 A records only fields the request or payload supplies and claims no global snapshot. IBKREVQ-04 B leaves the environment class open and approves no account. IBKREVQ-05 B forbids a connection and a read until a later decision names official read operations. This record names no operation and does not authorize E2. Cancel, modify, and replace are not reads. IBKREVQ-06 B requires a future oracle note to say that capture completeness is not broker completeness. Coverage stays `UNKNOWN`. No completeness token is stored. IBKREVQ-07 A allows a later reuse observation of `orderId`, `permId`, and `execId` inside one pinned tuple. That observation is not a uniqueness guarantee. No test is authorized today. A singleton candidate stays `SINGLE_CANDIDATE_NOT_AUTHORIZATION`. IBKREVQ-08 A adopts a correction or bust reference only when the new raw payload itself contains it. IBKREVQ-09 B keeps unknown-send crash and disconnect cases synthetic or offline. No fault injection, retry, resend, takeover, or E3 authorization. IBKREVQ-10 A is an isolation checklist. It is not takeover proof and not an operational approval path. CR-B01 stays ahead. A local fence is not broker acceptance. IBKREVQ-11 A keeps future raw payloads outside Git. No store and no payload are created. No secret is added to Git. IBKREVQ-12 A does not carry earlier external claims onto a changed tuple. No calendar period is invented. IBKREVQ-13 A stops the later stage. The marker is not a role. A missing required second approval is deny. No stage starts. IBKREVQ-14 B separates future capture and future ratification into different instructions and different commits. A review note is not a decision. Accepting named evidence later would still not authorize implementation or live trading.

`documentation_tests_passed` for repository rules is a different fact from `external_broker_tests_executed=false`. A green Cargo test does not change `NOT_EXECUTED` or `EXTERNAL_EVIDENCE_PENDING`.

## Open XC map

| Finding | Status after this record |
| --- | --- |
| XC-01 | OPEN. `EVIDENCE_PENDING`. No global identity uniqueness. |
| XC-02-PRED | OPEN. No predicate was adopted. |
| XC-03-MATRIX | OPEN. No OMS edge. |
| XC-04 | Design precedence remains `RESOLVED_BY_ARCH_DEC_03`. The operational remainder stays OPEN. CR-B01 precedence stays in force. A local epoch or lease is not external fencing. |
| XC-05-LOCK | OPEN. No lock order. |
| XC-06 | OPEN. No table name. |
| XC-07-ROUND | OPEN. No rounding mode. |
| XC-08-IBKR | OPEN. `EXTERNAL_EVIDENCE_PENDING`. No snapshot completeness. |
| XC-09-RESET | OPEN. No reset right. |
| XC-10-ACTION | OPEN. No broker action. |
| XC-11 | Status words stay OPEN. Population and supersession stay `RESOLVED_BY_ARCH_DEC_02`. |

The 27 IBKR-CLM rows stay `HYPOTHESIS`, `NOT_EXECUTED`, and `EXTERNAL_EVIDENCE_PENDING`. The 43 IBKR-SCN rows stay `NOT_EXECUTED`, `EXTERNAL_EVIDENCE_PENDING`, and `NOT_AUTHORIZED`. Where a historical question names no claim or scenario, the resolution index says `NOT_ESTABLISHED`. The decision list is not a verification result. Stages E0, E1, E2, E3, and E4 stay STOP.

ARCH-DEC-03, ARCH-DEC-04, ARCH-DEC-07, ARCH-DEC-13, ARCH-DEC-14, ARCH-DEC-17, ARCH-DEC-26, ARCH-DEC-33, ARCH-DEC-34, and ARCH-DEC-35 through ARCH-DEC-48 stay binding. This record does not supersede them. A single broker assignment does not post. No terminal status by itself is a finality proof or an approval.

## Prohibitions

No broker login, TWS or IB Gateway connection, account access, API read, broker test, fault injection, order send, paper order, retry, resend, or takeover is approved. No economic posting, correcting or reversal booking, reservation conversion, reduction, or release, economic-finality predicate, OMS transition, canonical schema, DDL, migration, identity resolver, UNIQUE index, or completeness token is approved. No live trading is approved. No raw payload and no secret were added to Git. Documentation tests do not prove broker behavior. The ignored PostgreSQL test was not run against a database. Ignored is not passed. **NO IMPLEMENTATION.** **NO VERIFICATION.**

## Source and ADFR integrity

Before any edit, all 12 files in `contracts/source-manifest.json` matched that manifest's current `sha256` and `bytes`. Mismatches: 0. The same check is asserted again in `ibkr_verification_strategy_49_through_62_stays_unimplemented`. `package_sha256` and `package_bytes` remain the TRD-ARCH-SYNC-01 intake values. DET-01 and DET-02 stay `reconstructed_draft` true, `SOURCE_REVIEW_PENDING`, `reviewed` false, and `approved` false.

`contracts/source-manifest.json` was not edited. The 12 manifest source files were not edited. `contracts/registry/registry.json` was not edited. The ADFR JSON and CSV were not edited. ARCH-DEC-01 through ARCH-DEC-48 were not edited. The historical plan index and the five other TRD-ARCH-03B.4 plan notes, other than the append-only approval addendum, were not edited. The first 29485 bytes of `TRD-ARCH-03B4-approval-requests.md` still hash to `7e5046f52d84e31496acb9938ab046b11ba4baf671a8ecd0703561ead2beaa63`. That is the pre-edit hash of the whole file. The ratification addendum begins after those bytes.

| Item | Value, unchanged from baseline |
| --- | --- |
| JSON count | 78 |
| CSV count | 78, same IDs and order |
| `verification` | 78 `NOT_VERIFIED` |
| Superseded | 1: `ADFR-CROSS-005`, `superseded_by` `ADFR-CR-B10-001` |
| Not superseded | 77 |
| JSON SHA-256 | `272a6dabdb411701a25dd708577dbb498c7a7cd2754d83c612e85454a178a1d5` (29880 bytes) |
| CSV SHA-256 | `b5577302650e3d497465b93063d149313ec1197a38ddad3a9e7a6f223757ec32` (13138 bytes) |
| Registry SHA-256 | `7abed2af5363726403175c6524bd7a4d05d36b8735ad48442fc26941f8d98197` (44332 bytes) |
| Manifest SHA-256 | `82b205d27172ad234d46cfa2a59f0f4785128dd7a6282dea1aab32b412fc2808` (17945 bytes) |

## Changed files

| Path | SHA-256 | Bytes |
| --- | --- | --- |
| `contracts/decisions/ARCH-DEC-49-bis-62-2026-11-15.md` | `5400a7d65d30a144f5369e8683533087aa63d3d06775f0b566f0cb3f61904648` | 51486 |
| `contracts/verification-plans/TRD-ARCH-03B42-resolution-index.json` | `b240fcde6fa22bc66932471ab4bda4284c7526abc8eff0013bf7d1d9086294c8` | 21069 |
| `contracts/verification-plans/TRD-ARCH-03B4-approval-requests.md` | `072b8cb5e4dc38657f5b4d221b93966a3413ee67c4f84dc27d9df79efd7033a3` | 36172 |
| `contracts/verification-plans/README.md` | `934aaaf779c18d0f586d712f1c1e42d7ba7daa51ac6612c9f3fb6e6c2e824a19` | 3113 |
| `contracts/traceability-index.md` | `7de2a2368afb5e31c8649da051530cb05f5c595f4d1b314aa88cc8e38c72d5f1` | 34913 |
| `contracts/domain/unresolved-decisions.md` | `18f8cff213e8c0a4a026569c27708d0e49e1044cf8d7f4109cf41623c216fc4b` | 30299 |
| `contracts/domain/glossary.md` | `1be2b573b735215730afe87b96b93486f4d674a3a2f06970c658f572770ea3b9` | 19527 |
| `contracts/design-proposals/README.md` | `36df7791cbd4f70b78f65f44e059929c351c39e2bda4c83ffb7206613b916ddb` | 10091 |
| `docs/architecture/README.md` | `c6f845fc0da355d243a50ff0e0cb8ef1416c2060448046506009f68ba514ff2d` | 16306 |
| `docs/cursor-prompts/trading/TRD-ARCH-03B4.2-Ratify-IBKR-Verification-Strategy.md` | `10b322209c90f8266500487af3623a26ae160f3056708b6dcf1d48ffccb8dacd` | 19898 |
| `tests/README.md` | `3885d9242030db360006c51d4d1ad90b4afa3141d4f9b4f9920d9a13eab5ef5b` | 2138 |
| `crates/trading-domain/tests/adfr_reconciliation.rs` | `5b6749b9f0716f315b18842856abe354237186a177993a65f6d65e908ced473f` | 185802 |
| `crates/trading-domain/tests/registry_skeleton.rs` | `e5fdf2cb041d17a0cffacfd8339ce53fc9d202b7e50fec4eae9003fab12c195c` | 38192 |
| `crates/trading-domain/tests/ibkr_verification_strategy.rs` | `f832cf1f23851e34bf712d2b44dc8f242675610765a4d19c39410d2d30402399` | 32104 |

This report's own hash is not listed. The approval-request hash above is the file after the addendum. Its first 29485 bytes remain `7e5046f52d84e31496acb9938ab046b11ba4baf671a8ecd0703561ead2beaa63`. No Cargo metadata change required a dependency justification.

No file under `crates/*/src` or `apps/*/src` was edited. `Cargo.toml` and `Cargo.lock` were not edited.

## Local checks

Remote CI was not run. No broker test was run. No network request was sent. The ignored PostgreSQL test was not executed against a database. Ignored is not passed. `documentation_tests_passed` below describes repository documentation rules only. `external_broker_tests_executed=false`.

The four commands below were run from the repository root on the tree that includes this report.

| Command | Exit code |
| --- | --- |
| `cargo fmt --all -- --check` | 0 |
| `cargo check --workspace --locked` | 0 |
| `cargo test --workspace --locked` | 0 |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | 0 |

`cargo test` result: 138 passed, 0 failed, 1 ignored. The ignored test is `migration_is_explicit_and_readiness_is_read_only` in `crates/trading-persistence/tests/connectivity.rs`, because `TRADING_ALLOW_DATABASE_INTEGRATION_TESTS=1` and `TRADING_TEST_DATABASE_URL` were not set. It was not run against a database. Ignored is not passed. `adfr_reconciliation` passed 17 tests. `ibkr_verification_strategy_49_through_62_stays_unimplemented` passed. `registry_skeleton_stays_unpublished` passed.

These documentation tests do not prove broker behavior. A passing assertion in `ibkr_verification_strategy_49_through_62_stays_unimplemented` is not IBKR evidence.

## Deviations

The forward guard that named `ARCH-DEC-49` as the next unused identifier now names `ARCH-DEC-63`. The historical decision files were not edited. The historical IBKREVQ questions were not edited. An append-only resolution addendum was added after byte 29485 of the approval-request file.

`TRD-ARCH-03B42-resolution-index.json` is allowlisted in `crates/trading-domain/tests/registry_skeleton.rs` as non-wire JSON. It is not an entry in `contracts/registry/registry.json`.

The user instruction marked emphasis with markdown bold inside the binding sentences. The decision record stores those sentences without the emphasis markers. The archived prompt keeps the emphasis. No choice was added or removed by that rendering.

The first run of `ibkr_verification_strategy_49_through_62_stays_unimplemented` failed because the assertion looked for the sentence "The second copy is not an independent empirical confirmation of broker behavior." The decision file already said "It is not an independent empirical confirmation of broker behavior." The assertion was changed to that sentence. No decision text was weakened to satisfy the test. No broker test was involved.

`cargo fmt --all` was run once before the recorded checks. The check is the command in the table below.

No evidence payload was created. No URL was invented. No broker method, version, account, or test result was invented.

## Gates

| Gate | Result |
| --- | --- |
| A03B42-G01 | PASS. Clean baseline. Predecessor commit `4bbfd6748103b806619a4c0ac762a12c427595d1` subject `TRD-ARCH-03B.4: Plan IBKR Evidence and Verification`. Identifiers 49 through 62 were free. No foreign change was present. |
| A03B42-G02 | PASS. Fourteen user choices, 8 times A and 6 times B, dated 2026-11-15. No signature artifact and no evidence URI were invented. |
| A03B42-G03 | PASS. Historical IBKREVQ text is the unchanged 29485-byte prefix. The resolution index is separate. |
| A03B42-G04 | PASS. 12/12 manifest sources match. Registry hash and flags match. 78 ADFR rows stay `NOT_VERIFIED` with one supersession. ARCH-DEC-01 through ARCH-DEC-48 are byte-identical. |
| A03B42-G05 | PASS. ARCH-DEC-49 through ARCH-DEC-51 record source quality, one pinned tuple, and supplied scope only. No evidence claim is made. |
| A03B42-G06 | PASS. ARCH-DEC-52 through ARCH-DEC-54 leave the environment open, forbid an unnamed read, and separate capture completeness from broker completeness. E0 through E4 stay STOP. |
| A03B42-G07 | PASS. ARCH-DEC-55 through ARCH-DEC-57 record reuse observations, payload-named correction links, and synthetic or offline unknown-send cases. None of those cases was executed. |
| A03B42-G08 | PASS. ARCH-DEC-58 through ARCH-DEC-61 are a checklist, an uncreated outside-Git store, a new-tuple rule, and a marker without a role. No new right is granted. |
| A03B42-G09 | PASS. ARCH-DEC-62 separates capture from ratification. `XC-01`, `XC-08-IBKR`, the XC-04 remainder, and `XC-02-PRED` stay OPEN. |
| A03B42-G10 | PASS. No runtime, schema, broker, economic, or live-trading change. Safety flags stay false. |
| A03B42-G11 | PASS. Documentation regressions and the four local Cargo commands on the final tree exited 0. 138 passed, 0 failed, 1 ignored. The ignored PostgreSQL test was not run. Remote CI was not run. |
| A03B42-G12 | PASS when this report and the prompt archive are in the one commit named in the Cursor output. Subject: `TRD-ARCH-03B.4.2: Ratify IBKR Verification Strategy`. Nothing was pushed. |

## Next step

Not started. No stage E0, E1, E2, E3, or E4 is authorized. No broker connection, read, order, paper session, or external test is authorized. No implementation and no live trading are authorized. Official primary texts have not been obtained. The environment class is unchosen. Read operations are unnamed. The raw-payload store does not exist. `XC-01`, `XC-08-IBKR`, `XC-02-PRED`, `XC-03-MATRIX`, and the operational remainder of `XC-04` stay OPEN. The next unused decision identifier is ARCH-DEC-63. A later instruction would be required before any of those steps, and this record does not grant it.
