# TRD-ARCH-03B.1 — Candidate transition coverage

**Date:** 2026-11-15  
**Role:** design proposal. `document_role` = `NON_RUNTIME_NON_WIRE_PROPOSAL`.  
**Ratification:** `NOT_RATIFIED`.  
**XC-03-MATRIX:** OPEN.  
**Approval of edges:** none. Every coverage row is `PROPOSED_REQUIRES_USER_DECISION` or `BLOCKED_BY_DEPENDENCY`.  
**Live trading:** NOT AUTHORIZED.

This file inventories labels that already exist and lists scenario families a future matrix would have to cover. It does not publish an enum, does not serialize candidate labels as canonical, and does not assert that any edge is permitted.

DET-01 §6 defines the conceptual function `transition(current_aggregate, command, evidence, policy_snapshot) -> Result<MutationPlan, Rejection>` and states that the allowed edges, guards, closure reasons, and effects are intentionally not listed. This draft stays on that side of the line.

## 1. Label inventory

Confidence is about presence in the repository, not about permission to transition.

| Confidence | Meaning |
| --- | --- |
| `HIGH_PRESENCE_DRAFT_SHELL` | The label is a variant in `crates/trading-domain/src/state.rs` and the same token appears in CR-B10 §2. The enum does not transition. CR-B10 §2 says the names are draft and the matrix is a separate acceptance item. UD-06 says the labels are frozen as draft registry candidates, not as executable rules. |
| `MEDIUM_PRESENCE_REFERENCE_TEXT` | The token appears as reference text in a source contract. It is not a Rust enum. ARCH-DEC-05 added no lifecycle assignment. ARCH-DEC-01 did not ratify reference designs as individual decisions. |
| `LOW_PRESENCE_PROSE` | The word appears in prose or in an ADFR decision title. It is not a state variant. |

### Order workflow labels (`OrderWorkflowState`)

Owner in CR-B10 §1: OMS. Source of the spellings: `crates/trading-domain/src/state.rs` and CR-B10 §2. Confidence: `HIGH_PRESENCE_DRAFT_SHELL`.

| Label | What the shell comment already says | Not implied |
| --- | --- | --- |
| `Draft` | The order has not entered validation. | A created broker order. |
| `Validating` | Validation has been requested. No validator runs. | A passed validation. |
| `AwaitingApproval` | Approval is outstanding. No approver runs. | An approval. |
| `Approved` | Approval was recorded. | Submission. |
| `SubmissionPending` | Submission is pending. | A broker call. |
| `SubmissionInFlight` | A submission attempt is in flight. | A broker call from this crate. |
| `Submitted` | A submission was recorded. | Broker acceptance. |
| `CancelRequested` | Cancellation was requested. | Broker contact, and not `Cancelled`. |
| `ReconciliationRequired` | This workflow requires reconciliation. | The reservation label of the same spelling, and not `ReconciliationRunState::Blocked`. |
| `Rejected` | Local workflow rejection. | `BrokerLifecycleState::Rejected`. |
| `Closed` | Internal workflow closure. | Economic finality, `Released`, and `Filled`. |

`Canceled` is rejected by the decoder. It is not a label. `RECONCILIATION_REQUIRED` is rejected. It is not a shared enum value.

### Broker lifecycle labels (`BrokerLifecycleState`)

Owner in CR-B10 §1: broker adapter. Confidence: `HIGH_PRESENCE_DRAFT_SHELL`.

`NotObserved`, `PendingSubmit`, `Accepted`, `Working`, `PendingCancel`, `Cancelled`, `Rejected`, `Expired`, `Unknown`.

`Accepted` is not `DispatchOutcome::ConfirmedAccepted` and not `RiskDecision::Allow`. `Cancelled` does not erase executions or release risk. `Unknown` is a recorded label. Other text does not become `Unknown`.

### Execution labels (`ExecutionState`)

Owner: execution ledger. CR-B10 §1 has no row for this enum. UD-07 assigns the owner from CR-B10 §2. Confidence: `HIGH_PRESENCE_DRAFT_SHELL`.

`Unfilled`, `PartiallyFilled`, `Filled`, `Disputed`.

`PartiallyFilled` is not finality. `Filled` is not workflow closure and not risk release. `Disputed` does not resolve the dispute.

### Risk reservation labels (`RiskReservationState`)

Owner: risk. Confidence: `HIGH_PRESENCE_DRAFT_SHELL`.

