# TRD-ARCH-03B.2 — Risk decision requests

**Date:** 2026-11-15  
**Role:** design proposal. `document_role` = `NON_RUNTIME_NON_WIRE_PROPOSAL`.  
**Ratification:** `NOT_RATIFIED`.  
**Status of every question:** `USER_DECISION_PENDING`.  
**XC-02-PRED:** OPEN.  
**Live trading:** NOT AUTHORIZED.  
**Canonical schemas:** not published. `part_b` = `NO-GO`. `broker_uniqueness_evidence` = `EVIDENCE_PENDING`.

A recommendation is not an approval. The questions are independent. Answering one does not answer another. No question authorizes economic posting, an economic-finality predicate, reservation release, an OMS transition, a canonical schema, broker dispatch, or live trading.

ARCH-DEC-13 (OMSQ-05 B) and ARCH-DEC-14 (OMSQ-06 A) stay binding for every alternative below. An alternative marked `CONFLICTS_WITH_BINDING_POLICY` is contrast. It is not available for ratification unless a later decision explicitly supersedes the named binding rule. This package does not supersede ARCH-DEC-09 through ARCH-DEC-20.

## RISKQ-01 — Hierarchical budget shape

**Status:** `USER_DECISION_PENDING`

**Question:** Which future constraint describes hierarchical risk-budget scope and deny-overrides, while executable budget math stays unapproved?

**Binding rules:** DET-02 §2 DET02-Q01 and DET02-Q02, §3 Deny-Overrides, RISK-INV-02. ARCH-DEC-01 exception: the reference inequality is not a netting, margin, or stress rule, and the composition algebra stays open. ARCH-DEC-04 gives `DESIGN_RESOLVED` in CR-B06 no priority.

**Sources:** `contracts/sources/DET-02-Risk-Contract-v1.0.md` §2, §3, §4 item 6, §10. `contracts/decisions/ARCH-DEC-01-bis-03-2026-11-15.md` DET-02 exceptions. `contracts/sources/P1-Resolution-Contracts-CR-B05-bis-CR-B09-v1.0.md` CR-B06 P1-B06-D03.

**Unresolved:** the policy-composition algebra, the metric catalogue, and any shared-budget arithmetic. XC-02-PRED stays OPEN.

**Alternatives:**

- **A — Fail-closed.** Keep the six scope names (global, account, strategy, instrument, sector, risk factor) and the deny-overrides principle as baseline text. Adopt no algebra and no numeric rule. Missing data does not produce `ALLOW`.
- **B — Conjunction shape only.** Same as A, plus a future constraint that hard denies compose by conjunction: one hard deny blocks, and a soft allow never overrides a hard deny. Still no numbers and no metric ids.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** Treat `used + reserved + proposed_delta <= limit` as the executable rule for every metric, including netting, margin, and stress. Conflicts with the ARCH-DEC-01 exception. Not available unless that exception is explicitly superseded.

| Alternative | Safety | Complexity | Recoverability | Auditability |
| --- | --- | --- | --- | --- |
| A | Highest. No formula can be misapplied. | Lowest. | A later algebra can be added as a new policy version. | The refusal cites the missing algebra. |
| B | High, if conjunction is not mistaken for arithmetic. | Moderate. | The shape is stable, the numbers remain replaceable. | Each deny names the hard rule that won. |
| C | Unsafe under the open exception. A single inequality hides netting. | Looks small and is not. | A wrong formula is hard to unwind once used. | A single comparison hides which scope failed. |

**Non-binding recommendation:** A. The baseline already states deny-overrides. Adopting B's extra sentence is a real choice and is not required to keep the system fail-closed. This recommendation is not an approval.

**Evidence needed:** a versioned metric list and an algebra the user later ratifies. Not present.

**Dependencies:** RISKQ-10 for units. XC-07-ROUND. Does not depend on a broker terminal rule.

**Would not authorize:** budget math, a policy register, a reservation writer, a release, a posting, or live trading.

## RISKQ-02 — Reservation ownership and contention

**Status:** `USER_DECISION_PENDING`

**Question:** How should a future reservation be identified and kept idempotent across concurrent scopes, without adopting a lock order?

**Binding rules:** DET-02 §4 items 4 and 5, RISK-INV-03, RISK-INV-08. CR-B10 §1: `RiskReservationId` is not `OrderId`. P0-B02-INV-02 and INV-03. ARCH-DEC-16 for identical replay. ARCH-DEC-18: no canonical lock protocol. ARCH-DEC-11: no second submission under an unknown send.

**Sources:** `contracts/sources/DET-02-Risk-Contract-v1.0.md` §4. `contracts/sources/CR-B10-Domain-Glossary-and-Schema-Registry-v1.0.md` §1. `contracts/sources/P0-Resolution-Contracts-CR-B01-bis-CR-B04-v1.0.md` §2.5. `contracts/decisions/ARCH-DEC-09-bis-20-2026-11-15.md` ARCH-DEC-16 and ARCH-DEC-18.

**Unresolved:** one reservation id versus one id per scope, the idempotency tuple, and the contention protocol. XC-05-LOCK stays OPEN.

**Alternatives:**

