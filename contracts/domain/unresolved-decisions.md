# Unresolved domain decisions

**Status:** DESIGN_RESOLVED. Each UD below records `IMPLEMENTED_AND_LOCALLY_TESTED` or `IMPLEMENTATION_PENDING` for the part that was actually delivered. A subset does not close the UD.  
**Milestone:** M0-02A.2 type adjustments. M0-02B-a is the registry skeleton only. M0-02B-b canonical schemas are not published.  
**Date:** 2026-11-14  
**Resolution:** [M0-02A-UD-01-bis-UD-09-Architecture-Resolution-v1.0.md](../sources/M0-02A-UD-01-bis-UD-09-Architecture-Resolution-v1.0.md)  
**Index:** [traceability-index.md](../traceability-index.md)

M0-02A needed Rust types whose full representation is not fixed by CR-B10. Where a shell was required, it fails closed and is named here. Do not treat a shell as production-proven. DET-01 through DET-10 and CR-B01 through CR-B09 were not in the repository when the shells were written. They were still not supplied with the resolution, so no transition or policy text from those documents was reviewed. If a later copy of those documents uses different names or scopes, that text overrides these shells and the vocabulary must be revised. Do not silently keep the shell.

The resolution refines CR-B10's explicitly provisional details. It does not replace a DET invariant. Broker uniqueness rules stay **EVIDENCE_PENDING**. At M0-02A.2, DET-01 through DET-10 and CR-B01 through CR-B09 were not in this repository and were not reviewed. TRD-ARCH-SYNC-01 placed those bodies in `contracts/sources/`. The index now says `PRESENT / NOT_REVIEWED`. These UD statuses were not rewritten from that text. DET-01 and DET-02 remain reconstructed drafts.

TRD-ARCH-03A does not change the UD statuses below. Status words are interpreted in [contract-status-model-v1.0.md](../status-model/contract-status-model-v1.0.md). That file's unassigned enums are `PROPOSED_NOT_RATIFIED`. The historical questions are [approval-requests.md](../status-model/approval-requests.md).

TRD-ARCH-03A.2 records SMQ-01 through SMQ-05 as DECIDED, each Alternative 1, in [ARCH-DEC-04-bis-08-2026-11-15.md](../decisions/ARCH-DEC-04-bis-08-2026-11-15.md). ARCH-DEC-06 reads `IMPLEMENTED_AND_LOCALLY_TESTED` as scoped evidence for the named type-shell part only. It does not verify the whole UD, a broker, a database, remote CI, or an ADFR row. The UD status lines below are not rewritten. Nothing in that interpretation approves a technical contract or a verification promotion.

## UD-01 — Two types named CorrelationId

**Status:** DESIGN_RESOLVED / IMPLEMENTED_AND_LOCALLY_TESTED

### What is in force

`trading_common::HttpRequestId` is the HTTP request identifier. It is process-local and is not a distributed trace. `trading_domain::CorrelationId` is the domain id that links commands and events. There is no conversion either way. The HTTP response header remains `x-correlation-id`, and the tracing field remains `correlation_id`. Those names are the existing wire and log format. They are not a second Rust type and they are not a public schema. The old `trading_common::CorrelationId` export was removed so the two identifiers are not published under one unqualified name. No deprecated alias was required: the only in-repository caller was the health middleware.

## UD-02 — Broker alias scope, including IBKR

**Status:** DESIGN_RESOLVED. Distinct client id and unverified `BrokerIdentityContext`: IMPLEMENTED_AND_LOCALLY_TESTED. Kind-specific uniqueness rules and verified scopes: IMPLEMENTATION_PENDING.  
**Broker uniqueness:** EVIDENCE_PENDING

### Shell still in force

CR-B10 says a broker order alias includes broker, account, client or session as appropriate, and the external id kind. It also says IBKR identity and session isolation are not yet evidenced.

The shell records:

- broker, account, and an environment label
- either no session id (`account_only`) or one internal `BrokerSessionId`
- an `UnverifiedSessionScope` value on every scope
- separate types for order alias and permanent alias

`BrokerOrderAlias` still does not carry a client id. Equality of an alias is structural equality of the recorded scope. It is not a uniqueness proof and it is not a merge. `session_scope` on the draft JSON shell must be the text `unverified`; `verified` is rejected. No identity resolver is implemented.

### Resolution

`UnverifiedSessionScope` stays a fail-closed shell. `BrokerIdentityContext` records broker, account, environment, optional broker client id, optional internal session id, optional adapter session epoch, external id kind, the raw external id, and optional evidence id. Client id is `BrokerClientId`, not `BrokerSessionId`. Adapter session epoch is not `WriterEpoch` and is not allocated here. Missing client, session, epoch, or evidence is an absent field. The record does not decide which absences are required for a kind, and it does not auto-merge. `permId` is not assumed globally unique. Matching raw ids across accounts, environments, clients, sessions, alias kinds, or evidence ids are not equal. A broker `orderId` is a scoped alias, not an order primary key. There is no verified constructor. Deserializing `verified` fails. Quarantined execution JSON does not become an unflagged identity or a context. Kind-specific uniqueness for `orderId`, `permId`, and `execId` still needs evidence-backed verified scopes and a separately approved rule table. That table is not implemented.

## UD-03 — Decimal representation, scale, and rounding

**Status:** DESIGN_RESOLVED. Storage bounds, structural equality, `numeric_eq` / `numeric_cmp`, and the `RoundingPolicy` vocabulary: IMPLEMENTED_AND_LOCALLY_TESTED. Applied rounding, `rescale`, division, foreign exchange, and tick quantization: IMPLEMENTATION_PENDING.

### Shell still in force

CR-B10 requires checked decimal arithmetic and an explicit rounding policy. It does not name the coefficient width, the scale bound, or the rounding mode.

The shell stores a signed `i128` coefficient and a scale from 0 through 38. Thirty-eight is the decimal digit budget of `i128`, not a tick size. Derived `Eq` and `Hash` stay structural: coefficient `10` at scale `1` is not representation-equal to coefficient `100` at scale `2`, and the two values are different hash keys. `numeric_eq` and `numeric_cmp` compare the mathematical value with checked powers of ten and do not use floats. When aligning a coefficient would overflow `i128`, that value has a larger magnitude than every coefficient at the higher scale. Addition and subtraction still require the same scale. `rescale`, `rescale_with_policy`, `checked_div`, and `quantize_to_tick` return `NotImplemented`, including when the scale is unchanged and including every `RoundingPolicy`. There is no `f32` or `f64` field. `RejectInexact` is the vocabulary default. Arithmetic does not apply it.

Currency codes are uppercase ASCII, 1 to 16 letters. That is not an ISO 4217 registry check. Lowercase is rejected rather than case-folded. Foreign exchange always returns `NotImplemented`.

Instrument ids are printable ASCII, 1 to 64 bytes, with no case folding and no symbology check.

### Resolution

The signed `i128` coefficient and scale `0..=38` stay a bounded storage primitive. That bound does not make every coefficient or scale valid for every operation. Overflow, including intermediate multiplication overflow, is rejected. Economic comparison uses `numeric_eq` and `numeric_cmp`. `representation_eq` is the structural operation, and it is what `Eq` and `Hash` use. Scale is not a tick size. `Money`, `Price`, and `Quantity` pin currency or instrument and definition versions. Their numeric comparison returns a mismatch error across those boundaries. It does not return `Ok(false)` for a different currency or instrument. Default arithmetic stays exact: no implicit rescale, foreign exchange, or rounding. `RoundingPolicy` is `RejectInexact` (vocabulary default), `TowardZero`, `AwayFromZero`, `HalfEven`, `Floor`, and `Ceiling`. No mode is applied. A mode would be allowed only when a caller supplies a validated target quantum or scale and a business-operation policy, and the result carries provenance. That implementation does not exist. Halfway ties are not tested as successful rounding. Division, conversion, and instrument-tick quantization return `NotImplemented`. `FINANCIAL_ROUNDING_IMPLEMENTED` stays false.

