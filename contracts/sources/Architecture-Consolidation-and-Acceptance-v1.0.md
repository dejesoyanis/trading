# Trading Platform — Architecture Consolidation & Acceptance Register

**Version:** 1.0-draft · **Datum:** 2026-11-14 · **Status:** DESIGN_RESOLVED / TESTS_NOT_EXECUTED / LIVE_NOT_AUTHORIZED

## Quellen und Grenzen

Dieses Register konsolidiert die im Gespräch bestätigten DET-01/02-Entscheidungen und die drei bereits vorhandenen Architekturdateien sowie die neue P1-Resolution. DET-01/02 liegen hier nicht als vollständige separate kanonische Markdown-Verträge vor: ihre Q01–Q10-Entscheidungen sind aufgenommen, aber eine maschinenlesbare vollständige Transition-/Policy-Spezifikation ist weiterhin ausstehend. Kein GitHub-Code-Audit und keine IBKR-Live-Verifikation wurden durchgeführt.

## Statusmatrix

| Bereich | Designentscheidung | Implementierungsnachweis | Live-Freigabe |
|---|---|---|---|
| DET-01/02 | 20 bestätigte Q-Entscheidungen | offen | nein |
| DET-03–08 | Architekturentwurf | offen | nein |
| DET-09/10 | Architekturentwurf | offen | nein |
| P0 CR-B01–B04 | vier Auflösungsprotokolle dokumentiert | P0-Gates offen | nein |
| P1 CR-B05–B09 | fünf Auflösungsprotokolle dokumentiert | P1-Gates offen | nein |
| P2 CR-B10 | Domain Glossary / Schema Registry | Entwurf spezifiziert; Schema-/CI-Verifikation offen | nein |

## ADFR-Konsolidierung

- 78 eindeutige Register-IDs: DET01=10, DET02=10, DET03–08=22, DET09/10 einschließlich CROSS=22, P0=8, P1=5.
- JSON und CSV sind die kanonischen **Indexdateien**; die Detailbegründungen stehen in den verlinkten Vertragsdateien. `CONFIRMED_DECISION` bedeutet bestätigte Architekturentscheidung, nicht getestete Implementierung.
- Bestehende ADFR-CROSS-Einträge mit früherem Status `OPEN P0` sind **historische Querverweise**. Die P0-Resolution definiert den neuen Designstatus, nicht die Verifikation.

## Verträge und Vorrang

1. Bestätigte REV/OPEN/DET01/DET02-Grundsätze bleiben verbindliche Constraints.
2. P0-Resolution präzisiert DET-03–10 für CR-B01–B04.
3. P1-Resolution präzisiert DET-03–10 für CR-B05–B09.
4. Widerspruch oder nicht spezifizierter externer Brokerfall -> Fail-Closed, Incident/Reconciliation, versionierte Entscheidung statt impliziter Annahme.

## Integrations-Gates vor Order-Submission-Implementierung

| Gate | Prüfkriterium |
|---|---|
| INT-01 | Kanonische Rust-Typen, Enums, Schema Registry und DET-01 Transition Matrix abgenommen |
| INT-02 | OMS/Risk/Audit/Outbox Unit of Work und Lock-Order durch PostgreSQL-Concurrency-/Crash-Tests belegt |
| INT-03 | Writer-Exklusivität, Crash-Fenster und kein Blind-Retry in Fault-Injection-Tests belegt |
| INT-04 | IBKR Order-/Execution-Identity, Source-Completeness und Cancel-/Modify-Capabilities durch Adaptertests belegt |
| INT-05 | Late Fill, Bust, Cancel/Fill-Race, Risk Rebooking und Replay in E2E bestanden |
| INT-06 | Valuation/FX/Instrument Policies versioniert und Grenzfalltests bestanden |
| INT-07 | Incident/Break-Glass/Reset-Autorisierung einschließlich Solo-Operator-Negativtests bestanden |
| INT-08 | Backup, Restore, Audit-Integrität und Trading-Readiness-Recovery bestanden |

**Wichtig:** `DESIGN_RESOLVED` ist nicht `VERIFIED`. Keine Implementierungs- oder Live-Submission-Freigabe durch dieses Dokument.

## Nächster Arbeitsauftrag (separate Freigabe erforderlich)

Zuerst CR-B10 ist als Entwurf dokumentiert; ausführbare Schema Registry und CI-Gates implementieren, danach die exakten DET-01/02-Zustands- und Policy-Tabellen sowie PostgreSQL-Constraints und API-Contracts kanonisieren. Anschliessend einen testorientierten Implementierungsplan mit überprüfbaren Milestones und CI-Gates erstellen. Kein Cursor-Prompt zum Live-Trading ohne explizite Freigabe.
