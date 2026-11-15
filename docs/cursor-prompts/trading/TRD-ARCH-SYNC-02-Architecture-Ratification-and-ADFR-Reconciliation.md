# TRD-ARCH-SYNC-02 — Architecture Ratification, ADFR Reconciliation & Dispatch Precedence

**Typ:** Cursor-Ausführungsauftrag, ausschließlich Dokumentation, Registerkonsistenz und Tests für Dokumentationsartefakte  
**Freigabe:** Nutzerentscheidung vom 2026-11-15 (ARCH-DEC-01/02/03)  
**Abhängigkeit:** TRD-ARCH-SYNC-01 abgeschlossen und als Quellenintegration geprüft  
**Strikte Grenze:** KEINE Trading-Implementierung; KEINE Freigabe für Broker-Dispatch, Live-Trading, kanonische Wire-Schemas oder Migrationen.

## 0. Auftrag und unveränderliche Entscheidungen

Setze ausschließlich die folgenden explizit bestätigten Architekturentscheidungen als versionierte Dokumentationsänderungen um:

- **ARCH-DEC-01:** Die rekonstruierten `DET-01-Order-Workflow-and-Finality-Contract-v1.0.md` und `DET-02-Risk-Contract-v1.0.md` werden als **verbindliche Design-Baseline** akzeptiert, **mit Ausnahme sämtlicher darin als offen, nicht definiert, unbestätigt oder verifikationspflichtig gekennzeichneten Details**. Keine Behauptung, die Original-Q&A-Transkripte seien vorhanden oder unabhängig verifiziert. Baseline-Ratifizierung ist keine Freigabe von Transition-Matrizen, OrderClosure-Gründen, wirtschaftlicher Finalität, Risk-Buchungen oder technischer Ausführung.
- **ARCH-DEC-02:** Das ADFR führt **78 historische IDs**, davon genau **eine abgelöste (superseded)**: `ADFR-CROSS-005`. `ADFR-CR-B10-001` ist die Nachfolgeentscheidung. JSON und CSV müssen dieselben 78 IDs, dieselben fachlichen Inhalte und denselben Supersession-Bezug enthalten. Keine historische ID löschen oder umbenennen. 77 aktive (= nicht superseded) historische Einträge ist eine **Zählableitung**, keine Aussage, dass 77 Entscheidungen implementiert oder verifiziert wären.
- **ARCH-DEC-03:** Für das Dispatch-/Failover-Design hat **CR-B01 Vorrang vor widersprechenden oder weniger restriktiven Aussagen aus DET-07**. Kein automatischer Takeover, solange der bisherige Writer noch senden könnte oder Submission-Ergebnisse ungeklärt sind. Ein Zeitablauf/Lease-Timeout allein ist kein Nachweis sicherer Übernahme. Dies ist **Design-Vorrang**, kein implementiertes Fencing und kein nachgewiesenes IBKR-Verhalten.

Alle übrigen Befunde aus TRD-ARCH-SYNC-01 (XC-01 bis XC-11) bleiben **OPEN**, sofern sie nicht ausschließlich durch eine der drei Nutzerentscheidungen dokumentarisch geklärt sind. **Keine weiteren fachlichen Entscheidungen ableiten.**

## 1. Vorbedingungen und Inventur (Fail-Closed)

1. Git-Branch, HEAD, Status und bestehenden TRD-ARCH-SYNC-01-Bericht protokollieren. Bei uncommitteten fremden Änderungen stoppen; nichts verwerfen, überschreiben oder staschen.
2. Quellen unter `contracts/sources/` und `contracts/source-manifest.json` auf Existenz, Integrität und Provenienz prüfen. Alle vorhandenen Hashes gegen den letzten Manifeststand verifizieren. Fehlt ein benötigtes Dokument oder weicht ein Hash unerklärt ab: **STOP / BLOCKED**, keine stille Rekonstruktion.
3. Die exakten Befunde XC-03-MATRIX, XC-04 und XC-11 sowie alle Verweise auf DET-01/02 und ADFR-CROSS-005 im letzten Bericht und den Vertragsdokumenten lesen.
4. Prüfen, ob der bestehende ADFR-JSON-Root `ARCHITECTURE_DRAFT_NOT_IMPLEMENTED` lautet und das Registry-Skeleton `SKELETON_NOT_CANONICAL`, `canonical_schemas_published=false`, `part_b=NO-GO`, `live_trading_authorized=false`, `broker_uniqueness_evidence=EVIDENCE_PENDING` enthält. Bei Abweichung STOP.

## 2. Dokumentierte Design-Baseline DET-01 / DET-02

