# TRD-ARCH-03B.3 — Dependency and conflict matrix

**Date:** 2026-11-15  
**Role:** design proposal. `document_role` = `NON_RUNTIME_NON_WIRE_PROPOSAL`.  
**Ratification:** `NOT_RATIFIED`.  
**Live trading:** NOT AUTHORIZED.

This map records what stays open. It does not close a finding, start an implementation, or authorize the economic-finality or OMS work it names.

## 1. XC findings

| Finding | Status in this package | Why it stays that way |
| --- | --- | --- |
| XC-01 | OPEN. `EVIDENCE_PENDING`. | Identity namespaces and candidate labels are proposals. No uniqueness proof. No canonical tuple. |
| XC-02-PRED | OPEN. | No economic-finality predicate is adopted. ARCH-DEC-14 and ARCH-DEC-28 stay binding. A match is not the predicate. |
| XC-02-ALIGN | Principle only. | Closure is not finality. ARCH-DEC-09 and ARCH-DEC-10. |
| XC-03-BOOK | Principle only. | Automatic booking stays forbidden by ARCH-DEC-13. |
| XC-03-MATRIX | OPEN. | No OMS edge, no matrix version, no `OrderClosure` vocabulary. |
| XC-04 | Precedence `RESOLVED_BY_ARCH_DEC_03`. Remainder OPEN. | CR-B01 outranks less restrictive DET-07 text. Session exclusivity, lease, heartbeat, dispatch scope, in-flight evidence, recovery tests, and IBKR behavior are not proved. |
| XC-05-OUTBOX | Principle only. | Broker and NATS stay outside the local commit. Not implemented. |
| XC-05-LOCK | OPEN. | No lock order and no isolation level. ARCH-DEC-18 and ARCH-DEC-31. |
| XC-06 | OPEN. | Distinct in-transaction audit record, no table name. ARCH-DEC-19 and ARCH-DEC-32. |
| XC-07-FX | Principle only. | Missing FX is indeterminate. No rate is chosen. |
| XC-07-ROUND | OPEN. | No rounding mode. |
| XC-08-RULE | Principle only. | An incomplete snapshot is not a negative proof. |
| XC-08-IBKR | OPEN. | No IBKR completeness criterion was obtained or tested. |
| XC-09-SOLO | Principle only. | No simulated second approver. |
| XC-09-RESET | OPEN. | No role and no reset procedure. ARCH-DEC-33. |
| XC-10-RACE | Principle only. | A cancel request is not confirmation. No cancel is dispatched. |
| XC-10-ACTION | OPEN. | No risk-reducing broker action. ARCH-DEC-20 and ARCH-DEC-27. |
| XC-11 | Status words OPEN. Population and supersession `RESOLVED_BY_ARCH_DEC_02`. | `DESIGN_RESOLVED` is not verified and not implemented. ARCH-DEC-04. |

## 2. Decisions already in force

| Record | What this package must not weaken |
| --- | --- |
| ARCH-DEC-01 | DET-01 and DET-02 stay reconstructed drafts with exceptions. Open details stay open. |
| ARCH-DEC-02 | 78 historical ADFR ids, 78 `NOT_VERIFIED`, one supersession `ADFR-CROSS-005` to `ADFR-CR-B10-001`. |
| ARCH-DEC-03 | CR-B01 precedence. No automatic takeover. Lease expiry is not proof. No second send. |
| ARCH-DEC-04 through ARCH-DEC-08 | No winning status word. No verification promotion. |
| ARCH-DEC-09 | One immutable historical closure record. Later facts sit outside it. |
| ARCH-DEC-10 | Reason family is strategy only. No tokens and no enum. |
| ARCH-DEC-11 | Unknown send does not close and does not authorize retry or takeover. |
| ARCH-DEC-12 | Request, observation, and fills stay separate facts. No risk movement from a cancel label. |
| ARCH-DEC-13 | Every proposed economic effect is quarantined, including one match. Manual review does not post. |
| ARCH-DEC-14 | No economic-finality predicate and no release. |
| ARCH-DEC-16 | Identical replay is not a new economic effect. No invented adjudicator. |
| ARCH-DEC-17 | No broker observation is authoritative terminal. |
| ARCH-DEC-18 | No canonical lock order or isolation level. |
| ARCH-DEC-19 | Distinct audit record inside the local transaction. No table name. |
| ARCH-DEC-20 | Modify and implicit cancel-replace stay blocked. Cancel during halt is not authorized. |
| ARCH-DEC-21 | Hard-deny conjunction shape only. No arithmetic. |
| ARCH-DEC-22 | One reservation identity. All-or-nothing scopes. No writer. |
| ARCH-DEC-23 | No reservation on missing or stale price, liquidity, or FX. |
| ARCH-DEC-24 | A fill is retained. The whole reservation stays. No conversion. |
| ARCH-DEC-25 | A residual obligation is not reduced by a partial fill or a cancel. |
| ARCH-DEC-26 | Corrections are retained. Unique matches stay quarantined. No compensating line. |
| ARCH-DEC-27 | No broker action on a risk halt. |
| ARCH-DEC-28 and ARCH-DEC-29 | No evidence list and no release proof. |
| ARCH-DEC-30 | Citation obligations only. No rounding mode and no FX rate. |
| ARCH-DEC-31 | Future concurrency and crash tests are required and `NOT_EXECUTED`. |
| ARCH-DEC-32 | The unnamed audit record is repeated. |
| ARCH-DEC-33 | No new role. |
| ARCH-DEC-34 | Future policy versions are immutable. Recovery does not release or resubmit. |

