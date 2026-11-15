# TRD-ARCH-03B.1 — OMS approval requests

**Date:** 2026-11-15  
**Role:** decision requests. `document_role` = `NON_RUNTIME_NON_WIRE_PROPOSAL`.  
**Ratification:** `NOT_RATIFIED`. Every question is `USER_DECISION_PENDING`.  
**Authority:** the user. This file does not record a decision.  
**XC-03-MATRIX:** OPEN.  
**Live trading:** NOT AUTHORIZED.

A recommendation is a review aid. It is not an approval, not an ARCH-DEC record, and not permission to implement. No broker evidence was supplied. None is invented.

## Why there are twelve questions

The task asked for at least ten independently decidable issues. Two of the listed topics each contained two decisions:

- Cancel/fill-race semantics can be stated without authorizing modify or replace. Modify stays blocked on broker evidence (XC-10-ACTION). Those are OMSQ-04 and OMSQ-12.
- The local unit-of-work principle is already a `NO_CONFLICT_FOUND` reading (XC-05-OUTBOX). The lock order (XC-05-LOCK) and the audit authority (XC-06) are different open findings. Those are OMSQ-10 and OMSQ-11.

The other eight topics are one question each (OMSQ-01 through OMSQ-03 and OMSQ-05 through OMSQ-09). A decision on one question does not decide another. Consistency constraints are noted where a later implementation would have to satisfy both answers. Those constraints are not dependencies that force a bundled vote.

Shared binding context for every question: ARCH-DEC-01 through ARCH-DEC-08, as recorded on 2026-11-15. DET-01 and DET-02 Q01–Q10 are baseline principles with verification cells OPEN. Reference designs are not individual decisions. ARCH-DEC-04 gives no precedence to `OPEN P0`, `PROPOSED_FOR_ACCEPTANCE`, or `DESIGN_RESOLVED`. CR-B01 design precedence over conflicting or less restrictive DET-07 text remains in force (ARCH-DEC-03). `ORDER_STATE_MACHINE_IMPLEMENTED` and `STATE_TRANSITIONS_IMPLEMENTED` stay false.

## OMSQ-01 — Closure record mechanics

**Status:** `USER_DECISION_PENDING`.

**Question:** When a historical workflow closure is recorded, is that closure fact written once and left byte-stable, with later economic facts stored only as separate append-only evidence, or may an explicit append-only correction record be added beside it without reopening the workflow and without editing the original closure row?

**Sources:** DET-01 §3, §5, DET01-Q08, DET01-Q09, OMS-INV-03. CR-B03 §3.1 and P0-B03-INV-03. CR-B10 §2. UD-07. ADFR-DET01-001 and ADFR-DET01-009 (`NOT_VERIFIED`).

**Already binding:** A historical workflow closure is not deleted and is not silently reopened. Corrections are append-only and keep original evidence. The original `closure_reason` is not overwritten (CR-B03 P0-B03-INV-03, source text, `NOT_REVIEWED`). Late economic facts remain evidence.

**Still open:** Whether "immutable" means one row and no further closure-shaped record, or one original row plus a later correction record that points at it. The reason vocabulary is open (OMSQ-02). XC-03-MATRIX stays OPEN.

**Alternatives:**

| Id | Alternative |
| --- | --- |
| A | Write the closure row once. Later facts are ledger and reconciliation evidence only. No second closure-shaped record. |
| B | Write the closure row once, and allow a later append-only correction record that cites the original row, states the new evidence id, and does not change workflow state and does not edit the original reason. |
| C | Leave the mechanics unspecified. Do not add `OrderClosure` until a later decision. Fail closed by having no closure writer. |

**Consequences:**

| Alternative | Safety | Complexity | Recoverability | Auditability |
| --- | --- | --- | --- | --- |
| A | Highest stability of the closure bytes. A mistaken reason can be corrected only by a new incident, not by a closure correction. | Lowest record count. | Replay reads one row. A wrong reason stays visible and must be explained elsewhere. | Clear original. The explanation of a mistake lives outside the closure. |
| B | Original bytes stay stable if the correction is forbidden from updating them. A buggy correction writer could still be mistaken for a reopen. | A second record type and a link rule. | Replay must apply corrections in evidence order without sorting by receipt time. | Both the original and the correction are visible. |
| C | No premature writer. | No new type. | Recovery cannot show a closure that was never defined. | The gap stays explicit. |

