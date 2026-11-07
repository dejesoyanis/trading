# M0-01: Rust Trading Platform Foundation

## 1. Context

We are developing a private, safety-critical, AI-assisted trading platform.

The long-term architecture consists of:

- A modular Rust monolith for the authoritative Trading Core.
- PostgreSQL for operational persistence.
- NATS JetStream for asynchronous integration.
- A broker-agnostic integration layer, initially targeting Interactive Brokers.
- A deterministic Risk Engine.
- A multidimensional Order Management System.
- A Trading Readiness Controller.
- A controlled Submission Authority and single-writer execution boundary.
- A Next.js frontend.
- A separate Python-based research and machine-learning environment.

The initial trading milestone will support manual **paper trading only**, using stocks, ETFs, market orders and limit orders.

The current task is strictly limited to establishing the software foundation.

**Do not implement trading functionality in this milestone.**

### Architecture references

The following architecture decisions are binding:

- REV-10: Modular Rust monolith.
- REV-11: PostgreSQL-based operational persistence.
- REV-12: Docker Compose and hybrid infrastructure.
- REV-17: Domain-oriented architecture and incremental development.
- REV-18: Rust workspace structure and safety-critical testing.
- REV-19: Paper-trading-first MVP.
- REV-20: Cross-contract consistency and safety invariants.
- ARCH-01A: Consolidated system architecture.
- ARCH-01B: Domain and transaction architecture.
- ARCH-01C: Architecture Decision & Foundations Register.
- ARCH-01D: Traceability and implementation readiness.

The following contracts have been conceptually approved but still require implementation-level specification:

- DET-01 through DET-10.

Do not invent missing safety-critical contracts.

---

## 2. Objective

Create a clean, maintainable, production-oriented Rust workspace that can serve as the foundation of the future trading platform.

The implementation must:

1. Compile successfully.
2. Provide clearly separated architectural modules.
3. Support automated testing.
4. Provide a local PostgreSQL development environment.
5. Establish structured configuration and observability.
6. Prepare the repository for CI.
7. Document the architecture and development workflow.
8. Avoid premature implementation of trading logic.

Prioritize simplicity, correctness, maintainability and explicit boundaries.

Do not introduce unnecessary abstractions or dependencies.

---

## 3. Implementation Strategy

Follow these phases in order.

### Phase 1 – Repository Inspection

Before modifying files:

1. Inspect the existing repository.
2. Identify existing source code, configuration and documentation.
3. Check whether a Cargo workspace already exists.
4. Inspect existing Git configuration and CI files.
5. Identify established naming conventions.

Do not overwrite existing files without understanding their purpose.

If the repository already contains useful infrastructure, extend it instead of recreating it.

Produce a concise implementation plan before making changes.

### Phase 2 – Rust Workspace

Create a Cargo workspace with the following initial structure:

```text
trading-platform/
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
├── .gitignore
├── .editorconfig
├── .env.example
│
├── apps/
│   └── trading-api/
│       ├── Cargo.toml
│       └── src/
│           └── main.rs
│
├── crates/
│   ├── trading-common/
│   ├── trading-domain/
│   ├── trading-application/
│   ├── trading-oms/
│   ├── trading-risk/
│   ├── trading-broker/
│   ├── trading-ibkr/
│   └── trading-persistence/
│
├── infrastructure/
│   └── docker/
│       └── compose.yaml
│
├── tests/
│
└── docs/
    ├── architecture/
    ├── development/
    └── cursor-prompts/
```

Additional modules such as `trading-readiness`, `trading-execution`, `trading-identity` and `trading-audit` are planned architectural candidates.

Do not create them merely to increase the number of crates.

#### Requirements

- Use Rust edition 2024 if supported by the selected stable toolchain.
- Use a stable, explicitly pinned Rust toolchain.
- Use Cargo workspace dependency management where appropriate.
- Establish consistent package naming.
- Avoid circular dependencies.
- Keep the dependency graph minimal.
- All crates must compile.

The exact toolchain version should be selected based on the installed environment and documented.

### Phase 3 – Module Responsibilities

Establish the following boundaries.

**trading-common**

Shared technical primitives that do not belong to a specific business domain.

Do not turn this crate into an unrestricted collection of utilities.

**trading-domain**

Foundation for future domain types and invariants.

Do not implement the Order State Machine yet.

**trading-application**

