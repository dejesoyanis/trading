# TRD-ARCH-03B.5 — public research authority and boundary

**Date:** 2026-11-16  
**Role:** boundary record for a narrowly scoped documentation task. `document_role` = `NON_RUNTIME_NON_WIRE_RESEARCH_NOTE`.  
**Baseline:** `ac60bf932355660548bf5751963009ea956b76a5` on `main`. Subject: `TRD-ARCH-03B.4.2: Ratify IBKR Verification Strategy`. Parent: `4bbfd6748103b806619a4c0ac762a12c427595d1`. Worktree at start: clean.  
**Live trading:** NOT AUTHORIZED.  
**Stages E0, E1, E2, E3, and E4:** STOP.  
**External fetch:** `NOT_PERFORMED`.

This note is not a stage go. It does not assign ARCH-DEC-63. It does not change ARCH-DEC-01 through ARCH-DEC-62. **NO IMPLEMENTATION.** **NO VERIFICATION.**

## What the user authorized

The user instruction dated 2026-11-16 authorizes only public official IBKR documentation research that can be retrieved without login. The same instruction says that permission is source work for a later E0 package. It is not a blanket E0 stage go. It does not move the E0, E1, E2, E3, or E4 checkpoints. It does not approve a broker connection, a read, an order, paper trading, or live trading.

The instruction also says: if the existing protocol expressly requires a different formal approval for every E0 action, stop before any external retrieval, document the conflict, and ask for a separate authorization.

## What the repository already requires

`contracts/verification-plans/TRD-ARCH-03B4-ibkr-verification-protocol.md` states that no stage is open and that a later go would be a separate dated user authorization that names the stage. E0 is defined as collecting official IBKR material and version metadata that was actually obtained. The recorded checkpoint is STOP. The recorded authorization is `NOT_AUTHORIZED`.

`contracts/verification-plans/TRD-ARCH-03B4-ibkr-evidence-capture-and-provenance.md` step 1 says an official copy is taken only under a stage whose checkpoint was moved to go by a separate authorization. The same row records the current state as STOP and nothing obtained.

ARCH-DEC-49 approves a design policy for obtained primary text and a second copy. The decision text says that choice does not start E0 and does not approve research or procurement in the name of an authorized stage. ARCH-DEC-53 forbids a broker connection and a read until a later dated decision names the exact official read operations. ARCH-DEC-62 separates later collection from later ratification.

## Conflict and stop

Collecting official primary text is the work the protocol places in stage E0. That stage starts only when a dated authorization names the stage and the checkpoint moves from STOP to go. The 2026-11-16 instruction does not name that go, and it keeps every checkpoint at STOP. The instruction's own stop rule therefore applies.

No external retrieval was performed. No documentation host was called. No redirect was followed. No login page, account page, session endpoint, TWS socket, IB Gateway socket, or broker API endpoint was called. A documentation URL is not a broker API endpoint, and none was requested.

This stop is a protocol conflict. It is not a network outage and not a license ruling. Because no copy was obtained, no statement is `OFFICIAL_DOCUMENTATION_OBTAINED`. `SECOND_COPY_PENDING` is not used: that label would imply a primary copy already exists.

## Allowed class, not used

If a later authorization both names stage E0 and moves that checkpoint to go, the only retrieval class contemplated by the 2026-11-16 instruction is an unauthenticated fetch of a public official documentation page. The final response would have to remain on a documentation host. A redirect to a login host, an account host, a session host, or an API host would not be followed. If the final host could not be kept inside that class, the source status would be `NOT_RETRIEVED` and the fetch would not be treated as success.

Concrete hostnames are `UNRECORDED`. Writing a hostname from memory would invent a URL. This milestone records none.

## What stays closed

Checkpoints E0 through E4 stay STOP. `stage_e0_authorized` stays false. `public_documentation_research_authorized=true` on the research index records the narrow permission in the instruction. That flag is not a stage go.

No broker login, account, credential, TWS or IB Gateway process, broker API call, paper order, live order, cancel, modify, replace, retry, resend, takeover, fault injection, production Rust change, migration, canonical schema, identity resolver, UNIQUE index, OMS transition, economic posting, risk release, or economic-finality predicate is authorized or performed.

Claims stay `HYPOTHESIS` and `EXTERNAL_EVIDENCE_PENDING`. Scenarios stay `NOT_EXECUTED`. ADFR `verification` stays `NOT_VERIFIED`. `XC-01`, `XC-08-IBKR`, `XC-02-PRED`, `XC-03-MATRIX`, and the operational remainder of `XC-04` stay OPEN. Documentation tests are not broker evidence.
