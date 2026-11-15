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

/// Non-schema JSON kept under `contracts/`.
///
/// The first three paths are the registry skeleton, provenance, and the ADFR
/// extract. The status draft is an interpretation file. None of them is a
/// published event, command, or API schema.
const ALLOWED_CONTRACT_JSON: &[&str] = &[
    "contracts/design-proposals/TRD-ARCH-03B1-proposal-index.json",
    "contracts/design-proposals/TRD-ARCH-03B12-resolution-index.json",
    "contracts/design-proposals/TRD-ARCH-03B2-proposal-index.json",
    "contracts/design-proposals/TRD-ARCH-03B22-resolution-index.json",
    "contracts/design-proposals/TRD-ARCH-03B3-proposal-index.json",
    "contracts/registry/registry.json",
    "contracts/source-manifest.json",
    "contracts/sources/ADFR-Consolidated-v1.0.json",
    "contracts/status-model/status-dimensions-draft-v0.json",
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
    ("trading_common::CorrelationId", "absent_type"),
    ("trading_common::HttpRequestId", "http_handle"),
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
    ("trading_domain::BrokerClientId", "identity"),
    ("trading_domain::AdapterSessionEpoch", "identity"),
    ("trading_domain::ExternalAliasKind", "identity"),
    ("trading_domain::IdentityEvidenceId", "identity"),
    ("trading_domain::BrokerOrderAlias", "identity"),
    ("trading_domain::BrokerPermIdAlias", "identity"),
    ("trading_domain::ExecutionIdentity", "identity"),
    ("trading_domain::QuarantinedExecutionIdentity", "identity"),
    ("trading_domain::BrokerIdentityContext", "identity"),
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
    ("trading_domain::RoundingPolicy", "value"),
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

    let http = find(entries, "trading_common::HttpRequestId");
    assert!(http.note.contains("x-correlation-id"));
    let retired = find(entries, "trading_common::CorrelationId");
    assert!(retired.note.contains("not a second identifier"));
    let rounding = find(entries, "trading_domain::RoundingPolicy");
    assert!(rounding.note.contains("not applied"));
    let context = find(entries, "trading_domain::BrokerIdentityContext");
    assert!(context.note.contains("EVIDENCE_PENDING"));
    assert!(context.note.contains("Not a verified"));
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
    let mut relative: Vec<String> = json_files
        .iter()
        .map(|path| contract_relative(root, path))
        .collect();
    relative.sort();
    let mut allowed: Vec<String> = ALLOWED_CONTRACT_JSON
        .iter()
        .map(|path| (*path).to_string())
        .collect();
    allowed.sort();
    assert_eq!(
        relative, allowed,
        "contracts JSON is limited to the registry skeleton, provenance, the ADFR extract, the non-wire status draft, the unratified OMS proposal index, the non-wire OMS policy resolution index, the unratified risk proposal index, the non-wire risk policy resolution index, and the unratified broker proposal index"
    );
    let proposal = fs::read_to_string(
        root.join("contracts/design-proposals/TRD-ARCH-03B1-proposal-index.json"),
    )
    .expect("OMS proposal index");
    assert!(proposal.contains("\"ratification\": \"NOT_RATIFIED\""));
    assert!(proposal.contains("\"xc_03_matrix\": \"OPEN\""));
    assert!(proposal.contains("\"order_closure_vocabulary\": \"NOT_APPROVED\""));
    assert!(proposal.contains("\"transition_edges\": \"NOT_APPROVED\""));
    assert!(proposal.contains("\"live_trading_authorized\": false"));
    assert!(proposal.contains("\"canonical_schemas_published\": false"));
    assert!(!proposal.contains("\"live_trading_authorized\": true"));
    assert!(!proposal.contains("\"canonical_schemas_published\": true"));

    let resolution = fs::read_to_string(
        root.join("contracts/design-proposals/TRD-ARCH-03B12-resolution-index.json"),
    )
    .expect("OMS policy resolution index");
    assert!(resolution.contains("\"document_role\": \"NON_RUNTIME_NON_WIRE_RESOLUTION_INDEX\""));
    assert!(resolution.contains("\"record_label\": \"RATIFIED_DESIGN_POLICY_ONLY\""));
    assert!(resolution.contains("\"implementation\": \"NOT_IMPLEMENTED\""));
    assert!(resolution.contains("\"verification\": \"NOT_VERIFIED\""));
    assert!(resolution.contains("\"record_labels_are_not_a_canonical_enum\": true"));
    assert!(resolution.contains("\"order_closure_enum_approved\": false"));
    assert!(resolution.contains("\"transition_edges\": \"NOT_APPROVED\""));
    assert!(resolution.contains("\"matrix_version_approved\": false"));
    assert!(resolution.contains("\"economic_finality_predicate_approved\": false"));
    assert!(resolution.contains("\"automatic_economic_posting_approved\": false"));
    assert!(resolution.contains("\"risk_release_approved\": false"));
    assert!(resolution.contains("\"broker_terminal_acceptance_approved\": false"));
    assert!(resolution.contains("\"modify_or_replace_dispatch_approved\": false"));
    assert!(resolution.contains("\"implementation_authorized\": false"));
    assert!(resolution.contains("\"closes_xc_findings\": false"));
    assert!(resolution.contains("\"external_evidence_claimed\": false"));
    assert!(resolution.contains("\"xc_03_matrix\": \"OPEN\""));
    assert!(resolution.contains("\"part_b\": \"NO-GO\""));
    assert!(resolution.contains("\"registry_status_required\": \"SKELETON_NOT_CANONICAL\""));
    assert!(resolution.contains("\"live_trading_authorized\": false"));
    assert!(resolution.contains("\"canonical_schemas_published\": false"));
    assert!(resolution.contains("\"canonical_schema_published_by_this_index\": false"));
    assert!(!resolution.contains("\"live_trading_authorized\": true"));
    assert!(!resolution.contains("\"canonical_schemas_published\": true"));
    assert!(!resolution.contains("\"order_closure_enum_approved\": true"));
    assert!(!resolution.contains("\"risk_release_approved\": true"));

    let risk = fs::read_to_string(
        root.join("contracts/design-proposals/TRD-ARCH-03B2-proposal-index.json"),
    )
    .expect("risk proposal index");
    assert!(risk.contains("\"document_role\": \"NON_RUNTIME_NON_WIRE_PROPOSAL_INDEX\""));
    assert!(risk.contains("\"ratification\": \"NOT_RATIFIED\""));
    assert!(risk.contains("\"not_a_schema\": true"));
    assert!(risk.contains("\"not_registry_json\": true"));
    assert!(risk.contains("\"risk_release_approved\": false"));
    assert!(risk.contains("\"economic_posting_approved\": false"));
    assert!(risk.contains("\"economic_finality_predicate_approved\": false"));
    assert!(risk.contains("\"implementation_authorized\": false"));
    assert!(risk.contains("\"closes_xc_findings\": false"));
    assert!(risk.contains("\"order_closure_enum_approved\": false"));
    assert!(risk.contains("\"xc_02_pred\": \"OPEN\""));
    assert!(risk.contains("\"xc_03_matrix\": \"OPEN\""));
    assert!(risk.contains("\"part_b\": \"NO-GO\""));
    assert!(risk.contains("\"registry_status_required\": \"SKELETON_NOT_CANONICAL\""));
    assert!(risk.contains("\"live_trading_authorized\": false"));
    assert!(risk.contains("\"canonical_schemas_published\": false"));
    assert!(risk.contains("\"canonical_schema_published_by_this_index\": false"));
    assert!(!risk.contains("\"live_trading_authorized\": true"));
    assert!(!risk.contains("\"canonical_schemas_published\": true"));
    assert!(!risk.contains("\"risk_release_approved\": true"));
    assert!(!risk.contains("\"economic_posting_approved\": true"));
    assert!(!risk.contains("\"economic_finality_predicate_approved\": true"));

    let risk_policy = fs::read_to_string(
        root.join("contracts/design-proposals/TRD-ARCH-03B22-resolution-index.json"),
    )
    .expect("risk policy resolution index");
    assert!(risk_policy.contains("\"document_role\": \"NON_RUNTIME_NON_WIRE_RESOLUTION_INDEX\""));
    assert!(!risk_policy.contains("\"document_role\": \"NON_RUNTIME_NON_WIRE_PROPOSAL_INDEX\""));
    assert!(risk_policy.contains("\"task\": \"TRD-ARCH-03B.2.2\""));
    assert!(risk_policy.contains("\"record_label\": \"RATIFIED_DESIGN_POLICY_ONLY\""));
    assert!(risk_policy.contains("\"implementation\": \"NOT_IMPLEMENTED\""));
    assert!(risk_policy.contains("\"verification\": \"NOT_VERIFIED\""));
    assert!(risk_policy.contains("\"record_labels_are_not_a_canonical_enum\": true"));
    assert!(risk_policy.contains("\"not_a_schema\": true"));
    assert!(risk_policy.contains("\"not_registry_json\": true"));
    assert!(risk_policy.contains("\"not_the_historical_proposal_index\": true"));
    assert!(risk_policy.contains("\"implementation_authorized\": false"));
    assert!(risk_policy.contains("\"risk_release_approved\": false"));
    assert!(risk_policy.contains("\"economic_posting_approved\": false"));
    assert!(risk_policy.contains("\"economic_finality_predicate_approved\": false"));
    assert!(risk_policy.contains("\"oms_transitions_approved\": false"));
    assert!(risk_policy.contains("\"broker_dispatch_approved\": false"));
    assert!(risk_policy.contains("\"closes_xc_findings\": false"));
    assert!(risk_policy.contains("\"tests_executed\": false"));
    assert!(risk_policy.contains("\"xc_02_pred\": \"OPEN\""));
    assert!(risk_policy.contains("\"part_b\": \"NO-GO\""));
    assert!(risk_policy.contains("\"live_trading_authorized\": false"));
    assert!(risk_policy.contains("\"canonical_schemas_published\": false"));
    assert!(risk_policy.contains("\"canonical_schema_published_by_this_index\": false"));
    assert!(!risk_policy.contains("\"live_trading_authorized\": true"));
    assert!(!risk_policy.contains("\"canonical_schemas_published\": true"));
    assert!(!risk_policy.contains("\"risk_release_approved\": true"));
    assert!(!risk_policy.contains("\"economic_posting_approved\": true"));
    assert!(!risk_policy.contains("\"oms_transitions_approved\": true"));
    assert!(!risk_policy.contains("\"broker_dispatch_approved\": true"));
    assert!(!risk_policy.contains("\"tests_executed\": true"));

    let broker = fs::read_to_string(
        root.join("contracts/design-proposals/TRD-ARCH-03B3-proposal-index.json"),
    )
    .expect("broker proposal index");
    assert!(broker.contains("\"document_role\": \"NON_RUNTIME_NON_WIRE_PROPOSAL_INDEX\""));
    assert!(broker.contains("\"ratification\": \"NOT_RATIFIED\""));
    assert!(broker.contains("\"not_a_schema\": true"));
    assert!(broker.contains("\"not_registry_json\": true"));
    assert!(broker.contains("\"implementation_authorized\": false"));
    assert!(broker.contains("\"broker_dispatch_authorized\": false"));
    assert!(broker.contains("\"economic_posting_authorized\": false"));
    assert!(broker.contains("\"risk_release_authorized\": false"));
    assert!(broker.contains("\"economic_finality_predicate_approved\": false"));
    assert!(broker.contains("\"oms_transitions_approved\": false"));
    assert!(broker.contains("\"canonical_broker_identity_tuple_approved\": false"));
    assert!(broker.contains("\"uniqueness_index_approved\": false"));
    assert!(broker.contains("\"identity_resolver_implemented\": false"));
    assert!(broker.contains("\"closes_xc_findings\": false"));
    assert!(broker.contains("\"verification_executed\": false"));
    assert!(broker.contains("\"external_broker_requests_executed\": false"));
    assert!(broker.contains("\"xc_01\": \"OPEN\""));
    assert!(broker.contains("\"xc_08_ibkr\": \"OPEN\""));
    assert!(broker.contains("\"xc_02_pred\": \"OPEN\""));
    assert!(broker.contains("\"xc_04_remainder\": \"OPEN\""));
    assert!(broker.contains("\"part_b\": \"NO-GO\""));
    assert!(broker.contains("\"registry_status_required\": \"SKELETON_NOT_CANONICAL\""));
    assert!(broker.contains("\"live_trading_authorized\": false"));
    assert!(broker.contains("\"canonical_schemas_published\": false"));
    assert!(broker.contains("\"canonical_schema_published_by_this_index\": false"));
    assert!(!broker.contains("\"live_trading_authorized\": true"));
    assert!(!broker.contains("\"canonical_schemas_published\": true"));
    assert!(!broker.contains("\"implementation_authorized\": true"));
    assert!(!broker.contains("\"broker_dispatch_authorized\": true"));
    assert!(!broker.contains("\"economic_posting_authorized\": true"));
    assert!(!broker.contains("\"risk_release_authorized\": true"));
    assert!(!broker.contains("\"verification_executed\": true"));
    assert!(!broker.contains("\"external_broker_requests_executed\": true"));

    let manifest = fs::read_to_string(root.join("contracts/source-manifest.json"))
        .expect("read contracts/source-manifest.json");
    assert!(
        manifest.contains("\"approved\": false"),
        "source manifest must not mark a source approved"
    );
    assert!(
        !manifest.contains("\"live_trading_authorized\": true"),
        "source manifest must not authorize live trading"
    );
    assert!(
        !manifest.contains("\"canonical_schemas_published\": true"),
        "source manifest must not publish canonical schemas"
    );

    let adfr = fs::read_to_string(root.join("contracts/sources/ADFR-Consolidated-v1.0.json"))
        .expect("read ADFR JSON");
    assert!(
        adfr.contains("ARCHITECTURE_DRAFT_NOT_IMPLEMENTED"),
        "ADFR JSON presence is not implementation"
    );
    assert!(
        !adfr.contains("\"canonical_wire\": true"),
        "ADFR JSON is not a canonical wire schema"
    );

    let status_draft =
        fs::read_to_string(root.join("contracts/status-model/status-dimensions-draft-v0.json"))
            .expect("read status draft");
    assert!(
        status_draft.contains("\"document_role\": \"NON_RUNTIME_NON_WIRE_DRAFT\""),
        "status draft must stay a non-wire document"
    );
    assert!(
        status_draft.contains("\"live_trading_authorized\": false"),
        "status draft must not authorize live trading"
    );
    assert!(
        !status_draft.contains("\"live_trading_authorized\": true"),
        "status draft must not authorize live trading"
    );
    assert!(
        !status_draft.contains("\"canonical_schemas_published\": true"),
        "status draft must not publish canonical schemas"
    );
}

fn contract_relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or_else(|_| panic!("path outside repo: {}", path.display()))
        .components()
        .map(|component| component.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join("/")
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
