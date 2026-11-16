# TRD-ARCH-03B.5 report

**Date:** 2026-11-16  
**Mode:** documentation boundary, provenance register, and documentation-only regression tests. No trading behavior was added. No official IBKR page was retrieved.  
**Live trading:** NOT AUTHORIZED  
**Canonical schemas:** not published  
**Broker connection:** not authorized  
**Read-only broker access:** not authorized  
**External broker tests:** `external_broker_tests_executed=false`  
**External evidence:** `EXTERNAL_EVIDENCE_PENDING`  
**External cases:** `NOT_EXECUTED`  
**Stages E0 through E4:** STOP  
**External fetch:** `NOT_PERFORMED`  
**Economic posting:** not approved  
**Reservation release:** forbidden  
**Economic-finality predicate:** not approved  
**OMS transitions:** none approved  
**Broker dispatch:** not approved  
**Remote CI:** `NOT_RUN`

`VERDICT: PARTIAL`

`PUSHED: NO`. The commit subject is `TRD-ARCH-03B.5: Research Official IBKR Documentation`. The commit SHA is the commit that adds this report. It is named in the Cursor output because this file cannot contain the hash of the commit that introduces it. Branch: `main`. `BASELINE_HEAD`: `ac60bf932355660548bf5751963009ea956b76a5`. `FINAL_HEAD` is that same Cursor output. The worktree was clean at baseline. The single commit is the only commit of this milestone.

The verdict is `PARTIAL` because official texts were not obtained. Gates G04 and G05 are met only as a negative record: nothing is marked obtained, and no second copy is pretended. They are not a completed acquisition.

## Baseline

| Item | Value |
| --- | --- |
| Branch | `main` |
| Worktree before edits | clean. `git status --short` printed nothing. |
| BASELINE_HEAD | `ac60bf932355660548bf5751963009ea956b76a5` |
| HEAD subject | `TRD-ARCH-03B.4.2: Ratify IBKR Verification Strategy` |
| Parent | `4bbfd6748103b806619a4c0ac762a12c427595d1` |
| Predecessor report | `docs/architecture/TRD-ARCH-03B.4.2-report.md` present in that commit |
| Predecessor prompt | `docs/cursor-prompts/trading/TRD-ARCH-03B4.2-Ratify-IBKR-Verification-Strategy.md` present in that commit |
| Destructive git | none. Nothing was discarded, reset, or stashed. |

IBKREVQ-01 through IBKREVQ-14 remain B, A, A, B, B, B, A, A, B, A, A, A, A, B. Count: 8 times A and 6 times B. ARCH-DEC-49 through ARCH-DEC-62 stay `RATIFIED_DESIGN_POLICY_ONLY`, `NOT_IMPLEMENTED`, and `NOT_VERIFIED`. No new decision identifier was assigned. `ARCH-DEC-63` stays unused.

## Authority and stop

The authority is the user instruction dated 2026-11-16, archived at `docs/cursor-prompts/trading/TRD-ARCH-03B5-IBKR-Official-Documentation-Research.md`. No signature artifact was supplied, and none is invented. No evidence URI is recorded.

That instruction authorizes unauthenticated public official documentation research, including retrieval of a public primary text and a second copy of the same version where possible. It says this permission is not an E0 stage go and that checkpoints E0 through E4 stay STOP. It also says to stop before any external retrieval if the existing protocol requires a different formal approval for every E0 action.

The protocol says a later go is a separate dated user authorization that names the stage. E0's work is collecting official material that was actually obtained, and its checkpoint is STOP with authorization `NOT_AUTHORIZED`. The capture note says an official copy is taken only after that checkpoint moves to go. ARCH-DEC-49 says its policy does not start E0.

Collecting official text is an E0 action under that protocol. The instruction does not name a stage go. The instruction's stop rule was applied. No host was called. No redirect was followed. No login, account, session, TWS, IB Gateway, or broker API endpoint was used.

`public_documentation_research_authorized=true` records the narrow permission. `public_documentation_research_authorized_is_stage_go=false`. `stage_e0_authorized` stays false.

Concrete documentation hostnames are `UNRECORDED`. Naming a host from memory would invent a URL. The allowed class, unused, is an unauthenticated fetch of a public documentation page whose final host stays a documentation host. A redirect toward login, account, session, or an API host would not be followed. If that class could not be kept, the status would be `NOT_RETRIEVED`.

## Sources

Obtained sources: 0. Second copies: 0. Retrieval status: `NOT_RETRIEVED`. `fetch_timestamp_utc` is null. No SHA-256, byte count, URL, version, content type, or redirect chain is recorded for an official text, because none was observed.