`Active`, `ReconciliationRequired`, `Released`.

There is no `Adjusted` variant. `Released` is not `OrderWorkflowState::Closed`.

### Dispatch outcome labels (`DispatchOutcome`)

Owner: dispatch. Confidence: `HIGH_PRESENCE_DRAFT_SHELL`.

`NotAttempted`, `MayHaveSent`, `ConfirmedAccepted`, `ConfirmedRejected`, `ReconciliationRequired`.

`MayHaveSent` is not a reconciliation run. `ConfirmedAccepted` is not a deserialization default. Unknown text does not become `ConfirmedAccepted`.

### Other draft shells that are not the order workflow

Confidence: `HIGH_PRESENCE_DRAFT_SHELL`. Listed so they are not folded into `OrderWorkflowState`.

| Type | Labels |
| --- | --- |
| `TradingReadinessState` | `NotReady`, `Initializing`, `Synchronizing`, `Ready`, `Degraded`, `Halted`, `Recovering`, `ManualReset` |
| `ReconciliationRunState` | `Created`, `Collecting`, `Comparing`, `Blocked`, `Resolved`, `Failed` |
| `RiskDecision` | `Allow`, `Deny`, `Indeterminate` |

`Ready` grants no submission authority. `Allow` is necessary and not sufficient for dispatch (CR-B10 §7 rule 5). `ManualReset` is not DET-06's reference name `MANUAL_RESET_REQUIRED` (XC-09-RESET). `Resolved` on a reconciliation run is not economic finality.

### Reference tokens that are not Rust labels

| Token | Where it appears | Confidence | Authority |
| --- | --- | --- | --- |
| `PREPARED`, `CLAIMED`, `DISPATCH_STARTED`, `ACKNOWLEDGED`, `OUTCOME_UNKNOWN`, `DEFINITIVELY_NOT_SENT` | DET-07-D03 and CR-B01 §1.3 | `MEDIUM_PRESENCE_REFERENCE_TEXT` | Reference attempt phases. Not an enum. ARCH-DEC-03 binds the fail-closed reading of unknown outcome and takeover. It does not ratify a state machine. |
| `INACTIVE`, `ACTIVE`, `REVOKING`, `FENCED`, `UNCERTAIN` | CR-B01 §1.2 item 1, as `authority_state` | `MEDIUM_PRESENCE_REFERENCE_TEXT` | Reference. Not implemented. |
| `MISSING_LOCAL`, `MISSING_BROKER`, `IDENTITY_CONFLICT`, `EXECUTION_MISMATCH`, `POSITION_MISMATCH`, `RISK_MISMATCH`, `SUBMISSION_UNKNOWN`, `STALE_SNAPSHOT` | DET-08-D03 | `MEDIUM_PRESENCE_REFERENCE_TEXT` | Difference classes, not workflow states. |
| `DETECTED`, `TRIAGED`, `CONTAINED`, `INVESTIGATING`, `RESOLVED`, `CLOSED`, `REOPENED` | DET-10-D03 | `MEDIUM_PRESENCE_REFERENCE_TEXT` | Incident list. Not adopted. `IncidentState` is not a Rust type (UD-07, XC-09-RESET). |
| `PLACE`, `CANCEL`, `MODIFY`, `RISK_REDUCE` | CR-B09 P1-B09-D01 | `MEDIUM_PRESENCE_REFERENCE_TEXT` | Command class names. `RISK_REDUCE` is a policy class, not a broker order type. |
| `COMPLETE`, `PARTIAL/UNKNOWN` | CR-B07 P1-B07-D02 and D03 | `MEDIUM_PRESENCE_REFERENCE_TEXT` | Snapshot completeness words. XC-08-IBKR is OPEN. |
| `closure_reason` | DET-01 §3, ADFR-DET01-001 title | `LOW_PRESENCE_PROSE` | A field name in the reference `OrderClosure` record. The vocabulary is unconfirmed. |
| `HALTED` / `RECOVERING` in CR-B01 §1.1 | CR-B01 §1.1 prose | `LOW_PRESENCE_PROSE` | The Rust spellings are `Halted` and `Recovering` on `TradingReadinessState` only. The CR-B01 sentence is not a second enum. |

No other workflow label was found in `crates/trading-domain/src/state.rs`. This inventory does not add one. Where a scenario needs a label this list does not contain, the coverage row uses `UNSPECIFIED`.

## 2. Transition-record template

