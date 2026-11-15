# TRD-ARCH-03B.3 — Broker decision requests

**Date:** 2026-11-15  
**Role:** design proposal. `document_role` = `NON_RUNTIME_NON_WIRE_PROPOSAL`.  
**Ratification:** `NOT_RATIFIED`.  
**Status of every question:** `USER_DECISION_PENDING`.  
**XC-01:** OPEN. **XC-08-IBKR:** OPEN. **XC-04 remainder:** OPEN. **XC-02-PRED:** OPEN.  
**Live trading:** NOT AUTHORIZED.  
**Canonical schemas:** not published. `part_b` = `NO-GO`. `broker_uniqueness_evidence` = `EVIDENCE_PENDING`.

A recommendation is not an approval. The questions are independent. Answering one does not answer another. No question authorizes economic posting, an economic-finality predicate, reservation release, an OMS transition, a canonical schema, a uniqueness index, broker dispatch, or live trading.

ARCH-DEC-03, ARCH-DEC-13, ARCH-DEC-14, ARCH-DEC-17, and ARCH-DEC-26 stay binding for every alternative below. An alternative marked `CONFLICTS_WITH_BINDING_POLICY` is contrast. It is not available for ratification unless a later decision explicitly supersedes the named binding rule. This package does not supersede ARCH-DEC-01 through ARCH-DEC-34.

## BROKERQ-01 — Identity tuple for a future record

**Status:** `USER_DECISION_PENDING`  
**Ratification:** `NOT_RATIFIED`

**Question:** Which components may a future evidence record require before a broker observation is stored, without approving uniqueness or a schema?

**Binding rules:** DET-03-D01 and D02. CR-B04 §4.1 and §4.4. UD-02: missing client, session, epoch, and evidence stay absent. No UNIQUE constraint on `orderId`, `permId`, or `execId` alone until a scope is evidenced (P0-B04-INV-02).

**Sources:** `contracts/sources/DET-03-bis-DET-08-Architecture-Contracts-v1.0.md` DET-03. `contracts/sources/P0-Resolution-Contracts-CR-B01-bis-CR-B04-v1.0.md` §4. `contracts/domain/unresolved-decisions.md` UD-02. `contracts/domain/glossary.md` identity rows.

**Unresolved:** which absences are required for which alias kind. XC-01 stays OPEN. No canonical tuple is approved by asking.

**Alternatives:**

- **A — Internal ids only.** Store platform ids and the raw payload. Do not record broker fields as structured components.
- **B — Supplied scope, no uniqueness.** Record broker, account, environment, alias kind, and raw id, plus client, session, adapter epoch, `permId`, and evidence id only when supplied. Absence stays absence. Equality is not a merge and not a uniqueness proof.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** Freeze a canonical UNIQUE tuple on `orderId`, `permId`, or `execId` alone. Conflicts with CR-B04 §4.1, P0-B04-INV-02, and UD-02.

| Alternative | Identity impact | Completeness impact | Recovery impact | Posting, release, dispatch |
| --- | --- | --- | --- | --- |
| A | Raw bytes only. Harder to review later. | No structured scope on the row. | Attempt correlation stays local. | Still prohibited. |
| B | Matches the unverified shell. | Scope fields can be absent honestly. | Does not identify a broker order as accepted. | Still prohibited. |
| C | Collapses reused ids. | A unique index pretends coverage. | A collision blocks honest recovery. | Still prohibited, and not ratifiable. |

**Non-binding recommendation:** B. It records what was supplied and claims no uniqueness. This recommendation is not an approval.

**Evidence needed:** none to keep B fail-closed. A later uniqueness rule needs XC-01 evidence that is `EXTERNAL_EVIDENCE_PENDING`.

**Dependencies:** BROKERQ-02, BROKERQ-03, BROKERQ-09. Does not depend on an economic-finality predicate.

**Would not authorize:** a schema, a migration, a uniqueness index, a posting, a release, or a dispatch.

## BROKERQ-02 — Ambiguity and candidate sets

**Status:** `USER_DECISION_PENDING`  
**Ratification:** `NOT_RATIFIED`

**Question:** May a future record keep a candidate set for a broker observation, and what may that set authorize?

**Binding rules:** DET-03 formal rule for `\|A(e)\|`. ID-INV-02. CR-B04 §4.3. ARCH-DEC-13 and ARCH-DEC-26: a single match does not post. The source sentence that only `MATCHED` may book is preserved and not followed.

**Sources:** DET-03-D04 and the formal rule in `DET-03-bis-DET-08-Architecture-Contracts-v1.0.md`. CR-B04 §4.3 and §4.6. `contracts/decisions/ARCH-DEC-09-bis-20-2026-11-15.md` ARCH-DEC-13. `contracts/decisions/ARCH-DEC-21-bis-34-2026-11-15.md` ARCH-DEC-26.

**Unresolved:** whether inference records are worth storing before uniqueness evidence exists. XC-01 stays OPEN.

