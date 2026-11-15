# Trading Platform — DET-09, DET-10 und Cross-Contract Architecture Review

**Version:** 1.0-draft  
**Datum:** 2026-11-14  
**Status:** Eigenständig entschiedener Architekturentwurf, technische Abnahme ausstehend; **keine Implementierungsfreigabe**.  
**Quellenbasis:** bestätigte OPEN-01–06 und REV-20; DET-01/02 aus den fachlichen Vertragsentwürfen im Gespräch; überprüftes Dokument `DET-03-bis-DET-08-Architecture-Contracts-v1.0.md` (v1.0-draft). Kein Repository-Code-Audit.

## 0. Normsprache und Geltung

**MUSS** = verpflichtende Anforderung dieses Vertragsentwurfs; **SOLL** = begründbarer Standard; **KANN** = Option. `Dxx` sind selbstständig festgelegte Designentscheidungen, `INV` Sicherheitsinvarianten, `T` Testfälle. Parameter, konkrete Schwellwerte und DB-Schemas bleiben vor Implementierung zu versionieren und abzunehmen. Die Begriffe *Evidenz*, *Snapshot*, *Finalität*, *Risk Reduction* und *Readiness* werden hier fachlich verwendet, nicht als Behauptung einer Broker-Garantie.

# DET-09 — Market Data Integrity & Valuation Contract

## 9.1 Zweck und Systemgrenze

DET-09 liefert dem OMS, der Risk Engine und dem Simulator **qualitätsbewertete, versionierte Bewertungsinputs**. Datenbeschaffung und Provider-Adapter können ausfallen; die Freigabe von Orders darf dadurch nicht auf unbewiesene Preise zurückfallen. DET-09 trifft **keine** autonome Handelsentscheidung und vergibt **keine** Risk-Overrides.

## 9.2 Architekturentscheidungen

- **DET-09-D01 — Providerunabhängige Datenidentität:** Jede Beobachtung besitzt `observation_id`, `provider_id`, `instrument_id`, `venue`, `data_type`, `event_time`, `received_at`, `sequence_or_snapshot_ref`, `raw_payload_hash`, `normalization_version`. Instrumentidentität und Handelsplatz werden explizit modelliert; ein Symbol allein genügt nicht.
- **DET-09-D02 — Quality Envelope:** Jede normalisierte Beobachtung trägt `quality_status` (`VALID`, `DEGRADED`, `INVALID`, `UNKNOWN`), `quality_reasons`, `max_age_policy_ref`, Provenienz und ggf. `supersedes_observation_id`. Qualitätsregeln sind versioniert und je Instrument/Marktphase/Verwendung konfigurierbar.
- **DET-09-D03 — Konservative Datenfreigabe:** Eine Risk- oder Submission-Prüfung darf nur Daten verwenden, die ihre expliziten Frische-, Vollständigkeits- und Plausibilitätsanforderungen erfüllen. Fallbacks sind nur erlaubt, wenn die jeweilige Policy einen konservativen Bewertungsweg nachweisbar definiert; sonst `INDETERMINATE`/Fail-Closed.
- **DET-09-D04 — Deterministische Valuation:** Preis, FX, Corporate-Action-Anpassung, Rundung, Währung, Handelskalender und Bewertungszeitpunkt werden als versionierte Inputs geführt. Binäre Floating-Point-Zwischenwerte dürfen keine unkontrollierten Risk-Limit-Entscheidungen bestimmen; geeignete Dezimal-/Fixed-Point-Typen und definierte Rundung sind erforderlich.
- **DET-09-D05 — Zeit- und Reihenfolgenmodell:** Exchange-/Provider-Event-Time und lokale Receive-Time bleiben getrennt. Verspätete und out-of-order Daten dürfen den neuesten gültigen Snapshot nicht unbemerkt ersetzen. Sequence-Gaps, Clock-Skew und inkonsistente Quotes lösen Quality-Degradation aus.
- **DET-09-D06 — Reproduzierbarkeit:** Jede relevante Risk Evaluation referenziert den unveränderlichen Daten-Snapshot oder einen eindeutig rekonstruierbaren Input-Satz inklusive Policy-/Valuation-Version und Hash. Replay benötigt keine aktuelle Live-Quote.
- **DET-09-D07 — Trennung von Research und Execution:** Research-/Backtesting-Daten dürfen nicht ohne expliziten, geprüften Promotion-Prozess als Live-Risk-Evidenz gelten. Paper- und Live-Namespaces sind getrennt.

