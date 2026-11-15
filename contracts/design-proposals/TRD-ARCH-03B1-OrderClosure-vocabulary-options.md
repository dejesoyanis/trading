# TRD-ARCH-03B.1 — OrderClosure vocabulary options

**Date:** 2026-11-15  
**Role:** design proposal. `document_role` = `NON_RUNTIME_NON_WIRE_PROPOSAL`.  
**Ratification:** `NOT_RATIFIED`. The recommendation in section 5 is not a user decision.  
**XC-03-MATRIX:** OPEN.  
**Live trading:** NOT AUTHORIZED.

No Rust enum, database column, wire payload, or automatic transition is specified here. The alternatives are review options for OMSQ-02.

## 1. Terms that already exist

| Term | Where it appears | What it is | What it is not |
| --- | --- | --- | --- |
| `OrderClosure` | CR-B10 §1 row "Closure", owner OMS. DET-01 §3 reference record. UD-07. Glossary "Explicitly absent". Registry README. | A named future record: closure reason, closing event id, closing `OrderVersion`, occurred and recorded time, audit reference. DET-01 §3 also lists `order_id` and evidence references. | A Rust type. `trading_domain::OrderClosure` is a compile-fail. Not a registry schema row that publishes a wire form. |
| `closure_reason` | DET-01 §3. ADFR-DET01-001 decision text "Hybrid Workflow Closure mit immutable closure_reason". CR-B03 §3.1 and P0-B03-INV-03. | The field that DET01-Q01 separates from workflow state. | A confirmed list of values. DET-01 §3 says the vocabulary is not confirmed from an authoritative primary contract. |
| `Closed` | `OrderWorkflowState` in `state.rs` and CR-B10 §2. | Draft label for internal workflow closure. | Economic finality. Not `Released`. Not `Filled`. Not broker `Cancelled`. |
| `Cancelled` | `BrokerLifecycleState`. | Draft label for a recorded broker cancellation. Spelling `Canceled` is rejected. | Proof that no execution occurred. Not a workflow edge. |
| `Filled`, `PartiallyFilled`, `Unfilled`, `Disputed` | `ExecutionState`. | Draft ledger labels. | Closure reasons. |
| `Released` | `RiskReservationState`. | Draft reservation label. | A closure reason. |
| `Rejected` | Both `OrderWorkflowState` and `BrokerLifecycleState`. | Two different labels on two types. | One shared reason code. |
| `Expired`, `Unknown` | `BrokerLifecycleState`. | Draft observation labels. | Closure reasons. |
| Reference attempt words | CR-B01 §1.3 and DET-07-D03: `ACKNOWLEDGED`, `OUTCOME_UNKNOWN`, `DEFINITIVELY_NOT_SENT`, and the phases before them. | Dispatch evidence words. | An `OrderClosure` vocabulary. |
| Incident `CLOSED` | DET-10-D03. | A reference incident state. Not adopted as `IncidentState`. | Order closure. |

ADFR-DET01-001 through ADFR-DET01-010 remain `CONFIRMED_DECISION` with source string `conversation: confirmed DET-01/02 Q01-Q10`, lifecycle `NOT_SUPERSEDED`, verification `NOT_VERIFIED`. ARCH-DEC-01 did not rewrite that provenance and did not verify the rows. The title of ADFR-DET01-001 is not a reason list.

### What is absent

The authoritative complete `closure_reason` vocabulary is absent. The permitted edges that would write a closure are absent (DET-01 §6, XC-03-MATRIX). No file in this repository contains an accepted set of reason tokens. This package does not supply that set as a decision.

## 2. Why one flag is not enough

A single `Closed` / `Cancelled` / `Filled` flag collapses dimensions the baseline keeps apart. Concrete cases:

