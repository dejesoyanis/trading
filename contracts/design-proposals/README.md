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

The proposal files above keep their creation-time text. Their `NOT_RATIFIED` lines and the pending question statuses are the historical request. The later record approves design policy only. It does not approve an `OrderClosure` vocabulary, a transition edge, a matrix version, an economic-finality predicate, automatic economic posting, risk release, broker terminal acceptance, modify or replace dispatch, or live trading. `XC-03-MATRIX` stays OPEN. ARCH-03B.2 and ARCH-03B.3 are not started.