## 3. Source conflicts kept visible

| Topic | Source reading | Binding reading | This package |
| --- | --- | --- | --- |
| Automatic booking of `MATCHED` | DET-03-D04 and CR-B04 §4.3 step 4 | ARCH-DEC-13, ARCH-DEC-26 | Do not book. |
| Economic consideration of executions | DET-08-D04 and CR-B03 §3.2 | ARCH-DEC-13, ARCH-DEC-26 | Evidence only. |
| Cancel during halt | DET-06-D03 and CR-B09 P1-B09-D02 | ARCH-DEC-20 | No cancel dispatch. XC-10-ACTION stays OPEN. |
| Failover wording | DET-07 where less restrictive | CR-B01 via ARCH-DEC-03 | No automatic takeover. |
| Status word `DESIGN_RESOLVED` | CR-B01 and CR-B04 headers | ARCH-DEC-04, XC-11 | Not verified. |

## 4. Upstream and downstream

| Direction | Item | Relation |
| --- | --- | --- |
| Upstream, already binding | ARCH-DEC-03 and CR-B01 | Fencing design. Not an implementation. |
| Upstream, already binding | ARCH-DEC-09 through ARCH-DEC-20 | OMS policy. No new edge. |
| Upstream, already binding | ARCH-DEC-21 through ARCH-DEC-34 | Risk policy. No release and no posting. |
| Upstream, not evidenced | DET-03, CR-B04, CR-B07, DET-08 | Hypotheses and proof obligations. |
| This package | BROKERQ-01 through BROKERQ-14 | `USER_DECISION_PENDING`. `NOT_RATIFIED`. |
| Downstream, not started | Economic-finality predicate | XC-02-PRED stays OPEN. Handoff is a pointer only. This package does not authorize that work. |
| Downstream, not started | OMS transition matrix | XC-03-MATRIX stays OPEN. Handoff is a pointer only. No edge is proposed as approved. |
| Downstream, not started | Broker adapter and dispatcher | No port, no call, no credential, no network. |
| Downstream, not started | Persistence of attempts, aliases, and snapshots | No migration and no table. |

Identity work does not unblock release. Completeness work does not unblock release. A future user ratification of a BROKERQ option would still leave `XC-02-PRED` and `XC-03-MATRIX` open, because those questions are not asked here.

## 5. Question dependencies

| Question | Depends on for its own evidence | Must not be read as |
| --- | --- | --- |
| BROKERQ-01 | XC-01 | A schema or a UNIQUE index |
| BROKERQ-02 | BROKERQ-01 | Permission to book a singleton |
| BROKERQ-03 | XC-01 null semantics | A zero-equals-missing rule |
| BROKERQ-04 | XC-01 `execId` behavior | A second economic effect |
| BROKERQ-05 | BROKERQ-04 | A compensating ledger line |
| BROKERQ-06 | XC-08-IBKR | A global snapshot |
| BROKERQ-07 | BROKERQ-06 | A release proof |
| BROKERQ-08 | BROKERQ-06 | An invented sequence number |
| BROKERQ-09 | BROKERQ-01 and BROKERQ-10 | A resubmit |
| BROKERQ-10 | ARCH-DEC-03 | `DEFINITIVELY_NOT_SENT` by timeout |
| BROKERQ-11 | BROKERQ-10 | Broker fencing by a database lease |
| BROKERQ-12 | ARCH-DEC-16 and ARCH-DEC-33 | A new role |
| BROKERQ-13 | ARCH-DEC-17 | A terminal winner |
| BROKERQ-14 | All of the above | Verification by this document |

## 6. Non-authorizations

No implementation, no canonical schema, no broker dispatch, no economic posting, no risk release, no economic-finality predicate, no OMS transition, and no live trading. `PUSHED` is not a concept this file decides. Live trading is NOT AUTHORIZED.
