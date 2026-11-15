# TRD-ARCH-03B.3 — Broker identity semantics and invariants

**Date:** 2026-11-15  
**Role:** design proposal. `document_role` = `NON_RUNTIME_NON_WIRE_PROPOSAL`.  
**Ratification:** `NOT_RATIFIED`.  
**XC-01:** OPEN. This file does not resolve it.  
**XC-08-IBKR:** OPEN.  
**XC-04:** design precedence remains `RESOLVED_BY_ARCH_DEC_03`. The operational remainder stays OPEN.  
**Live trading:** NOT AUTHORIZED.  
**Canonical schemas:** not published. `part_b` = `NO-GO`. `broker_uniqueness_evidence` = `EVIDENCE_PENDING`.  
**Current binding policy:** CR-B01 has design precedence over conflicting or less restrictive DET-07 dispatch text (ARCH-DEC-03). Every proposed economic effect stays quarantined, including a single consistent identity match (ARCH-DEC-13, ARCH-DEC-26). No economic-finality predicate and no reservation release (ARCH-DEC-14, ARCH-DEC-28, ARCH-DEC-29). No broker observation is authoritative terminal (ARCH-DEC-17). No automatic second send and no takeover while a prior writer may still send or a submission result is unknown (ARCH-DEC-03, ARCH-DEC-11). Modify and implicit cancel-replace stay blocked (ARCH-DEC-20). No role is invented (ARCH-DEC-16, ARCH-DEC-33).

This file separates identity namespaces so a later user decision can be reviewed. It is not a resolver, not a uniqueness constraint, not a schema, not a migration, and not permission to match, post, release, resubmit, cancel, or trade.

Reference names in DET and CR source files stay reference text. A label in `contracts/registry/registry.json` is a draft shell. `IDENTITY_RESOLVER_IMPLEMENTED` stays false. The Rust types in `crates/trading-domain/src/identity.rs` are descriptive shells. This package does not edit them and does not treat structural equality as a merge.

## Claim classes

| Class | Meaning in this file |
| --- | --- |
| `BINDING_EXISTING` | A recorded user decision in ARCH-DEC-03, ARCH-DEC-09 through ARCH-DEC-20, or ARCH-DEC-21 through ARCH-DEC-34, or a fail-closed principle those records keep in force. |
| `RECONSTRUCTED_BASELINE_WITH_EXCEPTIONS` | A DET-01 or DET-02 principle inside the ARCH-DEC-01 baseline. The file is `RECONSTRUCTED_DRAFT` and `SOURCE_REVIEW_PENDING`. Q-cell verification stays OPEN. |
| `REFERENCE_PROPOSAL` | A field list, table name, resolver outcome word, or test id in DET-03 through DET-10 or CR-B01 through CR-B10. ARCH-DEC-04 gives `DESIGN_RESOLVED` no priority over other status words. Presence is not ratification and not broker proof. |
| `EXTERNAL_EVIDENCE_PENDING` | IBKR uniqueness, stability, null semantics, session visibility, or snapshot completeness. This repository does not contain that proof. No URL is invented. |
| `NEW_PROPOSAL` | A candidate reading introduced here. It needs a later user decision. |
| `USER_DECISION_PENDING` | A BROKERQ question. A recommendation is not approval. |

## 1. Identity namespaces

These namespaces are independent. Sharing a text value across two of them is not equality. No row is an approved column, index, or wire field.

