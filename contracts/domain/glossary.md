# Domain glossary

**Status:** draft shell, M0-02A.2. Each UD is DESIGN_RESOLVED. Implementation status is per decision in [unresolved-decisions.md](unresolved-decisions.md). Not production-proven. `DESIGN_RESOLVED` here is not verification and not live authorization. The separate status vocabulary is [contract-status-model-v1.0.md](../status-model/contract-status-model-v1.0.md).  
**Source:** CR-B10 v1.0-draft, read as draft names rather than a transition model. Provisional details are refined by the architecture resolution. At M0-02A.2 the DET bodies were not supplied and were not reviewed. As of TRD-ARCH-SYNC-01 they are present under `contracts/sources/` and indexed as `PRESENT / NOT_REVIEWED`. This glossary was not rewritten from them. DET-01 and DET-02 are reconstructed drafts.  
**Code:** `crates/trading-domain`.  
**Decisions:** [unresolved-decisions.md](unresolved-decisions.md).  
**Resolution:** [M0-02A-UD-01-bis-UD-09-Architecture-Resolution-v1.0.md](../sources/M0-02A-UD-01-bis-UD-09-Architecture-Resolution-v1.0.md).  
**Registry:** [M0-02B-a skeleton](../registry/README.md). Canonical schemas are not published.  
**Index:** [traceability-index.md](../traceability-index.md).

At M0-02A.2, DET-01 through DET-10 and CR-B01 through CR-B09 were not in the repository. They are present as of TRD-ARCH-SYNC-01 and are `PRESENT / NOT_REVIEWED`. No transition row is claimed against them. B10-T01 through B10-T12 are not executed by this glossary or by the registry skeleton. Unit tests cover non-interchangeable ids, scoped alias inequality, fail-closed state decoding, and checked decimal mismatch. Those tests are not the full registry gates.

HTTP readiness is not `TradingReadinessState`. `trading_common::HttpRequestId` is not `CorrelationId` below. The HTTP header remains `x-correlation-id`.

## Traceability

