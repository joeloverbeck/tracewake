use tracewake_content::fixtures;
use tracewake_core::actions::{ReasonCode, ReportStatus};
use tracewake_core::ids::{ActorId, ItemId, SemanticActionId};
use tracewake_core::view_models::{
    ActionAvailability, ActionAvailabilityProvenance, ActionAvailabilityProvenanceKind,
};
use tracewake_tui::app::{AppError, TuiApp};
use tracewake_tui::render::{render_notebook, render_rejection};
use tracewake_tui::run::run_command_loop;
use tracewake_tui::transcript::{
    capture_representative_transcript, capture_representative_transcript_sections,
};

#[derive(Debug)]
struct AdversarialReviewArtifact {
    responsible_layer: &'static str,
    scenario_id: &'static str,
    actor_id: String,
    controller_id: Option<&'static str>,
    context_id: String,
    context_hash: String,
    semantic_id: Option<String>,
    typed_reason_codes: Vec<String>,
    provenance_refs: Vec<String>,
    debug_capability_present: bool,
    actor_surfaces_checked: Vec<&'static str>,
    debug_surfaces_checked: Vec<&'static str>,
    expected_result: &'static str,
    contamination_failure_mode: &'static str,
}

impl AdversarialReviewArtifact {
    fn assert_complete(&self) {
        assert!(!self.responsible_layer.is_empty());
        assert!(!self.scenario_id.is_empty());
        assert!(!self.actor_id.is_empty());
        assert!(self.context_id.starts_with("hkc."));
        assert!(self.context_hash.starts_with("hkc1-"));
        if let Some(controller_id) = self.controller_id {
            assert!(!controller_id.is_empty());
        }
        if let Some(semantic_id) = &self.semantic_id {
            assert!(!semantic_id.is_empty());
        }
        assert!(self
            .typed_reason_codes
            .iter()
            .all(|reason| !reason.is_empty()));
        assert!(self
            .provenance_refs
            .iter()
            .all(|provenance| !provenance.is_empty()));
        assert!(
            self.debug_capability_present
                || self.debug_surfaces_checked.is_empty()
                || self.responsible_layer == "test_oracle"
        );
        assert!(!self.actor_surfaces_checked.is_empty() || !self.debug_surfaces_checked.is_empty());
        assert!(!self.expected_result.is_empty());
        assert!(!self.contamination_failure_mode.is_empty());
    }
}

#[test]
fn adversarial_gates_debug_truth_does_not_enter_actor_surfaces() {
    let mut app = TuiApp::from_golden(fixtures::debug_omniscience_excluded_001()).unwrap();
    app.bind_actor(ActorId::new("actor_mara").unwrap()).unwrap();
    let before_checksum = app.physical_checksum();

    let item = app.render_debug_item_location_panel(&ItemId::new("food_hidden_pantry").unwrap());
    let projection = app.render_debug_projection_rebuild_panel();
    let epistemics = app.debug_epistemics_view();
    let planner = app.render_debug_planner_panel(&ActorId::new("actor_mara").unwrap());
    let replay = app.render_debug_replay_panel();
    let view = app.current_view().unwrap();
    let notebook = app.notebook_view().unwrap();
    let rendered_view = app.render_current_view().unwrap();
    let rendered_notebook = render_notebook(&notebook);

    assert!(epistemics.debug_only());
    assert!(item.contains("DEBUG NON-DIEGETIC"));
    assert!(projection.contains("DEBUG NON-DIEGETIC"));
    assert!(planner.contains("DEBUG NON-DIEGETIC"));
    assert!(replay.contains("DEBUG NON-DIEGETIC"));
    assert_eq!(app.physical_checksum(), before_checksum);
    assert!(view.semantic_actions.iter().all(|entry| !entry
        .target_ids
        .iter()
        .any(|target| target == "food_hidden_pantry")));
    assert!(notebook.typed_leads.is_empty());
    assert!(notebook.source_bound_beliefs.is_empty());
    assert!(!view.holder_known_context_source_summary.contains("debug"));
    for actor_surface in [rendered_view.as_str(), rendered_notebook.as_str()] {
        assert!(!actor_surface.contains("Knowledge context"));
        assert!(!actor_surface.contains("DEBUG NON-DIEGETIC"));
        assert!(!actor_surface.contains("food_hidden_pantry"));
        assert!(!actor_surface.contains("debug_omniscience"));
    }

    let artifact = AdversarialReviewArtifact {
        responsible_layer: "debug_quarantine",
        scenario_id: "debug_omniscience_excluded_001",
        actor_id: view.viewer_actor_id.as_str().to_string(),
        controller_id: Some("controller_human"),
        context_id: view.holder_known_context_id.as_str().to_string(),
        context_hash: view.holder_known_context_hash.as_str().to_string(),
        semantic_id: None,
        typed_reason_codes: Vec::new(),
        provenance_refs: vec![view.holder_known_context_source_summary],
        debug_capability_present: epistemics.debug_only(),
        actor_surfaces_checked: vec!["embodied_view", "notebook", "semantic_actions"],
        debug_surfaces_checked: vec![
            "debug_item_location",
            "debug_projection",
            "debug_epistemics",
            "debug_planner",
            "debug_replay",
        ],
        expected_result: "physical_checksum_unchanged",
        contamination_failure_mode: "debug_provenance_absent_from_holder_known_context",
    };
    artifact.assert_complete();
}