A future matrix row, if a user later approves a matrix, would need every field below. This template is `PROPOSED`. No row in this file fills it with an approved edge. `approval_status` on any use of the template in this package is `PROPOSED_REQUIRES_USER_DECISION`, `BLOCKED_BY_DEPENDENCY`, or, for the questions, `USER_DECISION_PENDING`. It is never an approval.

| Field | Required content |
| --- | --- |
| `source_dimension` | One of workflow, broker observation, execution ledger, risk reservation, dispatch attempt, readiness, reconciliation run, or administrative action. |
| `current_label` | A label from section 1, or `UNSPECIFIED`. |
| `trigger_kind` | Command, observation, derived projection, or administrative action. |
| `command_or_evidence` | The command id and payload hash, or the evidence id and raw reference. |
| `trust_and_authorization_guards` | Actor, authorization, readiness, writer epoch, and identity scope. Missing guard means the row cannot fire. |
| `expected_version` | Expected `OrderVersion`, and `WriterEpoch` when the trigger can dispatch. |
| `risk_precondition` | Reservation reference and the DET-02 predicate. If the predicate is undefined, the row cannot increase or release risk. |
| `candidate_next_label` | A label from the same dimension, or `UNSPECIFIED`. Not a cross-dimension assignment. |
| `ledger_effect` | Append, none, or quarantine. Never an overwrite of original evidence. |
| `reservation_effect` | None, rebook, release, compensating entry, or incident. Release requires the economic-finality predicate. |
| `audit_and_outbox_effect` | In the same local unit of work as the mutation. No broker or NATS call inside it. |
| `replay_behavior` | Same identity and same payload returns the stored result. Different payload conflicts. Same execution key has one economic effect. |
| `rejection_or_quarantine` | The fail-closed result when a guard is missing or evidence conflicts. |
| `recovery_behavior` | What restart does. Unknown send stays possibly sent. |
| `source_reference` | File and section. |
| `approval_status` | `PROPOSED_REQUIRES_USER_DECISION` or `BLOCKED_BY_DEPENDENCY` until a later user decision says otherwise. |

## 3. Candidate coverage matrix

These rows are scenario families. They are not permitted edges. `candidate_next_label` is `UNSPECIFIED` whenever choosing it would invent an edge. Missing guards are the reason the family cannot be treated as closed.