| Type | Owner | Meaning | Invariants | Distinct from | CR-B10 |
| --- | --- | --- | --- | --- | --- |
| `OrderId` | OMS | Stable internal order id | UUID newtype; never broker-derived; bits are not event order | `CommandId`, `BrokerOrderAlias`, broker text | §1 |
| `CommandId` | OMS | Command id | UUID newtype; payload-hash equality is not checked | `OrderId`, `EventId` | §1 |
| `EventId` | Shared infrastructure / producer | Immutable event id | UUID newtype | `CommandId`, `CausationId` | §1, §4 |
| `CorrelationId` | Shared | Domain correlation across commands and events | UUID newtype; not `trading_common::HttpRequestId`; no conversion (UD-01) | `HttpRequestId`, `CausationId` | §1, §4 |
| `CausationId` | Shared | Causal predecessor id | UUID newtype | `CorrelationId`, `EventId` | §1, §4 |
| `BrokerObservationId` | Broker integration | Id of a raw observation | UUID newtype; body is not stored | `BrokerSessionId`, `EventId` | §1 |
| `BrokerSessionId` | Broker integration | Internal session id | UUID newtype; not a broker client id | `BrokerClientId`, `BrokerAccountKey` | §1 |
| `SubmissionAttemptId` | Dispatch | Id of one submission attempt | UUID newtype; persistence is not implemented | `OrderId`, `WriterEpoch` | §1 |
| `RiskReservationId` | Risk | Reservation id | UUID newtype; order link is not stored | `OrderId`, `RiskEvaluationId` | §1 |
| `RiskEvaluationId` | Risk | Evaluation id | UUID newtype; inputs are not stored | `RiskReservationId`, `RiskPolicyVersion` | §1 |
| `ReconciliationRunId` | Recovery | Run id | UUID newtype | `ReconciliationRunState`, `IncidentId` | §1 |
| `IncidentId` | Operations | Incident id | UUID newtype; `IncidentState` is not defined | `OrderId`, `ReconciliationRunId` | §1 |
| `MarketObservationId` | Market data | Observation id | UUID newtype | `MarketSnapshotId` | §1 |
| `MarketSnapshotId` | Market data | Snapshot id | UUID newtype | `MarketObservationId`, `ValuationSnapshotId` | §1 |
| `ValuationSnapshotId` | Data / risk | Valuation snapshot id | UUID newtype; valuation math is absent | `MarketSnapshotId`, `ValuationModelVersion` | §1 |
| `AuditRecordId` | Audit | Append-only audit id | UUID newtype | `EventId` | §1 |
| `ExecutionCorrectionId` | Execution ledger | Append-only correction id | UUID newtype; target execution is not stored | `ExecutionIdentity` | §1 |
| `OrderVersion` | OMS | Aggregate counter | `u64`; `checked_next` does not wrap; `Ord` is counter order only | `SchemaVersion`, `WriterEpoch`, timestamps | §1 |
| `WriterEpoch` | Dispatch | Local fencing counter | `u64`; does not wrap; shell `checked_next` is not the DB fence required by UD-04 | `OrderVersion`, broker session | §1, §7.7 |
| `SchemaVersion` | Shared infrastructure | Schema integer | `u64`; no successor; larger does not mean compatible | `OrderVersion`, opaque versions | §3, §4 |
| `RiskPolicyVersion` | Risk | Published policy version | Opaque text; no ordering | `SchemaVersion`; section 3 `PolicyVersion` is not a second type | §1, §3 |
| `NormalizerVersion` | Broker adapter | Normalizer version | Opaque text; no ordering | `SchemaVersion`, `BrokerLifecycleState` | §3, §6 |
| `InstrumentDefinitionVersion` | Reference data | Pinned instrument definition | Opaque text; equality is exact | `SchemaVersion`, `InstrumentId` | §1, §3 |
| `ValuationModelVersion` | Data / risk | Pinned valuation model | Opaque text; model is not implemented | `ValuationSnapshotId` | §3 |
| `BrokerName` | Broker integration | Broker name token | Printable ASCII, 1–32 bytes; not a connection | `BrokerAccountCode` | §1 account key |
| `BrokerAccountCode` | Broker integration | Account code token | Printable ASCII, 1–64 bytes | `OrderId`, `BrokerName` | §1 |
| `BrokerEnvironmentLabel` | Broker integration | Environment label | Printable ASCII, 1–32 bytes; does not authorize trading | Process `DeploymentEnvironment` | §1 |
| `BrokerAccountKey` | Broker integration | Broker + account + environment | All three parts participate in equality | Any single component; `OrderId` | §1 |
| `UnverifiedSessionScope` | Broker identity | Marker that session or client scope is unverified | Required to build a scope; IBKR behavior is not claimed | A verified uniqueness proof | §1, §10 |
| `BrokerAliasScope` | Broker identity | Account-only or account plus internal session | Carries `UnverifiedSessionScope`; structural equality only | A resolved broker identity | §1, §7.2 |
| `BrokerClientId` | Broker identity | Broker client text | Printable ASCII, 1–64 bytes; not a UUID; equal text is not one client | `BrokerSessionId`, `OrderId` | UD-02 |
| `AdapterSessionEpoch` | Broker identity | Recorded adapter epoch | `u64` supplied by the caller; no successor; not a database fence | `WriterEpoch` | UD-02 |
| `ExternalAliasKind` | Broker identity | Draft alias kind label | `order_id`, `perm_id`, or `exec_id`; not a uniqueness rule | A verified scope | UD-02 |
| `IdentityEvidenceId` | Broker identity | Evidence token | Printable ASCII, 1–128 bytes; presence is not verification | A verified scope | UD-02 |
| `BrokerIdentityContext` | Broker identity | Supplied identity components | Unverified marker required; client, session, epoch, and evidence are optional; structural equality is not a merge | `BrokerOrderAlias`, `ExecutionIdentity` | UD-02 |
| `ExternalBrokerId` | Broker identity | External id text | Printable ASCII, 1–128 bytes; no broker grammar; raw text is preserved | `OrderId`, `BrokerSessionId` | §1, §7.2 |
| `BrokerOrderAlias` | Broker identity | Scoped broker order id | Kind, account, scope, and external id; no merge | `BrokerPermIdAlias`, `OrderId` | §1, §7.2 |
| `BrokerPermIdAlias` | Broker identity | Scoped permanent broker id | Same scope rule; global uniqueness is not assumed | `BrokerOrderAlias` | §1 |
| `ExecutionIdentity` | Execution ledger | Scoped execution id with no ambiguity flag recorded | `none_recorded` is not a uniqueness proof | `QuarantinedExecutionIdentity` | §1, §6 |
| `QuarantinedExecutionIdentity` | Execution ledger | Same key held for ambiguity | No conversion back to `ExecutionIdentity` | `ExecutionIdentity` | §1, §6 |
| `OrderWorkflowState` | OMS | Internal workflow label | Draft variants; no transitions; `Closed` is not economic finality | `BrokerLifecycleState`, `RiskReservationState`, `ExecutionState` | §1, §2 |
| `BrokerLifecycleState` | Broker adapter | Normalized broker observation | `Unknown` is a real label; other text does not become `Unknown`; spelling `Cancelled` | `OrderWorkflowState`, `DispatchOutcome` | §2 |
| `ExecutionState` | Execution ledger | Fill label | No fill processing. The execution ledger owns this label (UD-07). CR-B10 §1 has no row. No variants were added. | `OrderWorkflowState::Closed` | §2 |
| `RiskReservationState` | Risk | Reservation label | No `Adjusted` variant; `Released` is not workflow closure | `OrderWorkflowState` | §1, §2 |
| `TradingReadinessState` | Operations | Readiness label | Not HTTP readiness; unknown text does not become `Ready`; no decision procedure | HTTP status, `RiskDecision` | §2 |
| `ReconciliationRunState` | Recovery | Run label | `Blocked`, not the token `ReconciliationRequired` | Workflow and reservation labels of that name | §1, §2 |
| `RiskDecision` | Risk | Decision label | Unknown text does not become `Allow`; no policy evaluation | `TradingReadinessState`, `DispatchOutcome` | §2, §6 |
| `DispatchOutcome` | Dispatch | Outcome label | Unknown text does not become `ConfirmedAccepted`; no send | `BrokerLifecycleState::Accepted` | §2, §6 |
| `DecimalAmount` | Shared value | Signed decimal shell | `i128` coefficient and scale 0–38; `Eq`/`Hash` are structural; `numeric_eq` is mathematical and overflow-safe; scale is not a tick; no float; no silent rescale | `Money`, `Price`, `Quantity` | §3 |
| `RoundingPolicy` | Shared value | Rounding vocabulary | `RejectInexact` (vocabulary default), `TowardZero`, `AwayFromZero`, `HalfEven`, `Floor`, `Ceiling`; no mode is applied | A rounded amount | UD-03 |
| `CurrencyCode` | Shared value | Currency token | Uppercase ASCII, 1–16 letters; not an ISO registry | `InstrumentId` | §3 |
| `InstrumentId` | Reference data | Instrument token | Printable ASCII, 1–64 bytes; no case fold; no symbology | `CurrencyCode`, `OrderId` | §1, §3 |
| `Money` | Shared value | Amount plus currency | Same currency and scale to add; FX is `NotImplemented` | `Price` | §3 |
| `Price` | Shared value | Quote plus instrument definition | Same currency, instrument, definition version, and scale to add | `Money`, `Quantity` | §3 |
| `Quantity` | Shared value | Quantity plus instrument definition | Same instrument, definition version, and scale to add; sign is not an order side | `Price`, `Money` | §3 |
| `OccurredAt` | Shared | Source-event time | UTC microseconds; zero is present; nanos are rejected; opt-in RFC 3339 is separate from draft JSON | `ReceivedAt`, `PersistedAt` | §3 |
| `ReceivedAt` | Shared | Ingestion time | Same representation; no conversion from `OccurredAt` | `OccurredAt`, `PersistedAt` | §3 |
| `PersistedAt` | Shared | Commit-recorded time | Same representation; the persistence boundary does not assign it | `OccurredAt`, `ReceivedAt` | §3 |
| `UnknownState` | Shared | Decoder rejection | Does not store the rejected text; does not substitute a permissive variant | A successful state value | §5 |
| `FinancialGap` | Shared | Named unimplemented money operation | Returned instead of an invented rounding or FX result | A successful amount | §3 |
| `TimeGap` | Shared | Named unimplemented time conversion | Returned instead of truncation or a civil clock | A timestamp value | §3 |

