# Contract status model v1.0

**Document role:** normative vocabulary for readers of architecture records. This file is not a wire schema, not `contracts/registry/registry.json`, not a runtime flag, and not a second ADFR register.  
**Machine companion:** [status-dimensions-draft-v0.json](status-dimensions-draft-v0.json) is a non-runtime, non-wire draft. Its interpretation layer is `PROPOSED_NOT_RATIFIED`.  
**Mapping:** [legacy-status-mapping.md](legacy-status-mapping.md)  
**Approval requests:** [approval-requests.md](approval-requests.md)  
**Date:** 2026-11-15  
**Task:** TRD-ARCH-03A  
**Amendment:** 2026-11-15, TRD-ARCH-03A.2. [ARCH-DEC-04-bis-08-2026-11-15.md](../decisions/ARCH-DEC-04-bis-08-2026-11-15.md) records interpretation constraints. Those constraints do not ratify the proposed token sets and do not make this file a canonical schema.  
**Live trading:** NOT AUTHORIZED  
**Canonical schemas:** not published  

This model separates provenance, decision and design lifecycle, implementation, verification, safety authorization, severity, and supersession. A value in one dimension does not set a value in another. Existing `status`, `verification`, `lifecycle`, `supersedes`, and `superseded_by` fields stay the stored words. This file interprets them. It does not rewrite historical source text, dates, hashes, or register rows.

`PROPOSED_NOT_RATIFIED` means the closed token list under that heading is not a ratified enum. ARCH-DEC-04 through ARCH-DEC-08 are ratified interpretation constraints. They refuse a status-word winner, refuse the closed lifecycle enum, scope local-test claims, set an evidence gate without using it, and keep manifest placement. They do not move a register row, a manifest flag, or a registry flag onto a proposed token. The machine-readable draft stays `PROPOSED_NOT_RATIFIED` and `NON_RUNTIME_NON_WIRE_DRAFT`.

## 1. Ratified citations and proposed vocabulary

These citations are already in force in the named records. This file repeats them so the dimensions stay aligned with those records. It does not enlarge them.

| Citation | What is already in force | Where |
| --- | --- | --- |
| ARCH-DEC-01 | DET-01 and DET-02 files are `DESIGN_BASELINE_RATIFIED_WITH_EXCEPTIONS`. Provenance stays `RECONSTRUCTED_DRAFT`. `SOURCE_REVIEW_PENDING` stays. Details those files mark open, undefined, unconfirmed, or verification-required stay open. Q01–Q10 verification cells stay `OPEN`. The twenty `CONFIRMED_DECISION` rows keep `conversation: confirmed DET-01/02 Q01-Q10` and are not reclassified as verified. | [ARCH-DEC-01-bis-03-2026-11-15.md](../decisions/ARCH-DEC-01-bis-03-2026-11-15.md); manifest rows for the two files |
| ARCH-DEC-02 | 78 historical IDs in JSON and CSV. Exactly one lifecycle `SUPERSEDED`: `ADFR-CROSS-005`, `superseded_by` `ADFR-CR-B10-001`. That successor exists, `supersedes` `ADFR-CROSS-005`, and its lifecycle is `NOT_SUPERSEDED`. The other 77 lifecycle values are `NOT_SUPERSEDED`. That count is not acceptance. Every `verification` value stays `NOT_VERIFIED`. JSON root `status` stays `ARCHITECTURE_DRAFT_NOT_IMPLEMENTED`. Which of `OPEN P0`, `PROPOSED_FOR_ACCEPTANCE`, and `DESIGN_RESOLVED` wins is not decided. | ADFR JSON and CSV; decision record |
| ARCH-DEC-03 | For dispatch and failover design, CR-B01 has precedence over conflicting or less restrictive DET-07 text. No automatic takeover while the previous writer might still send or submission results are unresolved. Elapsed time or a lease timeout alone is not proof of a safe takeover. This is design precedence. It is not an implemented fence and not demonstrated IBKR behavior. XC-04 stays open apart from that precedence sub-finding. | decision record; CR-B01; DET-07 |
| ARCH-DEC-04 | SMQ-01, Alternative 1. No winning status among `OPEN P0`, `PROPOSED_FOR_ACCEPTANCE`, and `DESIGN_RESOLVED`. Source words stay. No default precedence. Historical `P0`, `P1`, and `P2` are not a decision-lifecycle state. The XC-11 status-word disagreement stays OPEN. No precedence is not equivalence. | [ARCH-DEC-04-bis-08-2026-11-15.md](../decisions/ARCH-DEC-04-bis-08-2026-11-15.md) |
| ARCH-DEC-05 | SMQ-02, Alternative 1. The closed lifecycle enum is not ratified. No new lifecycle assignment. In-force lifecycle facts remain `SUPERSEDED` and `NOT_SUPERSEDED`. Supersession is not withdrawal or rejection. | same decision record |
| ARCH-DEC-06 | SMQ-03, Alternative 1. `IMPLEMENTED_AND_LOCALLY_TESTED` is scoped to the named type-shell part. It is not the whole UD, not broker or database verification, not remote CI, and not ADFR verification. All 78 `NOT_VERIFIED` values and `EVIDENCE_PENDING` stay. | same decision record |
| ARCH-DEC-07 | SMQ-04, Alternative 1. A future move of ADFR `verification` off `NOT_VERIFIED` needs a dated explicit user approval and an evidence record with a URI or stable artifact reference, method, environment, timestamp, exact claim and scope, and limitations. Evidence must fit the claim. This decision does not promote any current row. | same decision record |
| ARCH-DEC-08 | SMQ-05, Alternative 1. Manifest `placement`, intake and current hashes, `reconstructed_draft`, `reviewed`, `approved`, and `source_text_status` stay. The other ten package files are not labeled `original` or `unknown`. DET-01 and DET-02 stay reconstructed, with the ARCH-DEC-01 baseline and exceptions. | same decision record; source manifest, unchanged |

