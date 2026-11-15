# TRD-ARCH-03B.4 — IBKR verification protocol

**Date:** 2026-11-15  
**Role:** staged future process. `document_role` = `NON_RUNTIME_NON_WIRE_VERIFICATION_PLAN`.  
**Execution:** `NOT_EXECUTED`.  
**Authorization:** `NOT_AUTHORIZED`. This protocol is not permission to run any stage.  
**Live trading:** NOT AUTHORIZED.

No stage below is open. The human checkpoint on every stage is STOP. A later go would be a separate dated user authorization that names the stage. This file is not that authorization. Paper trading is not approved by describing a stage that might someday use a paper account.

Documentation tests are not broker evidence. ARCH-DEC-48 stays unmet. ARCH-DEC-04: `DESIGN_RESOLVED` is not verification.

## Authority boundary

The proof-obligation label is `PROOF_OBLIGATION_OWNER_PLACEHOLDER`. It names the future seat that would owe the proof for a stage. It is not an operator role. It grants no post, no release, no dispatch, no reset, and no adjudication right. Creating the label does not satisfy ARCH-DEC-46. If a future two-person rule requires a second person and none is available, the result is deny. Simulated consent is not a substitute. No person is named.

The checkpoint label is `HUMAN_STOP_GO_CHECKPOINT`. For this milestone the recorded position is STOP on E0, E1, E2, E3, and E4. The placeholder does not hold a go.

## Stages

| Stage | Future work, not started | Checkpoint | Authorization |
| --- | --- | --- | --- |
| E0 | Collect official IBKR material and version metadata that was actually obtained. No URL is invented in this plan. | STOP | `NOT_AUTHORIZED` |
| E1 | Design offline fixtures and deterministic replay using synthetic data labeled `SYNTHETIC_NOT_EXTERNAL`. | STOP | `NOT_AUTHORIZED` |
| E2 | Isolated read-only integration experiments, only after a separate explicit user authorization, credential and sandbox governance, and verified network isolation. | STOP | `NOT_AUTHORIZED` |
| E3 | Controlled broker-connected fault scenarios and any order-related experiment, only after another separate authorization, a dedicated test account, an independent safety review, and a hard stop. | STOP | `NOT_AUTHORIZED` |
| E4 | Adjudicate evidence and, only then, ask for an explicit dated ratification. | STOP | `NOT_AUTHORIZED` |

### E0 — documentation collection

Future acquisition only. A bibliography would list titles, versions, and retrieval dates copied from copies that were actually obtained. This repository has no such bibliography. No official URL, document title, or version string is supplied here, and none is invented.

E0 output would still be `EXTERNAL_EVIDENCE_PENDING` until E4. Reading a document is not a uniqueness proof and not a completeness proof.

### E1 — offline fixtures

Fixtures are synthetic and carry the label `SYNTHETIC_NOT_EXTERNAL`. Replay is deterministic over those bytes. A passing fixture shows that a future reviewer can tell two local labels apart. It shows nothing about IBKR.

E1 cannot promote an ADFR `verification` cell. E1 cannot close `XC-01`, `XC-08-IBKR`, or the operational remainder of `XC-04`.

### E2 — isolated read-only experiments

E2 stays closed. Opening it requires all of the following, each absent today:

- a separate explicit user authorization that names E2 and names the read operations
- credential and sandbox governance written down outside this plan
- verified network isolation, evidenced, not assumed
- the E0 version tuple recorded
- a secret-free capture path

A read is still a broker request. This plan does not authorize it. An empty read is not a negative proof (IBKR-CLM-14). Paper mode is not automatically E2.

### E3 — controlled fault and order-related experiments

E3 stays closed. It is a further authorization, not a continuation of an E2 go. It requires a dedicated test account, an independent safety review, and a hard stop that aborts the stage on unexpected behavior. No test order is placed in this milestone. No cancel, modify, replace, retry, resubmit, or takeover is authorized.

CR-B01 precedence applies inside any future E3: after `DISPATCH_STARTED`, the attempt is possibly sent. The stage does not retry it.

### E4 — adjudication