| Situation | What a single flag would hide |
| --- | --- |
| Workflow `Closed` after a local reject, then a late fill with a scoped execution id | The flag would read as finished. DET-01 §5 requires the fill to be kept and, when identity is unambiguous, booked. The closure reason must still be the historical one (P0-B03-INV-03). |
| `CancelRequested`, then a partial fill, then a broker `Cancelled` | One flag cannot say whether the cancel was only requested, whether the fill stands, and whether any quantity remains unknown. XC-10-RACE forbids treating the request as confirmation or as risk release. |
| `DISPATCH_STARTED`, then a timeout, then no snapshot row | Calling that `Cancelled` or unsent treats a timeout as a broker result. ARCH-DEC-03 and CR-B01 §1.3 forbid that. |
| Broker `Cancelled` while the ledger has a fill the cancel does not mention | CR-B10 §7 rule 6: recorded cancellation does not erase executions or justify risk release. |
| Incomplete open-order snapshot that omits the order | XC-08-RULE: omission is not nonexistence and not `DEFINITIVELY_NOT_SENT`. A `Closed` flag would assert absence the snapshot did not prove. |
| Fill booked, reservation already `Released`, then a bust that increases exposure | CR-B03 §3.2: do not secretly reopen the old reservation; append a compensating entry and, if the budget breaks, incident and halt. A `Filled` flag has no place to put the bust. |
| Same idempotency key, two payloads, one of which says "close" | DET-01 §4 item 2: conflict, no mutation. A flag write would be the mutation. |

## 3. Alternatives

All three keep workflow closure distinct from economic finality. None is approved. None is a schema.

### Alternative A — Minimal reason families, evidence typed beside them

A small family set, recorded only as a design sketch:

- `LOCAL_END` for a workflow end that happened before a broker send was possible, with evidence that the attempt never reached `DISPATCH_STARTED`.
- `BROKER_OBSERVATION` for a closure justified by a quarantined-or-accepted broker terminal observation. The observation id is required. The family does not say the economics are final.
- `UNRESOLVED_HOLD` is intentionally not a closure family. Unknown send and incomplete snapshots do not close.

Each stored closure would carry the family, the closing `EventId`, the closing `OrderVersion`, timestamps, the audit reference, and a typed evidence reference. Economic finality would be a separate predicate over the ledger (XC-02-PRED), not a family.

| Topic | Reading under A |
| --- | --- |
| Closure versus economic finality | The family never means final economics. `LOCAL_END` means only that no send was evidenced. `BROKER_OBSERVATION` means only that a cited observation was accepted under OMSQ-09. |
| Evidence | `LOCAL_END` needs positive non-send evidence (CR-B01 §1.3: only `PREPARED` or `CLAIMED` can become `DEFINITIVELY_NOT_SENT`, and only with proof). `BROKER_OBSERVATION` needs scoped identity, raw payload, and completeness for that observation's scope. |
| Late fills and corrections | Append-only. Family and original reason stay. Ledger updates do not change the family. |
| Cancel versus fill | A cancel observation may justify `BROKER_OBSERVATION` only after the race is explicit in the evidence. The fill remains in the ledger. The family does not release risk. |
| Unknown outcome | No family. The attempt stays unknown. Workflow stays unclosed or `ReconciliationRequired` (the edge is still `UNSPECIFIED`). |
| Persistence and audit | One closure row plus evidence ids. Corrections are other tables. Names stay the DET-01 §4 reference list, not DDL. |
| Compatibility | Closest to the current absent type: one immutable record, as UD-07 describes. Adding a family later is a new major vocabulary version if a family meaning changes (CR-B10 §5, itself `NOT_REVIEWED`). |
| Validation burden | Small token set. The burden moves to the evidence reference, which XC-01 and XC-08-IBKR do not yet support. |
| Risk and broker dependency | Release stays outside the family. Broker terminality depends on XC-08-IBKR and XC-01. |
| Fail-closed behavior | Unknown maps to no closure. A missing evidence id rejects the closure write. |
| Advantage | Hard to misuse as a finality flag. Small review surface. |
| Disadvantage | Operators cannot see why a broker observation closed the workflow without opening the evidence. Two families may be too coarse for audit sampling. |

