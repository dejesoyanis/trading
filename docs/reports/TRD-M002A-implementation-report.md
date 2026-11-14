# TRD-M002A implementation report

**Milestone:** M0-02A canonical domain types and glossary  
**Date:** 2026-11-14  
**Branch:** `main`  
**Trading:** not implemented

## Preflight

Working tree before edits: clean. No pre-existing uncommitted changes were overwritten.

| Item | Observed value |
| --- | --- |
| Branch | `main`, tracking `origin/main` |
| Base SHA | `615b9faa1d41d86d2c695b010ead4ffeb21181ab` |
| HEAD subject | `M0-01A: Foundation Verification & Hardening.` |
| Previous commit | `3c4c145` `M0-01: Rust Trading Platform Foundation.` |
| Toolchain | `rustc 1.99.1 (58cf8f9c3 2026-10-13)`, `cargo 1.99.1` |
| Toolchain file | `rust-toolchain.toml` channel `1.99.1`, components `clippy` and `rustfmt` |

Workspace members: `apps/trading-api`, `crates/trading-common`, `crates/trading-domain`, `crates/trading-application`, `crates/trading-oms`, `crates/trading-risk`, `crates/trading-broker`, `crates/trading-ibkr`, `crates/trading-persistence`.

M0-01A is present in source, not only as a prompt. The API separates `migrate` from `GET /health/ready`, readiness is read-only, and `0001_infrastructure_metadata.sql` creates infrastructure metadata only. Reserved OMS, risk, broker, and IBKR crates still export only `*_IMPLEMENTED = false` flags. CI remains `.github/workflows/ci.yml` and was not changed.

DET-01 through DET-10, CR-B01 through CR-B09, and `Implementation-Roadmap-and-Cursor-Prompt-Plan-v1.0.md` were not in the repository or the task message. CR-B10 was supplied and labels its enums as draft names, with transition matrices deferred. Missing DET text blocks a state machine. It does not block fail-closed shells that do not invent transitions. Those gaps are recorded in `contracts/domain/unresolved-decisions.md` instead of being filled in.

Preflight commands, all exit 0:

- `cargo fmt --all -- --check`
- `cargo check --workspace --locked`
- `cargo test --workspace --locked`

`crates/trading-persistence/tests/connectivity.rs` reported `migration_is_explicit_and_readiness_is_read_only` as ignored. PostgreSQL was not contacted. That result is not a passed database test.

## Plan that was implemented

Put the vocabulary in `trading-domain`, which was already the boundary for future domain types. Leave `trading-oms`, `trading-risk`, `trading-broker`, and `trading-ibkr` as reserved behavior crates so their "not implemented" flags stay true. Do not depend on `trading-api`, `trading-common`, or `trading-persistence`, so the graph stays acyclic and domain code cannot open HTTP or PostgreSQL.

Dependencies added on `trading-domain`: existing locked `serde` 1.0.229, `thiserror` 2.0.21, and `uuid` 1.29.1 (version-4 feature only). Dev-dependency `serde_json` 1.0.154, already in the lockfile. `Cargo.lock` gained those edges and no other version bumps.

## Files

- `crates/trading-domain/src/lib.rs` — exports, closed behavior flags, compile-fail doctests
- `crates/trading-domain/src/identifier.rs` — seventeen UUID newtypes
- `crates/trading-domain/src/version.rs` — counters, schema integer, opaque versions
- `crates/trading-domain/src/state.rs` — eight draft state enums
- `crates/trading-domain/src/identity.rs` — scoped broker and execution identities
- `crates/trading-domain/src/amount.rs` — decimal, money, price, quantity
- `crates/trading-domain/src/time.rs` — `OccurredAt`, `ReceivedAt`, `PersistedAt`
- `crates/trading-domain/src/error.rs`, `parse.rs`
- `crates/trading-domain/Cargo.toml`
- `contracts/domain/glossary.md`
- `contracts/domain/unresolved-decisions.md`
- `docs/cursor-prompts/trading/TRD-M002A-Canonical-Domain-Types-and-Glossary.md`
- `docs/architecture/README.md`, `README.md` — descriptions that would otherwise still say the domain crate was empty
- `crates/trading-common/src/correlation.rs`, `lib.rs` — document the HTTP handle as a different type
- `Cargo.lock`

