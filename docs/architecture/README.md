# Architecture

M0-01 established the software foundation. M0-01A separates infrastructure migration from HTTP readiness and hardens process lifecycle. Neither milestone implements trading.

The decision records named below were binding inputs. Their full text is not in this repository, and this document does not restate or invent that text.

## Overview

One Cargo workspace builds a modular Rust monolith. `trading-api` is the composition root and the only executable. It reads configuration, starts structured logging, and serves two HTTP routes:

- `GET /health/live` — the process is running. This check does not touch PostgreSQL.
- `GET /health/ready` — PostgreSQL accepted a connection and the embedded infrastructure schema is present and compatible.

**HTTP readiness is not Trading Readiness and grants no submission authority.** Neither route authorizes an order. A future trading halt is a domain decision. It is not implemented by shutting down this process.

NATS JetStream, the Next.js frontend, and the Python research environment are outside this milestone.

## Migration and readiness

`Database::connect` builds a lazy pool. It does not open a socket and it does not migrate.

`trading-api migrate` is the explicit migration command. It applies the embedded SQLx migrations and exits. Startup, `GET /health/live`, `GET /health/ready`, and any other API request do not call it. A successful migration command is not itself API readiness: the API process still has to observe the schema.

`Database::check_ready` is read-only. It runs `SELECT 1`, reads `_sqlx_migrations`, and reads `infrastructure_metadata`. It does not run SQLx migrations and it does not execute DDL, `INSERT`, `UPDATE`, or `DELETE`. The result is not cached in the process. Each probe reads the database again, so a migration applied while the API is running can make a later probe succeed without restarting the API.

A fresh database that answers `SELECT 1` is not ready. Ready requires both of the following:

- every migration embedded in this build is recorded in `_sqlx_migrations` with `success = true` and the embedded SHA-384 checksum
- `infrastructure_metadata.schema_purpose` is `infrastructure-metadata-only`

The metadata value alone is not proof, because it can be inserted without the migration checksum. The checksum alone is not proof, because the metadata row can be removed or changed after a recorded migration. A missing history is `schema_missing`. A checksum mismatch, a failed migration row, an unknown version, or disagreeing metadata is `schema_incompatible`. Readiness does not repair any of those states. `Database::migrate` also refuses a checksum mismatch instead of rewriting it; that is SQLx's rule.

The initial migration creates `infrastructure_metadata` and inserts the purpose row. SQLx creates `_sqlx_migrations`. No order, risk, execution, or broker table is created.

Deployment order is: start PostgreSQL, run `trading-api migrate`, start `trading-api`, then request `GET /health/ready`. If the API starts first, liveness remains available and readiness stays HTTP 503 until the migration command succeeds.

The migration command needs permission to create objects in the target database. Readiness needs `CONNECT` and `SELECT` on `_sqlx_migrations` and `infrastructure_metadata`. This milestone uses one `DATABASE_URL` for both. The local development role is the PostgreSQL image superuser. A separate migration role, production credentials, and a deployment workflow are not introduced.

Readiness is bounded. Pool acquisition waits at most one second. API statements use PostgreSQL `statement_timeout` of one second. The client abandons a readiness check after two seconds. The migration command uses a thirty-second statement timeout on a connection that is closed before it can be reused, and a sixty-second client-side bound.

HTTP responses stay on the existing JSON shape. HTTP 200 means the check passed. Any failure is HTTP 503 with `postgres: unavailable`. The body does not include driver errors, schema diagnostics, or credentials. Routine logs record the classified reason and repeat it only when the result changes. Driver tracing targets are disabled.

## Process lifecycle

The process waits until Ctrl-C or, on Unix, SIGTERM. Windows builds install the Ctrl-C handler only. The HTTP task is then asked to stop and is given 10 seconds. If it stops in time, the process exits according to the task result. If the timeout elapses, the task is aborted and the process returns a shutdown-timeout error, which is a nonzero exit. An HTTP task that ends before a signal is also a nonzero exit.

`Drop` on the server handle sends the stop signal and aborts the task immediately. It does not wait. It is a best-effort fallback, not a graceful shutdown.

There is no orchestration framework and no trading-readiness state machine. M0-02A defines `TradingReadinessState` as a label. It does not decide readiness and it does not read HTTP status.

## Module responsibilities

