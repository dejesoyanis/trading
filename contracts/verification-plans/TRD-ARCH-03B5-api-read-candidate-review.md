# TRD-ARCH-03B.5 — API read candidate review

**Date:** 2026-11-16  
**Role:** candidate list only. `document_role` = `NON_RUNTIME_NON_WIRE_RESEARCH_NOTE`.  
**Decision recorded here:** `NO_READS_APPROVED`.  
**E2:** STOP. `stage_e2_authorized=false`.  
**Broker connection:** not authorized. **Read-only broker access:** not authorized.

ARCH-DEC-53 remains binding. No broker connection and no read occur before a separate dated decision names the exact officially documented read operations that would be allowed. This file names no operation. Cancel, modify, and replace are never reads. That sentence is the existing decision. It is not a newly retrieved IBKR operation name.

## Candidates

Count: 0.

| Candidate name | API family | Documentation anchor | Side effects and unclear points | Scope | Separate decision |
| --- | --- | --- | --- | --- | --- |
| none | `UNRECORDED` | `NOT_ESTABLISHED` | No name was obtained, so no side effect is inferred from a page | `UNRECORDED` | Not requested as a ratification. A later decision would have to name an operation that a retrieved official text actually uses |

No method, path, message type, callback, or request name is invented. TWS API, IB Gateway, and Client Portal API stay separate research topics (T-01, T-02, T-03). They were not retrieved, so they are not assigned any read. An empty candidate list is not evidence that no read exists. It records that this milestone did not obtain a name.

Order placement, cancel, modify, and replace stay in the not-read-only class under ARCH-DEC-53. No endpoint in that class is named here, and none is called.

A future candidate would still be a broker request. Naming it in a later note would not approve it. E2 would remain STOP until an authorization names E2 and names the operations. Paper mode is not E2.

## Not done

No socket was opened. No gateway or TWS process was started. No account was selected. No query was sent. No mutation was sent. `external_broker_tests_executed` stays false.

**NO IMPLEMENTATION.** **NO VERIFICATION.**
