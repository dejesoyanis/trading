# TRD-ARCH-03B.4 — dependency and conflict matrix

**Date:** 2026-11-15  
**Role:** evidence gates for open findings. `document_role` = `NON_RUNTIME_NON_WIRE_VERIFICATION_PLAN`.  
**Conflict closure:** none. `closes_xc_findings` = false.  
**Execution:** `NOT_EXECUTED`.

Design policy and external verification are different layers. ARCH-DEC-35 through ARCH-DEC-48 are design policy. They are `NOT_IMPLEMENTED` and `NOT_VERIFIED`. This matrix says what later evidence would have to contain before anyone could consider closing a finding. It does not close the finding. Considering a closure is not a closure.

## Policy versus verification

| Layer | Where it lives | What this package does |
| --- | --- | --- |
| Design policy | ARCH-DEC-01 through ARCH-DEC-48 | Leaves the records byte-identical. |
| Internal hypothesis | IBKR-CLM-01 through IBKR-CLM-27 | States questions. Executes nothing. |
| External evidence | Future E0–E4 artifacts | Absent. `EXTERNAL_EVIDENCE_PENDING`. |
| Adjudication | A later dated user decision | Not requested as a ratification in this package. `IBKREVQ` stays `USER_DECISION_PENDING`. |
| Implementation | Runtime, schema, dispatch | Not authorized. |

## Precedence that stays in force

| Rule | Binding source | Conflicting text that stays subordinate |
| --- | --- | --- |
| Unknown send and external fencing | CR-B01, ARCH-DEC-03, ARCH-DEC-11, ARCH-DEC-44, ARCH-DEC-45 | Less restrictive DET-07 dispatch and failover text. |
| Single match does not book | ARCH-DEC-13, ARCH-DEC-26 | DET-03-D04 and CR-B04 §4.3 sentence that only `MATCHED` may book. |
| No compensating ledger line | ARCH-DEC-26 | CR-B03 §3.2 compensating booking. |
| No completeness token | ARCH-DEC-41 | Any reading of `P1-B07-D02` that would store `COMPLETE` now. |
| No broker-wins rule | ARCH-DEC-47, DET-08-D03, ARCH-DEC-17 | A default that either side wins. |
| No new role | ARCH-DEC-33, ARCH-DEC-46 | A proof-owner label treated as authority. |

This package does not supersede ARCH-DEC-03, ARCH-DEC-11, ARCH-DEC-13, ARCH-DEC-14, ARCH-DEC-17, ARCH-DEC-26, ARCH-DEC-28, ARCH-DEC-29, ARCH-DEC-31, ARCH-DEC-33, or ARCH-DEC-34.

## What would be required to consider closing XC-01

`XC-01` stays OPEN. Evidence state stays `EVIDENCE_PENDING`. The list below is an entry condition for a later decision. It is not that decision.

A later package would need, for the pinned version tuple and no other tuple:

- official material actually obtained (E0), not a URL invented here
- raw payloads for `orderId` placement context, reuse across client and session, and reconnect or restart (IBKR-CLM-01, IBKR-CLM-02, IBKR-CLM-03)
- raw payloads that keep absent, null, and zero `permId` distinct, plus a later `permId` that does not overwrite (IBKR-CLM-04, IBKR-CLM-05, IBKR-CLM-06)
- identical `execId` replay and a contradictory payload under the same apparent key (IBKR-CLM-07)
- a correction or bust example that either names a reference or shows that none is present (IBKR-CLM-08)
- an out-of-order delivery example (IBKR-CLM-09)
- positive cases and negative cases, both executed in a future authorized stage and both adjudicated at E4
- an explicit statement that a singleton candidate remained `SINGLE_CANDIDATE_NOT_AUTHORIZATION`

Even that package would not approve a UNIQUE index, a canonical identity tuple, or a resolver. Those would be further decisions. `broker_uniqueness_evidence` stays `EVIDENCE_PENDING` until such a later decision, and this file does not make it.

## What would be required to consider closing XC-08-IBKR

`XC-08-IBKR` stays OPEN. Evidence state stays `EXTERNAL_EVIDENCE_PENDING`. No completeness proof is claimed.

A later package would need, per dataset that the request actually named:

- official completion, paging, retention, and empty-result notes for that dataset and that version
- negative cases: empty result, truncated window, pagination break, dropped connection, missing end marker (IBKR-CLM-12 through IBKR-CLM-14)
- a recorded distinction between local capture completeness and broker completeness
- local capture gaps marked only where the capture observed them
- no stored completeness token (ARCH-DEC-41)

