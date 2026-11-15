# Trading Platform — P0 Resolution Contracts CR-B01 bis CR-B04

**Version:** 1.0-draft  
**Datum:** 2026-11-14  
**Status:** Architekturentscheidungen eigenständig festgelegt; Implementierung und empirische Verifikation ausstehend. **Keine Freigabe für Live-Submission.**  
**Basis:** `DET-03-bis-DET-08-Architecture-Contracts-v1.0.md`, `DET-09-bis-DET-10-und-Cross-Contract-Review-v1.0.md`, fachliche DET-01/02-Entscheidungen.  
**Normsprache:** MUSS verpflichtend, DARF NICHT verboten, SOLL Standard mit dokumentierter Ausnahme.

## 0. Ergebnis und Schliesskriterien

| P0 | Architekturbeschluss | Dokumentarischer Status | Nachweis vor produktiver Freigabe |
|---|---|---|---|
| CR-B01 | Exklusive Broker-Session + persistierte Attempts + konservatives Active-Passive-Failover | **DESIGN_RESOLVED** | Partition-/Crash-/Failover-Fault-Injection, Broker-Session-Exklusivität im Zielbetrieb nachgewiesen |
| CR-B02 | Gemeinsame transaktionale Unit of Work mit fester Sperrordnung | **DESIGN_RESOLVED** | PostgreSQL-Concurrency-/Crash-Tests und atomare Audit-/Outbox-Prüfung |
| CR-B03 | Append-only Late-Fill-Kompensation, Neuberechnung und Risk Incident | **DESIGN_RESOLVED** | Replay-/Property-/Ledger- und Recovery-Tests inklusive Fill/Cancel/Bust-Races |
| CR-B04 | Broker-Identity-Scopes als **Hypothesen**, Resolver mit Quarantäne und verifizierungsabhängigen Constraints | **DESIGN_RESOLVED, BROKER_SEMANTICS_UNVERIFIED** | IBKR-Dokumentationsprüfung und Gateway-/Paper-Adaptertests für Identität und API-Abdeckung |

**Freigabeformel:** `LIVE_SUBMISSION_READY = architecture_approved AND B01_verified AND B02_verified AND B03_verified AND B04_verified AND cross_contract_gates_pass AND operational_approval`. Die vier P0 sind auf Designebene behandelt, aber **nicht als technisch/empirisch geschlossen** zu deklarieren.

---

# 1. CR-B01 — Writer-Fencing, Broker-Session und Failover

## 1.1 Entscheidung `P0-B01-D01` — konservatives Active-Passive

Für den ersten produktiven Betrieb ist **genau ein aktiver Execution-Writer je Broker-Account-/Connection-Scope** erlaubt. Die tatsächliche Scope-Grenze muss anhand der IBKR-Gateway-/TWS-Topologie und Account-Rechte getestet werden. Es gibt **kein automatisches Takeover**, solange ein alter Writer noch eine möglicherweise sendefähige Verbindung besitzen könnte oder Submission Attempts ungeklärt sind. Im Zweifel bleibt das System `HALTED`/`RECOVERING`.

**Kernprinzip:** Ein PostgreSQL-Lease ist **kein externes Broker-Fencing**. Epoch-Checks verhindern lokale veraltete Claims, können aber einen bereits laufenden Socket-Write nicht zurückholen. Deshalb ist die Sicherheitsbedingung eine **exklusiv kontrollierte Sendefähigkeit**, nicht bloss `lease_owner == self`.

## 1.2 Authority-/Session-Protokoll `P0-B01-D02`

