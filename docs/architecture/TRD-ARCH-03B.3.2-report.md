# TRD-ARCH-03B.3.2 report

**Date:** 2026-11-15  
**Mode:** documentation, traceability, and documentation-only tests. No trading behavior was added.  
**Live trading:** NOT AUTHORIZED  
**Canonical schemas:** not published  
**Broker uniqueness:** `EVIDENCE_PENDING`  
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
| BASELINE_HEAD | `1adfc2c32f02a1e53576282ed32e9c2ff2349873` |
| HEAD subject | `TRD-ARCH-03B.3: Draft Broker Identity and Snapshot Completeness Decisions` |
| Predecessor report | `docs/architecture/TRD-ARCH-03B.3-report.md` present in that commit |
| Predecessor prompt | `docs/cursor-prompts/trading/TRD-ARCH-03B3-Broker-Identity-and-Snapshot-Completeness-Design-Resolution.md` present in that commit |
| rustc | 1.99.1 (58cf8f9c3 2026-10-13) |
| cargo | 1.99.1 (5f94df478 2026-08-27) |
| git | 2.56.0.windows.3 |
| Destructive git | none. Nothing was discarded, reset, or stashed. |

ARCH-DEC-35 through ARCH-DEC-48 were unused as decision identifiers before this task. No decision file, proposal index, resolution index, or registry entry assigned them. The only mention was a forward guard in `risk_design_policy_21_through_34_stays_unimplemented` asserting that no decision filename contained `ARCH-DEC-35`. That guard was moved to `ARCH-DEC-49` when this package used 35 through 48. No identifier was renumbered. No existing decision file was overwritten.

Registry flags at baseline were `SKELETON_NOT_CANONICAL`, `canonical_schemas_published=false`, `part_b=NO-GO`, `live_trading_authorized=false`, `broker_uniqueness_evidence=EVIDENCE_PENDING`. Registry SHA-256 `7abed2af5363726403175c6524bd7a4d05d36b8735ad48442fc26941f8d98197` (44332 bytes) matches the previously reported value and is unchanged.

The decision filename `contracts/decisions/ARCH-DEC-35-bis-48-2026-11-15.md` follows the existing `ARCH-DEC-21-bis-34-2026-11-15.md` convention. No alternate name was required.

## Authority

The authority is the user ratification dated 2026-11-15 in the TRD-ARCH-03B.3.2 instruction, archived at `docs/cursor-prompts/trading/TRD-ARCH-03B3.2-Ratify-Broker-Design-Decisions.md`. No signature artifact was supplied, and none is invented. No evidence URI is recorded. No source transcript is invented. No independent approval is invented. Recommendations in the historical BROKERQ text are not this ratification. BROKERQ-02 and BROKERQ-08 were ratified as option B where the non-binding recommendation had been A. BROKERQ-07 was ratified as option A. BROKERQ-13 option A is not authority. BROKERQ-14 option A is not verification. Option C stays `CONFLICTS_WITH_BINDING_POLICY` for every question.

Record labels `RATIFIED_DESIGN_POLICY_ONLY`, `NOT_IMPLEMENTED`, `NOT_VERIFIED`, and `NON_RUNTIME_NON_WIRE` describe the new decision file and the resolution index. They are not a canonical status enum and they do not change an ADFR `verification` cell.

The archived prompt is the executed instruction. One sentence at the top of that file identifies it as the instruction and states that no signature or evidence URI is attached. It does not add a decision.

## Decision mapping

Each row is `RATIFIED_DESIGN_POLICY_ONLY` / `NOT_IMPLEMENTED` / `NOT_VERIFIED`. The source anchor is the historical heading in `contracts/design-proposals/TRD-ARCH-03B3-approval-requests.md`. The decision record is `contracts/decisions/ARCH-DEC-35-bis-48-2026-11-15.md`.

