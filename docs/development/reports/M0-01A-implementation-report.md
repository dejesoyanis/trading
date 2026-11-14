# M0-01A Implementation Report

## Summary

M0-01A separates infrastructure migration from HTTP readiness and makes process shutdown observable. `trading-api migrate` applies the existing infrastructure migration and exits. `Database::check_ready` only reads connectivity and schema compatibility. The API can serve `/health/live` when PostgreSQL is down or the schema is missing. Readiness does not authorize trading.

The work was done on `main` at parent `3c4c145` (`M0-01: Rust Trading Platform Foundation.`). The working tree was clean before the edit. No trading behavior was added.

## Status labels

| Area | Status |
| --- | --- |
| Migration command and read-only readiness | Implemented, locally verified for unreachable PostgreSQL and for the pure schema comparison. Fresh, migrated, and tampered databases are implemented and are **pending CI**. Not locally verified against PostgreSQL. |
| HTTP health shape, correlation id, transition logs, bounded readiness | Implemented and locally verified |
| Graceful shutdown, timeout, unexpected task exit, Drop fallback | Implemented and locally verified with injected tasks. Unix SIGTERM is compiled in and was not exercised with a live signal on this Windows machine. |
| Formatting, Clippy, unit tests, locked build | Locally verified |
| GitHub Actions workflow, including the real PostgreSQL test | Implemented. **Pending** a push and an observed run. Not CI verified. |
| Docker Compose startup (original M0-01 AC-08) | **Blocked** on this machine: Docker, WSL, and PostgreSQL are not installed. Not claimed. |
| Remote CI success (AC-A15) | **Pending** |

## Changed files

- `.env.example`
- `.github/workflows/ci.yml`
- `README.md`
- `apps/trading-api/Cargo.toml`
- `apps/trading-api/src/command.rs` (new)
- `apps/trading-api/src/error.rs`
- `apps/trading-api/src/health.rs`
- `apps/trading-api/src/lib.rs`
- `apps/trading-api/src/logging.rs`
- `apps/trading-api/src/main.rs`
- `apps/trading-api/src/server.rs`
- `crates/trading-persistence/Cargo.toml`
- `crates/trading-persistence/src/database.rs`
- `crates/trading-persistence/src/error.rs`
- `crates/trading-persistence/src/lib.rs`
- `crates/trading-persistence/src/readiness.rs` (new)
- `crates/trading-persistence/src/redact.rs`
- `crates/trading-persistence/tests/connectivity.rs`
- `docs/architecture/README.md`
- `docs/cursor-prompts/M0-01A-foundation-verification-hardening.md` (new)
- `docs/development/README.md`
- `docs/development/reports/M0-01A-implementation-report.md` (new)
- `tests/README.md`

`infrastructure/docker/compose.yaml`, the migration SQL, and `Cargo.lock` were not changed. No reserved crate gained behavior.

## Implementation decisions

`trading-api migrate` is the migration entry point. A second binary would have duplicated configuration and redaction. With no arguments the same binary serves HTTP and does not migrate. Unknown arguments exit before using the database URL.

`Database` now holds only the pool. `connect` does not migrate. `migrate` runs the embedded SQLx migrator on one connection, raises that connection's `statement_timeout` to 30 seconds, and closes the connection so the longer timeout cannot be reused by readiness. `check_ready` is called on every probe and is not cached.

Readiness is ready only when both of these hold:

- every embedded migration is in `_sqlx_migrations` with `success = true` and the embedded checksum
- `infrastructure_metadata.schema_purpose` equals `infrastructure-metadata-only`

A missing history is `schema_missing`. A checksum mismatch, failed row, unknown version, or disagreeing metadata is `schema_incompatible`. Neither check repairs the schema. SQLx itself rejects a checksum mismatch instead of rewriting it. The metadata value is not treated as sufficient on its own. This is still not a live attestation of every catalog object: a role that can write `_sqlx_migrations` can store the embedded checksum.

`PersistenceError` display and debug do not contain raw driver errors. Health logs use `public_reason` (`schema_missing`, `database_unreachable`, and the other classified codes) and emit that log only when the result changes. The migration command may log one redacted diagnostic. sqlx tracing targets are turned off.

