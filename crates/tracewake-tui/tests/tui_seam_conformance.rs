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
        test_name: "render_embodied_view_uses_sealed_view_model_accessors",
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
        requirement: "TUI-0061-004",
        layer: "tui/view-model",
        test_name: "embodied_screen_model_field_disposition",
        source: include_str!("tui_seam_conformance.rs"),
        evidence_kind: EvidenceKind::StaticSourceGuard,
        evidence_class: EvidenceClass::Positive,
        invariants: &["INV-067", "INV-069", "INV-095"],
        acceptance_condition:
            "every EmbodiedViewModel field has a named EmbodiedScreenModel pane disposition",
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
            [
                "PAR-002",
                "PAR-003",
                "SPINE-AC-009",
                "SPINE-AC-012",
                "TUI-0061-004"
            ]
            .contains(&evidence.requirement),
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
fn render_embodied_view_uses_sealed_view_model_accessors() {
    let render_source = include_str!("../src/render.rs");

    let function_body = source_after(render_source, "pub fn render_embodied_view(");
    assert!(
        !function_body.contains("let EmbodiedViewModel {"),
        "render_embodied_view must not destructure sealed EmbodiedViewModel fields"
    );
    assert!(
        render_source.contains("#[deny(unused_variables)]\npub fn render_embodied_view("),
        "render_embodied_view must carry a local unused_variables deny"
    );
    for required_accessor in [
        "view.viewer_actor_id()",
        "view.actor_known_interval_summary()",
    ] {
        assert!(
            render_source.contains(required_accessor),
            "render_embodied_view must use sealed accessor {required_accessor}"
        );
    }
}

#[test]
fn embodied_screen_model_field_disposition() {
    let view_model_source = include_str!("../../tracewake-core/src/view_models.rs");
    let screen_model_source = include_str!("../src/screen/model.rs");
    let builder_body = source_after(
        screen_model_source,
        "pub fn build_embodied_screen_model(\n    view: &EmbodiedViewModel,",
    );

    let view_model_fields = [
        ("view_model_id", "ScreenMetadata", "view.view_model_id()"),
        ("mode", "ScreenMetadata", "view.mode()"),
        (
            "viewer_actor_id",
            "ScreenMetadata",
            "view.viewer_actor_id()",
        ),
        ("sim_tick", "ScreenMetadata", "view.sim_tick()"),
        ("place_id", "PlacePane", "view.place_id.clone()"),
        ("place_label", "PlacePane", "view.place_label.clone()"),
        ("visible_exits", "ExitsPane", "view.visible_exits.clone()"),
        ("visible_doors", "DoorsPane", "view.visible_doors.clone()"),
        (
            "visible_containers",
            "ContainersPane",
            "view.visible_containers.clone()",
        ),
        ("visible_items", "ItemsPane", "view.visible_items.clone()"),
        (
            "carried_items",
            "InventoryPane",
            "view.carried_items.clone()",
        ),
        ("local_actors", "ActorsPane", "view.local_actors.clone()"),
        (
            "semantic_actions",
            "ActionsPane",
            "view.semantic_actions.clone()",
        ),
        (
            "phase3a_status",
            "Phase3AStatusPane",
            "view.phase3a_status.clone()",
        ),
        (
            "last_rejection_summary",
            "WhyNotPane",
            "view.last_rejection_summary.clone()",
        ),
        (
            "last_rejection_why_not",
            "WhyNotPane",
            "view.last_rejection_why_not.clone()",
        ),
        (
            "holder_known_context_id",
            "ScreenMetadata",
            "view.holder_known_context_id()",
        ),
        (
            "holder_known_context_hash",
            "ScreenMetadata",
            "view.holder_known_context_hash()",
        ),
        (
            "holder_known_context_frontier",
            "ScreenMetadata",
            "view.holder_known_context_frontier()",
        ),
        (
            "holder_known_context_source_summary",
            "ScreenMetadata",
            "holder_known_context_source_summary()",
        ),
        (
            "actor_known_interval_summary",
            "ActorKnownIntervalPane",
            "view.actor_known_interval_summary()",
        ),
        ("notebook", "NotebookPane", "view.notebook.clone()"),
        (
            "debug_available",
            "DebugPaneDisposition",
            "view.debug_available()",
        ),
    ];
    assert_eq!(
        view_model_fields.len(),
        23,
        "ticket 0061TUIEMBSCR-004 expects the current 23-field view-model disposition set"
    );

    let view_struct = source_between(
        view_model_source,
        "pub struct EmbodiedViewModel {",
        "}\n\nimpl EmbodiedViewModel",
    );
    for (field_name, pane_type, builder_snippet) in view_model_fields {
        assert!(
            view_struct.contains(&format!("{field_name}:")),
            "EmbodiedViewModel source is missing expected field {field_name}"
        );
        assert!(
            screen_model_source.contains(&format!("pub struct {pane_type}")),
            "screen model source is missing named pane {pane_type} for {field_name}"
        );
        assert!(
            builder_body.contains(builder_snippet),
            "build_embodied_screen_model lacks explicit disposition for {field_name}: {builder_snippet}"
        );
    }

    for forbidden in [
        "let EmbodiedViewModel {",
        "..",
        "_ =>",
        "Default::default()",
        "todo!()",
    ] {
        assert!(
            !builder_body.contains(forbidden),
            "build_embodied_screen_model must not use wildcard/default laundering token {forbidden}"
        );
    }

    let text_dump_source = include_str!("../src/screen/text_dump.rs");
    assert!(
        text_dump_source.contains(".map(render_actor_line)"),
        "ActorsPane local_actors must render through the actor activity disposition formatter"
    );
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
    let text_dump_source = include_str!("../src/screen/text_dump.rs");
    assert_match_has_no_wildcard_arm(
        source_after(text_dump_source, "pub(crate) fn activity_kind_label"),
        "activity_kind_label",
    );
    assert_match_has_no_wildcard_arm(
        source_after(text_dump_source, "pub(crate) fn activity_source_label"),
        "activity_source_label",
    );
    assert_match_has_no_wildcard_arm(
        source_after(debug_panels_source, "pub fn debug_view_model_panel_key"),
        "debug_view_model_panel_key",
    );

    for variant in enum_variant_names(view_model_source, "ObservedActorActivityKind") {
        assert!(
            source_after(text_dump_source, "pub(crate) fn activity_kind_label")
                .contains(&format!("ObservedActorActivityKind::{variant}")),
            "activity_kind_label is missing ObservedActorActivityKind::{variant}"
        );
    }
    for variant in enum_variant_names(view_model_source, "ActorKnownActivitySourceKind") {
        assert!(
            source_after(text_dump_source, "pub(crate) fn activity_source_label")
                .contains(&format!("ActorKnownActivitySourceKind::{variant}")),
            "activity_source_label is missing ActorKnownActivitySourceKind::{variant}"
        );
    }

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
        app_source.contains("self.runtime.debug_epistemics_view()"),
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
