# TRD-ARCH-SYNC-02 report

**Date:** 2026-11-15  
**Mode:** documentation, register reconciliation, and documentation tests. No trading behavior was added.  
**Live trading:** NOT AUTHORIZED  
**Canonical schemas:** not published  
**Remote CI:** remote CI was not run

`VERDICT: PASS`

`PUSHED: NO`. The commit SHA is the commit that adds this report. It is named in the Cursor output because this file cannot contain the hash of the commit that introduces it. The gates below are the basis for the verdict.

## Baseline

| Item | Value |
| --- | --- |
| Branch | `main` |
| Worktree before edits | clean |
| BASELINE_HEAD | `b2e8660b2c92a86b2877b150b1fb44a193759479` |
| HEAD subject | `TRD-ARCH-SYNC-01: Integrate Architecture Sources and Traceability` |
| Destructive git | none. Nothing was discarded, reset, or stashed. |

TRD-ARCH-SYNC-01 report was read and was not overwritten. Before any edit, all 12 files in `contracts/source-manifest.json` matched that manifest's SHA-256 and byte length. Mismatches: 0. ADFR JSON root status was `ARCHITECTURE_DRAFT_NOT_IMPLEMENTED`. `contracts/registry/registry.json` was `SKELETON_NOT_CANONICAL`, `canonical_schemas_published=false`, `part_b=NO-GO`, `live_trading_authorized=false`, `broker_uniqueness_evidence=EVIDENCE_PENDING`. Registry SHA-256 `7abed2af5363726403175c6524bd7a4d05d36b8735ad48442fc26941f8d98197` (44332 bytes) is unchanged.

## What was decided

The wording is the user decision of 2026-11-15, recorded in [ARCH-DEC-01-bis-03-2026-11-15.md](../../contracts/decisions/ARCH-DEC-01-bis-03-2026-11-15.md).

**ARCH-DEC-01.** The reconstructed DET-01 and DET-02 files are a binding design baseline, except every detail those files mark open, undefined, unconfirmed, or verification-required. Label: `DESIGN_BASELINE_RATIFIED_WITH_EXCEPTIONS`. Provenance stays `RECONSTRUCTED_DRAFT`. This task does not claim the original Q&A transcripts are present or independently verified.

Not decided: release of transition matrices, `OrderClosure` reasons, economic finality, risk postings, or technical execution. Q01–Q10 verification cells stay OPEN. The twenty `CONFIRMED_DECISION` rows keep the source string `conversation: confirmed DET-01/02 Q01-Q10`.

**ARCH-DEC-02.** The register has 78 historical IDs. Exactly one is superseded: `ADFR-CROSS-005`. The successor is `ADFR-CR-B10-001`. JSON and CSV carry the same IDs, the same six core fields, and the same supersession links. No historical ID was deleted or renamed. 77 not-superseded entries are a count, not a claim that those decisions are implemented or verified.

Not decided: which of `OPEN P0`, `PROPOSED_FOR_ACCEPTANCE`, and `DESIGN_RESOLVED` replaces the others. B10-T01 through B10-T12 were not run and are not passed.

**ARCH-DEC-03.** For dispatch and failover design, CR-B01 has precedence over conflicting or less restrictive DET-07 statements. No automatic takeover while the previous writer might still send or submission results are unresolved. Elapsed time or a lease timeout alone is not proof of a safe takeover. This is design precedence. It is not an implemented fence and not demonstrated IBKR behavior.

Not decided: broker-session exclusivity, lease and heartbeat protocol, dispatch-scope boundary, in-flight send evidence, recovery tests, or IBKR behavior. No default TTL, automatic resume, time-based failover, or new dispatch state was added.

## ADFR

| Item | Value |
| --- | --- |
| JSON count | 78 |
| CSV count | 78 |
| Unique IDs | 78, same set and same order in both files |
| Superseded count | 1 (`ADFR-CROSS-005`, lifecycle `SUPERSEDED`) |
| Not superseded | 77 |
| Successor | `ADFR-CR-B10-001` (`supersedes` `ADFR-CROSS-005`, lifecycle `NOT_SUPERSEDED`) |
| Field parity | `id`, `decision`, `source`, `status`, `basis`, `verification` match between JSON and CSV |
| Verification | every row `NOT_VERIFIED` |
| Root status | `ARCHITECTURE_DRAFT_NOT_IMPLEMENTED` |
| Revision | `1.0+ARCH-DEC-02-2026-11-15`. `schema_version` remains `1.0`. `as_of` remains `2026-11-14`. |

`lifecycle` is separate from `status` and `verification`. `ADFR-CROSS-005` stays `PROPOSED_FOR_ACCEPTANCE` / `NOT_VERIFIED`. `ADFR-CR-B10-001` stays `DESIGN_RESOLVED` / `NOT_VERIFIED`. `DESIGN_RESOLVED` is not a test pass. `NOT_SUPERSEDED` is not acceptance.

