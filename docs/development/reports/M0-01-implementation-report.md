# M0-01 Implementation Report

## 13.1 Summary

M0-01 creates the Rust workspace for the trading platform and nothing of the trading system itself. The repository was empty before this change: a git root on `main` with remote `https://github.com/dejesoyanis/trading.git` and no commits.

The workspace compiles, validates configuration, logs without the database password, and serves `GET /health/live` and `GET /health/ready`. PostgreSQL integration is limited to a lazy connection pool, one infrastructure-metadata migration, and a connectivity test. Order management, risk, broker connectivity, execution, and trading readiness are reserved crate boundaries with no behavior.

Two acceptance checks were not executed. Docker is not installed on this machine, so PostgreSQL was not started and the database integration test was not run. Those checks are prepared and are not claimed as passed.

## 13.2 Architecture

The workspace root is the git root. Direct workspace dependencies are:

```text
trading-api
├── trading-application
├── trading-common
└── trading-persistence

trading-common
└── uuid

trading-persistence
├── sqlx
├── thiserror
├── tokio
└── url
```

`trading-domain`, `trading-application`, `trading-oms`, `trading-risk`, `trading-broker`, and `trading-ibkr` depend on no other workspace crate. `trading-api` is the only crate that depends on another workspace crate. Nothing points back at `trading-api`, so the graph has no cycle.

`trading-api` is the composition root. It owns configuration, tracing, and HTTP. `trading-persistence` owns the PostgreSQL driver. Domain and application code do not reference axum, sqlx, NATS, or Interactive Brokers. No broker, database, or order-port trait was added.

`cargo tree -p trading-api --edges normal --depth 1` shows the API edges above and no reverse edges. The same command across the other workspace packages shows the six reserved or foundation crates with no further workspace dependencies.

## 13.3 Technical Decisions

### Rust

- Toolchain: Rust 1.99.1 (`58cf8f9c3`, 2026-10-13), pinned in `rust-toolchain.toml`.
- Edition: 2024. Resolver: 3.
- `unsafe` is forbidden by a workspace lint.
- The installed stable toolchain was already 1.99.1. The toolchain file caused rustup to install the versioned toolchain `1.99.1-x86_64-pc-windows-msvc` on first use.

### Main dependencies

Locked versions used by the workspace:

| Crate | Version | Used for |
| --- | --- | --- |
| axum | 0.8.9 | HTTP routes |
| tokio | 1.54.1 | Runtime, signals, sockets |
| tracing | 0.1.44 | Log events |
| tracing-subscriber | 0.3.23 | Text and JSON subscribers |
| thiserror | 2.0.21 | `ConfigError`, `AppError`, `PersistenceError` |
| dotenvy | 0.15.7 | Optional `.env` loading |
| serde | 1.0.229 | Health JSON |
| sqlx | 0.9.0 | PostgreSQL pool and migrations |
| url | 2.5.8 | URL parsing and password redaction |
| uuid | 1.29.1 | Correlation identifiers |

sqlx features are `macros`, `migrate`, `postgres`, `runtime-tokio`, and `tls-rustls`. Default features are off, so MySQL and SQLite are not enabled. The `macros` feature exists so `sqlx::migrate!` can embed SQL. Compile-time query macros are not used.

### Configuration

Configuration is a `Config` struct parsed from environment variables. There is no configuration framework. Keys are case-insensitive. Empty values fail. Unknown keys are ignored.

`APP_ENV`, `HTTP_BIND_ADDRESS`, `LOG_LEVEL`, and `LOG_FORMAT` have local defaults. `DATABASE_URL` is required and has no compiled-in default, because the URL contains a credential. `.env.example` documents the local development URL. The process loads `.env` only when that file exists and does not override variables already set.

`APP_ENV` means the deployment environment: `local`, `test`, or `production`. `paper` and `live` are rejected. No trading mode exists, and `production` does not enable live trading.

### HTTP

