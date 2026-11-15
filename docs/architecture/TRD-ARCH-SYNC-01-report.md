# TRD-ARCH-SYNC-01 report

**Date:** 2026-11-15  
**Mode:** documentation and provenance. No trading behavior was added.  
**Live trading:** NOT AUTHORIZED  
**Canonical schemas:** not published  
**Remote CI:** not run

This report records what was compared. It does not approve a contract, close a conflict, or implement a type.

## Baseline

| Item | Value |
| --- | --- |
| Branch | `main` |
| Worktree before edits | clean |
| HEAD | `41c518830a4f90626a4fa4100b9b9347490f6ca8` |
| HEAD subject | `TRD-M002A.2: Implement Resolved Domain Type Contracts` |
| Parent | `d0398bcc8710a36659bcf1d0240cb43112e61f2c` `M0-02A: Record UD-01..UD-09 and keep canonical schemas unpublished` |
| Destructive git | none |

`41c5188` is the latest TRD-M002A.2 commit on this branch. No changes were discarded.

## Package verification

ZIP: `TRD-ARCH-SYNC-01-Source-Package-v1.0.zip`  
SHA-256: `72e6fbfc6657561fe8362ab74c6f76db0729f3c1baee58c2026ba101bc222d0b`  
Bytes: 66115  
The ZIP is not stored in the repository.

The ZIP's `source-manifest.json` (SHA-256 `f56024d1ca067ae3b12f24649cd7123d822e44c5d631dc88825b69d91ebdbf3c`, 2469 bytes) lists 12 files. Each extracted file matched that manifest's SHA-256 and byte length. Mismatches: 0. The package manifest was not copied into `contracts/sources/`, so the repository has one provenance manifest.

Duplicate compare, SHA-256 and a byte compare:

| File | Existing repository SHA-256 | ZIP SHA-256 | Result |
| --- | --- | --- | --- |
| `CR-B10-Domain-Glossary-and-Schema-Registry-v1.0.md` | `3dfe82fc187fd8cd2703f1113ffd4e1c03d18849ec5c010ed3026740144e87b9` | same | reused, not overwritten |
| `M0-02A-UD-01-bis-UD-09-Architecture-Resolution-v1.0.md` | `b9bbb7c28634bbf906a2c5fb6dac685a4a4f5bccacfe37a4f543a3feb308ae17` | same | reused, not overwritten |

The prompt archive `docs/cursor-prompts/trading/M0-02A-UD-01-bis-UD-09-Architecture-Resolution-v1.0.md` has the same SHA-256. No `CONFLICT_REQUIRES_REVIEW` duplicate.

## Source inventory

| File | Bytes | SHA-256 | Placement |
| --- | --- | --- | --- |
| `DET-01-Order-Workflow-and-Finality-Contract-v1.0.md` | 10277 | `a79bec0d8084594a0f21399c453ab82e2499e237f9deabc2222ae8e28afbadad` | copied |
| `DET-02-Risk-Contract-v1.0.md` | 10594 | `edbdbc13ac25d9f4a0daa7624d1506353ad4f183aa6a02febec69500582e1646` | copied |
| `DET-03-bis-DET-08-Architecture-Contracts-v1.0.md` | 25363 | `330e8d6d20768d722a8b03dd0fb57fc549f8bdcbde39d55a5c95a968aebcb6c3` | copied |
| `DET-09-bis-DET-10-und-Cross-Contract-Review-v1.0.md` | 24778 | `bfe68c71440be3324e97a41fd4ef885783f252e17b3135937239516ed3537895` | copied |
| `P0-Resolution-Contracts-CR-B01-bis-CR-B04-v1.0.md` | 25957 | `5e55ea07ada2df9ff31a132e9cb42d769488bdb4c4cc7c22c143fe7bbea9a2bc` | copied |
| `P1-Resolution-Contracts-CR-B05-bis-CR-B09-v1.0.md` | 9562 | `9333b13bb6940effee045fddca326a20b54fc72aa3f7f3325fa924ad0b973ce8` | copied |
| `ADFR-Consolidated-v1.0.json` | 21514 | `4c587b94f668e34e9c088650bbcc4d86ee51c90cbd3602281ce2b0e93931452c` | copied |
| `ADFR-Consolidated-v1.0.csv` | 11831 | `242e4a1205ee1a050218fd52b6597af02e9badd13ac842e01df70116ca4dda13` | copied |
| `Architecture-Consolidation-and-Acceptance-v1.0.md` | 3676 | `334bcfcba5015534184d65c556d0cd2d308238bd76bf61429c2bfe6a877bfd67` | copied |
| `Implementation-Roadmap-and-Cursor-Prompt-Plan-v1.0.md` | 5213 | `37658b435462b368628fe2ba9c83520ca2f82db3703397fee66fab36aa3c8cb6` | copied |
| `CR-B10-Domain-Glossary-and-Schema-Registry-v1.0.md` | 13166 | `3dfe82fc187fd8cd2703f1113ffd4e1c03d18849ec5c010ed3026740144e87b9` | reused |
| `M0-02A-UD-01-bis-UD-09-Architecture-Resolution-v1.0.md` | 10850 | `b9bbb7c28634bbf906a2c5fb6dac685a4a4f5bccacfe37a4f543a3feb308ae17` | reused |

