use std::path::{Path, PathBuf};

#[derive(Clone, Copy)]
struct SpineEvidence {
    requirement: &'static str,
    layer: &'static str,
    test_name: &'static str,
    source_path: &'static str,
}

const REQUIREMENTS: &[&str] = &[
    "SPINE-AC-001",
    "SPINE-AC-002",
    "SPINE-AC-003",
    "SPINE-AC-004",
    "SPINE-AC-005",
    "SPINE-AC-006",
    "SPINE-AC-007",
    "SPINE-AC-008",
    "SPINE-AC-009",
    "SPINE-AC-010",
    "SPINE-AC-011",
    "SPINE-AC-012",
    "SPINE-AC-013",
    "SPINE-AC-014",
    "SPINE-AC-015",
];

const RESPONSIBLE_LAYERS: &[&str] = &[
    "core/events",
    "core/replay",
    "core/state",
    "core/scheduler",
    "core/actions",
    "core/agent",
    "content/schema",
    "content/validation",
    "tui/view-model",
    "tui/debug",
    "workspace/ci",
];

const SPINE_EVIDENCE: &[SpineEvidence] = &[
    SpineEvidence {
        requirement: "SPINE-AC-001",
        layer: "core/state",
        test_name: "guard_001_authoritative_state_fields_are_not_publicly_mutable",
        source_path: "crates/tracewake-core/tests/anti_regression_guards.rs",
    },
    SpineEvidence {
        requirement: "SPINE-AC-001",
        layer: "core/events",
        test_name: "guard_001_mutation_capability_is_private_to_event_application",
        source_path: "crates/tracewake-core/tests/anti_regression_guards.rs",
    },
    SpineEvidence {
        requirement: "SPINE-AC-002",
        layer: "core/replay",
        test_name: "adding_event_schema_version_requires_migrator_registration",
        source_path: "crates/tracewake-core/tests/anti_regression_guards.rs",
    },
    SpineEvidence {
        requirement: "SPINE-AC-003",
        layer: "core/events",
        test_name: "event_kind_metadata_is_total",
        source_path: "crates/tracewake-core/tests/anti_regression_guards.rs",
    },
    SpineEvidence {
        requirement: "SPINE-AC-003",
        layer: "core/replay",
        test_name: "non_world_stream_cannot_change_physical_checksum",
        source_path: "crates/tracewake-core/tests/anti_regression_guards.rs",
    },
    SpineEvidence {
        requirement: "SPINE-AC-004",
        layer: "core/replay",
        test_name: "checksum_coverage_is_total_for_authoritative_state",
        source_path: "crates/tracewake-core/tests/anti_regression_guards.rs",
    },
    SpineEvidence {
        requirement: "SPINE-AC-005",
        layer: "workspace/ci",
        test_name: "nondeterminism_api_gate",
        source_path: "crates/tracewake-core/tests/anti_regression_guards.rs",
    },
    SpineEvidence {
        requirement: "SPINE-AC-006",
        layer: "core/scheduler",
        test_name: "scheduler_never_direct_dispatches_primitive_action",
        source_path: "crates/tracewake-core/tests/anti_regression_guards.rs",
    },
    SpineEvidence {
        requirement: "SPINE-AC-006",
        layer: "core/scheduler",
        test_name: "guard_006_scheduler_has_no_direct_routine_or_need_proposal_bypass",
        source_path: "crates/tracewake-core/tests/anti_regression_guards.rs",
    },
    SpineEvidence {
        requirement: "SPINE-AC-007",
        layer: "core/actions",
        test_name: "forged_or_stale_source_context_rejected_by_reason_code",
        source_path: "crates/tracewake-core/tests/anti_regression_guards.rs",
    },
    SpineEvidence {
        requirement: "SPINE-AC-007",
        layer: "core/actions",
        test_name: "privileged_tui_proposal_requires_current_view_source_context",
        source_path: "crates/tracewake-core/tests/anti_regression_guards.rs",
    },
    SpineEvidence {
        requirement: "SPINE-AC-008",
        layer: "core/actions",
        test_name: "accepted_action_appends_before_authoritative_apply",
        source_path: "crates/tracewake-core/tests/anti_regression_guards.rs",
    },
    SpineEvidence {
        requirement: "SPINE-AC-008",
        layer: "core/events",
        test_name: "no_direct_apply_event_outside_event_replay_or_pipeline",
        source_path: "crates/tracewake-core/tests/anti_regression_guards.rs",
    },
    SpineEvidence {
        requirement: "SPINE-AC-009",
        layer: "core/agent",
        test_name: "actor_known_context_unforgeable_from_truth",
        source_path: "crates/tracewake-core/tests/hidden_truth_gates.rs",
    },
    SpineEvidence {
        requirement: "SPINE-AC-009",
        layer: "tui/debug",
        test_name: "debug_panel_does_not_change_embodied_affordances",
        source_path: "crates/tracewake-tui/tests/adversarial_gates.rs",
    },
    SpineEvidence {
        requirement: "SPINE-AC-010",
        layer: "content/validation",
        test_name: "content_prose_born_fact_rejected",
        source_path: "crates/tracewake-content/tests/forbidden_content.rs",
    },
    SpineEvidence {
        requirement: "SPINE-AC-010",
        layer: "content/schema",
        test_name: "content_new_field_requires_typed_validation_and_canonical_serialization_metadata",
        source_path: "crates/tracewake-content/tests/forbidden_content.rs",
    },
    SpineEvidence {
        requirement: "SPINE-AC-011",
        layer: "core/actions",
        test_name: "diagnostics_never_assert_display_label_as_authority",
        source_path: "crates/tracewake-core/tests/anti_regression_guards.rs",
    },
    SpineEvidence {
        requirement: "SPINE-AC-011",
        layer: "core/actions",
        test_name: "validation_report_keeps_typed_provenance_and_actor_debug_split",
        source_path: "crates/tracewake-core/tests/anti_regression_guards.rs",
    },
    SpineEvidence {
        requirement: "SPINE-AC-012",
        layer: "tui/view-model",
        test_name: "tui_current_view_submission_rejects_stale_selection",
        source_path: "crates/tracewake-tui/tests/adversarial_gates.rs",
    },
    SpineEvidence {
        requirement: "SPINE-AC-012",
        layer: "tui/view-model",
        test_name: "tui_transcript_snapshot_remains_byte_stable",
        source_path: "crates/tracewake-tui/tests/transcript_snapshot.rs",
    },
    SpineEvidence {
        requirement: "SPINE-AC-013",
        layer: "workspace/ci",
        test_name: "spine_conformance_maps_every_spine_requirement_to_named_evidence",
        source_path: "crates/tracewake-core/tests/spine_conformance.rs",
    },
    SpineEvidence {
        requirement: "SPINE-AC-014",
        layer: "workspace/ci",
        test_name: "doc_invariant_references_are_live",
        source_path: "crates/tracewake-core/tests/doc_invariant_references.rs",
    },
    SpineEvidence {
        requirement: "SPINE-AC-015",
        layer: "workspace/ci",
        test_name: "scoped_acceptance_artifact_present",
        source_path: "crates/tracewake-core/tests/spine_conformance.rs",
    },
];

