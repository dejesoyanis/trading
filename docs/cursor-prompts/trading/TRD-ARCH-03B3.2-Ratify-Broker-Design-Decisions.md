# TRD-ARCH-03B.3.2 — Ratify Broker Design Decisions

**Datum der Benutzerentscheidung:** 2026-11-15  
**Modus:** Dokumentation, Traceability, Dokumentations-/Regressionstests; **keine Runtime-Implementierung**.  
**Commit-Titel:** `TRD-ARCH-03B.3.2: Ratify Broker Design Decisions`  
**Git:** Genau ein lokaler Commit, **kein Push**.

This file is the TRD-ARCH-03B.3.2 instruction executed on 2026-11-15. It is the textual decision record. No signature artifact and no evidence URI are attached to it.

## 0. Verbindlicher Auftrag und Autorität

Der Benutzer hat am 2026-11-15 **ausdrücklich und verbindlich** die untenstehenden 14 Alternativen zu `TRD-ARCH-03B.3` bestätigt. Diese Benutzerentscheidung ist die einzige Autorität für die neue Ratifikation. Die Empfehlungen des historischen Vorschlags sind keine Autorität. Archivierung dieses vollständig ausgeführten Prompts unter dem im Repository etablierten Pfad `docs/cursor-prompts/trading/TRD-ARCH-03B3.2-Ratify-Broker-Design-Decisions.md` ist erforderlich; der Prompt ist selbst der textuelle Entscheidungsnachweis, **keine erfundene Unterschrift, kein erfundener Evidence-URI**.

**Ziel:** 14 datierte, nachweisbar rückverfolgbare **Design-Policy-Ratifikationen**, voraussichtlich `ARCH-DEC-35` bis `ARCH-DEC-48`, **ausschliesslich wenn alle Kennungen frei sind**. Sonst **STOP** und Bericht mit Kollision, nicht umnummerieren und nicht überschreiben. Neue Entscheidungen tragen explizit `RATIFIED_DESIGN_POLICY_ONLY`, `NOT_IMPLEMENTED`, `NOT_VERIFIED`, `NON_RUNTIME_NON_WIRE`. Diese Bezeichnungen sind **keine neue kanonische Status-Enum**.

## 1. Unverhandelbare Sicherheitsinvarianten

- **KEINE** Broker-Identitäts-Eindeutigkeitsgarantie, kein globaler Alias-Schlüssel, keine kanonische Broker-Identity-Tuple, kein UNIQUE-Index, kein Identity Resolver, kein kanonisches Schema, keine DDL oder Migration.
- **KEINE** wirtschaftliche Buchung, kein wirtschaftlicher Korrektur-/Kompensationsposten, keine Reservation-to-Exposure-Conversion, **kein Economic-Finality-Prädikat**, **kein Risk Release**, keine Reduktion bestehender Reservierungen.
- **KEINE** OMS-Transition, keine OrderClosure-Freigabe, kein terminaler Brokerstatus als alleinige Autorität, kein Cancel/Modify/Replace, keine Broker-Aktion, **kein Broker-Dispatch**, kein Retry/Resubmit, kein automatischer Takeover, kein Live-Trading und keine Paper-Broker-Requests.
- **KEINE** neue Operatorrolle, kein Adjudication- oder Reset-Recht, kein Dual-Approval-Ersatz, kein Lock-Order-/Isolation-Level-Entscheid, keine TTL-/Heartbeat-Freigabe.
- **KEINE** behauptete IBKR-Garantie zu `orderId`, `permId`, `execId`, Sequenz, Session-Lebensdauer, Response-Vollständigkeit oder Timeout. Keine Dokumentationstests als externe Broker-Evidenz ausgeben.
- Vorherige `ARCH-DEC-01..34` bleiben unverändert verbindlich; insbesondere `ARCH-DEC-03`, `ARCH-DEC-13`, `ARCH-DEC-14`, `ARCH-DEC-17`, `ARCH-DEC-26`, `ARCH-DEC-33`, `ARCH-DEC-34` und CR-B01-Dispatch-/Failover-Präzedenz.
- `XC-01`, `XC-08-IBKR`, `XC-04` operativer Rest und `XC-02-PRED` bleiben **OPEN**. Keine stillschweigende Konfliktauflösung, keine Evidence-Promotion.