## UD-04 — Version text versus version integers

**Status:** DESIGN_RESOLVED. Distinct `u64` newtypes, opaque versions, and overflow rejection: IMPLEMENTED_AND_LOCALLY_TESTED. Database epoch fencing and a published-schema compatibility validator: IMPLEMENTATION_PENDING.

### Shell still in force

`OrderVersion` is a `u64` because CR-B10 section 1 says so. `SchemaVersion` is a `u64` because the only envelope example is a JSON integer. It has no successor method. A larger integer is not a compatibility claim.

`WriterEpoch` is a `u64` because M0-02A requires overflow rejection for a monotonic local counter. The width is provisional. It is not a broker fence.

`RiskPolicyVersion`, `NormalizerVersion`, `InstrumentDefinitionVersion`, and `ValuationModelVersion` are opaque printable ASCII tokens, 1 to 128 bytes. CR-B10 does not say whether they are integers or semantic versions. They do not order. `PolicyVersion` in CR-B10 section 3 is not a second Rust type; the section 1 name `RiskPolicyVersion` is used. If those names diverge later, add a type rather than overloading this one.

### Resolution

`OrderVersion`, `WriterEpoch`, and `SchemaVersion` stay separate `u64` newtypes. Overflow fails closed. At runtime, `WriterEpoch` must be allocated by the database and transactionally fenced. It must not be inferred from an in-process counter. The shell's `checked_next` is that local counter and is not the fence. `SchemaVersion` denotes the major wire-contract version. Additive evolution must be explicitly compatible and fixture-tested. Numeric comparison is not compatibility. The four opaque versions stay immutable ASCII identifiers with exact equality and no implicit ordering. Registry metadata is where lineage and compatibility are declared. In a risk envelope, `PolicyVersion` means `RiskPolicyVersion`. Do not alias other policy domains onto it. Epoch fencing integration is deferred to the dispatcher milestone. No schema-compatibility validator for published wire schemas exists yet. The M0-02B-a skeleton validator does not perform that job.

## UD-05 — Timestamp resolution

**Status:** DESIGN_RESOLVED. Opt-in RFC 3339 UTC codec with six fractional digits: IMPLEMENTED_AND_LOCALLY_TESTED. `ClockQuality`, `TimestampProvenance`, and persistence-boundary assignment of `PersistedAt`: IMPLEMENTATION_PENDING.

### Shell still in force

`OccurredAt`, `ReceivedAt`, and `PersistedAt` store `i64` microseconds since the Unix epoch. Nanosecond values are rejected even when they are exact multiples of 1_000, so the shell does not truncate. `FromStr`, `Display`, and serde still use a canonical integer string. That draft JSON shape is not the RFC 3339 codec. Zero is an instant, not a missing timestamp. Clock quality is not stored on the timestamp. Ordering inside one type orders stored microseconds only. It does not order events or broker messages.

### Resolution

Signed `i64` Unix microseconds stay the in-memory precision. `from_rfc3339` and `to_rfc3339_utc` are opt-in. Output is normalized UTC with exactly six fractional digits and a `Z` suffix. Parsing requires an explicit `Z` or `±HH:MM` offset, converts with checked range, and rejects nonzero digits after the microsecond. Extra zero digits are exact and are not a truncation. Omitted fractions are exact zeros. Leap seconds and invalid civil dates are rejected. The day product uses a checked `i128` intermediate so `i64::MIN` round-trips without wrapping. An absent source timestamp is `Option<OccurredAt>`. `ClockQuality` and `TimestampProvenance` are named by the resolution and have no variant list in that document, so the enums were not invented and are not inferred. `PersistedAt` is not assigned by the persistence boundary and is not proof of an external broker order. Timestamp order is not broker precedence. Draft serde was not replaced.

