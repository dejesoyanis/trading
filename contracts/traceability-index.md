# Traceability index

**Status:** index only. This file does not approve, test, or implement a contract.  
**Current date:** 2026-11-15  
**Live trading:** NOT AUTHORIZED  
**Comparison:** [TRD-ARCH-SYNC-01-report.md](../docs/architecture/TRD-ARCH-SYNC-01-report.md)  
**Ratification:** [ARCH-DEC-01-bis-03-2026-11-15.md](decisions/ARCH-DEC-01-bis-03-2026-11-15.md) and [TRD-ARCH-SYNC-02-report.md](../docs/architecture/TRD-ARCH-SYNC-02-report.md). Interpretation constraints: [ARCH-DEC-04-bis-08-2026-11-15.md](decisions/ARCH-DEC-04-bis-08-2026-11-15.md). OMS design policy only: [ARCH-DEC-09-bis-20-2026-11-15.md](decisions/ARCH-DEC-09-bis-20-2026-11-15.md). That policy record is not an implementation approval.
**Status model:** [contract-status-model-v1.0.md](status-model/contract-status-model-v1.0.md). Unassigned enums there are `PROPOSED_NOT_RATIFIED`. The model does not replace ADFR `status`, `verification`, or `lifecycle`. The historical questions are [approval-requests.md](status-model/approval-requests.md). SMQ-01 through SMQ-05 are DECIDED as Alternative 1 by ARCH-DEC-04 through ARCH-DEC-08. Those decisions do not close the underlying XC or UD issues. The reading records are [TRD-ARCH-03A-report.md](../docs/architecture/TRD-ARCH-03A-report.md) and [TRD-ARCH-03A.2-report.md](../docs/architecture/TRD-ARCH-03A.2-report.md).

This index records which architecture sources are in the repository. `source_present` / `PRESENT` means the file bytes are in `contracts/sources/`. For files that TRD-ARCH-SYNC-02 did not amend, those bytes still match the TRD-ARCH-SYNC-01 package hashes in [source-manifest.json](source-manifest.json). Amended files keep `package_sha256` for the intake bytes and a current `sha256` for the amended bytes. `provenance` is where the bytes came from. `NOT_REVIEWED` means this index does not accept the document as independently reviewed source text. `SOURCE_REVIEW_PENDING` means a reconstructed draft is not the missing original Q&A transcript. `design_baseline_approved` is a separate user ratification. It is not review, implementation, or verification. A cross-contract comparison is recorded in the TRD-ARCH-SYNC-01 report. That comparison is not approval.

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

