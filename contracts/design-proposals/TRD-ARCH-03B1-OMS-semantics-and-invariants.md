# TRD-ARCH-03B.1 — OMS semantics and invariants

**Date:** 2026-11-15  
**Role:** design proposal. `document_role` = `NON_RUNTIME_NON_WIRE_PROPOSAL`.  
**Ratification:** `NOT_RATIFIED`.  
**XC-03-MATRIX:** OPEN. This file does not resolve it.  
**Live trading:** NOT AUTHORIZED.  
**Canonical schemas:** not published. `part_b` = `NO-GO`. `broker_uniqueness_evidence` = `EVIDENCE_PENDING`.  
**Implementation:** not authorized. `ORDER_STATE_MACHINE_IMPLEMENTED` and `STATE_TRANSITIONS_IMPLEMENTED` stay false. `OrderClosure` stays absent as a Rust type and as a wire schema.

This file decomposes OMS finality so a later user decision can be reviewed. It is not a transition matrix, not an `OrderClosure` vocabulary, and not permission to submit, cancel, modify, or release risk.

Labels in crates are draft shells. Reference names in DET and CR source files are reference text. ARCH-DEC-05 records that no new enum and no additional lifecycle assignment were adopted. A label's presence is not a permitted edge.

Claim classes used below:

| Class | Meaning in this file |
| --- | --- |
| `BINDING_EXISTING` | A recorded user decision, or a DET-01/DET-02 Q01–Q10 principle inside the ARCH-DEC-01 baseline. Verification of those Q cells stays OPEN. Exceptions the baseline already lists stay open. |
| `PROPOSED` | A candidate reading in this package. It needs a later user decision. |
| `OPEN` | Named, unresolved, and not decided here. |
| `DEPENDENCY` | An open cross-contract finding. An OMS-only answer cannot close it. |
| `UNVERIFIED` | Source text, a test id, or broker behavior without executed evidence. `NOT_REVIEWED` source text stays in this class unless a decision record quotes a narrower rule. |

## BINDING_EXISTING

### Decisions that bound this package

| Decision | Binding effect used here | What it leaves open |
| --- | --- | --- |
| ARCH-DEC-01 | DET-01 and DET-02 are `DESIGN_BASELINE_RATIFIED_WITH_EXCEPTIONS`. The Q01–Q10 principles in those files are the design baseline. | Original Q&A transcripts are not present. Every detail those files mark open, undefined, unconfirmed, reference design, or verification-required stays open. That includes the `OrderClosure` reason vocabulary, allowed transition edges, reference DDL, and OMS-T01..T12 as unexecuted tests. Q-cell verification stays OPEN. |
| ARCH-DEC-02 | 78 historical ADFR IDs, one supersession `ADFR-CROSS-005` to `ADFR-CR-B10-001`, 77 not superseded, every `verification` value `NOT_VERIFIED`. | Status-word disagreement stays OPEN (XC-11). `DESIGN_RESOLVED` is not a test pass. |
| ARCH-DEC-03 | Where DET-07 conflicts with CR-B01, or where DET-07 is less restrictive, CR-B01 is the design reading. No automatic takeover while the previous writer might still send or submission results are unresolved. Elapsed time or a lease timeout is not proof of a safe takeover. An in-flight send or an unknown submission result stays blocked as a design invariant. | Broker-session exclusivity, lease and heartbeat, dispatch scope, in-flight send evidence, recovery tests, and IBKR behavior stay OPEN (XC-04 apart from precedence). The precedence is not a runtime fence. |
| ARCH-DEC-04 | No winning status and no default precedence among `OPEN P0`, `PROPOSED_FOR_ACCEPTANCE`, and `DESIGN_RESOLVED`. Those words are not equivalent. | XC-11 status-word disagreement stays OPEN. |
| ARCH-DEC-05 | No new closed lifecycle enum. `rejected` and `withdrawn` stay unassigned. | The proposed lifecycle token set stays `PROPOSED_NOT_RATIFIED`. |
| ARCH-DEC-06 | `IMPLEMENTED_AND_LOCALLY_TESTED` is scoped to the named type-shell part. | It does not verify an ADFR row and it does not implement OMS. |
| ARCH-DEC-07 | A later verification promotion needs a dated user approval and an evidence record. | No evidence URI is recorded. No promotion is performed. |
| ARCH-DEC-08 | Manifest placement and provenance fields stay as recorded. | DET-01 and DET-02 stay `RECONSTRUCTED_DRAFT` and `SOURCE_REVIEW_PENDING`. |