**Alternatives:**

- **A — No candidate set.** Every observation stays unmatched as data. No inference record.
- **B — Candidate set, no authorization.** Store `A(e)` and a label. Size one is `SINGLE_CANDIDATE_NOT_AUTHORIZATION`. Size zero, many, or a contradiction cannot post either.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** Auto-match and book when `\|A(e)\| = 1`. Conflicts with ARCH-DEC-13 and ARCH-DEC-26. The DET-03-D04 booking sentence is the conflicting source, not a permission.

| Alternative | Identity impact | Completeness impact | Recovery impact | Posting, release, dispatch |
| --- | --- | --- | --- | --- |
| A | No false confidence. | No inferred coverage. | Manual review sees raw rows only. | Still prohibited. |
| B | Ambiguity is explicit. | A singleton is still not completeness. | Review can see why a row was not attached. | Still prohibited. |
| C | Hides residual doubt. | A match is mistaken for a full snapshot. | A wrong attach is hard to undo. | Still prohibited, and not ratifiable. |

**Non-binding recommendation:** A, until the user wants inference records. B is the evidence-rich candidate and is not required to stay fail-closed. This recommendation is not an approval.

**Evidence needed:** XC-01 before any future promotion of B toward a verified match. Not present.

**Dependencies:** BROKERQ-01, BROKERQ-04, BROKERQ-13. Independent of OMS transition edges.

**Would not authorize:** a resolver, a booking, or a release.

## BROKERQ-03 — Missing permId

**Status:** `USER_DECISION_PENDING`  
**Ratification:** `NOT_RATIFIED`

**Question:** How is a missing, null, or zero `permId` recorded?

**Binding rules:** CR-B04 §4.2 null and special values are unverified. UD-02: absence is an absent field. DET-03-D01: `permId` is not the system identity. P0-B04-T10 is `NOT_EXECUTED`.

**Sources:** CR-B04 §4.2 and §4.5 T10. UD-02. Glossary `BrokerPermIdAlias`.

**Unresolved:** whether the target broker uses zero as missing. That behavior is `EXTERNAL_EVIDENCE_PENDING`. This package does not assert it.

**Alternatives:**

- **A — Drop the observation** until a `permId` arrives.
- **B — Retain with permId absent.** The order-id observation is kept. Missing does not equal another missing value. A later `permId` is a new observation, not an overwrite.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** Store missing as zero and match all zeros together. Conflicts with UD-02 and P0-B04-INV-02.

| Alternative | Identity impact | Completeness impact | Recovery impact | Posting, release, dispatch |
| --- | --- | --- | --- | --- |
| A | Loses the early observation. | Looks like the event never arrived. | Conflicts with append-only retention. | Still prohibited. |
| B | Absence stays honest. | The snapshot is not completed by the gap. | Late `permId` can be appended. | Still prohibited. |
| C | False matches. | Many orders collapse. | Recovery cannot separate them. | Still prohibited, and not ratifiable. |

**Non-binding recommendation:** B. A would discard evidence ARCH-DEC-12 tells the platform to keep. This recommendation is not an approval.

**Evidence needed:** broker null and zero semantics. `EXTERNAL_EVIDENCE_PENDING`.

**Dependencies:** BROKERQ-01, BROKERQ-05. No dependency on a release rule.

**Would not authorize:** a `permId` uniqueness constraint or a booking when the field later appears.

## BROKERQ-04 — Duplicate executions

**Status:** `USER_DECISION_PENDING`  
**Ratification:** `NOT_RATIFIED`

**Question:** How are two execution observations with the same apparent broker key recorded?

**Binding rules:** CR-B04 §4.3 step 5. ID-INV-03. ARCH-DEC-16: identical payload replay creates no new economic effect. A different payload is a conflict, not an overwrite. Booking is already forbidden.

**Sources:** DET-03 ID-T04. CR-B04 §4.3. ARCH-DEC-16 in `ARCH-DEC-09-bis-20-2026-11-15.md`.

**Unresolved:** whether `execId` is stable across corrections. `EXTERNAL_EVIDENCE_PENDING`. XC-01 stays OPEN.

**Alternatives:**

- **A — Every repeat is a contradiction,** including an identical payload.
- **B — Idempotent observation.** Identical raw payload is one observation seen again and creates no economic effect and no second send. A different payload is `CONTRADICTION` and both are kept.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** Last write wins. Conflicts with DET-03-D05, ID-INV-04, and ARCH-DEC-16.

| Alternative | Identity impact | Completeness impact | Recovery impact | Posting, release, dispatch |
| --- | --- | --- | --- | --- |
| A | Identical replays look like conflicts. | Safe and noisy. | Harder to see a true disagreement. | Still prohibited. |
| B | Matches the replay rule. | A duplicate does not fill a gap. | Both payloads remain if they differ. | Still prohibited. |
| C | Destroys the first payload. | Hides the disagreement. | Cannot be audited. | Still prohibited, and not ratifiable. |

