# Legacy status mapping

**Role:** source-grounded reading of status phrases already present in the repository. This file does not rewrite those sources and does not choose a winner where they disagree.
**Model:** [contract-status-model-v1.0.md](contract-status-model-v1.0.md)
**Machine companion:** [status-dimensions-draft-v0.json](status-dimensions-draft-v0.json). Phrase rows below match that draft. The draft is PROPOSED_NOT_RATIFIED as an interpretation layer.
**Date:** 2026-11-15
**Live trading:** NOT AUTHORIZED

DEFINITIVE means the home-field reading and the forbidden implications are fixed by an existing source or by ARCH-DEC-01, ARCH-DEC-02, or ARCH-DEC-03. PROVISIONAL means a later document explains the cell without rewriting it. REQUIRES_USER_DECISION means two readings remain possible and this file does not pick one. See [approval-requests.md](approval-requests.md).

## Readings that stay explicit

**XC-11 status words.** contracts/sources/DET-09-bis-DET-10-und-Cross-Contract-Review-v1.0.md section ## ADFR-CROSS says OPEN P0 for ADFR-CROSS-001 through ADFR-CROSS-004, OPEN P2 for ADFR-CROSS-005, and OPEN P1 for ADFR-CROSS-006. The ADFR JSON and CSV say PROPOSED_FOR_ACCEPTANCE and NOT_VERIFIED for those rows, and DESIGN_RESOLVED / NOT_VERIFIED for ADFR-CR-B10-001. The P0 file says DESIGN_RESOLVED and P0_DESIGN_RESOLVED. The P1 file says P1_DESIGN_RESOLVED. The 2026-11-14 consolidation sentence that calls OPEN P0 a historical cross-reference is still in that file. ARCH-DEC-02 says which of OPEN P0, PROPOSED_FOR_ACCEPTANCE, and DESIGN_RESOLVED wins was not decided. Those equivalences are REQUIRES_USER_DECISION. The original sentences are not edited to agree.

**ARCH-DEC-01.** DET-01 and DET-02 are DESIGN_BASELINE_RATIFIED_WITH_EXCEPTIONS. Provenance remains RECONSTRUCTED_DRAFT. Q01-Q10 verification cells remain OPEN. The twenty CONFIRMED_DECISION rows are not relabeled and are not marked verified.

**ARCH-DEC-03.** CR-B01 has design precedence over conflicting or less restrictive DET-07 text. That precedence does not implement fencing, does not prove IBKR behavior, and does not authorize automatic takeover.

**Four different coordinates.** DESIGN_RESOLVED is not verified, not implemented, and not live-authorized. The register verification literal is NOT_VERIFIED. Live trading remains live_trading_authorized false.

**NO_CONFLICT_FOUND.** Agreement on a fail-closed principle in the TRD-ARCH-SYNC-01 comparison is not permission to implement an unresolved predicate or transition. XC-03-MATRIX stays open. XC-02-PRED stays open.

## Phrase index