## 9.3 Formales Bewertungs- und Qualitätsmodell

Für eine Order `o` und ihren Bewertungszeitpunkt `t` gilt:

`Eligible(o,t) := ∀ d ∈ RequiredInputs(o): Quality(d, Policy(o), t) = ACCEPTABLE`.

`RiskInput(o,t) := (OrderVersion, InstrumentMasterVersion, MarketSnapshotId, FxSnapshotId, PolicyVersion, ValuationVersion, ClockReference)`.

`EvaluateRisk(RiskInput) = ALLOW` ist nur bei erfülltem `Eligible` möglich; ein `UNKNOWN` ist **kein** `ALLOW`.

Beispiel für Preisprüfung (Parameter **nicht** vorab festgelegt): `age_ms <= max_age_ms`, Quote nicht crossed/invalid, positive zulässige Preise, korrekte Währung/Einheit, vollständige Instrumentzuordnung. Eine positive Quote allein beweist weder Handelbarkeit noch ausreichende Liquidität.

Bei FX gilt: Quelle, Basis-/Kurswährung, Quotierungsrichtung, Frische und Rundung müssen explizit sein. Kein stillschweigender `1:1`-Fallback.

## 9.4 Persistenz- und Schnittstellen-Referenzdesign

- `market_observations(observation_id, instrument_id, provider_id, venue, kind, event_time, received_at, sequence_ref, payload_hash, raw_ref)`
- `market_quality_assessments(assessment_id, observation_id, ruleset_version, status, reasons, evaluated_at)`
- `valuation_snapshots(snapshot_id, instrument_master_version, market_refs, fx_refs, valuation_version, content_hash, created_at)`
- `risk_evaluation_inputs(evaluation_id, order_id, order_version, snapshot_id, policy_version, input_hash)`

Schnittstelle: `evaluate_snapshot_quality(snapshot, purpose, policy) -> Acceptable | Degraded(reason) | Indeterminate(reason)`; `value_order(order, snapshot, valuation_policy) -> Valuation | Error`.

## 9.5 Fehler- und Recovery-Regeln

| Fehler | Verbindliche Reaktion |
|---|---|
| Fehlende/zu alte Quote | Neue risikosteigernde Freigabe blockieren, ausser explizit zugelassener konservativer Fallback |
| Sequence-Gap / unvollständiger Feed | Betroffene Datenquelle degradieren, Re-Snapshot/Backfill anfordern |
| FX fehlt oder ist widersprüchlich | Bewertung unbestimmt; keine stillschweigende Umrechnung |
| Corporate Action / Instrument-Mapping unklar | Instrumentbezogene Submission sperren, Stammdaten abgleichen |
| Feed reconnect | Synchronisieren, Sequenz/Abdeckung validieren, Quality Gate erneut prüfen |
| Replay | Ursprüngliche Snapshots/Versionen verwenden, nicht aktuelle Kurse |

## 9.6 Sicherheitsinvarianten

- **MD-INV-01:** Keine risikosteigernde Submission mit unzulässigen Bewertungsinputs.
- **MD-INV-02:** Jeder für Risk relevante Bewertungsinput ist versioniert und rekonstruierbar.
- **MD-INV-03:** Event-Time und Receive-Time werden nicht gleichgesetzt.
- **MD-INV-04:** Ein veraltetes oder verspätetes Update ersetzt nicht stillschweigend einen gültigeren Snapshot.
- **MD-INV-05:** Kein impliziter FX- oder Preis-Fallback.
- **MD-INV-06:** Research-/Paper-Daten werden nicht unautorisiert als Live-Evidenz verwendet.