`OFFICIAL_DOCUMENTATION_OBTAINED` is not assigned. `SECOND_COPY_PENDING` is not assigned, because that label would imply a primary copy. `DOCUMENTED_CLAIM_NOT_EMPIRICALLY_VERIFIED` is not assigned, because no official sentence was obtained. Every research topic is `UNVERIFIED_RESEARCH_TOPIC`. A second download would not have been empirical broker confirmation. No download occurred.

No local source archive was created. No full text was copied into Git. No raw payload store was created. No secret, cookie, token, account datum, or personal datum was added.

TWS API, IB Gateway, and Client Portal API stay separate unanswered questions. They were not collapsed into one product and none was retrieved.

## Claims and scenarios

All 27 claims from `IBKR-CLM-01` through `IBKR-CLM-27` are mapped once. Each official source and section anchor is `NOT_ESTABLISHED`. Each documentation class is `UNVERIFIED_RESEARCH_TOPIC`. Hypothesis status stays `HYPOTHESIS`. Evidence stays `EXTERNAL_EVIDENCE_PENDING`. Execution stays `NOT_EXECUTED`. No empirical upgrade.

All 43 scenarios from `IBKR-SCN-A01` through `IBKR-SCN-D06` are mapped once. Each stays `NOT_EXECUTED` and `EXTERNAL_EVIDENCE_PENDING`. Oracle limits say what a later official sentence would still not prove. Unknown send, snapshot gaps, corrections and busts, reconnect, and writer isolation are called out. Silence is not a negative finding. No documentation source closes `XC-01`, `XC-08-IBKR`, `XC-02-PRED`, `XC-03-MATRIX`, or the operational remainder of `XC-04`.

## Reads, version, and environment

Read candidates: 0. `NO_READS_APPROVED`. No operation name was invented. E2 stays STOP. ARCH-DEC-53 is unchanged. Cancel, modify, and replace are not reads.

Every version-tuple field is `UNRECORDED`. No environment class was selected. ARCH-DEC-52 stays in force. CR-B04 §4.5 names a paper or sandbox setting as historical internal text. That sentence is not an official page retrieved here and is not a selection.

## Open questions, not decided

1. A dated authorization that names stage E0 and moves that checkpoint to go, or an explicit statement that public retrieval may proceed while the checkpoint stays STOP.
2. A concrete host allowlist, which this milestone does not invent.
3. Later recognition of named copies, in a different instruction and a different commit, after two version-equivalent copies exist.
4. Environment class, after official words exist.
5. Exact read operations, after official names exist. E2 stays STOP until then.
6. Offline fixtures. E1 stays STOP.
7. Raw-payload location, access, and retention. No store was created.
8. Any empirical broker package. Not started.

None of these is ratified by this report. No next phase was started.

## Unchanged conflicts and protection hashes

`XC-01` stays OPEN and `EVIDENCE_PENDING`. `XC-08-IBKR` stays OPEN and `EXTERNAL_EVIDENCE_PENDING`. `XC-02-PRED` stays OPEN. `XC-03-MATRIX` stays OPEN. The XC-04 operational remainder stays OPEN. XC-04 design precedence stays `RESOLVED_BY_ARCH_DEC_03`. ADFR stays 78 entries, every `verification` value `NOT_VERIFIED`, with one supersession `ADFR-CROSS-005` to `ADFR-CR-B10-001`. Registry stays `SKELETON_NOT_CANONICAL`, `part_b=NO-GO`, `live_trading_authorized=false`, `broker_uniqueness_evidence=EVIDENCE_PENDING`.

Preflight hashes matched the repository after the edits. Before and after are the same bytes.

