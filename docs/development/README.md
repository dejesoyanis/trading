# Development

## Workflow

Work on the git repository root. The workspace manifest is `Cargo.toml`. Create a branch for each change, keep trading behavior out until its contract is specified, and run the checks below before committing.

Do not add broker credentials, production database passwords, or live-trading switches. `APP_ENV` selects where the process runs. It does not select a trading mode.

## Rust toolchain

`rust-toolchain.toml` pins Rust **1.99.1** and installs rustfmt and clippy. The edition is **2024**. rustup installs that toolchain on the first Cargo invocation if it is missing.

The same version is selected in `.github/workflows/ci.yml`. Change the toolchain file and the workflow together.

## Formatting and linting

```sh
cargo fmt --all
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

`rustfmt.toml` uses edition 2024 and Unix newlines. Workspace lints forbid `unsafe` and warn on missing documentation for public items. Clippy treats warnings as errors in CI.

## Testing

Unit tests run without Docker:

```sh
cargo test --workspace
cargo build --workspace
```

They cover configuration validation, secret redaction, startup, `GET /health/live`, `GET /health/ready` when PostgreSQL is unreachable, and the absence of an `/orders` route. The unreachable-database cases connect to `127.0.0.1:1` and fail the test if that port accepts a connection, so they do not migrate an unexpected server.

The database integration test is ignored because it needs PostgreSQL:

```sh
docker compose -f infrastructure/docker/compose.yaml up -d
$env:DATABASE_URL = "postgres://trading:trading@127.0.0.1:5432/trading"
cargo test -p trading-persistence --test connectivity -- --ignored --nocapture
```

On a POSIX shell, export `DATABASE_URL` instead of assigning `$env:DATABASE_URL`. The test applies `0001_infrastructure_metadata.sql` and reads `schema_purpose`. It does not create business tables.

CI runs the same ignored test against a `postgres:18.6` service container. That workflow has not been executed from this machine.

## Docker Compose

Run these commands from the repository root so Compose loads `.env` from that directory:

```sh
docker compose -f infrastructure/docker/compose.yaml up -d
docker compose -f infrastructure/docker/compose.yaml ps
docker compose -f infrastructure/docker/compose.yaml down
```

`down` keeps the named volume. `down -v` removes it.

The service publishes `127.0.0.1:${POSTGRES_PORT:-5432}` only. It is not reachable from other machines through that binding. The image tag is `postgres:18.6`. `POSTGRES_USER`, `POSTGRES_PASSWORD`, and `POSTGRES_DB` default to `trading`. Those defaults match `.env.example` and are development credentials, not production credentials.

Keep `DATABASE_URL` aligned with those values. The application reads `DATABASE_URL` and does not read `POSTGRES_PASSWORD` itself.

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

Logs may include the database user, host, port, and database name. They must not include the password or the full URL. `SecretString` displays as `[redacted]`. PostgreSQL errors are passed through `redact_connection_secrets` before they are logged. The HTTP readiness body says `unavailable` and does not include the driver error.
