# TRD-ARCH-03B.4.2 — Ratify IBKR Verification Strategy

This file is the TRD-ARCH-03B.4.2 instruction executed on 2026-11-15. It is the textual decision record. No signature artifact and no evidence URI are attached to it.

**Datum der Nutzerentscheidung:** 2026-11-15  
**Arbeitsmodus:** Ausschliesslich datierte Dokumentation, Traceability und dokumentationsbezogene Regressionstests.  
**Autorität:** Die ausdrückliche Nutzerbestätigung für IBKREVQ-01 bis IBKREVQ-14 in der Anweisung zu TRD-ARCH-03B.4.2. Keine zusätzliche Unterschrift, URL oder unabhängige Freigabe erfinden.

## 0. Auftrag und unüberschreitbare Grenzen

Ratifiere **ausschliesslich** die 14 unten festgelegten Verifikations-Designentscheidungen als **ARCH-DEC-49 bis ARCH-DEC-62**, **nur wenn alle 14 Kennungen nach Repository-Preflight noch frei sind**. Bei Kollision, fehlendem Vorgänger, unklarem Quellenstatus, verändertem geschütztem Bestand oder nicht eindeutig nachvollziehbarer Ratifikation: **STOP, kein Commit, Abweichung dokumentieren**. Bestehende ARCH-DEC-01..48 weder ändern noch umnummerieren oder stillschweigend superseden.

Die Ratifikation ist `RATIFIED_DESIGN_POLICY_ONLY`, `NOT_IMPLEMENTED`, `NOT_VERIFIED`, `NON_RUNTIME_NON_WIRE`. Sie autorisiert **keinen** Schritt E0, E1, E2, E3 oder E4; alle Checkpoints bleiben `STOP`. Sie autorisiert **keine** Recherche-/Beschaffungsaktion im Namen einer freigegebenen Stufe, keinen Broker-Login, keine Verbindung zu TWS/IB Gateway, keinen Accountzugriff, keinen API-Read, keinen Broker-Test, keine Fault Injection gegen Broker, keinen Orderversand (auch nicht Paper), keine Brokeraktion, keinen Retry/Resend/Takeover, keine wirtschaftliche Buchung oder Korrekturbuchung, keine Umwandlung/Reduktion/Freigabe einer Risk-Reservierung, kein Economic-Finality-Prädikat, keine OMS-Transition, keine kanonischen Schemas, keine DDL/Migration, keinen Identity Resolver, keinen UNIQUE-Index, keine Vollständigkeits-Tokens, keine Live-Trading-Freigabe und keine produktiven Rust-Codeänderungen.

**Wichtig:** Lokale Cargo-Tests, die Dokumentationsregeln prüfen, sind erlaubt und erforderlich. Sie sind **keine externen IBKR-Verifikationstests** und ändern weder `NOT_EXECUTED` noch `EXTERNAL_EVIDENCE_PENDING`.

## 1. Preflight: Repository, Quellen und Git

1. `git status --short`, `git branch --show-current`, `git rev-parse HEAD`, `git log -1 --format=%s`, Git-Status und Baseline-HEAD protokollieren. Erwarteter Vorgänger ist `TRD-ARCH-03B.4: Plan IBKR Evidence and Verification` (laut Vorbericht; tatsächlich im Repository prüfen). Bei schmutzigem Worktree STOP; nichts resetten, stashen, verwerfen oder überschreiben.
2. Im Repository die tatsächlichen Pfade und Konventionen prüfen, insbesondere:
   - `contracts/verification-plans/TRD-ARCH-03B4-approval-requests.md`
   - `contracts/verification-plans/TRD-ARCH-03B4-ibkr-evidence-inventory.md`
   - `contracts/verification-plans/TRD-ARCH-03B4-ibkr-verification-protocol.md`
   - `contracts/verification-plans/TRD-ARCH-03B4-ibkr-test-matrix.md`
   - `contracts/verification-plans/TRD-ARCH-03B4-ibkr-evidence-capture-and-provenance.md`
   - `contracts/verification-plans/TRD-ARCH-03B4-dependency-and-conflict-matrix.md`
   - `contracts/verification-plans/TRD-ARCH-03B4-plan-index.json`
   - `contracts/decisions/ARCH-DEC-35-bis-48-2026-11-15.md`
   - ältere ARCH-DEC-01..34, Registry, Source-Manifest, ADFR-JSON/-CSV und relevante Tests.