#[test]
fn debug_panel_does_not_change_embodied_affordances() {
    let mut app = TuiApp::from_golden(fixtures::debug_omniscience_excluded_001()).unwrap();
    app.bind_debug_actor(ActorId::new("actor_mara").unwrap())
        .unwrap();
    let before_view = app.current_view().unwrap();
    let before_checksum = app.physical_checksum();
    let before_actions = before_view
        .semantic_actions
        .iter()
        .map(|entry| {
            (
                entry.semantic_action_id.as_str().to_string(),
                entry.action_id.as_str().to_string(),
                entry.target_ids.clone(),
            )
        })
        .collect::<Vec<_>>();
    let before_context = (
        before_view.view_model_id.clone(),
        before_view.holder_known_context_id.clone(),
        before_view.holder_known_context_hash.clone(),
        before_view.holder_known_context_frontier,
        before_view.holder_known_context_source_summary.clone(),
    );

    let _item = app.render_debug_item_location_panel(&ItemId::new("food_hidden_pantry").unwrap());
    let _planner = app.render_debug_planner_panel(&ActorId::new("actor_mara").unwrap());
    let _replay = app.render_debug_replay_panel();
    let _epistemics = app.debug_epistemics_view();
    let after_view = app.current_view().unwrap();
    let after_actions = after_view
        .semantic_actions
        .iter()
        .map(|entry| {
            (
                entry.semantic_action_id.as_str().to_string(),
                entry.action_id.as_str().to_string(),
                entry.target_ids.clone(),
            )
        })
        .collect::<Vec<_>>();
    let after_context = (
        after_view.view_model_id.clone(),
        after_view.holder_known_context_id.clone(),
        after_view.holder_known_context_hash.clone(),
        after_view.holder_known_context_frontier,
        after_view.holder_known_context_source_summary.clone(),
    );

    assert_eq!(after_actions, before_actions);
    assert_eq!(after_context, before_context);
    assert_eq!(app.physical_checksum(), before_checksum);
    assert!(!after_view
        .holder_known_context_source_summary
        .contains("debug"));

    let artifact = AdversarialReviewArtifact {
        responsible_layer: "debug_quarantine",
        scenario_id: "debug_omniscience_excluded_001",
        actor_id: after_view.viewer_actor_id.as_str().to_string(),
        controller_id: Some("controller_human"),
        context_id: after_view.holder_known_context_id.as_str().to_string(),
        context_hash: after_view.holder_known_context_hash.as_str().to_string(),
        semantic_id: None,
        typed_reason_codes: Vec::new(),
        provenance_refs: vec![after_view.holder_known_context_source_summary],
        debug_capability_present: after_view.debug_available,
        actor_surfaces_checked: vec![
            "semantic_actions",
            "view_model_id",
            "holder_known_context_hash",
            "holder_known_context_frontier",
            "physical_checksum",
        ],
        debug_surfaces_checked: vec![
            "debug_item_location",
            "debug_planner",
            "debug_replay",
            "debug_epistemics",
        ],
        expected_result: "debug_rendering_does_not_change_embodied_affordances",
        contamination_failure_mode: "debug_truth_perturbs_current_view_or_source_context",
    };
    artifact.assert_complete();
}

