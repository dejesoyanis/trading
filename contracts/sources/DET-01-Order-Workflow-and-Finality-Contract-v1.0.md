# DET-01 — Order Workflow & Finalität: Architekturvertrag

**Version:** 1.0-draft · **Erstellt:** 2026-11-15  
**Status:** DESIGN_RECONSTRUCTED_FROM_CONFIRMED_DECISIONS / SOURCE_REVIEW_PENDING / IMPLEMENTATION_NOT_VERIFIED  
**Freigabe:** Keine Live-Submission, keine automatische Brokeraktion, keine kanonische Wire-Publikation.

## 0. Quellenstatus und Normsprache

Dieser Vertrag formalisiert die im Projekt bestätigten **DET-01 Q01–Q10 (jeweils Variante C)**. Der ursprüngliche vollständige Q&A-Primärtext liegt hier nicht als separates Quelldokument vor. Ergänzende technische Ausformulierungen sind als **Referenzdesign** und nicht als nachträglich bestätigte Einzelentscheidungen zu lesen. Vorrang haben bestätigte REV/OPEN/DET-Entscheidungen und die aufgelösten CR-B01–B09; Konflikte sind zu protokollieren und fail-closed zu behandeln. **MUSS** = Vertragsanforderung; **SOLL** = begründungspflichtiger Standard; **REFERENZ** = noch abnahmebedürftige konkrete Ausgestaltung.

## 1. Geltungsbereich

DET-01 definiert den fachlichen Order-Command-Workflow, die mehrdimensionale Finalität, persistierte Evidenz und die deterministische Verarbeitung verspäteter Brokerereignisse. Er **implementiert nicht** den Brokeradapter (DET-03/05), die Risk-Engine (DET-02), den Dispatcher (DET-07) oder Reconciliation (DET-08).

## 2. Beschlüsse Q01–Q10

| ID | Bestätigter Architekturgrundsatz | Verifikation |
|---|---|---|
| DET01-Q01 | Abschlussgrund separat vom Workflow-State; Workflow-Abschluss nicht mit wirtschaftlicher Finalität gleichsetzen | OPEN |
| DET01-Q02 | Evidenzbasierte konservative Konfliktlösung; Reconciliation oder manuelle Entscheidung bei Ambiguität | OPEN |
| DET01-Q03 | Monotone Aggregate-Version und persistierte Command-/Event-IDs sowie Broker-Evidenz | OPEN |
| DET01-Q04 | Hybride Concurrency Control: CAS und gezielte Datenbanksperren | OPEN |
| DET01-Q05 | Idempotentes Execution Ledger; Korrekturen nur kontrolliert und auditierbar | OPEN |
| DET01-Q06 | Relationales Order-Aggregate mit transaktionalen Event-/Command-/Evidence-Tabellen | OPEN |
| DET01-Q07 | Explizites Recovery-Protokoll; kein stillschweigendes Fortsetzen nach Unklarheit | OPEN |
| DET01-Q08 | Mehrdimensionale Finalität: Workflow historisch abgeschlossen, wirtschaftliche Ausführungen später fortschreibbar | OPEN |
| DET01-Q09 | Append-only Korrekturereignisse mit Originalevidenz, Neuberechnung und Audit | OPEN |
| DET01-Q10 | Formales Transition Model, Property-/Model-Based-Tests, deterministisches Replay und Safety-Invarianten | OPEN |

## 3. Fachmodell und Finalitätsdimensionen

Eine Order besitzt mindestens die **unabhängigen** Projektionen `workflow_state`, `broker_lifecycle_state`, `execution_state`, `risk_reservation_ref` und eine eigenständige `closure_reason`-Evidenz. Diese Dimensionen dürfen nicht zu einem einzigen linearen Status zusammengefasst werden. Die endgültige Liste der Rust-Enum-Varianten ist **nicht** mit diesem Dokument freigegeben; vorhandene Labels bleiben Entwurf bis zur formalen Transition Matrix und Cross-Contract-Abnahme.