## UD-06 — State transitions

**Status:** DESIGN_RESOLVED / IMPLEMENTATION_PENDING

### Shell still in force

CR-B10 section 2 gives draft variant names and says the transition matrix is a separate acceptance item. DET-01 was not supplied when this shell was written. M0-02A defines the enums and does not define edges, guards, or events. `RiskReservationAdjusted` is not a variant. `BrokerLifecycleState::Unknown` is a known label; unrecognized text does not become `Unknown`, `Ready`, `Allow`, `Released`, or `ConfirmedAccepted`.

Declaration order is not lifecycle order. The enums do not implement `Ord` or `Default`.

### Resolution

The enum labels are frozen as draft registry candidates. They are not executable transition rules. Do not add edges or guards until DET-01, DET-02, DET-06, and DET-07 are in the repository and cross-reviewed. Those bodies are present as of TRD-ARCH-SYNC-01 and were not adopted as an executable matrix. Unknown wire state fails closed. Do not implement `Ord` or `Default`. `RiskReservationAdjusted` stays an event, not a state. The draft spelling `Cancelled` stays. A broker adapter must normalize raw vendor spelling and preserve the raw evidence. Transition matrices are not implemented.

## UD-07 — Types named by CR-B10 and not implemented

**Status:** DESIGN_RESOLVED / IMPLEMENTATION_PENDING. `OrderClosure` and `IncidentState` were not added.

### Shell still in force

`OrderClosure` needs a reason vocabulary that CR-B10 does not list. `IncidentState` is named in section 1 and has no variant list. Neither type was invented. `ExecutionState` is implemented from section 2 even though section 1 has no row for it; the glossary marks that ownership as taken from section 2.

### Resolution

`OrderClosure` is to be an immutable record with closure reason, closing `EventId`, closing `OrderVersion`, occurred and recorded time, and an audit reference. The reconstructed DET-01 draft does not confirm a closure-reason vocabulary, so that vocabulary was not invented and the Rust type was not added. A compile-fail doctest rejects `trading_domain::OrderClosure`. It is not a canonical wire schema. `IncidentState` waits until a reviewed DET-10 transition model is adopted. The DET-10 text is present and was not adopted. `ExecutionState` is owned by the execution ledger. The glossary ownership row records that assignment. No state variants were fabricated.

## UD-08 — Canonical JSON

**Status:** DESIGN_RESOLVED / IMPLEMENTATION_PENDING. No canonical schema was published.

### Shell still in force

Serde implementations exist so unknown state text and float amounts can be rejected in tests. Field names and enum spellings are draft. They are not the M0-02B canonical envelope, and identical bytes are not promised across serializer versions. Enum labels use the CR-B10 PascalCase variant names. `RECONCILIATION_REQUIRED` is rejected. `Canceled` is rejected; the broker label is `Cancelled`.

### Resolution

Current serde shapes are an internal draft. M0-02B-b is the milestone that defines canonical event and command wire schemas: fixed field names, string encoding for arbitrary-precision coefficients, stable enum labels, `deny_unknown_fields` on safety-critical messages, bounded payload size, explicit UTC timestamp normalization, and deterministic key ordering and numeric formatting. Canonical bytes require a documented serializer profile and golden fixtures. Ordinary `serde_json` map iteration is not that profile. The event envelope follows CR-B10 §4, with nullability, schema name, major version, and causal identifiers documented separately. A payload hash needs an explicitly named algorithm and domain separator. Draft fields are not silently converted into that hash. M0-02B-a publishes the registry skeleton only. It does not publish those schemas. Golden-byte tests are not implemented.

## UD-09 — UUID version

**Status:** DESIGN_RESOLVED. UUIDv4 generation and typed parsing of arbitrary bits: IMPLEMENTED_AND_LOCALLY_TESTED. Nil rejection at a command trust boundary: IMPLEMENTATION_PENDING, because this repository has no such boundary.

