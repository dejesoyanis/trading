# TRD-ARCH-03B.4 — IBKR evidence inventory

**Date:** 2026-11-15  
**Role:** claim register for a future evidence package. `document_role` = `NON_RUNTIME_NON_WIRE_VERIFICATION_PLAN`.  
**Execution:** `NOT_EXECUTED`.  
**External evidence:** `EXTERNAL_EVIDENCE_PENDING`.  
**Ratification:** `NOT_RATIFIED`.  
**Authorization:** `NOT_AUTHORIZED`. This file does not authorize a broker connection, a read, an order, a cancel, a retry, or a test.  
**Live trading:** NOT AUTHORIZED.

Each claim below is a `HYPOTHESIS`. The hypothesis is internal source text read as an unverified question. It is not an IBKR guarantee. No official IBKR document, URL, raw payload, version string, or test result is attached. No API operation name is asserted. A research topic is a label for later reading. The words `UNVERIFIED_RESEARCH_TOPIC` mean the operation name, parameters, and completion signal remain unknown.

Documentation tests that read this file are not broker evidence.

ARCH-DEC-35 through ARCH-DEC-48 stay binding design policy and stay `NOT_IMPLEMENTED` and `NOT_VERIFIED`. This inventory does not supersede them. CR-B01 keeps design precedence over conflicting or less restrictive DET-07 text (ARCH-DEC-03). ARCH-DEC-13 and ARCH-DEC-26 keep the DET-03-D04 single-match booking sentence subordinate. No compensating ledger line is approved.

## How to read a claim

| Field | Meaning in this file |
| --- | --- |
| Status | `HYPOTHESIS` |
| Evidence | `EXTERNAL_EVIDENCE_PENDING` |
| Execution | `NOT_EXECUTED` |
| Authorization | `NOT_AUTHORIZED` |
| Research topic | `UNVERIFIED_RESEARCH_TOPIC`. Not a method signature and not a URL. |
| Positive test | A future case that would have to succeed before the hypothesis could be called evidenced. Not run. |
| Negative test | A future case that would have to fail closed. Not run. |
| Would not prove | Effects that stay blocked even if the hypothesis were later evidenced. |

`XC-01` stays OPEN and `EVIDENCE_PENDING`. `XC-08-IBKR` stays OPEN and `EXTERNAL_EVIDENCE_PENDING`. The operational remainder of `XC-04` stays OPEN. `XC-02-PRED` stays OPEN.

## IBKR-CLM-01 — orderId namespace and placement context

**Status:** `HYPOTHESIS`. **Evidence:** `EXTERNAL_EVIDENCE_PENDING`. **Execution:** `NOT_EXECUTED`. **Authorization:** `NOT_AUTHORIZED`.

**Internal source anchor:** DET-03-D01 and DET-03-D02 in `contracts/sources/DET-03-bis-DET-08-Architecture-Contracts-v1.0.md`. CR-B04 §4.1 `P0-B04-D01` and the `orderId` row of CR-B04 §4.2 in `contracts/sources/P0-Resolution-Contracts-CR-B01-bis-CR-B04-v1.0.md`. ARCH-DEC-35.

**Hypothesis:** `orderId` is a broker-side alias. Internal `order_id` stays the platform identity. The namespace of `orderId` is whatever client, session, account, and placement context a future raw payload and official text actually show. A bare `orderId` value is not an order identity. This repository does not know that namespace.

**Proposed official evidence type:** official product text for the pinned version, plus a raw placement observation and a raw open-order observation that contain only fields the payload actually carries.

**Research topic:** `UNVERIFIED_RESEARCH_TOPIC`. Order-placement observation and open-order dataset, field label `orderId`, including the client and account context printed beside it. Operation name unrecorded.

**Positive test:** a future case in which the captured payload shows the context fields that were actually delivered, and absence stays absence. Not run. Scenario `IBKR-SCN-A02`.

**Negative test:** a future case in which two equal `orderId` texts in different contexts stay two observations. Not run. Scenario `IBKR-SCN-A03`.

**Environmental prerequisites:** pinned product, API client, OS, and account mode. Unrecorded. Stage E0 has not been run.

**Known ambiguity:** whether the placing context is the client, the session, the account, the gateway process, or some other scope. All of those remain open.

**Would not prove:** uniqueness, a merge, a canonical identity tuple, a schema, or an economic match.

**Decision blockers:** `XC-01` OPEN. ARCH-DEC-35 records supplied fields only. Equality is not uniqueness.

## IBKR-CLM-02 — orderId reuse across clients and sessions

**Status:** `HYPOTHESIS`. **Evidence:** `EXTERNAL_EVIDENCE_PENDING`. **Execution:** `NOT_EXECUTED`. **Authorization:** `NOT_AUTHORIZED`.

**Internal source anchor:** CR-B04 §4.2 `orderId` row, columns "Vorläufige Resolver-Nutzung" and "Vor Produktivbetrieb zu verifizieren". DET-03 tests `ID-T01`. DET-03 recovery paragraph: session-specific `orderId` values are not interpreted without session and client context. ARCH-DEC-35. ARCH-DEC-43.

**Hypothesis:** An `orderId` value might be reused, or might be visible to another client or another session. Whether any of that happens is unverified. Equal text is not a merge and not a uniqueness proof.

**Proposed official evidence type:** official text on identifier lifetime and visibility, plus paired raw observations from two clients or two sessions.

**Research topic:** `UNVERIFIED_RESEARCH_TOPIC`. Visibility of an existing order alias across a second client and across a later session. Operation name unrecorded.

**Positive test:** both observations retained with their own session context. Not run. Scenario `IBKR-SCN-A01`.

**Negative test:** the second observation does not become `MATCHED` and does not post. Not run. Scenario `IBKR-SCN-A02`.

**Environmental prerequisites:** two client contexts and two session contexts, each labeled. Unrecorded. No second client is connected by this plan.

**Known ambiguity:** "reuse" can mean the broker assigned the same text again, or a reader collapsed two texts. Those are different events. This claim separates them and resolves neither.

**Would not prove:** that reuse is impossible, that reuse is guaranteed, or that a UNIQUE index is safe.

**Decision blockers:** `XC-01` OPEN. P0-B04-INV-02. UD-02.

## IBKR-CLM-03 — orderId across reconnect and restart

**Status:** `HYPOTHESIS`. **Evidence:** `EXTERNAL_EVIDENCE_PENDING`. **Execution:** `NOT_EXECUTED`. **Authorization:** `NOT_AUTHORIZED`.

**Internal source anchor:** DET-03 recovery paragraph. CR-B04 §4.5 `T02` and `T03`. ARCH-DEC-43. CR-B01 §1.2 item 8.

**Hypothesis:** A reconnect or a gateway or TWS restart may change which `orderId` values remain visible or stable. Prior evidence stays. An identifier crosses the session boundary only with an evidenced binding. The reconnect itself is not proof that a previous send did not happen.