- **Workflow-Abschluss:** Ein historisch persistierter Abschluss darf nicht rückwirkend gelöscht oder unbemerkt geöffnet werden.
- **Wirtschaftlicher Zustand:** Fills, Busts und Corrections können nach Workflow-Abschluss eintreffen; sie verändern die Ledger-basierte wirtschaftliche Projektion.
- **Broker-Lifecycle:** Ein Brokerstatus oder ACK beweist weder vollständige Ausführung noch vollständige Abwesenheit späterer Ausführungen.
- **Risk-Finalität:** Reservierungen werden nach DET-02 nur auf hinreichende wirtschaftliche Evidenz umgebucht oder freigegeben.

**REFERENZ `OrderClosure`:** unveränderlicher Record mit `order_id`, `closure_reason`, `closing_event_id`, `order_version`, `closed_at`, `audit_record_id` und Evidenzreferenzen. **Der konkrete Reason-Wortschatz ist noch nicht aus einem autoritativen DET-01-Primärvertrag bestätigt.** Kein Enum und keine kanonische Serialisierung ohne separate Vocabulary-Abnahme.

## 4. Command- und Transaktionsvertrag

1. Eingehende Commands erhalten interne `CommandId`, Idempotency-Key, Payload-Hash, Actor/Authorization-Kontext, `CorrelationId`, erwartete `OrderVersion` und referenzierte Eingabedatenversionen.
2. Wiederholung desselben Idempotency-Key mit identischem Payload MUSS denselben persistierten Ausgang liefern; abweichender Payload MUSS als Konflikt abgewiesen werden.
3. Zustandsänderungen erfolgen unter CAS (`expected_order_version`) und, wo erforderlich, gezielten PostgreSQL-Sperren in der globalen Sperrordnung von CR-B02.
4. Order-Aggregate, Command-Result, Domain-Event, autoritativer Audit-Eintrag, Outbox und erforderliche Risk-/Ledger-Mutationen MÜSSEN atomar in einer lokalen PostgreSQL-Unit-of-Work committed werden.
5. Keine Broker-/NATS-Netzwerkoperation innerhalb dieser Transaktion. Ein erfolgreicher Commit ist **keine** Bestätigung eines Broker-Sends.
6. Unbekannte oder veraltete Versionen, unbekannte Zustände und unvollständige Evidenz führen zu Ablehnung, Halt oder Reconciliation; kein permissiver Fallback.

**REFERENZ-Persistenzmodell:** `orders`, `order_commands`, `order_domain_events`, `order_evidence`, `order_closures`, `execution_ledger`, `execution_corrections`, `audit_events`, `outbox_messages`. Namen und DDL sind **nicht** finalisiert. Ereignisse und Korrekturen sind append-only; Projektionen werden versioniert und aus Evidenz reproduzierbar abgeleitet.

## 5. Konflikte, Late Fills, Busts und Korrekturen

- Eine Execution wird anhand der bestätigten, broker-/account-/session-/kind-spezifischen Identität dedupliziert (DET-03, CR-B04); unklare Identität -> Quarantäne.
- Ein nach Workflow-Abschluss eintreffender Fill MUSS als neue Evidenz erfasst und, sofern eindeutig, im Ledger verbucht werden; der historische Workflow-Abschluss bleibt bestehen.
- Wirtschaftliche Projektionen, Risk-Budgets, Reservierungen und Audit werden deterministisch korrigiert; bei unzureichender Budgetdeckung MUSS ein Risk Incident/Halt ausgelöst werden (CR-B03).
- Busts und Corrections dürfen Originalfills nicht überschreiben. Sie referenzieren Original-Execution und Beleg; eine neue Projektion wird aus der gesamten gültigen Evidenz berechnet.
- Cancel-/Modify-Anfragen haben eigene Autorisierungs-, Risiko- und Broker-Capability-Gates (CR-B09). `CancelRequested` ist kein Beweis für `Cancelled`; Cancel/Fill-Races sind explizit zu testen.
- Keine automatische Auflösung widersprüchlicher Brokerbeobachtungen allein durch Empfangszeit, UUID-Reihenfolge oder letzten Statusstring.

## 6. Formales Transition Model: Freigabegrenze

Die Transition-Funktion ist konzeptionell `transition(current_aggregate, command, evidence, policy_snapshot) -> Result<MutationPlan, Rejection>`. **Die Menge erlaubter Kanten, Guards, Abschlussgründe und Effekte ist hier absichtlich nicht erfunden.** Vor M0-03 MUSS eine versionierte Transition Matrix mit vollständigen Zustands-/Ereignis-Paaren, Guard-Bedingungen, Risk-Effekten, Persistenzaktionen und Negativfällen vorliegen. Bis dahin bleiben `ORDER_STATE_MACHINE_IMPLEMENTED=false` und `STATE_TRANSITIONS_IMPLEMENTED=false`.