3. Prüfen, dass IBKREVQ-01..14 in der historischen Vorlage `USER_DECISION_PENDING`/`NOT_RATIFIED` sind, mit A/B/C und C=`CONFLICTS_WITH_BINDING_POLICY`. **Historische Empfehlungen, Fragen und Optionen unverändert lassen**; Entscheidungen in einem separaten Ratifikationsdokument und einem nicht-runtime Resolution-Index abbilden. Zulässig ist ein klar abgegrenzter Append-only-Verweis/Addendum, sofern Repository-Konvention und historische Präfixintegrität gewahrt bleiben.
4. Prüfen, dass ARCH-DEC-49..62 nicht bereits in Entscheidungsdateien, Resolution-Indizes oder Registry vergeben sind. Referenzen als zukünftige Guard-IDs zählen nicht als Vergabe; falls ein solcher Test `ARCH-DEC-49` als nächste freie ID erwartet, nach Ratifikation gezielt auf `ARCH-DEC-63` ändern, ohne Historie zu verändern.
5. Alle 12 Quellen des `contracts/source-manifest.json` vor und nach der Arbeit gegen `sha256` und `bytes` prüfen; auch `package_sha256`/`package_bytes` nicht verändern. DET-01/02 bleiben `reconstructed_draft`, `SOURCE_REVIEW_PENDING`, `reviewed=false`, `approved=false`.
6. ADFR-JSON und -CSV: 78 Einträge, gleiche IDs/Reihenfolge, alle `NOT_VERIFIED`, genau eine Supersession `ADFR-CROSS-005 -> ADFR-CR-B10-001`; Registry bleibt `SKELETON_NOT_CANONICAL`, `canonical_schemas_published=false`, `part_b=NO-GO`, `live_trading_authorized=false`, `broker_uniqueness_evidence=EVIDENCE_PENDING`. Ausgangs- und Endhashes berichten. Erwarteter bisheriger Registry-SHA-256: `7abed2af5363726403175c6524bd7a4d05d36b8735ad48442fc26941f8d98197`; bei Abweichung zuerst STOP und Ursache prüfen, nicht auf einen erwarteten Hash zurückschreiben.
7. Sämtliche ARCH-DEC-01..48, bestehende Proposal-/Plan-Indizes und historische Quellartefakte vor/nach bytegenau vergleichen. Keine Behauptung externer Evidenz aus einem grünen Cargo-Test ableiten.

## 2. Verbindliche Nutzerentscheidungen (exakt, 8 × A und 6 × B)

Ordne jede Frage **eins zu eins** der nächsten ARCH-DEC-ID zu. Datum 2026-11-15, ausdrückliche Nutzerinstruktion als Autorität, historische Frage als Quelle, Ratifikationsstatus ausschliesslich Designpolicy. Beschreibe jeweils die positiven Vorgaben, **nicht** autorisierte Aktionen, Abhängigkeiten, offenen Konflikte und spätere Nachweispflichten.

