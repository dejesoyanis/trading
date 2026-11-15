//! Documentation checks for the ADFR register after ARCH-DEC-02.
//!
//! These checks do not implement trading behavior, do not pass B10-T01
//! through B10-T12, and do not treat `DESIGN_RESOLVED` as a test result.

use std::collections::BTreeSet;
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