**Non-binding recommendation:** B. This recommendation is not an approval.

**Evidence needed:** correction and replay examples from the target broker. Not in this repository.

**Dependencies:** BROKERQ-02, BROKERQ-05, BROKERQ-13.

**Would not authorize:** a second fill, a posting, or a dedup that deletes bytes.

## BROKERQ-05 — Correction and bust chain

**Status:** `USER_DECISION_PENDING`  
**Ratification:** `NOT_RATIFIED`

**Question:** How are correction and bust observations linked, if at all?

**Binding rules:** DET-03-D05 append-only. CR-B04 T06 is `NOT_EXECUTED`. CR-B03 §3.2 describes a compensating booking. ARCH-DEC-26 does not adopt that line. ARCH-DEC-09: the historical closure record is not rewritten.

**Sources:** DET-03-D05. CR-B03 §3.2. CR-B04 §4.5 T06. ARCH-DEC-09, ARCH-DEC-26.

**Unresolved:** whether a bust reuses `execId` or allocates another. `EXTERNAL_EVIDENCE_PENDING`.

**Alternatives:**

- **A — Unlinked observations.** Each correction is stored with no reference to the earlier evidence.
- **B — Append-only link when the observation carries one.** Keep the original. Store a reference only if the new evidence itself names one. No reversal, no release, no closure edit.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** Overwrite the original and post a reversal. Conflicts with DET-03-D05, ARCH-DEC-09, and ARCH-DEC-26.

| Alternative | Identity impact | Completeness impact | Recovery impact | Posting, release, dispatch |
| --- | --- | --- | --- | --- |
| A | Chain is invisible. | Both facts remain. | A reviewer must guess the link. | Still prohibited. |
| B | Link is evidence, not economics. | Does not prove the chain is complete. | Original and correction both remain. | Still prohibited. |
| C | Loses the original. | Pretends the bust is final. | Conflicts with quarantine. | Still prohibited, and not ratifiable. |

**Non-binding recommendation:** B. This recommendation is not an approval.

**Evidence needed:** a correction example with old and new identifiers. `EXTERNAL_EVIDENCE_PENDING`.

**Dependencies:** BROKERQ-04, BROKERQ-13. Does not authorize OMS closure changes.

**Would not authorize:** a compensating ledger line, a release, or a closure rewrite.

## BROKERQ-06 — Snapshot scope

**Status:** `USER_DECISION_PENDING`  
**Ratification:** `NOT_RATIFIED`

**Question:** What is the largest scope one broker response may be said to cover?

**Binding rules:** CR-B07 P1-B07-D01: completeness is per named scope, and the IBKR criterion is unverified. One response is not a global snapshot. GATE-P1-07 is `EXTERNAL_VERIFICATION_REQUIRED` in the source and is not met.

**Sources:** `contracts/sources/P1-Resolution-Contracts-CR-B05-bis-CR-B09-v1.0.md` CR-B07. DET-08-D02.

**Unresolved:** which IBKR calls return which datasets. `EXTERNAL_EVIDENCE_PENDING`. XC-08-IBKR stays OPEN.

**Alternatives:**

- **A — No scope label.** The response is bytes plus receive time only.
- **B — Named scope, still unknown.** Record the account, client, session, dataset, and window if they were requested, and mark coverage `UNKNOWN`. Do not treat the response as global.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** One account response covers every client, session, dataset, and all time. Conflicts with P1-B07-D01 and P1-B07-D03.

| Alternative | Identity impact | Completeness impact | Recovery impact | Posting, release, dispatch |
| --- | --- | --- | --- | --- |
| A | Rows lack scope. | Nothing is called complete. | Hard to see which session was read. | Still prohibited. |
| B | Scope is a label, not a match. | Explicitly not global. | Gaps stay visible. | Still prohibited. |
| C | Cross-session merges. | False completeness. | Missing rows look like non-existence. | Still prohibited, and not ratifiable. |

**Non-binding recommendation:** B. This recommendation is not an approval.

**Evidence needed:** per-dataset IBKR coverage notes. Not retrieved.

**Dependencies:** BROKERQ-07, BROKERQ-08, BROKERQ-09.

**Would not authorize:** a `COMPLETE` flag, a release, or a dispatch.

## BROKERQ-07 — Completeness proof

**Status:** `USER_DECISION_PENDING`  
**Ratification:** `NOT_RATIFIED`

**Question:** May a future record contain a completeness claim, and what may that claim authorize?

**Binding rules:** P1-B07-D02: `COMPLETE` only after a tested criterion, which is absent. An empty response is not a negative proof. ARCH-DEC-14 and ARCH-DEC-29: no release proof. ARCH-DEC-17: no authoritative terminal observation. ARCH-DEC-28: no evidence list is adopted as the predicate.

**Sources:** CR-B07. DET-08 RC-INV-04. ARCH-DEC-14, ARCH-DEC-17, ARCH-DEC-28, ARCH-DEC-29.

