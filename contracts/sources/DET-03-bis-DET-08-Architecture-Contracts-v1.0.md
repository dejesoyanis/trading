# Trading Platform — DET-03 bis DET-08 Architecture Contracts

**Version:** 1.0-draft  
**Datum:** 2026-11-14  
**Status:** Autonom ausgearbeitete Architekturentscheidungen, zur technischen Abnahme; keine Implementierungsfreigabe  
**Grundlagen:** REV-20, OPEN-01 bis OPEN-06, DET-01 und DET-02; modularer Rust-Monolith, PostgreSQL, NATS JetStream, IBKR-Adapter.

## 0. Geltungsbereich und Normsprache

**MUSS** = verbindliche Vertragsanforderung im Entwurf; **SOLL** = empfohlener Standard, Abweichung begründen; **KANN** = Option. Exakte Enum- und Tabellennamen sind Referenzdesign und werden bei der Cross-Contract-Abnahme eingefroren. Diese Dokumentation trifft die delegierten Architekturentscheidungen eigenständig, behauptet aber weder bereits erfolgte Implementierung noch formale Verifikation.

### Übergreifende Invarianten

- **SYS-INV-01:** Keine neue Broker-Submission ohne gültige Orderfreigabe, Risk Reservation, Trading Readiness und aktuelle Submission Authority.
- **SYS-INV-02:** Ein unbekanntes Submission-Ergebnis ist niemals Beweis für Nichtübermittlung; keine blinden Retries.
- **SYS-INV-03:** Externe Brokeraufrufe finden niemals innerhalb einer PostgreSQL-Transaktion statt.
- **SYS-INV-04:** Wirtschaftliche Fills werden dedupliziert, auditierbar und auch nach Workflow-Abschluss verarbeitet.
- **SYS-INV-05:** Reconciliation gibt nur nach ausreichender Evidenz wieder frei; Unklarheit führt zu Sperre/Quarantäne.
- **SYS-INV-06:** At-least-once-Zustellung wird durch Idempotenz, nicht durch angenommene Exactly-once-Garantie beherrscht.
- **SYS-INV-07:** Safety-Halts, harte Risk Policies und Fencing dürfen nicht von KI-Agenten übersteuert werden.

## DET-03 — Broker Identity Contract

### Entscheidung und Begründung

**DET-03-D01:** Interne `order_id: UUID` ist dauerhaft autoritativ. Broker-IDs sind kontextgebundene Aliasse und dürfen die interne Identität nicht ersetzen. `permId` ist bei IBKR eine wichtige dauerhafte Zuordnung, aber nicht als einzige systemweite Identität ausreichend.

**DET-03-D02:** Broker-Alias besteht aus `broker`, `account`, `client_id`, `session_epoch`, `broker_order_id`, optional `perm_id`, `external_order_ref`, `first_seen_at`, `last_seen_at`, `evidence_id`. Die genauen IBKR-Felder werden beim Adapter-Contract validiert. Ein Broker-Order-ID-Wert ohne Kontext ist keine eindeutige Orderidentität.

**DET-03-D03:** Separate Identitäten für `order_id`, `submission_attempt_id`, `command_id`, `execution_id`, `broker_execution_key` und `correction_id`. Idempotency Keys gelten pro Command- und Submission-Scope; gleiche Keys mit anderem Payload-Hash sind Konflikte.

**DET-03-D04:** Zuordnung über einen evidenzbasierten Identity Resolver mit Ergebnissen `MATCHED`, `UNMATCHED`, `AMBIGUOUS`, `CONFLICT`. Nur `MATCHED` darf automatisch wirtschaftlich gebucht werden. Unbekannte Brokerorders werden quarantänisiert und niemals stillschweigend einer bestehenden Order zugeschlagen.

**DET-03-D05:** Einmal bestätigte Identitätsbeziehungen werden nicht still überschrieben. Korrekturen erfolgen append-only mit Referenz auf frühere Evidenz; Projektionen werden deterministisch neu aufgebaut.

### Formale Zuordnungsregeln