The six core fields of the previous 77 JSON IDs are unchanged. Python `json.dumps` with ASCII escaping of those objects, in the pre-amendment order, is SHA-256 `f5ecf07d6ba9ed566be2aeaf6f17030fee66198615a4e87ee5319200a688608f` both before the amendment and on the amended file's core fields. The documentation test locks the `serde_json` compact encoding of the same objects, SHA-256 `f68e4b94c6437fba1e805a9184ec6b6663b4f8b7fd58c14c177ccd1913c544c2`. The justified additions are the copied CSV row `ADFR-CR-B10-001` and the three lifecycle fields on every row. ID order is the pre-amendment CSV order. JSON and CSV are UTF-8 with LF line endings, matching `.gitattributes`. The v1.0 filenames were amended in place, with the column change documented in [ADFR-README.md](../../contracts/sources/ADFR-README.md), so consumers are not left with a silent six-column contract.

Status counts inside the 78: 20 `CONFIRMED_DECISION`, 57 `PROPOSED_FOR_ACCEPTANCE`, 1 `DESIGN_RESOLVED`.

## Source manifest

12 source files are listed. Current `sha256` and `bytes` match the repository files. `package_sha256` and `package_bytes` keep the TRD-ARCH-SYNC-01 intake values.

| File | Package bytes still current |
| --- | --- |
| 9 unchanged sources | yes |
| `ADFR-Consolidated-v1.0.json` | no. Amended. Current `272a6dabdb411701a25dd708577dbb498c7a7cd2754d83c612e85454a178a1d5`, 29880 bytes. Package `4c587b94f668e34e9c088650bbcc4d86ee51c90cbd3602281ce2b0e93931452c`, 21514 bytes. |
| `ADFR-Consolidated-v1.0.csv` | no. Amended. Current `b5577302650e3d497465b93063d149313ec1197a38ddad3a9e7a6f223757ec32`, 13138 bytes, LF. Package `242e4a1205ee1a050218fd52b6597af02e9badd13ac842e01df70116ca4dda13`, 11831 bytes. |
| `Architecture-Consolidation-and-Acceptance-v1.0.md` | no. Dated addendum only. Current `9f1fdd36e0a45343762a2d608b7b71dccaa45f82e41f7adbddbf1560e18bd9a1`, 5770 bytes. Package `334bcfcba5015534184d65c556d0cd2d308238bd76bf61429c2bfe6a877bfd67`, 3676 bytes. |

DET-01 and DET-02 bytes are the package bytes. Their manifest rows gained `design_baseline` = `DESIGN_BASELINE_RATIFIED_WITH_EXCEPTIONS` and `open_details_remain_open` = true. `approved` stays false. `reviewed` stays false. The decision record is `ratification.document`, with `part_of_source_package` = false. It is not listed as a ZIP file. The intake block `hash_verification` still describes the ZIP check. It does not claim that every current byte equals the package.

## XC delta

| Finding | Result |
| --- | --- |
| XC-11 population and supersession | `RESOLVED_BY_ARCH_DEC_02` |
| XC-11 status-word disagreement (`OPEN P0` / `PROPOSED_FOR_ACCEPTANCE` / `DESIGN_RESOLVED`) | OPEN |
| XC-04 precedence of CR-B01 over conflicting or less restrictive DET-07 | `RESOLVED_BY_ARCH_DEC_03` |
| XC-04 broker-session exclusivity, lease/heartbeat, dispatch scope, in-flight send evidence, recovery tests, IBKR behavior | OPEN |
| XC-03-MATRIX | OPEN. ARCH-DEC-01 does not close it. |
| XC-01, XC-02-PRED, XC-05-LOCK, XC-06, XC-07-ROUND, XC-08-IBKR, XC-09-RESET, XC-10-ACTION | OPEN |

No other sub-finding was closed. `NO_CONFLICT_FOUND` rows from TRD-ARCH-SYNC-01 stay as that report left them.

## Safety

`contracts/registry/registry.json` is byte-identical to the baseline. Status remains `SKELETON_NOT_CANONICAL`. Canonical schemas are unpublished. `part_b` remains `NO-GO`. Live trading is unauthorized. No file under `crates/*/src` or `apps/*/src` was edited. No route, adapter, migration, credential, or runtime flag was added. B10-T01 through B10-T12 are not passed.

## Changed files