## 2. Preflight: Repository zuerst lesen

1. Bestimme Repo-Root über `git rev-parse --show-toplevel`; erfasse Branch, `HEAD` (SHA und Subject), `git status --short`, Versionen `rustc`, `cargo`, `git`. **Sauberer Worktree ist Pflicht**; bei Fremdänderungen STOP, keine Stashes, Resets, Clean oder destructive Git.
2. Verifiziere Vorgänger `TRD-ARCH-03B.3` im letzten Commit und dessen Bericht `docs/architecture/TRD-ARCH-03B.3-report.md`; prüfe archivierten Prompt. Lese insbesondere die **tatsächlichen** Dateien:
   - `contracts/design-proposals/TRD-ARCH-03B3-approval-requests.md` (BROKERQ-01..14; A/B/C-Originaltext)
   - `contracts/design-proposals/TRD-ARCH-03B3-broker-identity-semantics-and-invariants.md`
   - `contracts/design-proposals/TRD-ARCH-03B3-identity-snapshot-and-recovery-options.md`
   - `contracts/design-proposals/TRD-ARCH-03B3-snapshot-completeness-and-reconciliation-coverage-draft.md`
   - `contracts/design-proposals/TRD-ARCH-03B3-dependency-and-conflict-matrix.md`
   - `contracts/design-proposals/TRD-ARCH-03B3-proposal-index.json`
   - `contracts/decisions/` (ARCH-DEC-01..34), `contracts/source-manifest.json`, `contracts/registry/registry.json`, ADFR JSON und CSV, `contracts/traceability-index.md`, `contracts/domain/unresolved-decisions.md`, `contracts/domain/glossary.md`.
3. Prüfe, dass **alle** `ARCH-DEC-35..48` unbenutzt sind, auch in Indexen und Registern. Falls nicht: STOP mit präziser Kollisionsliste, keine Ersatznummern.
4. Verifiziere alle 12 manifestierten Quellartefakte anhand `sha256` **und** `bytes` vor und nach der Arbeit. Die Manifest-/Intake-Felder dürfen nicht geändert werden. DET-01/02 bleiben `reconstructed_draft`, `SOURCE_REVIEW_PENDING`, `reviewed=false`, `approved=false`.
5. Prüfe die Baseline: Registry SHA-256 `7abed2af5363726403175c6524bd7a4d05d36b8735ad48442fc26941f8d98197` (44332 Bytes), `SKELETON_NOT_CANONICAL`, `canonical_schemas_published=false`, `part_b=NO-GO`, `live_trading_authorized=false`, `broker_uniqueness_evidence=EVIDENCE_PENDING`. Wenn Baseline abweicht: untersuchen und dokumentieren; bei unerklärtem Widerspruch STOP.
6. ADFR: 78 JSON-Einträge, 78 CSV-Einträge, gleiche eindeutige IDs/Reihenfolge, 78 × `NOT_VERIFIED`; genau eine Supersession `ADFR-CROSS-005 -> ADFR-CR-B10-001`; keine Änderung der Dateien. Alte Entscheidungsdateien `ARCH-DEC-01..34`, historische Proposal-/Resolution-Indizes und ihre Hashes dürfen sich nicht ändern.

## 3. Exakt ratifizierte Benutzerentscheidungen (keine Substitution)