- **A — Fail-closed.** One reservation id, an explicit order association, and one command key. Same key and same payload return the stored result. Any other payload conflicts and creates no second reservation. A command that touches several scopes commits for all of them or for none. No lock order is adopted.
- **B — Per-scope ids under one command.** Each affected scope would get its own reservation id, all bound to one command id, still all-or-nothing. The id scheme is a future constraint, not a schema.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** Allow a second reservation or a second submission when the first send is uncertain, or leave a hold on one scope after another scope fails. Conflicts with RISK-INV-03, DET-02 §4 item 5, and ARCH-DEC-11.

| Alternative | Safety | Complexity | Recoverability | Auditability |
| --- | --- | --- | --- | --- |
| A | High. One id is easier to reconcile. | Lower. | Replay is one stored result. | One audit subject. |
| B | High if the set is atomic. A partial set is the failure mode. | Higher. | Each scope can be cited. Recovery must find the whole set. | One command, several reservation ids. |
| C | Unsafe. Double charge or a second order. | Misleadingly small. | The extra hold is hard to attribute. | Two effects share one story. |

**Non-binding recommendation:** A. Per-scope ids can wait until a lock protocol exists. This recommendation is not an approval.

**Evidence needed:** concurrency tests. They were not run. Deadlock tests were not run.

**Dependencies:** RISKQ-11. XC-05-LOCK. ARCH-DEC-18.

**Would not authorize:** a lock order, an isolation level, a table, a second send, or a release.

## RISKQ-03 — Inputs required before a reservation

**Status:** `USER_DECISION_PENDING`

**Question:** What happens when price, liquidity, or FX inputs are missing or stale at the moment a reservation would be created?

**Binding rules:** DET-02 §6 and RISK-INV-06. DET-09-D03 and MD-INV-01, MD-INV-05. CR-B06 P1-B06-D04: missing price, FX, market state, or unit is `INDETERMINATE` and not new risk. XC-07-FX is principle only. ARCH-DEC-01: no rounding mode is authorized.

**Sources:** `contracts/sources/DET-02-Risk-Contract-v1.0.md` §6. `contracts/sources/DET-09-bis-DET-10-und-Cross-Contract-Review-v1.0.md` §9.2 and §9.6. `contracts/sources/P1-Resolution-Contracts-CR-B05-bis-CR-B09-v1.0.md` CR-B06.

**Unresolved:** which fields count as required, and any numeric freshness limit. XC-07-ROUND stays OPEN.

**Alternatives:**

- **A — Fail-closed.** Missing, stale, unknown, or incomparable valuation yields no reservation and no `ALLOW`. Record the refusal if the local database is available. No fallback price and no 1:1 FX.
- **B — Scope block.** Same deny, and also block further risk-increasing commands for that instrument or account until a newer snapshot id is cited. Still no invented price.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** Use the last price, a zero FX rate, or a silent 1:1 conversion. Conflicts with MD-INV-05, DET-02 §6, and XC-07-FX.

| Alternative | Safety | Complexity | Recoverability | Auditability |
| --- | --- | --- | --- | --- |
| A | High per command. | Low. | The next command can succeed when inputs exist. | The refusal cites the missing input id. |
| B | Higher containment. | Needs a scope halt that is not a broker cancel. | The block needs an explicit clear, which is a separate open reset question. | The block and the refusal are both records. |
| C | Unsafe. A fake price becomes a limit decision. | Low and wrong. | The bad reservation has to be undone. | The invented input has no source. |

**Non-binding recommendation:** A. A scope block is RISKQ-07's subject and should not be bundled here. This recommendation is not an approval.

**Evidence needed:** a freshness policy with a number the user adopts. CR-B06 says not to invent one. None is adopted.

**Dependencies:** RISKQ-10. XC-07-ROUND. DET-09.

**Would not authorize:** a staleness number, a rounding mode, an FX conversion, or a reservation writer.

## RISKQ-04 — Partial fill, without posting

**Status:** `USER_DECISION_PENDING`

**Question:** How should a future design describe a partial fill across reservation, realized exposure, and residual obligation, while economic posting stays forbidden?

**Binding rules:** DET-02 §5 describes an atomic move from reservation to exposure. That move is a proposed economic effect. ARCH-DEC-13 quarantines it, including when identity looks unique. ARCH-DEC-14 forbids release of the residual. RISK-INV-05: one evidence event, at most one eventual effect.

**Sources:** `contracts/sources/DET-02-Risk-Contract-v1.0.md` §5 and RISK-INV-05. `contracts/decisions/ARCH-DEC-09-bis-20-2026-11-15.md` ARCH-DEC-13 and ARCH-DEC-14. `contracts/sources/P0-Resolution-Contracts-CR-B01-bis-CR-B04-v1.0.md` §3.2.

**Unresolved:** the attribution split. The current prohibition on performing the split is not unresolved.

**Alternatives:**

- **A — Fail-closed.** Record the fill. Leave the entire original reservation in place. Do not describe a conversion and do not release the residual.
- **B — Future split, not performed.** Write down that a later superseding decision could convert only the evidenced quantity and keep an audited residual. Choosing B does not perform the conversion and does not supersede ARCH-DEC-13.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** Book the partial fill now and release the residual. Conflicts with ARCH-DEC-13 and ARCH-DEC-14.

| Alternative | Safety | Complexity | Recoverability | Auditability |
| --- | --- | --- | --- | --- |
| A | Highest now. Capital stays held. | Low. | The full hold is still there when a bust arrives. | Evidence and hold are separate. |
| B | Safe only while the split stays on paper. | The paper split can be mistaken for a posting. | A later posting has a named residual to attach to. | The description must say it was not posted. |
| C | Unsafe. Residual release before finality. | A real ledger that is not authorized. | A late remainder or a bust finds no hold. | The booking looks decided. |