1. Ein Execution-Controller verwaltet `scope_id`, monotone `epoch`, `writer_id`, `lease_deadline`, `authority_state` (`INACTIVE`, `ACTIVE`, `REVOKING`, `FENCED`, `UNCERTAIN`) und `session_generation`.
2. Ein Writer darf nur über einen **einzigen kontrollierten Dispatch-Port** senden; direkte Broker-Sockets aus OMS, Risk oder Agenten sind verboten.
3. Vor jeder Submission wird eine **dauerhafte** Attempt-Zeile mit `order_id`, `attempt_id`, `command_id`, `epoch`, `payload_hash`, `broker_account`, `client/session identity`, `phase` und `evidence_ref` erzeugt.
4. Nach erneutem Gate-Check wird `DISPATCH_STARTED` **vor** jedem potenziellen Socket-Send committed. Ab diesem Commit ist die Operation bis zum eindeutigen Gegenbeweis **möglicherweise gesendet**.
5. Vor dem Adapteraufruf wird Authority nochmals geprüft; ein atomarer lokaler Check und ein externer Netzwerkaufruf sind **nicht** gemeinsam atomar. Der Writer MUSS die eigene Sendefähigkeit bei Lease-Verlust sofort deaktivieren; die Verbindung MUSS kontrolliert getrennt werden.
6. Ein Nachfolger darf erst aktiviert werden, nachdem der alte Sender **nachweislich isoliert** ist (Prozess/Container gestoppt **und** Brokerverbindung geschlossen bzw. Session-Exklusivität belastbar bestätigt) und sämtliche `DISPATCH_STARTED`-/`OUTCOME_UNKNOWN`-Attempts in Reconciliation übernommen wurden.
7. Eine fehlende Brokerorder in einer unvollständigen Abfrage ist **kein** Nachweis für `DEFINITIVELY_NOT_SENT`. Eine mögliche Submission wird nicht erneut gesendet, solange das Ergebnis unklar ist.
8. Bei Partition, Lease-Ablauf, Restart oder unbekannter Session-Lage: `UNCERTAIN`, globaler bzw. scopebezogener Submission Halt, Reconciliation, manuelle Autorisierung der Übernahme. Kein zeitgesteuertes blindes Auto-Failover.

**Operational constraint:** Für V1 ist die Brokerverbindung des aktiven Writers auf einen isolierten Gateway-/TWS-Deployment-Scope zu beschränken; zusätzliche manuelle TWS-Sessions oder externe Orderquellen sind entweder explizit als separate Actoren zu erfassen oder für die kontrollierte Betriebsart auszuschliessen. Ein Netzwerk-Kill allein beweist nicht, dass ein bereits übermittelter Auftrag nicht angenommen wurde.

## 1.3 Attempt-State-Machine

`PREPARED -> CLAIMED -> DISPATCH_STARTED -> ACKNOWLEDGED | OUTCOME_UNKNOWN`. `PREPARED`/`CLAIMED` können **nur bei belegtem Nichtversand** nach `DEFINITIVELY_NOT_SENT` wechseln. `DISPATCH_STARTED` ist konservativ ambig; `DEFINITIVELY_NOT_SENT` nach dieser Phase nur bei **positiver, hinreichender Evidenz**, nicht durch Timeout, leere Liste oder Neustart. `ACKNOWLEDGED` bedeutet Annahme-/Zuordnungsevidenz, **keine wirtschaftliche Finalität**. Fehler, Abbrüche und Broker-Rejects werden mit Rohbelegen gespeichert; eine erneute Submission erfordert einen eigenen explizit autorisierten Command und bewiesene Nichtannahme des vorherigen Attempts.

## 1.4 Race- und Crash-Matrix

| Zeitpunkt | Recovery-Regel |
|---|---|
| Vor Attempt-Commit | Kein Attempt autorisiert; normaler Command-Replay nach aktuellen Gates möglich |
| Nach `PREPARED`, vor `DISPATCH_STARTED` | Nur bei eindeutigem Nichtversand neu claimbar |
| Nach `DISPATCH_STARTED`, vor Socket-Write | **Trotzdem** als möglicherweise gesendet behandeln; keine blinde Wiederholung |
| Während Socket-Write | Ausgang unbekannt, Session isolieren und Broker-Reconciliation |
| Nach Brokerannahme, vor Ack-Persistenz | Über Broker-Identity/Execution-Evidenz zuordnen; kein Retry |
| Alter Writer lebt bei Epoch-Wechsel | Nachfolger sendet **nicht**; Failover blockiert |

## 1.5 Invarianten und Tests

