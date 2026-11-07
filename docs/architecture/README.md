# Architecture

M0-01 is the software foundation for a private trading platform. It establishes crate boundaries, configuration, logging, a health process, and PostgreSQL connectivity. It does not implement trading.

The decision records named below were binding inputs. Their full text is not in this repository, and this document does not restate or invent that text.

## Overview

One Cargo workspace builds a modular Rust monolith. `trading-api` is the composition root and the only executable. It reads configuration, starts structured logging, attempts a PostgreSQL readiness check, and serves two HTTP routes:

- `GET /health/live` — the process is running.
- `GET /health/ready` — PostgreSQL accepted a connection and the infrastructure migration is applied.

Application readiness is not trading readiness. Neither route authorizes an order.

NATS JetStream, the Next.js frontend, and the Python research environment are outside this milestone.

## Module responsibilities

| Crate | Responsibility now | Explicitly absent |
| --- | --- | --- |
| `trading-common` | `SecretString`, `CorrelationId`, and `error_chain` | General-purpose utility collection |
| `trading-domain` | Boundary for future types and invariants | Order state machine and DET implementations |
| `trading-application` | `Application` holding the deployment environment | Order submission workflows |
| `trading-oms` | Reserved package | Order processing |
| `trading-risk` | Reserved package | Risk policies and reservations |
| `trading-broker` | Reserved package | Broker-neutral submission interface |
| `trading-ibkr` | Reserved package | Interactive Brokers connection and credentials |
| `trading-persistence` | Lazy pool, embedded infrastructure migration, connectivity check | OMS, risk, execution, and broker-identity tables |
| `trading-api` | Process, configuration, logging, health routes | Trading commands |

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
trading-application
trading-oms
trading-risk
trading-broker
trading-ibkr
```

The last six crates have no workspace dependencies. `trading-api` is the only crate that depends on another workspace crate. The graph has no cycles.

Domain and application crates do not depend on axum, sqlx, NATS, Interactive Brokers libraries, or frontend types. `trading-persistence` does not depend on domain types. `trading-ibkr` does not depend on `trading-broker` because that crate does not yet define a contract.

No port trait was added for a future database, broker, or order workflow. The API constructs the concrete PostgreSQL pool directly.

`/health/ready` can apply the infrastructure migration the first time PostgreSQL is reachable. That side effect is limited to `infrastructure_metadata` and the sqlx migration table. It is not a business schema.

## Approved references

These identifiers were supplied as binding architecture decisions. Their bodies were not provided here:

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

DET-01 through DET-10 are conceptually approved and still need implementation-level specification. This milestone does not invent those contracts, and it does not implement order, risk, execution, or broker behavior against them.