Registry skeleton hash, unchanged: `contracts/registry/registry.json` SHA-256 `7abed2af5363726403175c6524bd7a4d05d36b8735ad48442fc26941f8d98197`. Flags remain `SKELETON_NOT_CANONICAL`, `canonical_schemas_published=false`, `part_b=NO-GO`, `broker_uniqueness_evidence=EVIDENCE_PENDING`, `live_trading_authorized=false`.

Pre-existing hashes, unchanged by this task's source copy: `contracts/domain/glossary.md` was edited only to record presence; `contracts/domain/unresolved-decisions.md` was edited only to record presence. UD implementation statuses were not changed. The M0-02A source file was not edited.

## Coverage

Section-level rows are in [contracts/traceability-index.md](../../contracts/traceability-index.md). Every ID DET-01..DET-10 and CR-B01..CR-B10 has a present file and a heading. ADFR JSON and CSV are present. The 2026-11-14 `NOT_PROVIDED` table is kept above the current table.

DET-01 and DET-02 are `RECONSTRUCTED_DRAFT` / `SOURCE_REVIEW_PENDING`. Their own §0 says the original Q&A primary text is not in the file. JSON `CONFIRMED_DECISION` for those twenty entries cites `conversation: confirmed DET-01/02 Q01-Q10`, not these markdown files. This report does not promote the drafts to ratified primary text.

## ADFR count

| Check | Result |
| --- | --- |
| JSON parses | yes |
| JSON `entries` | 77 unique |
| JSON root status | `ARCHITECTURE_DRAFT_NOT_IMPLEMENTED` |
| JSON `verification` | 77 `NOT_VERIFIED` |
| JSON `CONFIRMED_DECISION` | 20, only ADFR-DET01-001..010 and ADFR-DET02-001..010 |
| JSON other status | 57 `PROPOSED_FOR_ACCEPTANCE` |
| CSV parses | yes, 78 data rows, 78 unique IDs |
| Shared ID field compare | 0 mismatches on `decision`, `source`, `status`, `basis`, `verification` |
| CSV-only ID | `ADFR-CR-B10-001`, status `DESIGN_RESOLVED` |
| Stated 77 | matches the JSON count. Does not match the CSV count. |
| Consolidation `## ADFR-Konsolidierung` | says 78. The same bullet's breakdown sums to 77. |

Finding XC-11 below. The register was not edited to force one count. Semantic equivalence of JSON and CSV is not claimed.

## Cross-contract comparison

Read-only. Ten domains. A `NO_CONFLICT_FOUND` row is agreement on a fail-closed principle already written in the cited sections. It is not a test result and not permission to implement. An `OPEN` row is unresolved. No new rule was invented to close one.

User approval means a later explicit decision is required before code treats the cited text as binding behavior. This task does not grant that approval.

### 1. Identity scope

| ID | Status | Severity | User approval |
| --- | --- | --- | --- |
| XC-01 | OPEN | P0 | yes, before any uniqueness constraint or verified scope |

Sources: DET-03 `### Entscheidung und Begründung` DET-03-D02 and `### Persistenz-Referenzdesign`; CR-B04 `## 4.1` and `## 4.2`; CR-B10 §1 rows `BrokerOrderAlias`, `BrokerPermIdAlias`, and `BrokerAccountKey`; M0-02A `## UD-02`.

DET-03-D02 lists `client_id` and `session_epoch` on the alias. CR-B10 and UD-02 say client or session "as appropriate" and require kind-specific keys. CR-B04 forbids a production unique constraint on `orderId`, `permId`, or `execId` until the scope hypothesis is evidenced, and it says the matrix is not a broker guarantee. CR-B10's `BrokerAccountKey` includes environment. The DET-03 reference row uses `account_id` and does not name environment.

