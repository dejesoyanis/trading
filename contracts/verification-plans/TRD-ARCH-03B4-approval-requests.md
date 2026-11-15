# TRD-ARCH-03B.4 — IBKR evidence decision requests

**Date:** 2026-11-15  
**Role:** procedure questions for a future evidence package. `document_role` = `NON_RUNTIME_NON_WIRE_VERIFICATION_PLAN`.  
**Ratification:** `NOT_RATIFIED`.  
**Status of every question:** `USER_DECISION_PENDING`.  
**Execution:** `NOT_EXECUTED`.  
**External evidence:** `EXTERNAL_EVIDENCE_PENDING`.  
**Live trading:** NOT AUTHORIZED.  
**Canonical schemas:** not published. `part_b` = `NO-GO`. `broker_uniqueness_evidence` = `EVIDENCE_PENDING`.

A recommendation is not an approval. The questions are independent. Answering one does not answer another. Answering any of them does not authorize a broker connection, a read, an order, a test run, or a stage go. This file is not testing authority. Checkpoints for E0 through E4 stay STOP in the protocol note.

No question authorizes economic posting, an economic-finality predicate, reservation release, an OMS transition, a canonical schema, a uniqueness index, a completeness token, broker dispatch, or live trading.

ARCH-DEC-01 through ARCH-DEC-48 stay binding. An alternative marked `CONFLICTS_WITH_BINDING_POLICY` is contrast. It is not available for ratification unless a later decision explicitly supersedes the named binding rule. This package does not supersede those rules. Option C on every question below is that contrast.

`XC-01`, `XC-08-IBKR`, the operational remainder of `XC-04`, and `XC-02-PRED` stay OPEN.

## IBKREVQ-01 — Evidence source quality

**Status:** `USER_DECISION_PENDING`  
**Ratification:** `NOT_RATIFIED`

**Question:** What may a later package treat as official material for an IBKR claim?

**Binding constraints:** ARCH-DEC-48 requires official material that was actually obtained. ARCH-DEC-04: presence is not verification. This plan invents no URL and no document title.

**Sources:** ARCH-DEC-48 in `contracts/decisions/ARCH-DEC-35-bis-48-2026-11-15.md`. CR-B04 §4.5. `contracts/source-manifest.json` evidence flags.

**Implications:** A weak source would let a hypothesis be called evidenced without a primary text. A strict source leaves every current claim pending, which is the present state.

**Alternatives:**

- **A — Primary text plus version page.** Accept only material obtained from the broker's own documentation for the pinned version, archived with retrieval date. Commentary may be cited as commentary and is not evidence.
- **B — Primary text, and a second obtained copy.** Same as A, and require a second independently retrieved copy of the same versioned text before E4 may cite it.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** Treat this planning document, a forum post, or an unarchived paraphrase as the official source. Conflicts with ARCH-DEC-48 and ARCH-DEC-04.

| Alternative | Evidence effect | What stays blocked |
| --- | --- | --- |
| A | E0 can start only after a separate go, and only with obtained text. | Connection, orders, uniqueness. |
| B | E0 is slower and harder to fake. | Connection, orders, uniqueness. |
| C | Would relabel hypotheses as facts. | Not ratifiable. |

**Non-binding recommendation:** B. A second copy reduces a single bad retrieval. This recommendation is not an approval and does not start E0.

**Required evidence:** none until a go. The evidence this question asks for is itself absent.

**Dependencies:** IBKREVQ-02, IBKREVQ-14. Does not close `XC-01` or `XC-08-IBKR`.

## IBKREVQ-02 — Version pinning

**Status:** `USER_DECISION_PENDING`  
**Ratification:** `NOT_RATIFIED`

**Question:** How is a future evidence package bound to a product, API client, platform, OS, and account mode?

**Binding constraints:** CR-B04 §4.1 and §4.5. IBKR-CLM-26. ARCH-DEC-48. No version string is invented here.

**Sources:** CR-B04 §4.5 in `contracts/sources/P0-Resolution-Contracts-CR-B01-bis-CR-B04-v1.0.md`. ARCH-DEC-48. GATE-P1-07.

**Implications:** Mixing versions in one package makes a negative result ambiguous. Pinning one tuple leaves every other tuple uncovered, which matches the claim.

**Alternatives:**

- **A — One tuple per package.** The first future package names one product version, one API client version, one OS, and one account mode. Other tuples need other packages.
- **B — Mixed tuples allowed if every observation is labeled.** One package may hold several tuples, and no claim may span two tuples.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** Ignore version and treat any observation as universal. Conflicts with CR-B04 §4.5 and ARCH-DEC-48.

| Alternative | Evidence effect | What stays blocked |
| --- | --- | --- |
| A | The smallest honest package. | Other versions, uniqueness, live trading. |
| B | More rows in one review, still no cross-tuple claim. | Universality, uniqueness. |
| C | Would over-claim. | Not ratifiable. |

