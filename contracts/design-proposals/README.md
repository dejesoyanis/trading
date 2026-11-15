# Design proposals

This directory holds review packages and, after TRD-ARCH-03B.1.2, a non-runtime map to a decision record that lives under `contracts/decisions/`. A file here is not a canonical schema and not an implementation. The decision itself is the dated record, not this directory.

## TRD-ARCH-03B.1

OMS finality, `OrderClosure`, and transition coverage. Ratification: `NOT_RATIFIED`. `XC-03-MATRIX` stays OPEN. No transition edge is approved. No closure-reason vocabulary is approved. Live trading is not authorized.

| File | Role |
| --- | --- |
| [TRD-ARCH-03B1-OMS-semantics-and-invariants.md](TRD-ARCH-03B1-OMS-semantics-and-invariants.md) | Dimensions, identifiers, trust boundary, and invariants. |
| [TRD-ARCH-03B1-transition-coverage-draft.md](TRD-ARCH-03B1-transition-coverage-draft.md) | Label inventory and scenario families. Not a matrix. |
| [TRD-ARCH-03B1-OrderClosure-vocabulary-options.md](TRD-ARCH-03B1-OrderClosure-vocabulary-options.md) | Three vocabulary strategies. Recommendation is not a decision. |
| [TRD-ARCH-03B1-approval-requests.md](TRD-ARCH-03B1-approval-requests.md) | OMSQ-01 through OMSQ-12. Creation-time status lines stay `USER_DECISION_PENDING`. A dated resolution index at the end points at ARCH-DEC-09 through ARCH-DEC-20. |
| [TRD-ARCH-03B1-dependency-and-conflict-matrix.md](TRD-ARCH-03B1-dependency-and-conflict-matrix.md) | Open XC findings and later ARCH-03B.2 / ARCH-03B.3 dependencies. Sections 1 through 4 stay the proposal-time map. |
| [TRD-ARCH-03B1-proposal-index.json](TRD-ARCH-03B1-proposal-index.json) | Creation-time non-runtime index. `NON_RUNTIME_NON_WIRE_PROPOSAL_INDEX`. Not a registry and not a wire schema. |

The proposal index repeats the creation-time pending statuses. It does not publish schemas and it does not change `contracts/registry/registry.json`. It is unchanged by TRD-ARCH-03B.1.2.

## Later policy record (TRD-ARCH-03B.1.2)

On 2026-11-15 the user ratified design-policy choices for OMSQ-01 through OMSQ-12. The record is ARCH-DEC-09 through ARCH-DEC-20 in [ARCH-DEC-09-bis-20-2026-11-15.md](../decisions/ARCH-DEC-09-bis-20-2026-11-15.md). The machine-readable map is [TRD-ARCH-03B12-resolution-index.json](TRD-ARCH-03B12-resolution-index.json). That map is `NON_RUNTIME_NON_WIRE_RESOLUTION_INDEX`. It is not a registry and not a wire schema. Its labels are not a canonical enum.

The proposal files above keep their creation-time text. Their `NOT_RATIFIED` lines and the pending question statuses are the historical request. The later record approves design policy only. It does not approve an `OrderClosure` vocabulary, a transition edge, a matrix version, an economic-finality predicate, automatic economic posting, risk release, broker terminal acceptance, modify or replace dispatch, or live trading. `XC-03-MATRIX` stays OPEN.

## TRD-ARCH-03B.2

Risk reservation, partial fills, corrections, and economic finality. Ratification: `NOT_RATIFIED`. `XC-02-PRED` stays OPEN. No economic-finality predicate is approved. No reservation release is approved. No economic posting is approved. Live trading is not authorized. That package did not start ARCH-03B.3.

| File | Role |
| --- | --- |
| [TRD-ARCH-03B2-risk-semantics-and-invariants.md](TRD-ARCH-03B2-risk-semantics-and-invariants.md) | Dimensions, invariant inventory, and fail-closed boundaries. |
| [TRD-ARCH-03B2-economic-finality-and-release-coverage-draft.md](TRD-ARCH-03B2-economic-finality-and-release-coverage-draft.md) | Candidate evidence and scenario coverage. Not a predicate. |
| [TRD-ARCH-03B2-reservation-and-correction-options.md](TRD-ARCH-03B2-reservation-and-correction-options.md) | Three comparison architectures. Not allowed operations. |
| [TRD-ARCH-03B2-approval-requests.md](TRD-ARCH-03B2-approval-requests.md) | RISKQ-01 through RISKQ-14. Creation-time status lines stay `USER_DECISION_PENDING`. A dated resolution addendum at the end points at ARCH-DEC-21 through ARCH-DEC-34. |
| [TRD-ARCH-03B2-dependency-and-conflict-matrix.md](TRD-ARCH-03B2-dependency-and-conflict-matrix.md) | Open XC findings, including `XC-02-PRED`. |
| [TRD-ARCH-03B2-proposal-index.json](TRD-ARCH-03B2-proposal-index.json) | Non-runtime index. `NON_RUNTIME_NON_WIRE_PROPOSAL_INDEX`. Not a registry and not a wire schema. |

The recommendation inside the options note is not a decision. OMSQ-05 option B and OMSQ-06 option A stay binding. ARCH-DEC-09 through ARCH-DEC-20 are not reopened.

The proposal index repeats the creation-time pending statuses. It is unchanged by TRD-ARCH-03B.2.2.

## Later policy record (TRD-ARCH-03B.2.2)

