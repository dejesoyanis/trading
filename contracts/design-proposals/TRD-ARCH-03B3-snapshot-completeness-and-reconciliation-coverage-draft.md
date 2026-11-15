# TRD-ARCH-03B.3 — Snapshot completeness and reconciliation coverage

**Date:** 2026-11-15  
**Role:** design proposal. `document_role` = `NON_RUNTIME_NON_WIRE_PROPOSAL`.  
**Ratification:** `NOT_RATIFIED`.  
**XC-08-IBKR:** OPEN. This file does not prove completeness.  
**XC-01:** OPEN.  
**Live trading:** NOT AUTHORIZED.  
**Canonical schemas:** not published.

This file is a non-executable coverage draft. It is not a reconciliation job, not an adapter, and not a completeness certificate. An empty response, a timeout, a cancel label, a filled label, a disconnect, and a lease expiry are not proof of non-execution and not proof of economic finality (CR-B01 §1.3, CR-B07 P1-B07-D02, RC-INV-04, ARCH-DEC-14, ARCH-DEC-17).

## 1. Coverage axes

CR-B07 P1-B07-D01 names a scope tuple `(broker, account, connection_epoch, dataset, query_window, request_id, received_at, completeness_evidence, adapter_version)` and datasets for open orders, executions, positions, and possibly cash. That tuple is `REFERENCE_PROPOSAL`. It is not evidenced for IBKR. One response is not a global snapshot.

| Axis | What a later record would have to name | What this repository does not have |
| --- | --- | --- |
| Account | Which account the bytes describe | Multi-account and advisor visibility. `EXTERNAL_EVIDENCE_PENDING`. |
| Client and session | Which client and which connection generation | Cross-client visibility and reconnect behavior. `EXTERNAL_EVIDENCE_PENDING`. |
| Dataset | Open orders, executions, corrections, positions, or cash, separately | A rule that one call returns all of them. Not assumed. |
| Order lifecycle | Which observations mention which local attempt | A broker terminal mapping. ARCH-DEC-17 accepts none. |
| Time interval | The window the query asked for, and the window the bytes actually cover | History retention. CR-B07 says late delivery and limited history can keep reconciliation open. |
| Reconnect boundary | The connection generation of this response versus earlier ones | Proof that ids survive the boundary. |
| Event ordering | Receive time versus an optional broker timestamp (DET-05-D04) | An IBKR sequence number. None is invented. DET-09-D05 is market-data ordering, not an order-stream guarantee. |
| Authoritative source | The raw response and its request id | A rule that the broker wins or the database wins. DET-08-D03 forbids both blanket rules. |

## 2. Evidence kinds

| Kind | What it can show | What it cannot show |
| --- | --- | --- |
| Incremental stream | Observations that arrived, with receive time | That nothing else happened, or that arrival order is causal order |
| Point-in-time snapshot | The rows returned for one named request | That the rows are all rows, all accounts, or all time |
| Reconciliation run | A local comparison record with the sources it actually read (DET-08-D02) | Closure, release, or a second send |
| Absence | That this response did not contain a row | That the order was not accepted and not filled |
| Contradiction | That two retained observations disagree | Which one is economically final |

DET-08-D03 classification names (`MISSING_LOCAL`, `MISSING_BROKER`, `IDENTITY_CONFLICT`, `EXECUTION_MISMATCH`, `POSITION_MISMATCH`, `RISK_MISMATCH`, `SUBMISSION_UNKNOWN`, `STALE_SNAPSHOT`) are `REFERENCE_PROPOSAL`. Using one of those words does not authorize an automatic close (DET-08 protocol step 8 is source text; ARCH-DEC-13 and ARCH-DEC-14 forbid the economic consequence).

## 3. Non-executable proof obligations

A later completeness claim, if one is ever ratified, would have to show all of the following at once. None of them is shown here. The model is `NEW_PROPOSAL` and `NOT_EXECUTED`.

1. The dataset, account, client, session, and time window are named on the evidence.
2. The completion condition for that query type is a cited external result, not a local guess.
3. Paging, disconnect, and empty-page cases were part of that cited result.
4. Identity of every row was either unresolved on purpose or tied to retained raw evidence.
5. The claim still does not post, release, resubmit, or accept a broker label as terminal.