The following closed sets are `PROPOSED_NOT_RATIFIED`. ARCH-DEC-04 through ARCH-DEC-08 refuse to adopt them. Refusal is not a ratification of the set, and it is not an assignment of any proposed token.

| Proposed set | Values | Why it is not in force |
| --- | --- | --- |
| Provenance class | `original`, `reconstructed`, `external_evidence`, `unknown` | DET-01 and DET-02 are already `RECONSTRUCTED_DRAFT`. ARCH-DEC-08 keeps the other package files on their manifest placement and does not label them `original` or `unknown`. |
| Decision lifecycle | `proposed`, `open`, `ratified_with_exceptions`, `superseded`, `rejected`, `withdrawn` | ARCH-DEC-05 does not ratify this enum. The register lifecycle field has only `SUPERSEDED` and `NOT_SUPERSEDED`. No row is `rejected` or `withdrawn`. `OPEN P0` is not defined as `proposed`. |
| Implementation | `not_implemented`, `partial`, `implemented` | Stored literals differ (`NOT_IMPLEMENTED`, `IMPLEMENTATION_PENDING`, `IMPLEMENTED_AND_LOCALLY_TESTED`, `SKELETON_NOT_CANONICAL`). ARCH-DEC-06 scopes the local-test phrase and does not store `partial`. |
| Verification | `not_verified`, `evidence_pending`, `tested_locally`, `externally_verified`, `failed_or_blocked` | ADFR `verification` is only `NOT_VERIFIED`. ARCH-DEC-07 sets a gate for a future change and does not write `tested_locally` or `externally_verified` onto a row. |

## 2. Interpretation layer

Read a record as several coordinates. Do not collapse them into the ADFR `status` string.

| Stored field or flag | Dimension it belongs to | What a reader may not do |
| --- | --- | --- |
| `status` on an ADFR row | historical decision-status word | Treat it as lifecycle, verification, implementation, or live authorization |
| `verification` | verification | Treat `NOT_VERIFIED` as tested, externally verified, or broker-correct |
| `lifecycle`, `supersedes`, `superseded_by` | supersession | Treat `NOT_SUPERSEDED` as accepted, or `SUPERSEDED` as deleted, rejected, or verified |
| `design_baseline` on a manifest file | decision/design lifecycle for that file | Copy it onto ADFR rows or clear `reconstructed_draft` |
| `reconstructed_draft`, `source_text_status`, `placement` | provenance | Treat provenance as approval. `reviewed` and `approved` are separate manifest booleans and stay false on the package files |
| `implemented` on a manifest file | implementation of that source file | Read `implemented: false` as a rejection of the design text |
| registry `registry_status`, `part_b`, `canonical_schemas_published`, `live_trading_authorized`, `broker_uniqueness_evidence` | safety/authorization, plus implementation for the skeleton | Publish schemas or authorize trading from a documentation status |
| `P0`, `P1`, `P2` inside a label, and the XC severity column | severity | Treat priority as a lifecycle transition |