**Unresolved:** the criterion itself. XC-08-IBKR and XC-02-PRED stay OPEN.

**Alternatives:**

- **A — No completeness token** may be stored.
- **B — Token as a pointer only.** A future claim may name an external evidence id and still authorize nothing. Until that evidence exists the pointer is `EXTERNAL_EVIDENCE_PENDING`.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** `COMPLETE`, or an empty response, proves non-existence, finality, or release. Conflicts with P1-B07-D02, ARCH-DEC-14, and ARCH-DEC-17.

| Alternative | Identity impact | Completeness impact | Recovery impact | Posting, release, dispatch |
| --- | --- | --- | --- | --- |
| A | No extra claim. | Safest reading. | No false "all clear". | Still prohibited. |
| B | The claim is not an identity match. | Still not a proof in this repository. | Cannot start a resubmit. | Still prohibited. |
| C | Missing rows become false unmatched. | The open obligation is erased. | Invites a second send. | Still prohibited, and not ratifiable. |

**Non-binding recommendation:** A. B is available only as a non-authorizing pointer and is not recommended while no external evidence exists. This recommendation is not an approval.

**Evidence needed:** a documented completion condition for each query type, with negative cases. `NOT_EXECUTED`.

**Dependencies:** BROKERQ-06, BROKERQ-08. Does not close XC-02-PRED.

**Would not authorize:** release, non-existence, or terminal acceptance.

## BROKERQ-08 — Gap detection

**Status:** `USER_DECISION_PENDING`  
**Ratification:** `NOT_RATIFIED`

**Question:** How are holes in a captured stream recorded, without inventing a broker sequence number?

**Binding rules:** DET-04-D05: a gap in an aggregate sequence waits. That sentence is about local aggregate versions, not an IBKR sequence. DET-09-D05 is market data. P1-B07-D03: limited history keeps reconciliation open. No IBKR sequence number is stated in the sources as a guarantee, and none is invented here.

**Sources:** DET-04-D05. DET-05-D04. DET-09-D05. CR-B07 P1-B07-D03. B07-T03 and T04, both `NOT_EXECUTED`.

**Unresolved:** whether the target API exposes a gap signal. `EXTERNAL_EVIDENCE_PENDING`.

**Alternatives:**

- **A — No gap marker.** The stream is simply incomplete. Do not guess holes.
- **B — Local capture markers only.** If the capture itself saw a short page, a missing end, or a dropped connection, store that fact. Do not invent a broker sequence number. A marker keeps the scope incomplete and authorizes nothing.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** Fill gaps by interpolation or ignore them. Conflicts with P1-B07-D03 and with the refusal to invent broker behavior.

| Alternative | Identity impact | Completeness impact | Recovery impact | Posting, release, dispatch |
| --- | --- | --- | --- | --- |
| A | No invented order. | Coarse. | Safe and less informative. | Still prohibited. |
| B | Markers are not matches. | The hole is visible. | Still no resubmit. | Still prohibited. |
| C | Synthetic rows look real. | False coverage. | Can hide a live order. | Still prohibited, and not ratifiable. |

**Non-binding recommendation:** A until a capture implementation exists. B is the evidence-rich candidate and must not be read as a broker sequence. This recommendation is not an approval.

**Evidence needed:** the actual end-of-response and paging signals. Not retrieved.

**Dependencies:** BROKERQ-06, BROKERQ-07. Independent of XC-07-ROUND.

**Would not authorize:** backfill sends, synthetic executions, or a release.

## BROKERQ-09 — Reconnect

**Status:** `USER_DECISION_PENDING`  
**Ratification:** `NOT_RATIFIED`

**Question:** What happens to broker identifiers when the connection is established again?

**Binding rules:** DET-03 recovery text: session-specific `orderId` values are not interpreted without session and client context. BA-T02 and RC-T10 are `NOT_EXECUTED`. Adapter session epoch is not `WriterEpoch` (UD-02, UD-04). Reconnect does not prove the previous send did not happen (CR-B01 §1.2 item 8).

**Sources:** DET-03. DET-05-D05. CR-B04 T02 and T03. UD-02.

**Unresolved:** which ids survive a gateway restart. `EXTERNAL_EVIDENCE_PENDING`. XC-01 and the XC-04 remainder stay OPEN.

**Alternatives:**

- **A — Discard prior broker ids** on every reconnect.
- **B — New session context, prior evidence kept.** Ids are not carried across the boundary without a retained binding. A binding is still not a posting and not a resubmit.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** Reuse the previous session map and resubmit anything missing. Conflicts with DET-03, CR-B01, and ARCH-DEC-11.

| Alternative | Identity impact | Completeness impact | Recovery impact | Posting, release, dispatch |
| --- | --- | --- | --- | --- |
| A | Loses history that might still be the same order. | The new snapshot starts empty of links. | Review cannot see the old id. | Still prohibited. |
| B | Old and new stay distinct. | The new response does not complete the old window. | Uncertain sends stay uncertain. | Still prohibited. |
| C | Wrong orders merge. | A miss looks like a cancel. | Second send. | Still prohibited, and not ratifiable. |