The documents agree that a bare broker id is not an internal primary key and that ambiguity is not an automatic merge. They do not freeze the fields inside each kind's key. IBKR semantics are unverified (CR-B04 `## 4.5`, DET-03..08 `## 10` item 1).

Proposed resolution: keep `EVIDENCE_PENDING` and `UnverifiedSessionScope`. Do not add a unique constraint or a verified constructor. A later rule table has to name the key per kind after evidence, and it needs user approval.

### 2. Risk reservation finality

| ID | Status | Severity | User approval |
| --- | --- | --- | --- |
| XC-02-ALIGN | NO_CONFLICT_FOUND | P0 topic, principle only | no new approval for the principle |
| XC-02-PRED | OPEN | P0 | yes, before any release or rebooking code |

XC-02-ALIGN sources: DET-02 `## 2` Q06 and `## 5`; CR-B03 `## 3.1` and `## 3.2`; CR-B10 §2 (`Closed` is not `Released`); DET-01 `## 3` and `## 5`. Workflow closure, timeout, and cancel acknowledgement are not reservation release. A fill after release is still economic evidence.

XC-02-PRED sources: DET-02 `## 4` item 6 and `## 10`; CR-B03 `## 3.2` and `## 3.3`; CR-B06 `## CR-B06` P1-B06-D03. No executable predicate defines "confirmed economic finality". The reference inequality `used + reserved + proposed_delta <= limit` is explicitly not a netting, margin, or stress rule. CR-B03's compensating ledger entry after `RELEASED` is a proposal, not an implemented posting rule. Budget metrics are not canonical.

Proposed resolution: do not implement release, rebooking, or the reference inequality. A future policy version has to define the predicate and the compensating entry, with user approval, before code.

### 3. OMS late fills

| ID | Status | Severity | User approval |
| --- | --- | --- | --- |
| XC-03-BOOK | NO_CONFLICT_FOUND | P0 topic, principle only | no |
| XC-03-MATRIX | OPEN | P0 | yes, before `OrderClosure` or any transition edge |

XC-03-BOOK sources: DET-01 `## 5`; CR-B03 `## 3.1`; CR-B10 §2; DET-08 decision DET-08-D04. A late fill is new evidence. It does not rewrite historical workflow closure or the original fill. DET-01 books it when identity is unambiguous. CR-B03 books it after identity resolution and only automatic economic booking on a single consistent match (CR-B04 `## 4.3`). That reading does not authorize booking an ambiguous fill.

XC-03-MATRIX sources: DET-01 `## 3` reference `OrderClosure` and `## 6`; M0-02A `## UD-06` and `## UD-07`. DET-01 says the reason vocabulary is not confirmed and that allowed edges are intentionally not listed. The file is a reconstructed draft. `OrderClosure` stays absent. `ORDER_STATE_MACHINE_IMPLEMENTED` and `STATE_TRANSITIONS_IMPLEMENTED` stay false.

Proposed resolution: do not invent closure reasons or edges.

### 4. Dispatch epoch fencing

| ID | Status | Severity | User approval |
| --- | --- | --- | --- |
| XC-04 | OPEN | P0 | yes, before any dispatcher or failover |

Sources: DET-07 decisions DET-07-D01 and DET-07-D03, `### Kritische Dispatch-Sequenz`, and `**Grenze des Epoch-Fencing:**`; CR-B01 `## 1.1` through `## 1.3`; Architecture Consolidation `## Verträge und Vorrang` item 2; DET-03..08 `## 10` item 2; ADFR-CROSS-002 in DET-09/10 `## ADFR-CROSS` (source table says `OPEN P0`) and in the JSON (`PROPOSED_FOR_ACCEPTANCE`, `NOT_VERIFIED`).

Shared principle: `DISPATCH_STARTED` is written before a possible send, an unknown outcome is not proof of non-send, and a database lease does not retract an in-flight socket write. CR-B01 is stricter than DET-07's "manual intervention when takeover is ambiguous": no automatic takeover while the old writer might still send or attempts are unclear, and no time-based auto-failover. The consolidation document says the P0 text refines DET-07. That precedence is itself a draft. Lease TTL, heartbeat, dispatch-scope boundary, and broker-session exclusivity are not specified and are not IBKR-verified. The Rust `WriterEpoch::checked_next` is not this fence (UD-04).