**Non-binding recommendation:** A. One tuple is easier to reject later. This recommendation is not an approval.

**Required evidence:** the tuple copied from components. `UNRECORDED`.

**Dependencies:** IBKREVQ-01, IBKREVQ-12. `XC-01` and `XC-08-IBKR` stay OPEN.

## IBKREVQ-03 — Scope taxonomy

**Status:** `USER_DECISION_PENDING`  
**Ratification:** `NOT_RATIFIED`

**Question:** Which scope labels may a future capture use before any read is authorized?

**Binding constraints:** ARCH-DEC-35 and ARCH-DEC-40. `P1-B07-D01`. Absence stays absence. Coverage stays `UNKNOWN`. No global snapshot.

**Sources:** CR-B07 `P1-B07-D01`. ARCH-DEC-35. ARCH-DEC-40. IBKR-CLM-11.

**Implications:** A fixed vocabulary can force a label the request did not carry. A supplied-only rule can leave the row sparse. Sparse is the binding policy.

**Alternatives:**

- **A — Supplied labels only.** Account, client, session, dataset, time window, and environment are stored only when the request or the payload actually carried them.
- **B — Reviewer vocabulary drafted before capture.** A later note may list allowed dataset names from CR-B07, and a capture may use a name only if the request also carried it. Unused names stay unused.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** Treat one response as a global snapshot. Conflicts with ARCH-DEC-40 and `P1-B07-D01`.

| Alternative | Evidence effect | What stays blocked |
| --- | --- | --- |
| A | Honest absences. | Completeness, release. |
| B | Same absences, with a checklist that cannot fill them. | Completeness, release. |
| C | Would claim coverage the request did not have. | Not ratifiable. |

**Non-binding recommendation:** A. It matches ARCH-DEC-35 and ARCH-DEC-40. This recommendation is not an approval and does not authorize a read.

**Required evidence:** the request fields, later. Absent now.

**Dependencies:** IBKREVQ-05, IBKREVQ-06. `XC-08-IBKR` stays OPEN.

## IBKREVQ-04 — Test environments

**Status:** `USER_DECISION_PENDING`  
**Ratification:** `NOT_RATIFIED`

**Question:** Which environment class may a future E2 or E3 authorization name?

**Binding constraints:** CR-B04 §4.5 says the identity tests belong in a controlled paper or sandbox environment and that live submission stays blocked without broker evidence. This plan does not approve paper. Live trading stays NOT AUTHORIZED. No account is accessed.

**Sources:** CR-B04 §4.5. ARCH-DEC-48. Protocol stages E2 and E3.

**Implications:** Naming an environment class in advance does not open the stage. Refusing to name one keeps the next authorization specific.

**Alternatives:**

- **A — Dedicated non-production account, named later.** A future authorization must name a dedicated test account that is not a production trading account. Paper is not pre-approved by this choice. Production is excluded.
- **B — Defer the class.** Do not choose paper, sandbox, or any other label until E0 material uses the broker's own environment words, then ask again.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** Use a production account or send a live order. Conflicts with live-trading prohibition and with CR-B04 §4.5.

| Alternative | Evidence effect | What stays blocked |
| --- | --- | --- |
| A | The later authorization still has to say go. | This milestone's checkpoint stays STOP. |
| B | No environment word is frozen from guesswork. | E2 and E3 stay closed. |
| C | Would be a live send. | Not ratifiable. |

**Non-binding recommendation:** B. The broker's own environment words are not in this repository. This recommendation is not an approval and does not approve paper trading.

**Required evidence:** E0 text that uses the broker's environment labels. Absent.

**Dependencies:** IBKREVQ-01, IBKREVQ-05, IBKREVQ-09. `XC-10-ACTION` stays OPEN.

## IBKREVQ-05 — Read-only limits

**Status:** `USER_DECISION_PENDING`  
**Ratification:** `NOT_RATIFIED`

**Question:** What may a future E2 authorization include if it is ever given?

**Binding constraints:** This plan authorizes no read. ARCH-DEC-44 authorizes no cancel. A read is still a broker request. Empty results stay non-proofs under ARCH-DEC-41.

**Sources:** Protocol stage E2. ARCH-DEC-41. ARCH-DEC-44. IBKR-CLM-11. IBKR-CLM-14.

**Implications:** Allowing an unnamed "read-only" bucket can hide a status-changing call. Naming operations later keeps E2 shut until the list exists.

**Alternatives:**

- **A — Named reads only, after a separate go.** A future E2 authorization lists each read operation. Anything not listed stays forbidden. No order, cancel, modify, or replace is eligible for that list.
- **B — No connection until the operation list exists.** Refuse even a later E2 go until a separate decision has named the exact read operations. This question does not name them.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** Treat cancel, modify, or status-changing calls as read-only. Conflicts with ARCH-DEC-44 and with the prohibition on broker actions.