**Recommendation (not an approval):** A. It matches the smallest reading of OMS-INV-03 and P0-B03-INV-03. Alternative B remains available if the user wants the correction record; it is compatible with A’s immutability only when the original row is never updated. C is the fail-closed option until the user chooses.

**External evidence required:** none to choose A, B, or C as a policy. Implementation would still need OMSQ-02 and a matrix that does not exist.

## OMSQ-02 — Closure-reason strategy

**Status:** `USER_DECISION_PENDING`.

**Question:** Which modeling strategy should a future vocabulary follow: minimal reason families with typed evidence (option A in the vocabulary note), richer discriminated reasons (option B), or a reason family plus independent terminality slots (option C)? This question does not select the token strings.

**Sources:** [TRD-ARCH-03B1-OrderClosure-vocabulary-options.md](TRD-ARCH-03B1-OrderClosure-vocabulary-options.md). DET-01 §3. UD-07. CR-B10 §1 and §2. ARCH-DEC-01 exceptions list.

**Already binding:** The reason is separate from workflow state. Workflow closure is not economic finality. The concrete vocabulary is unconfirmed. No enum is authorized by the baseline.

**Still open:** The strategy and the minimum evidence for each family or token. Token strings. Edges that write them (XC-03-MATRIX).

**Alternatives:** A, B, and C as defined in the vocabulary note. C is the conservative fail-closed shape because incomplete broker and economic slots stay explicit. A is coarser. B is the rich discriminator.

**Consequences:**

| Alternative | Safety | Complexity | Recoverability | Auditability |
| --- | --- | --- | --- | --- |
| A | Strong if evidence ids are mandatory. Coarse families can hide which fact was missing. | Low. | Replay joins evidence. | Depends on the evidence record. |
| B | Weaker if a token is read as finality. Stronger sampling if tests forbid that reading. | High. Each token needs its own guard. | A wrong token is difficult to repair under immutability. | Readable without a join, and easier to over-read. |
| C | Incomplete slots block a finality reading. | Medium. Four evidence slots. | Replay shows the gap that existed at closure time. | The gap is on the closure row itself. |

**Recommendation (not an approval):** C, for the reasons in the vocabulary note section 5. The recommendation does not approve tokens or a type.

**External evidence required:** OMSQ-01; later, XC-01, XC-02-PRED, and XC-08-IBKR before any broker or economic slot can be marked sufficient. Those findings are not decided here.

## OMSQ-03 — Unknown send and workflow closure

**Status:** `USER_DECISION_PENDING`.

**Question:** May an unknown broker-send outcome ever be the evidence that closes a workflow, and if a later closure is allowed at all, what additional evidence and incident handling are required?

**Sources:** DET-01 §7, OMS-T12. DET-07-D03, EX-INV-02. CR-B01 §1.2 items 6–8 and §1.3. ARCH-DEC-03. `DispatchOutcome::MayHaveSent`.

**Already binding:** Timeout, restart, empty list, and lease expiry are not proof of non-send. After `DISPATCH_STARTED`, the attempt is possibly sent. No automatic second submit. No automatic takeover while results are unresolved. `ACKNOWLEDGED` is not economic finality.

**Still open:** Whether any future closure path exists once stronger evidence arrives, and what incident must be open while the outcome is unknown. XC-04 remains OPEN beyond design precedence. XC-08-IBKR remains OPEN.

**Alternatives:**

| Id | Alternative |
| --- | --- |
| A | Unknown outcome never closes the workflow. The attempt stays in reconciliation. A later closure requires a new authorized command and positive evidence that is not the timeout. |
| B | Unknown outcome may close the workflow as an explicit "outcome unknown" closure that leaves risk and economics open. |
| C | Unknown outcome may close the workflow and release local submission intent, while risk release stays separately forbidden. |

**Consequences:**

| Alternative | Safety | Complexity | Recoverability | Auditability |
| --- | --- | --- | --- | --- |
| A | Fail closed. No hidden send. | The workflow stays open or in `ReconciliationRequired` until evidence exists. The edge to that label is still unspecified. | Restart resumes reconciliation. No blind retry. | The unknown attempt remains the audit fact. |
| B | A closure row would exist while a send might be live. Readers can mistake it for completion unless slots from OMSQ-02 C are mandatory. | A new reason that must not release risk. | Harder, because operators look at closure first. | The unknown is labeled, and easy to under-weight. |
| C | Releases local intent that may still be at the broker. Conflicts with "possibly sent". | Looks simple and is unsafe. | A later fill has to reopen work the closure claimed was done. | The audit would show a closure the evidence does not support. |

