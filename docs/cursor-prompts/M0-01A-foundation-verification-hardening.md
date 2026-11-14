# M0-01A: Foundation Verification & Hardening

## 1. Mission and authority

Implement a narrowly scoped hardening milestone for the existing private Rust trading-platform repository. M0-01 created a Cargo workspace with `trading-api`, `trading-persistence`, foundational/reserved crates, Axum health endpoints, SQLx PostgreSQL integration, Docker Compose configuration and GitHub Actions.

**Scope is strictly infrastructure and lifecycle correctness.** Do not implement trading commands, OMS behavior, risk policies, risk reservations, broker interfaces or IBKR connectivity, AI agents, order submission, paper/live trading, or deployment automation.

The developer machine has **no local Docker, WSL or PostgreSQL installation**. Do not require or install these tools, and do not represent database integration tests as locally passed. Use GitHub Actions with a PostgreSQL service for database-backed verification. Do not automatically push to GitHub.

Approved architectural intent: modular Rust monolith; PostgreSQL persistence; explicit application lifecycle; safety-critical behavior only after DET-01 through DET-10 are specified. HTTP readiness must never mean authorization to trade.

## 2. Repository inspection and implementation plan

Before edits:

1. Inspect Git status, branch, current CI workflow, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `.env.example`, `infrastructure/docker/compose.yaml`, and existing docs.
2. Inspect the actual implementations and tests in `apps/trading-api/src/{main,lib,server,health,config,error,logging}.rs`, `crates/trading-persistence/src/{lib,database,endpoint,error,redact}.rs`, and migration SQL.
3. Preserve existing names and interfaces where feasible; explain any changes.
4. Check whether `Cargo.lock` is committed and whether CI runs Cargo with `--locked`.
5. Inspect existing GitHub Actions permissions and Postgres service setup. Do not assume the workflow has run merely because its YAML exists.
6. Provide a concise plan, file list and anticipated risks before implementing.

Do not overwrite unrelated work or perform destructive Git operations. Respect any existing uncommitted user changes.

## 3. WP-01: Separate migration from readiness

### Current behavior to replace

`Database` currently owns `PgPool`, `Arc<AtomicBool> migrated`, and `Arc<Mutex<()>> migrate_lock`. `Database::check_ready()` runs `sqlx::migrate!("./migrations").run(&self.pool)` on first successful invocation, then `SELECT 1`. The HTTP `/health/ready` handler invokes `check_ready()` on every request; therefore an HTTP health probe can perform DDL.

### Required behavior

- `Database::connect(...)` constructs the pool without performing migrations.
- Provide an **explicit migration operation** that is invoked from a dedicated, documented command or executable, e.g. a `trading-db` binary or a narrowly scoped `trading-api migrate` subcommand. Choose the smallest solution consistent with the repository.
- `Database::check_ready()` is **read-only**: it must not run SQLx migrations, DDL, INSERT, UPDATE or DELETE. It checks connectivity and that the *required infrastructure schema version/state* is present and compatible. `SELECT 1` alone is insufficient.
- A fresh database with no migration must return **not ready**, even when `SELECT 1` succeeds.
- A migrated database with the expected metadata must return ready.
- A missing, corrupted, or incompatible schema state must return not ready with an appropriately classified error; no automatic repair.
- Preserve the existing `infrastructure_metadata` migration and its intent. Prefer SQLx's recorded migration history together with a narrow, documented metadata check; do not invent domain tables or an elaborate schema framework. Avoid using an arbitrary metadata value as the sole proof of schema compatibility if it can be trivially forged.
- Avoid caching schema validity indefinitely in a process-local `AtomicBool`. If caching is used for performance, justify its invalidation and failure semantics; prefer simple direct checks for this small milestone.
- Do not conflate a migration command's success with the API process's readiness. The API may still serve `/health/live` when PostgreSQL is unavailable or schema is incompatible.
- Ensure no migration occurs as a side effect of application startup, `/health/live`, `/health/ready`, or an unrelated API request.
- Document the deployment order: start PostgreSQL -> explicitly run migration -> start API -> verify readiness. If API starts first, it should report not-ready until the explicit migration succeeds, then recover without restart.
- Document whether the migration operation requires separate privileges. Do not introduce production credentials or a production deployment workflow.

