use tracewake_content::fixtures;
use tracewake_core::actions::ReportStatus;
use tracewake_core::ids::{ActorId, ItemId, SemanticActionId};
use tracewake_tui::app::TuiApp;
use tracewake_tui::input::semantic_id_for_selection;
use tracewake_tui::render::render_notebook;
use tracewake_tui::run::run_command_loop;
use tracewake_tui::transcript::capture_representative_transcript;

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
    let embodied = app.render_current_view().unwrap();
    let after_run_events = app.event_count();
    let before_debug_checksum = app.physical_checksum();
    let metrics = app.render_debug_no_human_day_panel();
    let planner = app.render_debug_planner_panel(&ActorId::new("actor_mara").unwrap());
    let stuck = app.render_debug_stuck_panel();
    let after_debug_events = app.event_count();

    assert!(report.ordinary_pipeline_events > 0);
    assert!(after_run_events > before_events);
    assert!(embodied.contains("Needs:"));
    assert!(embodied.contains("- hunger: value=400 band=rising cause=tick_delta"));
    assert!(embodied.contains("Intention:"));
    assert!(embodied.contains("active:routine_tomas_go_work:work_block"));
    assert!(!embodied.contains("food_hidden_pantry"));
    assert!(metrics.contains("DEBUG NON-DIEGETIC: No Human Day"));
    assert!(metrics.contains("no_human_day_metrics_v1"));
    assert!(metrics.contains("routine_events=9"));
    assert!(metrics.contains("work_failed=4"));
    assert!(metrics.contains("need_crossings=2"));
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
    assert!(stuck.contains("stuck_diagnostic_count=0"));
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
    assert!(rendered.contains("Ran no-human day:"));
    assert!(rendered.contains("ordinary_events="));
    assert!(rendered.contains("DEBUG NON-DIEGETIC: No Human Day"));
    assert!(rendered.contains("work_failed=4"));
    assert!(rendered.contains("routine_interruptions=2"));
    assert!(rendered.contains("- hunger: value=400 band=rising cause=tick_delta"));
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

    let waited = app
        .submit_semantic_action(&SemanticActionId::new("wait.1_tick").unwrap())
        .unwrap();
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
    let waited = app
        .submit_semantic_action(&SemanticActionId::new("wait.1_tick").unwrap())
        .unwrap();
    assert_eq!(waited.report.status, ReportStatus::Accepted);
    transcript.push("Accepted: wait.1_tick".to_string());
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
