//! Documentation checks for the ADFR register after ARCH-DEC-02.
//!
//! These checks do not implement trading behavior, do not pass B10-T01
//! through B10-T12, and do not treat `DESIGN_RESOLVED` as a test result.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use serde_json::Value;
use sha2::{Digest, Sha256};

const SUPERSEDED_ID: &str = "ADFR-CROSS-005";
const SUCCESSOR_ID: &str = "ADFR-CR-B10-001";
/// SHA-256 of `serde_json::to_string` over the 77 pre-amendment core objects,
/// in their original order. Non-ASCII stays UTF-8. An independent Python
/// `json.dumps(..., ensure_ascii=True)` of the same objects is
/// `f5ecf07d6ba9ed566be2aeaf6f17030fee66198615a4e87ee5319200a688608f`
/// both before the amendment and on these core fields afterwards.
const ORIGINAL_77_CORE_SHA256: &str =
    "f68e4b94c6437fba1e805a9184ec6b6663b4f8b7fd58c14c177ccd1913c544c2";
const REGISTRY_SHA256: &str = "7abed2af5363726403175c6524bd7a4d05d36b8735ad48442fc26941f8d98197";

const CORE_FIELDS: &[&str] = &[
    "id",
    "decision",
    "source",
    "status",
    "basis",
    "verification",
];

/// SHA-256 and byte length of each source file at TRD-ARCH-SYNC-01 intake.
const PACKAGE_BYTES: &[(&str, &str, u64)] = &[
    (
        "DET-01-Order-Workflow-and-Finality-Contract-v1.0.md",
        "a79bec0d8084594a0f21399c453ab82e2499e237f9deabc2222ae8e28afbadad",
        10277,
    ),
    (
        "DET-02-Risk-Contract-v1.0.md",
        "edbdbc13ac25d9f4a0daa7624d1506353ad4f183aa6a02febec69500582e1646",
        10594,
    ),
    (
        "DET-03-bis-DET-08-Architecture-Contracts-v1.0.md",
        "330e8d6d20768d722a8b03dd0fb57fc549f8bdcbde39d55a5c95a968aebcb6c3",
        25363,
    ),
    (
        "DET-09-bis-DET-10-und-Cross-Contract-Review-v1.0.md",
        "bfe68c71440be3324e97a41fd4ef885783f252e17b3135937239516ed3537895",
        24778,
    ),
    (
        "P0-Resolution-Contracts-CR-B01-bis-CR-B04-v1.0.md",
        "5e55ea07ada2df9ff31a132e9cb42d769488bdb4c4cc7c22c143fe7bbea9a2bc",
        25957,
    ),
    (
        "P1-Resolution-Contracts-CR-B05-bis-CR-B09-v1.0.md",
        "9333b13bb6940effee045fddca326a20b54fc72aa3f7f3325fa924ad0b973ce8",
        9562,
    ),
    (
        "ADFR-Consolidated-v1.0.json",
        "4c587b94f668e34e9c088650bbcc4d86ee51c90cbd3602281ce2b0e93931452c",
        21514,
    ),
    (
        "ADFR-Consolidated-v1.0.csv",
        "242e4a1205ee1a050218fd52b6597af02e9badd13ac842e01df70116ca4dda13",
        11831,
    ),
    (
        "Architecture-Consolidation-and-Acceptance-v1.0.md",
        "334bcfcba5015534184d65c556d0cd2d308238bd76bf61429c2bfe6a877bfd67",
        3676,
    ),
    (
        "Implementation-Roadmap-and-Cursor-Prompt-Plan-v1.0.md",
        "37658b435462b368628fe2ba9c83520ca2f82db3703397fee66fab36aa3c8cb6",
        5213,
    ),
    (
        "CR-B10-Domain-Glossary-and-Schema-Registry-v1.0.md",
        "3dfe82fc187fd8cd2703f1113ffd4e1c03d18849ec5c010ed3026740144e87b9",
        13166,
    ),
    (
        "M0-02A-UD-01-bis-UD-09-Architecture-Resolution-v1.0.md",
        "b9bbb7c28634bbf906a2c5fb6dac685a4a4f5bccacfe37a4f543a3feb308ae17",
        10850,
    ),
];

