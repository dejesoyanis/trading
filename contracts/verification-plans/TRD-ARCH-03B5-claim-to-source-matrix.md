# TRD-ARCH-03B.5 — claim to source matrix

**Date:** 2026-11-16  
**Role:** map from the 27 existing claims to official text. `document_role` = `NON_RUNTIME_NON_WIRE_RESEARCH_NOTE`.  
**Official sources obtained:** 0.  
**Empirical upgrade:** none. Every claim stays `HYPOTHESIS`, `EXTERNAL_EVIDENCE_PENDING`, and `NOT_EXECUTED`.

The headings are copied from `contracts/verification-plans/TRD-ARCH-03B4-ibkr-evidence-inventory.md`. They are internal hypotheses. A research topic names which question in the source register would be taken to a future page. A research topic is not a retrieved source. The official source and the section anchor are `NOT_ESTABLISHED` on every row.

Silence is not a negative finding. A missing page does not show that an identifier is unique, that a snapshot is complete, that a send did not happen, or that recovery is safe.

No row closes `XC-01`, `XC-08-IBKR`, `XC-02-PRED`, `XC-03-MATRIX`, or the operational remainder of `XC-04`. A later official sentence could support a test oracle. It would still not prove uniqueness, broker completeness, economic finality, takeover, or recovery.

## Mapping