#[test]
fn adversarial_gates_forged_privileged_semantic_id_is_not_current_action() {
    let mut app = TuiApp::from_golden(fixtures::hidden_food_closed_container_001()).unwrap();
    app.bind_actor(ActorId::new("actor_mara").unwrap()).unwrap();
    let view = app.current_view().unwrap();
    let before_checksum = app.physical_checksum();
    let forged = SemanticActionId::new("eat.food.food_hidden_pantry").unwrap();

    let error = app.submit_semantic_action(&forged).unwrap_err();

    assert!(matches!(error, AppError::SemanticActionNotFound(_)));
    assert_eq!(app.physical_checksum(), before_checksum);
    assert!(view
        .semantic_actions
        .iter()
        .all(|entry| entry.semantic_action_id != forged));
    let artifact = AdversarialReviewArtifact {
        responsible_layer: "tui_input_binding",
        scenario_id: "hidden_food_closed_container_001",
        actor_id: view.viewer_actor_id.as_str().to_string(),
        controller_id: Some("controller_human"),
        context_id: view.holder_known_context_id.as_str().to_string(),
        context_hash: view.holder_known_context_hash.as_str().to_string(),
        semantic_id: Some(forged.as_str().to_string()),
        typed_reason_codes: vec!["semantic_action_not_current".to_string()],
        provenance_refs: vec![view.holder_known_context_source_summary],
        debug_capability_present: view.debug_available,
        actor_surfaces_checked: vec!["current_view.semantic_actions"],
        debug_surfaces_checked: vec![],
        expected_result: "no_pipeline_submission_no_checksum_change",
        contamination_failure_mode: "forged_hidden_target_token_not_in_current_view",
    };
    artifact.assert_complete();
}

#[test]
fn adversarial_gates_stale_view_token_fails_after_state_change() {
    let mut app = TuiApp::from_golden(fixtures::view_model_local_actions_001()).unwrap();
    app.bind_actor(ActorId::new("actor_lina").unwrap()).unwrap();
    let old_view = app.current_view().unwrap();
    let stale_open = semantic_action_for_action_id(&app, "open");
    let accepted = app.submit_semantic_action(&stale_open).unwrap();
    assert_eq!(accepted.report.status, ReportStatus::Accepted);
    let before_stale_retry = app.physical_checksum();

    let stale_retry = app.submit_semantic_action(&stale_open).unwrap();

    assert_eq!(stale_retry.report.status, ReportStatus::Rejected);
    assert_eq!(
        stale_retry.report.reason_codes,
        vec![ReasonCode::AlreadyOpen]
    );
    assert_eq!(app.physical_checksum(), before_stale_retry);
    assert!(app
        .current_view()
        .unwrap()
        .semantic_actions
        .iter()
        .any(|entry| entry.semantic_action_id == stale_open));
    let artifact = AdversarialReviewArtifact {
        responsible_layer: "proposal_construction",
        scenario_id: "view_model_local_actions_001",
        actor_id: old_view.viewer_actor_id.as_str().to_string(),
        controller_id: Some("controller_human"),
        context_id: old_view.holder_known_context_id.as_str().to_string(),
        context_hash: old_view.holder_known_context_hash.as_str().to_string(),
        semantic_id: Some(stale_open.as_str().to_string()),
        typed_reason_codes: vec!["already_open".to_string()],
        provenance_refs: vec![old_view.holder_known_context_source_summary],
        debug_capability_present: old_view.debug_available,
        actor_surfaces_checked: vec![
            "old_view.semantic_actions",
            "current_view.semantic_actions",
            "validation_report.reason_codes",
        ],
        debug_surfaces_checked: vec![],
        expected_result: "known_action_rejected_by_validator_without_checksum_change",
        contamination_failure_mode: "actor_known_action_bypasses_physical_precondition",
    };
    artifact.assert_complete();
}

#[test]
fn tui_current_view_submission_rejects_stale_selection() {
    let mut app = TuiApp::from_golden(fixtures::view_model_local_actions_001()).unwrap();
    app.bind_actor(ActorId::new("actor_lina").unwrap()).unwrap();
    let old_view = app.current_view().unwrap();
    let stale_open = semantic_action_for_action_id(&app, "open");
    let old_context = (
        old_view.holder_known_context_id.clone(),
        old_view.holder_known_context_hash.clone(),
        old_view.holder_known_context_frontier,
    );

    let accepted = app.submit_semantic_action(&stale_open).unwrap();
    assert_eq!(accepted.report.status, ReportStatus::Accepted);
    let new_view = app.current_view().unwrap();
    assert_ne!(
        (
            new_view.holder_known_context_id.clone(),
            new_view.holder_known_context_hash.clone(),
            new_view.holder_known_context_frontier,
        ),
        old_context,
        "accepted action must advance the current-view source context"
    );
    let before_stale_retry = app.physical_checksum();

    let stale_retry = app.submit_semantic_action(&stale_open).unwrap();

    assert_eq!(stale_retry.report.status, ReportStatus::Rejected);
    assert_eq!(
        stale_retry.report.reason_codes,
        vec![ReasonCode::AlreadyOpen]
    );
    assert_eq!(app.physical_checksum(), before_stale_retry);
    assert!(new_view
        .semantic_actions
        .iter()
        .any(|entry| entry.semantic_action_id == stale_open));
    let artifact = AdversarialReviewArtifact {
        responsible_layer: "tui_input_binding",
        scenario_id: "view_model_local_actions_001",
        actor_id: old_view.viewer_actor_id.as_str().to_string(),
        controller_id: Some("controller_human"),
        context_id: old_view.holder_known_context_id.as_str().to_string(),
        context_hash: old_view.holder_known_context_hash.as_str().to_string(),
        semantic_id: Some(stale_open.as_str().to_string()),
        typed_reason_codes: vec!["already_open".to_string()],
        provenance_refs: vec![old_view.holder_known_context_source_summary],
        debug_capability_present: old_view.debug_available,
        actor_surfaces_checked: vec![
            "old_view.holder_known_context",
            "new_view.holder_known_context",
            "current_view.semantic_actions",
            "validation_report.reason_codes",
        ],
        debug_surfaces_checked: vec![],
        expected_result: "current_actor_known_selection_validator_rejected_without_checksum_change",
        contamination_failure_mode:
            "actor_known_selection_mutates_after_physical_precondition_drift",
    };
    artifact.assert_complete();
}