**Proposed official evidence type:** official reconnect and restart notes for the pinned version, plus raw observations from before and after a future controlled restart.

**Research topic:** `UNVERIFIED_RESEARCH_TOPIC`. Order-alias visibility after a connection is established again, and after a gateway or TWS process restart. Operation name unrecorded.

**Positive test:** old observation retained beside a new session context. Not run. Scenario `IBKR-SCN-A01`.

**Negative test:** missing alias after restart is not `DEFINITIVELY_NOT_SENT`. Not run. Scenario `IBKR-SCN-C07`.

**Environmental prerequisites:** a future isolated gateway or TWS process whose version is pinned. Unrecorded. This plan does not start that process.

**Known ambiguity:** a restart of the client, of the gateway, and of TWS may differ. They are not collapsed here.

**Would not prove:** external fencing, a safe retry, or session exclusivity.

**Decision blockers:** `XC-01` OPEN. Operational remainder of `XC-04` OPEN. P0-B04-T02 and T03 stay `NOT_EXECUTED`.

## IBKR-CLM-04 — permId absent, null, and zero stay distinct

**Status:** `HYPOTHESIS`. **Evidence:** `EXTERNAL_EVIDENCE_PENDING`. **Execution:** `NOT_EXECUTED`. **Authorization:** `NOT_AUTHORIZED`.

**Internal source anchor:** CR-B04 §4.2 `permId` row. CR-B04 §4.5 `T10`. UD-02 in `contracts/domain/unresolved-decisions.md`. ARCH-DEC-37. Glossary row `BrokerPermIdAlias` in `contracts/domain/glossary.md`.

**Hypothesis:** A missing `permId`, a null `permId`, and a zero `permId` are three observations. They are not silently the same value. Missing does not equal another missing value. The broker's null and zero meaning is externally unchecked.

**Proposed official evidence type:** official field notes for unset, null, and zero, plus raw payloads that actually exhibit each case, kept as separate observations.

**Research topic:** `UNVERIFIED_RESEARCH_TOPIC`. Representation of an absent permanent alias in order and execution observations. Operation name unrecorded. This claim does not assert that the target broker uses zero as missing.

**Positive test:** an observation with `permId` absent is retained. Not run. Scenario `IBKR-SCN-A04`.

**Negative test:** null is not stored as zero, and zero is not stored as missing. Not run. Scenarios `IBKR-SCN-A05` and `IBKR-SCN-A06`.

**Environmental prerequisites:** raw payload bytes and an explicit absence marker that is not a numeric zero. Unrecorded.

**Known ambiguity:** a decoder can turn absence into zero before the evidence store sees it. That decoder bug is a local capture defect, not a broker fact.

**Would not prove:** a `permId` uniqueness constraint, a match of all zeros, or a booking when the field later appears.

**Decision blockers:** `XC-01` OPEN. External null and zero behavior is `EXTERNAL_EVIDENCE_PENDING`.

## IBKR-CLM-05 — permId assignment timing

**Status:** `HYPOTHESIS`. **Evidence:** `EXTERNAL_EVIDENCE_PENDING`. **Execution:** `NOT_EXECUTED`. **Authorization:** `NOT_AUTHORIZED`.

**Internal source anchor:** ARCH-DEC-37. DET-03-D01, sentence that `permId` is not the sole system identity. CR-B04 §4.2 `permId` row, stability at modify, cancel, and reconnect.

**Hypothesis:** A `permId` may be absent on an early observation and present on a later one. The later value is new evidence. It does not overwrite the earlier observation. Appearance of the field is not a match and not a posting.

**Proposed official evidence type:** official text on when the permanent alias is assigned, plus two raw observations of the same local attempt, one without the field and one with it.

**Research topic:** `UNVERIFIED_RESEARCH_TOPIC`. Time relationship between order acceptance and permanent-alias assignment. Operation name unrecorded.

**Positive test:** both observations remain stored, ordered by local receipt, with the later field appended. Not run. Scenario `IBKR-SCN-A07`.

**Negative test:** the early bytes are still present after the later field arrives. Not run. Same scenario, negative side.

**Environmental prerequisites:** monotonic local receipt timestamps plus wall-clock timestamps. Unrecorded.

**Known ambiguity:** "later" might be broker time, receipt time, or a replay. Receipt order is not evidence priority (DET-05-D04).

**Would not prove:** that every accepted order receives a `permId`, or that the value is unique.

**Decision blockers:** `XC-01` OPEN. ARCH-DEC-13 quarantines any economic reading of a later `permId`.

## IBKR-CLM-06 — permId stability, limits, and standalone non-uniqueness

**Status:** `HYPOTHESIS`. **Evidence:** `EXTERNAL_EVIDENCE_PENDING`. **Execution:** `NOT_EXECUTED`. **Authorization:** `NOT_AUTHORIZED`.

**Internal source anchor:** CR-B04 §4.1. CR-B04 §4.2 `permId` row: stability, scopes, null and special values, cross-account cases. P0-B04-INV-02. DET-03-D01. ARCH-DEC-35.

**Hypothesis:** `permId` may be stable across some later operations and may have format or range limits. Stability, cross-session observation, cross-account scope, and any uniqueness property are unverified. `permId` is not a standalone system identity and not a standalone uniqueness guarantee.

**Proposed official evidence type:** official limits and lifetime text, plus raw observations across modify, cancel, reconnect, and a second account. Modify and cancel are not performed by this plan.

**Research topic:** `UNVERIFIED_RESEARCH_TOPIC`. Lifetime and scope of the permanent alias across sessions and accounts. Operation name unrecorded.

**Positive test:** a repeated observation records the same supplied `permId` text without merging the observations into one order. Not run. Scenario `IBKR-SCN-A07`.

**Negative test:** one `permId` text seen with two conflicting payloads is `CONTRADICTION`, and both originals are kept. Not run. Scenario `IBKR-SCN-A12`.

**Environmental prerequisites:** version-pinned raw payloads. Unrecorded. No uniqueness index exists, and none is approved.

**Known ambiguity:** a limit on length or numeric range is not a uniqueness proof. Cross-account equality is not identity.

**Would not prove:** a global alias key, a UNIQUE index, or permission to book on a single match.

**Decision blockers:** `XC-01` OPEN. `broker_uniqueness_evidence` stays `EVIDENCE_PENDING`.

## IBKR-CLM-07 — execId namespace and duplicates

**Status:** `HYPOTHESIS`. **Evidence:** `EXTERNAL_EVIDENCE_PENDING`. **Execution:** `NOT_EXECUTED`. **Authorization:** `NOT_AUTHORIZED`.

**Internal source anchor:** CR-B04 §4.2 `execId` row. CR-B04 §4.3 step 5. DET-03 `ID-T04`. ID-INV-03. ARCH-DEC-38.