Sei `A(e)` die Menge intern möglicher Orders für Brokerbeobachtung `e` nach Kontext- und Evidenzprüfung. Automatische Zuordnung ist nur zulässig, wenn `|A(e)| = 1` und keine harte Inkonsistenz vorliegt. `|A(e)| = 0` -> `UNMATCHED`; `|A(e)| > 1` -> `AMBIGUOUS`. Ein Widerspruch zwischen bereits bestätigten Aliasbindungen -> `CONFLICT`.

### Persistenz-Referenzdesign

- `broker_identity_aliases(alias_id, order_id, broker, account_id, client_id, session_epoch, broker_order_id, perm_id, evidence_id, status, created_at)`
- `submission_attempts(attempt_id, order_id, idempotency_key, payload_hash, epoch, phase, created_at)`
- `broker_execution_aliases(execution_id, broker, account_id, broker_execution_key, order_id, evidence_id)`
- `identity_resolution_events(event_id, evidence_id, outcome, reason, supersedes_event_id)`

Unique-Constraints nur für nachweislich eindeutige Scopes; NULL-Werte und Wiederverwendung von brokerseitigen IDs müssen berücksichtigt werden.

### Fehler / Recovery

Unbekannte oder doppeldeutige Brokeridentität -> Quarantäne, Submission für betroffene Order blockieren, Broker-Snapshot und Audit-Trail abgleichen. Nach Reconnect dürfen Session-spezifische `orderId`-Werte nicht ohne Session-/Client-Kontext interpretiert werden. Ein `permId`-Konflikt erzwingt Eskalation.

### Invarianten

- **ID-INV-01:** Interne Order-UUID bleibt unveränderlich.
- **ID-INV-02:** Kein automatisches Zusammenführen mehrdeutiger Brokerbeobachtungen.
- **ID-INV-03:** Keine doppelte wirtschaftliche Buchung einer Broker-Execution.
- **ID-INV-04:** Kein Überschreiben bestätigter Alias-Evidenz ohne auditierte Korrektur.

### Tests

`ID-T01` Broker-ID-Wiederverwendung in anderer Session; `ID-T02` gleiche `permId` aus mehreren Beobachtungen; `ID-T03` widersprüchliche `permId`; `ID-T04` Execution-Duplikat; `ID-T05` unbekannte externe Order; `ID-T06` Replay von Alias-Korrekturen; `ID-T07` gleichzeitige Alias-Zuordnung.

## DET-04 — Transaction & Outbox Contract

### Entscheidungen

**DET-04-D01:** PostgreSQL ist Autorität für lokalen Order-/Risk-Zustand. Jeder fachliche Zustandswechsel schreibt in einer Transaktion Aggregate-Projektion, Command-/Event-Historie, erforderliche Risk-Mutationen und Outbox-Datensatz.

**DET-04-D02:** Transactional Outbox wird durch einen separat laufenden Publisher an NATS JetStream übertragen. Publisher kann nach erfolgreicher Zustellung vor Markierung abstürzen; Duplikate sind zulässig und müssen idempotent verarbeitet werden.

**DET-04-D03:** Konsumenten verwenden Inbox-Deduplizierung auf stabiler `message_id`, versionierte Schemas und transaktionale Anwendung lokaler Effekte. NATS-ACK erst nach dauerhaftem lokalem Commit. Die NATS-ACK- und DB-Commit-Grenze ist nicht atomar; Replays sind normal.

**DET-04-D04:** Retry-Policy unterscheidet transient, permanent und fachlich unklar. Begrenztes Backoff, Dead-Letter-Quarantäne, Audit und operative Eskalation. Ein DLQ-Eintrag ist kein fachlicher Orderabschluss.

**DET-04-D05:** Per-Order-Aggregate-Version und monotone Sequenz dienen als Konsistenzanker. Systemweite globale Reihenfolge über alle Streams wird nicht angenommen. Bei Lücken: Verarbeitung zurückstellen, fehlende Daten nachladen, gegebenenfalls Reconciliation.

### Zustell- und Commit-Protokoll