#[test]
fn debug_command_strings_are_not_embodied_commands() {
    let mut app = TuiApp::from_golden(fixtures::debug_omniscience_excluded_001()).unwrap();
    app.bind_debug_actor(ActorId::new("actor_mara").unwrap())
        .unwrap();
    let before_view = app.current_view().unwrap();
    let before_actions = semantic_ids(&before_view);
    let before_context = (
        before_view.holder_known_context_id.clone(),
        before_view.holder_known_context_hash.clone(),
        before_view.holder_known_context_frontier,
    );
    let before_checksum = app.physical_checksum();
    let before_events = app.event_count();
    let mut output = Vec::new();

    run_command_loop(
        &mut app,
        b"debug item food_hidden_pantry\ndo debug.item.food_hidden_pantry\nview\nquit\n".as_slice(),
        &mut output,
    )
    .unwrap();

    let rendered = String::from_utf8(output).unwrap();
    let after_view = app.current_view().unwrap();
    assert!(rendered.contains("DEBUG NON-DIEGETIC: Item Location"));
    assert!(rendered.contains("Error: no such current action"));
    assert_eq!(semantic_ids(&after_view), before_actions);
    assert_eq!(
        (
            after_view.holder_known_context_id.clone(),
            after_view.holder_known_context_hash.clone(),
            after_view.holder_known_context_frontier,
        ),
        before_context
    );
    assert_eq!(app.physical_checksum(), before_checksum);
    assert_eq!(app.event_count(), before_events);
    assert!(after_view
        .semantic_actions
        .iter()
        .all(|entry| !entry.semantic_action_id.as_str().contains("debug")));
    let artifact = AdversarialReviewArtifact {
        responsible_layer: "tui_input_binding",
        scenario_id: "debug_omniscience_excluded_001",
        actor_id: after_view.viewer_actor_id.as_str().to_string(),
        controller_id: Some("controller_human"),
        context_id: after_view.holder_known_context_id.as_str().to_string(),
        context_hash: after_view.holder_known_context_hash.as_str().to_string(),
        semantic_id: Some("debug.item.food_hidden_pantry".to_string()),
        typed_reason_codes: vec!["semantic_action_not_current".to_string()],
        provenance_refs: vec![after_view.holder_known_context_source_summary],
        debug_capability_present: after_view.debug_available,
        actor_surfaces_checked: vec!["command_loop", "semantic_actions", "holder_known_context"],
        debug_surfaces_checked: vec!["debug_item_location"],
        expected_result: "debug_command_rendered_but_not_submitted_as_action",
        contamination_failure_mode: "debug_command_string_not_embodied_affordance",
    };
    artifact.assert_complete();
}