**Hypothesis:** `execId` is a candidate execution alias inside an unverified broker, account, and correction context. An identical raw payload seen again is the same observation seen again and creates no second execution. A different payload under the same apparent key is `CONTRADICTION`. Both original payloads are kept.

**Proposed official evidence type:** official execution-identifier text, plus a replayed identical payload and a differing payload that shares the apparent key.

**Research topic:** `UNVERIFIED_RESEARCH_TOPIC`. Execution-report identity, duplicate delivery, and payload equality. Operation name unrecorded.

**Positive test:** identical replay has no economic effect. Not run. Scenario `IBKR-SCN-A08`.

**Negative test:** differing bytes under one apparent key do not overwrite. Not run. Scenario `IBKR-SCN-A09`.

**Environmental prerequisites:** payload hashes of the raw bytes before normalization. Unrecorded.

**Known ambiguity:** duplicate delivery, correction, and bust can look alike until the payload itself distinguishes them. This claim does not invent the distinction.

**Would not prove:** that `execId` is unique, that a duplicate fills a coverage gap, or that a second booking is allowed.

**Decision blockers:** `XC-01` OPEN. Whether `execId` is stable across corrections is `EXTERNAL_EVIDENCE_PENDING`.

## IBKR-CLM-08 — correction and bust references

**Status:** `HYPOTHESIS`. **Evidence:** `EXTERNAL_EVIDENCE_PENDING`. **Execution:** `NOT_EXECUTED`. **Authorization:** `NOT_AUTHORIZED`.

**Internal source anchor:** DET-03-D05. CR-B04 §4.5 `T06`. CR-B03 §3.2 in the P0 resolution file, which describes a compensating booking that ARCH-DEC-26 does not adopt. ARCH-DEC-39. ARCH-DEC-09. ARCH-DEC-26.

**Hypothesis:** A correction or bust may carry a reference to earlier evidence. A link is stored only when the new evidence itself names that reference. Absence of a reference means no link is invented. The link is evidence. It is not a reversal booking, a release, or a closure change.

**Proposed official evidence type:** a raw correction or bust example that either contains an explicit prior-reference field or demonstrably does not. The example is absent.

**Research topic:** `UNVERIFIED_RESEARCH_TOPIC`. Original-to-correction reference inside an execution correction or bust observation, including whether `execId` is reused. Operation name unrecorded.

**Positive test:** when the new payload names a reference, the original stays and the reference is stored as data. Not run. Scenario `IBKR-SCN-A10`.

**Negative test:** when the new payload names no reference, no link is added. Not run. Scenario `IBKR-SCN-A11`.

**Environmental prerequisites:** append-only retention of both payloads. Unrecorded. No ledger line is written by this plan.

**Known ambiguity:** a human-readable note inside a payload is not automatically a machine reference. What counts as "the evidence itself names a reference" waits for a later decision and for real bytes.

**Would not prove:** a compensating ledger line, a reservation release, or an OMS closure change.

**Decision blockers:** `XC-01` OPEN. `XC-02-PRED` OPEN. CR-B04 T06 stays `NOT_EXECUTED`.

## IBKR-CLM-09 — out-of-order execution delivery

**Status:** `HYPOTHESIS`. **Evidence:** `EXTERNAL_EVIDENCE_PENDING`. **Execution:** `NOT_EXECUTED`. **Authorization:** `NOT_AUTHORIZED`.

**Internal source anchor:** DET-05-D04: receipt order alone is not evidence priority. CR-B04 §4.5 `T10`. CR-B07 test label T05, out-of-order execution. ARCH-DEC-47.

**Hypothesis:** An execution observation can arrive out of the order in which the executions occurred. Arrival order is not priority. The first-received payload and the last-received payload do not win by default. Both are kept, and a comparison is inference.

**Proposed official evidence type:** official delivery-order notes, plus two raw execution payloads with broker timestamps and local receipt timestamps that disagree in order.

**Research topic:** `UNVERIFIED_RESEARCH_TOPIC`. Delivery order of execution observations relative to broker timestamps. Operation name unrecorded. No claim is made that a broker timestamp exists on every message.

**Positive test:** both payloads retained, with receipt order labeled as receipt order. Not run. Scenario `IBKR-SCN-A13`.

**Negative test:** sorting by arrival does not delete or overwrite the other payload. Not run. Same scenario, negative side.

**Environmental prerequisites:** monotonic local clock and wall clock, both recorded. Unrecorded. Clock synchronization method is unrecorded.

**Known ambiguity:** a missing broker timestamp cannot be invented to restore order.

**Would not prove:** a total order of executions, a broker-wins rule, or an economic conclusion.

**Decision blockers:** `XC-01` OPEN. `XC-08-IBKR` OPEN. `XC-02-PRED` OPEN.

## IBKR-CLM-10 — order status, execution observations, and local attempts

**Status:** `HYPOTHESIS`. **Evidence:** `EXTERNAL_EVIDENCE_PENDING`. **Execution:** `NOT_EXECUTED`. **Authorization:** `NOT_AUTHORIZED`.

**Internal source anchor:** DET-08-D03. DET-07 EX-INV-05. CR-B01 §1.3: `ACKNOWLEDGED` is acceptance or assignment evidence and is not economic finality. ARCH-DEC-17. ARCH-DEC-47. ARCH-DEC-13.

**Hypothesis:** An order-status observation, an execution observation, and a local attempt fact are three records. A broker status word, including a word that looks terminal, is not sole economic authority. On a disagreement, both the broker raw observation and the local attempt facts remain. Neither side wins.

**Proposed official evidence type:** paired raw status and execution payloads plus the local attempt label, with the comparison marked as inference.

**Research topic:** `UNVERIFIED_RESEARCH_TOPIC`. Order-status stream versus execution-report stream versus a local attempt row. Operation names unrecorded. Status vocabulary is not adopted from any guessed broker enum.

**Positive test:** a status observation and an execution observation can both be stored for one attempt without one erasing the other. Not run. Scenario `IBKR-SCN-A14`.

**Negative test:** a terminal-looking status does not release a reservation and does not close an order. Not run. Scenario `IBKR-SCN-D04`.

**Environmental prerequisites:** local attempt phase stored before any broker bytes. Unrecorded. No OMS transition is approved.

**Known ambiguity:** status and execution datasets may use different identifier fields. Which fields line up is `EXTERNAL_EVIDENCE_PENDING`.

**Would not prove:** economic finality, an OMS transition, a cancel, or a posting.

**Decision blockers:** `XC-02-PRED` OPEN. `XC-03-MATRIX` OPEN. `XC-08-IBKR` OPEN. ARCH-DEC-17 stays binding.

## IBKR-CLM-11 — request and response scope

**Status:** `HYPOTHESIS`. **Evidence:** `EXTERNAL_EVIDENCE_PENDING`. **Execution:** `NOT_EXECUTED`. **Authorization:** `NOT_AUTHORIZED`.