| Decision | Question | Option | User wording |
| --- | --- | --- | --- |
| ARCH-DEC-35 | BROKERQ-01 | B | Vorhandene Broker-/Konto-/Umgebungs-/Session-/Aliasfelder, Client und Adapter-Epoch sowie Evidence-ID nur soweit tatsächlich geliefert als mögliche zukünftige Evidenzkomponenten erfassen. Fehlend bleibt fehlend. Feldgleichheit ist weder Merge noch Uniqueness; kein Schema. |
| ARCH-DEC-36 | BROKERQ-02 | B | Zukünftige Kandidatenmenge `A(e)` und Unsicherheitslabel erlauben; selbst `|A(e)|=1` ist `SINGLE_CANDIDATE_NOT_AUTHORIZATION`; 0/mehrere/Widerspruch ebenso nicht autorisierend. Keine automatische Zuordnung mit wirtschaftlicher Wirkung. |
| ARCH-DEC-37 | BROKERQ-03 | B | Beobachtung auch ohne `permId` behalten; fehlend/null/zero nicht stillschweigend gleichsetzen. Spätere `permId` als neue Evidenz, kein Overwrite. Null-/Zero-Semantik bleibt extern ungeprüft. |
| ARCH-DEC-38 | BROKERQ-04 | B | Identischer Rohpayload bei Wiederholung idempotent als erneut gesehen; abweichender Payload unter scheinbar gleichem Schlüssel `CONTRADICTION`, beide Originale erhalten. Keine zweite Ausführung/Buchung, keine Bytes löschen. |
| ARCH-DEC-39 | BROKERQ-05 | B | Correction/Bust append-only mit Original-Evidenz verknüpfen, nur wenn die neue Evidenz selbst einen Bezug liefert. Kein erfundener Link, keine Reversal-Buchung, kein Release, keine Closure-Änderung. |
| ARCH-DEC-40 | BROKERQ-06 | B | Angefragten Account-/Client-/Session-/Dataset-/Zeitfenster-Scope soweit vorhanden dokumentieren; Coverage bleibt `UNKNOWN`, nicht global, keine `COMPLETE`-Behauptung. |
| ARCH-DEC-41 | BROKERQ-07 | A | Keine Completeness-Tokens speichern. Kein `COMPLETE`-Marker und kein externer Token-Platzhalter als Ratifikation; leere Antwort ist kein Nicht-Existenz-/Finalitäts-/Release-Beweis. |
| ARCH-DEC-42 | BROKERQ-08 | B | Nur tatsächlich lokal beobachtete Capture-Gaps (z.B. kurze Seite, fehlendes Antwortende, Drop) markieren; kein erfundener IBKR-Sequenzzähler, keine Interpolation, Scope bleibt unvollständig, keine Folgeaktion. |
| ARCH-DEC-43 | BROKERQ-09 | B | Reconnect erzeugt neuen Session-Kontext; alte Evidenz behalten, IDs nicht ohne nachgewiesene Bindung über Session-Grenzen übernehmen. Reconnect ist kein Send-Negativbeweis. |
| ARCH-DEC-44 | BROKERQ-10 | B | CR-B01-Crash-/Dispatch-Fenster als Labels dokumentieren. Nach `DISPATCH_STARTED` möglicherweise gesendet; kein Fenster dieses Pakets erlaubt Resend, Cancel, Release oder Dispatch. |
| ARCH-DEC-45 | BROKERQ-11 | B | Lokale Fence-Fakten (Epoch, Lease-Deadline, Authority-Label) von Broker-Akzeptanz strikt trennen; Successor blockiert solange früherer Writer noch senden könnte oder Sendestatus unbekannt ist. Lease-Ablauf/neuere Epoch beweist kein externes Fencing. Keine Implementierung oder TTL. |
| ARCH-DEC-46 | BROKERQ-12 | B | Nur nicht-autorisierenden Eskalationsmarker für spätere menschliche Prüfung vorsehen; keine Rolle, kein Post-/Release-/Dispatch-/Reset-Recht. Fehlende erforderliche zweite Person bedeutet Deny, nicht simulierte Zustimmung. |
| ARCH-DEC-47 | BROKERQ-13 | B | Broker-Rohbeobachtung und lokale Attempt-Fakten bei Widerspruch beide erhalten; Vergleich als Inferenz kennzeichnen. Keine Seite gewinnt pauschal, kein wirtschaftlicher Schluss, kein Cancel. |
| ARCH-DEC-48 | BROKERQ-14 | B | Für spätere Verifikation tatsächlich beschaffte offizielle Unterlagen, reproduzierbare positive/negative Fälle, Umgebung, Brokerprodukt/API-Version, Zeitstempel und Rohpayloads verlangen. Heute alles `NOT_EXECUTED`/`EXTERNAL_EVIDENCE_PENDING`; Dokumentations-/Unit-Tests sind keine Broker-Evidenz. |