| Namespace | What it is | What it is not | Class |
| --- | --- | --- | --- |
| Platform `OrderId` | Internal UUID minted by the platform. DET-03-D01 and CR-B04 §4.1 call the internal `order_id` authoritative for the platform record. Glossary: never broker-derived. UUID bits are not event order (UD-09). | A broker `orderId`, a `permId`, an execution id, or a reservation id. | The distinction is `REFERENCE_PROPOSAL` in DET-03/CR-B04 and already reflected by the shell. Uniqueness of broker fields is `EXTERNAL_EVIDENCE_PENDING`. |
| `CommandId` and command idempotency key | One command identity plus a key scoped to that command, checked with a payload hash. DET-03-D03. CR-B02 §2.4 input. Same key and same payload return a stored result and create no new economic effect (ARCH-DEC-16). A different payload is a conflict. | Permission to send again. Identical replay is not a new authorization (ARCH-DEC-16). It is not a broker idempotency guarantee (DET-07-D04). | `BINDING_EXISTING` for the local replay rule. Broker idempotency is `EXTERNAL_EVIDENCE_PENDING`. |
| `SubmissionAttemptId` | One local attempt. DET-03-D03 and CR-B01 §1.2 require an attempt record before a possible send. The shell is a UUID newtype. Persistence of attempts is not implemented. | Proof that a socket write happened, or proof that it did not. | The "record before a possible send" rule is `BINDING_EXISTING` as design (ARCH-DEC-03, CR-B01). The table is `REFERENCE_PROPOSAL`. |
| `RiskReservationId` | One reservation identity for an all-or-nothing multi-scope hold (ARCH-DEC-22). Glossary: the order link is not stored on the shell. | An `OrderId`, a fill, a release, or a writer. ARCH-DEC-22 authorizes no writer. ARCH-DEC-24 does not convert a hold on a fill. | `BINDING_EXISTING` for the identity shape and the hold. |
| Broker account context | Broker, account, and environment. Glossary `BrokerAccountKey` uses all three in equality. CR-B04 §4.2 lists `account` as a scope hypothesis, including multi-account cases that are not verified. | A global broker identity. An account code alone is not an order. | Shell distinction is descriptive. Account visibility across advisor or multi-account layouts is `EXTERNAL_EVIDENCE_PENDING`. |
| Client and session context | Optional `BrokerClientId`, optional internal `BrokerSessionId`, optional `AdapterSessionEpoch`. UD-02: client id is not session id. Adapter session epoch is not `WriterEpoch`. Missing fields stay absent. `UnverifiedSessionScope` rejects `verified`. | A verified IBKR session, a database fence, or a reason to equate raw ids across sessions. | `REFERENCE_PROPOSAL` plus the fail-closed shell. IBKR client and session visibility is `EXTERNAL_EVIDENCE_PENDING`. |
| `WriterEpoch` | A local monotonic fencing counter that, at a future runtime, must be allocated by the database (UD-04). The shell `checked_next` is not that fence. | Broker acceptance, lease safety, or `AdapterSessionEpoch`. Lease expiry is not proof of safe takeover (ARCH-DEC-03, CR-B01 §1.1). | `BINDING_EXISTING` as design precedence. Operational fencing is not proved. XC-04 remainder OPEN. |
| Broker `orderId` | A scoped alias hypothesis. DET-03-D02: a broker order id without context is not a unique order identity. CR-B04 §4.2: candidate alias inside broker, account, client, and session context. Reuse, cross-client visibility, and reconnect stability are unverified. | A primary key. A UNIQUE constraint on `orderId` alone is forbidden until a scope hypothesis is evidenced (CR-B04 §4.1, P0-B04-INV-02). | `REFERENCE_PROPOSAL`. `EXTERNAL_EVIDENCE_PENDING`. |
| `permId` | A long-lived correlation hint. DET-03-D01: not sufficient as the only system-wide identity. CR-B04 §4.2: not the internal id. Null, zero, late assignment, modify, cancel, reconnect, and cross-account behavior are unverified. A conflicting `permId` forces escalation in DET-03 recovery text. | Proof of uniqueness, proof of terminality, or a late-arriving permission to post. | `REFERENCE_PROPOSAL`. `EXTERNAL_EVIDENCE_PENDING`. |
| Execution and fill identity | Internal `execution_id` UUID is platform-minted (CR-B04 §4.1). Broker `execId` is a dedup candidate only inside a verified broker, account, and correction context (CR-B04 §4.2). Glossary: `ExecutionIdentity` with `none_recorded` is not a uniqueness proof. `QuarantinedExecutionIdentity` does not convert back. | A position posting, a reservation conversion, or a bust reversal. | Retention of fills is `BINDING_EXISTING` (ARCH-DEC-12, ARCH-DEC-24, ARCH-DEC-26). `execId` uniqueness is `EXTERNAL_EVIDENCE_PENDING`. |
| Correction and bust chain | A later observation that refers to earlier evidence. DET-03-D05 and CR-B04 §4.3 step 6 describe append-only correction. CR-B03 §3.2 describes economic reversal. That economic reading is not adopted. | Overwrite of the original observation, a compensating ledger line, or a release. ARCH-DEC-26 drafts no compensating line. | Append-only retention is `BINDING_EXISTING`. The economic reversal is `REFERENCE_PROPOSAL` and stays quarantined. |
| Correlation identifiers | Domain `CorrelationId` links commands and events. `HttpRequestId` is process-local and is not that id (UD-01). DET-05-D04 names `correlation_id` on an observation. | A broker execution id, an idempotency key, or a trace that proves broker order. | The split is already in the shell. It is not a new schema. |

