//! Documentation checks for the TRD-ARCH-03B.5 public-research boundary.
//!
//! These checks do not retrieve an IBKR page, do not connect to a broker,
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
    assert!(!bytes.contains(&b'\r'), "CR in {}", path.display());
    String::from_utf8(bytes).unwrap_or_else(|_| panic!("utf-8 {}", path.display()))
}

fn table_rows<'a>(text: &'a str, id_prefix: &str) -> Vec<Vec<&'a str>> {
    let mut rows = Vec::new();
    for line in text.lines() {
        if !line.starts_with("| ") || line.starts_with("| ---") || line.starts_with("| ID ") {
            continue;
        }
        let cells: Vec<&str> = line.trim_matches('|').split('|').map(str::trim).collect();
        if cells
            .first()
            .is_some_and(|cell| cell.starts_with(id_prefix))
        {
            rows.push(cells);
        }
    }
    rows
}

fn unquote(cell: &str) -> &str {
    cell.trim_matches('`')
}

fn inventory_headings(text: &str) -> Vec<(String, String)> {
    let mut rows = Vec::new();
    for line in text.lines() {
        let Some(rest) = line.strip_prefix("## IBKR-CLM-") else {
            continue;
        };
        let (number, heading) = rest
            .split_once(" — ")
            .unwrap_or_else(|| panic!("claim heading {line}"));
        rows.push((format!("IBKR-CLM-{number}"), heading.to_string()));
    }
    rows
}

fn matrix_titles(text: &str) -> Vec<(String, String)> {
    let mut rows = Vec::new();
    let mut current = None;
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("### IBKR-SCN-") {
            current = Some(format!("IBKR-SCN-{rest}"));
        } else if let Some(title) = line.strip_prefix("- **Title:** ") {
            let id = current
                .clone()
                .unwrap_or_else(|| panic!("title without id"));
            rows.push((id, title.to_string()));
            current = None;
        }
    }
    rows
}