1. Command annehmen, Payload-Hash und Idempotenz prüfen.
2. In DB-Transaktion Version prüfen, nötige Budgetzeilen sperren, Domain-Änderungen und Outbox schreiben.
3. Commit; danach Publisher versucht JetStream-Publish mit stabiler Message-ID.
4. Nach Broker-/JetStream-ACK Outbox als publiziert markieren; bei Crash darf dieselbe Nachricht erneut gesendet werden.
5. Consumer verarbeitet Inbox+fachliche Mutation in einer Transaktion und ACKt anschliessend.

**Nicht zulässig:** DB-Commit und NATS-Publish als angeblich atomare Exactly-once-Transaktion darstellen.

### Referenzschema

`outbox_messages(message_id, aggregate_id, aggregate_version, event_type, schema_version, payload, created_at, published_at, attempt_count)`; `inbox_messages(consumer_name, message_id, payload_hash, processed_at)`; `domain_events(event_id, aggregate_id, aggregate_version, causation_id, correlation_id, payload)`.

### Invarianten

- **TX-INV-01:** Kein persistierter fachlicher Commit ohne erforderlichen Outbox-Eintrag.
- **TX-INV-02:** Gleiches Inbox-Event verursacht höchstens einen lokalen wirtschaftlichen Effekt.
- **TX-INV-03:** Fehler beim Publish rollt bereits commitete DB-Änderung nicht zurück.
- **TX-INV-04:** Verlorene ACKs führen nicht zu verlorenen Events.

### Tests

`TX-T01` Crash vor Commit; `TX-T02` Crash nach Commit vor Publish; `TX-T03` Crash nach Publish vor Outbox-Markierung; `TX-T04` Consumer-Crash nach Commit vor ACK; `TX-T05` Out-of-order-Aggregate-Version; `TX-T06` Poison Message / DLQ; `TX-T07` parallele Risk-Reservierungen; `TX-T08` Schema-Version inkompatibel.

## DET-05 — Broker Adapter Contract

### Entscheidungen

**DET-05-D01:** Hexagonaler Rust-Port `BrokerAdapter` mit brokerunabhängigen Commands, Observations und Capabilities. IBKR-spezifische IDs und Status verbleiben an der Adaptergrenze; Raw Evidence wird unverändert persistiert.

**DET-05-D02:** Connection State Machine: `DISCONNECTED -> CONNECTING -> AUTHENTICATING -> SYNCHRONIZING -> READY`, mit `DEGRADED`, `BACKOFF`, `HALTED`. `READY` erst nach Identitäts-, Order- und Execution-Synchronisation, soweit für den jeweiligen Capability-Scope erforderlich.

**DET-05-D03:** Capabilities sind versionierte, laufzeitvalidierte Verträge (Instrumenttyp, Ordertyp, TIF, Änderungs-/Cancel-Unterstützung, Datenverfügbarkeit, Rate Limits). Nicht unterstützte Kombinationen werden vor Submission abgelehnt.

**DET-05-D04:** Adapter emittiert unveränderliche Beobachtungen mit `received_at`, optional `broker_timestamp`, `session_epoch`, `raw_payload_ref`, `normalization_version`, `correlation_id`. Empfangsreihenfolge allein ist keine Evidenzpriorität.

**DET-05-D05:** Rate Limits, Heartbeats, Reconnect und Backoff sind kontrolliert. Bei Verbindungsverlust werden unbekannte Submission-Ergebnisse zur Reconciliation weitergeleitet; keine eigenständigen Broker-Retries mit unklarer Wirkung.

### Port-Skizze (nicht implementiert)

```rust
trait BrokerAdapter {
    async fn capabilities(&self) -> Result<BrokerCapabilities, AdapterError>;
    async fn snapshot(&self, account: AccountId) -> Result<BrokerSnapshot, AdapterError>;
    async fn submit_once(&self, request: BrokerSubmit) -> Result<SubmitObservation, AdapterError>;
    async fn request_cancel(&self, request: BrokerCancel) -> Result<CancelObservation, AdapterError>;
}
```