## 9.7 Testmatrix

| ID | Test | Erwartung |
|---|---|---|
| MD-T01 | Stale Quote | Fail-Closed oder expliziter Policy-Fallback |
| MD-T02 | Out-of-order Update | Kein unbemerkter Snapshot-Rückschritt |
| MD-T03 | Sequence-Gap | Quality-Degradation |
| MD-T04 | Crossed/invalid Quote | Keine ungeprüfte Valuation |
| MD-T05 | Fehlender FX-Kurs | Keine 1:1-Annahme |
| MD-T06 | Unterschiedliche Rundungsgrenzen | Deterministisches Limitresultat |
| MD-T07 | Reconnect/Backfill | Erst nach belegter Abdeckung wieder freigeben |
| MD-T08 | Identischer Snapshot-Replay | Identischer Input-Hash und Valuation-Output |
| MD-T09 | Instrument-/Corporate-Action-Konflikt | Sperre/Incident statt stiller Anpassung |
| MD-T10 | Research-Input im Live-Scope | Ablehnung |

# DET-10 — Audit, Incident & Operational Control Contract

## 10.1 Zweck und Systemgrenze

DET-10 stellt sicher, dass sicherheitsrelevante Entscheidungen, Bedienhandlungen, Systemänderungen, Recovery und Notfallaktionen nachvollziehbar und überprüfbar sind. Er ist **kein Ersatz** für DET-06 Readiness oder DET-07 Dispatch; er setzt zusätzliche Autorisierung, Nachweisführung und Incident-Lifecycle durch.

## 10.2 Architekturentscheidungen

- **DET-10-D01 — Append-only Audit Evidence:** Alle sicherheitskritischen Commands, Risk Evaluations, Overrides, Submission Attempts, Brokerbeobachtungen, Reconciliation-Entscheidungen, Halts, Resets und Policy-Änderungen erzeugen unveränderliche Audit-Einträge mit Identität, Zeit, Korrelation, Version und Evidenzreferenz. Korrekturen sind neue Einträge, kein stilles Update.
- **DET-10-D02 — Zwei Ebenen von Nachweisen:** Transaktionale Domain Events und Outbox werden atomar mit fachlichen Änderungen geschrieben; ein separater Audit-/Observability-Export darf asynchron sein. Verlust des Exports darf die autoritative Auditquelle nicht löschen.
- **DET-10-D03 — Incident State Machine:** `DETECTED -> TRIAGED -> CONTAINED -> INVESTIGATING -> RESOLVED -> CLOSED`, mit explizitem `REOPENED`/neuem verknüpften Incident bei späterer Evidenz. Kritische Vorfälle können sofort Containment erzwingen; Status ist nicht gleich Trading Readiness.
- **DET-10-D04 — Least Privilege und Four-Eyes für kritische Eingriffe:** Rollenbasierte und ressourcenbezogene Autorisierung; für besonders kritische manuelle Freigaben/Overrides/Halt-Resets sind getrennte Initiator- und Approver-Identitäten als **vorgeschlagene Standardpolicy** vorgesehen. Einzelanwenderbetrieb benötigt dokumentierten Break-Glass-Pfad mit strengeren Einschränkungen und nachgelagerter Prüfung; er darf harte Safety-Invarianten nicht umgehen.
- **DET-10-D05 — Break-Glass begrenzt:** Nur vorab definierte Notfallaktionen, zeitlich befristete Autorisierung, begründeter Zweck, vollständiger Audit Trail und anschliessende Reconciliation. Kein generelles `ignore_risk` und kein Umgehen von Fencing.
- **DET-10-D06 — Operational Kill Switch:** Globaler und accountbezogener Halt blockiert neue risikosteigernde Dispatches serverseitig. Sichere Cancel-/Reconciliation-Aktionen können nach separaten Gates weiterlaufen. Ein Halt ist keine Bestätigung, dass Brokerorders storniert wurden.
- **DET-10-D07 — Wiederaufnahmeprotokoll:** Kritische Halts benötigen Ursachenbehebung, abgeschlossene erforderliche Reconciliation, gültige Broker-/Risk-/Market-Data-Gates, aktuelle Writer Authority und gegebenenfalls expliziten manuellen Reset. Kein automatischer Restart nur wegen wiederhergestellter Netzwerkverbindung.
- **DET-10-D08 — Nachweis- und Retentionspolitik:** Tamper-evident Verkettung oder signierte Batch-Manifestierung, kontrollierte Zugriffsrechte, verschlüsselte Backups, Retention nach schweizerischen/gegebenenfalls weiteren anwendbaren Vorgaben. Konkrete Aufbewahrungsfristen sind **nicht** ohne Compliance-Prüfung festgelegt. Keine Geheimnisse oder unnötigen personenbezogenen Daten in Logs.
- **DET-10-D09 — Beobachtbarkeit:** Korrelations-IDs über OMS/Risk/Outbox/Adapter/Reconciliation; Metriken für Unknown Submissions, stale data, Fencing-Konflikte, Audit-Lücken, Risk Drift, Dead-Letter, Readiness und Broker-Synchronisationsalter. Monitoring ist nicht autoritativ für die Orderbuchhaltung.