Provenance is not approval. Design ratification is not implementation. Implementation evidence is not broker or database verification. Verification is not live authorization. Supersession is not acceptance of the successor. Severity is not a decision state.

## 3. Provenance

**Purpose.** Say where the bytes or the citation came from, and whether they are the original transcript, a reconstruction, external evidence, or unknown.

**In force**

| Literal | Meaning |
| --- | --- |
| `RECONSTRUCTED_DRAFT` | DET-01 and DET-02 formalize stated Q01–Q10. They are not the original Q&A transcripts. ARCH-DEC-01 keeps this provenance. |
| `SOURCE_REVIEW_PENDING` | Those two files are not accepted as independently reviewed original text. |
| `NOT_REVIEWED` | This index and manifest do not record an independent source-text review. |
| `conversation: confirmed DET-01/02 Q01-Q10` | Provenance string on the twenty `CONFIRMED_DECISION` rows. ARCH-DEC-01 does not rewrite it to the markdown paths. |
| Placement `COPIED_VERBATIM`, `REUSED_BYTE_IDENTICAL_EXISTING`, `AMENDED_BY_ARCH_DEC_02` | How the repository file relates to the TRD-ARCH-SYNC-01 package. An amendment is not a new provenance class and not approval. |
| User decision record | `contracts/decisions/ARCH-DEC-01-bis-03-2026-11-15.md` is the 2026-11-15 user decision. `part_of_source_package` is false. |

**Proposed, not ratified, and not assigned:** `original`, `reconstructed`, `external_evidence`, `unknown`.

`RECONSTRUCTED_DRAFT` is the in-force word for the two reconstructed files. The proposed token `reconstructed` is not written onto them in place of that word. No package file is labeled `original`. No broker evidence file is accepted as `external_evidence`. ARCH-DEC-08 keeps the other ten package files on `placement`, `reconstructed_draft`, `reviewed`, `approved`, and `source_text_status`. It does not label them `original` or `unknown`, and it does not claim original Q&A provenance for them.

**Transitions in force.** A user decision may add a design baseline without changing provenance. ARCH-DEC-01 did that. A transition from `RECONSTRUCTED_DRAFT` to original transcript requires the transcript bytes and a user decision. Neither is present.

**Forbidden implications.** Provenance does not approve, implement, verify, or authorize live trading. `RECONSTRUCTED_DRAFT` does not mean the original Q&A is in the repository.

**Evidence.** Manifest `package_sha256` is the intake hash. Current `sha256` is the repository file. They differ only where `placement` is `AMENDED_BY_ARCH_DEC_02`.

**Who may approve a provenance change.** A later dated user decision. This model cannot.

**Compatibility.** Manifest `reconstructed_draft`, `source_text_status`, `provenance`, and `placement` remain the fields. The proposed four-value class is an unassigned overlay.

## 4. Decision and design lifecycle

**Purpose.** Say whether a design statement is still a proposal, open, ratified with exceptions, superseded, or — only where a source supports it — rejected or withdrawn.

**In force**

| Literal | Scope | Meaning |
| --- | --- | --- |
| `DESIGN_BASELINE_RATIFIED_WITH_EXCEPTIONS` | DET-01 and DET-02 files only | Binding design baseline except every detail the file marks open, undefined, unconfirmed, reference design, or verification-required. |
| `SUPERSEDED` | `lifecycle` of `ADFR-CROSS-005` only | The historical row is retained. The successor is `ADFR-CR-B10-001`. The row's `status` stays `PROPOSED_FOR_ACCEPTANCE`. |
| `NOT_SUPERSEDED` | `lifecycle` of the other 77 IDs | The row has no supersession link. This is not acceptance. |
| `OPEN` | DET-01 §2 and DET-02 §2, Q01–Q10 verification cells | Those verification cells stay open after the baseline. This `OPEN` is a verification-cell word, not the ADFR `status` field. |
| Design precedence | ARCH-DEC-03 | CR-B01 is the design reading where DET-07 conflicts or is less restrictive. Precedence is not a lifecycle value on either contract's ADFR rows. |