**Non-binding recommendation:** A. The future split is approach `reservation_to_exposure_with_residual` in the options note, and it stays comparison-only. This recommendation is not an approval.

**Evidence needed:** unique execution identity and a completeness proof before B could ever run. Both are `EVIDENCE_PENDING`.

**Dependencies:** RISKQ-06, RISKQ-08, RISKQ-09. XC-01, XC-02-PRED, XC-08-IBKR. ARCH-03B.3.

**Would not authorize:** a conversion, a residual release, a position posting, or an execution schema.

## RISKQ-05 — Cancel facts and residual obligation

**Status:** `USER_DECISION_PENDING`

**Question:** What residual risk obligation remains after a cancel request or a cancel observation when broker evidence is incomplete?

**Binding rules:** ARCH-DEC-12: keep the request, the observation, and every fill as separate facts, and do not move risk. DET-02 §5: cancel acknowledgement or `CancelRequested` does not release. CR-B09 P1-B09-D02: a cancel request is not confirmation, and a cancel does not prove non-execution when submission is unknown. ARCH-DEC-17: the observation is not terminal. ARCH-DEC-20 does not authorize cancel dispatch during a halt. XC-10-RACE is principle only.

**Sources:** `contracts/decisions/ARCH-DEC-09-bis-20-2026-11-15.md` ARCH-DEC-12, ARCH-DEC-17, ARCH-DEC-20. `contracts/sources/DET-02-Risk-Contract-v1.0.md` §5. `contracts/sources/P1-Resolution-Contracts-CR-B05-bis-CR-B09-v1.0.md` CR-B07 P1-B07-D03 and CR-B09 P1-B09-D02.

**Unresolved:** whether any future completeness proof could shrink a residual. The proof is not in this repository. XC-08-IBKR and XC-10-ACTION stay OPEN.

**Alternatives:**

- **A — Fail-closed.** Keep both facts. The residual obligation stays at the pre-cancel amount. An incomplete snapshot adds nothing and removes nothing.
- **B — Future shrink, blocked.** A later decision could shrink the residual only after a completeness proof and a predicate, both absent. Choosing B does not shrink anything now and does not adopt the predicate.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** Release on `CancelRequested`, on a cancel label, or on an empty snapshot. Conflicts with ARCH-DEC-12, ARCH-DEC-14, RISK-INV-04, and XC-08-RULE.

| Alternative | Safety | Complexity | Recoverability | Auditability |
| --- | --- | --- | --- | --- |
| A | High. A following fill still has a hold. | Low. | The hold matches the uncertainty. | Three facts stay distinct: request, observation, fills. |
| B | Safe only as a blocked future. | Easy to misread as permission. | A later proof has a place to attach. | The file must say the shrink did not happen. |
| C | Unsafe. The next fill is unreserved. | Low and wrong. | Budget is already gone. | The release cites a non-terminal fact. |

**Non-binding recommendation:** A. This recommendation is not an approval.

**Evidence needed:** IBKR cancel semantics and a scoped completeness proof. `EXTERNAL_EVIDENCE_PENDING`. ARCH-03B.3.

**Dependencies:** RISKQ-09, XC-08-IBKR, XC-01, XC-10-ACTION. Does not authorize the cancel command itself.

**Would not authorize:** a cancel dispatch, a terminal broker edge, a residual release, or a workflow edge from request to cancelled.

## RISKQ-06 — Late fill, duplicate, bust, and correction

**Status:** `USER_DECISION_PENDING`

**Question:** How are late fills, duplicate fills, busts, and corrections retained and quarantined under ARCH-DEC-13?

**Binding rules:** ARCH-DEC-13 quarantines every proposed economic effect, including one match. ARCH-DEC-09 keeps the original closure. ARCH-DEC-16: identical replay returns the stored result and creates no new economic effect; every other conflict stays quarantined. CR-B03 §3.1 and §3.2: append-only, no rewrite, no silent reopen, a bust can raise or lower risk. P0-B04-INV-01: ambiguous identity is not booked. CR-B04 §4.3 step 4's automatic booking on `MATCHED` is not available under ARCH-DEC-13.

**Sources:** `contracts/decisions/ARCH-DEC-09-bis-20-2026-11-15.md` ARCH-DEC-09, ARCH-DEC-13, ARCH-DEC-16. `contracts/sources/P0-Resolution-Contracts-CR-B01-bis-CR-B04-v1.0.md` §3 and §4.3. `contracts/sources/DET-01-Order-Workflow-and-Finality-Contract-v1.0.md` §5.

**Unresolved:** the shape of a future compensating entry. Whether to post is not unresolved: posting is forbidden.

**Alternatives:**

- **A — Fail-closed.** Retain every late, duplicate, bust, and correction fact. Quarantine every proposed effect. Draft no compensating ledger line. Leave the closure unchanged.
- **B — Describe, do not post.** Same retention and quarantine, plus an unratified description of a future compensating entry that would be append-only and would not delete a released reservation. The description is not a posting and does not supersede ARCH-DEC-13.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** Auto-post a single match, a bust reversal, or a correction. Conflicts with ARCH-DEC-13 and with the no-posting reading of ARCH-DEC-16.