**Internal source anchor:** CR-B07 `P1-B07-D01` in `contracts/sources/P1-Resolution-Contracts-CR-B05-bis-CR-B09-v1.0.md`. DET-08-D02. ARCH-DEC-40. GATE-P1-07, source status `EXTERNAL_VERIFICATION_REQUIRED`.

**Hypothesis:** A response covers at most the account, client, session, dataset, time window, and environment actually named on the request. Coverage stays `UNKNOWN`. One response is not a global snapshot. No `COMPLETE` claim is made.

**Proposed official evidence type:** official per-dataset notes of filter, account, and time-window behavior, plus the request parameters that were actually sent in a future authorized read.

**Research topic:** `UNVERIFIED_RESEARCH_TOPIC`. Scope of open-order, execution, position, and cash datasets. Dataset names are the internal CR-B07 list. Operation names unrecorded. Which call returns which dataset is not known here.

**Positive test:** the stored record names only the scope fields the request actually carried. Not run. Scenario `IBKR-SCN-B10`.

**Negative test:** a response for one account is not applied to another account. Not run. Scenario `IBKR-SCN-B08`.

**Environmental prerequisites:** account mode and environment label. Unrecorded. Paper mode is not approved by naming it.

**Known ambiguity:** a request that omits a filter might mean "default scope" or "rejected request". Both readings stay open. Default scope is not global coverage.

**Would not prove:** completeness, a release, or a dispatch.

**Decision blockers:** `XC-08-IBKR` OPEN. ARCH-DEC-14 and ARCH-DEC-17 stay binding.

## IBKR-CLM-12 — pagination and end of response

**Status:** `HYPOTHESIS`. **Evidence:** `EXTERNAL_EVIDENCE_PENDING`. **Execution:** `NOT_EXECUTED`. **Authorization:** `NOT_AUTHORIZED`.

**Internal source anchor:** CR-B07 test labels T03 and T04. `P1-B07-D02`. ARCH-DEC-42. DET-04-D05 is about local aggregate versions and is not an IBKR sequence.

**Hypothesis:** Paging and end-of-response signals, if a query type has them, are unknown here. A short page, a missing end marker, or a dropped connection is a locally observed capture gap. The gap is not an IBKR sequence number. No sequence counter is invented. No interpolation is approved.

**Proposed official evidence type:** official completion and paging notes for each query type, plus a raw capture that records page boundaries the transport actually exposed.

**Research topic:** `UNVERIFIED_RESEARCH_TOPIC`. Page continuation and end-of-response marker for open-order and execution queries. Marker name unrecorded.

**Positive test:** a missing end marker is stored as a local capture gap and the scope stays incomplete. Not run. Scenario `IBKR-SCN-B06`.

**Negative test:** the gap marker does not synthesize missing executions and does not start a follow-on action. Not run. Scenario `IBKR-SCN-D02`.

**Environmental prerequisites:** immutable raw capture of each received byte range. Unrecorded.

**Known ambiguity:** a transport that hides page boundaries cannot be given an invented page count.

**Would not prove:** broker-side completeness, a hole in the broker's own sequence, or a backfill send.

**Decision blockers:** `XC-08-IBKR` OPEN. B07-T03 and T04 stay `NOT_EXECUTED`.

## IBKR-CLM-13 — retention, truncation, delay, and gaps

**Status:** `HYPOTHESIS`. **Evidence:** `EXTERNAL_EVIDENCE_PENDING`. **Execution:** `NOT_EXECUTED`. **Authorization:** `NOT_AUTHORIZED`.

**Internal source anchor:** `P1-B07-D03`. DET-08-D02. ARCH-DEC-42. DET-08 recovery step 4: gaps yield `STALE_SNAPSHOT`.

**Hypothesis:** History may be truncated, delayed, or gapped. A late execution can arrive after the window that was requested. Missing historical coverage leaves the scope incomplete. The marker authorizes no backfill and no release.

**Proposed official evidence type:** official retention and delay notes per dataset, plus a raw example of a late execution and of a truncated window. Examples are absent.

**Research topic:** `UNVERIFIED_RESEARCH_TOPIC`. Retention horizon, truncation, and delayed execution delivery for each dataset. Durations unrecorded. No retention number is invented.

**Positive test:** a delayed execution is appended and the earlier window stays incomplete. Not run. Scenario `IBKR-SCN-B05`.

**Negative test:** a truncated window is not closed as if the missing tail were empty. Not run. Scenario `IBKR-SCN-B02`.

**Environmental prerequisites:** request window copied from the request, not inferred from the rows that arrived. Unrecorded.

**Known ambiguity:** delay, retention, and a dropped packet are different causes and can look alike in a short capture.

**Would not prove:** that the broker has no further rows, or that a residual reservation may be released.

**Decision blockers:** `XC-08-IBKR` OPEN. ARCH-DEC-14: an incomplete scope does not release a residual.

## IBKR-CLM-14 — empty response

**Status:** `HYPOTHESIS`. **Evidence:** `EXTERNAL_EVIDENCE_PENDING`. **Execution:** `NOT_EXECUTED`. **Authorization:** `NOT_AUTHORIZED`.

**Internal source anchor:** `P1-B07-D02`. `P1-B07-D04`. RC-INV-04. CR-B01 §1.2 item 7 and §1.3: an empty list is not `DEFINITIVELY_NOT_SENT`. ARCH-DEC-41.

**Hypothesis:** An empty response is an empty captured list for the requested scope. It is not proof that an order does not exist, not finality, not a release, and not proof that a send did not happen.

**Proposed official evidence type:** official text on empty results for each query type, plus a raw empty response kept beside the request scope. The text is not in this repository.

**Research topic:** `UNVERIFIED_RESEARCH_TOPIC`. Meaning of an empty open-order or execution response. Operation name unrecorded.

**Positive test:** the empty list is stored with coverage `UNKNOWN`. Not run. Scenario `IBKR-SCN-B01`.

**Negative test:** the empty list does not move an attempt to `DEFINITIVELY_NOT_SENT` and does not release risk. Not run. Scenario `IBKR-SCN-D03`.

**Environmental prerequisites:** the request scope and the byte length of the response. Unrecorded. A zero-row decode after a short read is a capture gap (IBKR-CLM-12), not an empty business result.

**Known ambiguity:** empty because the filter matched nothing, empty because the call failed, and empty because the page was truncated are different. This claim does not pick one.

**Would not prove:** non-existence, terminal acceptance, or a resubmit.

**Decision blockers:** `XC-08-IBKR` OPEN. `XC-04` operational remainder OPEN. `XC-02-PRED` OPEN.

## IBKR-CLM-15 — no completeness token

**Status:** `HYPOTHESIS`. **Evidence:** `EXTERNAL_EVIDENCE_PENDING`. **Execution:** `NOT_EXECUTED`. **Authorization:** `NOT_AUTHORIZED`.

**Internal source anchor:** ARCH-DEC-41, which is BROKERQ-07 option A. `P1-B07-D02`. ARCH-DEC-28. ARCH-DEC-29.