Proposed resolution: do not implement dispatch, lease TTL, or failover. Do not treat the source label `DESIGN_RESOLVED` as a passing fault-injection test.

### 5. Outbox transaction boundary

| ID | Status | Severity | User approval |
| --- | --- | --- | --- |
| XC-05-OUTBOX | NO_CONFLICT_FOUND | P1 topic, principle only | no |
| XC-05-LOCK | OPEN | P1 | yes, before a lock order or isolation level is treated as canonical |

XC-05-OUTBOX sources: DET-04 `### Zustell- und Commit-Protokoll`; CR-B02 `## 2.1`; DET-01 `## 4` items 4 and 5; CR-B10 §7 rule 10; DET-03..08 SYS-INV-03. The local commit includes the outbox row. Broker and NATS calls are outside that transaction. Publish after commit is at-least-once. A failed publish does not roll back the committed local write (DET-04 TX-INV-03).

XC-05-LOCK sources: CR-B02 `## 2.2` and `## 2.3`. The lock order and `READ COMMITTED` default are described as a contract proposal. Deadlock tests are not a substitute that has been run. `SERIALIZABLE` is allowed for selected transactions and is not the silent default.

Proposed resolution: do not create the trading unit of work or the lock order in code.

### 6. Audit transaction boundary

| ID | Status | Severity | User approval |
| --- | --- | --- | --- |
| XC-06 | OPEN | P1 | yes, before an audit table is chosen as the authority |

Sources: DET-04 decision DET-04-D01 (audit row not named); DET-10 decision DET-10-D02; CR-B08 `## CR-B08` P1-B08-D01 and D02; DET-01 `## 4` item 4; ADFR-CROSS-001 in DET-09/10 `## ADFR-CROSS` (`OPEN P0`) and in the JSON (`PROPOSED_FOR_ACCEPTANCE`, `NOT_VERIFIED`); P0 `## 2.1` and P1 CR-B08, which call the design resolved.

Shared direction: a safety-critical local commit needs authoritative audit evidence, and an export or NATS projection is not that authority (OPS-INV-01, CR-B08 D02). DET-04-D01 still does not name `audit_events`. DET-10-D02's two levels need the CR-B08 reading to avoid a second authority. The source documents disagree on whether this item is still `OPEN P0`, `DESIGN_RESOLVED`, or `PROPOSED_FOR_ACCEPTANCE`. Verification is `NOT_VERIFIED` in the JSON. Retention, signing keys, and backup are explicitly open (CR-B08 last sentence; DET-10-D08).

Proposed resolution: do not add an audit table, a second audit commit, or a hash-chain implementation. Do not pick a winner among the status words.

### 7. Market data, FX, and rounding

| ID | Status | Severity | User approval |
| --- | --- | --- | --- |
| XC-07-FX | NO_CONFLICT_FOUND | P0 topic, principle only | no |
| XC-07-ROUND | OPEN | P0 | yes, before any rounding mode is applied to a risk figure |

XC-07-FX sources: DET-02 `## 6`; DET-09 `## 9.3`, `## 9.5`, and MD-INV-05; CR-B06 P1-B06-D04. Missing or contradictory FX is indeterminate. There is no silent 1:1 conversion and no implicit FX.

XC-07-ROUND sources: CR-B06 P1-B06-D02 and D03; DET-02 `## 6`; DET-09-D04; M0-02A `## UD-03`; CR-B10 §3. CR-B06 proposes conservative directional rounding for buying power and released capacity, and separate tick and lot checks. UD-03 says a rounding mode is not a universal default and exists only when a caller supplies a validated quantum and a business policy. The Rust `RoundingPolicy` names modes and does not apply them. `FINANCIAL_ROUNDING_IMPLEMENTED` is false. No staleness, slippage, or margin number is stated, and CR-B06 says not to invent one. IBKR margin and FX semantics are unverified in CR-B06's own verification line.

Proposed resolution: do not apply a rounding mode, do not invent a threshold, and do not treat the vocabulary enum as permission to round.

### 8. Broker snapshot completeness

| ID | Status | Severity | User approval |
| --- | --- | --- | --- |
| XC-08-RULE | NO_CONFLICT_FOUND | P0 topic, principle only | no |
| XC-08-IBKR | OPEN | P0 | yes, before any adapter may report `COMPLETE` |