## 10.3 Formale Betriebsregeln

`CanResume(scope) := CausesResolved ∧ RequiredReconciliationComplete ∧ RiskGate ∧ MarketDataGate ∧ BrokerIdentityGate ∧ CurrentWriter ∧ RequiredHumanReset`.

`CanSubmitNewRisk(scope) := DET06.CanSubmit(scope) ∧ ¬ActiveBlockingHalt(scope) ∧ DET07.SubmissionGate(scope)`.

Ein Incident kann nachträglich ergänzt werden, ohne die ursprünglichen Beobachtungen zu verändern. Auditbelege müssen nach Scope, Order-ID, Attempt-ID, Execution-ID und Correlation-ID auffindbar sein.

## 10.4 Persistenz-Referenzdesign

- `audit_events(audit_id, occurred_at, recorded_at, actor_type, actor_id, scope, action, outcome, causation_id, correlation_id, evidence_ref, payload_hash, previous_hash)`
- `incidents(incident_id, severity, state, scope, detected_at, owner_ref, cause_ref, resolution_ref)`
- `incident_events(event_id, incident_id, event_type, actor_id, evidence_ref, created_at)`
- `operational_halts(halt_id, scope, reason, severity, created_by, created_at, cleared_at, clear_authorization_ref)`
- `approval_requests(request_id, action, scope, initiator, approver, expires_at, status, policy_version)`

Hash-Verkettung allein garantiert keine externe Unveränderlichkeit; geeignete Schlüssel-/Backup-/WORM-Strategie muss ergänzend spezifiziert werden.

## 10.5 Fehler und Recovery

| Fehler | Reaktion |
|---|---|
| Audit-Export nicht verfügbar | Autoritative transaktionale Auditquelle erhalten; Export später nachholen; Alarmierung |
| Audit-Lücke in Pflichtdaten | Kritischen Incident eröffnen, betroffene Freigaben sperren |
| Unautorisierter Reset | Ablehnen, auditieren, ggf. Incident |
| Kritischer Halt während Submission | Dispatch-Gate sperren; bereits möglicherweise gesendete Orders via DET-08 abgleichen |
| Incident während Recovery | Recovery nicht automatisch als erfolgreich markieren |
| Backups inkonsistent | Restore in Isolation prüfen, keine Live-Freigabe ohne Reconciliation |

## 10.6 Sicherheitsinvarianten

