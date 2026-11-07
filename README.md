# Trading Platform

Private foundation for a safety-critical, AI-assisted trading platform.

**Trading functionality is not implemented.** This repository is the M0-01 software foundation. It does not submit orders, connect to a broker, run strategies, authorize trades, or enable live trading.

## Purpose

The long-term platform is a modular Rust monolith for the authoritative trading core, with PostgreSQL for operational persistence and, later, NATS JetStream for asynchronous integration. The first product milestone is manual paper trading of stocks and ETFs with market and limit orders. That behavior is intentionally absent until its implementation-level contracts exist.

## Architecture

The Cargo workspace is the git repository root. `trading-api` is the only process. It serves health endpoints and can check PostgreSQL. Domain, application, OMS, risk, and broker crates mark their boundaries and do not implement trading.

```text
apps/trading-api            HTTP process and composition root
crates/trading-common       redacted secrets, correlation ids, error chains
crates/trading-domain       future domain types; no order state machine
crates/trading-application  deployment-environment context; no order workflows
crates/trading-oms          reserved; no order processing
crates/trading-risk         reserved; no risk policies
crates/trading-broker       reserved; no submission interface
crates/trading-ibkr         reserved; no broker connection and no credentials
crates/trading-persistence  PostgreSQL connectivity and infrastructure metadata
```

Infrastructure crates are composed by `trading-api`. Domain crates do not depend on HTTP, PostgreSQL, NATS, or Interactive Brokers. Details are in [docs/architecture/README.md](docs/architecture/README.md).

## Prerequisites

- Rust 1.99.1, with rustfmt and clippy. `rust-toolchain.toml` pins the toolchain and rustup installs it on the first Cargo command.
- Docker, only for the local PostgreSQL database. The Rust unit tests do not need it.

## Local development

From the repository root:

```powershell
Copy-Item .env.example .env
```

```sh
cp .env.example .env
```

`.env` is gitignored. The values in `.env.example` are local development defaults, including a development database password. Do not put production or broker credentials in either file.

Then:

```sh
cargo run -p trading-api
```

The process listens on `127.0.0.1:8080` by default. `GET /health/live` reports that the process is running. `GET /health/ready` reports whether PostgreSQL accepted a connection. Readiness is not permission to trade.

Stop the process with Ctrl-C.

## Build

```sh
cargo build --workspace
```

## Test

Unit tests do not need a database:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace
```

`cargo test --workspace` skips the PostgreSQL integration test. Run that test only after the database is up; the command is in [docs/development/README.md](docs/development/README.md).

## PostgreSQL

Start and stop the local database from the repository root:

```sh
docker compose -f infrastructure/docker/compose.yaml up -d
docker compose -f infrastructure/docker/compose.yaml down
```

The published port is bound to `127.0.0.1` only. The volume `trading_postgres_data` keeps data between restarts. `docker compose -f infrastructure/docker/compose.yaml down -v` deletes that volume.

The image is pinned to `postgres:18.6`. Credentials default to the local development role `trading` / `trading` and the database `trading`. This role is the database superuser created by the image. It is not a production role.

## Configuration

| Variable | Default | Meaning |
| --- | --- | --- |
| `APP_ENV` | `local` | Deployment environment: `local`, `test`, or `production`. Not a trading mode. |
| `HTTP_BIND_ADDRESS` | `127.0.0.1:8080` | IP and port. A hostname such as `localhost` is rejected. |
| `LOG_LEVEL` | `info` | `error`, `warn`, `info`, `debug`, or `trace`. |
| `LOG_FORMAT` | `text` | `text` or `json`. |
| `DATABASE_URL` | none | Required PostgreSQL URL. There is no compiled-in default. |

`APP_ENV=paper` and `APP_ENV=live` are rejected. `production` does not enable live trading. Invalid configuration stops the process before it serves traffic, and error text does not include the database URL. Logs record the database user, host, port, and database name, and omit the password.

## Current implementation status

| Area | Status |
| --- | --- |
| Workspace, configuration, logging, health endpoints | Implemented |
| PostgreSQL connectivity and infrastructure metadata | Implemented |
| Order management, risk, broker, execution, trading readiness | Not implemented |
| Paper trading and live trading | Not implemented |

## Documentation

- [Architecture](docs/architecture/README.md)
- [Development](docs/development/README.md)
- [M0-01 implementation report](docs/development/reports/M0-01-implementation-report.md)
- [M0-01 prompt](docs/cursor-prompts/M0-01-rust-trading-platform-foundation.md)