| Alternative | Safety | Complexity | Recoverability | Auditability |
| --- | --- | --- | --- | --- |
| A | Highest. Nothing economic moves. | Low. | Evidence is still there for a later decision. | Facts are the audit. |
| B | Safe if the description cannot be executed. | A sketched entry can be copied into code. | The later entry has a named shape. | The sketch must be labeled unratified. |
| C | Unsafe. A wrong match becomes a position. | A posting path. | Reversal is another posting, still possibly wrong. | The book hides the quarantine that was required. |

**Non-binding recommendation:** A. CR-B03's compensating entry stays `REFERENCE_PROPOSAL` inside the options note. This recommendation is not an approval.

**Evidence needed:** XC-01 uniqueness and a correction-chain example from the target broker. Not in this repository.

**Dependencies:** RISKQ-04, RISKQ-13. XC-01, XC-02-PRED, XC-07-ROUND. ARCH-03B.3 for the identity match.

**Would not authorize:** a posting, a compensation formula, a closure rewrite, or a release.

## RISKQ-07 — Insufficient budget, halt, and reduction

**Status:** `USER_DECISION_PENDING`

**Question:** How should insufficient budget and a possible limit breach be classified, and what authorization exists to reduce risk, without approving a broker action?

**Binding rules:** DET-02 §7: WARN, BLOCK, HALT, and SAFE_REDUCTION are different. A negative quantity is not automatically risk-reducing. RISK-INV-07. CR-B09 P1-B09-D04: a reduction that is not proved non-increasing is `DENY`. ARCH-DEC-20 does not authorize a risk-reducing action and does not authorize cancel during a halt. DET-10-D06: a halt is not proof the broker cancelled. OPS-INV-04.

**Sources:** `contracts/sources/DET-02-Risk-Contract-v1.0.md` §7 and RISK-T11. `contracts/sources/P1-Resolution-Contracts-CR-B05-bis-CR-B09-v1.0.md` CR-B09. `contracts/decisions/ARCH-DEC-09-bis-20-2026-11-15.md` ARCH-DEC-20. `contracts/sources/DET-09-bis-DET-10-und-Cross-Contract-Review-v1.0.md` DET-10-D06.

**Unresolved:** incident severity names, and the proof that an action reduces risk. XC-10-ACTION and XC-09-RESET stay OPEN. `IncidentState` is absent.

**Alternatives:**

- **A — Fail-closed.** Deny the new risk-increasing command when budget is insufficient. If already-held evidence would read as over a limit, block new risk-increasing actions in that scope and keep an incident record. Authorize no reduction and no broker cancel.
- **B — Checklist only.** Same blocks, plus a future non-binding checklist titled `RISK_REDUCE` that would require every affected metric to be shown non-increasing. The checklist authorizes no command.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** Automatically submit a reducing order or a cancel when a limit breaks. Conflicts with ARCH-DEC-20, CR-B09 until broker semantics exist, and XC-10-ACTION.

| Alternative | Safety | Complexity | Recoverability | Auditability |
| --- | --- | --- | --- | --- |
| A | High. No new broker action. | Low. | The block remains until a separate reset decision. | Deny and halt are explicit. |
| B | High if the checklist cannot dispatch. | The name `RISK_REDUCE` is easy to implement by mistake. | The checklist is a future test list. | Each metric comparison would need a source. |
| C | Unsafe. A "reduce" can raise risk or duplicate an order. | A dispatcher. | A bad reduce is a real fill. | The order looks authorized. |

**Non-binding recommendation:** A. This recommendation is not an approval.

**Evidence needed:** a proved non-increase test and IBKR action semantics. The test is `NOT_EXECUTED`. The semantics are `EXTERNAL_EVIDENCE_PENDING`.

**Dependencies:** RISKQ-01, RISKQ-13. XC-10-ACTION, XC-09-RESET. Does not decide cancel-during-halt.

**Would not authorize:** a broker order, a cancel, a modify, a break-glass bypass, or a negative budget.

## RISKQ-08 — Future evidence bar, under ARCH-DEC-14

**Status:** `USER_DECISION_PENDING`

**Question:** May a future evidence list be named as a candidate bar for economic finality, given that ARCH-DEC-14 forbids an executable predicate?

**Binding rules:** ARCH-DEC-14. Checklist B in the OMS proposal was not adopted. XC-02-PRED stays OPEN. XC-02-ALIGN is the principle that closure is not finality, not a predicate.

**Sources:** `contracts/decisions/ARCH-DEC-09-bis-20-2026-11-15.md` ARCH-DEC-14. `docs/architecture/TRD-ARCH-SYNC-01-report.md` section "2. Risk reservation finality". The candidate categories in `TRD-ARCH-03B2-economic-finality-and-release-coverage-draft.md` §1 are `NEW_PROPOSAL`.

**Unresolved:** whether the candidate list should remain an unratified note or be named as a future acceptance bar that is still not executable. Sufficiency is not on offer.

**Alternatives:**

- **A — Fail-closed.** Adopt no list. Economic finality stays undefined. The coverage draft remains an unratified note.
- **B — Named bar, not a predicate.** Call the coverage draft's evidence categories a future acceptance bar. The bar does not evaluate, does not pass, and does not release. It is not checklist B from the OMS package silently adopted.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** Adopt an executable predicate and allow release when the list is checked. Conflicts with ARCH-DEC-14. Not available without an explicit superseding decision.

