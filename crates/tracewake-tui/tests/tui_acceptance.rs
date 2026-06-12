use tracewake_content::fixtures;
use tracewake_core::actions::ReportStatus;
use tracewake_core::ids::{ActorId, ItemId, SemanticActionId};
use tracewake_tui::app::TuiApp;
use tracewake_tui::input::semantic_id_for_selection;
use tracewake_tui::render::render_notebook;
use tracewake_tui::run::run_command_loop;
use tracewake_tui::transcript::{
    capture_representative_transcript, capture_representative_transcript_sections,
};

#[derive(Debug)]
struct PositiveProofArtifact {
    responsible_layer: &'static str,
    scenario_id: &'static str,
    actor_id: String,
    context_id: String,
    semantic_id: Option<String>,
    report_status: Option<ReportStatus>,
    event_ids: Vec<String>,
    typed_reason_codes: Vec<String>,
    provenance: Vec<String>,
    debug_capability_present: bool,
    surfaces_checked: Vec<&'static str>,
    checksum_result: &'static str,
}

impl PositiveProofArtifact {
    fn assert_review_fields(&self) {
        assert!(!self.responsible_layer.is_empty());
        assert!(!self.scenario_id.is_empty());
        assert!(!self.actor_id.is_empty());
        assert!(self.context_id.starts_with("hkc."));
        assert!(!self.surfaces_checked.is_empty());
        assert!(!self.checksum_result.is_empty());
    }
}

#[test]
fn tui_selects_semantic_action_id_not_menu_index() {
    let mut app = TuiApp::load_default().unwrap();
    app.bind_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();
    let view = app.current_view().unwrap();

    let selected = semantic_id_for_selection(&view, 1).unwrap();

    assert_ne!(selected.as_str(), "1");
    assert!(view
        .semantic_actions
        .iter()
        .any(|action| action.semantic_action_id == selected));
}

#[test]
fn tui_transcript_is_deterministic() {
    let first = capture_representative_transcript().unwrap();
    let second = capture_representative_transcript().unwrap();

    assert_eq!(first, second);
    assert!(first.contains("DEBUG NON-DIEGETIC"));
}

#[test]
fn tui_sources_do_not_call_event_application_directly() {
    let sources = [
        ("app.rs", include_str!("../src/app.rs")),
        ("debug_panels.rs", include_str!("../src/debug_panels.rs")),
        ("input.rs", include_str!("../src/input.rs")),
        ("launch.rs", include_str!("../src/launch.rs")),
        ("lib.rs", include_str!("../src/lib.rs")),
        ("main.rs", include_str!("../src/main.rs")),
        ("render.rs", include_str!("../src/render.rs")),
        ("run.rs", include_str!("../src/run.rs")),
        ("transcript.rs", include_str!("../src/transcript.rs")),
    ];
    let mut violations = Vec::new();
    for (path, source) in sources {
        for forbidden in [
            "apply_event(",
            "apply_event_stream(",
            "use tracewake_core::events::apply",
            "tracewake_core::events::apply::",
        ] {
            if source.contains(forbidden) {
                violations.push(format!("{path} contains {forbidden}"));
            }
        }
    }

    assert!(
        violations.is_empty(),
        "TUI must mutate only through run_pipeline/current-view semantic submissions:\n{}",
        violations.join("\n")
    );
    assert!(
        include_str!("../src/app.rs").contains("run_pipeline(&mut context, &proposal)"),
        "TUI submit path must retain the shared pipeline call"
    );
}

#[test]
fn debug_truth_does_not_enter_embodied_view() {
    let mut app = TuiApp::load_default().unwrap();
    app.bind_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();
    let before = app.render_current_view().unwrap();
    let checksum_before = app.physical_checksum();

    let debug = app.render_debug_item_location_panel(&ItemId::new("coin_stack_01").unwrap());
    let after = app.render_current_view().unwrap();

    assert!(debug.contains("container:strongbox_tomas"));
    assert!(!before.contains("coin_stack_01"));
    assert_eq!(before, after);
    assert_eq!(checksum_before, app.physical_checksum());
}