XC-08-RULE sources: DET-08 decision DET-08-D02 and RC-INV-04; CR-B07 P1-B07-D02 and D03; DET-01 `## 7`; CR-B01 `## 1.2` item 7. An empty or incomplete snapshot is not proof that an order is absent and is not `DEFINITIVELY_NOT_SENT`.

XC-08-IBKR sources: CR-B07 P1-B07-D01 and the verification line; P1 gate table `GATE-P1-07` = `EXTERNAL_VERIFICATION_REQUIRED`; DET-05 port sketch `snapshot`. Completeness is scoped on paper. IBKR completion signals, history retention, and account scope are not evidenced. No adapter exists in this repository.

Proposed resolution: do not treat an empty query as a negative proof. Do not implement a completeness flag.

### 9. Incident, readiness, and reset

| ID | Status | Severity | User approval |
| --- | --- | --- | --- |
| XC-09-SOLO | NO_CONFLICT_FOUND | P1 topic, principle only | no |
| XC-09-RESET | OPEN | P0 | yes, before reset authorization or incident transitions |

XC-09-SOLO sources: CR-B05 P1-B05-D01; DET-10 decision DET-10-D04 and DET-10-D05. If an action requires an independent second person and none is available, the answer is deny. A simulated second approval is not authorized. Break-glass does not bypass hard risk limits or fencing.

XC-09-RESET sources: DET-06 decision DET-06-D02 (`MANUAL_RESET_REQUIRED`, reference name, orthogonal flag allowed); CR-B10 §2 `TradingReadinessState::ManualReset`; DET-10 decision DET-10-D03 (incident states `DETECTED` through `CLOSED`, plus `REOPENED`); CR-B05 P1-B05-D03. The Rust label is `ManualReset`. DET-06's reference name differs, and DET-06 allows a flag instead of an enum variant. Neither form is an implemented reset. `IncidentState` is not a Rust type. DET-10's state list was not adopted. Reset evidence binding, CAS on halt version, and role enforcement are specified as design only (`GATE-P1-05`). HTTP readiness is a different check and is not `TradingReadinessState`.

Proposed resolution: do not add `IncidentState`, do not rename `ManualReset`, and do not implement reset or break-glass.

### 10. Cancel and modify

| ID | Status | Severity | User approval |
| --- | --- | --- | --- |
| XC-10-RACE | NO_CONFLICT_FOUND | P0 topic, principle only | no |
| XC-10-ACTION | OPEN | P0 | yes, before any cancel, modify, or risk-reduce dispatch |

XC-10-RACE sources: DET-01 `## 5`; CR-B09 P1-B09-D02; CR-B10 §2 `CancelRequested` versus `Cancelled`. A cancel request is not a cancel confirmation and is not proof that an unknown submission was never sent. A recorded broker cancel does not erase executions or release risk (CR-B10 §7 rule 6; DET-02 `## 5`).

XC-10-ACTION sources: CR-B09 P1-B09-D01 through D05 and the verification line; DET-07 decision DET-07-D05; CR-B04 test `T04`. CR-B09 allows a cancel during a submission halt only under identity, writer, and connection conditions that are not implemented and not IBKR-verified. Modify is blocked in that proposal until broker semantics and the risk delta are verified. `RISK_REDUCE` is a policy class, not a broker order type, and is deny when the risk effect is indeterminate. `GATE-P1-09` is `EXTERNAL_VERIFICATION_REQUIRED`.

Proposed resolution: do not implement place, cancel, modify, or risk-reduce dispatch. Do not treat "cancel during halt" as currently authorized behavior.

### Register status drift

| ID | Status | Severity | User approval |
| --- | --- | --- | --- |
| XC-11 | OPEN | P1 | yes, before any ADFR count or supersession is treated as canonical |

Sources: Architecture Consolidation `## ADFR-Konsolidierung`; JSON `entries` (77) and root `status`; CSV row `ADFR-CR-B10-001`; CR-B10 `## 9`; DET-09/10 `## ADFR-CROSS`; JSON rows ADFR-CROSS-001..006.

The stated 77 matches the JSON and the consolidation breakdown, and it does not match the CSV or the consolidation's "78" sentence. CR-B10 §9 says ADFR-CR-B10-001 supersedes unresolved ADFR-CROSS-005. The JSON still has ADFR-CROSS-005 as `PROPOSED_FOR_ACCEPTANCE` and does not contain ADFR-CR-B10-001. The DET-09/10 table still says `OPEN P0` for CROSS-001..004. The JSON says `PROPOSED_FOR_ACCEPTANCE` and `NOT_VERIFIED`. The P0 and P1 files say design-resolved. Those words are not reconciled here. Parsing is not correctness.

