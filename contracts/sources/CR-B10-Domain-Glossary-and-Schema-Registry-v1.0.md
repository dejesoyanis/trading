# CR-B10 — Canonical Domain Glossary & Schema Registry

**Version:** 1.0-draft | **Date:** 2026-11-14 | **Status:** DESIGN_RESOLVED / IMPLEMENTATION_NOT_VERIFIED / LIVE_NOT_AUTHORIZED

## 0. Provenance and authority

This contract resolves the P2 terminology/schema ambiguity identified in `Architecture-Consolidation-and-Acceptance-v1.0.md` and `P1-Resolution-Contracts-CR-B05-bis-CR-B09-v1.0.md`. It is a proposed technical refinement, not evidence of implementation. Confirmed DET-01/02 Q01–Q10, existing DET-03–10 drafts, and CR-B01–B09 override any conflicting example here. A contradiction must be surfaced and resolved by versioned ADR; never silently normalized.

## 1. Canonical vocabulary and ownership

| Concept | Owning domain | Canonical type | Invariant |
|---|---|---|---|
| Order | OMS | `OrderId` (UUID newtype) | Stable, never broker-derived |
| Order aggregate revision | OMS | `OrderVersion` (u64 newtype) | Strictly monotonic on accepted aggregate mutations |
| Command | OMS | `CommandId` (UUID newtype) | Unique; repeat payload hash must match |
| Domain event | Shared infrastructure / producer | `EventId` (UUID newtype) | Unique; immutable envelope |
| Correlation / causation | Shared | `CorrelationId`, `CausationId` | Trace across commands/events |
| Order workflow | OMS | `OrderWorkflowState` | Not broker lifecycle |
| Closure | OMS | `OrderClosure` | Immutable reason, timestamp, event ID, closing version |
| Broker observation | Broker integration | `BrokerObservationId` | Raw source preserved |
| Broker normalized status | Broker adapter | `BrokerLifecycleState` | Observation, not execution truth |
| Broker account | Broker integration | `BrokerAccountKey` | Broker + account + environment |
| Connection session | Broker integration | `BrokerSessionId` | Unique internal session ID |
| Broker order alias | Broker identity | `BrokerOrderAlias` | Scope includes broker, account, client/session as appropriate, external ID kind |
| Permanent broker alias | Broker identity | `BrokerPermIdAlias` | No assumed global uniqueness without verified broker scope |
| Submission attempt | Dispatch | `SubmissionAttemptId` | Persisted before potential external send |
| Writer epoch | Dispatch | `WriterEpoch` | Monotonic DB-issued fencing; not external broker fence |
| Execution | Execution ledger | `ExecutionIdentity` | Scoped by broker/account and broker execution ID; ambiguity quarantined |
| Execution correction | Execution ledger | `ExecutionCorrectionId` | Append-only, references original evidence |
| Risk reservation | Risk | `RiskReservationId` | Separate from OrderId; order association explicit |
| Risk reservation lifecycle | Risk | `RiskReservationState` | Not OrderWorkflowState |
| Risk policy | Risk | `RiskPolicyVersion` | Immutable published version |
| Risk evaluation | Risk | `RiskEvaluationId` | Inputs and rule versions recorded |
| Readiness | Operations | `TradingReadinessState` | Separate from order/reconciliation states |
| Reconciliation run | Recovery | `ReconciliationRunId` and `ReconciliationRunState` | Completeness/evidence explicit |
| Incident | Operations | `IncidentId`, `IncidentState` | Persisted independent of order lifecycle |
| Market data | Data | `MarketObservationId`, `MarketSnapshotId` | Source and timestamps retained |
| Instrument | Reference data | `InstrumentId`, `InstrumentDefinitionVersion` | Instrument units/currency/version pinned |
| Valuation | Data/Risk | `ValuationSnapshotId` | Deterministic, pinned inputs |
| Audit record | Audit | `AuditRecordId` | Atomic source record, append-only |

**Naming rule:** `RECONCILIATION_REQUIRED` is not a shared enum value. Use `OrderWorkflowState::ReconciliationRequired`, `RiskReservationState::ReconciliationRequired` and `ReconciliationRunState::Blocked` as separate tagged types. A UI may render identical labels but cannot merge domain types.

## 2. Proposed typed state definitions

These are canonical **draft** names; exact transition matrices remain a separate acceptance item.

