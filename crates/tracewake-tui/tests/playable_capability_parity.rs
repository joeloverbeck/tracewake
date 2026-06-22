mod parity;

use std::collections::BTreeSet;

use parity::{
    registry, CapabilityClass, EvidenceFlag, OwnershipScope, SetupOperation, SurfaceDisposition,
    WitnessKind,
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
            semantic_action_id: "wait",
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