`NO_CONFLICT_FOUND` in the TRD-ARCH-SYNC-01 comparison is agreement on a fail-closed principle. It is not permission to implement a predicate or an edge. The principles cited that way, and treated here as binding only at that principle layer, are:

- Workflow closure, timeout, and cancel acknowledgement are not reservation release (XC-02-ALIGN; DET-02 §2 Q06 and §5; DET-01 §3 and §5; CR-B10 §2).
- A late fill is new evidence. It does not rewrite historical workflow closure or the original fill (XC-03-BOOK; DET-01 §5; CR-B03 §3.1; DET-08-D04). Automatic booking still requires a single consistent identity match. Ambiguous identity is not booked by that reading (CR-B04 §4.3).
- A cancel request is not a cancel confirmation and is not proof that an unknown submission was never sent. A recorded broker cancel does not erase executions or release risk (XC-10-RACE; DET-01 §5; CR-B09 P1-B09-D02; CR-B10 §2 and §7 rule 6; DET-02 §5).
- An empty or incomplete snapshot is not proof that an order is absent and is not `DEFINITIVELY_NOT_SENT` (XC-08-RULE; DET-08-D02; RC-INV-04; DET-01 §7; CR-B01 §1.2 item 7).
- The local commit includes the outbox row. Broker and NATS calls sit outside that transaction (XC-05-OUTBOX; DET-01 §4 items 4 and 5; DET-04; CR-B02 §2.1; CR-B10 §7 rule 10).
- A missing independent second person, where one is required, is deny. Break-glass does not bypass hard risk limits or fencing (XC-09-SOLO).
- Missing or contradictory FX is indeterminate. There is no silent 1:1 conversion (XC-07-FX).

### DET-01 principles inside the baseline

DET-01 §2 records these principles. Their verification cells stay OPEN. Reference designs in the same file are not individual confirmed decisions (DET-01 §0; ARCH-DEC-01).

| Id | Principle | Section that limits it |
| --- | --- | --- |
| DET01-Q01 | Closure reason is separate from workflow state. Workflow closure is not economic finality. | §3. The reason vocabulary is unconfirmed. |
| DET01-Q02 | Conflicts are resolved from evidence, conservatively. Ambiguity goes to reconciliation or a manual decision. | §5. No ordering by receipt time, UUID order, or last status string. |
| DET01-Q03 | The aggregate version is monotone. Command ids, event ids, and broker evidence are persisted. | §4. Identifier types stay non-interchangeable. |
| DET01-Q04 | Concurrency is CAS plus targeted PostgreSQL locks. | §4 item 3. The global lock order is CR-B02 §2.2 and is `OPEN` as XC-05-LOCK. |
| DET01-Q05 | The execution ledger is idempotent. Corrections are controlled and audited. | §5. Original fills are not overwritten. |
| DET01-Q06 | The order aggregate is relational, with transactional command, event, and evidence history. | §4. Table names and DDL are reference and not finalized. |
| DET01-Q07 | Recovery is explicit. Unclear state is not silently continued. | §7. `DISPATCH_STARTED` or `OUTCOME_UNKNOWN` is not treated as unsent because a timeout elapsed. |
| DET01-Q08 | Finality is multidimensional. Historical workflow closure can stand while later economic facts are still applied. | §3 and §5. |
| DET01-Q09 | Corrections are append-only, keep original evidence, and are auditable. | §5. |
| DET01-Q10 | A formal transition model, model-based tests, deterministic replay, and safety invariants are required before an executable machine. | §6. Allowed edges are intentionally absent. |

DET-01 invariants OMS-INV-01 through OMS-INV-07 are part of that same baseline text. They are not executed tests. OMS-T01 through OMS-T12 remain `TESTS_NOT_EXECUTED`.

Registry and shell facts that are already true in this repository, and that this package does not change:

- `OrderWorkflowState`, `BrokerLifecycleState`, `ExecutionState`, `RiskReservationState`, `DispatchOutcome`, `TradingReadinessState`, `ReconciliationRunState`, and `RiskDecision` are draft labels in `crates/trading-domain/src/state.rs` and CR-B10 §2. They do not transition.
- `OrderVersion` is the OMS aggregate counter. `WriterEpoch` is the local dispatch fencing counter. Equal numeric values are not the same fact (`crates/trading-domain/src/version.rs`). `AdapterSessionEpoch` is neither of those.
- `CommandId`, `EventId`, `OrderId`, `SubmissionAttemptId`, `RiskReservationId`, and broker alias types are distinct. Copying UUID bits does not link them.
- `ORDER_STATE_MACHINE_IMPLEMENTED` and `STATE_TRANSITIONS_IMPLEMENTED` are false. `trading_domain::OrderClosure` is rejected by a compile-fail doctest.

## PROPOSED

The decomposition below is the candidate model for review. It restates the baseline dimensions in a form a later matrix can cite. It adds no edge and no reason token.

### Independent dimensions

These projections stay independent. A value on one dimension does not write a value on another.

| Dimension | What it records | What a value does not prove |
| --- | --- | --- |
| Command / workflow state | Local command progress and historical workflow closure. Draft label set: `OrderWorkflowState`. | Broker acceptance, economic finality, risk release, or a successful socket send. `Closed` is historical workflow closure. |
| Broker lifecycle and raw evidence | A normalized observation plus the raw broker payload, scoped identity, and provenance. Draft label set: `BrokerLifecycleState`. | Execution completeness, absence of a later fill, or a unique broker identity. `Unknown` is a known label for an observed unknown status. Unrecognized text is not coerced to `Unknown`. `Accepted` is not `DispatchOutcome::ConfirmedAccepted`. |
| Execution ledger / economic projection | Append-only execution and correction evidence, and a projection derived from the valid evidence set. Draft label set: `ExecutionState`. | Workflow closure. `Filled` and `PartiallyFilled` are ledger labels. `Disputed` records a dispute. This crate does not resolve it. |
| Risk reservation reference | A reference to a risk reservation and its own lifecycle. Draft label set: `RiskReservationState`. | Workflow closure. `Released` is not `OrderWorkflowState::Closed`. `RiskReservationAdjusted` is an event name, not a variant. |
| Historical closure reason and evidence | A closure fact and the evidence that justified recording it. The reason vocabulary is `OPEN`. | Economic finality, broker terminality, or permission to release risk. |

Five outcomes stay distinct even when they occur around the same order:

| Outcome | Owning question | Fail-closed reading already in the baseline |
| --- | --- | --- |
| Workflow closure | Did the local workflow record a historical end? | The record is not deleted and is not silently reopened (DET01-Q01, DET01-Q08, OMS-INV-03). |
| Broker terminal observation | Did a scoped broker observation say the broker order was cancelled, rejected, or expired? | The observation is evidence. It is not execution truth and it does not release risk (CR-B10 §7 rules 4 and 6). |
| Economic finality | Is the executed quantity, including busts and corrections, evidenced completely enough for economics? | No unconfirmed broker status forces economic finality (OMS-INV-04). The predicate itself is `OPEN` (XC-02-PRED). |
| Risk-release eligibility | May reserved budget be released or rebooked? | Closure, timeout, cancel acknowledgement, and an incomplete snapshot do not release a reservation (DET-02 §5, RISK-INV-04). |
| Dispatch outcome | What is known about a particular send attempt? | Timeout is not "unsent". `ACKNOWLEDGED` in CR-B01 §1.3 is acceptance or correlation evidence, not economic finality. `DispatchOutcome::MayHaveSent` is not `ConfirmedAccepted`. |

`TradingReadinessState` and `ReconciliationRunState` are further independent labels. HTTP readiness is neither of them. `ReconciliationRequired` on the workflow, on the reservation, and on `DispatchOutcome` are three labels. `ReconciliationRunState` uses `Blocked` for the run.

### Identifiers that do not collapse

| Identifier | Role | Non-equivalence |
| --- | --- | --- |
| `OrderVersion` | Monotone counter of authoritative local order mutations. Overflow is an error (OMS-INV-01). | Not a timestamp order, not a UUID order, not `WriterEpoch`, not `SchemaVersion`. |
| `WriterEpoch` | Local dispatch fencing counter. `checked_next` in process is not the database fence (UD-04; `version.rs`). | Not an external broker fence. Not `AdapterSessionEpoch`. A database lease does not retract an in-flight socket write (ARCH-DEC-03). |
| Opaque broker identifiers | Broker-supplied `orderId`, `permId`, `execId`, and client text, stored as unverified aliases. | Not an internal primary key. Kind-specific uniqueness is `EVIDENCE_PENDING` (XC-01, CR-B04 §4.1–§4.2). |
| `CommandId` | One persisted command. | Not `OrderId`, not `EventId`, not an idempotency key by itself. |
| `EventId` | One persisted domain event. | Not `aggregate_version`. CR-B10 §4: schema version is not aggregate version. |
| Evidence and attempt ids | `SubmissionAttemptId`, broker observation id, execution evidence id, `audit_record` reference. | Not interchangeable with the order id. A new attempt is a new id. |