**Recommendation (not an approval):** A. B and C treat unknown as a kind of done. The baseline treats unknown as possibly sent. A still allows a later, separately authorized closure when positive evidence exists. That later step is not approved by this recommendation.

**External evidence required:** in-flight send evidence and IBKR session behavior (XC-04, XC-08-IBKR). Not present.

## OMSQ-04 — Cancel and fill race

**Status:** `USER_DECISION_PENDING`.

**Question:** When a cancel request and a fill can both be true, which facts must be retained, and which guards must pass before any workflow closure or risk movement is even eligible?

**Sources:** DET-01 §5, OMS-T07. CR-B09 P1-B09-D02. CR-B10 §2 `CancelRequested` and §7 rule 6. DET-02 §5. XC-10-RACE. XC-10-ACTION stays a separate question (OMSQ-12).

**Already binding:** `CancelRequested` is not `Cancelled`. A cancel request is not proof the unknown submission was never sent. A recorded broker cancel does not erase executions or release risk. Cancel and fill races are required tests, and those tests are not executed.

**Still open:** The guards for writing any closure after a cancel observation when fills exist or might exist. Whether a cancel command is admissible during a submission halt (that action gate is OMSQ-12 and XC-10-ACTION). The race principle does not authorize the command.

**Alternatives:**

| Id | Alternative |
| --- | --- |
| A | Retain the cancel request, the cancel observation, and every fill as separate facts. Do not close the workflow and do not move risk until identity, snapshot completeness, and the economic predicate are all sufficient. Until then, quarantine or reconciliation. |
| B | A broker `Cancelled` observation may close the workflow immediately, while fills stay in the ledger and risk stays unchanged. |
| C | A broker `Cancelled` observation closes the workflow and releases the reservation except for filled quantity. |

**Consequences:**

| Alternative | Safety | Complexity | Recoverability | Auditability |
| --- | --- | --- | --- | --- |
| A | Fail closed. No early release. | More orders remain in reconciliation. | Late fills append without undoing a closure that was not written. | All three facts remain. |
| B | Closure can run ahead of completeness. Risk is held, which limits the damage. | Medium. Closure and ledger diverge by design, which C in OMSQ-02 can represent. | A premature closure must not be reopened; corrections have to carry the explanation. | The closure reason must not be read as "no fills". |
| C | Can release budget for quantity the broker still might fill. Conflicts with RISK-INV-04. | Appears simpler. | A later fill after release needs the compensating path in CR-B03 §3.2, which is not an approved posting rule. | The release looks authorized when it was not. |

**Recommendation (not an approval):** A. It is the conservative reading of XC-10-RACE plus RISK-INV-04. B is distinguishable and still forbids release, but it closes the workflow before completeness. C is not recommended.

**External evidence required:** XC-08-IBKR completeness and XC-01 identity before A’s "sufficient" test can succeed. XC-02-PRED before any later release. None of that evidence is in this repository.

## OMSQ-05 — Late fill, bust, and correction effects

**Status:** `USER_DECISION_PENDING`.

**Question:** For a late fill, a duplicate fill, a bust, or a correction, what ledger effect is allowed, and how is the historical closure preserved when the budget or reservation cannot cover the result?

**Sources:** DET-01 §5, OMS-INV-02, OMS-T04, OMS-T05, OMS-T06. DET-02 §5, RISK-T06, RISK-T07 (unexecuted). CR-B03 §3.1–§3.5. DET-08-D04. XC-03-BOOK. XC-02-PRED.

**Already binding:** A late fill is not discarded because the workflow is closed. The original fill is not overwritten. The original closure reason is not changed. Duplicate identity has one economic effect. If budget is insufficient, the evidence is kept and a risk incident or halt is required. Ambiguous identity is not automatically booked (CR-B04 §4.3). The executable predicate and the compensating entry after `Released` are not approved (XC-02-PRED).

**Still open:** The automatic-booking threshold beyond "single consistent match", the incident severity, and the arithmetic of the compensating entry. Those stay outside this OMS question’s power to approve.

**Alternatives:**