Closing the finding would still leave coverage claims subject to a new decision. An empty response would still not be a negative proof.

## What would be required to consider closing the operational remainder of XC-04

Design precedence stays `RESOLVED_BY_ARCH_DEC_03`. The operational remainder stays OPEN. CR-B01 precedence stays in force.

A later package would need:

- evidence that the prior sender is stopped and the broker connection is closed, or a reliable confirmation of session exclusivity, plus reconciliation of every `DISPATCH_STARTED` and `OUTCOME_UNKNOWN` attempt (IBKR-CLM-24)
- crash-window records that keep post-`DISPATCH_STARTED` attempts possibly sent (IBKR-CLM-21, IBKR-CLM-22)
- a demonstration that lease expiry and a newer `WriterEpoch` were not treated as isolation (IBKR-CLM-23, IBKR-CLM-25)
- no automatic retry and no takeover in the recorded outcome

A network-kill note alone does not meet the bar. No TTL is chosen here. `XC-05-LOCK` stays OPEN beside this remainder.

## What stays open even if broker evidence arrives

| Finding | Status in this package | Why evidence would not close it |
| --- | --- | --- |
| XC-02-PRED | OPEN | No economic-finality predicate is adopted. Broker bytes are not the predicate. |
| XC-03-MATRIX | OPEN | No OMS edge is approved. |
| XC-05-LOCK | OPEN | No lock order and no isolation level. |
| XC-06 | OPEN | No audit table name. |
| XC-07-ROUND | OPEN | No rounding mode. |
| XC-09-RESET | OPEN | No reset right. |
| XC-10-ACTION | OPEN | No broker action. |
| XC-11 status words | OPEN | ARCH-DEC-04 chooses no winning status word. Population and supersession stay `RESOLVED_BY_ARCH_DEC_02`. |

Principle-only rows stay principle-only: XC-02-ALIGN, XC-03-BOOK, XC-05-OUTBOX, XC-07-FX, XC-08-RULE, XC-09-SOLO, XC-10-RACE. None of them authorizes an edge, a release, a uniqueness proof, or a posting.

## Evidence-to-decision gates

| Gate | Evidence required before a later decision could pass the gate | Decision this package makes |
| --- | --- | --- |
| G-EV-01 Identity scope | E0 text plus E3 or authorized E2 raw pairs for IBKR-CLM-01..09 | none |
| G-EV-02 No uniqueness claim | The package states that uniqueness remains unclaimed | none; the claim stays unclaimed now |
| G-EV-03 Snapshot scope | Per-dataset notes and negative cases for IBKR-CLM-11..15 | none |
| G-EV-04 No completeness token | The capture contains no token field | none; ARCH-DEC-41 already forbids the token |
| G-EV-05 Unknown send | Fault evidence that does not retry, IBKR-CLM-20..22 | none |
| G-EV-06 Isolation | IBKR-CLM-23..25 artifacts that are not a lease deadline | none |
| G-EV-07 Version pin | IBKR-CLM-26 tuple copied from the components | none; tuple is `UNRECORDED` |
| G-EV-08 Adjudication | E4 dated decision distinct from the capture package | none; `IBKREVQ-14` pending |
| G-EV-09 Economic predicate | Out of scope for broker evidence | `XC-02-PRED` stays OPEN |
| G-EV-10 OMS matrix | Out of scope for broker evidence | `XC-03-MATRIX` stays OPEN |

`IBKREVQ-01` through `IBKREVQ-14` are procedure questions. Answering one would not close a gate and would not close an XC finding.

## Scenario families and blockers

| Family | Scenarios | Claims | Blocker that stays OPEN |
| --- | --- | --- | --- |
| Identity | IBKR-SCN-A01..A14 | IBKR-CLM-01..10, IBKR-CLM-27 | XC-01 |
| Snapshots | IBKR-SCN-B01..B11 | IBKR-CLM-11..18, IBKR-CLM-26 | XC-08-IBKR |
| Unknown send and failover | IBKR-SCN-C01..C12 | IBKR-CLM-16..25 | XC-04 remainder |
| Negative assertions | IBKR-SCN-D01..D06 | IBKR-CLM-12, IBKR-CLM-14, IBKR-CLM-15, IBKR-CLM-21, IBKR-CLM-27 | XC-01, XC-08-IBKR, XC-04 remainder, XC-02-PRED |

Every scenario is `NOT_EXECUTED`.

## Non-closure statement

No row of this matrix changes a finding status. `XC-01`, `XC-08-IBKR`, the operational remainder of `XC-04`, and `XC-02-PRED` stay OPEN. **NO IMPLEMENTATION.** **NO VERIFICATION.**
