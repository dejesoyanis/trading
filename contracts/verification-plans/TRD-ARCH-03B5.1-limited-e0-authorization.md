# TRD-ARCH-03B.5.1 — limited E0 public-documentation authorization

**Date:** 2026-11-16  
**Role:** dated authorization record for one named action. `document_role` = `NON_RUNTIME_NON_WIRE_AUTHORIZATION_RECORD`.  
**Baseline:** `f6686d46ba1b8b1fb804506d207f7878a59c218e` on `main`. Subject: `TRD-ARCH-03B.5: Research Official IBKR Documentation`.  
**Predecessor:** `docs/architecture/TRD-ARCH-03B.5-report.md`, verdict `PARTIAL`, obtained sources 0, external fetch `NOT_PERFORMED`.  
**Live trading:** NOT AUTHORIZED.  
**New ARCH-DEC:** none. `ARCH-DEC-63` stays unused.  
**ARCH-DEC-49 satisfied:** false. This record does not ratify a source pair.

This file is not a design-policy ratification. It does not edit `contracts/verification-plans/TRD-ARCH-03B4-ibkr-verification-protocol.md`, `contracts/verification-plans/TRD-ARCH-03B4-ibkr-evidence-capture-and-provenance.md`, or ARCH-DEC-01 through ARCH-DEC-62. **NO IMPLEMENTATION.** **NO VERIFICATION.**

## What the user authorized on 2026-11-16

The user instruction for TRD-ARCH-03B.5.1, archived with this milestone, authorizes a limited E0 action: unauthenticated retrieval, verification, local archival, and analysis of public official IBKR documentation only.

The checkpoint label for that named action is `GO_LIMITED_PUBLIC_DOCS_ONLY`. The label is a documentary record in this file. It is not a runtime flag and not a wire field.

`GO_LIMITED_PUBLIC_DOCS_ONLY` means only:

- public documentation pages and public documentation assets
- HTTPS
- GET and HEAD
- hosts and path scopes written in `TRD-ARCH-03B5.1-host-allowlist.json` after publisher evidence
- no cookies, no authentication headers, no credential-bearing query parameters
- redirects inspected and accepted only when every hop stays on that allowlist

## Narrow precedence over the previous STOP

The 2026-11-15 protocol records E0 as STOP and `NOT_AUTHORIZED`, and it says a later go is a separate dated user authorization that names the stage. That protocol file says it is not itself the authorization. The 2026-11-16 TRD-ARCH-03B.5 instruction did not name a stage go, so retrieval stopped.

This 2026-11-16 TRD-ARCH-03B.5.1 instruction is the separate dated authorization. It names E0 and limits the go to the public-documentation operations above. For those operations only, this record takes narrow precedence over the previous E0 STOP. The historical protocol rows stay STOP. They are not rewritten.

Every E0 action that this instruction does not name stays STOP. That includes broker-observed version metadata, installed-product capture, non-public documents, and any retrieval that is not an allowlisted public documentation GET or HEAD.

## What stays STOP

| Stage or action | Position |
| --- | --- |
| E0 actions other than the named public-documentation GET and HEAD | STOP |
| E1 offline fixtures and replay | STOP |
| E2 broker reads | STOP |
| E3 broker fault injection and order experiments | STOP |
| E4 adjudication and ratification | STOP |
| Broker login, account, session, TWS, IB Gateway, Client Portal account or API access | STOP |
| Order placement, cancellation, modification, replacement, retry | STOP |
| Paper trading and live trading | STOP |
| Identity resolver, canonical schema, DDL, migration | STOP |
| Economic posting, reservation release, finality, OMS transition, broker dispatch | STOP |

`reads_approved` stays `NO_READS_APPROVED`. A documentation sentence that names an operation does not approve calling it. Cancel, modify, and replace are not reads. No environment class is selected. No version tuple is invented. ARCH-DEC-49 is not satisfied by this authorization. ARCH-DEC-62 keeps a later ratification in a different instruction and a different commit.

## Protocol expression

The limited checkpoint can be recorded here without editing the historical protocol and without opening E1, E2, E3, E4, or unnamed E0 actions. The binding protocol already treats a later go as a separate document. This file is that document, narrowed to public documentation. No wider stage go is granted.