On 2026-11-15 the user ratified design-policy choices for RISKQ-01 through RISKQ-14. The record is ARCH-DEC-21 through ARCH-DEC-34 in [ARCH-DEC-21-bis-34-2026-11-15.md](../decisions/ARCH-DEC-21-bis-34-2026-11-15.md). The machine-readable map is [TRD-ARCH-03B22-resolution-index.json](TRD-ARCH-03B22-resolution-index.json). That map is `NON_RUNTIME_NON_WIRE_RESOLUTION_INDEX`. It is not a registry, not the historical proposal index, and not a wire schema. Its labels are not a canonical enum.

The proposal files above keep their creation-time text. Their `NOT_RATIFIED` lines and the pending question statuses are the historical request. The later record approves design policy only. Options are B, A, A, A, A, A, A, A, A, B, B, A, A, B. It does not approve budget arithmetic, a reservation writer, reservation release, economic posting, an economic-finality predicate, a rounding mode, a lock order, an audit table, a role, a policy migration, an OMS transition, broker dispatch, or live trading. `XC-02-PRED` stays OPEN. ARCH-DEC-13 and ARCH-DEC-14 stay binding. That record did not start ARCH-03B.3. **NO IMPLEMENTATION.** **NO VERIFICATION.**

## TRD-ARCH-03B.3

Broker identity, snapshot completeness, and unknown-send recovery. Ratification: `NOT_RATIFIED`. BROKERQ-01 through BROKERQ-14 are `USER_DECISION_PENDING`. A recommendation is not a decision. `XC-01` stays OPEN. `XC-08-IBKR` stays OPEN. XC-04 design precedence stays `RESOLVED_BY_ARCH_DEC_03`, and the operational remainder stays OPEN. No canonical identity tuple, uniqueness index, resolver, economic posting, reservation release, OMS transition, or broker dispatch is approved. Live trading is not authorized.

| File | Role |
| --- | --- |
| [TRD-ARCH-03B3-broker-identity-semantics-and-invariants.md](TRD-ARCH-03B3-broker-identity-semantics-and-invariants.md) | Namespaces, trust boundaries, and candidate evidence. Not a tuple. |
| [TRD-ARCH-03B3-snapshot-completeness-and-reconciliation-coverage-draft.md](TRD-ARCH-03B3-snapshot-completeness-and-reconciliation-coverage-draft.md) | Coverage rows. Not a completeness proof. |
| [TRD-ARCH-03B3-identity-snapshot-and-recovery-options.md](TRD-ARCH-03B3-identity-snapshot-and-recovery-options.md) | Three approaches on each axis. The unsafe contrast is not ratifiable. |
| [TRD-ARCH-03B3-approval-requests.md](TRD-ARCH-03B3-approval-requests.md) | BROKERQ-01 through BROKERQ-14. Creation-time status lines stay `USER_DECISION_PENDING`. A dated resolution addendum at the end points at ARCH-DEC-35 through ARCH-DEC-48. |
| [TRD-ARCH-03B3-dependency-and-conflict-matrix.md](TRD-ARCH-03B3-dependency-and-conflict-matrix.md) | Open XC findings and the handoff that does not authorize later work. |
| [TRD-ARCH-03B3-proposal-index.json](TRD-ARCH-03B3-proposal-index.json) | Non-runtime index. `NON_RUNTIME_NON_WIRE_PROPOSAL_INDEX`. Not a registry and not a wire schema. |

The proposal index repeats the creation-time pending statuses. It is unchanged by TRD-ARCH-03B.3.2.

## Later policy record (TRD-ARCH-03B.3.2)

On 2026-11-15 the user ratified design-policy choices for BROKERQ-01 through BROKERQ-14. The record is ARCH-DEC-35 through ARCH-DEC-48 in [ARCH-DEC-35-bis-48-2026-11-15.md](../decisions/ARCH-DEC-35-bis-48-2026-11-15.md). The machine-readable map is [TRD-ARCH-03B32-resolution-index.json](TRD-ARCH-03B32-resolution-index.json). That map is `NON_RUNTIME_NON_WIRE_RESOLUTION_INDEX`. It is not a registry, not the historical proposal index, and not a wire schema. Its labels are not a canonical enum.

The `NOT_RATIFIED` line above is the creation-time package status. The proposal files keep their creation-time text. The later record approves design policy only. Options are B, B, B, B, B, B, A, B, B, B, B, B, B, B. It does not approve a canonical identity tuple, a uniqueness index, a resolver, a schema, a completeness token, economic posting, an economic-finality predicate, reservation release, an OMS transition, broker dispatch, a role, a resubmit, or live trading. `XC-01` stays OPEN. `XC-08-IBKR` stays OPEN. `XC-02-PRED` stays OPEN. The XC-04 operational remainder stays OPEN. ARCH-DEC-03, ARCH-DEC-13, ARCH-DEC-14, ARCH-DEC-17, ARCH-DEC-26, ARCH-DEC-33, and ARCH-DEC-34 stay binding. External broker cases stay `NOT_EXECUTED`. **NO IMPLEMENTATION.** **NO VERIFICATION.**

## TRD-ARCH-03B.4

IBKR evidence and verification planning lives under [contracts/verification-plans/README.md](../verification-plans/README.md). That package is `NOT_RATIFIED`. IBKREVQ-01 through IBKREVQ-14 are `USER_DECISION_PENDING`. Every external case is `NOT_EXECUTED` and `EXTERNAL_EVIDENCE_PENDING`. The plan index is `NON_RUNTIME_NON_WIRE_VERIFICATION_PLAN_INDEX`. It is not a registry and not a wire schema. The plan does not authorize a broker connection, a read, an order, or a test run. It does not close `XC-01`, `XC-08-IBKR`, the XC-04 operational remainder, or `XC-02-PRED`. ARCH-DEC-35 through ARCH-DEC-48 stay `NOT_IMPLEMENTED` and `NOT_VERIFIED`. Documentation tests are not broker evidence.
