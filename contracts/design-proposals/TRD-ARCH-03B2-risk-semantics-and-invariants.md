# TRD-ARCH-03B.2 — Risk semantics and invariants

**Date:** 2026-11-15  
**Role:** design proposal. `document_role` = `NON_RUNTIME_NON_WIRE_PROPOSAL`.  
**Ratification:** `NOT_RATIFIED`.  
**XC-02-PRED:** OPEN. This file does not resolve it.  
**Live trading:** NOT AUTHORIZED.  
**Canonical schemas:** not published. `part_b` = `NO-GO`. `broker_uniqueness_evidence` = `EVIDENCE_PENDING`.  
**Current binding policy:** no executable economic-finality predicate, and reservation release is forbidden (ARCH-DEC-14). Every proposed economic effect stays quarantined, including a single consistent identity match (ARCH-DEC-13).  

This file separates risk dimensions so a later user decision can be reviewed. It is not a risk policy, not a reservation ledger, not a posting rule, and not permission to submit, cancel, modify, release, or trade.

Reference names in DET and CR source files stay reference text. A label in `contracts/registry/registry.json` is a draft shell. `RISK_EVALUATION_IMPLEMENTED` stays false. No operation described here is approved.

## Claim classes

| Class | Meaning in this file |
| --- | --- |
| `BINDING_EXISTING` | A recorded user decision in ARCH-DEC-03 or ARCH-DEC-09 through ARCH-DEC-20, or a fail-closed principle those records keep in force. |
| `RECONSTRUCTED_BASELINE_WITH_EXCEPTIONS` | A DET-01 or DET-02 Q01–Q10 principle inside the ARCH-DEC-01 baseline. The file is `RECONSTRUCTED_DRAFT` and `SOURCE_REVIEW_PENDING`. Q-cell verification stays OPEN. Details the file marks open, undefined, unconfirmed, reference design, or verification-required stay open. |
| `REFERENCE_PROPOSAL` | A formula, table name, lock order, isolation level, signature, test id, or status word in a source file. ARCH-DEC-04 gives those status words no implicit priority. Presence is not ratification. |
| `EXTERNAL_EVIDENCE_PENDING` | Broker identity uniqueness, IBKR snapshot completeness, or another fact this repository does not contain. |
| `NEW_PROPOSAL` | A candidate reading introduced by this package. It needs a later user decision. |
| `USER_DECISION_PENDING` | A RISKQ question. A recommendation in that question is not approval. |

## 1. Dimensions

These dimensions are independent. Collapsing two of them into one word is a reading this package refuses. No dimension below is an approved state edge, a database column, or a wire field.

