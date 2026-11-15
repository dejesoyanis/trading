# DET-02 — Risk Contract: Architekturvertrag

**Version:** 1.0-draft · **Erstellt:** 2026-11-15  
**Status:** DESIGN_RECONSTRUCTED_FROM_CONFIRMED_DECISIONS / SOURCE_REVIEW_PENDING / IMPLEMENTATION_NOT_VERIFIED  
**Freigabe:** Keine produktive Risk-Entscheidung, keine Live-Submission, keine kanonische Wire-Publikation.

## 0. Quellenstatus und Normsprache

Dieses Dokument formalisiert die bestätigten **DET-02 Q01–Q10 (jeweils Variante C)**. Der ursprüngliche vollständige Q&A-Primärtext ist nicht als separates Vertragsdokument verfügbar. Konkrete API-, Tabellen- und Policy-Namen sind **REFERENZDESIGN**, nicht stillschweigend ratifizierte Implementierungsvorgaben. Vorrang: bestätigte REV/OPEN-Entscheidungen, CR-B01–B09 und Cross-Contract-Abnahme. **MUSS** = Vertragsanforderung; **SOLL** = begründungspflichtiger Standard.

## 1. Geltungsbereich

Die isolierte deterministische Rust-Risk-Domain bewertet Order-Commands, mehrdimensionale Budgets, atomare Reservierungen und wirtschaftliche Nachbuchungen. Sie ist keine Broker- oder Marktpreisquelle und darf keine eigenständigen Order-Submissions auslösen. Risk-Entscheidungen müssen auf versionierten, nachvollziehbaren Eingaben beruhen.

## 2. Beschlüsse Q01–Q10

| ID | Bestätigter Architekturgrundsatz | Verifikation |
|---|---|---|
| DET02-Q01 | Hierarchische mehrdimensionale Risk Budgets: global, Konto, Strategie, Instrument, Sektor und Risikofaktoren | OPEN |
| DET02-Q02 | Deterministische Policy-Komposition mit Deny-Overrides, Voraussetzungen, versionierten Regeln und Fail-Closed | OPEN |
| DET02-Q03 | Atomare PostgreSQL-Reservierungen mit Budget-Constraints, eindeutigen IDs und expliziten Transaktionsgrenzen | OPEN |
| DET02-Q04 | Preis-, Liquiditäts- und Expositionsprüfungen mit verifizierten Eingabedaten | OPEN |
| DET02-Q05 | Partial-Fill-Umbuchung von Reservation in wirtschaftliche Exposition | OPEN |
| DET02-Q06 | Freigabe nur bei bestätigter wirtschaftlicher Finalität | OPEN |
| DET02-Q07 | Vollständiger Risk-Audit-Trail und Reporting | OPEN |
| DET02-Q08 | Nicht übersteuerbare Sicherheitsinvarianten, eng begrenzte und befristete Ausnahmen | OPEN |
| DET02-Q09 | Mehrstufige Reaktion: Warnung, Blockierung, Halt und kontrollierte sichere Reduktion | OPEN |
| DET02-Q10 | Unit-/Property-Based-/Replay-Tests und reproduzierbare Safety-Gates | OPEN |

## 3. Budgetmodell und Policy-Komposition

Eine Order kann gleichzeitig mehrere Limits belasten: `global`, `account`, `strategy`, `instrument`, `sector`, `risk_factor`; mehrere Metriken (z. B. Notional, offene Orderexposition, Konzentration) sind getrennt zu bewerten. Ein **Referenzmodell** ist `BudgetKey(scope_type, scope_id, metric_id, currency_or_unit, policy_version)`; die finale Dimensionierung und Aggregationsformel müssen im Policy-Register festgelegt werden.

**Deny-Overrides:** Jede verletzte harte Regel blockiert, auch wenn andere Regeln erlauben. Fehlende Pflichtdaten, unbekannte Versionen, nicht vergleichbare Währungen/Einheiten, veraltete Marktpreise oder unbekannte Instrumentdefinitionen führen nicht zu `ALLOW`. Policy-Ausnahmen dürfen nur explizit autorisierte weiche Regeln betreffen; harte Sicherheitsinvarianten sind nicht übersteuerbar. Policy-Auswertung muss rein deterministisch aus einem versionierten Input-Snapshot erfolgen.