| Id | Alternative |
| --- | --- |
| A | On one consistent scoped match, append the ledger effect and recompute the projection. On any identity doubt, quarantine with no economic effect. On insufficient budget, keep the evidence, open an incident, and halt new risk-increasing submission for the affected scope. Never edit the closure. |
| B | Always quarantine economic effects, including the single consistent match, until a manual adjudication. Still never drop the evidence and never edit the closure. |
| C | Book every received fill immediately and net conflicts later. |

**Consequences:**

| Alternative | Safety | Complexity | Recoverability | Auditability |
| --- | --- | --- | --- | --- |
| A | Matches XC-03-BOOK. Depends on an identity scope that is not evidenced. | Needs the match rule and an incident path. | Replay is deterministic if the match key is stable. | Evidence, projection, and incident are separate. |
| B | Safer while XC-01 is open. Slower economics. | Manual queue. | Nothing is booked early, so less to unwind. | The queue is the audit. |
| C | Can double-book or book the wrong order. Conflicts with OMS-INV-02 and OMS-INV-06. | Low, and unsafe. | Corrections multiply. | The original mistake is hard to see once netted. |

**Recommendation (not an approval):** B until XC-01 has evidenced scopes, and A’s match rule only after that evidence and a user decision that adopts it. This recommendation does not adopt A now and does not approve a posting. If the user wants one choice today, the fail-closed choice is B.

**External evidence required:** XC-01 uniqueness evidence. XC-02-PRED and XC-07-ROUND before any rebooking quantity is computed. ARCH-03B.2 owns the risk posting. Not started.

## OMSQ-06 — Evidence predicate for economic finality

**Status:** `USER_DECISION_PENDING`.

**Question:** What evidence is required before economic finality may even be considered, given that this question must not approve a risk release?

**Sources:** DET-02 §2 DET02-Q06, §5, §10, RISK-INV-04. CR-B03 §3.3. CR-B07 P1-B07-D03. DET-01 OMS-INV-04. XC-02-PRED. XC-02-ALIGN.

**Already binding:** Release is not a consequence of workflow closure, timeout, cancel acknowledgement, or an incomplete snapshot. No unconfirmed broker status forces economic finality. The reference inequality in DET-02 is not a netting, margin, or stress rule. No executable predicate exists.

**Still open:** The predicate itself, budget metrics, and the compensating entry. This question asks only for the evidence bar a future predicate would have to clear. It does not define the predicate and does not authorize release.

**Alternatives:**

| Id | Alternative |
| --- | --- |
| A | No predicate is accepted. Economic finality stays undefined. Risk release stays forbidden. |
| B | A future predicate may be considered only when all of the following are cited: a single consistent execution identity, a completeness proof for the named snapshot scope, the full append-only correction chain, and a versioned valuation snapshot. Sufficiency of that list is not decided now. Release remains forbidden until DET-02 is separately decided. |
| C | Broker status `Cancelled` or `Filled`, plus workflow `Closed`, is enough to consider finality. |

**Consequences:**

| Alternative | Safety | Complexity | Recoverability | Auditability |
| --- | --- | --- | --- | --- |
| A | Fail closed. No false release. | No predicate to maintain. | Economics stay explicitly unfinished. | The gap is the record. |
| B | Raises the bar without turning it on. Still incomplete while XC-08-IBKR is open. | The list must be versioned so a later change is visible. | Replay can show which input was missing. | Each input is citable. |
| C | Treats labels as finality. Conflicts with OMS-INV-04 and RISK-INV-04. | Low, and unsafe. | Late busts arrive after release. | The audit would show a release the evidence does not support. |

**Recommendation (not an approval):** A as the decision now. B is a candidate checklist for a later DET-02 decision, not a predicate and not a release approval. C is rejected by the baseline principles and is listed only so the choice is explicit.

**External evidence required:** ARCH-03B.2 for any future predicate. XC-07-ROUND, XC-08-IBKR, and XC-01 before B could be evaluated. No risk posting is pre-approved.

## OMSQ-07 — Matrix coverage, negative edges, and versioning

**Status:** `USER_DECISION_PENDING`.

**Question:** What policy must a future transition matrix meet before any edge may be proposed for acceptance: full scenario coverage, explicit negative families, and a version, or a shorter positive-edge list?

**Sources:** DET-01 §6, DET01-Q10, OMS-T11. CR-B10 §2 and §10. UD-06. The coverage draft in this package. XC-03-MATRIX.

