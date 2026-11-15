//! Documentation checks for ARCH-DEC-49 through ARCH-DEC-62.
//!
//! These checks do not execute an IBKR test, do not connect to a broker,
//! and do not treat a passing assertion as external evidence.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;
use sha2::{Digest, Sha256};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn sha256_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let digest = Sha256::digest(bytes);
    let mut out = String::with_capacity(digest.len() * 2);
    for byte in digest {
        out.push(char::from(HEX[usize::from(byte >> 4)]));
        out.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    out
}

fn read_text(path: &Path) -> String {
    let bytes = fs::read(path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    assert!(
        !bytes.starts_with(&[0xEF, 0xBB, 0xBF]),
        "BOM in {}",
        path.display()
    );
    String::from_utf8(bytes).unwrap_or_else(|_| panic!("utf-8 {}", path.display()))
}

#[test]
fn ibkr_verification_strategy_49_through_62_stays_unimplemented() {
    let root = repo_root();
    let expected = [
        (
            "IBKREVQ-01",
            "ARCH-DEC-49",
            "B",
            "## IBKREVQ-01 — Evidence source quality",
            "PRIMARY_TEXT_PLUS_SECOND_COPY_NOT_EMPIRICAL",
            "Nur tatsächlich beschaffte, versionierte offizielle IBKR-Primärtexte mit Abrufdatum als Primärquelle; zusätzlich zweite unabhängig beschaffte Kopie derselben Version vor einer späteren E4-Zitierung. Zweite Kopie ist keine unabhängige empirische Brokerbestätigung. Keine Quelle, URL, Version oder Beschaffung erfinden.",
            &["NOT_ESTABLISHED"][..],
            &["NOT_ESTABLISHED"][..],
        ),
        (
            "IBKREVQ-02",
            "ARCH-DEC-50",
            "A",
            "## IBKREVQ-02 — Version pinning",
            "ONE_PINNED_TUPLE_PER_PACKAGE",
            "Pro zukünftigem Nachweispaket genau ein gepinntes Produkt-/API-Client-/Plattform-/OS-/Kontomodus-Tupel (und relevante Umgebungsangaben); andere Tupel benötigen getrennte Pakete. Keine Versionswerte erfinden, keine universelle Übertragung.",
            &["IBKR-CLM-26"],
            &["NOT_ESTABLISHED"],
        ),
        (
            "IBKREVQ-03",
            "ARCH-DEC-51",
            "A",
            "## IBKREVQ-03 — Scope taxonomy",
            "SUPPLIED_FIELDS_ONLY_NO_GLOBAL_SNAPSHOT",
            "Account, Client, Session, Dataset, Zeitfenster und Umgebung nur dokumentieren, soweit Request oder Payload sie tatsächlich liefern; fehlende Felder bleiben fehlend. Kein globaler Snapshot und kein Coverage-Nachweis.",
            &["IBKR-CLM-11"],
            &["NOT_ESTABLISHED"],
        ),
        (
            "IBKREVQ-04",
            "ARCH-DEC-52",
            "B",
            "## IBKREVQ-04 — Test environments",
            "ENVIRONMENT_CLASS_LEFT_OPEN",
            "Testumgebungsklasse (Paper/Sandbox/etc.) bis zur tatsächlichen offiziellen E0-Dokumentation offenlassen; später gesondert entscheiden. Keine Testumgebung oder Kontonutzung freigeben.",
            &["NOT_ESTABLISHED"],
            &["NOT_ESTABLISHED"],
        ),
        (
            "IBKREVQ-05",
            "ARCH-DEC-53",
            "B",
            "## IBKREVQ-05 — Read-only limits",
            "NO_READ_UNTIL_NAMED_OFFICIAL_OPERATIONS",
            "Vor gesonderter, datierter Entscheidung über exakt benannte und offiziell dokumentierte zulässige Leseoperationen keinerlei Brokerverbindung/Read. Diese Ratifikation benennt keine Operation und gibt E2 nicht frei. Cancel/Modify/Replace sind niemals als Read zu deklarieren.",
            &["IBKR-CLM-11", "IBKR-CLM-14"],
            &["NOT_ESTABLISHED"],
        ),
        (
            "IBKREVQ-06",
            "ARCH-DEC-54",
            "B",
            "## IBKREVQ-06 — Completeness oracle",
            "CAPTURE_COMPLETE_IS_NOT_BROKER_COMPLETE",
            "Zukünftige Oracle-Notiz dokumentiert Scope, gespeicherte Bytes und lokale Gaps und enthält explizit die Aussage, dass Capture-Vollständigkeit nicht Broker-Vollständigkeit ist und Broker-Vollständigkeit nicht nachgewiesen wurde. Coverage `UNKNOWN`, kein Completeness-Token, leere Antwort kein Negativbeweis.",
            &["IBKR-CLM-14", "IBKR-CLM-15"],
            &["NOT_ESTABLISHED"],
        ),
        (
            "IBKREVQ-07",
            "ARCH-DEC-55",
            "A",
            "## IBKREVQ-07 — Alias uniqueness verification",
            "REUSE_OBSERVATION_NOT_UNIQUENESS",
            "Spätere kontrollierte Untersuchungen dürfen Wiederverwendung/Nichtwiederverwendung von `orderId`, `permId`, `execId` innerhalb eines gepinnten Tupels als Beobachtungen berichten. Keine Eindeutigkeitsgarantie, kein UNIQUE-Index, keine automatische wirtschaftliche Zuordnung; Tests heute nicht autorisiert.",
            &["IBKR-CLM-02", "IBKR-CLM-06", "IBKR-CLM-07", "IBKR-CLM-27"],
            &["NOT_ESTABLISHED"],
        ),
        (
            "IBKREVQ-08",
            "ARCH-DEC-56",
            "A",
            "## IBKREVQ-08 — Correction chains",
            "LINK_ONLY_IF_NEW_PAYLOAD_NAMES_IT",
            "Spätere Correction-/Bust-Referenzen nur dann strukturiert übernehmen, wenn der neue Rohpayload selbst die Referenz enthält. Append-only Originale erhalten; keine erfundene Verknüpfung, keine Reversal-Buchung, kein Risk Release, keine Closure-Änderung.",
            &["IBKR-CLM-08"],
            &["IBKR-SCN-A10", "IBKR-SCN-A11"],
        ),
        (
            "IBKREVQ-09",
            "ARCH-DEC-57",
            "B",
            "## IBKREVQ-09 — Unknown-send fault injection",
            "UNKNOWN_SEND_SYNTHETIC_OFFLINE_ONLY",
            "Unknown-Send-Crash-/Disconnect-Szenarien bis zu einer neuen ausdrücklichen Entscheidung nur synthetisch/offline. Keine Fault Injection gegen Broker, keine E3-Freigabe, kein Retry/Resend/Takeover.",
            &["IBKR-CLM-20", "IBKR-CLM-21", "IBKR-CLM-22"],
            &["NOT_ESTABLISHED"],
        ),
        (
            "IBKREVQ-10",
            "ARCH-DEC-58",
            "A",
            "## IBKREVQ-10 — Writer isolation proof",
            "ISOLATION_CHECKLIST_NOT_TAKEOVER",
            "Zukünftige Writer-Isolation-Nachweise als separat zu belegende Prozess-Stopp- und Connection-Closed-Artefakte oder belastbare Session-Exklusivitätsbestätigung, zusammen mit Reconciliation unbekannter Versuche, verlangen. `WriterEpoch`/Lease-Deadline separat führen. Dies ist eine Nachweis-Checkliste, kein hinreichender Takeover-Beweis, kein operativer Freigabepfad; CR-B01 bleibt vorrangig.",
            &["IBKR-CLM-23", "IBKR-CLM-24", "IBKR-CLM-25"],
            &[
                "IBKR-SCN-C08",
                "IBKR-SCN-C09",
                "IBKR-SCN-C10",
                "IBKR-SCN-C11",
            ],
        ),
        (
            "IBKREVQ-11",
            "ARCH-DEC-59",
            "A",
            "## IBKREVQ-11 — Evidence retention",
            "RAW_PAYLOADS_OUTSIDE_GIT_NOT_CREATED",
            "Künftige Rohpayloads unveränderlich, append-only und geheimnisgeschützt ausserhalb des Git-Repositories verwahren; Hashes, Versions-Tupel und bereinigte Logs; widersprüchliche Originalbytes erhalten. Jetzt weder Store noch Payload erzeugen, keine Secrets ins Git. Speicherort, Zugriffsschutz und Retention bleiben später zu genehmigen.",
            &["IBKR-CLM-07"],
            &["IBKR-SCN-A09"],
        ),
        (
            "IBKREVQ-12",
            "ARCH-DEC-60",
            "A",
            "## IBKREVQ-12 — Revalidation cadence",
            "NEW_TUPLE_NEEDS_NEW_PACKAGE",
            "Bei Änderung von Produkt, API-Client, Plattform, OS oder Kontomodus gelten frühere externe Claims nicht automatisch für das neue Tupel; neues Paket und erneute Entscheidung erforderlich. Kein erfundener Kalenderzeitraum, keine universelle Aussage.",
            &["IBKR-CLM-26"],
            &["IBKR-SCN-B11"],
        ),
        (
            "IBKREVQ-13",
            "ARCH-DEC-61",
            "A",
            "## IBKREVQ-13 — Escalation",
            "STOP_MARKER_IS_NOT_A_ROLE",
            "Unerwartetes Verhalten stoppt die spätere Stufe; nicht autorisierenden Eskalationsmarker vorsehen. Marker ist weder Rolle noch Reset-/Post-/Release-/Dispatch-Recht. Fehlende erforderliche Zweitfreigabe bedeutet Deny. Kein tatsächlicher Stage-Start.",
            &["IBKR-CLM-19"],
            &["NOT_ESTABLISHED"],
        ),
        (
            "IBKREVQ-14",
            "ARCH-DEC-62",
            "B",
            "## IBKREVQ-14 — Future ratification gates",
            "CAPTURE_AND_RATIFICATION_SEPARATE_COMMITS",
            "Künftige Erfassung und Ratifikation in getrennten Instruktionen und getrennten Git-Commits. Eine Review-Notiz im Capture-Paket ist keine Entscheidung. Nur spätere datierte Nutzerentscheidung kann nach tatsächlichen Artefakten benannte Evidenz akzeptieren; selbst das autorisiert weder Implementierung noch Live-Trading.",
            &["NOT_ESTABLISHED"],
            &["NOT_ESTABLISHED"],
        ),
    ];

    let index_path = root.join("contracts/verification-plans/TRD-ARCH-03B42-resolution-index.json");
    let index_bytes = fs::read(&index_path).expect("resolution index");
    assert!(!index_bytes.starts_with(&[0xEF, 0xBB, 0xBF]));
    let index: Value = serde_json::from_slice(&index_bytes).expect("parse resolution index");
    assert_eq!(
        index["document_role"],
        "NON_RUNTIME_NON_WIRE_RESOLUTION_INDEX"
    );
    assert_ne!(
        index["document_role"],
        "NON_RUNTIME_NON_WIRE_VERIFICATION_PLAN_INDEX"
    );
    assert_eq!(index["task"], "TRD-ARCH-03B.4.2");
    assert_eq!(index["date"], "2026-11-15");
    assert_eq!(index["record_label"], "RATIFIED_DESIGN_POLICY_ONLY");
    assert_eq!(index["implementation"], "NOT_IMPLEMENTED");
    assert_eq!(index["verification"], "NOT_VERIFIED");
    assert_eq!(index["execution"], "NOT_EXECUTED");
    assert_eq!(index["external_evidence"], "EXTERNAL_EVIDENCE_PENDING");
    assert_eq!(index["choice_checksum"], "B,A,A,B,B,B,A,A,B,A,A,A,A,B");
    assert_eq!(index["choice_a_count"], 8);
    assert_eq!(index["choice_b_count"], 6);
    assert_eq!(index["next_unused_decision_id"], "ARCH-DEC-63");
    assert_eq!(index["option_c"], "CONFLICTS_WITH_BINDING_POLICY");
    assert_eq!(index["option_c_adopted"], false);
    assert_eq!(index["recommendation_is_approval"], false);
    assert_eq!(index["authority"], "user_ratification_dated_2026-11-15");
    assert!(index["evidence_uri"].is_null());
    assert!(index["signature_artifact"].is_null());
    assert_eq!(
        index["plan_package_historical_ratification"],
        "NOT_RATIFIED"
    );
    assert_eq!(
        index["plan_questions_historical_status"],
        "USER_DECISION_PENDING"
    );
    assert_eq!(index["historical_approval_prefix_bytes"], 29485);
    assert_eq!(
        index["historical_approval_prefix_sha256"],
        "7e5046f52d84e31496acb9938ab046b11ba4baf671a8ecd0703561ead2beaa63"
    );
    assert_eq!(
        index["tests_executed_scope"],
        "external_broker_and_coverage_tests"
    );
    assert_eq!(index["human_checkpoint"], "STOP");
    assert_eq!(index["part_b"], "NO-GO");
    assert_eq!(index["broker_uniqueness_evidence"], "EVIDENCE_PENDING");
    assert_eq!(index["registry_status_required"], "SKELETON_NOT_CANONICAL");
    assert_eq!(index["cr_b01_precedence_over_det07"], "IN_FORCE");
    assert_eq!(index["xc_01"], "OPEN");
    assert_eq!(index["xc_01_evidence"], "EVIDENCE_PENDING");
    assert_eq!(index["xc_08_ibkr"], "OPEN");
    assert_eq!(index["xc_08_ibkr_evidence"], "EXTERNAL_EVIDENCE_PENDING");
    assert_eq!(index["xc_04_precedence"], "RESOLVED_BY_ARCH_DEC_03");
    assert_eq!(index["xc_04_remainder"], "OPEN");
    assert_eq!(index["xc_02_pred"], "OPEN");
    assert_eq!(index["xc_03_matrix"], "OPEN");
    for stage in ["stage_e0", "stage_e1", "stage_e2", "stage_e3", "stage_e4"] {
        assert_eq!(index[stage], "STOP", "{stage}");
    }
    for flag in [
        "added_to_canonical_registry",
        "implementation_authorized",
        "testing_authorized",
        "broker_connection_authorized",
        "read_only_broker_access_authorized",
        "order_send_authorized",
        "paper_trading_authorized",
        "uniqueness_guaranteed",
        "uniqueness_index_approved",
        "identity_resolver_implemented",
        "completeness_proof_approved",
        "completeness_token_ratified",
        "canonical_schemas_published",
        "canonical_schema_published_by_this_index",
        "canonical_broker_identity_tuple_approved",
        "economic_posting_approved",
        "risk_release_approved",
        "economic_finality_predicate_approved",
        "oms_transitions_approved",
        "broker_dispatch_approved",
        "live_trading_authorized",
        "closes_xc_findings",
        "conflict_closure",
        "order_state_machine_implemented",
        "state_transitions_implemented",
        "order_closure_enum_approved",
        "broker_dispatch_implemented",
        "tests_executed",
        "external_broker_tests_executed",
        "external_evidence_claimed",
        "external_broker_requests_executed",
        "stage_e0_authorized",
        "stage_e1_authorized",
        "stage_e2_authorized",
        "stage_e3_authorized",
        "stage_e4_authorized",
        "recommendation_is_approval",
        "option_c_adopted",
    ] {
        assert_eq!(index[flag], false, "{flag} must stay false");
    }
    assert_eq!(
        index["documentation_regression_tests_are_not_broker_evidence"],
        true
    );
    assert_eq!(
        index["documentation_tests_do_not_set_external_broker_tests_executed"],
        true
    );
    assert_eq!(index["external_broker_tests_executed"], false);
    assert_ne!(
        index["documentation_regression_tests_are_not_broker_evidence"],
        index["external_broker_tests_executed"]
    );

    let questions = index["questions"].as_array().expect("questions");
    assert_eq!(questions.len(), 14);
    let mut choices = String::new();
    let mut seen_questions = BTreeSet::new();
    let mut seen_decisions = BTreeSet::new();
    let decision = read_text(&root.join("contracts/decisions/ARCH-DEC-49-bis-62-2026-11-15.md"));
    for (question, row) in questions.iter().zip(expected) {
        let (id, decision_id, choice, anchor, scope, wording, claims, scenarios) = row;
        assert!(seen_questions.insert(id), "duplicate {id}");
        assert!(
            seen_decisions.insert(decision_id),
            "duplicate {decision_id}"
        );
        assert_eq!(question["id"], id);
        assert_eq!(question["decision_id"], decision_id);
        assert_eq!(question["choice"], choice);
        assert_eq!(question["wording"], wording);
        assert_eq!(question["source_anchor"], anchor);
        assert_eq!(question["ratification_date"], "2026-11-15");
        assert_eq!(question["scope_code"], scope);
        assert_eq!(question["current_status"], "NOT_EXECUTED");
        assert_eq!(question["evidence"], "EXTERNAL_EVIDENCE_PENDING");
        assert_eq!(question["verification"], "NOT_VERIFIED");
        assert_eq!(question["policy_only"], true);
        assert_eq!(question["implemented"], false);
        assert!(
            !question["depends_on"]
                .as_array()
                .expect("depends")
                .is_empty()
        );
        let claim_ids = question["claims"].as_array().expect("claims").clone();
        let claim_text = claim_ids
            .iter()
            .map(|item| item.as_str().expect("claim").to_string())
            .collect::<Vec<_>>();
        assert_eq!(claim_text, claims, "{id} claims");
        let scenario_ids = question["scenarios"].as_array().expect("scenarios").clone();
        let scenario_text = scenario_ids
            .iter()
            .map(|item| item.as_str().expect("scenario").to_string())
            .collect::<Vec<_>>();
        assert_eq!(scenario_text, scenarios, "{id} scenarios");
        assert_eq!(decision.matches(&format!("## {decision_id}")).count(), 1);
        assert!(decision.contains(&format!("| {decision_id} | {id} | {choice} | {wording} |")));
        assert!(decision.contains(&format!("Scope code: `{scope}`")));
        assert!(decision.contains(anchor));
        assert!(
            decision.matches(wording).count() >= 2,
            "{decision_id} wording"
        );
        if !choices.is_empty() {
            choices.push(',');
        }
        choices.push_str(choice);
        let number: u32 = decision_id
            .trim_start_matches("ARCH-DEC-")
            .parse()
            .expect("decision number");
        assert!((49..=62).contains(&number));
    }
    assert_eq!(choices, "B,A,A,B,B,B,A,A,B,A,A,A,A,B");
    assert_eq!(
        questions
            .iter()
            .filter(|question| question["choice"] == "A")
            .count(),
        8
    );
    assert_eq!(
        questions
            .iter()
            .filter(|question| question["choice"] == "B")
            .count(),
        6
    );
    assert!(questions.iter().all(|question| question["choice"] != "C"));

    for required in [
        "RATIFIED_DESIGN_POLICY_ONLY",
        "NOT_IMPLEMENTED",
        "NOT_VERIFIED",
        "NON_RUNTIME_NON_WIRE",
        "not a canonical status enum",
        "No evidence URI is recorded",
        "none is invented",
        "**NO IMPLEMENTATION.**",
        "**NO VERIFICATION.**",
        "It is not an independent empirical confirmation of broker behavior.",
        "No version value is invented.",
        "There is no global snapshot and no coverage proof.",
        "This record does not choose paper, sandbox, or any other label.",
        "This ratification names no operation.",
        "Cancel, modify, and replace are never classified as reads.",
        "capture completeness is not broker completeness",
        "It is not a uniqueness guarantee.",
        "A structured correction or bust reference is adopted only when the new raw payload itself contains that reference.",
        "unknown-send crash and disconnect scenarios are synthetic or offline only.",
        "The checklist is not sufficient proof of takeover.",
        "This record creates neither a store nor a payload and puts no secret in Git.",
        "earlier external claims do not automatically apply to the new tuple.",
        "The marker is not a role.",
        "Future capture and future ratification use separate instructions and separate Git commits.",
        "A review note in the capture package is not a decision.",
        "`external_broker_tests_executed` stays false.",
        "E0, E1, E2, E3, and E4:** STOP",
        "ARCH-DEC-63",
    ] {
        assert!(decision.contains(required), "decision missing {required}");
    }
    assert!(!decision.contains("https://"));
    assert!(!decision.contains("http://"));
    assert!(!decision.contains("accountId"));
    assert!(!decision.contains("DU123"));
    let index_text = String::from_utf8(index_bytes).expect("index utf-8");
    assert!(!index_text.contains("https://"));
    assert!(!index_text.contains("http://"));
    assert!(!index_text.contains("\"external_broker_tests_executed\": true"));
    assert!(!index_text.contains("\"live_trading_authorized\": true"));
    assert!(!index_text.contains("\"testing_authorized\": true"));

    let approval_bytes =
        fs::read(root.join("contracts/verification-plans/TRD-ARCH-03B4-approval-requests.md"))
            .expect("approval requests");
    const HISTORICAL_BYTES: usize = 29485;
    assert!(approval_bytes.len() > HISTORICAL_BYTES);
    assert_eq!(
        sha256_hex(&approval_bytes[..HISTORICAL_BYTES]),
        "7e5046f52d84e31496acb9938ab046b11ba4baf671a8ecd0703561ead2beaa63"
    );
    let historical = std::str::from_utf8(&approval_bytes[..HISTORICAL_BYTES]).expect("historical");
    assert_eq!(historical.matches("USER_DECISION_PENDING").count(), 29);
    assert_eq!(historical.matches("NOT_RATIFIED").count(), 29);
    assert_eq!(
        historical.matches("CONFLICTS_WITH_BINDING_POLICY").count(),
        29
    );
    assert_eq!(historical.matches("Non-binding recommendation").count(), 14);
    assert_eq!(
        historical
            .matches("This recommendation is not an approval")
            .count(),
        14
    );
    assert!(!historical.contains("ARCH-DEC-49"));
    assert!(historical.contains("No choice is selected."));
    let addendum = std::str::from_utf8(&approval_bytes[HISTORICAL_BYTES..]).expect("addendum");
    assert!(addendum.contains("## Resolution index (2026-11-15, TRD-ARCH-03B.4.2)"));
    assert!(addendum.contains("NON_RUNTIME_NON_WIRE_RESOLUTION_INDEX"));
    assert!(addendum.contains("unchanged historical text"));
    assert!(addendum.contains("| IBKREVQ-07 | ARCH-DEC-55 | A |"));
    assert!(!addendum.contains("https://"));

    let plan_bytes =
        fs::read(root.join("contracts/verification-plans/TRD-ARCH-03B4-plan-index.json"))
            .expect("plan index");
    assert_eq!(plan_bytes.len(), 23386);
    assert_eq!(
        sha256_hex(&plan_bytes),
        "d7e62abc81d541ad08725c754d634e0792ea7f0d60165dab34b5fedb364a15f4"
    );
    let plan: Value = serde_json::from_slice(&plan_bytes).expect("plan");
    assert_eq!(plan["ratification"], "NOT_RATIFIED");
    assert_eq!(plan["questions_status"], "USER_DECISION_PENDING");
    assert_eq!(plan["execution"], "NOT_EXECUTED");
    assert_eq!(plan["external_evidence"], "EXTERNAL_EVIDENCE_PENDING");
    let claims = plan["claims"].as_array().expect("claims");
    let scenarios = plan["scenarios"].as_array().expect("scenarios");
    assert_eq!(claims.len(), 27);
    assert_eq!(scenarios.len(), 43);
    assert!(claims.iter().all(|claim| claim["status"] == "HYPOTHESIS"
        && claim["evidence"] == "EXTERNAL_EVIDENCE_PENDING"
        && claim["execution"] == "NOT_EXECUTED"));
    assert!(
        scenarios
            .iter()
            .all(|scenario| scenario["execution"] == "NOT_EXECUTED"
                && scenario["evidence"] == "EXTERNAL_EVIDENCE_PENDING"
                && scenario["authorization"] == "NOT_AUTHORIZED")
    );
    let scenario_ids = scenarios
        .iter()
        .map(|scenario| scenario["id"].as_str().expect("id").to_string())
        .collect::<BTreeSet<_>>();
    for cited in [
        "IBKR-SCN-A09",
        "IBKR-SCN-A10",
        "IBKR-SCN-A11",
        "IBKR-SCN-B11",
        "IBKR-SCN-C08",
        "IBKR-SCN-C09",
        "IBKR-SCN-C10",
        "IBKR-SCN-C11",
    ] {
        assert!(scenario_ids.contains(cited), "missing {cited}");
    }
    for stage in [
        "stage_e0_authorized",
        "stage_e1_authorized",
        "stage_e2_authorized",
        "stage_e3_authorized",
        "stage_e4_authorized",
    ] {
        assert_eq!(plan[stage], false, "{stage}");
    }
    assert_eq!(plan["human_checkpoint"], "STOP");

    let unchanged = [
        (
            "contracts/verification-plans/TRD-ARCH-03B4-ibkr-evidence-inventory.md",
            "3d25e0e52282a6a35649431ad315ec672bb1f7c8580e70b47378557795f89fc9",
            52371_usize,
        ),
        (
            "contracts/verification-plans/TRD-ARCH-03B4-ibkr-verification-protocol.md",
            "70eea6d1ab6047c56baed77f5ee42631c7ed4da348a2c4387796334a1335530c",
            8773,
        ),
        (
            "contracts/verification-plans/TRD-ARCH-03B4-ibkr-test-matrix.md",
            "d8dc4acc1ee3a2a987f25ac9d184d2e762ae0222202f358c4bc4483595166c45",
            49876,
        ),
        (
            "contracts/verification-plans/TRD-ARCH-03B4-ibkr-evidence-capture-and-provenance.md",
            "9468b6f25949565c8f677155ed045df97dd317d8144da520e24251f6f33bce7b",
            6110,
        ),
        (
            "contracts/verification-plans/TRD-ARCH-03B4-dependency-and-conflict-matrix.md",
            "2f3775d00fd3996503a86548799415ee2e9cb130d9ff0d789baeefaca0ed60e4",
            8083,
        ),
        (
            "contracts/decisions/ARCH-DEC-01-bis-03-2026-11-15.md",
            "9f946806667a3700cb45d5033e509355aeefc60d2216262647f2dbbae034e30a",
            9761,
        ),
        (
            "contracts/decisions/ARCH-DEC-04-bis-08-2026-11-15.md",
            "af2cd4b5c35912fe97d3a8ea36c459259aabf0a7b7d54657fdb7b7bce6149ddc",
            13679,
        ),
        (
            "contracts/decisions/ARCH-DEC-09-bis-20-2026-11-15.md",
            "ddaffca4b9d1d69c430a3241db14460e49adf1e8d54a33ac9f0f07b5b957a7a2",
            24820,
        ),
        (
            "contracts/decisions/ARCH-DEC-21-bis-34-2026-11-15.md",
            "a3afc65a0e1a965ac85a9015acc60da858be0ee57ffe104098f2cf56407e5b07",
            31894,
        ),
        (
            "contracts/decisions/ARCH-DEC-35-bis-48-2026-11-15.md",
            "5c16bc15b2a67fac16f476437d998de834ce823972f40f584647bf607e26b4f5",
            48595,
        ),
        (
            "contracts/registry/registry.json",
            "7abed2af5363726403175c6524bd7a4d05d36b8735ad48442fc26941f8d98197",
            44332,
        ),
        (
            "contracts/source-manifest.json",
            "82b205d27172ad234d46cfa2a59f0f4785128dd7a6282dea1aab32b412fc2808",
            17945,
        ),
        (
            "contracts/sources/ADFR-Consolidated-v1.0.json",
            "272a6dabdb411701a25dd708577dbb498c7a7cd2754d83c612e85454a178a1d5",
            29880,
        ),
        (
            "contracts/sources/ADFR-Consolidated-v1.0.csv",
            "b5577302650e3d497465b93063d149313ec1197a38ddad3a9e7a6f223757ec32",
            13138,
        ),
        (
            "Cargo.toml",
            "5843ca50c51c0c316bf716c7e3412e4d1be1d9a3870d278c6ef0ef09cd186d3f",
            1473,
        ),
        (
            "Cargo.lock",
            "76fd3cefad9eef856ec2d784b1ae373d53ba25a5030a77be00a77d9b8adb80f0",
            50294,
        ),
    ];
    for (relative, hash, len) in unchanged {
        let bytes = fs::read(root.join(relative)).expect(relative);
        assert_eq!(bytes.len(), len, "{relative}");
        assert_eq!(sha256_hex(&bytes), hash, "{relative}");
    }

    let manifest: Value = serde_json::from_slice(
        &fs::read(root.join("contracts/source-manifest.json")).expect("manifest"),
    )
    .expect("manifest json");
    let files = manifest["files"].as_array().expect("files");
    assert_eq!(files.len(), 12);
    for entry in files {
        let relative = entry["repository_path"].as_str().expect("path");
        let bytes = fs::read(root.join(relative)).expect(relative);
        assert_eq!(bytes.len() as u64, entry["bytes"].as_u64().expect("bytes"));
        assert_eq!(
            sha256_hex(&bytes),
            entry["sha256"].as_str().expect("sha256")
        );
        if entry["filename"]
            .as_str()
            .expect("name")
            .starts_with("DET-0")
            && entry["reconstructed_draft"] == true
        {
            assert_eq!(entry["source_text_status"], "SOURCE_REVIEW_PENDING");
            assert_eq!(entry["reviewed"], false);
            assert_eq!(entry["approved"], false);
        }
    }

    let adfr: Value = serde_json::from_slice(
        &fs::read(root.join("contracts/sources/ADFR-Consolidated-v1.0.json")).expect("adfr"),
    )
    .expect("adfr json");
    let entries = adfr["entries"].as_array().expect("entries");
    assert_eq!(entries.len(), 78);
    assert!(
        entries
            .iter()
            .all(|entry| entry["verification"] == "NOT_VERIFIED")
    );
    let superseded = entries
        .iter()
        .filter(|entry| entry["lifecycle"] == "SUPERSEDED")
        .collect::<Vec<_>>();
    assert_eq!(superseded.len(), 1);
    assert_eq!(superseded[0]["id"], "ADFR-CROSS-005");
    assert_eq!(superseded[0]["superseded_by"], "ADFR-CR-B10-001");
    let csv = read_text(&root.join("contracts/sources/ADFR-Consolidated-v1.0.csv"));
    let csv_lines = csv
        .lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();
    assert_eq!(csv_lines.len(), 79);
    let csv_ids = csv_lines
        .iter()
        .skip(1)
        .map(|line| {
            line.split(',')
                .next()
                .expect("id")
                .trim_matches('"')
                .to_string()
        })
        .collect::<Vec<_>>();
    let json_ids = entries
        .iter()
        .map(|entry| entry["id"].as_str().expect("id").to_string())
        .collect::<Vec<_>>();
    assert_eq!(csv_ids, json_ids);
    let registry: Value = serde_json::from_slice(
        &fs::read(root.join("contracts/registry/registry.json")).expect("registry"),
    )
    .expect("registry json");
    assert_eq!(registry["registry_status"], "SKELETON_NOT_CANONICAL");
    assert_eq!(registry["canonical_schemas_published"], false);
    assert_eq!(registry["part_b"], "NO-GO");
    assert_eq!(registry["live_trading_authorized"], false);
    assert_eq!(registry["broker_uniqueness_evidence"], "EVIDENCE_PENDING");
    let registry_text = read_text(&root.join("contracts/registry/registry.json"));
    assert!(!registry_text.contains("TRD-ARCH-03B42"));
    assert!(!registry_text.contains("ARCH-DEC-49"));

    let names = fs::read_dir(root.join("contracts/decisions"))
        .expect("decisions")
        .map(|item| {
            item.expect("entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect::<Vec<_>>();
    assert_eq!(names.len(), 6);
    assert_eq!(
        names
            .iter()
            .filter(|name| name.contains("ARCH-DEC-49"))
            .count(),
        1
    );
    assert!(!names.iter().any(|name| name.contains("ARCH-DEC-63")));

    fn walk_src(dir: &Path, hits: &mut Vec<PathBuf>) {
        let entries = fs::read_dir(dir).expect("read dir");
        for entry in entries {
            let entry = entry.expect("entry");
            let path = entry.path();
            if path.is_dir() {
                if path.file_name().and_then(|name| name.to_str()) == Some("src") {
                    walk_files(&path, hits);
                } else {
                    walk_src(&path, hits);
                }
            }
        }
    }
    fn walk_files(dir: &Path, hits: &mut Vec<PathBuf>) {
        for entry in fs::read_dir(dir).expect("read src") {
            let path = entry.expect("entry").path();
            if path.is_dir() {
                walk_files(&path, hits);
            } else {
                hits.push(path);
            }
        }
    }
    let mut sources = Vec::new();
    for top in ["crates", "apps"] {
        walk_src(&root.join(top), &mut sources);
    }
    assert!(!sources.is_empty());
    for path in sources {
        let text = fs::read_to_string(&path).unwrap_or_default();
        assert!(
            !text.contains("ARCH-DEC-49"),
            "production source names the new decision: {}",
            path.display()
        );
    }

    assert!(
        root.join(
            "docs/cursor-prompts/trading/TRD-ARCH-03B4.2-Ratify-IBKR-Verification-Strategy.md"
        )
        .is_file()
    );
    assert!(
        root.join("docs/architecture/TRD-ARCH-03B.4.2-report.md")
            .is_file()
    );
    let prompt =
        read_text(&root.join(
            "docs/cursor-prompts/trading/TRD-ARCH-03B4.2-Ratify-IBKR-Verification-Strategy.md",
        ));
    assert!(prompt.contains("No signature artifact and no evidence URI are attached"));
    assert!(prompt.contains("B, A, A, B, B, B, A, A, B, A, A, A, A, B"));
    assert!(!prompt.contains("https://interactivebrokers.com"));
    let report = read_text(&root.join("docs/architecture/TRD-ARCH-03B.4.2-report.md"));
    assert!(report.contains("VERDICT:"));
    assert!(report.contains("external_broker_tests_executed=false"));
    assert!(report.contains("PUSHED: NO"));
    assert!(report.contains("Remote CI was not run"));
}