- **P0-B01-INV-01:** Kein automatischer Zweitversand bei unbekanntem Attempt-Ausgang.
- **P0-B01-INV-02:** Keine aktivierte Nachfolge-Session bei unbewiesener Isolation des Vorgängers.
- **P0-B01-INV-03:** Jeder mögliche Broker-Send hat einen zuvor committed `DISPATCH_STARTED`-Attempt.
- **P0-B01-INV-04:** Ein globaler Halt blockiert neue risikosteigernde Dispatches auch nach einem Lease-Wechsel.
- **Tests:** `P0-B01-T01` Prozesscrash in jeder Phase; `T02` Netzwerkpartition; `T03` Lease-Ablauf während Socket-Write; `T04` alter Writer mit gültigem Socket nach Epoch-Wechsel; `T05` Gateway-Restart; `T06` unvollständige Broker-Snapshots; `T07` Halt zwischen Gate und Send; `T08` zwei konkurrierende Takeover-Requests. Alle Tests müssen Brokeraufrufe instrumentieren und unkontrollierte Doppelübermittlung ausschliessen bzw. konservatives Stoppen nachweisen.

---

# 2. CR-B02 — Gemeinsame PostgreSQL Unit of Work

## 2.1 Entscheidung `P0-B02-D01` — eine lokale Commit-Autorität

OMS, Risk, Domain Events, autoritative Audit-Evidenz und Outbox werden über **eine** PostgreSQL-Unit-of-Work (`TradingUnitOfWork`) mit **einem** Datenbank-Commit geschrieben. Fachliche Module liefern deterministische Mutationspläne; eine Infrastrukturkomponente setzt sie in derselben Transaktion um. **Keine Broker-, NATS- oder externen Netzwerkanfragen innerhalb der UoW.** Audit-Export und JetStream-Publish erfolgen erst nach Commit.

## 2.2 Sperrordnung `P0-B02-D02`

Vor Beginn der Mutationen wird der vollständige betroffene Ressourcen-Satz soweit möglich bestimmt. Für V1 gilt eine kanonische Ordnung:

1. `trading_control_scope` / Halt- und Authority-Version (falls schreibend betroffen)
2. `risk_budget`-Zeilen, sortiert nach `(scope_type, scope_id, metric_id, currency)`
3. `order_aggregate` nach UUID
4. `risk_reservation` nach UUID
5. `execution_ledger`-/Identity-Projektionen nach stabilen Schlüsseln
6. append-only `domain_events`, `audit_events`, `outbox_messages`

**Wichtig:** Diese Ordnung ist ein Vertragsvorschlag für den Datenbankzugriff, kein Ersatz für tatsächliche Deadlock-Tests. Wenn mehrere Orders/Budgets betroffen sind, werden sämtliche Keys innerhalb jeder Kategorie lexikografisch sortiert. Ein Codepfad darf die Ordnung nicht invertieren. Externe Readiness-/Market-Inputs werden vor der Transaktion als versionierte Snapshots gelesen und **vor Commit** gegen die erforderlichen Versionen/Freshness-Gates validiert; ein globaler Halt benötigt ein synchronisiertes DB-Gate bzw. monotone Halt-Epoch, damit ein älterer Snapshot nicht stillschweigend autorisiert.

## 2.3 Isolation und Retry `P0-B02-D03`

**Default:** PostgreSQL `READ COMMITTED` mit explizitem `SELECT ... FOR UPDATE` für alle betroffenen Budgetzeilen, bedingten Updates/Constraints, Order-Version-CAS und konsistenter Lock-Ordnung. `SERIALIZABLE` ist für ausgewählte Transaktionen zulässig, aber nicht stillschweigend vorausgesetzt. Transaktionen, die sich nicht durch die festgelegten Zeilen-/Predicate-Gates absichern lassen, müssen mit zusätzlichem Constraint oder `SERIALIZABLE` modelliert werden. `40001` (serialization_failure), `40P01` (deadlock_detected) und CAS-Konflikte werden nur als **gesamte lokale UoW** neu bewertet; keine Wiederverwendung einer veralteten Risk-Evaluation. Backoff begrenzt, danach fail-closed/operativer Fehler. **Niemals** einen externen Broker-Send im Rahmen eines DB-Retry wiederholen.