- **OPS-INV-01:** Kein sicherheitskritischer fachlicher Commit ohne erforderliche transaktionale Audit-Evidenz.
- **OPS-INV-02:** Kein manueller Reset ohne gültige Autorisierung und erfüllte Readiness-Gates.
- **OPS-INV-03:** Kein Break-Glass umgeht harte Risk- oder Fencing-Invarianten.
- **OPS-INV-04:** Ein Halt bedeutet niemals automatisch Broker-Cancel-Finalität.
- **OPS-INV-05:** Kein stilles Löschen/Überschreiben historischer sicherheitskritischer Evidenz.
- **OPS-INV-06:** Keine automatisierte Wiederaufnahme bei kritischer ungeklärter Reconciliation.
- **OPS-INV-07:** Operative Änderungen sind mit Actor, Scope, Version und Ursache nachvollziehbar.

## 10.7 Testmatrix

| ID | Test | Erwartung |
|---|---|---|
| OPS-T01 | Commit einer Risk-Policy-Änderung | Audit-Ereignis atomar vorhanden |
| OPS-T02 | Audit-Exporter fällt aus | Fachliche Auditquelle bleibt erhalten |
| OPS-T03 | Unautorisierter Reset | Verweigert und protokolliert |
| OPS-T04 | Kritischer Halt vor Dispatch | Keine neue risikosteigernde Submission |
| OPS-T05 | Halt während unklarem Versand | Reconciliation, kein Blind-Retry |
| OPS-T06 | Break-Glass versucht Risk-Invariante zu umgehen | Verweigert |
| OPS-T07 | Incident-Eskalation und spätere Evidenz | Historie append-only |
| OPS-T08 | Recovery ohne erforderlichen Reset | Trading bleibt blockiert |
| OPS-T09 | Audit-Hash/Manifest manipuliert | Integritätsprüfung schlägt an |
| OPS-T10 | Restore aus Backup | Isolation, Reconciliation, Gate-Prüfung |
| OPS-T11 | Geheimnisse in Telemetrie-Payload | Redaktions-/Leak-Test schlägt an |
| OPS-T12 | Actor-/Scope-Berechtigung abgelaufen | Operation verweigert |

# Cross-Contract Architecture Review — DET-01 bis DET-10

## CR-01 Prüfumfang und Aussagekraft

Geprüft wurden die im Gespräch spezifizierten fachlichen DET-01/02-Anforderungen und der tatsächliche Text des bestehenden DET-03–08-Dokuments sowie die vorstehenden DET-09/10-Entwürfe. Dies ist ein **statischer Architekturabgleich**, keine Code-, Broker-API- oder formale Verifikation. Aussagen über reale Brokersemantik benötigen später Adapter-Tests und dokumentierte IBKR-Quellen.

## CR-02 Verantwortungsmatrix

| Verantwortlichkeit | Autoritativer Vertrag | Benötigte Nachbarverträge |
|---|---|---|
| Order Workflow und Ledger | DET-01 | DET-03,04,08 |
| Risk Evaluation und Reservation | DET-02 | DET-01,04,09 |
| Brokeridentität | DET-03 | DET-05,07,08 |
| Lokaler Commit/Outbox/Inbox | DET-04 | DET-01,02,07,10 |
| Brokerprotokoll und Capabilities | DET-05 | DET-03,06,07,08 |
| Aktionsspezifische Readiness | DET-06 | DET-02,05,07,08,09,10 |
| Single-Writer/Submission | DET-07 | DET-01,02,03,04,05,06,08 |
| Reconciliation und Recovery | DET-08 | DET-01–07,09,10 |
| Bewertungsinputs und Qualität | DET-09 | DET-02,05,06 |
| Audit, Incidents und operative Freigaben | DET-10 | DET-01–09 |

## CR-03 Bestätigte Übereinstimmungen