**Historical status words that are not a lifecycle enum.** `PROPOSED_FOR_ACCEPTANCE`, `CONFIRMED_DECISION`, `DESIGN_RESOLVED`, `OPEN P0`, `OPEN P1`, and `OPEN P2` remain the literals of the sources that use them. ARCH-DEC-02 did not choose a winner among `OPEN P0`, `PROPOSED_FOR_ACCEPTANCE`, and `DESIGN_RESOLVED`. ARCH-DEC-04 decides that there is still no winner and no default precedence. The XC-11 status-word disagreement stays OPEN. The mapping keeps the crosswalk at `REQUIRES_USER_DECISION`. That unresolved mapping means the words are not equated. It does not mean a later reader may pick a default.

`CONFIRMED_DECISION` is the retained `status` of twenty rows. ARCH-DEC-01 does not make that word mean `DESIGN_BASELINE_RATIFIED_WITH_EXCEPTIONS` and does not mark the rows verified.

**Rejected or withdrawn.** No ADFR row uses those words. CR-B10 §9 says alternatives were rejected for `ADFR-CR-B10-001`. That sentence is about discarded design alternatives, not a lifecycle of `ADFR-CROSS-005`. ARCH-DEC-05 does not ratify the closed lifecycle enum and does not assign `rejected` or `withdrawn`. `SUPERSEDED` is not conflated with either word. The proposed tokens stay `PROPOSED_NOT_RATIFIED` and unassigned.

**Transitions in force**

1. Supersession. A dated user decision may set `lifecycle` to `SUPERSEDED` on one retained historical ID, set `superseded_by` to one existing successor, and set that successor's `supersedes` to the old ID, without changing `status` or `verification`. The only authorized instance is ARCH-DEC-02: `ADFR-CROSS-005` to `ADFR-CR-B10-001`.
2. Baseline with exceptions. A dated user decision may set `design_baseline` on a reconstructed file without setting `approved`, `reviewed`, or `implemented`, and without closing the file's open details. The only authorized instance is ARCH-DEC-01 for DET-01 and DET-02.
3. Design precedence. A dated user decision may rank one contract's design reading over another without implementing it. The only authorized instance is ARCH-DEC-03.

**Proposed transitions, not ratified.** `proposed` to `ratified_with_exceptions`, any state to `rejected` or `withdrawn`, and any unification of `OPEN P0` with `PROPOSED_FOR_ACCEPTANCE` or `DESIGN_RESOLVED`. ARCH-DEC-04 and ARCH-DEC-05 refuse those transitions. No such transition is applied.

**Forbidden implications.** A baseline does not close exceptions, release `OrderClosure` reasons, release a transition matrix, or authorize economic finality, risk postings, or technical execution. Supersession does not verify the old row or the successor. `DESIGN_RESOLVED` does not mean verified, implemented, or live-authorized. `NOT_SUPERSEDED` does not mean ratified.

**Evidence.** The decision record quotes the user text and names the sources. A source file's own status line is that file's claim. The manifest keeps `approved: false` on package files.

**Who may approve.** The user, in a dated record of the same kind as ARCH-DEC-01..03. This task does not.

**Compatibility.** Do not overwrite ADFR `status` to store lifecycle. `lifecycle` is already the separate supersession field. `design_baseline` is a manifest field on two files, not an ADFR column.

## 5. Implementation state

**Purpose.** Say whether software for a named scope exists. This is independent of design approval.

**In force literals, each with a scope**

| Literal | Scope in the repository |
| --- | --- |
| `NOT_IMPLEMENTED` | Named registry rows, including `OrderClosure`, `IncidentState`, `ClockQuality`, `TimestampProvenance`, and the `RiskReservationAdjusted` event name. |
| `IMPLEMENTATION_PENDING` | The undelivered remainder named on a UD line in `contracts/domain/unresolved-decisions.md`. |
| `IMPLEMENTED_AND_LOCALLY_TESTED` | The delivered type-shell part named on that same UD line. Local tests are the evidence recorded there. |
| `SKELETON_NOT_CANONICAL` | The registry file as a whole. |
| `INTERNAL_DRAFT_SHELL` | Registry rows that exist as draft shells. |
| Manifest `implemented: false` | The source file is not an implementation record. |