| Phrase | Context | Mapping |
| --- | --- | --- |
| OPEN P0 | DET-09/10 ADFR-CROSS-001..004 | REQUIRES_USER_DECISION |
| OPEN P1 | DET-09/10 ADFR-CROSS-006 | REQUIRES_USER_DECISION |
| OPEN P2 | DET-09/10 ADFR-CROSS-005 cell | REQUIRES_USER_DECISION |
| PROPOSED_FOR_ACCEPTANCE | ADFR status field, 57 rows | REQUIRES_USER_DECISION |
| CONFIRMED_DECISION | ADFR status field, 20 DET-01/02 rows | DEFINITIVE |
| DESIGN_RESOLVED | source self-label and one ADFR row | REQUIRES_USER_DECISION |
| DESIGN_BASELINE_RATIFIED_WITH_EXCEPTIONS | DET-01 and DET-02 manifest design_baseline | DEFINITIVE |
| RECONSTRUCTED_DRAFT | DET-01 and DET-02 provenance | DEFINITIVE |
| NOT_VERIFIED | ADFR verification, all 78 rows | DEFINITIVE |
| EVIDENCE_PENDING | broker uniqueness | DEFINITIVE |
| SUPERSEDED | lifecycle of ADFR-CROSS-005 | DEFINITIVE |
| NOT_SUPERSEDED | lifecycle of the other 77 IDs | DEFINITIVE |
| NOT_IMPLEMENTED | named registry rows | DEFINITIVE |
| SKELETON_NOT_CANONICAL | registry_status | DEFINITIVE |
| NO-GO | registry part_b | DEFINITIVE |
| NO_CONFLICT_FOUND | TRD-ARCH-SYNC-01 principle rows | DEFINITIVE |
| ARCHITECTURE_DRAFT_NOT_IMPLEMENTED | ADFR JSON root status | DEFINITIVE |
| TESTS_NOT_EXECUTED | CR-B10 section 9 and consolidation header | DEFINITIVE |
| IMPLEMENTED_AND_LOCALLY_TESTED | UD delivered type-shell part | REQUIRES_USER_DECISION |
| IMPLEMENTATION_PENDING | UD undelivered remainder | DEFINITIVE |
| P0_DESIGN_RESOLVED | P0 file Gesamtstatus | REQUIRES_USER_DECISION |
| P1_DESIGN_RESOLVED | P1 file section 0 | REQUIRES_USER_DECISION |
| DESIGN_RECONSTRUCTED_FROM_CONFIRMED_DECISIONS | DET-01 and DET-02 header | REQUIRES_USER_DECISION |
| SOURCE_REVIEW_PENDING | DET-01 and DET-02 source_text_status | DEFINITIVE |
| NOT_REVIEWED | index and manifest source-text review | DEFINITIVE |
| INTERNAL_DRAFT_SHELL | registry entry status | DEFINITIVE |
| BROKER_SEMANTICS_UNVERIFIED | CR-B04 cell and M0-02A header | DEFINITIVE |
| EXTERNAL_VERIFICATION_REQUIRED | GATE-P1-07 and GATE-P1-09 | DEFINITIVE |
| DESIGN_ONLY | GATE-P1-05, GATE-P1-06, GATE-P1-08 | DEFINITIVE |
| LIVE_NOT_AUTHORIZED | source headers | DEFINITIVE |
| LIVE_TRADING_NOT_AUTHORIZED | P0 Gesamtstatus | DEFINITIVE |
| IMPLEMENTATION_NOT_VERIFIED | DET-01, DET-02, P0, and M0-02A headers | DEFINITIVE |
| PLAN_ONLY | implementation roadmap | DEFINITIVE |
| IMPLEMENTATION_NOT_AUTHORIZED | implementation roadmap header | DEFINITIVE |
| NOT_READY_FOR_LIVE_SUBMISSION | P0 ADFR-P0-008 consequence | DEFINITIVE |
| RESOLVED_BY_ARCH_DEC_02 | XC-11 population and supersession only | DEFINITIVE |
| RESOLVED_BY_ARCH_DEC_03 | XC-04 design precedence only | DEFINITIVE |
| NOT_IMPLEMENTATION_APPROVED | DET-09/10 endstatus | DEFINITIVE |
| NOT AUTHORIZED | live-trading prose | DEFINITIVE |
| OPEN | TRD-ARCH-SYNC-01 comparison cell | PROVISIONAL |
| OPEN | DET-01/02 Q01-Q10 verification cell | DEFINITIVE |

## Phrase readings

### OPEN P0 (DET-09/10 ADFR-CROSS-001..004) — REQUIRES_USER_DECISION

- **Historical literal:** OPEN P0
- **Dimensions:** severity, historical_composite
- **Sources:** contracts/sources/DET-09-bis-DET-10-und-Cross-Contract-Review-v1.0.md
- **Safe interpretation:** Historical table cell. P0 is priority. The cell is not the ADFR status field.
- **Does not imply:** PROPOSED_FOR_ACCEPTANCE, DESIGN_RESOLVED, verified, implemented, ratified

### OPEN P1 (DET-09/10 ADFR-CROSS-006) — REQUIRES_USER_DECISION