`PROPOSED` correlation rule, already required as separate fields by DET-01 §4 item 1 and CR-B02 §2.4: a command carries `CommandId`, an idempotency key, a payload hash, an actor and authorization context, `CorrelationId`, the expected `OrderVersion`, and the input versions it read. Those fields identify the command. They do not identify the broker order.

### Historical closure and later economic facts

`BINDING_EXISTING` at principle level:

- A persisted historical closure is not deleted and is not silently reopened (DET-01 §3, OMS-INV-03, CR-B03 §3.1, CR-B10 §2).
- A fill after workflow closure is captured as new evidence. When identity is unambiguous it is booked. The closure remains (DET-01 §5, OMS-T04 as an unexecuted expectation).
- Duplicate execution identity does not create a second economic effect (OMS-INV-02, DET-02 §5).
- Busts and corrections reference the original execution. They do not overwrite it. The projection is recomputed from the valid evidence set (DET01-Q09, DET-01 §5, DET-02 §5).
- If the reservation or budget cannot cover the economic fact, the fact is still retained and a risk incident or halt is required (DET-01 §5, CR-B03 §3.2). Dropping the fill is not the fail-closed action.
- Partial fill rebooks open reservation into exposure only under a clear rule (DET02-Q05). That rule's arithmetic is `OPEN`.

`PROPOSED` handling, pending OMSQ-01 and OMSQ-05:

- The closure row is written once.
- Later fills, duplicates, partials, busts, and corrections are append-only evidence and append-only ledger corrections.
- A correction event may record that the economic projection changed. It does not edit `closure_reason` and it does not move `OrderWorkflowState` away from `Closed`.
- Conflicting evidence stays quarantined. It does not choose a winner by arrival time.

Whether the closure row itself may be followed by an explicit append-only correction record, without reopening the workflow, is OMSQ-01. This file does not decide it.

### Commands, observations, projections, administrative actions

| Kind | Examples present as names only | Trusted evidence before any future effect | Replay expectation in the baseline | Effect this package allows | Still prohibited or unknown |
| --- | --- | --- | --- | --- | --- |
| Command | Place, cancel, modify, and risk-reduce are named in CR-B09 P1-B09-D01. Local workflow commands are not separately enumerated. | `CommandId`, idempotency key, payload hash, actor, expected `OrderVersion`, referenced input versions (DET-01 §4). Authorization and risk gates are required (OMS-INV-07) and are not implemented. | Same key and same payload return the stored result. A different payload is a conflict and no mutation (DET-01 §4 item 2, OMS-T02). | None. No command handler exists. | Blind resend. A retry of a database unit of work is not a broker send (CR-B02 P0-B02-INV-04, `UNVERIFIED` as a test). |
| Observation | Broker lifecycle text, execution report, snapshot row. | Raw payload, scoped identity, normalizer version, received time, session or connection generation. Ambiguous identity goes to quarantine (DET-01 §5, CR-B04 §4.3). | Same scoped execution key has one economic effect (OMS-INV-02). | None in this repository. | Treating `Accepted`, `Cancelled`, `Rejected`, or `Expired` as economic finality. Treating an incomplete snapshot as absence. |
| Derived projection | Workflow label, execution label, position or risk usage. | Recomputed from persisted commands, events, and evidence (DET-01 §4 reference note). | Same pinned evidence yields the same projection (CR-B10 B10-T10, `TESTS_NOT_EXECUTED`). | None. | Using the projection as a substitute for missing evidence. |
| Administrative action | Manual reset, break-glass, incident transition, halt clear. Names exist in DET-10 and CR-B05. | Fresh bound evidence, expected halt version, separate authorization (CR-B05 P1-B05-D03; DET-10-D03..D07). | The action is a new audited command. It does not rewrite history (DET-10-D01, OPS-INV-05). | None. `IncidentState` is not a Rust type. | Silent reopen. Simulated second approval. Reset while a critical difference is unresolved (DET-08-D05, OPS-INV-06). XC-09-RESET stays OPEN. |