- **CR-OK-01:** DET-01,03,07,08 verwenden interne Order-UUID und evidenzbasierte Brokerzuordnung statt naiver Broker-ID-Autorität.
- **CR-OK-02:** DET-01,02,04 verlangen lokale Atomizität von OMS/Risk/Outbox; DET-07 vermeidet externe Brokeraufrufe innerhalb dieser Transaktion.
- **CR-OK-03:** DET-01,03,07,08 verbieten Blind-Retries bei unklarem Submission-Ergebnis.
- **CR-OK-04:** DET-01,02,08 akzeptieren Late Fills/Korrekturen nach Workflow-Abschluss.
- **CR-OK-05:** DET-06,07,08,10 behandeln Trading Halt als Dispatch-Sperre, nicht als wirtschaftliche Brokerfinalität.
- **CR-OK-06:** DET-02,09 teilen Policy-abhängiges Fail-Closed bei ungeeigneten Bewertungsinputs.

## CR-04 Offene Konflikte und Blocker (nicht als gelöst behaupten)

| ID | Priorität | Befund | Erforderliche Auflösung |
|---|---|---|---|
| CR-B01 | P0 | DET-07 verlangt vor externem Send persistierten `DISPATCH_STARTED`; Lease/Epoch in DB kann einen bereits laufenden externen Aufruf nicht physisch stoppen | Präzises Lease- und Failover-Protokoll, Broker-Session-Exklusivität, konservative Übernahme und Fault-Injection-Nachweis |
| CR-B02 | P0 | DET-01/02/04 fordern atomare Order/Risk/Outbox-Änderung, ohne gemeinsame Transaktions-API und Sperrreihenfolge vollständig zu definieren | Unit-of-Work-Vertrag, Lock-Ordering, Isolation Level, Commit-/Retry-Semantik |
| CR-B03 | P0 | Late Fill nach `CLOSED` kann bereits freigegebene Risk Reservation und Positionsprojektion widersprechen | Versionierte kompensierende Risikobuchung, Incident-Regel und Reconciliation-Gate |
| CR-B04 | P0 | DET-03 Identity Resolver benötigt exakte IBKR-Scopes; `permId`, `orderId`, `execId`-Eindeutigkeit und Wiederverwendung sind nicht brokerseitig verifiziert | Capability-/Identity-Matrix aus Adaptertests und Dokumentation; Constraints erst danach einfrieren |
| CR-B05 | P1 | DET-06/10 definieren Reset und Break-Glass, aber keine final abgenommene Solo-Operator-Policy | Rollen-/Scope-/Expiry-Matrix, sicherer Einzelanwender-Break-Glass ohne Hard-Invariant-Bypass |
| CR-B06 | P1 | DET-02/09 fehlen konkrete Preis-, FX-, Margin- und Rundungsparameter | Versionierte Instrument- und Bewertungs-Policy samt Grenzfalltests |
| CR-B07 | P1 | DET-08 Snapshots/Watermarks haben noch keine präzise Abdeckungsgarantie | Source-Completeness-Protokoll und negative Tests für unvollständige Brokerabfragen |
| CR-B08 | P1 | DET-10 Audit-Evidenz und DET-04 Outbox könnten doppelte oder nicht atomar verknüpfte Auditströme erzeugen | Autoritativen Audit-Event-Write in derselben UoW festlegen; Export als abgeleitete Projektion |
| CR-B09 | P1 | Cancel/Modify und Risk-Reduction-Aktionen brauchen eigene Dispatch-Gates und Race-Handling | Explizite Action-Policy und State/Identity-Preconditions je Brokercommand |
| CR-B10 | P2 | Enum-/Schema-Namen und `RECONCILIATION_REQUIRED` als Order-/Reservation-/Run-Status sind noch nicht kanonisiert | Gemeinsames typisiertes Domain Glossary und Schema Registry |

**Architekturfreigabe:** `NOT_READY_FOR_IMPLEMENTATION` für sicherheitskritische Order-Submission, solange P0-Blocker offen sind. Dies verhindert nicht isolierte Foundation-/Test-/Dokumentationsarbeit nach separater Freigabe.