### Shell still in force

CR-B10 proposes UUID version 7 and allows version 4. M0-02A generates version 4, matching the existing workspace dependency. Constructors accept any UUID bits. The version nibble and any timestamp bits are not interpreted. UUID order is not event order.

### Resolution

UUIDv4 generation stays for M0-02A and for the initial M0-02B schemas. UUIDv7 is an optional future ADR, not a migration prerequisite. `from_uuid` and canonical text parsing accept any UUID bits, including nil. That is typed parsing. It does not prove that an identifier was minted internally. `trading-oms` has no command ingress, so there is no trusted command boundary at which to reject nil. Those checks stay unimplemented. UUID sorting is not event order. Historical ids are not rewritten.

## XC findings after TRD-ARCH-03A and TRD-ARCH-03A.2

TRD-ARCH-03A does not change the UD statuses above and does not close these findings. TRD-ARCH-03A.2 does not close them either. SMQ-01 through SMQ-05 are DECIDED as interpretation constraints. The underlying XC and UD issues stay OPEN.

XC-01 stays OPEN. XC-02-PRED stays OPEN. XC-03-MATRIX stays OPEN. XC-05-LOCK stays OPEN. XC-06 stays OPEN. XC-07-ROUND stays OPEN. XC-08-IBKR stays OPEN. XC-09-RESET stays OPEN. XC-10-ACTION stays OPEN.

XC-11 status-word disagreement stays OPEN. ARCH-DEC-04 chooses no winning status and no default precedence. The population and supersession sub-finding remains `RESOLVED_BY_ARCH_DEC_02`. XC-04 design precedence remains `RESOLVED_BY_ARCH_DEC_03`. The rest of XC-04 stays OPEN, including broker-session exclusivity, lease and heartbeat, dispatch scope, in-flight send evidence, recovery tests, and IBKR behavior.

`NO_CONFLICT_FOUND` on a principle does not permit an unresolved predicate or transition. `IMPLEMENTED_AND_LOCALLY_TESTED` on a UD line stays scoped to that named part. It does not set an ADFR row to verified. DET-01 and DET-02 Q01–Q10 verification cells stay OPEN. Live trading stays NOT AUTHORIZED.

## OMS proposal after TRD-ARCH-03B.1

TRD-ARCH-03B.1 does not change the UD statuses above and does not close the XC findings in the previous section. The review package is [contracts/design-proposals/README.md](../design-proposals/README.md). As written, OMSQ-01 through OMSQ-12 are `USER_DECISION_PENDING`. `XC-03-MATRIX` stays OPEN. No transition edge and no `OrderClosure` vocabulary are approved. `OrderClosure` stays an absent type. `ORDER_STATE_MACHINE_IMPLEMENTED` and `STATE_TRANSITIONS_IMPLEMENTED` stay false.

## OMS design policy after TRD-ARCH-03B.1.2

TRD-ARCH-03B.1.2 records design-policy choices in [ARCH-DEC-09-bis-20-2026-11-15.md](../decisions/ARCH-DEC-09-bis-20-2026-11-15.md). It does not change the UD status lines above and does not close the XC findings in the previous section. The choices are `RATIFIED_DESIGN_POLICY_ONLY`, `NOT_IMPLEMENTED`, and `NOT_VERIFIED`.

OMSQ-01 option A keeps one immutable historical closure record and puts later facts outside it. OMSQ-02 option C is a modeling strategy only and does not approve tokens, an enum, fields, or an edge. OMSQ-03 option A forbids closure on an unknown send and forbids automatic retry and takeover. OMSQ-04 option A retains cancel and fill facts and does not release risk, because OMSQ-06 option A accepts no economic-finality predicate. OMSQ-05 option B quarantines every proposed economic effect, including a single match, and overrides any automatic posting reading of OMSQ-08. OMSQ-07 option A is an acceptance bar for a future matrix and does not approve `XC-03-MATRIX` or any edge. OMSQ-08 option A quarantines conflicts except identical-payload replay, which returns a stored result and creates no new economic effect. OMSQ-09 option A accepts no broker observation as authoritative terminal. OMSQ-10 option A leaves the lock order and isolation unadopted. OMSQ-11 option A requires a distinct in-transaction audit record and names no table. OMSQ-12 option A blocks modify and implicit cancel-replace and does not authorize cancel during halt.