const AMENDED: &[&str] = &[
    "ADFR-Consolidated-v1.0.json",
    "ADFR-Consolidated-v1.0.csv",
    "Architecture-Consolidation-and-Acceptance-v1.0.md",
];

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AdfrFile {
    schema_version: String,
    register_revision: String,
    as_of: String,
    amended_on: String,
    amendment: String,
    amendment_note: String,
    status: String,
    entries: Vec<AdfrEntry>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AdfrEntry {
    id: String,
    decision: String,
    source: String,
    status: String,
    basis: String,
    verification: String,
    lifecycle: String,
    supersedes: Option<String>,
    superseded_by: Option<String>,
}

#[test]
fn adfr_json_and_csv_share_78_historical_ids() {
    let root = repo_root();
    let adfr = load_adfr(&root);
    let csv = load_csv(&root);

    assert_eq!(adfr.schema_version, "1.0");
    assert_eq!(adfr.register_revision, "1.0+ARCH-DEC-02-2026-11-15");
    assert_eq!(adfr.as_of, "2026-11-14");
    assert_eq!(adfr.amended_on, "2026-11-15");
    assert_eq!(adfr.amendment, "ARCH-DEC-02");
    assert!(adfr.amendment_note.contains("NOT_VERIFIED"));
    assert!(adfr.amendment_note.contains("does not authorize trading"));
    assert_eq!(adfr.status, "ARCHITECTURE_DRAFT_NOT_IMPLEMENTED");
    assert_eq!(adfr.entries.len(), 78);
    assert_eq!(csv.len(), 78);

    let json_ids: Vec<&str> = adfr.entries.iter().map(|entry| entry.id.as_str()).collect();
    let csv_ids: Vec<&str> = csv.iter().map(|row| row[0].as_str()).collect();
    assert_eq!(json_ids, csv_ids, "JSON and CSV ID order diverged");
    assert_eq!(
        json_ids.iter().copied().collect::<BTreeSet<_>>().len(),
        78,
        "IDs are not unique"
    );

    for (entry, row) in adfr.entries.iter().zip(csv.iter()) {
        assert_eq!(row.len(), 9, "CSV row {} does not have 9 columns", entry.id);
        assert_eq!(entry.id, row[0]);
        assert_eq!(entry.decision, row[1]);
        assert_eq!(entry.source, row[2]);
        assert_eq!(entry.status, row[3]);
        assert_eq!(entry.basis, row[4]);
        assert_eq!(entry.verification, row[5]);
        assert_eq!(entry.lifecycle, row[6]);
        assert_eq!(entry.supersedes.as_deref().unwrap_or(""), row[7]);
        assert_eq!(entry.superseded_by.as_deref().unwrap_or(""), row[8]);
        for (label, value) in [
            ("decision", entry.decision.as_str()),
            ("source", entry.source.as_str()),
            ("status", entry.status.as_str()),
            ("basis", entry.basis.as_str()),
            ("verification", entry.verification.as_str()),
        ] {
            assert!(!value.is_empty(), "{} missing {label}", entry.id);
        }
        assert_eq!(entry.verification, "NOT_VERIFIED");
        assert!(
            entry.lifecycle == "NOT_SUPERSEDED" || entry.lifecycle == "SUPERSEDED",
            "{} has lifecycle {}",
            entry.id,
            entry.lifecycle
        );
    }

    let confirmed = adfr
        .entries
        .iter()
        .filter(|entry| entry.status == "CONFIRMED_DECISION")
        .collect::<Vec<_>>();
    assert_eq!(confirmed.len(), 20);
    assert_eq!(
        confirmed
            .iter()
            .filter(|entry| entry.basis == "DET01")
            .count(),
        10
    );
    assert_eq!(
        confirmed
            .iter()
            .filter(|entry| entry.basis == "DET02")
            .count(),
        10
    );
    for entry in &confirmed {
        assert_eq!(entry.source, "conversation: confirmed DET-01/02 Q01-Q10");
        assert_eq!(entry.lifecycle, "NOT_SUPERSEDED");
    }
    assert_eq!(
        adfr.entries
            .iter()
            .filter(|entry| entry.status == "PROPOSED_FOR_ACCEPTANCE")
            .count(),
        57
    );
    assert_eq!(
        adfr.entries
            .iter()
            .filter(|entry| entry.status == "DESIGN_RESOLVED")
            .count(),
        1
    );
}

#[test]
fn original_77_core_fields_are_unchanged() {
    let root = repo_root();
    let value = json_value(&root, "contracts/sources/ADFR-Consolidated-v1.0.json");
    let entries = value["entries"].as_array().expect("entries array");
    let mut core = Vec::new();
    for entry in entries {
        let object = entry.as_object().expect("entry object");
        for field in CORE_FIELDS {
            assert!(object.contains_key(*field), "missing {field}");
        }
        for field in ["lifecycle", "supersedes", "superseded_by"] {
            assert!(object.contains_key(field), "missing {field}");
        }
        if entry["id"] == SUCCESSOR_ID {
            continue;
        }
        let mut map = serde_json::Map::new();
        for field in CORE_FIELDS {
            map.insert((*field).to_string(), entry[*field].clone());
        }
        core.push(Value::Object(map));
    }
    assert_eq!(core.len(), 77);
    let blob = serde_json::to_string(&Value::Array(core)).expect("canonical core JSON");
    assert_eq!(
        sha256_hex(blob.as_bytes()),
        ORIGINAL_77_CORE_SHA256,
        "a pre-existing ADFR core field changed"
    );
}

#[test]
fn supersession_is_only_cross_005_to_cr_b10() {
    let adfr = load_adfr(&repo_root());
    let superseded = adfr
        .entries
        .iter()
        .filter(|entry| entry.lifecycle == "SUPERSEDED")
        .collect::<Vec<_>>();
    assert_eq!(superseded.len(), 1);
    let old = superseded[0];
    assert_eq!(old.id, SUPERSEDED_ID);
    assert_eq!(
        old.decision,
        "Kanonisches Domain Glossary und Schema Registry"
    );
    assert_eq!(
        old.source,
        "DET-09-bis-DET-10-und-Cross-Contract-Review-v1.0.md"
    );
    assert_eq!(old.status, "PROPOSED_FOR_ACCEPTANCE");
    assert_eq!(old.basis, "DET09-10");
    assert_eq!(old.verification, "NOT_VERIFIED");
    assert_eq!(old.supersedes, None);
    assert_eq!(old.superseded_by.as_deref(), Some(SUCCESSOR_ID));

    let successor = adfr
        .entries
        .iter()
        .find(|entry| entry.id == SUCCESSOR_ID)
        .expect("successor");
    assert_eq!(
        successor.decision,
        "Kanonische owner-spezifische Domain-Typen, unabhängige Status-Enums und versionierte Schema Registry mit Kompatibilitäts-Gates"
    );
    assert_eq!(
        successor.source,
        "CR-B10-Domain-Glossary-and-Schema-Registry-v1.0.md"
    );
    assert_eq!(successor.status, "DESIGN_RESOLVED");
    assert_eq!(successor.basis, "CR-B10");
    assert_eq!(successor.verification, "NOT_VERIFIED");
    assert_eq!(successor.lifecycle, "NOT_SUPERSEDED");
    assert_eq!(successor.supersedes.as_deref(), Some(SUPERSEDED_ID));
    assert_eq!(successor.superseded_by, None);

    assert_eq!(
        adfr.entries
            .iter()
            .filter(|entry| entry.lifecycle == "NOT_SUPERSEDED")
            .count(),
        77
    );
    for entry in &adfr.entries {
        if entry.id == SUPERSEDED_ID || entry.id == SUCCESSOR_ID {
            continue;
        }
        assert_eq!(entry.lifecycle, "NOT_SUPERSEDED");
        assert_eq!(entry.supersedes, None);
        assert_eq!(entry.superseded_by, None);
    }
}

#[test]
fn source_manifest_matches_file_bytes_and_keeps_package_hashes() {
    let root = repo_root();
    let manifest = json_value(&root, "contracts/source-manifest.json");
    assert_eq!(manifest["live_trading_authorized"].as_bool(), Some(false));
    assert_eq!(
        manifest["canonical_schemas_published"].as_bool(),
        Some(false)
    );
    assert_eq!(
        manifest["baseline_commit"],
        "41c518830a4f90626a4fa4100b9b9347490f6ca8"
    );
    assert_eq!(
        manifest["ratification"]["part_of_source_package"].as_bool(),
        Some(false)
    );
    assert_eq!(
        manifest["ratification"]["document"],
        "contracts/decisions/ARCH-DEC-01-bis-03-2026-11-15.md"
    );
    let decision = root.join("contracts/decisions/ARCH-DEC-01-bis-03-2026-11-15.md");
    assert!(decision.is_file(), "missing decision record");

    let files = manifest["files"].as_array().expect("files");
    assert_eq!(files.len(), PACKAGE_BYTES.len());
    let mut baselines = 0;
    for expected in PACKAGE_BYTES {
        let (filename, package_sha, package_bytes) = *expected;
        let entry = files
            .iter()
            .find(|item| item["filename"] == filename)
            .unwrap_or_else(|| panic!("manifest missing {filename}"));
        let relative = entry["repository_path"].as_str().expect("path");
        assert_ne!(
            relative,
            "contracts/decisions/ARCH-DEC-01-bis-03-2026-11-15.md"
        );
        let bytes = fs::read(root.join(relative)).unwrap_or_else(|error| {
            panic!("read {relative}: {error}");
        });
        assert_eq!(sha256_hex(&bytes), entry["sha256"].as_str().expect("sha"));
        assert_eq!(bytes.len() as u64, entry["bytes"].as_u64().expect("bytes"));
        assert_eq!(entry["package_sha256"].as_str(), Some(package_sha));
        assert_eq!(entry["package_bytes"].as_u64(), Some(package_bytes));
        assert_eq!(entry["approved"].as_bool(), Some(false));
        let amended = AMENDED.contains(&filename);
        if amended {
            assert_eq!(entry["placement"], "AMENDED_BY_ARCH_DEC_02");
            assert_ne!(sha256_hex(&bytes), package_sha);
        } else {
            assert_ne!(entry["placement"], "AMENDED_BY_ARCH_DEC_02");
            assert_eq!(sha256_hex(&bytes), package_sha);
            assert_eq!(bytes.len() as u64, package_bytes);
        }
        if entry["design_baseline"] == "DESIGN_BASELINE_RATIFIED_WITH_EXCEPTIONS" {
            baselines += 1;
            assert!(
                filename.starts_with("DET-01") || filename.starts_with("DET-02"),
                "{filename} is not a DET-01/02 baseline"
            );
            assert_eq!(entry["open_details_remain_open"].as_bool(), Some(true));
            assert_eq!(entry["reconstructed_draft"].as_bool(), Some(true));
            assert_eq!(entry["reviewed"].as_bool(), Some(false));
            assert_eq!(entry["implemented"].as_bool(), Some(false));
            assert_eq!(entry["design_baseline_decision"], "ARCH-DEC-01");
        }
    }
    assert_eq!(baselines, 2);
}

#[test]
fn baseline_and_precedence_do_not_claim_verification() {
    let root = repo_root();
    let decision = read_text(&root.join("contracts/decisions/ARCH-DEC-01-bis-03-2026-11-15.md"));
    for phrase in [
        "DESIGN_BASELINE_RATIFIED_WITH_EXCEPTIONS",
        "RECONSTRUCTED_DRAFT",
        "Keine Behauptung, die Original-Q&A-Transkripte seien vorhanden oder unabhängig verifiziert.",
        "OrderClosure",
        "Transition-Matrizen",
        "wirtschaftlicher Finalität",
        "Kein automatischer Takeover",
        "Lease-Timeout",
        "kein nachgewiesenes IBKR-Verhalten",
        "RESOLVED_BY_ARCH_DEC_03",
        "RESOLVED_BY_ARCH_DEC_02",
        "XC-03-MATRIX",
        "B10-T01",
        "not passed",
        "NOT AUTHORIZED",
    ] {
        assert!(
            decision.contains(phrase),
            "decision record missing {phrase}"
        );
    }

    let index = read_text(&root.join("contracts/traceability-index.md"));
    for phrase in [
        "source_present",
        "provenance",
        "design_baseline_approved",
        "details_open",
        "implementation",
        "verification",
        "DESIGN_BASELINE_RATIFIED_WITH_EXCEPTIONS",
        "RESOLVED_BY_ARCH_DEC_03",
        "XC-04 remains OPEN",
        "XC-03-MATRIX",
        "NOT_REVIEWED",
        "EVIDENCE_PENDING",
    ] {
        assert!(
            index.contains(phrase),
            "traceability index missing {phrase}"
        );
    }

    let report = read_text(&root.join("docs/architecture/TRD-ARCH-SYNC-02-report.md"));
    for phrase in [
        "VERDICT:",
        "PUSHED: NO",
        "remote CI was not run",
        "XC-03-MATRIX",
        "SKELETON_NOT_CANONICAL",
        "OPEN",
    ] {
        assert!(report.contains(phrase), "SYNC-02 report missing {phrase}");
    }
    assert!(
        !report.contains("remote CI passed"),
        "report must not claim remote CI"
    );

    let consolidation = read_text(
        &root.join("contracts/sources/Architecture-Consolidation-and-Acceptance-v1.0.md"),
    );
    assert!(consolidation.contains("78 eindeutige Register-IDs"));
    assert!(consolidation.contains("Nachtrag 2026-11-15"));
    assert!(consolidation.contains("ADFR-CROSS-005"));
    assert!(consolidation.contains("nicht superseded"));
}

#[test]
fn registry_skeleton_file_is_unchanged_and_closed() {
    let root = repo_root();
    let bytes = fs::read(root.join("contracts/registry/registry.json")).expect("registry");
    assert_eq!(sha256_hex(&bytes), REGISTRY_SHA256);
    let registry: Value = serde_json::from_slice(&bytes).expect("registry JSON");
    assert_eq!(registry["registry_status"], "SKELETON_NOT_CANONICAL");
    assert_eq!(
        registry["canonical_schemas_published"].as_bool(),
        Some(false)
    );
    assert_eq!(registry["live_trading_authorized"].as_bool(), Some(false));
    assert_eq!(registry["broker_uniqueness_evidence"], "EVIDENCE_PENDING");
    assert_eq!(registry["part_b"], "NO-GO");
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct StatusDraft {
    document_role: String,
    artifact_class: String,
    version: String,
    interpretation_layer_ratification: String,
    interpretation_constraints_ratification: String,
    interpretation_constraints_record: String,
    interpretation_constraints_do_not_ratify_token_sets: bool,
    records_ratified_citations: bool,
    not_a_canonical_wire_schema: bool,
    not_a_registry: bool,
    not_runtime: bool,
    adfr_register_copied: bool,
    replaces_adfr_fields: bool,
    mutates_registry: bool,
    closes_xc_findings: bool,
    do_not_publish_as: Vec<String>,
    live_trading_authorized: bool,
    canonical_schemas_published: bool,
    part_b: String,
    broker_uniqueness_evidence: String,
    dispatch_readiness: String,
    who_may_approve: String,
    ratified_fact_citations: Vec<String>,
    source_references: Vec<String>,
    invariants: Vec<StatusInvariant>,
    dimensions: Vec<StatusDimension>,
    legacy_phrases: Vec<LegacyPhrase>,
    example_interpretation: ExampleInterpretation,
    validation_rules: Vec<String>,
    forbidden_promotions: Vec<ForbiddenPromotion>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct StatusInvariant {
    id: String,
    statement: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct StatusDimension {
    id: String,
    ratification: String,
    purpose: String,
    in_force_literals: Vec<String>,
    proposed_values_not_ratified: Vec<String>,
    proposed_values_are_not_assigned: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct LegacyPhrase {
    phrase: String,
    context: String,
    mapping_status: String,
    dimensions: Vec<String>,
    safe_interpretation: String,
    does_not_imply: Vec<String>,
    sources: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ExampleInterpretation {
    example_only: bool,
    role: String,
    subject: String,
    provenance: String,
    design_baseline: String,
    implementation: String,
    verification: String,
    verified: bool,
    live_trading_authorized: bool,
    canonical_schemas_published: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ForbiddenPromotion {
    from: String,
    to: String,
}

const REQUIRED_PHRASES: &[&str] = &[
    "OPEN P0",
    "OPEN P1",
    "PROPOSED_FOR_ACCEPTANCE",
    "CONFIRMED_DECISION",
    "DESIGN_RESOLVED",
    "DESIGN_BASELINE_RATIFIED_WITH_EXCEPTIONS",
    "RECONSTRUCTED_DRAFT",
    "NOT_VERIFIED",
    "EVIDENCE_PENDING",
    "SUPERSEDED",
    "NOT_SUPERSEDED",
    "NOT_IMPLEMENTED",
    "SKELETON_NOT_CANONICAL",
    "NO-GO",
];

const DIMENSION_IDS: &[&str] = &[
    "provenance",
    "decision_lifecycle",
    "implementation",
    "verification",
    "safety_authorization",
    "severity",
    "supersession",
];

#[test]
fn status_draft_is_non_runtime_and_unratified() {
    let root = repo_root();
    let draft = load_status_draft(&root);
    assert_eq!(draft.document_role, "NON_RUNTIME_NON_WIRE_DRAFT");
    assert_eq!(draft.artifact_class, "STATUS_MODEL_INTERPRETATION_DRAFT");
    assert!(draft.version.contains("draft"));
    assert_eq!(
        draft.interpretation_layer_ratification,
        "PROPOSED_NOT_RATIFIED"
    );
    assert!(draft.records_ratified_citations);
    assert!(draft.not_a_canonical_wire_schema);
    assert!(draft.not_a_registry);
    assert!(draft.not_runtime);
    assert!(!draft.adfr_register_copied);
    assert!(!draft.replaces_adfr_fields);
    assert!(!draft.mutates_registry);
    assert!(!draft.closes_xc_findings);
    assert!(!draft.live_trading_authorized);
    assert!(!draft.canonical_schemas_published);
    assert_eq!(draft.part_b, "NO-GO");
    assert_eq!(draft.broker_uniqueness_evidence, "EVIDENCE_PENDING");
    assert_eq!(draft.dispatch_readiness, "NOT_AUTHORIZED");
    assert!(draft.who_may_approve.contains("does not approve"));
    assert!(draft.ratified_fact_citations.len() >= 8);
    assert_eq!(
        draft.interpretation_constraints_ratification,
        "CITED_FROM_DATED_USER_DECISION"
    );
    assert_eq!(
        draft.interpretation_constraints_record,
        "contracts/decisions/ARCH-DEC-04-bis-08-2026-11-15.md"
    );
    assert!(draft.interpretation_constraints_do_not_ratify_token_sets);
    for id in [
        "ARCH-DEC-01",
        "ARCH-DEC-02",
        "ARCH-DEC-03",
        "ARCH-DEC-04",
        "ARCH-DEC-05",
        "ARCH-DEC-06",
        "ARCH-DEC-07",
        "ARCH-DEC-08",
    ] {
        assert!(
            draft
                .ratified_fact_citations
                .iter()
                .any(|citation| citation.contains(id)),
            "missing citation {id}"
        );
    }
    for citation in &draft.ratified_fact_citations {
        assert!(
            citation.contains("ARCH-DEC-0"),
            "citation is not an ARCH-DEC fact: {citation}"
        );
    }
    for source in [
        "contracts/source-manifest.json",
        "contracts/sources/ADFR-Consolidated-v1.0.json",
        "contracts/registry/registry.json",
    ] {
        assert!(
            draft.source_references.iter().any(|item| item == source),
            "draft omits source {source}"
        );
    }
    for path in [
        "registry.yaml",
        "events/v1",
        "commands/v1",
        "api/v1",
        "sql/v1",
    ] {
        assert!(
            draft.do_not_publish_as.iter().any(|item| item == path),
            "draft may publish {path}"
        );
        assert!(
            !root.join("contracts").join(path).exists(),
            "contracts/{path} must stay absent"
        );
    }
    let invariant_ids: BTreeSet<&str> = draft
        .invariants
        .iter()
        .map(|item| item.id.as_str())
        .collect();
    for id in [
        "INV-HASHES",
        "INV-78",
        "INV-ONE-SUPERSESSION",
        "INV-NO-CYCLE",
        "INV-NOT-VERIFIED",
        "INV-RECONSTRUCTED",
        "INV-NO-LIVE",
        "INV-NO-CANONICAL-PUBLISH",
        "INV-NO-XC-CLOSURE",
    ] {
        assert!(invariant_ids.contains(id), "missing {id}");
    }
    for invariant in &draft.invariants {
        assert!(!invariant.statement.is_empty(), "{}", invariant.id);
    }
    assert_eq!(draft.dimensions.len(), DIMENSION_IDS.len());
    for (dimension, expected) in draft.dimensions.iter().zip(DIMENSION_IDS.iter()) {
        assert_eq!(dimension.id, *expected);
        assert!(!dimension.purpose.is_empty());
        assert!(dimension.proposed_values_are_not_assigned);
        assert!(matches!(
            dimension.ratification.as_str(),
            "IN_FORCE" | "MIXED" | "PROPOSED_NOT_RATIFIED"
        ));
    }
    let lifecycle = draft
        .dimensions
        .iter()
        .find(|dimension| dimension.id == "decision_lifecycle")
        .expect("lifecycle dimension");
    assert_eq!(lifecycle.ratification, "MIXED");
    for value in [
        "rejected",
        "withdrawn",
        "proposed",
        "ratified_with_exceptions",
    ] {
        assert!(
            lifecycle
                .proposed_values_not_ratified
                .iter()
                .any(|item| item == value),
            "lifecycle proposal missing {value}"
        );
    }
    let provenance = draft
        .dimensions
        .iter()
        .find(|dimension| dimension.id == "provenance")
        .expect("provenance");
    for value in ["original", "reconstructed", "external_evidence", "unknown"] {
        assert!(
            provenance
                .proposed_values_not_ratified
                .iter()
                .any(|item| item == value),
            "provenance proposal missing {value}"
        );
    }
    assert!(
        provenance
            .in_force_literals
            .iter()
            .any(|item| item == "RECONSTRUCTED_DRAFT")
    );
    let safety = draft
        .dimensions
        .iter()
        .find(|dimension| dimension.id == "safety_authorization")
        .expect("safety");
    assert_eq!(safety.ratification, "IN_FORCE");
    assert!(safety.proposed_values_not_ratified.is_empty());

    let example = &draft.example_interpretation;
    assert!(example.example_only);
    assert!(example.role.contains("Not a new decision"));
    assert!(
        example
            .subject
            .ends_with("DET-01-Order-Workflow-and-Finality-Contract-v1.0.md")
    );
    assert_eq!(example.provenance, "RECONSTRUCTED_DRAFT");
    assert_eq!(
        example.design_baseline,
        "DESIGN_BASELINE_RATIFIED_WITH_EXCEPTIONS"
    );
    assert_eq!(example.implementation, "NOT_IMPLEMENTED");
    assert_eq!(example.verification, "NOT_VERIFIED");
    assert!(!example.verified);
    assert!(!example.live_trading_authorized);
    assert!(!example.canonical_schemas_published);

    let model = read_text(&root.join("contracts/status-model/contract-status-model-v1.0.md"));
    for heading in [
        "## 3. Provenance",
        "## 4. Decision and design lifecycle",
        "## 5. Implementation state",
        "## 6. Verification state",
        "## 7. Safety and authorization",
        "## 8. Severity and priority",
        "## 9. Supersession",
        "PROPOSED_NOT_RATIFIED",
        "Provenance is not approval.",
        "XC-03-MATRIX",
        "XC-11",
    ] {
        assert!(model.contains(heading), "status model missing {heading}");
    }
    assert!(
        !model.contains("live_trading_authorized` is true")
            && !model.contains("live_trading_authorized=true"),
        "status model must not authorize live trading"
    );
}

#[test]
fn legacy_phrase_map_blocks_silent_equivalence() {
    let root = repo_root();
    let draft = load_status_draft(&root);
    let mapping = read_text(&root.join("contracts/status-model/legacy-status-mapping.md"));
    let mut seen = BTreeSet::new();
    for phrase in &draft.legacy_phrases {
        assert!(
            seen.insert((phrase.phrase.as_str(), phrase.context.as_str())),
            "duplicate phrase {} / {}",
            phrase.phrase,
            phrase.context
        );
        assert!(
            matches!(
                phrase.mapping_status.as_str(),
                "DEFINITIVE" | "PROVISIONAL" | "REQUIRES_USER_DECISION"
            ),
            "{} has status {}",
            phrase.phrase,
            phrase.mapping_status
        );
        assert!(!phrase.safe_interpretation.is_empty());
        assert!(!phrase.does_not_imply.is_empty(), "{}", phrase.phrase);
        assert!(!phrase.sources.is_empty(), "{}", phrase.phrase);
        assert!(!phrase.dimensions.is_empty(), "{}", phrase.phrase);
        let heading = format!(
            "### {} ({}) — {}",
            phrase.phrase, phrase.context, phrase.mapping_status
        );
        assert!(mapping.contains(&heading), "mapping missing {heading}");
        for source in &phrase.sources {
            assert!(
                mapping.contains(source),
                "mapping missing source {source} for {}",
                phrase.phrase
            );
        }
    }
    for required in REQUIRED_PHRASES {
        assert!(
            draft
                .legacy_phrases
                .iter()
                .any(|phrase| phrase.phrase == *required),
            "missing phrase {required}"
        );
    }
    assert_phrase_blocks(
        &draft,
        "OPEN P0",
        "REQUIRES_USER_DECISION",
        "PROPOSED_FOR_ACCEPTANCE",
    );
    assert_phrase_blocks(
        &draft,
        "OPEN P1",
        "REQUIRES_USER_DECISION",
        "PROPOSED_FOR_ACCEPTANCE",
    );
    assert_phrase_blocks(
        &draft,
        "PROPOSED_FOR_ACCEPTANCE",
        "REQUIRES_USER_DECISION",
        "DESIGN_RESOLVED",
    );
    assert_phrase_blocks(
        &draft,
        "DESIGN_RESOLVED",
        "REQUIRES_USER_DECISION",
        "verified",
    );
    assert_phrase_blocks(
        &draft,
        "DESIGN_RESOLVED",
        "REQUIRES_USER_DECISION",
        "implemented",
    );
    assert_phrase_blocks(
        &draft,
        "DESIGN_RESOLVED",
        "REQUIRES_USER_DECISION",
        "live_authorized",
    );
    assert_phrase_blocks(&draft, "NOT_VERIFIED", "DEFINITIVE", "verified");
    assert_phrase_blocks(&draft, "NOT_VERIFIED", "DEFINITIVE", "externally_verified");
    assert_phrase_blocks(
        &draft,
        "RECONSTRUCTED_DRAFT",
        "DEFINITIVE",
        "original_qa_transcript",
    );
    assert_phrase_blocks(
        &draft,
        "DESIGN_BASELINE_RATIFIED_WITH_EXCEPTIONS",
        "DEFINITIVE",
        "closed_exceptions",
    );
    assert_phrase_blocks(
        &draft,
        "NO_CONFLICT_FOUND",
        "DEFINITIVE",
        "implementation_permission",
    );
    assert_phrase_blocks(&draft, "SUPERSEDED", "DEFINITIVE", "verified");
    assert_phrase_blocks(&draft, "NOT_SUPERSEDED", "DEFINITIVE", "accepted");
    assert_phrase_blocks(&draft, "NO-GO", "DEFINITIVE", "live_authorized");
    assert_phrase_blocks(
        &draft,
        "CONFIRMED_DECISION",
        "DEFINITIVE",
        "DESIGN_BASELINE_RATIFIED_WITH_EXCEPTIONS",
    );
    for promotion in &draft.forbidden_promotions {
        if promotion.from == "NOT_VERIFIED" {
            assert_eq!(promotion.to, "verified");
        }
        let phrase = draft
            .legacy_phrases
            .iter()
            .find(|item| item.phrase == promotion.from)
            .unwrap_or_else(|| panic!("promotion from unknown phrase {}", promotion.from));
        assert!(
            phrase
                .does_not_imply
                .iter()
                .any(|item| item == &promotion.to),
            "{} does not block {}",
            promotion.from,
            promotion.to
        );
    }
    assert!(
        draft
            .validation_rules
            .iter()
            .any(|rule| rule.contains("NOT_VERIFIED"))
    );
    let approvals = read_text(&root.join("contracts/status-model/approval-requests.md"));
    assert!(approvals.contains("Do not self-approve."));
    assert!(approvals.contains("Nothing here is approved."));
    for id in ["SMQ-01", "SMQ-02", "SMQ-03", "SMQ-04", "SMQ-05"] {
        assert!(approvals.contains(id), "missing {id}");
    }
    assert!(!approvals.contains("Decision: approved"));
}

#[test]
fn status_model_keeps_register_provenance_and_open_findings() {
    let root = repo_root();
    let manifest = json_value(&root, "contracts/source-manifest.json");
    let files = manifest["files"].as_array().expect("files");
    assert_eq!(files.len(), 12);
    let mut reconstructed = 0;
    for entry in files {
        let relative = entry["repository_path"].as_str().expect("path");
        let bytes = fs::read(root.join(relative)).expect("source bytes");
        assert_eq!(
            sha256_hex(&bytes),
            entry["sha256"].as_str().expect("sha"),
            "current hash drifted for {relative}"
        );
        assert_eq!(bytes.len() as u64, entry["bytes"].as_u64().expect("bytes"));
        assert_eq!(entry["approved"].as_bool(), Some(false));
        assert_eq!(entry["implemented"].as_bool(), Some(false));
        if entry["reconstructed_draft"] == true {
            reconstructed += 1;
            assert_eq!(
                entry["design_baseline"],
                "DESIGN_BASELINE_RATIFIED_WITH_EXCEPTIONS"
            );
            assert_eq!(entry["source_text_status"], "SOURCE_REVIEW_PENDING");
            assert_eq!(entry["reviewed"].as_bool(), Some(false));
        }
    }
    assert_eq!(reconstructed, 2);
    assert_eq!(manifest["live_trading_authorized"].as_bool(), Some(false));
    assert_eq!(
        manifest["canonical_schemas_published"].as_bool(),
        Some(false)
    );

    let adfr = load_adfr(&root);
    let csv = load_csv(&root);
    assert_eq!(adfr.entries.len(), 78);
    assert_eq!(csv.len(), 78);
    let mut links = BTreeMap::new();
    let mut superseded = 0;
    for (entry, row) in adfr.entries.iter().zip(csv.iter()) {
        assert_eq!(entry.id, row[0]);
        assert_eq!(entry.verification, "NOT_VERIFIED");
        assert_eq!(entry.verification, row[5]);
        if let Some(target) = entry.supersedes.as_deref() {
            assert!(links.insert(entry.id.as_str(), target).is_none());
        }
        if entry.lifecycle == "SUPERSEDED" {
            superseded += 1;
            assert_eq!(entry.id, SUPERSEDED_ID);
            assert_eq!(entry.superseded_by.as_deref(), Some(SUCCESSOR_ID));
            assert_eq!(entry.status, "PROPOSED_FOR_ACCEPTANCE");
        }
    }
    assert_eq!(superseded, 1);
    assert_eq!(links.get(SUCCESSOR_ID).copied(), Some(SUPERSEDED_ID));
    assert_eq!(links.len(), 1);
    let mut seen = BTreeSet::new();
    let mut cursor = Some(SUCCESSOR_ID);
    while let Some(id) = cursor {
        assert!(seen.insert(id), "supersession cycle at {id}");
        cursor = links.get(id).copied();
    }
    let successor_known = adfr.entries.iter().any(|entry| entry.id == SUCCESSOR_ID);
    assert!(successor_known, "successor missing from the historical IDs");

    let registry_bytes = fs::read(root.join("contracts/registry/registry.json")).expect("registry");
    assert_eq!(sha256_hex(&registry_bytes), REGISTRY_SHA256);
    let registry: Value = serde_json::from_slice(&registry_bytes).expect("registry JSON");
    assert_eq!(registry["registry_status"], "SKELETON_NOT_CANONICAL");
    assert_eq!(registry["part_b"], "NO-GO");
    assert_eq!(
        registry["canonical_schemas_published"].as_bool(),
        Some(false)
    );
    assert_eq!(registry["live_trading_authorized"].as_bool(), Some(false));
    assert_eq!(registry["broker_uniqueness_evidence"], "EVIDENCE_PENDING");

    let index = read_text(&root.join("contracts/traceability-index.md"));
    assert!(
        index.contains("contracts/status-model/contract-status-model-v1.0.md")
            || index.contains("status-model/contract-status-model-v1.0.md")
    );
    assert!(index.contains("XC-04 remains OPEN"));
    assert!(index.contains("XC-03-MATRIX"));
    for finding in [
        "XC-01",
        "XC-02-PRED",
        "XC-03-MATRIX",
        "XC-05-LOCK",
        "XC-06",
        "XC-07-ROUND",
        "XC-08-IBKR",
        "XC-09-RESET",
        "XC-10-ACTION",
    ] {
        assert!(index.contains(finding), "traceability dropped {finding}");
    }

    let unresolved = read_text(&root.join("contracts/domain/unresolved-decisions.md"));
    assert!(unresolved.contains("TRD-ARCH-03A does not change the UD statuses"));
    for finding in [
        "XC-01 stays OPEN",
        "XC-02-PRED stays OPEN",
        "XC-03-MATRIX stays OPEN",
        "XC-05-LOCK stays OPEN",
        "XC-06 stays OPEN",
        "XC-07-ROUND stays OPEN",
        "XC-08-IBKR stays OPEN",
        "XC-09-RESET stays OPEN",
        "XC-10-ACTION stays OPEN",
        "XC-11 status-word disagreement stays OPEN",
    ] {
        assert!(unresolved.contains(finding), "unresolved dropped {finding}");
    }
    assert!(unresolved.contains("status-model/contract-status-model-v1.0.md"));
    assert!(unresolved.contains("DESIGN_RESOLVED"));
    assert!(!unresolved.contains("live_trading_authorized: true"));
}

#[test]
fn interpretation_decisions_04_through_08_stay_conservative() {
    let root = repo_root();
    let decision_path = root.join("contracts/decisions/ARCH-DEC-04-bis-08-2026-11-15.md");
    let decision = read_text(&decision_path);
    let names = fs::read_dir(root.join("contracts/decisions"))
        .expect("decisions")
        .map(|item| {
            item.expect("entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect::<Vec<_>>();
    assert_eq!(
        names
            .iter()
            .filter(|name| name.contains("ARCH-DEC-04"))
            .count(),
        1,
        "ARCH-DEC-04 must be recorded in one file"
    );

    for (heading, smq, wording) in [
        (
            "## ARCH-DEC-04",
            "| ARCH-DEC-04 | SMQ-01 | DECIDED | Alternative 1 |",
            "Keine Priorität; historische Statusbegriffe unverändert lassen",
        ),
        (
            "## ARCH-DEC-05",
            "| ARCH-DEC-05 | SMQ-02 | DECIDED | Alternative 1 |",
            "Kein neues Enum und keine zusätzlichen Lifecycle-Zuordnungen",
        ),
        (
            "## ARCH-DEC-06",
            "| ARCH-DEC-06 | SMQ-03 | DECIDED | Alternative 1 |",
            "Nur konkret getestete Teilkomponenten als lokal getestet behandeln",
        ),
        (
            "## ARCH-DEC-07",
            "| ARCH-DEC-07 | SMQ-04 | DECIDED | Alternative 1 |",
            "Datierter Nutzerentscheid plus geeigneter Evidenznachweis",
        ),
        (
            "## ARCH-DEC-08",
            "| ARCH-DEC-08 | SMQ-05 | DECIDED | Alternative 1 |",
            "Bestehende Manifestangaben beibehalten",
        ),
    ] {
        assert_eq!(
            decision.matches(heading).count(),
            1,
            "{heading} must be recorded once"
        );
        assert!(decision.contains(smq), "missing map {smq}");
        assert!(
            decision.contains(wording),
            "missing user wording for {heading}"
        );
        assert!(decision.contains("2026-11-15"));
    }
    assert!(decision.contains("Alternative 1"));
    assert!(decision.contains("does not ratify"));
    assert!(decision.contains("No evidence URI is recorded"));
    assert!(decision.contains("not approval of any present verification promotion"));
    assert!(decision.contains("IMPLEMENTED_AND_LOCALLY_TESTED"));
    assert!(decision.contains("not ADFR verification"));
    assert!(decision.contains("NOT_VERIFIED"));
    assert!(decision.contains("EVIDENCE_PENDING"));
    assert!(decision.contains("RECONSTRUCTED_DRAFT"));
    assert!(decision.contains("No default precedence"));
    assert!(decision.contains("is not a decision that the historical statuses are equivalent"));
    assert!(decision.contains("XC-11"));
    assert!(decision.contains("stays OPEN"));
    for field in [
        "URI or other stable artifact reference",
        "method",
        "environment",
        "timestamp",
        "exact claim",
        "scope",
        "limitations",
        "dated explicit user approval",
    ] {
        assert!(decision.contains(field), "evidence gate missing {field}");
    }
    for finding in [
        "XC-01",
        "XC-02-PRED",
        "XC-03-MATRIX",
        "XC-05-LOCK",
        "XC-06",
        "XC-07-ROUND",
        "XC-08-IBKR",
        "XC-09-RESET",
        "XC-10-ACTION",
    ] {
        assert!(decision.contains(finding), "decision dropped {finding}");
    }
    assert!(decision.contains("status-word remainder stays OPEN"));
    for banned in [
        "live_trading_authorized: true",
        "canonical_schemas_published: true",
        "verification` = `VERIFIED",
        "the closed lifecycle enum is ratified",
    ] {
        assert!(
            !decision.contains(banned),
            "decision contradicts itself with {banned}"
        );
    }

    let approvals = read_text(&root.join("contracts/status-model/approval-requests.md"));
    for alternative in [
        "1. No winner. Keep every literal in its source.",
        "1. Do not add the enum and do not assign `rejected` or `withdrawn`.",
        "1. Scoped evidence for the named type-shell part only.",
        "1. A dated user decision plus an evidence record naming URI, method, environment, timestamp, scope, and limitations.",
        "1. Leave those files on their manifest placement and `NOT_REVIEWED`.",
        "Nothing here is approved.",
        "Do not self-approve.",
        "## Resolution index (2026-11-15, TRD-ARCH-03A.2)",
    ] {
        assert!(
            approvals.contains(alternative),
            "approval text lost: {alternative}"
        );
    }
    assert!(!approvals.contains("Decision: approved"));

    let model = read_text(&root.join("contracts/status-model/contract-status-model-v1.0.md"));
    assert!(model.contains("PROPOSED_NOT_RATIFIED"));
    assert!(model.contains("no default precedence"));
    assert!(model.contains("does not ratify the closed lifecycle enum"));
    assert!(model.contains("XC-11 status-word disagreement stays OPEN"));
    assert!(model.contains("NON_RUNTIME_NON_WIRE_DRAFT"));

    let draft = load_status_draft(&root);
    assert_eq!(draft.legacy_phrases.len(), 41);
    assert_eq!(
        draft.interpretation_layer_ratification,
        "PROPOSED_NOT_RATIFIED"
    );
    assert!(draft.interpretation_constraints_do_not_ratify_token_sets);
    assert_phrase_blocks(
        &draft,
        "IMPLEMENTED_AND_LOCALLY_TESTED",
        "DEFINITIVE",
        "adfr_row_verified",
    );
    assert_phrase_blocks(
        &draft,
        "OPEN P0",
        "REQUIRES_USER_DECISION",
        "equivalent_status",
    );
    let lifecycle = draft
        .dimensions
        .iter()
        .find(|dimension| dimension.id == "decision_lifecycle")
        .expect("lifecycle");
    for value in [
        "proposed",
        "open",
        "ratified_with_exceptions",
        "superseded",
        "rejected",
        "withdrawn",
    ] {
        assert!(
            lifecycle
                .proposed_values_not_ratified
                .iter()
                .any(|item| item == value),
            "closed enum token disappeared: {value}"
        );
    }
    assert!(lifecycle.proposed_values_are_not_assigned);

    let json_bytes =
        fs::read(root.join("contracts/sources/ADFR-Consolidated-v1.0.json")).expect("ADFR JSON");
    let csv_bytes =
        fs::read(root.join("contracts/sources/ADFR-Consolidated-v1.0.csv")).expect("ADFR CSV");
    assert_eq!(
        sha256_hex(&json_bytes),
        "272a6dabdb411701a25dd708577dbb498c7a7cd2754d83c612e85454a178a1d5"
    );
    assert_eq!(json_bytes.len(), 29880);
    assert_eq!(
        sha256_hex(&csv_bytes),
        "b5577302650e3d497465b93063d149313ec1197a38ddad3a9e7a6f223757ec32"
    );
    assert_eq!(csv_bytes.len(), 13138);
    let manifest = json_value(&root, "contracts/source-manifest.json");
    for entry in manifest["files"].as_array().expect("files") {
        let name = entry["filename"].as_str().expect("filename");
        if name == "ADFR-Consolidated-v1.0.json" {
            assert_eq!(
                entry["sha256"].as_str(),
                Some("272a6dabdb411701a25dd708577dbb498c7a7cd2754d83c612e85454a178a1d5")
            );
        }
        if name == "ADFR-Consolidated-v1.0.csv" {
            assert_eq!(
                entry["sha256"].as_str(),
                Some("b5577302650e3d497465b93063d149313ec1197a38ddad3a9e7a6f223757ec32")
            );
        }
        let placement = entry["placement"].as_str().expect("placement");
        assert!(
            matches!(
                placement,
                "COPIED_VERBATIM" | "REUSED_BYTE_IDENTICAL_EXISTING" | "AMENDED_BY_ARCH_DEC_02"
            ),
            "{name} placement {placement}"
        );
        let source_text = entry["source_text_status"].as_str().expect("source text");
        assert!(source_text != "original" && source_text != "unknown");
        assert_eq!(entry["approved"].as_bool(), Some(false));
        assert_eq!(entry["reviewed"].as_bool(), Some(false));
    }
    let adfr = load_adfr(&root);
    let csv = load_csv(&root);
    assert_eq!(adfr.entries.len(), 78);
    assert_eq!(csv.len(), 78);
    let mut superseded = 0;
    for (entry, row) in adfr.entries.iter().zip(csv.iter()) {
        assert_eq!(entry.id, row[0]);
        assert_eq!(entry.verification, "NOT_VERIFIED");
        if entry.lifecycle == "SUPERSEDED" {
            superseded += 1;
            assert_eq!(entry.id, SUPERSEDED_ID);
            assert_eq!(entry.superseded_by.as_deref(), Some(SUCCESSOR_ID));
        }
    }
    assert_eq!(superseded, 1);
    assert_eq!(
        adfr.entries
            .iter()
            .filter(|entry| entry.lifecycle == "NOT_SUPERSEDED")
            .count(),
        77
    );

    let registry_bytes = fs::read(root.join("contracts/registry/registry.json")).expect("registry");
    assert_eq!(sha256_hex(&registry_bytes), REGISTRY_SHA256);
    assert_eq!(registry_bytes.len(), 44332);
    let registry: Value = serde_json::from_slice(&registry_bytes).expect("registry JSON");
    assert_eq!(registry["registry_status"], "SKELETON_NOT_CANONICAL");
    assert_eq!(registry["canonical_schemas_published"], false);
    assert_eq!(registry["live_trading_authorized"], false);
    assert_eq!(registry["part_b"], "NO-GO");
    assert_eq!(registry["broker_uniqueness_evidence"], "EVIDENCE_PENDING");
    assert!(!root.join("contracts/evidence").exists());
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct OmsProposalIndex {
    document_role: String,
    ratification: String,
    task: String,
    date: String,
    live_trading_authorized: bool,
    canonical_schemas_published: bool,
    part_b: String,
    broker_uniqueness_evidence: String,
    registry_status_required: String,
    order_state_machine_implemented: bool,
    state_transitions_implemented: bool,
    order_closure_vocabulary: String,
    transition_edges: String,
    xc_03_matrix: String,
    cr_b01_precedence_over_det07: String,
    questions: Vec<OmsQuestionStatus>,
    scenario_families: Vec<OmsScenarioStatus>,
    open_findings: Vec<String>,
    files: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct OmsQuestionStatus {
    id: String,
    status: String,
    recommendation_is_approval: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct OmsScenarioStatus {
    id: String,
    approval_status: String,
}

#[test]
fn oms_finality_proposals_stay_unratified_and_registers_stay_closed() {
    let root = repo_root();
    let index_path = root.join("contracts/design-proposals/TRD-ARCH-03B1-proposal-index.json");
    let index_bytes = fs::read(&index_path).expect("OMS proposal index");
    assert!(
        !index_bytes.starts_with(&[0xEF, 0xBB, 0xBF]),
        "proposal index has a BOM"
    );
    let index: OmsProposalIndex =
        serde_json::from_slice(&index_bytes).expect("parse proposal index");
    assert_eq!(index.document_role, "NON_RUNTIME_NON_WIRE_PROPOSAL_INDEX");
    assert_eq!(index.ratification, "NOT_RATIFIED");
    assert_eq!(index.task, "TRD-ARCH-03B.1");
    assert_eq!(index.date, "2026-11-15");
    assert!(!index.live_trading_authorized);
    assert!(!index.canonical_schemas_published);
    assert_eq!(index.part_b, "NO-GO");
    assert_eq!(index.broker_uniqueness_evidence, "EVIDENCE_PENDING");
    assert_eq!(index.registry_status_required, "SKELETON_NOT_CANONICAL");
    assert!(!index.order_state_machine_implemented);
    assert!(!index.state_transitions_implemented);
    assert_eq!(index.order_closure_vocabulary, "NOT_APPROVED");
    assert_eq!(index.transition_edges, "NOT_APPROVED");
    assert_eq!(index.xc_03_matrix, "OPEN");
    assert_eq!(index.cr_b01_precedence_over_det07, "IN_FORCE");

    let expected_questions = [
        "OMSQ-01", "OMSQ-02", "OMSQ-03", "OMSQ-04", "OMSQ-05", "OMSQ-06", "OMSQ-07", "OMSQ-08",
        "OMSQ-09", "OMSQ-10", "OMSQ-11", "OMSQ-12",
    ];
    assert_eq!(index.questions.len(), expected_questions.len());
    for (question, expected) in index.questions.iter().zip(expected_questions) {
        assert_eq!(question.id, expected);
        assert_eq!(question.status, "USER_DECISION_PENDING");
        assert!(!question.recommendation_is_approval);
    }

    let allowed_family_status = ["PROPOSED_REQUIRES_USER_DECISION", "BLOCKED_BY_DEPENDENCY"];
    assert_eq!(index.scenario_families.len(), 16);
    let mut proposed = 0;
    for family in &index.scenario_families {
        assert!(
            allowed_family_status.contains(&family.approval_status.as_str()),
            "{} has {}",
            family.id,
            family.approval_status
        );
        assert_ne!(family.approval_status, "APPROVED");
        if family.approval_status == "PROPOSED_REQUIRES_USER_DECISION" {
            proposed += 1;
        }
    }
    assert!(
        proposed >= 1,
        "create and validate must stay user decisions"
    );
    assert!(
        index
            .scenario_families
            .iter()
            .any(|family| family.id == "create"
                && family.approval_status == "PROPOSED_REQUIRES_USER_DECISION")
    );
    assert!(
        index
            .scenario_families
            .iter()
            .any(|family| family.id == "modify_request_or_confirm"
                && family.approval_status == "BLOCKED_BY_DEPENDENCY")
    );

    for finding in [
        "XC-01",
        "XC-02-PRED",
        "XC-03-MATRIX",
        "XC-05-LOCK",
        "XC-06",
        "XC-07-ROUND",
        "XC-08-IBKR",
        "XC-09-RESET",
        "XC-10-ACTION",
    ] {
        assert!(index.open_findings.iter().any(|item| item == finding));
    }

    let approvals =
        read_text(&root.join("contracts/design-proposals/TRD-ARCH-03B1-approval-requests.md"));
    let coverage = read_text(
        &root.join("contracts/design-proposals/TRD-ARCH-03B1-transition-coverage-draft.md"),
    );
    let vocabulary = read_text(
        &root.join("contracts/design-proposals/TRD-ARCH-03B1-OrderClosure-vocabulary-options.md"),
    );
    let semantics = read_text(
        &root.join("contracts/design-proposals/TRD-ARCH-03B1-OMS-semantics-and-invariants.md"),
    );
    let matrix = read_text(
        &root.join("contracts/design-proposals/TRD-ARCH-03B1-dependency-and-conflict-matrix.md"),
    );
    for text in [&approvals, &coverage, &vocabulary, &semantics, &matrix] {
        assert!(text.contains("NOT_RATIFIED"));
        assert!(text.contains("XC-03-MATRIX"));
        assert!(text.contains("OPEN"));
        assert!(text.contains("NOT AUTHORIZED"));
    }
    for id in expected_questions {
        assert!(approvals.contains(id), "approval request missing {id}");
        assert!(
            approvals.contains(&format!("## {id}")),
            "missing heading {id}"
        );
    }
    assert_eq!(approvals.matches("USER_DECISION_PENDING").count(), 13);
    assert!(approvals.contains("not an approval"));
    assert!(vocabulary.contains("Alternative C"));
    assert!(vocabulary.contains("Non-binding recommendation"));
    assert!(semantics.contains("BINDING_EXISTING"));
    assert!(semantics.contains("PROPOSED"));
    assert!(semantics.contains("DEPENDENCY"));
    assert!(semantics.contains("UNVERIFIED"));
    assert!(coverage.contains("UNSPECIFIED"));
    assert!(coverage.contains("TESTS_NOT_EXECUTED"));
    assert!(coverage.contains("PROPOSED_REQUIRES_USER_DECISION"));
    assert!(coverage.contains("BLOCKED_BY_DEPENDENCY"));
    assert!(matrix.contains("| XC-03-MATRIX | OPEN |"));
    assert!(matrix.contains("ARCH-03B.2"));
    assert!(matrix.contains("ARCH-03B.3"));
    for file in &index.files {
        assert!(root.join(file).is_file(), "missing {file}");
    }

    let json_bytes =
        fs::read(root.join("contracts/sources/ADFR-Consolidated-v1.0.json")).expect("ADFR JSON");
    let csv_bytes =
        fs::read(root.join("contracts/sources/ADFR-Consolidated-v1.0.csv")).expect("ADFR CSV");
    assert_eq!(
        sha256_hex(&json_bytes),
        "272a6dabdb411701a25dd708577dbb498c7a7cd2754d83c612e85454a178a1d5"
    );
    assert_eq!(json_bytes.len(), 29880);
    assert_eq!(
        sha256_hex(&csv_bytes),
        "b5577302650e3d497465b93063d149313ec1197a38ddad3a9e7a6f223757ec32"
    );
    assert_eq!(csv_bytes.len(), 13138);
    let adfr = load_adfr(&root);
    assert_eq!(adfr.entries.len(), 78);
    assert_eq!(adfr.status, "ARCHITECTURE_DRAFT_NOT_IMPLEMENTED");
    assert!(
        adfr.entries
            .iter()
            .all(|entry| entry.verification == "NOT_VERIFIED")
    );
    assert_eq!(
        adfr.entries
            .iter()
            .filter(|entry| entry.lifecycle == "SUPERSEDED")
            .count(),
        1
    );
    let superseded = adfr
        .entries
        .iter()
        .find(|entry| entry.lifecycle == "SUPERSEDED")
        .expect("superseded");
    assert_eq!(superseded.id, SUPERSEDED_ID);
    assert_eq!(superseded.superseded_by.as_deref(), Some(SUCCESSOR_ID));
    assert_eq!(
        adfr.entries
            .iter()
            .filter(|entry| entry.lifecycle == "NOT_SUPERSEDED")
            .count(),
        77
    );

    let manifest_bytes = fs::read(root.join("contracts/source-manifest.json")).expect("manifest");
    let manifest: Value = serde_json::from_slice(&manifest_bytes).expect("manifest JSON");
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
        assert!(entry.get("package_sha256").is_some());
        assert!(entry.get("package_bytes").is_some());
    }

    let registry_bytes = fs::read(root.join("contracts/registry/registry.json")).expect("registry");
    assert_eq!(sha256_hex(&registry_bytes), REGISTRY_SHA256);
    assert_eq!(registry_bytes.len(), 44332);
    let registry: Value = serde_json::from_slice(&registry_bytes).expect("registry JSON");
    assert_eq!(registry["registry_status"], "SKELETON_NOT_CANONICAL");
    assert_eq!(registry["canonical_schemas_published"], false);
    assert_eq!(registry["live_trading_authorized"], false);
    assert_eq!(registry["part_b"], "NO-GO");
    assert_eq!(registry["broker_uniqueness_evidence"], "EVIDENCE_PENDING");
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct OmsPolicyIndex {
    document_role: String,
    task: String,
    date: String,
    record_label: String,
    implementation: String,
    verification: String,
    record_labels_are_not_a_canonical_enum: bool,
    decision_record: String,
    proposal_package_historical_ratification: String,
    proposal_questions_historical_status: String,
    live_trading_authorized: bool,
    canonical_schemas_published: bool,
    canonical_schema_published_by_this_index: bool,
    part_b: String,
    broker_uniqueness_evidence: String,
    registry_status_required: String,
    order_state_machine_implemented: bool,
    state_transitions_implemented: bool,
    order_closure_vocabulary: String,
    order_closure_enum_approved: bool,
    transition_edges: String,
    matrix_version_approved: bool,
    economic_finality_predicate_approved: bool,
    automatic_economic_posting_approved: bool,
    risk_release_approved: bool,
    broker_terminal_acceptance_approved: bool,
    modify_or_replace_dispatch_approved: bool,
    implementation_authorized: bool,
    closes_xc_findings: bool,
    external_evidence_claimed: bool,
    evidence_uri: Option<String>,
    signature_artifact: Option<String>,
    xc_03_matrix: String,
    cr_b01_precedence_over_det07: String,
    omsq05_overrides_omsq08_single_match_posting: bool,
    omsq04_does_not_authorize_risk_release: bool,
    omsq02_is_strategy_only: bool,
    omsq09_accepts_no_terminal_observation: bool,
    questions: Vec<OmsPolicyChoice>,
    open_findings: Vec<String>,
    partial_findings: Vec<OmsPartialFinding>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct OmsPolicyChoice {
    id: String,
    decision_id: String,
    choice: String,
    wording: String,
    scope_code: String,
    policy_only: bool,
    implemented: bool,
    verification: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct OmsPartialFinding {
    finding: String,
    closed_part: String,
    by: String,
    remainder: String,
}

#[test]
fn oms_design_policy_09_through_20_stays_unimplemented() {
    let root = repo_root();
    let expected = [
        (
            "OMSQ-01",
            "ARCH-DEC-09",
            "A",
            "Ein ursprünglicher Abschlussdatensatz, spätere Fakten separat",
            "NO_CLOSURE_WRITER_NO_CORRECTION_RECORD_NO_REOPEN",
        ),
        (
            "OMSQ-02",
            "ARCH-DEC-10",
            "C",
            "Grundfamilie plus unabhängige Finalitätsfelder",
            "STRATEGY_ONLY_NO_TOKENS_NO_ENUM_NO_FIELDS_NO_EDGE",
        ),
        (
            "OMSQ-03",
            "ARCH-DEC-11",
            "A",
            "Keine Schliessung ohne neue autorisierte Aktion und positive Evidenz",
            "NO_UNKNOWN_SEND_CLOSURE_NO_AUTO_RETRY_NO_TAKEOVER",
        ),
        (
            "OMSQ-04",
            "ARCH-DEC-12",
            "A",
            "Alle Fakten behalten; Closure und Risk nur bei ausreichender Evidenz",
            "RETAIN_FACTS_NO_RISK_RELEASE_PREDICATE_NOT_SATISFIED",
        ),
        (
            "OMSQ-05",
            "ARCH-DEC-13",
            "B",
            "Alle wirtschaftlichen Effekte zunächst manuell prüfen",
            "QUARANTINE_ALL_ECONOMIC_EFFECTS_NO_POSTING",
        ),
        (
            "OMSQ-06",
            "ARCH-DEC-14",
            "A",
            "Kein Finalitätsprädikat akzeptieren; Risk Release verboten",
            "NO_PREDICATE_NO_RELEASE_CHECKLIST_B_NOT_ADOPTED",
        ),
        (
            "OMSQ-07",
            "ARCH-DEC-15",
            "A",
            "Vollständige Szenarien, Negativfälle und eigene Version verlangen",
            "ACCEPTANCE_BAR_ONLY_NO_MATRIX_VERSION_NO_EDGE",
        ),
        (
            "OMSQ-08",
            "ARCH-DEC-16",
            "A",
            "Quarantäne und strukturierte autorisierte Entscheidung",
            "QUARANTINE_EXCEPT_IDENTICAL_REPLAY_NO_SINGLE_MATCH_POSTING",
        ),
        (
            "OMSQ-09",
            "ARCH-DEC-17",
            "A",
            "Keine terminale Brokerbeobachtung akzeptieren",
            "NO_TERMINAL_ACCEPTANCE",
        ),
        (
            "OMSQ-10",
            "ARCH-DEC-18",
            "A",
            "Sperrreihenfolge und Isolation bleiben unbestätigter Vorschlag",
            "LOCK_ORDER_AND_ISOLATION_UNADOPTED_NO_BROKER_RESEND",
        ),
        (
            "OMSQ-11",
            "ARCH-DEC-19",
            "A",
            "Separaten transaktionalen Auditdatensatz verlangen",
            "DISTINCT_AUDIT_RECORD_NO_TABLE_NAME",
        ),
        (
            "OMSQ-12",
            "ARCH-DEC-20",
            "A",
            "Modify und implizites Cancel-Replace blockieren",
            "NO_MODIFY_NO_IMPLICIT_CANCEL_REPLACE_NO_DISPATCH",
        ),
    ];

    let index_path = root.join("contracts/design-proposals/TRD-ARCH-03B12-resolution-index.json");
    let index_bytes = fs::read(&index_path).expect("OMS policy index");
    assert!(!index_bytes.starts_with(&[0xEF, 0xBB, 0xBF]));
    let index: OmsPolicyIndex = serde_json::from_slice(&index_bytes).expect("parse policy index");
    assert_eq!(index.document_role, "NON_RUNTIME_NON_WIRE_RESOLUTION_INDEX");
    assert_eq!(index.task, "TRD-ARCH-03B.1.2");
    assert_eq!(index.date, "2026-11-15");
    assert_eq!(index.record_label, "RATIFIED_DESIGN_POLICY_ONLY");
    assert_eq!(index.implementation, "NOT_IMPLEMENTED");
    assert_eq!(index.verification, "NOT_VERIFIED");
    assert!(index.record_labels_are_not_a_canonical_enum);
    assert_eq!(
        index.decision_record,
        "contracts/decisions/ARCH-DEC-09-bis-20-2026-11-15.md"
    );
    assert_eq!(
        index.proposal_package_historical_ratification,
        "NOT_RATIFIED"
    );
    assert_eq!(
        index.proposal_questions_historical_status,
        "USER_DECISION_PENDING"
    );
    assert!(!index.live_trading_authorized);
    assert!(!index.canonical_schemas_published);
    assert!(!index.canonical_schema_published_by_this_index);
    assert_eq!(index.part_b, "NO-GO");
    assert_eq!(index.broker_uniqueness_evidence, "EVIDENCE_PENDING");
    assert_eq!(index.registry_status_required, "SKELETON_NOT_CANONICAL");
    assert!(!index.order_state_machine_implemented);
    assert!(!index.state_transitions_implemented);
    assert_eq!(index.order_closure_vocabulary, "NOT_APPROVED");
    assert!(!index.order_closure_enum_approved);
    assert_eq!(index.transition_edges, "NOT_APPROVED");
    assert!(!index.matrix_version_approved);
    assert!(!index.economic_finality_predicate_approved);
    assert!(!index.automatic_economic_posting_approved);
    assert!(!index.risk_release_approved);
    assert!(!index.broker_terminal_acceptance_approved);
    assert!(!index.modify_or_replace_dispatch_approved);
    assert!(!index.implementation_authorized);
    assert!(!index.closes_xc_findings);
    assert!(!index.external_evidence_claimed);
    assert!(index.evidence_uri.is_none());
    assert!(index.signature_artifact.is_none());
    assert_eq!(index.xc_03_matrix, "OPEN");
    assert_eq!(index.cr_b01_precedence_over_det07, "IN_FORCE");
    assert!(index.omsq05_overrides_omsq08_single_match_posting);
    assert!(index.omsq04_does_not_authorize_risk_release);
    assert!(index.omsq02_is_strategy_only);
    assert!(index.omsq09_accepts_no_terminal_observation);

    assert_eq!(index.questions.len(), expected.len());
    let mut choices = String::new();
    for (question, row) in index.questions.iter().zip(expected) {
        let (id, decision_id, choice, wording, scope_code) = row;
        assert_eq!(question.id, id);
        assert_eq!(question.decision_id, decision_id);
        assert_eq!(question.choice, choice);
        assert_eq!(question.wording, wording);
        assert_eq!(question.scope_code, scope_code);
        assert!(question.policy_only);
        assert!(!question.implemented);
        assert_eq!(question.verification, "NOT_VERIFIED");
        if !choices.is_empty() {
            choices.push(',');
        }
        choices.push_str(choice);
    }
    assert_eq!(choices, "A,C,A,A,B,A,A,A,A,A,A,A");

    let decision_names = fs::read_dir(root.join("contracts/decisions"))
        .expect("decisions")
        .map(|item| {
            item.expect("entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect::<Vec<_>>();
    assert_eq!(
        decision_names
            .iter()
            .filter(|name| name.contains("ARCH-DEC-09"))
            .count(),
        1
    );
    let decision = read_text(&root.join(index.decision_record));
    assert!(decision.contains("2026-11-15"));
    assert!(decision.contains("RATIFIED_DESIGN_POLICY_ONLY"));
    assert!(decision.contains("NOT_IMPLEMENTED"));
    assert!(decision.contains("NOT_VERIFIED"));
    assert!(decision.contains("not a canonical status enum"));
    assert!(decision.contains("No evidence URI is recorded"));
    assert!(decision.contains("none is invented"));
    for (id, decision_id, choice, wording, scope_code) in expected {
        assert_eq!(
            decision.matches(&format!("## {decision_id}")).count(),
            1,
            "{decision_id} must be one heading"
        );
        assert!(decision.contains(&format!("| {decision_id} | {id} | {choice} | {wording} |")));
        assert!(decision.contains(&format!("Scope code: `{scope_code}`")));
        assert!(decision.contains(wording));
    }
    for required in [
        "OMSQ-05 B overrides any appearance of automatic economic booking in OMSQ-08 A.",
        "Identical-payload replay may return a stored result idempotently.",
        "OMSQ-04 A does not authorize risk release.",
        "Adopt the modeling strategy only.",
        "No broker observation is currently accepted as authoritative terminal.",
        "Evidence checklist B in the proposal remains an unratified future candidate.",
        "No current matrix version.",
        "does not authorize cancel during a halt",
        "There is no canonical lock protocol",
        "No authoritative table name.",
        "CR-B01 keeps design precedence",
        "RESOLVED_BY_ARCH_DEC_02",
        "RESOLVED_BY_ARCH_DEC_03",
        "Q01–Q10 verification cells stay OPEN",
        "SOURCE_REVIEW_PENDING",
    ] {
        assert!(decision.contains(required), "decision missing {required}");
    }
    assert!(!decision.contains("enum OrderClosure"));
    assert!(!decision.contains("```rust"));
    assert!(decision.contains("no risk release is enabled"));

    let approvals =
        read_text(&root.join("contracts/design-proposals/TRD-ARCH-03B1-approval-requests.md"));
    let (historical, _resolution) = approvals
        .split_once("## Resolution index (2026-11-15, TRD-ARCH-03B.1.2)")
        .expect("resolution index");
    assert_eq!(historical.matches("USER_DECISION_PENDING").count(), 13);
    assert!(historical.contains("No OMSQ is `DECIDED`"));
    assert!(historical.contains("not an approval"));
    for (id, _, _, _, _) in expected {
        assert!(historical.contains(&format!("## {id}")));
    }

    for finding in [
        "XC-01",
        "XC-02-PRED",
        "XC-03-MATRIX",
        "XC-05-LOCK",
        "XC-06",
        "XC-07-ROUND",
        "XC-08-IBKR",
        "XC-09-RESET",
        "XC-10-ACTION",
    ] {
        assert!(index.open_findings.iter().any(|item| item == finding));
        assert!(decision.contains(finding), "decision missing {finding}");
    }
    assert_eq!(index.partial_findings.len(), 2);
    assert_eq!(index.partial_findings[0].finding, "XC-04");
    assert_eq!(index.partial_findings[0].closed_part, "design_precedence");
    assert_eq!(index.partial_findings[0].by, "RESOLVED_BY_ARCH_DEC_03");
    assert_eq!(index.partial_findings[0].remainder, "OPEN");
    assert_eq!(index.partial_findings[1].finding, "XC-11");
    assert_eq!(
        index.partial_findings[1].closed_part,
        "population_and_supersession"
    );
    assert_eq!(index.partial_findings[1].by, "RESOLVED_BY_ARCH_DEC_02");
    assert_eq!(index.partial_findings[1].remainder, "OPEN");

    let unresolved = read_text(&root.join("contracts/domain/unresolved-decisions.md"));
    for finding in [
        "XC-01 stays OPEN",
        "XC-02-PRED stays OPEN",
        "XC-03-MATRIX stays OPEN",
        "XC-05-LOCK stays OPEN",
        "XC-06 stays OPEN",
        "XC-07-ROUND stays OPEN",
        "XC-08-IBKR stays OPEN",
        "XC-09-RESET stays OPEN",
        "XC-10-ACTION stays OPEN",
        "XC-11 status-word disagreement stays OPEN",
    ] {
        assert!(unresolved.contains(finding), "unresolved dropped {finding}");
    }

    for (relative, hash, bytes) in [
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
            "contracts/design-proposals/TRD-ARCH-03B1-OMS-semantics-and-invariants.md",
            "80148a2ad3f7100dd4c27e3245f21996eb9929806b6b18df2f3a97a4dd7b530e",
            25741,
        ),
        (
            "contracts/design-proposals/TRD-ARCH-03B1-transition-coverage-draft.md",
            "8ab7185c1a066ab15271692d5876448f538ef532ef20f64be9167368525b1b9d",
            18724,
        ),
        (
            "contracts/design-proposals/TRD-ARCH-03B1-OrderClosure-vocabulary-options.md",
            "efcc9e2ced885700f8cd93657a7ba2aa4158725dd9d2612c77398512353780d7",
            16243,
        ),
        (
            "contracts/design-proposals/TRD-ARCH-03B1-proposal-index.json",
            "96dc34c61db04bc0d4eb0576a98f787fddb5632816b903985b1163993afc73cc",
            3654,
        ),
    ] {
        let file_bytes = fs::read(root.join(relative)).expect(relative);
        assert_eq!(file_bytes.len(), bytes, "{relative} byte count");
        assert_eq!(sha256_hex(&file_bytes), hash, "{relative} hash");
    }

    let json_bytes =
        fs::read(root.join("contracts/sources/ADFR-Consolidated-v1.0.json")).expect("ADFR JSON");
    let csv_bytes =
        fs::read(root.join("contracts/sources/ADFR-Consolidated-v1.0.csv")).expect("ADFR CSV");
    assert_eq!(
        sha256_hex(&json_bytes),
        "272a6dabdb411701a25dd708577dbb498c7a7cd2754d83c612e85454a178a1d5"
    );
    assert_eq!(json_bytes.len(), 29880);
    assert_eq!(
        sha256_hex(&csv_bytes),
        "b5577302650e3d497465b93063d149313ec1197a38ddad3a9e7a6f223757ec32"
    );
    assert_eq!(csv_bytes.len(), 13138);
    let adfr = load_adfr(&root);
    let csv = load_csv(&root);
    assert_eq!(adfr.entries.len(), 78);
    assert_eq!(csv.len(), 78);
    assert_eq!(adfr.status, "ARCHITECTURE_DRAFT_NOT_IMPLEMENTED");
    let json_ids: Vec<&str> = adfr.entries.iter().map(|entry| entry.id.as_str()).collect();
    let csv_ids: Vec<&str> = csv.iter().map(|row| row[0].as_str()).collect();
    assert_eq!(json_ids, csv_ids);
    assert!(
        adfr.entries
            .iter()
            .all(|entry| entry.verification == "NOT_VERIFIED")
    );
    assert_eq!(
        adfr.entries
            .iter()
            .filter(|entry| entry.lifecycle == "SUPERSEDED")
            .count(),
        1
    );
    let superseded = adfr
        .entries
        .iter()
        .find(|entry| entry.id == SUPERSEDED_ID)
        .expect("superseded id");
    assert_eq!(superseded.lifecycle, "SUPERSEDED");
    assert_eq!(superseded.superseded_by.as_deref(), Some(SUCCESSOR_ID));
    assert_eq!(superseded.verification, "NOT_VERIFIED");
    assert_eq!(
        adfr.entries
            .iter()
            .filter(|entry| entry.lifecycle == "NOT_SUPERSEDED")
            .count(),
        77
    );

    let manifest_bytes = fs::read(root.join("contracts/source-manifest.json")).expect("manifest");
    let manifest: Value = serde_json::from_slice(&manifest_bytes).expect("manifest JSON");
    let files = manifest["files"].as_array().expect("files");
    assert_eq!(files.len(), 12);
    for entry in files {
        let relative = entry["repository_path"].as_str().expect("path");
        let filename = entry["filename"].as_str().expect("filename");
        let bytes = fs::read(root.join(relative)).expect(relative);
        assert_eq!(bytes.len() as u64, entry["bytes"].as_u64().expect("bytes"));
        assert_eq!(
            sha256_hex(&bytes),
            entry["sha256"].as_str().expect("sha256")
        );
        let package_hash = entry["package_sha256"].as_str().expect("package hash");
        let package_bytes = entry["package_bytes"].as_u64().expect("package bytes");
        let package = PACKAGE_BYTES
            .iter()
            .find(|row| row.0 == filename)
            .unwrap_or_else(|| panic!("missing package row {filename}"));
        assert_eq!(package_hash, package.1);
        assert_eq!(package_bytes, package.2);
        assert!(!entry["reviewed"].as_bool().expect("reviewed"));
        assert!(!entry["approved"].as_bool().expect("approved"));
        if filename.starts_with("DET-01") || filename.starts_with("DET-02") {
            assert!(entry["reconstructed_draft"].as_bool().expect("draft"));
            assert_eq!(entry["source_text_status"], "SOURCE_REVIEW_PENDING");
        }
    }

    let registry_bytes = fs::read(root.join("contracts/registry/registry.json")).expect("registry");
    assert_eq!(sha256_hex(&registry_bytes), REGISTRY_SHA256);
    assert_eq!(registry_bytes.len(), 44332);
    let registry: Value = serde_json::from_slice(&registry_bytes).expect("registry JSON");
    assert_eq!(registry["registry_status"], "SKELETON_NOT_CANONICAL");
    assert_eq!(registry["canonical_schemas_published"], false);
    assert_eq!(registry["live_trading_authorized"], false);
    assert_eq!(registry["part_b"], "NO-GO");
    assert_eq!(registry["broker_uniqueness_evidence"], "EVIDENCE_PENDING");

    let domain_lib = read_text(&root.join("crates/trading-domain/src/lib.rs"));
    assert!(domain_lib.contains("pub const ORDER_STATE_MACHINE_IMPLEMENTED: bool = false"));
    assert!(domain_lib.contains("pub const STATE_TRANSITIONS_IMPLEMENTED: bool = false"));
    assert!(!root.join("contracts/sql").exists());
    assert!(!root.join("contracts/events").exists());
}

fn load_status_draft(root: &Path) -> StatusDraft {
    let path = root.join("contracts/status-model/status-dimensions-draft-v0.json");
    let bytes = fs::read(&path).expect("status draft");
    assert!(
        !bytes.windows(3).any(|window| window == [0xEF, 0xBB, 0xBF]),
        "status draft has a BOM"
    );
    let text = String::from_utf8(bytes).expect("status draft is UTF-8");
    assert!(
        !text.contains("Kanonische owner-spezifische"),
        "status draft copied an ADFR decision body"
    );
    serde_json::from_str(&text).expect("parse status draft")
}

fn assert_phrase_blocks(draft: &StatusDraft, phrase: &str, status: &str, blocked: &str) {
    let matches = draft
        .legacy_phrases
        .iter()
        .filter(|item| item.phrase == phrase && item.mapping_status == status)
        .collect::<Vec<_>>();
    assert!(!matches.is_empty(), "missing {phrase} as {status}");
    assert!(
        matches
            .iter()
            .any(|item| item.does_not_imply.iter().any(|value| value == blocked)),
        "{phrase} does not block {blocked}"
    );
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn load_adfr(root: &Path) -> AdfrFile {
    let bytes =
        fs::read(root.join("contracts/sources/ADFR-Consolidated-v1.0.json")).expect("ADFR JSON");
    assert!(
        !bytes.starts_with(&[0xEF, 0xBB, 0xBF]),
        "ADFR JSON has a BOM"
    );
    serde_json::from_slice(&bytes).expect("parse ADFR JSON")
}

fn load_csv(root: &Path) -> Vec<Vec<String>> {
    let bytes =
        fs::read(root.join("contracts/sources/ADFR-Consolidated-v1.0.csv")).expect("ADFR CSV");
    assert!(
        !bytes.starts_with(&[0xEF, 0xBB, 0xBF]),
        "ADFR CSV has a BOM"
    );
    let text = String::from_utf8(bytes).expect("ADFR CSV is UTF-8");
    let mut rows = parse_csv(&text);
    assert!(!rows.is_empty(), "ADFR CSV is empty");
    let header = rows.remove(0);
    assert_eq!(
        header,
        vec![
            "id",
            "decision",
            "source",
            "status",
            "basis",
            "verification",
            "lifecycle",
            "supersedes",
            "superseded_by",
        ]
    );
    rows
}

fn json_value(root: &Path, relative: &str) -> Value {
    let bytes = fs::read(root.join(relative)).unwrap_or_else(|error| {
        panic!("read {relative}: {error}");
    });
    serde_json::from_slice(&bytes).unwrap_or_else(|error| {
        panic!("parse {relative}: {error}");
    })
}

fn read_text(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| {
        panic!("read {}: {error}", path.display());
    })
}

fn parse_csv(text: &str) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    let mut row = Vec::new();
    let mut field = String::new();
    let mut quoted = false;
    let mut chars = text.chars().peekable();
    while let Some(ch) = chars.next() {
        if quoted {
            if ch == '"' {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    field.push('"');
                } else {
                    quoted = false;
                }
            } else {
                field.push(ch);
            }
            continue;
        }
        match ch {
            '"' => {
                assert!(field.is_empty(), "quote inside an unquoted CSV field");
                quoted = true;
            }
            ',' => row.push(std::mem::take(&mut field)),
            '\n' => {
                row.push(std::mem::take(&mut field));
                rows.push(std::mem::take(&mut row));
            }
            '\r' => {}
            other => field.push(other),
        }
    }
    assert!(!quoted, "unterminated CSV quote");
    if !field.is_empty() || !row.is_empty() {
        row.push(field);
        rows.push(row);
    }
    rows
}

fn sha256_hex(bytes: &[u8]) -> String {
    hex_encode(Sha256::digest(bytes))
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RiskProposalIndex {
    document_role: String,
    ratification: String,
    task: String,
    date: String,
    not_a_schema: bool,
    not_registry_json: bool,
    risk_release_approved: bool,
    economic_posting_approved: bool,
    economic_finality_predicate_approved: bool,
    live_trading_authorized: bool,
    canonical_schemas_published: bool,
    canonical_schema_published_by_this_index: bool,
    part_b: String,
    broker_uniqueness_evidence: String,
    registry_status_required: String,
    order_state_machine_implemented: bool,
    state_transitions_implemented: bool,
    order_closure_enum_approved: bool,
    implementation_authorized: bool,
    closes_xc_findings: bool,
    omsq05_b_quarantine_remains_binding: bool,
    omsq06_a_no_predicate_no_release_remains_binding: bool,
    preference_is_approval: bool,
    non_binding_preferred_approach: String,
    xc_02_pred: String,
    xc_03_matrix: String,
    questions: Vec<OmsQuestionStatus>,
    reservation_approaches: Vec<RiskApproachStatus>,
    coverage_scenarios: Vec<RiskCoverageStatus>,
    open_findings: Vec<String>,
    partial_findings: Vec<OmsPartialFinding>,
    files: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RiskApproachStatus {
    id: String,
    status: String,
    recommendation_is_approval: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RiskCoverageStatus {
    id: String,
    record_evidence: String,
    book_economics: String,
    release_budget: String,
    test_oracle: String,
}

#[test]
fn risk_reservation_proposals_stay_unratified_and_release_stays_forbidden() {
    let root = repo_root();
    let index_path = root.join("contracts/design-proposals/TRD-ARCH-03B2-proposal-index.json");
    let index_bytes = fs::read(&index_path).expect("risk proposal index");
    assert!(
        !index_bytes.starts_with(&[0xEF, 0xBB, 0xBF]),
        "risk proposal index has a BOM"
    );
    let index: RiskProposalIndex =
        serde_json::from_slice(&index_bytes).expect("parse risk proposal index");
    assert_eq!(index.document_role, "NON_RUNTIME_NON_WIRE_PROPOSAL_INDEX");
    assert_eq!(index.ratification, "NOT_RATIFIED");
    assert_eq!(index.task, "TRD-ARCH-03B.2");
    assert_eq!(index.date, "2026-11-15");
    assert!(index.not_a_schema);
    assert!(index.not_registry_json);
    assert!(!index.risk_release_approved);
    assert!(!index.economic_posting_approved);
    assert!(!index.economic_finality_predicate_approved);
    assert!(!index.live_trading_authorized);
    assert!(!index.canonical_schemas_published);
    assert!(!index.canonical_schema_published_by_this_index);
    assert_eq!(index.part_b, "NO-GO");
    assert_eq!(index.broker_uniqueness_evidence, "EVIDENCE_PENDING");
    assert_eq!(index.registry_status_required, "SKELETON_NOT_CANONICAL");
    assert!(!index.order_state_machine_implemented);
    assert!(!index.state_transitions_implemented);
    assert!(!index.order_closure_enum_approved);
    assert!(!index.implementation_authorized);
    assert!(!index.closes_xc_findings);
    assert!(index.omsq05_b_quarantine_remains_binding);
    assert!(index.omsq06_a_no_predicate_no_release_remains_binding);
    assert!(!index.preference_is_approval);
    assert_eq!(
        index.non_binding_preferred_approach,
        "hold_until_evidenced_finality"
    );
    assert_eq!(index.xc_02_pred, "OPEN");
    assert_eq!(index.xc_03_matrix, "OPEN");

    let expected_questions = [
        "RISKQ-01", "RISKQ-02", "RISKQ-03", "RISKQ-04", "RISKQ-05", "RISKQ-06", "RISKQ-07",
        "RISKQ-08", "RISKQ-09", "RISKQ-10", "RISKQ-11", "RISKQ-12", "RISKQ-13", "RISKQ-14",
    ];
    assert_eq!(index.questions.len(), expected_questions.len());
    for (question, expected) in index.questions.iter().zip(expected_questions) {
        assert_eq!(question.id, expected);
        assert_eq!(question.status, "USER_DECISION_PENDING");
        assert!(!question.recommendation_is_approval);
    }

    let expected_approaches = [
        "hold_until_evidenced_finality",
        "reservation_to_exposure_with_residual",
        "staged_allocation_subject_to_proofs",
    ];
    assert_eq!(
        index.reservation_approaches.len(),
        expected_approaches.len()
    );
    for (approach, expected) in index.reservation_approaches.iter().zip(expected_approaches) {
        assert_eq!(approach.id, expected);
        assert_eq!(approach.status, "COMPARISON_ONLY");
        assert!(!approach.recommendation_is_approval);
    }

    let expected_scenarios = [
        "initial_reserve",
        "rejected_pre_dispatch",
        "may_have_sent",
        "ack_without_fills",
        "partial_fill",
        "complete_fill",
        "cancel_requested",
        "broker_cancel_observation",
        "cancel_fill_race",
        "disconnect_incomplete_snapshot",
        "duplicate_fill",
        "late_fill",
        "bust",
        "correction",
        "insufficient_budget",
        "stale_price_or_fx",
        "cross_scope_contention",
        "cas_conflict",
        "restart_reconciliation",
        "manual_adjudication",
        "emergency_halt",
    ];
    assert_eq!(index.coverage_scenarios.len(), expected_scenarios.len());
    for (scenario, expected) in index.coverage_scenarios.iter().zip(expected_scenarios) {
        assert_eq!(scenario.id, expected);
        assert_eq!(scenario.record_evidence, "RETAIN");
        assert_eq!(scenario.book_economics, "PROHIBITED");
        assert_eq!(scenario.release_budget, "PROHIBITED");
        assert_eq!(scenario.test_oracle, "NOT_EXECUTED");
    }

    for finding in [
        "XC-01",
        "XC-02-PRED",
        "XC-03-MATRIX",
        "XC-05-LOCK",
        "XC-06",
        "XC-07-ROUND",
        "XC-08-IBKR",
        "XC-09-RESET",
        "XC-10-ACTION",
    ] {
        assert!(
            index.open_findings.iter().any(|item| item == finding),
            "missing {finding}"
        );
    }
    assert_eq!(index.partial_findings.len(), 2);
    assert_eq!(index.partial_findings[0].finding, "XC-04");
    assert_eq!(index.partial_findings[0].by, "RESOLVED_BY_ARCH_DEC_03");
    assert_eq!(index.partial_findings[0].remainder, "OPEN");
    assert_eq!(index.partial_findings[1].finding, "XC-11");
    assert_eq!(index.partial_findings[1].by, "RESOLVED_BY_ARCH_DEC_02");
    assert_eq!(index.partial_findings[1].remainder, "OPEN");

    let approval_bytes =
        fs::read(root.join("contracts/design-proposals/TRD-ARCH-03B2-approval-requests.md"))
            .expect("risk approval requests");
    const RISK_REQUEST_HISTORICAL_BYTES: usize = 39350;
    assert!(approval_bytes.len() > RISK_REQUEST_HISTORICAL_BYTES);
    assert_eq!(
        sha256_hex(&approval_bytes[..RISK_REQUEST_HISTORICAL_BYTES]),
        "2d32dd7381a83745a9477b5b21905872fdbf45985a65fe2a810f6b4e0d4b4e71"
    );
    assert!(
        approval_bytes[RISK_REQUEST_HISTORICAL_BYTES..]
            .starts_with(b"\n## Resolution index (2026-11-15, TRD-ARCH-03B.2.2)\n")
    );
    let approvals = String::from_utf8(approval_bytes).expect("risk approvals are UTF-8");
    let semantics = read_text(
        &root.join("contracts/design-proposals/TRD-ARCH-03B2-risk-semantics-and-invariants.md"),
    );
    let coverage = read_text(&root.join(
        "contracts/design-proposals/TRD-ARCH-03B2-economic-finality-and-release-coverage-draft.md",
    ));
    let options = read_text(
        &root
            .join("contracts/design-proposals/TRD-ARCH-03B2-reservation-and-correction-options.md"),
    );
    let matrix = read_text(
        &root.join("contracts/design-proposals/TRD-ARCH-03B2-dependency-and-conflict-matrix.md"),
    );
    for text in [&approvals, &semantics, &coverage, &options, &matrix] {
        assert!(text.contains("NOT_RATIFIED"));
        assert!(text.contains("NOT AUTHORIZED"));
        assert!(text.contains("XC-02-PRED"));
    }
    for id in expected_questions {
        assert!(
            approvals.contains(&format!("## {id}")),
            "missing heading {id}"
        );
        assert!(approvals.contains("USER_DECISION_PENDING"));
    }
    assert!(approvals.contains("CONFLICTS_WITH_BINDING_POLICY"));
    assert!(approvals.contains("Would not authorize"));
    assert!(approvals.contains("not an approval"));
    assert!(semantics.contains("BINDING_EXISTING"));
    assert!(semantics.contains("RECONSTRUCTED_BASELINE_WITH_EXCEPTIONS"));
    assert!(semantics.contains("REFERENCE_PROPOSAL"));
    assert!(semantics.contains("EXTERNAL_EVIDENCE_PENDING"));
    assert!(semantics.contains("NEW_PROPOSAL"));
    assert!(semantics.contains("USER_DECISION_PENDING"));
    assert!(coverage.contains("NO EXECUTABLE ECONOMIC-FINALITY PREDICATE"));
    assert!(coverage.contains("RISK RELEASE FORBIDDEN"));
    assert!(coverage.contains("NOT_EXECUTED"));
    assert!(coverage.contains("PROHIBITED"));
    for id in expected_scenarios {
        assert!(
            coverage.contains(&format!("`{id}`")),
            "coverage missing {id}"
        );
    }
    for id in expected_approaches {
        assert!(options.contains(&format!("`{id}`")), "options missing {id}");
    }
    assert!(options.contains("OMSQ-05"));
    assert!(options.contains("OMSQ-06"));
    assert!(matrix.contains("| XC-02-PRED | OPEN |"));
    assert!(matrix.contains("ARCH-03B.3"));
    assert!(matrix.contains("XC-03-MATRIX"));
    for file in &index.files {
        assert!(root.join(file).is_file(), "missing {file}");
    }

    let unchanged = [
        (
            "contracts/source-manifest.json",
            "82b205d27172ad234d46cfa2a59f0f4785128dd7a6282dea1aab32b412fc2808",
            17945_usize,
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
            "contracts/registry/registry.json",
            "7abed2af5363726403175c6524bd7a4d05d36b8735ad48442fc26941f8d98197",
            44332,
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
            "contracts/design-proposals/TRD-ARCH-03B1-proposal-index.json",
            "96dc34c61db04bc0d4eb0576a98f787fddb5632816b903985b1163993afc73cc",
            3654,
        ),
        (
            "contracts/design-proposals/TRD-ARCH-03B12-resolution-index.json",
            "e4bd482d2b8b971612e241e069009e86810bfde3cf499784725e6d3b5019ca00",
            6179,
        ),
        (
            "contracts/design-proposals/TRD-ARCH-03B1-OMS-semantics-and-invariants.md",
            "80148a2ad3f7100dd4c27e3245f21996eb9929806b6b18df2f3a97a4dd7b530e",
            25741,
        ),
        (
            "contracts/design-proposals/TRD-ARCH-03B1-transition-coverage-draft.md",
            "8ab7185c1a066ab15271692d5876448f538ef532ef20f64be9167368525b1b9d",
            18724,
        ),
        (
            "contracts/design-proposals/TRD-ARCH-03B1-OrderClosure-vocabulary-options.md",
            "efcc9e2ced885700f8cd93657a7ba2aa4158725dd9d2612c77398512353780d7",
            16243,
        ),
        (
            "contracts/design-proposals/TRD-ARCH-03B1-approval-requests.md",
            "3412b82299b8d31ea6a56fe3ec1b219830690633f35d6ea9c1a3fefde5b72011",
            36374,
        ),
        (
            "contracts/design-proposals/TRD-ARCH-03B1-dependency-and-conflict-matrix.md",
            "94be3ca9163b36f32cbe96e2c6cf7042e61633d625e8061f8c15ff0469bda706",
            9802,
        ),
        (
            "contracts/design-proposals/TRD-ARCH-03B2-proposal-index.json",
            "6520b2d64ebcdd00793872ac97a618c9c635e0283d3cf479f7926fc83dac62e6",
            7035,
        ),
        (
            "contracts/design-proposals/TRD-ARCH-03B2-risk-semantics-and-invariants.md",
            "4da327a9126e1a0bfc8eb296176c2b0deb1c89949da6850091cd3bb918e0cfdc",
            26145,
        ),
        (
            "contracts/design-proposals/TRD-ARCH-03B2-economic-finality-and-release-coverage-draft.md",
            "067d2f0f9b743d9f86706a5c317f6ad4f22f17b7ec5f94ee4bd0a7c5d4f7cb83",
            20069,
        ),
        (
            "contracts/design-proposals/TRD-ARCH-03B2-reservation-and-correction-options.md",
            "eeb3e631f843cd1a204583a8d293fba696f94fc4d0667ccca70b5ab1fc58e925",
            13718,
        ),
        (
            "contracts/design-proposals/TRD-ARCH-03B2-dependency-and-conflict-matrix.md",
            "5c67b0aba6340a2a2c8e849e3c5075186d91be9f88b9356c8816331ed81f3473",
            9310,
        ),
    ];
    for (relative, expected_hash, expected_len) in unchanged {
        let bytes = fs::read(root.join(relative)).expect(relative);
        assert_eq!(bytes.len(), expected_len, "{relative}");
        assert_eq!(sha256_hex(&bytes), expected_hash, "{relative}");
    }

    let registry: Value = serde_json::from_slice(
        &fs::read(root.join("contracts/registry/registry.json")).expect("registry"),
    )
    .expect("registry JSON");
    assert_eq!(registry["registry_status"], "SKELETON_NOT_CANONICAL");
    assert_eq!(registry["canonical_schemas_published"], false);
    assert_eq!(registry["live_trading_authorized"], false);
    assert_eq!(registry["part_b"], "NO-GO");
    assert_eq!(registry["broker_uniqueness_evidence"], "EVIDENCE_PENDING");

    let adfr = load_adfr(&root);
    assert_eq!(adfr.entries.len(), 78);
    assert!(
        adfr.entries
            .iter()
            .all(|entry| entry.verification == "NOT_VERIFIED")
    );
    assert_eq!(
        adfr.entries
            .iter()
            .filter(|entry| entry.lifecycle == "SUPERSEDED")
            .count(),
        1
    );

    const {
        assert!(!trading_domain::ORDER_STATE_MACHINE_IMPLEMENTED);
        assert!(!trading_domain::STATE_TRANSITIONS_IMPLEMENTED);
        assert!(!trading_domain::FINANCIAL_ROUNDING_IMPLEMENTED);
        assert!(!trading_domain::BROKER_DISPATCH_IMPLEMENTED);
        assert!(!trading_domain::RISK_EVALUATION_IMPLEMENTED);
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RiskPolicyIndex {
    document_role: String,
    task: String,
    date: String,
    record_label: String,
    implementation: String,
    verification: String,
    record_labels_are_not_a_canonical_enum: bool,
    decision_record: String,
    source_approval_document: String,
    instruction_archive: String,
    historical_proposal_index: String,
    proposal_package_historical_ratification: String,
    proposal_questions_historical_status: String,
    not_a_schema: bool,
    not_registry_json: bool,
    not_the_historical_proposal_index: bool,
    implementation_authorized: bool,
    risk_release_approved: bool,
    economic_posting_approved: bool,
    economic_finality_predicate_approved: bool,
    oms_transitions_approved: bool,
    broker_dispatch_approved: bool,
    canonical_schemas_published: bool,
    canonical_schema_published_by_this_index: bool,
    live_trading_authorized: bool,
    closes_xc_findings: bool,
    part_b: String,
    broker_uniqueness_evidence: String,
    registry_status_required: String,
    order_state_machine_implemented: bool,
    state_transitions_implemented: bool,
    risk_evaluation_implemented: bool,
    financial_rounding_implemented: bool,
    order_closure_enum_approved: bool,
    reservation_writer_approved: bool,
    lock_order_adopted: bool,
    isolation_level_adopted: bool,
    rounding_mode_adopted: bool,
    fx_conversion_approved: bool,
    policy_migration_approved: bool,
    tests_executed: bool,
    external_evidence_claimed: bool,
    evidence_uri: Option<String>,
    signature_artifact: Option<String>,
    xc_02_pred: String,
    xc_03_matrix: String,
    arch_dec_13_quarantine_remains_binding: bool,
    arch_dec_14_no_predicate_no_release_remains_binding: bool,
    riskq01_b_is_conjunction_shape_only: bool,
    riskq02_a_is_identity_and_atomicity_only: bool,
    riskq04_05_06_prohibit_convert_shrink_release: bool,
    riskq08_09_define_no_predicate: bool,
    riskq10_b_is_citation_only: bool,
    riskq11_b_does_not_mark_tests_executed: bool,
    riskq12_a_does_not_resolve_xc_06: bool,
    riskq13_a_does_not_supersede_arch_dec_13: bool,
    riskq14_b_does_not_authorize_migration_or_resubmit: bool,
    cr_b01_precedence_over_det07: String,
    depends_on: Vec<String>,
    questions: Vec<RiskPolicyChoice>,
    open_findings: Vec<String>,
    partial_findings: Vec<OmsPartialFinding>,
    principle_only_findings: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RiskPolicyChoice {
    id: String,
    decision_id: String,
    choice: String,
    wording: String,
    source_anchor: String,
    ratification_date: String,
    scope_code: String,
    depends_on: Vec<String>,
    policy_only: bool,
    implemented: bool,
    verification: String,
}

#[test]
fn risk_design_policy_21_through_34_stays_unimplemented() {
    let root = repo_root();
    let expected = [
        (
            "RISKQ-01",
            "ARCH-DEC-21",
            "B",
            "Zusätzlich harte Ablehnungen als Konjunktionsregel festlegen.",
            "## RISKQ-01 — Hierarchical budget shape",
            "CONJUNCTION_SHAPE_ONLY_NO_ARITHMETIC",
        ),
        (
            "RISKQ-02",
            "ARCH-DEC-22",
            "A",
            "Eine Reservierungs-ID, Idempotenz und atomare Multi-Scope-Reservierung.",
            "## RISKQ-02 — Reservation ownership and contention",
            "ONE_ID_ATOMIC_NO_WRITER_NO_LOCK",
        ),
        (
            "RISKQ-03",
            "ARCH-DEC-23",
            "A",
            "Keine Reservierung bei fehlenden oder veralteten Daten.",
            "## RISKQ-03 — Inputs required before a reservation",
            "NO_RESERVATION_ON_MISSING_OR_STALE_INPUT",
        ),
        (
            "RISKQ-04",
            "ARCH-DEC-24",
            "A",
            "Fill erfassen; gesamte Reservierung halten; keine Umbuchung.",
            "## RISKQ-04 — Partial fill, without posting",
            "RETAIN_FILL_HOLD_ALL_NO_CONVERSION",
        ),
        (
            "RISKQ-05",
            "ARCH-DEC-25",
            "A",
            "Restverpflichtung bleibt unverändert.",
            "## RISKQ-05 — Cancel facts and residual obligation",
            "RESIDUAL_UNCHANGED_NO_CANCEL_DISPATCH",
        ),
        (
            "RISKQ-06",
            "ARCH-DEC-26",
            "A",
            "Alle Fakten behalten und wirtschaftliche Effekte quarantänisieren.",
            "## RISKQ-06 — Late fill, duplicate, bust, and correction",
            "RETAIN_QUARANTINE_NO_COMPENSATING_LINE",
        ),
        (
            "RISKQ-07",
            "ARCH-DEC-27",
            "A",
            "Neue risikosteigernde Aktionen blockieren; keine Broker-Aktion.",
            "## RISKQ-07 — Insufficient budget, halt, and reduction",
            "DENY_NEW_RISK_NO_BROKER_ACTION",
        ),
        (
            "RISKQ-08",
            "ARCH-DEC-28",
            "A",
            "Keine Nachweisliste als verbindlich übernehmen.",
            "## RISKQ-08 — Future evidence bar, under ARCH-DEC-14",
            "NO_EVIDENCE_LIST_NO_PREDICATE",
        ),
        (
            "RISKQ-09",
            "ARCH-DEC-29",
            "A",
            "Keinen Freigabenachweis als ausreichend anerkennen.",
            "## RISKQ-09 — Future release proofs",
            "NO_RELEASE_PROOF_ACCEPTED",
        ),
        (
            "RISKQ-10",
            "ARCH-DEC-30",
            "B",
            "Künftige Bewertungs- und Policy-Referenzen versionierbar machen.",
            "## RISKQ-10 — Valuation, FX, and rounding governance",
            "CITATION_OBLIGATIONS_ONLY_NO_ROUNDING",
        ),
        (
            "RISKQ-11",
            "ARCH-DEC-31",
            "B",
            "Zusätzlich verpflichtende zukünftige Concurrency- und Crash-Tests.",
            "## RISKQ-11 — Isolation, locks, CAS, and retries",
            "FUTURE_TESTS_NOT_EXECUTED_NO_LOCK",
        ),
        (
            "RISKQ-12",
            "ARCH-DEC-32",
            "A",
            "Separaten transaktionalen Auditdatensatz verlangen, ohne Tabellennamen.",
            "## RISKQ-12 — Audit and reconciliation evidence",
            "DISTINCT_AUDIT_RECORD_NO_TABLE_NAME",
        ),
        (
            "RISKQ-13",
            "ARCH-DEC-33",
            "A",
            "Keine Rollen oder Freigabewege einführen; Quarantäne bleibt.",
            "## RISKQ-13 — Manual adjudication",
            "NO_ROLES_QUARANTINE_STANDS",
        ),
        (
            "RISKQ-14",
            "ARCH-DEC-34",
            "B",
            "Zusätzlich unveränderliche Policy-Versionen und negative Tests verlangen.",
            "## RISKQ-14 — Recovery, replay, and policy versions",
            "IMMUTABLE_VERSION_TESTS_NOT_EXECUTED",
        ),
    ];

    let proposal_path = root.join("contracts/design-proposals/TRD-ARCH-03B2-proposal-index.json");
    let proposal_bytes = fs::read(&proposal_path).expect("historical risk proposal index");
    assert_eq!(proposal_bytes.len(), 7035);
    assert_eq!(
        sha256_hex(&proposal_bytes),
        "6520b2d64ebcdd00793872ac97a618c9c635e0283d3cf479f7926fc83dac62e6"
    );
    let proposal: RiskProposalIndex =
        serde_json::from_slice(&proposal_bytes).expect("parse historical risk proposal index");
    assert_eq!(
        proposal.document_role,
        "NON_RUNTIME_NON_WIRE_PROPOSAL_INDEX"
    );
    assert_eq!(proposal.ratification, "NOT_RATIFIED");
    assert_eq!(proposal.task, "TRD-ARCH-03B.2");
    assert!(!proposal.risk_release_approved);
    assert!(!proposal.economic_posting_approved);
    assert!(!proposal.economic_finality_predicate_approved);
    assert!(!proposal.implementation_authorized);
    assert!(!proposal.closes_xc_findings);
    assert!(!proposal.live_trading_authorized);
    assert!(!proposal.canonical_schemas_published);
    let proposal_text = String::from_utf8(proposal_bytes).expect("proposal index utf-8");
    assert!(!proposal_text.contains("ARCH-DEC-21"));
    assert!(!proposal_text.contains("\"decision_id\""));

    let index_path = root.join("contracts/design-proposals/TRD-ARCH-03B22-resolution-index.json");
    let index_bytes = fs::read(&index_path).expect("risk policy index");
    assert!(!index_bytes.starts_with(&[0xEF, 0xBB, 0xBF]));
    let index: RiskPolicyIndex =
        serde_json::from_slice(&index_bytes).expect("parse risk policy index");
    assert_eq!(index.document_role, "NON_RUNTIME_NON_WIRE_RESOLUTION_INDEX");
    assert_ne!(index.document_role, proposal.document_role);
    assert_eq!(index.task, "TRD-ARCH-03B.2.2");
    assert_eq!(index.date, "2026-11-15");
    assert_eq!(index.record_label, "RATIFIED_DESIGN_POLICY_ONLY");
    assert_eq!(index.implementation, "NOT_IMPLEMENTED");
    assert_eq!(index.verification, "NOT_VERIFIED");
    assert!(index.record_labels_are_not_a_canonical_enum);
    assert_eq!(
        index.decision_record,
        "contracts/decisions/ARCH-DEC-21-bis-34-2026-11-15.md"
    );
    assert_eq!(index.source_approval_document, index.decision_record);
    assert_eq!(
        index.instruction_archive,
        "docs/cursor-prompts/trading/TRD-ARCH-03B2.2-Ratify-Risk-Design-Decisions.md"
    );
    assert_eq!(
        index.historical_proposal_index,
        "contracts/design-proposals/TRD-ARCH-03B2-proposal-index.json"
    );
    assert_eq!(
        index.proposal_package_historical_ratification,
        "NOT_RATIFIED"
    );
    assert_eq!(
        index.proposal_questions_historical_status,
        "USER_DECISION_PENDING"
    );
    assert!(index.not_a_schema);
    assert!(index.not_registry_json);
    assert!(index.not_the_historical_proposal_index);
    assert!(!index.implementation_authorized);
    assert!(!index.risk_release_approved);
    assert!(!index.economic_posting_approved);
    assert!(!index.economic_finality_predicate_approved);
    assert!(!index.oms_transitions_approved);
    assert!(!index.broker_dispatch_approved);
    assert!(!index.canonical_schemas_published);
    assert!(!index.canonical_schema_published_by_this_index);
    assert!(!index.live_trading_authorized);
    assert!(!index.closes_xc_findings);
    assert_eq!(index.part_b, "NO-GO");
    assert_eq!(index.broker_uniqueness_evidence, "EVIDENCE_PENDING");
    assert_eq!(index.registry_status_required, "SKELETON_NOT_CANONICAL");
    assert!(!index.order_state_machine_implemented);
    assert!(!index.state_transitions_implemented);
    assert!(!index.risk_evaluation_implemented);
    assert!(!index.financial_rounding_implemented);
    assert!(!index.order_closure_enum_approved);
    assert!(!index.reservation_writer_approved);
    assert!(!index.lock_order_adopted);
    assert!(!index.isolation_level_adopted);
    assert!(!index.rounding_mode_adopted);
    assert!(!index.fx_conversion_approved);
    assert!(!index.policy_migration_approved);
    assert!(!index.tests_executed);
    assert!(!index.external_evidence_claimed);
    assert!(index.evidence_uri.is_none());
    assert!(index.signature_artifact.is_none());
    assert_eq!(index.xc_02_pred, "OPEN");
    assert_eq!(index.xc_03_matrix, "OPEN");
    assert!(index.arch_dec_13_quarantine_remains_binding);
    assert!(index.arch_dec_14_no_predicate_no_release_remains_binding);
    assert!(index.riskq01_b_is_conjunction_shape_only);
    assert!(index.riskq02_a_is_identity_and_atomicity_only);
    assert!(index.riskq04_05_06_prohibit_convert_shrink_release);
    assert!(index.riskq08_09_define_no_predicate);
    assert!(index.riskq10_b_is_citation_only);
    assert!(index.riskq11_b_does_not_mark_tests_executed);
    assert!(index.riskq12_a_does_not_resolve_xc_06);
    assert!(index.riskq13_a_does_not_supersede_arch_dec_13);
    assert!(index.riskq14_b_does_not_authorize_migration_or_resubmit);
    assert_eq!(index.cr_b01_precedence_over_det07, "IN_FORCE");
    for prior in [
        "ARCH-DEC-01",
        "ARCH-DEC-03",
        "ARCH-DEC-13",
        "ARCH-DEC-14",
        "ARCH-DEC-18",
        "ARCH-DEC-19",
    ] {
        assert!(index.depends_on.iter().any(|item| item == prior));
    }

    assert_eq!(index.questions.len(), expected.len());
    let mut choices = String::new();
    let mut seen_questions = BTreeSet::new();
    let mut seen_decisions = BTreeSet::new();
    for (question, row) in index.questions.iter().zip(expected) {
        let (id, decision_id, choice, wording, source_anchor, scope_code) = row;
        assert!(seen_questions.insert(id), "duplicate {id}");
        assert!(
            seen_decisions.insert(decision_id),
            "duplicate {decision_id}"
        );
        assert_eq!(question.id, id);
        assert_eq!(question.decision_id, decision_id);
        assert_eq!(question.choice, choice);
        assert_eq!(question.wording, wording);
        assert_eq!(question.source_anchor, source_anchor);
        assert_eq!(question.ratification_date, "2026-11-15");
        assert_eq!(question.scope_code, scope_code);
        assert!(!question.depends_on.is_empty());
        assert!(question.policy_only);
        assert!(!question.implemented);
        assert_eq!(question.verification, "NOT_VERIFIED");
        if !choices.is_empty() {
            choices.push(',');
        }
        choices.push_str(choice);
    }
    assert_eq!(choices, "B,A,A,A,A,A,A,A,A,B,B,A,A,B");
    assert_eq!(seen_questions.len(), 14);
    assert_eq!(seen_decisions.len(), 14);

    let decision_names = fs::read_dir(root.join("contracts/decisions"))
        .expect("decisions")
        .map(|item| {
            item.expect("entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect::<Vec<_>>();
    assert_eq!(
        decision_names
            .iter()
            .filter(|name| name.contains("ARCH-DEC-21"))
            .count(),
        1
    );
    assert!(
        !decision_names
            .iter()
            .any(|name| name.contains("ARCH-DEC-49")),
        "ARCH-DEC-49 stays unused after ARCH-DEC-35 through ARCH-DEC-48"
    );
    let decision = read_text(&root.join(index.decision_record));
    assert!(decision.contains("2026-11-15"));
    assert!(decision.contains("RATIFIED_DESIGN_POLICY_ONLY"));
    assert!(decision.contains("NOT_IMPLEMENTED"));
    assert!(decision.contains("NOT_VERIFIED"));
    assert!(decision.contains("not a canonical status enum"));
    assert!(decision.contains("No evidence URI is recorded"));
    assert!(decision.contains("none is invented"));
    assert!(decision.contains("**NO IMPLEMENTATION.**"));
    assert!(decision.contains("**NO VERIFICATION.**"));
    for (id, decision_id, choice, wording, source_anchor, scope_code) in expected {
        assert_eq!(
            decision.matches(&format!("## {decision_id}")).count(),
            1,
            "{decision_id} must be one heading"
        );
        assert!(decision.contains(&format!("| {decision_id} | {id} | {choice} | {wording} |")));
        assert!(decision.contains(&format!("Scope code: `{scope_code}`")));
        assert!(decision.contains(source_anchor));
        assert!(
            decision.matches(wording).count() >= 2,
            "{decision_id} wording must appear in the index and the decision body"
        );
    }
    assert!(root.join(&index.instruction_archive).is_file());
    for required in [
        "This is not a universal mathematical aggregation and not a numeric approval predicate.",
        "No metric catalogue.",
        "not permission to create a reservation",
        "Do not convert reservation to exposure.",
        "An incomplete snapshot cannot shrink or release the residual obligation.",
        "No compensating ledger line is drafted as ratified design.",
        "including a uniquely matched event",
        "This sentence does not imply that a runtime block is implemented.",
        "No candidate evidence checklist is ratified",
        "No release proof is accepted as sufficient.",
        "not a canonical valuation schema",
        "not an FX rate, not a rounding mode",
        "The ignored PostgreSQL connectivity test in this repository is not a pass",
        "No lock order. No isolation level.",
        "`XC-06` stays OPEN.",
        "This choice cannot supersede ARCH-DEC-13.",
        "Manual review alone never posts.",
        "No version migration.",
        "Recovery neither releases risk nor resubmits a broker order.",
        "does not supersede ARCH-DEC-13 or ARCH-DEC-14",
        "ARCH-DEC-13 quarantines every proposed economic effect",
        "Reservation release stays forbidden under ARCH-DEC-14.",
        "Q01–Q10 verification cells stay OPEN",
        "SOURCE_REVIEW_PENDING",
        "reviewed=false",
        "approved=false",
        "RESOLVED_BY_ARCH_DEC_02",
        "RESOLVED_BY_ARCH_DEC_03",
        "CR-B01 keeps design precedence",
        "ARCH-03B.3 is not started",
    ] {
        assert!(decision.contains(required), "decision missing {required}");
    }
    assert!(!decision.contains("```rust"));
    assert!(!decision.contains("enum OrderClosure"));

    let approval_bytes =
        fs::read(root.join("contracts/design-proposals/TRD-ARCH-03B2-approval-requests.md"))
            .expect("risk approval requests");
    const RISK_REQUEST_HISTORICAL_BYTES: usize = 39350;
    assert_eq!(
        sha256_hex(&approval_bytes[..RISK_REQUEST_HISTORICAL_BYTES]),
        "2d32dd7381a83745a9477b5b21905872fdbf45985a65fe2a810f6b4e0d4b4e71"
    );
    let historical = std::str::from_utf8(&approval_bytes[..RISK_REQUEST_HISTORICAL_BYTES])
        .expect("historical request utf-8");
    assert!(historical.starts_with("# TRD-ARCH-03B.2 — Risk decision requests"));
    assert!(historical.contains("**Ratification:** `NOT_RATIFIED`."));
    let mut historical_sections = historical.split("## RISKQ-");
    let preamble = historical_sections.next().expect("preamble");
    assert!(preamble.contains("USER_DECISION_PENDING"));
    assert!(preamble.contains("A recommendation is not an approval."));
    let historical_sections: Vec<&str> = historical_sections.collect();
    assert_eq!(historical_sections.len(), 14);
    for section in &historical_sections {
        assert!(section.contains("**Status:** `USER_DECISION_PENDING`"));
        assert!(section.contains("**Non-binding recommendation:**"));
        assert!(section.contains("CONFLICTS_WITH_BINDING_POLICY"));
        assert!(section.contains("**Alternatives:**"));
    }
    assert!(historical.contains("used + reserved + proposed_delta <= limit"));
    for (_, _, _, _, source_anchor, _) in expected {
        assert_eq!(historical.matches(source_anchor).count(), 1);
    }
    let addendum = std::str::from_utf8(&approval_bytes[RISK_REQUEST_HISTORICAL_BYTES..])
        .expect("addendum utf-8");
    assert!(addendum.contains("## Resolution index (2026-11-15, TRD-ARCH-03B.2.2)"));
    assert_eq!(addendum.matches("## RISKQ-").count(), 0);
    assert!(addendum.contains("**NO IMPLEMENTATION.**"));
    assert!(addendum.contains("**NO VERIFICATION.**"));

    for finding in [
        "XC-01",
        "XC-02-PRED",
        "XC-03-MATRIX",
        "XC-05-LOCK",
        "XC-06",
        "XC-07-ROUND",
        "XC-08-IBKR",
        "XC-09-RESET",
        "XC-10-ACTION",
    ] {
        assert!(index.open_findings.iter().any(|item| item == finding));
        assert!(decision.contains(finding), "decision missing {finding}");
    }
    assert_eq!(
        index.principle_only_findings,
        [
            "XC-02-ALIGN",
            "XC-03-BOOK",
            "XC-05-OUTBOX",
            "XC-07-FX",
            "XC-08-RULE",
            "XC-09-SOLO",
            "XC-10-RACE",
        ]
    );
    assert_eq!(index.partial_findings.len(), 2);
    assert_eq!(index.partial_findings[0].finding, "XC-04");
    assert_eq!(index.partial_findings[0].closed_part, "design_precedence");
    assert_eq!(index.partial_findings[0].by, "RESOLVED_BY_ARCH_DEC_03");
    assert_eq!(index.partial_findings[0].remainder, "OPEN");
    assert_eq!(index.partial_findings[1].finding, "XC-11");
    assert_eq!(
        index.partial_findings[1].closed_part,
        "population_and_supersession"
    );
    assert_eq!(index.partial_findings[1].by, "RESOLVED_BY_ARCH_DEC_02");
    assert_eq!(index.partial_findings[1].remainder, "OPEN");

    let manifest_bytes = fs::read(root.join("contracts/source-manifest.json")).expect("manifest");
    assert_eq!(
        sha256_hex(&manifest_bytes),
        "82b205d27172ad234d46cfa2a59f0f4785128dd7a6282dea1aab32b412fc2808"
    );
    assert_eq!(manifest_bytes.len(), 17945);
    let manifest: Value = serde_json::from_slice(&manifest_bytes).expect("manifest JSON");
    let files = manifest["files"].as_array().expect("files");
    assert_eq!(files.len(), 12);
    for entry in files {
        let relative = entry["repository_path"].as_str().expect("path");
        let filename = entry["filename"].as_str().expect("filename");
        let bytes = fs::read(root.join(relative)).expect(relative);
        assert_eq!(bytes.len() as u64, entry["bytes"].as_u64().expect("bytes"));
        assert_eq!(
            sha256_hex(&bytes),
            entry["sha256"].as_str().expect("sha256")
        );
        assert!(!entry["reviewed"].as_bool().expect("reviewed"));
        assert!(!entry["approved"].as_bool().expect("approved"));
        if filename.starts_with("DET-01") || filename.starts_with("DET-02") {
            assert!(entry["reconstructed_draft"].as_bool().expect("draft"));
            assert_eq!(entry["source_text_status"], "SOURCE_REVIEW_PENDING");
            assert_eq!(
                entry["design_baseline"],
                "DESIGN_BASELINE_RATIFIED_WITH_EXCEPTIONS"
            );
        }
    }

    let json_bytes =
        fs::read(root.join("contracts/sources/ADFR-Consolidated-v1.0.json")).expect("ADFR JSON");
    let csv_bytes =
        fs::read(root.join("contracts/sources/ADFR-Consolidated-v1.0.csv")).expect("ADFR CSV");
    assert_eq!(
        sha256_hex(&json_bytes),
        "272a6dabdb411701a25dd708577dbb498c7a7cd2754d83c612e85454a178a1d5"
    );
    assert_eq!(json_bytes.len(), 29880);
    assert_eq!(
        sha256_hex(&csv_bytes),
        "b5577302650e3d497465b93063d149313ec1197a38ddad3a9e7a6f223757ec32"
    );
    assert_eq!(csv_bytes.len(), 13138);
    let adfr = load_adfr(&root);
    let csv = load_csv(&root);
    assert_eq!(adfr.entries.len(), 78);
    assert_eq!(csv.len(), 78);
    let json_ids: Vec<&str> = adfr.entries.iter().map(|entry| entry.id.as_str()).collect();
    let csv_ids: Vec<&str> = csv.iter().map(|row| row[0].as_str()).collect();
    assert_eq!(json_ids, csv_ids);
    assert_eq!(json_ids.iter().collect::<BTreeSet<_>>().len(), 78);
    assert!(
        adfr.entries
            .iter()
            .all(|entry| entry.verification == "NOT_VERIFIED")
    );
    assert_eq!(
        adfr.entries
            .iter()
            .filter(|entry| entry.lifecycle == "SUPERSEDED")
            .count(),
        1
    );
    let superseded = adfr
        .entries
        .iter()
        .find(|entry| entry.id == SUPERSEDED_ID)
        .expect("superseded id");
    assert_eq!(superseded.superseded_by.as_deref(), Some(SUCCESSOR_ID));

    let registry_bytes = fs::read(root.join("contracts/registry/registry.json")).expect("registry");
    assert_eq!(sha256_hex(&registry_bytes), REGISTRY_SHA256);
    assert_eq!(registry_bytes.len(), 44332);
    let registry: Value = serde_json::from_slice(&registry_bytes).expect("registry JSON");
    assert_eq!(registry["registry_status"], "SKELETON_NOT_CANONICAL");
    assert_eq!(registry["canonical_schemas_published"], false);
    assert_eq!(registry["live_trading_authorized"], false);
    assert_eq!(registry["part_b"], "NO-GO");
    assert_eq!(registry["broker_uniqueness_evidence"], "EVIDENCE_PENDING");

    for (relative, hash, bytes) in [
        (
            "contracts/decisions/ARCH-DEC-01-bis-03-2026-11-15.md",
            "9f946806667a3700cb45d5033e509355aeefc60d2216262647f2dbbae034e30a",
            9761_usize,
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
            "contracts/design-proposals/TRD-ARCH-03B12-resolution-index.json",
            "e4bd482d2b8b971612e241e069009e86810bfde3cf499784725e6d3b5019ca00",
            6179,
        ),
        (
            "contracts/design-proposals/TRD-ARCH-03B1-proposal-index.json",
            "96dc34c61db04bc0d4eb0576a98f787fddb5632816b903985b1163993afc73cc",
            3654,
        ),
    ] {
        let file_bytes = fs::read(root.join(relative)).expect(relative);
        assert_eq!(file_bytes.len(), bytes, "{relative}");
        assert_eq!(sha256_hex(&file_bytes), hash, "{relative}");
    }

    let prior = read_text(&root.join("contracts/decisions/ARCH-DEC-09-bis-20-2026-11-15.md"));
    assert!(prior.contains(
        "Quarantine all proposed economic effects, including an apparently single match"
    ));
    assert!(prior.contains("No executable economic-finality predicate is accepted."));
    assert!(prior.contains("No reservation release."));

    const {
        assert!(!trading_domain::ORDER_STATE_MACHINE_IMPLEMENTED);
        assert!(!trading_domain::STATE_TRANSITIONS_IMPLEMENTED);
        assert!(!trading_domain::FINANCIAL_ROUNDING_IMPLEMENTED);
        assert!(!trading_domain::BROKER_DISPATCH_IMPLEMENTED);
        assert!(!trading_domain::RISK_EVALUATION_IMPLEMENTED);
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct BrokerProposalIndex {
    document_role: String,
    ratification: String,
    task: String,
    date: String,
    not_a_schema: bool,
    not_registry_json: bool,
    implementation_authorized: bool,
    broker_dispatch_authorized: bool,
    economic_posting_authorized: bool,
    risk_release_authorized: bool,
    economic_finality_predicate_approved: bool,
    oms_transitions_approved: bool,
    canonical_schemas_published: bool,
    canonical_schema_published_by_this_index: bool,
    canonical_broker_identity_tuple_approved: bool,
    uniqueness_index_approved: bool,
    live_trading_authorized: bool,
    part_b: String,
    broker_uniqueness_evidence: String,
    registry_status_required: String,
    order_state_machine_implemented: bool,
    state_transitions_implemented: bool,
    order_closure_enum_approved: bool,
    identity_resolver_implemented: bool,
    closes_xc_findings: bool,
    verification_executed: bool,
    external_broker_requests_executed: bool,
    omsq05_b_quarantine_remains_binding: bool,
    omsq06_a_no_predicate_no_release_remains_binding: bool,
    arch_dec_03_cr_b01_precedence_remains_binding: bool,
    preference_is_approval: bool,
    non_binding_preferred_identity: String,
    non_binding_preferred_completeness: String,
    non_binding_preferred_recovery: String,
    xc_01: String,
    xc_02_pred: String,
    xc_03_matrix: String,
    xc_04_remainder: String,
    xc_04_precedence: String,
    xc_05_lock: String,
    xc_06: String,
    xc_07_round: String,
    xc_08_ibkr: String,
    xc_09_reset: String,
    xc_10_action: String,
    xc_11_status_words: String,
    questions: Vec<OmsQuestionStatus>,
    identity_approaches: Vec<RiskApproachStatus>,
    completeness_approaches: Vec<RiskApproachStatus>,
    recovery_approaches: Vec<RiskApproachStatus>,
    coverage_scenarios: Vec<BrokerCoverageStatus>,
    open_findings: Vec<String>,
    partial_findings: Vec<OmsPartialFinding>,
    files: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct BrokerCoverageStatus {
    id: String,
    evidence: String,
    booking: String,
    release: String,
    resubmit: String,
    broker_terminal_acceptance: String,
    oracle: String,
}

#[test]
fn broker_identity_proposals_stay_unratified_and_dispatch_stays_forbidden() {
    let root = repo_root();
    let index_path = root.join("contracts/design-proposals/TRD-ARCH-03B3-proposal-index.json");
    let index_bytes = fs::read(&index_path).expect("broker proposal index");
    assert!(
        !index_bytes.starts_with(&[0xEF, 0xBB, 0xBF]),
        "broker proposal index has a BOM"
    );
    let index: BrokerProposalIndex =
        serde_json::from_slice(&index_bytes).expect("parse broker proposal index");
    assert_eq!(index.document_role, "NON_RUNTIME_NON_WIRE_PROPOSAL_INDEX");
    assert_eq!(index.ratification, "NOT_RATIFIED");
    assert_eq!(index.task, "TRD-ARCH-03B.3");
    assert_eq!(index.date, "2026-11-15");
    assert!(index.not_a_schema);
    assert!(index.not_registry_json);
    assert!(!index.implementation_authorized);
    assert!(!index.broker_dispatch_authorized);
    assert!(!index.economic_posting_authorized);
    assert!(!index.risk_release_authorized);
    assert!(!index.economic_finality_predicate_approved);
    assert!(!index.oms_transitions_approved);
    assert!(!index.canonical_schemas_published);
    assert!(!index.canonical_schema_published_by_this_index);
    assert!(!index.canonical_broker_identity_tuple_approved);
    assert!(!index.uniqueness_index_approved);
    assert!(!index.live_trading_authorized);
    assert_eq!(index.part_b, "NO-GO");
    assert_eq!(index.broker_uniqueness_evidence, "EVIDENCE_PENDING");
    assert_eq!(index.registry_status_required, "SKELETON_NOT_CANONICAL");
    assert!(!index.order_state_machine_implemented);
    assert!(!index.state_transitions_implemented);
    assert!(!index.order_closure_enum_approved);
    assert!(!index.identity_resolver_implemented);
    assert!(!index.closes_xc_findings);
    assert!(!index.verification_executed);
    assert!(!index.external_broker_requests_executed);
    assert!(index.omsq05_b_quarantine_remains_binding);
    assert!(index.omsq06_a_no_predicate_no_release_remains_binding);
    assert!(index.arch_dec_03_cr_b01_precedence_remains_binding);
    assert!(!index.preference_is_approval);
    assert_eq!(
        index.non_binding_preferred_identity,
        "strict_scoped_observation"
    );
    assert_eq!(
        index.non_binding_preferred_completeness,
        "fail_closed_partial_unknown"
    );
    assert_eq!(
        index.non_binding_preferred_recovery,
        "manual_escalation_no_resubmit"
    );
    assert_eq!(index.xc_01, "OPEN");
    assert_eq!(index.xc_02_pred, "OPEN");
    assert_eq!(index.xc_03_matrix, "OPEN");
    assert_eq!(index.xc_04_remainder, "OPEN");
    assert_eq!(index.xc_04_precedence, "RESOLVED_BY_ARCH_DEC_03");
    assert_eq!(index.xc_05_lock, "OPEN");
    assert_eq!(index.xc_06, "OPEN");
    assert_eq!(index.xc_07_round, "OPEN");
    assert_eq!(index.xc_08_ibkr, "OPEN");
    assert_eq!(index.xc_09_reset, "OPEN");
    assert_eq!(index.xc_10_action, "OPEN");
    assert_eq!(index.xc_11_status_words, "OPEN");

    let expected_questions = [
        "BROKERQ-01",
        "BROKERQ-02",
        "BROKERQ-03",
        "BROKERQ-04",
        "BROKERQ-05",
        "BROKERQ-06",
        "BROKERQ-07",
        "BROKERQ-08",
        "BROKERQ-09",
        "BROKERQ-10",
        "BROKERQ-11",
        "BROKERQ-12",
        "BROKERQ-13",
        "BROKERQ-14",
    ];
    assert_eq!(index.questions.len(), expected_questions.len());
    for (question, expected) in index.questions.iter().zip(expected_questions) {
        assert_eq!(question.id, expected);
        assert_eq!(question.status, "USER_DECISION_PENDING");
        assert!(!question.recommendation_is_approval);
    }

    let identity = [
        ("strict_scoped_observation", "COMPARISON_ONLY"),
        ("evidence_graph_candidate_sets", "COMPARISON_ONLY"),
        (
            "optimistic_global_id_merge",
            "CONFLICTS_WITH_BINDING_POLICY",
        ),
    ];
    assert_eq!(index.identity_approaches.len(), identity.len());
    for (approach, (expected_id, expected_status)) in index.identity_approaches.iter().zip(identity)
    {
        assert_eq!(approach.id, expected_id);
        assert_eq!(approach.status, expected_status);
        assert!(!approach.recommendation_is_approval);
    }
    let completeness = [
        ("fail_closed_partial_unknown", "COMPARISON_ONLY"),
        ("named_scope_proof_obligations", "COMPARISON_ONLY"),
        (
            "single_response_as_global_complete",
            "CONFLICTS_WITH_BINDING_POLICY",
        ),
    ];
    assert_eq!(index.completeness_approaches.len(), completeness.len());
    for (approach, (expected_id, expected_status)) in
        index.completeness_approaches.iter().zip(completeness)
    {
        assert_eq!(approach.id, expected_id);
        assert_eq!(approach.status, expected_status);
        assert!(!approach.recommendation_is_approval);
    }
    let recovery = [
        ("manual_escalation_no_resubmit", "COMPARISON_ONLY"),
        ("evidence_rich_manual_recovery", "COMPARISON_ONLY"),
        (
            "automatic_retry_or_lease_failover",
            "CONFLICTS_WITH_BINDING_POLICY",
        ),
    ];
    assert_eq!(index.recovery_approaches.len(), recovery.len());
    for (approach, (expected_id, expected_status)) in index.recovery_approaches.iter().zip(recovery)
    {
        assert_eq!(approach.id, expected_id);
        assert_eq!(approach.status, expected_status);
        assert!(!approach.recommendation_is_approval);
    }

    let expected_scenarios = [
        "reconnect_session_boundary",
        "client_or_account_change",
        "missing_executions",
        "empty_open_orders_snapshot",
        "empty_executions_snapshot",
        "reused_broker_order_id",
        "late_perm_id",
        "missing_perm_id",
        "unknown_send_dispatch_started",
        "lost_ack",
        "crash_before_attempt_commit",
        "crash_after_commit_before_socket",
        "prior_writer_still_active",
        "lease_expiry_alone",
        "split_brain",
        "epoch_change_open_attempt",
        "partial_fill",
        "cancel_request_race",
        "cancel_observation_label",
        "late_fill",
        "correction_bust_chain",
        "duplicate_execution_same_payload",
        "duplicate_execution_conflicting_payload",
        "stale_snapshot",
        "incomplete_time_window",
        "out_of_order_updates",
        "contradictory_observations",
        "timeout_or_disconnect",
        "filled_or_cancelled_label",
        "unknown_external_order",
    ];
    assert!(index.coverage_scenarios.len() >= 24);
    assert_eq!(index.coverage_scenarios.len(), expected_scenarios.len());
    for (scenario, expected) in index.coverage_scenarios.iter().zip(expected_scenarios) {
        assert_eq!(scenario.id, expected);
        assert_eq!(scenario.evidence, "EVIDENCE_RETAINED");
        assert_eq!(scenario.booking, "BOOKING_PROHIBITED");
        assert_eq!(scenario.release, "RELEASE_PROHIBITED");
        assert_eq!(scenario.resubmit, "RESUBMIT_PROHIBITED");
        assert_eq!(
            scenario.broker_terminal_acceptance,
            "BROKER_TERMINAL_ACCEPTANCE_PROHIBITED"
        );
        assert_eq!(scenario.oracle, "ORACLE_NOT_EXECUTED");
    }

    for finding in [
        "XC-01",
        "XC-02-PRED",
        "XC-03-MATRIX",
        "XC-05-LOCK",
        "XC-06",
        "XC-07-ROUND",
        "XC-08-IBKR",
        "XC-09-RESET",
        "XC-10-ACTION",
    ] {
        assert!(
            index.open_findings.iter().any(|item| item == finding),
            "missing {finding}"
        );
    }
    assert_eq!(index.partial_findings.len(), 2);
    assert_eq!(index.partial_findings[0].finding, "XC-04");
    assert_eq!(index.partial_findings[0].closed_part, "design precedence");
    assert_eq!(index.partial_findings[0].by, "RESOLVED_BY_ARCH_DEC_03");
    assert_eq!(index.partial_findings[0].remainder, "OPEN");
    assert_eq!(index.partial_findings[1].finding, "XC-11");
    assert_eq!(
        index.partial_findings[1].closed_part,
        "population and supersession"
    );
    assert_eq!(index.partial_findings[1].by, "RESOLVED_BY_ARCH_DEC_02");
    assert_eq!(index.partial_findings[1].remainder, "OPEN");

    let approvals =
        read_text(&root.join("contracts/design-proposals/TRD-ARCH-03B3-approval-requests.md"));
    let semantics = read_text(&root.join(
        "contracts/design-proposals/TRD-ARCH-03B3-broker-identity-semantics-and-invariants.md",
    ));
    let coverage = read_text(&root.join(
        "contracts/design-proposals/TRD-ARCH-03B3-snapshot-completeness-and-reconciliation-coverage-draft.md",
    ));
    let options = read_text(&root.join(
        "contracts/design-proposals/TRD-ARCH-03B3-identity-snapshot-and-recovery-options.md",
    ));
    let matrix = read_text(
        &root.join("contracts/design-proposals/TRD-ARCH-03B3-dependency-and-conflict-matrix.md"),
    );
    for text in [&approvals, &semantics, &coverage, &options, &matrix] {
        assert!(text.contains("NOT_RATIFIED"));
        assert!(text.contains("NOT AUTHORIZED"));
        assert!(text.contains("USER_DECISION_PENDING") || text.contains("NOT_RATIFIED"));
    }
    for id in expected_questions {
        assert!(
            approvals.contains(&format!("## {id}")),
            "missing heading {id}"
        );
    }
    assert!(approvals.contains("USER_DECISION_PENDING"));
    assert!(approvals.contains("CONFLICTS_WITH_BINDING_POLICY"));
    assert!(approvals.contains("not an approval"));
    assert!(semantics.contains("BINDING_EXISTING"));
    assert!(semantics.contains("RECONSTRUCTED_BASELINE_WITH_EXCEPTIONS"));
    assert!(semantics.contains("REFERENCE_PROPOSAL"));
    assert!(semantics.contains("EXTERNAL_EVIDENCE_PENDING"));
    assert!(semantics.contains("NEW_PROPOSAL"));
    assert!(semantics.contains("SINGLE_CANDIDATE_NOT_AUTHORIZATION"));
    assert!(coverage.contains("EXTERNAL_EVIDENCE_PENDING"));
    assert!(coverage.contains("NOT_EXECUTED"));
    for id in expected_scenarios {
        assert!(
            coverage.contains(&format!("`{id}`")),
            "coverage missing {id}"
        );
        assert!(coverage.contains("EVIDENCE_RETAINED"));
        assert!(coverage.contains("BOOKING_PROHIBITED"));
        assert!(coverage.contains("RELEASE_PROHIBITED"));
        assert!(coverage.contains("RESUBMIT_PROHIBITED"));
        assert!(coverage.contains("BROKER_TERMINAL_ACCEPTANCE_PROHIBITED"));
        assert!(coverage.contains("ORACLE_NOT_EXECUTED"));
    }
    for (id, _) in identity {
        assert!(options.contains(&format!("`{id}`")), "options missing {id}");
    }
    for (id, _) in completeness {
        assert!(options.contains(&format!("`{id}`")), "options missing {id}");
    }
    for (id, _) in recovery {
        assert!(options.contains(&format!("`{id}`")), "options missing {id}");
    }
    assert!(options.contains("CONFLICTS_WITH_BINDING_POLICY"));
    assert!(matrix.contains("| XC-01 | OPEN"));
    assert!(matrix.contains("| XC-08-IBKR | OPEN"));
    assert!(matrix.contains("| XC-02-PRED | OPEN"));
    assert!(matrix.contains("RESOLVED_BY_ARCH_DEC_03"));
    assert!(matrix.contains("does not authorize"));
    for file in &index.files {
        assert!(root.join(file).is_file(), "missing {file}");
    }

    let unchanged = [
        (
            "contracts/source-manifest.json",
            "82b205d27172ad234d46cfa2a59f0f4785128dd7a6282dea1aab32b412fc2808",
            17945_usize,
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
            "contracts/registry/registry.json",
            "7abed2af5363726403175c6524bd7a4d05d36b8735ad48442fc26941f8d98197",
            44332,
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
            "contracts/design-proposals/TRD-ARCH-03B1-proposal-index.json",
            "96dc34c61db04bc0d4eb0576a98f787fddb5632816b903985b1163993afc73cc",
            3654,
        ),
        (
            "contracts/design-proposals/TRD-ARCH-03B12-resolution-index.json",
            "e4bd482d2b8b971612e241e069009e86810bfde3cf499784725e6d3b5019ca00",
            6179,
        ),
        (
            "contracts/design-proposals/TRD-ARCH-03B2-proposal-index.json",
            "6520b2d64ebcdd00793872ac97a618c9c635e0283d3cf479f7926fc83dac62e6",
            7035,
        ),
        (
            "contracts/design-proposals/TRD-ARCH-03B22-resolution-index.json",
            "5a09d0daedef4bd9faebfd2ab1f22844fefc8bdb6ac1f906222aea94adb54718",
            10530,
        ),
    ];
    for (relative, expected_hash, expected_len) in unchanged {
        let bytes = fs::read(root.join(relative)).expect(relative);
        assert_eq!(bytes.len(), expected_len, "{relative}");
        assert_eq!(sha256_hex(&bytes), expected_hash, "{relative}");
    }

    let registry: Value = serde_json::from_slice(
        &fs::read(root.join("contracts/registry/registry.json")).expect("registry"),
    )
    .expect("registry JSON");
    assert_eq!(registry["registry_status"], "SKELETON_NOT_CANONICAL");
    assert_eq!(registry["canonical_schemas_published"], false);
    assert_eq!(registry["live_trading_authorized"], false);
    assert_eq!(registry["part_b"], "NO-GO");
    assert_eq!(registry["broker_uniqueness_evidence"], "EVIDENCE_PENDING");

    let adfr = load_adfr(&root);
    assert_eq!(adfr.entries.len(), 78);
    assert!(
        adfr.entries
            .iter()
            .all(|entry| entry.verification == "NOT_VERIFIED")
    );
    assert_eq!(
        adfr.entries
            .iter()
            .filter(|entry| entry.lifecycle == "SUPERSEDED")
            .count(),
        1
    );

    const {
        assert!(!trading_domain::ORDER_STATE_MACHINE_IMPLEMENTED);
        assert!(!trading_domain::STATE_TRANSITIONS_IMPLEMENTED);
        assert!(!trading_domain::IDENTITY_RESOLVER_IMPLEMENTED);
        assert!(!trading_domain::FINANCIAL_ROUNDING_IMPLEMENTED);
        assert!(!trading_domain::BROKER_DISPATCH_IMPLEMENTED);
        assert!(!trading_domain::RISK_EVALUATION_IMPLEMENTED);
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct BrokerPolicyIndex {
    document_role: String,
    task: String,
    date: String,
    record_label: String,
    implementation: String,
    verification: String,
    record_labels_are_not_a_canonical_enum: bool,
    decision_record: String,
    source_approval_document: String,
    instruction_archive: String,
    historical_proposal_index: String,
    historical_approval_requests: String,
    historical_approval_prefix_bytes: u64,
    historical_approval_prefix_sha256: String,
    proposal_package_historical_ratification: String,
    proposal_questions_historical_status: String,
    not_a_schema: bool,
    not_registry_json: bool,
    not_the_historical_proposal_index: bool,
    implementation_authorized: bool,
    uniqueness_guaranteed: bool,
    uniqueness_index_approved: bool,
    completeness_proof_approved: bool,
    canonical_schemas_published: bool,
    canonical_schema_published_by_this_index: bool,
    canonical_broker_identity_tuple_approved: bool,
    economic_posting_approved: bool,
    risk_release_approved: bool,
    economic_finality_predicate_approved: bool,
    oms_transitions_approved: bool,
    broker_dispatch_approved: bool,
    live_trading_authorized: bool,
    closes_xc_findings: bool,
    conflict_closure: bool,
    part_b: String,
    broker_uniqueness_evidence: String,
    registry_status_required: String,
    order_state_machine_implemented: bool,
    state_transitions_implemented: bool,
    identity_resolver_implemented: bool,
    broker_dispatch_implemented: bool,
    order_closure_enum_approved: bool,
    tests_executed: bool,
    tests_executed_scope: String,
    documentation_regression_tests_are_not_broker_evidence: bool,
    external_evidence_claimed: bool,
    external_broker_requests_executed: bool,
    evidence_uri: Option<String>,
    signature_artifact: Option<String>,
    authority: String,
    recommendation_is_approval: bool,
    option_c: String,
    choice_checksum: String,
    choice_b_count: u64,
    choice_a_count: u64,
    xc_01: String,
    xc_01_evidence: String,
    xc_08_ibkr: String,
    xc_08_ibkr_evidence: String,
    xc_04_precedence: String,
    xc_04_remainder: String,
    xc_02_pred: String,
    xc_03_matrix: String,
    xc_05_lock: String,
    xc_06: String,
    xc_07_round: String,
    xc_09_reset: String,
    xc_10_action: String,
    xc_11_status_words: String,
    design_resolved_is_not_verification: bool,
    det03_d04_single_match_books_subordinate_to_arch_dec_13_and_26: bool,
    cr_b01_precedence_over_det07: String,
    arch_dec_03_remains_binding: bool,
    arch_dec_13_quarantine_remains_binding: bool,
    arch_dec_14_no_predicate_no_release_remains_binding: bool,
    arch_dec_17_no_terminal_broker_observation_remains_binding: bool,
    arch_dec_26_quarantine_remains_binding: bool,
    arch_dec_33_no_role_remains_binding: bool,
    arch_dec_34_no_resubmit_remains_binding: bool,
    brokerq01_b_records_supplied_fields_only: bool,
    brokerq02_b_singleton_is_not_authorization: bool,
    brokerq03_b_missing_perm_id_is_not_zero: bool,
    brokerq04_b_identical_replay_has_no_economic_effect: bool,
    brokerq05_b_does_not_invent_a_link: bool,
    brokerq06_b_coverage_stays_unknown: bool,
    brokerq07_a_stores_no_completeness_token: bool,
    brokerq08_b_local_gap_is_not_an_ibkr_sequence: bool,
    brokerq09_b_reconnect_is_not_a_negative_send_proof: bool,
    brokerq10_b_authorizes_no_resend: bool,
    brokerq11_b_fence_is_not_broker_acceptance: bool,
    brokerq12_b_marker_is_not_a_role: bool,
    brokerq13_b_has_no_blanket_winner: bool,
    brokerq13_option_a_is_not_authority: bool,
    brokerq14_b_external_cases_not_executed: bool,
    brokerq14_option_a_is_not_verification: bool,
    brokerq14_requires_positive_and_negative_external_evidence: bool,
    depends_on: Vec<String>,
    questions: Vec<RiskPolicyChoice>,
    open_findings: Vec<String>,
    partial_findings: Vec<OmsPartialFinding>,
    principle_only_findings: Vec<String>,
}

#[test]
fn broker_design_policy_35_through_48_stays_unimplemented() {
    let root = repo_root();
    let expected = [
        (
            "BROKERQ-01",
            "ARCH-DEC-35",
            "B",
            r#"Vorhandene Broker-/Konto-/Umgebungs-/Session-/Aliasfelder, Client und Adapter-Epoch sowie Evidence-ID nur soweit tatsächlich geliefert als mögliche zukünftige Evidenzkomponenten erfassen. Fehlend bleibt fehlend. Feldgleichheit ist weder Merge noch Uniqueness; kein Schema."#,
            "## BROKERQ-01 — Identity tuple for a future record",
            "SUPPLIED_SCOPE_NO_MERGE_NO_SCHEMA",
        ),
        (
            "BROKERQ-02",
            "ARCH-DEC-36",
            "B",
            r#"Zukünftige Kandidatenmenge `A(e)` und Unsicherheitslabel erlauben; selbst `|A(e)|=1` ist `SINGLE_CANDIDATE_NOT_AUTHORIZATION`; 0/mehrere/Widerspruch ebenso nicht autorisierend. Keine automatische Zuordnung mit wirtschaftlicher Wirkung."#,
            "## BROKERQ-02 — Ambiguity and candidate sets",
            "CANDIDATE_SET_NOT_AUTHORIZATION",
        ),
        (
            "BROKERQ-03",
            "ARCH-DEC-37",
            "B",
            r#"Beobachtung auch ohne `permId` behalten; fehlend/null/zero nicht stillschweigend gleichsetzen. Spätere `permId` als neue Evidenz, kein Overwrite. Null-/Zero-Semantik bleibt extern ungeprüft."#,
            "## BROKERQ-03 — Missing permId",
            "RETAIN_ABSENT_PERMID_NO_ZERO_EQUATION",
        ),
        (
            "BROKERQ-04",
            "ARCH-DEC-38",
            "B",
            r#"Identischer Rohpayload bei Wiederholung idempotent als erneut gesehen; abweichender Payload unter scheinbar gleichem Schlüssel `CONTRADICTION`, beide Originale erhalten. Keine zweite Ausführung/Buchung, keine Bytes löschen."#,
            "## BROKERQ-04 — Duplicate executions",
            "IDENTICAL_REPLAY_OR_CONTRADICTION_KEEP_BOTH",
        ),
        (
            "BROKERQ-05",
            "ARCH-DEC-39",
            "B",
            r#"Correction/Bust append-only mit Original-Evidenz verknüpfen, nur wenn die neue Evidenz selbst einen Bezug liefert. Kein erfundener Link, keine Reversal-Buchung, kein Release, keine Closure-Änderung."#,
            "## BROKERQ-05 — Correction and bust chain",
            "APPEND_LINK_ONLY_IF_EVIDENCE_NAMES_IT",
        ),
        (
            "BROKERQ-06",
            "ARCH-DEC-40",
            "B",
            r#"Angefragten Account-/Client-/Session-/Dataset-/Zeitfenster-Scope soweit vorhanden dokumentieren; Coverage bleibt `UNKNOWN`, nicht global, keine `COMPLETE`-Behauptung."#,
            "## BROKERQ-06 — Snapshot scope",
            "REQUESTED_SCOPE_COVERAGE_UNKNOWN",
        ),
        (
            "BROKERQ-07",
            "ARCH-DEC-41",
            "A",
            r#"Keine Completeness-Tokens speichern. Kein `COMPLETE`-Marker und kein externer Token-Platzhalter als Ratifikation; leere Antwort ist kein Nicht-Existenz-/Finalitäts-/Release-Beweis."#,
            "## BROKERQ-07 — Completeness proof",
            "NO_COMPLETENESS_TOKEN",
        ),
        (
            "BROKERQ-08",
            "ARCH-DEC-42",
            "B",
            r#"Nur tatsächlich lokal beobachtete Capture-Gaps (z.B. kurze Seite, fehlendes Antwortende, Drop) markieren; kein erfundener IBKR-Sequenzzähler, keine Interpolation, Scope bleibt unvollständig, keine Folgeaktion."#,
            "## BROKERQ-08 — Gap detection",
            "LOCAL_CAPTURE_GAP_NOT_IBKR_SEQUENCE",
        ),
        (
            "BROKERQ-09",
            "ARCH-DEC-43",
            "B",
            r#"Reconnect erzeugt neuen Session-Kontext; alte Evidenz behalten, IDs nicht ohne nachgewiesene Bindung über Session-Grenzen übernehmen. Reconnect ist kein Send-Negativbeweis."#,
            "## BROKERQ-09 — Reconnect",
            "NEW_SESSION_NO_CROSS_SESSION_ID_CARRY",
        ),
        (
            "BROKERQ-10",
            "ARCH-DEC-44",
            "B",
            r#"CR-B01-Crash-/Dispatch-Fenster als Labels dokumentieren. Nach `DISPATCH_STARTED` möglicherweise gesendet; kein Fenster dieses Pakets erlaubt Resend, Cancel, Release oder Dispatch."#,
            "## BROKERQ-10 — Unknown send",
            "WINDOW_LABELS_NO_RESEND",
        ),
        (
            "BROKERQ-11",
            "ARCH-DEC-45",
            "B",
            r#"Lokale Fence-Fakten (Epoch, Lease-Deadline, Authority-Label) von Broker-Akzeptanz strikt trennen; Successor blockiert solange früherer Writer noch senden könnte oder Sendestatus unbekannt ist. Lease-Ablauf/neuere Epoch beweist kein externes Fencing. Keine Implementierung oder TTL."#,
            "## BROKERQ-11 — Epoch fencing and broker acceptance",
            "LOCAL_FENCE_NOT_BROKER_ACCEPTANCE",
        ),
        (
            "BROKERQ-12",
            "ARCH-DEC-46",
            "B",
            r#"Nur nicht-autorisierenden Eskalationsmarker für spätere menschliche Prüfung vorsehen; keine Rolle, kein Post-/Release-/Dispatch-/Reset-Recht. Fehlende erforderliche zweite Person bedeutet Deny, nicht simulierte Zustimmung."#,
            "## BROKERQ-12 — Manual escalation",
            "ESCALATION_MARKER_NO_ROLE",
        ),
        (
            "BROKERQ-13",
            "ARCH-DEC-47",
            "B",
            r#"Broker-Rohbeobachtung und lokale Attempt-Fakten bei Widerspruch beide erhalten; Vergleich als Inferenz kennzeichnen. Keine Seite gewinnt pauschal, kein wirtschaftlicher Schluss, kein Cancel."#,
            "## BROKERQ-13 — Evidence authority",
            "RETAIN_BOTH_NO_WINNER",
        ),
        (
            "BROKERQ-14",
            "ARCH-DEC-48",
            "B",
            r#"Für spätere Verifikation tatsächlich beschaffte offizielle Unterlagen, reproduzierbare positive/negative Fälle, Umgebung, Brokerprodukt/API-Version, Zeitstempel und Rohpayloads verlangen. Heute alles `NOT_EXECUTED`/`EXTERNAL_EVIDENCE_PENDING`; Dokumentations-/Unit-Tests sind keine Broker-Evidenz."#,
            "## BROKERQ-14 — Future verification bar",
            "EXTERNAL_BAR_NOT_EXECUTED",
        ),
    ];

    let proposal_path = root.join("contracts/design-proposals/TRD-ARCH-03B3-proposal-index.json");
    let proposal_bytes = fs::read(&proposal_path).expect("historical broker proposal index");
    assert_eq!(proposal_bytes.len(), 13593);
    assert_eq!(
        sha256_hex(&proposal_bytes),
        "e808dcf8a2339845b4ef49085275b7aad9094862086830d57d13a415841703ce"
    );
    let proposal_text = String::from_utf8(proposal_bytes).expect("proposal index utf-8");
    assert!(proposal_text.contains("\"ratification\": \"NOT_RATIFIED\""));
    assert!(proposal_text.contains("\"USER_DECISION_PENDING\""));
    assert!(!proposal_text.contains("ARCH-DEC-35"));
    assert!(!proposal_text.contains("\"decision_id\""));

    let index_path = root.join("contracts/design-proposals/TRD-ARCH-03B32-resolution-index.json");
    let index_bytes = fs::read(&index_path).expect("broker policy index");
    assert!(!index_bytes.starts_with(&[0xEF, 0xBB, 0xBF]));
    let index: BrokerPolicyIndex =
        serde_json::from_slice(&index_bytes).expect("parse broker policy index");
    assert_eq!(index.document_role, "NON_RUNTIME_NON_WIRE_RESOLUTION_INDEX");
    assert_ne!(index.document_role, "NON_RUNTIME_NON_WIRE_PROPOSAL_INDEX");
    assert_eq!(index.task, "TRD-ARCH-03B.3.2");
    assert_eq!(index.date, "2026-11-15");
    assert_eq!(index.record_label, "RATIFIED_DESIGN_POLICY_ONLY");
    assert_eq!(index.implementation, "NOT_IMPLEMENTED");
    assert_eq!(index.verification, "NOT_VERIFIED");
    assert!(index.record_labels_are_not_a_canonical_enum);
    assert_eq!(
        index.decision_record,
        "contracts/decisions/ARCH-DEC-35-bis-48-2026-11-15.md"
    );
    assert_eq!(index.source_approval_document, index.decision_record);
    assert_eq!(
        index.instruction_archive,
        "docs/cursor-prompts/trading/TRD-ARCH-03B3.2-Ratify-Broker-Design-Decisions.md"
    );
    assert_eq!(
        index.historical_proposal_index,
        "contracts/design-proposals/TRD-ARCH-03B3-proposal-index.json"
    );
    assert_eq!(
        index.historical_approval_requests,
        "contracts/design-proposals/TRD-ARCH-03B3-approval-requests.md"
    );
    assert_eq!(index.historical_approval_prefix_bytes, 31525);
    assert_eq!(
        index.historical_approval_prefix_sha256,
        "0ae902d63e138833545b85fe23efa745ef20f9a1f1a07d4790af76374290975f"
    );
    assert_eq!(
        index.proposal_package_historical_ratification,
        "NOT_RATIFIED"
    );
    assert_eq!(
        index.proposal_questions_historical_status,
        "USER_DECISION_PENDING"
    );
    assert!(index.not_a_schema);
    assert!(index.not_registry_json);
    assert!(index.not_the_historical_proposal_index);
    assert!(!index.implementation_authorized);
    assert!(!index.uniqueness_guaranteed);
    assert!(!index.uniqueness_index_approved);
    assert!(!index.completeness_proof_approved);
    assert!(!index.canonical_schemas_published);
    assert!(!index.canonical_schema_published_by_this_index);
    assert!(!index.canonical_broker_identity_tuple_approved);
    assert!(!index.economic_posting_approved);
    assert!(!index.risk_release_approved);
    assert!(!index.economic_finality_predicate_approved);
    assert!(!index.oms_transitions_approved);
    assert!(!index.broker_dispatch_approved);
    assert!(!index.live_trading_authorized);
    assert!(!index.closes_xc_findings);
    assert!(!index.conflict_closure);
    assert_eq!(index.part_b, "NO-GO");
    assert_eq!(index.broker_uniqueness_evidence, "EVIDENCE_PENDING");
    assert_eq!(index.registry_status_required, "SKELETON_NOT_CANONICAL");
    assert!(!index.order_state_machine_implemented);
    assert!(!index.state_transitions_implemented);
    assert!(!index.identity_resolver_implemented);
    assert!(!index.broker_dispatch_implemented);
    assert!(!index.order_closure_enum_approved);
    assert!(!index.tests_executed);
    assert_eq!(
        index.tests_executed_scope,
        "external_broker_and_coverage_tests"
    );
    assert!(index.documentation_regression_tests_are_not_broker_evidence);
    assert!(!index.external_evidence_claimed);
    assert!(!index.external_broker_requests_executed);
    assert!(index.evidence_uri.is_none());
    assert!(index.signature_artifact.is_none());
    assert_eq!(index.authority, "user_ratification_dated_2026-11-15");
    assert!(!index.recommendation_is_approval);
    assert_eq!(index.option_c, "CONFLICTS_WITH_BINDING_POLICY");
    assert_eq!(index.choice_checksum, "B,B,B,B,B,B,A,B,B,B,B,B,B,B");
    assert_eq!(index.choice_b_count, 13);
    assert_eq!(index.choice_a_count, 1);
    assert_eq!(index.xc_01, "OPEN");
    assert_eq!(index.xc_01_evidence, "EVIDENCE_PENDING");
    assert_eq!(index.xc_08_ibkr, "OPEN");
    assert_eq!(index.xc_08_ibkr_evidence, "EXTERNAL_EVIDENCE_PENDING");
    assert_eq!(index.xc_04_precedence, "RESOLVED_BY_ARCH_DEC_03");
    assert_eq!(index.xc_04_remainder, "OPEN");
    assert_eq!(index.xc_02_pred, "OPEN");
    assert_eq!(index.xc_03_matrix, "OPEN");
    assert_eq!(index.xc_05_lock, "OPEN");
    assert_eq!(index.xc_06, "OPEN");
    assert_eq!(index.xc_07_round, "OPEN");
    assert_eq!(index.xc_09_reset, "OPEN");
    assert_eq!(index.xc_10_action, "OPEN");
    assert_eq!(index.xc_11_status_words, "OPEN");
    assert!(index.design_resolved_is_not_verification);
    assert!(index.det03_d04_single_match_books_subordinate_to_arch_dec_13_and_26);
    assert_eq!(index.cr_b01_precedence_over_det07, "IN_FORCE");
    assert!(index.arch_dec_03_remains_binding);
    assert!(index.arch_dec_13_quarantine_remains_binding);
    assert!(index.arch_dec_14_no_predicate_no_release_remains_binding);
    assert!(index.arch_dec_17_no_terminal_broker_observation_remains_binding);
    assert!(index.arch_dec_26_quarantine_remains_binding);
    assert!(index.arch_dec_33_no_role_remains_binding);
    assert!(index.arch_dec_34_no_resubmit_remains_binding);
    assert!(index.brokerq01_b_records_supplied_fields_only);
    assert!(index.brokerq02_b_singleton_is_not_authorization);
    assert!(index.brokerq03_b_missing_perm_id_is_not_zero);
    assert!(index.brokerq04_b_identical_replay_has_no_economic_effect);
    assert!(index.brokerq05_b_does_not_invent_a_link);
    assert!(index.brokerq06_b_coverage_stays_unknown);
    assert!(index.brokerq07_a_stores_no_completeness_token);
    assert!(index.brokerq08_b_local_gap_is_not_an_ibkr_sequence);
    assert!(index.brokerq09_b_reconnect_is_not_a_negative_send_proof);
    assert!(index.brokerq10_b_authorizes_no_resend);
    assert!(index.brokerq11_b_fence_is_not_broker_acceptance);
    assert!(index.brokerq12_b_marker_is_not_a_role);
    assert!(index.brokerq13_b_has_no_blanket_winner);
    assert!(index.brokerq13_option_a_is_not_authority);
    assert!(index.brokerq14_b_external_cases_not_executed);
    assert!(index.brokerq14_option_a_is_not_verification);
    assert!(index.brokerq14_requires_positive_and_negative_external_evidence);
    assert!(!index.depends_on.is_empty());
    for finding in [
        "XC-01",
        "XC-02-PRED",
        "XC-03-MATRIX",
        "XC-05-LOCK",
        "XC-06",
        "XC-07-ROUND",
        "XC-08-IBKR",
        "XC-09-RESET",
        "XC-10-ACTION",
    ] {
        assert!(
            index.open_findings.iter().any(|item| item == finding),
            "missing {finding}"
        );
    }
    assert_eq!(index.partial_findings.len(), 2);
    assert_eq!(index.partial_findings[0].finding, "XC-04");
    assert_eq!(index.partial_findings[0].closed_part, "design_precedence");
    assert_eq!(index.partial_findings[0].by, "RESOLVED_BY_ARCH_DEC_03");
    assert_eq!(index.partial_findings[0].remainder, "OPEN");
    assert_eq!(index.partial_findings[1].finding, "XC-11");
    assert_eq!(
        index.partial_findings[1].closed_part,
        "population_and_supersession"
    );
    assert_eq!(index.partial_findings[1].by, "RESOLVED_BY_ARCH_DEC_02");
    assert_eq!(index.partial_findings[1].remainder, "OPEN");
    for principle in [
        "XC-02-ALIGN",
        "XC-03-BOOK",
        "XC-05-OUTBOX",
        "XC-07-FX",
        "XC-08-RULE",
        "XC-09-SOLO",
        "XC-10-RACE",
    ] {
        assert!(
            index
                .principle_only_findings
                .iter()
                .any(|item| item == principle),
            "missing {principle}"
        );
    }

    assert_eq!(index.questions.len(), expected.len());
    let mut seen_questions = BTreeSet::new();
    let mut seen_decisions = BTreeSet::new();
    let mut choices = String::new();
    for (question, row) in index.questions.iter().zip(expected) {
        let (id, decision_id, choice, wording, source_anchor, scope_code) = row;
        assert!(seen_questions.insert(id), "duplicate {id}");
        assert!(
            seen_decisions.insert(decision_id),
            "duplicate {decision_id}"
        );
        assert_eq!(question.id, id);
        assert_eq!(question.decision_id, decision_id);
        assert_eq!(question.choice, choice);
        assert_eq!(question.wording, wording);
        assert_eq!(question.source_anchor, source_anchor);
        assert_eq!(question.ratification_date, "2026-11-15");
        assert_eq!(question.scope_code, scope_code);
        assert!(!question.depends_on.is_empty());
        assert!(question.policy_only);
        assert!(!question.implemented);
        assert_eq!(question.verification, "NOT_VERIFIED");
        assert_eq!(question.verification, "NOT_VERIFIED");
        if !choices.is_empty() {
            choices.push(',');
        }
        choices.push_str(choice);
    }
    assert_eq!(choices, "B,B,B,B,B,B,A,B,B,B,B,B,B,B");
    assert_eq!(seen_questions.len(), 14);
    assert_eq!(seen_decisions.len(), 14);
    assert_eq!(
        index
            .questions
            .iter()
            .filter(|question| question.choice == "A")
            .map(|question| question.id.as_str())
            .collect::<Vec<_>>(),
        vec!["BROKERQ-07"]
    );
    assert_eq!(
        index
            .questions
            .iter()
            .filter(|question| question.choice == "B")
            .count(),
        13
    );

    let decision_names = fs::read_dir(root.join("contracts/decisions"))
        .expect("decisions")
        .map(|item| {
            item.expect("entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect::<Vec<_>>();
    assert_eq!(
        decision_names
            .iter()
            .filter(|name| name.contains("ARCH-DEC-35"))
            .count(),
        1
    );
    assert!(
        !decision_names
            .iter()
            .any(|name| name.contains("ARCH-DEC-49"))
    );
    let decision = read_text(&root.join(index.decision_record));
    assert!(decision.contains("2026-11-15"));
    assert!(decision.contains("RATIFIED_DESIGN_POLICY_ONLY"));
    assert!(decision.contains("NOT_IMPLEMENTED"));
    assert!(decision.contains("NOT_VERIFIED"));
    assert!(decision.contains("NON_RUNTIME_NON_WIRE"));
    assert!(decision.contains("not a canonical status enum"));
    assert!(decision.contains("No evidence URI is recorded"));
    assert!(decision.contains("none is invented"));
    assert!(decision.contains("**NO IMPLEMENTATION.**"));
    assert!(decision.contains("**NO VERIFICATION.**"));
    for (id, decision_id, choice, wording, source_anchor, scope_code) in expected {
        assert_eq!(
            decision.matches(&format!("## {decision_id}")).count(),
            1,
            "{decision_id} must be one heading"
        );
        assert!(decision.contains(&format!("| {decision_id} | {id} | {choice} | {wording} |")));
        assert!(decision.contains(&format!("Scope code: `{scope_code}`")));
        assert!(decision.contains(source_anchor));
        assert!(
            decision.matches(wording).count() >= 2,
            "{decision_id} wording must appear in the index and the decision body"
        );
    }
    assert!(root.join(&index.instruction_archive).is_file());
    for required in [
        "Missing `permId` does not equal zero and does not equal another missing value.",
        "A singleton candidate is `SINGLE_CANDIDATE_NOT_AUTHORIZATION`.",
        "Identical raw payload replay creates no economic effect and no second execution.",
        "A contradiction keeps both original payloads.",
        "A correction link is stored only when the new evidence itself names one.",
        "No invented link is authorized.",
        "Coverage stays `UNKNOWN`.",
        "No completeness token is stored.",
        "An empty response is not a negative proof",
        "A local capture gap is not an IBKR sequence number.",
        "Reconnect creates a new session context.",
        "Reconnect is not proof that the previous send did not happen.",
        "Unknown send does not authorize resubmit or takeover.",
        "Lease expiry does not authorize takeover.",
        "A local fence is not broker acceptance.",
        "An escalation marker is not a role.",
        "Neither the broker observation nor the local record wins.",
        "Positive and negative external cases are required later and are `NOT_EXECUTED`.",
        "Documentation tests are not broker evidence.",
        "`DESIGN_RESOLVED` is not verification.",
        "The DET-03-D04 and CR-B04 §4.3 single-match booking sentence stays subordinate to ARCH-DEC-13 and ARCH-DEC-26.",
        "CR-B01 keeps design precedence",
        "`XC-01` stays OPEN",
        "`XC-08-IBKR` stays OPEN",
        "`XC-02-PRED` stays OPEN",
        "The remainder of XC-04 stays OPEN. XC-04 design precedence stays `RESOLVED_BY_ARCH_DEC_03`.",
        "`XC-03-MATRIX` stays OPEN",
        "`XC-05-LOCK` stays OPEN",
        "`XC-07-ROUND` stays OPEN",
        "`XC-09-RESET` stays OPEN",
        "`XC-10-ACTION` stays OPEN",
        "status-word remainder stays OPEN",
        "This record states no IBKR guarantee",
        "EXTERNAL_EVIDENCE_PENDING",
        "NOT_EXECUTED",
    ] {
        assert!(decision.contains(required), "decision missing {required}");
    }
    assert!(!decision.contains("```rust"));
    assert!(!decision.contains("enum OrderClosure"));
    assert!(!decision.contains("UNIQUE index is approved"));

    let approval_bytes =
        fs::read(root.join("contracts/design-proposals/TRD-ARCH-03B3-approval-requests.md"))
            .expect("broker approval requests");
    const BROKER_REQUEST_HISTORICAL_BYTES: usize = 31525;
    assert_eq!(
        sha256_hex(&approval_bytes[..BROKER_REQUEST_HISTORICAL_BYTES]),
        "0ae902d63e138833545b85fe23efa745ef20f9a1f1a07d4790af76374290975f"
    );
    let historical = std::str::from_utf8(&approval_bytes[..BROKER_REQUEST_HISTORICAL_BYTES])
        .expect("historical broker request utf-8");
    assert!(historical.contains("USER_DECISION_PENDING"));
    assert!(historical.contains("NOT_RATIFIED"));
    assert!(historical.contains("## BROKERQ-07 — Completeness proof"));
    assert!(!historical.contains("ARCH-DEC-35"));
    assert!(approval_bytes.len() > BROKER_REQUEST_HISTORICAL_BYTES);
    let addendum = std::str::from_utf8(&approval_bytes[BROKER_REQUEST_HISTORICAL_BYTES..])
        .expect("addendum utf-8");
    assert!(addendum.contains("## Resolution index (2026-11-15, TRD-ARCH-03B.3.2)"));
    assert!(addendum.contains("NON_RUNTIME_NON_WIRE_RESOLUTION_INDEX"));
    assert!(addendum.contains("BROKERQ-07"));
    assert!(addendum.contains("| A |"));

    let unchanged = [
        (
            "contracts/design-proposals/TRD-ARCH-03B3-broker-identity-semantics-and-invariants.md",
            "51f098c9b4e4caa0f9d204496c8976c113139e2ff6a74a8654637202637a3583",
            18980_usize,
        ),
        (
            "contracts/design-proposals/TRD-ARCH-03B3-identity-snapshot-and-recovery-options.md",
            "5939cb69b1fc198d961904e1f4c9371afe2d7810e33c5ff168734b220ec6e2c8",
            13127,
        ),
        (
            "contracts/design-proposals/TRD-ARCH-03B3-snapshot-completeness-and-reconciliation-coverage-draft.md",
            "5252d0d5a6864273e065bde7cfd0d2d0ecb75b70f7d43334b9ba4c967b515173",
            17455,
        ),
        (
            "contracts/design-proposals/TRD-ARCH-03B3-dependency-and-conflict-matrix.md",
            "479ee5cf92d9f09ba80d5fd780d85218ae61c8746d268c85c7b13e14b40020c7",
            7916,
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
            "contracts/registry/registry.json",
            "7abed2af5363726403175c6524bd7a4d05d36b8735ad48442fc26941f8d98197",
            44332,
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
            "contracts/design-proposals/TRD-ARCH-03B1-proposal-index.json",
            "96dc34c61db04bc0d4eb0576a98f787fddb5632816b903985b1163993afc73cc",
            3654,
        ),
        (
            "contracts/design-proposals/TRD-ARCH-03B12-resolution-index.json",
            "e4bd482d2b8b971612e241e069009e86810bfde3cf499784725e6d3b5019ca00",
            6179,
        ),
        (
            "contracts/design-proposals/TRD-ARCH-03B2-proposal-index.json",
            "6520b2d64ebcdd00793872ac97a618c9c635e0283d3cf479f7926fc83dac62e6",
            7035,
        ),
        (
            "contracts/design-proposals/TRD-ARCH-03B22-resolution-index.json",
            "5a09d0daedef4bd9faebfd2ab1f22844fefc8bdb6ac1f906222aea94adb54718",
            10530,
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
    for (relative, expected_hash, expected_len) in unchanged {
        let bytes = fs::read(root.join(relative)).expect(relative);
        assert_eq!(bytes.len(), expected_len, "{relative}");
        assert_eq!(sha256_hex(&bytes), expected_hash, "{relative}");
    }

    let registry_text =
        fs::read_to_string(root.join("contracts/registry/registry.json")).expect("registry text");
    assert!(!registry_text.contains("TRD-ARCH-03B32"));
    assert!(!registry_text.contains("ARCH-DEC-35"));
    assert!(!root.join("contracts/sql").exists());
    assert!(!root.join("contracts/events").exists());

    let manifest_bytes = fs::read(root.join("contracts/source-manifest.json")).expect("manifest");
    let manifest: Value = serde_json::from_slice(&manifest_bytes).expect("manifest JSON");
    let files = manifest["files"].as_array().expect("files");
    assert_eq!(files.len(), 12);
    for entry in files {
        let relative = entry["repository_path"].as_str().expect("path");
        let filename = entry["filename"].as_str().expect("filename");
        let bytes = fs::read(root.join(relative)).expect(relative);
        assert_eq!(bytes.len() as u64, entry["bytes"].as_u64().expect("bytes"));
        assert_eq!(
            sha256_hex(&bytes),
            entry["sha256"].as_str().expect("sha256")
        );
        assert!(!entry["reviewed"].as_bool().expect("reviewed"));
        assert!(!entry["approved"].as_bool().expect("approved"));
        if filename.starts_with("DET-01") || filename.starts_with("DET-02") {
            assert!(entry["reconstructed_draft"].as_bool().expect("draft"));
            assert_eq!(entry["source_text_status"], "SOURCE_REVIEW_PENDING");
        }
    }

    let adfr = load_adfr(&root);
    let csv = load_csv(&root);
    assert_eq!(adfr.entries.len(), 78);
    assert_eq!(csv.len(), 78);
    let json_ids: Vec<&str> = adfr.entries.iter().map(|entry| entry.id.as_str()).collect();
    let csv_ids: Vec<&str> = csv.iter().map(|row| row[0].as_str()).collect();
    assert_eq!(json_ids, csv_ids);
    assert!(
        adfr.entries
            .iter()
            .all(|entry| entry.verification == "NOT_VERIFIED")
    );
    assert_eq!(
        adfr.entries
            .iter()
            .filter(|entry| entry.lifecycle == "SUPERSEDED")
            .count(),
        1
    );
    let superseded = adfr
        .entries
        .iter()
        .find(|entry| entry.lifecycle == "SUPERSEDED")
        .expect("superseded");
    assert_eq!(superseded.id, "ADFR-CROSS-005");
    assert_eq!(superseded.superseded_by.as_deref(), Some("ADFR-CR-B10-001"));

    fn walk_src(dir: &std::path::Path) {
        for item in fs::read_dir(dir).expect("read src tree") {
            let path = item.expect("entry").path();
            if path.is_dir() {
                walk_src(&path);
            } else if path
                .components()
                .any(|component| component.as_os_str() == "src")
            {
                let bytes = fs::read(&path).unwrap_or_default();
                let text = String::from_utf8_lossy(&bytes);
                assert!(
                    !text.contains("ARCH-DEC-35"),
                    "production source must not embed the broker policy record: {}",
                    path.display()
                );
            }
        }
    }
    for top in ["crates", "apps"] {
        let base = root.join(top);
        if base.is_dir() {
            walk_src(&base);
        }
    }

    const {
        assert!(!trading_domain::ORDER_STATE_MACHINE_IMPLEMENTED);
        assert!(!trading_domain::STATE_TRANSITIONS_IMPLEMENTED);
        assert!(!trading_domain::IDENTITY_RESOLVER_IMPLEMENTED);
        assert!(!trading_domain::FINANCIAL_ROUNDING_IMPLEMENTED);
        assert!(!trading_domain::BROKER_DISPATCH_IMPLEMENTED);
        assert!(!trading_domain::RISK_EVALUATION_IMPLEMENTED);
    }
}

fn hex_encode(bytes: impl AsRef<[u8]>) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let bytes = bytes.as_ref();
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(char::from(HEX[usize::from(byte >> 4)]));
        out.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    out
}