**REFERENZ-Entscheidungsfunktion:** `evaluate(policy_version, order_intent, market_snapshot, instrument_definition, positions, open_exposure, reservations, budgets) -> RiskEvaluation`. `RiskEvaluation` enthält Entscheidung, abgelehnte/erfüllte Regeln, berechnete Budgetdeltas, verwendete Snapshot-/Versionsreferenzen und Auditbelege. Diese Signatur ist nicht als kanonische API veröffentlicht.

## 4. Reservierung und Atomarität

1. Vor einer neuen risikosteigernden Order MUSS der Risk-Verbrauch inklusive bereits reservierter und offener Exposition auf **allen** betroffenen Budgetdimensionen geprüft werden.
2. Betroffene Budgetzeilen werden in der kanonischen Reihenfolge von CR-B02 gesperrt; die Risk-Evaluation wird gegen die gesperrten aktuellen Werte erneut validiert.
3. Risk Reservation, Ordermutation, Command-Result, Audit und Outbox werden in **derselben PostgreSQL-Unit-of-Work** committed (CR-B02/CR-B08).
4. Eindeutige `RiskReservationId` und idempotente Command-/Reservation-Referenzen verhindern doppelte Budgetbelastung. Keine Broker- oder NATS-Aufrufe innerhalb der Transaktion.
5. Eine abgelehnte Reservierung darf keinen partiellen Budgetverbrauch zurücklassen. Lock-Konflikte und Deadlocks führen zu vollständiger lokaler Neubewertung oder sicherer Ablehnung, nicht zu stillschweigender Freigabe.
6. Für `budget_limit`, `used`, `reserved` gelten operationstyp-spezifische Invarianten. **REFERENZ:** `used + reserved + proposed_delta <= limit` für eine positive, in derselben Einheit bewertete Metrik; negative/offsetting Exposition, Netting, Margin und Stressmetriken erfordern eigene versionierte Regeln und dürfen nicht durch diese einfache Formel implizit erlaubt werden.

## 5. Partial Fills, Late Fills und Release

- **Partial Fill:** bestätigte Ausführung reduziert die zugehörige offene Reservation nach klarer Umbuchungsregel und erhöht die gebuchte wirtschaftliche Exposition atomar. Doppelte Execution darf keinen zweiten Effekt haben.
- **Late Fill nach Workflow-Closure:** muss auch nach vermeintlicher Orderfinalität verarbeitet werden (DET-01/CR-B03). Überlimit kann Risk Incident und Halt auslösen; wirtschaftliche Evidenz darf nicht verworfen werden.
- **Cancel/Reject:** Broker-Acknowledgement oder `CancelRequested` allein rechtfertigen keine vollständige Reservation-Freigabe. Es braucht nachgewiesene wirtschaftliche Finalität, inklusive ausreichender Broker-/Execution-Snapshot-Vollständigkeit (CR-B07).
- **Bust/Correction:** append-only Ledger-Korrektur und deterministische Neubewertung; keine unsichtbare negative Reservierung oder rückwirkendes Überschreiben von Fills.
- **Unbekannte Brokerlage:** konservative Reservation behalten oder in expliziten Recovery-/Incident-Zustand überführen; keine automatische Freigabe aufgrund von Timeout.

## 6. Preise, Instrumente, FX und Rundung

Risk darf ausschliesslich versionierte, qualitativ geprüfte `MarketSnapshotId`, Instrumentdefinitionen, Tick-/Lot-Regeln und FX-Kurse verwenden (DET-09, CR-B06). `DecimalAmount` und `Money` bleiben exakte, checked Werte; numerische Gleichheit ist nicht automatisch Repräsentationsgleichheit. Kein implizites FX, kein stilles Rescale, keine erfundene Rundung. Ein `RoundingPolicy`-Enum ohne implementierte, getestete Operation erteilt keine Freigabe für Tick-/FX-Berechnungen. Preis-, Liquiditäts- oder FX-Lücken -> fail-closed. Vor Commit müssen Version und Freshness der Eingaben erneut geprüft werden.

## 7. Risk Response und Ausnahmeprotokoll

**WARN:** dokumentierter Hinweis ohne Umgehung harter Gates. **BLOCK:** einzelne Aktion wird verweigert. **HALT:** neue risikosteigernde Aktionen werden scopebezogen oder global gestoppt. **SAFE_REDUCTION:** nur nach separater Prüfung, dass die konkrete Reduktionsaktion tatsächlich Risiko verringert, keine verbotenen Nebenwirkungen hat und durch Broker-/Readiness-/Authority-Gates zulässig ist (CR-B09). Eine Order mit negativem Mengen- oder Notional-Vorzeichen ist **nicht automatisch** risikoreduzierend.