#[test]
fn adversarial_gates_possession_rebind_does_not_transfer_notebook_or_debug_truth() {
    let mut app = TuiApp::from_golden(fixtures::expectation_contradiction_001()).unwrap();
    app.bind_debug_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();
    app.submit_semantic_action(&SemanticActionId::new("open.container.strongbox_tomas").unwrap())
        .unwrap();
    app.submit_semantic_action(&SemanticActionId::new("check.container.strongbox_tomas").unwrap())
        .unwrap();
    let tomas_notebook = app.notebook_view().unwrap();
    assert_eq!(tomas_notebook.typed_leads.len(), 1);
    let _debug_truth = app.render_debug_item_location_panel(&ItemId::new("coin_stack_01").unwrap());
    let checksum_before_rebind = app.physical_checksum();

    app.bind_debug_actor(ActorId::new("actor_mara").unwrap())
        .unwrap();
    let mara_view = app.current_view().unwrap();
    let mara_notebook = app.notebook_view().unwrap();
    let mara_rendered_view = app.render_current_view().unwrap();
    let mara_rendered_notebook = render_notebook(&mara_notebook);
    let tomas_rendered_notebook = render_notebook(&tomas_notebook);

    assert_eq!(app.physical_checksum(), checksum_before_rebind);
    assert_eq!(mara_view.viewer_actor_id.as_str(), "actor_mara");
    assert!(mara_notebook.typed_leads.is_empty());
    assert!(mara_notebook.source_bound_beliefs.is_empty());
    assert!(tomas_rendered_notebook.contains("contradiction="));
    assert!(tomas_rendered_notebook.contains("source_kind="));
    assert!(
        !tomas_rendered_notebook
            .lines()
            .skip_while(|line| *line != "Leads:")
            .take_while(|line| *line != "Beliefs:")
            .any(|line| line == "- none"),
        "typed notebook leads must suppress the legacy empty marker: {tomas_rendered_notebook}"
    );
    for actor_surface in [mara_rendered_view.as_str(), mara_rendered_notebook.as_str()] {
        assert!(!actor_surface.contains("Knowledge context"));
        assert!(!actor_surface.contains("DEBUG NON-DIEGETIC"));
        assert!(!actor_surface.contains("belief_tomas"));
    }
    assert!(mara_view
        .phase3a_status
        .as_ref()
        .is_none_or(|status| status.intention_summary.is_none()));
    let artifact = AdversarialReviewArtifact {
        responsible_layer: "tui_input_binding",
        scenario_id: "expectation_contradiction_001",
        actor_id: mara_view.viewer_actor_id.as_str().to_string(),
        controller_id: Some("controller_human"),
        context_id: mara_view.holder_known_context_id.as_str().to_string(),
        context_hash: mara_view.holder_known_context_hash.as_str().to_string(),
        semantic_id: None,
        typed_reason_codes: Vec::new(),
        provenance_refs: vec![mara_view.holder_known_context_source_summary],
        debug_capability_present: mara_view.debug_available,
        actor_surfaces_checked: vec!["notebook", "needs", "intention", "context_id"],
        debug_surfaces_checked: vec!["debug_item_location"],
        expected_result: "rebind_changes_controller_only",
        contamination_failure_mode: "actor_a_notebook_and_debug_truth_absent_from_actor_b_context",
    };
    artifact.assert_complete();
}

#[test]
fn adversarial_gates_possession_rebind_does_not_transfer_rejection_why_not() {
    let mut app = TuiApp::from_golden(fixtures::expectation_contradiction_001()).unwrap();
    app.bind_debug_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();
    let blocked_action = SemanticActionId::new("check.container.strongbox_tomas").unwrap();
    let rejected = app.submit_semantic_action(&blocked_action).unwrap();
    let tomas_view = app.current_view().unwrap();

    assert_eq!(rejected.report.status, ReportStatus::Rejected);
    assert!(tomas_view.last_rejection_summary.is_some());
    assert!(tomas_view.last_rejection_why_not.is_some());

    app.bind_debug_actor(ActorId::new("actor_mara").unwrap())
        .unwrap();
    let mara_view = app.current_view().unwrap();
    let mara_rendered_view = app.render_current_view().unwrap();

    assert_eq!(mara_view.viewer_actor_id.as_str(), "actor_mara");
    assert_eq!(mara_view.last_rejection_summary, None);
    assert_eq!(mara_view.last_rejection_why_not, None);
    assert!(!mara_rendered_view.contains("Why-not:"));
    assert!(!mara_rendered_view.contains("Why-not facts:"));

    let artifact = AdversarialReviewArtifact {
        responsible_layer: "tui_input_binding",
        scenario_id: "expectation_contradiction_001",
        actor_id: mara_view.viewer_actor_id.as_str().to_string(),
        controller_id: Some("controller_human"),
        context_id: mara_view.holder_known_context_id.as_str().to_string(),
        context_hash: mara_view.holder_known_context_hash.as_str().to_string(),
        semantic_id: Some(blocked_action.to_string()),
        typed_reason_codes: rejected
            .report
            .reason_codes
            .iter()
            .map(|reason| reason.stable_id().to_string())
            .collect(),
        provenance_refs: rejected
            .report
            .actor_visible_facts
            .iter()
            .map(tracewake_core::actions::CheckedFact::render_pair)
            .collect(),
        debug_capability_present: mara_view.debug_available,
        actor_surfaces_checked: vec!["embodied_view.last_rejection", "rendered_why_not"],
        debug_surfaces_checked: vec!["debug_action_rejection"],
        expected_result: "rebind_after_rejection_clears_previous_actor_why_not",
        contamination_failure_mode: "actor_a_rejection_absent_from_actor_b_context",
    };
    artifact.assert_complete();
}

