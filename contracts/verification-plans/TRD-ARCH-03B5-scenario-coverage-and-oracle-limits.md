# TRD-ARCH-03B.5 — scenario coverage and oracle limits

**Date:** 2026-11-16  
**Role:** coverage of the 43 existing scenarios against official text. `document_role` = `NON_RUNTIME_NON_WIRE_RESEARCH_NOTE`.  
**Execution:** every row stays `NOT_EXECUTED`.  
**External evidence:** every row stays `EXTERNAL_EVIDENCE_PENDING`.  
**Authorization:** `NOT_AUTHORIZED`. No scenario was run.

Titles, families, and claim links are copied from `contracts/verification-plans/TRD-ARCH-03B4-ibkr-test-matrix.md`. The historical action words `offline`, `read-only`, and `controlled` stay names of a future stage. They are not permission to run it. Official section anchors are `NOT_ESTABLISHED` because no official text was retrieved.

The oracle-limit tokens below are documentary labels in this research note. They are not a canonical enum, not a registry entry, and not a wire schema.

## Oracle-limit labels

- `IDENTITY_OBSERVATION_IS_NOT_UNIQUENESS` — A later official sentence would not prove that the alias is unique or that two observations are one order.
- `CORRECTION_TEXT_IS_NOT_A_REVERSAL` — A later official sentence would not authorize a reversal booking, a risk release, or a closure change. ARCH-DEC-56 still requires the new raw payload itself to name any structured link.
- `STATUS_TEXT_IS_NOT_ECONOMIC_FINALITY` — A later official sentence about a status word would not approve an economic-finality predicate.
- `SNAPSHOT_TEXT_IS_NOT_BROKER_COMPLETENESS` — A later official sentence would not prove broker completeness. The end of a response is not global completeness. An empty capture is not broker absence.
- `SEND_TEXT_IS_NOT_OUTCOME_OR_TAKEOVER` — A later official sentence would not prove whether a send occurred and would not authorize retry, resend, or takeover.
- `RECONNECT_TEXT_IS_NOT_A_NEGATIVE_SEND_PROOF` — A later official sentence about reconnect or restart would not prove that an unknown send did not happen.
- `ISOLATION_TEXT_IS_NOT_FENCING_OR_TAKEOVER` — A later official sentence would not be a broker fencing guarantee and would not prove takeover. A checklist remains a checklist under ARCH-DEC-58.
- `NEGATIVE_TEXT_IS_NOT_AUTHORIZATION` — A later official sentence would not authorize the effect this negative row forbids.

## Mapping