**Proposed, not ratified, and not assigned:** `not_implemented`, `partial`, `implemented`. ARCH-DEC-06 reads `IMPLEMENTED_AND_LOCALLY_TESTED` as scoped evidence for the explicitly named type-shell part on that UD line. The same line's `IMPLEMENTATION_PENDING` remainder stays undelivered. The phrase is not the whole UD, not broker or database verification, not remote CI, and not an ADFR `verification` change. Calling that line `partial` is not a stored status.

**Transitions.** A later implementation task may record scoped evidence. This task records none and changes no UD line. No transition from design ratification to `implemented` is authorized.

**Forbidden implications.** `DESIGN_RESOLVED`, `DESIGN_BASELINE_RATIFIED_WITH_EXCEPTIONS`, `CONFIRMED_DECISION`, and `NO_CONFLICT_FOUND` do not mean the behavior is implemented. `NOT_IMPLEMENTED` does not mean the design was rejected. Local presence of a Rust type shell is not an OMS transition, a risk posting, a fence, or a broker adapter.

**Evidence.** For `IMPLEMENTED_AND_LOCALLY_TESTED`, the UD text names the part. Passing `cargo test` in this workspace is not evidence that an ADFR row is implemented.

**Who may approve an implementation claim.** A later user decision that names the scope. This task does not authorize trading implementation.

**Compatibility.** Registry `status` and manifest `implemented` stay as they are. Do not copy `IMPLEMENTED_AND_LOCALLY_TESTED` into ADFR `status` or `verification`.

## 6. Verification state

**Purpose.** Say what evidence has been recorded, and the limits of that evidence. Passing local unit tests does not imply broker or database correctness.

**In force**

| Literal | Meaning |
| --- | --- |
| `NOT_VERIFIED` | The only ADFR `verification` value. All 78 rows. `DESIGN_RESOLVED` does not change it. |
| `EVIDENCE_PENDING` | Broker kind-specific uniqueness. Registry field `broker_uniqueness_evidence`. |
| `TESTS_NOT_EXECUTED` | CR-B10 §9 and the consolidation header. B10-T01 through B10-T12 are not passed. |
| `IMPLEMENTATION_NOT_VERIFIED` | Header claim on DET-01, DET-02, the P0 file, and the M0-02A resolution. |
| `BROKER_SEMANTICS_UNVERIFIED` | CR-B04's own cell, and the M0-02A header. |
| `EXTERNAL_VERIFICATION_REQUIRED` | P1 gates GATE-P1-07 and GATE-P1-09. The external evidence is not recorded as obtained. |
| `DESIGN_ONLY` | P1 gates GATE-P1-05, GATE-P1-06, and GATE-P1-08. |
| Q-cell `OPEN` | DET-01 and DET-02 Q01–Q10 verification is not done. |

**Proposed, not ratified, and not assigned:** `not_verified`, `evidence_pending`, `tested_locally`, `externally_verified`, `failed_or_blocked`.

No ADFR row is `externally_verified`. No row is `failed_or_blocked`. `tested_locally` is not written onto the register. ARCH-DEC-06 keeps `IMPLEMENTED_AND_LOCALLY_TESTED` as an implementation-scope phrase for the named UD type shell. It is not a promotion of `verification`.

**Evidence required before `verification` may leave `NOT_VERIFIED`.** ARCH-DEC-07 ratifies this procedural gate. It is not approval of any present promotion, and it does not ratify the proposed verification token set. Before any ADFR `verification` value changes, a dated explicit user approval must name the row and the claim, and an evidence record must be appropriate to that claim. The record contains all of: a URI or other stable artifact reference, the method, the environment, a timestamp, the exact claim and scope, and the limitations. Ignored tests are not a pass. Remote CI that was not run is not a pass. `DESIGN_RESOLVED` is not external proof. An isolated local test is not external proof for a broker, database, or register-verification claim. No such evidence record exists for a current row, and this model does not invent one.