**Hypothesis:** No completeness token is stored. No `COMPLETE` marker is ratified. No external token placeholder stands in for evidence. A future completeness criterion would be a separate decision and is not made here. Writing this plan does not create that criterion.

**Proposed official evidence type:** a documented completion condition per query type, with negative cases, before any later claim. Those cases are `NOT_EXECUTED`. Until then the token stays absent.

**Research topic:** `UNVERIFIED_RESEARCH_TOPIC`. Whether any query type exposes a completion signal. Signal name unrecorded. Presence of a signal in a future document would still not ratify storing a token under ARCH-DEC-41.

**Positive test:** a capture record has no completeness-token field. Not run. Scenario `IBKR-SCN-D03`.

**Negative test:** an empty response does not become a token. Not run. Scenario `IBKR-SCN-B01`.

**Environmental prerequisites:** a capture layout that omits the token. The proposed layout is in the capture note and is not a schema.

**Known ambiguity:** a broker field that looks like "done" is still not a ratified token.

**Would not prove:** coverage, release, or permission to close `XC-08-IBKR`.

**Decision blockers:** `XC-08-IBKR` OPEN. `XC-02-PRED` OPEN. ARCH-DEC-41 stays binding.

## IBKR-CLM-16 — reconnect is a new session context

**Status:** `HYPOTHESIS`. **Evidence:** `EXTERNAL_EVIDENCE_PENDING`. **Execution:** `NOT_EXECUTED`. **Authorization:** `NOT_AUTHORIZED`.

**Internal source anchor:** ARCH-DEC-43. DET-05-D05. CR-B01 §1.2 item 8. DET-03 recovery paragraph. BA-INV-03.

**Hypothesis:** A reconnect creates a new session context. Prior evidence is retained. Broker identifiers are not carried across the boundary unless a retained binding is evidenced. A binding is still not a posting and not a resubmit. Reconnect does not prove that the previous send did not happen.

**Proposed official evidence type:** official session-lifetime text, plus raw session identifiers from before and after a future reconnect. Identifier field names are taken from the payload, not invented here.

**Research topic:** `UNVERIFIED_RESEARCH_TOPIC`. Session context after reconnect, including which aliases remain visible. Operation name unrecorded.

**Positive test:** old evidence remains addressable beside the new session label. Not run. Scenario `IBKR-SCN-B07`.

**Negative test:** the reconnect is not used as a negative send proof and does not authorize a retry. Not run. Scenario `IBKR-SCN-C06`.

**Environmental prerequisites:** distinct session labels for the two connections. Unrecorded. This plan opens no connection.

**Known ambiguity:** a client restart, a socket drop, and a gateway restart are three events. IBKR-CLM-17 covers gateway and TWS restart as its own claim.

**Would not prove:** takeover, external fencing, or `DEFINITIVELY_NOT_SENT`.

**Decision blockers:** `XC-01` OPEN. Operational remainder of `XC-04` OPEN. BA-T02 and RC-T10 stay `NOT_EXECUTED`.

## IBKR-CLM-17 — gateway or TWS restart

**Status:** `HYPOTHESIS`. **Evidence:** `EXTERNAL_EVIDENCE_PENDING`. **Execution:** `NOT_EXECUTED`. **Authorization:** `NOT_AUTHORIZED`.

**Internal source anchor:** CR-B04 §4.5 `T03`. CR-B01 §1.5 test label `T05`. ARCH-DEC-43. CR-B01 §1.3: restart is not `DEFINITIVELY_NOT_SENT`.

**Hypothesis:** A gateway or TWS restart may change visibility and alias binding. What survives is unverified. A restart is not proof that an in-flight send was not accepted. Client-id change across the restart is a separate unverified question.

**Proposed official evidence type:** official restart behavior for the pinned gateway or TWS build, plus raw observations immediately before the stop and after the new process is up.

**Research topic:** `UNVERIFIED_RESEARCH_TOPIC`. Alias visibility and client context across a gateway or TWS process restart. Operation name unrecorded. This plan does not start or stop a gateway.

**Positive test:** pre-restart evidence is retained and labeled with the old session. Not run. Scenario `IBKR-SCN-C07`.

**Negative test:** the restart does not authorize a second send of an attempt that had reached `DISPATCH_STARTED`. Not run. Same scenario, negative side.

**Environmental prerequisites:** process identity of the gateway or TWS, version string, and stop and start times from the future package. Unrecorded.

**Known ambiguity:** TWS and IB Gateway may differ. Platform and version may differ. IBKR-CLM-26 requires revalidation when they differ.

**Would not prove:** that the old socket can no longer send, or that open orders were cancelled by the restart.

**Decision blockers:** `XC-01` OPEN. Operational remainder of `XC-04` OPEN.

## IBKR-CLM-18 — disconnect, missing executions, partial fills, and late fills

**Status:** `HYPOTHESIS`. **Evidence:** `EXTERNAL_EVIDENCE_PENDING`. **Execution:** `NOT_EXECUTED`. **Authorization:** `NOT_AUTHORIZED`.

**Internal source anchor:** DET-05 test `BA-T01`. CR-B07 test labels T02 and T05. DET-08 test `RC-T03`. CR-B03 §3.1, read under ARCH-DEC-26 so the economic effect stays quarantined. ARCH-DEC-42.

**Hypothesis:** A disconnect during a read can omit executions from the local capture. A partial fill and a late fill can arrive after the disconnect. Absence from the captured stream is not absence at the broker. A late fill does not by itself post, release, or rewrite a closure.

**Proposed official evidence type:** raw capture that ends mid-stream, plus a later raw execution payload. Both absent.

**Research topic:** `UNVERIFIED_RESEARCH_TOPIC`. Behavior of an execution stream when the connection drops, and delivery of a further fill after reconnect. Operation name unrecorded.

**Positive test:** the dropped capture is a local gap, and a later fill is appended. Not run. Scenario `IBKR-SCN-C12`.

**Negative test:** the gap does not become an interpolated fill, and the late fill does not release the residual reservation. Not run. Scenario `IBKR-SCN-B04`.

**Environmental prerequisites:** connection-lifecycle log without secrets, and the byte offset where the read stopped. Unrecorded.

**Known ambiguity:** a partial fill, a correction, and a duplicate can share surface fields. IBKR-CLM-07 and IBKR-CLM-08 stay separate claims.

**Would not prove:** the full fill quantity, a position, or a risk release.

**Decision blockers:** `XC-08-IBKR` OPEN. `XC-04` operational remainder OPEN. `XC-02-PRED` OPEN.

## IBKR-CLM-19 — cancel races and conflicting snapshots

**Status:** `HYPOTHESIS`. **Evidence:** `EXTERNAL_EVIDENCE_PENDING`. **Execution:** `NOT_EXECUTED`. **Authorization:** `NOT_AUTHORIZED`.