| Id | Document | Section | source_present / provenance | Approval |
| --- | --- | --- | --- | --- |
| DET-01 | `DET-01-Order-Workflow-and-Finality-Contract-v1.0.md` | The whole file. Headings §0 through §10. §3 `OrderClosure` and §6 transition boundary are the closure and matrix limits. | PRESENT / RECONSTRUCTED_DRAFT / SOURCE_REVIEW_PENDING / NOT_REVIEWED. design_baseline_approved: DESIGN_BASELINE_RATIFIED_WITH_EXCEPTIONS (ARCH-DEC-01, 2026-11-15). | not approved as original Q&A or as implementation. Design baseline only, with exceptions. |
| DET-02 | `DET-02-Risk-Contract-v1.0.md` | The whole file. Headings §0 through §10. §4 reservation atomicity, §5 release, and §6 FX/rounding are the risk sections used in the comparison. | PRESENT / RECONSTRUCTED_DRAFT / SOURCE_REVIEW_PENDING / NOT_REVIEWED. design_baseline_approved: DESIGN_BASELINE_RATIFIED_WITH_EXCEPTIONS (ARCH-DEC-01, 2026-11-15). | not approved as original Q&A or as implementation. Design baseline only, with exceptions. |
| DET-03 | `DET-03-bis-DET-08-Architecture-Contracts-v1.0.md` | `## DET-03 — Broker Identity Contract` through the paragraph before `## DET-04 — Transaction & Outbox Contract`. Decisions DET-03-D01..D05. | PRESENT / NOT_REVIEWED | not approved |
| DET-04 | `DET-03-bis-DET-08-Architecture-Contracts-v1.0.md` | `## DET-04 — Transaction & Outbox Contract` through the paragraph before `## DET-05 — Broker Adapter Contract`. Decisions DET-04-D01..D05. | PRESENT / NOT_REVIEWED | not approved |
| DET-05 | `DET-03-bis-DET-08-Architecture-Contracts-v1.0.md` | `## DET-05 — Broker Adapter Contract` through the paragraph before `## DET-06 — Trading Readiness Contract`. Decisions DET-05-D01..D05. | PRESENT / NOT_REVIEWED | not approved |
| DET-06 | `DET-03-bis-DET-08-Architecture-Contracts-v1.0.md` | `## DET-06 — Trading Readiness Contract` through the paragraph before `## DET-07 — Submission & Execution Control Contract`. Decisions DET-06-D01..D05. | PRESENT / NOT_REVIEWED | not approved |
| DET-07 | `DET-03-bis-DET-08-Architecture-Contracts-v1.0.md` | `## DET-07 — Submission & Execution Control Contract` through the paragraph before `## DET-08 — Reconciliation & Recovery Contract`. Decisions DET-07-D01..D05. | PRESENT / NOT_REVIEWED. Dispatch precedence where it conflicts with CR-B01: see ARCH-DEC-03 below. | not approved |
| DET-08 | `DET-03-bis-DET-08-Architecture-Contracts-v1.0.md` | `## DET-08 — Reconciliation & Recovery Contract` through the paragraph before `## 7. Cross-Contract-Schnittstellen und Verantwortlichkeiten`. Decisions DET-08-D01..D05. | PRESENT / NOT_REVIEWED | not approved |
| DET-09 | `DET-09-bis-DET-10-und-Cross-Contract-Review-v1.0.md` | `# DET-09 — Market Data Integrity & Valuation Contract`, sections `## 9.1` through `## 9.7`. | PRESENT / NOT_REVIEWED | not approved |
| DET-10 | `DET-09-bis-DET-10-und-Cross-Contract-Review-v1.0.md` | `# DET-10 — Audit, Incident & Operational Control Contract`, sections `## 10.1` through `## 10.7`. | PRESENT / NOT_REVIEWED | not approved |
| CR-B01 | `P0-Resolution-Contracts-CR-B01-bis-CR-B04-v1.0.md` | `# 1. CR-B01 — Writer-Fencing, Broker-Session und Failover`, sections `## 1.1` through `## 1.5`. | PRESENT / NOT_REVIEWED. Design precedence over conflicting or less restrictive DET-07 text: ARCH-DEC-03. | not approved as implementation or as IBKR evidence |
| CR-B02 | `P0-Resolution-Contracts-CR-B01-bis-CR-B04-v1.0.md` | `# 2. CR-B02 — Gemeinsame PostgreSQL Unit of Work`, sections `## 2.1` through `## 2.5`. | PRESENT / NOT_REVIEWED | not approved |
| CR-B03 | `P0-Resolution-Contracts-CR-B01-bis-CR-B04-v1.0.md` | `# 3. CR-B03 — Late Fill, Bust und Risikokompensation`, sections `## 3.1` through `## 3.5`. | PRESENT / NOT_REVIEWED | not approved |
| CR-B04 | `P0-Resolution-Contracts-CR-B01-bis-CR-B04-v1.0.md` | `# 4. CR-B04 — IBKR Identity Contract und Verifikationsmatrix`, sections `## 4.1` through `## 4.6`. | PRESENT / NOT_REVIEWED | not approved |
| CR-B05 | `P1-Resolution-Contracts-CR-B05-bis-CR-B09-v1.0.md` | `## CR-B05 — Solo-Operator, Reset & Break-Glass`. | PRESENT / NOT_REVIEWED | not approved |
| CR-B06 | `P1-Resolution-Contracts-CR-B05-bis-CR-B09-v1.0.md` | `## CR-B06 — Instrument-, Bewertungs- & Rundungspolicy`. | PRESENT / NOT_REVIEWED | not approved |
| CR-B07 | `P1-Resolution-Contracts-CR-B05-bis-CR-B09-v1.0.md` | `## CR-B07 — Broker Snapshot Completeness`. | PRESENT / NOT_REVIEWED | not approved |
| CR-B08 | `P1-Resolution-Contracts-CR-B05-bis-CR-B09-v1.0.md` | `## CR-B08 — Audit-Atomizität & Export`. | PRESENT / NOT_REVIEWED | not approved |
| CR-B09 | `P1-Resolution-Contracts-CR-B05-bis-CR-B09-v1.0.md` | `## CR-B09 — Cancel, Modify & Risk Reduction Dispatch`. | PRESENT / NOT_REVIEWED | not approved |
| CR-B10 | `CR-B10-Domain-Glossary-and-Schema-Registry-v1.0.md` | The whole file, §0 through §10. Byte-identical to the pre-existing copy. §9 is the prose source for `ADFR-CR-B10-001`. | PRESENT / NOT_REVIEWED | not approved |
| ADFR | `ADFR-Consolidated-v1.0.json` and `ADFR-Consolidated-v1.0.csv` | JSON `entries`. CSV data rows. Embedded ADFR tables in the DET, P0, P1, and CR-B10 files are source tables, not a second accepted register. | PRESENT / NOT_REVIEWED. Amended 2026-11-15 by ARCH-DEC-02. See the current count below. | not approved |
| M0-02A UD-01..UD-09 | `M0-02A-UD-01-bis-UD-09-Architecture-Resolution-v1.0.md` | Headings `## UD-01` through `## UD-09`. Byte-identical to the pre-existing copy and to the prompt archive. | PRESENT / NOT_REVIEWED as a new approval. Prior local-test notes stay in `contracts/domain/unresolved-decisions.md`. | not re-approved by this index |
| ARCH-DEC-01..03 | `contracts/decisions/ARCH-DEC-01-bis-03-2026-11-15.md` | The whole file. | PRESENT as a user decision record dated 2026-11-15. Not a ZIP source. provenance is the user decision, not the source package. | recorded. Not an implementation approval. |
| ARCH-DEC-04..08 | `contracts/decisions/ARCH-DEC-04-bis-08-2026-11-15.md` | The whole file. SMQ-01 through SMQ-05, each Alternative 1. | PRESENT as a user decision record dated 2026-11-15. Not a ZIP source. Interpretation constraints only. | recorded. Not an implementation approval and not a verification promotion. |
| ARCH-DEC-09..20 | `contracts/decisions/ARCH-DEC-09-bis-20-2026-11-15.md` | The whole file. OMSQ-01 through OMSQ-12. Options A, C, A, A, B, A, A, A, A, A, A, A. | PRESENT as a user decision record dated 2026-11-15. Not a ZIP source. Design policy only. `NOT_IMPLEMENTED`. `NOT_VERIFIED`. | recorded. Not an `OrderClosure` vocabulary, not a transition edge, and not a verification promotion. |

