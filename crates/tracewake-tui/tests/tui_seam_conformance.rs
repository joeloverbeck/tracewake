#[derive(Clone, Copy)]
struct TuiSeamEvidence {
    requirement: &'static str,
    layer: &'static str,
    test_name: &'static str,
    source: &'static str,
}

const TUI_SEAM_EVIDENCE: &[TuiSeamEvidence] = &[
    TuiSeamEvidence {
        requirement: "SPINE-AC-009",
        layer: "tui/debug",
        test_name: "debug_panel_does_not_change_embodied_affordances",
        source: include_str!("adversarial_gates.rs"),
    },
    TuiSeamEvidence {
        requirement: "SPINE-AC-012",
        layer: "tui/view-model",
        test_name: "tui_current_view_submission_rejects_stale_selection",
        source: include_str!("adversarial_gates.rs"),
    },
    TuiSeamEvidence {
        requirement: "SPINE-AC-012",
        layer: "tui/debug",
        test_name: "debug_command_strings_are_not_embodied_commands",
        source: include_str!("adversarial_gates.rs"),
    },
    TuiSeamEvidence {
        requirement: "SPINE-AC-012",
        layer: "tui/view-model",
        test_name: "tui_sources_do_not_call_event_application_directly",
        source: include_str!("tui_acceptance.rs"),
    },
    TuiSeamEvidence {
        requirement: "SPINE-AC-012",
        layer: "tui/view-model",
        test_name: "tui_transcript_snapshot_remains_byte_stable",
        source: include_str!("transcript_snapshot.rs"),
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