**Already binding:** Allowed edges are intentionally not listed. Declaration order is not lifecycle order. The enums do not implement `Ord` or `Default`. Unknown text fails closed. This package’s coverage rows are not approvals. `STATE_TRANSITIONS_IMPLEMENTED` stays false.

**Still open:** Whether the user will require the coverage families and the negative table as an acceptance bar, and how a matrix version is identified. Accepting the bar would still not accept any edge.

**Alternatives:**

| Id | Alternative |
| --- | --- |
| A | No matrix version is accepted until it covers every family in the coverage draft, states the negative readings as forbidden, names missing guards, and has its own version distinct from code and from schema version. Edges remain individually unapproved. |
| B | Accept a short positive-edge list first and add negative cases later. |
| C | Treat the current draft labels and their declaration order as the matrix. |

**Consequences:**

| Alternative | Safety | Complexity | Recoverability | Auditability |
| --- | --- | --- | --- | --- |
| A | Refuses an incomplete machine. | High before any code. | Versioned replay of the matrix text. | The version is citable under ARCH-DEC-07 if a later promotion is attempted. |
| B | Positive edges without negative tests miss timeout, race, and snapshot cases. | Faster, and weaker. | Hard to know which case was excluded. | The omissions are invisible. |
| C | Conflicts with UD-06 and DET-01 §6. Unknown text could be read as a default. | Lowest, and unsafe. | Not recoverable in a defined way. | There is no matrix document to audit. |

**Recommendation (not an approval):** A. It does not approve XC-03-MATRIX and does not approve an edge. B and C would weaken DET01-Q10.

**External evidence required:** none to adopt the bar. Every blocked family still needs its XC finding before an edge in that family can be proposed.

## OMSQ-08 — Replay conflicts and manual adjudication

**Status:** `USER_DECISION_PENDING`.

**Question:** Who may resolve a replay conflict or a contradictory evidence set, and what must they record before any economic effect?

**Sources:** DET-01 §4 item 2, §5, DET01-Q02, OMS-T02. DET-08-D03 and DET-08-D04. CR-B04 §4.3. DET-10-D04 and CR-B05 P1-B05-D01 (XC-09-SOLO). CR-B10 §7 rule 9.

**Already binding:** Same idempotency key and same payload return the stored result. A different payload is a conflict and no mutation. Contradictions are not resolved by receipt time, UUID order, or the last status string. Ambiguous broker identity is quarantine. Where an independent second person is required and absent, the answer is deny. A simulated second approval is not authorized.

**Still open:** Whether a single operator may adjudicate a quarantined economic conflict under break-glass, and which fields the adjudication record must contain. XC-09-RESET stays OPEN. Role enforcement is not implemented.

**Alternatives:**

| Id | Alternative |
| --- | --- |
| A | Automatic effect only for the identical-payload replay and for a single consistent identity match that OMSQ-05 has allowed. Every other conflict stays quarantined until an authorized manual record cites the evidence ids, the actor, the expected `OrderVersion`, and the reason. If the action class requires a second person and none exists, deny. |
| B | Any single operator may pick a winner and write the ledger, with an audit note. |
| C | Conflicts are dropped after a timeout. |

**Consequences:**

| Alternative | Safety | Complexity | Recoverability | Auditability |
| --- | --- | --- | --- | --- |
| A | Fail closed. Matches DET01-Q02 and XC-09-SOLO. | A queue and an authorization check that is not built. | Quarantine can be replayed. The manual record is a new command, not an edit. | Actor, evidence, and version are present, or the write is refused. |
| B | One person can move money on ambiguous evidence. | Low. | A bad choice needs a correction chain. | A note is weaker than a structured record. |
| C | Drops evidence. Conflicts with OPS-INV-05 and with late-fill retention. | Low, and unsafe. | The dropped fact is gone. | The timeout hides the conflict. |

**Recommendation (not an approval):** A. It does not name the role that is allowed to adjudicate, because XC-09-RESET has not fixed roles, reset evidence, or halt-version CAS. Until that exists, the practical effect of A is that conflicts stay quarantined.

**External evidence required:** XC-09-RESET and, for identity conflicts, XC-01. ARCH-03B.3 for broker-side conflicts. Not started.

## OMSQ-09 — Accepting a broker terminal observation

**Status:** `USER_DECISION_PENDING`.

