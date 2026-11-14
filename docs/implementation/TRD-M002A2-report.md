# TRD-M002A.2 implementation report

**Milestone:** M0-02A.2 resolved domain type contracts  
**Date:** 2026-11-14  
**Branch:** `main`  
**Live trading:** NOT AUTHORIZED  
**M0-02B canonical publication:** NO-GO

## Baseline

Working tree before edits: clean. No unrelated changes were present.

| Item | Observed value |
| --- | --- |
| Branch | `main` |
| HEAD before this change | `d0398bcc8710a36659bcf1d0240cb43112e61f2c` |
| HEAD subject | `M0-02A: Record UD-01..UD-09 and keep canonical schemas unpublished` |
| Recorded baseline is an ancestor of HEAD | yes; it was HEAD |
| Toolchain | `rustc 1.99.1 (58cf8f9c3 2026-10-13)`, `cargo 1.99.1` |
| Toolchain file | `rust-toolchain.toml` channel `1.99.1`, components `clippy` and `rustfmt` |

Workspace members: `apps/trading-api`, `crates/trading-common`, `crates/trading-domain`, `crates/trading-application`, `crates/trading-oms`, `crates/trading-risk`, `crates/trading-broker`, `crates/trading-ibkr`, `crates/trading-persistence`.

Remote GitHub Actions were not run.

## Source inventory

Present and read from the repository:

- `contracts/sources/M0-02A-UD-01-bis-UD-09-Architecture-Resolution-v1.0.md`
- `contracts/sources/CR-B10-Domain-Glossary-and-Schema-Registry-v1.0.md`
- `contracts/domain/unresolved-decisions.md`
- `contracts/domain/glossary.md`
- `contracts/traceability-index.md`
- `contracts/registry/registry.json`

Absent, and left `NOT_PROVIDED / NOT_REVIEWED`: DET-01 through DET-10, CR-B01 through CR-B09, ADFR / ARCH-01C, and the other documents already listed in the traceability index. The resolution's basis line cites DET-03 through DET-08. Those bodies are not in the repository. That citation was not treated as a review. No conflict between the resolution text in the repository and the implementation prompt required a subtask to stop.

## Implementation strategy

Keep structural equality, draft serde, and fail-closed operations. Add only the comparisons, vocabulary, and opt-in codec whose invariants are tested. Do not publish a canonical schema.

`trading-domain` now depends on `trading-common` so the non-interchangeability doctest can name both identifier types. `trading-common` does not open a socket or a database. The API binary's crate set is unchanged apart from the renamed HTTP type inside `trading-common`.

No property-test crate was added. Numeric properties use a deterministic xorshift seeded with `0x4D30_325F_5544_3033`. `Cargo.lock` gained one edge, `trading-domain` to `trading-common`, and no version bumps.

## Changed files

- `crates/trading-common/src/correlation.rs`, `lib.rs` — `HttpRequestId`
- `apps/trading-api/src/health.rs`, `server.rs` — call site and header regression
- `crates/trading-domain/src/amount.rs`, `error.rs` — numeric comparison and `RoundingPolicy`
- `crates/trading-domain/src/time.rs` — opt-in RFC 3339 codec
- `crates/trading-domain/src/identity.rs` — unverified broker context
- `crates/trading-domain/src/identifier.rs`, `version.rs`, `lib.rs` — boundary tests and docs
- `crates/trading-domain/Cargo.toml`, `Cargo.lock`
- `crates/trading-domain/tests/registry_skeleton.rs`
- `contracts/domain/glossary.md`, `contracts/domain/unresolved-decisions.md`
- `contracts/traceability-index.md`, `contracts/registry/registry.json`, `contracts/registry/README.md`
- `docs/architecture/README.md`
- this report and the archived prompt

## Behavioral changes

`trading_common::HttpRequestId` replaces the exported HTTP type. `Display` is still a lowercase hyphenated UUID. The response header remains `x-correlation-id`. The tracing field remains `correlation_id`. Health JSON bodies are unchanged. There is no conversion to `trading_domain::CorrelationId`. No deprecated alias was added: the only in-repository caller was the health middleware, and exporting both names would have left two identifiers under the correlation name.