Options in order: B, B, B, B, B, B, A, B, B, B, B, B, B, B. Count: 13 times B and 1 time A. BROKERQ-07 is the only A.

BROKERQ-01 B records supplied fields only. Equality is not a merge and not uniqueness. BROKERQ-02 B allows a future candidate set. A singleton is `SINGLE_CANDIDATE_NOT_AUTHORIZATION`. BROKERQ-03 B keeps a missing `permId` distinct from null and from zero, and a later `permId` is new evidence. BROKERQ-04 B treats an identical raw payload as seen again, with no economic effect, and keeps both originals on contradiction. BROKERQ-05 B links a correction only when the new evidence itself names the link. BROKERQ-06 B leaves coverage `UNKNOWN`. BROKERQ-07 A stores no completeness token. An empty response is not a negative proof. BROKERQ-08 B marks only a locally observed capture gap and invents no IBKR sequence. BROKERQ-09 B creates a new session context. Reconnect is not proof that a send did not happen. BROKERQ-10 B labels CR-B01 windows and authorizes no resend. BROKERQ-11 B keeps a local fence distinct from broker acceptance. Lease expiry is not takeover. BROKERQ-12 B is a marker and not a role. BROKERQ-13 B names no winner. BROKERQ-14 B requires later positive and negative external evidence. Those cases are `NOT_EXECUTED` and `EXTERNAL_EVIDENCE_PENDING`. Documentation tests are not broker evidence.

## Open XC map

| Finding | Status after this record |
| --- | --- |
| XC-01 | OPEN. `EVIDENCE_PENDING`. |
| XC-02-PRED | OPEN. No predicate was adopted. |
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

The DET-03-D04 and CR-B04 §4.3 sentence that a single match books stays subordinate to ARCH-DEC-13 and ARCH-DEC-26. `DESIGN_RESOLVED` is not verification. This record states no IBKR guarantee for `orderId`, `permId`, `execId`, sequence, session lifetime, response completeness, or timeout.

ARCH-DEC-03, ARCH-DEC-13, ARCH-DEC-14, ARCH-DEC-17, ARCH-DEC-26, ARCH-DEC-33, and ARCH-DEC-34 stay binding. This record does not supersede them.

## Prohibitions

No uniqueness guarantee, global alias key, canonical identity tuple, UNIQUE index, identity resolver, schema, DDL, or migration is approved. No economic posting, compensating or reversal line, reservation-to-exposure conversion, economic-finality predicate, risk release, or reduction of an existing reservation is approved. No OMS transition, `OrderClosure` release, terminal broker status as sole authority, cancel, modify, replace, broker action, broker dispatch, retry, resubmit, automatic takeover, live trading, or paper-broker request is approved. No new operator role, adjudication right, reset right, dual-approval substitute, lock order, isolation level, TTL, or heartbeat is approved. Documentation tests do not prove broker behavior, reconciliation, crash freedom, or economic safety. The ignored PostgreSQL test was not run against a database. Ignored is not passed. **NO IMPLEMENTATION.** **NO VERIFICATION.**

## Source and ADFR integrity

Before any edit, all 12 files in `contracts/source-manifest.json` matched that manifest's current `sha256` and `bytes`. Mismatches: 0. The same check is asserted again in `broker_design_policy_35_through_48_stays_unimplemented`. `package_sha256` and `package_bytes` remain the TRD-ARCH-SYNC-01 intake values. DET-01 and DET-02 stay `reconstructed_draft` true, `SOURCE_REVIEW_PENDING`, `reviewed` false, and `approved` false. Their design baseline with exceptions stays binding under ARCH-DEC-01.

