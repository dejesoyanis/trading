//! Fail-closed checks for the M0-02B-a schema registry skeleton.
//!
//! These checks are not B10-T01 through B10-T12. They reject a published
//! schema, a verified broker-uniqueness claim, a public `CorrelationId`, and
//! a transition flag on this skeleton.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Registry {
    milestone: String,
    registry_status: String,
    canonical_schemas_published: bool,
    live_trading_authorized: bool,
    broker_uniqueness_evidence: String,
    part_b: String,
    entries: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Entry {
    name: String,
    kind: String,
    owner: String,
    status: String,
    source_contract: String,
    source_section: String,
    schema_path: Option<String>,
    canonical_wire: bool,
    public_schema: bool,
    transitions_implemented: bool,
    uniqueness_verified: bool,
    note: String,
}

const ALLOWED_SOURCES: &[&str] = &[
    "CR-B10-Domain-Glossary-and-Schema-Registry-v1.0.md",
    "M0-02A-UD-01-bis-UD-09-Architecture-Resolution-v1.0.md",
];

const FORBIDDEN_DIRS: &[&str] = &[
    "contracts/events",
    "contracts/commands",
    "contracts/api",
    "contracts/sql",
    "contracts/examples",
];

const EXPECTED: &[(&str, &str)] = &[
    ("trading_domain::OrderId", "identifier"),
    ("trading_domain::CommandId", "identifier"),
    ("trading_domain::EventId", "identifier"),
    ("trading_domain::CorrelationId", "identifier"),
    ("trading_domain::CausationId", "identifier"),
    ("trading_domain::BrokerObservationId", "identifier"),
    ("trading_domain::BrokerSessionId", "identifier"),
    ("trading_domain::SubmissionAttemptId", "identifier"),
    ("trading_domain::RiskReservationId", "identifier"),
    ("trading_domain::RiskEvaluationId", "identifier"),
    ("trading_domain::ReconciliationRunId", "identifier"),
    ("trading_domain::IncidentId", "identifier"),
    ("trading_domain::MarketObservationId", "identifier"),
    ("trading_domain::MarketSnapshotId", "identifier"),
    ("trading_domain::ValuationSnapshotId", "identifier"),
    ("trading_domain::AuditRecordId", "identifier"),
    ("trading_domain::ExecutionCorrectionId", "identifier"),
    ("trading_common::CorrelationId", "http_handle"),
    ("trading_common::HttpRequestId", "absent_type"),
    ("trading_domain::OrderVersion", "version"),
    ("trading_domain::WriterEpoch", "version"),
    ("trading_domain::SchemaVersion", "version"),
    ("trading_domain::RiskPolicyVersion", "version"),
    ("trading_domain::NormalizerVersion", "version"),
    ("trading_domain::InstrumentDefinitionVersion", "version"),
    ("trading_domain::ValuationModelVersion", "version"),
    ("trading_domain::BrokerName", "identity"),
    ("trading_domain::BrokerAccountCode", "identity"),
    ("trading_domain::BrokerEnvironmentLabel", "identity"),
    ("trading_domain::BrokerAccountKey", "identity"),
    ("trading_domain::UnverifiedSessionScope", "identity"),
    ("trading_domain::BrokerAliasScope", "identity"),
    ("trading_domain::ExternalBrokerId", "identity"),
    ("trading_domain::BrokerOrderAlias", "identity"),
    ("trading_domain::BrokerPermIdAlias", "identity"),
    ("trading_domain::ExecutionIdentity", "identity"),
    ("trading_domain::QuarantinedExecutionIdentity", "identity"),
    ("BrokerIdentityContext", "absent_type"),
    ("trading_domain::OrderWorkflowState", "state_label"),
    ("trading_domain::BrokerLifecycleState", "state_label"),
    ("trading_domain::ExecutionState", "state_label"),
    ("trading_domain::RiskReservationState", "state_label"),
    ("trading_domain::TradingReadinessState", "state_label"),
    ("trading_domain::ReconciliationRunState", "state_label"),
    ("trading_domain::RiskDecision", "state_label"),
    ("trading_domain::DispatchOutcome", "state_label"),
    ("RiskReservationAdjusted", "event_name"),
    ("trading_domain::DecimalAmount", "value"),
    ("trading_domain::CurrencyCode", "value"),
    ("trading_domain::InstrumentId", "value"),
    ("trading_domain::Money", "value"),
    ("trading_domain::Price", "value"),
    ("trading_domain::Quantity", "value"),
    ("trading_domain::OccurredAt", "value"),
    ("trading_domain::ReceivedAt", "value"),
    ("trading_domain::PersistedAt", "value"),
    ("RoundingPolicy", "absent_type"),
    ("ClockQuality", "absent_type"),
    ("TimestampProvenance", "absent_type"),
    ("OrderClosure", "absent_type"),
    ("IncidentState", "absent_type"),
    ("trading_domain::UnknownState", "error"),
    ("trading_domain::FinancialGap", "error"),
    ("trading_domain::TimeGap", "error"),
    ("trading_domain::IdParseError", "error"),
    ("trading_domain::IntegerParseError", "error"),
    ("trading_domain::TokenError", "error"),
    ("trading_domain::VersionOverflow", "error"),
    ("trading_domain::DecimalError", "error"),
    ("trading_domain::MoneyError", "error"),
    ("trading_domain::PriceError", "error"),
    ("trading_domain::QuantityError", "error"),
    ("trading_domain::TimeError", "error"),
    ("trading_domain::IdentityParseError", "error"),
    (
        "trading_domain::ORDER_STATE_MACHINE_IMPLEMENTED",
        "closed_flag",
    ),
    (
        "trading_domain::STATE_TRANSITIONS_IMPLEMENTED",
        "closed_flag",
    ),
    (
        "trading_domain::IDENTITY_RESOLVER_IMPLEMENTED",
        "closed_flag",
    ),
    (
        "trading_domain::FINANCIAL_ROUNDING_IMPLEMENTED",
        "closed_flag",
    ),
    ("trading_domain::BROKER_DISPATCH_IMPLEMENTED", "closed_flag"),
    ("trading_domain::RISK_EVALUATION_IMPLEMENTED", "closed_flag"),
];

