# TRD-ARCH-03B.4 — IBKR test matrix

**Date:** 2026-11-15  
**Role:** scenario register. `document_role` = `NON_RUNTIME_NON_WIRE_VERIFICATION_PLAN`.  
**Execution:** every row is `NOT_EXECUTED`.  
**External evidence:** every row is `EXTERNAL_EVIDENCE_PENDING`.  
**Authorization:** `NOT_AUTHORIZED`. No row is a result.

Future action types are `offline`, `read-only`, or `controlled`. They name the stage a later authorization would have to open. They are not permission to run the stage. An `offline` row uses a synthetic fixture labeled `SYNTHETIC_NOT_EXTERNAL` and proves no external behavior. A `read-only` row is still a broker request and stays unauthorized. A `controlled` row includes any order-related or fault-injection experiment and stays unauthorized. Paper mode is not approved.

No expected cell below is an observed result. Oracle limits say what a future pass would still not prove.

Count: 43 scenarios. Identity 14, snapshots 11, unknown send and failover 12, negative assertions 6.

## Identity

### IBKR-SCN-A01

- **Scenario ID:** IBKR-SCN-A01
- **Title:** Reused `orderId` in another session
- **Claim IDs:** IBKR-CLM-02, IBKR-CLM-03
- **Internal source anchors:** DET-03 `ID-T01`; DET-03 recovery paragraph; CR-B04 §4.2 `orderId` row; ARCH-DEC-43
- **Setup prerequisite:** two session labels on one synthetic fixture, then a future controlled pair only after a separate authorization. No broker is contacted.
- **Future action type:** controlled. `NOT_EXECUTED`.
- **Positive expectation:** the same `orderId` text can sit in two session contexts as two observations.
- **Negative expectation:** the second observation does not merge into the first.
- **Data to capture:** both raw payloads, both session labels, client, account, environment, clocks, payload hashes. Values `UNRECORDED`.
- **Oracle limits:** a synthetic pass does not show IBKR reuse rules. A future controlled pass would still not prove uniqueness.
- **Failure classification:** `ORACLE_INSUFFICIENT` when session context is missing. `CONTRADICTION` when payloads disagree under one assumed key.
- **Linked XC finding:** XC-01 OPEN, `EVIDENCE_PENDING`.
- **Safety prohibition:** no order send, cancel, modify, replace, retry, resubmit, takeover, posting, release, or dispatch.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-A02

- **Scenario ID:** IBKR-SCN-A02
- **Title:** Same `orderId` text seen by two clients
- **Claim IDs:** IBKR-CLM-01, IBKR-CLM-02
- **Internal source anchors:** CR-B04 §4.2 `orderId` and `clientId` rows; CR-B04 §4.5 `T07`; DET-03-D02; ARCH-DEC-35
- **Setup prerequisite:** two client labels. The second client is not connected.
- **Future action type:** controlled. `NOT_EXECUTED`.
- **Positive expectation:** each observation keeps the client label that was actually supplied.
- **Negative expectation:** equal `orderId` text across clients is not a merge and not a match.
- **Data to capture:** raw payloads and supplied client labels. `UNRECORDED`.
- **Oracle limits:** visibility across clients stays unverified. The row does not prove a global namespace.
- **Failure classification:** `ORACLE_INSUFFICIENT` when the client label is absent and then invented.
- **Linked XC finding:** XC-01 OPEN.
- **Safety prohibition:** no second login, no order, no posting.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-A03

- **Scenario ID:** IBKR-SCN-A03
- **Title:** Same `orderId` text on two accounts
- **Claim IDs:** IBKR-CLM-01, IBKR-CLM-02
- **Internal source anchors:** CR-B04 §4.2 `account` row; DET-03-D02; ARCH-DEC-35; UD-02
- **Setup prerequisite:** two account labels supplied by the fixture. No account is accessed.
- **Future action type:** controlled. `NOT_EXECUTED`.
- **Positive expectation:** account scope stays on the observation that carried it.
- **Negative expectation:** one account's alias is not applied to the other account.
- **Data to capture:** account labels as supplied, raw payloads. `UNRECORDED`.
- **Oracle limits:** multi-account behavior is `EXTERNAL_EVIDENCE_PENDING`. No advisor topology is asserted.
- **Failure classification:** `CONTRADICTION` if one payload is copied onto the other account.
- **Linked XC finding:** XC-01 OPEN.
- **Safety prohibition:** no account access, no order, no posting.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-A04

- **Scenario ID:** IBKR-SCN-A04
- **Title:** Absent `permId` is retained
- **Claim IDs:** IBKR-CLM-04
- **Internal source anchors:** ARCH-DEC-37; CR-B04 §4.5 `T10`; UD-02; glossary `BrokerPermIdAlias`
- **Setup prerequisite:** a synthetic observation with the `permId` field omitted, labeled `SYNTHETIC_NOT_EXTERNAL`.
- **Future action type:** offline. `NOT_EXECUTED`.
- **Positive expectation:** the observation is kept and the field stays absent.
- **Negative expectation:** absence is not rewritten as zero and not dropped.
- **Data to capture:** the raw fixture bytes and an explicit absence marker that is not a numeric zero.
- **Oracle limits:** the fixture does not show how IBKR encodes absence.
- **Failure classification:** `CONTRADICTION` if absence is stored as zero.
- **Linked XC finding:** XC-01 OPEN.
- **Safety prohibition:** no broker read and no booking when the field is missing.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-A05

- **Scenario ID:** IBKR-SCN-A05
- **Title:** Null `permId` is not zero
- **Claim IDs:** IBKR-CLM-04
- **Internal source anchors:** ARCH-DEC-37; CR-B04 §4.2 `permId` row; CR-B04 §4.5 `T10`
- **Setup prerequisite:** a synthetic payload whose null is distinct from a missing field and from zero. The broker's real encoding is unknown.
- **Future action type:** offline. `NOT_EXECUTED`.
- **Positive expectation:** null stays null on the stored observation.
- **Negative expectation:** null is not equated with zero or with absence.
- **Data to capture:** raw token for null as it appeared in the fixture. `UNRECORDED` for any real payload.
- **Oracle limits:** a fixture null is not an IBKR null. External null semantics stay pending.
- **Failure classification:** `CONTRADICTION` on silent coercion.
- **Linked XC finding:** XC-01 OPEN.
- **Safety prohibition:** no match of nulls, no posting.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-A06

