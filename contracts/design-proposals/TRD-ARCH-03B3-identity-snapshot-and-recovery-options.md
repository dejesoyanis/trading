# TRD-ARCH-03B.3 — Identity, snapshot, and recovery options

**Date:** 2026-11-15  
**Role:** design proposal. `document_role` = `NON_RUNTIME_NON_WIRE_PROPOSAL`.  
**Ratification:** `NOT_RATIFIED`.  
**Live trading:** NOT AUTHORIZED.  
**Canonical schemas:** not published. `broker_uniqueness_evidence` = `EVIDENCE_PENDING`.

Three approaches are compared on each axis. The first is a safe fail-closed baseline. The second is a future evidence-rich candidate that still does not post, release, or send. The third is an unsafe contrast. An approach marked `CONFLICTS_WITH_BINDING_POLICY` cannot be ratified unless the named binding decision is explicitly superseded. This package does not supersede ARCH-DEC-03 or ARCH-DEC-09 through ARCH-DEC-34.

A preference below is not an approval. `preference_is_approval` stays false.

## Axis I — Identity

### `strict_scoped_observation`

Fail-closed baseline. Store raw evidence and the scope fields that were actually supplied. Do not build a candidate set. Do not equate raw ids across account, environment, client, session, alias kind, or evidence id. Absence stays absence. Structural equality of a shell record is not a merge, which is already how `BrokerIdentityContext` is documented in UD-02.

| Property | Reading |
| --- | --- |
| Safety | Highest among the three. There is no inferred match to misuse. |
| Complexity | Lowest. It matches the current shell and adds no resolver. |
| Recoverability | A later candidate set can be computed from retained raw evidence. Evidence that was never stored cannot be reconstructed. |
| Auditability | Every retained byte has provenance. There is no inference record to audit. |
| Testability | Documentation checks can forbid a uniqueness claim. ID-T01 through ID-T07 are not executed. |
| Blockers | XC-01 stays OPEN. No IBKR scope evidence. ARCH-DEC-13 still quarantines economics even if a human later sees one obvious order. |

### `evidence_graph_candidate_sets`

Future evidence-rich candidate. Retain the same raw evidence, and also retain an append-only inference record: the candidate set `A(e)` from the DET-03 formal rule, the label `NO_CANDIDATE`, `SINGLE_CANDIDATE_NOT_AUTHORIZATION`, `MULTIPLE_CANDIDATES`, `CONTRADICTION`, or `INSUFFICIENT_SCOPE`, and the evidence ids used. A set of size one does not book and does not release. Corrections append. They do not overwrite (DET-03-D05). Identical payload replay adds no economic effect and no second send (ARCH-DEC-16).

| Property | Reading |
| --- | --- |
| Safety | High only while the singleton label cannot post. The DET-03-D04 sentence that would book a `MATCHED` result conflicts with ARCH-DEC-13 and is not part of this approach. |
| Complexity | Higher. Inference version, input evidence ids, and contradiction rules must stay reproducible. |
| Recoverability | Better for later manual review because the candidate set is explicit. Still no automatic repair. |
| Auditability | Stronger than the baseline if every inference cites raw evidence ids (P0-B04-INV-03 as an obligation, not a built audit table). XC-06 stays OPEN. No table name. |
| Testability | Requires future fixtures for reuse, missing `permId`, late `permId`, duplicate payload, and conflicting payload. Those fixtures are `NOT_EXECUTED`. |
| Blockers | XC-01. No verified scope. A candidate set computed from unverified hypotheses can look more certain than it is. |

### `optimistic_global_id_merge`

`CONFLICTS_WITH_BINDING_POLICY`. Treat broker `orderId` or `permId` as globally unique, auto-merge across sessions, and book when one candidate appears. That conflicts with CR-B04 P0-B04-INV-02, UD-02, DET-03-D01, ARCH-DEC-13, and ARCH-DEC-26. It is contrast only.

| Property | Reading |
| --- | --- |
| Safety | Unsafe. A reused id or a late `permId` merges the wrong order and the source would then book it. |
| Complexity | Looks small and hides the collision cases CR-B04 §4.2 lists as unverified. |
| Recoverability | A silent merge is hard to undo, and this package forbids the compensating line that might try (ARCH-DEC-26). |
| Auditability | Poor if the raw disagreement is overwritten (ID-INV-04). |
| Testability | Would require the broker proof this repository does not have, and would still be blocked by ARCH-DEC-13. |
| Blockers | Cannot be ratified without explicit supersession of the named binding rules. |