## 2.4 UoW-Commit-Vertrag

**Input:** `command_id`, `idempotency_key`, `payload_hash`, `expected_order_version`, `policy_version`, `market_snapshot_id`, `actor`, `correlation_id`.

**Atomarer Output:** `order_aggregate(version+1)`, `risk_budget_usage`, `risk_reservations`, `command_result`, `domain_events`, `audit_events`, `outbox_messages`, optional `execution_ledger`-/Identity-Mutationen.

**Pseudoprotokoll:**

```text
begin transaction
  lock all required budget rows in canonical order
  lock/CAS order aggregate and required reservation rows
  verify idempotency identity and payload hash
  verify latest persisted halt/authority and input-version gates
  evaluate deterministic policy against locked/current budgets
  validate all invariants and calculate mutation plan
  write order + risk + ledger + command result
  append domain event + authoritative audit + outbox
commit
```

`risk_budget_usage` darf nicht lediglich ein ungeprüfter Cache sein: jede harte Budgetdimension braucht eine transaktional geschützte Zähl-/Constraint-Strategie, die überlappende Reservierungen berücksichtigt. Im Fehlerfall vollständiger Rollback. Für nicht mutierende `DENY`/`INDETERMINATE`-Entscheidungen muss ein auditierter Command-Ausgang persistiert werden, soweit keine DB-Störung vorliegt; bei DB-Störung darf kein `ALLOW` entstehen.

## 2.5 Invarianten und Tests

- **P0-B02-INV-01:** Kein fachlicher Commit ohne erforderliche Risk-/Audit-/Outbox-Begleitdaten.
- **P0-B02-INV-02:** Keine Überreservierung bei konkurrierenden Commands.
- **P0-B02-INV-03:** Gleicher Command/Idempotency-Key und Payload erzeugt höchstens einen Effekt; abweichender Hash erzeugt Konflikt.
- **P0-B02-INV-04:** Ein UoW-Retry löst keinen externen Brokeraufruf aus.
- **Tests:** `P0-B02-T01` parallele Orders für identisches Budget; `T02` überlappende globale/Konto/Sektor-Budgets; `T03` CAS-Konflikt; `T04` Deadlock/Serialization Retry; `T05` Crash vor/nach Commit; `T06` Audit-/Outbox-Insert-Fehler; `T07` Inbox-Duplikat; `T08` globaler Halt während Reservierung; `T09` Version/Freshness-Wechsel vor Commit.

---

# 3. CR-B03 — Late Fill, Bust und Risikokompensation

## 3.1 Entscheidung `P0-B03-D01` — wirtschaftliche Wahrheit ist append-only

`workflow_state=CLOSED` und `closure_reason` bleiben historisch unveränderlich. Späte Broker-Fills, Korrekturen und Busts werden nach evidenzbasierter Identity Resolution **immer** im Execution Ledger als neue Belege verarbeitet. Die wirtschaftliche Projektion wird deterministisch aus Ledger + Korrekturkette berechnet. Es erfolgt **kein** stilles Wiederöffnen des Workflows und **kein** Überschreiben des ursprünglichen Abschlusses.

## 3.2 Risk Rebooking `P0-B03-D02`

Jedes neu bestätigte wirtschaftliche Ereignis erzeugt eine idempotente Rebooking-Operation mit `economic_event_id`, `order_id`, `position_scope`, `old_projection_version`, `new_projection_version`, `risk_policy_version`, `valuation_snapshot_ref`, `correlation_id`.

