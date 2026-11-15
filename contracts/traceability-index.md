# Traceability index

**Status:** index only. This file does not approve, test, or implement a contract.  
**Current date:** 2026-11-15  
**Live trading:** NOT AUTHORIZED  
**Comparison:** [TRD-ARCH-SYNC-01-report.md](../docs/architecture/TRD-ARCH-SYNC-01-report.md)

This index records which architecture sources are in the repository. `PRESENT` means the file bytes are in `contracts/sources/` and match the TRD-ARCH-SYNC-01 package manifest. `NOT_REVIEWED` means this commit does not accept the document as approved source text. `SOURCE_REVIEW_PENDING` means the file is a reconstructed draft and is not the missing original Q&A transcript. A cross-contract comparison is recorded in the report. That comparison is not approval.

Hashes and the independent status dimensions are in [source-manifest.json](source-manifest.json).

## Prior snapshot (commit 41c518830a4f90626a4fa4100b9b9347490f6ca8, 2026-11-14)

The following table is the index as it stood before TRD-ARCH-SYNC-01. It is retained so the earlier `NOT_PROVIDED` fact is not erased. It is not the current presence status.

| Id | What this repository knew on 2026-11-14 | Version | Draft status | Review |
| --- | --- | --- | --- | --- |
| DET-01 | Cited by CR-B10 and by the resolution as the order transition table and closure-reason vocabulary. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| DET-02 | Cited as reservation transitions. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| DET-03 | Named in the resolution basis line. Body not attached. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| DET-04 | Named in the resolution basis line. Body not attached. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| DET-05 | Named in the resolution basis line. Body not attached. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| DET-06 | Cited as readiness gates. Body not attached. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| DET-07 | Cited as dispatch outcomes. Body not attached. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| DET-08 | Named in the resolution basis line. Body not attached. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| DET-09 | No body, and the resolution did not add a separate citation. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| DET-10 | Cited as the incident transition model. Body not attached. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| CR-B01 | Named by CR-B10 as an overriding contract. Title not supplied. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| CR-B02 | Named by CR-B10 as an overriding contract. Title not supplied. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| CR-B03 | Named by CR-B10 as an overriding contract. Title not supplied. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| CR-B04 | Named by CR-B10 as an overriding contract. Title not supplied. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| CR-B05 | Named by CR-B10 as an overriding contract. Title not supplied. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| CR-B06 | Named by CR-B10 as an overriding contract. Title not supplied. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| CR-B07 | Named by CR-B10 as an overriding contract. Title not supplied. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| CR-B08 | Named by CR-B10 as an overriding contract. Title not supplied. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| CR-B09 | Named by CR-B10 as an overriding contract. Title not supplied. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| ADFR / ARCH-01C | Architecture Decision and Foundations Register. The body was not supplied. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| ADFR-CR-B10-001 | One entry inside CR-B10 §9. Status in that file: DESIGN_RESOLVED / TESTS_NOT_EXECUTED. | 1.0-draft, as part of CR-B10 | DESIGN_RESOLVED / TESTS_NOT_EXECUTED | Present only inside the CR-B10 file. Not the register. Not independently reviewed. |
| `Architecture-Consolidation-and-Acceptance-v1.0.md` | Named by CR-B10 §0. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| `P1-Resolution-Contracts-CR-B05-bis-CR-B09-v1.0.md` | Named by CR-B10 §0. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| `Implementation-Roadmap-and-Cursor-Prompt-Plan-v1.0.md` | Not attached. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |

CR-B10 and the M0-02A resolution were already present on that date. Their bytes were compared with the package and were identical. They were not overwritten.

## Current presence