Supporting errors `IdParseError`, `IntegerParseError`, `TokenError`, `VersionOverflow`, `DecimalError`, `MoneyError`, `PriceError`, `QuantityError`, `TimeError`, and `IdentityParseError` describe those rejections. Their text does not include the rejected input.

Constants `ORDER_STATE_MACHINE_IMPLEMENTED`, `STATE_TRANSITIONS_IMPLEMENTED`, `IDENTITY_RESOLVER_IMPLEMENTED`, `FINANCIAL_ROUNDING_IMPLEMENTED`, `BROKER_DISPATCH_IMPLEMENTED`, and `RISK_EVALUATION_IMPLEMENTED` are all `false`.

## Identifier rules

Each UUID newtype has a private field, `generate` (UUID version 4), `from_uuid`, `as_uuid`, `Display`, and `FromStr`. Parsed text must be the lowercase hyphenated form. URN, unhyphenated, and uppercase forms fail. There is no `From` implementation between identifier types. Copying bits requires the destination constructor and does not link the values. Generation does not read a clock. `from_uuid` does not inspect the version nibble. UUIDv4 generation stays for M0-02A and the initial M0-02B schemas. UUIDv7 is not a migration prerequisite. Accepting bits is typed parsing, not proof that the id was minted internally. Non-nil and provenance checks at command boundaries are not implemented. UUID order is not event order.