axum serves two routes. `GET /health/live` returns HTTP 200 and `{"service":"trading-api","status":"ok","version":"0.1.0"}`. `GET /health/ready` returns HTTP 200 only after `SELECT 1` succeeds against PostgreSQL. Otherwise it returns HTTP 503 and `postgres: unavailable`. The body does not include the driver error. Each response carries `x-correlation-id`. That identifier is a random UUID for one request, not a distributed trace.

### PostgreSQL

`infrastructure/docker/compose.yaml` pins `postgres:18.6`, stores data in the named volume `trading_postgres_data`, health-checks with `pg_isready`, and publishes the port on `127.0.0.1` only. Development credentials default to user `trading`, password `trading`, and database `trading`. They are local defaults, not production credentials.

The pool connects lazily, uses at most five connections, and waits at most one second for a connection. The first successful readiness check applies `crates/trading-persistence/migrations/0001_infrastructure_metadata.sql`, which creates `infrastructure_metadata` and inserts `schema_purpose = infrastructure-metadata-only`. A `build.rs` prints `cargo:rerun-if-changed=migrations` so later migration files rebuild the crate.

### Errors and logging

Recoverable failures use `Result`. `ConfigError` and `AppError` are process errors. `PersistenceError` is the infrastructure error. Startup logs the deployment environment, bind address, log settings, and a database endpoint with the password and query string removed. PostgreSQL failures are rendered with `error_chain` and then `redact_connection_secrets` before tracing records them. `SecretString` debug and display output are `[redacted]`.

### Testing

Unit tests call `Config::from_pairs` instead of mutating process environment variables. Health tests bind `127.0.0.1:0` and speak HTTP/1.1 to the running server. Database-down tests use `127.0.0.1:1` and abort if that port accepts a connection, so they cannot migrate an unexpected server. The real PostgreSQL test is ignored unless `--ignored` is passed.

Testcontainers was not added. Docker Compose plus the ignored test, and the GitHub Actions Postgres service, are the reproducible path. Docker is not available here, so a Testcontainers dependency could not be justified or validated.

### Trade-offs

- Requiring `DATABASE_URL` is slightly less convenient than a compiled-in local URL. It avoids embedding a credential in Rust source.
- Readiness applies the infrastructure migration. That makes the first successful probe a write. The alternative was a probe that could report ready before the metadata migration existed.
- Reserved crates compile and state, with a const assertion, that their behavior is absent. They do not depend on each other yet, because there is no contract to implement.
- GitHub Actions are referenced by tag (`actions/checkout@v4`, `dtolnay/rust-toolchain@1.99.1`, `Swatinem/rust-cache@v2`) rather than commit SHA. The Rust compiler version is pinned. The action code is not.

## 13.4 Verification Results

Executed in this session on Windows, after the final source was in place.

| Check | Result |
| --- | --- |
| `cargo fmt --all -- --check` | Executed. Exit 0. No formatting diff. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Executed. Exit 0. |
| `cargo test --workspace` | Executed. Exit 0. 25 unit tests passed. 1 integration test ignored. Doc-tests ran and contained no tests. |
| `cargo build --workspace` | Executed. Exit 0. Dev profile finished. |
| PostgreSQL integration test | Not executed. `cargo test --workspace` reports `connects_migrates_and_reads_infrastructure_metadata ... ignored`. |
| Docker Compose startup | Not executed. `docker` is not on `PATH`, Docker Desktop is not installed, WSL is not installed, and `psql` is not installed. |

`cargo test --workspace` package results:

- `trading-api`: 9 passed
- `trading-application`: 1 passed
- `trading-broker`: 1 passed
- `trading-common`: 3 passed
- `trading-domain`: 1 passed
- `trading-ibkr`: 1 passed
- `trading-oms`: 1 passed
- `trading-persistence` library: 7 passed
- `trading-persistence` connectivity: 1 ignored
- `trading-risk`: 1 passed

The GitHub Actions workflow is present and was not run from this machine.

### Acceptance criteria

