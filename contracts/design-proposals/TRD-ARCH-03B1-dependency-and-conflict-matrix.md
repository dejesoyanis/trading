# TRD-ARCH-03B.1 — Dependency and conflict matrix

**Date:** 2026-11-15  
**Role:** design proposal. `document_role` = `NON_RUNTIME_NON_WIRE_PROPOSAL`.  
**Ratification:** `NOT_RATIFIED`.  
**XC-03-MATRIX:** OPEN. This matrix does not resolve it.  
**Live trading:** NOT AUTHORIZED.

No user decision is recorded here. No broker evidence is invented. Recommendations in the approval-request file are not approvals. A future acceptance of a recommendation would still leave the findings below OPEN until that finding's own evidence and decision exist.

## 1. Finding status

| Finding | Status in this package | What this package is allowed to say |
| --- | --- | --- |
| XC-01 | OPEN | Identity scope is unresolved. Late-fill booking and terminal observations stay blocked. |
| XC-02-ALIGN | `NO_CONFLICT_FOUND` on the principle only | Closure, timeout, and cancel acknowledgement are not risk release. Not an executable predicate. |
| XC-02-PRED | OPEN | No economic-finality predicate and no risk release. |
| XC-03-BOOK | `NO_CONFLICT_FOUND` on the principle only | A late fill is retained and does not rewrite closure. Ambiguous identity is not booked. |
| XC-03-MATRIX | OPEN | Coverage families and negative readings are not edges. `OrderClosure` vocabulary is not approved. |
| XC-04 | OPEN except design precedence | ARCH-DEC-03 remains the precedence rule. Session exclusivity, lease, heartbeat, scope, in-flight evidence, recovery tests, and IBKR behavior stay OPEN. |
| XC-05-OUTBOX | `NO_CONFLICT_FOUND` on the principle only | Local commit includes the outbox. Broker and NATS stay outside the transaction. |
| XC-05-LOCK | OPEN | CR-B02 lock order is an unadopted proposal (OMSQ-10). |
| XC-06 | OPEN | Audit artifact is unnamed (OMSQ-11). |
| XC-07-FX | `NO_CONFLICT_FOUND` on the principle only | Missing FX is indeterminate. No OMS use of FX is added. |
| XC-07-ROUND | OPEN | No rounding mode is applied to a quantity in this package. |
| XC-08-RULE | `NO_CONFLICT_FOUND` on the principle only | Incomplete snapshot is not absence and is not `DEFINITIVELY_NOT_SENT`. |
| XC-08-IBKR | OPEN | No adapter and no `COMPLETE` evidence. |
| XC-09-SOLO | `NO_CONFLICT_FOUND` on the principle only | Missing required second person is deny. |
| XC-09-RESET | OPEN | No reset and no `IncidentState`. |
| XC-10-RACE | `NO_CONFLICT_FOUND` on the principle only | Cancel request is not cancel confirmation and is not risk release. |
| XC-10-ACTION | OPEN | Cancel, modify, and risk-reduce dispatch are not authorized. |
| XC-11 | OPEN on status words | ARCH-DEC-04 stands. Population and supersession stay `RESOLVED_BY_ARCH_DEC_02` only. |

CR-B01 precedence over conflicting or less restrictive DET-07 text stays in force. That precedence is not an implementation and not a closed XC-04.

## 2. Questions against findings

`USER_DECISION_PENDING` on every row. "Later task" names work that is not started.

| Question | XC-03-MATRIX | Other findings that stay OPEN even after a future answer | Later task that would still be required |
| --- | --- | --- | --- |
| OMSQ-01 closure mechanics | An answer chooses record shape only. It does not list an edge. Matrix stays OPEN. | None by itself. Vocabulary still OMSQ-02. | None for the policy. Implementation is not authorized. |
| OMSQ-02 vocabulary strategy | A strategy is not a token list and not an edge. Matrix stays OPEN. | XC-01, XC-02-PRED, XC-08-IBKR before any slot can be marked sufficient. | ARCH-03B.2 for the economic slot. ARCH-03B.3 for the broker slot. |
| OMSQ-03 unknown send | Forbidding closure on unknown outcome removes a bad edge. It does not approve a replacement edge. Matrix stays OPEN. | XC-04 remainder, XC-08-IBKR. | ARCH-03B.3 for in-flight and session evidence. |
| OMSQ-04 cancel/fill race | Race retention is not an edge from `CancelRequested` to `Cancelled`. Matrix stays OPEN. | XC-10-ACTION, XC-01, XC-08-IBKR, XC-02-PRED. | ARCH-03B.2 before any risk movement. ARCH-03B.3 before a cancel observation is terminal. |
| OMSQ-05 late fill and bust | Keeping the closure and appending evidence is the XC-03-BOOK principle. The booking threshold is not a matrix. Matrix stays OPEN. | XC-01, XC-02-PRED, XC-07-ROUND. | ARCH-03B.2 for rebooking and incidents. ARCH-03B.3 for the identity match. |
| OMSQ-06 economic-finality evidence | The predicate is DET-02's. Refusing it here does not close the matrix. Matrix stays OPEN. | XC-02-PRED, XC-07-ROUND, XC-08-IBKR, XC-01. | ARCH-03B.2. This package does not pre-approve release. |
| OMSQ-07 matrix policy | Adopting a coverage bar would still leave every edge unapproved. Matrix stays OPEN. | Every blocked family in the coverage draft. | Later matrix proposal, not this package. |
| OMSQ-08 adjudication | A quarantine rule is not an edge. Matrix stays OPEN. | XC-09-RESET, XC-01, XC-09-SOLO already constrains the second-person case. | ARCH-03B.3 for broker conflicts. Reset evidence stays XC-09-RESET. |
| OMSQ-09 terminal observation | A broker-lifecycle acceptance bar is not a workflow edge. Matrix stays OPEN. | XC-01, XC-08-IBKR, XC-08-RULE already forbids the absence reading. | ARCH-03B.3. |
| OMSQ-10 lock order | Concurrency policy is not a workflow edge. Matrix stays OPEN. | XC-05-LOCK. | A later persistence task with deadlock evidence. Not started. |
| OMSQ-11 audit authority | Naming policy is not a workflow edge. Matrix stays OPEN. | XC-06. XC-05-OUTBOX stays a principle, not a table. | A later decision that picks an artifact under ARCH-DEC-04. Not started. |
| OMSQ-12 modify and replace | Blocking modify does not create a cancel-replace edge. Matrix stays OPEN. | XC-10-ACTION, XC-01, XC-02-PRED, XC-04. | ARCH-03B.3 for broker capability. ARCH-03B.2 for the risk delta. |