| Dimension | What it is | What it is not | Class |
| --- | --- | --- | --- |
| Risk budget | A versioned limit for a named scope and metric. DET-02 §3 names global, account, strategy, instrument, sector, and risk-factor scopes. | An available-cash number, a broker buying-power field, or an adopted aggregation formula. | `RECONSTRUCTED_BASELINE_WITH_EXCEPTIONS` for the scope names. The formula is `REFERENCE_PROPOSAL` and stays open (ARCH-DEC-01, DET-02 §3 and §10). |
| Exposure | Economic position and open-order obligation as a risk input. DET-02 §1 and §5 speak of booked economic exposure separately from an open reservation. | A fill, a workflow state, or a number this package calculates. | `RECONSTRUCTED_BASELINE_WITH_EXCEPTIONS` as a distinction. Any quantity is `USER_DECISION_PENDING`. |
| Reservation | A held risk obligation, identified separately from the order, that blocks budget until a later evidenced change. CR-B10 §1: `RiskReservationId` is not `OrderId`. | Consumption, release, workflow closure, or `RiskReservationState::Released` as an implemented transition. The enum is a draft label. | `RECONSTRUCTED_BASELINE_WITH_EXCEPTIONS` for the obligation. The lifecycle edge is not approved. |
| Execution evidence | Append-only observations of fills, busts, and corrections, including raw broker payloads. DET-01 §5 and CR-B03 §3.1. | A booking, a unique broker key, or proof that the set of observations is complete. | `BINDING_EXISTING` for retention (ARCH-DEC-12, ARCH-DEC-13). Completeness is `EXTERNAL_EVIDENCE_PENDING`. |
| Economic ledger / projection | A derived view of position and obligation that CR-B03 §3.1 says is computed from the evidence chain. | An executable posting. ARCH-DEC-13 quarantines every proposed economic effect. | Principle `BINDING_EXISTING` at the "do not rewrite history" layer. The projection algorithm is `REFERENCE_PROPOSAL` and is not authorized. |
| Valuation snapshot | Versioned price, FX, instrument definition, and freshness inputs. DET-02 §6, DET-09-D01 through DET-09-D06, CR-B06 P1-B06-D01. | A chosen rounding mode, a staleness number, or an IBKR margin figure. | Fail-closed on missing inputs is `RECONSTRUCTED_BASELINE_WITH_EXCEPTIONS`. The numeric policy is `USER_DECISION_PENDING` (XC-07-ROUND OPEN). |
| Workflow closure | One immutable historical closure record. Later facts sit outside it. ARCH-DEC-09. | Economic finality, broker terminality, or reservation release. ARCH-DEC-10 is strategy only. | `BINDING_EXISTING`. |
| Broker evidence completeness | Whether a named scope, dataset, window, and connection epoch was covered. CR-B07 P1-B07-D01. An empty response is not a negative proof (P1-B07-D02, RC-INV-04). | An IBKR `COMPLETE` flag. XC-08-IBKR stays OPEN. ARCH-DEC-17 accepts no broker observation as authoritative terminal. | The absence rule is `BINDING_EXISTING` at principle level (XC-08-RULE). The IBKR proof is `EXTERNAL_EVIDENCE_PENDING`. |
| Risk incident / halt | A scope or global stop on new risk-increasing actions, plus an incident when evidence and budget disagree. DET-02 §7, DET-10-D06. | Broker cancel, a risk-reducing order, or `IncidentState`. That type is absent. XC-09-RESET stays OPEN. | The halt principle is `RECONSTRUCTED_BASELINE_WITH_EXCEPTIONS`. Dispatch of a reduction is forbidden by ARCH-DEC-20 until a later decision. `BINDING_EXISTING` for that block. |
| Authorization | Actor, policy version, and, where a second person is required, a real second identity. CR-B05 P1-B05-D01. A missing required second person is deny (XC-09-SOLO). | A simulated approver, a role this package creates, or permission to post. ARCH-DEC-16 names no adjudicator role. Manual review is not authorization to post (ARCH-DEC-13). | `BINDING_EXISTING`. |

## 2. Five words that are not synonyms

| Word | Conceptual meaning | Status |
| --- | --- | --- |
| Reservation | Hold budget against an order obligation before economic evidence of a fill. DET-02 §4. | The hold, as a principle before a risk-increasing submission, is `RECONSTRUCTED_BASELINE_WITH_EXCEPTIONS` (RISK-INV-01). No reservation writer is approved. |
| Consumption | The part of a reservation that a later rule might treat as realized exposure after evidenced execution. DET-02 §5 "Partial Fill". | `REFERENCE_PROPOSAL`. ARCH-DEC-13 forbids booking that conversion now. `USER_DECISION_PENDING` (RISKQ-04). |
| Reallocation | Moving a held obligation across scopes or metrics without treating the move as a fill or a release. | Missing definition. DET-02 §10 leaves the composition algebra and the metric set open. `NEW_PROPOSAL` only as a name that this package does not define arithmetically. |
| Compensating adjustment | A new append-only risk-ledger entry when evidence arrives after a reservation was described as released, or when a bust changes exposure. CR-B03 §3.2. | `REFERENCE_PROPOSAL`. XC-02-PRED stays OPEN. ARCH-DEC-13 forbids posting it. The entry is not approved. |
| Release | Dropping a held obligation because economic finality is evidenced. DET-02 Q06 and RISK-INV-04. | `BINDING_EXISTING`: release is forbidden. ARCH-DEC-14. No checklist in this package is that predicate. |

No row authorizes an operation.