#[test]
fn ibkr_public_documentation_research_stays_unretrieved() {
    let root = repo_root();
    let plan = root.join("contracts/verification-plans");
    let index_path = plan.join("TRD-ARCH-03B5-source-index.json");
    let index_bytes = fs::read(&index_path).expect("research index");
    assert!(
        !index_bytes
            .windows(64)
            .any(|window| { window.iter().all(|byte| byte.is_ascii_hexdigit()) })
    );
    let index: Value = serde_json::from_slice(&index_bytes).expect("parse research index");
    assert_eq!(
        index["document_role"],
        "NON_RUNTIME_NON_WIRE_RESEARCH_INDEX"
    );
    assert_ne!(
        index["document_role"],
        "NON_RUNTIME_NON_WIRE_RESOLUTION_INDEX"
    );
    assert_ne!(
        index["document_role"],
        "NON_RUNTIME_NON_WIRE_VERIFICATION_PLAN_INDEX"
    );
    assert_eq!(index["task"], "TRD-ARCH-03B.5");
    assert_eq!(index["date"], "2026-11-16");
    assert_eq!(
        index["baseline_head"],
        "ac60bf932355660548bf5751963009ea956b76a5"
    );
    assert_eq!(index["authority_is_stage_go"], false);
    assert_eq!(index["external_fetch_performed"], false);
    assert_eq!(index["obtained_source_count"], 0);
    assert_eq!(index["second_copy_count"], 0);
    assert!(index["fetch_timestamp_utc"].is_null());
    assert_eq!(index["local_source_archive_created"], false);
    assert_eq!(index["raw_payload_store_created"], false);
    assert_eq!(index["sources"].as_array().expect("sources").len(), 0);
    assert_eq!(index["read_candidates"].as_array().expect("reads").len(), 0);
    assert_eq!(index["reads_approved"], "NO_READS_APPROVED");
    assert_eq!(index["environment_selected"], false);
    assert_eq!(index["new_arch_dec_assigned"], false);
    assert_eq!(index["next_unused_decision_id_observed"], "ARCH-DEC-63");
    assert_eq!(index["documentation_host_status"], "UNRECORDED");
    assert_eq!(
        index["documentation_hosts_recorded"]
            .as_array()
            .expect("hosts")
            .len(),
        0
    );
    assert_eq!(index["public_documentation_research_authorized"], true);
    assert_eq!(
        index["public_documentation_research_authorized_is_stage_go"],
        false
    );
    assert_eq!(index["external_broker_tests_executed"], false);
    assert_eq!(index["external_broker_requests_executed"], false);
    assert_eq!(index["human_checkpoint"], "STOP");
    assert_eq!(index["part_b"], "NO-GO");
    assert_eq!(index["broker_uniqueness_evidence"], "EVIDENCE_PENDING");
    assert_eq!(index["registry_status_required"], "SKELETON_NOT_CANONICAL");
    assert_eq!(index["closes_xc_findings"], false);
    for stage in ["stage_e0", "stage_e1", "stage_e2", "stage_e3", "stage_e4"] {
        assert_eq!(index[stage], "STOP", "{stage}");
        let authorized = format!("{stage}_authorized");
        assert_eq!(index[authorized.as_str()], false, "{authorized}");
    }
    for flag in [
        "broker_connection_authorized",
        "read_only_broker_access_authorized",
        "broker_testing_authorized",
        "order_send_authorized",
        "paper_trading_authorized",
        "live_trading_authorized",
        "economic_posting_approved",
        "risk_release_approved",
        "economic_finality_predicate_approved",
        "oms_transitions_approved",
        "broker_dispatch_approved",
        "canonical_schemas_published",
        "uniqueness_guaranteed",
        "uniqueness_index_approved",
        "completeness_proof_approved",
        "completeness_token_ratified",
        "implementation_authorized",
        "added_to_canonical_registry",
    ] {
        assert_eq!(index[flag], false, "{flag}");
    }
    let tuple = &index["version_tuple"];
    for field in [
        "product_name",
        "product_version",
        "api_client_name",
        "api_client_version",
        "platform",
        "operating_system",
        "account_mode",
        "environment_class",
        "publication_date",
        "change_date",
    ] {
        assert_eq!(tuple[field], "UNRECORDED", "{field}");
    }

    let topics = index["research_topics"].as_array().expect("topics");
    assert_eq!(topics.len(), 14);
    let mut seen_topics = BTreeSet::new();
    for topic in topics {
        let id = topic["id"].as_str().expect("topic id");
        assert!(seen_topics.insert(id.to_string()), "duplicate {id}");
        assert_eq!(topic["documentation_class"], "UNVERIFIED_RESEARCH_TOPIC");
        assert_eq!(topic["retrieval_status"], "NOT_RETRIEVED");
        assert_eq!(topic["official_source_id"], "NOT_ESTABLISHED");
        assert_eq!(topic["section_anchor"], "NOT_ESTABLISHED");
        assert!(!topic["question"].as_str().expect("question").is_empty());
    }
    assert!(seen_topics.contains("T-01"));
    assert!(seen_topics.contains("T-14"));

    let inventory = read_text(&plan.join("TRD-ARCH-03B4-ibkr-evidence-inventory.md"));
    let historical_claims = inventory_headings(&inventory);
    assert_eq!(historical_claims.len(), 27);
    let claims = index["claims"].as_array().expect("claims");
    assert_eq!(claims.len(), 27);
    let claim_matrix = read_text(&plan.join("TRD-ARCH-03B5-claim-to-source-matrix.md"));
    let claim_rows = table_rows(&claim_matrix, "IBKR-CLM-");
    assert_eq!(claim_rows.len(), 27);
    let mut seen_claims = BTreeSet::new();
    for ((historical_id, historical_heading), claim) in historical_claims.iter().zip(claims) {
        let id = claim["id"].as_str().expect("claim id");
        assert_eq!(id, historical_id);
        assert!(seen_claims.insert(id.to_string()), "duplicate {id}");
        assert_eq!(claim["heading"], historical_heading.as_str());
        assert_eq!(claim["documentation_class"], "UNVERIFIED_RESEARCH_TOPIC");
        assert_eq!(claim["official_source_id"], "NOT_ESTABLISHED");
        assert_eq!(claim["section_anchor"], "NOT_ESTABLISHED");
        assert_eq!(claim["hypothesis_status"], "HYPOTHESIS");
        assert_eq!(claim["evidence"], "EXTERNAL_EVIDENCE_PENDING");
        assert_eq!(claim["execution"], "NOT_EXECUTED");
        assert_eq!(claim["empirical_upgrade"], false);
        let topic = claim["research_topic"].as_str().expect("topic");
        for part in topic.split('+') {
            assert!(seen_topics.contains(part), "{id} topic {part}");
        }
        let row = claim_rows
            .iter()
            .find(|cells| cells[0] == id)
            .unwrap_or_else(|| panic!("matrix missing {id}"));
        assert_eq!(row.len(), 9, "{id}");
        assert_eq!(row[1], historical_heading.as_str());
        assert_eq!(row[2], topic);
        assert_eq!(unquote(row[3]), "UNVERIFIED_RESEARCH_TOPIC");
        assert_eq!(unquote(row[4]), "NOT_ESTABLISHED");
        assert_eq!(unquote(row[5]), "NOT_ESTABLISHED");
        assert_eq!(unquote(row[6]), "HYPOTHESIS");
        assert_eq!(unquote(row[7]), "EXTERNAL_EVIDENCE_PENDING");
        assert_eq!(unquote(row[8]), "NOT_EXECUTED");
    }
    assert_eq!(seen_claims.len(), 27);
    assert!(claim_matrix.contains("Silence is not a negative finding."));
    assert!(!claim_matrix.contains("http://"));
    assert!(!claim_matrix.contains("https://"));

    let matrix = read_text(&plan.join("TRD-ARCH-03B4-ibkr-test-matrix.md"));
    let historical_scenarios = matrix_titles(&matrix);
    assert_eq!(historical_scenarios.len(), 43);
    let scenarios = index["scenarios"].as_array().expect("scenarios");
    assert_eq!(scenarios.len(), 43);
    let scenario_note =
        read_text(&plan.join("TRD-ARCH-03B5-scenario-coverage-and-oracle-limits.md"));
    let scenario_rows = table_rows(&scenario_note, "IBKR-SCN-");
    assert_eq!(scenario_rows.len(), 43);
    let mut seen_scenarios = BTreeSet::new();
    for ((historical_id, historical_title), scenario) in historical_scenarios.iter().zip(scenarios)
    {
        let id = scenario["id"].as_str().expect("scenario id");
        assert_eq!(id, historical_id);
        assert!(seen_scenarios.insert(id.to_string()), "duplicate {id}");
        assert_eq!(scenario["title"], historical_title.as_str());
        assert_eq!(scenario["documentation_class"], "UNVERIFIED_RESEARCH_TOPIC");
        assert_eq!(scenario["official_source_id"], "NOT_ESTABLISHED");
        assert_eq!(scenario["section_anchor"], "NOT_ESTABLISHED");
        assert_eq!(scenario["execution"], "NOT_EXECUTED");
        assert_eq!(scenario["evidence"], "EXTERNAL_EVIDENCE_PENDING");
        assert_eq!(scenario["authorization"], "NOT_AUTHORIZED");
        let oracle = scenario["oracle_limit"].as_str().expect("oracle");
        assert!(scenario_note.contains(&format!("`{oracle}`")));
        let claim_ids = scenario["claims"].as_array().expect("scenario claims");
        assert!(!claim_ids.is_empty(), "{id}");
        for claim_id in claim_ids {
            assert!(
                seen_claims.contains(claim_id.as_str().expect("claim")),
                "{id}"
            );
        }
        let row = scenario_rows
            .iter()
            .find(|cells| cells[0] == id)
            .unwrap_or_else(|| panic!("scenario note missing {id}"));
        assert_eq!(row.len(), 8, "{id}");
        assert_eq!(row[1], historical_title.as_str());
        assert_eq!(unquote(row[3]), "NOT_ESTABLISHED");
        assert_eq!(unquote(row[4]), "UNVERIFIED_RESEARCH_TOPIC");
        assert_eq!(unquote(row[5]), oracle);
        assert_eq!(unquote(row[6]), "NOT_EXECUTED");
        assert_eq!(unquote(row[7]), "EXTERNAL_EVIDENCE_PENDING");
    }
    assert_eq!(seen_scenarios.len(), 43);
    for marker in [
        "## Unknown send",
        "## Snapshot gaps",
        "## Corrections and busts",
        "## Reconnect",
        "## Writer isolation",
        "IBKR-SCN-C01",
        "IBKR-SCN-A10",
        "IBKR-SCN-C08",
    ] {
        assert!(scenario_note.contains(marker), "missing {marker}");
    }
    assert!(!scenario_note.contains("http://"));
    assert!(!scenario_note.contains("https://"));

    let register = read_text(&plan.join("TRD-ARCH-03B5-official-source-register.md"));
    assert!(register.contains("**Obtained sources:** 0."));
    assert!(register.contains("NOT_RETRIEVED"));
    for topic in &seen_topics {
        assert!(
            register.contains(&format!("| {topic} |")),
            "register missing {topic}"
        );
    }
    let reads = read_text(&plan.join("TRD-ARCH-03B5-api-read-candidate-review.md"));
    assert!(reads.contains("`NO_READS_APPROVED`"));
    assert!(reads.contains("Count: 0."));
    assert!(reads.contains("E2"));
    let versions = read_text(&plan.join("TRD-ARCH-03B5-version-and-environment-findings.md"));
    assert!(versions.contains("`UNRECORDED`"));
    assert!(versions.contains("ARCH-DEC-52"));
    assert!(versions.contains("No paper, sandbox, or other environment is selected."));
    let gaps = read_text(&plan.join("TRD-ARCH-03B5-evidence-gaps-and-next-approval-request.md"));
    assert!(gaps.contains("This file is not a ratification."));
    assert!(gaps.contains("ARCH-DEC-63"));
    assert!(gaps.contains("`PARTIAL`"));
    let boundary = read_text(&plan.join("TRD-ARCH-03B5-public-research-authority-and-boundary.md"));
    assert!(boundary.contains("External fetch:** `NOT_PERFORMED`"));
    assert!(boundary.contains("does not name that go"));
    assert!(boundary.contains("Concrete hostnames are `UNRECORDED`."));
    for text in [&register, &reads, &versions, &gaps, &boundary] {
        assert!(!text.contains("http://"), "url scheme");
        assert!(!text.contains("https://"), "url scheme");
    }

    let index_text = String::from_utf8(index_bytes).expect("index utf-8");
    assert!(!index_text.contains("OFFICIAL_DOCUMENTATION_OBTAINED"));
    assert!(!index_text.contains("SECOND_COPY_PENDING"));
    assert!(!index_text.contains("http://"));
    assert!(!index_text.contains("https://"));
    assert!(!index_text.contains("raw_bytes"));
    assert!(!index_text.contains("api_key"));
    assert!(!index_text.contains("password"));

    let prompt =
        read_text(&root.join(
            "docs/cursor-prompts/trading/TRD-ARCH-03B5-IBKR-Official-Documentation-Research.md",
        ));
    assert!(prompt.contains("No signature artifact and no evidence URI are attached"));
    assert!(prompt.contains("keine pauschale E0-Stage-GO-Freigabe"));
    assert!(!prompt.contains("http://"));
    assert!(!prompt.contains("https://"));
    let report = read_text(&root.join("docs/architecture/TRD-ARCH-03B.5-report.md"));
    assert!(report.contains("VERDICT: PARTIAL"));
    assert!(report.contains("PUSHED: NO"));
    assert!(report.contains("ac60bf932355660548bf5751963009ea956b76a5"));
    assert!(report.contains("**Remote CI:** `NOT_RUN`"));
    assert!(report.contains("external_broker_tests_executed=false"));
}