`OrderVersion` and `WriterEpoch` use `checked_next`. At `u64::MAX` the error is `VersionOverflow` and the original value is unchanged.

## State rules

Wire labels are the PascalCase variant names. Decoding accepts only those strings. JSON numbers, null, booleans, and unknown strings fail. `Default` is not implemented, so a missing value cannot become `Ready`, `Allow`, `Released`, or `ConfirmedAccepted`.

`RECONCILIATION_REQUIRED` is not accepted. `ReconciliationRequired` is a variant of `OrderWorkflowState`, `RiskReservationState`, and `DispatchOutcome` separately. `ReconciliationRunState` uses `Blocked` and rejects `ReconciliationRequired`.

`Closed` does not imply `Released`, `Filled`, or broker `Cancelled`. No method transitions one enum into another.

`ExecutionState` is owned by the execution ledger. CR-B10 §1 does not have this row. UD-07 assigns that owner. No variants were added to fill the gap.

`RiskReservationAdjusted` is an event name in CR-B10 §2. It is not a `RiskReservationState` variant. The enum labels are draft registry candidates. Edges and guards wait until DET-01, DET-02, DET-06, and DET-07 are supplied and cross-reviewed. Those bodies are now present and are not an approved transition matrix.

## Money, price, quantity, and time

`DecimalAmount`, `Money`, `Price`, and `Quantity` reject mixed currency, instrument, definition version, or scale on exact addition. Numeric comparison of money, price, and quantity returns a mismatch error across those boundaries. Overflow uses checked `i128` arithmetic. JSON coefficients and scales are strings. Float tokens and unknown fields fail. This JSON shape is an internal draft. It is not a canonical envelope. `Eq` stays scale-sensitive. `numeric_eq` recognizes `1.0` and `1.00`, including negatives and zero, without floats. `RoundingPolicy` names modes and does not round. `rescale`, division, foreign exchange, and tick quantization return `NotImplemented`. Halfway ties are not an implemented result.

`OccurredAt`, `ReceivedAt`, and `PersistedAt` do not convert. Microsecond order inside one type is not event order and not broker order. `from_rfc3339` accepts an explicit offset, normalizes to UTC, writes exactly six fractional digits on output, and rejects nonzero sub-microsecond digits. Draft JSON remains the integer microsecond string. Zero is a present timestamp. Absence is `Option`, not zero. `PersistedAt` is not assigned by a persistence boundary and is not proof of a broker order. `ClockQuality` and `TimestampProvenance` are not types: the resolution names them and does not list variants.

## Explicitly absent

No broker socket, credential, order submission, risk evaluation, readiness decision, identity resolver, migration, or state transition is implemented in this vocabulary. `OrderClosure` is not a Rust type and not a wire schema. UD-07 requires an immutable record once DET-01 supplies a closure-reason vocabulary: closure reason, closing `EventId`, closing `OrderVersion`, occurred and recorded time, and an audit reference. The reconstructed DET-01 draft says that reason vocabulary is not confirmed, so no reason enum was invented. `IncidentState` stays undefined. DET-10 is present and not reviewed, and its incident-state list was not adopted. `ClockQuality` and `TimestampProvenance` stay undefined because the resolution does not list their variants. `BrokerIdentityContext` records unverified components and does not verify them.

M0-02B-a adds the registry skeleton under `contracts/registry/`. Canonical schema files remain deferred to M0-02B-b. The skeleton does not publish provisional serde shapes.