| ID | Title | Claims | Official section | Class | Oracle limit | Execution | Evidence |
| --- | --- | --- | --- | --- | --- | --- | --- |
| IBKR-SCN-A01 | Reused `orderId` in another session | IBKR-CLM-02, IBKR-CLM-03 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `IDENTITY_OBSERVATION_IS_NOT_UNIQUENESS` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-A02 | Same `orderId` text seen by two clients | IBKR-CLM-01, IBKR-CLM-02 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `IDENTITY_OBSERVATION_IS_NOT_UNIQUENESS` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-A03 | Same `orderId` text on two accounts | IBKR-CLM-01, IBKR-CLM-02 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `IDENTITY_OBSERVATION_IS_NOT_UNIQUENESS` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-A04 | Absent `permId` is retained | IBKR-CLM-04 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `IDENTITY_OBSERVATION_IS_NOT_UNIQUENESS` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-A05 | Null `permId` is not zero | IBKR-CLM-04 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `IDENTITY_OBSERVATION_IS_NOT_UNIQUENESS` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-A06 | Zero `permId` is not missing | IBKR-CLM-04, IBKR-CLM-06 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `IDENTITY_OBSERVATION_IS_NOT_UNIQUENESS` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-A07 | Late `permId` is appended | IBKR-CLM-05, IBKR-CLM-06 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `IDENTITY_OBSERVATION_IS_NOT_UNIQUENESS` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-A08 | Duplicate `execId` with identical raw payload | IBKR-CLM-07 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `IDENTITY_OBSERVATION_IS_NOT_UNIQUENESS` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-A09 | Duplicate `execId` with a different payload | IBKR-CLM-07 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `IDENTITY_OBSERVATION_IS_NOT_UNIQUENESS` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-A10 | Correction or bust whose new evidence names a reference | IBKR-CLM-08 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `CORRECTION_TEXT_IS_NOT_A_REVERSAL` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-A11 | Correction or bust whose new evidence names no reference | IBKR-CLM-08 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `CORRECTION_TEXT_IS_NOT_A_REVERSAL` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-A12 | Contradictory raw payload under one apparent key | IBKR-CLM-06, IBKR-CLM-07, IBKR-CLM-10 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `IDENTITY_OBSERVATION_IS_NOT_UNIQUENESS` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-A13 | Out-of-order execution evidence | IBKR-CLM-09 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `IDENTITY_OBSERVATION_IS_NOT_UNIQUENESS` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-A14 | Terminal-looking status beside an unknown local attempt | IBKR-CLM-10 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `STATUS_TEXT_IS_NOT_ECONOMIC_FINALITY` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-B01 | Empty result | IBKR-CLM-14, IBKR-CLM-15 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `SNAPSHOT_TEXT_IS_NOT_BROKER_COMPLETENESS` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-B02 | Truncated window | IBKR-CLM-11, IBKR-CLM-13 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `SNAPSHOT_TEXT_IS_NOT_BROKER_COMPLETENESS` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-B03 | Pagination break | IBKR-CLM-12 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `SNAPSHOT_TEXT_IS_NOT_BROKER_COMPLETENESS` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-B04 | Dropped connection during a read | IBKR-CLM-12, IBKR-CLM-18 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `SNAPSHOT_TEXT_IS_NOT_BROKER_COMPLETENESS` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-B05 | Delayed execution | IBKR-CLM-13, IBKR-CLM-18 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `SNAPSHOT_TEXT_IS_NOT_BROKER_COMPLETENESS` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-B06 | Missing end marker | IBKR-CLM-12, IBKR-CLM-15 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `SNAPSHOT_TEXT_IS_NOT_BROKER_COMPLETENESS` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-B07 | Cross-session snapshot confusion | IBKR-CLM-11, IBKR-CLM-16 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `SNAPSHOT_TEXT_IS_NOT_BROKER_COMPLETENESS` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-B08 | Cross-account snapshot confusion | IBKR-CLM-11 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `SNAPSHOT_TEXT_IS_NOT_BROKER_COMPLETENESS` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-B09 | Stale cache | IBKR-CLM-11, IBKR-CLM-13 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `SNAPSHOT_TEXT_IS_NOT_BROKER_COMPLETENESS` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-B10 | Mismatched request scope | IBKR-CLM-11 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `SNAPSHOT_TEXT_IS_NOT_BROKER_COMPLETENESS` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-B11 | Different API or product versions | IBKR-CLM-26, IBKR-CLM-11 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `SNAPSHOT_TEXT_IS_NOT_BROKER_COMPLETENESS` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-C01 | Unknown send before `DISPATCH_STARTED` | IBKR-CLM-20 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `SEND_TEXT_IS_NOT_OUTCOME_OR_TAKEOVER` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-C02 | Unknown send after `DISPATCH_STARTED` | IBKR-CLM-21 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `SEND_TEXT_IS_NOT_OUTCOME_OR_TAKEOVER` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-C03 | Timeout after `DISPATCH_STARTED` | IBKR-CLM-21 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `SEND_TEXT_IS_NOT_OUTCOME_OR_TAKEOVER` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-C04 | Lost acknowledgement | IBKR-CLM-22 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `SEND_TEXT_IS_NOT_OUTCOME_OR_TAKEOVER` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-C05 | Crash after a possible send | IBKR-CLM-21, IBKR-CLM-22 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `SEND_TEXT_IS_NOT_OUTCOME_OR_TAKEOVER` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-C06 | Reconnect while a send is unknown | IBKR-CLM-16, IBKR-CLM-21 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `RECONNECT_TEXT_IS_NOT_A_NEGATIVE_SEND_PROOF` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-C07 | Gateway or TWS restart while a send is unknown | IBKR-CLM-17, IBKR-CLM-03, IBKR-CLM-21 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `RECONNECT_TEXT_IS_NOT_A_NEGATIVE_SEND_PROOF` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-C08 | Prior writer still possibly active | IBKR-CLM-23, IBKR-CLM-24 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `ISOLATION_TEXT_IS_NOT_FENCING_OR_TAKEOVER` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-C09 | Expired lease | IBKR-CLM-23, IBKR-CLM-25 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `ISOLATION_TEXT_IS_NOT_FENCING_OR_TAKEOVER` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-C10 | Split brain | IBKR-CLM-23, IBKR-CLM-24 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `ISOLATION_TEXT_IS_NOT_FENCING_OR_TAKEOVER` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-C11 | Newer local epoch | IBKR-CLM-25, IBKR-CLM-23 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `ISOLATION_TEXT_IS_NOT_FENCING_OR_TAKEOVER` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-C12 | Partial fill during disconnect | IBKR-CLM-18, IBKR-CLM-21 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `SEND_TEXT_IS_NOT_OUTCOME_OR_TAKEOVER` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-D01 | Singleton candidate is not authorization | IBKR-CLM-27 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `NEGATIVE_TEXT_IS_NOT_AUTHORIZATION` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-D02 | Local capture gap is not an IBKR sequence | IBKR-CLM-12 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `NEGATIVE_TEXT_IS_NOT_AUTHORIZATION` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-D03 | No completeness token | IBKR-CLM-15, IBKR-CLM-14 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `SNAPSHOT_TEXT_IS_NOT_BROKER_COMPLETENESS` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-D04 | No broker-wins rule | IBKR-CLM-10, IBKR-CLM-19 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `NEGATIVE_TEXT_IS_NOT_AUTHORIZATION` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-D05 | No economic effect and no release | IBKR-CLM-08, IBKR-CLM-14, IBKR-CLM-27 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `NEGATIVE_TEXT_IS_NOT_AUTHORIZATION` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |
| IBKR-SCN-D06 | No dispatch | IBKR-CLM-21, IBKR-CLM-23 | `NOT_ESTABLISHED` | `UNVERIFIED_RESEARCH_TOPIC` | `NEGATIVE_TEXT_IS_NOT_AUTHORIZATION` | `NOT_EXECUTED` | `EXTERNAL_EVIDENCE_PENDING` |