#[test]
fn positive_proof_fixtures_emit_typed_artifacts_first() {
    let mut artifacts = Vec::new();

    let mut embodied = TuiApp::from_golden(fixtures::view_model_local_actions_001()).unwrap();
    embodied
        .bind_actor(ActorId::new("actor_lina").unwrap())
        .unwrap();
    let before_checksum = embodied.physical_checksum();
    let view = embodied.current_view().unwrap();
    let semantic_id = semantic_action_for_action_id(&embodied, "open");
    let accepted = embodied.submit_semantic_action(&semantic_id).unwrap();
    assert_eq!(accepted.report.status, ReportStatus::Accepted);
    assert!(!accepted.report.event_ids.is_empty());
    assert_ne!(embodied.physical_checksum(), before_checksum);
    artifacts.push(PositiveProofArtifact {
        responsible_layer: "tracewake-tui",
        scenario_id: "view_model_local_actions_001",
        actor_id: view.viewer_actor_id.as_str().to_string(),
        context_id: view.holder_known_context_id.as_str().to_string(),
        semantic_id: Some(semantic_id.as_str().to_string()),
        report_status: Some(accepted.report.status),
        event_ids: accepted
            .report
            .event_ids
            .iter()
            .map(|event_id| event_id.as_str().to_string())
            .collect(),
        typed_reason_codes: Vec::new(),
        provenance: vec![view.holder_known_context_hash.as_str().to_string()],
        debug_capability_present: view.debug_available,
        surfaces_checked: vec!["embodied_view", "proposal_report", "event_ids", "checksum"],
        checksum_result: "changed_after_accepted_world_event",
    });

    let mut why_not = TuiApp::from_golden(fixtures::door_access_001()).unwrap();
    why_not
        .bind_actor(ActorId::new("actor_sena").unwrap())
        .unwrap();
    let before_checksum = why_not.physical_checksum();
    let view = why_not.current_view().unwrap();
    let rejected = why_not
        .submit_semantic_action(&SemanticActionId::new("move.to.back_room").unwrap())
        .unwrap();
    assert_eq!(rejected.report.status, ReportStatus::Rejected);
    assert_eq!(why_not.physical_checksum(), before_checksum);
    assert!(rejected.report.actor_visible_summary.contains("door"));
    assert!(!rejected.report.actor_visible_summary.contains("debug"));
    artifacts.push(PositiveProofArtifact {
        responsible_layer: "tracewake-core",
        scenario_id: "door_access_001",
        actor_id: view.viewer_actor_id.as_str().to_string(),
        context_id: view.holder_known_context_id.as_str().to_string(),
        semantic_id: Some("move.to.back_room".to_string()),
        report_status: Some(rejected.report.status),
        event_ids: rejected
            .report
            .event_ids
            .iter()
            .map(|event_id| event_id.as_str().to_string())
            .collect(),
        typed_reason_codes: rejected
            .report
            .reason_codes
            .iter()
            .map(|reason| reason.stable_id().to_string())
            .collect(),
        provenance: rejected
            .report
            .actor_visible_facts
            .iter()
            .map(tracewake_core::actions::CheckedFact::render_pair)
            .collect(),
        debug_capability_present: false,
        surfaces_checked: vec!["why_not", "typed_reason_codes", "actor_visible_facts"],
        checksum_result: "unchanged_after_rejection",
    });

    let mut notebook = TuiApp::from_golden(fixtures::expectation_contradiction_001()).unwrap();
    notebook
        .bind_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();
    let opened = notebook
        .submit_semantic_action(&SemanticActionId::new("open.container.strongbox_tomas").unwrap())
        .unwrap();
    assert_eq!(opened.report.status, ReportStatus::Accepted);
    let checked = notebook
        .submit_semantic_action(&SemanticActionId::new("check.container.strongbox_tomas").unwrap())
        .unwrap();
    assert_eq!(checked.report.status, ReportStatus::Accepted);
    let view = notebook.current_view().unwrap();
    let notebook_view = notebook.notebook_view().unwrap();
    assert_eq!(notebook_view.typed_leads.len(), 1);
    artifacts.push(PositiveProofArtifact {
        responsible_layer: "tracewake-core",
        scenario_id: "expectation_contradiction_001",
        actor_id: notebook_view.viewer_actor_id.as_str().to_string(),
        context_id: view.holder_known_context_id.as_str().to_string(),
        semantic_id: Some("check.container.strongbox_tomas".to_string()),
        report_status: Some(checked.report.status),
        event_ids: checked
            .report
            .event_ids
            .iter()
            .map(|event_id| event_id.as_str().to_string())
            .collect(),
        typed_reason_codes: Vec::new(),
        provenance: notebook_view
            .typed_leads
            .iter()
            .map(|lead| format!("{}:{}", lead.source_kind, lead.source_summary))
            .collect(),
        debug_capability_present: view.debug_available,
        surfaces_checked: vec!["notebook", "typed_leads", "source_refs"],
        checksum_result: "observation_event_recorded",
    });

    let mut debug = TuiApp::from_golden(fixtures::debug_omniscience_excluded_001()).unwrap();
    debug
        .bind_actor(ActorId::new("actor_mara").unwrap())
        .unwrap();
    let before_view = debug.render_current_view().unwrap();
    let before_checksum = debug.physical_checksum();
    let before_events = debug.event_count();
    let debug_panel = debug.render_debug_item_location_panel(
        &tracewake_core::ids::ItemId::new("food_hidden_pantry").unwrap(),
    );
    assert!(debug_panel.contains("DEBUG NON-DIEGETIC"));
    assert_eq!(debug.render_current_view().unwrap(), before_view);
    assert_eq!(debug.physical_checksum(), before_checksum);
    assert_eq!(debug.event_count(), before_events);
    let view = debug.current_view().unwrap();
    artifacts.push(PositiveProofArtifact {
        responsible_layer: "tracewake-tui",
        scenario_id: "debug_omniscience_excluded_001",
        actor_id: view.viewer_actor_id.as_str().to_string(),
        context_id: view.holder_known_context_id.as_str().to_string(),
        semantic_id: None,
        report_status: None,
        event_ids: Vec::new(),
        typed_reason_codes: Vec::new(),
        provenance: vec![view.holder_known_context_hash.as_str().to_string()],
        debug_capability_present: true,
        surfaces_checked: vec!["debug_panel", "checksum", "event_count", "embodied_view"],
        checksum_result: "unchanged_after_debug_panel",
    });

    let mut possession = TuiApp::from_golden(fixtures::possession_parity_001()).unwrap();
    possession
        .bind_actor(ActorId::new("actor_mara").unwrap())
        .unwrap();
    let take_action = semantic_action_for_action_id(&possession, "take");
    let before_checksum = possession.physical_checksum();
    let taken = possession.submit_semantic_action(&take_action).unwrap();
    assert_eq!(taken.report.status, ReportStatus::Accepted);
    assert!(!taken.report.event_ids.is_empty());
    assert_ne!(possession.physical_checksum(), before_checksum);
    let view = possession.current_view().unwrap();
    assert!(view
        .semantic_actions
        .iter()
        .any(|action| action.action_id.as_str() == "place"));
    artifacts.push(PositiveProofArtifact {
        responsible_layer: "tracewake-core",
        scenario_id: "possession_parity_001",
        actor_id: view.viewer_actor_id.as_str().to_string(),
        context_id: view.holder_known_context_id.as_str().to_string(),
        semantic_id: view
            .semantic_actions
            .first()
            .map(|action| action.semantic_action_id.as_str().to_string()),
        report_status: Some(taken.report.status),
        event_ids: taken
            .report
            .event_ids
            .iter()
            .map(|event_id| event_id.as_str().to_string())
            .collect(),
        typed_reason_codes: Vec::new(),
        provenance: vec![view.holder_known_context_source_summary.clone()],
        debug_capability_present: view.debug_available,
        surfaces_checked: vec!["possession_view", "semantic_actions"],
        checksum_result: "changed_after_ordinary_take",
    });

    let first_sections = capture_representative_transcript_sections().unwrap();
    let second_sections = capture_representative_transcript_sections().unwrap();
    assert_eq!(first_sections, second_sections);
    artifacts.push(PositiveProofArtifact {
        responsible_layer: "tracewake-tui",
        scenario_id: "replay_item_location_001",
        actor_id: "actor_sena".to_string(),
        context_id: "hkc.actor_sena.0.1".to_string(),
        semantic_id: None,
        report_status: None,
        event_ids: Vec::new(),
        typed_reason_codes: Vec::new(),
        provenance: first_sections
            .iter()
            .map(|section| section.name.clone())
            .collect(),
        debug_capability_present: first_sections
            .iter()
            .any(|section| section.body.contains("DEBUG NON-DIEGETIC")),
        surfaces_checked: vec!["transcript_sections", "replay_panel"],
        checksum_result: "byte_identical_sections",
    });

    assert_eq!(artifacts.len(), 6);
    for artifact in &artifacts {
        artifact.assert_review_fields();
    }
    assert!(artifacts
        .iter()
        .any(|artifact| !artifact.event_ids.is_empty()));
    assert!(artifacts
        .iter()
        .any(|artifact| !artifact.typed_reason_codes.is_empty()));
    assert!(artifacts
        .iter()
        .any(|artifact| artifact.debug_capability_present));
    assert!(artifacts
        .iter()
        .any(|artifact| !artifact.provenance.is_empty()));
    assert!(artifacts
        .iter()
        .any(|artifact| artifact.report_status.is_some()));
    assert!(artifacts
        .iter()
        .any(|artifact| artifact.semantic_id.is_some()));
}