`submit_once` bedeutet **ein lokaler Dispatch-Versuch**, nicht brokerseitige Exactly-once-Garantie. Der Aufruf ist nur über DET-07 zulässig.

### Invarianten

- **BA-INV-01:** Keine Submission bei nicht validierter Capability.
- **BA-INV-02:** Raw Evidence bleibt erhalten und wird versioniert normalisiert.
- **BA-INV-03:** Kein impliziter Retry unbekannter Submission-Ergebnisse.
- **BA-INV-04:** Adapter-READY allein berechtigt nicht zum Trading.

### Tests

`BA-T01` Verbindungsausfall während Submit; `BA-T02` Reconnect mit Session-Wechsel; `BA-T03` unzulässiger Ordertyp; `BA-T04` Heartbeat-Stale; `BA-T05` doppelte Statusbeobachtungen; `BA-T06` Rate-Limit-Backoff; `BA-T07` widersprüchliche Broker-Evidenz.

## DET-06 — Trading Readiness Contract

### Entscheidungen

**DET-06-D01:** Globale Readiness ist deterministische Projektion aus Subsystem-Gates: DB/Schema, OMS, Risk, Broker, Identity, Reconciliation, Dispatcher/Fencing, Marktdaten und operativen Halts.

**DET-06-D02:** Zustände `NOT_READY`, `INITIALIZING`, `SYNCHRONIZING`, `READY`, `DEGRADED`, `HALTED`, `RECOVERING`, `MANUAL_RESET_REQUIRED`. Die Namen sind Referenzdesign; `MANUAL_RESET_REQUIRED` kann als orthogonales Flag statt Enum umgesetzt werden, sofern Semantik erhalten bleibt.

**DET-06-D03:** Readiness ist aktionsspezifisch. `CAN_SUBMIT_NEW_RISK`, `CAN_CANCEL`, `CAN_RECONCILE`, `CAN_REDUCE_RISK` haben unterschiedliche Voraussetzungen. Ein globaler Halt verbietet neue Risiken, aber darf sichere Cancel-/Reconciliation-Prozesse nicht automatisch unterbinden.

**DET-06-D04:** Zustandswechsel und Ursachen werden persistiert. Auto-Recovery nur bei explizit freigegebenen Ursachen; kritische Halts benötigen autorisierten manuellen Reset **und** erneute erfolgreiche Gate-Evaluation.

**DET-06-D05:** UI zeigt Readiness samt Ursache, betroffenen Accounts, Zeitpunkt und erlaubten Aktionen. UI-Anzeige ist nicht autoritativ; Submission Gate prüft serverseitig erneut.

### Formale Freigabe

`CAN_SUBMIT_NEW_RISK = DB_OK ∧ OMS_OK ∧ RISK_OK ∧ BROKER_READY ∧ IDENTITY_OK ∧ RECONCILED ∧ FENCE_VALID ∧ MARKET_DATA_OK ∧ ¬HALT ∧ POLICY_ALLOW`.

Fehlender oder veralteter Gate-Status wird als nicht erfüllt gewertet. Ein `READY`-Snapshot ist keine dauerhafte Berechtigung: unmittelbar vor Dispatch ist erneute Prüfung erforderlich.

### Invarianten

- **RD-INV-01:** `READY` ohne alle erforderlichen Gates ist verboten.
- **RD-INV-02:** Ein manueller Reset allein hebt keine fehlgeschlagene Sicherheitsprüfung auf.
- **RD-INV-03:** `DEGRADED` gewährt keine implizite Erlaubnis für neue Risiken.
- **RD-INV-04:** Readiness-Übergänge sind auditierbar.

### Tests

`RD-T01` DB nicht erreichbar; `RD-T02` Broker verbunden, Reconciliation offen; `RD-T03` Fencing ungültig; `RD-T04` stale market data; `RD-T05` manuell gesperrter Account; `RD-T06` Reset ohne behobene Ursache; `RD-T07` sicherer Cancel während Submission Halt; `RD-T08` Readiness-Race vor Dispatch.