- **Scenario ID:** IBKR-SCN-A06
- **Title:** Zero `permId` is not missing
- **Claim IDs:** IBKR-CLM-04, IBKR-CLM-06
- **Internal source anchors:** ARCH-DEC-37; P0-B04-INV-02; CR-B04 §4.5 `T10`
- **Setup prerequisite:** a synthetic payload that contains a zero `permId` value. This row does not assert that IBKR uses zero.
- **Future action type:** offline. `NOT_EXECUTED`.
- **Positive expectation:** zero is stored as the supplied zero and not as absence.
- **Negative expectation:** all zeros are not matched to one another as one order.
- **Data to capture:** the raw zero field and the session scope. `UNRECORDED`.
- **Oracle limits:** the row does not prove that zero means unset, or that zero is unique.
- **Failure classification:** `ORACLE_INSUFFICIENT` if the row is read as a broker guarantee.
- **Linked XC finding:** XC-01 OPEN.
- **Safety prohibition:** no uniqueness constraint and no posting.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-A07

- **Scenario ID:** IBKR-SCN-A07
- **Title:** Late `permId` is appended
- **Claim IDs:** IBKR-CLM-05, IBKR-CLM-06
- **Internal source anchors:** ARCH-DEC-37; DET-03-D01; DET-05-D04; ARCH-DEC-13
- **Setup prerequisite:** two synthetic observations, the first without `permId` and the second with a supplied value. Receipt order is labeled as receipt order.
- **Future action type:** offline. `NOT_EXECUTED`.
- **Positive expectation:** the later value is new evidence beside the early observation.
- **Negative expectation:** the early bytes are not overwritten and the new field does not post.
- **Data to capture:** both payloads, monotonic receipt times, wall-clock times. `UNRECORDED`.
- **Oracle limits:** the fixture does not show when a broker assigns `permId`.
- **Failure classification:** `CONTRADICTION` on overwrite. `ORACLE_INSUFFICIENT` if receipt order is treated as broker order.
- **Linked XC finding:** XC-01 OPEN. Economic reading also leaves XC-02-PRED OPEN.
- **Safety prohibition:** no overwrite, no booking, no release.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-A08

- **Scenario ID:** IBKR-SCN-A08
- **Title:** Duplicate `execId` with identical raw payload
- **Claim IDs:** IBKR-CLM-07
- **Internal source anchors:** ARCH-DEC-38; CR-B04 §4.3 step 5; DET-03 `ID-T04`; ID-INV-03; ARCH-DEC-16
- **Setup prerequisite:** one synthetic payload delivered twice, byte-identical, labeled `SYNTHETIC_NOT_EXTERNAL`.
- **Future action type:** offline. `NOT_EXECUTED`.
- **Positive expectation:** the second delivery is the same observation seen again.
- **Negative expectation:** no second execution and no economic effect are recorded.
- **Data to capture:** both payload hashes, which must match, and both receipt times.
- **Oracle limits:** identical replay in a fixture is not proof that IBKR deduplicates, and it does not fill a coverage gap.
- **Failure classification:** `CONTRADICTION` if the second copy is booked.
- **Linked XC finding:** XC-01 OPEN.
- **Safety prohibition:** no second booking, no dispatch.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-A09

- **Scenario ID:** IBKR-SCN-A09
- **Title:** Duplicate `execId` with a different payload
- **Claim IDs:** IBKR-CLM-07
- **Internal source anchors:** ARCH-DEC-38; ID-INV-04; CR-B04 §4.3 step 5
- **Setup prerequisite:** two synthetic payloads that share an apparent `execId` text and differ in at least one byte.
- **Future action type:** offline. `NOT_EXECUTED`.
- **Positive expectation:** the pair is labeled `CONTRADICTION` and both originals remain.
- **Negative expectation:** neither payload is deleted and neither posts.
- **Data to capture:** both raw payloads and both hashes. `UNRECORDED` for any external example.
- **Oracle limits:** the fixture does not show whether IBKR reuses `execId` for corrections.
- **Failure classification:** `CONTRADICTION` is the expected label, not a permission to repair.
- **Linked XC finding:** XC-01 OPEN.
- **Safety prohibition:** no last-write-wins, no deletion, no posting.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-A10

- **Scenario ID:** IBKR-SCN-A10
- **Title:** Correction or bust whose new evidence names a reference
- **Claim IDs:** IBKR-CLM-08
- **Internal source anchors:** ARCH-DEC-39; DET-03-D05; CR-B04 §4.5 `T06`; ARCH-DEC-09
- **Setup prerequisite:** a synthetic new payload that itself contains a reference field pointing at an earlier evidence id. The field is fixture data, not an IBKR field name.
- **Future action type:** offline. `NOT_EXECUTED`.
- **Positive expectation:** the original remains and the reference already present in the new bytes is stored as data.
- **Negative expectation:** no reversal booking, no release, and no closure change.
- **Data to capture:** original payload, new payload, and the reference string copied from the new payload.
- **Oracle limits:** a fixture reference is not an IBKR correction chain.
- **Failure classification:** `ORACLE_INSUFFICIENT` if the reference was added by the reviewer rather than by the payload.
- **Linked XC finding:** XC-01 OPEN. XC-02-PRED stays OPEN.
- **Safety prohibition:** no compensating ledger line and no release.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-A11

- **Scenario ID:** IBKR-SCN-A11
- **Title:** Correction or bust whose new evidence names no reference
- **Claim IDs:** IBKR-CLM-08
- **Internal source anchors:** ARCH-DEC-39; CR-B03 §3.2 subordinate to ARCH-DEC-26; CR-B04 §4.5 `T06`
- **Setup prerequisite:** a synthetic new payload with no reference field.
- **Future action type:** offline. `NOT_EXECUTED`.
- **Positive expectation:** the new observation is appended and no link is invented.
- **Negative expectation:** the reviewer does not add a link from context, timing, or equal quantities.
- **Data to capture:** the new payload and an explicit statement that no reference field was present.
- **Oracle limits:** absence of a fixture field does not describe IBKR's real correction format.
- **Failure classification:** `CONTRADICTION` if a link is invented.
- **Linked XC finding:** XC-01 OPEN. XC-02-PRED stays OPEN.
- **Safety prohibition:** no invented link, no reversal, no release.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-A12