`Architecture-Consolidation-and-Acceptance-v1.0.md` and `Implementation-Roadmap-and-Cursor-Prompt-Plan-v1.0.md` are also present and `NOT_REVIEWED`. The consolidation file keeps its 2026-11-14 wording and adds a dated 2026-11-15 addendum. The roadmap is `PLAN_ONLY` inside its own file. This index does not execute it.

## ADFR count before ARCH-DEC-02 (TRD-ARCH-SYNC-01 snapshot)

This section is the count recorded before the 2026-11-15 reconciliation. It is not the current count.

Counted from the files as they stood after TRD-ARCH-SYNC-01, not from the prose claim of 77:

| Artifact | Count | What was checked |
| --- | --- | --- |
| `ADFR-Consolidated-v1.0.json` `entries` | 77 unique IDs | JSON parses. 20 `CONFIRMED_DECISION`, all of them DET-01/DET-02, source string `conversation: confirmed DET-01/02 Q01-Q10`. 57 `PROPOSED_FOR_ACCEPTANCE`. Every `verification` value is `NOT_VERIFIED`. |
| `ADFR-Consolidated-v1.0.csv` data rows | 78 unique IDs | CSV parses. The 77 shared IDs match the JSON fields `decision`, `source`, `status`, `basis`, and `verification`. |
| Extra CSV ID | `ADFR-CR-B10-001` | Present in the CSV and in CR-B10 §9. Absent from the JSON. CSV status `DESIGN_RESOLVED`. |
| Consolidation prose | says 78 | `## ADFR-Konsolidierung` says 78 unique IDs. The same bullet's breakdown is 10+10+22+22+8+5 = 77. |