## 7. Recovery und Readiness

Bei Restart werden Command-Resultate, Orderversionen, Outbox, Ledger und rohe Broker-Evidenz rekonstruiert. `DISPATCH_STARTED` oder `OUTCOME_UNKNOWN` dürfen niemals allein wegen Timeout als nicht gesendet gelten (CR-B01). Reconciliation (DET-08/CR-B07) prüft Quellvollständigkeit; fehlende Beobachtungen in unvollständigen Snapshots beweisen nichts. Unaufgelöste Konflikte blockieren neue risikosteigernde Submissionen. Manual Reset benötigt separate Autorisierung und Audit (DET-10/CR-B05).

## 8. Invarianten

- **OMS-INV-01:** `order_version` steigt bei jeder autoritativen Mutation monoton; Overflow -> Fehler.
- **OMS-INV-02:** Gleiches Command/gleiche Broker-Execution erzeugt keine doppelte wirtschaftliche Wirkung.
- **OMS-INV-03:** Ein historischer Workflow-Abschluss bleibt unveränderlich; späte wirtschaftliche Änderungen sind append-only.
- **OMS-INV-04:** Kein unbestätigter Brokerstatus erzwingt wirtschaftliche Finalität.
- **OMS-INV-05:** Jede autoritative Mutation hat transaktional persistierte Audit- und Event-Evidenz.
- **OMS-INV-06:** Unklare Identität, unvollständige Brokerbelege oder unbekannte Transition -> fail-closed.
- **OMS-INV-07:** Kein Command umgeht Risk-, Readiness-, Authority- oder Submission-Gates.

## 9. Abnahmetests

| Test-ID | Szenario | Erwartung |
|---|---|---|
| OMS-T01 | Gleiches Command zweimal | Genau eine wirtschaftliche Wirkung, stabiles Resultat |
| OMS-T02 | Gleicher Idempotency-Key, anderer Payload | Konflikt, keine Mutation |
| OMS-T03 | Parallele Commands auf gleiche Version | CAS-/Lock-Sicherheit, kein Lost Update |
| OMS-T04 | Workflow Closed, danach Fill | Ledger/Risk korrigiert, Closure historisch unverändert |
| OMS-T05 | Duplicate Fill, anderer Empfangszeitpunkt | Keine doppelte Buchung |
| OMS-T06 | Fill-Bust-Correction-Replay | Deterministische Projektion und Audit-Kette |
| OMS-T07 | Cancel/Fill-Race | Kein voreiliges Finalisieren oder Freigeben |
| OMS-T08 | Crash vor/nach UoW-Commit | Atomarität von OMS/Risk/Audit/Outbox |
| OMS-T09 | Unbekannter Brokerstatus / Alias | Quarantäne/Halt statt positiver Transition |
| OMS-T10 | Reconciliation mit unvollständigem Snapshot | Keine Abwesenheitsannahme |
| OMS-T11 | Randomisierte Commands und Replay | Safety-Invarianten gelten für alle erzeugten Folgen |
| OMS-T12 | Recovery nach unbekanntem Send-Ausgang | Kein Blind-Retry |

## 10. Abhängigkeiten und offene Punkte

**Benötigt:** DET-02 für Reservation-Rebooking; DET-03 für Identity; DET-04 und CR-B02/08 für Transaktionsatomarität; DET-06 für Readiness; DET-07/CR-B01 für Dispatch-Fencing; DET-08/CR-B07 für Recovery; DET-10/CR-B05 für Audit/Reset; CR-B09 für Cancel/Modify. **Offen:** verbindliche Transition Matrix, `OrderClosure`-Reason-Vokabular, SQL-DDL, Broker-Evidenz-Scopes, property-based Implementierung und Datenbank-/Crash-Tests.

**Abnahme:** Dokumentarische Ergänzung, **keine** technische Implementierungs- oder Live-Freigabe. Alle Abweichungen müssen im Traceability-Index dokumentiert und vom Nutzer freigegeben werden.