#[test]
fn ibkr_public_research_leaves_protected_bytes_unchanged() {
    let root = repo_root();
    let unchanged = [
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
            "contracts/decisions/ARCH-DEC-49-bis-62-2026-11-15.md",
            "5400a7d65d30a144f5369e8683533087aa63d3d06775f0b566f0cb3f61904648",
            51486,
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
    let superseded: Vec<&Value> = entries
        .iter()
        .filter(|entry| entry["lifecycle"] == "SUPERSEDED")
        .collect();
    assert_eq!(superseded.len(), 1);
    assert_eq!(superseded[0]["id"], "ADFR-CROSS-005");
    assert_eq!(superseded[0]["superseded_by"], "ADFR-CR-B10-001");

    let registry =
        fs::read_to_string(root.join("contracts/registry/registry.json")).expect("registry");
    assert!(!registry.contains("TRD-ARCH-03B5"));
    assert!(!registry.contains("\"live_trading_authorized\": true"));
    let decisions = fs::read_dir(root.join("contracts/decisions")).expect("decisions");
    let names = decisions
        .map(|item| {
            item.expect("entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect::<Vec<_>>();
    assert_eq!(names.len(), 6);
    assert!(!names.iter().any(|name| name.contains("ARCH-DEC-63")));
}