| Alternative | Evidence effect | What stays blocked |
| --- | --- | --- |
| A | E2 can be specific if a later decision lists operations. | Orders and cancels. |
| B | E2 cannot start from this package at all. | Every broker call. |
| C | Would dispatch a cancel under a read label. | Not ratifiable. |

**Non-binding recommendation:** B. The operation names are unverified, so they are not listed here. This recommendation is not an approval.

**Required evidence:** official operation descriptions from E0. Absent. No method name is asserted.

**Dependencies:** IBKREVQ-01, IBKREVQ-03, IBKREVQ-04. `XC-08-IBKR` stays OPEN.

## IBKREVQ-06 — Completeness oracle

**Status:** `USER_DECISION_PENDING`  
**Ratification:** `NOT_RATIFIED`

**Question:** How does a future reviewer record the limit of a snapshot oracle without storing a completeness token?

**Binding constraints:** ARCH-DEC-41 stores no completeness token. An empty response is not a negative proof. ARCH-DEC-42 marks only a locally observed capture gap. ARCH-DEC-40 leaves coverage `UNKNOWN`.

**Sources:** ARCH-DEC-40, ARCH-DEC-41, ARCH-DEC-42. `P1-B07-D02`. IBKR-CLM-14. IBKR-CLM-15. RC-INV-04.

**Implications:** The oracle can say what the capture holds. It cannot say that the broker dataset ends. Those sentences must stay apart.

**Alternatives:**

- **A — Capture facts only.** The oracle states requested scope, bytes stored, and local gaps. Coverage remains `UNKNOWN`. No token field exists.
- **B — Separate written limit.** Same as A, plus a prose sentence in the future package that capture completeness and broker completeness are different and that broker completeness was not shown.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** Store a `COMPLETE` token when the list is empty or when a page ends. Conflicts with ARCH-DEC-41, `P1-B07-D02`, and RC-INV-04.

| Alternative | Evidence effect | What stays blocked |
| --- | --- | --- |
| A | The smallest record. | Release, non-existence, `XC-08-IBKR` closure. |
| B | The distinction is harder to skip at review. | The same blocks. |
| C | Would ratify a token. | Not ratifiable. |

**Non-binding recommendation:** B. The extra sentence is still not a token. This recommendation is not an approval and does not close `XC-08-IBKR`.

**Required evidence:** negative cases in the test matrix, all `NOT_EXECUTED`.

**Dependencies:** IBKREVQ-03, IBKREVQ-11. `XC-02-PRED` stays OPEN.

## IBKREVQ-07 — Alias uniqueness verification

**Status:** `USER_DECISION_PENDING`  
**Ratification:** `NOT_RATIFIED`

**Question:** What may a future package say about uniqueness of `orderId`, `permId`, or `execId`?

**Binding constraints:** P0-B04-INV-02. ARCH-DEC-35. CR-B04 §4.1. `broker_uniqueness_evidence` stays `EVIDENCE_PENDING`. A singleton is `SINGLE_CANDIDATE_NOT_AUTHORIZATION` (ARCH-DEC-36). No UNIQUE index is approved.

**Sources:** CR-B04 §4.1 and §4.4. ARCH-DEC-35. ARCH-DEC-36. IBKR-CLM-02. IBKR-CLM-06. IBKR-CLM-07. IBKR-CLM-27.

**Implications:** Experiments can falsify a global-uniqueness guess. They cannot install a constraint. Refusing experiments leaves the guess untested and still unclaimed.

**Alternatives:**

- **A — Uniqueness stays unclaimed.** A future package may report observations of reuse or of non-reuse inside one pinned tuple. It may not call any alias unique and may not add a UNIQUE index. A later index would be another decision.
- **B — No uniqueness experiments.** Do not design reuse trials even as future controlled cases. Keep `EVIDENCE_PENDING` with no additional rows aimed at uniqueness.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** Declare `orderId`, `permId`, or `execId` unique now and add a constraint. Conflicts with P0-B04-INV-02, CR-B04 §4.1, and ARCH-DEC-35.

| Alternative | Evidence effect | What stays blocked |
| --- | --- | --- |
| A | Reuse can be reported without a guarantee. | UNIQUE index, resolver, posting. |
| B | No extra trial design. | The same blocks, and less future data. |
| C | Would freeze an unevidenced key. | Not ratifiable. |

**Non-binding recommendation:** A. Reporting a reuse observation is not a uniqueness claim. This recommendation is not an approval and authorizes no experiment.

**Required evidence:** paired raw payloads. Absent. `XC-01` stays OPEN.

**Dependencies:** IBKREVQ-02, IBKREVQ-08. Does not depend on `XC-02-PRED`.

## IBKREVQ-08 — Correction chains