| Family | Current labels that exist | Missing guards | Candidate next label | Row status |
| --- | --- | --- | --- | --- |
| Create | No order yet. `Draft` exists as a shell label. | Who may create; idempotency key; instrument and account identity; no risk effect defined. | `UNSPECIFIED` | `PROPOSED_REQUIRES_USER_DECISION` (OMSQ-07) |
| Validate | `Draft`, `Validating` | Validator identity, input versions, fail-closed result for unknown instrument. No validator runs. | `UNSPECIFIED` | `PROPOSED_REQUIRES_USER_DECISION` (OMSQ-07) |
| Risk reserve | `RiskDecision` labels exist. `RiskReservationState::Active` exists. | XC-02-PRED predicate, XC-05-LOCK, XC-07-ROUND, budget dimensions (DET-02 §10). | `UNSPECIFIED` | `BLOCKED_BY_DEPENDENCY` XC-02-PRED, XC-05-LOCK, XC-07-ROUND |
| Submit preparation | `Approved`, `SubmissionPending` | Readiness gate (DET-06, not reviewed), reservation id, writer epoch, capability. `Allow` is not sufficient (CR-B10 §7 rule 5). | `UNSPECIFIED` | `BLOCKED_BY_DEPENDENCY` XC-02-PRED, XC-04, XC-09-RESET |
| Dispatch started | Reference phase `DISPATCH_STARTED`. Shell label `SubmissionInFlight`. `DispatchOutcome::MayHaveSent`. | Persisted attempt before any send (CR-B01 §1.3, EX-INV-04). Fence does not retract a socket write (ARCH-DEC-03). | `UNSPECIFIED` | `BLOCKED_BY_DEPENDENCY` XC-04 |
| Broker ACK or reject | `BrokerLifecycleState::Accepted`, `Rejected`. `DispatchOutcome::ConfirmedAccepted`, `ConfirmedRejected`. Reference `ACKNOWLEDGED`. | Scoped identity (XC-01), raw evidence, distinction between local `Rejected` and broker `Rejected`. ACK is not economic finality (EX-INV-05, CR-B01 §1.3). | `UNSPECIFIED` | `BLOCKED_BY_DEPENDENCY` XC-01, XC-04, XC-08-IBKR |
| Partial or full fill | `PartiallyFilled`, `Filled` | Execution identity scope (XC-01), idempotent ledger key, rebooking rule (XC-02-PRED). | `UNSPECIFIED` | `BLOCKED_BY_DEPENDENCY` XC-01, XC-02-PRED |
| Cancel request or confirm | `CancelRequested`. Broker `PendingCancel`, `Cancelled`. | XC-10-ACTION gates. Request is not confirmation (XC-10-RACE). Confirm is not risk release. | `UNSPECIFIED` | `BLOCKED_BY_DEPENDENCY` XC-10-ACTION, XC-08-IBKR |
| Modify request or confirm | No modify label in `OrderWorkflowState`. | Entire action. CR-B09 P1-B09-D03 blocks modify until broker semantics and the risk delta are verified. | `UNSPECIFIED` | `BLOCKED_BY_DEPENDENCY` XC-10-ACTION, XC-01, XC-02-PRED |
| Timeout or outcome unknown | Reference `OUTCOME_UNKNOWN`. Shell `MayHaveSent`, `ReconciliationRequired`. | Positive evidence before `DEFINITIVELY_NOT_SENT` (CR-B01 §1.3). Timeout is not that evidence (ARCH-DEC-03, DET-01 §7). | `UNSPECIFIED` | `BLOCKED_BY_DEPENDENCY` XC-04 |
| Disconnect or restart | `TradingReadinessState::Recovering`. Reference `UNCERTAIN`. | Isolation of the old writer, attempt reconstruction, no blind retry (OMS-T12, P0-B01-INV-01). | `UNSPECIFIED` | `BLOCKED_BY_DEPENDENCY` XC-04, XC-08-IBKR |
| Reconcile | Workflow `ReconciliationRequired`, run states, DET-08-D03 classes. | Completeness proof (XC-08-IBKR), identity (XC-01), no "broker wins" rule (DET-08-D03). | `UNSPECIFIED` | `BLOCKED_BY_DEPENDENCY` XC-01, XC-08-IBKR, XC-09-RESET |
| Duplicate or conflicting evidence | No dedicated label. `Disputed` exists on the ledger only. | Same-key rule is a principle (OMS-INV-02, DET-01 §4 item 2). Cross-evidence adjudication authority is OMSQ-08. Identity scope is XC-01. | `UNSPECIFIED` | `BLOCKED_BY_DEPENDENCY` XC-01; user decision still required as OMSQ-08 |
| Late fill after closure | `Closed` exists. No late-fill label. | Unambiguous identity before booking (XC-03-BOOK plus CR-B04 §4.3). Closure must stay. Budget incident rule depends on XC-02-PRED. | `UNSPECIFIED`. Closure label stays `Closed` under the baseline principle. | `BLOCKED_BY_DEPENDENCY` XC-01, XC-02-PRED |
| Bust or correction | No bust label. `Disputed` is not a bust. | Original-evidence reference, append-only correction, recomputed projection (DET01-Q09). Valuation and rounding (XC-07-ROUND). | `UNSPECIFIED` | `BLOCKED_BY_DEPENDENCY` XC-01, XC-02-PRED, XC-07-ROUND |
| Manual intervention | `ManualReset` exists as a readiness label. Incident states are reference text and were not adopted. | XC-09-RESET evidence, halt-version CAS, separate authorization. Must not reopen a workflow silently. | `UNSPECIFIED` | `BLOCKED_BY_DEPENDENCY` XC-09-RESET |

No family has `approval_status` `APPROVED`. Create and validate are the only families whose blocker is a user decision on the matrix rather than an external finding, and even those families have `candidate_next_label` `UNSPECIFIED`.

## 4. Negative families and invariants

These are forbidden readings. They are not edges from a label to a label. Stating them does not approve the complementary positive edge.