#[test]
fn registry_skeleton_stays_unpublished() {
    let root = repo_root();
    let registry = load_registry(&root);
    assert_registry_closed(&registry);
    assert_inventory(&registry.entries);
    assert_entries_fail_closed(&registry.entries);
    assert_sources_exist(&root);
    assert_index_does_not_claim_review(&root);
    assert_no_published_schema_tree(&root);
    assert_domain_flags_stay_closed();
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn load_registry(root: &Path) -> Registry {
    let path = root.join("contracts/registry/registry.json");
    let text = fs::read_to_string(&path).unwrap_or_else(|error| {
        panic!("read {}: {error}", path.display());
    });
    serde_json::from_str(&text).unwrap_or_else(|error| {
        panic!("parse {}: {error}", path.display());
    })
}

fn assert_registry_closed(registry: &Registry) {
    assert_eq!(registry.milestone, "M0-02B-a");
    assert_eq!(registry.registry_status, "SKELETON_NOT_CANONICAL");
    assert!(!registry.canonical_schemas_published);
    assert!(!registry.live_trading_authorized);
    assert_eq!(registry.broker_uniqueness_evidence, "EVIDENCE_PENDING");
    assert_eq!(registry.part_b, "NO-GO");
}

fn assert_inventory(entries: &[Entry]) {
    assert_eq!(entries.len(), EXPECTED.len());
    let mut seen = HashSet::new();
    for entry in entries {
        assert!(
            seen.insert(entry.name.clone()),
            "duplicate registry name {}",
            entry.name
        );
    }
    for (name, kind) in EXPECTED {
        let entry = entries
            .iter()
            .find(|entry| entry.name == *name)
            .unwrap_or_else(|| panic!("missing registry entry {name}"));
        assert_eq!(entry.kind, *kind, "{name}");
    }
}

fn assert_entries_fail_closed(entries: &[Entry]) {
    let public_correlation = entries
        .iter()
        .filter(|entry| entry.public_schema && entry.name.ends_with("CorrelationId"))
        .count();
    assert_eq!(public_correlation, 0);

    for entry in entries {
        assert!(!entry.name.is_empty(), "empty registry name");
        assert_ne!(
            entry.name, "CorrelationId",
            "an unqualified CorrelationId must not be exported"
        );
        assert!(!entry.owner.is_empty(), "{}", entry.name);
        assert!(!entry.note.is_empty(), "{}", entry.name);
        assert!(!entry.source_section.is_empty(), "{}", entry.name);
        assert!(
            ALLOWED_SOURCES.contains(&entry.source_contract.as_str()),
            "{} cites {}",
            entry.name,
            entry.source_contract
        );
        assert!(
            entry.schema_path.is_none(),
            "{} has a schema path",
            entry.name
        );
        assert!(!entry.canonical_wire, "{}", entry.name);
        assert!(!entry.public_schema, "{}", entry.name);
        assert!(!entry.transitions_implemented, "{}", entry.name);
        assert!(!entry.uniqueness_verified, "{}", entry.name);
        assert_eq!(entry.status, expected_status(&entry.kind), "{}", entry.name);
    }

    let scope = find(entries, "trading_domain::UnverifiedSessionScope");
    assert!(scope.note.contains("stays unverified"));
    assert!(scope.note.contains("EVIDENCE_PENDING"));

    let perm = find(entries, "trading_domain::BrokerPermIdAlias");
    assert!(perm.note.contains("not assumed"));
    assert!(perm.note.contains("EVIDENCE_PENDING"));

    let closure = find(entries, "OrderClosure");
    assert!(closure.note.contains("Not a canonical wire schema"));
}

fn expected_status(kind: &str) -> &'static str {
    match kind {
        "identifier" | "version" | "state_label" | "value" | "identity" | "error"
        | "closed_flag" | "http_handle" => "INTERNAL_DRAFT_SHELL",
        "absent_type" | "event_name" => "NOT_IMPLEMENTED",
        other => panic!("unknown registry kind {other}"),
    }
}

