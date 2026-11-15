# Trading Platform — Verification-First Implementation Roadmap & Cursor Prompt Plan

**Version:** 1.0-draft | **Date:** 2026-11-14 | **Status:** PLAN_ONLY / IMPLEMENTATION_NOT_AUTHORIZED / LIVE_NOT_AUTHORIZED

## Preconditions

M0-01/M0-01A foundation exists by conversation history; independently verify current repository and CI before relying on its state. `CR-B10-Domain-Glossary-and-Schema-Registry-v1.0.md` is a proposed design refinement, not a completed schema implementation. The following milestones are sequential safety gates; parallel coding is permitted only for nonoverlapping interfaces behind frozen contracts. Each Cursor task must archive its full prompt in `docs/cursor-prompts/trading/`, create one scoped commit with the task ID and title, and provide a test/evidence report. **Do not execute these prompts or enable live broker calls without explicit user approval.**

## Milestones and gates

| ID | Scope | Deliverables | Mandatory acceptance |
|---|---|---|---|
| M0-02 | Architecture registry foundation | Domain glossary, newtypes, registry YAML, event/command JSON schemas, fixtures | B10-T01..T12, cargo test, schema compatibility checks |
| M0-03 | DET-01 OMS reference model | Executable transition matrix, typed commands, pure reducer, property tests | Invalid transitions rejected, immutable closure, replay |
| M0-04 | DET-02 Risk reference model | Typed risk metrics, policy evaluator, deterministic decimal calculations | Deny overrides, fail-closed, rounding and FX tests |
| M0-05 | PostgreSQL transactional core | OMS/Risk/Audit/Outbox UoW, reservations, locks, inbox | Concurrency, deadlock, crash, atomic rollback; INT-02 |
| M0-06 | Execution ledger | Scoped execution identity, corrections/busts, late fills, risk rebooking | Idempotency, replay, reconciliation, INT-05 |
| M0-07 | Broker simulator and adapter | Capability contract, raw evidence, connection FSM, deterministic fake broker | No network dependency, simulated ack loss, duplicate/reorder |
| M0-08 | Dispatch safety | Single writer, persisted attempts, epoch, submission gate | Fault injection; no blind retry, INT-03 |
| M0-09 | Reconciliation/readiness | Snapshot completeness, incidents, halt/reset workflow | Recovery and evidence completeness, INT-07/08 |
| M0-10 | IBKR paper integration | Paper-only gateway, identity mapping, adapter observations | External evidence for INT-04, negative tests, operator review |
| M0-11 | Consolidated safety review | Cross-contract matrix, threat model, runbooks, test artifacts | INT-01..08 evidence, unresolved gaps listed |

## Suggested Cursor task sequence (not execution authorization)

- `TRD-M002A: Canonical Domain Types & Glossary` — create newtypes, strict module ownership and glossary; no broker code.
- `TRD-M002B: Versioned Schema Registry & Compatibility Gates` — machine-readable schemas, canonical fixtures, CI validators.
- `TRD-M003A: OMS Pure Transition Reducer` — formal transition model and exhaustive transition tests.
- `TRD-M004A: Risk Pure Policy Evaluator` — versioned policy composition and checked decimal math.
- `TRD-M005A: Atomic OMS-Risk-Outbox Unit of Work` — PostgreSQL transaction boundaries and lock ordering.
- `TRD-M006A: Execution Ledger & Correction Replay` — scoped IDs, idempotency and late-fill economics.
- `TRD-M007A: Deterministic Broker Simulator` — mock capabilities, failure scripts, no external send.
- `TRD-M008A: Fenced Dispatcher Simulation` — no-live network dispatch; crash-window property tests.
- `TRD-M009A: Recovery and Readiness Gates` — replay, reconciliation and manual reset.
- `TRD-M010A: IBKR Paper Adapter Verification` — separately approved, explicit paper-only guard.

## Standard prompt contract

Every Cursor prompt shall include: ID/title; authoritative source documents and versions; scope/non-goals; allowed files; design constraints; implementation strategy; threat and failure scenarios; acceptance criteria; commands to execute; evidence artifacts; report format; prompt archival path; commit title; explicit forbidden actions. Require a report with files changed, tests executed, skipped tests, failures, unresolved assumptions, and commit SHA. CI failures cannot be waived by AI agents. No credentials in fixtures or logs.

## Cross-contract verification matrix

| Gate | Owning milestones | Evidence |
|---|---|---|
| INT-01 | M0-02/03/04 | Type/schema/transition/policy checks |
| INT-02 | M0-05 | PostgreSQL isolation + crash tests |
| INT-03 | M0-08 | Writer-fencing fault injection |
| INT-04 | M0-07/10 | IBKR paper-only mapping and capability tests |
| INT-05 | M0-06/09 | Late fill, bust, cancel/fill race, replay |
| INT-06 | M0-04/10 | Valuation/FX/instrument versioning |
| INT-07 | M0-09 | Break-glass/reset negative tests |
| INT-08 | M0-09/11 | Backup/restore/audit integrity and readiness |

## Release policy

Architecture design resolved != tests passed. Passing paper-trading tests != live-trading authorization. Live trading requires a separate explicit approval and documented operational controls, financial limits, broker validation and incident response readiness. The next safe implementation request is **M0-02**, not live execution.