| Path | SHA-256 | Bytes |
| --- | --- | --- |
| `contracts/decisions/ARCH-DEC-01-bis-03-2026-11-15.md` | `9f946806667a3700cb45d5033e509355aeefc60d2216262647f2dbbae034e30a` | 9761 |
| `contracts/sources/ADFR-Consolidated-v1.0.json` | `272a6dabdb411701a25dd708577dbb498c7a7cd2754d83c612e85454a178a1d5` | 29880 |
| `contracts/sources/ADFR-Consolidated-v1.0.csv` | `b5577302650e3d497465b93063d149313ec1197a38ddad3a9e7a6f223757ec32` | 13138 |
| `contracts/sources/ADFR-README.md` | `c0428f05e13cc8af6bb2875331d7d5cfb6983ab7fe12cccad4566b0b7f4c58bd` | 3990 |
| `contracts/sources/Architecture-Consolidation-and-Acceptance-v1.0.md` | `9f1fdd36e0a45343762a2d608b7b71dccaa45f82e41f7adbddbf1560e18bd9a1` | 5770 |
| `contracts/source-manifest.json` | `82b205d27172ad234d46cfa2a59f0f4785128dd7a6282dea1aab32b412fc2808` | 17945 |
| `contracts/traceability-index.md` | `8d1ffb5ee16eb788dfd26bd750f5fbddf57299cecae3b88b7c7ba3cb66421b0c` | 17928 |
| `contracts/registry/README.md` | `d6cd68722d892d0f3d77140472d68e3280e4d50d5d69fe40060e2f208194dc5f` | 2408 |
| `docs/architecture/README.md` | `0607038a6a36acd8ca5cbfef44b5e47c56fe98701a4289bf3e5ed68c6fed809c` | 11222 |
| `docs/cursor-prompts/trading/TRD-ARCH-SYNC-02-Architecture-Ratification-and-ADFR-Reconciliation.md` | `a3c47ca4db738ed2c0239046d8059abfe2a09b09bb88096f9b448fe63d331cba` | 14299 |
| `tests/README.md` | `27fe2bd1b4544d3dadf2d27adb57716597365b2308998f96c1967544de1cd7f2` | 791 |
| `crates/trading-domain/Cargo.toml` | `95b8fa661cb41272faec5ac2ae54302c7cba12de1e58af9e1ff882257bfc9f88` | 519 |
| `crates/trading-domain/tests/adfr_reconciliation.rs` | `377e0c18334bb83bc9b87711f5eae1177a1b659ea85bd5025ef817051c439f0c` | 19646 |
| `Cargo.lock` | `76fd3cefad9eef856ec2d784b1ae373d53ba25a5030a77be00a77d9b8adb80f0` | 50294 |

`Cargo.lock` gains one edge: `trading-domain` depends on `sha2 0.10.9`, which was already in the lockfile as a crate. This report's own hash is not listed.

## Local checks

Remote CI was not run. The ignored PostgreSQL test was not executed against a database.

| Command | Exit code |
| --- | --- |
| `cargo fmt --all -- --check` | 0 |
| `cargo check --workspace --locked` | 0 |
| `cargo test --workspace --locked` | 0 |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | 0 |

`cargo test` left one test ignored: `crates/trading-persistence/tests/connectivity.rs`, `migration_is_explicit_and_readiness_is_read_only`, because `TRADING_ALLOW_DATABASE_INTEGRATION_TESTS=1` and `TRADING_TEST_DATABASE_URL` were not set. It was not run against a database. Ignored is not passed.

After the hash constant in the documentation test was corrected, `cargo clippy -p trading-domain --all-targets --locked -- -D warnings` also exited 0. The documentation test was run again after this section was written.

## Gates

| Gate | Result |
| --- | --- |
| AS02-G01 | PASS. Clean baseline, 12/12 intake hashes matched before edits. |
| AS02-G02 | PASS. ARCH-DEC-01/02/03 dated and quoted. |
| AS02-G03 | PASS. Design baseline with reconstructed provenance and open details. |
| AS02-G04 | PASS. 78 identical historical IDs. |
| AS02-G05 | PASS. One superseded ID, named successor, 77 not superseded. |
| AS02-G06 | PASS. Manifest current hashes match bytes. Package hashes kept. Core texts of the old 77 IDs unchanged. |
| AS02-G07 | PASS. CR-B01 precedence recorded. XC-04 remainder OPEN. |
| AS02-G08 | PASS. Listed XC findings stay OPEN. No implementation approval. |
| AS02-G09 | PASS. Registry bytes unchanged. No trading runtime edit. |
| AS02-G10 | PASS. `adfr_reconciliation` 6/6 and `registry_skeleton_stays_unpublished` passed. |
| AS02-G11 | PASS. The four local Cargo commands exited 0. One PostgreSQL test stayed ignored and was not run. |
| AS02-G12 | PASS when this report and the prompt archive are in the one commit named in the Cursor output. Nothing was pushed. |

## Deviations

The v1.0 ADFR filenames were amended in place. A parallel v1.1 pair would have left the v1.0 JSON at 77 IDs. The added CSV columns and JSON keys are documented in `ADFR-README.md` and in the JSON `amendment_note`. DET-01 and DET-02 source bytes were not edited. The 2026-11-14 consolidation sentences were not rewritten. An addendum was appended.

## Next step

Plan only, not an instruction to implement: the next documentation or design decision, if the user wants one, would have to pick a status vocabulary for the rows that still disagree (`OPEN P0`, `PROPOSED_FOR_ACCEPTANCE`, `DESIGN_RESOLVED`) and would have to supply the still-open XC-03-MATRIX and XC-04 evidence (closure reasons, transition edges, lease/heartbeat, dispatch scope, and IBKR session behavior). Until those exist, dispatch, failover, `OrderClosure`, and live trading stay unimplemented and fail-closed.