**Internal source anchor:** DET-07 test `EX-T07`. DET-08-D03. `P1-B09-D02`, which says a cancel request is not a cancel confirmation. ARCH-DEC-34 blocks modify and does not authorize cancel during halt as an implementation. ARCH-DEC-47. This plan authorizes no cancel.

**Hypothesis:** A cancel observation, a fill observation, and a local attempt can disagree, including across two snapshots. Both raw sides are kept. A comparison is inference. Neither side wins. No cancel is sent to produce the disagreement.

**Proposed official evidence type:** paired raw snapshots that disagree, obtained later only under a separate authorization. Not obtained.

**Research topic:** `UNVERIFIED_RESEARCH_TOPIC`. Concurrent cancel-state and fill-state observations, and two snapshots of the same requested scope that disagree. Operation names unrecorded.

**Positive test:** both snapshots retained and labeled as inference if compared. Not run. Scenario `IBKR-SCN-D04`.

**Negative test:** the disagreement does not select a winner and does not send a cancel. Not run. Same scenario, negative side.

**Environmental prerequisites:** two request scopes that are explicit about account, session, and window. Unrecorded.

**Known ambiguity:** a stale cache (IBKR-CLM-13) and a true broker-side conflict can look alike. The plan does not pick a winner to resolve that ambiguity.

**Would not prove:** that cancel is safe, that a broker status is terminal, or that a position matches.

**Decision blockers:** `XC-10-ACTION` OPEN. `XC-08-IBKR` OPEN. `XC-02-PRED` OPEN.

## IBKR-CLM-20 — unknown send before DISPATCH_STARTED

**Status:** `HYPOTHESIS`. **Evidence:** `EXTERNAL_EVIDENCE_PENDING`. **Execution:** `NOT_EXECUTED`. **Authorization:** `NOT_AUTHORIZED`.

**Internal source anchor:** CR-B01 §1.4 row "Nach `PREPARED`, vor `DISPATCH_STARTED`". DET-07 test `EX-T01`. CR-B01 §1.3: `PREPARED` and `CLAIMED` move to `DEFINITIVELY_NOT_SENT` only when non-send is evidenced. ARCH-DEC-44. ARCH-DEC-11.

**Hypothesis:** Before `DISPATCH_STARTED`, a later claim is available only when non-send is positively evidenced. This plan does not produce that evidence. Timeout and an empty broker list are not that evidence. No new claim is authorized.

**Proposed official evidence type:** a future local durability record showing the attempt never reached `DISPATCH_STARTED`, plus an argument that no socket write was possible. The broker side of that argument is `EXTERNAL_EVIDENCE_PENDING`.

**Research topic:** `UNVERIFIED_RESEARCH_TOPIC`. What, if anything, a broker observation can add to a local proof that a socket write had not started. Operation name unrecorded. CR-B01 still treats the phase as a local attempt fact first.

**Positive test:** a fixture whose phase is `PREPARED` or `CLAIMED` gains no send. Not run. Scenario `IBKR-SCN-C01`.

**Negative test:** lack of a broker row does not by itself become `DEFINITIVELY_NOT_SENT`. Not run. Same scenario, negative side.

**Environmental prerequisites:** attempt phase stored in the fixture. Unrecorded. No attempt row is written by this plan.

**Known ambiguity:** a crash in this window and a crash after `DISPATCH_STARTED` are different. IBKR-CLM-21 covers the later window. Mixing them is a plan failure.

**Would not prove:** that a process may retry, or that the broker has no order.

**Decision blockers:** operational remainder of `XC-04` OPEN. ARCH-DEC-03 and ARCH-DEC-11 stay binding.

## IBKR-CLM-21 — unknown send after DISPATCH_STARTED

**Status:** `HYPOTHESIS`. **Evidence:** `EXTERNAL_EVIDENCE_PENDING`. **Execution:** `NOT_EXECUTED`. **Authorization:** `NOT_AUTHORIZED`.

**Internal source anchor:** CR-B01 §1.2 item 4. CR-B01 §1.3. CR-B01 §1.4 row "Nach `DISPATCH_STARTED`, vor Socket-Write". DET-07-D03. EX-INV-02. ARCH-DEC-44. ARCH-DEC-03. ARCH-DEC-34.

**Hypothesis:** After `DISPATCH_STARTED`, including before a socket write, the attempt is possibly sent. Timeout, an empty list, and restart are not counter-evidence. No window in this package authorizes resend, cancel, release, or dispatch. CR-B01 keeps precedence where DET-07 text is less restrictive.

**Proposed official evidence type:** positive evidence, sufficient under a future decision, that the broker did not accept the attempt. Absent. Until that evidence exists the attempt stays possibly sent.

**Research topic:** `UNVERIFIED_RESEARCH_TOPIC`. Broker-side acceptance evidence that could, in a later decision, count as the positive counter-proof CR-B01 requires. Operation name unrecorded. This plan does not define that proof as met.

**Positive test:** a fixture in `DISPATCH_STARTED` stays possibly sent through timeout, empty list, and restart labels. Not run. Scenarios `IBKR-SCN-C02` and `IBKR-SCN-C03`.

**Negative test:** no second send is labeled legal. Not run. Scenario `IBKR-SCN-D06`.

**Environmental prerequisites:** the phase label committed before any network step in the future harness. The harness does not exist.

**Known ambiguity:** "before socket write" cannot be observed from the broker. The local rule is still "possibly sent".

**Would not prove:** a safe retry, a cancel, or economic finality. `ACKNOWLEDGED` would still not be economic finality.

**Decision blockers:** operational remainder of `XC-04` OPEN. ARCH-DEC-31 marks fault-injection tests `NOT_EXECUTED`.

## IBKR-CLM-22 — lost acknowledgement and crash after send

**Status:** `HYPOTHESIS`. **Evidence:** `EXTERNAL_EVIDENCE_PENDING`. **Execution:** `NOT_EXECUTED`. **Authorization:** `NOT_AUTHORIZED`.

**Internal source anchor:** DET-08 test `RC-T01`. DET-07 test `EX-T03`. CR-B01 §1.4 row "Nach Brokerannahme, vor Ack-Persistenz". BA-INV-03. RC-INV-01. ARCH-DEC-44.

**Hypothesis:** A lost acknowledgement, or a crash after a possible send and before the acknowledgement is stored, leaves the outcome unknown. Recovery does not retry. A later broker observation may be linked only as evidence under the identity claims, and that link is not a new send.

**Proposed official evidence type:** a raw acceptance or execution observation that arrives without a stored local acknowledgement, kept beside the attempt. Absent.

**Research topic:** `UNVERIFIED_RESEARCH_TOPIC`. Observations that can show acceptance when the local acknowledgement row was not stored. Operation name unrecorded.

**Positive test:** the attempt label stays `OUTCOME_UNKNOWN` when the acknowledgement is missing. Not run. Scenario `IBKR-SCN-C04`.