#[test]
fn adversarial_gates_public_tui_boundary_rejects_direct_dispatch_shape() {
    let mut app = TuiApp::load_default().unwrap();
    app.bind_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();
    let view = app.current_view().unwrap();
    let before_checksum = app.physical_checksum();
    let mut output = Vec::new();

    run_command_loop(&mut app, b"do apply_event\nquit\n".as_slice(), &mut output).unwrap();

    assert_eq!(app.physical_checksum(), before_checksum);
    assert!(!view
        .semantic_actions
        .iter()
        .any(|entry| entry.semantic_action_id.as_str() == "apply_event"));
    let rendered = String::from_utf8(output).unwrap();
    assert!(rendered.contains("Error: no such current action"));
    let artifact = AdversarialReviewArtifact {
        responsible_layer: "tui_input_binding",
        scenario_id: "strongbox_001",
        actor_id: view.viewer_actor_id.as_str().to_string(),
        controller_id: Some("controller_human"),
        context_id: view.holder_known_context_id.as_str().to_string(),
        context_hash: view.holder_known_context_hash.as_str().to_string(),
        semantic_id: Some("apply_event".to_string()),
        typed_reason_codes: vec!["semantic_action_not_current".to_string()],
        provenance_refs: vec![view.holder_known_context_source_summary],
        debug_capability_present: view.debug_available,
        actor_surfaces_checked: vec!["command_loop", "current_view.semantic_actions"],
        debug_surfaces_checked: vec![],
        expected_result: "no_event_application_public_command",
        contamination_failure_mode: "raw_dispatch_shape_never_becomes_semantic_action",
    };
    artifact.assert_complete();
}

#[test]
fn adversarial_gates_tui_rule_inference_cannot_apply_hidden_food_target() {
    let mut app = TuiApp::from_golden(fixtures::hidden_food_unknown_route_001()).unwrap();
    app.bind_actor(ActorId::new("actor_mara").unwrap()).unwrap();
    let view = app.current_view().unwrap();
    let before_checksum = app.physical_checksum();

    let hidden_food = SemanticActionId::new("eat.food.food_hidden_pantry").unwrap();
    let error = app.submit_semantic_action(&hidden_food).unwrap_err();

    assert!(matches!(error, AppError::SemanticActionNotFound(_)));
    assert_eq!(app.physical_checksum(), before_checksum);
    assert!(view.semantic_actions.iter().all(|entry| !entry
        .target_ids
        .iter()
        .any(|target| target == "food_hidden_pantry")));
    let artifact = AdversarialReviewArtifact {
        responsible_layer: "view_model",
        scenario_id: "hidden_food_unknown_route_001",
        actor_id: view.viewer_actor_id.as_str().to_string(),
        controller_id: Some("controller_human"),
        context_id: view.holder_known_context_id.as_str().to_string(),
        context_hash: view.holder_known_context_hash.as_str().to_string(),
        semantic_id: Some(hidden_food.as_str().to_string()),
        typed_reason_codes: vec!["semantic_action_not_current".to_string()],
        provenance_refs: vec![view.holder_known_context_source_summary],
        debug_capability_present: view.debug_available,
        actor_surfaces_checked: vec!["semantic_actions", "target_ids"],
        debug_surfaces_checked: vec![],
        expected_result: "hidden_food_not_promoted_to_action",
        contamination_failure_mode: "tui_does_not_infer_rules_from_physical_hidden_truth",
    };
    artifact.assert_complete();
}

