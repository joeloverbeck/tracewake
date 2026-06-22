mod parity;

use std::collections::BTreeSet;

use parity::{
    registry,
    runner::run_conformance_with_render_probe,
    runner::{registered_action_coverage_failures, run_conformance},
    CapabilityClass, EvidenceFlag, OwnershipScope, SetupOperation, SurfaceDisposition, WitnessKind,
};
use tracewake_content::fixtures;

#[test]
fn playable_capability_registry_smoke_test() {
    let entries = registry();
    assert!(!entries.is_empty(), "capability registry must not be empty");

    let keys = entries.iter().map(|entry| entry.key).collect::<Vec<_>>();
    let mut sorted_keys = keys.clone();
    sorted_keys.sort_unstable();
    assert_eq!(keys, sorted_keys, "capability keys must be deterministic");

    let unique_keys = keys.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(
        unique_keys.len(),
        keys.len(),
        "capability keys must be unique"
    );

    for entry in &entries {
        assert!(!entry.key.trim().is_empty());
        assert!(!entry.disposition_rationale.trim().is_empty());
        assert!(!entry.fixture_ids.is_empty());
        assert!(!entry.viewer_actor.trim().is_empty());
        assert!(!entry.typed_witness.assertion.trim().is_empty());
        assert!(matches!(
            entry.ownership_scope,
            OwnershipScope::Base | OwnershipScope::FuturePack { .. }
        ));
        assert!(matches!(
            entry.capability_class,
            CapabilityClass::SemanticAction
                | CapabilityClass::ActorObservableState
                | CapabilityClass::ActorObservableConsequence
                | CapabilityClass::NotebookRecordSurface
                | CapabilityClass::DebugOnlyInfrastructure
                | CapabilityClass::HeadlessInfrastructure
        ));
        assert!(matches!(
            entry.surface_disposition,
            SurfaceDisposition::Embodied
                | SurfaceDisposition::Notebook
                | SurfaceDisposition::DebugOnly
                | SurfaceDisposition::Headless
        ));
        assert!(matches!(
            entry.setup_operation,
            SetupOperation::BindViewer
                | SetupOperation::SubmitSemanticAction { .. }
                | SetupOperation::AdvanceNoHuman
        ));
        assert!(matches!(
            entry.typed_witness.kind,
            WitnessKind::TypedCausal
                | WitnessKind::ActorKnowledge
                | WitnessKind::RenderedText
                | WitnessKind::GoldenReference
                | WitnessKind::AntiLeakNegative
                | WitnessKind::DebugQuarantine
        ));
        assert!(matches!(
            entry.replay_evidence,
            EvidenceFlag::Required | EvidenceFlag::NotApplicable { .. }
        ));
        assert!(matches!(
            entry.no_human_evidence,
            EvidenceFlag::Required | EvidenceFlag::NotApplicable { .. }
        ));

        for fixture_id in &entry.fixture_ids {
            assert!(
                fixtures::by_id(fixture_id).is_some(),
                "registry entry {} references unknown fixture {fixture_id}",
                entry.key
            );
        }
        for fixture_id in &entry.anti_leak_fixtures {
            assert!(
                fixtures::by_id(fixture_id).is_some(),
                "registry entry {} references unknown anti-leak fixture {fixture_id}",
                entry.key
            );
        }
    }
}

#[test]
fn playable_capability_registry_stays_test_side_metadata() {
    let registry_source = include_str!("parity/mod.rs");
    for forbidden in [
        "apply_event",
        "append_event",
        "EventLog",
        "DeterministicScheduler",
        "validate_proposal",
        "commit_proposal",
        "cognition",
    ] {
        assert!(
            !registry_source.contains(forbidden),
            "capability registry must not consume authoritative simulation API: {forbidden}"
        );
    }
}