| Crate | Responsibility now | Explicitly absent |
| --- | --- | --- |
| `trading-common` | `SecretString`, `HttpRequestId`, and `error_chain` | General-purpose utility collection. `HttpRequestId` is not a domain correlation id |
| `trading-domain` | Draft identifiers, state labels, decimal shells, timestamps, and the M0-02A.2 type adjustments | State transitions, risk evaluation, broker dispatch, applied rounding, verified broker identity, DET transition tables, and canonical wire schemas |
| `trading-application` | `Application` holding the deployment environment | Order submission workflows |
| `trading-oms` | Reserved package | Order processing |
| `trading-risk` | Reserved package | Risk policies and reservations |
| `trading-broker` | Reserved package | Broker-neutral submission interface |
| `trading-ibkr` | Reserved package | Interactive Brokers connection and credentials |
| `trading-persistence` | Lazy pool, explicit infrastructure migration, read-only schema check | OMS, risk, execution, and broker-identity tables |
| `trading-api` | Process, configuration, logging, health routes, migration command | Trading commands |

`trading-readiness`, `trading-execution`, `trading-identity`, and `trading-audit` are later candidates. They are not crates yet.

`APP_ENV` is the deployment environment (`local`, `test`, `production`). It is not a trading mode. `paper` and `live` are invalid values. `production` does not turn on live trading.

## Dependency rules

Workspace edges, from the composition root outward:

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

trading-domain
├── serde
├── thiserror
├── trading-common
└── uuid

