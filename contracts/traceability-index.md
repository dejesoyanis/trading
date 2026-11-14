# Traceability index

**Status:** index only. This file does not review contracts.  
**Date:** 2026-11-14  
**Live trading:** NOT AUTHORIZED

This index records which architecture sources are in the repository. A row marked `NOT_PROVIDED` has no body here. A row marked `NOT_REVIEWED` was not reviewed for this commit. Copying CR-B10 means the supplied draft file is present. It does not mean DET-01 through DET-10 or CR-B01 through CR-B09 were compared with it.

The architecture resolution cites DET-03 through DET-08 in its basis line. Those bodies were not attached to the prompt and are not in this repository. That citation is not a review.

Where the resolution refines a detail CR-B10 marks as provisional, the resolution applies. A DET body, once it is actually supplied, stays authoritative for its domain invariants. No DET body is supplied here.

## Supplied sources

| Document | Source name | Version | Status carried from the source | Repository |
| --- | --- | --- | --- | --- |
| CR-B10 | `CR-B10-Domain-Glossary-and-Schema-Registry-v1.0.md` | 1.0-draft | DESIGN_RESOLVED / IMPLEMENTATION_NOT_VERIFIED / LIVE_NOT_AUTHORIZED | Copied verbatim at `contracts/sources/`. Draft enum names. Not a transition model. |
| M0-02A resolution UD-01..UD-09 | `M0-02A-UD-01-bis-UD-09-Architecture-Resolution-v1.0.md` | dated 2026-11-14; the document does not declare a separate version number | DESIGN_RESOLVED; IMPLEMENTATION_NOT_VERIFIED; BROKER_SEMANTICS_UNVERIFIED; LIVE_NOT_AUTHORIZED | Copied verbatim at `contracts/sources/` and archived at `docs/cursor-prompts/trading/`. Implementation is pending. |

## Not supplied

| Id | What this repository knows | Version | Draft status | Review |
| --- | --- | --- | --- | --- |
| DET-01 | Cited by CR-B10 and by the resolution as the order transition table and closure-reason vocabulary. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| DET-02 | Cited as reservation transitions. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| DET-03 | Named in the resolution basis line. Body not attached. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| DET-04 | Named in the resolution basis line. Body not attached. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| DET-05 | Named in the resolution basis line. Body not attached. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| DET-06 | Cited as readiness gates. Body not attached. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| DET-07 | Cited as dispatch outcomes. Body not attached. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| DET-08 | Named in the resolution basis line. Body not attached. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| DET-09 | No body, and the resolution does not add a separate citation. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| DET-10 | Cited as the incident transition model. Body not attached. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| CR-B01 | Named by CR-B10 as an overriding contract. Title not supplied. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| CR-B02 | Named by CR-B10 as an overriding contract. Title not supplied. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| CR-B03 | Named by CR-B10 as an overriding contract. Title not supplied. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| CR-B04 | Named by CR-B10 as an overriding contract. Title not supplied. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| CR-B05 | Named by CR-B10 as an overriding contract. Title not supplied. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| CR-B06 | Named by CR-B10 as an overriding contract. Title not supplied. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| CR-B07 | Named by CR-B10 as an overriding contract. Title not supplied. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| CR-B08 | Named by CR-B10 as an overriding contract. Title not supplied. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| CR-B09 | Named by CR-B10 as an overriding contract. Title not supplied. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| ADFR / ARCH-01C | Architecture Decision and Foundations Register. The body was not supplied. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| ADFR-CR-B10-001 | One entry inside CR-B10 §9. Status in that file: DESIGN_RESOLVED / TESTS_NOT_EXECUTED. | 1.0-draft, as part of CR-B10 | DESIGN_RESOLVED / TESTS_NOT_EXECUTED | Present only inside the CR-B10 file. Not the register. Not independently reviewed. |
| `Architecture-Consolidation-and-Acceptance-v1.0.md` | Named by CR-B10 §0. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| `P1-Resolution-Contracts-CR-B05-bis-CR-B09-v1.0.md` | Named by CR-B10 §0. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |
| `Implementation-Roadmap-and-Cursor-Prompt-Plan-v1.0.md` | Not attached. | not supplied | not supplied | NOT_PROVIDED / NOT_REVIEWED |

## Broker evidence

Kind-specific uniqueness for `orderId`, `permId`, and `execId` is **EVIDENCE_PENDING**. `UnverifiedSessionScope` stays unverified. This index does not publish a uniqueness rule table.

## M0-02B split

Part (a), recorded with this index: the registry skeleton and its validator. See [contracts/registry/README.md](registry/README.md).

Part (b), not recorded as done: canonical event and command schemas, golden bytes, and compatibility fixtures. Those wait until the type adjustments in the resolution are implemented and tested. **NO-GO.**

## Shell records

[contracts/domain/glossary.md](domain/glossary.md) is the type inventory of the fail-closed shell.

[contracts/domain/unresolved-decisions.md](domain/unresolved-decisions.md) marks UD-01 through UD-09 **DESIGN_RESOLVED / IMPLEMENTATION_PENDING** and keeps the shell limitations. Broker uniqueness stays **EVIDENCE_PENDING**.