`conId` and `orderRef` appear in CR-B04 §4.2 as instrument correlation and an optional round-trip hint. Neither is an order identity. Their stability is `EXTERNAL_EVIDENCE_PENDING`. This package does not add them to a tuple.

## 2. Trust boundaries

| Boundary | Inside | Outside | Rule this package keeps |
| --- | --- | --- | --- |
| Platform minting | `OrderId`, `CommandId`, `SubmissionAttemptId`, `RiskReservationId`, internal execution id, evidence id | Any broker field | Broker text never replaces an internal UUID (DET-03-D01). |
| Supplied context | Fields a caller actually provided, each optional field independently absent | A guessed client, session, epoch, or `permId` | UD-02: absence stays absent. No auto-merge. |
| Raw observation | Immutable payload, payload hash, receive time, adapter version, session context that was actually attached | A normalized label treated as the original | DET-05-D01 and DET-05-D04. Receive order is not evidence priority (DET-05-D04, UD-05). |
| Inference | A candidate set, a gap marker, a "looks unique" note | An observation | Inference is labeled inference. It cannot post or release. |
| Local attempt | Phase names in CR-B01 §1.3, including `DISPATCH_STARTED` and `OUTCOME_UNKNOWN` | Broker acceptance | After `DISPATCH_STARTED`, the attempt is possibly sent until positive evidence of non-send. Timeout, empty list, and restart are not that evidence (CR-B01 §1.3). |
| Database lease and epoch | Local claim fencing | A socket write already in flight, and any broker acceptance | CR-B01 §1.1: a PostgreSQL lease is not external broker fencing. ARCH-DEC-03. |
| Economics | Quarantine of every proposed effect | Booking, conversion, release | ARCH-DEC-13, ARCH-DEC-14, ARCH-DEC-26. |

## 3. Candidate evidence classes

These names are `NEW_PROPOSAL`. They are not an enum, not a registry entry, and not a wire value.

| Class | Observed or inferred | May it post or release? |
| --- | --- | --- |
| Raw broker evidence | Observed, if the bytes and provenance were stored | No. |
| Local attempt and command record | Observed locally. It does not observe the broker. | No. |
| Supplied scope components | Observed only for fields that were present | No. |
| Candidate set `A(e)` from DET-03 formal rule | Inferred | No, at every cardinality, including one. |
| Ambiguity or contradiction note | Inferred | No. |
| Manual escalation marker | A quarantine label. Not a person and not an approval. | No. ARCH-DEC-13: manual review is not authorization to post. ARCH-DEC-33 adds no role. |

DET-03-D04 and CR-B04 §4.3 step 4 say only `MATCHED` may automatically book, and they define `MATCHED` as exactly one consistent order. ARCH-DEC-13 and ARCH-DEC-26 quarantine every proposed economic effect, including a single or uniquely matched event. Both texts are cited. This package does not choose the source's automatic booking. A unique-looking candidate remains `SINGLE_CANDIDATE_NOT_AUTHORIZATION`. `MATCHED` in the source is not a posting permission under the binding decisions.