- **Scenario ID:** IBKR-SCN-A12
- **Title:** Contradictory raw payload under one apparent key
- **Claim IDs:** IBKR-CLM-06, IBKR-CLM-07, IBKR-CLM-10
- **Internal source anchors:** ARCH-DEC-38; ARCH-DEC-47; DET-08-D03; ID-INV-04
- **Setup prerequisite:** two synthetic raw payloads, same apparent alias text, different bytes, one labeled as a status observation and one as an execution observation.
- **Future action type:** offline. `NOT_EXECUTED`.
- **Positive expectation:** both originals remain and the comparison is marked inference.
- **Negative expectation:** neither side wins and no bytes are dropped.
- **Data to capture:** both raw payloads, both hashes, the inference note.
- **Oracle limits:** the fixture does not establish a broker-wins or database-wins rule.
- **Failure classification:** `CONTRADICTION`.
- **Linked XC finding:** XC-01 OPEN. XC-08-IBKR OPEN. XC-02-PRED OPEN.
- **Safety prohibition:** no economic conclusion and no cancel.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-A13

- **Scenario ID:** IBKR-SCN-A13
- **Title:** Out-of-order execution evidence
- **Claim IDs:** IBKR-CLM-09
- **Internal source anchors:** DET-05-D04; CR-B04 §4.5 `T10`; CR-B07 test label T05; ARCH-DEC-47
- **Setup prerequisite:** two synthetic executions whose receipt order differs from the broker timestamps written in the fixture. A missing broker timestamp is not invented.
- **Future action type:** offline. `NOT_EXECUTED`.
- **Positive expectation:** both payloads remain, and receipt order is labeled as receipt order.
- **Negative expectation:** arrival order does not delete the other payload and does not set priority.
- **Data to capture:** receipt timestamps, wall-clock timestamps, and broker timestamps only if the fixture included them.
- **Oracle limits:** the fixture order is not an IBKR delivery guarantee.
- **Failure classification:** `ORACLE_INSUFFICIENT` if a missing broker timestamp is filled in.
- **Linked XC finding:** XC-01 OPEN. XC-08-IBKR OPEN.
- **Safety prohibition:** no reordering write, no posting.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-A14

- **Scenario ID:** IBKR-SCN-A14
- **Title:** Terminal-looking status beside an unknown local attempt
- **Claim IDs:** IBKR-CLM-10
- **Internal source anchors:** ARCH-DEC-17; EX-INV-05; CR-B01 §1.3; DET-08-D03; ARCH-DEC-47
- **Setup prerequisite:** a synthetic status word that looks terminal, an execution payload, and a local attempt still `OUTCOME_UNKNOWN`. The status word is not an adopted broker enum.
- **Future action type:** offline. `NOT_EXECUTED`.
- **Positive expectation:** status, execution, and attempt remain three records.
- **Negative expectation:** the status word is not sole economic authority and does not close the attempt.
- **Data to capture:** the three records and an inference note. `UNRECORDED` externally.
- **Oracle limits:** no status vocabulary is verified. XC-02-PRED stays OPEN even if the three records are stored.
- **Failure classification:** `ORACLE_INSUFFICIENT` if the status word is treated as finality.
- **Linked XC finding:** XC-02-PRED OPEN. XC-03-MATRIX OPEN. XC-08-IBKR OPEN.
- **Safety prohibition:** no OMS transition, no release, no terminal acceptance.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

## Snapshots

### IBKR-SCN-B01

- **Scenario ID:** IBKR-SCN-B01
- **Title:** Empty result
- **Claim IDs:** IBKR-CLM-14, IBKR-CLM-15
- **Internal source anchors:** `P1-B07-D02`; `P1-B07-D04`; RC-INV-04; ARCH-DEC-41; CR-B01 §1.2 item 7
- **Setup prerequisite:** a synthetic empty list with an explicit request scope. A zero-row decode after a short read is IBKR-SCN-B03 or IBKR-SCN-B04, not this row.
- **Future action type:** read-only. `NOT_EXECUTED`.
- **Positive expectation:** the empty list is stored with coverage `UNKNOWN`.
- **Negative expectation:** the empty list is not non-existence, finality, release, or `DEFINITIVELY_NOT_SENT`.
- **Data to capture:** request scope, response byte length, row count zero, no completeness token.
- **Oracle limits:** emptiness of a capture is not emptiness at the broker.
- **Failure classification:** `ORACLE_INSUFFICIENT` if absence is inferred.
- **Linked XC finding:** XC-08-IBKR OPEN. XC-04 remainder stays OPEN for any send question.
- **Safety prohibition:** no resubmit from an empty list, no release.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-B02

- **Scenario ID:** IBKR-SCN-B02
- **Title:** Truncated window
- **Claim IDs:** IBKR-CLM-11, IBKR-CLM-13
- **Internal source anchors:** `P1-B07-D03`; DET-08-D02; ARCH-DEC-40; ARCH-DEC-42
- **Setup prerequisite:** a request window copied from the request, and a response whose last broker timestamp, if any, falls inside the window while the capture ended early.
- **Future action type:** read-only. `NOT_EXECUTED`.
- **Positive expectation:** the stored scope is the requested window and coverage stays `UNKNOWN`.
- **Negative expectation:** the missing tail is not treated as an empty tail.
- **Data to capture:** requested window, received row span, local gap marker if the capture observed the cut.
- **Oracle limits:** the row does not invent a retention duration.
- **Failure classification:** `CAPTURE_GAP` when the cut was locally observed. `ORACLE_INSUFFICIENT` when the tail is assumed empty.
- **Linked XC finding:** XC-08-IBKR OPEN.
- **Safety prohibition:** no backfill send and no release.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-B03

- **Scenario ID:** IBKR-SCN-B03
- **Title:** Pagination break
- **Claim IDs:** IBKR-CLM-12
- **Internal source anchors:** CR-B07 test label T04; ARCH-DEC-42; DET-04-D05 is not an IBKR sequence
- **Setup prerequisite:** a synthetic multi-page capture that stops between pages. Page-marker names are not asserted.
- **Future action type:** read-only. `NOT_EXECUTED`.
- **Positive expectation:** the break is a local capture gap and no page count is invented.
- **Negative expectation:** missing pages are not interpolated.
- **Data to capture:** bytes of each received page and the fact that the next page was not received.
- **Oracle limits:** the fixture does not identify a real paging API.
- **Failure classification:** `CAPTURE_GAP`.
- **Linked XC finding:** XC-08-IBKR OPEN. B07-T04 stays `NOT_EXECUTED`.
- **Safety prohibition:** no invented sequence and no follow-on request authorized by this row.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-B04