Foundation for application-level orchestration.

Do not implement order submission workflows.

**trading-oms**

Reserved for the future Order Management System.

No order processing implementation.

**trading-risk**

Reserved for the deterministic Risk Engine.

No provisional risk policies or risk reservations.

**trading-broker**

Reserved for broker-neutral interfaces and types.

No order submission interface that can be invoked operationally yet.

**trading-ibkr**

Reserved for the future Interactive Brokers adapter.

Do not connect to IBKR.

Do not introduce broker credentials.

**trading-persistence**

Foundation for PostgreSQL integration.

Do not create business-domain tables yet.

**trading-api**

Application entry point.

It may expose a minimal HTTP health endpoint.

It must not expose trading commands.

### Dependency Rules

Prefer a dependency direction in which infrastructure depends on abstractions rather than business logic depending on infrastructure.

Avoid introducing direct dependencies from domain crates to:

- HTTP frameworks
- PostgreSQL drivers
- NATS
- IBKR libraries
- frontend-specific types

Document the resulting crate dependency graph.

Do not add traits solely because an abstraction might be useful later.

---

## 4. Configuration

Implement a minimal configuration system.

Requirements:

- Support environment variables.
- Provide documented defaults for non-sensitive local development settings.
- Fail clearly on invalid configuration.
- Do not hardcode credentials.
- Do not commit secrets.
- Provide `.env.example`.
- Ensure sensitive configuration values are not printed in logs.

Use a simple, established configuration approach.

Avoid building a custom configuration framework.

At minimum, prepare configuration for:

- Application environment.
- HTTP bind address.
- Logging level.
- PostgreSQL connection.

Paper/live configuration must not imply that live trading is supported.

If an environment identifier is introduced, document its meaning and ensure that no live-trading capability is activated.

---

## 5. PostgreSQL Development Infrastructure

Create a local PostgreSQL development setup using Docker Compose.

Requirements:

- Use a version-pinned PostgreSQL image.
- Configure a persistent development volume.
- Provide a health check.
- Use environment variables for local credentials.
- Document startup and shutdown commands.
- Keep the database inaccessible from public networks.
- Prefer binding published development ports to localhost.
- Do not introduce production credentials.

Provide a minimal connectivity test.

The test must verify that the application can establish a PostgreSQL connection.

Do not create OMS, Risk, Execution or Broker Identity schemas.

### Migration Foundation

Prepare a standard database migration mechanism if it can be introduced without unnecessary complexity.

A minimal initial migration may establish infrastructure metadata.

Do not create speculative business tables.

---

## 6. Observability

Introduce structured logging using established Rust libraries.

Recommended candidates:

- `tracing`
- `tracing-subscriber`

Requirements:

- Structured application logs.
- Configurable log levels.
- Clear startup and shutdown messages.
- Useful error context.
- No sensitive information in logs.

Prepare correlation identifiers where appropriate, but do not build a complete distributed tracing infrastructure.

### Health Endpoint

Provide a minimal HTTP endpoint:

`GET /health/live`

It should report whether the application process is running.

Optionally provide:

`GET /health/ready`

If implemented, readiness must accurately reflect only the dependencies that are actually checked.

**Application readiness is not Trading Readiness.**

Do not reuse the health endpoint as authorization to submit orders.

---

## 7. Error Handling

Establish consistent error-handling conventions.

Use:

- `Result<T, E>` for recoverable errors.
- Explicit error types where they provide value.
- Contextual error propagation.
- Clear separation between application errors and infrastructure errors.

Avoid:

- Unnecessary panics.
- Unchecked `unwrap()` calls in operational code.
- Silent error suppression.
- Excessive generic error wrappers.

Do not design the complete future trading error hierarchy in this milestone.

---

## 8. Testing Infrastructure

Establish automated testing.

At minimum:

1. Workspace compilation test.
2. Unit tests for implemented foundational functionality.
3. Configuration validation tests.
4. Application startup test.
5. HTTP health endpoint test.
6. PostgreSQL connectivity integration test.

Database-dependent tests must be clearly separated from tests that run without Docker.

If practical, use Testcontainers or an equivalent reproducible approach.

Do not make ordinary unit tests depend on a running local database.

### Required Commands