| Alternative | Safety | Complexity | Recoverability | Auditability |
| --- | --- | --- | --- | --- |
| A | Highest. No list can be coded as a boolean. | Lowest. | A later predicate starts from a blank approval. | There is no false "passed" record. |
| B | Safe only with an explicit non-executable label. | The bar will be copied into a function. | Reviewers can see which facts were contemplated. | Each category still needs a source citation. |
| C | Unsafe. A checklist becomes finality. | A predicate. | False finals are positions. | The pass mark hides missing broker proof. |

**Non-binding recommendation:** A. Naming the bar is a separate temptation this package should not decide by recommendation-as-adoption. The coverage draft already exists as `NOT_RATIFIED`. This recommendation is not an approval.

**Evidence needed:** before B could be promoted further, XC-01, XC-07-ROUND, and XC-08-IBKR would have to be evidenced. They are not.

**Dependencies:** ARCH-DEC-14. RISKQ-09. XC-02-PRED.

**Would not authorize:** a predicate, a release, a posting, or a claim that any scenario in the coverage draft has passed.

## RISKQ-09 — Future release proofs

**Status:** `USER_DECISION_PENDING`

**Question:** What proof obligations would a future reservation release have to meet, given that release stays forbidden now?

**Binding rules:** DET-02 Q06 and RISK-INV-04. ARCH-DEC-14 forbids release. ARCH-DEC-12: an evidence requirement is not satisfaction of that requirement. CR-B07: missing history is not release.

**Sources:** `contracts/sources/DET-02-Risk-Contract-v1.0.md` §2 DET02-Q06, §5, RISK-INV-04. `contracts/decisions/ARCH-DEC-09-bis-20-2026-11-15.md` ARCH-DEC-14 and ARCH-DEC-12. `contracts/sources/P1-Resolution-Contracts-CR-B05-bis-CR-B09-v1.0.md` CR-B07 P1-B07-D03.

**Unresolved:** the contents of a future proof. The prohibition is resolved and stays in force.

**Alternatives:**

- **A — Fail-closed.** Accept no proof obligation as sufficient. No operation named release is enabled. The word stays a forbidden effect.
- **B — Obligations listed, still blocked.** If a superseding decision ever allows a release, that release would have to cite a predicate that is still undefined, a complete scoped snapshot, a unique identity, an empty quarantine, a fresh valuation version, an audit record, and a CAS version. Choosing B does not define the predicate and does not allow the release.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** Release on workflow closure, timeout, cancel acknowledgement, a filled label, or an empty snapshot. Conflicts with RISK-INV-04 and ARCH-DEC-14.

| Alternative | Safety | Complexity | Recoverability | Auditability |
| --- | --- | --- | --- | --- |
| A | Highest. | Lowest. | Nothing was released, so nothing must be restored. | No release record exists to mistake for success. |
| B | The list is a future gate and a present hazard if coded. | High. | Each cited input can be replayed. | The list is long and still not sufficient today. |
| C | Unsafe. | Low. | A following fill has no budget. | The release cites the wrong fact. |

**Non-binding recommendation:** A. Listing B's obligations in a decision would be read as progress toward release. The coverage draft already shows why those facts are not sufficient. This recommendation is not an approval.

**Evidence needed:** the predicate and the broker proofs. Absent.

**Dependencies:** RISKQ-08, XC-02-PRED, XC-01, XC-08-IBKR. ARCH-DEC-14.

**Would not authorize:** a release, a predicate, or a statement that any current order is economically final.

## RISKQ-10 — Valuation, FX, and rounding governance

**Status:** `USER_DECISION_PENDING`

**Question:** How are valuation, FX, precision, and rounding governed, versioned, and failed closed, without adopting a rounding policy?

**Binding rules:** DET-02 §6. DET-09-D04. CR-B06 P1-B06-D01 through D04, including the instruction not to invent a universal slippage or staleness number. UD-03: modes exist only with a validated quantum and a business policy, and `RejectInexact` is a vocabulary default rather than an applied operation. `FINANCIAL_ROUNDING_IMPLEMENTED` is false. `RoundingPolicy` does not round. ARCH-DEC-01 exception: no rounding mode is authorized. XC-07-ROUND OPEN. XC-07-FX principle: missing FX is indeterminate.

**Sources:** `contracts/sources/DET-02-Risk-Contract-v1.0.md` §6. `contracts/sources/M0-02A-UD-01-bis-UD-09-Architecture-Resolution-v1.0.md` UD-03. `contracts/sources/P1-Resolution-Contracts-CR-B05-bis-CR-B09-v1.0.md` CR-B06. `contracts/domain/glossary.md` section "Money, price, quantity, and time".

**Unresolved:** the mode, the quantum, the staleness limit, and the FX source. Fail-closed on a gap is the baseline and is not a mode.

**Alternatives:**

- **A — Fail-closed.** Choose no mode, no rate, and no threshold. A gap blocks a new risk-increasing reservation. Exact comparisons stay inside the existing shell. No new arithmetic.
- **B — Version fields only.** Require a future valuation record to be able to cite instrument policy version, market snapshot, FX snapshot, pricing method, rounding-policy version, base currency, and as-of time, as CR-B06 lists them. Cite them only. Choose no mode and no number.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** Pick `HalfEven`, a directional reserve rounding, or a universal staleness number now. Conflicts with the ARCH-DEC-01 exception, UD-03, and XC-07-ROUND.