### Error and security constraints

- Do not log raw `DATABASE_URL` or passwords.
- Do not require each API caller to remember to redact an arbitrary driver error string. Prefer a narrowly typed/sanitized public error representation for routine health logging, with detailed error chains only when safely classified/redacted.
- Preserve parameterized SQL queries.

## 4. WP-02: HTTP health behavior and observability

- Keep `/health/live` dependency-independent and `/health/ready` a read-only availability + schema compatibility check.
- Keep existing JSON response shapes where practical; return HTTP 200 only when the required checks pass, otherwise 503. Never expose raw driver errors or secrets in HTTP responses.
- Maintain per-request `x-correlation-id` and tracing span.
- Prevent noisy error logging on every repeated failing health probe. Prefer bounded logging, severity appropriate for health checks, or transition-based logging; do not build an unnecessarily complex state manager.
- Document explicitly: **HTTP readiness is not Trading Readiness and grants no submission authority**.
- Ensure any readiness timeout is bounded and appropriate for health checks; avoid indefinite hangs.

## 5. WP-03: Lifecycle and graceful shutdown

### Current behavior to address

`main.rs` awaits only `tokio::signal::ctrl_c()`, so unexpected termination of the Axum server task may leave the process waiting indefinitely. `RunningServer::shutdown(self)` sends a oneshot shutdown signal and awaits the JoinHandle. `Drop` sends the signal and immediately calls `task.abort()`, which is not graceful.

### Required behavior

- Detect unexpected termination of the HTTP server task while the process is running, and return a nonzero exit status for unexpected failure.
- Handle Ctrl+C and, under Unix/Linux, SIGTERM as graceful termination signals. Use conditional compilation where necessary so Windows builds remain supported.
- Provide an explicit, bounded graceful-shutdown path. Specify what happens when the timeout expires (e.g. abort remaining task and return an error).
- Clearly document that `Drop` is a best-effort fallback, not a graceful-shutdown guarantee. Do not block inside `Drop` or invent async destructors.
- Avoid double-awaiting or losing ownership of a JoinHandle. If the API changes, update tests and `main.rs` coherently.
- Keep the lifecycle implementation small; no generic orchestration framework or Trading Readiness state machine.
- Do not turn a future Trading Halt into an application-process shutdown; these are distinct concepts.

## 6. WP-04: Testing, including real PostgreSQL in CI

### Local tests (must run without Docker)

