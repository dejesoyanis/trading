# Unresolved domain decisions

**Status:** open. These are not closed architecture decisions.  
**Milestone:** M0-02A provisional shells only.  
**Date:** 2026-11-14

M0-02A needed Rust types whose full representation is not fixed by CR-B10. Where a shell was required, it fails closed and is named here. Do not treat a shell as production-proven. DET-01 through DET-10 and CR-B01 through CR-B09 were not in the repository, so no transition or policy text from those documents was available to conflict with CR-B10. If a later copy of those documents uses different names or scopes, that text overrides these shells and the vocabulary must be revised. Do not silently keep the shell.

## UD-01 — Two types named CorrelationId

`trading_common::CorrelationId` is the existing HTTP request handle. It is process-local and is not a distributed trace. CR-B10 `CorrelationId` is the domain id that links commands and events.

M0-02A keeps both types. There is no conversion. Renaming either one would change a foundation type or invent a name the contract does not use. A later ADR should rename one of them before any API publishes both on the same document.

## UD-02 — Broker alias scope, including IBKR

CR-B10 says a broker order alias includes broker, account, client or session as appropriate, and the external id kind. It also says IBKR identity and session isolation are not yet evidenced.

The shell records:

- broker, account, and an environment label
- either no session id (`account_only`) or one internal `BrokerSessionId`
- an `UnverifiedSessionScope` value on every scope
- separate types for order alias and permanent alias

A separate client id is not modeled. Equality is structural equality of the recorded scope. It is not a uniqueness proof and it is not a merge. `session_scope` on the draft JSON shell must be the text `unverified`; `verified` is rejected. No identity resolver is implemented.

## UD-03 — Decimal representation, scale, and rounding

CR-B10 requires checked decimal arithmetic and an explicit rounding policy. It does not name the coefficient width, the scale bound, or the rounding mode.

The shell stores a signed `i128` coefficient and a scale from 0 through 38. Thirty-eight is the decimal digit budget of `i128`, not a tick size. Values at different scales are not normalized: coefficient `10` at scale `1` is not equal to coefficient `100` at scale `2`. Addition and subtraction require the same scale. `rescale` always returns `NotImplemented`, including when the scale is unchanged. There is no `f32` or `f64` field.

Currency codes are uppercase ASCII, 1 to 16 letters. That is not an ISO 4217 registry check. Lowercase is rejected rather than case-folded. Foreign exchange always returns `NotImplemented`.

Instrument ids are printable ASCII, 1 to 64 bytes, with no case folding and no symbology check.

## UD-04 — Version text versus version integers

`OrderVersion` is a `u64` because CR-B10 section 1 says so. `SchemaVersion` is a `u64` because the only envelope example is a JSON integer. It has no successor method. A larger integer is not a compatibility claim.

`WriterEpoch` is a `u64` because M0-02A requires overflow rejection for a monotonic local counter. The width is provisional. It is not a broker fence.

`RiskPolicyVersion`, `NormalizerVersion`, `InstrumentDefinitionVersion`, and `ValuationModelVersion` are opaque printable ASCII tokens, 1 to 128 bytes. CR-B10 does not say whether they are integers or semantic versions. They do not order. `PolicyVersion` in CR-B10 section 3 is not a second Rust type; the section 1 name `RiskPolicyVersion` is used. If those names diverge later, add a type rather than overloading this one.

## UD-05 — Timestamp resolution

`OccurredAt`, `ReceivedAt`, and `PersistedAt` store `i64` microseconds since the Unix epoch. The resolution is a storage choice. Nanosecond values are rejected even when they are exact multiples of 1_000, so the shell does not truncate. RFC 3339 parsing is not implemented. Zero is an instant, not a missing timestamp. Clock quality is not represented. Ordering inside one type orders stored microseconds only. It does not order events or broker messages. Calendar logic is absent.

## UD-06 — State transitions

CR-B10 section 2 gives draft variant names and says the transition matrix is a separate acceptance item. DET-01 was not supplied. M0-02A defines the enums and does not define edges, guards, or events. `RiskReservationAdjusted` is not a variant. `BrokerLifecycleState::Unknown` is a known label; unrecognized text does not become `Unknown`, `Ready`, `Allow`, `Released`, or `ConfirmedAccepted`.

Declaration order is not lifecycle order. The enums do not implement `Ord` or `Default`.

## UD-07 — Types named by CR-B10 and not implemented

`OrderClosure` needs a reason vocabulary that CR-B10 does not list. `IncidentState` is named in section 1 and has no variant list. Neither type was invented. `ExecutionState` is implemented from section 2 even though section 1 has no row for it; the glossary marks that ownership as taken from section 2.

## UD-08 — Canonical JSON

Serde implementations exist so unknown state text and float amounts can be rejected in tests. Field names and enum spellings are draft. They are not the M0-02B canonical envelope, and identical bytes are not promised across serializer versions. Enum labels use the CR-B10 PascalCase variant names. `RECONCILIATION_REQUIRED` is rejected. `Canceled` is rejected; the broker label is `Cancelled`.

## UD-09 — UUID version

CR-B10 proposes UUID version 7 and allows version 4. M0-02A generates version 4, matching the existing workspace dependency. Constructors accept any UUID bits. The version nibble and any timestamp bits are not interpreted. UUID order is not event order.