### Alternative B — Richer discriminated reasons

One token per story, still only a sketch: local validation rejection, approval withdrawn, broker rejected, broker cancelled, broker expired, superseded by an explicit replacement after proven non-execution, and manually closed under a separate authorization. Each token would imply its own evidence shape. Economic tokens such as "fully filled" would be excluded so the reason cannot mean economic finality. Unknown send would have no token.

| Topic | Reading under B |
| --- | --- |
| Closure versus economic finality | A token explains the workflow story. It still does not set `ExecutionState` or `Released`. The exclusion of a "filled" reason has to be enforced by review, because the token list looks like an outcome list. |
| Evidence | Each token has a required evidence shape. A cancelled token requires a scoped cancel observation and does not require an empty ledger. A local-rejection token requires proof the attempt was not sent. |
| Late fills and corrections | The token stays. A late fill after `broker cancelled` is the normal late-fill case, which is why the token must not mean "no fill exists". |
| Cancel versus fill | The token can say a cancel observation was the closure trigger while the ledger still holds fills. The race needs a guard that the token is not written before conflicting evidence is classified. That guard is not defined (XC-10-ACTION). |
| Unknown outcome | No token. Writing the nearest token, such as cancelled or rejected, is a forbidden coercion. |
| Persistence and audit | The reason column becomes a discriminator. Adding a token is a vocabulary version. Old rows keep their token. |
| Compatibility | Higher migration cost. A wrong token is hard to correct without violating immutability, so OMSQ-01's correction rule becomes more important. |
| Validation burden | Every token needs a test that it cannot be produced from timeout, from an incomplete snapshot, or from the other dimension's label of similar spelling (`Rejected` versus broker `Rejected`). |
| Risk and broker dependency | Stronger temptation to release risk on `broker cancelled`. The alternative forbids that, but the dependency on XC-02-PRED remains. Broker tokens depend on XC-08-IBKR. |
| Fail-closed behavior | Good if unknown tokens fail closed. Weak if a rich list invites a default token. |
| Advantage | Audit can group closures without joining evidence. Discriminated evidence shapes can be checked. |
| Disadvantage | Looks like the single flag section 2 rejects, unless every token's non-implications are tested. Easy to grow into a hidden state machine. Not authorizable while XC-10-ACTION and XC-08-IBKR are open. |

### Alternative C — Reason family plus independent terminality evidence

The closure record stores a coarse family from a set no larger than Alternative A, and it stores separate evidence slots that are allowed to stay empty or `UNKNOWN`:

- workflow-closure evidence (why the workflow row was closed)
- broker-terminality evidence (present, absent, or incomplete)
- economic-finality evidence (present, absent, or incomplete)
- risk-release evidence (a reference only; release is a DET-02 decision)

A closure is valid only when the workflow-closure slot is sufficient for that family. The other slots may remain incomplete. Incomplete economic or broker slots do not prevent recording a local historical end when non-send is positively evidenced, and they do not authorize risk release. They also do not allow a broker-observation family to be written.