### Conservative trust boundary

`BINDING_EXISTING` fail-closed rules, restated as the boundary any later matrix has to keep:

| Condition | Required posture |
| --- | --- |
| Unknown broker alias or unknown external order | Quarantine. No automatic match (CR-B04 §4.3 item 4, OMS-INV-06, OMS-T09). |
| Incomplete snapshot or missing coverage | No absence proof and no `DEFINITIVELY_NOT_SENT` (XC-08-RULE, DET-01 §7, RC-INV-04). |
| Missing broker uniqueness evidence | `EVIDENCE_PENDING`. No production uniqueness constraint (XC-01). |
| Stale `OrderVersion` or stale `WriterEpoch` | Reject or halt. No permissive fallback (DET-01 §4 item 6, EX-INV-03). |
| Unknown submission outcome, including timeout after `DISPATCH_STARTED` | Treat as possibly sent. No blind resend. No automatic takeover (ARCH-DEC-03, DET-01 §7, CR-B01 §1.3, EX-INV-02). |
| Conflicting evidence | No resolution by receipt time, UUID order, or last status string (DET-01 §5). |
| Request to reopen a closed workflow without an explicit new decision | Refuse. This package proposes no such decision. |
| Risk release requested from a cancel request, a timeout, or workflow closure alone | Refuse (RISK-INV-04). |

`PROPOSED` additional boundary, pending OMSQ-03 and OMSQ-09: an unknown send outcome cannot be the evidence that closes the workflow. A broker terminal label cannot be accepted until scoped identity, completeness for the named scope, and provenance are all present. Until then the observation is quarantine or reconciliation, not closure.

### Transactional shape

`BINDING_EXISTING` at principle level (DET-01 §4 items 3–5, DET02-Q03, XC-05-OUTBOX):

- One local PostgreSQL unit of work commits the order aggregate, command result, domain event, authoritative audit entry, outbox row, and the risk and ledger mutations that belong to that decision.
- No broker call and no NATS call run inside that transaction. A successful commit is not a broker send.
- CAS uses `expected_order_version`. Locks, where required, follow the global lock order. That order is not canonical (XC-05-LOCK).
- Idempotency conflicts and lock failures roll back the local unit. They do not release risk and they do not repeat an external send.

`PROPOSED` and explicitly not a DDL claim: the reference names `orders`, `order_commands`, `order_domain_events`, `order_evidence`, `order_closures`, `execution_ledger`, `execution_corrections`, `audit_events`, and `outbox_messages` remain the DET-01 §4 reference list. This package does not finalize those names, does not create tables, and does not claim the unit of work is implemented.

`OPEN` inside that shape: which row is the authoritative audit record (XC-06), the lock order and isolation level (XC-05-LOCK), and how a failed outbox publish is retried after commit without a second economic effect (the principle says publish is after commit and at-least-once; the operational retry policy is not implemented).

## OPEN

| Item | Why it stays open |
| --- | --- |
| XC-03-MATRIX | Allowed edges, guards, closure reasons, and effects are intentionally absent (DET-01 §6). This package draws coverage and negative families only. |
| `OrderClosure` reason vocabulary | DET-01 §3 says it is not confirmed. UD-07 left the type unimplemented. OMSQ-02 asks for a strategy. |
| Executable economic-finality predicate | XC-02-PRED. DET-02 §10. No release rule is approved here. |
| Broker uniqueness and terminal-observation acceptance | XC-01 and XC-08-IBKR. |
| Unknown-send closure | OMSQ-03. The baseline forbids treating timeout as unsent. It does not by itself choose a closure reason. |
| Cancel, modify, and replace guards | XC-10-ACTION. The race inequality is a principle. The action gates are not implemented and not IBKR-verified. |
| Manual adjudication and reset | XC-09-RESET. OMSQ-08. |
| Lock order, isolation, and audit authority | XC-05-LOCK and XC-06. OMSQ-10 and OMSQ-11. |
| Q01–Q10 verification | OPEN cells. Baseline ratification is not verification. |

## DEPENDENCY

An OMS-only document cannot close these. Each one needs evidence or a decision outside the order workflow.