`DecimalAmount`'s derived `Eq` and `Hash` stay structural. `representation_eq` names that relation. `numeric_eq` and `numeric_cmp` compare mathematical value by checked multiplication by a power of ten. If that product does not fit in `i128`, the lower-scale value has a larger magnitude than every coefficient at the higher scale, so the comparison does not wrap. Money, price, and quantity numeric comparison returns a typed mismatch across currency, instrument, or definition version. It does not return `Ok(false)` for those mismatches. `RoundingPolicy` lists `RejectInexact` (vocabulary default), `TowardZero`, `AwayFromZero`, `HalfEven`, `Floor`, and `Ceiling`. `rescale`, `rescale_with_policy`, `checked_div`, `quantize_to_tick`, foreign exchange, and instrument conversion return `NotImplemented`. `FINANCIAL_ROUNDING_IMPLEMENTED` stays false. Halfway-tie results are not produced.

`from_rfc3339` and `to_rfc3339_utc` are opt-in. Output is UTC with exactly six fractional digits and a `Z` suffix. Input requires `Z` or `±HH:MM`. Nonzero digits after the microsecond are rejected. Extra zeros are exact. Fewer than six fractional digits are padded, not truncated. Draft `Display`, `FromStr`, and serde remain the integer microsecond string. The day product uses a checked `i128` intermediate so `i64::MIN` round-trips. Zero is still a present instant. Absence is `Option`. Timestamp order is not event order.

`BrokerClientId` is a printable-ASCII token, not `BrokerSessionId`. `BrokerIdentityContext` records broker, account, environment, alias kind, and the raw external id, plus optional client id, internal session, adapter epoch, and evidence id. Every context carries `UnverifiedSessionScope`. There is no verified constructor, merge, or resolver. Draft JSON rejects `verified`. Quarantined execution JSON does not deserialize as an unflagged identity or as a context. `IDENTITY_RESOLVER_IMPLEMENTED` stays false. Broker uniqueness stays `EVIDENCE_PENDING`.

`OrderVersion`, `WriterEpoch`, and `SchemaVersion` are unchanged in behavior. `WriterEpoch::checked_next` remains an in-process counter, not a database fence. Opaque versions remain unordered. State enums were not given transitions, `Ord`, or `Default`. `OrderClosure` and `IncidentState` were not added. `ClockQuality` and `TimestampProvenance` were not added: the resolution names them and does not list variants. UUIDv4 generation is unchanged. Nil UUID bits still parse. `trading-oms` has no command ingress, so no nil rejection was added at a boundary that does not exist. The registry remains `SKELETON_NOT_CANONICAL`. No schema directory was created.

## Per-decision status

| Decision | Delivered | Still pending |
| --- | --- | --- |
| UD-01 | `IMPLEMENTED_AND_LOCALLY_TESTED` | Public schema exposure |
| UD-02 | Client id and unverified context: `IMPLEMENTED_AND_LOCALLY_TESTED` | Kind-specific uniqueness and verified scopes: `IMPLEMENTATION_PENDING`. Uniqueness: `EVIDENCE_PENDING` |
| UD-03 | Numeric comparison and `RoundingPolicy` vocabulary: `IMPLEMENTED_AND_LOCALLY_TESTED` | Applied rounding, rescale, division, FX, tick quantization: `IMPLEMENTATION_PENDING` |
| UD-04 | Distinct versions and overflow checks remain `IMPLEMENTED_AND_LOCALLY_TESTED` | Database epoch fence and schema-compatibility validator: `IMPLEMENTATION_PENDING` |
| UD-05 | Opt-in RFC 3339 codec: `IMPLEMENTED_AND_LOCALLY_TESTED` | `ClockQuality`, `TimestampProvenance`, and persistence assignment of `PersistedAt`: `IMPLEMENTATION_PENDING` |
| UD-06 | Draft labels unchanged | Transitions: `IMPLEMENTATION_PENDING` |
| UD-07 | `ExecutionState` ownership unchanged | `OrderClosure` and `IncidentState`: `IMPLEMENTATION_PENDING` because DET-01 and DET-10 are absent |
| UD-08 | Draft serde retained | Canonical bytes: `IMPLEMENTATION_PENDING` |
| UD-09 | UUIDv4 and typed parsing remain `IMPLEMENTED_AND_LOCALLY_TESTED` | Nil rejection at a command trust boundary: `IMPLEMENTATION_PENDING` |