Proposed resolution: do not delete, merge, or relabel register rows in this task.

## Verified and unverified

Verified in this task:

- Clean `main` at `41c518830a4f90626a4fa4100b9b9347490f6ca8` before edits.
- ZIP manifest SHA-256 and byte length for all 12 source files.
- Byte identity of the existing CR-B10 and M0-02A copies with the ZIP.
- ADFR JSON parses; CSV parses; counts and the one extra CSV ID above.
- Registry flags listed above were not changed. `contracts/registry/registry.json` bytes were not edited.
- No broker connection, credential, migration, HTTP route, order transition, risk evaluation, or dispatch was added.

Not verified:

- Original DET-01 and DET-02 Q&A transcripts. They are not in the package.
- That JSON `CONFIRMED_DECISION` ratifies the reconstructed markdown.
- That 77 or 78 is the correct decision population.
- IBKR `orderId`, `permId`, `execId`, `clientId`, snapshot completion, history, cancel, and modify.
- PostgreSQL lock order, isolation, crash windows, and audit atomicity.
- Any economic, FX, rounding, or valuation result.
- Remote CI.

## Gap matrix

CR-B10 names against `crates/trading-domain` and `contracts/registry/registry.json`. This is a gap list. No type was added. Draft label enums that match CR-B10 §2 spelling are still labels: `transitions_implemented` is false on every registry entry, and `schema_path` is null.

| CR-B10 name | Section | Rust / registry | Gap |
| --- | --- | --- | --- |
| `OrderId`, `CommandId`, `EventId`, `CorrelationId`, `CausationId` | §1, §4 | newtypes, `INTERNAL_DRAFT_SHELL` | IDs only. Payload-hash idempotency is not checked. `trading_common::CorrelationId` is `NOT_IMPLEMENTED`. `HttpRequestId` is not `CorrelationId`. |
| `OrderVersion` | §1 | `u64` newtype | `checked_next` is not an aggregate mutation. |
| `OrderWorkflowState` | §1, §2 | draft variants match §2 spelling | No edges. `Closed` is not economic finality. |
| `OrderClosure` | §1, §2 | registry `NOT_IMPLEMENTED` | Reason vocabulary unconfirmed. Not a wire schema. |
| `BrokerObservationId`, `BrokerSessionId` | §1 | ID shells | Observation body and session persistence are absent. |
| `BrokerLifecycleState` | §2 | draft variants, spelling `Cancelled` | No adapter. `Unknown` is not a decoder default. |
| `BrokerAccountKey` | §1 | broker, account, environment | Shell matches CR-B10. DET-03's reference row does not name environment (XC-01). |
| `BrokerOrderAlias` | §1, §7.2 | scoped external id, no client id on the alias | Client id sits only on optional `BrokerIdentityContext`. Equality is not a uniqueness proof. |
| `BrokerPermIdAlias` | §1 | same unverified scope | Global uniqueness is not assumed. `EVIDENCE_PENDING`. |
| `SubmissionAttemptId` | §1 | ID shell | Attempt persistence and `DISPATCH_STARTED` are absent. `BROKER_DISPATCH_IMPLEMENTED=false`. |
| `WriterEpoch` | §1, §7.7 | `u64`, `checked_next` | Not the database fence required by UD-04 and CR-B01. |
| `ExecutionIdentity` | §1, §6 | scoped key, separate quarantined type | No resolver. `IDENTITY_RESOLVER_IMPLEMENTED=false`. |
| `ExecutionCorrectionId` | §1 | ID shell | Correction target is not stored. |
| `ExecutionState` | §2 | draft variants | CR-B10 §1 has no ownership row. UD-07 assigns the execution ledger. No fill processing. |
| `RiskReservationId`, `RiskEvaluationId` | §1 | ID shells | No order link, inputs, or evaluation. `RISK_EVALUATION_IMPLEMENTED=false`. |
| `RiskReservationState` | §1, §2 | `Active`, `ReconciliationRequired`, `Released` | No `Adjusted` variant. Release is not implemented (XC-02-PRED). |
| `RiskReservationAdjusted` | §2 | registry event name, `NOT_IMPLEMENTED` | Not a state and not a published event schema. |
| `RiskPolicyVersion` | §1, §3 | opaque token | §3 `PolicyVersion` is not a second Rust type. |
| `RiskDecision` | §2 | `Allow`, `Deny`, `Indeterminate` | Unknown text does not become `Allow`. No policy runs. |
| `TradingReadinessState` | §2 | includes `ManualReset` | Not HTTP readiness. Not DET-06's `MANUAL_RESET_REQUIRED` (XC-09-RESET). No gate formula is executed. |
| `ReconciliationRunId`, `ReconciliationRunState` | §1, §2 | ID plus draft labels including `Blocked` | No reconciliation run. |
| `IncidentId` | §1 | ID shell | `IncidentState` is `NOT_IMPLEMENTED`. DET-10's list was not adopted. |
| `MarketObservationId`, `MarketSnapshotId`, `ValuationSnapshotId` | §1 | ID shells | No market or valuation data. |
| `InstrumentId`, `InstrumentDefinitionVersion`, `ValuationModelVersion`, `NormalizerVersion`, `SchemaVersion` | §1, §3 | tokens / `u64` major version | No symbology, no compatibility proof, no ordering on opaque versions. |
| `AuditRecordId` | §1 | ID shell | No audit write. |
| `Money`, `Price`, `Quantity` | §3 | exact add only | FX, rescale, division, and tick quantization return `NotImplemented`. |
| `DecimalAmount` | §3, via UD-03 | `i128` and scale 0..=38 | Not named as a CR-B10 type. Structural `Eq` is not numeric equality. `numeric_eq` does not round. |
| `RoundingPolicy` | §3, modes from UD-03 | vocabulary only | Not applied. `FINANCIAL_ROUNDING_IMPLEMENTED=false`. |
| `OccurredAt`, `ReceivedAt`, `PersistedAt` | §3 | microsecond newtypes | No conversion between them. `PersistedAt` is not assigned by a persistence boundary. |
| `ClockQuality`, `TimestampProvenance` | §3 quality attribute, types named by UD-05 | both `NOT_IMPLEMENTED` | No variant list was supplied, so none was invented. |
| UUID v7 | §3 | generation is UUID v4 | UD-09 keeps v4. v7 was not adopted. |
| §4 envelope and §5 `registry.yaml`, `events/v1`, `commands/v1`, `api/v1`, `sql/v1`, `examples` | §4, §5 | absent | Skeleton JSON only. `part_b=NO-GO`. B10-T01..T12 are not passed. |