At that snapshot, the JSON count matched the stated 77 and the CSV count did not. `ADFR-CROSS-005` was `PROPOSED_FOR_ACCEPTANCE` in the JSON. CR-B10 §9 said `ADFR-CR-B10-001` supersedes it. That supersession was not applied in TRD-ARCH-SYNC-01.

`CONFIRMED_DECISION` in the JSON cites the conversation, not the reconstructed DET-01/DET-02 markdown. Those markdown files stay `SOURCE_REVIEW_PENDING`.

## ADFR count (current, after ARCH-DEC-02, 2026-11-15)

| Artifact | Count | What this index claims |
| --- | --- | --- |
| JSON `entries` and CSV data rows | 78 unique historical IDs, same set | Field parity on `id`, `decision`, `source`, `status`, `basis`, and `verification`. |
| Superseded | 1: `ADFR-CROSS-005` | lifecycle `SUPERSEDED`. `superseded_by` is `ADFR-CR-B10-001`. Core `status` remains `PROPOSED_FOR_ACCEPTANCE`. `verification` remains `NOT_VERIFIED`. |
| Successor | `ADFR-CR-B10-001` | lifecycle `NOT_SUPERSEDED`. `supersedes` is `ADFR-CROSS-005`. Core `status` remains `DESIGN_RESOLVED`. `verification` remains `NOT_VERIFIED`. |
| Not superseded | 77 | Count only. Not acceptance, implementation, or verification. |

JSON root `status` remains `ARCHITECTURE_DRAFT_NOT_IMPLEMENTED`. Every `verification` value remains `NOT_VERIFIED`. The twenty `CONFIRMED_DECISION` rows keep the source string `conversation: confirmed DET-01/02 Q01-Q10`. ARCH-DEC-01 does not rewrite that provenance. The older status words `OPEN P0`, `PROPOSED_FOR_ACCEPTANCE`, and `DESIGN_RESOLVED` are not reconciled with each other. That part of XC-11 stays OPEN. The population and supersession part is `RESOLVED_BY_ARCH_DEC_02`.

## Design baseline dimensions (ARCH-DEC-01)

| Dimension | DET-01 | DET-02 |
| --- | --- | --- |
| source_present | PRESENT | PRESENT |
| provenance | RECONSTRUCTED_DRAFT | RECONSTRUCTED_DRAFT |
| design_baseline_approved | DESIGN_BASELINE_RATIFIED_WITH_EXCEPTIONS | DESIGN_BASELINE_RATIFIED_WITH_EXCEPTIONS |
| details_open | yes. OrderClosure reasons, transition matrix, reference DDL, and every file-marked open or unconfirmed detail stay open. Q01–Q10 verification stays OPEN. | yes. Reference policy names, the reference inequality, rounding, and every file-marked open detail stay open. Q01–Q10 verification stays OPEN. |
| implementation | not implemented | not implemented |
| verification | not verified | not verified |

The baseline does not claim that the original Q&A transcripts are present or independently verified. It does not release economic finality, risk postings, or technical execution.

## Dispatch precedence (ARCH-DEC-03)

| Question | Status |
| --- | --- |
| Which reading has design precedence where DET-07 and CR-B01 conflict, or where DET-07 is less restrictive? | `RESOLVED_BY_ARCH_DEC_03`. CR-B01 in `P0-Resolution-Contracts-CR-B01-bis-CR-B04-v1.0.md` has precedence over DET-07 in `DET-03-bis-DET-08-Architecture-Contracts-v1.0.md`. |
| Automatic takeover while the previous writer might still send, or while submission results are unresolved | Design invariant: no automatic takeover. Not implemented. |
| Lease timeout or elapsed time alone as proof of safe takeover | Design invariant: not proof. Not implemented. |
| In-flight or unknown submission result | Design invariant: conservative block. Not a runtime change. |
| Broker-session exclusivity, lease and heartbeat protocol, dispatch-scope boundary, in-flight send evidence, recovery tests, IBKR behavior | OPEN. XC-04 remains OPEN apart from the precedence sub-finding. |