Non-binding preference on this axis: `strict_scoped_observation` until a user chooses whether inference records are worth keeping. `evidence_graph_candidate_sets` is the evidence-rich alternative if the user wants candidate sets without economic effect. The preference is not an approval.

## Axis II — Completeness

### `fail_closed_partial_unknown`

Fail-closed baseline. Every broker response is `PARTIAL/UNKNOWN` for economic and dispatch purposes. No `COMPLETE` token is stored. Empty, timeout, disconnect, filled, and cancelled are ordinary retained observations. This matches P1-B07-D02 and P1-B07-D03 as far as those sentences forbid a negative proof, and it refuses the source's `COMPLETE` token until evidence exists.

| Property | Reading |
| --- | --- |
| Safety | Highest. Nothing can be mistaken for a finished snapshot. |
| Complexity | Lowest. |
| Recoverability | A later completeness proof can be attached to retained responses. A discarded response cannot. |
| Auditability | The lack of a `COMPLETE` claim is itself visible. |
| Testability | A documentation check can forbid a completeness flag. B07-T01 through T07 are `NOT_EXECUTED`. |
| Blockers | XC-08-IBKR. Operators see no coverage distinction between a short page and a long one. That is intentional. |

### `named_scope_proof_obligations`

Future evidence-rich candidate. A response may carry the CR-B07 scope fields that were actually known, plus a gap or truncation marker when the local capture itself saw a short page, a missing end, or a dropped stream. A future `COMPLETE` claim may be stored only as a pointer to external evidence that is not in this repository, and the claim authorizes nothing: no release, no non-existence proof, no terminal acceptance, no resubmit. Until that external evidence exists, the marker stays `EXTERNAL_EVIDENCE_PENDING`.

| Property | Reading |
| --- | --- |
| Safety | High if the claim cannot authorize. Unsafe if a reader treats `COMPLETE` as finality. ARCH-DEC-14 and ARCH-DEC-17 forbid that reading. |
| Complexity | Moderate. Scope fields and gap markers must not invent a broker sequence number. |
| Recoverability | Gaps stay visible, which is what DET-08-D02 asks a run record to keep. |
| Auditability | Each coverage statement cites request id, window, and receive time, or says those were absent. |
| Testability | Needs the future plan in the coverage draft. All of it is `NOT_EXECUTED`. |
| Blockers | XC-08-IBKR. No completion signal is documented here. DET-09 sequence fields are market-data reference text, not an IBKR order sequence. |

### `single_response_as_global_complete`

`CONFLICTS_WITH_BINDING_POLICY`. One account response, an empty list, a filled label, or a cancelled label proves global completeness, non-execution, or finality, and may release or resubmit. That conflicts with P1-B07-D02, RC-INV-04, CR-B01 §1.3, ARCH-DEC-14, ARCH-DEC-17, ARCH-DEC-11, and ARCH-DEC-03. Contrast only.

| Property | Reading |
| --- | --- |
| Safety | Unsafe. The empty and timeout cases are exactly the cases the binding rules refuse to treat as proof. |
| Complexity | Deceptively low. |
| Recoverability | A false "not sent" leads to a second send, which P0-B01-INV-01 forbids. |
| Auditability | The missing rows are not in the record, so the absence cannot be audited later. |
| Testability | Blocked. No IBKR completeness oracle exists here. |
| Blockers | Cannot be ratified without explicit supersession. |

Non-binding preference: `fail_closed_partial_unknown`. The named-scope obligations are available if the user wants gap markers that still authorize nothing.

## Axis III — Recovery and fencing

### `manual_escalation_no_resubmit`

Fail-closed baseline. An unknown send stays unknown. Lease expiry does not activate a successor. A prior writer that might still send blocks takeover. No retry, no reconciliation cancel, no broker call. Escalation is a quarantine marker with no role and no posting authority (ARCH-DEC-16, ARCH-DEC-33, ARCH-DEC-13). This is the CR-B01 §1.1 and §1.2 design, which ARCH-DEC-03 already ranks above less restrictive DET-07 text. It is design, not an operational proof.

