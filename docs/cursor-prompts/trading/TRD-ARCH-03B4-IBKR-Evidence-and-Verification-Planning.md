# TRD-ARCH-03B.4 — IBKR Evidence & Verification Planning

This file is the TRD-ARCH-03B.4 instruction executed on 2026-11-15, and no signature artifact or evidence URI is attached to it.

**Mode:** Documentation-only, evidence acquisition and verification *planning* only. **No external execution, no trading behavior.**
**Date:** 2026-11-15 (planning instruction; actual report date must reflect execution date).
**Requested Git commit subject:** `TRD-ARCH-03B.4: Plan IBKR Evidence and Verification`

## 0. Mission and authority

Work inside the existing Rust trading-platform repository, respecting the actual repository layout. Develop a rigorous, reproducible, reviewable plan to obtain and test external Interactive Brokers (IBKR) evidence for broker identity, order/execution alias scope, snapshot completeness, reconnect, crash/unknown-send, corrections/busts, and single-writer failover. Do **not** execute the plan, connect to a broker, send orders, or claim broker behavior verified. The user has authorized only this documentation-only planning milestone.

Existing ratifications ARCH-DEC-01..48 are binding design policies, not implemented or externally verified behavior. In particular ARCH-DEC-35..48 record 13 B choices and one A (BROKERQ-07: no completeness tokens). The earlier decisions ARCH-DEC-03, 11, 13, 14, 17, 26, 28, 29, 31, 33, 34 remain binding. Preserve CR-B01 precedence on unknown sends and external fencing. Never silently supersede prior decisions.

### Absolute prohibitions

- NO broker login, connection, API request, TWS/IB Gateway startup, account access, paper trading, order send, cancel, modify, replace, retry, resubmit, takeover or dispatch.
- NO economic posting, compensating/reversal booking, reservation conversion/shrink/release, economic-finality predicate, OMS transition, terminal acceptance, or live-trading authorization.
- NO canonical identity tuple, uniqueness constraint/index, identity resolver, schema/DDL/migration, production dependency, runtime code, credentials, secrets, CI network calls, or new trading endpoint.
- NO fabricated official IBKR documentation, version guarantee, evidence URL, raw payload, test execution, coverage claim, timestamp, signature, approval, or test result.
- NO change to `contracts/registry/registry.json`, original 12 manifest sources, source manifest, ADFR JSON/CSV, ARCH-DEC-01..48, or historical proposal/resolution indexes. No ADFR verification promotion. No push.

## 1. Mandatory preflight — fail closed

1. Identify repository root, current branch, `git status --short`, `git rev-parse HEAD`, current HEAD subject, tool versions, and relevant architecture paths. If the worktree is dirty, STOP and report; do not discard/stash/reset.
2. Verify predecessor `TRD-ARCH-03B.3.2` report and archived prompt exist, and `contracts/decisions/ARCH-DEC-35-bis-48-2026-11-15.md` is present. Read their actual contents, not assumed descriptions. Verify ARCH-DEC-35..48 choice mapping and their nonimplementation status.
3. Read `contracts/source-manifest.json`, `contracts/registry/registry.json`, ADFR JSON/CSV, source files for DET-03, DET-05, DET-07, DET-08, CR-B01, CR-B04, CR-B07, all relevant unresolved-decisions and traceability entries. Verify the 12 manifest source hashes and byte counts before edits. Confirm the 78 ADFR records, all `NOT_VERIFIED`, and one supersession `ADFR-CROSS-005 -> ADFR-CR-B10-001`.
4. Verify expected registry SHA-256 `7abed2af5363726403175c6524bd7a4d05d36b8735ad48442fc26941f8d98197` if still applicable; if mismatch STOP and explain, do not rewrite registry.
5. Record open blockers accurately: `XC-01`, `XC-08-IBKR`, operational remainder of `XC-04`, `XC-02-PRED`, and related downstream unresolved items. No automatic conflict closure.
6. If preflight finds a conflict or missing source, STOP with a truthful PARTIAL/FAIL report; do not improvise.

## 2. Deliverables — all explicitly NON_RUNTIME, NOT_EXECUTED