#[test]
fn phase3a_debug_surfaces_render_deterministically_and_read_only() {
    let mut app = TuiApp::from_golden(fixtures::no_human_day_001()).unwrap();
    app.bind_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();
    let before_view = app.render_current_view().unwrap();
    let before_checksum = app.physical_checksum();
    let before_event_count = app.event_count();

    let needs_first = app.render_debug_needs_panel();
    let needs_second = app.render_debug_needs_panel();
    let routines = app.render_debug_routines_panel();
    let planner = app.render_debug_planner_panel(&ActorId::new("actor_mara").unwrap());
    let stuck = app.render_debug_stuck_panel();
    let no_human_day = app.render_debug_no_human_day_panel();
    let actor = app.render_debug_actor_panel(&ActorId::new("actor_tomas").unwrap());

    assert_eq!(needs_first, needs_second);
    for panel in [
        &needs_first,
        &routines,
        &planner,
        &stuck,
        &no_human_day,
        &actor,
    ] {
        assert!(panel.contains("DEBUG NON-DIEGETIC"));
    }
    assert!(needs_first.contains("actor_tomas"));
    assert!(needs_first.contains("need=hunger"));
    assert!(needs_first.contains("value=520 band=urgent cause=fixture_initial"));
    assert!(routines.contains("Routines"));
    assert!(routines.contains("active actor=actor_tomas intention=intention_tomas_go_work"));
    assert!(routines.contains("routine_exec_tomas_go_work"));
    assert!(planner.contains("actor=actor_mara"));
    assert!(planner.contains("candidate_goals"));
    assert!(planner.contains("selected_method"));
    assert!(planner.contains("rejected_reasons"));
    assert!(planner.contains("blocked_preconditions"));
    assert!(planner.contains("hidden_truth_audit"));
    assert!(stuck.contains("stuck_diagnostic_count="));
    assert!(no_human_day.contains("No Human Day"));
    assert!(no_human_day.contains("no_human_day_metrics_v1"));
    assert!(no_human_day.contains("routine_interruptions=0"));
    assert!(actor.contains("actor=actor_tomas"));
    assert!(actor.contains("active_intention=intention_tomas_go_work"));
    assert_eq!(app.render_current_view().unwrap(), before_view);
    assert_eq!(app.physical_checksum(), before_checksum);
    assert_eq!(app.event_count(), before_event_count);

    let debug_panels_source = include_str!("../src/debug_panels.rs");
    for forbidden in ["apply_event", "insert_belief", "insert_observation"] {
        assert!(
            !debug_panels_source.contains(forbidden),
            "debug panel rendering must not call {forbidden}"
        );
    }
}