P1-B07-D02 says an adapter may emit `COMPLETE` only after a documented, tested completion criterion for that IBKR query type. No such criterion is in this repository. GATE-P1-07 is `EXTERNAL_VERIFICATION_REQUIRED` in the source. This package does not satisfy it.

Negative evidence that an order does not exist requires both sufficient coverage and attempt evidence, and otherwise quarantine (P1-B07-D04). This package does not treat any row below as that negative evidence.

## 4. Scenario rows

Thirty rows. Each marker is mandatory. No row loosens a binding prohibition. `ORACLE_NOT_EXECUTED` means no broker, database, or fault-injection oracle was run. Source test ids in the situation text are names from the contracts, not results.

| Id | Situation | Evidence | Booking | Release | Resubmit | Broker terminal | Oracle |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `reconnect_session_boundary` | Reconnect with a new session context. Prior `orderId` values are not reinterpreted without that context (DET-03; CR-B04 T02; RC-T10). | `EVIDENCE_RETAINED` | `BOOKING_PROHIBITED` | `RELEASE_PROHIBITED` | `RESUBMIT_PROHIBITED` | `BROKER_TERMINAL_ACCEPTANCE_PROHIBITED` | `ORACLE_NOT_EXECUTED` |
| `client_or_account_change` | Same raw id observed under a different client or account. Cross-client visibility is unverified (CR-B04 §4.2, T07). | `EVIDENCE_RETAINED` | `BOOKING_PROHIBITED` | `RELEASE_PROHIBITED` | `RESUBMIT_PROHIBITED` | `BROKER_TERMINAL_ACCEPTANCE_PROHIBITED` | `ORACLE_NOT_EXECUTED` |
| `missing_executions` | A named execution window returns some rows and not others, or history is limited (P1-B07-D03, B07-T04). | `EVIDENCE_RETAINED` | `BOOKING_PROHIBITED` | `RELEASE_PROHIBITED` | `RESUBMIT_PROHIBITED` | `BROKER_TERMINAL_ACCEPTANCE_PROHIBITED` | `ORACLE_NOT_EXECUTED` |
| `empty_open_orders_snapshot` | Open-order response contains no rows. An empty response is not a negative proof (P1-B07-D02, B07-T01, RC-INV-04). | `EVIDENCE_RETAINED` | `BOOKING_PROHIBITED` | `RELEASE_PROHIBITED` | `RESUBMIT_PROHIBITED` | `BROKER_TERMINAL_ACCEPTANCE_PROHIBITED` | `ORACLE_NOT_EXECUTED` |
| `empty_executions_snapshot` | Execution response contains no rows for the requested window. Absence of rows is not absence of fills. | `EVIDENCE_RETAINED` | `BOOKING_PROHIBITED` | `RELEASE_PROHIBITED` | `RESUBMIT_PROHIBITED` | `BROKER_TERMINAL_ACCEPTANCE_PROHIBITED` | `ORACLE_NOT_EXECUTED` |
| `reused_broker_order_id` | A broker `orderId` value appears again in another session or after a gap (ID-T01). No UNIQUE constraint is inferred. | `EVIDENCE_RETAINED` | `BOOKING_PROHIBITED` | `RELEASE_PROHIBITED` | `RESUBMIT_PROHIBITED` | `BROKER_TERMINAL_ACCEPTANCE_PROHIBITED` | `ORACLE_NOT_EXECUTED` |
| `late_perm_id` | `permId` is absent on an earlier observation and present on a later one. The later value is appended, not written over the earlier evidence (DET-03-D05, ID-INV-04). | `EVIDENCE_RETAINED` | `BOOKING_PROHIBITED` | `RELEASE_PROHIBITED` | `RESUBMIT_PROHIBITED` | `BROKER_TERMINAL_ACCEPTANCE_PROHIBITED` | `ORACLE_NOT_EXECUTED` |
| `missing_perm_id` | `permId` is null, zero, or absent. Missing is not equal to another missing value (CR-B04 T10, UD-02). | `EVIDENCE_RETAINED` | `BOOKING_PROHIBITED` | `RELEASE_PROHIBITED` | `RESUBMIT_PROHIBITED` | `BROKER_TERMINAL_ACCEPTANCE_PROHIBITED` | `ORACLE_NOT_EXECUTED` |
| `unknown_send_dispatch_started` | Local phase reached `DISPATCH_STARTED` or `OUTCOME_UNKNOWN`. The attempt stays possibly sent (CR-B01 §1.3, SYS-INV-02, ARCH-DEC-11). | `EVIDENCE_RETAINED` | `BOOKING_PROHIBITED` | `RELEASE_PROHIBITED` | `RESUBMIT_PROHIBITED` | `BROKER_TERMINAL_ACCEPTANCE_PROHIBITED` | `ORACLE_NOT_EXECUTED` |
| `lost_ack` | A broker acknowledgement may have been issued and was not stored locally (CR-B01 §1.4 row "after broker acceptance, before ack persistence"; RC-T01). | `EVIDENCE_RETAINED` | `BOOKING_PROHIBITED` | `RELEASE_PROHIBITED` | `RESUBMIT_PROHIBITED` | `BROKER_TERMINAL_ACCEPTANCE_PROHIBITED` | `ORACLE_NOT_EXECUTED` |
| `crash_before_attempt_commit` | Crash before the attempt commit. CR-B01 §1.4 says no attempt was authorized. This row does not itself authorize a replay send. | `EVIDENCE_RETAINED` | `BOOKING_PROHIBITED` | `RELEASE_PROHIBITED` | `RESUBMIT_PROHIBITED` | `BROKER_TERMINAL_ACCEPTANCE_PROHIBITED` | `ORACLE_NOT_EXECUTED` |
| `crash_after_commit_before_socket` | Crash after `DISPATCH_STARTED` and before a socket write. CR-B01 §1.4 still treats the attempt as possibly sent. | `EVIDENCE_RETAINED` | `BOOKING_PROHIBITED` | `RELEASE_PROHIBITED` | `RESUBMIT_PROHIBITED` | `BROKER_TERMINAL_ACCEPTANCE_PROHIBITED` | `ORACLE_NOT_EXECUTED` |
| `prior_writer_still_active` | A previous writer may still send after an epoch change (CR-B01 §1.4 last row, P0-B01-INV-02, T04). The successor does not send. | `EVIDENCE_RETAINED` | `BOOKING_PROHIBITED` | `RELEASE_PROHIBITED` | `RESUBMIT_PROHIBITED` | `BROKER_TERMINAL_ACCEPTANCE_PROHIBITED` | `ORACLE_NOT_EXECUTED` |
| `lease_expiry_alone` | The database lease deadline passed. Expiry is not isolation and not broker rejection (ARCH-DEC-03, CR-B01 §1.1). | `EVIDENCE_RETAINED` | `BOOKING_PROHIBITED` | `RELEASE_PROHIBITED` | `RESUBMIT_PROHIBITED` | `BROKER_TERMINAL_ACCEPTANCE_PROHIBITED` | `ORACLE_NOT_EXECUTED` |
| `split_brain` | Two writers can both reach a broker socket. CR-B01 §1.1 and DET-07 fencing limit: local fencing cannot retract an in-flight write. No second send is authorized. | `EVIDENCE_RETAINED` | `BOOKING_PROHIBITED` | `RELEASE_PROHIBITED` | `RESUBMIT_PROHIBITED` | `BROKER_TERMINAL_ACCEPTANCE_PROHIBITED` | `ORACLE_NOT_EXECUTED` |
| `epoch_change_open_attempt` | Epoch increases while a `DISPATCH_STARTED` attempt is unresolved (CR-B01 §1.2 item 6). Successor activation stays blocked. | `EVIDENCE_RETAINED` | `BOOKING_PROHIBITED` | `RELEASE_PROHIBITED` | `RESUBMIT_PROHIBITED` | `BROKER_TERMINAL_ACCEPTANCE_PROHIBITED` | `ORACLE_NOT_EXECUTED` |
| `partial_fill` | One or more execution observations cover part of an order quantity (CR-B04 T05, ARCH-DEC-24). The hold is not reduced. | `EVIDENCE_RETAINED` | `BOOKING_PROHIBITED` | `RELEASE_PROHIBITED` | `RESUBMIT_PROHIBITED` | `BROKER_TERMINAL_ACCEPTANCE_PROHIBITED` | `ORACLE_NOT_EXECUTED` |
| `cancel_request_race` | A cancel request is recorded while a fill may already exist. The request is not confirmation (CR-B09 P1-B09-D02, ARCH-DEC-12, ARCH-DEC-20). No cancel is dispatched by this row. | `EVIDENCE_RETAINED` | `BOOKING_PROHIBITED` | `RELEASE_PROHIBITED` | `RESUBMIT_PROHIBITED` | `BROKER_TERMINAL_ACCEPTANCE_PROHIBITED` | `ORACLE_NOT_EXECUTED` |
| `cancel_observation_label` | A broker payload uses a cancel or cancelled label. The label is not terminal and not a release (ARCH-DEC-17, ARCH-DEC-12). | `EVIDENCE_RETAINED` | `BOOKING_PROHIBITED` | `RELEASE_PROHIBITED` | `RESUBMIT_PROHIBITED` | `BROKER_TERMINAL_ACCEPTANCE_PROHIBITED` | `ORACLE_NOT_EXECUTED` |
| `late_fill` | An execution observation arrives after a historical closure record (ARCH-DEC-09, CR-B03 §3.1, RC-T03). The closure record is not rewritten. The fill is not booked. | `EVIDENCE_RETAINED` | `BOOKING_PROHIBITED` | `RELEASE_PROHIBITED` | `RESUBMIT_PROHIBITED` | `BROKER_TERMINAL_ACCEPTANCE_PROHIBITED` | `ORACLE_NOT_EXECUTED` |
| `correction_bust_chain` | A later observation corrects or busts an earlier execution (CR-B04 T06, RC-T09). Both stay. No compensating ledger line (ARCH-DEC-26). | `EVIDENCE_RETAINED` | `BOOKING_PROHIBITED` | `RELEASE_PROHIBITED` | `RESUBMIT_PROHIBITED` | `BROKER_TERMINAL_ACCEPTANCE_PROHIBITED` | `ORACLE_NOT_EXECUTED` |
| `duplicate_execution_same_payload` | The same execution payload is observed again (ID-T04, ARCH-DEC-16). The second observation creates no economic effect and no second send. | `EVIDENCE_RETAINED` | `BOOKING_PROHIBITED` | `RELEASE_PROHIBITED` | `RESUBMIT_PROHIBITED` | `BROKER_TERMINAL_ACCEPTANCE_PROHIBITED` | `ORACLE_NOT_EXECUTED` |
| `duplicate_execution_conflicting_payload` | A repeated execution key carries a different payload (CR-B04 §4.3 step 5). The result is a contradiction, not an overwrite. | `EVIDENCE_RETAINED` | `BOOKING_PROHIBITED` | `RELEASE_PROHIBITED` | `RESUBMIT_PROHIBITED` | `BROKER_TERMINAL_ACCEPTANCE_PROHIBITED` | `ORACLE_NOT_EXECUTED` |
| `stale_snapshot` | Snapshot watermarks or source versions are older than a later local fact (DET-08-D03 `STALE_SNAPSHOT`, DET-09-D05 for market data only). The stale bytes stay. They do not replace later evidence and do not release risk. | `EVIDENCE_RETAINED` | `BOOKING_PROHIBITED` | `RELEASE_PROHIBITED` | `RESUBMIT_PROHIBITED` | `BROKER_TERMINAL_ACCEPTANCE_PROHIBITED` | `ORACLE_NOT_EXECUTED` |
| `incomplete_time_window` | The query window does not cover the attempt time, or coverage of the window is not evidenced (P1-B07-D03, DET-08-D02). | `EVIDENCE_RETAINED` | `BOOKING_PROHIBITED` | `RELEASE_PROHIBITED` | `RESUBMIT_PROHIBITED` | `BROKER_TERMINAL_ACCEPTANCE_PROHIBITED` | `ORACLE_NOT_EXECUTED` |
| `out_of_order_updates` | Execution or status observations arrive out of receive order or broker-timestamp order (DET-05-D04, B07-T05, CR-B04 T10). No winner is chosen. | `EVIDENCE_RETAINED` | `BOOKING_PROHIBITED` | `RELEASE_PROHIBITED` | `RESUBMIT_PROHIBITED` | `BROKER_TERMINAL_ACCEPTANCE_PROHIBITED` | `ORACLE_NOT_EXECUTED` |
| `contradictory_observations` | Two retained observations disagree on status, quantity, or identity (BA-T07, DET-08-D03). Neither becomes authoritative terminal (ARCH-DEC-17). | `EVIDENCE_RETAINED` | `BOOKING_PROHIBITED` | `RELEASE_PROHIBITED` | `RESUBMIT_PROHIBITED` | `BROKER_TERMINAL_ACCEPTANCE_PROHIBITED` | `ORACLE_NOT_EXECUTED` |
| `timeout_or_disconnect` | The call times out or the stream drops, including a missing end marker (B07-T02, B07-T03, BA-T01). Timeout is not `DEFINITIVELY_NOT_SENT`. | `EVIDENCE_RETAINED` | `BOOKING_PROHIBITED` | `RELEASE_PROHIBITED` | `RESUBMIT_PROHIBITED` | `BROKER_TERMINAL_ACCEPTANCE_PROHIBITED` | `ORACLE_NOT_EXECUTED` |
| `filled_or_cancelled_label` | A payload says filled or cancelled and the execution set is incomplete or unverified. The label is evidence text only (ARCH-DEC-17, ARCH-DEC-24, ARCH-DEC-25). | `EVIDENCE_RETAINED` | `BOOKING_PROHIBITED` | `RELEASE_PROHIBITED` | `RESUBMIT_PROHIBITED` | `BROKER_TERMINAL_ACCEPTANCE_PROHIBITED` | `ORACLE_NOT_EXECUTED` |
| `unknown_external_order` | A broker row matches no local order (ID-T05, B07-T07). It is not attached to the nearest order. | `EVIDENCE_RETAINED` | `BOOKING_PROHIBITED` | `RELEASE_PROHIBITED` | `RESUBMIT_PROHIBITED` | `BROKER_TERMINAL_ACCEPTANCE_PROHIBITED` | `ORACLE_NOT_EXECUTED` |