| Neue ID | Frage | Wahl | Verbindlicher Inhalt und Grenzen |
|---|---|---|---|
| ARCH-DEC-49 | IBKREVQ-01 | **B** | Nur tatsächlich beschaffte, versionierte offizielle IBKR-Primärtexte mit Abrufdatum als Primärquelle; zusätzlich zweite **unabhängig beschaffte Kopie derselben Version** vor einer späteren E4-Zitierung. Zweite Kopie ist keine unabhängige empirische Brokerbestätigung. Keine Quelle, URL, Version oder Beschaffung erfinden. |
| ARCH-DEC-50 | IBKREVQ-02 | **A** | Pro zukünftigem Nachweispaket genau ein gepinntes Produkt-/API-Client-/Plattform-/OS-/Kontomodus-Tupel (und relevante Umgebungsangaben); andere Tupel benötigen getrennte Pakete. Keine Versionswerte erfinden, keine universelle Übertragung. |
| ARCH-DEC-51 | IBKREVQ-03 | **A** | Account, Client, Session, Dataset, Zeitfenster und Umgebung nur dokumentieren, soweit Request oder Payload sie tatsächlich liefern; fehlende Felder bleiben fehlend. Kein globaler Snapshot und kein Coverage-Nachweis. |
| ARCH-DEC-52 | IBKREVQ-04 | **B** | Testumgebungsklasse (Paper/Sandbox/etc.) bis zur tatsächlichen offiziellen E0-Dokumentation offenlassen; später gesondert entscheiden. Keine Testumgebung oder Kontonutzung freigeben. |
| ARCH-DEC-53 | IBKREVQ-05 | **B** | Vor gesonderter, datierter Entscheidung über **exakt benannte** und offiziell dokumentierte zulässige Leseoperationen keinerlei Brokerverbindung/Read. Diese Ratifikation benennt keine Operation und gibt E2 nicht frei. Cancel/Modify/Replace sind niemals als Read zu deklarieren. |
| ARCH-DEC-54 | IBKREVQ-06 | **B** | Zukünftige Oracle-Notiz dokumentiert Scope, gespeicherte Bytes und lokale Gaps und enthält explizit die Aussage, dass **Capture-Vollständigkeit nicht Broker-Vollständigkeit** ist und Broker-Vollständigkeit nicht nachgewiesen wurde. Coverage `UNKNOWN`, kein Completeness-Token, leere Antwort kein Negativbeweis. |
| ARCH-DEC-55 | IBKREVQ-07 | **A** | Spätere kontrollierte Untersuchungen dürfen Wiederverwendung/Nichtwiederverwendung von `orderId`, `permId`, `execId` **innerhalb eines gepinnten Tupels** als Beobachtungen berichten. Keine Eindeutigkeitsgarantie, kein UNIQUE-Index, keine automatische wirtschaftliche Zuordnung; Tests heute nicht autorisiert. |
| ARCH-DEC-56 | IBKREVQ-08 | **A** | Spätere Correction-/Bust-Referenzen nur dann strukturiert übernehmen, wenn **der neue Rohpayload selbst** die Referenz enthält. Append-only Originale erhalten; keine erfundene Verknüpfung, keine Reversal-Buchung, kein Risk Release, keine Closure-Änderung. |
| ARCH-DEC-57 | IBKREVQ-09 | **B** | Unknown-Send-Crash-/Disconnect-Szenarien bis zu einer neuen ausdrücklichen Entscheidung **nur synthetisch/offline**. Keine Fault Injection gegen Broker, keine E3-Freigabe, kein Retry/Resend/Takeover. |
| ARCH-DEC-58 | IBKREVQ-10 | **A** | Zukünftige Writer-Isolation-Nachweise als **separat zu belegende** Prozess-Stopp- und Connection-Closed-Artefakte **oder** belastbare Session-Exklusivitätsbestätigung, zusammen mit Reconciliation unbekannter Versuche, verlangen. `WriterEpoch`/Lease-Deadline separat führen. Dies ist eine Nachweis-Checkliste, **kein hinreichender Takeover-Beweis**, kein operativer Freigabepfad; CR-B01 bleibt vorrangig. |
| ARCH-DEC-59 | IBKREVQ-11 | **A** | Künftige Rohpayloads unveränderlich, append-only und geheimnisgeschützt **ausserhalb des Git-Repositories** verwahren; Hashes, Versions-Tupel und bereinigte Logs; widersprüchliche Originalbytes erhalten. Jetzt weder Store noch Payload erzeugen, keine Secrets ins Git. Speicherort, Zugriffsschutz und Retention bleiben später zu genehmigen. |
| ARCH-DEC-60 | IBKREVQ-12 | **A** | Bei Änderung von Produkt, API-Client, Plattform, OS oder Kontomodus gelten frühere externe Claims nicht automatisch für das neue Tupel; neues Paket und erneute Entscheidung erforderlich. Kein erfundener Kalenderzeitraum, keine universelle Aussage. |
| ARCH-DEC-61 | IBKREVQ-13 | **A** | Unerwartetes Verhalten stoppt die spätere Stufe; nicht autorisierenden Eskalationsmarker vorsehen. Marker ist weder Rolle noch Reset-/Post-/Release-/Dispatch-Recht. Fehlende erforderliche Zweitfreigabe bedeutet Deny. Kein tatsächlicher Stage-Start. |
| ARCH-DEC-62 | IBKREVQ-14 | **B** | Künftige Erfassung und Ratifikation **in getrennten Instruktionen und getrennten Git-Commits**. Eine Review-Notiz im Capture-Paket ist keine Entscheidung. Nur spätere datierte Nutzerentscheidung kann nach tatsächlichen Artefakten benannte Evidenz akzeptieren; selbst das autorisiert weder Implementierung noch Live-Trading. |

