# TRD-ARCH-03B.4 — evidence capture and provenance

**Date:** 2026-11-15  
**Role:** proposed future capture labels and chain of custody. `document_role` = `NON_RUNTIME_NON_WIRE_VERIFICATION_PLAN`.  
**Execution:** `NOT_EXECUTED`.  
**External evidence:** `EXTERNAL_EVIDENCE_PENDING`.  
**Authorization:** `NOT_AUTHORIZED`.  
**Schema status:** not a canonical schema, not a wire schema, not DDL, not a registry entry, and not an index definition.

No capture file is stored by this milestone. The labels below are names a later package might use if a separate decision adopts them. Adopting the names would still not approve uniqueness, a completeness token, a posting, or a dispatch.

## Proposed future capture labels

These labels describe a future envelope around raw bytes. They are `PROPOSED_FUTURE_CAPTURE_LABEL` values. They are not fields of a published schema.

| Label | Rule |
| --- | --- |
| `evidence_id` | Local identifier for one stored observation. Absent until a future package mints one. Not a broker id. |
| `raw_bytes` | Bytes as received, before normalization. Absent in this repository. |
| `payload_sha256` | Hash of `raw_bytes`. Not a substitute for the bytes. |
| `synthetic_label` | `SYNTHETIC_NOT_EXTERNAL` on every E1 fixture. Forbidden on a claim of external behavior. |
| `product_name` | Copied from the component. `UNRECORDED`. |
| `product_version` | Copied from the component. `UNRECORDED`. |
| `api_client_name` | Copied from the component. `UNRECORDED`. |
| `api_client_version` | Copied from the component. `UNRECORDED`. |
| `operating_system` | The machine that ran the stage. `UNRECORDED`. |
| `account_mode` | The mode named in a future authorization. `UNRECORDED`. Paper is not implied. |
| `account_label` | Present only if the payload or the authorized request supplied it. Absence stays absence. |
| `client_label` | Present only if supplied. Absence stays absence. |
| `session_label` | New on reconnect (ARCH-DEC-43). Not `WriterEpoch`. |
| `writer_epoch` | Local fence fact. Not broker acceptance. |
| `adapter_session_epoch` | Connection generation. Not `WriterEpoch`. |
| `broker_acceptance` | A raw observation labeled as acceptance evidence only. Not economic finality. |
| `isolation_evidence` | External isolation artifacts. A lease deadline is not this label. |
| `dataset_label` | Open orders, executions, positions, or cash, and only when the request named that dataset. |
| `request_window` | The window copied from the request. Not inferred from the rows that arrived. |
| `correlation_id` | Ties local attempt facts to broker bytes. Not `HttpRequestId`. |
| `monotonic_received_at` | Local monotonic timestamp. Not event order by itself. |
| `wall_clock_received_at` | Wall clock, recorded beside the monotonic timestamp. |
| `broker_timestamp` | Stored only when the payload contains one. Not invented. |
| `connection_lifecycle` | Observed connect, drop, or restart. Not a retry instruction. |
| `local_capture_gap` | Short page, missing end, or drop, and only when locally observed. Not an IBKR sequence number. |
| `attempt_phase` | CR-B01 window label. Not permission to send. |
| `candidate_set_label` | Optional future `A(e)` label. Size one is `SINGLE_CANDIDATE_NOT_AUTHORIZATION`. |
| `inference_note` | Comparison text marked as inference. Not a winner. |
| `escalation_marker` | Non-authorizing marker. Not a role. |
| `chain_item` | Custody step below. |

Forbidden labels, not present as ratified fields:

- a completeness token, a `COMPLETE` marker, or an external token placeholder (ARCH-DEC-41)
- an invented IBKR sequence counter (ARCH-DEC-42)
- a UNIQUE alias key
- a canonical identity tuple
- an economic-posting flag
- a release flag
- a dispatch-authorized flag

## Chain of custody

A future package that stores bytes would keep an append-only custody list. This milestone starts no list.

| Step | Future obligation | Now |
| --- | --- | --- |
| 1. Obtain | The official copy or the raw capture is taken under a stage whose checkpoint was moved to go by a separate authorization. | STOP. Nothing obtained. |
| 2. Hash | `payload_sha256` is computed on the bytes before redaction of a separate log. Raw evidence bytes that are themselves secret-bearing are not committed into this repository. | No bytes. |
| 3. Redact logs | Logs that accompany the bytes are secret-free. Account numbers and credentials are removed from logs. The raw evidence file, if it must retain an account label, stays outside the git history until a retention decision (`IBKREVQ-11`). | No logs. |
| 4. Label | Version tuple, scope, session, and clocks are attached. Missing labels stay missing. | `UNRECORDED`. |
| 5. Append | A later observation is appended. Identical replay does not create an economic effect. A contradiction keeps both originals. | No store. |
| 6. Gap | A locally observed gap is marked. No rows are interpolated. | No gap marker stored. |
| 7. Review | E4 adjudication is a separate dated decision. The custody list does not ratify itself. | `NOT_RATIFIED`. |
| 8. Halt | Unexpected content stops the stage. The marker grants no action. | Not in a stage. |

Deletion of contradictory bytes is not a custody step. ARCH-DEC-38 keeps both originals.

## Separation rules

Local capture completeness and broker completeness stay separate statements. An empty `raw_bytes` list is an empty capture. It is not broker absence.

`writer_epoch`, `adapter_session_epoch`, `broker_acceptance`, and `isolation_evidence` are four labels. Filling one does not fill another (IBKR-CLM-25).

A `correlation_id` ties records for review. It is not a match and not uniqueness.

## Retention

This plan retains nothing. `IBKREVQ-11` asks how a future package should retain bytes and is `USER_DECISION_PENDING`. Until that question is answered, no raw IBKR payload is added to the repository.

## Non-claims

This note does not verify `orderId`, `permId`, `execId`, session lifetime, response completeness, or timeout. It does not publish a schema. It does not change `contracts/registry/registry.json`. **NO IMPLEMENTATION.** **NO VERIFICATION.**
