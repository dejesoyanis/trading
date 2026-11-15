# Status-model approval requests

**Date:** 2026-11-15  
**Task:** TRD-ARCH-03A  
**Status of the questions below:** historical request text from TRD-ARCH-03A. Nothing here is approved.  
**Resolution index:** dated addendum at the end of this file, 2026-11-15. The addendum records user decisions. It does not rewrite the questions.  
**Live trading:** NOT AUTHORIZED  

Do not self-approve. A decision, if the user wants one, belongs in a later dated record under `contracts/decisions/`. Until then the interpretation layer in [contract-status-model-v1.0.md](contract-status-model-v1.0.md) stays `PROPOSED_NOT_RATIFIED` where that file says so, and the historical literals stay as written.

The paragraph above is the request as written. The addendum at the end is the later record of what the user decided. The questions themselves are not edited to pretend they were already approved.

ARCH-DEC-01, ARCH-DEC-02, and ARCH-DEC-03 are already recorded. These questions are the points those decisions do not settle.

## SMQ-01 — Precedence among `OPEN P0`, `PROPOSED_FOR_ACCEPTANCE`, and `DESIGN_RESOLVED`

**Question.** For one historical topic that carries more than one of these words, which word is the decision-lifecycle reading?

**Sources.** `contracts/sources/DET-09-bis-DET-10-und-Cross-Contract-Review-v1.0.md` section `## ADFR-CROSS` (`OPEN P0`, `OPEN P1`, `OPEN P2`). `contracts/sources/ADFR-Consolidated-v1.0.json` (`PROPOSED_FOR_ACCEPTANCE` on those cross rows, `DESIGN_RESOLVED` on `ADFR-CR-B10-001`, every `verification` = `NOT_VERIFIED`). `contracts/sources/P0-Resolution-Contracts-CR-B01-bis-CR-B04-v1.0.md` (`DESIGN_RESOLVED`, `P0_DESIGN_RESOLVED`). `contracts/sources/P1-Resolution-Contracts-CR-B05-bis-CR-B09-v1.0.md` (`P1_DESIGN_RESOLVED`). `contracts/sources/Architecture-Consolidation-and-Acceptance-v1.0.md` 2026-11-14 sentence that calls `OPEN P0` a historical cross-reference, plus the 2026-11-15 addendum that does not adopt that sentence as a reconciliation. `contracts/decisions/ARCH-DEC-01-bis-03-2026-11-15.md` ARCH-DEC-02, paragraph beginning "Not decided". `docs/architecture/TRD-ARCH-SYNC-02-report.md` XC-11 status-word row, OPEN.

**Alternatives.**

1. No winner. Keep every literal in its source. The interpretation layer records the conflict and does not equate the words. Recommended.
2. The amended JSON/CSV `status` is the only decision-lifecycle word. Table cells become historical notes with no design force.
3. The later P0 or P1 `DESIGN_RESOLVED` replaces `OPEN P0` and `PROPOSED_FOR_ACCEPTANCE` for design lifecycle, and verification stays `NOT_VERIFIED`.

**Consequences.** Alternative 1 leaves XC-11's status-word remainder open and blocks any silent equivalence. Alternative 2 would treat `PROPOSED_FOR_ACCEPTANCE` as the design state even where a P0 section says `DESIGN_RESOLVED`. Alternative 3 would close the remainder by preferring the resolution contracts. The consolidation sentence points toward alternative 3, and ARCH-DEC-02 left that choice unmade. Verification stays `NOT_VERIFIED` under all three. None of them authorizes implementation or live trading.

**Recommendation.** Alternative 1.

## SMQ-02 — A closed lifecycle enum, including rejected and withdrawn

**Question.** May the repository adopt `proposed`, `open`, `ratified_with_exceptions`, `superseded`, `rejected`, and `withdrawn` as a decision-lifecycle enum?

**Sources.** ARCH-DEC-02 lifecycle values actually stored: `SUPERSEDED` and `NOT_SUPERSEDED` only. No ADFR row is rejected or withdrawn. CR-B10 §9 "Alternatives rejected" refers to discarded design alternatives of `ADFR-CR-B10-001`, not to a lifecycle of `ADFR-CROSS-005`. `contracts/status-model/contract-status-model-v1.0.md` section 4.

**Alternatives.**

1. Do not add the enum and do not assign `rejected` or `withdrawn`. Recommended.
2. Ratify the enum now and leave every row unassigned.
3. Treat `SUPERSEDED` as withdrawn and rewrite `ADFR-CROSS-005`.

**Consequences.** Alternative 1 keeps the two lifecycle literals ARCH-DEC-02 already applied. Alternative 2 publishes a canonical-looking enum before any row uses `rejected` or `withdrawn`. Alternative 3 contradicts ARCH-DEC-02, which kept `ADFR-CROSS-005` at `status` `PROPOSED_FOR_ACCEPTANCE` and `verification` `NOT_VERIFIED`.

**Recommendation.** Alternative 1.

## SMQ-03 — What `IMPLEMENTED_AND_LOCALLY_TESTED` may support

**Question.** Which verification or implementation claim may follow from a UD line that says `IMPLEMENTED_AND_LOCALLY_TESTED` for one part and `IMPLEMENTATION_PENDING` for the rest?

**Sources.** `contracts/domain/unresolved-decisions.md` UD-01, UD-02, UD-03, UD-04, UD-05, and UD-09. `contracts/sources/M0-02A-UD-01-bis-UD-09-Architecture-Resolution-v1.0.md` header `IMPLEMENTATION_NOT_VERIFIED`. Registry rows whose `status` is `NOT_IMPLEMENTED`. ADFR `verification` `NOT_VERIFIED` on all 78 rows.

**Alternatives.**