**Transitions in force.** None that change ADFR `verification`. ARCH-DEC-02 kept every row at `NOT_VERIFIED`.

**Forbidden implications.** `NOT_VERIFIED` does not mean verified, externally verified, tested locally, broker-correct, or database-correct. `EVIDENCE_PENDING` does not mean the uniqueness rules were verified. `TESTS_NOT_EXECUTED` does not mean those tests passed. A green local unit test does not verify IBKR semantics, fencing, snapshot completeness, or PostgreSQL crash behavior.

**Who may approve.** ARCH-DEC-07 states the gate and does not apply it. A later dated user decision, together with an evidence record that meets that gate, is what would be required to change a row. A source file's `DESIGN_RESOLVED` line does not approve verification. This model does not.

**Compatibility.** The ADFR field remains `verification`. Do not store lifecycle or priority in it.

## 7. Safety and authorization

**Purpose.** A separate gate for production and live trading, canonical schema publication, broker-identity uniqueness, and dispatch readiness.

**In force.** All of these gates are closed.

| Gate | State |
| --- | --- |
| Live trading | `live_trading_authorized` is false. Prose forms include `NOT AUTHORIZED`, `LIVE_NOT_AUTHORIZED`, and `LIVE_TRADING_NOT_AUTHORIZED`. |
| Canonical schema publication | `canonical_schemas_published` is false. Registry status is `SKELETON_NOT_CANONICAL`. `part_b` is `NO-GO`. Paths `registry.yaml`, `events/v1`, `commands/v1`, `api/v1`, and `sql/v1` are not published. |
| Broker identity uniqueness | `broker_uniqueness_evidence` is `EVIDENCE_PENDING`. |
| Dispatch readiness | No ready flag. ARCH-DEC-03 does not authorize automatic takeover, a lease-timeout failover, or an implemented fence. |

**Transitions.** None are authorized. Opening any gate requires a later user decision that names that gate. Design baseline, register reconciliation, and design precedence do not open them.

**Forbidden implications.** `NO-GO` does not mean go. `SKELETON_NOT_CANONICAL` does not mean the skeleton is a canonical wire profile. Design precedence does not mean dispatch is ready.

**Who may approve.** The user, explicitly, for the named gate. This task does not.

**Compatibility.** Registry booleans and `part_b` stay the store of record. This model does not add a runtime configuration key.

## 8. Severity and priority

**Purpose.** Rank a topic. Priority is not lifecycle.

**In force.** `P0`, `P1`, and `P2` occur inside historical labels (`OPEN P0`, `OPEN P1`, `OPEN P2`) and in the TRD-ARCH-SYNC-01 severity column (`P0`, `P1`, `P0 topic, principle only`). The XC severity column is not the status column.

**Meaning of `OPEN P0`.** It is a composite historical label in the DET-09/10 `## ADFR-CROSS` table: an unresolved marker in that table plus priority `P0`. ARCH-DEC-04 says that priority is not a decision-lifecycle state. `OPEN P0` is not a synonym of `PROPOSED_FOR_ACCEPTANCE` or of `DESIGN_RESOLVED`. The same reading applies to `OPEN P1` and `OPEN P2` for priority `P1` and `P2`. The cross-source mapping stays `REQUIRES_USER_DECISION` because ARCH-DEC-04 establishes no default precedence and does not treat the words as equivalent.

**Forbidden implications.** `P0` does not mean proposed, ratified, implemented, or verified. A `P0` XC row can be `NO_CONFLICT_FOUND` on a principle and still leave a predicate open (`XC-02-PRED`, `XC-03-MATRIX`, `XC-07-ROUND`, `XC-08-IBKR`, `XC-10-ACTION`).

**Who may approve a priority change.** A later user decision. This task does not relabel priorities.

## 9. Supersession

**Purpose.** Keep a historical decision and point at a successor without deleting the historical record and without accepting the successor.

**In force, from ARCH-DEC-02**