**Wahl-Checksumme:** `B, A, A, B, B, B, A, A, B, A, A, A, A, B` = **8 × A, 6 × B**. Alle 14 C-Varianten bleiben `CONFLICTS_WITH_BINDING_POLICY`; nicht übernehmen. Die Nutzerbestätigung ersetzt **keine** externe Test- oder Stufenfreigabe.

## 3. Dokumentationsartefakte und Traceability

Erstelle nach Prüfung der Repository-Konventionen mindestens:

1. `contracts/decisions/ARCH-DEC-49-bis-62-2026-11-15.md` (falls frei) mit vollständigen Einzelentscheidungen, Quellenankern IBKREVQ-01..14, Entscheidungstext, Status, Abgrenzungen, Risiken, Konflikten und expliziter datierter Nutzerautorität. Keine erfundene Signatur/Evidence-URI.
2. `contracts/verification-plans/TRD-ARCH-03B42-resolution-index.json` als **`NON_RUNTIME_NON_WIRE_RESOLUTION_INDEX`**, nicht Registry, nicht Schema. Die Struktur an bestehenden Resolution-Indizes orientieren; bei notwendiger JSON-Test-Allowlist nur den konkreten nicht-kanonischen Dokumentationsindex zulassen. `evidence_uri=null`, `signature_artifact=null`, externe Tests `NOT_EXECUTED`, `EXTERNAL_EVIDENCE_PENDING`, `implementation_authorized=false`, `testing_authorized=false`, `broker_connection_authorized=false`, `read_only_broker_access_authorized=false`, `order_send_authorized=false`, `paper_trading_authorized=false`, `uniqueness_guaranteed=false`, `completeness_proof_approved=false`, `economic_posting_approved=false`, `risk_release_approved=false`, `economic_finality_predicate_approved=false`, `oms_transitions_approved=false`, `broker_dispatch_approved=false`, `live_trading_authorized=false`, `closes_xc_findings=false`. E0..E4 STOP.
3. Falls üblich, **Append-only** Ratifikationsverweis in `TRD-ARCH-03B4-approval-requests.md`; die historischen Fragen, Empfehlungen und Optionsvergleiche byteidentisch erhalten. Historische `USER_DECISION_PENDING`-Abschnitte nicht rückwirkend als ursprüngliche Freigabe umschreiben.
4. Nur notwendige Verweise in `contracts/traceability-index.md`, `contracts/domain/unresolved-decisions.md`, passenden READMEs/Glossar; die bestehende 27-Claim-/43-Szenario-Matrix und ihren Status nicht heimlich ändern. Die Entscheidungsliste ist kein Verifikationsnachweis.
5. `docs/architecture/TRD-ARCH-03B.4.2-report.md` als Abschlussbericht, `docs/cursor-prompts/trading/TRD-ARCH-03B4.2-Ratify-IBKR-Verification-Strategy.md` als **exakte ausgeführte Anweisung** archivieren; keine neuen Nutzerentscheidungen hineininterpretieren.

Traceability mindestens: `IBKREVQ -> ARCH-DEC -> Quelle/Abschnitt -> IBKR-CLM / IBKR-SCN (falls relevant) -> XC -> spätere Proof Obligation -> gegenwärtiger Status`. Bei fehlender konkreter Zuordnung `NOT_ESTABLISHED` statt erfundener Verweise.

## 4. Konflikt- und Sicherheitsinvarianten