Row count: 30. Required themes present: reconnect, missing executions, empty snapshot, reused id, late permId, unknown send, prior writer, split brain, partial fill, cancel race, late fill, correction/bust, duplicate, stale snapshot, incomplete time window, out-of-order updates.

Partial fills, cancel requests, cancel observations, late fills, corrections, and busts change evidence tracking only. They do not post a ledger line, convert a reservation, or release a hold (ARCH-DEC-12, ARCH-DEC-24, ARCH-DEC-25, ARCH-DEC-26).

## 5. Future verification plan

Every line below is `NOT_EXECUTED` and `EXTERNAL_EVIDENCE_PENDING`. No official document was retrieved. No URL is cited. No paper or live broker was called.

| Future case | What would have to be recorded | Status |
| --- | --- | --- |
| Identity fields on a new order | Raw `orderId`, `permId`, `clientId`, `account`, `orderRef` if present, timestamps, product and API version. Source name P0-B04-T01. | `NOT_EXECUTED` / `EXTERNAL_EVIDENCE_PENDING` |
| Reconnect and gateway restart | Which ids stayed, which changed, which disappeared. P0-B04-T02 and T03. | `NOT_EXECUTED` / `EXTERNAL_EVIDENCE_PENDING` |
| Modify, cancel, partial fill, correction, bust | Identity stability and whether `execId` changes. P0-B04-T04, T05, T06. | `NOT_EXECUTED` / `EXTERNAL_EVIDENCE_PENDING` |
| Second client or session | Visibility and collisions. P0-B04-T07. | `NOT_EXECUTED` / `EXTERNAL_EVIDENCE_PENDING` |
| Null, zero, missing, and out-of-order callbacks | No false match. P0-B04-T10. | `NOT_EXECUTED` / `EXTERNAL_EVIDENCE_PENDING` |
| Open-order, execution, and position queries | Completion signal, paging, empty page, disconnect, history limit, account scope. B07-T01 through T07. GATE-P1-07. | `NOT_EXECUTED` / `EXTERNAL_EVIDENCE_PENDING` |
| Negative cases | Empty snapshot does not prove non-send. Timeout does not prove non-send. A filled or cancelled label does not prove finality. A lease expiry does not prove isolation. | `NOT_EXECUTED` / `EXTERNAL_EVIDENCE_PENDING` |

The environment for any future run would have to be named in the evidence itself: broker product, API library version, gateway or TWS version, account type, paper or other non-production label, clock source, and the time the bytes were captured. This package does not choose that environment and does not run it. CR-B04 §4.5 says productive order sends are out of scope for that future test report. This milestone makes no broker request at all.

## 6. Non-authorizations

No snapshot is `COMPLETE`. No reconciliation run is approved. No economic posting, reservation release, resubmit, cancel, modify, replace, or broker terminal acceptance follows from any row. `XC-08-IBKR` stays OPEN. `XC-02-PRED` stays OPEN. Live trading is NOT AUTHORIZED.
