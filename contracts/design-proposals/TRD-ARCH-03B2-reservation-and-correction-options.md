# TRD-ARCH-03B.2 — Reservation and correction options

**Date:** 2026-11-15  
**Role:** design proposal. `document_role` = `NON_RUNTIME_NON_WIRE_PROPOSAL`.  
**Ratification:** `NOT_RATIFIED`.  
**XC-02-PRED:** OPEN.  
**Live trading:** NOT AUTHORIZED.  
**Canonical schemas:** not published.

The three approaches below are comparison architectures for a future reservation accounting model. They are not allowed operations. No approach authorizes a ledger posting, a reservation release, a broker call, or a transition edge.

**Binding regardless of which approach a later reader prefers:**

- OMSQ-05 option B, ARCH-DEC-13: every proposed economic effect stays quarantined, including a single consistent identity match. Manual review is not authorization to post.
- OMSQ-06 option A, ARCH-DEC-14: no executable economic-finality predicate, and reservation release is forbidden.

A preference in section 5 does not change those two rules.

## 0. Conservation, without an executable formula

CR-B03 §3.3 states a reference balance: net quantity from valid execution and correction evidence equals the booked position change, and risk usage is a function of positions, open orders, policy, and a market snapshot. Unknown open quantity is not set to zero because a cancel is suspected. Quantity above the original order is an overfill incident, not a clipped fill.

That equation is `REFERENCE_PROPOSAL`. ARCH-DEC-01 keeps DET-02's reference inequality outside netting, margin, and stress. This package does not promote either equation to a ratified contract. Exact arithmetic, FX, and rounding stay OPEN (XC-07-ROUND, DET-02 §6, UD-03). `FINANCIAL_ROUNDING_IMPLEMENTED` stays false.

Double-entry, where the sources justify the idea and not a chart of accounts: a fill would move an amount from an open-obligation bucket to a position-exposure bucket, and a bust would append the reverse movement rather than delete the original (CR-B03 §3.2, DET-02 §5). The buckets are not tables. The amounts are not computed here. An imbalance is a reason to keep the case quarantined and to block new risk-increasing actions. It is not a reason to post.

## 1. Approach `hold_until_evidenced_finality`

Conservative hold until evidenced finality.

The original reservation remains for the full order obligation until a future predicate, which does not exist, says the economic outcome is final. Fills, cancels, busts, and corrections are stored as evidence. They do not move budget. Because ARCH-DEC-14 forbids release, this approach is the only one that matches current operations: nothing is released, and nothing is converted.

| Topic | Reading under this approach |
| --- | --- |
| Partial fill | The filled quantity is evidence. The residual unfilled obligation stays inside the original hold. No part becomes realized exposure. |
| Late fill after closure | Evidence is appended outside the closure record. The hold remains. A limit breach opens an incident and blocks new risk-increasing actions. It does not book. |
| Bust, reversal, correction | The chain is appended. The hold is not reduced because a bust "looks smaller", and it is not increased by a posted amount. The uncertainty itself can widen the incident. |
| Duplicate | Identical replay returns the stored evidence result. A conflicting payload is kept and quarantined. Neither changes the hold. |
| Insufficient budget | A new risk-increasing command is denied. Already-held obligation is not driven negative to make the new command fit. |
| Multi-scope contention | Either every affected scope can hold the new command under the future, still-unapproved check, or the command fails with no partial hold. Over-reservation of one scope and not another is a failed unit. |
| Recovery | After a crash, the hold is whatever the last local commit stored. Unclear sends stay held. Reconciliation does not release them. |

Evidence and guards: raw events, command identity, and a refusal or incident record. No broker completeness is required to keep the hold, because the hold is the fail-closed default. Idempotency is the stored command result (ARCH-DEC-16). Audit is the distinct record ARCH-DEC-19 already requires, still unnamed.

Worst case: capital stays reserved through a long uncertainty, including after the broker has in fact finished. That is operationally expensive and is the point of the approach until evidence exists. A second worst case is an overfill that the hold does not cover. The response is an incident and a block, not a silent negative budget.