Create the following documentation-only files under `contracts/verification-plans/` (or repository's already-established equivalent, explain if different):

1. `TRD-ARCH-03B4-ibkr-evidence-inventory.md` — evidence inventory and claim registry.
2. `TRD-ARCH-03B4-ibkr-verification-protocol.md` — reproducible verification protocol and isolation/safety prerequisites.
3. `TRD-ARCH-03B4-ibkr-test-matrix.md` — positive, negative, fault-injection, ambiguity, and boundary cases.
4. `TRD-ARCH-03B4-ibkr-evidence-capture-and-provenance.md` — proposed future capture format and chain of custody, NOT a canonical runtime/wire schema.
5. `TRD-ARCH-03B4-dependency-and-conflict-matrix.md` — dependency map and evidence-to-decision gates.
6. `TRD-ARCH-03B4-approval-requests.md` — at least 12 independent `IBKREVQ-01...` questions, each with A/B/C alternatives, implications, binding constraints, sources, dependencies, nonbinding recommendation, required evidence and explicit `USER_DECISION_PENDING` / `NOT_RATIFIED`. Mark incompatible alternatives `CONFLICTS_WITH_BINDING_POLICY`; do not ratify anything.
7. `TRD-ARCH-03B4-plan-index.json` — non-runtime/non-wire plan index with safety flags false, evidence statuses pending, execution flags false, and traceable document paths. Do not add it to the canonical schema registry.
8. `docs/architecture/TRD-ARCH-03B.4-report.md` — structured completion report described below.
9. Archive the **exact executed prompt** at `docs/cursor-prompts/trading/TRD-ARCH-03B4-IBKR-Evidence-and-Verification-Planning.md` following repository conventions.

Update only appropriate README, glossary, unresolved-decision and traceability pointers and documentation-only tests needed to make the plan discoverable and enforce nonpromotion. Keep source quotations minimal and precise; preserve original sources byte-for-byte.

## 3. Evidence inventory — separate claims from assumptions

Create individual claim IDs and source anchors, with statuses `HYPOTHESIS`, `EXTERNAL_EVIDENCE_PENDING`, `NOT_EXECUTED`, and explicit nonauthorization. Cover at minimum:

- `orderId`: namespace, client/session scope, reuse, reconnect/restart, order placement context.
- `permId`: absent/null/zero handling, assignment timing, cross-session observations, stability and limits; no standalone uniqueness guarantee.
- `execId`: namespace, duplicates, corrections/busts, original-to-correction references, out-of-order deliveries.
- Order status vs execution observations vs local attempts; no broker terminal status as sole economic authority.
- API request/response scope by account, client, session, dataset, time window, and environment; pagination, end-of-response, request completion, retention/truncation, empty responses, delayed data, and gaps.
- Reconnect, IB Gateway/TWS restart, disconnect during read, missing executions, partial fills, late fills, cancel races, and conflicting snapshots.
- Unknown send before/after `DISPATCH_STARTED`, lost acknowledgement, crash windows, prior writer possibly active, lease expiry, split brain, and session isolation. CR-B01 precedence: no automatic retry or takeover.
- Difference between local `WriterEpoch`, adapter session epoch, broker acceptance, and externally evidenced isolation.
- API/product/version/platform/environment differences, and how future evidence would be revalidated after version change.

For each claim record: claim ID, exact internal source anchor, hypothesis text, proposed official evidence type, exact API operation/topic *to be researched later* (mark as unverified unless evidenced), planned positive and negative tests, environmental prerequisites, known ambiguity, what it would NOT prove, and decision blockers. Do not present conjectured API guarantees as facts.

## 4. Verification protocol and safety barriers

Define a staged future process, **not permission to run it**:

- Stage E0: documentation collection from official IBKR materials and version metadata; future acquisition only, no URLs invented.
- Stage E1: offline fixture and deterministic replay design using synthetic clearly labeled fixtures, no claims of external behavior.
- Stage E2: isolated read-only integration experiments, subject to a **separate explicit user authorization**, credentials/sandbox governance, and verified network isolation.
- Stage E3: controlled broker-connected fault scenarios and any order-related experiment **only after another separate authorization**, dedicated test account, independent safety review and hard stop. No test order in this milestone; paper environment is not automatically approved.
- Stage E4: evidence adjudication and explicit dated ratification; no status promotion merely because tests were written or executed.

Specify prerequisites: exact environment and IBKR API/client version, OS, account mode, clock synchronization, immutable raw capture, correlation IDs, monotonic local timestamps plus wall-clock, transport/connection lifecycle, sanitized secret-free logs, reproducible setup, expected-vs-observed assertions, negative control, retry-safe test harness, crash injection isolation, cleanup, rollback and incident procedure. Do not prescribe a working broker command or connection procedure in this milestone.

The future protocol must define a human stop/go checkpoint for each stage, a proof-obligation owner *role placeholder without creating an operational authorization role*, and conditions under which any unexpected behavior halts verification. Separate capture completeness from broker completeness. Do not infer absence from empty lists.

## 5. Coverage matrix — minimum 36 independent cases

Create at least 36 uniquely identified scenarios across:

A. identity (reused IDs, multiple clients/accounts, absent `permId`, late `permId`, duplicate `execId`, correction/bust, contradictory raw payload, out-of-order evidence);
B. snapshots (empty result, truncated window, pagination break, dropped connection, delayed execution, missing end marker, cross-session/account confusion, stale cache, mismatched scope, different API versions);
C. unknown send/recovery (before `DISPATCH_STARTED`, after it, timeout, lost ACK, crash after send, reconnect, gateway restart, prior writer still active, expired lease, split brain, new epoch, partial fill during disconnect);
D. cross-cutting negative assertions (singleton candidate not authorization, local capture gap not IBKR sequence, no completeness token, no broker-wins rule, no economic effect, no release, no dispatch).

Each row must include scenario ID, claim IDs, internal source anchors, setup prerequisite, future action type (offline/read-only/controlled; NOT EXECUTED), positive/negative expectation, data to capture, oracle limits, failure classification, linked XC finding, explicit safety prohibition, and `NOT_EXECUTED` / `EXTERNAL_EVIDENCE_PENDING` where applicable. No invented execution results.

## 6. Conflict/dependency matrix and ratification questions

Explicitly distinguish design policy from external verification. Show what evidence is needed to consider closing `XC-01` and `XC-08-IBKR` and operational `XC-04`, without actually closing them. `XC-02-PRED` and OMS matrix remain open even if broker evidence arrives. Preserve precedence CR-B01 over conflicting DET-07 text, ARCH-DEC-13/26 over DET-03 single-match booking, and no unapproved compensating ledger line.

Produce at least 12 independent `IBKREVQ` questions on evidence source quality, version pinning, scope taxonomy, test environments, read-only limits, completeness oracle, alias uniqueness verification, correction chains, unknown-send fault injection, writer isolation proof, evidence retention, revalidation cadence, escalation and future ratification gates. Every choice remains pending; avoid suggesting that this planning document grants testing authority. C alternatives may be contrast-only and clearly marked if they conflict with binding policy.

## 7. Regression tests and verification of documentation

Add narrowly scoped tests only under existing test conventions, checking:

- Plan documents and index exist and are explicitly non-runtime/non-wire; all external tests `NOT_EXECUTED`, external evidence pending, no ratification or conflict closure.
- At least 36 distinct scenario IDs and at least 12 distinct pending questions, with source and blocker traceability.
- ARCH-DEC-01..48 unchanged, original 12 manifest sources and hashes unchanged, registry unchanged, 78 ADFR `NOT_VERIFIED`, one supersession, historical proposal indexes unchanged.
- No authorization flags, no `COMPLETE` token ratified, no uniqueness claim, no economic posting/release/finality/OMS/dispatch/live trading.
- Documentation test assertions cannot be misreported as IBKR evidence.

Run from repository root, after edits:

```
cargo fmt --all -- --check
cargo check --workspace --locked
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
```

If formatting fails, apply the smallest targeted correction and rerun. Document exact command outcomes and ignored tests, particularly the PostgreSQL integration test; ignored is NOT passed. Do not run broker tests, remote CI, or network requests. If the repository has existing additional document integrity checks, run them and report results.

## 8. Acceptance gates (12; all mandatory)

- **A03B4-G01** Clean baseline and predecessor verified; no conflicting identifiers or dirty worktree.
- **A03B4-G02** 12/12 manifest sources validated before and after, immutable provenance intact.
- **A03B4-G03** 78 ADFR rows, JSON/CSV parity, `NOT_VERIFIED`, single supersession, registry hash and safety flags preserved.
- **A03B4-G04** ARCH-DEC-01..48 and CR-B01 precedence preserved; no unauthorized decision or status promotion.
- **A03B4-G05** Claim inventory covers all requested identity and broker-alias cases, with explicit uncertainties and no uniqueness assertion.
- **A03B4-G06** Snapshot scope/completeness plan distinguishes local capture from broker proof; no invented token or empty-response guarantee.
- **A03B4-G07** Unknown-send, crash, fencing and failover protocol remains non-authorizing and safe under uncertainty.
- **A03B4-G08** At least 36 scenario rows, each with explicit oracle, evidence requirements and `NOT_EXECUTED`.
- **A03B4-G09** At least 12 independent pending `IBKREVQ` questions, alternatives, traceability and no ratification.
- **A03B4-G10** No production/runtime/schema/DDL/credential/broker/network/trading changes; all blocked actions remain blocked.
- **A03B4-G11** All four local Cargo checks pass; report ignored tests and lack of remote/external verification truthfully.
- **A03B4-G12** Prompt archived, complete report written, exactly ONE local Git commit with requested subject, no push, actual final SHA in Cursor response.

If any mandatory gate fails, verdict `PARTIAL` or `FAIL` with precise reason; never print PASS by assumption.

## 9. Git and completion report

1. Before commit, inspect `git diff --check`, `git diff --stat`, file list and ensure no protected/production files changed. Stage only intended paths. If unexpected changes exist, STOP; do not discard them.
2. Create **exactly one** local commit with subject `TRD-ARCH-03B.4: Plan IBKR Evidence and Verification`. Do not amend prior commits, do not push, do not create a second commit.
3. Print `git rev-parse HEAD`, `git show --stat --oneline HEAD`, and `git status --short` in Cursor's final response. A report file within the same commit need not contain its own SHA; the final Cursor response must provide the real SHA.
4. In `docs/architecture/TRD-ARCH-03B.4-report.md`, include: verdict, date, baseline, authority/scope, deliverable paths and hashes, claim count, scenario count, question count, source integrity and ADFR verification, explicit open XC statuses, nonimplementation prohibitions, test commands and exact outcomes, ignored tests, deviations, all 12 gate results, final commit subject (SHA in Cursor response), `PUSHED: NO`, and next proposed decision step.
5. Do not start TRD-ARCH-03B.4.2, do not ratify `IBKREVQ` alternatives, and do not start IBKR integration.

**STOP CONDITION:** When in doubt, preserve evidence and block action. Documentation is not verification. Broker behavior remains unproven until separately authorized, actually collected and evaluated evidence exists.