- **Scenario ID:** IBKR-SCN-B04
- **Title:** Dropped connection during a read
- **Claim IDs:** IBKR-CLM-12, IBKR-CLM-18
- **Internal source anchors:** CR-B07 test label T02; DET-05 `BA-T01`; ARCH-DEC-42
- **Setup prerequisite:** a synthetic stream that ends without the end marker the fixture itself defined. No socket is opened.
- **Future action type:** controlled. `NOT_EXECUTED`.
- **Positive expectation:** the drop is a local capture gap.
- **Negative expectation:** rows that never arrived are not created, and the gap does not prove they do not exist.
- **Data to capture:** byte offset where the read stopped, connection-lifecycle label `dropped`, secret-free.
- **Oracle limits:** a fixture drop is not an IBKR disconnect. Broker completeness stays unknown.
- **Failure classification:** `CAPTURE_GAP`.
- **Linked XC finding:** XC-08-IBKR OPEN.
- **Safety prohibition:** no reconnect send, no retry, no interpolated fill.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-B05

- **Scenario ID:** IBKR-SCN-B05
- **Title:** Delayed execution
- **Claim IDs:** IBKR-CLM-13, IBKR-CLM-18
- **Internal source anchors:** `P1-B07-D03`; DET-08 `RC-T03`; ARCH-DEC-14; ARCH-DEC-26
- **Setup prerequisite:** a first window stored as incomplete, then a later synthetic execution. No live fill is produced.
- **Future action type:** read-only. `NOT_EXECUTED`.
- **Positive expectation:** the late execution is appended and the earlier window stays incomplete.
- **Negative expectation:** the late row does not release a residual reservation and does not rewrite a closure.
- **Data to capture:** both receipt times and the original request window.
- **Oracle limits:** the fixture delay is not a measured IBKR delay. No retention number is claimed.
- **Failure classification:** `ORACLE_INSUFFICIENT` if the first window is closed because a later row arrived.
- **Linked XC finding:** XC-08-IBKR OPEN. XC-02-PRED OPEN.
- **Safety prohibition:** no release and no posting.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-B06

- **Scenario ID:** IBKR-SCN-B06
- **Title:** Missing end marker
- **Claim IDs:** IBKR-CLM-12, IBKR-CLM-15
- **Internal source anchors:** CR-B07 test label T03; ARCH-DEC-42; ARCH-DEC-41
- **Setup prerequisite:** a capture whose fixture expected an end marker and did not observe one. The marker's real name is `UNVERIFIED_RESEARCH_TOPIC`.
- **Future action type:** read-only. `NOT_EXECUTED`.
- **Positive expectation:** the missing marker is a local capture gap and coverage stays `UNKNOWN`.
- **Negative expectation:** no completeness token is stored to paper over the gap.
- **Data to capture:** the list of markers actually observed, which may be empty, and the gap label.
- **Oracle limits:** not observing a marker whose name is unknown is not proof the broker omitted it.
- **Failure classification:** `CAPTURE_GAP` only when the stage had named the marker in advance from obtained official text. Otherwise `ORACLE_INSUFFICIENT`.
- **Linked XC finding:** XC-08-IBKR OPEN. B07-T03 stays `NOT_EXECUTED`.
- **Safety prohibition:** no `COMPLETE` marker and no follow-on action.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-B07

- **Scenario ID:** IBKR-SCN-B07
- **Title:** Cross-session snapshot confusion
- **Claim IDs:** IBKR-CLM-11, IBKR-CLM-16
- **Internal source anchors:** ARCH-DEC-43; ARCH-DEC-40; CR-B07 test label T06; `P1-B07-D01`
- **Setup prerequisite:** two session labels and two synthetic snapshots. Identifiers do not cross the boundary without an evidenced binding.
- **Future action type:** read-only. `NOT_EXECUTED`.
- **Positive expectation:** each snapshot keeps its session label, and old evidence remains.
- **Negative expectation:** the new session does not inherit the old map and is not a negative send proof.
- **Data to capture:** both session labels and both raw snapshots. `UNRECORDED`.
- **Oracle limits:** the fixture does not show which IBKR ids survive reconnect.
- **Failure classification:** `CONTRADICTION` if session maps are merged.
- **Linked XC finding:** XC-08-IBKR OPEN. XC-01 OPEN. XC-04 remainder OPEN.
- **Safety prohibition:** no resubmit of a row missing from the new session.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-B08

- **Scenario ID:** IBKR-SCN-B08
- **Title:** Cross-account snapshot confusion
- **Claim IDs:** IBKR-CLM-11
- **Internal source anchors:** CR-B04 §4.2 `account` row; `P1-B07-D01`; ARCH-DEC-40
- **Setup prerequisite:** two account labels on two synthetic snapshots.
- **Future action type:** read-only. `NOT_EXECUTED`.
- **Positive expectation:** each snapshot is limited to the account named on its request.
- **Negative expectation:** rows are not copied across accounts to fill a gap.
- **Data to capture:** request account label and response bytes per snapshot.
- **Oracle limits:** advisor and multi-account routing stays unverified.
- **Failure classification:** `CONTRADICTION` on a cross-account copy.
- **Linked XC finding:** XC-08-IBKR OPEN. XC-01 OPEN.
- **Safety prohibition:** no account access and no global snapshot claim.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-B09

- **Scenario ID:** IBKR-SCN-B09
- **Title:** Stale cache
- **Claim IDs:** IBKR-CLM-11, IBKR-CLM-13
- **Internal source anchors:** DET-08-D03 `STALE_SNAPSHOT`; DET-08 recovery step 4; `P1-B07-D03`
- **Setup prerequisite:** a synthetic snapshot whose request window is older than a later observation, without a claim that the broker cached it.
- **Future action type:** read-only. `NOT_EXECUTED`.
- **Positive expectation:** the older snapshot is labeled incomplete relative to the later observation.
- **Negative expectation:** the older snapshot is not refreshed by invention and is not discarded.
- **Data to capture:** both captures, their windows, and their receipt times.
- **Oracle limits:** staleness in the fixture is not a measured broker cache lifetime.
- **Failure classification:** `ORACLE_INSUFFICIENT`. `STALE_SNAPSHOT` is a local label, not a broker code.
- **Linked XC finding:** XC-08-IBKR OPEN.
- **Safety prohibition:** no backfill and no release.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-B10

- **Scenario ID:** IBKR-SCN-B10
- **Title:** Mismatched request scope
- **Claim IDs:** IBKR-CLM-11
- **Internal source anchors:** ARCH-DEC-40; `P1-B07-D01`; DET-08-D02
- **Setup prerequisite:** a request that names one dataset and one window, and a response fixture that a careless reader might apply more widely.
- **Future action type:** read-only. `NOT_EXECUTED`.
- **Positive expectation:** the stored scope is only what the request named. Coverage stays `UNKNOWN`.
- **Negative expectation:** the response is not called global and is not called `COMPLETE`.
- **Data to capture:** the request fields that were present. Absent fields stay absent.
- **Oracle limits:** an omitted filter is not defaulted into full coverage.
- **Failure classification:** `ORACLE_INSUFFICIENT` on a global reading.
- **Linked XC finding:** XC-08-IBKR OPEN.
- **Safety prohibition:** no completeness claim and no dispatch.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-B11