fn find<'a>(entries: &'a [Entry], name: &str) -> &'a Entry {
    entries
        .iter()
        .find(|entry| entry.name == name)
        .unwrap_or_else(|| panic!("missing registry entry {name}"))
}

fn assert_sources_exist(root: &Path) {
    for name in ALLOWED_SOURCES {
        let path = root.join("contracts/sources").join(name);
        assert!(path.is_file(), "missing source {}", path.display());
    }
}

fn assert_index_does_not_claim_review(root: &Path) {
    let path = root.join("contracts/traceability-index.md");
    let text = fs::read_to_string(&path).unwrap_or_else(|error| {
        panic!("read {}: {error}", path.display());
    });
    for required in [
        "NOT_PROVIDED",
        "NOT_REVIEWED",
        "EVIDENCE_PENDING",
        "CR-B10-Domain-Glossary-and-Schema-Registry-v1.0.md",
        "M0-02A-UD-01-bis-UD-09-Architecture-Resolution-v1.0.md",
        "DET-01",
        "CR-B09",
        "ADFR",
    ] {
        assert!(text.contains(required), "index missing {required}");
    }
}

fn assert_no_published_schema_tree(root: &Path) {
    for relative in FORBIDDEN_DIRS {
        let path = root.join(relative);
        assert!(
            !path.exists(),
            "published schema directory exists: {}",
            path.display()
        );
    }

    let mut json_files = Vec::new();
    collect_json(root.join("contracts"), &mut json_files);
    assert_eq!(json_files.len(), 1, "contracts JSON files: {json_files:?}");
    assert!(
        json_files[0].ends_with("registry.json"),
        "unexpected contracts JSON {}",
        json_files[0].display()
    );
}

fn collect_json(dir: PathBuf, found: &mut Vec<PathBuf>) {
    let entries = fs::read_dir(&dir).unwrap_or_else(|error| {
        panic!("read {}: {error}", dir.display());
    });
    for item in entries {
        let item = item.unwrap_or_else(|error| {
            panic!("read entry under {}: {error}", dir.display());
        });
        let path = item.path();
        if path.is_dir() {
            collect_json(path, found);
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
            found.push(path);
        }
    }
}

fn assert_domain_flags_stay_closed() {
    const {
        assert!(!trading_domain::ORDER_STATE_MACHINE_IMPLEMENTED);
        assert!(!trading_domain::STATE_TRANSITIONS_IMPLEMENTED);
        assert!(!trading_domain::IDENTITY_RESOLVER_IMPLEMENTED);
        assert!(!trading_domain::FINANCIAL_ROUNDING_IMPLEMENTED);
        assert!(!trading_domain::BROKER_DISPATCH_IMPLEMENTED);
        assert!(!trading_domain::RISK_EVALUATION_IMPLEMENTED);
    }
}
