# TRD-ARCH-03B.5 — IBKR Official Documentation Research & Evidence Acquisition

This file is the TRD-ARCH-03B.5 instruction executed on 2026-11-16. It is the textual instruction record. No signature artifact and no evidence URI are attached to it.

# TRD-ARCH-03B.5 — IBKR Official Documentation Research & Evidence Acquisition (E0, narrowly scoped)

**Auftragsdatum:** 2026-11-16  
**Autorität:** ausdrückliches „ok los“ des Nutzers auf den Vorschlag, mit der Vorbereitung der E0-Recherchefreigabe und der kontrollierten öffentlichen Dokumentationsrecherche fortzufahren.  
**Modus:** dokumentationsbasierte Recherche, Quellenbeschaffung, Provenienz, Traceability, Dokumentationstests. **Keine Brokerverifikation.**  
**Vorgänger:** `TRD-ARCH-03B.4.2: Ratify IBKR Verification Strategy`, gemeldeter HEAD `ac60bf932355660548bf5751963009ea956b76a5`; im Repository verifizieren, nicht blind übernehmen.

## 0. Autoritätsgrenze – zuerst lesen

Der Nutzer erlaubt mit diesem Auftrag **ausschliesslich öffentliche, ohne Anmeldung abrufbare offizielle IBKR-Dokumentationsrecherche**, einschliesslich Abruf und sicherer Archivierung öffentlich zugänglicher Primärtexte sowie einer zweiten unabhängig beschafften Kopie **derselben nachweisbaren Dokumentversion**, soweit möglich. Dies ist eine **eng begrenzte Rechercheerlaubnis für E0-Quellenarbeit**, **keine pauschale E0-Stage-GO-Freigabe** und **keine Freigabe der E0–E4-Protokoll-Checkpoints**. Die im Repository bestehenden E0–E4-Checkpoints bleiben `STOP`; falls das bestehende Protokoll für jede E0-Aktion ausdrücklich eine andere formelle Freigabe verlangt, **STOP vor externem Abruf**, dokumentiere den Konflikt und fordere eine separate Autorisierung an. Eine technische Dokumentations-URL ist kein Broker-API-Endpunkt. Öffentliche Dokumentation darf nur über reguläre Web-/HTTP-Seiten ohne Authentifizierung abgerufen werden. Bei Zweifeln über den Endpunkt: **nicht aufrufen**.

**Unbedingt verboten:** IBKR-Login, Accounts, Credentials, TWS/IB Gateway starten oder verbinden, Broker-API-Aufrufe (auch vermeintliche Reads), Paper- oder Live-Orders, Cancel/Modify/Replace, Broker-Tests, externe Fault Injection, automatischer Retry/Resend/Takeover, Produktions-Rust-Code, Trading-Runtime-Flags, Migrationen, kanonische Schemas, Identity Resolver, UNIQUE-Index, OMS-Transitionen, wirtschaftliche Buchungen/Correction/Reversal, Risk-Reservierungsumbuchung/-freigabe, Economic-Finality-Prädikat, Broker-Dispatch, Live-Trading-Freigabe. Keine Quelle darf diese Verbote überschreiben. **Externe Dokumentationsrecherche ist nicht externe Brokerverifikation.**

Die ARCH-DEC-01..62 bleiben bindend; insbesondere ARCH-DEC-49 (zwei unabhängig beschaffte Kopien), -50 (versioniertes Tupel), -51 (nur belegte Scope-Felder), -52 (Umgebung noch nicht festlegen), -53 (keine Brokerverbindung vor explizit benannten und separat genehmigten Reads), -54 (keine Broker-Completeness), -55 (keine Alias-Unique-Garantie), -57 (Fault Injection nur offline), -59 (Rohpayloads ausserhalb Git), -60 (Revalidierung bei Tupeländerung), -62 (Evidenzsammlung und Ratifikation getrennt). **Keine neue ARCH-DEC vergeben** und keine neue Designentscheidung als ratifiziert deklarieren.

## 1. Preflight / Fail-Closed