Registry note drift, left in place: the `IncidentState` note still says DET-10 is not in the repository. The file is present and not reviewed. The note was not edited, so the skeleton bytes and the allowed source-contract list stay as M0-02B-a left them.

## Deviations

1. `crates/trading-domain/tests/registry_skeleton.rs` previously required `contracts/` to contain only `registry.json`. This task requires `contracts/source-manifest.json` and `contracts/sources/ADFR-Consolidated-v1.0.json`. The assertion is now an allowlist of those three paths. Forbidden schema directories remain rejected. The test also rejects a manifest that marks a source approved, authorizes live trading, or publishes canonical schemas, and it rejects an ADFR file that drops `ARCHITECTURE_DRAFT_NOT_IMPLEMENTED` or sets canonical wire. No domain type, serde shape, runtime flag, or registry entry changed. This is the one code edit. It is a test inventory change required by the mandated artifacts, not an implementation of CR-B10.
2. `contracts/registry/registry.json` was not modified. The stale `IncidentState` absence sentence remains on purpose.
3. Presence sentences were added to the glossary, the unresolved-decision record, the registry README, and `docs/architecture/README.md` so those files do not keep saying the new bodies are missing. UD statuses and registry flags were not retitled.
4. The ZIP's own `source-manifest.json` was verified and not stored as a second manifest.

## Tests

Commands were run locally on 2026-11-15 from the repository root. Remote CI was not run.

| Command | Exit |
| --- | --- |
| `cargo fmt --all -- --check` | 0 |
| `cargo check --workspace --locked` | 0 |
| `cargo test --workspace --locked` | 0 |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | 0 |

`cargo test --workspace --locked` reported 121 passed, 0 failed, and 1 ignored. The ignored test is `crates/trading-persistence/tests/connectivity.rs` `migration_is_explicit_and_readiness_is_read_only`: `ignored, requires TRADING_ALLOW_DATABASE_INTEGRATION_TESTS=1 and TRADING_TEST_DATABASE_URL`. It was not run with `--ignored`. The 121 passed count is the sum of the crate test binaries and `trading-domain` doctests printed by that command (17 + 1 + 1 + 3 + 52 + 1 + 1 + 1 + 15 + 1 + 27). Empty binaries and empty doctest crates added 0. `registry_skeleton_stays_unpublished` passed.