## 3. Conflicts that are recorded and not chosen

ARCH-DEC-04 forbids a silent winner. The conflicts below stay recorded.

| Topic | Sources that disagree or stay narrower | Resolution in this package |
| --- | --- | --- |
| Status words `OPEN P0`, `PROPOSED_FOR_ACCEPTANCE`, `DESIGN_RESOLVED` | Source tables versus ADFR `status`. ARCH-DEC-04. | No precedence. Not equivalence. XC-11 stays OPEN. |
| DET-07 dispatch text versus CR-B01 | DET-07-D01, DET-07-D03, the dispatch sequence, and the epoch-fencing limit, versus CR-B01 §1.1–§1.3. | Already `RESOLVED_BY_ARCH_DEC_03` for design precedence only. XC-04 stays OPEN otherwise. This package does not add a transition. |
| Audit table name | DET-04-D01 does not name `audit_events`. CR-B08 and DET-10 do, at different status words. | OMSQ-11 recommends leaving the name unset. XC-06 stays OPEN. |
| Readiness reset spelling | DET-06 `MANUAL_RESET_REQUIRED` versus `TradingReadinessState::ManualReset`. DET-10 incident list versus absent `IncidentState`. | XC-09-RESET stays OPEN. No rename. |
| Identity key fields | DET-03 reference row versus CR-B10 `BrokerAccountKey` and CR-B04 hypotheses. | XC-01 stays OPEN. `EVIDENCE_PENDING`. |
| Economic release | Principle agreement (XC-02-ALIGN) versus missing predicate (XC-02-PRED) and CR-B03's proposed compensating entry. | No release. No compensating posting is approved. |

## 4. What an OMS-only acceptance would still not close

If a later user accepted every recommendation in the approval-request file, the following would remain true:

- XC-03-MATRIX would remain OPEN, because no edge list would have been accepted.
- XC-02-PRED would remain OPEN, because OMSQ-06's recommendation refuses a predicate.
- XC-01 and XC-08-IBKR would remain OPEN, because no broker evidence was added.
- XC-04's unresolved remainder would remain OPEN.
- XC-05-LOCK, XC-06, XC-07-ROUND, XC-09-RESET, and XC-10-ACTION would remain OPEN.
- `OrderClosure` would remain an absent type.
- `canonical_schemas_published` would remain false, `part_b` would remain `NO-GO`, and `live_trading_authorized` would remain false.

ARCH-03B.2 is the risk decision set that would have to address OMSQ-05's posting effects and OMSQ-06's predicate, together with XC-02-PRED and XC-07-ROUND. ARCH-03B.3 is the broker decision set that would have to address OMSQ-03, OMSQ-09, and OMSQ-12, together with XC-01, XC-08-IBKR, and the open remainder of XC-04. Neither task is started. Neither task is authorized by this file.

## 5. Dated policy record (2026-11-15, TRD-ARCH-03B.1.2)

Sections 1 through 4 are the proposal-time conflict map. They are not rewritten. A later user decision, recorded in [ARCH-DEC-09-bis-20-2026-11-15.md](../decisions/ARCH-DEC-09-bis-20-2026-11-15.md), selects design policy for OMSQ-01 through OMSQ-12. That selection does not close a finding in section 1. This file still records no broker evidence.

Still OPEN after that policy record: XC-01, XC-02-PRED, XC-03-MATRIX, XC-05-LOCK, XC-06, XC-07-ROUND, XC-08-IBKR, XC-09-RESET, and XC-10-ACTION. XC-04 stays OPEN apart from design precedence (`RESOLVED_BY_ARCH_DEC_03`). XC-11 status-word disagreement stays OPEN. Population and supersession stay `RESOLVED_BY_ARCH_DEC_02`.

CR-B01 precedence over conflicting or less restrictive DET-07 text stays in force. ARCH-03B.2 and ARCH-03B.3 are not started.

OMSQ-05 option B quarantines every proposed economic effect and overrides any reading of the OMSQ-08 proposal text that would post a single identity match automatically. OMSQ-06 option A leaves the economic-finality predicate undefined, so OMSQ-04 does not release risk. OMSQ-07 option A is an acceptance bar and does not accept `XC-03-MATRIX` or any edge. OMSQ-02 option C is a modeling strategy only. OMSQ-09 option A accepts no broker observation as authoritative terminal. OMSQ-10 and OMSQ-11 do not adopt a lock order, an isolation level, or an audit table name. OMSQ-12 does not authorize cancel during halt. XC-10-ACTION stays OPEN. No automatic economic posting, risk release, or broker dispatch is approved.