**Non-binding recommendation:** B. A discards retained evidence. This recommendation is not an approval.

**Evidence needed:** P0-B04-T02 and T03. `NOT_EXECUTED`.

**Dependencies:** BROKERQ-01, BROKERQ-10, BROKERQ-11.

**Would not authorize:** resubmit, takeover, or a posting.

## BROKERQ-10 — Unknown send

**Status:** `USER_DECISION_PENDING`  
**Ratification:** `NOT_RATIFIED`

**Question:** Which crash-window labels may be stored for a submission attempt, and which of them allow another send?

**Binding rules:** CR-B01 §1.3 and §1.4. SYS-INV-02. BA-INV-03. RC-INV-01. ARCH-DEC-03. ARCH-DEC-11: no closure and no automatic retry or takeover on an unknown send. ARCH-DEC-34: recovery does not resubmit. After `DISPATCH_STARTED`, timeout, empty list, and restart are not `DEFINITIVELY_NOT_SENT`.

**Sources:** CR-B01 §1.3 and §1.4. DET-07-D03, read under CR-B01 precedence. ARCH-DEC-11. ARCH-DEC-34.

**Unresolved:** how a process would prove that a socket write did not start after `DISPATCH_STARTED`. That proof is not defined here and is not available. XC-04 remainder stays OPEN.

**Alternatives:**

- **A — One unknown label** for every attempt that passed `PREPARED`. No finer window. No resend.
- **B — Window labels, no resend.** Store the CR-B01 window as a label. After `DISPATCH_STARTED`, the label is possibly sent. No window in this package authorizes a resend, a cancel, or a release.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** Timeout or an empty snapshot means not sent, then retry. Conflicts with CR-B01 §1.3, ARCH-DEC-03, and ARCH-DEC-11.

| Alternative | Identity impact | Completeness impact | Recovery impact | Posting, release, dispatch |
| --- | --- | --- | --- | --- |
| A | Coarse and safe. | Snapshot cannot clear it. | No retry. | Still prohibited. |
| B | The phase is visible. | Still not a broker acceptance. | Still no retry. | Still prohibited. |
| C | A missing row becomes an identity conclusion. | Empty snapshot is misread. | Double send. | Still prohibited, and not ratifiable. |

**Non-binding recommendation:** B, with every window non-authorizing. A is also fail-closed. This recommendation is not an approval.

**Evidence needed:** fault-injection notes. ARCH-DEC-31 marks those tests `NOT_EXECUTED`. They were not run.

**Dependencies:** BROKERQ-11, BROKERQ-12. Does not approve a dispatch implementation.

**Would not authorize:** retry, takeover, cancel, posting, or release.

## BROKERQ-11 — Epoch fencing and broker acceptance

**Status:** `USER_DECISION_PENDING`  
**Ratification:** `NOT_RATIFIED`

**Question:** How are a database epoch and a broker acceptance kept distinct?

**Binding rules:** ARCH-DEC-03. CR-B01 §1.1: a PostgreSQL lease is not external broker fencing. P0-B01-INV-02: no successor session while the previous sender is not proved isolated. UD-04: `WriterEpoch` is not inferred from the shell counter and is not `AdapterSessionEpoch`. Lease expiry is not proof of safe takeover.

**Sources:** CR-B01 §1.1, §1.2, §1.4. DET-07 fencing-limit paragraph, subordinate to CR-B01. UD-04. Architecture consolidation note that ARCH-DEC-03 is design precedence, not proved IBKR behavior.

**Unresolved:** dispatch-scope boundary, lease heartbeat, and real session exclusivity. XC-04 remainder stays OPEN. No TTL is chosen.

**Alternatives:**

- **A — Epoch only as a local integer,** with no recorded relationship to attempts.
- **B — Local fence record, distinct from acceptance.** Store epoch, lease deadline, and authority label as local facts. Broker acceptance is a separate observation. A successor stays blocked while a prior writer may send or an attempt is unknown.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** Lease expiry or a new epoch means the broker rejected the old writer and the new writer may send. Conflicts with ARCH-DEC-03 and CR-B01 §1.1.

| Alternative | Identity impact | Completeness impact | Recovery impact | Posting, release, dispatch |
| --- | --- | --- | --- | --- |
| A | Epoch is disconnected from attempts. | No effect. | Harder to see why a send was blocked. | Still prohibited. |
| B | Fence is not a broker id. | A new epoch does not complete a snapshot. | Successor stays blocked. | Still prohibited. |
| C | Confuses fence with broker state. | Lease expiry looks like a full snapshot. | Automatic takeover. | Still prohibited, and not ratifiable. |

**Non-binding recommendation:** B. This recommendation is not an approval. It does not claim the fence is implemented.

