# Trading Platform — P1 Resolution Contracts CR-B05 bis CR-B09

**Version:** 1.0-draft  
**Datum:** 2026-11-14  
**Status:** Architekturentwurf eigenständig entschieden; nicht implementiert und nicht brokerseitig verifiziert  
**Grundlage:** DET-01–10, Cross-Contract Review und P0-Resolution-Contracts. Die ursprünglichen P1-Befunde stammen aus `DET-09-bis-DET-10-und-Cross-Contract-Review-v1.0.md`.

## 0. Geltung und Freigabekriterium

`P1_DESIGN_RESOLVED` bedeutet, dass ein technischer Lösungsvertrag dokumentiert ist, **nicht**, dass externe Brokerannahmen, Policies oder Softwaretests nachgewiesen wurden. P0-Entscheidungen bleiben vorrangig. Keine Live-Trading- oder Implementierungsfreigabe. Jede neue sicherheitsrelevante Abweichung erfordert versionierte ADFR-Änderung und Regressionstests.

## CR-B05 — Solo-Operator, Reset & Break-Glass

**Betroffene Verträge:** DET-06/10.

**Entscheidung P1-B05-D01:** Einzelanwenderbetrieb nutzt getrennte logische Berechtigungen statt fingierter Zweitfreigaben: `OPERATOR`, `RISK_ADMIN`, `INCIDENT_RESPONDER`, `AUDITOR`; Identitäten und Aktionen werden auditiert. Für sicherheitskritische Produktionsfreigaben, die eine unabhängige Zweitperson verlangen, gilt ohne verfügbare Zweitperson `DENY` und nicht ein simuliertes Dual Approval. **P1-B05-D02:** Break-Glass ist ein zeitlich begrenztes, auf Aktion/Incident/Account eingeschränktes Token mit Grund, starker Re-Authentisierung, Ablauf, einmaligem Nonce und Audit; niemals Bypass von Risk Hard Limits, Writer-Fencing, Identity/Reconciliation oder persistiertem Halt ohne definierte Reset-Gates. **P1-B05-D03:** `MANUAL_RESET` benötigt einen frischen, gebundenen Evidence-Snapshot (Readiness, Risk, Broker, Writer, Audit), erwartete Halt-Version und CAS; unvollständige Evidenz -> `DENY`. **Tests:** B05-T01 fehlende zweite Person; T02 abgelaufenes Token; T03 Scope-Fehler; T04 Hard-Invariant-Bypass; T05 konkurrierender Reset; T06 veraltete Evidenz. **Verifikation offen:** tatsächliche Identity-/MFA-Anbindung und Berechtigungsdurchsetzung.

## CR-B06 — Instrument-, Bewertungs- & Rundungspolicy

**Betroffene Verträge:** DET-02/09.

**P1-B06-D01:** Jede Bewertung bindet `instrument_policy_version`, `market_snapshot_id`, `fx_snapshot_id`, `pricing_method_version`, `rounding_policy_version`, `account_base_currency` und `valuation_as_of`. Policies werden vor Orderannahme festgelegt und im Audit referenziert. **P1-B06-D02:** Kein Float für Geld, Preise, Mengen oder FX; dezimale/skalierte Typen mit geprüfter Overflow-Behandlung. Kaufkraft-/Reservebedarf wird konservativ aufgerundet, freigegebene Kapazität konservativ abgerundet; instrumentbezogene Tick- und Lot-Schritte sind separat geprüft. **P1-B06-D03:** Für V1 Aktien/ETFs: bewertete Notional-Exposure, FX-Umrechnung und Konzentration als verpflichtende Basismetriken; Margin-/Short-/Derivate-Capabilities nur bei gesondert getesteter Policy. Kein erfundener universeller Slippage- oder Staleness-Grenzwert: Policy-Parameter sind versionierte Konfiguration und vor Live-Betrieb zu kalibrieren und abzunehmen. **P1-B06-D04:** Fehlen Preis, FX, Marktzustand oder instrumentgerechte Einheit, erfolgt `INDETERMINATE` und kein neues Risiko; nur vorab freigegebene konservative Fallback-Policy zulässig. **Tests:** B06-T01 Preis-Tick; T02 FX-Stale; T03 Rundungsgrenze; T04 Overflow; T05 unterschiedliche Währungen; T06 fehlende Margin-Capability; T07 Snapshot-Replay. **Verifikation offen:** echte Instrumentstammdaten, IBKR-Margin-/FX-Semantik und konkrete Limits.