| Neue Entscheidung (nur falls frei) | Frage | Option | Verbindliche Design-Policy und ausdrückliche Grenze |
| --- | --- | --- | --- |
| ARCH-DEC-35 | BROKERQ-01 | **B** | Vorhandene Broker-/Konto-/Umgebungs-/Session-/Aliasfelder, Client und Adapter-Epoch sowie Evidence-ID **nur soweit tatsächlich geliefert** als mögliche zukünftige Evidenzkomponenten erfassen. Fehlend bleibt fehlend. Feldgleichheit ist weder Merge noch Uniqueness; kein Schema. |
| ARCH-DEC-36 | BROKERQ-02 | **B** | Zukünftige Kandidatenmenge `A(e)` und Unsicherheitslabel erlauben; selbst `|A(e)|=1` ist `SINGLE_CANDIDATE_NOT_AUTHORIZATION`; 0/mehrere/Widerspruch ebenso nicht autorisierend. Keine automatische Zuordnung mit wirtschaftlicher Wirkung. |
| ARCH-DEC-37 | BROKERQ-03 | **B** | Beobachtung auch ohne `permId` behalten; fehlend/null/zero nicht stillschweigend gleichsetzen. Spätere `permId` als **neue** Evidenz, kein Overwrite. Null-/Zero-Semantik bleibt extern ungeprüft. |
| ARCH-DEC-38 | BROKERQ-04 | **B** | Identischer Rohpayload bei Wiederholung idempotent als erneut gesehen; abweichender Payload unter scheinbar gleichem Schlüssel `CONTRADICTION`, beide Originale erhalten. Keine zweite Ausführung/Buchung, keine Bytes löschen. |
| ARCH-DEC-39 | BROKERQ-05 | **B** | Correction/Bust append-only mit Original-Evidenz verknüpfen, **nur wenn die neue Evidenz selbst einen Bezug liefert**. Kein erfundener Link, keine Reversal-Buchung, kein Release, keine Closure-Änderung. |
| ARCH-DEC-40 | BROKERQ-06 | **B** | Angefragten Account-/Client-/Session-/Dataset-/Zeitfenster-Scope soweit vorhanden dokumentieren; Coverage bleibt `UNKNOWN`, nicht global, keine `COMPLETE`-Behauptung. |
| ARCH-DEC-41 | BROKERQ-07 | **A** | **Keine Completeness-Tokens** speichern. Kein `COMPLETE`-Marker und kein externer Token-Platzhalter als Ratifikation; leere Antwort ist kein Nicht-Existenz-/Finalitäts-/Release-Beweis. |
| ARCH-DEC-42 | BROKERQ-08 | **B** | Nur tatsächlich lokal beobachtete Capture-Gaps (z.B. kurze Seite, fehlendes Antwortende, Drop) markieren; kein erfundener IBKR-Sequenzzähler, keine Interpolation, Scope bleibt unvollständig, keine Folgeaktion. |
| ARCH-DEC-43 | BROKERQ-09 | **B** | Reconnect erzeugt neuen Session-Kontext; alte Evidenz behalten, IDs nicht ohne nachgewiesene Bindung über Session-Grenzen übernehmen. Reconnect ist kein Send-Negativbeweis. |
| ARCH-DEC-44 | BROKERQ-10 | **B** | CR-B01-Crash-/Dispatch-Fenster als Labels dokumentieren. Nach `DISPATCH_STARTED` möglicherweise gesendet; **kein** Fenster dieses Pakets erlaubt Resend, Cancel, Release oder Dispatch. |
| ARCH-DEC-45 | BROKERQ-11 | **B** | Lokale Fence-Fakten (Epoch, Lease-Deadline, Authority-Label) von Broker-Akzeptanz strikt trennen; Successor blockiert solange früherer Writer noch senden könnte oder Sendestatus unbekannt ist. Lease-Ablauf/neuere Epoch beweist kein externes Fencing. Keine Implementierung oder TTL. |
| ARCH-DEC-46 | BROKERQ-12 | **B** | Nur nicht-autorisierenden Eskalationsmarker für spätere menschliche Prüfung vorsehen; **keine** Rolle, kein Post-/Release-/Dispatch-/Reset-Recht. Fehlende erforderliche zweite Person bedeutet Deny, nicht simulierte Zustimmung. |
| ARCH-DEC-47 | BROKERQ-13 | **B** | Broker-Rohbeobachtung und lokale Attempt-Fakten bei Widerspruch beide erhalten; Vergleich als Inferenz kennzeichnen. Keine Seite gewinnt pauschal, kein wirtschaftlicher Schluss, kein Cancel. |
| ARCH-DEC-48 | BROKERQ-14 | **B** | Für spätere Verifikation tatsächlich beschaffte offizielle Unterlagen, reproduzierbare positive/negative Fälle, Umgebung, Brokerprodukt/API-Version, Zeitstempel und Rohpayloads verlangen. **Heute alles `NOT_EXECUTED`/`EXTERNAL_EVIDENCE_PENDING`**; Dokumentations-/Unit-Tests sind keine Broker-Evidenz. |