**Negative test:** the crash window does not authorize a second submit. Not run. Scenario `IBKR-SCN-C05`.

**Environmental prerequisites:** crash injection isolated from any broker socket. Unrecorded. This plan injects nothing.

**Known ambiguity:** lost on the wire, lost in the process, and never sent can look the same after a crash. The conservative label is possibly sent once `DISPATCH_STARTED` was committed.

**Would not prove:** that the broker accepted the order, or that it did not.

**Decision blockers:** operational remainder of `XC-04` OPEN. `XC-01` OPEN for any later link.

## IBKR-CLM-23 — prior writer, lease expiry, and split brain

**Status:** `HYPOTHESIS`. **Evidence:** `EXTERNAL_EVIDENCE_PENDING`. **Execution:** `NOT_EXECUTED`. **Authorization:** `NOT_AUTHORIZED`.

**Internal source anchor:** CR-B01 §1.1 `P0-B01-D01`. CR-B01 §1.2 items 6 and 8. CR-B01 §1.4 last row. P0-B01-INV-02. DET-07 fencing-limit paragraph, subordinate to CR-B01. ARCH-DEC-45.

**Hypothesis:** A successor stays blocked while an earlier writer might still send or the send status is unknown. Lease expiry is not proof of safe takeover. A newer local epoch is not proof that the broker rejected the old writer. Split brain stays `UNCERTAIN`. No automatic failover is approved.

**Proposed official evidence type:** an isolation record that meets IBKR-CLM-24, plus the attempt inventory. Absent. A lease deadline timestamp by itself is not that record.

**Research topic:** `UNVERIFIED_RESEARCH_TOPIC`. Whether a broker session can still accept a send from a writer the database considers expired. Operation name unrecorded. No TTL is chosen.

**Positive test:** successor send stays blocked in a fixture where the prior writer is labeled possibly active. Not run. Scenario `IBKR-SCN-C08`.

**Negative test:** an expired lease label does not flip the successor to active. Not run. Scenario `IBKR-SCN-C09`.

**Environmental prerequisites:** separate local fence facts and broker observations. Unrecorded. No lease implementation is approved.

**Known ambiguity:** "might still send" includes a live socket, a paused process, and an unknown process. All three block the successor.

**Would not prove:** external fencing, a lock order, or a heartbeat.

**Decision blockers:** operational remainder of `XC-04` OPEN. `XC-05-LOCK` OPEN. ARCH-DEC-18 leaves the lock order unadopted.

## IBKR-CLM-24 — session isolation is external evidence

**Status:** `HYPOTHESIS`. **Evidence:** `EXTERNAL_EVIDENCE_PENDING`. **Execution:** `NOT_EXECUTED`. **Authorization:** `NOT_AUTHORIZED`.

**Internal source anchor:** CR-B01 §1.2 item 6: the prior sender is demonstrably isolated (process or container stopped, and the broker connection closed or session exclusivity reliably confirmed), and every `DISPATCH_STARTED` or `OUTCOME_UNKNOWN` attempt is in reconciliation. P0-B01-INV-02. CR-B01 operational constraint: a network kill alone does not prove a submitted order was not accepted. ARCH-DEC-45.

**Hypothesis:** Externally evidenced isolation is a separate fact from a local epoch. It requires evidence that the prior sender is stopped and that the broker connection is closed, or a reliable confirmation of session exclusivity, plus reconciliation of unknown attempts. A local label does not meet that bar. This plan holds no such evidence.

**Proposed official evidence type:** process-stop evidence, connection-closed evidence, and a version-specific note of what session exclusivity means for the pinned gateway or TWS. All absent.

**Research topic:** `UNVERIFIED_RESEARCH_TOPIC`. Session exclusivity for one gateway or TWS deployment scope. Operation name unrecorded. Topology is an open CR-B01 question, not a measured fact.

**Positive test:** a fixture without stop evidence and without connection-closed evidence keeps the successor blocked. Not run. Scenario `IBKR-SCN-C10`.

**Negative test:** killing a description of the network in a document does not count as isolation. Not run. The negative control is the absence of those artifacts.

**Environmental prerequisites:** a future dedicated test account and an isolated gateway scope, each authorized separately. Not authorized here.

**Known ambiguity:** "session exclusivity reliably confirmed" has no operational test in this repository. Inventing a confirmation phrase would not confirm it.

**Would not prove:** that a new writer may send, or that unknown attempts are economically closed.

**Decision blockers:** operational remainder of `XC-04` OPEN. `XC-10-ACTION` OPEN.

## IBKR-CLM-25 — WriterEpoch, adapter session epoch, acceptance, and isolation

**Status:** `HYPOTHESIS`. **Evidence:** `EXTERNAL_EVIDENCE_PENDING`. **Execution:** `NOT_EXECUTED`. **Authorization:** `NOT_AUTHORIZED`.

**Internal source anchor:** DET-07 fencing-limit paragraph. CR-B01 §1.1: a PostgreSQL lease is not external broker fencing. Glossary `WriterEpoch` in `contracts/domain/glossary.md`: the shell counter is not the database fence. ARCH-DEC-43: adapter session epoch is not `WriterEpoch`. ARCH-DEC-45.

**Hypothesis:** Four facts stay distinct. `WriterEpoch` is a local fencing counter. Adapter session epoch is a connection-generation label. Broker acceptance is a raw observation. Externally evidenced isolation is the CR-B01 isolation record. None of them substitutes for another. A newer `WriterEpoch` is not broker acceptance and is not isolation.

**Proposed official evidence type:** four separately labeled artifacts in a future package. This plan stores none of them.

**Research topic:** `UNVERIFIED_RESEARCH_TOPIC`. Which broker observations, if any, correspond to acceptance rather than to a local epoch. Operation name unrecorded.

**Positive test:** a fixture can hold all four labels at once without equating them. Not run. Scenario `IBKR-SCN-C11`.

**Negative test:** advancing the local epoch does not mark the old writer isolated and does not mark an order accepted. Not run. Same scenario, negative side.

**Environmental prerequisites:** the glossary distinction preserved in the capture labels. No runtime type is added.

**Known ambiguity:** DET-07-D01 says one writer per dispatch scope and that the scope is defined at cross-contract review. The scope boundary stays inside the open remainder of `XC-04`.

**Would not prove:** a single-writer implementation, a TTL, or a broker-side fence.

**Decision blockers:** operational remainder of `XC-04` OPEN. `XC-05-LOCK` OPEN.

## IBKR-CLM-26 — version, platform, and revalidation

**Status:** `HYPOTHESIS`. **Evidence:** `EXTERNAL_EVIDENCE_PENDING`. **Execution:** `NOT_EXECUTED`. **Authorization:** `NOT_AUTHORIZED`.

**Internal source anchor:** CR-B04 §4.1: uniqueness, stability, null semantics, and scope must be verified per TWS or Gateway API version. CR-B04 §4.5: the test report is for the concrete TWS or Gateway version, API library, and configuration. ARCH-DEC-48. GATE-P1-07.