- Positive Execution: ausgeführte Menge von offener Orderverpflichtung auf Positionsexposition umbuchen; verbleibende Orderverpflichtung nur nach ausreichender Evidenz reduzieren.
- Fill nach `RELEASED`: keine alte Reservierung löschen oder heimlich neu öffnen; stattdessen **neuen kompensierenden Risk-Ledger-Eintrag** für die tatsächliche Position und eine auditierte Abweichung erzeugen. Falls Budgetgrenze verletzt oder Markt-/Brokerdaten unklar: `RISK_RECONCILIATION_REQUIRED`, `BLOCK_NEW_RISK` für betroffenen Scope und Incident.
- Execution Bust/Korrektur: Original-Evidenz referenzieren, Differenz append-only buchen und Position/Risk-Projektionen neu berechnen. Ein Bust kann Exposition reduzieren **oder** durch neue offene Verpflichtungen Risiko erhöhen; keine pauschale Freigabe.
- Broker-Positionen, lokale Positionen und Reservations müssen getrennt abgeglichen werden. Ein `CLOSED`-Workflow kann wirtschaftlich `RECONCILIATION_REQUIRED` sein.

## 3.3 Formale Bilanzbedingung

Sei `F(o)` die aus gültigen Ausführungs- und Korrekturbelegen abgeleitete Nettomenge, `P(o)` die daraus gebuchte Positionsänderung, `R_open(o)` die durch bestätigte noch offene Orderverpflichtungen erforderliche Risikoreservierung. Dann gilt unter einem konsistenten Snapshot:

`P(o) = F(o)` (in instrument- und side-korrekter Mengeneinheit) und `RiskUsage = Risk(positions, open_orders, policy, market_snapshot)`.

Bei unbekannter offener Menge darf `R_open(o)` nicht allein aus einer vermuteten Cancel-Finalität auf null gesetzt werden. Falls Evidenz die Menge über die ursprünglich bestellte Menge hinaus zeigt, wird sie nicht abgeschnitten, sondern als Overfill-Incident erfasst.

## 3.4 Atomare Anwendung

`execution_evidence` + `execution_ledger` + `position_projection` + `risk_rebooking` + `audit_event` + `outbox` werden in einer gemeinsamen UoW angewendet, sofern die Daten innerhalb derselben lokalen DB-Domain liegen. Ist die benötigte Bewertung nicht verfügbar, wird die Ausführung **trotzdem sicher erfasst** und die Projektion als unvollständig markiert; neue Risikofreigaben sind bis zur Nachbewertung blockiert. Ein gescheiterter Risk-Check darf eine bereits real stattgefundene Ausführung nicht aus dem Ledger entfernen.

## 3.5 Invarianten und Tests

- **P0-B03-INV-01:** Kein gültiger Late Fill wird wegen `CLOSED` verworfen.
- **P0-B03-INV-02:** Keine doppelte Positions-/Risk-Buchung derselben wirtschaftlichen Execution.
- **P0-B03-INV-03:** Keine Änderung des ursprünglichen `closure_reason`.
- **P0-B03-INV-04:** Wirtschaftliche Inkonsistenz blockiert neue Risikoerhöhungen; historische Evidenz bleibt erhalten.
- **Tests:** `P0-B03-T01` Fill nach Cancel; `T02` Fill nach `RELEASED`; `T03` Fill-Duplikat; `T04` Bust nach Late Fill; `T05` Out-of-order Correction; `T06` Fill und Cancel gleichzeitig; `T07` Marktdaten fehlen bei Fill; `T08` Overfill; `T09` Crash zwischen Ledger und Risk; `T10` Replay derselben Evidenzfolge.

---

# 4. CR-B04 — IBKR Identity Contract und Verifikationsmatrix

## 4.1 Entscheidung `P0-B04-D01` — Identitätsmodell ohne unbelegte Eindeutigkeitsannahmen

**Interne Autorität:** `order_id UUID`, `submission_attempt_id UUID`, `execution_id UUID`, `evidence_id UUID` und `command_id UUID` werden intern erzeugt. IBKR-Felder (`orderId`, `permId`, `execId`, `clientId`, `account`, `conId`, `orderRef`) sind **brokerseitige Identifikatoren/Attribute**, deren tatsächliche Eindeutigkeit, Stabilität, Null-/Unset-Semantik und Gültigkeitsbereich je TWS-/Gateway-API-Version verifiziert werden müssen.

