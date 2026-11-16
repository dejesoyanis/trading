# TRD-ARCH-03B.5.1 report

**Date:** 2026-11-16  
**Mode:** unauthenticated public-documentation retrieval, local archival, and analysis. No trading behavior was added. No broker session was opened.  
**Live trading:** NOT AUTHORIZED  
**Canonical schemas:** not published  
**Broker connection:** not authorized  
**Read-only broker access:** not authorized  
**External broker tests:** `external_broker_tests_executed=false`  
**External evidence:** `EXTERNAL_EVIDENCE_PENDING`  
**External cases:** `NOT_EXECUTED`  
**Named E0 action:** `GO_LIMITED_PUBLIC_DOCS_ONLY`  
**E0 actions other than that named retrieval:** STOP  
**Stages E1 through E4:** STOP  
**Reads:** `NO_READS_APPROVED`  
**Environment:** not selected  
**Version tuple:** `UNRECORDED`  
**ARCH-DEC-49:** not satisfied  
**Economic posting:** not approved  
**Reservation release:** forbidden  
**Economic-finality predicate:** not approved  
**OMS transitions:** none approved  
**Broker dispatch:** not approved  
**Remote CI:** `NOT_RUN`

`VERDICT: PARTIAL`

`PUSHED: NO`. The commit subject is `TRD-ARCH-03B.5.1: Acquire Official IBKR E0 Documentation`. The commit SHA is the commit that adds this report. It is named in the Cursor output because this file cannot contain the hash of the commit that introduces it. Branch: `main`. `BASELINE_HEAD`: `f6686d46ba1b8b1fb804506d207f7878a59c218e`. `FINAL_HEAD` is that same Cursor output. The worktree was clean at baseline. The single commit is the only commit of this milestone.

The verdict is `PARTIAL` because real official pages were archived, and the independent second copy required before an ARCH-DEC-49 pair is still `SECOND_COPY_PENDING`. A green documentation test is not an IBKR verification.

## Baseline

| Item | Value |
| --- | --- |
| Branch | `main` |
| Worktree before edits | clean. `git status --porcelain` printed nothing. |
| BASELINE_HEAD | `f6686d46ba1b8b1fb804506d207f7878a59c218e` |
| HEAD subject | `TRD-ARCH-03B.5: Research Official IBKR Documentation` |
| Predecessor report | `docs/architecture/TRD-ARCH-03B.5-report.md`, verdict `PARTIAL`, obtained sources 0, external fetch `NOT_PERFORMED` |
| Predecessor prompt | `docs/cursor-prompts/trading/TRD-ARCH-03B5-IBKR-Official-Documentation-Research.md` |
| Toolchain | rustc 1.99.1 (58cf8f9c3 2026-10-13), cargo 1.99.1 (5f94df478 2026-08-27), git 2.56.0.windows.3 |
| Destructive git | none |

ARCH-DEC-49 through ARCH-DEC-62 stay `RATIFIED_DESIGN_POLICY_ONLY`, `NOT_IMPLEMENTED`, and `NOT_VERIFIED`. No new decision identifier was assigned. `ARCH-DEC-63` stays unused. The historical protocol file still records E0 as STOP and `NOT_AUTHORIZED`.

## Authorization

The user instruction dated 2026-11-16, archived at `docs/cursor-prompts/trading/TRD-ARCH-03B5.1-Acquire-Official-IBKR-E0-Documentation.md`, authorizes one E0 action: unauthenticated retrieval, verification, local archival, and analysis of public official IBKR documentation.

That action is recorded as `GO_LIMITED_PUBLIC_DOCS_ONLY` in `contracts/verification-plans/TRD-ARCH-03B5.1-limited-e0-authorization.md`. The 2026-11-15 protocol already says a later go is a separate dated authorization and that the protocol file is not itself the authorization. This record is that separate document. It takes narrow precedence over the previous E0 STOP only for allowlisted public documentation GET and HEAD. It does not rewrite the historical protocol. Unnamed E0 actions stay STOP. E1, E2, E3, and E4 stay STOP.

No broker login, account, session, TWS process, IB Gateway process, Client Portal session, order, cancel, modify, replace, retry, paper trade, live trade, fault injection, schema, migration, or raw broker payload was authorized or performed. A public documentation GET is not a broker API read.

## Allowlist

The repository contained no official documentation URL at preflight. Search snippets were not treated as permission to fetch. One identity probe of `https://www.interactivebrokers.com/docs/` was made with redirects disabled, cookies disabled, and TLS validation left on.

The certificate is DigiCert EV, subject `CN=interactivebrokers.com, O=IBG LLC, L=Greenwich, S=Connecticut, C=US`, thumbprint `1A69CDC7F710867C2694474B2F1F00440522C65A`, valid from 2026-07-30 to 2027-02-13. The SAN includes `www.interactivebrokers.com`. The response was HTTP 200, `text/html`, title `IBKR API Home | Interactive Brokers | Documentation`. That verifies the host. It does not verify every other name on the certificate.

Allowlisted documentation fetches:

| Id | Host | Path scope | Status |
| --- | --- | --- | --- |
| H-01 | `www.interactivebrokers.com` | `/docs/`, plus exact `/docs` and `/docs.md` | `VERIFIED_OFFICIAL_DOCUMENTATION_HOST` |
| H-02 | `ibkrcampus.com` | `/docs/`, plus exact `/docs` and `/docs.md` | `LINKED_FROM_VERIFIED_IBKR_DOCUMENTATION_PAGE` |

H-02 is linked from the verified documentation home. Its certificate is Let's Encrypt DV `CN=ibkrcampus.com` and does not name IBG LLC. The three requested documentation URLs on that host returned HTTP 403 with the 17-byte body `error code: 1000`. Those responses are failed retrievals, not official documents and not second copies.

`interactivebrokers.github.io` and `ibkrguides.com` stay `HOST_UNVERIFIED` and were not fetched. Rejected without a request: `ndcdyn.interactivebrokers.com` login and account-application links, `api.ibkr.com`, `localhost`, and third-party links on the documentation home. Two same-host 307 redirects, from `/docs/tws-api` to `/docs/tws-api/doc/introduction` and from `/docs/web-api` to `/docs/web-api/introduction`, were checked against the allowlist before they were followed.

## What was retrieved

Obtained official sources: 29. All are HTTPS 200 from `www.interactivebrokers.com` under `/docs`. Bytes and SHA-256 are in `contracts/verification-plans/TRD-ARCH-03B5.1-acquisition-register.json`. The bodies are in the git-ignored directory `research-archive/ibkr-public-docs/`. That directory is a documentation archive. It is not a broker raw-payload store. No full page text is committed.

Document version is `UNRECORDED` on every source. No API, operating-system, or account-mode tuple is recorded. The download page cites TWS/IB Gateway version `974+` for daily autorestart. That citation is not a pinned tuple. HTTP `Date` headers were not stored. Timestamps are the client clock immediately before the GET, except the documentation-home body, whose timestamp is the local write time of the response bytes, `2026-11-16T17:17:02.2649271Z`.

A second GET of `https://www.interactivebrokers.com/docs/tws-api/doc/quick-start/order-id.md` at `2026-11-16T17:22:56.5834663Z` was byte-identical to the first (958 bytes, SHA-256 `8e3e498387fe0e1ecf61e51f1ff8bebcb1240c35f415971556a71a3764317fdd`). Two GETs of the same URL are two retrievals, not independent source corroboration. The path without `.md` returned the same markdown bytes because the request `Accept` included `text/markdown`. It is not a separate HTML copy. Complete source pairs: 0. `second_copy_status` is `SECOND_COPY_PENDING`. `arch_dec_49_satisfied` is false.

TWS API, IB Gateway, and the Client Portal Web API are kept apart. The download page says that for TWS API purposes TWS and IB Gateway are synonymous, and the execution page limits IB Gateway to the current trading day since midnight while describing a TWS Trade Log setting of up to 7 days. Both sentences are recorded. They are not merged into one product claim. Client Portal Gateway is the Web API local gateway, not the TWS IB Gateway.

## Claims and scenarios

All 27 claims `IBKR-CLM-01` through `IBKR-CLM-27` appear once. 23 have `OFFICIAL_DOCUMENTATION_OBTAINED`. 4 stay `NOT_RETRIEVED`. 13 have `DOCUMENTED_CLAIM_NOT_EMPIRICALLY_VERIFIED` for a cited sentence. 14 stay `HYPOTHESIS`. Every claim stays `EXTERNAL_EVIDENCE_PENDING` and `NOT_EXECUTED`. A quotation used for a documented claim is a substring of the cited archived file.

All 43 scenarios `IBKR-SCN-A01` through `IBKR-SCN-D06` appear once. 27 have a related official passage. 16 stay `NOT_RETRIEVED`. Every scenario stays `NOT_EXECUTED` and `EXTERNAL_EVIDENCE_PENDING`. Oracle limits are unchanged in meaning: an identity sentence is not uniqueness, a correction sentence is not a reversal, a status word is not economic finality, a response end is not broker completeness, and a reconnect sentence is not proof that a send did not happen.

Contradictions left unresolved: `orderId` is called a client-managed strictly increasing id, possibly not unique to an account, the order's unique id, and in one parameter description the order's client id. Execution scope is written as since midnight, the last 24 hours, and a 7-day TWS setting. `permId` is described as identifying an order in an account, and also as 0 for trades originating outside IB. Absent and null `permId` are not distinguished. The word bust does not appear in the retrieved pages. That silence is not a negative finding. `PendingSubmit` is not `DISPATCH_STARTED`.

`XC-01`, `XC-08-IBKR`, `XC-02-PRED`, `XC-03-MATRIX`, and the operational remainder of `XC-04` stay OPEN. ADFR `verification` stays `NOT_VERIFIED`.

## Reads

Twelve official operation names are listed in the research register. None is approved. `EClient.reqOpenOrders` and `EClient.reqAutoOpenOrders` bind orders for client id 0. `EClient.cancelOrder`, `EClient.reqGlobalCancel`, and `EClient.placeOrder` as a modify are not reads. `GET /iserver/account/orders` is documented to affect a websocket topic and, with `force=true`, to clear saved information and return a blank array. That blank array is not broker absence. No operation was called. E2 stays STOP. No environment was selected, including the Paper Trading Account the limitations page names.