- **Historical literal:** OPEN P1
- **Dimensions:** severity, historical_composite
- **Sources:** contracts/sources/DET-09-bis-DET-10-und-Cross-Contract-Review-v1.0.md
- **Safe interpretation:** Historical table cell for ADFR-CROSS-006. P1 is priority.
- **Does not imply:** PROPOSED_FOR_ACCEPTANCE, DESIGN_RESOLVED, verified, implemented

### OPEN P2 (DET-09/10 ADFR-CROSS-005 cell) — REQUIRES_USER_DECISION

- **Historical literal:** OPEN P2
- **Dimensions:** severity, historical_composite
- **Sources:** contracts/sources/DET-09-bis-DET-10-und-Cross-Contract-Review-v1.0.md
- **Safe interpretation:** Historical table cell left in place. Register lifecycle SUPERSEDED does not rewrite this cell.
- **Does not imply:** verified, deleted, status_rewritten

### PROPOSED_FOR_ACCEPTANCE (ADFR status field, 57 rows) — REQUIRES_USER_DECISION

- **Historical literal:** PROPOSED_FOR_ACCEPTANCE
- **Dimensions:** historical_status_word
- **Sources:** contracts/sources/ADFR-Consolidated-v1.0.json
- **Safe interpretation:** Retained ADFR status word, including on superseded ADFR-CROSS-005. Not a ratified lifecycle enum.
- **Does not imply:** DESIGN_RESOLVED, OPEN P0, verified, implemented, accepted

### CONFIRMED_DECISION (ADFR status field, 20 DET-01/02 rows) — DEFINITIVE

- **Historical literal:** CONFIRMED_DECISION
- **Dimensions:** historical_status_word
- **Sources:** contracts/sources/ADFR-Consolidated-v1.0.json; contracts/decisions/ARCH-DEC-01-bis-03-2026-11-15.md
- **Safe interpretation:** Retained status for rows whose source string is conversation: confirmed DET-01/02 Q01-Q10. ARCH-DEC-01 does not reclassify these rows as verified and does not relabel them as the file baseline.
- **Does not imply:** verified, implemented, original_qa_transcript, DESIGN_BASELINE_RATIFIED_WITH_EXCEPTIONS

### DESIGN_RESOLVED (source self-label and one ADFR row) — REQUIRES_USER_DECISION

- **Historical literal:** DESIGN_RESOLVED
- **Dimensions:** historical_status_word
- **Sources:** contracts/sources/ADFR-Consolidated-v1.0.json; contracts/sources/CR-B10-Domain-Glossary-and-Schema-Registry-v1.0.md; contracts/sources/P0-Resolution-Contracts-CR-B01-bis-CR-B04-v1.0.md
- **Safe interpretation:** A source's claim that a design text exists. ARCH-DEC-02 left its relationship to OPEN P0 and PROPOSED_FOR_ACCEPTANCE undecided. On ADFR-CR-B10-001 the status field is this word and verification is NOT_VERIFIED.
- **Does not imply:** verified, implemented, live_authorized, PROPOSED_FOR_ACCEPTANCE, test_pass

### DESIGN_BASELINE_RATIFIED_WITH_EXCEPTIONS (DET-01 and DET-02 manifest design_baseline) — DEFINITIVE

- **Historical literal:** DESIGN_BASELINE_RATIFIED_WITH_EXCEPTIONS
- **Dimensions:** decision_lifecycle
- **Sources:** contracts/decisions/ARCH-DEC-01-bis-03-2026-11-15.md; contracts/source-manifest.json
- **Safe interpretation:** ARCH-DEC-01 design baseline for the two reconstructed files, with every file-marked open detail still open.
- **Does not imply:** implemented, verified, original_qa_transcript, closed_exceptions, live_authorized

### RECONSTRUCTED_DRAFT (DET-01 and DET-02 provenance) — DEFINITIVE

