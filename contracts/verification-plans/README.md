# Verification plans

This directory holds documentation-only plans for future evidence. A file here is not a canonical schema, not a wire format, not a registry entry, and not an execution record.

## TRD-ARCH-03B.4

IBKR evidence and verification planning. Ratification: `NOT_RATIFIED`. Every `IBKREVQ` question is `USER_DECISION_PENDING`. Every external case is `NOT_EXECUTED`. External evidence is `EXTERNAL_EVIDENCE_PENDING`. This package does not authorize a broker connection, a read, an order, or a test run. Live trading is not authorized.

| File | Role |
| --- | --- |
| [TRD-ARCH-03B4-ibkr-evidence-inventory.md](TRD-ARCH-03B4-ibkr-evidence-inventory.md) | Claim register. Hypotheses only. |
| [TRD-ARCH-03B4-ibkr-verification-protocol.md](TRD-ARCH-03B4-ibkr-verification-protocol.md) | Future stages E0–E4. Each checkpoint is STOP. |
| [TRD-ARCH-03B4-ibkr-test-matrix.md](TRD-ARCH-03B4-ibkr-test-matrix.md) | Scenario rows. No execution results. |
| [TRD-ARCH-03B4-ibkr-evidence-capture-and-provenance.md](TRD-ARCH-03B4-ibkr-evidence-capture-and-provenance.md) | Proposed future capture labels. Not a wire schema. |
| [TRD-ARCH-03B4-dependency-and-conflict-matrix.md](TRD-ARCH-03B4-dependency-and-conflict-matrix.md) | Evidence gates. No finding is closed. |
| [TRD-ARCH-03B4-approval-requests.md](TRD-ARCH-03B4-approval-requests.md) | IBKREVQ-01 through IBKREVQ-14. Pending. |
| [TRD-ARCH-03B4-plan-index.json](TRD-ARCH-03B4-plan-index.json) | Non-runtime index. Not `registry.json`. |

The reading record is [TRD-ARCH-03B.4-report.md](../../docs/architecture/TRD-ARCH-03B.4-report.md). ARCH-DEC-01 through ARCH-DEC-48 stay in force. A recommendation in the approval requests is not an approval. **NO IMPLEMENTATION.** **NO VERIFICATION.**