**Status:** `USER_DECISION_PENDING`  
**Ratification:** `NOT_RATIFIED`

**Question:** How are correction and bust links represented if a future payload arrives?

**Binding constraints:** ARCH-DEC-39. A link is stored only when the new evidence itself names it. No invented link. No reversal booking. ARCH-DEC-26 quarantines the economic effect. CR-B03 §3.2 is not adopted.

**Sources:** ARCH-DEC-39. DET-03-D05. CR-B04 §4.5 `T06`. ARCH-DEC-26. IBKR-CLM-08. Scenarios IBKR-SCN-A10 and IBKR-SCN-A11.

**Implications:** Storing a named reference preserves a review path. Storing no link field at all avoids a false chain. Both can refuse a reversal.

**Alternatives:**

- **A — Store the reference the payload already has.** Append the new observation. Copy a reference only from the new bytes. If the bytes name none, store none.
- **B — No link field in the capture.** Append both observations and leave any chain to a later decision. Do not copy a reference even if the bytes contain one; quote the bytes only as raw payload.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** Invent a link from timing or quantity and post a reversal. Conflicts with ARCH-DEC-39 and ARCH-DEC-26.

| Alternative | Evidence effect | What stays blocked |
| --- | --- | --- |
| A | The chain is only as strong as the payload. | Reversal, release, closure change. |
| B | The raw bytes still hold whatever reference they had. | A structured link, and the same economic blocks. |
| C | Would book a compensation. | Not ratifiable. |

**Non-binding recommendation:** A. It is the binding rule already. This recommendation is not an approval and authorizes no bust trial.

**Required evidence:** a raw correction example. Absent. T06 stays `NOT_EXECUTED`.

**Dependencies:** IBKREVQ-07, IBKREVQ-11. `XC-02-PRED` stays OPEN.

## IBKREVQ-09 — Unknown-send fault injection

**Status:** `USER_DECISION_PENDING`  
**Ratification:** `NOT_RATIFIED`

**Question:** May a future package inject crashes or disconnects around a possible send?

**Binding constraints:** CR-B01. ARCH-DEC-44. ARCH-DEC-03. ARCH-DEC-11. ARCH-DEC-31 marks fault-injection tests `NOT_EXECUTED`. After `DISPATCH_STARTED` the attempt is possibly sent. No window authorizes resend. This plan injects nothing.

**Sources:** CR-B01 §1.3 and §1.4. ARCH-DEC-44. ARCH-DEC-31. IBKR-CLM-20. IBKR-CLM-21. IBKR-CLM-22. Protocol stage E3.

**Implications:** A controlled injection can show that a harness refuses retry. It can also send an order if the harness is wrong. Offline labels show the rule without a broker.

**Alternatives:**

- **A — E3 only, under a further authorization.** Fault injection against a broker waits for a separate E3 go, a dedicated test account, an independent safety review, and a hard stop. The harness refuses retry. This choice is not that go.
- **B — Offline labels only.** Crash and disconnect stay synthetic marks. No process is killed against a broker under this question, even later, unless a new question is asked.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** Inject faults on a live connection and retry the attempt. Conflicts with CR-B01, ARCH-DEC-44, and EX-INV-02.

| Alternative | Evidence effect | What stays blocked |
| --- | --- | --- |
| A | A later package could seek E3. It does not have it. | Retry, takeover, this milestone. |
| B | No broker fault is even proposed. | External crash evidence. |
| C | Would resend under uncertainty. | Not ratifiable. |

**Non-binding recommendation:** B until a separate question names the harness. This recommendation is not an approval and is not an E3 authorization.

**Required evidence:** none for B. For A, the E3 authorization, which is absent.

**Dependencies:** IBKREVQ-04, IBKREVQ-10. XC-04 remainder stays OPEN.

## IBKREVQ-10 — Writer isolation proof

**Status:** `USER_DECISION_PENDING`  
**Ratification:** `NOT_RATIFIED`

**Question:** What may count as evidence that a prior writer can no longer send?

**Binding constraints:** CR-B01 §1.2 item 6. P0-B01-INV-02. ARCH-DEC-45. Lease expiry is not takeover. A newer epoch is not external fencing. A network kill alone is not proof the broker rejected an order. `PROOF_OBLIGATION_OWNER_PLACEHOLDER` is not a role.

**Sources:** CR-B01 §1.1 and §1.2. ARCH-DEC-45. IBKR-CLM-23. IBKR-CLM-24. IBKR-CLM-25. Scenarios IBKR-SCN-C08 through IBKR-SCN-C11.

**Implications:** Requiring stop evidence plus connection-closed evidence keeps the successor blocked. Treating the database lease as the fence would violate CR-B01.

**Alternatives:**