Candidate ambiguity labels, also `NEW_PROPOSAL`, and not confidence percentages:

| Label | When it would be used | Economic effect |
| --- | --- | --- |
| `NO_CANDIDATE` | No internal order is supported by the supplied scope. DET-03: `\|A(e)\| = 0` is `UNMATCHED`. | None. Unknown external orders are not silently attached (DET-03-D04, ID-INV-02, P0-B04-T08 as an unrun expectation). |
| `SINGLE_CANDIDATE_NOT_AUTHORIZATION` | One candidate and no recorded contradiction. | None. Not a booking. |
| `MULTIPLE_CANDIDATES` | More than one candidate. DET-03: `AMBIGUOUS`. | None. |
| `CONTRADICTION` | A new observation disagrees with a retained binding, payload, or `permId`. DET-03: `CONFLICT`. | None. Escalation stays a quarantine marker. |
| `INSUFFICIENT_SCOPE` | A raw id arrives without the context that hypothesis would require, including a session-scoped `orderId` after reconnect (DET-03 recovery text). | None. The raw id is still retained. |

No label is a probability. No label is broker terminality (ARCH-DEC-17).

## 4. Matching situations

Each situation is analysis. None is an approved matcher.

| Situation | Source anchor | Failure if treated as unique | This package |
| --- | --- | --- | --- |
| Reconnect | DET-03 recovery; CR-B04 T02; DET-08 RC-T10; BA-T02 | A new session reuses an `orderId` that meant a different order, or hides one that is still live. | New session context. Prior observations stay. No carry-over without a retained binding, and a binding still does not post. |
| Account or client change | CR-B04 §4.2 `account`, `clientId`; UD-02 | The same raw id in another account or client is a different order, or the same order is invisible to the new client. | Do not equate across account, environment, client, or session. Visibility is `EXTERNAL_EVIDENCE_PENDING`. |
| Session boundary | DET-03-D02 `session_epoch`; UD-02 adapter epoch is not `WriterEpoch` | Session epoch is mistaken for the writer fence, or a stale session map is reused. | Keep the two epochs distinct. |
| Reused broker id | ID-T01; CR-B04 §4.2 reuse row; P0-B04-INV-02 | A UNIQUE index on the raw id collapses two orders. | No uniqueness index. |
| Missing `permId` | CR-B04 §4.2 null and special values; T10 | Missing is stored as zero and then matches another missing value. | Absence stays absence. |
| Late `permId` | CR-B04 §4.2 stability unknown; DET-03 `permId` conflict escalates | The late value silently rewrites an earlier alias (ID-INV-04). | Append the new observation. Do not overwrite. A disagreement is `CONTRADICTION`. |
| Duplicate execution | ID-T04; CR-B04 §4.3 step 5; ID-INV-03 | A second payload overwrites the first, or a duplicate becomes a second fill. | Identical payload is an idempotent observation and creates no economic effect (ARCH-DEC-16). A different payload is `CONTRADICTION`. Booking is already forbidden. |
| Correction or bust chain | CR-B04 T06; DET-03-D05; CR-B03 §3.2 | The original evidence is replaced and a release or reversal is posted. | Keep both. Link only when the observation itself carries a reference. No compensating line (ARCH-DEC-26). |
| Out-of-order evidence | DET-05-D04; CR-B04 T10; DET-09-D05 is market-data ordering, not an order-id sequence guarantee | Receive order or a guessed sequence number picks a winner. | Retain all. Receive order is not priority. No IBKR sequence number is invented. |
| Unknown external order | ID-T05; CR-B04 §4.3 step 4 | The order is attached to the nearest local order. | `NO_CANDIDATE` and quarantine. |

## 5. Uniqueness proof obligations