## 3. Invariant inventory

Verification of every DET-02 Q cell stays OPEN. Source test ids are expectations. They are `NOT_EXECUTED`. Passing a documentation test does not execute RISK-T01 through RISK-T14, P0-B02-T01 through T09, or P0-B03-T01 through T10.

| Anchor | Statement this package treats as already binding or baseline | Provenance | What stays missing |
| --- | --- | --- | --- |
| DET-02 §2 DET02-Q01 | Hierarchical budgets: global, account, strategy, instrument, sector, risk factor. | `RECONSTRUCTED_BASELINE_WITH_EXCEPTIONS` | Aggregation formula, metric ids, shared-budget arithmetic. DET-02 §3 and §10. |
| DET-02 §2 DET02-Q02, §3 Deny-Overrides, RISK-INV-02 | A violated hard rule blocks even if another rule allows. Missing required data, unknown version, incomparable unit, stale price, or unknown instrument does not produce `ALLOW`. | `RECONSTRUCTED_BASELINE_WITH_EXCEPTIONS` | The policy-composition algebra. |
| DET-02 §2 DET02-Q03, §4 items 1–5, RISK-INV-01, RISK-INV-03, RISK-INV-08 | A risk-increasing order needs a current positive decision and an atomic reservation across every affected dimension. A rejected reservation leaves no partial consumption. The same command key does not double-charge. OMS, risk, audit, and outbox commit together locally. | `RECONSTRUCTED_BASELINE_WITH_EXCEPTIONS` for the principle. The lock order it cites is not adopted. | SQL DDL, canonical lock order, isolation level (ARCH-DEC-18, XC-05-LOCK OPEN). |
| DET-02 §4 item 6 | Reference inequality `used + reserved + proposed_delta <= limit` for one positive metric in one unit. Negative, offsetting, netting, margin, and stress need their own versioned rules. | `REFERENCE_PROPOSAL`. ARCH-DEC-01 exception. XC-02-PRED source. | Not an executable rule. |
| DET-02 §2 DET02-Q04, §6, RISK-INV-06 | Price, liquidity, and exposure checks use versioned inputs. Missing price, FX, or definition fails closed. No implicit FX, no silent rescale, no invented rounding. | `RECONSTRUCTED_BASELINE_WITH_EXCEPTIONS`. XC-07-FX principle. | Rounding mode, staleness number, FX policy (XC-07-ROUND OPEN). |
| DET-02 §2 DET02-Q05, §5 Partial Fill, RISK-INV-05 | A confirmed execution would move reservation toward exposure, once, per unique evidence event. | `RECONSTRUCTED_BASELINE_WITH_EXCEPTIONS` as a direction. Booking it is forbidden now. | Attribution formula. ARCH-DEC-13. |
| DET-02 §2 DET02-Q06, §5 Cancel/Reject, RISK-INV-04 | Release only on confirmed economic finality. Cancel acknowledgement, `CancelRequested`, workflow closure, timeout, and an incomplete snapshot do not release. | `RECONSTRUCTED_BASELINE_WITH_EXCEPTIONS` plus `BINDING_EXISTING` ARCH-DEC-12 and ARCH-DEC-14. | The predicate itself. XC-02-PRED OPEN. |
| DET-02 §5 Late Fill, Bust/Correction; CR-B03 §3.1–§3.5; DET-01 §5; OMS-INV-03, OMS-INV-04 | Late evidence is kept. Closure is not rewritten. Busts append. Ambiguous identity is not booked. A failed risk check does not delete a real execution from the evidence record. | Retention and immutability are `BINDING_EXISTING` (ARCH-DEC-09, ARCH-DEC-13). Automatic booking in CR-B04 §4.3 step 4 is overridden. | Posting algorithm, compensation arithmetic. |
| DET-02 §2 DET02-Q07, §8 RISK-INV-08; DET-01 §4 item 4; CR-B02 §2.1; CR-B08 P1-B08-D01; OPS-INV-01 | A safety-critical local mutation needs authoritative audit in the same local unit. An export is not that authority. | `BINDING_EXISTING` ARCH-DEC-19 for the distinct record. | Table name. XC-06 OPEN. |
| DET-02 §2 DET02-Q08, §7, RISK-INV-07; CR-B05 P1-B05-D02; OPS-INV-03 | Hard safety invariants are not overridable by an agent or by break-glass. | `RECONSTRUCTED_BASELINE_WITH_EXCEPTIONS` and XC-09-SOLO principle. | Role names and reset evidence. XC-09-RESET OPEN. |
| DET-02 §2 DET02-Q09, §7; CR-B09 P1-B09-D04; RISK-T11 | WARN, BLOCK, HALT, and a separately proved reduction are different responses. A negative quantity is not automatically risk-reducing. | `RECONSTRUCTED_BASELINE_WITH_EXCEPTIONS`. Dispatch of the reduction is `BINDING_EXISTING` blocked (ARCH-DEC-20). | Proof method. XC-10-ACTION OPEN. |
| DET-02 §2 DET02-Q10, §9 | Unit, property, and replay tests are required before a safety gate can pass. | `RECONSTRUCTED_BASELINE_WITH_EXCEPTIONS` as a requirement. | All of those tests. `NOT_EXECUTED`. |
| DET-01 §4 items 2 and 5; P0-B02-INV-03; ARCH-DEC-16 | Same idempotency key and same payload return the stored result. Any other conflict stays quarantined. Identical replay creates no new economic effect. | `BINDING_EXISTING`. | No write path. |
| DET-01 §7; CR-B01 §1.1–§1.3; ARCH-DEC-03; ARCH-DEC-11 | Unknown send does not close the workflow, does not prove non-send, and does not authorize a second submission or takeover. CR-B01 keeps precedence over conflicting or less restrictive DET-07 text. | `BINDING_EXISTING`. | XC-04 remainder OPEN. |
| DET-08 RC-INV-03, RC-INV-04; CR-B07 P1-B07-D02 | Unclear reservations are not released. An incomplete snapshot does not prove absence. | Principle `BINDING_EXISTING` via ARCH-DEC-14 and ARCH-DEC-17. | IBKR completeness. `EXTERNAL_EVIDENCE_PENDING`. |
| CR-B04 §4.3 and §4.6 P0-B04-INV-01 | Ambiguous or conflicting identity is not an automatic economic booking. | The quarantine outcome is `BINDING_EXISTING` (ARCH-DEC-13, ARCH-DEC-16). CR-B04's "only MATCHED may book automatically" is not a posting permission under ARCH-DEC-13. | Uniqueness evidence. XC-01 OPEN. |
| OMS decisions ARCH-DEC-09 through ARCH-DEC-20 | Immutable closure; reason family separate from terminality as strategy only; no unknown-send closure; facts retained; all economic effects quarantined; no predicate and no release; future matrix needs its own version; contradictory replay quarantined; no authoritative terminal broker observation; lock order and isolation unadopted; distinct unnamed audit record; modify and implicit cancel-replace blocked. | `BINDING_EXISTING`. | See section 8. |