#[test]
fn adversarial_gates_why_not_actor_surface_uses_typed_non_leaking_facts() {
    let mut app = TuiApp::from_golden(fixtures::door_access_001()).unwrap();
    app.bind_actor(ActorId::new("actor_sena").unwrap()).unwrap();
    let view = app.current_view().unwrap();
    let rejected = app
        .submit_semantic_action(&SemanticActionId::new("move.to.back_room").unwrap())
        .unwrap();
    let rendered_view = app.render_current_view().unwrap();
    let rendered_rejection = render_rejection(&rejected.report);
    let why_not = app
        .current_view()
        .unwrap()
        .last_rejection_why_not
        .expect("rejection renders actor why-not");

    assert_eq!(rejected.report.status, ReportStatus::Rejected);
    assert_eq!(
        rejected
            .report
            .reason_codes
            .iter()
            .map(|reason| reason.stable_id())
            .collect::<Vec<_>>(),
        vec!["door_closed_blocks_movement"]
    );
    assert_eq!(
        why_not.reason_codes,
        vec!["door_closed_blocks_movement".to_string()]
    );
    assert!(why_not
        .actor_visible_facts
        .iter()
        .all(|fact| !fact.contains("holder_known_context")));
    assert!(why_not
        .actor_visible_facts
        .iter()
        .chain(std::iter::once(&why_not.actor_known_summary))
        .all(|text| {
            !text.contains("DEBUG NON-DIEGETIC")
                && !text.contains("debug")
                && !text.contains("culprit")
                && !text.contains("actor_mara")
        }));
    assert!(app.render_debug_action_rejection_panel().is_some());
    assert!(rendered_view.contains("Why-not:"));
    assert!(rendered_rejection.contains("Why-not:"));
    assert!(rendered_rejection.contains("door_closed_blocks_movement"));
    assert!(rendered_rejection.contains(&rejected.report.actor_visible_summary));
    let artifact = AdversarialReviewArtifact {
        responsible_layer: "action_validation",
        scenario_id: "door_access_001",
        actor_id: view.viewer_actor_id.as_str().to_string(),
        controller_id: Some("controller_human"),
        context_id: view.holder_known_context_id.as_str().to_string(),
        context_hash: view.holder_known_context_hash.as_str().to_string(),
        semantic_id: Some("move.to.back_room".to_string()),
        typed_reason_codes: why_not.reason_codes,
        provenance_refs: rejected
            .report
            .actor_visible_facts
            .iter()
            .map(tracewake_core::actions::CheckedFact::render_pair)
            .collect(),
        debug_capability_present: true,
        actor_surfaces_checked: vec!["why_not.reason_codes", "actor_visible_facts"],
        debug_surfaces_checked: vec!["debug_action_rejection"],
        expected_result: "rejected_without_hidden_context_fields_in_actor_view",
        contamination_failure_mode: "debug_only_context_fields_quarantined",
    };
    artifact.assert_complete();
}

#[test]
fn adversarial_gates_typed_diagnostics_are_independent_of_display_text() {
    let provenance = vec![ActionAvailabilityProvenance::new(
        ActionAvailabilityProvenanceKind::HolderKnownContext,
        "hkc.actor_sena.0.1",
    )];
    let original = ActionAvailability::disabled(
        vec![ReasonCode::DoorClosedBlocksMovement],
        "The door is closed.",
        provenance.clone(),
        Vec::new(),
    );
    let reworded = ActionAvailability::disabled(
        vec![ReasonCode::DoorClosedBlocksMovement],
        "A closed door blocks travel.",
        provenance.clone(),
        Vec::new(),
    );
    let typed_changed = ActionAvailability::disabled(
        vec![ReasonCode::TargetNotReachable],
        "The door is closed.",
        provenance,
        Vec::new(),
    );

    assert_eq!(original.reason_codes(), reworded.reason_codes());
    assert_ne!(original.actor_safe_summary(), reworded.actor_safe_summary());
    assert_ne!(original.reason_codes(), typed_changed.reason_codes());
    let artifact = AdversarialReviewArtifact {
        responsible_layer: "test_oracle",
        scenario_id: "typed_diagnostics_wording_change",
        actor_id: "actor_sena".to_string(),
        controller_id: None,
        context_id: "hkc.actor_sena.0.1".to_string(),
        context_hash: "hkc1-typed-diagnostic-test".to_string(),
        semantic_id: Some("move.to.back_room".to_string()),
        typed_reason_codes: original
            .reason_codes()
            .iter()
            .map(|reason| reason.stable_id().to_string())
            .collect(),
        provenance_refs: original
            .provenance_refs()
            .iter()
            .map(|reference| {
                format!(
                    "{}:{}",
                    reference.kind.stable_id(),
                    reference.reference.as_str()
                )
            })
            .collect(),
        debug_capability_present: false,
        actor_surfaces_checked: vec!["action_availability.reason_codes"],
        debug_surfaces_checked: vec![],
        expected_result: "wording_change_keeps_semantic_outcome_typed_change_fails",
        contamination_failure_mode: "display_text_not_test_authority",
    };
    artifact.assert_complete();
}