CR-B04 §4.4 allows internal UUID uniqueness and a command-scope idempotency key with a payload hash. It defers broker-alias uniqueness until after broker evidence. That split is `REFERENCE_PROPOSAL`. This package approves neither a new constraint nor a migration.

A future uniqueness claim for `orderId`, `permId`, or `execId` would need, and does not have:

- the broker product, API library, and gateway or TWS version actually observed
- the account and client topology actually observed
- null, zero, and missing-field cases
- reconnect, client change, and a second session
- modify, cancel, partial fill, correction, and bust
- timestamps and raw payloads retained in this repository

Until those exist, the claim stays `EXTERNAL_EVIDENCE_PENDING` and `NOT_EXECUTED`. P0-B04-T01 through T10 and ID-T01 through ID-T07 are source test names. They were not executed here. Documentation tests are not those tests.

No canonical broker identity tuple is approved. BROKERQ-01 asks which components a future record may require. An answer would still not publish a schema.

## 6. Preserved conflicts

| Source text | Binding decision | This package |
| --- | --- | --- |
| DET-03-D04 and CR-B04 §4.3 step 4: only `MATCHED` may book automatically. | ARCH-DEC-13 and ARCH-DEC-26: quarantine every proposed economic effect, including one match. | The source sentence stays on record. Booking stays prohibited. |
| DET-08-D04: valid executions are economically considered even when workflow state disagrees. | ARCH-DEC-13 and ARCH-DEC-26. | Evidence is retained. Economics stay quarantined. |
| CR-B03 §3.2: compensating risk-ledger entry after a late fill. | ARCH-DEC-26: no compensating line is drafted as ratified design. | Not adopted. |
| DET-06-D03 and CR-B09 P1-B09-D02: a cancel may be allowed during a submission halt under stated conditions. | ARCH-DEC-20: modify and implicit cancel-replace are blocked, and cancel during halt is not authorized. | No cancel is dispatched. The conflict stays open for a later decision. XC-10-ACTION stays OPEN. |
| DET-07 failover wording that is less restrictive than CR-B01. | ARCH-DEC-03: CR-B01 has precedence. | No automatic takeover. |
| CR-B01 and CR-B04 headers say `DESIGN_RESOLVED`. | ARCH-DEC-04: no status word wins. XC-11 stays OPEN. | `DESIGN_RESOLVED` is not verified and not implemented. |

## 7. Failure modes

| Failure | Why it is unsafe | What remains allowed |
| --- | --- | --- |
| Global uniqueness of `permId` or `orderId` | CR-B04 §4.2 lists the scope as unverified. A collision merges two orders. | Retain both observations. |
| Auto-match on a singleton candidate | ARCH-DEC-13 forbids the booking that DET-03-D04 would allow. | Record the candidate as inference. |
| Missing `permId` equals missing `permId` | Absence is not a shared key (UD-02). | Leave the field absent. |
| Overwrite of a confirmed alias | ID-INV-04 and DET-03-D05 require an audited append. | Append. |
| Replay creates a second send or a second economic effect | ARCH-DEC-16 and P0-B01-INV-01. | Return the stored observation. Create nothing economic and send nothing. |
| Manual review posts | ARCH-DEC-13 and ARCH-DEC-33. | Escalation is a marker only. |
| Adapter `READY` authorizes trading | BA-INV-04. DET-06: a ready snapshot is not a lasting permission. | No dispatch. |

## 8. Explicit non-authorizations

This file does not authorize an identity resolver, a uniqueness index, a schema, a migration, a canonical tuple, economic posting, reservation conversion, reservation release, an economic-finality predicate, an OMS transition, broker terminal acceptance, modify, implicit cancel-replace, a risk-reducing broker order, automatic retry, automatic takeover, a cancel, a role, a lock order, an audit table name, live trading, or a broker call. `XC-01` stays OPEN. `XC-02-PRED` stays OPEN. `XC-03-MATRIX` stays OPEN. `XC-08-IBKR` stays OPEN. The XC-04 remainder stays OPEN.

Live trading is NOT AUTHORIZED.
