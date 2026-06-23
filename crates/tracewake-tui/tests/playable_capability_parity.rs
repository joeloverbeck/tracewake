mod parity;

use std::collections::BTreeSet;

use parity::{
    census_actions::registered_action_ids,
    census_families::FAMILY_KEYS,
    registry,
    runner::run_conformance_with_measurement_probe,
    runner::run_conformance_with_render_probe,
    runner::{registered_action_coverage_failures, run_conformance},
    scenario::{
        assert_actor_surface_does_not_leak, assert_matches_checked_in_golden, run_real_pipeline,
        ScenarioMeasuredEvidence,
    },
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
        if let Some(registry_action_id) = entry.registry_action_id {
            assert!(!registry_action_id.trim().is_empty());
        }
        assert!(!entry.typed_witness.assertion.trim().is_empty());
        assert!(!entry.actor_knowledge_witness.assertion.trim().is_empty());
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
                | SetupOperation::HumanWaitOneTick
                | SetupOperation::StartSleepThenAdvanceUntil { .. }
                | SetupOperation::MoveWorkThenAdvanceUntil { .. }
                | SetupOperation::StartSleepThenWaitConflict
                | SetupOperation::SubmitSemanticAction { .. }
                | SetupOperation::SubmitRegistryAction { .. }
                | SetupOperation::ObserveQueryOnly { .. }
                | SetupOperation::AdvanceNoHuman
                | SetupOperation::RenderNotebook
                | SetupOperation::RenderDebugOverlay
                | SetupOperation::RunNoHumanDay
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
            entry.actor_knowledge_witness.kind,
            WitnessKind::ActorKnowledge
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
fn playable_capability_registry_includes_spec0047_time_control_pack() {
    let entries = registry();
    let base_count = entries
        .iter()
        .filter(|entry| matches!(entry.ownership_scope, OwnershipScope::Base))
        .count();
    let spec0047_keys = entries
        .iter()
        .filter(|entry| {
            matches!(
                entry.ownership_scope,
                OwnershipScope::FuturePack {
                    namespace: "spec0047_tui_authoritative_world_advance"
                }
            )
        })
        .map(|entry| entry.key)
        .collect::<Vec<_>>();

    assert_eq!(base_count, 21, "spec-0046 baseline entries must remain");
    assert_eq!(
        spec0047_keys,
        vec![
            "spec0047.time.actor_known_interval_summary",
            "spec0047.time.advance_until_stop_reason",
            "spec0047.time.human_sleep_terminal",
            "spec0047.time.human_wait_world_step",
            "spec0047.time.human_work_terminal",
            "spec0047.time.open_duration_wait_conflict",
        ]
    );
    assert_eq!(entries.len(), base_count + spec0047_keys.len());
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
        SetupOperation::HumanWaitOneTick,
        SetupOperation::StartSleepThenAdvanceUntil { max_ticks: 4 },
        SetupOperation::MoveWorkThenAdvanceUntil { max_ticks: 4 },
        SetupOperation::StartSleepThenWaitConflict,
        SetupOperation::SubmitSemanticAction {
            semantic_action_id: "wait.1_tick",
        },
        SetupOperation::SubmitRegistryAction { action_id: "wait" },
        SetupOperation::ObserveQueryOnly { action_id: "look" },
        SetupOperation::AdvanceNoHuman,
        SetupOperation::RenderNotebook,
        SetupOperation::RenderDebugOverlay,
        SetupOperation::RunNoHumanDay,
    ];
    assert_eq!(operations.len(), 12);

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
fn playable_capability_registry_covers_every_registered_action_definition() {
    let entries = registry();
    let action_ids = registered_action_ids();
    assert_eq!(
        action_ids,
        vec![
            "check_container",
            "close",
            "continue_routine",
            "eat",
            "inspect_entity",
            "inspect_place",
            "look",
            "move",
            "open",
            "place",
            "sleep",
            "take",
            "truthful_accuse_probe",
            "wait",
            "work_block",
        ],
        "current action registry count changed; update the census deliberately"
    );

    let failures = registered_action_coverage_failures(
        &entries,
        action_ids.iter().map(String::as_str).collect::<Vec<_>>(),
    );
    assert!(
        failures.is_empty(),
        "every registered action must have a capability disposition: {failures:#?}"
    );
}

#[test]
fn playable_capability_registry_covers_baseline_non_action_families() {
    let entries = registry();
    for family_key in FAMILY_KEYS {
        let entry = entries
            .iter()
            .find(|entry| entry.key == family_key)
            .unwrap_or_else(|| panic!("missing family capability entry {family_key}"));
        assert!(
            entry.registry_action_id.is_none(),
            "{family_key} must be a non-action family entry"
        );
        assert!(
            entry.rendered_witness.is_some(),
            "{family_key} must have a rendered/debug/notebook witness"
        );
    }
}

#[test]
fn playable_capability_scenarios_match_checked_in_real_pipeline_goldens() {
    let entries = registry();

    for entry in &entries {
        let witnesses = run_real_pipeline(entry)
            .unwrap_or_else(|error| panic!("{} scenario failed: {error:#?}", entry.key));
        assert_eq!(
            witnesses.ordered_witnesses.len(),
            3,
            "{} must assert typed -> actor-knowledge -> rendered witnesses",
            entry.key
        );
        assert_eq!(witnesses.ordered_witnesses[0], "typed_measured=true");
        assert_eq!(
            witnesses.ordered_witnesses[1],
            "actor_knowledge_measured=true"
        );
        if entry.rendered_witness.is_some() {
            assert_eq!(witnesses.ordered_witnesses[2], "rendered_measured=true");
        }
        assert!(
            witnesses.measured_evidence.typed,
            "{} must return measured typed evidence",
            entry.key
        );
        assert_matches_checked_in_golden(entry, &witnesses.rendered);
        assert_actor_surface_does_not_leak(entry, &witnesses.rendered);
    }
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

    let mut missing_actor_knowledge = complete[0].clone();
    missing_actor_knowledge.key = "synthetic.missing_actor_knowledge";
    missing_actor_knowledge.actor_knowledge_witness.assertion = "";
    assert_has_failure(
        &[missing_actor_knowledge],
        "missing_actor_knowledge_witness",
    );

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

    let uncovered_actions =
        registered_action_coverage_failures(&complete, ["wait", "synthetic_missing_action"]);
    assert!(
        uncovered_actions
            .iter()
            .any(|failure| failure.code == "registered_action_uncovered"
                && failure.key.as_deref() == Some("synthetic_missing_action")),
        "missing registered action coverage must fail: {uncovered_actions:#?}"
    );

    let human_wait = complete
        .iter()
        .find(|entry| entry.key == "spec0047.time.human_wait_world_step")
        .unwrap()
        .clone();
    let no_human = complete
        .iter()
        .find(|entry| entry.key == "base.family.no_human_autonomy")
        .unwrap()
        .clone();

    assert_has_failure_with_measurement(
        std::slice::from_ref(&human_wait),
        measured_evidence_with(|evidence| {
            evidence.typed = false;
        }),
        "missing_measured_typed_evidence",
    );
    assert_has_failure_with_measurement(
        std::slice::from_ref(&no_human),
        measured_evidence_with(|evidence| {
            evidence.autonomous_work = false;
        }),
        "missing_measured_no_human_work",
    );
    assert_has_failure_with_measurement(
        &[no_human],
        measured_evidence_with(|evidence| {
            evidence.marker_counted = false;
        }),
        "missing_measured_no_human_work",
    );
    assert_has_failure_with_measurement(
        std::slice::from_ref(&human_wait),
        measured_evidence_with(|evidence| {
            evidence.actor_knowledge = false;
        }),
        "missing_measured_actor_knowledge",
    );
    assert_has_failure_with_measurement(
        &[human_wait],
        measured_evidence_with(|evidence| {
            evidence.marker_counted = false;
        }),
        "missing_measured_frontier_marker",
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

fn assert_has_failure_with_measurement(
    entries: &[parity::CapabilityEntry],
    measured: ScenarioMeasuredEvidence,
    code: &str,
) {
    let report = run_conformance_with_measurement_probe(entries, |_entry| Some(measured.clone()));
    assert!(
        report.failures.iter().any(|failure| failure.code == code),
        "expected failure {code}; got {:#?}",
        report.failures
    );
}

fn measured_evidence_with(
    mutate: impl FnOnce(&mut ScenarioMeasuredEvidence),
) -> ScenarioMeasuredEvidence {
    let mut evidence = ScenarioMeasuredEvidence {
        typed: true,
        actor_knowledge: true,
        rendered: true,
        replay_match: true,
        frontier_advanced: true,
        marker_counted: true,
        autonomous_work: true,
        duration_terminal: true,
        holder_known_sources: true,
        typed_stop_reason: true,
        debug_or_embodied_disposition: true,
    };
    mutate(&mut evidence);
    evidence
}