No migration, CI workflow, broker crate behavior, or API route was added.

## Type decisions

UUID ids are distinct newtypes. `generate` uses version 4. `from_uuid` accepts any bits and does not read a clock. `Display` / `FromStr` accept only the lowercase hyphenated form. There is no `From` conversion between id types.

`OrderVersion` and `WriterEpoch` are `u64` counters. `checked_next` returns `VersionOverflow` at `u64::MAX` and does not wrap. `SchemaVersion` is a `u64` with no successor. The other four versions are opaque tokens because CR-B10 does not fix their encoding. Details are UD-04.

Broker aliases carry broker, account, environment, an external id, and either no session or one internal `BrokerSessionId`. Every scope includes `UnverifiedSessionScope`. A separate IBKR client id is not modeled. `BrokerOrderAlias` and `BrokerPermIdAlias` are different types. `QuarantinedExecutionIdentity` has no conversion back to `ExecutionIdentity`. There is no resolver. Details are UD-02.

State enums use the CR-B10 §2 names, including `Cancelled` rather than `Canceled`. Unknown text returns `UnknownState` and is not stored on the error. `BrokerLifecycleState::Unknown` is a real label; other strings do not become it. `Default` is not implemented. `RiskReservationAdjusted` is not a variant. No transition methods exist.

Amounts use `i128` plus a scale of 0 through 38. Mixed currency, instrument, definition version, or scale fails. Rescale, foreign exchange, and instrument conversion return `NotImplemented`. No `f32` or `f64` field exists. The only `f64` token in the crate is a compile-fail doctest that must not construct a decimal. Timestamps are `i64` microseconds. Nanoseconds and RFC 3339 are rejected, including exact multiples of 1_000. Details are UD-03 and UD-05.

`trading_common::CorrelationId` stays the HTTP request handle. `trading_domain::CorrelationId` is the domain id. No conversion was added. Details are UD-01.

`OrderClosure` and `IncidentState` were not invented. `ExecutionState` is taken from CR-B10 §2, which has no §1 ownership row. Details are UD-07.

Serde exists so tests can reject unknown states and float amounts. It is not the M0-02B canonical envelope. Details are UD-08.

## Checks after the change

All four commands exited 0:

```text
cargo fmt --all -- --check
cargo check --workspace --locked
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
```

`cargo test --workspace --locked` executed 83 library tests and 17 `trading-domain` doctests (1 example and 16 `compile_fail` examples). All of those passed. The doctests include mismatched identifier types, `Closed` versus `Released`, missing `Default` on readiness, risk, and dispatch, `f64` rejected by `DecimalAmount::try_new`, and no `Add` for `Money`.

`trading-persistence` connectivity stayed ignored:

```text
test migration_is_explicit_and_readiness_is_read_only ... ignored, requires TRADING_ALLOW_DATABASE_INTEGRATION_TESTS=1 and TRADING_TEST_DATABASE_URL
test result: ok. 0 passed; 0 failed; 1 ignored
```

Existing API health, shutdown, configuration, and persistence unit tests stayed green (17 API tests, 15 persistence unit tests).

## Tests not run

- PostgreSQL integration test. Opt-in variables were not set. Docker Compose was not started. This report does not claim that test passed.
- GitHub Actions. The workflow file was not changed and this commit was not pushed.
- B10-T01, T02, T03, T04, T06, T11, T12. Registry and canonical-byte gates are M0-02B.
- B10-T07, T08, T10. Economic replay and transition tests are M0-03 or later.
- B10-T05 and B10-T09 are only partially represented by the unit tests below. They are not claimed as passed registry gates.