`/health/live` and `/health/ready` keep the previous JSON fields. Success is HTTP 200. Failure is HTTP 503 with `postgres: unavailable`. Each response still carries `x-correlation-id`. Readiness waits at most 2 seconds. Pool acquisition waits at most 1 second, and API statements use a 1 second PostgreSQL `statement_timeout`.

The process selects on the HTTP task and a shutdown signal. Ctrl-C is handled on every platform. Unix builds also handle SIGTERM. After the signal, shutdown waits 10 seconds, then aborts the task and returns `ShutdownTimeout`. `Drop` signals and aborts without waiting. An HTTP task that finishes before a signal fails the process. Process shutdown is not a trading halt.

The integration test ignores `DATABASE_URL`. It runs only with `TRADING_ALLOW_DATABASE_INTEGRATION_TESTS=1` and a `TRADING_TEST_DATABASE_URL` whose database name contains `_test`. Destructive steps use uniquely named databases created for the test and dropped afterward. The same `Database` value is not-ready before `migrate` and ready afterward, which is the persistence-layer form of recovery without an API restart. The HTTP handler calls `check_ready` on every request and does not remember a successful schema, so the same transition applies to a running API. An HTTP-level test against PostgreSQL was not added. Other backends of the isolated database are terminated to exercise pool recovery. A PostgreSQL process restart was not tested.

## Dependencies

No third-party crate was added or removed. `sqlx` and `url` are now also dev-dependencies of `trading-persistence` so the integration test can create and inspect databases. Both were already workspace dependencies. `Cargo.lock` is unchanged.

## Deviations

- The migration entry point is `trading-api migrate`, not a separate `trading-db` binary.
- Schema recovery without restart is tested on `Database`, not through a live HTTP server connected to PostgreSQL.
- SIGTERM is wired with `cfg(unix)` and was not delivered to a test process. The graceful path is tested with a deterministic signal future. This Windows session cannot observe SIGTERM.
- Full PostgreSQL restart recovery is deferred. Backend termination is the disconnect test that does not restart the service.
- The commit hash is not stored in this file. This report is inside the commit, and writing the hash into the tree would change the hash. The handoff states `git rev-parse HEAD`.

## Local command results

Executed on Windows with the toolchain in `rust-toolchain.toml`, after the final source was in place.

| Command | Result |
| --- | --- |
| `cargo fmt --all -- --check` | Exit 0 |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | Exit 0 |
| `cargo test --workspace --locked` | Exit 0. 41 tests passed. 1 integration test ignored. Doc-tests contained no tests. |
| `cargo build --workspace --locked` | Exit 0 |
| PostgreSQL integration test | Not executed. `cargo test --workspace` reports `migration_is_explicit_and_readiness_is_read_only ... ignored`. |
| Docker Compose | Not executed. Docker, WSL, and PostgreSQL are not installed. |

`cargo test --workspace --locked` package results:

- `trading-api` library: 17 passed
- `trading-api` binary: 0 tests
- `trading-application`: 1 passed
- `trading-broker`: 1 passed
- `trading-common`: 3 passed
- `trading-domain`: 1 passed
- `trading-ibkr`: 1 passed
- `trading-oms`: 1 passed
- `trading-persistence` library: 15 passed
- `trading-persistence` connectivity: 1 ignored
- `trading-risk`: 1 passed

## Integration test invocation

Local `cargo test --workspace --locked` does not run it.

GitHub Actions job `postgres` runs:

```sh
cargo test -p trading-persistence --test connectivity --locked -- --ignored --nocapture
```

with `TRADING_ALLOW_DATABASE_INTEGRATION_TESTS=1` and `TRADING_TEST_DATABASE_URL` pointing at the workflow's `trading_test` database on `postgres:18.6`. The service health check is `pg_isready -U trading -d trading_test`.

## CI

The workflow grants `contents: read` only. Formatting, Clippy, unit tests, and build use `--locked`. There is no deployment job and no push step. `pull_request` is the normal unprivileged event, not `pull_request_target`.