`contracts/source-manifest.json` was not edited. The 12 manifest source files were not edited. `contracts/registry/registry.json` was not edited. The ADFR JSON and CSV were not edited. ARCH-DEC-01 through ARCH-DEC-34 were not edited. The earlier proposal and resolution indexes were not edited. The four other TRD-ARCH-03B.3 proposal notes were not edited. The first 31525 bytes of `TRD-ARCH-03B3-approval-requests.md` still hash to `0ae902d63e138833545b85fe23efa745ef20f9a1f1a07d4790af76374290975f`. That is the pre-edit hash of the whole file. The ratification addendum begins after those bytes.

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
| Historical broker proposal index SHA-256 | `e808dcf8a2339845b4ef49085275b7aad9094862086830d57d13a415841703ce` (13593 bytes) |
| Broker semantics note SHA-256 | `51f098c9b4e4caa0f9d204496c8976c113139e2ff6a74a8654637202637a3583` (18980 bytes) |
| Broker options note SHA-256 | `5939cb69b1fc198d961904e1f4c9371afe2d7810e33c5ff168734b220ec6e2c8` (13127 bytes) |
| Broker coverage note SHA-256 | `5252d0d5a6864273e065bde7cfd0d2d0ecb75b70f7d43334b9ba4c967b515173` (17455 bytes) |
| Broker conflict matrix SHA-256 | `479ee5cf92d9f09ba80d5fd780d85218ae61c8746d268c85c7b13e14b40020c7` (7916 bytes) |
| ARCH-DEC-01..03 SHA-256 | `9f946806667a3700cb45d5033e509355aeefc60d2216262647f2dbbae034e30a` (9761 bytes) |
| ARCH-DEC-04..08 SHA-256 | `af2cd4b5c35912fe97d3a8ea36c459259aabf0a7b7d54657fdb7b7bce6149ddc` (13679 bytes) |
| ARCH-DEC-09..20 SHA-256 | `ddaffca4b9d1d69c430a3241db14460e49adf1e8d54a33ac9f0f07b5b957a7a2` (24820 bytes) |
| ARCH-DEC-21..34 SHA-256 | `a3afc65a0e1a965ac85a9015acc60da858be0ee57ffe104098f2cf56407e5b07` (31894 bytes) |
| Prior OMS proposal index SHA-256 | `96dc34c61db04bc0d4eb0576a98f787fddb5632816b903985b1163993afc73cc` (3654 bytes) |
| Prior OMS resolution index SHA-256 | `e4bd482d2b8b971612e241e069009e86810bfde3cf499784725e6d3b5019ca00` (6179 bytes) |
| Historical risk proposal index SHA-256 | `6520b2d64ebcdd00793872ac97a618c9c635e0283d3cf479f7926fc83dac62e6` (7035 bytes) |
| Prior risk resolution index SHA-256 | `5a09d0daedef4bd9faebfd2ab1f22844fefc8bdb6ac1f906222aea94adb54718` (10530 bytes) |

Pre-edit and post-edit hashes of those protected files are the same.

## Non-implementation

No file under `crates/*/src` or `apps/*/src` was edited. No route, adapter, migration, credential, runtime flag, order or risk state machine, broker call, lease, audit write, or financial rounding was added. `Cargo.toml` and `Cargo.lock` were not changed. Their hashes stay `5843ca50c51c0c316bf716c7e3412e4d1be1d9a3870d278c6ef0ef09cd186d3f` (1473 bytes) and `76fd3cefad9eef856ec2d784b1ae373d53ba25a5030a77be00a77d9b8adb80f0` (50294 bytes). No production dependency was added. `ORDER_STATE_MACHINE_IMPLEMENTED`, `STATE_TRANSITIONS_IMPLEMENTED`, `IDENTITY_RESOLVER_IMPLEMENTED`, `BROKER_DISPATCH_IMPLEMENTED`, `RISK_EVALUATION_IMPLEMENTED`, and `FINANCIAL_ROUNDING_IMPLEMENTED` stay false. `OrderClosure` stays absent. No economic posting, risk release, broker terminal acceptance, modify or replace dispatch, or live trading was enabled.

`TRD-ARCH-03B32-resolution-index.json` is `NON_RUNTIME_NON_WIRE_RESOLUTION_INDEX`. It is not `registry.json`, not the historical proposal index, and not a wire schema. It is allowlisted beside the other non-wire indexes and is not registered as a schema. Its flags include `implementation_authorized=false`, `uniqueness_guaranteed=false`, `uniqueness_index_approved=false`, `completeness_proof_approved=false`, `canonical_schemas_published=false`, `economic_posting_approved=false`, `risk_release_approved=false`, `economic_finality_predicate_approved=false`, `oms_transitions_approved=false`, `broker_dispatch_approved=false`, `live_trading_authorized=false`, `closes_xc_findings=false`, and `conflict_closure=false`. `evidence_uri` and `signature_artifact` are null. `tests_executed` is false. That flag means external broker and coverage tests. It does not describe the documentation regression tests in this commit. Those documentation tests ran, and they are not broker evidence. External cases stay `NOT_EXECUTED` and `EXTERNAL_EVIDENCE_PENDING`.