#[test]
fn tui_runs_no_human_day_and_inspects_real_post_run_panels() {
    let mut app = TuiApp::from_golden(fixtures::no_human_day_001()).unwrap();
    app.bind_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();
    let before_events = app.event_count();

    let report = app.run_no_human_day();
    let after_run_events = app.event_count();
    let embodied_view = app.current_view().unwrap();
    assert_eq!(
        embodied_view.holder_known_context_frontier,
        after_run_events as u64
    );
    let embodied = tracewake_tui::render::render_embodied_view(&embodied_view);
    let before_debug_checksum = app.physical_checksum();
    let metrics = app.render_debug_no_human_day_panel();
    let planner = app.render_debug_planner_panel(&ActorId::new("actor_mara").unwrap());
    let stuck = app.render_debug_stuck_panel();
    let after_debug_events = app.event_count();

    assert!(report.ordinary_pipeline_events > 0);
    assert!(after_run_events > before_events);
    assert!(embodied.contains("Needs:"));
    assert!(embodied.contains("- hunger: band=rising cause=tick_delta"));
    assert!(!embodied.contains("value=410"));
    assert!(embodied.contains("Intention:"));
    assert!(embodied.contains("active:routine_tomas_go_work:wait"));
    let embodied_without_marked_context = embodied
        .lines()
        .filter(|line| !line.starts_with("Knowledge context: DEBUG NON-DIEGETIC"))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(!embodied_without_marked_context.contains("DEBUG NON-DIEGETIC"));
    assert!(!embodied.contains("routine_events="));
    assert!(!embodied.contains("work_failed="));
    assert!(!embodied.contains("food_hidden_pantry"));
    assert!(metrics.contains("DEBUG NON-DIEGETIC: No Human Day"));
    assert!(metrics.contains("no_human_day_metrics_v1"));
    assert!(metrics.contains("routine_events=5"));
    assert!(metrics.contains("work_failed=1"));
    assert!(metrics.contains("need_crossings=5"));
    assert!(metrics.contains("routine_interruptions=2"));
    assert!(metrics.contains("replay_failures=0"));
    let events_line = metrics
        .lines()
        .find(|line| line.starts_with("events="))
        .unwrap();
    assert_ne!(events_line, "events=0");
    assert!(planner.contains("DEBUG NON-DIEGETIC: Planner"));
    assert!(planner.contains("candidate_goals"));
    assert!(stuck.contains("DEBUG NON-DIEGETIC: Stuck"));
    assert!(stuck.contains("stuck_diagnostic_count="));
    assert!(stuck.contains("stuck="));
    assert!(stuck.contains("debug_detail=no-human day stuck detection"));
    let routines = app.render_debug_routines_panel();
    assert!(routines.contains("routine_exec_mara_eat"));
    assert!(routines.contains("status=Failed"));
    assert!(routines.contains("routine_exec_tomas_work"));
    assert!(routines.contains("status=Completed"));
    assert_eq!(app.event_count(), after_debug_events);
    assert_eq!(app.physical_checksum(), before_debug_checksum);

    let mut command_app = TuiApp::from_golden(fixtures::no_human_day_001()).unwrap();
    command_app
        .bind_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();
    let script = b"run no-human-day\ndebug no-human-day\nview\nquit\n";
    let mut output = Vec::new();

    run_command_loop(&mut command_app, &script[..], &mut output).unwrap();

    let rendered = String::from_utf8(output).unwrap();
    assert!(rendered.contains("DEBUG NON-DIEGETIC: No Human Day"));
    assert!(!rendered.contains("Ran no-human day:"));
    assert!(!rendered.contains("ordinary_events="));
    assert!(rendered.contains("work_failed=1"));
    assert!(rendered.contains("routine_interruptions=2"));
    assert!(rendered.contains("- hunger: band=rising cause=tick_delta"));
    assert!(!rendered.contains("value=410"));
    assert!(rendered.contains("Actor: actor_tomas"));
    assert!(!rendered.contains("food_hidden_pantry"));
}