- **Historical literal:** RECONSTRUCTED_DRAFT
- **Dimensions:** provenance
- **Sources:** contracts/source-manifest.json; contracts/decisions/ARCH-DEC-01-bis-03-2026-11-15.md
- **Safe interpretation:** The files formalize stated Q01-Q10 and are not the original transcripts. ARCH-DEC-01 keeps this provenance.
- **Does not imply:** approval, original_qa_transcript, verified, reviewed

### NOT_VERIFIED (ADFR verification, all 78 rows) — DEFINITIVE

- **Historical literal:** NOT_VERIFIED
- **Dimensions:** verification
- **Sources:** contracts/sources/ADFR-Consolidated-v1.0.json
- **Safe interpretation:** The only verification value in the register. Lifecycle SUPERSEDED does not change it.
- **Does not imply:** verified, externally_verified, tested_locally, broker_correct, database_correct

### EVIDENCE_PENDING (broker uniqueness) — DEFINITIVE

- **Historical literal:** EVIDENCE_PENDING
- **Dimensions:** verification, safety_authorization
- **Sources:** contracts/registry/registry.json; contracts/domain/unresolved-decisions.md
- **Safe interpretation:** Kind-specific uniqueness for orderId, permId, and execId has no accepted evidence record.
- **Does not imply:** broker_uniqueness_verified, implemented, externally_verified

### SUPERSEDED (lifecycle of ADFR-CROSS-005) — DEFINITIVE

- **Historical literal:** SUPERSEDED
- **Dimensions:** supersession
- **Sources:** contracts/sources/ADFR-Consolidated-v1.0.json; contracts/decisions/ARCH-DEC-01-bis-03-2026-11-15.md
- **Safe interpretation:** Historical row retained. superseded_by is ADFR-CR-B10-001. status stays PROPOSED_FOR_ACCEPTANCE and verification stays NOT_VERIFIED.
- **Does not imply:** verified, rejected, withdrawn, deleted, status_rewritten

### NOT_SUPERSEDED (lifecycle of the other 77 IDs) — DEFINITIVE

- **Historical literal:** NOT_SUPERSEDED
- **Dimensions:** supersession
- **Sources:** contracts/sources/ADFR-Consolidated-v1.0.json
- **Safe interpretation:** No supersession link. The count 77 is not acceptance, implementation, or verification.
- **Does not imply:** accepted, implemented, verified, ratified

### NOT_IMPLEMENTED (named registry rows) — DEFINITIVE

- **Historical literal:** NOT_IMPLEMENTED
- **Dimensions:** implementation
- **Sources:** contracts/registry/registry.json
- **Safe interpretation:** That registry row is absent as a type or event schema. OrderClosure and IncidentState are in this set.
- **Does not imply:** design_rejected, decision_withdrawn

### SKELETON_NOT_CANONICAL (registry_status) — DEFINITIVE

- **Historical literal:** SKELETON_NOT_CANONICAL
- **Dimensions:** implementation, safety_authorization
- **Sources:** contracts/registry/registry.json
- **Safe interpretation:** M0-02B-a skeleton only. Canonical wire schemas are unpublished.
- **Does not imply:** canonical_schemas_published, wire_schema

### NO-GO (registry part_b) — DEFINITIVE

- **Historical literal:** NO-GO
- **Dimensions:** safety_authorization
- **Sources:** contracts/registry/registry.json
- **Safe interpretation:** M0-02B-b canonical schemas and fixtures are not authorized.
- **Does not imply:** live_authorized, canonical_schemas_published, part_b_go

### NO_CONFLICT_FOUND (TRD-ARCH-SYNC-01 principle rows) — DEFINITIVE

- **Historical literal:** NO_CONFLICT_FOUND
- **Dimensions:** comparison_label
- **Sources:** docs/architecture/TRD-ARCH-SYNC-01-report.md
- **Safe interpretation:** Cited sections already agree on a fail-closed principle. The label is not a test result.
- **Does not imply:** implementation_permission, verified, predicate_closed, transition_authorized

### ARCHITECTURE_DRAFT_NOT_IMPLEMENTED (ADFR JSON root status) — DEFINITIVE