## Tests

| Command | Exit code |
| --- | --- |
| `cargo fmt --all -- --check` | 0 |
| `cargo check --workspace --locked` | 0 |
| `cargo test --workspace --locked` | 0 |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | 0 |

`cargo test --workspace --locked` results:

- `trading-api` library: 17 passed
- `trading-application`: 1 passed
- `trading-broker`: 1 passed
- `trading-common`: 3 passed
- `trading-domain` library: 52 passed
- `trading-domain` `registry_skeleton`: 1 passed
- `trading-ibkr`: 1 passed
- `trading-oms`: 1 passed
- `trading-persistence` library: 15 passed
- `trading-risk`: 1 passed
- `trading-domain` doctests: 27 passed, including 26 compile-fail doctests
- `trading-persistence` `connectivity`: 1 ignored

The PostgreSQL integration test `migration_is_explicit_and_readiness_is_read_only` is **ignored, not passed**. It requires `TRADING_ALLOW_DATABASE_INTEGRATION_TESTS=1` and `TRADING_TEST_DATABASE_URL`. This run did not set them and did not contact a database.

Remote GitHub Actions were **not run**.

## Skipped and unresolved

- Applied rounding, including half-even ties, rescale, division, foreign exchange, and tick quantization
- `ClockQuality` and `TimestampProvenance` variants
- Verified broker scopes and kind-specific uniqueness
- `OrderClosure` and `IncidentState`
- State transition matrices
- Database writer-epoch fencing
- Canonical event and command schemas, golden bytes, and compatibility fixtures
- Nil UUID rejection at a command boundary
- PostgreSQL integration and remote CI

## Security review

No order path, risk decision, broker socket, credential, migration, or trading flag was added. Health responses still omit database errors and credentials. The request id header is still UUID text and is not copied into the health JSON body. Identity documents that claim `verified` are rejected. Quarantine still has no promotion path. Decimal and timestamp code uses checked integers. Nonzero sub-microsecond digits are rejected rather than truncated. Cross-currency and cross-instrument numeric comparison returns an error. Error text still does not echo rejected input. This is not an authorization review of a live broker.

## Deviations

- `trading-domain` depends on `trading-common` for the cross-crate compile-fail doctest. The previous architecture note said domain did not depend on common. The README now records the edge and why domain still cannot open HTTP or PostgreSQL.
- The old HTTP type name was removed instead of kept as a deprecated alias. The header name is the compatibility adapter.
- RFC 3339 input accepts an omitted fraction and fewer than six digits, and it accepts extra digits only when they are `0`. Output always has six digits. Nonzero extra digits fail.
- `rescale_with_policy` was added beside the existing `rescale`. Both return `NotImplemented`.

## Gates

| Gate | Result |
| --- | --- |
| A01 | PASS |
| A02 | PASS |
| A03 | PASS |
| A04 | PASS |
| A05 | PASS |
| A06 | PASS |
| A07 | PASS |
| A08 | PASS |
| A09 | PASS |
| A10 | PASS for unit tests and health/readiness. PostgreSQL integration is ignored, not passed |
| A11 | PASS as a report. The ignored integration test and the unrun remote CI are named above. Broker uniqueness remains `EVIDENCE_PENDING` |
| A12 | The prompt and this report are in the commit. The subject is `TRD-M002A.2: Implement Resolved Domain Type Contracts`. The resulting SHA is reported after the commit and is not written in this file |

These local gates do not accept M0-02B canonical publication and do not authorize live trading.

## Next step

Supply and cross-review DET-01, DET-02, DET-06, DET-07, and DET-10 before `OrderClosure`, incident states, or transition matrices. Keep M0-02B-b NO-GO until those sources, the still-pending rounding and identity evidence, and a separate canonical wire gate are in place. Do not treat this commit as permission to submit orders.