| Alternative | Safety | Complexity | Recoverability | Auditability |
| --- | --- | --- | --- | --- |
| A | High. No silent rescale. | Low. | A later policy is a new version. | The gap is the record. |
| B | High if the fields are not treated as a computation. | The seven names look like a schema. | Replay can point at versions once they exist. | Citations without values are honest and incomplete. |
| C | Unsafe. A mode without a quantum changes money. | A calculator. | Rounded history is hard to replay exactly. | The mode looks approved. |

**Non-binding recommendation:** A. CR-B06's field list is `REFERENCE_PROPOSAL` under ARCH-DEC-04. This recommendation is not an approval.

**Evidence needed:** instrument master data and IBKR FX and margin semantics. CR-B06 marks that verification open. `EXTERNAL_EVIDENCE_PENDING`.

**Dependencies:** XC-07-ROUND, XC-07-FX. RISKQ-03. No dependency on a release decision, because no figure is being released.

**Would not authorize:** a rounding operation, an FX conversion, a tick rule, a stress metric, or a change to `RoundingPolicy`.

## RISKQ-11 — Isolation, locks, CAS, and retries

**Status:** `USER_DECISION_PENDING`

**Question:** What future constraint applies to transaction isolation, lock ordering, compare-and-swap, and crash retries, without adopting the CR-B02 lock order or `READ COMMITTED`?

**Binding rules:** ARCH-DEC-18. The CR-B02 §2.2 order and the §2.3 `READ COMMITTED` default stay proposals. A future retry is local only and never resends. DET-01 Q04 names CAS and targeted locks as a principle. DET-02 §4 item 2 points at the CR-B02 order and does not override ARCH-DEC-18. P0-B02-INV-04. XC-05-OUTBOX is the principle that broker and NATS stay outside the local commit. XC-05-LOCK stays OPEN. CR-B01 precedence over less restrictive DET-07 dispatch text stays in force (ARCH-DEC-03) and is not a lock protocol.

**Sources:** `contracts/decisions/ARCH-DEC-09-bis-20-2026-11-15.md` ARCH-DEC-18. `contracts/sources/P0-Resolution-Contracts-CR-B01-bis-CR-B04-v1.0.md` §2.2 and §2.3. `docs/architecture/TRD-ARCH-SYNC-01-report.md` section "5. Outbox transaction boundary".

**Unresolved:** the protocol and the isolation level. Deadlock and serialization tests were not run. This package does not claim they passed.

**Alternatives:**

- **A — Fail-closed.** Leave the lock order and the isolation level unadopted. State again that any future retry is the local unit only.
- **B — Proof obligation, no protocol.** A later protocol, if one is proposed, must cover multi-scope budgets, order-version CAS, and crash before and after commit, and must be shown by tests that do not yet exist. B names no order and no isolation level.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** Adopt `READ COMMITTED` and the six-step CR-B02 order as canonical now. Conflicts with ARCH-DEC-18.

| Alternative | Safety | Complexity | Recoverability | Auditability |
| --- | --- | --- | --- | --- |
| A | High. No false canonical protocol. | Low. | Nothing concurrency-related is deployed to undo. | The decision record already says unadopted. |
| B | High if the tests stay required. | The obligation can be mistaken for the protocol. | The future tests are the recovery evidence. | Test ids would have to stay `NOT_EXECUTED` until run. |
| C | Unsafe as a silent adoption. The order may deadlock or miss a predicate. | A schema and a lock path. | A wrong order is a production incident. | Lock waits are not the audit ARCH-DEC-19 requires. |

**Non-binding recommendation:** A. B's test list is already the coverage draft's `NOT_EXECUTED` section. Putting it in a decision adds no safety. This recommendation is not an approval.

**Evidence needed:** PostgreSQL deadlock and serialization tests under a future user decision and ARCH-DEC-07 evidence rules. Not supplied. The ignored connectivity test in this repository is not those tests.

**Dependencies:** XC-05-LOCK. ARCH-DEC-18. RISKQ-02. A persistence task that is not started.

**Would not authorize:** a lock order, an isolation level, a trading transaction, a broker retry, or a claim that deadlock tests passed.

## RISKQ-12 — Audit and reconciliation evidence

**Status:** `USER_DECISION_PENDING`

**Question:** What audit authority and reconciliation evidence must accompany a future reservation change, without naming a table or enabling a posting?

**Binding rules:** ARCH-DEC-19: a distinct in-transaction audit record, and the mutation fails if it cannot be written. No table name. DET-02 Q07. DET-04-D01 does not name `audit_events`. CR-B08 P1-B08-D01 does. ARCH-DEC-04 assigns no precedence, and ARCH-DEC-19 does not adopt the name. OPS-INV-01 and OPS-INV-05. XC-06 OPEN. An outbox or export is not the authority (CR-B08 P1-B08-D02, XC-05-OUTBOX principle).

**Sources:** `contracts/decisions/ARCH-DEC-09-bis-20-2026-11-15.md` ARCH-DEC-19. `contracts/sources/DET-03-bis-DET-08-Architecture-Contracts-v1.0.md` DET-04-D01. `contracts/sources/P1-Resolution-Contracts-CR-B05-bis-CR-B09-v1.0.md` CR-B08. `docs/architecture/TRD-ARCH-SYNC-01-report.md` section "6. Audit transaction boundary".

**Unresolved:** the artifact. The requirement to have one is not unresolved.

**Alternatives:**