1. Scoped evidence for the named type-shell part only. ADFR `verification` stays `NOT_VERIFIED`. The phrase is not broker correctness, not database-integration correctness, and not remote CI. Recommended.
2. Promote the parent UD to externally verified.
3. Promote the related `CONFIRMED_DECISION` rows to a tested verification value.

**Consequences.** Alternative 1 matches the split already written on the UD lines and leaves broker uniqueness `EVIDENCE_PENDING`. Alternative 2 treats local unit tests as external proof. Alternative 3 writes a verification claim ARCH-DEC-01 and ARCH-DEC-02 refused. Passing local unit tests still would not show broker or database correctness.

**Recommendation.** Alternative 1.

## SMQ-04 — Evidence required before ADFR `verification` leaves `NOT_VERIFIED`

**Question.** What record is required before any ADFR `verification` value changes from `NOT_VERIFIED`?

**Sources.** All 78 ADFR `verification` values. ARCH-DEC-02. CR-B10 §9 `TESTS_NOT_EXECUTED` and B10-T01 through B10-T12. P0 freigabeformel in `P0-Resolution-Contracts-CR-B01-bis-CR-B04-v1.0.md` section `## 0`. Registry flags `live_trading_authorized` false, `canonical_schemas_published` false, `part_b` `NO-GO`, `broker_uniqueness_evidence` `EVIDENCE_PENDING`.

**Alternatives.**

1. A dated user decision plus an evidence record naming URI, method, environment, timestamp, scope, and limitations. A local `cargo test` success is not that record for broker or database claims. An ignored test is not a pass. Remote CI that was not run is not a pass. Recommended.
2. A green local `cargo test` rewrites `verification`.
3. A source file's `DESIGN_RESOLVED` is enough to set `verification` to a verified value.

**Consequences.** Alternative 1 keeps the register unchanged until a named evidence record exists. Alternative 2 would mark architectural decisions verified because type-shell tests passed. Alternative 3 would treat a design label as a test result, which the consolidation file and ARCH-DEC-02 already deny.

**Recommendation.** Alternative 1.

## SMQ-05 — Provenance class for package files other than DET-01 and DET-02

**Question.** May files other than DET-01 and DET-02 be labeled with the proposed set `original`, `reconstructed`, `external_evidence`, or `unknown`?

**Sources.** `contracts/source-manifest.json`: DET-01 and DET-02 have `reconstructed_draft` true and `source_text_status` `SOURCE_REVIEW_PENDING`. The other ten package files have `reconstructed_draft` false, `reviewed` false, and `approved` false. Their `placement` is `COPIED_VERBATIM`, `REUSED_BYTE_IDENTICAL_EXISTING`, or `AMENDED_BY_ARCH_DEC_02`. No manifest row is labeled original Q&A. No external broker-evidence file is listed as accepted.

**Alternatives.**

1. Leave those files on their manifest placement and `NOT_REVIEWED`. Do not assign `original` or `unknown`. Recommended.
2. Label every non-reconstructed package file `original`.
3. Label every file that is not an original transcript `unknown`.

**Consequences.** Alternative 1 keeps the intake hashes and the reconstructed-draft bit as the provenance record. Alternative 2 would call conversation-drafted contracts original transcripts. Alternative 3 would hide that the bytes are the package text identified by `package_sha256`.

**Recommendation.** Alternative 1.

## What is not asked again

ARCH-DEC-01 already fixed the DET-01 and DET-02 baseline with exceptions and the reconstructed provenance. ARCH-DEC-02 already fixed the 78 IDs and the one supersession. ARCH-DEC-03 already fixed CR-B01 design precedence and refused automatic takeover. Those three are not reopened by this list.

## Resolution index (2026-11-15, TRD-ARCH-03A.2)

The questions SMQ-01 through SMQ-05 above are unchanged historical text, including their alternatives and the recommendation of Alternative 1. They are not rewritten as approvals. The user decided each question on 2026-11-15 by choosing Alternative 1. The decision record is [ARCH-DEC-04-bis-08-2026-11-15.md](../decisions/ARCH-DEC-04-bis-08-2026-11-15.md). This index is a map to that record. It is not a second decision, not a signature, and not an evidence URI.

| Question | Decision | Outcome | Chosen alternative | User wording |
| --- | --- | --- | --- | --- |
| SMQ-01 | ARCH-DEC-04 | DECIDED | Alternative 1 | Keine Priorität; historische Statusbegriffe unverändert lassen |
| SMQ-02 | ARCH-DEC-05 | DECIDED | Alternative 1 | Kein neues Enum und keine zusätzlichen Lifecycle-Zuordnungen |
| SMQ-03 | ARCH-DEC-06 | DECIDED | Alternative 1 | Nur konkret getestete Teilkomponenten als lokal getestet behandeln |
| SMQ-04 | ARCH-DEC-07 | DECIDED | Alternative 1 | Datierter Nutzerentscheid plus geeigneter Evidenznachweis |
| SMQ-05 | ARCH-DEC-08 | DECIDED | Alternative 1 | Bestehende Manifestangaben beibehalten |

ARCH-DEC-04 leaves the XC-11 status-word disagreement OPEN. No winning word is chosen among `OPEN P0`, `PROPOSED_FOR_ACCEPTANCE`, and `DESIGN_RESOLVED`, and those words are not treated as equivalent. ARCH-DEC-05 does not ratify the closed lifecycle enum. ARCH-DEC-06 does not promote ADFR `verification`. ARCH-DEC-07 is a procedural gate for a future change and does not promote any current row. ARCH-DEC-08 does not relabel manifest provenance. The proposed token sets in the status model stay `PROPOSED_NOT_RATIFIED`. Live trading stays NOT AUTHORIZED. No evidence record is created here.
