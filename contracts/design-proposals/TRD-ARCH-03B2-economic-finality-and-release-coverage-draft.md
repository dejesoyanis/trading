# TRD-ARCH-03B.2 — Economic-finality evidence and release coverage draft

**Date:** 2026-11-15  
**Role:** design proposal. `document_role` = `NON_RUNTIME_NON_WIRE_PROPOSAL`.  
**Ratification:** `NOT_RATIFIED`.  
**Live trading:** NOT AUTHORIZED.  
**Canonical schemas:** not published. `part_b` = `NO-GO`. `broker_uniqueness_evidence` = `EVIDENCE_PENDING`.

## CURRENT BINDING POLICY: NO EXECUTABLE ECONOMIC-FINALITY PREDICATE; RISK RELEASE FORBIDDEN

ARCH-DEC-14 (OMSQ-06 option A, 2026-11-15) is `BINDING_EXISTING`. Economic finality stays undefined. No reservation release is enabled. The evidence checklist below is a proposal. It is not an adopted predicate, not evidence that the listed facts are sufficient, and not permission to post.

ARCH-DEC-13 remains binding beside it: every proposed economic effect is quarantined pending authorized manual review, including a single consistent identity match. Manual review is not authorization to post.

XC-02-PRED stays OPEN. Source: [TRD-ARCH-SYNC-01-report.md](../../docs/architecture/TRD-ARCH-SYNC-01-report.md) section "2. Risk reservation finality". DET-02 §4 item 6 and §10, CR-B03 §3.2 and §3.3, and CR-B06 P1-B06-D03 do not define an executable predicate.

## 1. Evidence categories

A row can carry a principle mark and still have broker proof pending. The principle does not prove the external fact.

| Category | Mark | Source | What this package does not claim |
| --- | --- | --- | --- |
| Scoped broker and execution identity | `REQUIRED_BY_EXISTING_PRINCIPLE` for quarantine of ambiguity. Uniqueness proof is `EVIDENCE_PENDING`. | DET-03-D02; CR-B04 §4.1–§4.3 and P0-B04-INV-01; ARCH-DEC-13; ARCH-DEC-17. XC-01 OPEN. | No IBKR uniqueness. No production unique constraint on `orderId`, `permId`, or `execId`. `broker_uniqueness_evidence` stays `EVIDENCE_PENDING`. |
| Snapshot dataset, window, connection, and provenance | `REQUIRED_BY_EXISTING_PRINCIPLE` that an incomplete or empty snapshot is not absence. Completeness proof is `EVIDENCE_PENDING`. | DET-08-D02; RC-INV-04; CR-B07 P1-B07-D01 through D04; GATE-P1-07 `EXTERNAL_VERIFICATION_REQUIRED`. XC-08-RULE principle. XC-08-IBKR OPEN. | No adapter reports `COMPLETE`. No IBKR history-retention proof. |
| Raw events | `REQUIRED_BY_EXISTING_PRINCIPLE` | CR-B04 §4.3 step 1; DET-01 §5; ARCH-DEC-12; ARCH-DEC-17. Raw payload may be kept and still has no terminal effect. | No parser and no terminal mapping. |
| Append-only fill, bust, and correction chain | `REQUIRED_BY_EXISTING_PRINCIPLE` for retention and for "original evidence is not overwritten". | DET-01 §5; CR-B03 §3.1–§3.2; P0-B03-INV-01 through INV-03; ARCH-DEC-09; ARCH-DEC-13. | The chain is not a posting. A bust delta is not booked. |
| Versioned valuation and FX | `REQUIRED_BY_EXISTING_PRINCIPLE` for fail-closed on missing or stale inputs. Any numeric policy is `CANDIDATE_ONLY`. | DET-02 §6; DET-09-D03, DET-09-D04, MD-INV-05; CR-B06 P1-B06-D01 and D04; UD-03. XC-07-FX principle. XC-07-ROUND OPEN. | No rounding mode, no staleness threshold, no FX rate, no IBKR margin number. |
| Reservation-ledger reconciliation | `CANDIDATE_ONLY` as a ledger shape. The refusal to release an unclear reservation is `REQUIRED_BY_EXISTING_PRINCIPLE`. | DET-08-D01 and RC-INV-03; CR-B03 §3.2; ARCH-DEC-14. | No ledger, no reconciliation run, and no release. |
| Authorization and incident state | `REQUIRED_BY_EXISTING_PRINCIPLE` for halt on new risk-increasing actions and for deny when a required second person is absent. The incident type is not adopted. | DET-02 §7; DET-10-D06; CR-B05 P1-B05-D01; XC-09-SOLO. XC-09-RESET OPEN. `IncidentState` is absent. | No role, no reset, no break-glass implementation, no simulated second approver. |
| Audit record and CAS version | `REQUIRED_BY_EXISTING_PRINCIPLE` | ARCH-DEC-19; DET-01 Q04 and OMS-INV-01; OPS-INV-01; CR-B02 P0-B02-INV-01. The name `audit_events` is `REFERENCE_PROPOSAL` and is not adopted. XC-06 OPEN. | No table, no DDL, no hash chain, no isolation level. |