#[test]
fn spine_conformance_maps_every_spine_requirement_to_named_evidence() {
    for evidence in SPINE_EVIDENCE {
        assert!(
            REQUIREMENTS.contains(&evidence.requirement),
            "unknown spine requirement {} in {}",
            evidence.requirement,
            evidence.test_name
        );
        assert!(
            RESPONSIBLE_LAYERS.contains(&evidence.layer),
            "unknown responsible layer {} for {}",
            evidence.layer,
            evidence.requirement
        );
        assert_test_exists(evidence);
    }

    for requirement in REQUIREMENTS {
        assert!(
            SPINE_EVIDENCE
                .iter()
                .any(|evidence| evidence.requirement == *requirement),
            "missing named spine conformance evidence for {requirement}"
        );
    }
}

#[test]
fn spine_conformance_reports_every_responsible_layer() {
    for layer in RESPONSIBLE_LAYERS {
        assert!(
            SPINE_EVIDENCE
                .iter()
                .any(|evidence| evidence.layer == *layer),
            "missing named spine conformance evidence for responsible layer {layer}"
        );
    }
}

#[test]
fn scoped_acceptance_artifact_present() {
    let artifact = include_str!("../../../docs/4-specs/0003_ACCEPTANCE_ARTIFACT_TEMPLATE.md");

    for required in [
        "Exact commit under test",
        "Gates run",
        "Changed files",
        "Per-requirement acceptance evidence",
        "Residual convention-only items",
        "Phase 1 / Phase 1A spine hardening remediation accepted for this commit.",
        "Forbidden wording",
    ] {
        assert!(
            artifact.contains(required),
            "acceptance artifact template is missing required section or wording: {required}"
        );
    }
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("tracewake-core is nested under crates")
        .to_path_buf()
}

fn assert_test_exists(evidence: &SpineEvidence) {
    let source = read_workspace_file(evidence.source_path);
    let needle = format!("fn {}(", evidence.test_name);
    assert!(
        source.contains(&needle),
        "missing {} evidence for {} in {}",
        evidence.test_name,
        evidence.requirement,
        evidence.source_path
    );
}

#[allow(
    clippy::disallowed_methods,
    reason = "conformance test scans source/test files; this is not simulation outcome code"
)]
fn read_workspace_file(relative_path: &str) -> String {
    std::fs::read_to_string(repo_root().join(relative_path))
        .unwrap_or_else(|error| panic!("{} is readable: {error}", relative_path))
}