| Criterion | Status |
| --- | --- |
| AC-01 Workspace valid | Met. Cargo build and test ran the workspace. |
| AC-02 Crates compile | Met. `cargo build --workspace` exited 0. |
| AC-03 No cycles | Met. Direct dependency inspection shows a one-way graph. |
| AC-04 Application starts | Met. Startup test binds loopback and shuts down. |
| AC-05 Health endpoint | Met. `GET /health/live` returned 200. `GET /health/ready` returned 503 when PostgreSQL was unreachable. |
| AC-06 Invalid configuration rejected | Met. Config tests cover empty, missing, malformed, and trading-mode values. |
| AC-07 Secrets not logged | Met. Debug, startup, and PostgreSQL error tests reject `super-secret-password`. |
| AC-08 PostgreSQL starts through Compose | Not met. Docker is unavailable. Not claimed as passed. |
| AC-09 Database connectivity verified | Not met. The integration test was not run. Not claimed as passed. |
| AC-10 Unit tests pass | Met. 25 passed. |
| AC-11 Format and Clippy | Met. Both commands exited 0. |
| AC-12 CI | Met. `.github/workflows/ci.yml` exists for the GitHub remote. The workflow itself was not executed. |
| AC-13 Documentation | Met. Root, architecture, and development documents are present. |
| AC-14 No trading functionality | Met. No submission API, no broker client, `/orders` returns 404, and the migration test rejects business tables. |
| AC-15 Limitations reported | Met. This report lists them. |

The milestone is not fully accepted until AC-08 and AC-09 have been executed.

## 13.5 Deviations

- The workspace lives at the git root. The prompt tree named that root `trading-platform/`. Creating a nested directory would have left the git root empty. The Compose project name is `trading-platform`.
- `DATABASE_URL` has no default in code. Non-secret settings do. The local URL is documented in `.env.example` and Compose, not compiled into the binary.
- `APP_ENV` is only a deployment environment. `paper` and `live` fail validation instead of becoming unused trading modes.
- Testcontainers was not used. The ignored sqlx test and the CI Postgres service are the database path. See the testing decision above.
- `trading-readiness`, `trading-execution`, `trading-identity`, and `trading-audit` were not created.
- NATS, the frontend, and the Python research environment were not added.
- CI actions are pinned by tag, not by commit SHA.
- `LOG_FORMAT` is an extra setting beyond the required log level so text and JSON output can be selected without a second logging stack.

## 13.6 Known Limitations

- No order types, order state machine, order submission, risk checks, risk reservations, broker interface, or Interactive Brokers connection.
- DET-01 through DET-10 have no implementation-level specification and were not invented.
- Paper trading and live trading are not implemented. Nothing in the process can approve a trade.
- AC-08 and AC-09 are unverified until Docker is available.
- The local database role is the superuser created by the Postgres image. There is no least-privilege application role yet.
- `/health/ready` writes when it applies the infrastructure migration. It checks PostgreSQL only.
- Correlation identifiers are per HTTP request. There is no distributed tracing.
- Secret values are redacted in logs. They are not wiped from process memory.
- Redaction covers the full connection string and its userinfo password, including a percent-decoded password. Other query parameters are removed only when the full URL is removed.
- The pool has a fixed size of five and a one-second acquire timeout. There is no retry supervisor beyond the next readiness check.
- The CI workflow has not been executed.
- GitHub Action references are mutable tags.
- NATS, identity, audit, execution authority, and the frontend are absent.

## 13.7 Next Steps

Do not start trading behavior in the next change.

1. Install Docker, start Compose, and run the ignored connectivity test. Record the result against AC-08 and AC-09.
2. Specify the implementation-level contracts for DET-01 through DET-10, starting with the contracts required for a later paper-trading slice. Do not implement the order state machine, risk engine, or broker adapter in that specification milestone.
3. After those contracts exist, implement the first paper-only vertical slice for manual stock and ETF market and limit orders. Keep live trading unavailable.
4. Pin the GitHub Actions to commit SHAs once the workflow has run successfully.
5. Replace the local database superuser with a restricted role when the first real schema is introduced.