`CANDIDATE_ONLY` means a future reviewer might ask for the fact. It does not mean the fact is sufficient, and it does not mean the fact has been observed.

## 2. Candidate coverage template

This template is not executable logic. For every row:

- **Record evidence** is `RETAIN`.
- **Book economics** is `PROHIBITED`.
- **Release budget** is `PROHIBITED`.
- **Approval** is `NOT_RATIFIED`.
- **Test oracle** is `NOT_EXECUTED`.

Those four marks are repeated in the table so a row cannot be skimmed as an approval. Candidate reservation effect is a description of what a later decision might study. It is not an effect this package performs.

| Id | Scenario | Evidence received | Evidence missing | Identity | Completeness | Ledger ambiguity | Valuation | Budget scope | Candidate reservation effect | Prohibited effect | Incident / quarantine | Recovery / replay | Dependencies |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `initial_reserve` | Initial reserve | Command, actor, policy version, input refs | Broker acceptance, fill | Internal ids only | Not a broker question yet | No ledger posting | Must be present and fresh or the command fails closed | Every affected scope, formula unapproved | Hold obligation if a future writer exists | Book exposure, release, send | Deny and audit the refusal when inputs fail | Identical replay returns the refusal. No second hold | XC-07-ROUND, XC-05-LOCK, RISKQ-01, RISKQ-03 |
| `rejected_pre_dispatch` | Rejected pre-dispatch command | The rejection and its reason | Any broker evidence, because send did not start | Internal | Not applicable | None to book | Whatever was missing stays missing | Unchanged | No partial consumption remains | A leftover reservation, an outbox send | Audit the deny. No incident required unless the halt gate itself failed | Replay returns the stored deny | DET-02 §4 item 5, ARCH-DEC-19 |
| `may_have_sent` | `MayHaveSent` | Attempt marked before a possible send. Outcome unknown | Proof of non-send, broker id, fills | Unresolved | Unknown | Obligation remains | Not a reason to release | Stays reserved | Keep the obligation. No second submission | Close the workflow, retry, release, takeover | Quarantine / reconciliation | Restart reconstructs the attempt and does not resend | ARCH-DEC-11, XC-04, XC-08-IBKR. Label is a draft shell, not an edge |
| `ack_without_fills` | ACK without fills | Broker acknowledgement, raw payload | Fills, cancel, completeness | `EVIDENCE_PENDING` | Not shown | No economic posting | Unchanged | Obligation stays | ACK is evidence only | Treat ACK as economic finality or as release | Keep raw payload. No terminal effect | Replay of the same payload is one stored result | ARCH-DEC-17, EX-INV-05, XC-01 |
| `partial_fill` | Partial fill | One execution observation and raw payload | Residual quantity at the broker, completeness | `EVIDENCE_PENDING` | Open | Proposed split is quarantined | Pinned if present, else projection stays incomplete | Full obligation remains | Describe filled versus residual for later study | Convert filled quantity to exposure, release the residual | Quarantine the proposed conversion | Duplicate of the same evidence has no second effect | ARCH-DEC-13, XC-02-PRED, XC-01, RISKQ-04 |
| `complete_fill` | Complete fill | Observations whose quantities sum to the order in some reading | Proof that no further bust or correction will arrive, and that the sum is the broker's quantity | `EVIDENCE_PENDING` | Open | "Filled" is not finality | Pinned if present | Obligation stays until a predicate exists | None now | Book the fill, release the reservation, equate the label with finality | Quarantine the proposed booking | A later bust must still be appendable | ARCH-DEC-14, CR-B03 §3.2, RISKQ-08 |
| `cancel_requested` | Cancel requested | The cancel command and its authorization facts | Broker observation, proof the order is gone | Internal command id | Open | Residual obligation unchanged | Not a release input | Unchanged | Retain the request as its own fact | Release, or treat the request as confirmation | No dispatch is approved by this row | Replay of the same cancel command returns the stored result | ARCH-DEC-12, ARCH-DEC-20, XC-10-ACTION |
| `broker_cancel_observation` | Broker cancel observation | A cancel-shaped broker payload | Completeness, identity proof, assurance no fill follows | `EVIDENCE_PENDING` | Not `COMPLETE` | Residual stays | Not a release input | Unchanged | Retain the observation | Accept it as terminal, release the residual | No downstream terminal effect | Same payload once. Different payload conflicts | ARCH-DEC-17, XC-08-IBKR, XC-01 |
| `cancel_fill_race` | Cancel/fill race | Both the cancel fact and a fill fact | Which one is last at the broker, and whether more fills exist | May be ambiguous | Open | Both facts kept. Neither books | As available | Obligation stays at least as large as the unfilled uncertainty | Retain both | Drop the fill because a cancel exists, or release because a cancel exists | Quarantine any proposed net quantity | Order of arrival is not economic order | ARCH-DEC-12, XC-10-RACE, OMS-T07 `NOT_EXECUTED` |
| `disconnect_incomplete_snapshot` | Disconnect / incomplete snapshot | A partial stream or empty page, with scope and window metadata if any | End marker, full window, session continuity | Unknown | `PARTIAL` or `UNKNOWN` | Local obligation kept | May be stale | Unchanged | Do not infer absence | Release because the snapshot is empty, or mark the order definitively not sent | Reconciliation stays open | Reconnect with another session does not merge aliases | CR-B07, RC-INV-04, XC-08-IBKR |
| `duplicate_fill` | Duplicate fill | Two observations | Whether they are one execution or two | Conflict if payloads differ | Open | At most one eventual effect, and that effect is not booked now | Unchanged | No second hold and no second booking | Identical replay returns the stored result | Book twice, or drop the second payload without keeping it | Different payload is quarantine | Replay of the identical payload is idempotent | ARCH-DEC-16, CR-B04 §4.3 step 5, RISK-INV-05 |
| `late_fill` | Late fill | Fill after the historical closure record | A predicate that would book it, uniqueness evidence | `EVIDENCE_PENDING` even for one candidate | Open | Closure unchanged. Proposed effect quarantined | May be missing. Evidence still retained | Obligation or incident, not a silent overdraft | Keep the fill outside the closure record | Reopen the closure, book the match, discard the fill | Incident and halt on new risk-increasing actions if a future reading would breach a limit. No broker reduce | Replay does not rewrite the closure | ARCH-DEC-09, ARCH-DEC-13, P0-B03-INV-01 |
| `bust` | Bust | A correction that points at an earlier execution | The economic delta's sign under an adopted policy | Must reference the original evidence or it is unresolved | Open | Append-only. Original fill stays | Revalue only under a future policy | A bust can raise or lower obligation | Keep both records | Delete the original fill, or release because a bust "reduces risk" | Quarantine the proposed delta | Out-of-order correction stays deterministic only after a future rule. That rule is absent | CR-B03 §3.2, DET-01 §5 |
| `correction` | Correction | A new evidence row and a link to the original | The replacement quantity's broker meaning | `EVIDENCE_PENDING` | Open | Chain, not overwrite | Version pinned or projection incomplete | No silent negative reservation | Keep the chain | Rewrite the original, book the difference | Quarantine | Same chain replayed once | DET-02 §5, CR-B03 §3.4 |
| `insufficient_budget` | Insufficient budget | The command and the budgets it would touch | An adopted metric formula | Internal | Not a broker question | No partial consumption | If valuation is also missing, deny for that reason too | Hard deny on every affected dimension | Refuse the new risk-increasing command | Allow a negative remainder, net the shortfall away | Audit the deny. Halt is a separate decision if already-held obligation is over a limit | Retry after a budget change is a new command, not a silent replay | RISK-INV-02, DET-02 §4 item 5, RISKQ-07 |
| `stale_price_or_fx` | Stale price / FX | The stale, missing, or contradictory input and its version | A fresh snapshot, an FX rate, a comparability rule | Internal | Market-data completeness is its own question | No new reservation | Fail closed | No new risk-increasing hold | Deny `ALLOW` | Invent a price, use 1:1 FX, round into a limit | Block new risk. Keep already stored evidence | Replay with the same stale snapshot still denies | DET-02 §6, MD-INV-05, XC-07-ROUND, XC-07-FX |
| `cross_scope_contention` | Cross-scope shared budget contention | Two commands touching an overlapping scope | The lock protocol and the aggregation rule | Internal ids | Not broker completeness | No partial winner that consumed one scope and not another | Shared snapshot or both deny | All-or-nothing across the scopes the command named | One command may win only under a future tested protocol | Oversubscribe, or leave a partial hold after a failed sibling scope | Fail the local unit. No broker call | Local retry only, with a fresh evaluation | XC-05-LOCK, P0-B02-T02 `NOT_EXECUTED`, RISKQ-02 |
| `cas_conflict` | CAS conflict | Expected version and the stored version | Which command is authorized to proceed | Internal | Not broker completeness | The losing command writes nothing | Not reused from the stale evaluation | Unchanged by the loser | Store the conflict | Apply the stale evaluation, resend to the broker | Quarantine if the payloads disagree | Identical replay of the winner returns the stored result | ARCH-DEC-16, ARCH-DEC-18, OMS-INV-01 |
| `restart_reconciliation` | Restart / reconciliation | Local commands, attempts, ledger, raw evidence, and any partial broker snapshot | Proof the snapshot covers the scope | Often `EVIDENCE_PENDING` | Often `UNKNOWN` | Differences classified, not auto-resolved | May be stale | Unclear reservations stay | Reconstruct and block new risk-increasing submission | Blind retry, release because the broker list is empty, auto-reset | `SUBMISSION_UNKNOWN` and `RISK_MISMATCH` stay open | A second reconciliation run does not double an effect | DET-08, RC-INV-01, RC-INV-03, XC-08-IBKR, XC-09-RESET |
| `manual_adjudication` | Manual adjudication | Evidence ids, actor, expected version, reason, if a future record is ever written | A second person when one is required, and a posting authorization that does not exist | As cited, or still ambiguous | As cited, or still incomplete | The proposed effect stays quarantined | Cited version or the case stays blocked | Unchanged until a future superseding decision | Describe the case | Post because a person looked, simulate a second approver | Stay quarantined. Absence of a required second person is deny | The adjudication record would itself be replayable and would not create an economic effect by being repeated | ARCH-DEC-13, ARCH-DEC-16, XC-09-SOLO, RISKQ-13 |
| `emergency_halt` | Emergency halt | Halt scope, actor, version | Proof that broker orders were cancelled | Unchanged by the halt | Halt is not a snapshot | New risk-increasing actions stop | Not a reason to release existing holds | Existing obligations remain | Block new risk-increasing commands in the scope | Treat the halt as broker cancel, or as release, or as permission to send a reduce | Incident containment is a separate open type | Restart does not clear the halt because the network returned | DET-10-D06, OPS-INV-04, ARCH-DEC-20, XC-10-ACTION |