## DET-07 — Submission & Execution Control Contract

**Benennung:** Eigenständiger Vertrag aus REV-20; trennt operative Submission-Autorität von OMS (DET-01) und Broker Adapter (DET-05).

### Entscheidungen

**DET-07-D01:** Pro Broker-/Account-Dispatch-Scope existiert genau ein autoritativer Writer mit persistierter Lease/Epoch. Jede neue Writer-Übernahme erhöht die Epoch monoton. Der Scope wird beim Cross-Contract-Review exakt definiert.

**DET-07-D02:** Submission Gate prüft unmittelbar vor dem externen Dispatch: Order-Version und Zustand, gültige Risk Reservation, DET-06 Action Readiness, Capability, Identity, Writer Epoch, Halt-/Reset-Flags und persistierten Submission Attempt.

**DET-07-D03:** Ein Submission Attempt besitzt `PREPARED`, `CLAIMED`, `DISPATCH_STARTED`, `ACKNOWLEDGED`, `OUTCOME_UNKNOWN`, `DEFINITIVELY_NOT_SENT` (Referenzzustände). `DISPATCH_STARTED` wird **vor** dem möglichen externen Send dauerhaft geschrieben. Ein Crash danach ist grundsätzlich ambig und darf nicht blind wiederholt werden.

**DET-07-D04:** Idempotency Keys und Broker-Aliasse schützen vor internen Doppelaufrufen, ersetzen aber keine nachgewiesene brokerseitige Idempotenz. Exakt-einmalige externe Submission wird nicht behauptet.

**DET-07-D05:** Cancel-/Modify-Commands verwenden eigene autorisierte, auditierte Versuche und müssen mit den brokerseitigen Identitäten sowie dem aktuellen wirtschaftlichen Zustand abgeglichen werden.

### Kritische Dispatch-Sequenz

1. Command, Risk Reservation und Outbox atomar persistieren.
2. Single Writer beansprucht Auftrag mit gültiger Epoch.
3. Submission Gate bewertet sämtliche Voraussetzungen neu.
4. `DISPATCH_STARTED` vor Netzwerkaufruf dauerhaft markieren; danach **genau einen** Adapteraufruf ausführen.
5. Antwort oder unbekanntes Ergebnis als Evidenz persistieren; bei Unklarheit Reconciliation statt Retry.

**Grenze des Epoch-Fencing:** PostgreSQL-Fencing verhindert konkurrierende lokale Claims und lässt stale Writer erkennen. Es kann einen bereits gestarteten Netzwerkaufruf eines alten Writers nicht physisch zurücknehmen. Daher müssen Lease-Verlust, Stop-the-world-Gates, Prozessisolation und Broker-Reconciliation gemeinsam eingesetzt werden. Ohne brokerseitig durchgesetztes Fencing gibt es keine absolute Garantie gegen zwei zeitlich überlappende externe Sends bei Netzwerkpartitionen. Für den ersten produktiven Betrieb gilt deshalb ein konservatives Active-Passive-Modell mit manuellem Eingriff bei uneindeutiger Writer-Übernahme.

### Invarianten

- **EX-INV-01:** Kein Dispatch ohne alle Gate-Voraussetzungen.
- **EX-INV-02:** Kein zweiter automatischer Submit für `DISPATCH_STARTED` mit unbekanntem Ergebnis.
- **EX-INV-03:** Kein lokaler Claim durch stale Epoch.
- **EX-INV-04:** Jeder mögliche externe Send hat vorher eine persistierte Attempt-Identität.
- **EX-INV-05:** Broker-ACK ist Evidenz, nicht automatisch vollständige wirtschaftliche Finalität.

### Tests

`EX-T01` Crash vor Dispatch-Markierung; `EX-T02` Crash nach Markierung vor Netzwerk; `EX-T03` Crash nach Netzwerk vor ACK-Persistenz; `EX-T04` Writer-Failover während Send; `EX-T05` stale Epoch; `EX-T06` Readiness fällt zwischen Claim und Gate; `EX-T07` Cancel/Fill-Race; `EX-T08` Reconnect nach unbekanntem Submit.