| Topic | Reading under C |
| --- | --- |
| Closure versus economic finality | The record has both facts as separate slots. Emptiness of the economic slot is an explicit unknown, not an implied zero. |
| Evidence | Workflow closure cites the command or the observation that met the family rule. Broker terminality cites a scoped observation plus completeness for the named scope, or is marked incomplete. Economic finality cites the ledger predicate, which remains `OPEN` under XC-02-PRED, so the slot stays incomplete. |
| Late fills and corrections | A late fill appends ledger evidence and may fill or still leave incomplete the economic slot. It does not rewrite the workflow-closure slot (OMS-INV-03, P0-B03-INV-03). |
| Cancel versus fill | The cancel observation may complete the broker-terminality slot without completing the economic slot. The fill remains. Neither slot releases risk. |
| Unknown outcome | All terminality slots stay incomplete. No closure family is written. The dispatch attempt stays `OUTCOME_UNKNOWN` or `MayHaveSent` as already-named labels, without a new edge. |
| Persistence and audit | One immutable closure row when OMSQ-01 alternative "write once" is chosen, with slots that are themselves immutable. Later knowledge is a new evidence row linked from the ledger or from reconciliation, not an update of the closure slots. If OMSQ-01 later chooses append-only correction records, those records add a link. They do not edit the original slots. |
| Compatibility | Matches UD-07's immutable record and DET-01's multidimensional finality more closely than B. Slot meanings need a version if they change. |
| Validation burden | Higher than A because each slot has its own sufficiency rule. Lower operational risk than B because missing evidence stays visible. |
| Risk and broker dependency | The risk-release slot cannot be set until XC-02-PRED is decided. The broker slot cannot be set to sufficient until XC-01 and XC-08-IBKR are decided. The workflow slot for a pre-send local end still needs the non-send proof from CR-B01, which is not implemented. |
| Fail-closed behavior | A consumer that treats a closed workflow as flat economically has to read an explicit incomplete slot, not infer finality from the row's existence. |
| Advantage | Preserves the five outcomes in the semantics note. Makes incomplete knowledge durable. Hardest of the three to misuse as `Closed` equals done. |
| Disadvantage | More fields to explain. Still not implementable for any broker-facing family until external evidence exists. A careless reader can still ignore the incomplete slots unless tests forbid that reading (B10-T08, `TESTS_NOT_EXECUTED`). |

## 4. Shared constraints

These apply to A, B, and C. They are baseline constraints, not a hidden approval of any alternative.

- No alternative may treat timeout as non-send or as a closure reason (DET-01 §7, ARCH-DEC-03).
- No alternative may treat cancel acknowledgement as economic finality or as risk release (DET-02 §5, RISK-INV-04).
- No alternative may drop a late fill or overwrite the original closure reason (DET-01 §5, P0-B03-INV-01, P0-B03-INV-03).
- No alternative may reopen the workflow because new evidence arrived (OMS-INV-03).
- No alternative is a Rust enum, a `CHECK` constraint, or a wire string in this task.
- CR-B01 precedence over conflicting or less restrictive DET-07 text stays in force.
- Choosing an alternative later still leaves XC-03-MATRIX OPEN until a versioned edge list is separately accepted. A vocabulary is not a matrix.

## 5. Recommendation for review

**Non-binding recommendation:** Alternative C.

Rationale, offered for the user to accept or reject: DET01-Q01 and DET01-Q08 already separate closure reason, workflow closure, and later economic facts. CR-B10 §2 says `Closed` is not irrevocable economic finality and that `OrderClosure` is immutable. Alternative A is compatible with that split but hides incomplete broker and economic knowledge inside an evidence join. Alternative B encodes stories that this repository cannot evidence (XC-08-IBKR, XC-10-ACTION) and invites the single-flag mistakes in section 2. Alternative C keeps the incomplete slots on the closure record so a later reader cannot treat the row as proof of finality without also reading an explicit gap.

This recommendation does not approve Alternative C. It does not select tokens. It does not authorize a type, a table, or a transition. Status of OMSQ-02: `USER_DECISION_PENDING`.

External evidence still required before any alternative could be implemented, and not supplied here:

- A user decision on OMSQ-01 (how corrections sit beside an immutable closure).
- A user decision on OMSQ-09 (when a broker observation may fill the broker slot).
- Resolution of XC-02-PRED before the economic slot can ever be marked sufficient. This package does not resolve it.
- Broker uniqueness evidence for XC-01 and IBKR completeness evidence for XC-08-IBKR. `broker_uniqueness_evidence` stays `EVIDENCE_PENDING`.