**Keine produktive UNIQUE-Constraint allein auf `orderId`, `permId` oder `execId`, solange die jeweilige Scope-Hypothese nicht belegt ist.** Stattdessen unveränderliche Rohbeobachtungen, versionierte Aliasse und kontrollierte Resolver-Projektionen. Interne UUID- und Idempotency-Constraints sind davon unabhängig verpflichtend.

## 4.2 Hypothesen- und Evidenzmatrix (nicht als Broker-Garantien zu lesen)

| Feld | Vorläufige Resolver-Nutzung | Vor Produktivbetrieb zu verifizieren |
|---|---|---|
| `orderId` | Kandidatenalias im Broker-/Account-/Client-/Session-Kontext | Vergabe, Wiederverwendung, Sichtbarkeit über Clients/Sessions, Reconnect |
| `permId` | Starker langfristiger Korrelationshinweis, **nicht alleinige interne ID** | Stabilität bei Modify/Cancel/Reconnect, Scopes, Null-/Sonderwerte, Kontoübergreifende Fälle |
| `execId` | Kandidat für Execution-Dedup mit Broker-/Account-/Kontext und Korrekturbezug | Eindeutigkeit, Format/Veränderung bei Korrekturen/Busts, Replay-Verhalten |
| `clientId` | API-Client-/Session-Kontext | Auswirkungen auf Order-Sichtbarkeit, Master-/Client-Varianten |
| `account` | Scope für Positionen, Orders und Executions | Multi-Account/Advisor-Zuordnung, Account-Feld in allen Eventtypen |
| `conId` | Instrument-Korrelation, nicht Orderidentität | Instrument-/Contract-Mapping, Corporate Actions, Handelsplätze |
| `orderRef` | Optionaler Korrelationshinweis, wenn tatsächlich übertragen/zurückgeliefert | Länge, Persistenz, Roundtrip und Manipulierbarkeit |

## 4.3 Resolver-Algorithmus `P0-B04-D02`

1. Jede Brokerbeobachtung wird als unveränderliche `raw_broker_evidence` mit Payload-Hash, Provider-/Adapter-Version, Empfangszeit und Session-Kontext gespeichert.
2. Kandidaten werden anhand **aller verfügbaren** Aliasbindungen und gesicherter Account-/Client-/Session-Kontexte gebildet. Es werden keine Feldwerte aus anderen Sessions ohne expliziten Bindungsnachweis gleichgesetzt.
3. `MATCHED` nur bei **genau einer** widerspruchsfreien Order mit ausreichend starker Evidenz; `UNMATCHED` bei keiner; `AMBIGUOUS` bei mehreren; `CONFLICT` bei widersprechender bestätigter Bindung.
4. Nur `MATCHED` darf eine automatische wirtschaftliche Orderbuchung auslösen. Alle anderen Ergebnisse gehen in Quarantäne und Reconciliation; unbekannte externe Orders dürfen nicht stillschweigend einer bestehenden Order zugeschlagen werden.
5. Broker-Execution-Duplikate werden über verifizierte Schlüssel und Payload-/Korrekturketten erkannt. Gleicher vermeintlicher Schlüssel mit anderem Payload erzeugt `CONFLICT`, nicht stilles Überschreiben.
6. Bestätigte Aliasbindungen werden nur über append-only `identity_resolution_events` korrigiert. Jede Korrektur muss bestehende wirtschaftliche Buchungen auf Konflikte prüfen.

## 4.4 Vorläufige DB-Constraints `P0-B04-D03`

**Jetzt zulässig:** Primary/Unique Keys auf internen UUIDs; `UNIQUE(command_scope, idempotency_key)` mit Payload-Hash-Prüfung; `UNIQUE(consumer_name, message_id)` für Inbox; referenzielle Integrität zwischen Attempt, Order und Evidence.

**Erst nach Broker-Nachweis:** Eindeutigkeitsconstraints für Broker-Aliasse im bestätigten tatsächlichen Scope, mit expliziter Behandlung fehlender IDs, Sessionwechsel und Korrekturen. Bis dahin sind **nicht eindeutige Aliaswerte zulässig**, aber dürfen keinen automatischen `MATCHED`-Status erzwingen.