- `XC-01`: OPEN, `EVIDENCE_PENDING`; keine globale Identitäts-Eindeutigkeit.
- `XC-08-IBKR`: OPEN, `EXTERNAL_EVIDENCE_PENDING`; keine Snapshot-Vollständigkeit.
- `XC-04`: CR-B01-Designpräzedenz bleibt, operativer Recovery-/Failover-Rest OPEN; lokale Epoch/Lease ist kein externes Fencing, kein Successor-Dispatch bei unbekanntem Sendestatus.
- `XC-02-PRED`: OPEN, kein Economic-Finality-Prädikat; `XC-03-MATRIX`: OPEN, keine OMS-Kanten.
- `XC-05-LOCK`, `XC-07-ROUND`, `XC-09-RESET`, `XC-10-ACTION`, `XC-11` bleiben im bisherigen offenen bzw. nur prinzipiell gelösten Zustand. Keine implizite Konfliktschliessung.
- CR-B01 hat weiterhin Vorrang gegenüber schwächerer DET-07-Auslegung. Eine einzelne Broker-Zuordnung erlaubt keine Buchung (ARCH-DEC-13/26/36). Kein Terminalstatus allein ist Finalitäts- oder Freigabenachweis.
- `ARCH-DEC-49..62` dürfen keine `verification`-Zelle der 78 ADFR-Einträge ändern. Historische ARCH-DEC-01..48 bleiben bindend und unverändert.
- Unterscheide ausdrücklich `documentation_tests_passed` von `external_broker_tests_executed=false`.

## 5. Regressionstests und negative Assertions

Ergänze **nur dokumentationsbezogene** Tests nach bestehenden Rust-Testkonventionen, ohne produktive Quellen zu ändern. Prüfe mindestens:

1. Exakt 14 fortlaufende Entscheidungen ARCH-DEC-49..62, eindeutige IDs, datiert, korrektes 8A/6B-Mapping und Quellenanker.
2. Alle Entscheidungen `RATIFIED_DESIGN_POLICY_ONLY`, `NOT_IMPLEMENTED`, `NOT_VERIFIED`, `NON_RUNTIME_NON_WIRE`; kein Autorisierungs- oder Broker-Evidenzstatus.
3. Historische Approval-Fragen bleiben unverändert; C bleibt überall Konfliktoption; Empfehlungen sind keine Nutzerentscheidung.
4. 27 Claims bleiben Hypothesen und externe Evidenz ausstehend; 43 Szenarien bleiben `NOT_EXECUTED`; E0..E4 bleiben STOP.
5. Keine tatsächliche Broker-Operation, keine frei erfundene IBKR-Methode/Version/URL/Account/Testergebnis.
6. Keine Uniqueness-Garantie, kein Resolver, kein Completeness-Token, keine globale Snapshot-Behauptung; Singleton-Kandidat ist keine Autorisierung.
7. Unknown Send bleibt fail-closed, kein Retry/Resend/Takeover; lokale Fencing-Fakten sind keine Broker-Akzeptanz.
8. Korrektur-/Bust-Links nur bei tatsächlichem Payload-Verweis; kein Posting/Release/Closure-Änderung.
9. Keine wirtschaftliche Buchung, Risk Release, Economic-Finality-Prädikat, OMS-Transition, Broker-Dispatch, Live-Trading-Freigabe oder Schema-Publikation.
10. 12/12 Source-Manifest-Hash- und Byteprüfungen, 78 ADFR mit Parität/Status/Supersession, Registry-Sicherheitsflags, ARCH-DEC-01..48 byteidentisch.
11. Resolution-Index explizit non-wire, nicht in kanonischer Registry, keine Rechtevergabe; keine Geheimnisse/Rohpayloads in Git.
12. Keine produktiven Dateien `crates/*/src`, `apps/*/src`, `Cargo.toml`, `Cargo.lock`, Migrationen, Credentials oder Runtime-Flags geändert. Der nächste freie ARCH-DEC-Identifier ist 63, sofern keine andere legitime Vergabe besteht.

Führe aus Repository-Root auf finalem Arbeitsbaum aus und berichte **Exitcodes**:

```bash
cargo fmt --all -- --check
cargo check --workspace --locked
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
```

Falls Formatierung nötig ist, nur gezielte Änderungen, erste Fehlschläge und Korrekturen ehrlich protokollieren, anschliessend alle vier Prüfungen erneut auf finalem Stand ausführen. Der bislang ignorierte PostgreSQL-Test ist **nicht bestanden**, sondern `ignored/not run`, solange seine Voraussetzungen fehlen. Remote CI nicht als ausgeführt behaupten. Keine externen Broker-Tests ausführen.

## 6. Zwölf Acceptance-Gates (alle erforderlich)