**Question:** What is the minimum set of identity, completeness, and provenance conditions before a broker observation may be treated as terminal for the broker-lifecycle dimension?

**Sources:** DET-03 decisions, `NOT_REVIEWED`. CR-B04 §4.1–§4.5. CR-B07 P1-B07-D01–D04. DET-08-D02, RC-INV-04. CR-B10 §7 rules 2 and 8. XC-01. XC-08-IBKR. `BrokerLifecycleState` comments in `state.rs`.

**Already binding:** A bare broker id is not an internal primary key. Ambiguity is not an automatic merge. An incomplete snapshot is not absence and is not `DEFINITIVELY_NOT_SENT`. `Cancelled` does not erase executions or release risk. Kind-specific uniqueness is `EVIDENCE_PENDING`. CR-B04’s matrix is a hypothesis, not a broker guarantee.

**Still open:** The key fields per alias kind, the IBKR completion signal, history retention, and account scope. This question does not pretend those are known.

**Alternatives:**

| Id | Alternative |
| --- | --- |
| A | Do not accept any observation as terminal. `BrokerLifecycleState` labels may be stored as unverified text mappings only when the raw payload is kept, and they have no downstream effect. |
| B | A future acceptance rule must cite scoped identity, a named completeness proof for `(broker, account, connection, dataset, window)`, raw payload, adapter and normalizer versions, and received time. Until each of those is evidenced, acceptance fails closed as in A. Terminal acceptance still would not write workflow closure, economics, or risk release. |
| C | The normalized label `Cancelled`, `Rejected`, or `Expired` is terminal by itself. |

**Consequences:**

| Alternative | Safety | Complexity | Recoverability | Auditability |
| --- | --- | --- | --- | --- |
| A | Fail closed while evidence is pending. | No acceptance path. | Raw payloads remain for a later rule. | The raw payload is the record. The label is not authority. |
| B | States the bar without claiming it is met. | The scope tuple has to be versioned. | A failed check is explainable. | Each missing input is visible. |
| C | Promotes an unverified label. Conflicts with XC-01 and XC-08-RULE. | Low, and unsafe. | Wrong terminals are hard to undo without a reversal policy. | The label hides the missing snapshot. |

**Recommendation (not an approval):** A for current behavior, with B as the bar a later ARCH-03B.3 decision would have to meet. C is incompatible with the baseline. Neither A nor B approves a terminal transition.

**External evidence required:** broker uniqueness evidence and IBKR completeness evidence. Status remains `EVIDENCE_PENDING`. No evidence file is added by this package.

## OMSQ-10 — Lock order and isolation

**Status:** `USER_DECISION_PENDING`.

**Question:** Should the CR-B02 §2.2 lock order and the `READ COMMITTED` default in §2.3 be adopted as the candidate concurrency policy, or must they stay an unadopted proposal until deadlock tests exist?

**Sources:** DET01-Q04. CR-B02 §2.2 and §2.3. XC-05-LOCK. XC-05-OUTBOX is a different, already aligned principle and is not this question.

**Already binding:** CAS plus targeted locks are the baseline concurrency approach. The local unit of work excludes broker and NATS calls. A database retry is not a broker resend. CR-B02 itself says the lock order is a proposal and is not a substitute for deadlock tests that have not been run. `SERIALIZABLE` is allowed for selected transactions and is not the silent default.

**Still open:** Canonical order, isolation choice, and any proof the order is deadlock-free. Adopting the text as canonical would over-claim the source.

**Alternatives:**

| Id | Alternative |
| --- | --- |
| A | Keep the CR-B02 order and `READ COMMITTED` text as an unadopted proposal. Do not call it canonical. A future unit of work must fail closed on lock conflict by retrying the whole local unit, without a broker call. |
| B | Adopt the published order as canonical now, before deadlock tests. |
| C | Choose `SERIALIZABLE` for every trading transaction now. |

**Consequences:**

| Alternative | Safety | Complexity | Recoverability | Auditability |
| --- | --- | --- | --- | --- |
| A | Avoids a false canonical order. Fail closed on conflict. | The proposal must be re-checked before code. | No lock protocol to unwind because none is adopted. | The proposal’s status stays visible. |
| B | A wrong order deadlocks or, if ignored under pressure, races. | Enables earlier coding against an untested order. | Deadlocks need an operational playbook that is not written. | The audit would say "canonical" without a test record. ARCH-DEC-07 would not be satisfied. |
| C | Stronger isolation is not automatically correct, and CR-B02 says it is not the default. | Higher abort rate. | Retries must still be local only. | Aborts need a reason code that is not defined. |