- **Historical literal:** ARCHITECTURE_DRAFT_NOT_IMPLEMENTED
- **Dimensions:** historical_status_word, implementation
- **Sources:** contracts/sources/ADFR-Consolidated-v1.0.json
- **Safe interpretation:** Root status preserved by ARCH-DEC-02. The register is not an implementation record.
- **Does not imply:** implemented, verified

### TESTS_NOT_EXECUTED (CR-B10 section 9 and consolidation header) — DEFINITIVE

- **Historical literal:** TESTS_NOT_EXECUTED
- **Dimensions:** verification
- **Sources:** contracts/sources/CR-B10-Domain-Glossary-and-Schema-Registry-v1.0.md; contracts/sources/Architecture-Consolidation-and-Acceptance-v1.0.md
- **Safe interpretation:** Named tests, including B10-T01 through B10-T12, are specified and not passed.
- **Does not imply:** test_pass, verified, B10-T01_passed

### IMPLEMENTED_AND_LOCALLY_TESTED (UD delivered type-shell part) — REQUIRES_USER_DECISION

- **Historical literal:** IMPLEMENTED_AND_LOCALLY_TESTED
- **Dimensions:** implementation
- **Sources:** contracts/domain/unresolved-decisions.md
- **Safe interpretation:** The named UD part has local type-shell tests. Whether any broader verification label may be used is undecided.
- **Does not imply:** externally_verified, broker_correct, database_correct, adfr_row_verified

### IMPLEMENTATION_PENDING (UD undelivered remainder) — DEFINITIVE

- **Historical literal:** IMPLEMENTATION_PENDING
- **Dimensions:** implementation
- **Sources:** contracts/domain/unresolved-decisions.md
- **Safe interpretation:** The remainder named on that UD line was not delivered.
- **Does not imply:** implemented, verified

### P0_DESIGN_RESOLVED (P0 file Gesamtstatus) — REQUIRES_USER_DECISION

- **Historical literal:** P0_DESIGN_RESOLVED
- **Dimensions:** historical_status_word
- **Sources:** contracts/sources/P0-Resolution-Contracts-CR-B01-bis-CR-B04-v1.0.md
- **Safe interpretation:** Composite header on the P0 file, paired there with IMPLEMENTATION_NOT_VERIFIED and LIVE_TRADING_NOT_AUTHORIZED.
- **Does not imply:** verified, implemented, live_authorized, PROPOSED_FOR_ACCEPTANCE

### P1_DESIGN_RESOLVED (P1 file section 0) — REQUIRES_USER_DECISION

- **Historical literal:** P1_DESIGN_RESOLVED
- **Dimensions:** historical_status_word
- **Sources:** contracts/sources/P1-Resolution-Contracts-CR-B05-bis-CR-B09-v1.0.md
- **Safe interpretation:** The P1 file says this means a written solution contract, not broker proof or software tests. Crosswalk to ADFR status is undecided.
- **Does not imply:** verified, implemented, broker_proof, software_test_pass

### DESIGN_RECONSTRUCTED_FROM_CONFIRMED_DECISIONS (DET-01 and DET-02 header) — REQUIRES_USER_DECISION

- **Historical literal:** DESIGN_RECONSTRUCTED_FROM_CONFIRMED_DECISIONS
- **Dimensions:** provenance, historical_composite
- **Sources:** contracts/sources/DET-01-Order-Workflow-and-Finality-Contract-v1.0.md; contracts/sources/DET-02-Risk-Contract-v1.0.md
- **Safe interpretation:** Header composite on the reconstructed files. ARCH-DEC-01 does not treat it as the original transcript or as a relabel of ADFR rows.
- **Does not imply:** original_qa_transcript, verified, CONFIRMED_DECISION

### SOURCE_REVIEW_PENDING (DET-01 and DET-02 source_text_status) — DEFINITIVE

- **Historical literal:** SOURCE_REVIEW_PENDING
- **Dimensions:** provenance
- **Sources:** contracts/source-manifest.json
- **Safe interpretation:** The reconstructed draft is not an independently reviewed original transcript.
- **Does not imply:** reviewed, original_qa_transcript

### NOT_REVIEWED (index and manifest source-text review) — DEFINITIVE