## CR-05 Systemweite Sicherheitsinvarianten

- **SYS-X-INV-01:** Kein neuer risikosteigernder Brokerdispatch ohne erfolgreiche, aktuelle Order-, Risk-, Data-, Readiness-, Identity- und Writer-Gates.
- **SYS-X-INV-02:** Unbekannter Broker-Submission-Ausgang impliziert Reconciliation statt blindem Retry.
- **SYS-X-INV-03:** Keine wirtschaftliche Execution wird aufgrund von Brokerstatus, Workflow Closure oder Zustellungsduplikaten verworfen oder doppelt gebucht.
- **SYS-X-INV-04:** Keine Budgetfreigabe ohne hinreichende wirtschaftliche Evidenz; Late Fills erzeugen auditierte Korrekturen.
- **SYS-X-INV-05:** Alle fachlichen lokalen Änderungen und ihre erforderlichen Outbox-/Audit-Belege committen atomar.
- **SYS-X-INV-06:** Keine externe Brokeraktion innerhalb einer lokalen DB-Transaktion.
- **SYS-X-INV-07:** Hard Risk Invariants, globale Halts und Fencing sind nicht durch KI oder Break-Glass umgehbar.
- **SYS-X-INV-08:** Recovery erfordert nachgewiesene Identität, Datenabdeckung, Risk-Reconciliation und erlaubte Readiness-Aktionen.

## CR-06 End-to-End Testgates

| Gate | Minimaler Nachweis |
|---|---|
| E2E-01 | Zwei konkurrierende Orders mit überlappendem Budget: keine Überreservierung |
| E2E-02 | Crash vor/bei/nach externem Dispatch: kein Blind-Retry |
| E2E-03 | Failover zwischen Writer-Epochen mit laufendem Brokeraufruf: keine unkontrollierte Doppelübermittlung |
| E2E-04 | Cancel + Late Fill + Bust: Ledger, Position und Risk konsistent oder konservativ isoliert |
| E2E-05 | Broker reconnect mit Session-ID-Wiederverwendung: korrekte Identity Resolution |
| E2E-06 | At-least-once JetStream Duplikat und Out-of-order Event: ein wirtschaftlicher Effekt |
| E2E-07 | Stale Market Data und fehlender FX-Kurs: keine neue Risikofreigabe |
| E2E-08 | Global Halt + unbekannter Submission-Ausgang: Dispatch blockiert, Recovery auditierbar |
| E2E-09 | Restore + unvollständiger Broker-Snapshot: keine automatische Trading-Freigabe |
| E2E-10 | Deterministisches Replay aller versionierten Inputs: identische fachliche Projektion |

# ADFR — Neue Einträge und Konsolidierungsregister

## ADFR-DET09

| ID | Entscheidung | Alternativen verworfen | Konsequenz |
|---|---|---|---|
| ADFR-DET09-001 | Providerunabhängige Beobachtungsidentität | Nur Symbol/Zeit | Eindeutige Provenienz nötig |
| ADFR-DET09-002 | Versionierter Quality Envelope | Ungelabelte Daten | Quality-Checks als Freigabegate |
| ADFR-DET09-003 | Fail-Closed und explizite Fallbacks | Last-known-price immer zulassen | Weniger Verfügbarkeit, höhere Sicherheit |
| ADFR-DET09-004 | Deterministische Valuation | Ad-hoc Float-Berechnung | Versionierte Rundung/FX |
| ADFR-DET09-005 | Event-/Receive-Time getrennt | Arrival-time-wins | Out-of-order-Behandlung |
| ADFR-DET09-006 | Snapshot-gebundenes Replay | Live-Quotes beim Replay | Auditierbare Inputs |
| ADFR-DET09-007 | Research/Live-Trennung | Gemeinsamer ungeprüfter Feed | Promotion-Gates |