No default TTL, automatic resume, time-based failover, or new dispatch state transition is stated here. ARCH-DEC-03 claims no technical verification.

## Broker evidence

Kind-specific uniqueness for `orderId`, `permId`, and `execId` is **EVIDENCE_PENDING**. `UnverifiedSessionScope` stays unverified. This index does not publish a uniqueness rule table. CR-B04 §4.2 labels its IBKR matrix as hypotheses, not broker guarantees.

## M0-02B split

Part (a), unchanged as a skeleton: the registry and its validator. See [contracts/registry/README.md](registry/README.md). Status remains `SKELETON_NOT_CANONICAL`, `part_b=NO-GO`, `EVIDENCE_PENDING`, `live_trading_authorized=false`.

Part (b), not recorded as done: canonical event and command schemas, golden bytes, and compatibility fixtures. Supplying DET and CR text, and ratifying a design baseline with exceptions, does not publish those schemas.

## Shell records

[contracts/domain/glossary.md](domain/glossary.md) is the type inventory of the fail-closed shell. A presence note was added by TRD-ARCH-SYNC-01. The type rows were not rewritten from the DET and CR bodies, and TRD-ARCH-SYNC-02 does not rewrite them.

[contracts/domain/unresolved-decisions.md](domain/unresolved-decisions.md) still marks each of UD-01 through UD-09 **DESIGN_RESOLVED** and then either **IMPLEMENTED_AND_LOCALLY_TESTED** or **IMPLEMENTATION_PENDING** for the part that was delivered before this index. Broker uniqueness stays **EVIDENCE_PENDING**. This index does not change those UD statuses.

XC-01, XC-02-PRED, XC-03-MATRIX, XC-05-LOCK, XC-06, XC-07-ROUND, XC-08-IBKR, XC-09-RESET, and XC-10-ACTION stay OPEN. The closed sub-findings are only the ones named in this file: register population and supersession (`RESOLVED_BY_ARCH_DEC_02`) and dispatch-reading precedence (`RESOLVED_BY_ARCH_DEC_03`).

## Status vocabulary (TRD-ARCH-03A)

[contract-status-model-v1.0.md](status-model/contract-status-model-v1.0.md) separates provenance, decision lifecycle, implementation, verification, safety authorization, severity, and supersession. [legacy-status-mapping.md](status-model/legacy-status-mapping.md) keeps each historical literal. The machine-readable file [status-dimensions-draft-v0.json](status-model/status-dimensions-draft-v0.json) is a non-runtime, non-wire draft. Its interpretation layer is `PROPOSED_NOT_RATIFIED`. It is not `registry.json` and it does not publish canonical schemas.

The XC-11 status-word disagreement stays OPEN. ARCH-DEC-04 records that there is no winning status and no default precedence. `OPEN P0` is not equated with `PROPOSED_FOR_ACCEPTANCE` or `DESIGN_RESOLVED`. No precedence is not equivalence. DET-01 and DET-02 stay `RECONSTRUCTED_DRAFT` and `DESIGN_BASELINE_RATIFIED_WITH_EXCEPTIONS`. Their Q01–Q10 verification cells stay OPEN. SMQ-01 through SMQ-05 are DECIDED as Alternative 1. The closed lifecycle enum stays unratified. ADFR `verification` stays `NOT_VERIFIED`. Manifest placement is unchanged.

## OMS finality proposal (TRD-ARCH-03B.1)