## Acceptance gates

| Gate | Result |
| --- | --- |
| A01 | Met. Compile-fail doctests reject cross-type `==` and argument mismatch for order/command ids, correlation/causation ids, and version counters. A generated test checks distinct `TypeId`s for every UUID newtype. |
| A02 | Met. `equal_raw_ids_in_different_scopes_are_not_equal` covers account, broker, environment, and session differences. Account-only and account-plus-session with the same raw id are not equal. |
| A03 | Met. Unknown labels, including near-misses of `Ready`, `Allow`, `Released`, and `ConfirmedAccepted`, fail `TryFrom` and `serde_json`. Exact labels still decode. Garbage does not become `BrokerLifecycleState::Unknown`. |
| A04 | Met. Compile-fail doctests reject `OrderWorkflowState::Closed` as a `RiskReservationState` and reject the missing variants `Released`, `Closed`, and `Adjusted`. No transition method exists. `STATE_TRANSITIONS_IMPLEMENTED` is `false`. |
| A05 | Met. Currency, instrument, and definition-version mismatches return typed errors. Scale mismatch and `i128` overflow, including negation of `i128::MIN`, are tested. |
| A06 | Met. Coefficients are `i128`. JSON rejects numeric and textual floats. Compile-fail doctest passes `f64` to `DecimalAmount::try_new`. |
| A07 | Met. `contracts/domain/glossary.md` has owner, meaning, invariants, neighbors, and a CR-B10 section for each introduced type. DET bodies were absent, and the table says so. |
| A08 | Met. The four commands above exited 0. The ignored database test is reported, not hidden. |
| A09 | Met. No broker client, socket, credential, submission function, or migration was added. Dispatch, resolver, rounding, risk evaluation, and the order state machine flags are `false`. |
| A10 | Met. Health, readiness, and persistence unit tests passed unchanged in behavior. HTTP readiness was not wired to `TradingReadinessState`. |

## Deviations

These are provisional shells, not closed ADRs. The open record is `contracts/domain/unresolved-decisions.md`.

- Domain types live in `trading-domain` submodules rather than the reserved OMS, risk, and broker crates. Those crates still say the corresponding behavior is absent. Moving types there would blur that boundary.
- `RiskPolicyVersion`, `NormalizerVersion`, `InstrumentDefinitionVersion`, and `ValuationModelVersion` are opaque text because a numeric encoding would have been a guess.
- Two `CorrelationId` types coexist until a later ADR renames one.
- `OrderClosure` and `IncidentState` were not added.
- This file cannot contain the SHA of the commit that adds it. That SHA is a hash of these bytes. The parent is `615b9faa1d41d86d2c695b010ead4ffeb21181ab`. The milestone commit is the single commit on `main` whose subject is `TRD-M002A: Canonical Domain Types & Glossary`.

## Security

No credentials, broker sessions, or database URLs were introduced. Parse errors do not echo the rejected text. Alias JSON rejects `session_scope` other than `unverified` and rejects a perm-id document decoded as an order alias. Quarantined execution JSON does not decode as an unflagged `ExecutionIdentity`. Environment labels `paper` and `live` are strings, not authorization. No network stack was added to `trading-domain`. Schema SQL was not changed.

## Remaining M0-02B prerequisites

- Decide UD-01 through UD-09, especially broker client/session scope, decimal rounding, version encoding, and the two correlation ids, before publishing a registry.
- Add the schema registry, event and command schemas, and referential checks for B10-T01 through T04.
- Define canonical serialization and fixtures for B10-T06, T11, and T12 without treating the current serde shapes as that canonical form.
- Obtain DET transition tables before any state machine. Do not promote these draft labels to production semantics by default.

## Final commit SHA

The commit that contains this report is identified by subject `TRD-M002A: Canonical Domain Types & Glossary`. Its SHA is not written here, because writing it would change the hash. Read it with `git rev-parse HEAD` after that commit. This commit was not pushed.