| Property | Reading |
| --- | --- |
| Safety | Highest. P0-B01-INV-01 and INV-02 stay intact. |
| Complexity | Lowest as a prohibition. The human procedure is not designed here and must not be invented. |
| Recoverability | The platform stays halted for new risk on that scope (P0-B01-INV-04, ARCH-DEC-27). Recovery of trading is a later decision. |
| Auditability | The attempt phase and the escalation marker are the record. No table name (ARCH-DEC-19, ARCH-DEC-32). XC-06 stays OPEN. |
| Testability | P0-B01-T01 through T08 and EX-T01 through EX-T08 are `NOT_EXECUTED`. ARCH-DEC-31 requires future concurrency and crash tests and does not execute them. |
| Blockers | XC-04 remainder: session exclusivity, lease and heartbeat, dispatch scope, in-flight evidence, recovery tests, and IBKR behavior. |

### `evidence_rich_manual_recovery`

Future evidence-rich candidate. Retain the crash-window label as evidence only:

| Window | Label this approach would store | Send allowed? |
| --- | --- | --- |
| Before attempt commit | No attempt was committed, per CR-B01 §1.4 | No send is authorized by this package. A later command is a different decision. |
| After commit, before `DISPATCH_STARTED` | Possibly unsent, still not proved | No blind retry. |
| After `DISPATCH_STARTED`, including before the socket write | Possibly sent | No retry (CR-B01 §1.4). |
| Lost acknowledgement | Possibly accepted | No retry. |
| Delayed `permId` | Identity evidence appended | No retry and no booking. |
| Reconnect | New session context | No resubmit of the uncertain attempt. |
| Prior writer or split brain | Successor blocked | No send. |
| Conflicting broker observations | Both retained | No terminal acceptance and no cancel-to-reconcile. |

A person is not named. If a future rule required a second person and none was available, the result would be deny (ARCH-DEC-16). Review does not post.

| Property | Reading |
| --- | --- |
| Safety | High if labels cannot trigger send, cancel, or release. |
| Complexity | Higher. Window labels must not be read as `DEFINITIVELY_NOT_SENT`. |
| Recoverability | Clearer history for a later, separate authorization. This approach does not grant that authorization. |
| Auditability | Each window cites the local phase and the broker observations actually retained. |
| Testability | Same unrun fault-injection list as the baseline. |
| Blockers | XC-04 remainder. Distinguishing "before socket write" from "during socket write" needs instrumentation this repository does not have. Until then the conservative label is "possibly sent" after `DISPATCH_STARTED`. |

### `automatic_retry_or_lease_failover`

`CONFLICTS_WITH_BINDING_POLICY`. Retry an uncertain send, fail over because the lease expired, or cancel at the broker because a snapshot missed the order. That conflicts with ARCH-DEC-03, CR-B01 §1.1 through §1.4, P0-B01-INV-01, SYS-INV-02, BA-INV-03, RC-INV-01, ARCH-DEC-11, and ARCH-DEC-34. Contrast only. No broker call is made by describing it.

| Property | Reading |
| --- | --- |
| Safety | Unsafe. The lease is not a broker fence, and a missing row is not proof of non-send. |
| Complexity | The happy path looks simple. The double-send case is the complexity. |
| Recoverability | Two live orders are worse than a halt, and this package cannot release or flatten them (ARCH-DEC-20, ARCH-DEC-27). |
| Auditability | A blind retry destroys the meaning of the first attempt id. |
| Testability | The source tests that would have to fail this path were not run. |
| Blockers | Cannot be ratified without explicit supersession of ARCH-DEC-03 and the unknown-send decisions. |

Non-binding preference: `manual_escalation_no_resubmit`. The evidence-rich labels are available if the user wants the crash windows recorded without enabling recovery actions.

## Cross-axis rules that none of the safe approaches may drop

Evidence stays append-only. Provenance and correlation ids that were actually present stay attached. Conflicts stay quarantined. Duplicate identical payloads are idempotent observations. Replay does not create an economic effect or a second send. Partial fills, cancel requests, cancel observations, late fills, corrections, and busts are evidence only. No implicit cancel-replace, modify, risk-reducing order, or broker terminal acceptance is introduced.

`XC-01`, `XC-02-PRED`, `XC-03-MATRIX`, `XC-05-LOCK`, `XC-06`, `XC-07-ROUND`, `XC-08-IBKR`, `XC-09-RESET`, `XC-10-ACTION`, and the XC-11 status-word disagreement stay OPEN. XC-04 precedence stays `RESOLVED_BY_ARCH_DEC_03`. The rest of XC-04 stays OPEN.

Live trading is NOT AUTHORIZED.