- **A — Fail-closed.** Restate ARCH-DEC-19 for any future reservation change: distinct audit record in the same local unit, or the change does not commit. Reconciliation evidence is retained when booking is blocked. No name and no posting.
- **B — Citation list.** Same rule, plus a future record would cite reservation id, command id, policy version, and evidence refs. The list is not a schema and not a table.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** Use the outbox or an export as the audit authority, or adopt the name `audit_events` now. Conflicts with ARCH-DEC-19.

| Alternative | Safety | Complexity | Recoverability | Auditability |
| --- | --- | --- | --- | --- |
| A | Matches the binding rule. | Low. | The record is in the same commit as the change, once a change exists. | Authority stays one record, unnamed. |
| B | High if the list is not DDL. | Four names are almost a schema. | Citations help replay. | Better trace, higher chance of a premature table. |
| C | Unsafe. Export loss would erase authority. | Two authorities. | The export can disagree with the commit. | The wrong store becomes the story. |

**Non-binding recommendation:** A. This recommendation is not an approval.

**Evidence needed:** a later decision that resolves the DET-04 / DET-10 / CR-B08 name disagreement. Not this question.

**Dependencies:** XC-06. ARCH-DEC-19. RISKQ-14 for replay of the record.

**Would not authorize:** a table, DDL, a hash chain, a retention period, a posting, or a release.

## RISKQ-13 — Manual adjudication

**Status:** `USER_DECISION_PENDING`

**Question:** What manual adjudication and second-person rule applies to quarantined economic effects, without inventing a second approver?

**Binding rules:** ARCH-DEC-13: manual review is not authorization to post. ARCH-DEC-16: no adjudicator role is approved; if a second approver is required and unavailable, deny; if authorization evidence is absent, stay quarantined. A simulated second approval is not authorized. CR-B05 P1-B05-D01: no fake dual approval. XC-09-SOLO is that principle. XC-09-RESET stays OPEN for roles, reset evidence, and halt-version CAS. ARCH-DEC-16 does not choose a two-person exception.

**Sources:** `contracts/decisions/ARCH-DEC-09-bis-20-2026-11-15.md` ARCH-DEC-13 and ARCH-DEC-16. `contracts/sources/P1-Resolution-Contracts-CR-B05-bis-CR-B09-v1.0.md` CR-B05. `docs/architecture/TRD-ARCH-SYNC-01-report.md` section "9. Incident, readiness, and reset".

**Unresolved:** whether the risk package adds a second-person requirement beyond ARCH-DEC-16. Roles are not chosen.

**Alternatives:**

- **A — Fail-closed.** Add no adjudication workflow and no role. Quarantine stands. Where a future rule requires a second person and none exists, deny. No simulated approver. Review does not post.
- **B — Future record shape.** If a later decision authorizes an adjudication record, that record would cite evidence ids, actor, expected order version, and reason, and would still not post. B does not create the actor and does not choose a two-person exception.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** A solo operator self-approves a posting, or a single identity match skips review. Conflicts with ARCH-DEC-13, ARCH-DEC-16, and CR-B05.

| Alternative | Safety | Complexity | Recoverability | Auditability |
| --- | --- | --- | --- | --- |
| A | Highest. No one can post by reviewing. | Low. | The quarantine is intact. | There is no fake approval to audit. |
| B | Safe if the record cannot post. | A form without a writer. | The citations are what a later reviewer would need. | The record is not the economic effect. |
| C | Unsafe. One person becomes the book. | A bypass. | A bad post needs another post to reverse. | The approval is theatre. |

**Non-binding recommendation:** A. ARCH-DEC-16 already refused to choose roles. This recommendation is not an approval.

**Evidence needed:** an identity system and a real second person, if a later decision requires one. Not present. No second approver is invented here.

**Dependencies:** XC-09-RESET, XC-09-SOLO. RISKQ-06. Does not decide reset.

**Would not authorize:** a role, a posting, a simulated approver, a reset, or break-glass bypass of a hard risk limit.

## RISKQ-14 — Recovery, replay, and policy versions

**Status:** `USER_DECISION_PENDING`

**Question:** What recovery, replay, and test rule applies to reservation state and to a future risk-policy change?

**Binding rules:** DET-02 Q10 and §9: the named tests are requirements, not results. DET-01 §7 and OMS-T12: unknown send is not a blind retry. RC-INV-01, RC-INV-03, RC-INV-05. ARCH-DEC-11. ARCH-DEC-15 is an OMS matrix bar and is not a risk-policy version. ARCH-DEC-16 limits automatic replay to the identical payload. A policy input is versioned in the baseline (DET-02 §3) and the version catalogue is open.

**Sources:** `contracts/sources/DET-02-Risk-Contract-v1.0.md` §9 and §10. `contracts/sources/DET-03-bis-DET-08-Architecture-Contracts-v1.0.md` DET-08 invariants. `contracts/decisions/ARCH-DEC-09-bis-20-2026-11-15.md` ARCH-DEC-11, ARCH-DEC-15, ARCH-DEC-16.

**Unresolved:** the test suite and the version-migration rule. Execution of the suite is not claimed.

**Alternatives:**

- **A — Fail-closed.** No recovery path releases risk or resubmits. Identical-payload replay returns the stored result only. Every named risk test stays `NOT_EXECUTED`. A policy-version change does not rewrite historical evaluations.
- **B — Version pin.** Same prohibitions, plus a future policy change must be a new immutable version id. Historical evaluations keep the id they cited. Negative tests are required before any later implementation and are not run now.
- **C — `CONFLICTS_WITH_BINDING_POLICY`.** Restart or reconciliation releases reservations the broker snapshot does not show, or retries the broker send. Conflicts with RC-INV-03, RC-INV-04, ARCH-DEC-11, and ARCH-DEC-14.