## DET-08 — Reconciliation & Recovery Contract

**Benennung:** Eigenständiger Vertrag aus OPEN-01/03/06 und REV-20; systemweiter Abgleich über OMS, Risk, Identity und Broker.

### Entscheidungen

**DET-08-D01:** Reconciliation ist evidenzbasierter, persistierter Prozess pro Account/Broker-Scope und Order. Quellen: lokale Orderprojektionen, Commands, Submission Attempts, Execution Ledger, Broker-Open-Orders, Execution Reports, Positionen und Risk Reservations.

**DET-08-D02:** Ein Reconciliation Run hat `run_id`, Scope, `started_at`, Snapshot-/Watermark-Referenzen, Quellenversionen, erkannte Differenzen, Aktionen, `completed_at`, Ergebnis. Ein Snapshot ist nur dann vollständig, wenn Quelle und Abdeckungszeitraum ausreichend nachgewiesen sind.

**DET-08-D03:** Differenzen werden klassifiziert: `MISSING_LOCAL`, `MISSING_BROKER`, `IDENTITY_CONFLICT`, `EXECUTION_MISMATCH`, `POSITION_MISMATCH`, `RISK_MISMATCH`, `SUBMISSION_UNKNOWN`, `STALE_SNAPSHOT`. Keine pauschale Regel „Broker gewinnt“ oder „DB gewinnt“.

**DET-08-D04:** Auflösung über deterministische, dokumentierte Evidenzregeln; gültige Executions sind wirtschaftlich zu berücksichtigen, selbst wenn Brokerstatus und lokaler Workflow widersprechen. Nicht auflösbare Fälle bleiben isoliert und können manuelle Freigabe benötigen.

**DET-08-D05:** Recovery erfolgt gestuft: globale Submission sperren, DB und Writer Authority prüfen, Broker synchronisieren, Identitäten zuordnen, Executions korrigieren, Positionen/Risk abgleichen, Differenzen behandeln, Readiness erneut bewerten. Kein automatischer Trading-Neustart bei kritischer ungeklärter Differenz.

### Reconciliation-Protokoll

1. `RECOVERING` / `SYNCHRONIZING` setzen; risikosteigernden Dispatch sperren.
2. Persistierte Command-/Event-/Outbox-/Inbox- und Attempt-Daten rekonstruieren.
3. Neue Writer-Epoch herstellen und alte Writer isolieren.
4. Vollständigkeit und Zeitfenster der Broker-Snapshots nachweisen; bei Lücken `STALE_SNAPSHOT`.
5. Identity Resolver aus DET-03 anwenden; unklare Zuordnungen quarantänisieren.
6. Execution Ledger append-only ergänzen/korrigieren; OMS-Projektionen aktualisieren.
7. Positionen, offene Verpflichtungen und Risk Reservations abgleichen.
8. Differenzen mit Evidenz und Bearbeitungsstatus persistieren; nur eindeutige Fälle automatisch schliessen.
9. DET-06-Gates neu evaluieren; kritische Fälle erfordern autorisierten manuellen Reset.

### Invarianten

- **RC-INV-01:** Kein Recovery-bedingter Blind-Retry einer möglicherweise übermittelten Order.
- **RC-INV-02:** Kein Schliessen von Differenzen ohne dokumentierte Evidenz.
- **RC-INV-03:** Keine automatische Freigabe unklarer Risikoreservierungen.
- **RC-INV-04:** Ein unvollständiger Broker-Snapshot gilt nicht als Nachweis, dass Orders fehlen.
- **RC-INV-05:** Replay und wiederholte Reconciliation erzeugen keine doppelten wirtschaftlichen Effekte.

### Tests

`RC-T01` verlorenes Broker-ACK; `RC-T02` unbekannte Brokerorder; `RC-T03` Late Fill nach Closed; `RC-T04` widersprüchliche Positionen; `RC-T05` unvollständiger Snapshot; `RC-T06` doppelte Reconciliation Runs; `RC-T07` Crash mitten im Recovery; `RC-T08` manueller Reset trotz offener Differenz; `RC-T09` Execution Bust; `RC-T10` Broker-Reconnect mit geänderter Session.

