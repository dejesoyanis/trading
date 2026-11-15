# ADFR consolidated register

**Register revision:** `1.0+ARCH-DEC-02-2026-11-15`  
**JSON root status:** `ARCHITECTURE_DRAFT_NOT_IMPLEMENTED`  
**Live trading:** NOT AUTHORIZED  
**Decision:** [ARCH-DEC-01-bis-03-2026-11-15.md](../decisions/ARCH-DEC-01-bis-03-2026-11-15.md)

`ADFR-Consolidated-v1.0.json` and `ADFR-Consolidated-v1.0.csv` are the register files. TRD-ARCH-SYNC-01 copied them from the source package. On 2026-11-15, ARCH-DEC-02 amended those two filenames in place. The package bytes are still identified by `package_sha256` in [source-manifest.json](../source-manifest.json). Current `sha256` is the amended file.

## Why the v1.0 names were amended in place

The reconciliation had to put the same 78 historical IDs in both existing files. A second v1.1 pair would have left the v1.0 JSON at 77 IDs. The six core columns stay in their previous order. Three lifecycle columns are appended. That is a format change for a reader that requires exactly six CSV columns. Readers that use the header and ignore unknown columns still see the same core fields. `schema_version` stays `1.0` for those core fields. `register_revision` names the amendment.

## Counts

| Count | Meaning |
| --- | --- |
| 78 | Historical IDs in JSON and in CSV. Same set, once each. |
| 1 | Superseded historical ID: `ADFR-CROSS-005`. |
| 77 | Not superseded. This is a count. It is not acceptance, implementation, or verification. |

No historical ID was deleted or renamed. `ADFR-CR-B10-001` was already the CSV row and the CR-B10 §9 entry. The amendment copies that row into the JSON. It does not invent a decision.

## Fields

Core fields, unchanged in meaning, and identical between JSON and CSV:

| Field | Role |
| --- | --- |
| `id` | Historical register ID. |
| `decision` | Decision text retained from the pre-amendment register. |
| `source` | Source citation retained from that register. |
| `status` | Decision-status word retained from that register. Not a lifecycle. |
| `basis` | Basis token retained from that register. |
| `verification` | Evidence word. Every current value is `NOT_VERIFIED`. |

Lifecycle fields, added by the amendment. They are not verification and they do not replace `status`.

| Field | Values |
| --- | --- |
| `lifecycle` | `NOT_SUPERSEDED` or `SUPERSEDED`. `NOT_SUPERSEDED` does not mean accepted, implemented, or verified. `SUPERSEDED` does not overwrite `verification`. |
| `supersedes` | Set only on `ADFR-CR-B10-001`, to `ADFR-CROSS-005`. Empty otherwise. |
| `superseded_by` | Set only on `ADFR-CROSS-005`, to `ADFR-CR-B10-001`. Empty otherwise. |

`ADFR-CROSS-005` keeps `status` `PROPOSED_FOR_ACCEPTANCE` and `verification` `NOT_VERIFIED`. Its lifecycle is `SUPERSEDED`.

`ADFR-CR-B10-001` keeps `status` `DESIGN_RESOLVED` and `verification` `NOT_VERIFIED`. CR-B10 §9 also says `TESTS_NOT_EXECUTED`. B10-T01 through B10-T12 are not passed. `DESIGN_RESOLVED` is not a test pass. Its lifecycle is `NOT_SUPERSEDED`, and `supersedes` is `ADFR-CROSS-005`.

The twenty `CONFIRMED_DECISION` rows still cite `conversation: confirmed DET-01/02 Q01-Q10`. ARCH-DEC-01 does not rewrite that provenance and does not mark those rows verified.

## Order and encoding

Both files use the pre-amendment CSV order: `ADFR-CR-B10-001`, then the previous JSON order. JSON is UTF-8, LF, two-space indent. CSV is UTF-8, LF, with RFC-style quoting when a field contains a comma. LF matches `.gitattributes` (`* text=auto eol=lf`), so the manifest hash is the checkout used by Ubuntu CI. Empty lifecycle references are empty CSV fields and JSON `null`.

## What this register is not

Parsing these files does not publish a schema, authorize live trading, or show that a decision was implemented. Embedded ADFR tables inside other source files are those files' own tables. They are not a second accepted register. The status-word differences among `OPEN P0`, `PROPOSED_FOR_ACCEPTANCE`, and `DESIGN_RESOLVED` in the older source tables are not reconciled by this amendment.