- **A — External artifacts, separately labeled.** Isolation evidence is process-stopped and connection-closed, or a reliable session-exclusivity confirmation, plus reconciliation of unknown attempts. `WriterEpoch` stays a different field.
- **B — Do not define the proof in this plan.** Leave the operational remainder of `XC-04` untouched and add no isolation checklist beyond the CR-B01 quotation already in the inventory.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** Treat lease expiry or a newer epoch as sufficient fencing and allow the successor to send. Conflicts with ARCH-DEC-45 and CR-B01 §1.1.

| Alternative | Evidence effect | What stays blocked |
| --- | --- | --- |
| A | A later package knows which artifacts to seek. | Successor send until those artifacts exist. |
| B | No new checklist. | Successor send, still. |
| C | Would take over on a local timer. | Not ratifiable. |

**Non-binding recommendation:** A as a checklist for a future package, with the successor still blocked. This recommendation is not an approval and does not implement a lease.

**Required evidence:** stop evidence and connection-closed evidence. Absent. `NOT_EXECUTED`.

**Dependencies:** IBKREVQ-09, IBKREVQ-13. XC-04 remainder stays OPEN. `XC-05-LOCK` stays OPEN.

## IBKREVQ-11 — Evidence retention

**Status:** `USER_DECISION_PENDING`  
**Ratification:** `NOT_RATIFIED`

**Question:** What may a future package retain, and what may it delete?

**Binding constraints:** ARCH-DEC-38 keeps both originals on contradiction. ARCH-DEC-12 requires retained facts. Raw secrets do not belong in git. This plan retains no payload. The capture note is not a schema.

**Sources:** ARCH-DEC-38. Capture and provenance note. IBKR-CLM-07. IBKR-SCN-A09.

**Implications:** Keeping raw bytes supports later review and can leak secrets if committed. Keeping only hashes avoids the leak and loses the contradiction check.

**Alternatives:**

- **A — Immutable secret-free custody outside this git history.** A future package appends raw bytes in a store that is not this repository, with hashes, version tuple, and a secret-free log. Contradictory bytes are kept. Nothing is retained by this milestone.
- **B — Hashes and redacted metadata only.** A future package stores hashes and scope labels, not payloads, until a further retention decision. Contradiction checks wait for that decision.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** Delete the older payload when a newer one shares an apparent key. Conflicts with ARCH-DEC-38.

| Alternative | Evidence effect | What stays blocked |
| --- | --- | --- |
| A | Bytes exist for E4 without entering git now. | Deletion, posting, this repository as a payload store. |
| B | Smaller exposure, weaker review. | Payload-level contradiction tests. |
| C | Would destroy evidence. | Not ratifiable. |

**Non-binding recommendation:** A, with the store still uncreated. This recommendation is not an approval and adds no file of broker bytes.

**Required evidence:** a retention location named by a later decision. Absent.

**Dependencies:** IBKREVQ-01, IBKREVQ-08. Does not close `XC-06`.

## IBKREVQ-12 — Revalidation cadence

**Status:** `USER_DECISION_PENDING`  
**Ratification:** `NOT_RATIFIED`

**Question:** When must a future evidence package be done again?

**Binding constraints:** IBKR-CLM-26. CR-B04 §4.5. ARCH-DEC-48. One tuple does not cover the next. No cadence number is invented as a measured fact.

**Sources:** IBKR-CLM-26. ARCH-DEC-48. CR-B04 §4.1. Scenario IBKR-SCN-B11.

**Implications:** Version change is a minimum bar. Behavior can also change without a version bump. A calendar rule is a policy choice, not a broker measurement.

**Alternatives:**

- **A — Any version or environment change.** A change of product, API client, platform, OS, or account mode invalidates prior external claims until a new package is adjudicated.
- **B — Version change plus a calendar review.** A adds a review on a cadence a later decision would set. This question does not set the number of days.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** One successful run covers all later versions. Conflicts with CR-B04 §4.5 and ARCH-DEC-48.

| Alternative | Evidence effect | What stays blocked |
| --- | --- | --- |
| A | Stale tuples cannot be reused. | A universal claim. |
| B | Also catches silent behavior change if the later decision sets a period. | The same universal claim. |
| C | Would freeze one run as permanent. | Not ratifiable. |

**Non-binding recommendation:** A now, and leave the calendar period to a later decision rather than inventing a number. This recommendation is not an approval.

**Required evidence:** the version tuple of each package. `UNRECORDED`.

**Dependencies:** IBKREVQ-02, IBKREVQ-14. `XC-01` and `XC-08-IBKR` stay OPEN across tuples.

## IBKREVQ-13 — Escalation

**Status:** `USER_DECISION_PENDING`  
**Ratification:** `NOT_RATIFIED`

**Question:** What happens when a future stage sees unexpected behavior?

