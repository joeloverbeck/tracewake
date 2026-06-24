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
        requirement: "PAR-002",
        layer: "tui/view-model",
        test_name: "render_embodied_view_uses_exhaustive_view_model_destructure",
        source: include_str!("tui_seam_conformance.rs"),
        evidence_kind: EvidenceKind::StaticSourceGuard,
        evidence_class: EvidenceClass::Positive,
        invariants: &["INV-066", "INV-067", "INV-069"],
        acceptance_condition:
            "render_embodied_view names every EmbodiedViewModel field with no rest or wildcard omission",
    },
    TuiSeamEvidence {
        requirement: "PAR-003",
        layer: "tui/debug",
        test_name: "closed_presentation_enum_matches_are_exhaustive_without_wildcards",
        source: include_str!("tui_seam_conformance.rs"),
        evidence_kind: EvidenceKind::StaticSourceGuard,
        evidence_class: EvidenceClass::Positive,
        invariants: &["INV-067", "INV-068", "INV-069", "INV-070"],
        acceptance_condition:
            "closed presentation enum owners match named variants without wildcard fallback",
    },
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
            ["PAR-002", "PAR-003", "SPINE-AC-009", "SPINE-AC-012"].contains(&evidence.requirement),
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
fn render_embodied_view_uses_exhaustive_view_model_destructure() {
    let render_source = include_str!("../src/render.rs");
    let view_model_source = include_str!("../../tracewake-core/src/view_models.rs");

    let function_body = source_after(render_source, "pub fn render_embodied_view(");
    let destructure = source_between(function_body, "let EmbodiedViewModel {", "} = view;");

    assert!(
        !destructure.contains(".."),
        "render_embodied_view must not use a rest pattern in the EmbodiedViewModel destructure"
    );
    assert!(
        !destructure
            .lines()
            .any(|line| line.split_once(':').is_some_and(|(_, binding)| {
                binding.trim().trim_end_matches(',') == "_"
            })),
        "render_embodied_view must use named underscore bindings with rationale comments, not field: _"
    );
    assert!(
        !destructure
            .lines()
            .any(|line| line.trim().trim_end_matches(',') == "_"),
        "render_embodied_view must not use a bare wildcard in the EmbodiedViewModel destructure"
    );
    assert!(
        render_source.contains("#[deny(unused_variables)]\npub fn render_embodied_view("),
        "render_embodied_view must carry a local unused_variables deny"
    );

    for field_name in embodied_view_model_field_names(view_model_source) {
        assert!(
            destructure.contains(&format!("{field_name}:"))
                || destructure
                    .lines()
                    .any(|line| line.trim().starts_with(&format!("{field_name},"))),
            "render_embodied_view destructure is missing EmbodiedViewModel field {field_name}"
        );
    }
}

fn source_after<'a>(source: &'a str, needle: &str) -> &'a str {
    let start = source
        .find(needle)
        .unwrap_or_else(|| panic!("missing source marker {needle}"));
    &source[start..]
}

fn source_between<'a>(source: &'a str, start_marker: &str, end_marker: &str) -> &'a str {
    let start = source
        .find(start_marker)
        .unwrap_or_else(|| panic!("missing source marker {start_marker}"));
    let after_start = start + start_marker.len();
    let end = source[after_start..]
        .find(end_marker)
        .unwrap_or_else(|| panic!("missing source marker {end_marker}"))
        + after_start;
    &source[after_start..end]
}

fn embodied_view_model_field_names(source: &str) -> Vec<&str> {
    let struct_body = source_between(source, "pub struct EmbodiedViewModel {", "}\n\n#[derive");
    struct_body
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            trimmed
                .strip_prefix("pub ")
                .and_then(|field| field.split_once(':'))
                .map(|(field_name, _)| field_name.trim())
        })
        .collect()
}

#[test]
fn closed_presentation_enum_matches_are_exhaustive_without_wildcards() {
    let render_source = include_str!("../src/render.rs");
    let debug_panels_source = include_str!("../src/debug_panels.rs");
    let view_model_source = include_str!("../../tracewake-core/src/view_models.rs");

    assert_match_has_no_wildcard_arm(
        source_after(view_model_source, "impl WhyNotFailureKind"),
        "WhyNotFailureKind::stable_id",
    );
    assert_match_has_no_wildcard_arm(
        source_after(view_model_source, "pub fn actor_safe_summary"),
        "ActionAvailability::actor_safe_summary",
    );
    assert_match_has_no_wildcard_arm(
        source_after(view_model_source, "impl ActionAvailabilityProvenanceKind"),
        "ActionAvailabilityProvenanceKind::stable_id",
    );
    assert_match_has_no_wildcard_arm(
        source_after(render_source, "fn visible_item_source_label"),
        "visible_item_source_label",
    );
    assert_match_has_no_wildcard_arm(
        source_after(debug_panels_source, "pub fn debug_view_model_panel_key"),
        "debug_view_model_panel_key",
    );

    for variant in enum_variant_names(view_model_source, "DebugViewModel") {
        assert!(
            source_after(debug_panels_source, "pub fn debug_view_model_panel_key")
                .contains(&format!("DebugViewModel::{variant}(")),
            "debug_view_model_panel_key is missing DebugViewModel::{variant}"
        );
    }

    assert!(
        !render_source.contains("ActionEffect") && !view_model_source.contains("ActionEffect"),
        "ActionEffect must not gain actor-facing presentation as part of PAR-003"
    );
}

fn assert_match_has_no_wildcard_arm(source: &str, owner: &str) {
    let match_body = source_between(source, "match ", "\n    }");
    assert!(
        !match_body
            .lines()
            .any(|line| line.trim_start().starts_with("_ =>")),
        "{owner} must not use a wildcard match arm"
    );
}

fn enum_variant_names<'a>(source: &'a str, enum_name: &str) -> Vec<&'a str> {
    let enum_body = source_between(source, &format!("pub enum {enum_name} {{"), "}\n\n");
    enum_body
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                return None;
            }
            let variant = trimmed
                .split(['(', '{', ','])
                .next()
                .expect("split always yields a first segment")
                .trim();
            (!variant.is_empty()).then_some(variant)
        })
        .collect()
}

#[test]
fn tui_epistemic_debug_uses_core_builder_not_raw_projection_storage() {
    let app_source = include_str!("../src/app.rs");

    assert!(
        app_source.contains("self.runtime.epistemic_projection().debug_epistemics_view()"),
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
