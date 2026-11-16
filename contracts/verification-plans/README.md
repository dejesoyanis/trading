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

## Later policy record (TRD-ARCH-03B.4.2)

On 2026-11-15 the user ratified design-policy choices for IBKREVQ-01 through IBKREVQ-14 as ARCH-DEC-49 through ARCH-DEC-62. The record is [ARCH-DEC-49-bis-62-2026-11-15.md](../decisions/ARCH-DEC-49-bis-62-2026-11-15.md). The machine-readable map is [TRD-ARCH-03B42-resolution-index.json](TRD-ARCH-03B42-resolution-index.json), role `NON_RUNTIME_NON_WIRE_RESOLUTION_INDEX`. It is not a registry, not the historical plan index, and not a wire schema.

The `NOT_RATIFIED` line above is the creation-time plan status. The historical questions, recommendations, and option tables stay as written. The later record approves design policy only. Options are B, A, A, B, B, B, A, A, B, A, A, A, A, B. Count: 8 times A and 6 times B. Option C stays `CONFLICTS_WITH_BINDING_POLICY`. Stages E0 through E4 stay STOP. The record does not authorize a broker connection, a read, an order, paper trading, a uniqueness guarantee, a completeness token, economic posting, risk release, an economic-finality predicate, an OMS transition, broker dispatch, or live trading. External cases stay `NOT_EXECUTED`. External evidence stays `EXTERNAL_EVIDENCE_PENDING`. `XC-01`, `XC-08-IBKR`, `XC-02-PRED`, and the XC-04 operational remainder stay OPEN. Documentation tests are not broker evidence. **NO IMPLEMENTATION.** **NO VERIFICATION.**

## Public research boundary (TRD-ARCH-03B.5)

On 2026-11-16 the user authorized unauthenticated public documentation research and kept stages E0 through E4 at STOP. The existing protocol requires a separate dated authorization that names stage E0 before official text is obtained. No external retrieval was performed. Obtained sources: 0. The machine-readable map is [TRD-ARCH-03B5-source-index.json](TRD-ARCH-03B5-source-index.json), role `NON_RUNTIME_NON_WIRE_RESEARCH_INDEX`. It is not a registry, not the historical plan index, and not a wire schema. `public_documentation_research_authorized=true` is not a stage go. Claims stay `HYPOTHESIS`. Scenarios stay `NOT_EXECUTED`. No read is approved. The reading record is [TRD-ARCH-03B.5-report.md](../../docs/architecture/TRD-ARCH-03B.5-report.md). Its verdict is `PARTIAL`. **NO IMPLEMENTATION.** **NO VERIFICATION.**

## Limited public-documentation acquisition (TRD-ARCH-03B.5.1)

On 2026-11-16 a later instruction named a limited E0 action for unauthenticated public documentation only. The record is [TRD-ARCH-03B5.1-limited-e0-authorization.md](TRD-ARCH-03B5.1-limited-e0-authorization.md). The checkpoint label for that named action is `GO_LIMITED_PUBLIC_DOCS_ONLY`. It is not a general E0 go. E1 through E4 stay STOP. Unnamed E0 actions stay STOP. The 2026-11-15 protocol file is unchanged and still says STOP.

The host allowlist is [TRD-ARCH-03B5.1-host-allowlist.json](TRD-ARCH-03B5.1-host-allowlist.json). The acquisition register is [TRD-ARCH-03B5.1-acquisition-register.json](TRD-ARCH-03B5.1-acquisition-register.json). The claim and scenario register is [TRD-ARCH-03B5.1-research-register.json](TRD-ARCH-03B5.1-research-register.json). All three are `NON_RUNTIME_NON_WIRE`. They are not registry entries. `ARCH-DEC-49` is not satisfied: the second copy is `SECOND_COPY_PENDING`. No read is approved. No environment is selected. The reading record is [TRD-ARCH-03B.5.1-report.md](../../docs/architecture/TRD-ARCH-03B.5.1-report.md). **NO IMPLEMENTATION.** **NO VERIFICATION.**