- Einen versionierten Ratifikationsnachweis anlegen, bevorzugt `contracts/decisions/ARCH-DEC-01-bis-03-2026-11-15.md` (Verzeichnis bei Bedarf nur für Dokumentation erstellen). Exakten Nutzerbeschluss, Datum, Quellen, Wirkung, Grenzen, noch offene Fragen und Abnahmestatus festhalten.
- DET-01/02 **nicht rückwirkend als Original-Q&A** kennzeichnen. `RECONSTRUCTED_DRAFT` als Provenienz erhalten; zusätzlich eindeutig `DESIGN_BASELINE_RATIFIED_WITH_EXCEPTIONS` oder ein ebenso klar dokumentiertes, nicht mit Teststatus verwechselbares Feld verwenden.
- Im Traceability-Index zwischen `source_present`, `provenance`, `design_baseline_approved`, `details_open`, `implementation`, `verification` unterscheiden. Keine der bestehenden offenen Details durch die Baseline-Ratifizierung als gelöst deklarieren.
- Wenn eine Statusanpassung direkt in den DET-Quelldateien erforderlich ist, die Änderung transparent als datierten Ratifikationsvermerk vornehmen, niemals den ursprünglichen Entwurfstext still überschreiben. Alle geänderten Quellhashes im Manifest korrekt nachziehen. Bevorzugt: unveränderte historische Quellen + separates Ratifikationsdokument.

## 3. ADFR-JSON und -CSV konsistent auf 78 historische IDs bringen

Aktueller Befund laut TRD-ARCH-SYNC-01: JSON 77 IDs, CSV 78 IDs; CSV-only `ADFR-CR-B10-001`; `ADFR-CROSS-005` historisch vorhanden; CR-B10 §9 beschreibt die Ablösung.

1. Originale JSON- und CSV-Strukturen vollständig einlesen, nicht aus dem Bericht rekonstruieren.
2. `ADFR-CR-B10-001` anhand der **bestehenden CSV-Zeile und CR-B10 §9** in JSON aufnehmen. Bestehende sechs Kernfelder (`id`, `decision`, `source`, `status`, `basis`, `verification`) inhaltlich deckungsgleich halten; zusätzliche Felder für `supersedes` / `superseded_by` / `lifecycle` nur nach dokumentierter, einheitlicher Konvention ergänzen.
3. `ADFR-CROSS-005` in **beiden** Formaten ausdrücklich als `SUPERSEDED` (Lebenszyklus, nicht Verifikationsstatus) kennzeichnen und auf `ADFR-CR-B10-001` verweisen. Den bisherigen fachlichen Entscheidungstext und die ursprüngliche Herkunft erhalten. Die Nachfolgeentscheidung verweist auf die abgelöste ID. Falls die vorhandenen Statusfelder fachliche Entscheidung und Lifecycle vermischen, ein getrenntes Feld ergänzen, statt `NOT_VERIFIED` zu überschreiben.
4. Alle 78 historischen IDs müssen in beiden Formaten exakt einmal vorkommen. Genau eine historische ID ist superseded; die anderen 77 bleiben nicht-superseded. Das bedeutet **nicht**, dass alle 77 akzeptiert oder verifiziert sind. Die 20 `CONFIRMED_DECISION`-Einträge zu DET-01/02 nicht pauschal umdeuten; ihre Provenienz und die neue Baseline-Freigabe getrennt dokumentieren.
5. JSON-Rootstatus `ARCHITECTURE_DRAFT_NOT_IMPLEMENTED` und alle `verification=NOT_VERIFIED` erhalten, sofern der tatsächliche Quellenstand nicht ausdrücklich eine andere, belegte Verifikation enthält. `DESIGN_RESOLVED` ist kein Test-PASS.
6. Stabile Reihenfolge der IDs und deterministische Serialisierung wählen; UTF-8, CSV-Quoting und Zeilenumbrüche korrekt behandeln. Keine Einträge löschen, zusammenführen oder erfunden ergänzen.
7. `Architecture-Consolidation-and-Acceptance-v1.0.md`, Traceability, ADFR-README und weitere **direkt betroffene** Dokumentationsstellen mit präzisen Zahlen aktualisieren: 78 historische, 1 superseded, 77 nicht-superseded; JSON/CSV inhaltlich synchron. Historische Aussagen als historische Momentaufnahme belassen und klar datieren.

## 4. Dispatch-Precedence CR-B01 > DET-07