Operational burden: high capital tie-up, low posting risk. Migration: nothing to migrate, because no posting ledger exists. Compatibility: matches ARCH-DEC-13 and ARCH-DEC-14 without a special case.

Tests, all `NOT_EXECUTED`: hold unchanged across the shortcut list in the coverage draft; crash before and after commit; duplicate payload; overfill incident without a negative available figure.

ARCH-03B.3 dependency: none to keep the hold. Any future exit from the hold needs identity uniqueness and snapshot completeness, which this approach does not grant.

## 2. Approach `reservation_to_exposure_with_residual`

Explicit reservation-to-exposure conversion, with an independently audited residual obligation.

This is the direction DET-02 §5 and CR-B03 §3.2 describe: evidenced execution would move quantity from open obligation to position exposure, and the residual unfilled obligation would stay reserved. After a reservation had been released, a later fill would not reopen the old reservation. It would append a compensating entry (CR-B03 §3.2). Those sentences are `REFERENCE_PROPOSAL`. Under current policy the conversion and the compensating entry are proposed economic effects. They stay quarantined. Choosing this approach in a later decision would still not post them unless that decision explicitly supersedes ARCH-DEC-13 and ARCH-DEC-14. This package does not supersede them.

| Topic | Reading under this approach, if a future superseding decision allowed posting |
| --- | --- |
| Partial fill | Only the evidenced quantity would leave the reservation. The residual would remain an audited obligation. The two legs would commit in one local unit or not at all. |
| Late fill after closure | The closure would stay. The fill would be a new ledger leg. If the reservation were already gone, the leg would be a new compensating entry, not a resurrection of the old id. |
| Bust, reversal, correction | Append a reversing or adjusting leg that references the original evidence. Recompute the projection from the chain. A bust that recreates an open obligation would increase the residual, not "free" budget. |
| Duplicate | One economic effect per unique evidence event (RISK-INV-05). The audit record would cite that event id. |
| Insufficient budget | The new command would still deny. A late fill that breaches a limit would book only if the superseding decision said so, and would halt new risk. This package does not say so. |
| Multi-scope contention | Each scope would convert the same evidenced quantity or the whole conversion would roll back. Shared budgets would not be netted by this approach. |
| Recovery | A crash between ledger and risk is the failure CR-B03 §3.4 and P0-B03-T09 name. The local unit would have to contain evidence, projection, and reservation together. Those tests are `NOT_EXECUTED`. |

Evidence and guards: scoped identity at `MATCHED` with a uniqueness proof that is `EVIDENCE_PENDING`, a completeness status that is also pending, a valuation version, a policy version, and the audit record. Ambiguous identity would still quarantine (P0-B04-INV-01), and ARCH-DEC-13 quarantines the matched case too until superseded.

Worst case: a false match converts reservation to exposure and releases residual, then a bust or a second execution arrives. The compensating entry can record the damage and cannot undo a broker fill. That is why the approach is not available now.

Operational burden: a real sub-ledger, reconciliation, and incident staffing. Migration: historical orders have no such ledger today, so a future migration would be a separate decision. Compatibility: conflicts with current posting prohibition until a superseding decision.

Tests, all `NOT_EXECUTED`: partial conversion conservation, late fill after release, bust sign both ways, duplicate suppression, crash between evidence and reservation, single-match still quarantined under today's policy.

ARCH-03B.3 dependency: the identity match and the completeness proof are preconditions for any later use of this approach. They are not in this repository.

## 3. Approach `staged_allocation_subject_to_proofs`

Staged allocation and adjustment, each stage gated by a proof that is not yet defined.

Stages a future design might name, none of them approved:

1. Reserve the full obligation when inputs are fresh and every hard scope allows. Fail closed otherwise.
2. On partial evidence, adjust only a slice that a named proof ties to one evidence id, and leave the residual explicit.
3. On cancel evidence plus a completeness proof, adjust the residual. The proof does not exist (XC-08-IBKR).
4. On bust or correction, append an adjustment that cites the earlier stage.
5. Release only at a final stage whose predicate ARCH-DEC-14 has not adopted.