[contracts/design-proposals/README.md](design-proposals/README.md) indexes a review package dated 2026-11-15. As written, that package is `NOT_RATIFIED` and asks OMSQ-01 through OMSQ-12 with creation-time status `USER_DECISION_PENDING`. Those proposal lines are not rewritten. `XC-03-MATRIX` stays OPEN. No transition edge is approved. No `OrderClosure` vocabulary is approved. The package does not change ARCH-DEC-01 through ARCH-DEC-08, the ADFR rows, the source manifest, or the registry flags. ARCH-03B.2 and ARCH-03B.3 are named as later dependencies and are not started. The reading record is [TRD-ARCH-03B.1-report.md](../docs/architecture/TRD-ARCH-03B.1-report.md).

## OMS design policy (TRD-ARCH-03B.1.2)

[ARCH-DEC-09-bis-20-2026-11-15.md](decisions/ARCH-DEC-09-bis-20-2026-11-15.md) records the user policy choices of 2026-11-15. ARCH-DEC-09 is OMSQ-01 option A. ARCH-DEC-10 is OMSQ-02 option C. ARCH-DEC-11 is OMSQ-03 option A. ARCH-DEC-12 is OMSQ-04 option A. ARCH-DEC-13 is OMSQ-05 option B. ARCH-DEC-14 is OMSQ-06 option A. ARCH-DEC-15 is OMSQ-07 option A. ARCH-DEC-16 is OMSQ-08 option A. ARCH-DEC-17 is OMSQ-09 option A. ARCH-DEC-18 is OMSQ-10 option A. ARCH-DEC-19 is OMSQ-11 option A. ARCH-DEC-20 is OMSQ-12 option A. The record is `RATIFIED_DESIGN_POLICY_ONLY`, `NOT_IMPLEMENTED`, and `NOT_VERIFIED`. Those words are documentary labels, not a canonical enum.

The choices do not approve an `OrderClosure` enum, a token list, a writer, a transition edge, a matrix version, an economic-finality predicate, automatic economic posting, risk release, broker terminal acceptance, modify or replace dispatch, a canonical lock order, an isolation level, or an audit table. OMSQ-05 option B overrides automatic posting of a single identity match. OMSQ-06 option A leaves risk release forbidden. OMSQ-02 option C is strategy only. OMSQ-09 option A accepts no terminal broker observation. `XC-03-MATRIX` stays OPEN. XC-01, XC-02-PRED, XC-05-LOCK, XC-06, XC-07-ROUND, XC-08-IBKR, XC-09-RESET, and XC-10-ACTION stay OPEN. XC-04 stays OPEN apart from `RESOLVED_BY_ARCH_DEC_03`. XC-11 status words stay OPEN apart from `RESOLVED_BY_ARCH_DEC_02`. CR-B01 precedence stays in force. DET-01 and DET-02 Q01–Q10 verification cells stay OPEN. The reading record is [TRD-ARCH-03B.1.2-report.md](../docs/architecture/TRD-ARCH-03B.1.2-report.md).

## Risk reservation proposal (TRD-ARCH-03B.2)

[contracts/design-proposals/README.md](design-proposals/README.md) indexes a second review package dated 2026-11-15. As written, that package is `NOT_RATIFIED` and asks RISKQ-01 through RISKQ-14 with status `USER_DECISION_PENDING`. A recommendation in that package is not a decision. `XC-02-PRED` stays OPEN. No economic-finality predicate, no economic posting, and no reservation release are approved. The package does not change ARCH-DEC-01 through ARCH-DEC-20, the ADFR rows, the source manifest, the registry flags, or the TRD-ARCH-03B.1 proposal snapshot. ARCH-03B.3 is not started. OMS transitions stay unapproved. The reading record is [TRD-ARCH-03B.2-report.md](../docs/architecture/TRD-ARCH-03B.2-report.md).

## Risk design policy (TRD-ARCH-03B.2.2)