```rust
pub enum OrderWorkflowState {
    Draft, Validating, AwaitingApproval, Approved,
    SubmissionPending, SubmissionInFlight, Submitted,
    CancelRequested, ReconciliationRequired, Rejected, Closed,
}
pub enum BrokerLifecycleState {
    NotObserved, PendingSubmit, Accepted, Working,
    PendingCancel, Cancelled, Rejected, Expired, Unknown,
}
pub enum ExecutionState { Unfilled, PartiallyFilled, Filled, Disputed }
pub enum RiskReservationState { Active, ReconciliationRequired, Released }
pub enum TradingReadinessState {
    NotReady, Initializing, Synchronizing, Ready,
    Degraded, Halted, Recovering, ManualReset,
}
pub enum ReconciliationRunState { Created, Collecting, Comparing, Blocked, Resolved, Failed }
pub enum RiskDecision { Allow, Deny, Indeterminate }
pub enum DispatchOutcome { NotAttempted, MayHaveSent, ConfirmedAccepted, ConfirmedRejected, ReconciliationRequired }
```

`ADJUSTED` is a reservation **event** (`RiskReservationAdjusted`), not a lifecycle state: an adjusted reservation remains `Active` unless reconciliation is required or it is released. This removes a previous ambiguity without changing DET-02-Q05.

`Closed` means internal workflow closure, never irrevocable economic finality. `OrderClosure` is immutable. A late fill/correction changes execution/economic projections through new events, and may create a discrepancy/incident, without rewriting closure reason or resubmitting.

`Rejected` is a local workflow rejection. A broker rejection is a `BrokerLifecycleState::Rejected` observation that can drive a separate closure transition after sufficient evidence.

## 3. Scalar types, precision and time

- UUID v7 is a **proposal** for new internal IDs; UUID v4 remains acceptable. Identity must not rely on timestamp ordering.
- All monetary/price/quantity amounts use explicit instrument scale, checked decimal arithmetic and explicit rounding policy. No floating-point persistence for authoritative money, quantity or risk amounts.
- `Money { amount, currency }`, `Price { amount, quote_currency, instrument_id, definition_version }`, `Quantity { amount, instrument_id, definition_version }` are distinct types; addition across currencies/instruments without an explicit conversion is invalid.
- `OccurredAt` (source event time), `ReceivedAt` (ingestion time), `PersistedAt` (DB commit-recorded time) are separate UTC timestamps. Clock uncertainty and missing source timestamps are explicit quality attributes.
- `SchemaVersion`, `PolicyVersion`, `NormalizerVersion`, `InstrumentDefinitionVersion` and `ValuationModelVersion` are immutable, independent version identifiers.
- Do not assume timestamps alone establish broker event ordering.

## 4. Event envelope v1 (proposal)

```json
{
  "schema_name": "trading.order.execution_recorded",
  "schema_version": 1,
  "event_id": "<uuid>",
  "aggregate_type": "order",
  "aggregate_id": "<order-uuid>",
  "aggregate_version": 42,
  "correlation_id": "<uuid>",
  "causation_id": "<uuid>",
  "occurred_at": "2026-11-14T12:00:00Z",
  "received_at": "2026-11-14T12:00:01Z",
  "producer": "execution-ledger",
  "payload": { "execution_identity": "<scoped-execution-key>", "evidence_id": "<uuid>" }
}
```

Envelope requirements: stable name/version, producer, IDs, causal link, explicit source/evidence references, deterministic serialization for replay, payload size bounds, no secrets or unredacted personal credentials. Event schema version does **not** equal aggregate version. DB sequence is the authoritative local ordering within a transaction; transport delivery order is not authoritative.

## 5. Registry layout and compatibility

Proposed repository structure:

```text
contracts/
  registry.yaml
  domain/glossary.md
  domain/identifiers.yaml
  domain/states.yaml
  events/v1/*.schema.json
  commands/v1/*.schema.json
  api/v1/*.schema.json
  sql/v1/*.sql
  invariants/*.md
  examples/*.json
```

Each registry entry has: `name`, `kind`, `owner`, `version`, `status`, `source_contract`, `schema_path`, `compatibility`, `consumers`, `invariants`, `test_refs`. All refs must resolve in CI. Schema names are globally unique; event names are immutable once published. Required-field removal, semantic reinterpretation, ID-scope change or state meaning change requires a new major version and migration/replay compatibility plan. Additive optional fields require tests for old consumers. Unknown safety-critical enum values must fail closed, never default to `Ready` or `Allow`.

## 6. Contract boundaries