### OMS decisions this package must not weaken

| Decision | Binding rule that constrains risk |
| --- | --- |
| ARCH-DEC-09 | One historical closure record. A late fill does not reopen it and is not a closure-shaped correction. |
| ARCH-DEC-10 | Workflow closure, broker terminality, economic finality, and evidence completeness stay separate dimensions. No tokens and no fields are approved. |
| ARCH-DEC-11 | Unknown send stays in reconciliation or quarantine. No second submission. |
| ARCH-DEC-12 | Cancel request, cancel observation, and every fill stay separate facts. Risk does not move until a predicate that ARCH-DEC-14 has not approved is sufficient. |
| ARCH-DEC-13 | Quarantine every proposed economic effect, including one identity match. Manual review is not permission to post. |
| ARCH-DEC-14 | No executable economic-finality predicate. Reservation release is forbidden. Checklist language in a proposal is not that predicate. |
| ARCH-DEC-15 | A future OMS matrix needs full families, negative edges, and its own version. This package approves no edge. `XC-03-MATRIX` stays OPEN. |
| ARCH-DEC-16 | Only identical-payload replay returns a stored result. It creates no new economic effect. No adjudicator role. |
| ARCH-DEC-17 | No broker observation is authoritative terminal. |
| ARCH-DEC-18 | CR-B02 lock order and `READ COMMITTED` stay proposals. A future retry is local only. |
| ARCH-DEC-19 | Distinct in-transaction audit record, failing closed if it cannot be written. No table name. |
| ARCH-DEC-20 | No modify and no implicit cancel-replace. No risk-reducing broker action is authorized. |

