# Development

## Workflow

Work on the git repository root. The workspace manifest is `Cargo.toml`. Create a branch for each change, keep trading behavior out until its contract is specified, and run the checks below before committing.

Do not add broker credentials, production database passwords, or live-trading switches. `APP_ENV` selects where the process runs. It does not select a trading mode.

Normal Rust development does not need Docker, WSL, or a local PostgreSQL installation.

## Rust toolchain

`rust-toolchain.toml` pins Rust **1.99.1** and installs rustfmt and clippy. The edition is **2024**. rustup installs that toolchain on the first Cargo invocation if it is missing.

The same version is selected in `.github/workflows/ci.yml` by pinning `dtolnay/rust-toolchain` to the commit whose `action.yml` hardcodes toolchain 1.99.1. Change the toolchain file and the workflow together.

## Formatting and linting

```sh
cargo fmt --all
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
```

`rustfmt.toml` uses edition 2024 and Unix newlines. Workspace lints forbid `unsafe` and warn on missing documentation for public items. Clippy treats warnings as errors in CI. Clippy, tests, and builds use the committed `Cargo.lock` through `--locked`.

## Local tests

These commands do not need a database and do not execute ignored tests:

```sh
cargo test --workspace --locked
cargo build --workspace --locked
```

They cover configuration validation, secret redaction, command parsing, startup, graceful shutdown, shutdown timeout, unexpected server-task completion, `GET /health/live`, `GET /health/ready` when PostgreSQL is unreachable, and the absence of an `/orders` route. The unreachable-database cases connect to `127.0.0.1:1` and fail the test if that port accepts a connection.

`cargo test --workspace` reports the PostgreSQL integration test as ignored. That is expected on a machine without the opt-in database. Do not describe that result as a passed database test.

## PostgreSQL integration tests

The integration test is `crates/trading-persistence/tests/connectivity.rs`. It is ignored until all of the following are true:

- the test command passes `--ignored`
- `TRADING_ALLOW_DATABASE_INTEGRATION_TESTS` is exactly `1`
- `TRADING_TEST_DATABASE_URL` is set
- the database name in that URL contains `_test`

`DATABASE_URL` is ignored, including a developer or production URL that happens to be exported in the shell. The test creates uniquely named databases on that server, runs the destructive schema checks there, and drops them. It does not migrate the opted-in database itself.

GitHub Actions runs this command against a `postgres:18.6` service whose database is `trading_test`, with the development role `trading` / `trading`:

```sh
cargo test -p trading-persistence --test connectivity --locked -- --ignored --nocapture
```

The workflow sets:

```text
TRADING_ALLOW_DATABASE_INTEGRATION_TESTS=1
TRADING_TEST_DATABASE_URL=postgres://trading:trading@127.0.0.1:5432/trading_test
```

The service has a `pg_isready` health check. The job has not been observed on GitHub until this commit is pushed and the workflow completes.

A developer with PostgreSQL can run the same command locally. Docker is one way to provide that database. It is not required to compile or to run the unit tests, and Compose startup is not claimed as verified.

The test checks a fresh database, the explicit `Database::migrate` operation, read-only readiness, idempotent migration, tampered schema without repair, and recovery after other backends of the isolated database are terminated. It does not restart the PostgreSQL process. Restart recovery remains deferred.

## Commands

```sh
cargo run -p trading-api -- migrate
cargo run -p trading-api
```

`migrate` applies `0001_infrastructure_metadata.sql` and exits. Starting `trading-api` does not migrate. `GET /health/live` stays available when PostgreSQL is down or the schema is missing. `GET /health/ready` is HTTP 200 only when connectivity and the infrastructure schema check both succeed. After a migration applied while the API is already running, the next readiness request can succeed without restarting the process.

The migration command executes DDL and needs permission to create objects. Readiness only executes `SELECT`. This milestone uses one `DATABASE_URL` for both. The local Compose role is the image superuser. It is a development credential, not a production role, and this repository does not provision a separate migration role.

## Docker Compose

Run these commands from the repository root so Compose loads `.env` from that directory:

```sh
docker compose -f infrastructure/docker/compose.yaml up -d
docker compose -f infrastructure/docker/compose.yaml ps
docker compose -f infrastructure/docker/compose.yaml down
```

`down` keeps the named volume. `down -v` removes it.

The service publishes `127.0.0.1:${POSTGRES_PORT:-5432}` only. It is not reachable from other machines through that binding. The image tag is `postgres:18.6`. `POSTGRES_USER`, `POSTGRES_PASSWORD`, and `POSTGRES_DB` default to `trading`. Those defaults match `.env.example` and are development credentials, not production credentials.

Keep `DATABASE_URL` aligned with those values. The application reads `DATABASE_URL` and does not read `POSTGRES_PASSWORD` itself. Compose was not executed during M0-01A.

## Configuration

Copy the example file once:

```powershell
Copy-Item .env.example .env
```

`trading-api` loads `.env` without overriding variables that are already set. A missing file is ignored. A malformed file stops startup with `failed to read .env` and does not print the file.

| Variable | Required | Default | Notes |
| --- | --- | --- | --- |
| `APP_ENV` | no | `local` | `local`, `test`, `production`. `paper` and `live` fail. |
| `HTTP_BIND_ADDRESS` | no | `127.0.0.1:8080` | Numeric IP and port. |
| `LOG_LEVEL` | no | `info` | `error`, `warn`, `info`, `debug`, `trace`. `RUST_LOG` is not read. |
| `LOG_FORMAT` | no | `text` | `text` or `json`. |
| `DATABASE_URL` | yes | none | `postgres://` or `postgresql://` with a host. |

Keys are case-insensitive. An empty value is invalid. Unknown variables are ignored.

Logs may include the database user, host, port, and database name. They must not include the password or the full URL. `SecretString` displays as `[redacted]`. Routine readiness logs record a classified reason such as `schema_missing` or `database_unreachable` and do not include driver text. sqlx tracing targets are disabled. The migration command may log a redacted diagnostic once; health probes do not.

## Process lifecycle

Ctrl-C requests graceful shutdown on every supported platform. Unix builds also treat SIGTERM as the same request. Windows builds do not install a SIGTERM handler. The server task is given 10 seconds to finish. If it does not, the task is aborted and the process exits nonzero. `Drop` for the server handle signals and aborts without waiting. It is not a graceful shutdown.

If the HTTP task stops before a shutdown signal, the process exits nonzero. That failure is process lifecycle. It is not a trading halt, and a future trading halt must not be implemented by stopping this process.