| Path | SHA-256 | Bytes |
| --- | --- | --- |
| `contracts/registry/registry.json` | `7abed2af5363726403175c6524bd7a4d05d36b8735ad48442fc26941f8d98197` | 44332 |
| `contracts/source-manifest.json` | `82b205d27172ad234d46cfa2a59f0f4785128dd7a6282dea1aab32b412fc2808` | 17945 |
| `contracts/sources/ADFR-Consolidated-v1.0.json` | `272a6dabdb411701a25dd708577dbb498c7a7cd2754d83c612e85454a178a1d5` | 29880 |
| `contracts/sources/ADFR-Consolidated-v1.0.csv` | `b5577302650e3d497465b93063d149313ec1197a38ddad3a9e7a6f223757ec32` | 13138 |
| `contracts/decisions/ARCH-DEC-01-bis-03-2026-11-15.md` | `9f946806667a3700cb45d5033e509355aeefc60d2216262647f2dbbae034e30a` | 9761 |
| `contracts/decisions/ARCH-DEC-04-bis-08-2026-11-15.md` | `af2cd4b5c35912fe97d3a8ea36c459259aabf0a7b7d54657fdb7b7bce6149ddc` | 13679 |
| `contracts/decisions/ARCH-DEC-09-bis-20-2026-11-15.md` | `ddaffca4b9d1d69c430a3241db14460e49adf1e8d54a33ac9f0f07b5b957a7a2` | 24820 |
| `contracts/decisions/ARCH-DEC-21-bis-34-2026-11-15.md` | `a3afc65a0e1a965ac85a9015acc60da858be0ee57ffe104098f2cf56407e5b07` | 31894 |
| `contracts/decisions/ARCH-DEC-35-bis-48-2026-11-15.md` | `5c16bc15b2a67fac16f476437d998de834ce823972f40f584647bf607e26b4f5` | 48595 |
| `contracts/decisions/ARCH-DEC-49-bis-62-2026-11-15.md` | `5400a7d65d30a144f5369e8683533087aa63d3d06775f0b566f0cb3f61904648` | 51486 |

The twelve manifest sources matched `sha256` and `bytes` on each entry. Those hashes live on the manifest and are not repeated here as if they were a new retrieval.

## Cargo

Commands were run locally on this worktree. Remote CI was not run.

| Command | Exit code |
| --- | --- |
| `cargo fmt --all -- --check` | 0 |
| `cargo check --workspace --locked` | 0 |
| `cargo test --workspace --locked` | 0 |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | 0 |

`cargo test --workspace --locked` reported 140 passed, 0 failed, and 1 ignored. The ignored test is `trading-persistence` `connectivity::migration_is_explicit_and_readiness_is_read_only`. It did not run. It asks for `TRADING_ALLOW_DATABASE_INTEGRATION_TESTS=1` and `TRADING_TEST_DATABASE_URL`. That is a local database integration test, not a broker test, and it is not executed. Measured toolchain: rustc 1.99.1 (58cf8f9c3 2026-10-13), cargo 1.99.1 (5f94df478 2026-08-27), git 2.56.0.windows.3.

A documentation test that passes is not an external IBKR test. Broker tests were not run.

## Gates

| Gate | Result | Reason |
| --- | --- | --- |
| A03B5-G01 | PASS | Clean `main` at `ac60bf932355660548bf5751963009ea956b76a5`. The protocol conflict is recorded and was not silently resolved. |
| A03B5-G02 | PASS | Twelve sources, registry, ADFR, and ARCH-DEC-01 through ARCH-DEC-62 are byte-identical. Hashes are above. |
| A03B5-G03 | PASS | Public research is recorded as not a stage go. Checkpoints stay STOP. No API access. |
| A03B5-G04 | PARTIAL | Zero sources are marked obtained, and no URL or version was invented. The acquisition itself did not happen. |
| A03B5-G05 | PARTIAL | No primary copy exists, so no second copy is claimed. ARCH-DEC-49's pair is absent and is not labeled as a pending pair of an obtained text. |
| A03B5-G06 | PASS | All 27 claims have an explicit gap. None was upgraded empirically. |
| A03B5-G07 | PASS | All 43 scenarios keep oracle limits and stay `NOT_EXECUTED`. |
| A03B5-G08 | PASS | The candidate list is empty. `NO_READS_APPROVED`. |
| A03B5-G09 | PASS | Tuple fields stay `UNRECORDED`. No environment was selected. The three API families stay separate questions. |
| A03B5-G10 | PASS | No runtime, schema, broker, order, risk, or OMS change. Safety flags stay fail-closed. |
| A03B5-G11 | PASS | The four local cargo commands exited 0. The one ignored database test was not executed. Remote CI was `NOT_RUN`. |
| A03B5-G12 | PASS | This report and the prompt archive are in the single local commit. No push. |

## Files changed

New research notes and the non-wire index under `contracts/verification-plans/TRD-ARCH-03B5-*`. Append-only sections in `contracts/verification-plans/README.md`, `contracts/traceability-index.md`, `contracts/domain/unresolved-decisions.md`, and `docs/architecture/README.md`. Documentation tests in `crates/trading-domain/tests/ibkr_official_documentation_research.rs` and the JSON allowlist in `crates/trading-domain/tests/registry_skeleton.rs`. The prompt archive and this report. No production `src` change. No registry change. No manifest change. No decision-file change.

**NO IMPLEMENTATION.** **NO VERIFICATION.**