## 4. Hierarchy, shared constraints, and ownership

`RECONSTRUCTED_BASELINE_WITH_EXCEPTIONS`. DET-02 §3 says one order can load several limits at once, and that several metrics (notional, open-order exposure, concentration) are evaluated separately. The reference `BudgetKey(scope_type, scope_id, metric_id, currency_or_unit, policy_version)` is `REFERENCE_PROPOSAL`. It is not a schema.

Shared-budget constraint, as a principle only: every affected hard dimension is checked, and one hard deny blocks the action (RISK-INV-02). That sentence does not say how overlapping global, account, and sector rows are added, netted, or ordered. CR-B02 §2.5 test P0-B02-T02 names overlapping budgets as a required future test. That test is `NOT_EXECUTED`.

Reservation ownership, as a principle only: CR-B10 §1 separates `RiskReservationId` from `OrderId` and requires an explicit order association. This package does not define whether one reservation id covers every scope the order touches, or whether each scope gets its own id. Concurrent contention across those scopes has no adopted lock protocol (ARCH-DEC-18). RISK-INV-03 forbids oversubscription. The mechanism is `USER_DECISION_PENDING` (RISKQ-01, RISKQ-02).

Missing definitions, stated so they are not silently filled:

- Policy-composition algebra and the metric catalogue (DET-02 §10).
- Netting, margin, stress, and short-exposure rules (DET-02 §4 item 6, CR-B06 P1-B06-D03).
- Which scope owns a shared reservation, and the idempotency key that binds command, order, and reservation (DET-02 §4 item 4).
- The partial-fill split between residual obligation and realized exposure (DET-02 §5).
- The predicate DET-02 Q06 calls "bestätigte wirtschaftliche Finalität" (XC-02-PRED).
- A sufficiency test for broker snapshot completeness (XC-08-IBKR).
- Valuation freshness limits and rounding quanta (XC-07-ROUND).
- The audit artifact name (XC-06).
- Incident-state type and reset roles (XC-09-RESET).

## 5. Conceptual lifecycle of an order's risk obligation

These are scenario descriptions. They are not approved state edges, not an `OrderClosure` transition, and not DDL. In every scenario the current policy is the same: record evidence, do not book economics, do not release budget.