**Recommendation (not an approval):** A. B would mark XC-05-LOCK resolved without the tests the source requires. C selects an isolation the source refused to make default.

**External evidence required:** deadlock and serialization tests that this package does not run. A later implementation task would have to supply them under ARCH-DEC-07 before any promotion.

## OMSQ-11 — Audit authority inside the unit of work

**Status:** `USER_DECISION_PENDING`.

**Question:** Which persisted artifact is the authoritative audit record for an OMS mutation, given that an outbox or export projection is not that authority?

**Sources:** DET-01 §4 item 4, OMS-INV-05. DET-04-D01. DET-10-D01 and DET-10-D02. CR-B08 P1-B08-D01 and D02. OPS-INV-01. XC-06. ADFR-CROSS-001 (`PROPOSED_FOR_ACCEPTANCE`, `NOT_VERIFIED`). XC-05-OUTBOX.

**Already binding:** An authoritative mutation carries transactional audit and event evidence. Export and NATS are not the audit authority. The local commit that includes the outbox does not by itself name the audit table. Source status words for this topic disagree, and ARCH-DEC-04 assigns them no precedence.

**Still open:** The artifact name and whether DET-10’s two levels create a second authority. Retention, signing keys, and backup are explicitly open in the source and are not decided here.

**Alternatives:**

| Id | Alternative |
| --- | --- |
| A | Require one in-transaction audit record distinct from the outbox row, and refuse the mutation if that record cannot be written. Do not name the table and do not adopt a hash chain. |
| B | Adopt the reference name `audit_events` from CR-B08 as the authority now. |
| C | Treat the outbox row as the audit authority. |

**Consequences:**

| Alternative | Safety | Complexity | Recoverability | Auditability |
| --- | --- | --- | --- | --- |
| A | Preserves "no commit without audit" without inventing a table. | The name stays a gap, so implementation cannot start. | Nothing to migrate. | The requirement is clear and the location is not. |
| B | Names an authority the sources have not agreed, under ARCH-DEC-04. | Unblocks a schema that this task forbids. | A later rename is a migration. | Stronger only if the disagreement is actually settled. It is not. |
| C | A publish failure or a projection bug becomes an audit gap. Conflicts with CR-B08 D02 and OPS-INV-01. | Low, and unsafe. | Export loss looks like history loss. | The projection is not the authority. |

**Recommendation (not an approval):** A. It keeps XC-06 OPEN. B would pick a winner among status words. C collapses audit into the outbox.

**External evidence required:** a later user decision that names the artifact after the DET-04 / DET-10 / CR-B08 disagreement is explicitly resolved. Not this package.

## OMSQ-12 — Modify and replace

**Status:** `USER_DECISION_PENDING`.

**Question:** May a modify or cancel-replace be represented as an OMS command before broker semantics and the risk delta are verified, or does modify stay blocked while cancel-replace is only a later new order after evidenced finality?

**Sources:** CR-B09 P1-B09-D01 and P1-B09-D03. DET-07-D05. GATE-P1-09 `EXTERNAL_VERIFICATION_REQUIRED`. XC-10-ACTION. There is no modify variant on `OrderWorkflowState`.

**Already binding:** Modify is a separate command class in the source proposal, and that proposal blocks it until broker semantics, cancel-replace identity, and the risk-delta reservation are verified. Those verifications are absent. `RISK_REDUCE` is a policy class, not a broker order type, and is deny when the effect is indeterminate. This question does not authorize cancel-during-halt. The cancel/fill race is OMSQ-04.

**Still open:** The entire modify path. Identity of a replacement order. Risk delta. IBKR capability.

**Alternatives:**

| Id | Alternative |
| --- | --- |
| A | No modify and no implicit cancel-replace. A replacement, if ever allowed, is a new order after a separately evidenced finality and a new risk decision. Until then the command is rejected. |
| B | Allow a modify command that is stored and not dispatched. |
| C | Allow cancel-replace as one dispatch now. |

**Consequences:**

