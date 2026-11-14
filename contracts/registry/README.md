# Schema registry skeleton (M0-02B-a)

**Status:** `SKELETON_NOT_CANONICAL`  
**Part (b):** NO-GO  
**Live trading:** NOT AUTHORIZED  
**Broker uniqueness:** EVIDENCE_PENDING

M0-02B is split.

**(a) This directory.** [`registry.json`](registry.json) lists the domain shell and the types the architecture resolution requires later. `crates/trading-domain/tests/registry_skeleton.rs` rejects a published schema, a verified broker-uniqueness claim, a public `CorrelationId`, a schema path, and a transition flag set on this skeleton.

**(b) Canonical schemas and fixtures.** Event and command wire schemas, golden bytes, and compatibility fixtures wait until the type adjustments in [the resolution](../sources/M0-02A-UD-01-bis-UD-09-Architecture-Resolution-v1.0.md) are implemented and tested. Those adjustments include the HTTP id rename, numeric decimal equality, `RoundingPolicy`, RFC 3339 wire time, and an `OrderClosure` record whose reason vocabulary still depends on DET-01. Current serde shapes stay internal drafts. This directory does not serialize them as production.

CR-B10 §5 proposes `contracts/registry.yaml` plus `events/v1`, `commands/v1`, `api/v1`, `sql/v1`, and `examples`. Those paths are not created. The skeleton uses JSON because `serde_json` is already locked. That file is not the canonical serializer profile from UD-08. Ordinary `serde_json` map iteration is not canonical bytes.

B10-T01 through B10-T12 are not passed by this skeleton. The validator checks the skeleton's own fail-closed rules only.

`UnverifiedSessionScope` stays unverified. Renaming a field does not make a scope verified. Kind-specific uniqueness for `orderId`, `permId`, and `execId` stays **EVIDENCE_PENDING**.

The HTTP rename to `HttpRequestId`, the rounding-policy enum, RFC 3339 parsing, and the `OrderClosure` record are design-resolved and not introduced here. Incomplete `OrderClosure` is not a canonical wire schema.