- **Scenario ID:** IBKR-SCN-B11
- **Title:** Different API or product versions
- **Claim IDs:** IBKR-CLM-26, IBKR-CLM-11
- **Internal source anchors:** CR-B04 §4.1; CR-B04 §4.5; ARCH-DEC-48; GATE-P1-07
- **Setup prerequisite:** two synthetic packages with two different version tuples. Neither tuple is a real version string. Real versions are `UNRECORDED`.
- **Future action type:** offline. `NOT_EXECUTED`.
- **Positive expectation:** the packages stay separate and neither inherits the other's claims.
- **Negative expectation:** a pass on one tuple is not written onto the other tuple.
- **Data to capture:** product, API client, OS, and account mode per package, or an explicit `UNRECORDED` when a component is missing.
- **Oracle limits:** the row invents no version and no URL. Revalidation cadence remains `IBKREVQ-12`, pending.
- **Failure classification:** `ORACLE_INSUFFICIENT` if one package is treated as universal.
- **Linked XC finding:** XC-08-IBKR OPEN. XC-01 OPEN.
- **Safety prohibition:** no live environment and no version guess promoted to evidence.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

## Unknown send, crash, and failover

### IBKR-SCN-C01

- **Scenario ID:** IBKR-SCN-C01
- **Title:** Unknown send before `DISPATCH_STARTED`
- **Claim IDs:** IBKR-CLM-20
- **Internal source anchors:** CR-B01 §1.4 row before `DISPATCH_STARTED`; DET-07 `EX-T01`; CR-B01 §1.3; ARCH-DEC-44; ARCH-DEC-11
- **Setup prerequisite:** a synthetic attempt in `PREPARED` or `CLAIMED`. No socket exists.
- **Future action type:** controlled. `NOT_EXECUTED`.
- **Positive expectation:** the phase stays before send, and no broker row is required to say that.
- **Negative expectation:** an empty external list is not by itself `DEFINITIVELY_NOT_SENT`, and no new claim is authorized.
- **Data to capture:** the attempt phase and the absence of a `DISPATCH_STARTED` mark in the fixture.
- **Oracle limits:** the fixture is not a process crash and not a broker proof.
- **Failure classification:** `AMBIGUOUS_SEND` if the phase mark is missing. `ORACLE_INSUFFICIENT` if emptiness is used as the proof.
- **Linked XC finding:** XC-04 remainder OPEN. Design precedence stays `RESOLVED_BY_ARCH_DEC_03`.
- **Safety prohibition:** no retry and no takeover.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-C02

- **Scenario ID:** IBKR-SCN-C02
- **Title:** Unknown send after `DISPATCH_STARTED`
- **Claim IDs:** IBKR-CLM-21
- **Internal source anchors:** CR-B01 §1.2 item 4; CR-B01 §1.4 row after `DISPATCH_STARTED` and before socket write; DET-07-D03; EX-INV-02; ARCH-DEC-44; ARCH-DEC-03
- **Setup prerequisite:** a synthetic attempt whose `DISPATCH_STARTED` mark is committed and whose socket result is absent.
- **Future action type:** controlled. `NOT_EXECUTED`.
- **Positive expectation:** the attempt is labeled possibly sent.
- **Negative expectation:** no resend, cancel, release, or dispatch is labeled legal.
- **Data to capture:** the committed phase and the missing outcome. `UNRECORDED`.
- **Oracle limits:** the local mark is not broker acceptance. CR-B01 precedence over less restrictive DET-07 text stays in force.
- **Failure classification:** `AMBIGUOUS_SEND`.
- **Linked XC finding:** XC-04 remainder OPEN.
- **Safety prohibition:** no blind retry.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-C03

- **Scenario ID:** IBKR-SCN-C03
- **Title:** Timeout after `DISPATCH_STARTED`
- **Claim IDs:** IBKR-CLM-21
- **Internal source anchors:** CR-B01 §1.3; ARCH-DEC-44; ARCH-DEC-31
- **Setup prerequisite:** the C02 fixture plus a timeout label. No timeout duration is chosen.
- **Future action type:** controlled. `NOT_EXECUTED`.
- **Positive expectation:** the timeout leaves the attempt possibly sent.
- **Negative expectation:** timeout is not `DEFINITIVELY_NOT_SENT`.
- **Data to capture:** the timeout label and the phase. Duration `UNRECORDED`.
- **Oracle limits:** no IBKR timeout value is asserted.
- **Failure classification:** `AMBIGUOUS_SEND`.
- **Linked XC finding:** XC-04 remainder OPEN.
- **Safety prohibition:** no retry on timeout.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-C04

- **Scenario ID:** IBKR-SCN-C04
- **Title:** Lost acknowledgement
- **Claim IDs:** IBKR-CLM-22
- **Internal source anchors:** DET-08 `RC-T01`; BA-INV-03; CR-B01 §1.4 acknowledgement row; ARCH-DEC-44
- **Setup prerequisite:** a synthetic attempt at `DISPATCH_STARTED` with no stored acknowledgement.
- **Future action type:** controlled. `NOT_EXECUTED`.
- **Positive expectation:** the label stays `OUTCOME_UNKNOWN`.
- **Negative expectation:** a missing acknowledgement does not authorize a second submit.
- **Data to capture:** attempt id, phase, and the explicit absence of an acknowledgement row.
- **Oracle limits:** lost, never sent, and accepted-but-unstored stay indistinguishable in this fixture.
- **Failure classification:** `AMBIGUOUS_SEND`.
- **Linked XC finding:** XC-04 remainder OPEN. XC-01 OPEN if a later link is proposed.
- **Safety prohibition:** no retry and no posting from a guessed acceptance.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-C05

