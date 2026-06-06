use tracewake_content::fixtures;
use tracewake_core::actions::ReportStatus;
use tracewake_core::ids::{ActorId, SemanticActionId};
use tracewake_tui::app::TuiApp;

#[test]
fn bind_render_submit_rerender_and_show_why_not() {
    let mut app = TuiApp::load_default().unwrap();
    app.bind_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();

    let first = app.render_current_view().unwrap();
    assert!(first.contains("Actor: actor_tomas"));
    assert!(first.contains("open.container.strongbox_tomas"));

    let accepted = app
        .submit_semantic_action(&SemanticActionId::new("open.container.strongbox_tomas").unwrap())
        .unwrap();
    assert_eq!(accepted.report.status, ReportStatus::Accepted);

    let second = app.render_current_view().unwrap();
    assert!(second.contains("coin_stack_01"));

    let mut door_app = TuiApp::from_golden(fixtures::door_access_001()).unwrap();
    door_app
        .bind_actor(ActorId::new("actor_sena").unwrap())
        .unwrap();
    let rejected = door_app
        .submit_semantic_action(&SemanticActionId::new("move.to.back_room").unwrap())
        .unwrap();
    assert_eq!(rejected.report.status, ReportStatus::Rejected);

    let why_not = door_app.render_current_view().unwrap();
    assert!(why_not.contains("Why-not:"));
    assert!(why_not.contains(&rejected.report.actor_visible_summary));
}

#[test]
fn tui_does_not_call_event_applier() {
    let app_source = include_str!("../src/app.rs");
    let render_source = include_str!("../src/render.rs");
    let input_source = include_str!("../src/input.rs");

    assert!(!app_source.contains("apply_event"));
    assert!(!render_source.contains("apply_event"));
    assert!(!input_source.contains("apply_event"));
}