| Rule | State |
| --- | --- |
| Historical IDs | 78, unique, same set and order in JSON and CSV |
| Superseded | Exactly one: `ADFR-CROSS-005` |
| Successor | `ADFR-CR-B10-001`, and that ID exists in the register |
| Not superseded | 77 |
| Direction | `ADFR-CR-B10-001.supersedes` is `ADFR-CROSS-005`. `ADFR-CROSS-005.superseded_by` is `ADFR-CR-B10-001`. No other row sets either link. |
| Cycles | The only edge has no path back. A successor must not supersede itself through a cycle. |
| Core fields | `id`, `decision`, `source`, `status`, `basis`, and `verification` of the previous 77 IDs stay the pre-amendment values. The successor uses the pre-amendment CSV core fields. |
| Automatic acceptance | Not granted. The successor stays `DESIGN_RESOLVED` and `NOT_VERIFIED`. |

The DET-09/10 table cell for `ADFR-CROSS-005` still says `OPEN P2`. That cell was not rewritten. Register lifecycle `SUPERSEDED` does not erase that historical cell. Reading `OPEN P2` as if the cell had been edited to `SUPERSEDED` is not authorized.

**Forbidden implications.** Supersession does not verify, implement, or delete. It does not change `verification`. It does not mean the successor's tests passed. `NOT_SUPERSEDED` does not mean the decision was accepted.

**Who may approve another supersession.** A later dated user decision that names the retained ID and an existing successor. This task adds none.

## 10. Explicit readings required by the current records

**XC-11 status words.** Population and supersession are `RESOLVED_BY_ARCH_DEC_02`. ARCH-DEC-04 records that there is no winning status and no default precedence. The XC-11 status-word disagreement stays OPEN. The 2026-11-14 consolidation sentence that calls `OPEN P0` a historical cross-reference is retained as that file's wording. ARCH-DEC-02 did not adopt it as the reconciliation, and ARCH-DEC-04 does not adopt it either. The historical words are not equivalent.

**ARCH-DEC-01.** The design baseline and the reconstructed provenance stand together. Q01–Q10 verification stays open. `OrderClosure` reasons and transition matrices stay unreleased.

**ARCH-DEC-03.** CR-B01 design precedence stands. Unsafe takeover is not authorized. Fencing is not implemented. IBKR behavior is not demonstrated.

**These four are different coordinates.** `DESIGN_RESOLVED` is not `VERIFIED`. `VERIFIED` is not a value in the ADFR `verification` column. `IMPLEMENTED` is not established by a design label. `LIVE_AUTHORIZED` is not a true flag. The in-force denial literals are `NOT_VERIFIED`, the registry implementation literals, and `live_trading_authorized: false`.

**`NO_CONFLICT_FOUND`.** In the TRD-ARCH-SYNC-01 comparison, that label means the cited sections already agree on a fail-closed principle. It is not a test result and not permission to implement an unresolved predicate or transition. `XC-03-MATRIX`, `XC-02-PRED`, and the other open XC rows stay open. See section 11.

## 11. Findings this model does not close

XC-01, XC-02-PRED, XC-03-MATRIX, XC-05-LOCK, XC-06, XC-07-ROUND, XC-08-IBKR, XC-09-RESET, and XC-10-ACTION stay OPEN.

XC-11: the population and supersession sub-finding stays `RESOLVED_BY_ARCH_DEC_02`. The status-word sub-finding stays OPEN. ARCH-DEC-04 is the decision that no precedence is established. XC-11 status-word disagreement stays OPEN.

XC-04: the design-precedence sub-finding stays `RESOLVED_BY_ARCH_DEC_03`. Broker-session exclusivity, lease and heartbeat, dispatch scope, in-flight send evidence, recovery tests, and IBKR behavior stay OPEN.

`NO_CONFLICT_FOUND` rows from TRD-ARCH-SYNC-01 stay as that report left them: XC-02-ALIGN, XC-03-BOOK, XC-05-OUTBOX, XC-07-FX, XC-08-RULE, XC-09-SOLO, and XC-10-RACE. None of them authorizes an implementation.

## 12. Who may approve, for every dimension

A later dated user decision, recorded under `contracts/decisions/`, may change a dimension by naming the record, the old literal, the new literal, and the evidence. ARCH-DEC-04 through ARCH-DEC-08 are interpretation constraints only. They do not approve a technical contract, a runtime type, or a present verification change. Source-file self-labels do not approve repository acceptance. Local tests do not approve verification of broker or database behavior. This document, the draft JSON, and the documentation tests do not approve.