Befristete Ausnahmen benötigen Actor, Begründung, betroffene Regel, Scope, Ablaufzeit, Freigabebeleg und Audit; keine Ausnahme darf die nicht übersteuerbaren Safety-Invarianten, Writer-Fencing oder einen globalen Halt aushebeln (CR-B05). Nach Ablauf oder Versionswechsel ist die Ausnahme unwirksam.

## 8. Invarianten

- **RISK-INV-01:** Keine risikosteigernde Submission ohne aktuelle positive Risk-Freigabe und atomare Reservation.
- **RISK-INV-02:** Jede betroffene Budgetdimension wird geprüft; Deny-Overrides gilt ohne Ausnahme für harte Regeln.
- **RISK-INV-03:** Kein Oversubscription durch konkurrierende Orders oder wiederholte Commands.
- **RISK-INV-04:** Keine Freigabe einer Reservation allein durch Workflow-Closure, Timeout oder unvollständigen Broker-Snapshot.
- **RISK-INV-05:** Jeder Fill/Bust/Correction erzeugt höchstens einen wirtschaftlichen Effekt pro eindeutigem Evidenzereignis und ist auditierbar.
- **RISK-INV-06:** Unbekannte oder veraltete Daten, unzulässige FX-/Rundungspfade und Overflow -> fail-closed.
- **RISK-INV-07:** Harte Safety-Gates sind weder durch KI-Agenten noch durch Break-Glass übersteuerbar.
- **RISK-INV-08:** Risk-/OMS-/Audit-/Outbox-Zustand wird atomar committed.

## 9. Abnahmetests

| Test-ID | Szenario | Erwartung |
|---|---|---|
| RISK-T01 | Zwei parallele Orders konkurrieren um letztes Budget | Höchstens eine belastet unzulässig kein Limit |
| RISK-T02 | Gleicher Command/Reservation-Key zweimal | Keine Doppelreservierung |
| RISK-T03 | Globale Regel verbietet, Strategie erlaubt | DENY |
| RISK-T04 | Fehlender FX-Kurs, veralteter Preis, unbekannte Definition | BLOCK statt Default-ALLOW |
| RISK-T05 | Partial Fill | Reservation/Exposition atomar umgebucht |
| RISK-T06 | Cancel-Ack ohne vollständige Execution-Evidenz | Reservation bleibt konservativ |
| RISK-T07 | Late Fill nach Closure | Wirtschaftlich gebucht, Incident bei Überlimit |
| RISK-T08 | Duplicate Execution und Bust | Keine Doppelwirkung, append-only Korrektur |
| RISK-T09 | PostgreSQL-Deadlock, Crash vor/nach Commit | Keine inkonsistenten Teilmutationen |
| RISK-T10 | Ausnahme überschreitet harte Regel | Ablehnung, Audit |
| RISK-T11 | Safe Reduction mit versteckter Risikosteigerung | BLOCK |
| RISK-T12 | Replay randomisierter Budget-/Fill-Sequenzen | Determinismus und Invarianten |
| RISK-T13 | i128 Overflow, Scale-/Currency-Mismatch | Expliziter Fehler, keine stille Rundung |
| RISK-T14 | Halt zwischen Risk-Evaluation und Commit | Commit-Gate verweigert risikosteigernde Aktion |

## 10. Abhängigkeiten und offene Punkte

**Benötigt:** DET-01 Order/Execution Ledger; DET-04/CR-B02/08 transaktionale UoW; DET-06 Readiness; DET-07 Writer Authority; DET-08/CR-B07 Broker-Vollständigkeit; DET-09/CR-B06 Preise/FX; DET-10/CR-B05 Incident/Ausnahmen; CR-B09 Cancel/Modify. **Offen:** formale Policy-Kompositionsalgebra, konkrete Budgetmetriken und Netting-/Margin-Regeln, SQL-DDL und Lock-Order-Verifikation, quantisierte Rundung und FX, Broker-Evidenz, Property-/DB-/Fault-Injection-Tests.

**Abnahme:** Fachliche Vertragsrekonstruktion zur Quellenintegration, **kein** Nachweis einer implementierten Risk Engine. Kein produktiver Risk-Entscheid oder Live-Trading ohne gesonderte Gates.