- **A03B42-G01:** Clean Baseline, korrekter Vorgänger, IDs 49..62 frei, keine fremden Änderungen.
- **A03B42-G02:** Exakt 14 Nutzerentscheidungen, 8A/6B, Datum/Autorität korrekt, keine erfundene Signatur.
- **A03B42-G03:** Historische IBKREVQ-Fragen/Empfehlungen unverändert; separate Resolution und nachvollziehbare Verweise.
- **A03B42-G04:** 12 Quellen, Manifest, Registry, 78 ADFR, 48 frühere Entscheidungen geschützt und nachgeprüft.
- **A03B42-G05:** Quellenqualität, Tupel-Pinning, Scope (01..03) ohne Evidenzbehauptung korrekt ratifiziert.
- **A03B42-G06:** Umgebungs-/Read-Sperren und Completeness-Grenzen (04..06) korrekt; E0..E4 STOP.
- **A03B42-G07:** Alias-/Correction-/Unknown-Send-Regeln (07..09) korrekt; keine Ausführung.
- **A03B42-G08:** Isolation, Retention, Revalidation, Eskalation (10..13) korrekt, keine neuen Rechte.
- **A03B42-G09:** Sammlung und Ratifikation getrennt (14); XC-01/XC-08-IBKR/XC-04-Rest/XC-02-PRED weiterhin OPEN.
- **A03B42-G10:** Keine Runtime-/Schema-/Broker-/Wirtschafts-/Live-Trading-Änderung; Safety-Flags false.
- **A03B42-G11:** Dokumentationsregressionen und alle vier Cargo-Prüfungen auf finalem Baum PASS; ignored/CI sauber ausgewiesen.
- **A03B42-G12:** Bericht und exakter Prompt archiviert; **genau ein lokaler Git-Commit**, kein Push; Git-Status und Commit-Hash nach Commit ausgegeben.

Bei einem nicht bestandenen Gate: **`VERDICT: PARTIAL` oder `FAIL`**, kein PASS behaupten; Sicherheits- oder Quellenintegritätsverletzungen sind STOP-Bedingungen. Kein Commit bei ungeklärter Preflight-Kollision oder Schutzverletzung.

## 7. Abschlussbericht, Commit und Arbeitsdisziplin

Bericht muss enthalten: `VERDICT`, Datum, Branch, `BASELINE_HEAD`, `FINAL_HEAD` **im Cursor-Terminaloutput** (nicht selbstreferenziell im gerade zu commitenden Bericht erzwingen), Commit-Subject, `PUSHED: NO`, Worktree-Status, 14er-Mapping und 8A/6B-Checksumme, neue/angepasste Dateien mit Hashes, Quellen-/ADFR-/Registry-Integrität, Traceability, offene XC, 27 Claims und 43 Szenarien unverändert unbewiesen, E0..E4 STOP, alle Cargo-Kommandos mit Exitcodes, Anzahl passed/failed/ignored, Remote CI nicht ausgeführt, Abweichungen/Korrekturen, Gate-Tabelle G01..G12 und nächste Schritte **ohne automatische Freigabe**.

Archivierte Promptdatei muss den ausgeführten Inhalt exakt wiedergeben (ggf. eine klar deklarierte Metadatenzeile voranstellen, falls bestehende Konvention dies verlangt). Änderungen vor Commit auf unerlaubte Pfade prüfen. Dann **genau einen** lokalen Commit erstellen:

```text
TRD-ARCH-03B.4.2: Ratify IBKR Verification Strategy
```

**Niemals `git push` ausführen.** Kein zweiter Commit, kein `--amend`, kein Reset/Rebase, kein destruktiver Cleanup. Nach Commit im Cursor-Terminal `git rev-parse HEAD`, `git log -1 --format=%s`, `git status --short` und optional `git show --stat --oneline HEAD` ausgeben, damit die Commit-Existenz extern überprüfbar ist. Falls Git-Commit fehlschlägt, transparent `PARTIAL` melden statt Commit zu behaupten.

**Endzustand:** ARCH-DEC-49..62 sind ausschliesslich datierte Designrichtlinien; kein E0–E4-Go, keine Brokerverbindung, keine Testausführung, keine wirtschaftliche Wirkung, keine Implementierungs- oder Live-Trading-Autorisierung.