E4 is a human decision about evidence that E0 through E3 actually produced. Writing the tests, or even running them later, does not enter E4 and does not ratify a result. A dated ratification would still not approve economic posting, reservation release, an economic-finality predicate, an OMS transition, broker dispatch, or live trading. ADFR `verification` leaves `NOT_VERIFIED` only under ARCH-DEC-07, which this file does not satisfy.

## Prerequisites a future package would record

Every value below is `UNRECORDED`. This list is not a connection procedure. No host, port, login step, client identifier, or broker command is prescribed.

| Prerequisite | Record | Value now |
| --- | --- | --- |
| IBKR product name | copied from the obtained component | `UNRECORDED` |
| IBKR product version | copied from the obtained component | `UNRECORDED` |
| API or client library name and version | copied from the obtained component | `UNRECORDED` |
| Operating system | the machine that ran the stage | `UNRECORDED` |
| Account mode | the mode named in the separate authorization | `UNRECORDED` |
| Clock synchronization | method and bound actually used | `UNRECORDED` |
| Immutable raw capture | store, hash algorithm, and who may append | `UNRECORDED` |
| Correlation id | how a future capture ties local and broker bytes | `UNRECORDED` |
| Monotonic local timestamp | clock source | `UNRECORDED` |
| Wall-clock timestamp | clock source, distinct from the monotonic clock | `UNRECORDED` |
| Transport and connection lifecycle | connected, dropped, restarted, as observed | `UNRECORDED` |
| Sanitized logs | secret-free check before retention | `UNRECORDED` |
| Reproducible setup | steps in the future package, not in this file | `UNRECORDED` |
| Expected-versus-observed assertions | written before the run | `UNRECORDED` |
| Negative control | a case that must fail closed | `UNRECORDED` |
| Retry-safe harness | the harness refuses a second send and refuses automatic retry | `UNRECORDED` |
| Crash-injection isolation | faults stay off any broker socket unless E3 was separately authorized | `UNRECORDED` |
| Cleanup | how the future package leaves the dedicated account | `UNRECORDED` |
| Rollback | how a bad capture is retained and marked, not deleted | `UNRECORDED` |
| Incident procedure | halt, escalate, deny if the second person is absent | `UNRECORDED` |

Rollback retains contradictory bytes (ARCH-DEC-38). It does not delete them. Cleanup does not include a broker cancel authorized by this plan.

## Stop rules

Any unexpected behavior halts the active stage. Unexpected behavior includes:

- a payload shape the stage did not list in its expected-versus-observed list
- a secret in a log or capture
- a connection, read, or order that the stage authorization did not name
- an empty list read as absence, finality, release, or `DEFINITIVELY_NOT_SENT`
- a proposal to store a completeness token
- a successor send while a prior writer might still send or send status is unknown
- a retry, resubmit, cancel, modify, or replace
- any economic posting, reservation release, or OMS transition
- a version tuple that differs from the pinned tuple for that package
- a local capture gap treated as an IBKR sequence number
- a contradiction resolved by deleting one payload

Halt stores a non-authorizing escalation marker only (ARCH-DEC-46). Halt does not retry, cancel, post, or reset. The `PROOF_OBLIGATION_OWNER_PLACEHOLDER` records the halt and does not clear it.

## Capture completeness and broker completeness

Capture completeness is a statement about bytes this process stored. Broker completeness is a statement about the broker dataset. They are different statements. Neither is granted. ARCH-DEC-41 forbids storing a completeness token for either one. A local capture gap (ARCH-DEC-42) speaks only about the capture.

An empty captured list supports only the statement that this capture stored no rows. It does not support absence at the broker.

## What a finished stage would still leave open

`XC-02-PRED` stays OPEN if broker evidence arrives. The OMS transition matrix stays OPEN. `XC-03-MATRIX` stays OPEN. Economic posting, reservation release, broker dispatch, and live trading stay unapproved. A pass on one version tuple does not cover the next tuple (IBKR-CLM-26).

## Safety

No broker login, connection, API request, TWS or IB Gateway startup, account access, paper order, or live order is performed by following this file. No credential is stored. No network call is described as a command. **NO IMPLEMENTATION.** **NO VERIFICATION.**