Approval status of every row: `NOT_RATIFIED`. Test oracle of every row: `NOT_EXECUTED`. Record evidence: `RETAIN`. Book economics: `PROHIBITED`. Release budget: `PROHIBITED`.

## 3. Shortcuts that are unsafe

Each shortcut below has an adversarial counterexample. None of these readings is available.

| Shortcut | Why it fails | Counterexample | Sources |
| --- | --- | --- | --- |
| `Cancelled` means release | A cancel-shaped label is an observation or a request, not economic finality. | A cancel observation is stored, the reservation is imagined released, and a later fill arrives for the same order. The budget then has no hold for a real execution. | DET-02 §5, RISK-INV-04, ARCH-DEC-12, ARCH-DEC-14, CR-B09 P1-B09-D02 |
| `Filled` means economic finality | A fill label does not end the correction chain. | An execution is marked filled, then a bust removes it or a correction changes the quantity. The booked exposure and the evidence disagree. | CR-B03 §3.2, DET-01 §5, RISK-T08 as an unexecuted expectation |
| `Closed` means no future fill | Workflow closure is historical and is not economic finality. | The closure record is immutable, and a fill arrives afterward. Discarding it loses evidence. Reopening the closure rewrites history. | ARCH-DEC-09, ARCH-DEC-10, P0-B03-INV-01, CR-B10 §2 |
| Timeout means not sent | Elapsed time is not proof of non-send. | `DISPATCH_STARTED` is durable, the process crashes, the broker accepts the order, and a timeout marks it unsent. A second submission then creates a second order. | ARCH-DEC-11, ARCH-DEC-03, DET-07-D03, CR-B01 §1.3 |
| An empty broker snapshot means no order | Emptiness without a proved window is lack of evidence. | The stream drops before the end marker, the page is empty, and a working order exists in a window the query did not cover. | CR-B07 P1-B07-D02, RC-INV-04, XC-08-RULE, GATE-P1-07 |
| The same nominal quantity is the same execution | Quantity is not identity. Scale-equal amounts are not broker keys. | Two executions share a quantity and differ in payload, or one `execId` is corrected into another. Booking both double-counts. Collapsing them drops a bust. | CR-B04 §4.2 and §4.3 step 5, UD-03, XC-01 |
| A risk reduction is safe without broker semantics | A negative sign is not proof of lower risk, and the action is not authorized. | An order intended to reduce a position increases gross exposure, crosses a sector limit, or is actually a new submission during a halt. | DET-02 §7, CR-B09 P1-B09-D04, RISK-T11 as an unexecuted expectation, ARCH-DEC-20 |