## Unchanged protection

ADFR stays 78 entries, every `verification` value `NOT_VERIFIED`, with `ADFR-CROSS-005` superseded by `ADFR-CR-B10-001`. Registry stays `SKELETON_NOT_CANONICAL`, `part_b=NO-GO`, `live_trading_authorized=false`, `broker_uniqueness_evidence=EVIDENCE_PENDING`. The twelve manifest sources matched `sha256` and `bytes`.

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

## Cargo

Commands were run locally on this worktree after the documentation edits. Remote CI was not run. No broker test was run.

| Command | Exit code |
| --- | --- |
| `cargo fmt --all -- --check` | 0 |
| `cargo check --workspace --locked` | 0 |
| `cargo test --workspace --locked` | 0 |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | 0 |

`cargo test --workspace --locked` reported 141 passed, 0 failed, and 1 ignored. The ignored test is `trading-persistence` `connectivity::migration_is_explicit_and_readiness_is_read_only`. It did not run. It asks for `TRADING_ALLOW_DATABASE_INTEGRATION_TESTS=1` and `TRADING_TEST_DATABASE_URL`. The local documentation test re-hashed the git-ignored archive because those files were present. A checkout without that archive cannot repeat the byte check. The metadata check still runs.

## Gates

| Gate | Result | Reason |
| --- | --- | --- |
| A03B51-G01 | PASS | Clean `main` at `f6686d46ba1b8b1fb804506d207f7878a59c218e`. Predecessor `PARTIAL` and the E0 STOP conflict are recorded. ARCH-DEC-01 through 62 are unchanged. `ARCH-DEC-63` is unused. |
| A03B51-G02 | PASS | Twelve sources, registry, ADFR, and the decision files match the baseline hashes. ADFR remains 78 and `NOT_VERIFIED`. |
| A03B51-G03 | PASS | `GO_LIMITED_PUBLIC_DOCS_ONLY` is dated and limited to public documentation GET/HEAD. E1 through E4 stay STOP. The historical protocol was not rewritten. |
| A03B51-G04 | PASS | The allowlist names exact HTTPS hosts and `/docs` paths. Login, API, and loopback URLs are rejected. The two 307 hops were checked before they were followed. |
| A03B51-G05 | PASS | 29 official pages were archived with URL, status, bytes, SHA-256, and timestamp. The documentation test checks those bytes when the archive is present. |
| A03B51-G06 | PARTIAL | No complete pair is claimed. The same-URL second GET is byte-identical and is labeled not independent. `ibkrcampus.com` returned 403. `SECOND_COPY_PENDING`. ARCH-DEC-49 is not satisfied. |
| A03B51-G07 | PASS | 27/27 claims are mapped once. Documented sentences and hypotheses are separated. None is empirically upgraded. |
| A03B51-G08 | PASS | 43/43 scenarios are mapped once. All stay `NOT_EXECUTED` and `EXTERNAL_EVIDENCE_PENDING`. |
| A03B51-G09 | PASS | TWS API, IB Gateway, and Client Portal Web API stay separate. The version tuple is `UNRECORDED`. No environment and no read is approved. |
| A03B51-G10 | PASS | No runtime, schema, broker, order, risk, or OMS change. Registry and ADFR bytes are unchanged. |
| A03B51-G11 | PASS | The four local cargo commands exited 0. 141 passed, 0 failed, 1 ignored. The ignored test was not executed. Remote CI was `NOT_RUN`. |
| A03B51-G12 | PASS | This report and the prompt archive are in the single local commit. No push. |

## Files changed

New authorization, allowlist, acquisition register, and research register under `contracts/verification-plans/TRD-ARCH-03B5.1-*`. Append-only sections in `contracts/verification-plans/README.md`, `contracts/traceability-index.md`, `contracts/domain/unresolved-decisions.md`, and `docs/architecture/README.md`. Documentation test `crates/trading-domain/tests/ibkr_e0_public_documentation_acquisition.rs` and the JSON allowlist in `crates/trading-domain/tests/registry_skeleton.rs`. `.gitignore` ignores `/research-archive/`. The prompt archive and this report. No production `src` change. No registry change. No manifest change. No decision-file change.

## Still unresolved

1. An independent second official copy of the same version. Until it exists, ARCH-DEC-49 is not satisfied and these pages are not an E4 citation.
2. A document or API version tuple. Publication dates were not treated as one.
3. Whether absent, null, and zero `permId` are distinct. Only zero is documented in the retrieved pages.
4. Any named read. E2 stays STOP.
5. An environment class. Paper and live are named in the limitations page and were not selected.
6. Offline fixtures. E1 stays STOP. Broker fault injection stays unauthorized. E3 stays STOP.
7. Ratification of any of these sentences. ARCH-DEC-62 requires a later instruction and a later commit.

**NO IMPLEMENTATION.** **NO VERIFICATION.**