Each stage would be idempotent, audited, and refused when the proof is missing. "Staged" does not mean "release early". A missing proof keeps the prior stage, which under current policy is the full hold and no booking.

| Topic | Reading |
| --- | --- |
| Partial fill | A stage-2 adjustment is a proposal. Without the proof, the stage does not run. |
| Late fill after closure | A new stage is appended. The closure is not a stage and is not reopened. |
| Bust, reversal, correction | A further stage references the original evidence id. It does not edit the earlier stage. |
| Duplicate | Replaying a stage key returns that stage's stored result and does not add a stage. |
| Insufficient budget | Stage 1 denies. A later stage that would breach a limit halts new risk and does not invent budget. |
| Multi-scope contention | A stage names every scope it touches and commits all of them or none. There is no adopted lock order to implement that. |
| Recovery | Restart resumes at the last committed stage. It does not synthesize a stage from a timeout. |

Evidence and guards: every stage would cite evidence ids, valuation version, policy version, expected reservation version, and the audit record. The proofs for stages 2 through 5 are `USER_DECISION_PENDING` and, for broker facts, `EXTERNAL_EVIDENCE_PENDING`.

Worst case: a proof that is too weak is treated as a stage gate, and residual obligation is released while a fill is still possible. The approach is unsafe unless the gates stay stricter than ARCH-DEC-14, which currently allows no release gate at all. A second worst case is operational complexity that hides a missing stage in reconciliation.

Operational burden: the highest of the three. Every stage is a policy version. Migration and compatibility require a versioned stage catalogue that this package does not write. A policy change would be a new version, not an edit of historical stages (DET-09-D06's reproducibility idea, still `REFERENCE_PROPOSAL`).

Tests, all `NOT_EXECUTED`: negative tests for each missing proof, property tests that a stage cannot skip, replay of a stage key, and a counterexample where completeness is partial.

ARCH-03B.3 dependency: stages that mention broker cancel, fill identity, or snapshot completeness cannot be specified until that evidence exists. This approach does not wait by implementing them early.

## 4. Shared failure modes

| Failure | Hold | Conversion | Staged |
| --- | --- | --- | --- |
| False "done" | Capital stays held. No false release. | False conversion and a possible false residual release. | False release if a stage gate is weak. |
| Lost evidence | The hold may be too large. Evidence gap becomes an incident. | A missing leg understates exposure. | A missing stage looks like an earlier stage and can hide a fill. |
| Duplicate | No second hold if idempotency holds. | Double conversion if the evidence key is wrong. | Double stage if the stage key is wrong. |
| Crash | Last local commit, or full rollback. | Split brain if evidence and reservation are not one unit. | Same split, with more rows. |
| Over-reservation across scopes | Denied command, no partial hold, once the unit is actually atomic. The unit is not built. | Partial conversion if one scope commits alone. | Partial stage if the stage omits a scope. |
| Negative available budget | Refused by the deny rule. Not implemented. | A late fill can drive usage over the limit and must halt rather than clip. | Same, at the stage that would have posted. |

All three depend on a local atomic unit that ARCH-DEC-18 has not given a lock order or an isolation level. All three depend on audit that ARCH-DEC-19 has not named. None of them may call the broker inside that unit.

## 5. Non-binding recommendation

For user consideration only: approach `hold_until_evidenced_finality`.

Reason: it is the only comparison that can be operated under ARCH-DEC-13 and ARCH-DEC-14 without a posting path. The conversion approach and the staged approach describe future shapes that stay blocked until a superseding decision and until ARCH-03B.3 evidence exists. Preferring the hold does not ratify a reservation writer, a budget formula, or a release. It records that this package does not choose a posting architecture.

This recommendation is not approval. OMSQ-05 B and OMSQ-06 A stay binding if a later decision prefers another approach. That later decision is not this file.