## 4.5 Broker-Adapter-Verifikation als zwingendes Gate

Für die konkret eingesetzte IBKR-TWS-/Gateway-Version, API-Bibliothek und Konfiguration MUSS ein Testbericht mit Rohbelegen erstellt werden:

| Test | Beobachtung / Acceptance |
|---|---|
| `P0-B04-T01` | Neue Order: `orderId`, `permId`, `clientId`, `account`, `orderRef` und Events dokumentieren |
| `T02` | Reconnect: welche IDs bleiben stabil, welche ändern sich? |
| `T03` | Gateway-Restart und Client-ID-Wechsel: Sichtbarkeit und Aliasbindung |
| `T04` | Modify/Cancel: Identitätsstabilität und Callback-Reihenfolge |
| `T05` | Partial Fill und mehrere Executions: `execId`-Zuordnung und Duplikate |
| `T06` | Execution Correction/Bust: alte/neue `execId` und Korrekturkette |
| `T07` | Zwei Clients / andere Session: Order-Sichtbarkeit und Kollisionsverhalten |
| `T08` | Unbekannte externe Order: Quarantäne statt falscher Match |
| `T09` | Reconciliation-APIs: Vollständigkeit/Zeiträume/Filter dokumentieren |
| `T10` | NULL/0/fehlende Felder und Out-of-order-Callbacks: kein False Match |

**Testumgebung:** ausschließlich kontrollierte IBKR-Paper-/Sandbox-Umgebung, keine produktiven Order-Sends. Rohbelege mit Zeitstempel, Adapter-/Gateway-Version und Konfiguration archivieren. Bei fehlendem Broker-Nachweis bleibt Live-Submission gesperrt.

## 4.6 Invarianten

- **P0-B04-INV-01:** Keine automatische wirtschaftliche Buchung einer `AMBIGUOUS`/`CONFLICT`-Beobachtung.
- **P0-B04-INV-02:** Keine globale Eindeutigkeitsannahme für unbestätigte Broker-IDs.
- **P0-B04-INV-03:** Jede automatische Brokerzuordnung ist auf Rohbelege und Resolver-Version rückführbar.
- **P0-B04-INV-04:** Identitätskorrekturen sind append-only und lösen erforderliche Reconciliation aus.

---

# 5. Cross-Contract-Integration

| Integrationspunkt | Verantwortlicher Vertrag | Ergänzung durch P0-Resolution |
|---|---|---|
| Order/Submission Attempt | DET-01/07 | B01: unveränderlicher Attempt, unbekanntes Ergebnis blockiert Retry |
| Risk/Order/Outbox/Audit | DET-02/04/10 | B02: gemeinsame UoW, Sperrordnung, Commit-Semantik |
| Execution/Position/Risk | DET-01/02/08 | B03: kompensierende Buchungen und Incident nach Late Fill |
| Broker Alias/Execution Identity | DET-03/05/08 | B04: verifizierungsabhängige Scopes und Quarantäne |
| Readiness/Operational Halt | DET-06/10 | B01/B03/B04: keine Freigabe bei Writer-, Risk- oder Identity-Unklarheit |
| Market Snapshot/Valuation | DET-09 | B02/B03: versionierte Risk-Eingaben, Nachbewertung bei fehlenden Daten |

## 5.1 Systemweite Acceptance Gates

- `GATE-P0-01`: Eindeutige Sendefähigkeit je Dispatch-Scope; konservatives Failover mit Fault Injection nachgewiesen.
- `GATE-P0-02`: Order/Risk/Audit/Outbox-Atomizität und Budget-Serialisierung in PostgreSQL nachgewiesen.
- `GATE-P0-03`: Late-Fill-/Bust-/Cancel-Races inklusive Risk-Rebooking und deterministischem Replay bestanden.
- `GATE-P0-04`: IBKR-Identitäts-/Reconciliation-Matrix für Zielversion mit dokumentierten Rohbelegen verifiziert.
- `GATE-P0-05`: End-to-End `halt -> recover -> reconcile -> authorized reset -> ready` erfolgreich; bei unvollständiger Evidenz `NOT_READY`.