| Alternative | Safety | Complexity | Recoverability | Auditability |
| --- | --- | --- | --- | --- |
| A | Fail closed. No second live order from an unproven replace. | No modify state. | Nothing in flight to reconcile for modify. | The rejection is the audit. |
| B | A stored modify can be dispatched later by mistake. | A command with no legal edge. | The stored command becomes recovery work. | Looks like an accepted modify. |
| C | Can double-submit while the original outcome is unknown. Conflicts with EX-INV-02 and ARCH-DEC-03. | High, and unsafe. | Two broker orders and an unclear link. | The link would not be evidenced. |

**Recommendation (not an approval):** A. It follows the CR-B09 proposal’s block without promoting that proposal to an executed gate. B and C create a command the label inventory cannot represent safely.

**External evidence required:** IBKR modify and cancel-replace semantics, identity scope (XC-01), and a risk-delta rule (XC-02-PRED). GATE-P1-09 stays `EXTERNAL_VERIFICATION_REQUIRED`. ARCH-03B.3 would have to bring that evidence. It is not in this task.

## What this list does not decide

- No OMSQ is `DECIDED`, `APPROVED`, or `USER_DECISION_RECORDED`.
- XC-03-MATRIX stays OPEN even if a later user accepts every recommendation.
- No `OrderClosure` vocabulary is approved.
- No transition edge is approved.
- Live trading stays unauthorized. Canonical schemas stay unpublished. `part_b` stays `NO-GO`. `broker_uniqueness_evidence` stays `EVIDENCE_PENDING`.

## Resolution index (2026-11-15, TRD-ARCH-03B.1.2)

The questions OMSQ-01 through OMSQ-12 above are unchanged historical text, including their alternatives, consequence tables, and recommendations. They are not rewritten as approvals. The creation-time status lines stay as they were written. This addendum does not change those lines and does not record a second decision.

On 2026-11-15 the user ratified a design-policy choice for each question. The decision record is [ARCH-DEC-09-bis-20-2026-11-15.md](../decisions/ARCH-DEC-09-bis-20-2026-11-15.md). The map below points at that record. It is not a signature and not an evidence URI. The record labels there are documentary. They are not a canonical status enum.

| Question | Decision | Option | User wording |
| --- | --- | --- | --- |
| OMSQ-01 | ARCH-DEC-09 | A | Ein ursprünglicher Abschlussdatensatz, spätere Fakten separat |
| OMSQ-02 | ARCH-DEC-10 | C | Grundfamilie plus unabhängige Finalitätsfelder |
| OMSQ-03 | ARCH-DEC-11 | A | Keine Schliessung ohne neue autorisierte Aktion und positive Evidenz |
| OMSQ-04 | ARCH-DEC-12 | A | Alle Fakten behalten; Closure und Risk nur bei ausreichender Evidenz |
| OMSQ-05 | ARCH-DEC-13 | B | Alle wirtschaftlichen Effekte zunächst manuell prüfen |
| OMSQ-06 | ARCH-DEC-14 | A | Kein Finalitätsprädikat akzeptieren; Risk Release verboten |
| OMSQ-07 | ARCH-DEC-15 | A | Vollständige Szenarien, Negativfälle und eigene Version verlangen |
| OMSQ-08 | ARCH-DEC-16 | A | Quarantäne und strukturierte autorisierte Entscheidung |
| OMSQ-09 | ARCH-DEC-17 | A | Keine terminale Brokerbeobachtung akzeptieren |
| OMSQ-10 | ARCH-DEC-18 | A | Sperrreihenfolge und Isolation bleiben unbestätigter Vorschlag |
| OMSQ-11 | ARCH-DEC-19 | A | Separaten transaktionalen Auditdatensatz verlangen |
| OMSQ-12 | ARCH-DEC-20 | A | Modify und implizites Cancel-Replace blockieren |

The proposal package above stays a proposal. Its header remains the creation-time request. The later record approves design policy only. It does not approve an `OrderClosure` vocabulary, a token list, a transition edge, a matrix version, an economic-finality predicate, automatic economic posting, risk release, broker terminal acceptance, modify or replace dispatch, a lock order, an isolation level, or an audit table. `XC-03-MATRIX` stays OPEN. OMSQ-05 option B overrides any reading of the OMSQ-08 proposal text that would post a single identity match automatically. OMSQ-06 option A leaves the predicate undefined, so OMSQ-04 does not release risk. OMSQ-02 option C is a modeling strategy only. OMSQ-09 option A accepts no broker observation as authoritative terminal. Live trading stays NOT AUTHORIZED. No evidence record is created here.