**Kontrollsumme der Optionen:** `B B B B B B A B B B B B B B` = **13×B, 1×A**. 14 unabhängige Ratifikationen, keine impliziten C-Alternativen. C bleibt `CONFLICTS_WITH_BINDING_POLICY`. Bei BROKERQ-13/14 auch die problematischen A-Alternativen **nicht** als alternative Autorität übernehmen.

## 4. Artefakte und Traceability

Erzeuge nur nach erfolgreichem Preflight:

1. `contracts/decisions/ARCH-DEC-35-bis-48-2026-11-15.md` (falls Repositorykonvention kompatibel; andernfalls entsprechend vorhandener Konvention, mit Begründung). Pro Entscheidung: ID, Datum, exakte Frage und bestätigte Option, unveränderte Benutzerformulierung, aus Originalquelle abgeleitete präzise Semantik, erlaubte **zukünftige** Designobligation, explizite Nicht-Autorisierungen, Referenzen auf BROKERQ-Abschnitt, ursprüngliche Quellen/Abschnitte, Vorgängerentscheidungen, Konflikte, Evidenz-/Verifikationsstatus, offene Abhängigkeiten. Keine Signatur erfinden.
2. Eigenständigen maschinenlesbaren `contracts/design-proposals/TRD-ARCH-03B32-resolution-index.json` als `NON_RUNTIME_NON_WIRE_RESOLUTION_INDEX` erstellen; getrennt vom historischen `TRD-ARCH-03B3-proposal-index.json` und von `registry.json`. 14 exakt referenzierte Einträge; Benutzerentscheid als Authority, `evidence_uri=null`, `signature_artifact=null`, `tests_executed=false` **für externe Broker-/Coverage-Tests**. Dokumentations-Regressionstests getrennt davon berichten. Flags für Implementation, uniqueness, completeness proof, schema, booking, release, predicate, OMS, broker dispatch, live trading, conflict closure alle false.
3. `TRD-ARCH-03B3-approval-requests.md` **historischen Text byte-identisch erhalten**, allenfalls einen klar getrennten Ratifikations-Anhang mit Pointer auf Entscheidungen und Resolution-Index ergänzen. Vorherigen Dateihash und Hash des unveränderten Original-Präfixes erfassen. Niemals Empfehlungen nachträglich in Bestätigungen umschreiben.
4. Nur erforderliche Pointer/Statushinweise in `contracts/design-proposals/README.md`, `contracts/traceability-index.md`, `contracts/domain/unresolved-decisions.md`, `contracts/domain/glossary.md`, `docs/architecture/README.md`, `tests/README.md` aktualisieren. Historische Statusaussagen mit Zeitbezug erhalten statt stillschweigend umdeuten.
5. Kein Edit an `contracts/registry/registry.json`, `contracts/source-manifest.json`, 12 Source-Dateien, ADFR JSON/CSV, alten Entscheidungsdateien oder alten Proposal-/Resolution-Indizes.