## 7. Cross-Contract-Schnittstellen und Verantwortlichkeiten

| Bereich | Autorität | Abhängige Verträge |
|---|---|---|
| Interne Order und Workflow | DET-01 | DET-03, 04, 07, 08 |
| Risk Evaluation / Reservation | DET-02 | DET-01, 04, 06, 08 |
| Broker-Identität / Execution-Zuordnung | DET-03 | DET-01, 05, 07, 08 |
| DB-Transaktionen / Outbox / Inbox | DET-04 | alle |
| Broker I/O / Raw Observations | DET-05 | DET-03, 07, 08 |
| Aktionsbezogene Trading Readiness | DET-06 | DET-02, 05, 07, 08 |
| Autoritativer Dispatch / Epoch | DET-07 | DET-01 bis 06, 08 |
| Evidenzbasierter Wiederabgleich | DET-08 | DET-01 bis 07 |

**Abhängigkeitsauflösung:** Fachliche Domänenmodule kennen keine konkreten Infrastrukturadapter. Ein Application Service orchestriert lokale DB-Transaktionen und Broker-Dispatch. Reconciliation liest die nötigen Projektionen über definierte Ports; es schreibt nicht unkontrolliert in fremde Aggregate.

## 8. ADFR — Architecture Decision & Foundations Register

Alle folgenden Einträge haben `status=PROPOSED_FOR_ACCEPTANCE`, `decision_date=2026-11-14`, `supersedes=none` und referenzieren die oben dokumentierten Vertragsabschnitte. Bestätigte OPEN-/REV-Entscheidungen bleiben als übergeordnete Constraints erhalten.

| ADFR-ID | Entscheidung | Grundlage | Konsequenz |
|---|---|---|---|
| ADFR-DET03-001 | UUID-Orderidentität und kontextgebundene Aliasse | OPEN-03 | Kein Broker-ID-Only-Mapping |
| ADFR-DET03-002 | Separate Attempt-/Execution-Identitäten | OPEN-03 | Idempotenz pro Scope |
| ADFR-DET03-003 | Evidenzbasierter Identity Resolver | OPEN-03 | Quarantäne bei Ambiguität |
| ADFR-DET03-004 | Append-only Identity Corrections | DET-01 | Reproduzierbares Audit |
| ADFR-DET04-001 | Lokale PostgreSQL-Transaktionsautorität | OPEN-04 | Konsistenter OMS/Risk-Commit |
| ADFR-DET04-002 | Transactional Outbox + JetStream | OPEN-04 | At-least-once statt Exactly-once-Annahme |
| ADFR-DET04-003 | Inbox-Idempotenz und ACK nach Commit | OPEN-04 | Replay-sichere Konsumenten |
| ADFR-DET04-004 | DLQ/Retry/Schema-Versionierung | OPEN-04 | Operative Fehlerisolierung |
| ADFR-DET05-001 | Hexagonaler Broker-Port | OPEN-05 | Austauschbarer IBKR-Adapter |
| ADFR-DET05-002 | Connection State Machine und Capabilities | OPEN-05 | Kein Dispatch ohne validierte Fähigkeiten |
| ADFR-DET05-003 | Raw Evidence und Normalisierungsversion | OPEN-01/05 | Brokerbeobachtungen nachvollziehbar |
| ADFR-DET06-001 | Mehrdimensionale Readiness-Gates | OPEN-06 | Deterministische Freigaben |
| ADFR-DET06-002 | Aktionsspezifische Berechtigungen | OPEN-06 | Sichere Cancels trotz Submission Halt möglich |
| ADFR-DET06-003 | Persistierte Halts und manueller Reset | OPEN-06 | Keine stillen Wiederanläufe |
| ADFR-DET07-001 | Single-Writer mit Epoch-Fencing | REV-20 | Nur autoritativer Dispatcher |
| ADFR-DET07-002 | Persistierter Submission Attempt vor Send | REV-20 | Ambige Ergebnisse sicher behandeln |
| ADFR-DET07-003 | Revalidierendes Submission Gate | REV-20 | Keine veraltete Freigabe |
| ADFR-DET07-004 | Kein blindes Retry; begrenzte Fencing-Garantie | OPEN-01/REV-20 | Konservatives Failover |
| ADFR-DET08-001 | Systemweite evidenzbasierte Reconciliation | OPEN-01/03/06 | Konsistente Wiederherstellung |
| ADFR-DET08-002 | Snapshot-Vollständigkeit und Watermarks | OPEN-03/06 | Keine falschen Negativnachweise |
| ADFR-DET08-003 | Differenzklassifikation und Quarantäne | OPEN-01/03 | Explizite Konfliktbehandlung |
| ADFR-DET08-004 | Gestufte Recovery und Readiness-Gate | OPEN-06/REV-20 | Kein unsicherer Neustart |