**Evidence needed:** isolation proof for a real process and socket. Not present. `NOT_EXECUTED`.

**Dependencies:** BROKERQ-10, BROKERQ-12. XC-05-LOCK stays OPEN and is not decided here.

**Would not authorize:** a lease implementation, a heartbeat, a second send, or a broker call.

## BROKERQ-12 — Manual escalation

**Status:** `USER_DECISION_PENDING`  
**Ratification:** `NOT_RATIFIED`

**Question:** What is an escalation when identity, completeness, or an unknown send cannot be resolved?

**Binding rules:** ARCH-DEC-13: manual review is not authorization to post. ARCH-DEC-16 and ARCH-DEC-33: no adjudicator role is added; if a second person is required and absent, deny. CR-B05 P1-B05-D01: no simulated dual approval. XC-09-SOLO is that principle. XC-09-RESET stays OPEN.

**Sources:** ARCH-DEC-13, ARCH-DEC-16, ARCH-DEC-33. CR-B05. DET-08-D05 manual reset text, which is not adopted as a role or as a trading restart.

**Unresolved:** who may reset a halt, and what evidence a reset would require. Not decided. No person is invented.

**Alternatives:**

- **A — No escalation marker.** The row simply stays quarantined.
- **B — Marker without a role.** An escalation flag says a human decision is required later. It names no role, grants no post, no release, and no dispatch. A missing required second person denies.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** Invent an operator who may post, release, and resubmit from the quarantine queue. Conflicts with ARCH-DEC-13, ARCH-DEC-16, and ARCH-DEC-33.

| Alternative | Identity impact | Completeness impact | Recovery impact | Posting, release, dispatch |
| --- | --- | --- | --- | --- |
| A | Ambiguity stays implicit. | Incomplete stays incomplete. | No false approver. | Still prohibited. |
| B | The need for review is visible. | Does not complete the snapshot. | Does not take over the writer. | Still prohibited. |
| C | A person-shaped bypass. | Review is mistaken for coverage. | Resubmit and release. | Still prohibited, and not ratifiable. |

**Non-binding recommendation:** B. This recommendation is not an approval.

**Evidence needed:** none for the marker. A future role would be a separate XC-09-RESET decision. Not made.

**Dependencies:** BROKERQ-02, BROKERQ-10, BROKERQ-13. Does not decide reset.

**Would not authorize:** a role, a posting, a release, or a dispatch.

## BROKERQ-13 — Evidence authority

**Status:** `USER_DECISION_PENDING`  
**Ratification:** `NOT_RATIFIED`

**Question:** When a broker observation and a local record disagree, which one wins?

**Binding rules:** DET-08-D03: no blanket "broker wins" and no blanket "database wins". ARCH-DEC-17: no broker observation is authoritative terminal. ARCH-DEC-13: quarantine the economic effect. Receive order is not priority (DET-05-D04).

**Sources:** DET-08-D03. DET-05-D04. ARCH-DEC-17. ARCH-DEC-13.

**Unresolved:** a future, evidenced rule for a specific disagreement. Not in this package. XC-08-IBKR and XC-02-PRED stay OPEN.

**Alternatives:**

- **A — Broker text outranks local records** for status and quantity.
- **B — Retain both, infer nothing economic.** Observed bytes stay observed. Local attempt facts stay local facts. A comparison note is inference and is labeled as inference. Neither side posts, releases, or cancels.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** The local row wins and broker disagreements are dropped. Conflicts with append-only evidence (DET-03-D05, DET-05-D01) and with DET-08-D03.

| Alternative | Identity impact | Completeness impact | Recovery impact | Posting, release, dispatch |
| --- | --- | --- | --- | --- |
| A | Broker labels become identity. | One payload looks complete. | Conflicts with ARCH-DEC-17. | Still prohibited. A is not recommended. |
| B | Match stays unprivileged. | Disagreement stays visible. | No auto-cancel. | Still prohibited. |
| C | Local guess drops broker facts. | Gaps are deleted. | Audit trail breaks. | Still prohibited, and not ratifiable. |

A is not marked `CONFLICTS_WITH_BINDING_POLICY` because DET-08 leaves room for a later evidenced rule, but A is not available as a terminal or posting rule under ARCH-DEC-17 and ARCH-DEC-13. It must not be ratified as authority to post, release, or accept a broker terminal state.

**Non-binding recommendation:** B. This recommendation is not an approval.

**Evidence needed:** paired examples of disagreement. Not in this repository.

**Dependencies:** BROKERQ-02, BROKERQ-04, BROKERQ-07. Does not close XC-02-PRED.

**Would not authorize:** a winner, a posting, a release, or a cancel.

## BROKERQ-14 — Future verification bar

**Status:** `USER_DECISION_PENDING`  
**Ratification:** `NOT_RATIFIED`

**Question:** What must a later package show before any broker identity or completeness hypothesis may be called evidenced?