- ARCH-DEC-03 als explizite, datierte **Design-Precedence-Regel** im Ratifikationsnachweis und Traceability-Index dokumentieren. Präzise Quelle: `P0-Resolution-Contracts-CR-B01-bis-CR-B04-v1.0.md` CR-B01 gegenüber `DET-03-bis-DET-08-Architecture-Contracts-v1.0.md` DET-07.
- `XC-04` nicht insgesamt schließen. Nur den Teilbefund „welcher Vertragslesart gebührt Vorrang?“ als `RESOLVED_BY_ARCH_DEC_03` markieren. **Weiter offen:** tatsächliche Broker-Session-Exklusivität, Lease-/Heartbeat-Protokoll, Dispatch-Grenze, In-flight-Send-Nachweise, Recovery- und IBKR-Tests.
- Keine Default-TTL, keine automatische Wiederaufnahme, kein zeitbasiertes Failover, keine neuen Dispatch-State-Transitions erfinden.
- Bei in-flight oder unbekannten Submission-Ergebnissen konservative Sperre als Design-Invariante dokumentieren; keinerlei Runtime- oder Konfigurationsänderung.

## 5. Quellenintegrität und Statusmodell

- Wenn `contracts/source-manifest.json` alle Quellartefakte abdeckt, bei geänderten ADFR-Dateien die SHA-256-Hashes und Byte-Längen aktualisieren. Das Manifest darf nicht fälschlich alle Quellen als unabhängig geprüft oder fachlich genehmigt darstellen.
- Prüfen, ob das Manifest neue Entscheidungsdokumente getrennt referenzieren soll; Provenienz nicht mit Ratifikation vermengen.
- `contracts/registry/registry.json` **unverändert** lassen. Kein `schema_path`, kein neues Wire-Schema, keine Änderung an `SKELETON_NOT_CANONICAL` oder `part_b=NO-GO`.
- Ursprünglichen TRD-ARCH-SYNC-01-Bericht nicht überschreiben. Die neue Bewertung in einem eigenen Bericht mit Delta zu XC-03/XC-04/XC-11 festhalten.
- XC-01, XC-02-PRED, XC-03-MATRIX, XC-05-LOCK, XC-06, XC-07-ROUND, XC-08-IBKR, XC-09-RESET, XC-10-ACTION bleiben offen. Weitere Teilbefunde nur auf Basis ausdrücklicher ARCH-DEC-01..03 schließen.

## 6. Automatisierte Validierung (nur Dokumentations-Tests)

Erstelle oder erweitere eine kleine deterministische Prüfroutine, vorzugsweise im vorhandenen Dokumentations-/Testframework. **Keine Runtime-Logik.** Prüfe:

- ADFR JSON und CSV parsebar; exakt 78 eindeutige IDs pro Format, identische ID-Mengen, identische gemeinsame Fachfelder.
- Genau `ADFR-CROSS-005` hat Lifecycle `SUPERSEDED`, genau `ADFR-CR-B10-001` ist sein Nachfolger; bidirektionale Referenzen stimmen.
- Jede ID hat eine nichtleere `source`, `decision`, `status`, `basis`, `verification`; `verification` bleibt ehrlich.
- Alle alten 77 JSON-IDs bleiben vorhanden, keine fachlichen Texte unbemerkt verändert. Änderungen als Diff und begründete Ausnahme berichten.
- Quellmanifest stimmt zu tatsächlichen Datei-Hashes und Byte-Längen; bereits vorhandene unveränderte Quellen bleiben byte-identisch.
- Dokumentierte Baseline für DET-01/02 schließt ausdrücklich nicht die offenen Details; ARCH-DEC-03 beansprucht keine technische Verifikation.
- Registry bleibt unpublished und Live-Trading bleibt nicht autorisiert.

Zusätzlich lokale Checks ausführen: `cargo fmt --all -- --check`, `cargo check --workspace --locked`, `cargo test --workspace --locked`, `cargo clippy --workspace --all-targets --locked -- -D warnings`. Bereits vorhandene ignorierte PostgreSQL-Tests separat ausweisen; **nicht** ohne ausdrückliche Testumgebungsfreigabe gegen eine echte Datenbank ausführen. Remote CI nicht als ausgeführt ausgeben.

## 7. Explizit verboten

- Keine Änderungen an `crates/*/src`, `apps/*/src`, HTTP-Routen, Broker-Adaptern, Risk/OMS-Implementierung, Domain-Typen, Serde-Wire-Formaten, SQL-Migrationen oder Deployment-Konfiguration.
- Keine neuen Broker-/Marktdatenverbindungen, Credentials, Order-Platzierungen, Cancel/Modify, Reconciliation, automatische Resets oder Break-Glass-Funktionen.
- Keine kanonischen Schemas publizieren; keine Tests B10-T01..T12 als bestanden markieren, wenn sie nicht tatsächlich durchgeführt wurden.
- Keine weiteren Architekturentscheidungen selbstständig ratifizieren; bei fachlicher Mehrdeutigkeit `BLOCKED` mit konkreter Rückfrage dokumentieren.
- Kein Git-Push.

