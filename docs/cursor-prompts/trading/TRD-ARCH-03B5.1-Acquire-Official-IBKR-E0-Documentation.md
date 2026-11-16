# Cursor execution prompt — TRD-ARCH-03B.5.1

## Explicit, narrowly scoped E0 official IBKR documentation acquisition

**Instruction date:** 2026-11-16  
**Authority:** Explicit user authorization to perform only the E0 public-documentation acquisition described here.  
**Mode:** Documentation/research/provenance only; no trading-runtime implementation.  
**Git:** Exactly one local commit if acceptance gates pass; **NO PUSH**.  
**Target commit subject:** `TRD-ARCH-03B.5.1: Acquire Official IBKR E0 Documentation`

This file archives the executed instruction. It is not a second authorization and not a broker approval.

## Authorization boundary

The user explicitly authorized a limited E0 GO for unauthenticated retrieval, verification, local archival, and analysis of public, official IBKR documentation only. This is a specific exception to the previous E0 STOP solely for the named public-documentation operations. It is not a blanket E0 approval. The E0 checkpoint may be recorded as `GO_LIMITED_PUBLIC_DOCS_ONLY` for the duration and scope of this task, with all unlisted E0 actions still STOP. Historical approvals are not edited to manufacture authority.

Never authorized: E1, E2, E3, and E4; broker login; account or session access; TWS or IB Gateway startup or connection; Client Portal account or API access; authenticated requests; API reads; order placement, cancellation, modification, replacement, or retry; paper or live trading; broker test execution; fault injection; broker dispatch; risk reservation or release; economic posting, finality, or closure; OMS transitions; identity resolver; canonical schema, DDL, or migration; secrets or credentials; production source changes. Public-documentation HTTP GET and HEAD requests are not broker API reads.

Successful documentation retrieval is not proof of broker uniqueness, snapshot completeness, writer fencing, operational recovery, or financial safety. No ADFR `verification` promotion and no closure of XC findings.

## Required work

1. Preflight the clean worktree, the TRD-ARCH-03B.5 predecessor, ARCH-DEC-01 through ARCH-DEC-62, and the unused state of ARCH-DEC-63. Do not allocate a new ARCH-DEC.
2. Verify the twelve manifest sources, the 78 ADFR entries, the registry, and the decision files by actual hash.
3. Before documentation fetches, write an exact-host HTTPS allowlist from publisher evidence. Do not fetch a remembered hostname or a search-result URL merely because its snippet says IBKR. Reject login, account, API, loopback, and off-allowlist redirects.
4. Retrieve public documentation for TWS API, IB Gateway, and Client Portal API as separate families. Archive bytes outside git. Record a second retrieval honestly. Two GETs of the same URL are not independent corroboration. If no independent second copy exists, mark `SECOND_COPY_PENDING` and do not claim ARCH-DEC-49 satisfied.
5. Map all 27 `IBKR-CLM-01` through `IBKR-CLM-27` and all 43 `IBKR-SCN-A01` through `IBKR-SCN-D06` once. Separate documented sentences from empirical verification. Broker tests stay `NOT_EXECUTED` and `EXTERNAL_EVIDENCE_PENDING`.
6. If official text names operations, list them as unapproved candidates. Preserve `NO_READS_APPROVED` and E2 STOP. Do not label cancel, modify, or replace as read-only. Do not select an environment.
7. Add documentation regression tests. Run `cargo fmt --all -- --check`, `cargo check --workspace --locked`, `cargo test --workspace --locked`, and `cargo clippy --workspace --all-targets --locked -- -D warnings`.
8. Report every gate `A03B51-G01` through `A03B51-G12`. `PASS` only when every gate passes, including a real acquisition. Pending second copies stay partial. Do not push.

**NO IMPLEMENTATION.** **NO VERIFICATION.**