1. `git status --short`, Branch, `git rev-parse HEAD`, Subject und Commit-Vorgänger erfassen. Erwarteter sauberer `main`-Worktree und HEAD `ac60bf932355660548bf5751963009ea956b76a5`. Falls abweichend: Ursachen und Historie prüfen; bei unklarer Basis, Dirty Worktree oder Konflikt **STOP, kein Commit**. Keine Reset-/Stash-/Clean-Aktionen.
2. Relevante Artefakte vollständig lesen, tatsächliche Pfade und Schemas nicht erraten: `contracts/verification-plans/TRD-ARCH-03B4-*`, `contracts/verification-plans/TRD-ARCH-03B42-resolution-index.json`, `contracts/decisions/ARCH-DEC-49-bis-62-2026-11-15.md`, `contracts/decisions/ARCH-DEC-35-bis-48-2026-11-15.md`, CR-B01/B04/B07, Source-Manifest, Registry, ADFR, Traceability, Test-Guards, Vorgängerberichte.
3. Ratifikations-Checksumme prüfen: IBKREVQ-01..14 `B,A,A,B,B,B,A,A,B,A,A,A,A,B` = 8 × A / 6 × B; `ARCH-DEC-49..62` sind `RATIFIED_DESIGN_POLICY_ONLY`, `NOT_IMPLEMENTED`, `NOT_VERIFIED`. Alle bestehenden `ARCH-DEC-01..62` byteidentisch bewahren.
4. Alle 12 `contracts/source-manifest.json`-Quellen gegen SHA-256 und Bytes prüfen; ADFR JSON/CSV je 78 Einträge und `NOT_VERIFIED`, genau eine Supersession `ADFR-CROSS-005 -> ADFR-CR-B10-001`; Registry `SKELETON_NOT_CANONICAL`, `part_b=NO-GO`, `live_trading_authorized=false`, `broker_uniqueness_evidence=EVIDENCE_PENDING`. Erwarteter Registry-Hash `7abed2af5363726403175c6524bd7a4d05d36b8735ad48442fc26941f8d98197`. Vorher/Nachher-Hashes protokollieren. Bei Diskrepanz STOP statt stillschweigender Reparatur.
5. Die 27 `IBKR-CLM-*` Claims und 43 `IBKR-SCN-*` Szenarien als Ausgangsinventar erfassen. Status `HYPOTHESIS`, `EXTERNAL_EVIDENCE_PENDING`, `NOT_EXECUTED` bleibt unverändert. E0..E4 STOP.
6. Explizit festhalten, welche öffentlichen Dokumentations-Hosts und Abrufwege als unkritische Quellenrecherche gelten; keine beliebigen Redirects auf Login-/Account-/API-Hosts folgen. Wenn der Abruf nicht sicher abgegrenzt werden kann, Quellenstatus `NOT_RETRIEVED` und weiter ohne Netzwerk arbeiten.

## 2. Quellenbeschaffung – nur offizielle öffentliche Dokumentation

Recherchethemen, keine vorab behaupteten API-Tatsachen:

- Offizielle IBKR-Dokumentation zu TWS API / IB Gateway / Client Portal API, jeweils **als unterschiedliche Produkte/Interfaces** behandeln, nicht zusammenwerfen.
- Identität und Lebenszyklus von `orderId`, `permId`, `execId`; Account-/Client-/Session-Kontext, Neustarts, Wiederverwendung, verspätete IDs, Fehler-/Korrekturbeobachtungen.
- Status-Updates versus Ausführungsberichte versus Sendeversuche; fehlende, doppelte und widersprüchliche Beobachtungen.
- Offizielle Beschreibungen von Snapshot-/Abfrage-Scopes, Zeitfenstern, Filtern, Paging, Reconnect und möglicher Unvollständigkeit. **Keine globale Vollständigkeit aus einem Response-Ende ableiten.**
- Dokumentierte Session-/Verbindungsmodelle und bekannte Beschränkungen für Recovery und Writer-Isolation; **keine Broker-Fencing-Garantie hineininterpretieren**.
- Offizielle Bezeichnungen von Paper-/Sandbox-/Testumgebungen; **nicht auswählen oder freigeben**.
- Kandidaten für später ausdrücklich zu genehmigende Leseoperationen. Nur dokumentieren, **nicht aufrufen**; Mutations-/Order-/Cancel-/Modify-Endpunkte gesondert als **nicht read-only** klassifizieren.
- Produkt-/API-Client-Version, Plattform, OS, Kontomodus, Veröffentlichungs-/Änderungsdatum: nur tatsächlich belegte Werte erfassen; fehlende Angaben `UNRECORDED`.