`cargo fmt --all -- --check` failed once on the new test assertion layout, the layout was formatted, and the same command then exited 0. The exit code in the table is the passing rerun.

## Changed files

- Added under `contracts/sources/`: the ten files that were not already present. CR-B10 and M0-02A were left byte-identical.
- Added `contracts/source-manifest.json`.
- Replaced `contracts/traceability-index.md`, keeping the 2026-11-14 `NOT_PROVIDED` snapshot.
- Added this report and `docs/cursor-prompts/trading/TRD-ARCH-SYNC-01-Architecture-Source-Integration.md`.
- Presence sentences in `contracts/domain/glossary.md`, `contracts/domain/unresolved-decisions.md`, `contracts/registry/README.md`, and `docs/architecture/README.md`.
- Test allowlist in `crates/trading-domain/tests/registry_skeleton.rs`.

`git diff --stat` for files that were already tracked, captured after the test table was written and before this paragraph:

```text
 contracts/domain/glossary.md                     |  8 +--
 contracts/domain/unresolved-decisions.md         |  8 +--
 contracts/registry/README.md                     |  4 +-
 contracts/traceability-index.md                  | 84 ++++++++++++++++++------
 crates/trading-domain/tests/registry_skeleton.rs | 60 +++++++++++++++--
 docs/architecture/README.md                      |  6 +-
 6 files changed, 132 insertions(+), 38 deletions(-)
```

Untracked files included in the same commit:

- `contracts/source-manifest.json`
- `contracts/sources/ADFR-Consolidated-v1.0.csv`
- `contracts/sources/ADFR-Consolidated-v1.0.json`
- `contracts/sources/Architecture-Consolidation-and-Acceptance-v1.0.md`
- `contracts/sources/DET-01-Order-Workflow-and-Finality-Contract-v1.0.md`
- `contracts/sources/DET-02-Risk-Contract-v1.0.md`
- `contracts/sources/DET-03-bis-DET-08-Architecture-Contracts-v1.0.md`
- `contracts/sources/DET-09-bis-DET-10-und-Cross-Contract-Review-v1.0.md`
- `contracts/sources/Implementation-Roadmap-and-Cursor-Prompt-Plan-v1.0.md`
- `contracts/sources/P0-Resolution-Contracts-CR-B01-bis-CR-B04-v1.0.md`
- `contracts/sources/P1-Resolution-Contracts-CR-B05-bis-CR-B09-v1.0.md`
- `docs/architecture/TRD-ARCH-SYNC-01-report.md`
- `docs/cursor-prompts/trading/TRD-ARCH-SYNC-01-Architecture-Source-Integration.md`

Inserting this list changes only this report. The commit response quotes `git show --stat` for the committed tree. That stat is the exact commit diff.

Not changed: `contracts/registry/registry.json`, `contracts/sources/CR-B10-Domain-Glossary-and-Schema-Registry-v1.0.md`, `contracts/sources/M0-02A-UD-01-bis-UD-09-Architecture-Resolution-v1.0.md`, domain Rust types, serde, migrations, HTTP routes, and runtime flags.

## Skipped checks

- Remote GitHub Actions.
- `cargo test --workspace --ignored` and any PostgreSQL integration test.
- IBKR paper or gateway calls.
- B10-T01 through B10-T12.
- Semantic review of each of the 77 ADFR decision sentences beyond the count, status, and field comparison above.

## Next step

Do not implement M0-03, a transition matrix, rounding, dispatch, or a schema publication from the roadmap. The roadmap file is `PLAN_ONLY`.

A follow-up that is still documentation, if the user wants it, would ask the user to decide three open items before any behavior change: whether the reconstructed DET-01/02 drafts may be cited as primary text; which ADFR population is canonical (JSON 77, CSV 78, or neither); and whether CR-B01's no-automatic-takeover rule is the binding reading of DET-07. Until those decisions exist, identity uniqueness, reservation release, closure reasons, fencing, audit authority, rounding, snapshot completion, reset, and cancel/modify stay unimplemented and fail-closed.

Suggested next prompt title, not an instruction to run it: `TRD-ARCH-SYNC-02: User decisions on reconstructed DET-01/02, ADFR population, and CR-B01 precedence`. That prompt should remain documentation-only unless the user separately authorizes a later implementation milestone.