## CR-B07 — Broker Snapshot Completeness

**Betroffene Verträge:** DET-05/08.

**P1-B07-D01:** Ein Broker-Snapshot ist nur für einen benannten Scope vollständig: `(broker, account, connection_epoch, dataset, query_window, request_id, received_at, completeness_evidence, adapter_version)`. Datensätze: offene Orders, Executions, Positionen, gegebenenfalls Cash. **P1-B07-D02:** `COMPLETE` darf der Adapter nur nach dokumentiertem, getesteten Abschlusskriterium für den jeweiligen IBKR-Query-Typ melden. Ein einzelner leerer Response ist **kein** Negativnachweis. **P1-B07-D03:** Snapshot-Watermarks sind datensatz- und scope-spezifisch; Ausführungen mit verspäteter Zustellung oder begrenzter History können Reconciliation offenhalten. Fehlende historische Abdeckung -> `PARTIAL/UNKNOWN` und kein automatischer Abschluss oder Release. **P1-B07-D04:** Negative Evidenz für 'Order existiert nicht' erfordert hinreichend vollständige Abdeckung *und* Identitäts-/Attempt-Evidenz; im Zweifel Quarantäne. **Tests:** B07-T01 leere Teilantwort; T02 Disconnect während Stream; T03 fehlendes Endmarker-Signal; T04 Paging-/Window-Lücke; T05 out-of-order Execution; T06 Reconnect mit anderer Session; T07 unbekannte externe Order. **Verifikation offen:** IBKR-spezifische Completion-Signale, History-Retention und Account-Scope durch Adaptertests.

## CR-B08 — Audit-Atomizität & Export

**Betroffene Verträge:** DET-04/10.

**P1-B08-D01:** Ein kanonischer `audit_events`-Eintrag wird im **gleichen PostgreSQL Unit of Work** wie OMS-/Risk-/Outbox-Mutation geschrieben. `audit_event_id` und `domain_event_id` sind getrennt, mit `causation_id`, `correlation_id`, `aggregate_version`, `actor`, `policy_version`, `evidence_refs`, `occurred_at`, `recorded_at`. **P1-B08-D02:** Outbox und NATS transportieren abgeleitete, idempotent konsumierbare Projektionen; weder Export noch externe Log-Plattform ist alleinige Audit-Autorität. **P1-B08-D03:** append-only, manipulationserschwerende Verkettung/Signierung in überprüfbaren Batches; ein Hash-Chain-Nachweis ersetzt keine sichere Schlüsselverwaltung oder unveränderliche externe Ablage. **P1-B08-D04:** Löschung/Retention nach Datenklassifikation und Schweizer/DSGVO-Review; keine pauschale unbegrenzte Aufbewahrung. **Tests:** B08-T01 Rollback entfernt Mutation und Audit gemeinsam; T02 Commit enthält Audit+Outbox; T03 Export-Duplikat; T04 Hash-Chain-Manipulation; T05 Exportausfall; T06 Replay aus kanonischer Quelle. **Verifikation offen:** Retention-Policy, Signaturschlüssel, Backup-/Restore-Nachweis.

## CR-B09 — Cancel, Modify & Risk Reduction Dispatch

**Betroffene Verträge:** DET-01/02/05/06/07.