XC-01 stays OPEN. XC-02-PRED stays OPEN. XC-03-MATRIX stays OPEN. XC-05-LOCK stays OPEN. XC-06 stays OPEN. XC-07-ROUND stays OPEN. XC-08-IBKR stays OPEN. XC-09-RESET stays OPEN. XC-10-ACTION stays OPEN. XC-11 status-word disagreement stays OPEN. XC-04 design precedence remains `RESOLVED_BY_ARCH_DEC_03`. The rest of XC-04 stays OPEN. Live trading stays NOT AUTHORIZED.

## Risk proposal after TRD-ARCH-03B.2

TRD-ARCH-03B.2 does not change the UD status lines above and does not close the XC findings in the previous sections. The review package is [contracts/design-proposals/README.md](../design-proposals/README.md). RISKQ-01 through RISKQ-14 are `USER_DECISION_PENDING`. A recommendation in that package is not a decision. `XC-02-PRED` stays OPEN. No economic-finality predicate, no economic posting, and no reservation release are approved. OMSQ-05 option B and OMSQ-06 option A stay binding. ARCH-03B.3 is not started. Live trading stays NOT AUTHORIZED.

## Risk design policy after TRD-ARCH-03B.2.2

TRD-ARCH-03B.2.2 records design-policy choices in [ARCH-DEC-21-bis-34-2026-11-15.md](../decisions/ARCH-DEC-21-bis-34-2026-11-15.md). It does not change the UD status lines above and does not close the XC findings in the previous sections. The choices are `RATIFIED_DESIGN_POLICY_ONLY`, `NOT_IMPLEMENTED`, and `NOT_VERIFIED`. The historical proposal text stays `USER_DECISION_PENDING`. A recommendation in that proposal was not the approval.

RISKQ-01 option B keeps the six scope names and composes hard denies by conjunction, without arithmetic. RISKQ-02 option A requires one reservation identity and all-or-nothing scopes and authorizes no writer. RISKQ-03 option A refuses a reservation when required valuation inputs are missing or stale. RISKQ-04 option A retains a fill and holds the entire reservation. RISKQ-05 option A leaves the residual obligation unchanged. RISKQ-06 option A retains corrections and quarantines every proposed economic effect, including a unique match, and drafts no compensating ledger line. RISKQ-07 option A blocks new risk-increasing commands and authorizes no broker action. RISKQ-08 option A adopts no evidence list. RISKQ-09 option A accepts no release proof. RISKQ-10 option B requires future valuation citations only and adopts no rounding mode and no FX rate. RISKQ-11 option B requires future concurrency and crash tests, marked `NOT_EXECUTED`, and adopts no lock order. RISKQ-12 option A repeats the unnamed in-transaction audit record. RISKQ-13 option A adds no role and does not let review post. RISKQ-14 option B requires a new immutable policy version for a future change and does not release risk or resubmit on recovery.

XC-01 stays OPEN. XC-02-PRED stays OPEN. XC-03-MATRIX stays OPEN. XC-05-LOCK stays OPEN. XC-06 stays OPEN. XC-07-ROUND stays OPEN. XC-08-IBKR stays OPEN. XC-09-RESET stays OPEN. XC-10-ACTION stays OPEN. XC-11 status-word disagreement stays OPEN. XC-04 design precedence remains `RESOLVED_BY_ARCH_DEC_03`. The rest of XC-04 stays OPEN. ARCH-DEC-13 and ARCH-DEC-14 stay binding. ARCH-03B.3 is not started. Live trading stays NOT AUTHORIZED.