## 4. Tests proposed, and not executed

These names are proposals for a later suite. They have not been run. Documentation tests in this repository do not execute them and do not prove financial safety.

| Proposal | Kind | Oracle | Status |
| --- | --- | --- | --- |
| Reservation obligation never shrinks on timeout, cancel request, closure, ACK, or incomplete snapshot | Model-based | `PROHIBITED` release | `NOT_EXECUTED` |
| Identical evidence replay is idempotent and a conflicting payload quarantines | Property-based | No second effect | `NOT_EXECUTED` |
| Fill, bust, and correction chains are append-only and do not rewrite closure | Replay | Historical closure byte-stable | `NOT_EXECUTED` |
| Concurrent commands cannot leave a partial multi-scope hold | Concurrency, after a lock protocol exists | All-or-nothing local unit | `NOT_EXECUTED`. Deadlock tests were not run |
| Missing FX, stale price, and overflow deny, with no silent rounding | Property-based | Explicit error | `NOT_EXECUTED` |
| Crash before and after the local commit leaves no split between evidence, reservation, audit, and outbox | Fault injection | Atomic local unit | `NOT_EXECUTED` |
| Empty and partial snapshots do not prove absence | Adversarial fixtures, after ARCH-03B.3 evidence | Obligation unchanged | `NOT_EXECUTED` |
| A single identity match still does not post | Negative test of ARCH-DEC-13 | Quarantine | `NOT_EXECUTED` |

Source expectations RISK-T01 through RISK-T14, OMS-T01 through OMS-T12, P0-B02-T01 through T09, P0-B03-T01 through T10, and B07-T01 through T07 remain `NOT_EXECUTED`. They are not passed because they are named.

No final rounding rule, valuation policy, stress metric, transaction lock order, or broker-completeness proof is specified here. XC-07-ROUND, XC-05-LOCK, and XC-08-IBKR stay OPEN.