Count: 43. Each scenario identifier appears once in the table. No row records a pass, a fail, a broker timestamp, or a signature.

## Unknown send

IBKR-SCN-C01 through IBKR-SCN-C07 and IBKR-SCN-C12 stay `NOT_EXECUTED`. They cover an unknown send before `DISPATCH_STARTED`, after `DISPATCH_STARTED`, timeout, a lost acknowledgement, a crash after a possible send, reconnect or restart while a send is unknown, and a partial fill during disconnect. Official text was not obtained. ARCH-DEC-57 keeps crash and disconnect cases for an unknown send synthetic or offline until a new explicit decision. This note does not make that decision. It does not authorize fault injection, retry, resend, takeover, or E3.

## Snapshot gaps

IBKR-SCN-B01 through IBKR-SCN-B11, IBKR-SCN-D02, and IBKR-SCN-D03 stay `NOT_EXECUTED`. They cover an empty result, a truncated window, a pagination break, a dropped connection, delay, a missing end marker, cross-session and cross-account confusion, a stale cache, a mismatched scope, a version difference, a local capture gap, and the absence of a completeness token. No official description of filters, paging, or reconnect was retrieved. A response end is not global completeness. Capture completeness is not broker completeness. Coverage stays `UNKNOWN`. No completeness token is stored.

## Corrections and busts

IBKR-SCN-A10 and IBKR-SCN-A11 stay `NOT_EXECUTED`. One historical row is a correction whose new evidence names a reference. The other is a correction whose new evidence names none. No official correction text was retrieved. ARCH-DEC-56 still allows a structured link only when the new raw payload itself contains it. No payload exists. No reversal, risk release, or closure change is approved.

## Reconnect

IBKR-SCN-A01, IBKR-SCN-B07, IBKR-SCN-C06, and IBKR-SCN-C07 stay `NOT_EXECUTED`. They ask about another session, cross-session snapshot confusion, reconnect during an unknown send, and a gateway or TWS restart during an unknown send. TWS API and IB Gateway were not retrieved and are not treated as one product. Reconnect is not recorded as proof that a send did not happen. No session identifier was copied from a broker page.

## Writer isolation

IBKR-SCN-C08 through IBKR-SCN-C11 stay `NOT_EXECUTED`. They ask about a prior writer, an expired lease, split brain, and a newer local epoch. No official session or recovery limit was retrieved. Nothing in this file is read as a broker fencing guarantee. ARCH-DEC-58 remains a checklist. It is not takeover proof. `WriterEpoch`, adapter session epoch, broker acceptance, and isolation evidence stay four separate facts.

## Later tests that would still be required

A retrieved page would not execute any row. A later controlled or read-only case would need its own authorization, a pinned tuple, and raw observations. Those cases are not designed here and are not started. Offline synthetic fixtures remain stage E1, which stays STOP.

**NO IMPLEMENTATION.** **NO VERIFICATION.**
