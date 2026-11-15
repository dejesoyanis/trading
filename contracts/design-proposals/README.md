# Design proposals

This directory holds review packages. A file here is not a user decision, not a canonical schema, and not an implementation.

## TRD-ARCH-03B.1

OMS finality, `OrderClosure`, and transition coverage. Ratification: `NOT_RATIFIED`. `XC-03-MATRIX` stays OPEN. No transition edge is approved. No closure-reason vocabulary is approved. Live trading is not authorized.

| File | Role |
| --- | --- |
| [TRD-ARCH-03B1-OMS-semantics-and-invariants.md](TRD-ARCH-03B1-OMS-semantics-and-invariants.md) | Dimensions, identifiers, trust boundary, and invariants. |
| [TRD-ARCH-03B1-transition-coverage-draft.md](TRD-ARCH-03B1-transition-coverage-draft.md) | Label inventory and scenario families. Not a matrix. |
| [TRD-ARCH-03B1-OrderClosure-vocabulary-options.md](TRD-ARCH-03B1-OrderClosure-vocabulary-options.md) | Three vocabulary strategies. Recommendation is not a decision. |
| [TRD-ARCH-03B1-approval-requests.md](TRD-ARCH-03B1-approval-requests.md) | OMSQ-01 through OMSQ-12. Each is `USER_DECISION_PENDING`. |
| [TRD-ARCH-03B1-dependency-and-conflict-matrix.md](TRD-ARCH-03B1-dependency-and-conflict-matrix.md) | Open XC findings and later ARCH-03B.2 / ARCH-03B.3 dependencies. |
| [TRD-ARCH-03B1-proposal-index.json](TRD-ARCH-03B1-proposal-index.json) | Non-runtime index for documentation checks. Not a registry and not a wire schema. |

The index machine-readable file repeats the pending statuses. It does not publish schemas and it does not change `contracts/registry/registry.json`.
