use tracewake_content::fixtures;
use tracewake_core::ids::ActorId;
use tracewake_tui::app::TuiApp;
use tracewake_tui::transcript::capture_representative_transcript;

#[test]
fn transcript_snapshot_is_byte_identical_across_runs() {
    let first = capture_representative_transcript().unwrap();
    let second = capture_representative_transcript().unwrap();

    assert_eq!(first.as_bytes(), second.as_bytes());
    assert!(first.contains("== view.initial =="));
    assert!(first.contains("== notebook.actor_sena =="));
    assert!(first.contains("Why-not:"));
    assert!(first.contains("DEBUG NON-DIEGETIC: Event Log"));
    assert!(first.contains("DEBUG NON-DIEGETIC: Replay"));
    assert!(first.contains("DEBUG NON-DIEGETIC: Epistemics"));
    assert!(first.contains("DEBUG NON-DIEGETIC: Beliefs"));
    assert!(first.contains("DEBUG NON-DIEGETIC: Observations"));
}

#[test]
fn phase3a_debug_snapshot_is_byte_identical_across_runs() {
    let first = capture_phase3a_debug_snapshot();
    let second = capture_phase3a_debug_snapshot();

    assert_eq!(first.as_bytes(), second.as_bytes());
    assert!(first.contains("DEBUG NON-DIEGETIC: Needs"));
    assert!(first.contains("DEBUG NON-DIEGETIC: Routines"));
    assert!(first.contains("DEBUG NON-DIEGETIC: Planner"));
    assert!(first.contains("DEBUG NON-DIEGETIC: Stuck"));
    assert!(first.contains("DEBUG NON-DIEGETIC: No Human Day"));
}

fn capture_phase3a_debug_snapshot() -> String {
    let mut app = TuiApp::from_golden(fixtures::no_human_day_001()).unwrap();
    app.bind_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();
    [
        app.render_current_view().unwrap(),
        app.render_debug_needs_panel(),
        app.render_debug_routines_panel(),
        app.render_debug_planner_panel(&ActorId::new("actor_mara").unwrap()),
        app.render_debug_stuck_panel(),
        app.render_debug_no_human_day_panel(),
        app.render_debug_actor_panel(&ActorId::new("actor_tomas").unwrap()),
    ]
    .join("\n---\n")
}