## Changed files

| Path | SHA-256 | Bytes |
| --- | --- | --- |
| `contracts/decisions/ARCH-DEC-35-bis-48-2026-11-15.md` | `5c16bc15b2a67fac16f476437d998de834ce823972f40f584647bf607e26b4f5` | 48595 |
| `contracts/design-proposals/README.md` | `57424b66d7d5da93a02709d1e20b2b92a75581aca43afda2a2703f8c76a32f4a` | 8893 |
| `contracts/design-proposals/TRD-ARCH-03B3-approval-requests.md` | `bc26c0664c4f3a200cbc449544a3726b17956a02a450afcaa6ba6945e9c6a33e` | 37183 |
| `contracts/design-proposals/TRD-ARCH-03B32-resolution-index.json` | `e962623370a87fc7e93acc0c90b92728ccfae4678d4247fade0d5f2259e5a6a9` | 14376 |
| `contracts/domain/glossary.md` | `7c7f26e36133a1f7d5f1230b6fe3255acb03cceb6ccc0b57d7c44490df3dfe84` | 18385 |
| `contracts/domain/unresolved-decisions.md` | `1af1c98924a89e5ec89935104dd3619bac9efe277960d09a7200311ac5d61d5e` | 26844 |
| `contracts/traceability-index.md` | `c6b5b4c4015fc40f7cdc2167ade1a3154b822dd4dfde6751d665d27cde1a0b55` | 30571 |
| `crates/trading-domain/tests/adfr_reconciliation.rs` | `61ebfbb18d144c010175633888674b44716a13c1859a533c7b4789949020e0c4` | 167885 |
| `crates/trading-domain/tests/registry_skeleton.rs` | `df7cac6578355b707b19ec334de1d2527a6e0eeb6d70d673fa230ff337dd60f8` | 29739 |
| `docs/architecture/README.md` | `f19e16902daf8cda24afb54009e980772527ee42c2f5aa5786c4afd05b70edc9` | 15130 |
| `docs/cursor-prompts/trading/TRD-ARCH-03B3.2-Ratify-Broker-Design-Decisions.md` | `75009c083876f0b1b8cd5a2aaf17e49aff321f04c509859e35a1b0d15c284de9` | 18369 |
| `tests/README.md` | `4929442224e07bee932937ae477b9c3ea86c8b6805cc92f637ce37a5d52caca9` | 1762 |

This report's own hash is not listed. No Cargo metadata change required a dependency justification.

## Local checks

Remote CI was not run. The ignored PostgreSQL test was not executed against a database. Ignored is not passed. No external broker request was sent.

`cargo fmt --all -- --check` was run once before formatting and exited 1. The diff was layout in `crates/trading-domain/tests/adfr_reconciliation.rs` only. `cargo fmt -- crates/trading-domain/tests/adfr_reconciliation.rs` was then applied. It did not format unrelated files. `cargo fmt --all -- --check` then exited 0.

The first `cargo test --workspace --locked` exited 0 and printed one `dead_code` warning for three unread fields on the new policy-index struct. Those fields were then asserted. The final test command below is the run after that correction. It printed no warning.

The four commands below were run from the repository root on the tree that this commit records. None failed. Clippy did not fail on the final run, so there was no second clippy correction.

| Command | Exit code |
| --- | --- |
| `cargo fmt --all -- --check` | 0 |
| `cargo check --workspace --locked` | 0 |
| `cargo test --workspace --locked` | 0 |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | 0 |

`cargo test` result: 136 passed, 0 failed, 1 ignored. The ignored test is `migration_is_explicit_and_readiness_is_read_only` in `crates/trading-persistence/tests/connectivity.rs`, because `TRADING_ALLOW_DATABASE_INTEGRATION_TESTS=1` and `TRADING_TEST_DATABASE_URL` were not set. It was not run against a database. `adfr_reconciliation` passed 16 tests, including `broker_identity_proposals_stay_unratified_and_dispatch_stays_forbidden` and `broker_design_policy_35_through_48_stays_unimplemented`. `registry_skeleton_stays_unpublished` passed.

