use tracewake_content::fixtures;
use tracewake_core::actions::ReportStatus;
use tracewake_core::ids::{ActorId, ItemId, SemanticActionId};
use tracewake_tui::app::TuiApp;
use tracewake_tui::input::semantic_id_for_selection;
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