| Forbidden reading | Baseline source | Coverage consequence |
| --- | --- | --- |
| `CancelRequested` proves `Cancelled` | CR-B10 §2; DET-01 §5; XC-10-RACE | Cancel-request and cancel-observation stay different dimensions. |
| ACK proves executed | DET-07 EX-INV-05; CR-B01 §1.3; DET01-Q01 | `ACKNOWLEDGED` and `ConfirmedAccepted` do not write `ExecutionState::Filled`. |
| Timeout proves unsent | DET-01 §7; CR-B01 §1.3; ARCH-DEC-03 | `OUTCOME_UNKNOWN` is not `DEFINITIVELY_NOT_SENT` and not `DispatchOutcome::NotAttempted`. |
| Workflow closed proves economic finality | DET01-Q01, DET01-Q08; CR-B10 §2 | `Closed` does not write `Filled` and does not release risk. |
| Snapshot missing proves nonexistence | DET-08 RC-INV-04; CR-B07; XC-08-RULE | Incomplete coverage stays `STALE_SNAPSHOT` or unknown. It does not close the attempt. |
| Same idempotency key and a different payload is a replay | DET-01 §4 item 2; OMS-T02 | Conflict, no mutation. |
| Unknown broker identity may be matched by order of arrival | DET-01 §5; CR-B04 §4.3 | Quarantine. |
| A closed workflow may be silently reopened | OMS-INV-03; CR-B03 §3.1 | No reverse edge is proposed. |
| A late fill may be dropped because the workflow is closed | P0-B03-INV-01; DET-01 §5 | The fill remains evidence. Booking still waits on identity. |
| Unverified economic finality may release risk | DET-02 §5; RISK-INV-04; XC-02-PRED | `Released` is not a consequence of closure. |

Further invariants from the semantics note apply here unchanged: CR-B01 precedence over conflicting or less restrictive DET-07 text, no broker or NATS call inside the local unit of work, and no second economic effect for the same execution key.

## 5. Test scenarios

Status of every row: `TESTS_NOT_EXECUTED`. This package does not run OMS functional tests and does not claim they passed. B10-T01 through B10-T12 stay not passed. The existing domain tests check draft labels, ADFR bytes, and registry flags. They do not execute a transition.

| Proposed scenario | Maps to | What a future test would have to show | Status |
| --- | --- | --- | --- |
| Same command twice | OMS-T01, B10-T07 | One economic effect and a stable stored result. | `TESTS_NOT_EXECUTED` |
| Same idempotency key, different payload | OMS-T02 | Conflict and no mutation. | `TESTS_NOT_EXECUTED` |
| Concurrent commands on one version | OMS-T03, P0-B02-T03 | CAS or lock failure, no lost update. Lock order still XC-05-LOCK. | `TESTS_NOT_EXECUTED` |
| Fill after workflow closure | OMS-T04, B10-T08, RC-T03, P0-B03-T01 | Ledger or quarantine updated, closure bytes unchanged. | `TESTS_NOT_EXECUTED` |
| Duplicate fill, different receive time | OMS-T05, P0-B03-T03 | No second booking. | `TESTS_NOT_EXECUTED` |
| Fill, bust, correction, replay | OMS-T06, P0-B03-T04, T05, T10, B10-T10 | Same projection from the same evidence. Original fill retained. | `TESTS_NOT_EXECUTED` |
| Cancel and fill race | OMS-T07, EX-T07, P0-B03-T06 | Neither side finalizes the other or releases risk. | `TESTS_NOT_EXECUTED` |
| Crash before and after local commit | OMS-T08, P0-B02-T05 | Local order, risk, audit, and outbox commit together, or not at all. Not a broker-send proof. | `TESTS_NOT_EXECUTED` |
| Unknown broker status or alias | OMS-T09, CR-B04 T08, B10-T04, B10-T09 | Quarantine or halt. No positive transition. | `TESTS_NOT_EXECUTED` |
| Reconciliation with an incomplete snapshot | OMS-T10, RC-T05, P0-B01-T06 | No absence conclusion. | `TESTS_NOT_EXECUTED` |
| Randomized commands and replay | OMS-T11, B10-T10 | Invariants in section 4 hold for generated sequences. | `TESTS_NOT_EXECUTED` |
| Recovery after an unknown send | OMS-T12, EX-T02, EX-T03, EX-T08, P0-B01-T01 | No blind retry. | `TESTS_NOT_EXECUTED` |
| Workflow versus economic closure | B10-T08 | Late evidence changes the ledger projection only. Closure reason unchanged. | `TESTS_NOT_EXECUTED` |
| Cross-contract fixture round trip | B10-T12, INT-01 | Not runnable until schemas and an approved matrix exist. They do not. | `TESTS_NOT_EXECUTED` |

Model-based generation is required by DET01-Q10 before an executable machine. This document does not supply the generator, the oracle, or a fixture.