trading-application
trading-oms
trading-risk
trading-broker
trading-ibkr
```

`trading-domain` depends on `serde`, `thiserror`, `uuid`, and `trading-common`. The `trading-common` edge names `HttpRequestId` in the non-interchangeability doctest. `trading-common` has no HTTP server and no database. Domain code does not open a socket. The other five crates have no workspace dependencies. `trading-api` is the composition root. The graph has no cycles. `trading-domain` does not depend on `trading-api` or `trading-persistence`.

Domain and application crates do not depend on axum, sqlx, NATS, Interactive Brokers libraries, or frontend types. `trading-persistence` does not depend on domain types. `trading-ibkr` does not depend on `trading-broker` because that crate does not yet define a contract.

No port trait was added for a future database, broker, or order workflow. The API constructs the concrete PostgreSQL pool directly.

## Approved references

These identifiers were supplied as binding architecture decisions. Their original bodies were not provided here. TRD-ARCH-SYNC-01 added draft contracts and an ADFR extract under `contracts/sources/`. That extract is not a review of ARCH-01A, ARCH-01B, or ARCH-01D, and it is not approval of ARCH-01C:

- REV-10: Modular Rust monolith.
- REV-11: PostgreSQL-based operational persistence.
- REV-12: Docker Compose and hybrid infrastructure.
- REV-17: Domain-oriented architecture and incremental development.
- REV-18: Rust workspace structure and safety-critical testing.
- REV-19: Paper-trading-first MVP.
- REV-20: Cross-contract consistency and safety invariants.
- ARCH-01A: Consolidated system architecture.
- ARCH-01B: Domain and transaction architecture.
- ARCH-01C: Architecture Decision and Foundations Register.
- ARCH-01D: Traceability and implementation readiness.

## Outstanding technical contracts

DET-01 through DET-10, CR-B01 through CR-B09, and a consolidated Architecture Decision and Foundations Register extract are present under `contracts/sources/` as of TRD-ARCH-SYNC-01. The index is [contracts/traceability-index.md](../../contracts/traceability-index.md). Source-text review stays `NOT_REVIEWED`. DET-01 and DET-02 remain reconstructed drafts (`RECONSTRUCTED_DRAFT`, `SOURCE_REVIEW_PENDING`), not the original Q&A transcripts. On 2026-11-15, ARCH-DEC-01 ratified those two drafts as `DESIGN_BASELINE_RATIFIED_WITH_EXCEPTIONS`. The exceptions include every detail those files mark open, undefined, unconfirmed, or verification-required, including the `OrderClosure` reason vocabulary and the transition matrix. ARCH-DEC-02 reconciled the register to 78 historical IDs, with exactly one superseded ID (`ADFR-CROSS-005`, succeeded by `ADFR-CR-B10-001`) and 77 not superseded. That 77 is a count, not implementation or verification. ARCH-DEC-03 gives CR-B01 design precedence over conflicting or less restrictive DET-07 dispatch text. It does not implement fencing and it does not demonstrate IBKR behavior. The decision record is [ARCH-DEC-01-bis-03-2026-11-15.md](../../contracts/decisions/ARCH-DEC-01-bis-03-2026-11-15.md). The comparison in [TRD-ARCH-SYNC-01-report.md](TRD-ARCH-SYNC-01-report.md) and the delta in [TRD-ARCH-SYNC-02-report.md](TRD-ARCH-SYNC-02-report.md) do not authorize trading. Earlier statements in this file that those bodies were absent describe the repository before TRD-ARCH-SYNC-01.

The contract status vocabulary is [contract-status-model-v1.0.md](../../contracts/status-model/contract-status-model-v1.0.md). It separates provenance, lifecycle, implementation, verification, authorization, and severity. Unassigned enums are `PROPOSED_NOT_RATIFIED`. The record is [TRD-ARCH-03A-report.md](TRD-ARCH-03A-report.md). On 2026-11-15, ARCH-DEC-04 through ARCH-DEC-08 ratified interpretation constraints for SMQ-01 through SMQ-05, each Alternative 1. Those decisions do not choose a status-word winner, do not ratify a closed lifecycle enum, do not promote ADFR verification, and do not relabel source provenance. The XC-11 status-word disagreement stays OPEN. The record is [TRD-ARCH-03A.2-report.md](TRD-ARCH-03A.2-report.md) and [ARCH-DEC-04-bis-08-2026-11-15.md](../../contracts/decisions/ARCH-DEC-04-bis-08-2026-11-15.md).

TRD-ARCH-03B.1 adds an OMS review package at [contracts/design-proposals/README.md](../../contracts/design-proposals/README.md). As written, that package is `NOT_RATIFIED` and OMSQ-01 through OMSQ-12 are `USER_DECISION_PENDING`. `XC-03-MATRIX` stays OPEN. No OMS transition and no `OrderClosure` vocabulary are approved. The record is [TRD-ARCH-03B.1-report.md](TRD-ARCH-03B.1-report.md).

TRD-ARCH-03B.1.2 records design-policy choices for those twelve questions as ARCH-DEC-09 through ARCH-DEC-20, options A, C, A, A, B, A, A, A, A, A, A, A. The record is [ARCH-DEC-09-bis-20-2026-11-15.md](../../contracts/decisions/ARCH-DEC-09-bis-20-2026-11-15.md) and [TRD-ARCH-03B.1.2-report.md](TRD-ARCH-03B.1.2-report.md). The choices do not approve an `OrderClosure` enum, a transition edge, a matrix version, an economic-finality predicate, automatic economic posting, risk release, broker terminal acceptance, or modify and replace dispatch.

TRD-ARCH-03B.2 adds a risk review package on the same proposals index. As written, that package is `NOT_RATIFIED` and RISKQ-01 through RISKQ-14 are `USER_DECISION_PENDING`. `XC-02-PRED` stays OPEN. No economic-finality predicate, no economic posting, and no reservation release are approved. ARCH-03B.3 is not started. The record is [TRD-ARCH-03B.2-report.md](TRD-ARCH-03B.2-report.md).

TRD-ARCH-03B.2.2 records design-policy choices for those fourteen questions as ARCH-DEC-21 through ARCH-DEC-34, options B, A, A, A, A, A, A, A, A, B, B, A, A, B. The record is [ARCH-DEC-21-bis-34-2026-11-15.md](../../contracts/decisions/ARCH-DEC-21-bis-34-2026-11-15.md) and [TRD-ARCH-03B.2.2-report.md](TRD-ARCH-03B.2.2-report.md). The choices are design policy only. They do not approve arithmetic, a reservation writer, reservation release, economic posting, an economic-finality predicate, broker dispatch, or an OMS transition. `XC-02-PRED` stays OPEN. ARCH-03B.3 is not started.

CR-B10 v1.0-draft is copied at `contracts/sources/CR-B10-Domain-Glossary-and-Schema-Registry-v1.0.md`. Its enum names stay draft. The architecture resolution for UD-01 through UD-09 is copied beside it. That resolution is DESIGN_RESOLVED / IMPLEMENTATION_PENDING. It refines provisional CR-B10 details and does not replace a DET invariant. Broker uniqueness stays EVIDENCE_PENDING.

M0-02A adds the draft vocabulary in `trading-domain` and `contracts/domain/glossary.md`. It does not invent transition tables, and it does not implement order, risk, execution, or broker behavior. M0-02B-a adds a registry skeleton that cannot publish canonical schemas. M0-02A.2 locally tested a subset of the resolution's type adjustments. M0-02B-b, the canonical schemas and fixtures, stays NO-GO. The DET and CR bodies are now present and not reviewed. Canonical wire gates are not implemented. Safety-critical trading behavior stays absent. Live trading is not authorized.
