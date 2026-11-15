# Schema registry skeleton (M0-02B-a)

**Status:** `SKELETON_NOT_CANONICAL`  
**Part (b):** NO-GO  
**Live trading:** NOT AUTHORIZED  
**Broker uniqueness:** EVIDENCE_PENDING

M0-02B is split.

**(a) This directory.** [`registry.json`](registry.json) lists the domain shell and the types the architecture resolution requires later. `crates/trading-domain/tests/registry_skeleton.rs` rejects a published schema, a verified broker-uniqueness claim, a public `CorrelationId`, a schema path, and a transition flag set on this skeleton.

**(b) Canonical schemas and fixtures.** Event and command wire schemas, golden bytes, and compatibility fixtures are not in this directory. M0-02A.2 locally tested the HTTP id rename, numeric decimal equality, the `RoundingPolicy` vocabulary, and opt-in RFC 3339. `OrderClosure` still depends on a closure-reason vocabulary. TRD-ARCH-SYNC-01 added a reconstructed DET-01 draft that says the vocabulary is not confirmed. ARCH-DEC-01 (2026-11-15) ratifies that draft as `DESIGN_BASELINE_RATIFIED_WITH_EXCEPTIONS`. The unconfirmed reason vocabulary is one of those exceptions, so none was invented and the draft was not adopted as an `OrderClosure` schema. Applied rounding and verified broker uniqueness are not implemented. Current serde shapes stay internal drafts. This directory does not serialize them as production.

CR-B10 §5 proposes `contracts/registry.yaml` plus `events/v1`, `commands/v1`, `api/v1`, `sql/v1`, and `examples`. Those paths are not created. The skeleton uses JSON because `serde_json` is already locked. That file is not the canonical serializer profile from UD-08. Ordinary `serde_json` map iteration is not canonical bytes.

B10-T01 through B10-T12 are not passed by this skeleton. The validator checks the skeleton's own fail-closed rules only.

`UnverifiedSessionScope` stays unverified. Renaming a field does not make a scope verified. Kind-specific uniqueness for `orderId`, `permId`, and `execId` stays **EVIDENCE_PENDING**.

`HttpRequestId`, numeric decimal comparison, the `RoundingPolicy` vocabulary, opt-in RFC 3339, and unverified `BrokerIdentityContext` are recorded in the skeleton as internal drafts. `OrderClosure` is still absent. The reconstructed DET-01 draft does not confirm a closure-reason vocabulary, so none was invented. Incomplete `OrderClosure` is not a canonical wire schema. `RoundingPolicy` does not authorize rounding.