Action commits were resolved from the GitHub API on 2026-11-14 and pinned:

| Action | Resolved from | Commit |
| --- | --- | --- |
| `actions/checkout` | lightweight tag `v4` | `11d5960a326750d5838078e36cf38b85af677262` |
| `Swatinem/rust-cache` | annotated tag `v2` (2.9.3), peeled | `d1527c8f67ef9af357290475d4af4b0fab8f12e0` |
| `dtolnay/rust-toolchain` | branch `1.99.1` | `d124f86f28ff42855784ff3319f48ac182c248f8` |

The rust-toolchain commit's `action.yml` hardcodes toolchain `1.99.1`, so the pin keeps that compiler. The tag and branch names were not left as floating references.

No GitHub Actions run was observed. There is no run URL. Remote CI is pending until this commit is pushed.

## Acceptance criteria

| Criterion | Status |
| --- | --- |
| AC-A01 Format, Clippy, compile | Locally verified. Exit 0. |
| AC-A02 Unit tests | Locally verified. 41 passed. |
| AC-A03 Explicit migration command | Implemented. `trading-api migrate` does not serve HTTP. Not executed against PostgreSQL locally. |
| AC-A04 Startup and readiness do not migrate | Implemented. Local unreachable-port test returns `NotReady`, not a migration error. The no-write proof is in the CI integration test and is pending. |
| AC-A05 Fresh versus migrated readiness | Implemented. Pending CI. Not locally verified. |
| AC-A06 Incompatible schema, no repair | Implemented in the comparison unit tests and the CI integration test. Pending CI for a real database. |
| AC-A07 No credentials in HTTP or routine logs | Locally verified against an unreachable URL and redaction tests. |
| AC-A08 Signals and unexpected task exit | Graceful shutdown and unexpected completion are locally verified. Ctrl-C is the Windows signal. SIGTERM is implemented for Unix and not executed here. |
| AC-A09 Bounded shutdown and Drop fallback | Locally verified. Timeout aborts and returns `ShutdownTimeout`. Drop aborts without blocking. |
| AC-A10 HTTP helper timeout | Locally verified. A stalled response panics instead of returning partial bytes. |
| AC-A11 CI executes the PostgreSQL test | Implemented in the workflow with `--ignored`. Pending a real run. |
| AC-A12 Least privilege, `--locked`, pinned actions | Implemented. Permissions are `contents: read`. Pins were verified against GitHub on 2026-11-14. The workflow run is pending. |
| AC-A13 No trading behavior | Implemented. No order route, broker client, or trading-domain table was added. |
| AC-A14 Docs, prompt, report | Implemented in this commit. |
| AC-A15 Remote CI | Pending. Not claimed. |
| AC-A16 M0-01 AC-08 and AC-09 | AC-08 remains open. Compose was not executed. AC-09 remains pending until the GitHub Actions PostgreSQL job succeeds. A service container is not evidence that Compose works. |

## Known risks

- Schema compatibility trusts SQLx history plus one metadata row. It does not hash the live catalog.
- Migration and the API share `DATABASE_URL`. The documented local role can create objects. A restricted readiness role is not provisioned.
- API statements are limited to 1 second. The migration connection allows 30 seconds and is then closed.
- Disabling sqlx logs hides driver diagnostics from the process log on purpose. A migration failure still has one redacted detail line.
- Pinned actions will not pick up later upstream fixes until the SHAs are updated.
- The integration test's backend-termination recovery can fail if the test role cannot call `pg_terminate_backend`. The CI role is the image superuser. This was not executed locally.
- A panic in the integration test drops its private databases afterward; the opted-in database is not migrated by the test. A failed `DROP DATABASE` fails the test without printing the connection URL.
- Compose, a full PostgreSQL restart, and a live SIGTERM delivery remain unverified.

## Commit

This report is part of the commit `M0-01A: Foundation Verification & Hardening.` The hash is produced by that commit and is reported in the handoff. It is not duplicated here.

## Out of scope

M0-02 and DET-01 through DET-10 were not started. No order, risk, broker, execution, paper-trading, or live-trading behavior was added.
