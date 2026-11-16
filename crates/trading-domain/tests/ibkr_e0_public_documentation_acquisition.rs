//! Documentation checks for the TRD-ARCH-03B.5.1 public-documentation acquisition.
//!
//! A passing check is not an IBKR verification, not a broker read, and not a
//! uniqueness or completeness result.

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

fn read_bytes(path: &Path) -> Vec<u8> {
    fs::read(path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

fn read_json(path: &Path) -> Value {
    let bytes = read_bytes(path);
    assert!(
        !bytes.starts_with(&[0xEF, 0xBB, 0xBF]),
        "BOM in {}",
        path.display()
    );
    assert!(!bytes.contains(&b'\r'), "CR in {}", path.display());
    serde_json::from_slice(&bytes)
        .unwrap_or_else(|error| panic!("json {}: {error}", path.display()))
}

fn strings(value: &Value) -> Vec<&str> {
    value
        .as_array()
        .unwrap_or_else(|| panic!("expected array"))
        .iter()
        .map(|item| item.as_str().unwrap_or_else(|| panic!("expected string")))
        .collect()
}

const PROTECTED: &[(&str, &str, u64)] = &[
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
        "contracts/decisions/ARCH-DEC-49-bis-62-2026-11-15.md",
        "5400a7d65d30a144f5369e8683533087aa63d3d06775f0b566f0cb3f61904648",
        51486,
    ),
];

#[test]
fn limited_e0_public_documentation_acquisition_stays_narrow() {
    let root = repo_root();
    let plan = root.join("contracts/verification-plans");
    let auth = String::from_utf8(read_bytes(
        &plan.join("TRD-ARCH-03B5.1-limited-e0-authorization.md"),
    ))
    .expect("authorization utf-8");
    assert!(!auth.contains('\r'));
    assert!(auth.contains("2026-11-16"));
    assert!(auth.contains("GO_LIMITED_PUBLIC_DOCS_ONLY"));
    assert!(auth.contains("E1 offline fixtures and replay | STOP"));
    assert!(auth.contains("E2 broker reads | STOP"));
    assert!(auth.contains("E3 broker fault injection and order experiments | STOP"));
    assert!(auth.contains("E4 adjudication and ratification | STOP"));
    assert!(auth.contains("not a design-policy ratification"));
    assert!(!auth.contains("ARCH-DEC-63 —"));

    let allow = read_json(&plan.join("TRD-ARCH-03B5.1-host-allowlist.json"));
    assert_eq!(
        allow["document_role"],
        "NON_RUNTIME_NON_WIRE_HOST_ALLOWLIST"
    );
    assert_eq!(allow["scheme_allowed"], "https");
    assert_eq!(allow["automatic_redirect_following_allowed"], false);
    assert_eq!(allow["cookies_allowed"], false);
    assert_eq!(allow["tls_validation_bypass_allowed"], false);
    assert_eq!(
        allow["stage_e0_checkpoint_for_named_action"],
        "GO_LIMITED_PUBLIC_DOCS_ONLY"
    );
    assert_eq!(allow["stage_e0_general"], "STOP");
    for stage in ["stage_e1", "stage_e2", "stage_e3", "stage_e4"] {
        assert_eq!(allow[stage], "STOP");
    }
    assert_eq!(allow["reads_approved"], "NO_READS_APPROVED");
    assert_eq!(allow["broker_connection_authorized"], false);
    assert_eq!(allow["live_trading_authorized"], false);
    let hosts = allow["hosts"].as_array().expect("hosts");
    assert!(!hosts.is_empty(), "allowlist has no verified host");
    for host in hosts {
        assert_eq!(host["scheme"], "https");
        let name = host["host"].as_str().unwrap();
        assert!(!name.contains('*'));
        assert!(!name.contains('/'));
        assert_ne!(name, "localhost");
        assert_eq!(host["path_scope"]["prefix"], "/docs/");
        assert_eq!(host["query_allowed"], false);
        assert_eq!(host["userinfo_allowed"], false);
    }
    let rejected = allow["rejected"].as_array().expect("rejected");
    let rejected_text = rejected
        .iter()
        .map(|item| item["url"].as_str().unwrap().to_string() + item["reason"].as_str().unwrap())
        .collect::<String>()
        .to_ascii_lowercase();
    assert!(rejected_text.contains("sso/login") || rejected_text.contains("login"));
    assert!(rejected_text.contains("localhost"));
    assert!(rejected_text.contains("api.ibkr.com"));
    assert!(rejected_text.contains("403"));
    assert_eq!(allow["candidates"][0]["status"], "HOST_UNVERIFIED");

    let acq = read_json(&plan.join("TRD-ARCH-03B5.1-acquisition-register.json"));
    assert_eq!(
        acq["document_role"],
        "NON_RUNTIME_NON_WIRE_ACQUISITION_REGISTER"
    );
    assert_eq!(acq["not_a_broker_raw_payload_store"], true);
    assert_eq!(acq["arch_dec_49_satisfied"], false);
    assert_eq!(acq["complete_source_pairs"], 0);
    assert_eq!(acq["second_copy_status"], "SECOND_COPY_PENDING");
    assert_eq!(acq["reads_approved"], "NO_READS_APPROVED");
    assert_eq!(acq["environment_selected"], false);
    assert_eq!(acq["version_tuple"], "UNRECORDED");
    assert_eq!(acq["external_broker_requests_executed"], false);
    let sources = acq["sources"].as_array().expect("sources");
    assert!(!sources.is_empty());
    assert_eq!(acq["obtained_source_count"], sources.len());
    let archive_present = root
        .join("research-archive/ibkr-public-docs/bodies")
        .is_dir();
    for source in sources {
        assert_eq!(source["obtained"], true);
        assert_eq!(source["arch_dec_49_pair"], false);
        assert_eq!(source["http_status"], 200);
        assert!(source["sha256"].as_str().unwrap().len() == 64);
        assert!(source["bytes"].as_u64().unwrap() > 0);
        let url = source["final_url"].as_str().unwrap();
        assert!(url.starts_with("https://www.interactivebrokers.com/docs"));
        assert!(!url.contains('@'));
        assert!(
            source["document_version"] == "UNRECORDED"
                || source["product_version_cited"].is_string()
        );
        let relative = source["archive_path"].as_str().unwrap();
        assert!(relative.starts_with("research-archive/"));
        if archive_present {
            let file = root.join(relative);
            let bytes = read_bytes(&file);
            assert_eq!(bytes.len() as u64, source["bytes"].as_u64().unwrap());
            assert_eq!(sha256_hex(&bytes), source["sha256"].as_str().unwrap());
        }
    }
    let second = &acq["same_url_second_retrievals"][0];
    assert_eq!(
        second["independence"],
        "TWO_GETS_SAME_URL_NOT_INDEPENDENT_SOURCE_CORROBORATION"
    );
    assert_eq!(second["arch_dec_49_satisfied"], false);
    assert_eq!(second["byte_comparison"], "BYTE_IDENTICAL_TO_PRIMARY");
    assert!(!acq["failed_retrievals"].as_array().unwrap().is_empty());

    let research = read_json(&plan.join("TRD-ARCH-03B5.1-research-register.json"));
    assert_eq!(
        research["document_role"],
        "NON_RUNTIME_NON_WIRE_RESEARCH_REGISTER"
    );
    assert_eq!(
        research["stage_e0_named_action"],
        "GO_LIMITED_PUBLIC_DOCS_ONLY"
    );
    assert_eq!(research["stage_e0_general"], "STOP");
    for stage in ["stage_e1", "stage_e2", "stage_e3", "stage_e4"] {
        assert_eq!(research[stage], "STOP");
    }
    assert_eq!(research["new_arch_dec_assigned"], false);
    assert_eq!(research["next_unused_decision_id_observed"], "ARCH-DEC-63");
    assert_eq!(research["closes_xc_findings"], false);
    assert_eq!(research["uniqueness_guaranteed"], false);
    assert_eq!(research["completeness_proof_approved"], false);
    assert_eq!(research["live_trading_authorized"], false);
    assert_eq!(research["broker_dispatch_approved"], false);
    assert_eq!(research["oms_transitions_approved"], false);
    assert_eq!(research["risk_release_approved"], false);
    assert_eq!(research["economic_posting_approved"], false);
    assert_eq!(research["external_broker_tests_executed"], false);
    let families = strings(&research["api_families_not_merged"]);
    assert_eq!(
        families,
        vec!["TWS_API", "IB_GATEWAY", "CLIENT_PORTAL_WEB_API"]
    );

    let claims = research["claims"].as_array().expect("claims");
    assert_eq!(claims.len(), 27);
    let mut claim_ids = BTreeSet::new();
    for claim in claims {
        let id = claim["id"].as_str().unwrap();
        assert!(claim_ids.insert(id.to_string()), "duplicate {id}");
        assert_eq!(claim["evidence"], "EXTERNAL_EVIDENCE_PENDING");
        assert_eq!(claim["execution"], "NOT_EXECUTED");
        assert_eq!(claim["empirical_upgrade"], false);
        let class = claim["documentation_class"].as_str().unwrap();
        assert!(class == "OFFICIAL_DOCUMENTATION_OBTAINED" || class == "NOT_RETRIEVED");
        let hypothesis = claim["hypothesis_status"].as_str().unwrap();
        assert!(
            hypothesis == "DOCUMENTED_CLAIM_NOT_EMPIRICALLY_VERIFIED" || hypothesis == "HYPOTHESIS"
        );
        if hypothesis == "DOCUMENTED_CLAIM_NOT_EMPIRICALLY_VERIFIED" {
            let quote = claim["quotation"].as_str().unwrap();
            assert!(!quote.is_empty(), "{id} documented without a quotation");
            if archive_present {
                assert!(
                    quotation_is_in_cited_archive(&root, &acq, claim, quote),
                    "{id} quotation is not in the cited archive"
                );
            }
        }
    }
    assert_eq!(claim_ids.len(), 27);

    let scenarios = research["scenarios"].as_array().expect("scenarios");
    assert_eq!(scenarios.len(), 43);
    let mut scenario_ids = BTreeSet::new();
    for scenario in scenarios {
        let id = scenario["id"].as_str().unwrap();
        assert!(scenario_ids.insert(id.to_string()), "duplicate {id}");
        assert_eq!(scenario["execution"], "NOT_EXECUTED");
        assert_eq!(scenario["evidence"], "EXTERNAL_EVIDENCE_PENDING");
        assert!(!scenario["oracle_limit"].as_str().unwrap().is_empty());
    }
    assert_eq!(scenario_ids.len(), 43);

    let reads = research["read_candidates"].as_array().expect("reads");
    assert!(!reads.is_empty());
    for read in reads {
        assert_eq!(read["approved"], false);
        assert_eq!(read["read_only"], false);
    }
    assert!(
        reads
            .iter()
            .any(|read| read["name"] == "EClient.cancelOrder")
    );
    assert!(
        reads
            .iter()
            .any(|read| read["name"] == "EClient.placeOrder")
    );

    let gitignore = String::from_utf8(read_bytes(&root.join(".gitignore"))).unwrap();
    assert!(gitignore.contains("/research-archive/"));
    let registry =
        String::from_utf8(read_bytes(&root.join("contracts/registry/registry.json"))).unwrap();
    assert!(!registry.contains("TRD-ARCH-03B5.1"));
    let protocol = String::from_utf8(read_bytes(
        &plan.join("TRD-ARCH-03B4-ibkr-verification-protocol.md"),
    ))
    .unwrap();
    assert!(protocol.contains("| E0 |"));
    assert!(protocol.contains("| STOP | `NOT_AUTHORIZED` |"));
    for (relative, expected_hash, expected_len) in PROTECTED {
        let bytes = read_bytes(&root.join(relative));
        assert_eq!(bytes.len() as u64, *expected_len, "{relative}");
        assert_eq!(sha256_hex(&bytes), *expected_hash, "{relative}");
    }
    let adfr = read_json(&root.join("contracts/sources/ADFR-Consolidated-v1.0.json"));
    let entries = adfr["entries"].as_array().expect("adfr");
    assert_eq!(entries.len(), 78);
    assert!(
        entries
            .iter()
            .all(|entry| entry["verification"] == "NOT_VERIFIED")
    );
}

fn quotation_is_in_cited_archive(
    root: &Path,
    acquisition: &Value,
    claim: &Value,
    quote: &str,
) -> bool {
    let sources = claim["source_ids"].as_array().unwrap();
    let archived = acquisition["sources"].as_array().unwrap();
    sources.iter().any(|source_id| {
        archived.iter().any(|source| {
            source["id"] == *source_id
                && read_bytes(&root.join(source["archive_path"].as_str().unwrap()))
                    .windows(quote.len())
                    .any(|window| window == quote.as_bytes())
        })
    })
}