| Id | Document | Section | Presence | Approval |
| --- | --- | --- | --- | --- |
| DET-01 | `DET-01-Order-Workflow-and-Finality-Contract-v1.0.md` | The whole file. Headings §0 through §10. §3 `OrderClosure` and §6 transition boundary are the closure and matrix limits. | PRESENT / RECONSTRUCTED_DRAFT / SOURCE_REVIEW_PENDING / NOT_REVIEWED | not approved |
| DET-02 | `DET-02-Risk-Contract-v1.0.md` | The whole file. Headings §0 through §10. §4 reservation atomicity, §5 release, and §6 FX/rounding are the risk sections used in the comparison. | PRESENT / RECONSTRUCTED_DRAFT / SOURCE_REVIEW_PENDING / NOT_REVIEWED | not approved |
| DET-03 | `DET-03-bis-DET-08-Architecture-Contracts-v1.0.md` | `## DET-03 — Broker Identity Contract` through the paragraph before `## DET-04 — Transaction & Outbox Contract`. Decisions DET-03-D01..D05. | PRESENT / NOT_REVIEWED | not approved |
| DET-04 | `DET-03-bis-DET-08-Architecture-Contracts-v1.0.md` | `## DET-04 — Transaction & Outbox Contract` through the paragraph before `## DET-05 — Broker Adapter Contract`. Decisions DET-04-D01..D05. | PRESENT / NOT_REVIEWED | not approved |
| DET-05 | `DET-03-bis-DET-08-Architecture-Contracts-v1.0.md` | `## DET-05 — Broker Adapter Contract` through the paragraph before `## DET-06 — Trading Readiness Contract`. Decisions DET-05-D01..D05. | PRESENT / NOT_REVIEWED | not approved |
| DET-06 | `DET-03-bis-DET-08-Architecture-Contracts-v1.0.md` | `## DET-06 — Trading Readiness Contract` through the paragraph before `## DET-07 — Submission & Execution Control Contract`. Decisions DET-06-D01..D05. | PRESENT / NOT_REVIEWED | not approved |
| DET-07 | `DET-03-bis-DET-08-Architecture-Contracts-v1.0.md` | `## DET-07 — Submission & Execution Control Contract` through the paragraph before `## DET-08 — Reconciliation & Recovery Contract`. Decisions DET-07-D01..D05. | PRESENT / NOT_REVIEWED | not approved |
| DET-08 | `DET-03-bis-DET-08-Architecture-Contracts-v1.0.md` | `## DET-08 — Reconciliation & Recovery Contract` through the paragraph before `## 7. Cross-Contract-Schnittstellen und Verantwortlichkeiten`. Decisions DET-08-D01..D05. | PRESENT / NOT_REVIEWED | not approved |
| DET-09 | `DET-09-bis-DET-10-und-Cross-Contract-Review-v1.0.md` | `# DET-09 — Market Data Integrity & Valuation Contract`, sections `## 9.1` through `## 9.7`. | PRESENT / NOT_REVIEWED | not approved |
| DET-10 | `DET-09-bis-DET-10-und-Cross-Contract-Review-v1.0.md` | `# DET-10 — Audit, Incident & Operational Control Contract`, sections `## 10.1` through `## 10.7`. | PRESENT / NOT_REVIEWED | not approved |
| CR-B01 | `P0-Resolution-Contracts-CR-B01-bis-CR-B04-v1.0.md` | `# 1. CR-B01 — Writer-Fencing, Broker-Session und Failover`, sections `## 1.1` through `## 1.5`. | PRESENT / NOT_REVIEWED | not approved |
| CR-B02 | `P0-Resolution-Contracts-CR-B01-bis-CR-B04-v1.0.md` | `# 2. CR-B02 — Gemeinsame PostgreSQL Unit of Work`, sections `## 2.1` through `## 2.5`. | PRESENT / NOT_REVIEWED | not approved |
| CR-B03 | `P0-Resolution-Contracts-CR-B01-bis-CR-B04-v1.0.md` | `# 3. CR-B03 — Late Fill, Bust und Risikokompensation`, sections `## 3.1` through `## 3.5`. | PRESENT / NOT_REVIEWED | not approved |
| CR-B04 | `P0-Resolution-Contracts-CR-B01-bis-CR-B04-v1.0.md` | `# 4. CR-B04 — IBKR Identity Contract und Verifikationsmatrix`, sections `## 4.1` through `## 4.6`. | PRESENT / NOT_REVIEWED | not approved |
| CR-B05 | `P1-Resolution-Contracts-CR-B05-bis-CR-B09-v1.0.md` | `## CR-B05 — Solo-Operator, Reset & Break-Glass`. | PRESENT / NOT_REVIEWED | not approved |
| CR-B06 | `P1-Resolution-Contracts-CR-B05-bis-CR-B09-v1.0.md` | `## CR-B06 — Instrument-, Bewertungs- & Rundungspolicy`. | PRESENT / NOT_REVIEWED | not approved |
| CR-B07 | `P1-Resolution-Contracts-CR-B05-bis-CR-B09-v1.0.md` | `## CR-B07 — Broker Snapshot Completeness`. | PRESENT / NOT_REVIEWED | not approved |
| CR-B08 | `P1-Resolution-Contracts-CR-B05-bis-CR-B09-v1.0.md` | `## CR-B08 — Audit-Atomizität & Export`. | PRESENT / NOT_REVIEWED | not approved |
| CR-B09 | `P1-Resolution-Contracts-CR-B05-bis-CR-B09-v1.0.md` | `## CR-B09 — Cancel, Modify & Risk Reduction Dispatch`. | PRESENT / NOT_REVIEWED | not approved |
| CR-B10 | `CR-B10-Domain-Glossary-and-Schema-Registry-v1.0.md` | The whole file, §0 through §10. Byte-identical to the pre-existing copy. | PRESENT / NOT_REVIEWED | not approved |
| ADFR | `ADFR-Consolidated-v1.0.json` and `ADFR-Consolidated-v1.0.csv` | JSON `entries`. CSV data rows. Embedded ADFR tables in the DET, P0, P1, and CR-B10 files are source tables, not a second accepted register. | PRESENT / NOT_REVIEWED | not approved |
| M0-02A UD-01..UD-09 | `M0-02A-UD-01-bis-UD-09-Architecture-Resolution-v1.0.md` | Headings `## UD-01` through `## UD-09`. Byte-identical to the pre-existing copy and to the prompt archive. | PRESENT / NOT_REVIEWED as a new approval. Prior local-test notes stay in `contracts/domain/unresolved-decisions.md`. | not re-approved by this index |

