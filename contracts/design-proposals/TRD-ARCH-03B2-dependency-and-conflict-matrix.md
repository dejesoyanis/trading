# TRD-ARCH-03B.2 — Dependency and conflict matrix

**Date:** 2026-11-15  
**Role:** design proposal. `document_role` = `NON_RUNTIME_NON_WIRE_PROPOSAL`.  
**Ratification:** `NOT_RATIFIED`.  
**Live trading:** NOT AUTHORIZED.  
**Canonical schemas:** not published.

This matrix maps questions to sources. It does not edit an ADFR row, does not change a verification value, and does not resolve a finding.

## 1. Open findings

| Finding | Status | Principle already aligned | Executable rule unapproved |
| --- | --- | --- | --- |
| XC-01 | OPEN | Ambiguous identity is not an automatic merge or booking. | Key fields, uniqueness constraints, IBKR proof. `EVIDENCE_PENDING`. |
| XC-02-ALIGN | Principle only. `NO_CONFLICT_FOUND` on the principle. | Closure, timeout, and cancel acknowledgement are not release. | Not a predicate and not a release path. |
| XC-02-PRED | OPEN | Release requires confirmed economic finality, and that confirmation is undefined. | No executable predicate. No compensating posting. No reference inequality as a netting rule. |
| XC-03-BOOK | Principle only. | A late fill is retained and does not rewrite closure. | ARCH-DEC-13 forbids automatic booking, including a single match. |
| XC-03-MATRIX | OPEN | A future matrix needs families, negative edges, and its own version (ARCH-DEC-15). | No edge, no `OrderClosure` vocabulary, no risk effect on an edge. |
| XC-04 | OPEN except design precedence | CR-B01 precedence is `RESOLVED_BY_ARCH_DEC_03`. Unknown send is not non-send. | Session exclusivity, lease, heartbeat, scope, in-flight evidence, recovery tests, IBKR behavior. |
| XC-05-OUTBOX | Principle only. | Local commit includes the outbox. Broker and NATS stay outside. | Not a table. |
| XC-05-LOCK | OPEN | Retries of a future local unit do not resend (ARCH-DEC-18). | Lock order and isolation level unadopted. Deadlock tests not run. |
| XC-06 | OPEN | A distinct in-transaction audit record is required (ARCH-DEC-19). | Table name, DDL, retention, signing. |
| XC-07-FX | Principle only. | Missing or contradictory FX is indeterminate. | No conversion. |
| XC-07-ROUND | OPEN | No implicit rounding and no invented threshold. | Mode, quantum, staleness number, margin figure. |
| XC-08-RULE | Principle only. | Incomplete or empty snapshot is not absence and is not `DEFINITIVELY_NOT_SENT`. | Not a completeness implementation. |
| XC-08-IBKR | OPEN | Completeness is scoped on paper. | No adapter, no `COMPLETE` evidence, no history-retention proof. |
| XC-09-SOLO | Principle only. | A missing required second person is deny. No simulated dual approval. | No role check. |
| XC-09-RESET | OPEN | Halt is not broker-cancel finality. | `IncidentState` absent. Reset evidence and roles unapproved. |
| XC-10-RACE | Principle only. | Cancel request is not confirmation and is not release. | Not an edge. |
| XC-10-ACTION | OPEN | Modify and implicit cancel-replace stay blocked (ARCH-DEC-20). | Cancel during halt and risk-reduce dispatch unauthorized. |
| XC-11 | OPEN on status words | Population and supersession stay `RESOLVED_BY_ARCH_DEC_02`. | No winning status among `OPEN P0`, `PROPOSED_FOR_ACCEPTANCE`, and `DESIGN_RESOLVED`. |

`XC-02-PRED` is OPEN. This matrix does not close it.

## 2. Questions