- **Historical literal:** NOT_REVIEWED
- **Dimensions:** provenance
- **Sources:** contracts/traceability-index.md; contracts/source-manifest.json
- **Safe interpretation:** Presence in the repository is not an independent review of the source text.
- **Does not imply:** approved, independently_reviewed

### INTERNAL_DRAFT_SHELL (registry entry status) — DEFINITIVE

- **Historical literal:** INTERNAL_DRAFT_SHELL
- **Dimensions:** implementation
- **Sources:** contracts/registry/registry.json
- **Safe interpretation:** Draft shell row. schema_path is null and canonical_wire is false on those rows.
- **Does not imply:** canonical_wire, public_schema, transitions_implemented

### BROKER_SEMANTICS_UNVERIFIED (CR-B04 cell and M0-02A header) — DEFINITIVE

- **Historical literal:** BROKER_SEMANTICS_UNVERIFIED
- **Dimensions:** verification
- **Sources:** contracts/sources/P0-Resolution-Contracts-CR-B01-bis-CR-B04-v1.0.md
- **Safe interpretation:** IBKR semantics are not evidenced. CR-B04 calls its matrix hypotheses.
- **Does not imply:** ibkr_verified, broker_uniqueness_verified

### EXTERNAL_VERIFICATION_REQUIRED (GATE-P1-07 and GATE-P1-09) — DEFINITIVE

- **Historical literal:** EXTERNAL_VERIFICATION_REQUIRED
- **Dimensions:** verification
- **Sources:** contracts/sources/P1-Resolution-Contracts-CR-B05-bis-CR-B09-v1.0.md
- **Safe interpretation:** The gate names external evidence that is not recorded as obtained.
- **Does not imply:** externally_verified, ibkr_verified

### DESIGN_ONLY (GATE-P1-05, GATE-P1-06, GATE-P1-08) — DEFINITIVE

- **Historical literal:** DESIGN_ONLY
- **Dimensions:** implementation, verification
- **Sources:** contracts/sources/P1-Resolution-Contracts-CR-B05-bis-CR-B09-v1.0.md
- **Safe interpretation:** The gate is a design statement. The named tests are not recorded as passed.
- **Does not imply:** implemented, verified

### LIVE_NOT_AUTHORIZED (source headers) — DEFINITIVE

- **Historical literal:** LIVE_NOT_AUTHORIZED
- **Dimensions:** safety_authorization
- **Sources:** contracts/sources/Architecture-Consolidation-and-Acceptance-v1.0.md; contracts/sources/CR-B10-Domain-Glossary-and-Schema-Registry-v1.0.md
- **Safe interpretation:** Prose denial of live trading on the consolidation, CR-B10, M0-02A, and roadmap headers.
- **Does not imply:** live_authorized

### LIVE_TRADING_NOT_AUTHORIZED (P0 Gesamtstatus) — DEFINITIVE

- **Historical literal:** LIVE_TRADING_NOT_AUTHORIZED
- **Dimensions:** safety_authorization
- **Sources:** contracts/sources/P0-Resolution-Contracts-CR-B01-bis-CR-B04-v1.0.md
- **Safe interpretation:** The P0 file's own denial of live trading, in the same composite as P0_DESIGN_RESOLVED.
- **Does not imply:** live_authorized

### IMPLEMENTATION_NOT_VERIFIED (DET-01, DET-02, P0, and M0-02A headers) — DEFINITIVE

- **Historical literal:** IMPLEMENTATION_NOT_VERIFIED
- **Dimensions:** verification
- **Sources:** contracts/sources/DET-01-Order-Workflow-and-Finality-Contract-v1.0.md; contracts/sources/M0-02A-UD-01-bis-UD-09-Architecture-Resolution-v1.0.md
- **Safe interpretation:** Those headers say implementation has not been verified.
- **Does not imply:** verified, implemented

### PLAN_ONLY (implementation roadmap) — DEFINITIVE

- **Historical literal:** PLAN_ONLY
- **Dimensions:** safety_authorization
- **Sources:** contracts/sources/Implementation-Roadmap-and-Cursor-Prompt-Plan-v1.0.md
- **Safe interpretation:** The roadmap does not authorize execution of its prompts.
- **Does not imply:** execution_authorized, implementation_permission