#[test]
fn adversarial_gates_no_human_operator_output_stays_debug_only() {
    let mut app = TuiApp::from_golden(fixtures::no_human_day_001()).unwrap();
    app.bind_debug_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();
    let before_events = app.event_count();
    let mut output = Vec::new();

    run_command_loop(
        &mut app,
        b"debug run no-human-day\nview\nquit\n".as_slice(),
        &mut output,
    )
    .unwrap();

    let view = app.current_view().unwrap();
    assert!(app.event_count() > before_events);
    assert_eq!(view.holder_known_context_frontier, app.event_count() as u64);
    assert!(!view
        .holder_known_context_source_summary
        .contains("ordinary_events"));
    assert!(!view
        .holder_known_context_source_summary
        .contains("routine_events"));
    assert!(String::from_utf8(output)
        .unwrap()
        .contains("DEBUG NON-DIEGETIC: No Human Day"));
    let artifact = AdversarialReviewArtifact {
        responsible_layer: "scheduler",
        scenario_id: "no_human_day_001",
        actor_id: view.viewer_actor_id.as_str().to_string(),
        controller_id: Some("controller_human"),
        context_id: view.holder_known_context_id.as_str().to_string(),
        context_hash: view.holder_known_context_hash.as_str().to_string(),
        semantic_id: None,
        typed_reason_codes: Vec::new(),
        provenance_refs: vec![view.holder_known_context_source_summary],
        debug_capability_present: view.debug_available,
        actor_surfaces_checked: vec!["embodied_view", "holder_known_context_frontier"],
        debug_surfaces_checked: vec!["debug_no_human_day"],
        expected_result: "operator_metrics_debug_only_actor_view_rebuilt",
        contamination_failure_mode: "no_human_metrics_not_actor_knowledge",
    };
    artifact.assert_complete();
}

#[test]
fn adversarial_gates_rendering_does_not_change_typed_order_or_replay() {
    let mut app = TuiApp::from_golden(fixtures::door_access_001()).unwrap();
    app.bind_debug_actor(ActorId::new("actor_sena").unwrap())
        .unwrap();
    let before = app.current_view().unwrap();
    let before_action_ids = semantic_ids(&before);
    let before_checksum = app.physical_checksum();

    let _rendered = app.render_current_view().unwrap();
    let _debug = app.render_debug_replay_panel();
    let after = app.current_view().unwrap();
    let first_sections = capture_representative_transcript_sections().unwrap();
    let second_sections = capture_representative_transcript_sections().unwrap();

    assert_eq!(semantic_ids(&after), before_action_ids);
    assert_eq!(
        after.holder_known_context_hash,
        before.holder_known_context_hash
    );
    assert_eq!(app.physical_checksum(), before_checksum);
    assert_eq!(first_sections, second_sections);
    let artifact = AdversarialReviewArtifact {
        responsible_layer: "replay",
        scenario_id: "door_access_001",
        actor_id: after.viewer_actor_id.as_str().to_string(),
        controller_id: Some("controller_human"),
        context_id: after.holder_known_context_id.as_str().to_string(),
        context_hash: after.holder_known_context_hash.as_str().to_string(),
        semantic_id: before_action_ids.first().cloned(),
        typed_reason_codes: Vec::new(),
        provenance_refs: first_sections
            .iter()
            .map(|section| section.name.clone())
            .collect(),
        debug_capability_present: after.debug_available,
        actor_surfaces_checked: vec!["semantic_action_order", "context_hash", "physical_checksum"],
        debug_surfaces_checked: vec!["debug_replay", "transcript_sections"],
        expected_result: "render_and_debug_replay_are_read_only_and_deterministic",
        contamination_failure_mode: "terminal_render_state_not_behavior_authority",
    };
    artifact.assert_complete();
}

#[test]
fn tui_transcript_marks_debug_sections_non_diegetic() {
    let transcript = capture_representative_transcript().unwrap();
    for heading in [
        "DEBUG NON-DIEGETIC: Event Log",
        "DEBUG NON-DIEGETIC: Replay",
        "DEBUG NON-DIEGETIC: Epistemics",
        "DEBUG NON-DIEGETIC: Beliefs",
        "DEBUG NON-DIEGETIC: Observations",
    ] {
        assert!(
            transcript.contains(heading),
            "transcript missing debug marker {heading}"
        );
    }
    assert!(!transcript.contains("Embodied View\nDEBUG NON-DIEGETIC"));
}

fn semantic_action_for_action_id(app: &TuiApp, action_id: &str) -> SemanticActionId {
    app.current_view()
        .unwrap()
        .semantic_actions
        .iter()
        .find(|action| action.action_id.as_str() == action_id)
        .map(|action| action.semantic_action_id.clone())
        .expect("current view surfaces requested action")
}

fn semantic_ids(view: &tracewake_core::view_models::EmbodiedViewModel) -> Vec<String> {
    view.semantic_actions
        .iter()
        .map(|entry| entry.semantic_action_id.as_str().to_string())
        .collect()
}