| ID | Internal heading | Research topic | Documentation class | Official source | Section anchor | Hypothesis | Evidence | Execution |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| IBKR-CLM-01 | orderId namespace and placement context | T-04 | `UNVERIFIED_RESEARCH_TOPIC` | `NOT_ESTABLISHED` | `NOT_ESTABLISHED` | `HYPOTHESIS` | `EXTERNAL_EVIDENCE_PENDING` | `NOT_EXECUTED` |
| IBKR-CLM-02 | orderId reuse across clients and sessions | T-04 | `UNVERIFIED_RESEARCH_TOPIC` | `NOT_ESTABLISHED` | `NOT_ESTABLISHED` | `HYPOTHESIS` | `EXTERNAL_EVIDENCE_PENDING` | `NOT_EXECUTED` |
| IBKR-CLM-03 | orderId across reconnect and restart | T-04 | `UNVERIFIED_RESEARCH_TOPIC` | `NOT_ESTABLISHED` | `NOT_ESTABLISHED` | `HYPOTHESIS` | `EXTERNAL_EVIDENCE_PENDING` | `NOT_EXECUTED` |
| IBKR-CLM-04 | permId absent, null, and zero stay distinct | T-05 | `UNVERIFIED_RESEARCH_TOPIC` | `NOT_ESTABLISHED` | `NOT_ESTABLISHED` | `HYPOTHESIS` | `EXTERNAL_EVIDENCE_PENDING` | `NOT_EXECUTED` |
| IBKR-CLM-05 | permId assignment timing | T-05 | `UNVERIFIED_RESEARCH_TOPIC` | `NOT_ESTABLISHED` | `NOT_ESTABLISHED` | `HYPOTHESIS` | `EXTERNAL_EVIDENCE_PENDING` | `NOT_EXECUTED` |
| IBKR-CLM-06 | permId stability, limits, and standalone non-uniqueness | T-05 | `UNVERIFIED_RESEARCH_TOPIC` | `NOT_ESTABLISHED` | `NOT_ESTABLISHED` | `HYPOTHESIS` | `EXTERNAL_EVIDENCE_PENDING` | `NOT_EXECUTED` |
| IBKR-CLM-07 | execId namespace and duplicates | T-06 | `UNVERIFIED_RESEARCH_TOPIC` | `NOT_ESTABLISHED` | `NOT_ESTABLISHED` | `HYPOTHESIS` | `EXTERNAL_EVIDENCE_PENDING` | `NOT_EXECUTED` |
| IBKR-CLM-08 | correction and bust references | T-06 | `UNVERIFIED_RESEARCH_TOPIC` | `NOT_ESTABLISHED` | `NOT_ESTABLISHED` | `HYPOTHESIS` | `EXTERNAL_EVIDENCE_PENDING` | `NOT_EXECUTED` |
| IBKR-CLM-09 | out-of-order execution delivery | T-08 | `UNVERIFIED_RESEARCH_TOPIC` | `NOT_ESTABLISHED` | `NOT_ESTABLISHED` | `HYPOTHESIS` | `EXTERNAL_EVIDENCE_PENDING` | `NOT_EXECUTED` |
| IBKR-CLM-10 | order status, execution observations, and local attempts | T-08 | `UNVERIFIED_RESEARCH_TOPIC` | `NOT_ESTABLISHED` | `NOT_ESTABLISHED` | `HYPOTHESIS` | `EXTERNAL_EVIDENCE_PENDING` | `NOT_EXECUTED` |
| IBKR-CLM-11 | request and response scope | T-09 | `UNVERIFIED_RESEARCH_TOPIC` | `NOT_ESTABLISHED` | `NOT_ESTABLISHED` | `HYPOTHESIS` | `EXTERNAL_EVIDENCE_PENDING` | `NOT_EXECUTED` |
| IBKR-CLM-12 | pagination and end of response | T-09 | `UNVERIFIED_RESEARCH_TOPIC` | `NOT_ESTABLISHED` | `NOT_ESTABLISHED` | `HYPOTHESIS` | `EXTERNAL_EVIDENCE_PENDING` | `NOT_EXECUTED` |
| IBKR-CLM-13 | retention, truncation, delay, and gaps | T-09 | `UNVERIFIED_RESEARCH_TOPIC` | `NOT_ESTABLISHED` | `NOT_ESTABLISHED` | `HYPOTHESIS` | `EXTERNAL_EVIDENCE_PENDING` | `NOT_EXECUTED` |
| IBKR-CLM-14 | empty response | T-09 | `UNVERIFIED_RESEARCH_TOPIC` | `NOT_ESTABLISHED` | `NOT_ESTABLISHED` | `HYPOTHESIS` | `EXTERNAL_EVIDENCE_PENDING` | `NOT_EXECUTED` |
| IBKR-CLM-15 | no completeness token | T-09 | `UNVERIFIED_RESEARCH_TOPIC` | `NOT_ESTABLISHED` | `NOT_ESTABLISHED` | `HYPOTHESIS` | `EXTERNAL_EVIDENCE_PENDING` | `NOT_EXECUTED` |
| IBKR-CLM-16 | reconnect is a new session context | T-10 | `UNVERIFIED_RESEARCH_TOPIC` | `NOT_ESTABLISHED` | `NOT_ESTABLISHED` | `HYPOTHESIS` | `EXTERNAL_EVIDENCE_PENDING` | `NOT_EXECUTED` |
| IBKR-CLM-17 | gateway or TWS restart | T-01+T-02 | `UNVERIFIED_RESEARCH_TOPIC` | `NOT_ESTABLISHED` | `NOT_ESTABLISHED` | `HYPOTHESIS` | `EXTERNAL_EVIDENCE_PENDING` | `NOT_EXECUTED` |
| IBKR-CLM-18 | disconnect, missing executions, partial fills, and late fills | T-08 | `UNVERIFIED_RESEARCH_TOPIC` | `NOT_ESTABLISHED` | `NOT_ESTABLISHED` | `HYPOTHESIS` | `EXTERNAL_EVIDENCE_PENDING` | `NOT_EXECUTED` |
| IBKR-CLM-19 | cancel races and conflicting snapshots | T-13 | `UNVERIFIED_RESEARCH_TOPIC` | `NOT_ESTABLISHED` | `NOT_ESTABLISHED` | `HYPOTHESIS` | `EXTERNAL_EVIDENCE_PENDING` | `NOT_EXECUTED` |
| IBKR-CLM-20 | unknown send before DISPATCH_STARTED | T-08 | `UNVERIFIED_RESEARCH_TOPIC` | `NOT_ESTABLISHED` | `NOT_ESTABLISHED` | `HYPOTHESIS` | `EXTERNAL_EVIDENCE_PENDING` | `NOT_EXECUTED` |
| IBKR-CLM-21 | unknown send after DISPATCH_STARTED | T-08 | `UNVERIFIED_RESEARCH_TOPIC` | `NOT_ESTABLISHED` | `NOT_ESTABLISHED` | `HYPOTHESIS` | `EXTERNAL_EVIDENCE_PENDING` | `NOT_EXECUTED` |
| IBKR-CLM-22 | lost acknowledgement and crash after send | T-08 | `UNVERIFIED_RESEARCH_TOPIC` | `NOT_ESTABLISHED` | `NOT_ESTABLISHED` | `HYPOTHESIS` | `EXTERNAL_EVIDENCE_PENDING` | `NOT_EXECUTED` |
| IBKR-CLM-23 | prior writer, lease expiry, and split brain | T-10 | `UNVERIFIED_RESEARCH_TOPIC` | `NOT_ESTABLISHED` | `NOT_ESTABLISHED` | `HYPOTHESIS` | `EXTERNAL_EVIDENCE_PENDING` | `NOT_EXECUTED` |
| IBKR-CLM-24 | session isolation is external evidence | T-10 | `UNVERIFIED_RESEARCH_TOPIC` | `NOT_ESTABLISHED` | `NOT_ESTABLISHED` | `HYPOTHESIS` | `EXTERNAL_EVIDENCE_PENDING` | `NOT_EXECUTED` |
| IBKR-CLM-25 | WriterEpoch, adapter session epoch, acceptance, and isolation | T-10 | `UNVERIFIED_RESEARCH_TOPIC` | `NOT_ESTABLISHED` | `NOT_ESTABLISHED` | `HYPOTHESIS` | `EXTERNAL_EVIDENCE_PENDING` | `NOT_EXECUTED` |
| IBKR-CLM-26 | version, platform, and revalidation | T-14 | `UNVERIFIED_RESEARCH_TOPIC` | `NOT_ESTABLISHED` | `NOT_ESTABLISHED` | `HYPOTHESIS` | `EXTERNAL_EVIDENCE_PENDING` | `NOT_EXECUTED` |
| IBKR-CLM-27 | a singleton candidate is not authorization | T-12 | `UNVERIFIED_RESEARCH_TOPIC` | `NOT_ESTABLISHED` | `NOT_ESTABLISHED` | `HYPOTHESIS` | `EXTERNAL_EVIDENCE_PENDING` | `NOT_EXECUTED` |

Count: 27. Each claim identifier appears once in the table. The class on every row is `UNVERIFIED_RESEARCH_TOPIC`. `OFFICIAL_DOCUMENTATION_OBTAINED` is not used. `DOCUMENTED_CLAIM_NOT_EMPIRICALLY_VERIFIED` is not used, because no official sentence was obtained to which that label could attach.

**NO IMPLEMENTATION.** **NO VERIFICATION.**