### IMPLEMENTATION_NOT_AUTHORIZED (implementation roadmap header) — DEFINITIVE

- **Historical literal:** IMPLEMENTATION_NOT_AUTHORIZED
- **Dimensions:** safety_authorization
- **Sources:** contracts/sources/Implementation-Roadmap-and-Cursor-Prompt-Plan-v1.0.md
- **Safe interpretation:** The roadmap header denies implementation authority.
- **Does not imply:** implementation_permission

### NOT_READY_FOR_LIVE_SUBMISSION (P0 ADFR-P0-008 consequence) — DEFINITIVE

- **Historical literal:** NOT_READY_FOR_LIVE_SUBMISSION
- **Dimensions:** safety_authorization
- **Sources:** contracts/sources/P0-Resolution-Contracts-CR-B01-bis-CR-B04-v1.0.md
- **Safe interpretation:** The P0 table says productive release waits on P0-gate evidence.
- **Does not imply:** live_authorized

### RESOLVED_BY_ARCH_DEC_02 (XC-11 population and supersession only) — DEFINITIVE

- **Historical literal:** RESOLVED_BY_ARCH_DEC_02
- **Dimensions:** finding_disposition
- **Sources:** contracts/decisions/ARCH-DEC-01-bis-03-2026-11-15.md
- **Safe interpretation:** Only the population and supersession sub-finding. The status-word disagreement stays open.
- **Does not imply:** status_words_reconciled, verified, implemented

### RESOLVED_BY_ARCH_DEC_03 (XC-04 design precedence only) — DEFINITIVE

- **Historical literal:** RESOLVED_BY_ARCH_DEC_03
- **Dimensions:** finding_disposition
- **Sources:** contracts/decisions/ARCH-DEC-01-bis-03-2026-11-15.md
- **Safe interpretation:** Only which design reading has precedence. Takeover stays unauthorized.
- **Does not imply:** fencing_implemented, ibkr_verified, takeover_authorized, xc04_closed

### NOT_IMPLEMENTATION_APPROVED (DET-09/10 endstatus) — DEFINITIVE

- **Historical literal:** NOT_IMPLEMENTATION_APPROVED
- **Dimensions:** safety_authorization
- **Sources:** contracts/sources/DET-09-bis-DET-10-und-Cross-Contract-Review-v1.0.md
- **Safe interpretation:** The DET-09/10 endstatus denies implementation approval.
- **Does not imply:** implementation_permission

### NOT AUTHORIZED (live-trading prose) — DEFINITIVE

- **Historical literal:** NOT AUTHORIZED
- **Dimensions:** safety_authorization
- **Sources:** contracts/decisions/ARCH-DEC-01-bis-03-2026-11-15.md
- **Safe interpretation:** Prose form of live_trading_authorized false on the decision record and the traceability index.
- **Does not imply:** live_authorized

### OPEN (TRD-ARCH-SYNC-01 comparison cell) — PROVISIONAL

- **Historical literal:** OPEN
- **Dimensions:** comparison_label
- **Sources:** docs/architecture/TRD-ARCH-SYNC-01-report.md
- **Safe interpretation:** In that report, OPEN means the comparison did not close the row. Later sub-finding labels are separate records and do not rewrite the report.
- **Does not imply:** PROPOSED_FOR_ACCEPTANCE, verified, whole_finding_closed

### OPEN (DET-01/02 Q01-Q10 verification cell) — DEFINITIVE

- **Historical literal:** OPEN
- **Dimensions:** verification
- **Sources:** contracts/sources/DET-01-Order-Workflow-and-Finality-Contract-v1.0.md; contracts/decisions/ARCH-DEC-01-bis-03-2026-11-15.md
- **Safe interpretation:** ARCH-DEC-01 keeps DET01-Q01 through DET01-Q10 and DET02-Q01 through DET02-Q10 verification cells open.
- **Does not imply:** verified, design_baseline_cancelled