#[test]
fn playable_capability_registry_schema_exposes_all_closed_enum_variants() {
    let ownership_scopes = [
        OwnershipScope::Base,
        OwnershipScope::FuturePack {
            namespace: "future_pack",
        },
    ];
    assert_eq!(ownership_scopes.len(), 2);

    let classes = [
        CapabilityClass::SemanticAction,
        CapabilityClass::ActorObservableState,
        CapabilityClass::ActorObservableConsequence,
        CapabilityClass::NotebookRecordSurface,
        CapabilityClass::DebugOnlyInfrastructure,
        CapabilityClass::HeadlessInfrastructure,
    ];
    assert_eq!(classes.len(), 6);

    let dispositions = [
        SurfaceDisposition::Embodied,
        SurfaceDisposition::Notebook,
        SurfaceDisposition::DebugOnly,
        SurfaceDisposition::Headless,
    ];
    assert_eq!(dispositions.len(), 4);

    let operations = [
        SetupOperation::BindViewer,
        SetupOperation::SubmitSemanticAction {
            semantic_action_id: "wait.1_tick",
        },
        SetupOperation::AdvanceNoHuman,
    ];
    assert_eq!(operations.len(), 3);

    let witness_kinds = [
        WitnessKind::TypedCausal,
        WitnessKind::ActorKnowledge,
        WitnessKind::RenderedText,
        WitnessKind::GoldenReference,
        WitnessKind::AntiLeakNegative,
        WitnessKind::DebugQuarantine,
    ];
    assert_eq!(witness_kinds.len(), 6);
}

#[test]
fn playable_capability_runner_passes_live_registry_and_reports_deterministically() {
    let entries = registry();
    let first = run_conformance(&entries);
    let second = run_conformance(&entries);

    assert!(
        first.is_pass(),
        "live parity registry must pass conformance: {:#?}",
        first.failures
    );
    assert_eq!(
        first.to_deterministic_text(),
        second.to_deterministic_text()
    );
}

#[test]
fn playable_capability_runner_fails_closed_for_synthetic_gaps() {
    let complete = registry();
    assert!(run_conformance(&complete).is_pass());

    let mut duplicate = complete.clone();
    duplicate.push(duplicate[0].clone());
    assert_has_failure(&duplicate, "duplicate_key");

    let mut missing_fixture = complete[0].clone();
    missing_fixture.key = "synthetic.missing_fixture";
    missing_fixture.fixture_ids = Vec::new();
    assert_has_failure(&[missing_fixture], "missing_fixture");

    let mut unknown_fixture = complete[0].clone();
    unknown_fixture.key = "synthetic.unknown_fixture";
    unknown_fixture.fixture_ids = vec!["missing_fixture_999"];
    assert_has_failure(&[unknown_fixture], "unknown_fixture");

    let mut missing_rendered = complete[0].clone();
    missing_rendered.key = "synthetic.missing_rendered";
    missing_rendered.rendered_witness = None;
    assert_has_failure(&[missing_rendered], "missing_rendered_witness");

    let mut missing_anti_leak = complete[0].clone();
    missing_anti_leak.key = "synthetic.notebook.epistemic";
    missing_anti_leak.capability_class = CapabilityClass::NotebookRecordSurface;
    missing_anti_leak.anti_leak_fixtures = Vec::new();
    assert_has_failure(&[missing_anti_leak], "missing_anti_leak");

    let mut missing_action = complete[0].clone();
    missing_action.key = "synthetic.missing_action";
    missing_action.setup_operation = SetupOperation::SubmitSemanticAction {
        semantic_action_id: "missing.semantic.action",
    };
    assert_has_failure(&[missing_action], "declared_action_absent");

    let empty_render = run_conformance_with_render_probe(&complete, |_entry| Some(String::new()));
    assert!(
        empty_render
            .failures
            .iter()
            .any(|failure| failure.code == "empty_embodied_render"),
        "empty embodied render must fail: {:#?}",
        empty_render.failures
    );

    let uncovered_actions = registered_action_coverage_failures(&complete, ["wait.1_tick", "move"]);
    assert!(
        uncovered_actions
            .iter()
            .any(|failure| failure.code == "registered_action_uncovered"
                && failure.key.as_deref() == Some("move")),
        "missing registered action coverage must fail: {uncovered_actions:#?}"
    );
}

fn assert_has_failure(entries: &[parity::CapabilityEntry], code: &str) {
    let report = run_conformance(entries);
    assert!(
        report.failures.iter().any(|failure| failure.code == code),
        "expected failure {code}; got {:#?}",
        report.failures
    );
}