The following commands must succeed:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace
```

Document any additional commands required for database integration tests.

---

## 9. CI Foundation

Create a basic CI pipeline appropriate for the repository's existing Git hosting platform.

If no CI platform is identifiable, document the recommended setup rather than assuming a provider.

The pipeline should include:

- Formatting validation.
- Clippy.
- Workspace build.
- Unit tests.
- Dependency caching where appropriate.

Do not introduce deployment automation.

Do not configure access to production infrastructure.

Do not add live-trading credentials.

---

## 10. Documentation

Create or update:

### README.md

Include:

- Project purpose.
- High-level architecture.
- Prerequisites.
- Local development setup.
- Build instructions.
- Test instructions.
- PostgreSQL startup.
- Current implementation status.
- Explicit statement that trading functionality is not yet implemented.

### docs/architecture/README.md

Include:

- Architectural overview.
- Module responsibilities.
- Dependency rules.
- References to the approved architecture decisions.
- Outstanding technical contracts.

Do not fabricate detailed ADR contents that have not been provided.

### docs/development/README.md

Include:

- Development workflow.
- Rust toolchain.
- Formatting and linting.
- Testing.
- Docker Compose.
- Configuration.

### docs/cursor-prompts/

Save this development prompt as:

`M0-01-rust-trading-platform-foundation.md`

Preserve the complete prompt content.

---

## 11. Security Requirements

This milestone must not contain:

- Real broker connections.
- Order submission functionality.
- Live-trading functionality.
- Automated trading strategies.
- AI trading agents.
- Hardcoded credentials.
- Publicly accessible databases.
- Production deployment automation.
- Placeholder security checks that always approve requests.

Do not implement mock trading authorization and present it as a functioning safety mechanism.

Future safety-critical components must be implemented against their approved technical contracts.

---

## 12. Acceptance Criteria

The milestone is complete only when:

**AC-01:** The Cargo workspace is valid.

**AC-02:** All implemented crates compile successfully.

**AC-03:** The crate dependency graph contains no circular dependencies.

**AC-04:** The application starts successfully.

**AC-05:** The health endpoint responds correctly.

**AC-06:** Configuration validation rejects invalid values.

**AC-07:** Sensitive configuration is not exposed in logs.

**AC-08:** PostgreSQL starts successfully through Docker Compose.

**AC-09:** Database connectivity can be verified.

**AC-10:** Unit tests pass.

**AC-11:** Formatting and Clippy checks pass.

**AC-12:** CI configuration is present or its absence is explicitly justified.

**AC-13:** Development documentation is complete.

**AC-14:** No broker execution or trading functionality has been introduced.

**AC-15:** The implementation report identifies all remaining limitations.

If an acceptance criterion cannot be met, report it explicitly.

Do not claim successful verification without executing the corresponding check.

---

## 13. Implementation Report

After implementation, create:

`docs/development/reports/M0-01-implementation-report.md`

The report must contain:

### 13.1 Summary

Describe the implemented foundation.

### 13.2 Architecture

Explain the crate structure and dependency relationships.

### 13.3 Technical Decisions

Document:

- Rust version.
- Main dependencies.
- Configuration approach.
- HTTP framework, if used.
- PostgreSQL integration.
- Testing approach.

Explain important trade-offs.

### 13.4 Verification Results

Provide the actual results of:

- Cargo build.
- Cargo test.
- Cargo fmt.
- Cargo clippy.
- PostgreSQL integration tests.

Distinguish between executed, skipped and unavailable checks.

### 13.5 Deviations

Document any deviations from this prompt.

### 13.6 Known Limitations

List incomplete functionality and technical limitations.

### 13.7 Next Steps

Identify suitable follow-up tasks without implementing them.

---

## 14. Git Commit

After implementation and verification:

1. Review the complete Git diff.
2. Ensure no credentials or generated build artifacts are staged.
3. Stage only files belonging to this milestone.
4. Create a Git commit with the exact message:

`M0-01: Rust Trading Platform Foundation.`

If mandatory verification fails, do not claim successful completion. Fix the issue where possible; otherwise document the blocker and do not create a success commit.

Do not push automatically.

---

## 15. Final Response

Provide a concise but technically meaningful summary containing:

1. Implemented components.
2. Created workspace structure.
3. Important architectural decisions.
4. Executed tests and results.
5. Documentation files.
6. Git commit hash.
7. Remaining limitations.
8. Recommended next development milestone.

Do not begin M0-02 or any trading-related implementation automatically.

**Implement M0-01 only.**