| Question | Binding sources | ADFR area, not a status change | Findings that stay open | Handoff |
| --- | --- | --- | --- | --- |
| RISKQ-01 | DET-02 §2 Q01–Q02, §3, §4 item 6, §10. ARCH-DEC-01 exception. | DET-02 Q01–Q02 rows stay `NOT_VERIFIED`. | XC-02-PRED, XC-07-ROUND | None to the broker. Algebra stays a user decision. |
| RISKQ-02 | DET-02 §4, RISK-INV-03. CR-B10 §1. CR-B02 §2.5. ARCH-DEC-16, ARCH-DEC-18. | P0 budget-serialization rows stay `NOT_VERIFIED`. | XC-05-LOCK, XC-01 | Persistence tests later. Not started. |
| RISKQ-03 | DET-02 §6, RISK-INV-06. DET-09-D03, MD-INV-05. CR-B06 D04. | DET-09 and P1 valuation rows stay `NOT_VERIFIED`. | XC-07-FX principle, XC-07-ROUND | No price feed is authorized. |
| RISKQ-04 | DET-02 §5. ARCH-DEC-13, ARCH-DEC-14. CR-B03 §3.2. | DET-02 Q05 and late-fill rows stay `NOT_VERIFIED`. | XC-02-PRED, XC-01, XC-03-BOOK principle | ARCH-03B.3 before any match could be considered. Posting still forbidden. |
| RISKQ-05 | ARCH-DEC-12, ARCH-DEC-17, ARCH-DEC-20. DET-02 §5. CR-B07, CR-B09. | Cancel and snapshot rows stay `NOT_VERIFIED`. | XC-08-IBKR, XC-10-ACTION, XC-10-RACE principle | ARCH-03B.3 for cancel observation and completeness. |
| RISKQ-06 | ARCH-DEC-09, ARCH-DEC-13, ARCH-DEC-16. CR-B03 §3, CR-B04 §4.3. DET-01 §5. | CROSS and P0 late-fill rows stay `NOT_VERIFIED`. | XC-01, XC-02-PRED, XC-03-MATRIX | OMS matrix work for any workflow edge. ARCH-03B.3 for identity. |
| RISKQ-07 | DET-02 §7. CR-B09 D04. DET-10-D06. ARCH-DEC-20. | P1 action rows stay `NOT_VERIFIED`. GATE-P1-09 stays source text `EXTERNAL_VERIFICATION_REQUIRED`. | XC-10-ACTION, XC-09-RESET | No broker action. |
| RISKQ-08 | ARCH-DEC-14. SYNC-01 XC-02-PRED. | No ADFR `verification` leaves `NOT_VERIFIED`. | XC-02-PRED, XC-01, XC-07-ROUND, XC-08-IBKR | A future predicate is a different decision. Not asked as an adoption. |
| RISKQ-09 | DET-02 Q06, RISK-INV-04. ARCH-DEC-14, ARCH-DEC-12. CR-B07 D03. | DET-02 Q06 stays `NOT_VERIFIED`. | XC-02-PRED | Release stays forbidden even if proofs are listed. |
| RISKQ-10 | DET-02 §6. UD-03. CR-B06. DET-09-D04. ARCH-DEC-01. | Rounding and FX rows stay `NOT_VERIFIED`. | XC-07-ROUND, XC-07-FX principle | No mode. IBKR margin and FX stay `EXTERNAL_EVIDENCE_PENDING`. |
| RISKQ-11 | ARCH-DEC-18. CR-B02 §2.2–§2.3. DET-01 Q04. | P0 lock and isolation rows stay `NOT_VERIFIED`. | XC-05-LOCK, XC-05-OUTBOX principle | A later persistence task with deadlock evidence. Not started. |
| RISKQ-12 | ARCH-DEC-19. DET-04-D01. CR-B08. OPS-INV-01. | Audit rows stay `NOT_VERIFIED`. | XC-06 | A later naming decision. Not this package. |
| RISKQ-13 | ARCH-DEC-13, ARCH-DEC-16. CR-B05. XC-09-SOLO. | Reset and break-glass rows stay `NOT_VERIFIED`. | XC-09-RESET | No role and no second approver are created. |
| RISKQ-14 | DET-02 §9–§10. DET-08 RC-INV-01, RC-INV-03, RC-INV-05. ARCH-DEC-11, ARCH-DEC-16. | Test gates in the consolidation register stay unmet. | XC-03-MATRIX, XC-08-IBKR, XC-09-RESET | OMS matrix work for edges. ARCH-03B.3 for broker fixtures. |