**P1-B09-D01:** Commands werden typisiert: `PLACE`, `CANCEL`, `MODIFY`, `RISK_REDUCE` (letzteres als Policy-Klasse, nicht automatisch als Broker-Ordertyp). Jede Aktion hat eigene Capability-, Identity-, Readiness-, Writer-, State- und Audit-Gates. **P1-B09-D02:** `CANCEL` darf während eines Submission-Halts zulässig sein, wenn Zielorder eindeutig identifiziert, Writer autorisiert und Brokerverbindung geeignet ist; Cancel-Request != Cancel-Bestätigung. Bei unbekannter Submission kann Cancel nicht als Beweis für Nichtausführung dienen. **P1-B09-D03:** `MODIFY` ist standardmässig gesperrt, bis Broker-Semantik, Änderungs-/Cancel-Replace-Identitäten und Risiko-Differenzreservierung verifiziert sind; V1 bevorzugt explizites Cancel+neue Order nur nach belegter Finalität und erneuter Freigabe. **P1-B09-D04:** `RISK_REDUCE` erfordert beweisbar nicht erhöhte Risikometriken unter aktuellen Positionen und offenen Orders; bei Unbestimmtheit `DENY`. **P1-B09-D05:** Jeder Broker-Command erhält eigene `action_attempt_id`, persistiertes `DISPATCH_STARTED` vor externer Übermittlung, unknown-outcome-Quarantäne und keine blinden Retries. **Tests:** B09-T01 Cancel während Halt; T02 Cancel/Fill-Race; T03 Modify nach Partial Fill; T04 unbekannter Place+Cancel; T05 alter Writer; T06 risikosteigernder vermeintlicher Reduce; T07 doppelte Cancel-Ack; T08 Cancel-Broker-Timeout. **Verifikation offen:** IBKR-spezifische Modify-/Cancel- und OrderRef-Semantik.

## 6. Cross-Contract-Acceptance-Gates

| Gate | Nachweis | Status |
|---|---|---|
| GATE-P1-05 | Authz/Reset/Break-Glass-Matrix und negative Tests | DESIGN_ONLY |
| GATE-P1-06 | Versionierte Instrument-/FX-/Bewertungspolicies mit Grenzfalltests | DESIGN_ONLY |
| GATE-P1-07 | IBKR-Source-Completeness je Dataset nachgewiesen | EXTERNAL_VERIFICATION_REQUIRED |
| GATE-P1-08 | OMS/Risk/Audit/Outbox-UoW und Audit-Export-Fault-Tests | DESIGN_ONLY |
| GATE-P1-09 | Action-Matrix, Cancel-/Modify-Races und IBKR-Capability-Tests | EXTERNAL_VERIFICATION_REQUIRED |

**P2-Folgepunkt CR-B10:** Domain Glossary und Schema Registry müssen vor Codegenerierung kanonisiert werden. Gleiche Statuswörter in unterschiedlichen Aggregaten sind als verschiedene Typen zu definieren.

## 7. ADFR-Ergänzungen

| ADFR-ID | Entscheidung | Status | Verifikation |
|---|---|---|---|
| ADFR-P1-001 | Solo-Operator, Reset & Break-Glass — konservativer Vertragsansatz | PROPOSED_FOR_ACCEPTANCE | GATE-P1-05 |
| ADFR-P1-002 | Instrument-, Bewertungs- & Rundungspolicy — konservativer Vertragsansatz | PROPOSED_FOR_ACCEPTANCE | GATE-P1-06 |
| ADFR-P1-003 | Broker Snapshot Completeness — konservativer Vertragsansatz | PROPOSED_FOR_ACCEPTANCE | GATE-P1-07 |
| ADFR-P1-004 | Audit-Atomizität & Export — konservativer Vertragsansatz | PROPOSED_FOR_ACCEPTANCE | GATE-P1-08 |
| ADFR-P1-005 | Cancel, Modify & Risk Reduction Dispatch — konservativer Vertragsansatz | PROPOSED_FOR_ACCEPTANCE | GATE-P1-09 |

## 8. Verbleibende Risiken

- Produktive Sicherheit bleibt von PostgreSQL-, IBKR- und End-to-End-Tests abhängig.
- Preis-/FX-/Margin-Parameter sind absichtlich nicht als universelle Zahlen erfunden.
- Ein Single-Operator-Setup kann unabhängige Freigabe nicht glaubwürdig simulieren; solche Aktionen bleiben gesperrt.
- Brokerseitige Exactly-once-Semantik wird nicht angenommen.