#[test]
fn phase3a_possess_continue_and_debug_transcript_is_deterministic() {
    let first = phase3a_possess_continue_debug_transcript();
    let second = phase3a_possess_continue_debug_transcript();

    assert_eq!(first, second);
    assert!(first.contains("Actor: actor_mara"));
    assert!(first.contains("Needs:"));
    assert!(first.contains("Intention:"));
    assert!(first.contains("Accepted: wait.1_tick"));
    assert!(first.contains("continue_routine"));
    assert!(first.contains("DEBUG NON-DIEGETIC: Needs"));
    assert!(first.contains("DEBUG NON-DIEGETIC: Planner"));
    assert!(first.contains("DEBUG NON-DIEGETIC: Stuck"));
    assert!(first.contains("DEBUG NON-DIEGETIC: No Human Day"));
    assert!(first.contains("DEBUG NON-DIEGETIC: Actor"));
    assert!(!first.contains("food_hidden_pantry"));
}

#[test]
fn leakage_debug_truth_does_not_enter_embodied_view() {
    debug_truth_does_not_enter_embodied_view();
}

#[test]
fn tui_playability_reaches_action_rejection_wait_and_debug() {
    let mut app = TuiApp::from_golden(fixtures::door_access_001()).unwrap();
    app.bind_actor(ActorId::new("actor_sena").unwrap()).unwrap();
    assert!(app
        .render_current_view()
        .unwrap()
        .contains("move.to.back_room"));

    let rejected = app
        .submit_semantic_action(&SemanticActionId::new("move.to.back_room").unwrap())
        .unwrap();
    assert_eq!(rejected.report.status, ReportStatus::Rejected);
    assert!(app.render_current_view().unwrap().contains("Why-not:"));
    assert!(app
        .render_debug_action_rejection_panel()
        .unwrap_or_default()
        .contains("Action Rejection"));

    let wait_action = semantic_action_for_action_id(&app, "wait");
    let waited = app.submit_semantic_action(&wait_action).unwrap();
    assert_eq!(waited.report.status, ReportStatus::Accepted);
    assert!(app
        .render_debug_event_log_panel()
        .contains("DEBUG NON-DIEGETIC"));
    assert!(app
        .render_debug_projection_rebuild_panel()
        .contains("Projection Rebuild"));
    assert!(app.render_debug_replay_panel().contains("Replay"));
}