`Architecture-Consolidation-and-Acceptance-v1.0.md` and `Implementation-Roadmap-and-Cursor-Prompt-Plan-v1.0.md` are also present and `NOT_REVIEWED`. The roadmap is `PLAN_ONLY` inside its own file. This index does not execute it.

## ADFR count

Counted from the files, not from the prose claim of 77:

| Artifact | Count | What was checked |
| --- | --- | --- |
| `ADFR-Consolidated-v1.0.json` `entries` | 77 unique IDs | JSON parses. 20 `CONFIRMED_DECISION`, all of them DET-01/DET-02, source string `conversation: confirmed DET-01/02 Q01-Q10`. 57 `PROPOSED_FOR_ACCEPTANCE`. Every `verification` value is `NOT_VERIFIED`. |
| `ADFR-Consolidated-v1.0.csv` data rows | 78 unique IDs | CSV parses. The 77 shared IDs match the JSON fields `decision`, `source`, `status`, `basis`, and `verification`. |
| Extra CSV ID | `ADFR-CR-B10-001` | Present in the CSV and in CR-B10 §9. Absent from the JSON. CSV status `DESIGN_RESOLVED`. |
| Consolidation prose | says 78 | `## ADFR-Konsolidierung` says 78 unique IDs. The same bullet's breakdown is 10+10+22+22+8+5 = 77. |

The JSON count matches the stated 77. The CSV count does not. The consolidation sentence and its own breakdown do not match each other. None of these counts was edited. Parsing does not make the 77 entries correct. JSON and CSV are not claimed to be semantically equivalent. `ADFR-CROSS-005` remains `PROPOSED_FOR_ACCEPTANCE` in the JSON. CR-B10 §9 says `ADFR-CR-B10-001` supersedes it. That supersession is not applied here.

`CONFIRMED_DECISION` in the JSON cites the conversation, not the reconstructed DET-01/DET-02 markdown. Those markdown files stay `SOURCE_REVIEW_PENDING`.

## Broker evidence

Kind-specific uniqueness for `orderId`, `permId`, and `execId` is **EVIDENCE_PENDING**. `UnverifiedSessionScope` stays unverified. This index does not publish a uniqueness rule table. CR-B04 §4.2 labels its IBKR matrix as hypotheses, not broker guarantees.

## M0-02B split

Part (a), unchanged as a skeleton: the registry and its validator. See [contracts/registry/README.md](registry/README.md). Status remains `SKELETON_NOT_CANONICAL`, `part_b=NO-GO`, `EVIDENCE_PENDING`, `live_trading_authorized=false`.

Part (b), not recorded as done: canonical event and command schemas, golden bytes, and compatibility fixtures. Supplying DET and CR text does not publish those schemas.

## Shell records

[contracts/domain/glossary.md](domain/glossary.md) is the type inventory of the fail-closed shell. A presence note was added. The type rows were not rewritten from the new DET and CR bodies.

[contracts/domain/unresolved-decisions.md](domain/unresolved-decisions.md) still marks each of UD-01 through UD-09 **DESIGN_RESOLVED** and then either **IMPLEMENTED_AND_LOCALLY_TESTED** or **IMPLEMENTATION_PENDING** for the part that was delivered before this index. Broker uniqueness stays **EVIDENCE_PENDING**. This index does not change those UD statuses.

Open conflicts from the read-only comparison are in the TRD-ARCH-SYNC-01 report. They are not resolved by this index.