**Binding constraints:** ARCH-DEC-46. The marker is not a role and grants no post, release, dispatch, or reset. Missing a required second person means deny. ARCH-DEC-33 adds no role. The protocol halt list is descriptive. `PROOF_OBLIGATION_OWNER_PLACEHOLDER` is not an operator.

**Sources:** ARCH-DEC-46. ARCH-DEC-33. Protocol stop rules. IBKR-CLM-19. `XC-09-RESET` stays OPEN.

**Implications:** A marker makes the halt visible. Requiring a new written decision before any continuation avoids treating the marker as consent.

**Alternatives:**

- **A — Halt and store a non-authorizing marker.** Unexpected behavior stops the stage. The marker names no person and grants no action. If a second person is required and absent, deny.
- **B — Halt with no stored marker.** Stop the stage and require a new written user decision before continuation. Do not store an escalation field in the capture.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** Create an operator role that may post, release, dispatch, or reset. Conflicts with ARCH-DEC-46 and ARCH-DEC-33.

| Alternative | Evidence effect | What stays blocked |
| --- | --- | --- |
| A | Reviewers can see that a halt happened. | Any action by the marker. |
| B | The halt exists only in the decision trail. | A capture field that could be misread as authority. |
| C | Would mint authority. | Not ratifiable. |

**Non-binding recommendation:** A. The marker matches ARCH-DEC-46 and still authorizes nothing. This recommendation is not an approval and names no person.

**Required evidence:** none for the marker. A reset right would be a separate `XC-09-RESET` decision. Not made.

**Dependencies:** IBKREVQ-09, IBKREVQ-10, IBKREVQ-14. `XC-09-RESET` stays OPEN.

## IBKREVQ-14 — Future ratification gates

**Status:** `USER_DECISION_PENDING`  
**Ratification:** `NOT_RATIFIED`

**Question:** What may turn future evidence into a decision, and what may not?

**Binding constraints:** ARCH-DEC-48. ARCH-DEC-04. ARCH-DEC-07: an ADFR `verification` cell leaves `NOT_VERIFIED` only with a dated approval of a named row and an evidence record. Documentation tests are not broker evidence. This package is not that evidence record. E4 is STOP.

**Sources:** ARCH-DEC-48. ARCH-DEC-04. ARCH-DEC-07 in `contracts/decisions/ARCH-DEC-04-bis-08-2026-11-15.md`. Dependency matrix gates G-EV-01 through G-EV-10.

**Implications:** Separating the capture package from the decision package prevents a test log from ratifying itself. Allowing same-package ratification would make the runner the approver.

**Alternatives:**

- **A — Later dated decision after E0–E4 artifacts exist.** A user decision may accept evidence only in a package that is not this one, after the artifacts exist. Acceptance would still not implement posting, release, OMS, or dispatch.
- **B — Adjudication cannot share a package with collection.** The decision that accepts evidence must be a different commit and a different instruction from the commit that added the captures. A review note in the capture package is not a decision.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** Passing this plan's documentation tests, or writing the scenarios, ratifies IBKR behavior and promotes ADFR verification. Conflicts with ARCH-DEC-48, ARCH-DEC-04, and ARCH-DEC-07.

| Alternative | Evidence effect | What stays blocked |
| --- | --- | --- |
| A | A future decision has a bar. This file is not that decision. | Implementation and live trading. |
| B | The collector cannot self-approve. | Same-package promotion. |
| C | Would call this file broker evidence. | Not ratifiable. |

**Non-binding recommendation:** B. Collection and acceptance stay apart. This recommendation is not an approval, does not ratify A or B, and does not start E4.

**Required evidence:** the future artifacts. Absent. All external cases stay `NOT_EXECUTED`.

**Dependencies:** IBKREVQ-01 through IBKREVQ-13. Passing the bar would still leave `XC-02-PRED` and the OMS matrix OPEN.

## Question index