| Producer | Consumer | Payload contract | Failure behavior |
|---|---|---|---|
| OMS | Risk | OrderId, OrderVersion, scoped risk request, policy/snapshot refs | No reservation -> no submission |
| Risk | OMS | RiskEvaluationId, reservation ID, policy version, decision | Indeterminate/Deny -> no submission |
| Dispatch | Broker adapter | SubmissionAttemptId, WriterEpoch, scoped broker identity | Unknown outcome -> reconciliation |
| Broker adapter | Ledger | Raw observation + normalizer version + scoped execution identity | Ambiguous -> quarantine |
| Ledger | Risk | Append-only execution/correction IDs + economic delta | Rebook atomically; halt on unresolvable mismatch |
| Reconciliation | Readiness | Completeness proof, run ID, unresolved issues | Incomplete -> not ready |
| All domains | Audit/Outbox | EventId + transaction/correlation refs | Audit source failure -> transaction rollback |

## 7. Cross-contract consistency rules

1. A single internal `OrderId` can have multiple `SubmissionAttemptId`s only when a formally authorized safe retry/replacement protocol proves no duplicate economic submission; unknown outcome forbids new attempts.
2. Broker `orderId` is never globally unique by itself. Mapping is scoped and evidence-based; no automatic alias merge from coincidental numbers.
3. Execution IDs are scoped and deduplicated before ledger effect; correction references prior execution evidence.
4. Workflow `Closed` does not imply `RiskReservationState::Released` or execution finality.
5. Risk `Allow` is necessary but insufficient for broker dispatch: Readiness, authority, fencing, broker capabilities and current order version must also pass.
6. Broker `Cancelled` does not itself erase already confirmed executions or justify risk release without reconciled economics.
7. `WriterEpoch` fencing controls local authority but cannot retroactively cancel an external call; failover must fence broker session/old writer and reconcile.
8. Reconciliation completeness is scoped to account, broker session, data stream, window and watermark; empty result alone is not proof.
9. Any change to authoritative economic projection is append-only evidenced and audited.
10. No external broker call occurs inside the local PostgreSQL transaction.

## 8. Acceptance tests and CI gates

| Gate | Test | Pass condition |
|---|---|---|
| B10-T01 | Registry uniqueness | No duplicate schema/type names or conflicting owner |
| B10-T02 | Referential integrity | Every schema/event/command ref resolves |
| B10-T03 | Backward compatibility | Prior v1 fixtures decode or explicit version migration exists |
| B10-T04 | Unknown enum variants | Fail-closed; no implicit Ready/Allow |
| B10-T05 | Currency/instrument mixing | Type check or runtime validation rejects |
| B10-T06 | Deterministic serialization | Same canonical input -> same bytes/hash |
| B10-T07 | Idempotent events | Duplicate IDs have no second economic effect |
| B10-T08 | Workflow vs economic closure | Late fill updates ledger, immutable closure unchanged |
| B10-T09 | Broker alias scope | Same orderId in distinct sessions/accounts not merged |
| B10-T10 | Replay | Same pinned versions/evidence -> same projection |
| B10-T11 | Migration | Schema upgrade preserves old audit/event interpretation |
| B10-T12 | Cross-contract fixture | OMS/Risk/Dispatch/Adapter/Reconciliation event round trip |

All tests are **specified, not executed**. `INT-01` remains open until schema files, executable transition matrices, and test evidence exist.

## 9. ADFR entry

**ADFR-CR-B10-001** — Adopt canonical, owner-scoped domain types, independent state enums, versioned event/command envelopes and compatibility-gated schema registry. **Status:** DESIGN_RESOLVED / TESTS_NOT_EXECUTED. **Supersedes:** unresolved `ADFR-CROSS-005` P2 finding (historical entry retained). **Alternatives rejected:** shared stringly-typed statuses; broker IDs as primary keys; implicit payload evolution. **Consequences:** stronger type boundaries, migration discipline, additional schema governance. **Dependencies:** DET-01..10, CR-B01..09. **Verification:** B10-T01..T12, INT-01.

## 10. Residual gaps and release boundary

- DET-01 complete executable transition table and event precedence not yet canonicalized.
- DET-02 machine-readable policy DSL, budget dimensions, locks and DB constraints not yet canonicalized.
- Actual IBKR identity, session isolation, source completeness and cancel/modify semantics require integration evidence.
- No production broker dispatch, no live-trading authorization and no automatic merge of proposed contracts into the repository.