| Finding | What OMS still needs from outside | Why an OMS answer is not enough |
| --- | --- | --- |
| XC-01 | Kind-specific identity scopes for `orderId`, `permId`, and `execId`, after broker evidence. | Dedup, quarantine, and late-fill booking key on that scope (DET-01 §5, CR-B04). |
| XC-02-PRED | The predicate "confirmed economic finality", budget metrics, and the compensating entry after release. | DET-02 owns release. OMS must not release risk by closing a workflow. |
| XC-04 | Session exclusivity, lease and heartbeat, dispatch scope, in-flight evidence, recovery tests, IBKR behavior. ARCH-DEC-03 settled only the reading precedence. | `DISPATCH_STARTED` and takeover are dispatch facts. CR-B01 remains more restrictive than DET-07 where they differ. |
| XC-05-LOCK | A lock order and isolation level that have been tested, then accepted. | DET01-Q04 points at CR-B02 §2.2, and that section calls itself a proposal pending deadlock tests. |
| XC-06 | Which persisted audit artifact is authoritative, distinct from outbox export. | DET-04-D01 does not name `audit_events`. Source status words disagree. |
| XC-08-IBKR | Tested IBKR completion signals, history retention, and account scope before any adapter reports `COMPLETE`. | Snapshot completeness is a broker-evidence fact (CR-B07). OMS cannot invent a negative proof. |
| XC-09-RESET | Reset evidence, halt-version CAS, and role enforcement. `IncidentState` is not adopted. | Manual reopen or reset is an operations decision (DET-10, CR-B05), not an OMS edge. |
| XC-10-ACTION | Broker capability and risk-delta evidence for cancel during halt, modify, and risk-reduce. | CR-B09 is `EXTERNAL_VERIFICATION_REQUIRED` at GATE-P1-09. |

XC-07-ROUND is a further dependency for any economic quantity that would be rounded. This package does not choose a rounding mode. XC-11 stays OPEN and is not an OMS transition question. ARCH-DEC-04 remains in force.

Later work is named only as a dependency, not started: ARCH-03B.2 for risk release and rebooking, ARCH-03B.3 for broker evidence and terminal observations. No broker evidence was supplied to this task. None is invented.

## UNVERIFIED

- DET-01 and DET-02 remain `RECONSTRUCTED_DRAFT`, `SOURCE_REVIEW_PENDING`, `reviewed` false, `approved` false.
- DET-03 through DET-10 and CR-B01 through CR-B10 remain `NOT_REVIEWED`. Their `DESIGN_RESOLVED` or `P1_DESIGN_RESOLVED` self-labels are source claims. ARCH-DEC-04 forbids treating those words as a higher status.
- CR-B04 identity matrix is a hypothesis list. `broker_uniqueness_evidence` stays `EVIDENCE_PENDING`.
- OMS-T01..T12, B10-T01..T12, RISK-T01..T14, P0-B01 tests, and the DET-07/DET-08 test ids are specified and not executed.
- No IBKR session, gateway, or paper adapter was run.
- Reference DDL, API signatures, and policy names are not schemas.

## Invariants this proposal keeps

These are the baseline invariants plus the non-implications this package refuses to weaken. They are not new runtime checks.

1. `order_version` advances only on an authoritative local mutation, and overflow fails (OMS-INV-01).
2. The same command identity and the same scoped execution identity have one economic effect (OMS-INV-02).
3. Historical workflow closure stays in place while later economic evidence is appended (OMS-INV-03).
4. An unconfirmed broker status does not force economic finality (OMS-INV-04).
5. An authoritative mutation carries transactional audit and event evidence (OMS-INV-05). The audit artifact's name is `OPEN` (XC-06).
6. Unclear identity, incomplete broker evidence, or an unknown transition fails closed (OMS-INV-06).
7. No command skips risk, readiness, authority, or submission gates (OMS-INV-07). Those gates are not implemented.
8. `CancelRequested` does not prove `Cancelled`. A broker ACK does not prove execution. A timeout does not prove the attempt was unsent. Workflow closure does not prove economic finality. A missing snapshot does not prove nonexistence.
9. The same idempotency key with a different payload is a conflict.
10. Unknown broker identity is quarantine.
11. A closed workflow is not silently reopened.
12. A late fill is not discarded because the workflow is closed.
13. Unverified economic finality does not release risk.
14. CR-B01 precedence over conflicting or less restrictive DET-07 text stays in force (ARCH-DEC-03).