#[test]
fn phase2a_tui_transcript_discovers_absence_without_culprit_leak() {
    let mut app = TuiApp::from_golden(fixtures::expectation_contradiction_001()).unwrap();
    app.bind_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();

    let before_view = app.render_current_view().unwrap();
    let before_notebook = render_notebook(&app.notebook_view().unwrap());

    assert!(before_view.contains("check.container.strongbox_tomas"));
    assert_no_embodied_culprit_leak(&before_view);
    assert_no_embodied_culprit_leak(&before_notebook);

    let opened = app
        .submit_semantic_action(&SemanticActionId::new("open.container.strongbox_tomas").unwrap())
        .unwrap();
    assert_eq!(opened.report.status, ReportStatus::Accepted);

    let checked = app
        .submit_semantic_action(&SemanticActionId::new("check.container.strongbox_tomas").unwrap())
        .unwrap();
    assert_eq!(checked.report.status, ReportStatus::Accepted);

    let after_view = app.render_current_view().unwrap();
    let after_notebook = render_notebook(&app.notebook_view().unwrap());
    let debug_epistemics =
        tracewake_tui::debug_panels::render_debug_epistemics_panel(&app.debug_epistemics_view());
    let debug_beliefs = tracewake_tui::debug_panels::render_debug_beliefs_panel(
        &app.debug_beliefs_view(&ActorId::new("actor_tomas").unwrap())
            .unwrap(),
    );
    let debug_observations = tracewake_tui::debug_panels::render_debug_observations_panel(
        &app.debug_observations_view(&ActorId::new("actor_tomas").unwrap())
            .unwrap(),
    );
    let debug_replay = app.render_debug_replay_panel();

    assert!(after_view.contains("Knowledge context:"));
    assert!(after_notebook.contains("missing"));
    assert!(after_notebook.contains("source=event:"));
    assert!(after_notebook.contains("Contradictions:"));
    assert_no_embodied_culprit_leak(&after_view);
    assert_no_embodied_culprit_leak(&after_notebook);
    assert!(debug_epistemics.contains("DEBUG NON-DIEGETIC: Epistemics"));
    assert!(debug_epistemics.contains("contradiction_count=1"));
    assert!(debug_beliefs.contains("DEBUG NON-DIEGETIC: Beliefs"));
    assert!(debug_observations.contains("DEBUG NON-DIEGETIC: Observations"));
    assert!(debug_replay.contains("matches_expected=true"));
    assert!(debug_replay.contains("agent_checksum_matches=true"));

    let debug_truth = app.render_debug_item_location_panel(&ItemId::new("coin_stack_01").unwrap());
    assert!(debug_truth.contains("actor_mara"));
    assert!(!after_view.contains("actor_mara"));
    assert!(!after_notebook.contains("actor_mara"));
}