- **Scenario ID:** IBKR-SCN-C05
- **Title:** Crash after a possible send
- **Claim IDs:** IBKR-CLM-21, IBKR-CLM-22
- **Internal source anchors:** DET-07 `EX-T03`; RC-INV-01; CR-B01 §1.4; ARCH-DEC-34
- **Setup prerequisite:** a synthetic crash mark after `DISPATCH_STARTED`. Crash injection is not performed. No process is killed.
- **Future action type:** controlled. `NOT_EXECUTED`.
- **Positive expectation:** recovery stays in reconciliation and the attempt stays possibly sent.
- **Negative expectation:** recovery does not resubmit.
- **Data to capture:** the phase committed before the crash mark. `UNRECORDED`.
- **Oracle limits:** a document crash mark is not a fault-injection result. ARCH-DEC-31 tests stay `NOT_EXECUTED`.
- **Failure classification:** `AMBIGUOUS_SEND`. `STOP_STAGE` if a harness ever sends while this row is open.
- **Linked XC finding:** XC-04 remainder OPEN.
- **Safety prohibition:** no crash against a broker and no resubmit.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-C06

- **Scenario ID:** IBKR-SCN-C06
- **Title:** Reconnect while a send is unknown
- **Claim IDs:** IBKR-CLM-16, IBKR-CLM-21
- **Internal source anchors:** ARCH-DEC-43; CR-B01 §1.2 item 8; DET-05-D05; DET-07 `EX-T08`
- **Setup prerequisite:** C02 plus a new session label. No reconnect is performed.
- **Future action type:** controlled. `NOT_EXECUTED`.
- **Positive expectation:** the new session is a new context and the old attempt stays possibly sent.
- **Negative expectation:** reconnect is not proof the send did not happen and does not carry ids across without an evidenced binding.
- **Data to capture:** old session, new session, attempt phase.
- **Oracle limits:** the fixture is not a gateway session.
- **Failure classification:** `AMBIGUOUS_SEND` for the attempt. `CONTRADICTION` if maps are merged.
- **Linked XC finding:** XC-04 remainder OPEN. XC-01 OPEN.
- **Safety prohibition:** no retry on reconnect.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-C07

- **Scenario ID:** IBKR-SCN-C07
- **Title:** Gateway or TWS restart while a send is unknown
- **Claim IDs:** IBKR-CLM-17, IBKR-CLM-03, IBKR-CLM-21
- **Internal source anchors:** CR-B04 §4.5 `T03`; CR-B01 §1.5 test label `T05`; CR-B01 §1.3; ARCH-DEC-43
- **Setup prerequisite:** a synthetic restart label. TWS and IB Gateway are not started or stopped.
- **Future action type:** controlled. `NOT_EXECUTED`.
- **Positive expectation:** pre-restart evidence is retained and the attempt stays possibly sent.
- **Negative expectation:** restart is not `DEFINITIVELY_NOT_SENT` and does not authorize a second send.
- **Data to capture:** old session, restart label, version tuple `UNRECORDED`.
- **Oracle limits:** TWS and IB Gateway are not asserted to behave alike.
- **Failure classification:** `AMBIGUOUS_SEND`.
- **Linked XC finding:** XC-04 remainder OPEN. XC-01 OPEN.
- **Safety prohibition:** no gateway startup and no resend.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-C08

- **Scenario ID:** IBKR-SCN-C08
- **Title:** Prior writer still possibly active
- **Claim IDs:** IBKR-CLM-23, IBKR-CLM-24
- **Internal source anchors:** CR-B01 §1.1; CR-B01 §1.4 last row; P0-B01-INV-02; ARCH-DEC-45
- **Setup prerequisite:** a successor label and a prior-writer label `possibly_active`. No second process is started.
- **Future action type:** controlled. `NOT_EXECUTED`.
- **Positive expectation:** the successor stays blocked.
- **Negative expectation:** the successor does not send.
- **Data to capture:** both writer labels and the missing isolation evidence.
- **Oracle limits:** the fixture does not prove a broker will reject the old writer.
- **Failure classification:** `STOP_STAGE` if a successor send is proposed.
- **Linked XC finding:** XC-04 remainder OPEN.
- **Safety prohibition:** no takeover and no second send.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-C09

- **Scenario ID:** IBKR-SCN-C09
- **Title:** Expired lease
- **Claim IDs:** IBKR-CLM-23, IBKR-CLM-25
- **Internal source anchors:** CR-B01 §1.2 item 8; ARCH-DEC-45; DET-07 fencing-limit paragraph
- **Setup prerequisite:** a synthetic lease deadline in the past. No TTL is chosen and no lease is implemented.
- **Future action type:** controlled. `NOT_EXECUTED`.
- **Positive expectation:** the expired deadline stays a local fact and the successor stays blocked.
- **Negative expectation:** lease expiry is not external fencing and not broker rejection of the old writer.
- **Data to capture:** lease deadline, epoch, authority label, and a separate empty isolation record.
- **Oracle limits:** a past timestamp is not takeover evidence.
- **Failure classification:** `STOP_STAGE` if expiry activates the successor.
- **Linked XC finding:** XC-04 remainder OPEN. XC-05-LOCK OPEN.
- **Safety prohibition:** no heartbeat, no TTL, no takeover.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-C10

- **Scenario ID:** IBKR-SCN-C10
- **Title:** Split brain
- **Claim IDs:** IBKR-CLM-23, IBKR-CLM-24
- **Internal source anchors:** CR-B01 §1.2 item 8; P0-B01-INV-02; CR-B01 operational constraint on network kill; ARCH-DEC-45
- **Setup prerequisite:** two writer labels and no stop evidence and no connection-closed evidence. No partition is created.
- **Future action type:** controlled. `NOT_EXECUTED`.
- **Positive expectation:** the scope label stays `UNCERTAIN` and submission stays halted in the fixture.
- **Negative expectation:** a network-kill sentence in this document is not isolation evidence.
- **Data to capture:** the two labels and the empty isolation record.
- **Oracle limits:** the row does not demonstrate session exclusivity.
- **Failure classification:** `STOP_STAGE`.
- **Linked XC finding:** XC-04 remainder OPEN. XC-10-ACTION OPEN.
- **Safety prohibition:** no automatic failover and no dispatch.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-C11

- **Scenario ID:** IBKR-SCN-C11
- **Title:** Newer local epoch
- **Claim IDs:** IBKR-CLM-25, IBKR-CLM-23
- **Internal source anchors:** glossary `WriterEpoch`; ARCH-DEC-43; ARCH-DEC-45; CR-B01 §1.1; DET-07-D01 scope still undefined
- **Setup prerequisite:** a fixture that holds `WriterEpoch`, adapter session epoch, broker-acceptance label, and isolation label as four fields. The acceptance and isolation fields are empty.
- **Future action type:** offline. `NOT_EXECUTED`.
- **Positive expectation:** the four fields stay distinct, and a larger epoch does not fill the empty ones.
- **Negative expectation:** the newer epoch does not mark the old writer isolated and does not mark an order accepted.
- **Data to capture:** the four labels. `UNRECORDED` for any real counter.
- **Oracle limits:** the glossary shell is not the database fence and not a broker fence.
- **Failure classification:** `STOP_STAGE` if one field is copied into another.
- **Linked XC finding:** XC-04 remainder OPEN. XC-05-LOCK OPEN.
- **Safety prohibition:** no implementation of fencing and no send.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-C12