**ARCH-DEC-49 Doppelbeschaffung:** Für jede als `OFFICIAL_PRIMARY_OBTAINED` eingestufte Aussage Primärtext und eine **zweite unabhängig beschaffte Kopie derselben Version** anstreben. Unterschiedliche Abrufzeit/Fetch-Vorgang oder unabhängiger offizieller Archiv-/Release-Zugang dokumentieren; zweimal dieselbe Cacheantwort nicht als unabhängige Kopie ausgeben. Kopien mit SHA-256, Bytezahl, Original-URL, finaler URL, UTC-Abrufzeit, Version/Datum, Abrufmethode, Redirects, Content-Type und Quellenstatus erfassen. Falls zweite Kopie fehlt oder nicht versionsgleich ist: `SECOND_COPY_PENDING`/`VERSION_EQUIVALENCE_UNPROVEN`, keine E4-fähige Primärquellenbehauptung. Ein zweiter Download ist **kein empirischer Nachweis** des dokumentierten Verhaltens.

**Speicherung:** Nur öffentliches, nicht authentifiziertes Quellenmaterial und kurze zulässige Belegauszüge. Kein Volltext fremder geschützter Dokumentation ins Git kopieren, sofern Nutzungsrechte das nicht erlauben. Wenn Archivierung zulässig und sinnvoll: ausserhalb des Repositories in einem explizit dokumentierten lokalen Quellarchiv; im Git nur Manifest, Hashes, URLs, präzise Abschnittsanker, kurze Paraphrasen und minimale Zitate. Keine Rohpayloads, Secrets, Cookies, Tokens, Accountdaten oder personenbezogene Informationen ins Git. Kein neues Payload-Store für Brokerdaten. Bei unbekannten Archivierungsrechten: Metadaten/Hashes und präzise Quellenverweise statt Volltextkopie.

## 3. Evidenzklassifikation und Nachweisgrenzen

Für **jede** Aussage exakt eine nachvollziehbare Klasse verwenden:

- `OFFICIAL_DOCUMENTATION_OBTAINED` – Originalquelle wirklich abgerufen, Quelle/Version/Abschnitt nachprüfbar, ggf. zweite Kopie mit eigenem Status.
- `OFFICIAL_DOCUMENTATION_CONFLICT` – widersprüchliche oder versionsunklare offizielle Texte, beide belegen, keine stille Auflösung.
- `SECOND_COPY_PENDING` – Primärquelle vorhanden, zweite unabhängige gleichversionierte Kopie nicht nachgewiesen.
- `UNVERIFIED_RESEARCH_TOPIC` – Such-/Fragethema, keine Quelle erhalten.
- `DOCUMENTED_CLAIM_NOT_EMPIRICALLY_VERIFIED` – offizielle Dokumentation sagt etwas; tatsächliches Brokerverhalten bleibt ungetestet.
- `EXTERNAL_EVIDENCE_PENDING` / `NOT_EXECUTED` – für alle externen Broker-Claims/-Szenarien unverändert.

Diese Labels sind **Dokumentationsmetadaten**, keine Änderung der bestehenden ADFR-Verifikationsfelder, kein Broker-Completeness-Token und kein neuer kanonischer Contract. Eine offizielle Aussage kann den späteren Test-Oracle begründen, aber nicht die Eindeutigkeit von IDs, Vollständigkeit eines Snapshots, wirtschaftliche Finalität, sicheren Takeover oder erfolgreiche Recovery beweisen. Keine Dokumentationsquelle kann `XC-01`, `XC-08-IBKR`, `XC-02-PRED`, `XC-03-MATRIX` oder den operativen Rest von `XC-04` automatisch schliessen.

## 4. Geforderte Artefakte

Pfadkonventionen des Repositories bevorzugen; bei Bedarf `contracts/verification-plans/TRD-ARCH-03B5-*` anlegen:

1. `TRD-ARCH-03B5-public-research-authority-and-boundary.md`: Umfang der ausdrücklich begrenzten E0-Recherche, erlaubte öffentliche Abrufe, Stop-Bedingungen, E0–E4-Checkpoints weiterhin STOP, kein API-Zugriff.
2. `TRD-ARCH-03B5-official-source-register.md` und maschinenlesbarer **NON_RUNTIME_NON_WIRE** Quellenindex: je Quelle URL, Host, Titel, Produkt, belegte Version, Abschnittsanker, Zeitstempel, SHA-256/Bytes soweit tatsächlich beschafft, zweite Kopie und unabhängiger Beschaffungsweg, Lizenz-/Archivierungsstatus, Abrufstatus. Keine URLs, Hashes, Zeitstempel oder Versionen erfinden.
3. `TRD-ARCH-03B5-claim-to-source-matrix.md`: alle 27 Claims, zugeordnete offizielle Quellen und konkrete Abschnitte oder `NOT_ESTABLISHED`; dokumentierte Aussage von Hypothese und empirischer Verifikation klar trennen. Negative Aussagen nicht aus Schweigen ableiten.
4. `TRD-ARCH-03B5-scenario-coverage-and-oracle-limits.md`: alle 43 Szenarien, relevante offizielle Dokumentationsanker, verbleibende Evidenzlücken, benötigte spätere kontrollierte Tests; weiterhin `NOT_EXECUTED`. Besonders Unknown-Send, Snapshot-Lücken, Corrections/Busts, Reconnect und Writer-Isolation.
5. `TRD-ARCH-03B5-api-read-candidate-review.md`: **nur Kandidaten**, pro Kandidat exakter offiziell belegter Name, API-Produktfamilie, Dokumentationsanker, vermutete Nebenwirkungen/Unklarheiten, Scope und notwendige separate Entscheidung. Bei nicht belegten Namen keine erfundenen Operationen; `NO_READS_APPROVED`.
6. `TRD-ARCH-03B5-version-and-environment-findings.md`: offiziell belegte Produkt-/Versions-/Umgebungsbegriffe, offene Tupelfelder; **keine Auswahl** eines konkreten Kontos oder Testmodus.
7. `TRD-ARCH-03B5-evidence-gaps-and-next-approval-request.md`: klare Vorschläge für **separate** Nutzerentscheidungen (etwa offizielle Quellenanerkennung, Testumgebung, exakte Reads, Offline-Simulator), ohne sie zu ratifizieren oder E1–E4 zu starten. Quellenrecherche-Ergebnisse dürfen keine automatische GO-Entscheidung auslösen.
8. `TRD-ARCH-03B5-source-index.json` als `NON_RUNTIME_NON_WIRE_RESEARCH_INDEX`, nicht Registry und nicht Schema. Flags `broker_connection_authorized=false`, `read_only_broker_access_authorized=false`, `broker_testing_authorized=false`, `order_send_authorized=false`, `paper_trading_authorized=false`, `live_trading_authorized=false`, `economic_posting_approved=false`, `risk_release_approved=false`, `oms_transitions_approved=false`, `canonical_schemas_published=false`, `uniqueness_guaranteed=false`, `completeness_proof_approved=false`, `closes_xc_findings=false`. `external_broker_tests_executed=false`; E0–E4 Checkpoints STOP. `public_documentation_research_authorized=true` ist **kein** Stage-GO.
9. Notwendige Append-only-Verweise in Traceability, unresolved decisions, Dokumentations-README und Tests. Historische Quellen, Entscheidungen und Fragen nicht umschreiben.
10. `docs/architecture/TRD-ARCH-03B.5-report.md` und `docs/cursor-prompts/trading/TRD-ARCH-03B5-IBKR-Official-Documentation-Research.md` (ausgeführte Anweisung, vollständig archiviert).

## 5. Regressionstests / Prüfungen

Ergänze gezielte dokumentationsbezogene Tests an bestehenden Testorten, ohne Produktions-Rust-Code zu ändern. Teste mindestens:

- alle 27 Claim-IDs und 43 Szenario-IDs genau einmal im jeweiligen Mapping; Referenzen auf gültige Quellen-IDs; keine erfundenen URLs, SHA-Werte oder Fetch-Ergebnisse;
- Quellenstatus gegen tatsächliche Metadaten; bei `OFFICIAL_DOCUMENTATION_OBTAINED` zwingend echte Primärquelle und nachvollziehbare Provenienz; zweite Kopie nur bei separater Provenienz und nachgewiesener Versionsgleichheit;
- jede als dokumentiert bezeichnete Aussage hat einen präzisen Abschnittsanker; fehlende Quellen `NOT_ESTABLISHED`;
- `NOT_EXECUTED` und `EXTERNAL_EVIDENCE_PENDING` für sämtliche externen Tests; keine automatischen ADFR-Promotions;
- alle 14 ARCH-DEC-49..62 byteidentisch; alle 12 Quellen und Registry unverändert; ADFR 78/78 und Supersession unverändert;
- kein API-Read, keine Verbindung, keine Stage-GO, keine Produktionsimplementierung, keine Credentials/Secrets/Raw Broker Payloads in Git;
- Nicht-Wire-JSON-Index bleibt ausserhalb der kanonischen Registry; falls Guard-Allowlist angepasst wird, nur für diesen dokumentarischen Index;
- bestehende Schutztests gegen Alias-Unique, Broker-Completeness, wirtschaftliche Finalität und Dispatch bleiben wirksam.