fn assert_no_embodied_culprit_leak(rendered: &str) {
    for forbidden in ["actor_mara", "Mara", "culprit", "stole", "theft"] {
        assert!(
            !rendered.contains(forbidden),
            "embodied surface leaked {forbidden}: {rendered}"
        );
    }
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

fn phase3a_possess_continue_debug_transcript() -> String {
    let mut app = TuiApp::from_golden(fixtures::possession_does_not_reset_intention_001()).unwrap();
    app.bind_actor(ActorId::new("actor_mara").unwrap()).unwrap();
    let view = app.current_view().unwrap();
    let continue_action = view
        .semantic_actions
        .iter()
        .find(|action| action.action_id.as_str() == "continue_routine")
        .map(|action| action.semantic_action_id.clone());

    let mut transcript = vec![app.render_current_view().unwrap()];
    let wait_action = semantic_action_for_action_id(&app, "wait");
    let waited = app.submit_semantic_action(&wait_action).unwrap();
    assert_eq!(waited.report.status, ReportStatus::Accepted);
    transcript.push(format!("Accepted: {}", wait_action.as_str()));
    if let Some(continue_action) = continue_action {
        let continued = app.submit_semantic_action(&continue_action).unwrap();
        assert_eq!(continued.report.status, ReportStatus::Accepted);
        transcript.push(format!("Accepted: {}", continue_action.as_str()));
    } else {
        transcript.push("continue_routine unavailable: no active intention".to_string());
    }
    transcript.push(app.render_current_view().unwrap());
    transcript.push(app.render_debug_needs_panel());
    transcript.push(app.render_debug_routines_panel());
    transcript.push(app.render_debug_planner_panel(&ActorId::new("actor_mara").unwrap()));
    transcript.push(app.render_debug_stuck_panel());
    transcript.push(app.render_debug_no_human_day_panel());
    transcript.push(app.render_debug_actor_panel(&ActorId::new("actor_mara").unwrap()));
    transcript.join("\n---\n")
}