- **Scenario ID:** IBKR-SCN-C12
- **Title:** Partial fill during disconnect
- **Claim IDs:** IBKR-CLM-18, IBKR-CLM-21
- **Internal source anchors:** CR-B07 test labels T02 and T05; DET-08 `RC-T03`; ARCH-DEC-42; ARCH-DEC-26
- **Setup prerequisite:** a synthetic partial execution and a dropped read, with the attempt already `DISPATCH_STARTED`. No order is placed.
- **Future action type:** controlled. `NOT_EXECUTED`.
- **Positive expectation:** the partial row is appended and the drop is a local gap. The attempt stays possibly sent for any unobserved remainder.
- **Negative expectation:** the gap is not filled with an invented remainder, and the partial row does not release the rest.
- **Data to capture:** the partial payload, the gap marker, and the attempt phase.
- **Oracle limits:** the fixture quantity is not a broker fill. Position and risk stay uncomputed.
- **Failure classification:** `CAPTURE_GAP` plus `AMBIGUOUS_SEND`.
- **Linked XC finding:** XC-08-IBKR OPEN. XC-04 remainder OPEN. XC-02-PRED OPEN.
- **Safety prohibition:** no interpolated fill, no release, no retry.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

## Negative assertions

### IBKR-SCN-D01

- **Scenario ID:** IBKR-SCN-D01
- **Title:** Singleton candidate is not authorization
- **Claim IDs:** IBKR-CLM-27
- **Internal source anchors:** DET-03 formal rule for `A(e)`; ID-INV-02; ARCH-DEC-36; ARCH-DEC-13; ARCH-DEC-26
- **Setup prerequisite:** a synthetic candidate set of size one, labeled `SYNTHETIC_NOT_EXTERNAL`.
- **Future action type:** offline. `NOT_EXECUTED`.
- **Positive expectation:** the label stored is `SINGLE_CANDIDATE_NOT_AUTHORIZATION`.
- **Negative expectation:** the singleton does not become an economic match and does not book.
- **Data to capture:** the candidate set and the label. No resolver output is executed.
- **Oracle limits:** DET-03-D04's booking sentence stays subordinate and is not followed.
- **Failure classification:** `STOP_STAGE` if the singleton posts.
- **Linked XC finding:** XC-01 OPEN. XC-02-PRED OPEN.
- **Safety prohibition:** no automatic assignment and no posting.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-D02

- **Scenario ID:** IBKR-SCN-D02
- **Title:** Local capture gap is not an IBKR sequence
- **Claim IDs:** IBKR-CLM-12
- **Internal source anchors:** ARCH-DEC-42; CR-B07 `P1-B07-D03`; DET-04-D05
- **Setup prerequisite:** a local gap marker from B03, B04, or B06. No sequence counter is allocated.
- **Future action type:** offline. `NOT_EXECUTED`.
- **Positive expectation:** the marker cites only the local observation, such as a short page, a missing end, or a drop.
- **Negative expectation:** no IBKR sequence number is invented and no missing row is interpolated.
- **Data to capture:** the local gap reason and nothing that pretends to be a broker counter.
- **Oracle limits:** the marker does not describe a hole in a broker sequence.
- **Failure classification:** `CAPTURE_GAP`. `STOP_STAGE` if a sequence number is minted.
- **Linked XC finding:** XC-08-IBKR OPEN.
- **Safety prohibition:** no backfill send.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-D03

- **Scenario ID:** IBKR-SCN-D03
- **Title:** No completeness token
- **Claim IDs:** IBKR-CLM-15, IBKR-CLM-14
- **Internal source anchors:** ARCH-DEC-41; `P1-B07-D02`; ARCH-DEC-28; ARCH-DEC-29
- **Setup prerequisite:** any snapshot fixture in this matrix, including the empty result.
- **Future action type:** offline. `NOT_EXECUTED`.
- **Positive expectation:** the capture has no completeness-token field and no `COMPLETE` marker.
- **Negative expectation:** an empty response does not become a token, a non-existence proof, or a release.
- **Data to capture:** the field list, which omits the token.
- **Oracle limits:** a future official completion signal would still not ratify a token under ARCH-DEC-41.
- **Failure classification:** `STOP_STAGE` if a token is stored.
- **Linked XC finding:** XC-08-IBKR OPEN. XC-02-PRED OPEN.
- **Safety prohibition:** no `COMPLETE` claim and no release.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-D04

- **Scenario ID:** IBKR-SCN-D04
- **Title:** No broker-wins rule
- **Claim IDs:** IBKR-CLM-10, IBKR-CLM-19
- **Internal source anchors:** DET-08-D03; ARCH-DEC-47; ARCH-DEC-17; `P1-B09-D02`
- **Setup prerequisite:** two disagreeing synthetic snapshots, one called broker raw and one called local attempt. No cancel is sent.
- **Future action type:** offline. `NOT_EXECUTED`.
- **Positive expectation:** both sides remain and the comparison is inference.
- **Negative expectation:** neither side wins, and no cancel is inferred.
- **Data to capture:** both payloads and the inference note.
- **Oracle limits:** the row does not choose a priority for a future disagreement.
- **Failure classification:** `CONTRADICTION`. `STOP_STAGE` if a winner is applied.
- **Linked XC finding:** XC-08-IBKR OPEN. XC-10-ACTION OPEN. XC-02-PRED OPEN.
- **Safety prohibition:** no cancel, no posting, no terminal acceptance.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-D05

- **Scenario ID:** IBKR-SCN-D05
- **Title:** No economic effect and no release
- **Claim IDs:** IBKR-CLM-08, IBKR-CLM-14, IBKR-CLM-27
- **Internal source anchors:** ARCH-DEC-13; ARCH-DEC-14; ARCH-DEC-26; ARCH-DEC-29; ID-INV-03
- **Setup prerequisite:** any identity or snapshot fixture in this matrix, including a unique-looking match and an empty list.
- **Future action type:** offline. `NOT_EXECUTED`.
- **Positive expectation:** the fixture stores observations only.
- **Negative expectation:** no posting, no compensating line, no reservation conversion, no shrink, and no release.
- **Data to capture:** an explicit empty action list.
- **Oracle limits:** broker evidence, if it later arrives, still does not adopt `XC-02-PRED`.
- **Failure classification:** `STOP_STAGE` if an economic effect is attached.
- **Linked XC finding:** XC-02-PRED OPEN. XC-03-BOOK stays principle-only and does not authorize a posting.
- **Safety prohibition:** no economic posting and no reservation release.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