| Alternative | Safety | Complexity | Recoverability | Auditability |
| --- | --- | --- | --- | --- |
| A | High. | Low. | State is the last local commit plus retained evidence. | Tests are honestly unrun. |
| B | High if the new id cannot retcon history. | Version bookkeeping. | Replay uses the old id. | The id is the lineage. |
| C | Unsafe. Absence becomes release, and a retry can double-send. | A hidden migration. | Released budget and a live order. | The snapshot gap is mislabeled as proof. |

**Non-binding recommendation:** A. B is a useful future constraint and is not needed to keep today's tests unrun and today's release forbidden. This recommendation is not an approval.

**Evidence needed:** the property, database, fault-injection, and broker suites named in DET-02 §10. `NOT_EXECUTED`. Broker cases depend on ARCH-03B.3.

**Dependencies:** RISKQ-11, XC-08-IBKR, XC-09-RESET, XC-03-MATRIX for any workflow edge the recovery might suggest. No edge is approved.

**Would not authorize:** a test pass, a policy migration, a release, a resubmit, a transition matrix, or live trading.

## What an answer would still leave blocked

A user decision on RISKQ-01 through RISKQ-14 may record a future design constraint. It would still leave implementation blocked. It would not approve economic posting, an economic-finality predicate, reservation release, an OMS transition, a canonical schema, broker dispatch, or live trading. `XC-02-PRED` would stay OPEN until a decision explicitly about that predicate says otherwise, and RISKQ-08 option A and option B both refuse to adopt the predicate. Option C is not available.

## Resolution index (2026-11-15, TRD-ARCH-03B.2.2)

The questions RISKQ-01 through RISKQ-14 above are unchanged historical text, including their alternatives, consequence tables, and non-binding recommendations. They are not rewritten as approvals. The creation-time status lines stay as they were written. This addendum does not change those lines and does not record a second decision. The historical proposal index stays `NOT_RATIFIED`.

On 2026-11-15 the user ratified a design-policy choice for each question. The decision record is [ARCH-DEC-21-bis-34-2026-11-15.md](../decisions/ARCH-DEC-21-bis-34-2026-11-15.md). The machine-readable map is [TRD-ARCH-03B22-resolution-index.json](TRD-ARCH-03B22-resolution-index.json). That map is `NON_RUNTIME_NON_WIRE_RESOLUTION_INDEX`. It is not a signature and not an evidence URI. The record labels there are documentary. They are not a canonical status enum. **NO IMPLEMENTATION.** **NO VERIFICATION.**

| Question | Decision | Option | User wording |
| --- | --- | --- | --- |
| RISKQ-01 | ARCH-DEC-21 | B | Zusätzlich harte Ablehnungen als Konjunktionsregel festlegen. |
| RISKQ-02 | ARCH-DEC-22 | A | Eine Reservierungs-ID, Idempotenz und atomare Multi-Scope-Reservierung. |
| RISKQ-03 | ARCH-DEC-23 | A | Keine Reservierung bei fehlenden oder veralteten Daten. |
| RISKQ-04 | ARCH-DEC-24 | A | Fill erfassen; gesamte Reservierung halten; keine Umbuchung. |
| RISKQ-05 | ARCH-DEC-25 | A | Restverpflichtung bleibt unverändert. |
| RISKQ-06 | ARCH-DEC-26 | A | Alle Fakten behalten und wirtschaftliche Effekte quarantänisieren. |
| RISKQ-07 | ARCH-DEC-27 | A | Neue risikosteigernde Aktionen blockieren; keine Broker-Aktion. |
| RISKQ-08 | ARCH-DEC-28 | A | Keine Nachweisliste als verbindlich übernehmen. |
| RISKQ-09 | ARCH-DEC-29 | A | Keinen Freigabenachweis als ausreichend anerkennen. |
| RISKQ-10 | ARCH-DEC-30 | B | Künftige Bewertungs- und Policy-Referenzen versionierbar machen. |
| RISKQ-11 | ARCH-DEC-31 | B | Zusätzlich verpflichtende zukünftige Concurrency- und Crash-Tests. |
| RISKQ-12 | ARCH-DEC-32 | A | Separaten transaktionalen Auditdatensatz verlangen, ohne Tabellennamen. |
| RISKQ-13 | ARCH-DEC-33 | A | Keine Rollen oder Freigabewege einführen; Quarantäne bleibt. |
| RISKQ-14 | ARCH-DEC-34 | B | Zusätzlich unveränderliche Policy-Versionen und negative Tests verlangen. |

The proposal package above stays a proposal. Its header remains the creation-time request. The later record approves design policy only. It does not approve budget arithmetic, a reservation writer, a conversion, a residual release, a compensating ledger line, an economic-finality predicate, economic posting, a rounding mode, an FX conversion, a lock order, an isolation level, an audit table, a role, a policy migration, an OMS transition, broker dispatch, or live trading. Live trading stays NOT AUTHORIZED. `XC-02-PRED` stays OPEN. ARCH-DEC-13 and ARCH-DEC-14 stay binding. RISKQ-01 option B is conjunction shape only. RISKQ-08 option A and RISKQ-09 option A adopt no predicate and no release. No evidence record is created here. ARCH-03B.3 is not started.