**Explizite Konfliktmatrix:** `XC-01` OPEN / `EVIDENCE_PENDING`; `XC-08-IBKR` OPEN / `EXTERNAL_EVIDENCE_PENDING`; `XC-04` Design-Präzedenz durch ARCH-DEC-03, operativer Rest OPEN; `XC-02-PRED` OPEN; `XC-03-MATRIX`, `XC-05-LOCK`, `XC-07-ROUND`, `XC-09-RESET`, `XC-10-ACTION`, `XC-11` nicht versehentlich schliessen. Quellkonflikt DET-03-D04/CR-B04 §4.3 (single match books) bleibt gegenüber ARCH-DEC-13/26 untergeordnet; CR-B01 ist bei Send-/Failover-Konflikten vorrangig. `DESIGN_RESOLVED` ist kein Verifikationsbeweis.

## 5. Regressionstests und Beweisgrenzen

Ergänze **nur Dokumentations-/Policy-Regressionstests** in bestehenden passenden Testdateien (z.B. `crates/trading-domain/tests/adfr_reconciliation.rs`, `registry_skeleton.rs`) unter Einhaltung vorhandener Testkonventionen. Tests sollen maschinenlesbar und negativ absichern:

- 14 exakt eindeutige Frage-zu-Entscheid-ID-Mappings und Optionen; `BROKERQ-07=A`, alle anderen B; alle mit `RATIFIED_DESIGN_POLICY_ONLY`, `NOT_IMPLEMENTED`, `NOT_VERIFIED`.
- Fehlende `permId` erzeugt keinen impliziten Zero-Match; Singleton-Kandidat erzeugt keine Autorisierung; identischer Replay keine wirtschaftliche Wirkung; Widerspruch und Correction behalten Original-Evidenz; keine automatisch autorisierte Link-Inferenz.
- Snapshot-Scope bleibt `UNKNOWN`, keine Completeness-Tokens; lokale Gaps sind keine IBKR-Sequenz; leere Response kein Negativbeweis.
- Reconnect trennt Sessions; unknown send und lease expiry erlauben **keinen** Resubmit/Takeover; Fence ist nicht Broker-Akzeptanz; Eskalation ist keine Rolle; keine Broker-vs-DB-Priorität.
- BROKERQ-14 erfordert später externe positive **und negative** Evidenz, die aktuell **nicht vorhanden und nicht ausgeführt** ist.
- Registry-Skeleton bleibt unpubliziert; neuen nichtkanonischen JSON-Index nur bei Bedarf in die bestehende Test-Allowlist aufnehmen, niemals als Schema registrieren.
- 12/12 Source-Hashes/Bytes unverändert, ADFR 78/78 und `NOT_VERIFIED`, genau eine Supersession, frühere Entscheidungen/Indizes unverändert.
- Keine Produktionsdatei unter `crates/*/src` oder `apps/*/src`, kein `Cargo.toml`/`Cargo.lock`/Migration/Schema geändert. Dokumentationstests **beweisen nicht** Brokerverhalten, Reconciliation, Crash-Freiheit oder wirtschaftliche Sicherheit.

Führe im Repo-Root aus und protokolliere Exitcodes:

```powershell
cargo fmt --all -- --check
cargo check --workspace --locked
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
```

Falls Formatierung nötig ist, nur betroffene Testdateien formatieren, ursprünglichen Fehlschlag und erneuten erfolgreichen Check dokumentieren. PostgreSQL-Integrationstests nur mit expliziter sicherer Test-DB-Konfiguration; sonst als `IGNORED/NOT_EXECUTED`, **nicht** PASS. Remote CI nicht behaupten. Keine externen Broker-Requests ausführen.

## 6. Acceptance-Gates (12/12 erforderlich)