These documentation tests do not prove broker behavior, reconciliation, crash freedom, or economic safety.

## Deviations

`tests/README.md` received one sentence so the test index names the new design-policy record beside the still-unratified historical proposal index. `contracts/domain/glossary.md` received one paragraph stating that ARCH-DEC-35 through ARCH-DEC-48 still do not approve uniqueness, a schema, a resolver, a completeness token, a posting, a release, a predicate, an OMS transition, or a broker dispatch. Type rows were not rewritten.

The historical proposal comparison tables were not rewritten. The resolution text is an addendum after the original 31525 bytes. The historical prefix hash was checked again and was unchanged. The Rust sources under `crates/*/src` and `apps/*/src` were not changed.

The existing risk-policy test's unused-id guard named `ARCH-DEC-35`. It now names `ARCH-DEC-49`, the next unused identifier. The risk decision file and the risk resolution index were not edited.

The first test compilation warned that three index fields were unread. The final test reads them. That correction is in this same commit.

No evidence file was created. `contracts/source-manifest.json`, `contracts/registry/registry.json`, the 12 package source files, the ADFR JSON and CSV, and the earlier decision records were not edited.

## Gates

| Gate | Result |
| --- | --- |
| A03B32-G01 | PASS. Clean baseline. Predecessor commit `1adfc2c32f02a1e53576282ed32e9c2ff2349873` includes the 03B.3 report. ARCH-DEC-35..48 were unassigned. 12/12 current manifest hashes match. Registry hash matches the previously reported value. |
| A03B32-G02 | PASS. Fourteen user choices map to ARCH-DEC-35..48, dated 2026-11-15, checksum 13 B and 1 A, with the instruction as authority. No signature or evidence URI was invented. |
| A03B32-G03 | PASS. The historical BROKERQ request prefix and `TRD-ARCH-03B3-proposal-index.json` are byte-identical. The resolution addendum and `TRD-ARCH-03B32-resolution-index.json` are separate. Recommendations were not rewritten. |
| A03B32-G04 | PASS. 78 IDs, JSON/CSV parity, 78 `NOT_VERIFIED`, one supersession, 12 manifest sources. Protected hashes match preflight. ARCH-DEC-01..34 files are byte-identical. |
| A03B32-G05 | PASS. BROKERQ-01..05 are the ratified options. No UNIQUE index, merge, zero-match, second execution, invented link, or posting is approved. |
| A03B32-G06 | PASS. BROKERQ-06..08 leave coverage `UNKNOWN`, store no completeness token, and invent no IBKR sequence. An empty response is not a negative proof. |
| A03B32-G07 | PASS. BROKERQ-09..11 separate sessions, authorize no resend, and do not treat a local fence or lease expiry as broker acceptance. |
| A03B32-G08 | PASS. BROKERQ-12 adds no role. BROKERQ-13 names no winner. BROKERQ-14 requires unrun positive and negative external evidence. |
| A03B32-G09 | PASS. XC-01, XC-08-IBKR, the XC-04 remainder, and XC-02-PRED stay OPEN. CR-B01 precedence stays in force. DET-03-D04 stays subordinate to ARCH-DEC-13 and ARCH-DEC-26. |
| A03B32-G10 | PASS. No production source, schema, migration, dependency, broker, booking, release, OMS, or live-trading change. Safety flags stay false. |
| A03B32-G11 | PASS on the final run. The first `cargo fmt --check` exit 1 and the intermediate dead-code warning are recorded above. One PostgreSQL test stayed ignored and was not run. Remote CI was not run. |
| A03B32-G12 | PASS when this report and the prompt archive are in the one commit named in the Cursor output. Nothing was pushed. |

## Next step

Not started. External broker identity and snapshot evidence remain `NOT_EXECUTED`. The economic-finality predicate remains unresolved. The OMS transition matrix, an `OrderClosure` enum, economic posting, risk release, broker dispatch, deployment, and live trading are not authorized and are not implemented. `XC-01`, `XC-02-PRED`, and `XC-08-IBKR` stay OPEN.