**Binding rules:** CR-B04 §4.5 and CR-B07 verification-open notes. ARCH-DEC-04: `DESIGN_RESOLVED` is not verification. Documentation tests are not broker evidence. `broker_uniqueness_evidence` stays `EVIDENCE_PENDING`. All 78 ADFR `verification` cells stay `NOT_VERIFIED`.

**Sources:** CR-B04 §4.5. CR-B07. `contracts/source-manifest.json` evidence flags. `contracts/traceability-index.md` broker-evidence paragraph.

**Unresolved:** the entire external corpus. No URL is invented.

**Alternatives:**

- **A — Accept the written hypotheses** in CR-B04 §4.2 as verified because they are in the repository.
- **B — External bar, nothing executed now.** A future package must include official material actually obtained, reproducible cases, environment, broker product and API versions, timestamps, raw payloads, and the negative cases in the coverage draft. Until then every external case stays `NOT_EXECUTED` and `EXTERNAL_EVIDENCE_PENDING`. This documentation package does not meet that bar.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** Treat this file, `DESIGN_RESOLVED`, or a passing documentation test as broker evidence. Conflicts with ARCH-DEC-04 and with the manifest rule that evidence pending is independent of presence.

| Alternative | Identity impact | Completeness impact | Recovery impact | Posting, release, dispatch |
| --- | --- | --- | --- | --- |
| A | Hypotheses become false facts. | A draft tuple looks proved. | Recovery would trust an unverified scope. | Still prohibited. A is not recommended. |
| B | Uniqueness stays pending. | Completeness stays pending. | No test is pretended to have run. | Still prohibited. |
| C | The index would lie. | A unit test would look like IBKR. | Unsafe to build on. | Still prohibited, and not ratifiable. |

A conflicts with the evidence-pending rule in practice. It is not given the conflict tag only because the source file does contain the hypotheses; ratifying A as verification would still contradict ARCH-DEC-04 and the manifest. It is not recommended and is not a verification.

**Non-binding recommendation:** B. This recommendation is not an approval.

**Evidence needed:** the future package itself. Absent. `EXTERNAL_EVIDENCE_PENDING`.

**Dependencies:** all earlier BROKERQ items. Passing B's bar would still not approve posting, release, an OMS transition, or live trading.

**Would not authorize:** a verification promotion, an ADFR status change, a broker call, or live trading.

## What a later decision would still not do

A user decision on BROKERQ-01 through BROKERQ-14 may record a future design constraint. It would still leave implementation blocked. It would not approve a canonical identity tuple, a uniqueness index, a resolver, economic posting, an economic-finality predicate, reservation release, an OMS transition, broker dispatch, or live trading. `XC-01` and `XC-08-IBKR` stay OPEN unless a later decision is explicitly about closing them, and none of the options above is that closer. Option C is not available. Live trading stays NOT AUTHORIZED.

## Resolution index (2026-11-15, TRD-ARCH-03B.3.2)

The questions BROKERQ-01 through BROKERQ-14 above are unchanged historical text, including their alternatives, consequence tables, and non-binding recommendations. They are not rewritten as approvals. The creation-time status lines stay as they were written. This addendum does not change those lines and does not record a second decision. The historical proposal index stays `NOT_RATIFIED`.

On 2026-11-15 the user ratified a design-policy choice for each question. The authority is that user decision. The decision record is [ARCH-DEC-35-bis-48-2026-11-15.md](../decisions/ARCH-DEC-35-bis-48-2026-11-15.md). The machine-readable map is [TRD-ARCH-03B32-resolution-index.json](TRD-ARCH-03B32-resolution-index.json). That map is `NON_RUNTIME_NON_WIRE_RESOLUTION_INDEX`. It is not a signature and not an evidence URI. `evidence_uri` is null. `signature_artifact` is null. The record labels there are documentary. They are not a canonical status enum. **NO IMPLEMENTATION.** **NO VERIFICATION.**