## ADFR-DET10

| ID | Entscheidung | Alternativen verworfen | Konsequenz |
|---|---|---|---|
| ADFR-DET10-001 | Append-only Audit | Überschreibbare Logs | Historien-/Speicherbedarf |
| ADFR-DET10-002 | Transaktionale Auditquelle, asynchroner Export | Export als alleinige Autorität | UoW-Anbindung |
| ADFR-DET10-003 | Persistierter Incident-Lifecycle | Freitext-Tickets allein | Zustands-/Berechtigungsregeln |
| ADFR-DET10-004 | Least Privilege, kritische Dual Approval als Standard | Unbeschränkter Admin | Solo-Operator-Ausnahme definieren |
| ADFR-DET10-005 | Begrenztes Break-Glass | Universeller Notfall-Bypass | Harte Invarianten bleiben |
| ADFR-DET10-006 | Kill Switch mit separaten Action Gates | Alle Aktionen stoppen | Sichere Cancel/Reconcile möglich |
| ADFR-DET10-007 | Evidenzbasierte Wiederaufnahme | Auto-Restart bei Reconnect | Konservativer Betrieb |
| ADFR-DET10-008 | Tamper-evident Audit/Retention | Unkontrollierte Logs | Retention-/Compliance-Review |
| ADFR-DET10-009 | Korrelierte Observability | Unverbundene Metriken | Diagnose/Incident-Triage |

## ADFR-CROSS

| ID | Entscheidung / offene Aufgabe | Status |
|---|---|---|
| ADFR-CROSS-001 | Ein gemeinsamer transaktionaler OMS/Risk/Outbox/Audit-Unit-of-Work | OPEN P0 |
| ADFR-CROSS-002 | Fenced Dispatch + konservatives Failover unter in-flight Brokeraufrufen | OPEN P0 |
| ADFR-CROSS-003 | Late-Fill-Kompensation nach Risk-Release | OPEN P0 |
| ADFR-CROSS-004 | IBKR-Identitäts- und Eindeutigkeitsmatrix | OPEN P0 |
| ADFR-CROSS-005 | Kanonisches Domain Glossary und Schema Registry | OPEN P2 |
| ADFR-CROSS-006 | Abnahmefähige Cross-Contract-CI-Gates | OPEN P1 |

**ADFR-Gesamtstand:** Bestehende 10 DET-01- und 10 DET-02-Entscheidungen (aus Gespräch), 22 Einträge aus dem geprüften DET-03–08-Dokument, 7 neue DET-09- und 9 neue DET-10-Entscheidungen = **58 fachliche Entscheidungs-/Registereinträge** vor eventueller Dublettenbereinigung. Zusätzlich 6 Cross-Contract-Registereinträge. Die bestehende Datei enthält ihre eigenen 22 ADFR-Zeilen; sie werden hier nicht wortgleich dupliziert. Eine einzige kanonische, maschinenlesbare ADFR-Datei ist als Folgeschritt offen.

# Nächste Architekturarbeit — ohne Implementierungsfreigabe

1. **P0-Blocker CR-B01–B04 auflösen**, jeweils mit formaler Entscheidung, Evidenz und reproduzierbarem Test.
2. Kanonische Rust-Domain-Typen und SQL-Schemas als gemeinsame Referenzspezifikation erstellen.
3. IBKR-Adapteridentität und Broker-Capabilities mit offiziellen Dokumenten und Sandbox-/Paper-Tests verifizieren.
4. Risk-/Market-Data-Parameter und Rundungsregeln festlegen.
5. Vollständiges maschinenlesbares ADFR mit Referenzen auf DET-01–10 konsolidieren.
6. Erst danach separate Freigabe für safety-kritische Implementierungsmeilensteine einholen.

**Endstatus:** `DET-01..10 DECISION-DRAFTED / CROSS-CONTRACT-REVIEWED / NOT_IMPLEMENTATION_APPROVED`.