## 8. Acceptance Gates

| Gate | Nachweis |
|---|---|
| AS02-G01 | Clean Baseline, vollständige Quelleninventur und Hashprüfung |
| AS02-G02 | ARCH-DEC-01/02/03 wortgetreu und datiert dokumentiert |
| AS02-G03 | DET-01/02 Design-Baseline ratifiziert, Rekonstruktionsprovenienz und offene Details sichtbar |
| AS02-G04 | ADFR JSON und CSV exakt 78 identische historische IDs |
| AS02-G05 | `ADFR-CROSS-005` superseded; `ADFR-CR-B10-001` Nachfolger; 77 nicht-superseded |
| AS02-G06 | Quellmanifest und Registerdokumentation konsistent, keine unbegründete Datenänderung |
| AS02-G07 | CR-B01 Vorrang vor DET-07 dokumentiert; XC-04-Restfragen OPEN |
| AS02-G08 | Übrige XC-Befunde bleiben OPEN, keine implizite Implementierungsfreigabe |
| AS02-G09 | Registry, Trading-Runtime und Sicherheitsflags unverändert |
| AS02-G10 | Deterministische ADFR-/Provenienzprüfungen PASS |
| AS02-G11 | Lokale Cargo-Checks PASS oder ehrlich als BLOCKED dokumentiert |
| AS02-G12 | Prompt archiviert, Abschlussbericht geschrieben, genau ein nachvollziehbarer Commit; kein Push |

**Gesamturteil:** PASS nur wenn G01–G12 PASS. Sonst `PARTIAL`/`BLOCKED` und keine Statusbeschönigung.

## 9. Deliverables und Commit

1. `contracts/decisions/ARCH-DEC-01-bis-03-2026-11-15.md` (oder konsistenter gleichwertiger Pfad).
2. Synchronisierte `contracts/sources/ADFR-Consolidated-v1.0.json` und `.csv` mit explizitem Änderungs-/Versionsvermerk; bei materieller Formatänderung ggf. neue versionierte Dateien plus Verweis, nicht stilles Brechen von Konsumenten.
3. Aktualisiertes `contracts/source-manifest.json`, `contracts/traceability-index.md` und nur betroffene Dokumentations-READMEs.
4. Deterministischer Dokumentations-Validierungstest.
5. `docs/architecture/TRD-ARCH-SYNC-02-report.md` mit Baseline, geänderten Dateien, SHA-256, 78/77/1-Zählung, Entscheidungen, weiterhin offenen XC-Befunden, Testergebnissen, Abweichungen und Next-Gate.
6. Prompt wortgetreu unter `docs/cursor-prompts/trading/TRD-ARCH-SYNC-02-Architecture-Ratification-and-ADFR-Reconciliation.md` archivieren.
7. Nach erfolgreicher lokaler Prüfung **einen Commit** erstellen: `TRD-ARCH-SYNC-02: Ratify Design Baselines and Reconcile ADFR`. Commit-SHA und `git status --short` im Abschlussbericht bzw. finalen Cursor-Output nennen; keinen Push.

## 10. Finaler Cursor-Report (Pflichtformat)

- `VERDICT: PASS | PARTIAL | BLOCKED`
- `BASELINE_HEAD`, `FINAL_COMMIT`, `BRANCH`, `WORKTREE_STATUS`, `PUSHED: NO`
- `ARCH-DEC-01/02/03`: exakt was beschlossen und was **nicht** beschlossen wurde
- `ADFR`: JSON count, CSV count, unique IDs, superseded count, successor ID, field parity
- `SOURCE_MANIFEST`: Anzahl, Hash-Verifikation, veränderte Quellen
- `XC_DELTA`: nur konkret geschlossene Teilbefunde; übrige offene Risiken
- `TESTS`: Befehle, Exit-Codes, ignored tests, remote CI nicht ausgeführt
- `SAFETY`: Registry-Skeleton unverändert, canonical schemas unpublished, no trading behavior, live trading unauthorized
- `NEXT_STEP`: Vorschlag nur als Plan; keine automatische Implementierung

**Wichtig:** Das Ziel ist eine überprüfbare, widerspruchsarme Architektur-Dokumentation. Keine der drei Nutzerentscheidungen erteilt eine Freigabe zur Implementierung sicherheitskritischer Handelsfunktionen.