| Situation | Obligation reading | Class |
| --- | --- | --- |
| Submission uncertainty | `DISPATCH_STARTED` or `MayHaveSent` means the order may exist at the broker. The obligation stays. A second submission is forbidden (ARCH-DEC-11, EX-INV-02, CR-B01). Timeout is not proof of non-send. | `BINDING_EXISTING` |
| Partial fill | Evidence of a fill is retained. The unfilled remainder stays an obligation. Moving the filled part into exposure is a proposed effect and stays quarantined (ARCH-DEC-13). | `BINDING_EXISTING` for quarantine. The split is `USER_DECISION_PENDING`. |
| Cancellation request | The request is its own fact. It does not shrink the obligation (DET-02 §5, ARCH-DEC-12, XC-10-RACE). | `BINDING_EXISTING` |
| Cancellation observation | A broker cancel observation is retained and is not terminal (ARCH-DEC-17). It does not release the residual while completeness and identity are open (XC-08-IBKR, XC-01). | `BINDING_EXISTING` |
| Late fill after workflow closure | The closure record stays. The fill is new evidence (CR-B03 P0-B03-INV-01, ARCH-DEC-09). Any economic effect, including a single match, is quarantined (ARCH-DEC-13). Over-limit can justify an incident and a halt on new risk-increasing actions. It does not authorize a broker reduce. | `BINDING_EXISTING` |
| Bust or correction | Original evidence stays. The change is append-only (DET-01 §5, CR-B03 §3.2). A bust can raise or lower obligation. No blanket release. The proposed delta is quarantined. | `BINDING_EXISTING` for retention. The delta is not posted. |
| Duplicate evidence | Identical payload and key return the stored result and add no second effect (ARCH-DEC-16, RISK-INV-05). Same apparent key with a different payload is a conflict and stays quarantined (CR-B04 §4.3 step 5). | `BINDING_EXISTING` |
| Valuation staleness | Missing, stale, or incomparable valuation does not produce `ALLOW` and does not justify a new risk-increasing reservation (DET-02 §6, MD-INV-05). Evidence already received is still retained. | `RECONSTRUCTED_BASELINE_WITH_EXCEPTIONS` |
| Broker disconnect | An incomplete stream is not an empty book (CR-B07, RC-INV-04). The obligation stays. | `BINDING_EXISTING` at principle level. IBKR behavior is `EXTERNAL_EVIDENCE_PENDING`. |
| Restart | Reconstruct commands, versions, outbox, ledger, and raw evidence (DET-01 §7). Do not resume an unclear send and do not release an unclear reservation (RC-INV-01, RC-INV-03). | `RECONSTRUCTED_BASELINE_WITH_EXCEPTIONS` plus ARCH-DEC-11 and ARCH-DEC-14. |
| Reconciliation | Differences are classified, not awarded to the broker or the database by default (DET-08-D03). Unresolved risk mismatch blocks new risk-increasing submission. It does not auto-close the difference (RC-INV-02). | `RECONSTRUCTED_BASELINE_WITH_EXCEPTIONS`. No run is implemented. |

## 6. Fail-closed principles

These principles are already in force. This package restates them so a later RISKQ answer cannot be read as waiving them.

- No reservation release on timeout, cancel request, workflow closure, broker acknowledgement, or an incomplete snapshot. RISK-INV-04, ARCH-DEC-12, ARCH-DEC-14, XC-08-RULE.
- No second submission while send outcome is unknown. ARCH-DEC-11, EX-INV-02, CR-B01 precedence over less restrictive DET-07 text (ARCH-DEC-03).
- No silent exposure netting. The reference inequality is not a netting rule (DET-02 §4 item 6, ARCH-DEC-01).
- No accidental negative available budget. A rejected reservation leaves no partial consumption (DET-02 §4 item 5). An overfill is an incident, not a clipped quantity (CR-B03 §3.3).
- Ambiguous identity is quarantined (P0-B04-INV-01, ARCH-DEC-13).
- Duplicates have at most one eventual economic effect, and that effect is not approved now (RISK-INV-05, ARCH-DEC-16).
- Evidence is kept when booking is blocked (CR-B03 §3.4, ARCH-DEC-13).
- Audit and source lineage stay attached. A mutation that cannot write its audit record fails (ARCH-DEC-19, OPS-INV-05).

## 7. Local PostgreSQL unit of work, at design level

`REFERENCE_PROPOSAL` for every table name, lock step, and isolation level. `BINDING_EXISTING` for the boundary rules ARCH-DEC-18 and ARCH-DEC-19 already set.

The sources describe one local commit that would carry the command result, the order mutation, the risk reservation change, the evidence or ledger append, the audit record, and the outbox row (DET-02 §4 item 3, DET-01 §4 item 4, CR-B02 §2.1 and §2.4, CR-B08 P1-B08-D01). That description is not an implemented transaction. `trading-persistence` has no OMS or risk tables.

Design constraints that are already binding:

- Idempotency is the stored result of the same key and the same payload. A different payload conflicts (P0-B02-INV-03, ARCH-DEC-16).
- Compare-and-swap on an order version is a DET-01 Q04 principle. It does not choose an isolation level.
- Targeted locks are named by CR-B02 §2.2. That order is not canonical (ARCH-DEC-18, XC-05-LOCK OPEN). This file does not adopt it.
- A retry, if one is ever built, retries the local database unit only. It does not call the broker and it does not publish to NATS inside the transaction (P0-B02-INV-04, XC-05-OUTBOX principle, ARCH-DEC-18).
- A stale risk evaluation is not reused after `40001`, `40P01`, or a version conflict (CR-B02 §2.3). That sentence is `REFERENCE_PROPOSAL` until tests exist. Those tests were not run. This package does not claim a deadlock test, a serialization test, or a reconciliation test has passed.
- If the database is unavailable, the outcome is not `ALLOW` (CR-B02 §2.4).

## 8. Conflicts and what this package cannot decide

| Topic | Tension | What stays unresolved |
| --- | --- | --- |
| XC-02-PRED | DET-02 Q06 requires confirmed economic finality before release. No source defines an executable predicate. CR-B03 §3.3's balance equation and CR-B06's metric list are proposals. | OPEN. ARCH-DEC-14 forbids the predicate and the release. |
| XC-02-ALIGN | Closure, timeout, and cancel acknowledgement are not release. | Principle only. It does not define release. |
| XC-03-MATRIX | Risk effects are among the guards a future matrix must name (DET-01 §6). | OPEN. No edge. Handoff remains the OMS matrix work. |
| XC-03-BOOK | Late fill is retained and does not rewrite closure. CR-B04 §4.3 would allow automatic booking on one match. | ARCH-DEC-13 overrides that automatic booking. The finding's booking threshold stays unapproved. |
| XC-01 | Economic attribution needs a scoped identity. The key fields are not frozen. | OPEN. `EXTERNAL_EVIDENCE_PENDING`. ARCH-03B.3. |
| XC-04 | DET-07's dispatch sequence persists a reservation and then sends. CR-B01 is stricter on unknown outcome and takeover. | Precedence is `RESOLVED_BY_ARCH_DEC_03`. Session, lease, heartbeat, scope, in-flight evidence, recovery tests, and IBKR behavior stay OPEN. |
| XC-05-LOCK | DET-02 §4 item 2 tells the reader to lock in the CR-B02 order. ARCH-DEC-18 leaves that order unadopted. | OPEN. No isolation level. |
| XC-05-OUTBOX | Local commit includes the outbox. Broker and NATS stay outside. | Principle only. Not a table. |
| XC-06 | Audit is required and unnamed. CR-B08 says `audit_events`. DET-04-D01 does not. | OPEN. ARCH-DEC-19 does not adopt the name. |
| XC-07-ROUND | CR-B06 suggests directional rounding. UD-03 and DET-02 §6 refuse an implicit mode. | OPEN. No mode adopted. |
| XC-07-FX | Missing FX is indeterminate. | Principle only. |
| XC-08-IBKR | Completeness is a scoped claim. IBKR signals are not in this repository. | OPEN. ARCH-03B.3. This package must not treat a snapshot as complete. |
| XC-08-RULE | Empty or incomplete is not absence and is not `DEFINITIVELY_NOT_SENT`. | Principle only. |
| XC-09-RESET | Halt and reset spellings disagree. `IncidentState` is absent. | OPEN. |
| XC-09-SOLO | Missing required second person is deny. | Principle only. No role is created. |
| XC-10-ACTION | Cancel during halt, modify, and risk-reduce dispatch need broker semantics and a risk-delta rule. | OPEN. ARCH-DEC-20 blocks modify and does not authorize the reduce. |
| XC-10-RACE | Cancel request is not cancel confirmation and is not release. | Principle only. |
| XC-11 | `OPEN P0`, `PROPOSED_FOR_ACCEPTANCE`, and `DESIGN_RESOLVED` have no winning status. | OPEN on the words. Population and supersession stay `RESOLVED_BY_ARCH_DEC_02`. |

ARCH-03B.2 cannot decide broker identity uniqueness, IBKR completion signals, modify and cancel-replace semantics, session fencing, or in-flight send evidence. Those belong to ARCH-03B.3 and remain unauthorized. This package does not start that work.

CR-B01 precedence over conflicting or less restrictive DET-07 text remains binding. A risk reservation is not a second send and is not a takeover.