[ARCH-DEC-21-bis-34-2026-11-15.md](decisions/ARCH-DEC-21-bis-34-2026-11-15.md) records the user policy choices of 2026-11-15. ARCH-DEC-21 is RISKQ-01 option B. ARCH-DEC-22 is RISKQ-02 option A. ARCH-DEC-23 is RISKQ-03 option A. ARCH-DEC-24 is RISKQ-04 option A. ARCH-DEC-25 is RISKQ-05 option A. ARCH-DEC-26 is RISKQ-06 option A. ARCH-DEC-27 is RISKQ-07 option A. ARCH-DEC-28 is RISKQ-08 option A. ARCH-DEC-29 is RISKQ-09 option A. ARCH-DEC-30 is RISKQ-10 option B. ARCH-DEC-31 is RISKQ-11 option B. ARCH-DEC-32 is RISKQ-12 option A. ARCH-DEC-33 is RISKQ-13 option A. ARCH-DEC-34 is RISKQ-14 option B. The record is `RATIFIED_DESIGN_POLICY_ONLY`, `NOT_IMPLEMENTED`, and `NOT_VERIFIED`. Those words are documentary labels, not a canonical enum. **NO IMPLEMENTATION.** **NO VERIFICATION.**

The historical RISKQ request and [TRD-ARCH-03B2-proposal-index.json](design-proposals/TRD-ARCH-03B2-proposal-index.json) stay the creation-time `NOT_RATIFIED` snapshot. The later map is [TRD-ARCH-03B22-resolution-index.json](design-proposals/TRD-ARCH-03B22-resolution-index.json), role `NON_RUNTIME_NON_WIRE_RESOLUTION_INDEX`.

RISKQ-01 option B is hard-deny conjunction shape only and adopts no arithmetic. RISKQ-02 option A is one reservation identity and all-or-nothing scopes, with no writer. RISKQ-03 option A makes no reservation on missing or stale inputs. RISKQ-04 option A, RISKQ-05 option A, and RISKQ-06 option A retain evidence and do not convert, shrink, or release a hold. RISKQ-07 option A denies new risk-increasing commands and authorizes no broker action. RISKQ-08 option A and RISKQ-09 option A adopt no evidence bar and no release proof. RISKQ-10 option B is seven citation obligations and not a rounding or FX approval. RISKQ-11 option B requires future concurrency and crash tests and marks them `NOT_EXECUTED`. RISKQ-12 option A repeats the unnamed audit record and does not close `XC-06`. RISKQ-13 option A adds no role and cannot supersede ARCH-DEC-13. RISKQ-14 option B requires immutable future policy versions and does not authorize a migration, a release, or a resubmit.

ARCH-DEC-13 and ARCH-DEC-14 stay binding. `XC-02-PRED` stays OPEN. XC-01, XC-03-MATRIX, XC-05-LOCK, XC-06, XC-07-ROUND, XC-08-IBKR, XC-09-RESET, and XC-10-ACTION stay OPEN. XC-04 stays OPEN apart from `RESOLVED_BY_ARCH_DEC_03`. XC-11 status words stay OPEN apart from `RESOLVED_BY_ARCH_DEC_02`. CR-B01 precedence stays in force. DET-01 and DET-02 stay `RECONSTRUCTED_DRAFT` and `SOURCE_REVIEW_PENDING`. The reading record is [TRD-ARCH-03B.2.2-report.md](../docs/architecture/TRD-ARCH-03B.2.2-report.md). ARCH-03B.3 is not started.

## Broker identity proposal (TRD-ARCH-03B.3)

[contracts/design-proposals/README.md](design-proposals/README.md) indexes a third review package dated 2026-11-15. As written, that package is `NOT_RATIFIED` and asks BROKERQ-01 through BROKERQ-14 with status `USER_DECISION_PENDING`. A recommendation in that package is not a decision. `XC-01` stays OPEN. `XC-08-IBKR` stays OPEN. No canonical broker identity tuple and no uniqueness index are approved. No economic-finality predicate, no economic posting, and no reservation release are approved. The package does not change ARCH-DEC-01 through ARCH-DEC-34, the ADFR rows, the source manifest, the registry flags, or the earlier proposal indexes. OMS transitions stay unapproved. Broker dispatch stays unapproved. XC-04 stays OPEN apart from `RESOLVED_BY_ARCH_DEC_03`. The reading record is [TRD-ARCH-03B.3-report.md](../docs/architecture/TRD-ARCH-03B.3-report.md). The sentence above that says ARCH-03B.3 is not started belongs to the TRD-ARCH-03B.2.2 reading. Live trading stays NOT AUTHORIZED.
