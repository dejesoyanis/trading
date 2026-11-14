# Trading Platform

Private foundation for a safety-critical, AI-assisted trading platform.

**Trading functionality is not implemented.** This repository is the M0-01 software foundation, hardened by M0-01A, with a draft domain vocabulary added in M0-02A. It does not submit orders, connect to a broker, run strategies, authorize trades, or enable live trading.

**HTTP readiness is not Trading Readiness and grants no submission authority.**

## Purpose

The long-term platform is a modular Rust monolith for the authoritative trading core, with PostgreSQL for operational persistence and, later, NATS JetStream for asynchronous integration. The first product milestone is manual paper trading of stocks and ETFs with market and limit orders. That behavior is intentionally absent until its implementation-level contracts exist.

## Architecture

The Cargo workspace is the git repository root. `trading-api` is the only process. It serves health endpoints and can check PostgreSQL. Domain, application, OMS, risk, and broker crates mark their boundaries and do not implement trading.

```text
apps/trading-api            HTTP process, migration command, and composition root
crates/trading-common       redacted secrets, correlation ids, error chains
crates/trading-domain       draft domain vocabulary; no order state machine
crates/trading-application  deployment-environment context; no order workflows
crates/trading-oms          reserved; no order processing
crates/trading-risk         reserved; no risk policies
crates/trading-broker       reserved; no submission interface
crates/trading-ibkr         reserved; no broker connection and no credentials
crates/trading-persistence  PostgreSQL connectivity, explicit migration, read-only readiness
```

Infrastructure crates are composed by `trading-api`. Domain crates do not depend on HTTP, PostgreSQL, NATS, or Interactive Brokers. Details are in [docs/architecture/README.md](docs/architecture/README.md).

## Prerequisites

- Rust 1.99.1, with rustfmt and clippy. `rust-toolchain.toml` pins the toolchain and rustup installs it on the first Cargo command.
- Docker is not required for normal Rust development. Formatting, Clippy, unit tests, and `cargo build` do not need Docker, WSL, or a local PostgreSQL installation.
- Docker Compose remains an optional way to run a local PostgreSQL for developers who already have Docker. Compose startup has not been executed in this repository's verification record.

## Local development

From the repository root:

```powershell
Copy-Item .env.example .env
```

```sh
cp .env.example .env
```

`.env` is gitignored. The values in `.env.example` are local development defaults, including a development database password. Do not put production or broker credentials in either file.

Deployment order when a database is available:

1. Start PostgreSQL.
2. Apply infrastructure migrations explicitly. This does not start the HTTP server:

```sh
cargo run -p trading-api -- migrate
```

3. Start the API:

```sh
cargo run -p trading-api
```

4. `GET /health/ready` returns HTTP 200 only after step 2 has succeeded against that database.

The process listens on `127.0.0.1:8080` by default. `GET /health/live` reports that the process is running and does not touch PostgreSQL. `GET /health/ready` is a read-only connectivity and schema-compatibility check. It does not migrate. If the API starts before step 2, liveness stays HTTP 200 and readiness stays HTTP 503 until the migration command succeeds. The API then becomes ready without a restart.

Stop the process with Ctrl-C. On Unix the process also treats SIGTERM as a graceful shutdown. A future trading halt is not this process shutdown.

## Build

```sh
cargo build --workspace --locked
```

## Test

Unit tests do not need a database:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo build --workspace --locked
```

`cargo test --workspace` does not run the ignored PostgreSQL integration test. That test runs in GitHub Actions against a PostgreSQL service. The exact command is in [docs/development/README.md](docs/development/README.md). Do not treat a green local `cargo test --workspace` as evidence that the database tests passed.

## PostgreSQL

The optional local database, for machines that have Docker:

```sh
docker compose -f infrastructure/docker/compose.yaml up -d
docker compose -f infrastructure/docker/compose.yaml down
```

The published port is bound to `127.0.0.1` only. The volume `trading_postgres_data` keeps data between restarts. `docker compose -f infrastructure/docker/compose.yaml down -v` deletes that volume.

The image is pinned to `postgres:18.6`. Credentials default to the local development role `trading` / `trading` and the database `trading`. This role is the database superuser created by the image. It is not a production role. Compose itself was not executed during M0-01A.

`trading-api migrate` runs DDL. It needs permission to create objects in the target database. The API readiness check only reads. This milestone uses the same `DATABASE_URL` for both. A separate migration role and production credentials are not part of this repository.

## Configuration

| Variable | Default | Meaning |
| --- | --- | --- |
| `APP_ENV` | `local` | Deployment environment: `local`, `test`, or `production`. Not a trading mode. |
| `HTTP_BIND_ADDRESS` | `127.0.0.1:8080` | IP and port. A hostname such as `localhost` is rejected. |
| `LOG_LEVEL` | `info` | `error`, `warn`, `info`, `debug`, or `trace`. |
| `LOG_FORMAT` | `text` | `text` or `json`. |
| `DATABASE_URL` | none | Required PostgreSQL URL. There is no compiled-in default. |

`APP_ENV=paper` and `APP_ENV=live` are rejected. `production` does not enable live trading. Invalid configuration stops the process before it serves traffic, and error text does not include the database URL. Logs record the database user, host, port, and database name, and omit the password. Routine readiness logs use a classified reason and do not include driver text.

## Continuous integration

GitHub Actions is the CI provider. The workflow checks formatting, Clippy, locked builds, unit tests, and the real PostgreSQL integration test. It does not deploy and it does not push. Confirm a run on GitHub after pushing this commit. A workflow file is not evidence that the run succeeded.

## Current implementation status

| Area | Status |
| --- | --- |
| Workspace, configuration, logging, health endpoints | Implemented |
| Explicit infrastructure migration and read-only readiness | Implemented |
| PostgreSQL integration test in GitHub Actions | Implemented in the workflow; remote run pending |
| Docker Compose startup | Not verified |
| Canonical domain vocabulary | Draft shell. UD-01..UD-09 are design-resolved and implementation-pending. Canonical schemas are not published. |
| Order management, risk, broker, execution, trading readiness | Not implemented |
| Paper trading and live trading | Not implemented |

## Documentation

- [Architecture](docs/architecture/README.md)
- [Development](docs/development/README.md)
- [M0-01 implementation report](docs/development/reports/M0-01-implementation-report.md)
- [M0-01A implementation report](docs/development/reports/M0-01A-implementation-report.md)
- [M0-01 prompt](docs/cursor-prompts/M0-01-rust-trading-platform-foundation.md)
- [M0-01A prompt](docs/cursor-prompts/M0-01A-foundation-verification-hardening.md)
- [Domain glossary](contracts/domain/glossary.md)
- [Domain decisions](contracts/domain/unresolved-decisions.md)
- [Traceability index](contracts/traceability-index.md)
- [Schema registry skeleton](contracts/registry/README.md)
- [M0-02A implementation report](docs/reports/TRD-M002A-implementation-report.md)
- [UD-01..UD-09 resolution prompt](docs/cursor-prompts/trading/M0-02A-UD-01-bis-UD-09-Architecture-Resolution-v1.0.md)