| Question | Topic | Option C | Status |
| --- | --- | --- | --- |
| IBKREVQ-01 | evidence source quality | `CONFLICTS_WITH_BINDING_POLICY` | `USER_DECISION_PENDING` / `NOT_RATIFIED` |
| IBKREVQ-02 | version pinning | `CONFLICTS_WITH_BINDING_POLICY` | `USER_DECISION_PENDING` / `NOT_RATIFIED` |
| IBKREVQ-03 | scope taxonomy | `CONFLICTS_WITH_BINDING_POLICY` | `USER_DECISION_PENDING` / `NOT_RATIFIED` |
| IBKREVQ-04 | test environments | `CONFLICTS_WITH_BINDING_POLICY` | `USER_DECISION_PENDING` / `NOT_RATIFIED` |
| IBKREVQ-05 | read-only limits | `CONFLICTS_WITH_BINDING_POLICY` | `USER_DECISION_PENDING` / `NOT_RATIFIED` |
| IBKREVQ-06 | completeness oracle | `CONFLICTS_WITH_BINDING_POLICY` | `USER_DECISION_PENDING` / `NOT_RATIFIED` |
| IBKREVQ-07 | alias uniqueness verification | `CONFLICTS_WITH_BINDING_POLICY` | `USER_DECISION_PENDING` / `NOT_RATIFIED` |
| IBKREVQ-08 | correction chains | `CONFLICTS_WITH_BINDING_POLICY` | `USER_DECISION_PENDING` / `NOT_RATIFIED` |
| IBKREVQ-09 | unknown-send fault injection | `CONFLICTS_WITH_BINDING_POLICY` | `USER_DECISION_PENDING` / `NOT_RATIFIED` |
| IBKREVQ-10 | writer isolation proof | `CONFLICTS_WITH_BINDING_POLICY` | `USER_DECISION_PENDING` / `NOT_RATIFIED` |
| IBKREVQ-11 | evidence retention | `CONFLICTS_WITH_BINDING_POLICY` | `USER_DECISION_PENDING` / `NOT_RATIFIED` |
| IBKREVQ-12 | revalidation cadence | `CONFLICTS_WITH_BINDING_POLICY` | `USER_DECISION_PENDING` / `NOT_RATIFIED` |
| IBKREVQ-13 | escalation | `CONFLICTS_WITH_BINDING_POLICY` | `USER_DECISION_PENDING` / `NOT_RATIFIED` |
| IBKREVQ-14 | future ratification gates | `CONFLICTS_WITH_BINDING_POLICY` | `USER_DECISION_PENDING` / `NOT_RATIFIED` |

Count: 14 questions. No choice is selected. No recommendation is a decision. This file does not grant testing authority. **NO IMPLEMENTATION.** **NO VERIFICATION.**

## Resolution index (2026-11-15, TRD-ARCH-03B.4.2)

The questions IBKREVQ-01 through IBKREVQ-14 above are unchanged historical text, including their alternatives, consequence tables, and non-binding recommendations. They are not rewritten as approvals. The creation-time status lines stay as they were written. This addendum does not change those lines and does not record a second decision. The historical plan index stays `NOT_RATIFIED`. A recommendation in the historical text is not this ratification.

On 2026-11-15 the user ratified a design-policy choice for each question. The authority is that user decision in the TRD-ARCH-03B.4.2 instruction. The decision record is [ARCH-DEC-49-bis-62-2026-11-15.md](../decisions/ARCH-DEC-49-bis-62-2026-11-15.md). The machine-readable map is [TRD-ARCH-03B42-resolution-index.json](TRD-ARCH-03B42-resolution-index.json). That map is `NON_RUNTIME_NON_WIRE_RESOLUTION_INDEX`. It is not a signature and not an evidence URI. `evidence_uri` is null. `signature_artifact` is null. The record labels there are documentary. They are not a canonical status enum. **NO IMPLEMENTATION.** **NO VERIFICATION.**