ADFR ids are not restated as a new register. The consolidated file still has 78 ids, all `verification` = `NOT_VERIFIED`, one supersession `ADFR-CROSS-005` to `ADFR-CR-B10-001`, and 77 `NOT_SUPERSEDED`. This package does not invent an ADFR status.

## 3. What is aligned, and what is not an executable rule

Aligned and already binding: quarantine of economic effects (ARCH-DEC-13), no predicate and no release (ARCH-DEC-14), immutable closure (ARCH-DEC-09), no terminal broker observation (ARCH-DEC-17), local retry only and unadopted locks (ARCH-DEC-18), unnamed distinct audit (ARCH-DEC-19), no modify (ARCH-DEC-20), CR-B01 precedence (ARCH-DEC-03), fail-closed on missing valuation (DET-02 §6, XC-07-FX), incomplete snapshot is not absence (XC-08-RULE).

Not an executable rule: DET-02 §4 item 6 inequality, CR-B03 §3.3 balance equation, CR-B03 §3.2 compensating entry, CR-B02 lock order, `READ COMMITTED`, CR-B06 directional rounding, CR-B08 table name, CR-B04 automatic booking on a single match, any coverage-draft row, any of the three reservation approaches.

## 4. Handoffs

**ARCH-03B.3, not started and not authorized.** Broker identity scope and uniqueness (XC-01), IBKR snapshot completeness (XC-08-IBKR), cancel and modify semantics (XC-10-ACTION), and the open remainder of XC-04. RISKQ-04, RISKQ-05, RISKQ-06, and any future use of the conversion or staged approaches wait on that evidence. This matrix does not start ARCH-03B.3.

**Later OMS matrix work, not started.** `XC-03-MATRIX` stays OPEN. ARCH-DEC-15 is an acceptance bar, not a matrix. No risk effect is attached to a transition edge because no edge exists. `ORDER_STATE_MACHINE_IMPLEMENTED` and `STATE_TRANSITIONS_IMPLEMENTED` stay false. `OrderClosure` stays absent.

## 5. Blockers

| Future work | Blocker |
| --- | --- |
| Financial arithmetic | XC-07-ROUND, XC-07-FX as a principle with no rate, ARCH-DEC-01 rounding exception, `FINANCIAL_ROUNDING_IMPLEMENTED` false. RISKQ-10 does not remove the blocker. |
| Concurrency tests | XC-05-LOCK, ARCH-DEC-18, no trading tables, deadlock tests not run. The ignored PostgreSQL connectivity test is not a concurrency pass. |
| Reconciliation | XC-01, XC-08-IBKR, XC-09-RESET, DET-08 rules without a run. No claim that a reconciliation test passed. |
| Risk release | ARCH-DEC-14, XC-02-PRED OPEN, RISKQ-09 option C unavailable. Release stays forbidden. |
| Economic posting | ARCH-DEC-13. A single match is not a bypass. |
| Canonical schema or live trading | Registry flags stay `SKELETON_NOT_CANONICAL`, `canonical_schemas_published` false, `part_b` `NO-GO`, `live_trading_authorized` false. |

## 6. Decisions this matrix does not reopen

ARCH-DEC-01 through ARCH-DEC-20 stay as recorded. DET-01 and DET-02 stay `RECONSTRUCTED_DRAFT`, `SOURCE_REVIEW_PENDING`, `DESIGN_BASELINE_RATIFIED_WITH_EXCEPTIONS`, with Q01–Q10 verification cells OPEN. UD-01 through UD-09 status lines are unchanged. CR-B01 precedence over conflicting or less restrictive DET-07 text stays in force.