Führe am Ende aus und protokolliere Exitcodes: `cargo fmt --all -- --check`, `cargo check --workspace --locked`, `cargo test --workspace --locked`, `cargo clippy --workspace --all-targets --locked -- -D warnings`. Wenn ein Test wegen fehlender PostgreSQL-Umgebung `ignored` ist, ausdrücklich als **nicht ausgeführt** ausweisen. Remote CI nur als `NOT_RUN` dokumentieren, nicht behaupten.

## 6. Acceptance-Gates (alle 12 erforderlich)

| Gate | Bestehensbedingung |
|---|---|
| A03B5-G01 | Baseline sauber, Vorgänger und Nutzerautorität geprüft, keine stillen Konflikte. |
| A03B5-G02 | 12/12 Quellen, Registry, ADFR und ARCH-DEC-01..62 unverändert und Hashes dokumentiert. |
| A03B5-G03 | Öffentliche Recherche klar von E0–E4 Stage-GO und Brokerzugriff abgegrenzt. |
| A03B5-G04 | Offizielle Quellen nur bei tatsächlichem Abruf als beschafft geführt; keine erfundenen URLs/Versionen. |
| A03B5-G05 | ARCH-DEC-49 Doppelbeschaffung mit Provenienz und fehlender Zweitkopie korrekt abgebildet. |
| A03B5-G06 | Alle 27 Claims mit Quellenanker oder expliziter Evidenzlücke erfasst; keine empirische Aufwertung. |
| A03B5-G07 | Alle 43 Szenarien mit Oracle-Grenzen, weiterhin NOT_EXECUTED. |
| A03B5-G08 | API-Read-Kandidaten nur dokumentiert; kein benannter oder genehmigter Brokerzugriff. |
| A03B5-G09 | Produkt-/Version-/Umgebungskontext ohne Vermischung oder voreilige Testumgebungswahl. |
| A03B5-G10 | Keine Runtime-/Schema-/Broker-/Order-/Risk-/OMS-Änderung; sämtliche Sicherheitsflags fail-closed. |
| A03B5-G11 | Vier lokale Cargo-Kommandos erfolgreich; Dokumentationstests und ignorierte Tests transparent berichtet. |
| A03B5-G12 | Vollständiger Bericht und Prompt archiviert, genau **ein** lokaler Commit, Worktree sauber, **NO PUSH**. |

Bei einem nicht bestandenen Gate: `VERDICT: BLOCKED` oder `PARTIAL`, kein PASS behaupten; keine unautorisierte Folgearbeit. Wenn externe öffentliche Dokumentationsrecherche wegen Tool-/Netzwerk-/Lizenzgrenzen nicht möglich ist, **keine Quellen erfinden**: die offline erstellbaren Artefakte und Lücken dokumentieren; G04/G05 nur soweit nachweislich erfüllt und Endurteil entsprechend `PARTIAL`.

## 7. Bericht und Git

Bericht enthält: `VERDICT`, Branch, `BASELINE_HEAD`, `FINAL_HEAD` (falls bekannt), Commit-Subject, `PUSHED: NO`, sauberen Worktree; explizite E0-Research-Authority und STOP-Checkpoints; pro Quelle reale URL/Version/Abrufstatus/Zweitkopie/Hashes (keine erfundenen Werte); 27-Claim-/43-Szenario-Abdeckung; offene Fragen; unveränderte XC-Konflikte; Schutz-Hashes und Source-Manifest; alle geänderten Dateien; Cargo-Exitcodes, Testzahlen, ignorierte Tests, nicht ausgeführte Remote CI und Broker-Tests; Abweichungen; 12 Gate-Ergebnisse.

**Einziger erlaubter Commit nach allen Gates:** `TRD-ARCH-03B.5: Research Official IBKR Documentation`.

Keine weiteren Commits, **kein Push**, keine Git-Historienänderung, keine destruktiven Befehle. Commit nur projektbezogene Dokumentationsartefakte und erforderliche Tests. Wenn der Bericht den eigenen Commit-Hash nicht kennen kann, ihn im Cursor-Abschluss separat ausgeben. **Nach diesem Meilenstein keine eigenmächtige nächste Phase beginnen.**