### IBKR-SCN-D06

- **Scenario ID:** IBKR-SCN-D06
- **Title:** No dispatch
- **Claim IDs:** IBKR-CLM-21, IBKR-CLM-23
- **Internal source anchors:** ARCH-DEC-44; ARCH-DEC-34; EX-INV-02; P0-B01-INV-01; BA-INV-03
- **Setup prerequisite:** the unknown-send fixtures C01 through C12. No harness is built.
- **Future action type:** offline. `NOT_EXECUTED`.
- **Positive expectation:** every window label stays non-authorizing.
- **Negative expectation:** no resend, cancel, release, or dispatch is enabled by any window, including before `DISPATCH_STARTED`.
- **Data to capture:** the phase label and the blocked-action list.
- **Oracle limits:** a documentation assertion of this row is not a crash test and not broker evidence.
- **Failure classification:** `STOP_STAGE` if any send flag is true.
- **Linked XC finding:** XC-04 remainder OPEN. XC-10-ACTION OPEN.
- **Safety prohibition:** no broker dispatch, no retry, no takeover, no live trading.
- **Status:** `NOT_EXECUTED`. `EXTERNAL_EVIDENCE_PENDING`. `NOT_AUTHORIZED`.

## Scenario index

| ID | Family | Action | XC | Status |
| --- | --- | --- | --- | --- |
| IBKR-SCN-A01 | identity | controlled | XC-01 | `NOT_EXECUTED` |
| IBKR-SCN-A02 | identity | controlled | XC-01 | `NOT_EXECUTED` |
| IBKR-SCN-A03 | identity | controlled | XC-01 | `NOT_EXECUTED` |
| IBKR-SCN-A04 | identity | offline | XC-01 | `NOT_EXECUTED` |
| IBKR-SCN-A05 | identity | offline | XC-01 | `NOT_EXECUTED` |
| IBKR-SCN-A06 | identity | offline | XC-01 | `NOT_EXECUTED` |
| IBKR-SCN-A07 | identity | offline | XC-01 | `NOT_EXECUTED` |
| IBKR-SCN-A08 | identity | offline | XC-01 | `NOT_EXECUTED` |
| IBKR-SCN-A09 | identity | offline | XC-01 | `NOT_EXECUTED` |
| IBKR-SCN-A10 | identity | offline | XC-01 | `NOT_EXECUTED` |
| IBKR-SCN-A11 | identity | offline | XC-01 | `NOT_EXECUTED` |
| IBKR-SCN-A12 | identity | offline | XC-01 | `NOT_EXECUTED` |
| IBKR-SCN-A13 | identity | offline | XC-01 | `NOT_EXECUTED` |
| IBKR-SCN-A14 | identity | offline | XC-02-PRED | `NOT_EXECUTED` |
| IBKR-SCN-B01 | snapshot | read-only | XC-08-IBKR | `NOT_EXECUTED` |
| IBKR-SCN-B02 | snapshot | read-only | XC-08-IBKR | `NOT_EXECUTED` |
| IBKR-SCN-B03 | snapshot | read-only | XC-08-IBKR | `NOT_EXECUTED` |
| IBKR-SCN-B04 | snapshot | controlled | XC-08-IBKR | `NOT_EXECUTED` |
| IBKR-SCN-B05 | snapshot | read-only | XC-08-IBKR | `NOT_EXECUTED` |
| IBKR-SCN-B06 | snapshot | read-only | XC-08-IBKR | `NOT_EXECUTED` |
| IBKR-SCN-B07 | snapshot | read-only | XC-08-IBKR | `NOT_EXECUTED` |
| IBKR-SCN-B08 | snapshot | read-only | XC-08-IBKR | `NOT_EXECUTED` |
| IBKR-SCN-B09 | snapshot | read-only | XC-08-IBKR | `NOT_EXECUTED` |
| IBKR-SCN-B10 | snapshot | read-only | XC-08-IBKR | `NOT_EXECUTED` |
| IBKR-SCN-B11 | snapshot | offline | XC-08-IBKR | `NOT_EXECUTED` |
| IBKR-SCN-C01 | unknown send | controlled | XC-04 | `NOT_EXECUTED` |
| IBKR-SCN-C02 | unknown send | controlled | XC-04 | `NOT_EXECUTED` |
| IBKR-SCN-C03 | unknown send | controlled | XC-04 | `NOT_EXECUTED` |
| IBKR-SCN-C04 | unknown send | controlled | XC-04 | `NOT_EXECUTED` |
| IBKR-SCN-C05 | unknown send | controlled | XC-04 | `NOT_EXECUTED` |
| IBKR-SCN-C06 | unknown send | controlled | XC-04 | `NOT_EXECUTED` |
| IBKR-SCN-C07 | unknown send | controlled | XC-04 | `NOT_EXECUTED` |
| IBKR-SCN-C08 | unknown send | controlled | XC-04 | `NOT_EXECUTED` |
| IBKR-SCN-C09 | unknown send | controlled | XC-04 | `NOT_EXECUTED` |
| IBKR-SCN-C10 | unknown send | controlled | XC-04 | `NOT_EXECUTED` |
| IBKR-SCN-C11 | unknown send | offline | XC-04 | `NOT_EXECUTED` |
| IBKR-SCN-C12 | unknown send | controlled | XC-04 | `NOT_EXECUTED` |
| IBKR-SCN-D01 | negative | offline | XC-01 | `NOT_EXECUTED` |
| IBKR-SCN-D02 | negative | offline | XC-08-IBKR | `NOT_EXECUTED` |
| IBKR-SCN-D03 | negative | offline | XC-08-IBKR | `NOT_EXECUTED` |
| IBKR-SCN-D04 | negative | offline | XC-08-IBKR | `NOT_EXECUTED` |
| IBKR-SCN-D05 | negative | offline | XC-02-PRED | `NOT_EXECUTED` |
| IBKR-SCN-D06 | negative | offline | XC-04 | `NOT_EXECUTED` |

No row records an execution result, a pass, a fail against a broker, a timestamp of a run, or a signature. **NO IMPLEMENTATION.** **NO VERIFICATION.**