| Question | Decision | Option | User wording |
| --- | --- | --- | --- |
| IBKREVQ-01 | ARCH-DEC-49 | B | Nur tatsächlich beschaffte, versionierte offizielle IBKR-Primärtexte mit Abrufdatum als Primärquelle; zusätzlich zweite unabhängig beschaffte Kopie derselben Version vor einer späteren E4-Zitierung. Zweite Kopie ist keine unabhängige empirische Brokerbestätigung. Keine Quelle, URL, Version oder Beschaffung erfinden. |
| IBKREVQ-02 | ARCH-DEC-50 | A | Pro zukünftigem Nachweispaket genau ein gepinntes Produkt-/API-Client-/Plattform-/OS-/Kontomodus-Tupel (und relevante Umgebungsangaben); andere Tupel benötigen getrennte Pakete. Keine Versionswerte erfinden, keine universelle Übertragung. |
| IBKREVQ-03 | ARCH-DEC-51 | A | Account, Client, Session, Dataset, Zeitfenster und Umgebung nur dokumentieren, soweit Request oder Payload sie tatsächlich liefern; fehlende Felder bleiben fehlend. Kein globaler Snapshot und kein Coverage-Nachweis. |
| IBKREVQ-04 | ARCH-DEC-52 | B | Testumgebungsklasse (Paper/Sandbox/etc.) bis zur tatsächlichen offiziellen E0-Dokumentation offenlassen; später gesondert entscheiden. Keine Testumgebung oder Kontonutzung freigeben. |
| IBKREVQ-05 | ARCH-DEC-53 | B | Vor gesonderter, datierter Entscheidung über exakt benannte und offiziell dokumentierte zulässige Leseoperationen keinerlei Brokerverbindung/Read. Diese Ratifikation benennt keine Operation und gibt E2 nicht frei. Cancel/Modify/Replace sind niemals als Read zu deklarieren. |
| IBKREVQ-06 | ARCH-DEC-54 | B | Zukünftige Oracle-Notiz dokumentiert Scope, gespeicherte Bytes und lokale Gaps und enthält explizit die Aussage, dass Capture-Vollständigkeit nicht Broker-Vollständigkeit ist und Broker-Vollständigkeit nicht nachgewiesen wurde. Coverage `UNKNOWN`, kein Completeness-Token, leere Antwort kein Negativbeweis. |
| IBKREVQ-07 | ARCH-DEC-55 | A | Spätere kontrollierte Untersuchungen dürfen Wiederverwendung/Nichtwiederverwendung von `orderId`, `permId`, `execId` innerhalb eines gepinnten Tupels als Beobachtungen berichten. Keine Eindeutigkeitsgarantie, kein UNIQUE-Index, keine automatische wirtschaftliche Zuordnung; Tests heute nicht autorisiert. |
| IBKREVQ-08 | ARCH-DEC-56 | A | Spätere Correction-/Bust-Referenzen nur dann strukturiert übernehmen, wenn der neue Rohpayload selbst die Referenz enthält. Append-only Originale erhalten; keine erfundene Verknüpfung, keine Reversal-Buchung, kein Risk Release, keine Closure-Änderung. |
| IBKREVQ-09 | ARCH-DEC-57 | B | Unknown-Send-Crash-/Disconnect-Szenarien bis zu einer neuen ausdrücklichen Entscheidung nur synthetisch/offline. Keine Fault Injection gegen Broker, keine E3-Freigabe, kein Retry/Resend/Takeover. |
| IBKREVQ-10 | ARCH-DEC-58 | A | Zukünftige Writer-Isolation-Nachweise als separat zu belegende Prozess-Stopp- und Connection-Closed-Artefakte oder belastbare Session-Exklusivitätsbestätigung, zusammen mit Reconciliation unbekannter Versuche, verlangen. `WriterEpoch`/Lease-Deadline separat führen. Dies ist eine Nachweis-Checkliste, kein hinreichender Takeover-Beweis, kein operativer Freigabepfad; CR-B01 bleibt vorrangig. |
| IBKREVQ-11 | ARCH-DEC-59 | A | Künftige Rohpayloads unveränderlich, append-only und geheimnisgeschützt ausserhalb des Git-Repositories verwahren; Hashes, Versions-Tupel und bereinigte Logs; widersprüchliche Originalbytes erhalten. Jetzt weder Store noch Payload erzeugen, keine Secrets ins Git. Speicherort, Zugriffsschutz und Retention bleiben später zu genehmigen. |
| IBKREVQ-12 | ARCH-DEC-60 | A | Bei Änderung von Produkt, API-Client, Plattform, OS oder Kontomodus gelten frühere externe Claims nicht automatisch für das neue Tupel; neues Paket und erneute Entscheidung erforderlich. Kein erfundener Kalenderzeitraum, keine universelle Aussage. |
| IBKREVQ-13 | ARCH-DEC-61 | A | Unerwartetes Verhalten stoppt die spätere Stufe; nicht autorisierenden Eskalationsmarker vorsehen. Marker ist weder Rolle noch Reset-/Post-/Release-/Dispatch-Recht. Fehlende erforderliche Zweitfreigabe bedeutet Deny. Kein tatsächlicher Stage-Start. |
| IBKREVQ-14 | ARCH-DEC-62 | B | Künftige Erfassung und Ratifikation in getrennten Instruktionen und getrennten Git-Commits. Eine Review-Notiz im Capture-Paket ist keine Entscheidung. Nur spätere datierte Nutzerentscheidung kann nach tatsächlichen Artefakten benannte Evidenz akzeptieren; selbst das autorisiert weder Implementierung noch Live-Trading. |

The plan package above stays a plan. Its header and question index remain the creation-time request. The later record approves design policy only. Options are B, A, A, B, B, B, A, A, B, A, A, A, A, B. Count: 8 times A and 6 times B. Option C stays `CONFLICTS_WITH_BINDING_POLICY`. No option C is ratified. The later record does not authorize E0, E1, E2, E3, or E4. Every checkpoint stays STOP. It does not authorize a broker connection, a read, an order, paper trading, fault injection, retry, resend, takeover, a uniqueness guarantee, a completeness token, economic posting, risk release, an economic-finality predicate, an OMS transition, broker dispatch, or live trading. Documentation tests are not broker evidence. External cases stay `NOT_EXECUTED`. External evidence stays `EXTERNAL_EVIDENCE_PENDING`. `XC-01` stays OPEN. `XC-08-IBKR` stays OPEN. `XC-02-PRED` stays OPEN. The XC-04 operational remainder stays OPEN. XC-04 design precedence stays `RESOLVED_BY_ARCH_DEC_03`. CR-B01 precedence stays in force. ARCH-DEC-01 through ARCH-DEC-48 stay binding. No evidence record is created here.
