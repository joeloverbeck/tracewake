#[derive(Clone, Copy)]
struct TuiSeamEvidence {
    requirement: &'static str,
    layer: &'static str,
    test_name: &'static str,
    source: &'static str,
    evidence_kind: EvidenceKind,
    evidence_class: EvidenceClass,
    invariants: &'static [&'static str],
    acceptance_condition: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EvidenceKind {
    RuntimeNegative,
    ReplayDeterminism,
    StaticSourceGuard,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EvidenceClass {
    Positive,
    Negative,
}

const TUI_SEAM_EVIDENCE: &[TuiSeamEvidence] = &[
    TuiSeamEvidence {
        requirement: "SPINE-AC-009",
        layer: "tui/debug",
        test_name: "debug_panel_does_not_change_embodied_affordances",
        source: include_str!("adversarial_gates.rs"),
        evidence_kind: EvidenceKind::RuntimeNegative,
        evidence_class: EvidenceClass::Negative,
        invariants: &["INV-008", "INV-107"],
        acceptance_condition: "debug panels do not change embodied affordances",
    },
    TuiSeamEvidence {
        requirement: "SPINE-AC-012",
        layer: "tui/view-model",
        test_name: "tui_current_view_submission_rejects_stale_selection",
        source: include_str!("adversarial_gates.rs"),
        evidence_kind: EvidenceKind::RuntimeNegative,
        evidence_class: EvidenceClass::Negative,
        invariants: &["INV-008", "INV-108"],
        acceptance_condition: "stale current-view selections are rejected",
    },
    TuiSeamEvidence {
        requirement: "SPINE-AC-012",
        layer: "tui/debug",
        test_name: "debug_command_strings_are_not_embodied_commands",
        source: include_str!("adversarial_gates.rs"),
        evidence_kind: EvidenceKind::RuntimeNegative,
        evidence_class: EvidenceClass::Negative,
        invariants: &["INV-008", "INV-107"],
        acceptance_condition: "debug command strings are not embodied commands",
    },
    TuiSeamEvidence {
        requirement: "SPINE-AC-012",
        layer: "tui/view-model",
        test_name: "tui_sources_do_not_call_event_application_directly",
        source: include_str!("tui_acceptance.rs"),
        evidence_kind: EvidenceKind::StaticSourceGuard,
        evidence_class: EvidenceClass::Negative,
        invariants: &["INV-009", "INV-104"],
        acceptance_condition: "TUI sources do not call event application directly",
    },
    TuiSeamEvidence {
        requirement: "SPINE-AC-012",
        layer: "tui/view-model",
        test_name: "tui_epistemic_debug_uses_core_builder_not_raw_projection_storage",
        source: include_str!("tui_seam_conformance.rs"),
        evidence_kind: EvidenceKind::StaticSourceGuard,
        evidence_class: EvidenceClass::Negative,
        invariants: &["INV-024", "INV-067", "INV-107"],
        acceptance_condition:
            "TUI asks core for debug epistemics and does not read raw projection storage",
    },
    TuiSeamEvidence {
        requirement: "SPINE-AC-012",
        layer: "tui/view-model",
        test_name: "tui_transcript_snapshot_remains_byte_stable",
        source: include_str!("transcript_snapshot.rs"),
        evidence_kind: EvidenceKind::ReplayDeterminism,
        evidence_class: EvidenceClass::Positive,
        invariants: &["INV-018", "INV-092"],
        acceptance_condition: "TUI transcript snapshot remains byte-stable",
    },
];

#[test]
fn tui_seam_conformance_maps_tui_spine_requirements_to_named_tests() {
    for evidence in TUI_SEAM_EVIDENCE {
        assert!(
            ["SPINE-AC-009", "SPINE-AC-012"].contains(&evidence.requirement),
            "unexpected TUI seam requirement {}",
            evidence.requirement
        );
        assert!(
            ["tui/view-model", "tui/debug"].contains(&evidence.layer),
            "unknown TUI seam conformance layer {}",
            evidence.layer
        );
        assert!(!evidence.invariants.is_empty());
        assert!(!evidence.acceptance_condition.is_empty());
        assert_ne!(format!("{:?}", evidence.evidence_kind), "");
        assert_ne!(format!("{:?}", evidence.evidence_class), "");
        let needle = format!("fn {}(", evidence.test_name);
        assert!(
            evidence.source.contains(&needle),
            "missing {} evidence for {} in {}",
            evidence.test_name,
            evidence.requirement,
            evidence.layer
        );
    }

    assert!(TUI_SEAM_EVIDENCE
        .iter()
        .any(|evidence| evidence.layer == "tui/view-model"));
    assert!(TUI_SEAM_EVIDENCE
        .iter()
        .any(|evidence| evidence.layer == "tui/debug"));
}

#[test]
fn tui_epistemic_debug_uses_core_builder_not_raw_projection_storage() {
    let app_source = include_str!("../src/app.rs");

    assert!(
        app_source.contains("self.epistemic_projection.debug_epistemics_view()"),
        "TUI debug epistemics must be built by core, not from raw projection storage"
    );
    for forbidden in [
        ".observations_by_id",
        ".observations_by_actor",
        ".beliefs_by_id",
        ".beliefs_by_holder",
        ".contradictions_by_id",
        ".contradictions_by_holder",
        ".notebook_entries_by_actor",
        ".insert_belief(",
        ".insert_observation(",
        ".insert_contradiction(",
    ] {
        assert!(
            !app_source.contains(forbidden),
            "TUI app must not access raw projection API: {forbidden}"
        );
    }
}