**Hypothesis:** A future evidence package is scoped to the product, API or client library, platform, OS, and account mode recorded with the raw payload. A change of any of those invalidates prior external claims until a new package is obtained and adjudicated. This repository has no package and no version string. No version is guessed.

**Proposed official evidence type:** version metadata copied from the obtained official material and from the running components in a future authorized environment. Not copied here.

**Research topic:** `UNVERIFIED_RESEARCH_TOPIC`. Where the pinned product prints its version, and which behavioral notes are tied to that version. Location unrecorded. No URL is invented.

**Positive test:** two captures with different version tuples stay two packages. Not run. Scenario `IBKR-SCN-B11`.

**Negative test:** a pass on one tuple is not written onto another tuple. Not run. Same scenario, negative side.

**Environmental prerequisites:** the E0 bibliography and the E2 or E3 environment record. Neither exists. Stage checkpoints are STOP.

**Known ambiguity:** a version string can stay the same while behavior changes. Revalidation cadence is `IBKREVQ-12` and is `USER_DECISION_PENDING`.

**Would not prove:** that the next build behaves the same, or that live trading is authorized.

**Decision blockers:** `XC-01` OPEN. `XC-08-IBKR` OPEN. ARCH-DEC-48 stays `NOT_EXECUTED`.

## IBKR-CLM-27 — a singleton candidate is not authorization

**Status:** `HYPOTHESIS`. **Evidence:** `EXTERNAL_EVIDENCE_PENDING`. **Execution:** `NOT_EXECUTED`. **Authorization:** `NOT_AUTHORIZED`.

**Internal source anchor:** DET-03 formal rule for `A(e)`. ID-INV-02. CR-B04 §4.3 steps 3 and 4, subordinate to ARCH-DEC-13 and ARCH-DEC-26. ARCH-DEC-36.

**Hypothesis:** A future record may keep the candidate set `A(e)` and an uncertainty label. Size one is `SINGLE_CANDIDATE_NOT_AUTHORIZATION`. Size zero, many, or a contradiction authorizes nothing either. No automatic assignment with economic effect is approved.

**Proposed official evidence type:** none for the label itself. Any later promotion of a candidate toward a verified match needs `XC-01` evidence that is absent.

**Research topic:** `UNVERIFIED_RESEARCH_TOPIC`. Not an API call. The claim is a policy constraint on how future observations may be labeled.

**Positive test:** a fixture with one candidate stores the singleton label and posts nothing. Not run. Scenario `IBKR-SCN-D01`.

**Negative test:** the singleton label does not become `MATCHED` with an economic effect. Not run. Same scenario, negative side.

**Environmental prerequisites:** synthetic candidates labeled `SYNTHETIC_NOT_EXTERNAL`. No resolver is implemented.

**Known ambiguity:** DET-03-D04 says only `MATCHED` may book. That sentence stays preserved and is not followed.

**Would not prove:** a resolver, a booking, or closure of `XC-01`.

**Decision blockers:** `XC-01` OPEN. `XC-02-PRED` OPEN. ARCH-DEC-13 and ARCH-DEC-26 stay binding.

## Claim index

| Claim | Topic | Primary blocker | Execution |
| --- | --- | --- | --- |
| IBKR-CLM-01 | `orderId` namespace and placement context | XC-01 | `NOT_EXECUTED` |
| IBKR-CLM-02 | `orderId` reuse across clients and sessions | XC-01 | `NOT_EXECUTED` |
| IBKR-CLM-03 | `orderId` across reconnect and restart | XC-01, XC-04 remainder | `NOT_EXECUTED` |
| IBKR-CLM-04 | `permId` absent, null, and zero | XC-01 | `NOT_EXECUTED` |
| IBKR-CLM-05 | `permId` assignment timing | XC-01 | `NOT_EXECUTED` |
| IBKR-CLM-06 | `permId` stability and non-uniqueness | XC-01 | `NOT_EXECUTED` |
| IBKR-CLM-07 | `execId` namespace and duplicates | XC-01 | `NOT_EXECUTED` |
| IBKR-CLM-08 | correction and bust references | XC-01, XC-02-PRED | `NOT_EXECUTED` |
| IBKR-CLM-09 | out-of-order execution delivery | XC-01, XC-08-IBKR | `NOT_EXECUTED` |
| IBKR-CLM-10 | status, execution, and local attempt | XC-02-PRED, XC-03-MATRIX | `NOT_EXECUTED` |
| IBKR-CLM-11 | request and response scope | XC-08-IBKR | `NOT_EXECUTED` |
| IBKR-CLM-12 | pagination and end of response | XC-08-IBKR | `NOT_EXECUTED` |
| IBKR-CLM-13 | retention, truncation, delay, and gaps | XC-08-IBKR | `NOT_EXECUTED` |
| IBKR-CLM-14 | empty response | XC-08-IBKR, XC-04 remainder | `NOT_EXECUTED` |
| IBKR-CLM-15 | no completeness token | XC-08-IBKR | `NOT_EXECUTED` |
| IBKR-CLM-16 | reconnect session context | XC-01, XC-04 remainder | `NOT_EXECUTED` |
| IBKR-CLM-17 | gateway or TWS restart | XC-01, XC-04 remainder | `NOT_EXECUTED` |
| IBKR-CLM-18 | disconnect, partial fill, late fill | XC-08-IBKR, XC-04 remainder | `NOT_EXECUTED` |
| IBKR-CLM-19 | cancel races and conflicting snapshots | XC-10-ACTION, XC-08-IBKR | `NOT_EXECUTED` |
| IBKR-CLM-20 | unknown send before `DISPATCH_STARTED` | XC-04 remainder | `NOT_EXECUTED` |
| IBKR-CLM-21 | unknown send after `DISPATCH_STARTED` | XC-04 remainder | `NOT_EXECUTED` |
| IBKR-CLM-22 | lost acknowledgement and crash after send | XC-04 remainder | `NOT_EXECUTED` |
| IBKR-CLM-23 | prior writer, lease expiry, split brain | XC-04 remainder | `NOT_EXECUTED` |
| IBKR-CLM-24 | session isolation as external evidence | XC-04 remainder | `NOT_EXECUTED` |
| IBKR-CLM-25 | four distinct epoch and acceptance facts | XC-04 remainder | `NOT_EXECUTED` |
| IBKR-CLM-26 | version and revalidation | XC-01, XC-08-IBKR | `NOT_EXECUTED` |
| IBKR-CLM-27 | singleton candidate is not authorization | XC-01 | `NOT_EXECUTED` |

Count: 27 claims. Each is `HYPOTHESIS`, `EXTERNAL_EVIDENCE_PENDING`, `NOT_EXECUTED`, and `NOT_AUTHORIZED`. No uniqueness assertion is made. No broker behavior is verified.