### Dokumentationsstandard pro ADFR-Eintrag

`id`, `title`, `status`, `decision`, `context`, `alternatives`, `rationale`, `consequences`, `invariants`, `tests`, `dependencies`, `decision_date`, `supersedes`, `approver`. Die Tabelle ist das kompakte Register; die jeweiligen Vertragsteile liefern Kontext, Begründung und Testreferenzen.

## 9. Verifikationsplan und verbindliche CI-Gates

- **Unit:** Transition- und Policy-Evaluation, Identity Resolver, Normalisierung, Gate-Logik.
- **PostgreSQL Integration:** CAS, Unique Constraints, konkurrierende Reservierungen, atomare OMS/Risk/Outbox-Transaktionen, Inbox-Deduplikation.
- **Model-Based / Property-Based:** generierte Order-/Broker-/Execution-Sequenzen; Invarianten und Replay.
- **Fault Injection:** Crash an jedem Commit-/Publish-/ACK-/Dispatch-Grenzpunkt, Partitionen, doppelte Events, Writer-Failover.
- **Simulierter Broker:** Reconnect, Teilfills, Late Fills, Busts, Order-ID-Wiederverwendung, unvollständige Snapshots.
- **Safety Gates:** Jede Verletzung von `SYS-INV-*`, `ID-INV-*`, `TX-INV-*`, `BA-INV-*`, `RD-INV-*`, `EX-INV-*`, `RC-INV-*` blockiert Merge/Release. KI darf diese Gates nicht deaktivieren.

## 10. Offene Punkte vor Implementierungsfreigabe

1. **IBKR-Feldsemantik verifizieren:** Lebensdauer, Eindeutigkeit und Scope von `orderId`, `permId`, `execId`, Client-/Session-Kontext sowie verfügbare Reconciliation-Endpunkte anhand aktueller IBKR-Dokumentation und Sandbox-Tests.
2. **Fencing-Grenze:** Exakte Dispatch-Scope-Definition, Lease-TTL, Heartbeats, Writer-Prozessisolation und Verhalten bei Netzwerkpartitionen.
3. **Schema-Freeze:** PostgreSQL-Constraints, Indizes, Transaktionsisolation und Migrationen für alle acht Verträge gemeinsam abnehmen.
4. **Event-Schema:** Typisierte Payloads, Schema-Evolution, NATS-Subjects und Retention festlegen.
5. **Reconciliation-Evidenz:** Snapshot-Vollständigkeit, Watermarks und zulässige automatische Konfliktauflösung pro Broker definieren.
6. **Risk-Action-Matrix:** Wann Cancels/Reduktionen trotz Halt erlaubt sind; wer einen manuellen Reset genehmigen darf.
7. **Vertragstests:** Aus den Referenztests ausführbare Akzeptanzkriterien und unabhängige Referenzmodelle erzeugen.

**Nächster Architektur-Gate:** Cross-Contract Review DET-01 bis DET-08, danach DET-09 und DET-10. Kein Live-Trading und keine Implementierungsfreigabe durch dieses Dokument.