## 5.2 Restliche Risiken

1. Ohne Broker-seitiges Fencing existiert keine mathematisch absolute Exactly-Once-Garantie für externe Sends. V1 verwendet konservatives Stoppen, exklusive Sessionkontrolle, Identitätsabgleich und **keine** blinden Retries.
2. Die tatsächliche Vollständigkeit von IBKR-Reconciliation-Abfragen und die Semantik der IDs sind noch nicht empirisch verifiziert.
3. Der Lock-Order- und Isolation-Vertrag muss durch PostgreSQL-Tests und Schema-Review bestätigt werden; komplexe bereichsübergreifende Prädikate können `SERIALIZABLE` oder zusätzliche Guard-Zeilen erfordern.
4. Risk-Bewertung nach Late Fill kann bei fehlenden Preisen/FX unbestimmt bleiben; wirtschaftliche Evidenz wird trotzdem erfasst, neue Risiken werden blockiert.

---

# 6. ADFR — P0-Auflösungsentscheidungen

| ADFR-ID | Entscheidung | Verworfen | Konsequenz / Nachweis |
|---|---|---|---|
| `ADFR-P0-001` | Konservatives Active-Passive mit exklusiver Broker-Sendefähigkeit | Automatisches Failover allein nach Lease-TTL | Isolation und Broker-Reconciliation vor Takeover |
| `ADFR-P0-002` | `DISPATCH_STARTED` vor Socket-Send, Unknown = kein Retry | Timeout als Nichtversandbeweis | Persistierte Attempt-Evidenz und Recovery |
| `ADFR-P0-003` | Gemeinsame PostgreSQL-UoW mit kanonischer Sperrordnung | Separate OMS-/Risk-/Audit-Commits | Atomizität, Concurrency-Tests |
| `ADFR-P0-004` | READ COMMITTED mit expliziten Guards, CAS und Locks | Ungeschützte Read-then-Write-Budgets | Predicate-Review, Isolationstests |
| `ADFR-P0-005` | Append-only Risk-Rebooking bei Late Fill/Bust | Änderung/Löschung alter Reservation ohne Audit | Kompensation, Risk Incident, Replay |
| `ADFR-P0-006` | IBKR-Alias-Scopes erst nach Verifikation einfrieren | Unbelegte globale UNIQUE-Constraints | Adapter-Testmatrix als Live-Gate |
| `ADFR-P0-007` | Quarantäne für unbekannte/mehrdeutige Brokerbeobachtungen | Best-effort-Zuordnung nach Empfangsreihenfolge | Resolver-Audit und Reconciliation |
| `ADFR-P0-008` | Produktive Freigabe nur nach Nachweis aller P0-Gates | Dokumentarische Entscheidung als ausreichender Test | `NOT_READY_FOR_LIVE_SUBMISSION` bis zur Verifikation |

## 6.1 Änderungsanweisungen an die bestehenden Verträge

- **DET-03:** Broker-Alias-Constraints bleiben bis `GATE-P0-04` offen; Resolver-Quarantäne ist bindend.
- **DET-04:** `TradingUnitOfWork` und gemeinsame transaktionale Auditquelle aus DET-10 übernehmen.
- **DET-07:** Writer-Takeover erst nach Isolation des Vorgängers; `DISPATCH_STARTED` ist ambig und nie automatisch retrybar.
- **DET-08:** Recovery darf Risk-Rebooking und Identity-Quarantäne nicht überspringen.
- **DET-02:** Freigegebene Reservierungen bleiben historisch erhalten; Late Fills führen zu neuen kompensierenden Buchungen.
- **DET-06/10:** Readiness/Reset prüft Writer-Isolation, Risk-Konsistenz, Broker-Evidenz und Audit-Vollständigkeit.

**Gesamtstatus:** `P0_DESIGN_RESOLVED / IMPLEMENTATION_NOT_VERIFIED / LIVE_TRADING_NOT_AUTHORIZED`.