| Question | Decision | Option | User wording |
| --- | --- | --- | --- |
| BROKERQ-01 | ARCH-DEC-35 | B | Vorhandene Broker-/Konto-/Umgebungs-/Session-/Aliasfelder, Client und Adapter-Epoch sowie Evidence-ID nur soweit tatsächlich geliefert als mögliche zukünftige Evidenzkomponenten erfassen. Fehlend bleibt fehlend. Feldgleichheit ist weder Merge noch Uniqueness; kein Schema. |
| BROKERQ-02 | ARCH-DEC-36 | B | Zukünftige Kandidatenmenge `A(e)` und Unsicherheitslabel erlauben; selbst `|A(e)|=1` ist `SINGLE_CANDIDATE_NOT_AUTHORIZATION`; 0/mehrere/Widerspruch ebenso nicht autorisierend. Keine automatische Zuordnung mit wirtschaftlicher Wirkung. |
| BROKERQ-03 | ARCH-DEC-37 | B | Beobachtung auch ohne `permId` behalten; fehlend/null/zero nicht stillschweigend gleichsetzen. Spätere `permId` als neue Evidenz, kein Overwrite. Null-/Zero-Semantik bleibt extern ungeprüft. |
| BROKERQ-04 | ARCH-DEC-38 | B | Identischer Rohpayload bei Wiederholung idempotent als erneut gesehen; abweichender Payload unter scheinbar gleichem Schlüssel `CONTRADICTION`, beide Originale erhalten. Keine zweite Ausführung/Buchung, keine Bytes löschen. |
| BROKERQ-05 | ARCH-DEC-39 | B | Correction/Bust append-only mit Original-Evidenz verknüpfen, nur wenn die neue Evidenz selbst einen Bezug liefert. Kein erfundener Link, keine Reversal-Buchung, kein Release, keine Closure-Änderung. |
| BROKERQ-06 | ARCH-DEC-40 | B | Angefragten Account-/Client-/Session-/Dataset-/Zeitfenster-Scope soweit vorhanden dokumentieren; Coverage bleibt `UNKNOWN`, nicht global, keine `COMPLETE`-Behauptung. |
| BROKERQ-07 | ARCH-DEC-41 | A | Keine Completeness-Tokens speichern. Kein `COMPLETE`-Marker und kein externer Token-Platzhalter als Ratifikation; leere Antwort ist kein Nicht-Existenz-/Finalitäts-/Release-Beweis. |
| BROKERQ-08 | ARCH-DEC-42 | B | Nur tatsächlich lokal beobachtete Capture-Gaps (z.B. kurze Seite, fehlendes Antwortende, Drop) markieren; kein erfundener IBKR-Sequenzzähler, keine Interpolation, Scope bleibt unvollständig, keine Folgeaktion. |
| BROKERQ-09 | ARCH-DEC-43 | B | Reconnect erzeugt neuen Session-Kontext; alte Evidenz behalten, IDs nicht ohne nachgewiesene Bindung über Session-Grenzen übernehmen. Reconnect ist kein Send-Negativbeweis. |
| BROKERQ-10 | ARCH-DEC-44 | B | CR-B01-Crash-/Dispatch-Fenster als Labels dokumentieren. Nach `DISPATCH_STARTED` möglicherweise gesendet; kein Fenster dieses Pakets erlaubt Resend, Cancel, Release oder Dispatch. |
| BROKERQ-11 | ARCH-DEC-45 | B | Lokale Fence-Fakten (Epoch, Lease-Deadline, Authority-Label) von Broker-Akzeptanz strikt trennen; Successor blockiert solange früherer Writer noch senden könnte oder Sendestatus unbekannt ist. Lease-Ablauf/neuere Epoch beweist kein externes Fencing. Keine Implementierung oder TTL. |
| BROKERQ-12 | ARCH-DEC-46 | B | Nur nicht-autorisierenden Eskalationsmarker für spätere menschliche Prüfung vorsehen; keine Rolle, kein Post-/Release-/Dispatch-/Reset-Recht. Fehlende erforderliche zweite Person bedeutet Deny, nicht simulierte Zustimmung. |
| BROKERQ-13 | ARCH-DEC-47 | B | Broker-Rohbeobachtung und lokale Attempt-Fakten bei Widerspruch beide erhalten; Vergleich als Inferenz kennzeichnen. Keine Seite gewinnt pauschal, kein wirtschaftlicher Schluss, kein Cancel. |
| BROKERQ-14 | ARCH-DEC-48 | B | Für spätere Verifikation tatsächlich beschaffte offizielle Unterlagen, reproduzierbare positive/negative Fälle, Umgebung, Brokerprodukt/API-Version, Zeitstempel und Rohpayloads verlangen. Heute alles `NOT_EXECUTED`/`EXTERNAL_EVIDENCE_PENDING`; Dokumentations-/Unit-Tests sind keine Broker-Evidenz. |

The proposal package above stays a proposal. Its header remains the creation-time request. The later record approves design policy only. Options are B, B, B, B, B, B, A, B, B, B, B, B, B, B. Option C stays `CONFLICTS_WITH_BINDING_POLICY`. BROKERQ-02 option B and BROKERQ-08 option B were ratified where the non-binding recommendation had been A. BROKERQ-13 option A is not authority. BROKERQ-14 option A is not verification. The later record does not approve a canonical identity tuple, a uniqueness index, a resolver, a schema, economic posting, an economic-finality predicate, reservation release, an OMS transition, broker dispatch, a role, a resubmit, or live trading. Live trading stays NOT AUTHORIZED. `XC-01` stays OPEN. `XC-08-IBKR` stays OPEN. `XC-02-PRED` stays OPEN. The XC-04 operational remainder stays OPEN. XC-04 design precedence stays `RESOLVED_BY_ARCH_DEC_03`. CR-B01 precedence stays in force. ARCH-DEC-13 and ARCH-DEC-26 stay binding. No evidence record is created here.