- Existing `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `cargo build --workspace` continue to pass.
- Test that liveness remains 200 with unreachable PostgreSQL.
- Test readiness returns 503 with unreachable PostgreSQL and does not leak secrets.
- Test startup/shutdown and unexpected server task completion using deterministic mechanisms, avoiding brittle port assumptions where feasible.
- Test graceful shutdown behavior and timeout behavior, with bounded test durations.
- HTTP test helper must fail on timeout/incomplete response rather than silently returning partial bytes.
- Unit-test error classification and configuration validation.

### PostgreSQL integration tests (run in GitHub Actions)

Using a real, isolated PostgreSQL service in CI:

1. Fresh database: connectivity succeeds, but readiness fails before migration.
2. Run explicit migration command/operation: migration succeeds and metadata is present.
3. Readiness now succeeds without executing migrations.
4. Repeated readiness calls do not alter migration history or schema.
5. Explicit migration is idempotent according to SQLx migration semantics.
6. Missing or incompatible schema state produces not-ready, not automatic migration/repair.
7. Readiness transitions from failure to success when an explicit migration is applied while the API remains running (or test at persistence layer if the API-level test is prohibitively complex; document scope).
8. Test database disconnect/recovery if feasible without brittle service restarts; otherwise clearly record it as a deferred test.
9. Verify the initial migration creates no trading-domain tables.

Use a unique test database or otherwise isolate destructive schema-tampering tests; never modify a shared or production database. A test must not accidentally run against a developer's unrelated `DATABASE_URL`. Require explicit opt-in and safe test connection configuration for database-dependent tests.

Do not claim local execution of PostgreSQL tests. CI test commands must **actually invoke** the integration tests (e.g. `--ignored` or dedicated test target, depending on the implementation). Do not let them silently remain ignored in CI.

## 7. WP-05: CI and supply-chain hardening

- Keep GitHub Actions as the CI provider because the repository is hosted on GitHub.
- Ensure workflow includes Rust formatting, Clippy, build and unit tests using `--locked` where applicable.
- Run real PostgreSQL integration tests against a PostgreSQL service with a health check and limited test credentials.
- Use minimum workflow permissions (e.g. `contents: read` unless more are demonstrably necessary).
- Pin third-party GitHub Actions to full, verified commit SHAs where practical. **Never invent SHAs**; if unable to verify them, leave the existing action references, record the unresolved item, and do not claim it is hardened.
- Avoid logging secrets and avoid untrusted pull-request execution with elevated credentials.
- Do not add production deployment, live credentials, or auto-push.
- Document the exact CI run URL and outcome **only if actually accessible and verified**. Otherwise mark the remote CI run as pending user push / verification.

## 8. Documentation and traceability

Update:

- Root README: explicit database migration workflow, health semantics, no local Docker prerequisite for normal Rust development, GitHub CI verification steps.
- `docs/development/README.md`: local vs CI tests and exact commands.
- `docs/architecture/README.md`: migration/readiness boundary, lifecycle decisions and outstanding DET contracts.
- Save this prompt verbatim to `docs/cursor-prompts/M0-01A-foundation-verification-hardening.md`.
- Create `docs/development/reports/M0-01A-implementation-report.md`.

Report must include: changed files, implementation decisions, dependencies added/removed, deviations, actual command results, local test counts, integration-test invocation, CI status, known risks, remaining acceptance criteria, and commit hash. Distinguish **implemented**, **locally verified**, **CI verified**, **pending**, and **blocked**. Do not claim AC-08 (Compose startup) as passed if Docker Compose itself was not executed; a GitHub Actions PostgreSQL service validates database behavior but is not proof that the Compose file works.

## 9. Acceptance criteria

- **AC-A01:** Workspace compiles, Clippy and formatting pass.
- **AC-A02:** All existing unit tests pass; new lifecycle/readiness unit tests pass.
- **AC-A03:** Migration is explicitly invokable without a health request.
- **AC-A04:** Neither startup nor readiness invokes migrations or writes schema.
- **AC-A05:** Fresh unmigrated PostgreSQL is not-ready; migrated compatible PostgreSQL is ready.
- **AC-A06:** Incompatible/missing schema is not-ready; no auto-repair.
- **AC-A07:** HTTP response bodies and routine logs do not disclose database credentials.
- **AC-A08:** Ctrl+C and Linux SIGTERM support graceful shutdown; unexpected server task completion is detected.
- **AC-A09:** Graceful shutdown has a bounded timeout and documented fallback.
- **AC-A10:** HTTP test timeouts fail explicitly.
- **AC-A11:** GitHub Actions workflow explicitly executes real PostgreSQL integration tests.
- **AC-A12:** GitHub Actions workflow uses least-privilege permissions and `--locked` builds; action pinning is either verified or explicitly deferred.
- **AC-A13:** No trading-domain functionality, broker calls, or live-trading support introduced.
- **AC-A14:** Documentation, saved prompt and implementation report are present.
- **AC-A15:** Remote CI success is verified only after a real run; otherwise remains pending.
- **AC-A16:** Original M0-01 AC-08 (Docker Compose startup) remains open unless Compose is actually tested; AC-09 (real PostgreSQL connectivity) can be closed by successful CI integration tests, with the evidence recorded.

## 10. Verification commands

Run locally (with the pinned toolchain):

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo build --workspace --locked
```

Run any additional local tests introduced by the implementation. Execute real PostgreSQL tests in GitHub Actions and include the exact command in CI. Do not assume that `cargo test --workspace` executes `#[ignore]` tests.

## 11. Commit and handoff

1. Inspect Git diff, test results and secrets before staging.
2. Stage only milestone files.
3. Commit with the exact message: `M0-01A: Foundation Verification & Hardening.`
4. Do not push automatically.
5. Present the commit hash, summary, tests and blockers.
6. Tell the user to push and check GitHub Actions; do not claim remote CI success without evidence.
7. Do not begin M0-02 or DET implementation automatically.

**Implement M0-01A only.**