- **A03B32-G01 – Preflight:** sauberer Git-Stand, Vorgänger, 14 freie IDs, Versions-/Baseline-Evidenz.
- **A03B32-G02 – Authority:** exakte 13B/1A-Benutzerentscheidung, Datum und Herkunft, keine erfundene Signatur.
- **A03B32-G03 – Immutable history:** historische BROKERQ-Fragen, Quellen und Proposal-Index unverändert; Ratifikation separat.
- **A03B32-G04 – Provenance:** 12/12 Source-Dateien, Registry, 78 ADFR und ARCH-DEC-01..34 geschützt.
- **A03B32-G05 – Identity:** BROKERQ-01..05 exakt, keine UNIQUE-/Merge-/Posting-Garantie.
- **A03B32-G06 – Snapshot:** BROKERQ-06..08 exakt, UNKNOWN, **kein Token**, keine falsche Sequenz.
- **A03B32-G07 – Recovery:** BROKERQ-09..11 exakt, kein Retry, keine Broker-Fencing-Behauptung.
- **A03B32-G08 – Escalation/Evidence:** BROKERQ-12..14 exakt, keine Rolle, keine Priorität, externe Tests pending.
- **A03B32-G09 – Conflicts:** XC-01, XC-08-IBKR, XC-04 Rest, XC-02-PRED OPEN; CR-B01-Präzedenz.
- **A03B32-G10 – Non-implementation:** keine Runtime, Schema, Broker, Posting, Release, OMS, Live-Trading-Änderung.
- **A03B32-G11 – Regression:** alle vier Cargo-Checks PASS, dokumentierte Testzählung, Ignored/CI ehrlich.
- **A03B32-G12 – Report/Commit:** archivierter Prompt, datierter Abschlussbericht, **genau ein lokaler Commit**, **kein Push**, Commit-SHA aus tatsächlichem `git rev-parse HEAD` im Cursor-Endoutput, Worktree danach sauber.

Ein Gate-Fehler bedeutet `PARTIAL` oder `FAIL` und **keinen** erfundenen PASS. Bei Blockern STOP; keine unsicheren Workarounds. Die Berichtdatei muss im einen Commit enthalten sein.

## 7. Abschlussbericht und genau ein lokaler Commit

Schreibe `docs/architecture/TRD-ARCH-03B.3.2-report.md` mit:

1. `VERDICT: PASS|PARTIAL|FAIL`, Datum, Dokumentationsmodus, **kein** Live-Trading.
2. Git-Baseline SHA/Branch/Clean-Status, tatsächliche Änderungsliste, SHA-256 und Bytes der neuen Artefakte, 14er Mapping, Status aller Entscheidungen.
3. Quellen-/ADFR-/Registry-Integrität mit Vorher/Nachher-Prüfung, offenen XC-Konflikten und fehlenden externen Nachweisen.
4. Sicherheits-Nichtimplementierung, dokumentierte Abweichungen, alle vier Cargo-Exitcodes, Testzahlen inklusive ignorierter Tests, Remote-CI-Status.
5. Zwölf Acceptance-Gates mit expliziter Evidenz und Ergebnis.
6. Commit-Titel, `PUSHED: NO`. Da der Bericht Teil des Commits ist, **nicht** versuchen, den endgültigen SHA des ihn enthaltenden Commits in denselben Bericht hineinzuschreiben. Den tatsächlichen SHA nach dem Commit in der Cursor-Endausgabe nennen.

**Git-Protokoll:** Alle beabsichtigten Dateien inklusive Prompt-Archiv, Tests und Bericht einmalig stagen, `git diff --cached --check` und staged Diff kontrollieren, dann **genau ein** `git commit -m "TRD-ARCH-03B.3.2: Ratify Broker Design Decisions"`. Anschliessend `git rev-parse HEAD`, `git status --short`, `git log -1 --format=%s` ausgeben. **Niemals `git push` ausführen.** Keine zusätzlichen Korrektur-Commits; nötige Korrekturen vor dem Commit durchführen.

## 8. Stop-Kriterien

STOP bei belegter ID-Kollision, unsauberem Baseline-Worktree, unerklärter Source-/Registry-/ADFR-Abweichung, Widerspruch zu ARCH-DEC-01..34, Notwendigkeit einer Runtime-Änderung, nicht ratifizierter neuer Alternative, unerlaubter Schema-/Broker-/Finanzaktion oder fehlender Nachvollziehbarkeit. In diesem Fall `PARTIAL`/`FAIL` dokumentieren und **nicht** stillschweigend ratifizieren.

**Arbeite erst nach vollständigem Preflight. Keine Implementierungsfreigabe. Keine wirtschaftliche oder Broker-Aktion.**
