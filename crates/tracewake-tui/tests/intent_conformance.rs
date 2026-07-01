use tracewake_content::fixtures;
use tracewake_core::debug_capability::DEBUG_NON_DIEGETIC_MARKER;
use tracewake_core::ids::ActorId;
use tracewake_tui::app::TuiApp;
use tracewake_tui::input::DebugCommand;
use tracewake_tui::intent::headless::{run_key_script, run_semantic_script};
use tracewake_tui::intent::reducer::{reduce, ReduceOutcome};
use tracewake_tui::intent::{PresentationState, UiIntent};
use tracewake_tui::screen::{
    build_embodied_screen_model, render_embodied_screen_dump, RenderOptions, TerminalSize,
};

#[test]
fn key_and_semantic_scripts_produce_byte_identical_screen_dumps() {
    let key = run_key_script(&mut bound_app(), "Tab w n", TerminalSize::new(80, 24)).unwrap();
    let semantic = run_semantic_script(
        &mut bound_app(),
        "focus exits\nwait_one_tick\nnotebook\n",
        TerminalSize::new(80, 24),
    )
    .unwrap();

    assert_eq!(key.dumps, semantic.dumps);
}

#[test]
fn driven_headless_flow_records_focus_selection_action_time_and_notebook() {
    let script = "\
focus actions
move down
submit semantic_action_id=open.container.strongbox_tomas
wait_one_tick
continue_until max_ticks=1
notebook
";
    let run = run_semantic_script(&mut bound_app(), script, TerminalSize::new(80, 24)).unwrap();

    assert_eq!(run.dumps.len(), 6);
    assert!(run.transcript.contains("input: focus actions"));
    assert!(run.transcript.contains("intent: MoveSelection(Down)"));
    assert!(run
        .transcript
        .contains("semantic_action_id: open.container.strongbox_tomas"));
    assert!(run.transcript.contains("input: wait_one_tick"));
    assert!(run.transcript.contains("input: continue_until max_ticks=1"));
    assert!(run.transcript.contains("focused_pane: Notebook"));
    assert!(run.transcript.contains("rendered_pane_text:"));
}

#[test]
fn debug_authority_intent_is_marked_and_does_not_enter_embodied_dump() {
    let mut app = TuiApp::from_golden_operator_debug(fixtures::strongbox_001()).unwrap();
    app.bind_debug_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();
    let mut state = PresentationState::default();

    let outcome = reduce(
        &mut app,
        &mut state,
        UiIntent::SubmitDebugCommand(DebugCommand::Overlay),
    )
    .unwrap();
    let ReduceOutcome::DebugRendered(debug_output) = outcome else {
        panic!("debug intent returns debug output");
    };
    assert!(debug_output.contains(DEBUG_NON_DIEGETIC_MARKER));

    let view = app.current_view().unwrap();
    let screen = build_embodied_screen_model(
        &view,
        RenderOptions {
            terminal_size: TerminalSize::new(80, 24),
            focused_pane: state.focused_pane(),
            ..RenderOptions::default()
        },
    );
    let dump = render_embodied_screen_dump(&screen);
    assert!(!dump.contains(DEBUG_NON_DIEGETIC_MARKER));
}

#[test]
fn reducer_source_has_no_direct_world_mutation_path() {
    let source = include_str!("../src/intent/reducer.rs");
    for forbidden in [
        "RuntimeCommand::",
        ".submit_command(",
        "EventLog",
        "apply_event",
        "append_event",
        "events::apply",
    ] {
        assert!(
            !source.contains(forbidden),
            "intent reducer must not contain direct world mutation token {forbidden}"
        );
    }
    for required in [
        ".submit_semantic_action(",
        ".advance_until(",
        "run::render_debug(",
    ] {
        assert!(
            source.contains(required),
            "intent reducer must route through authorized seam {required}"
        );
    }
}

#[test]
fn same_script_yields_byte_identical_transcript_and_dumps() {
    let script = "focus actions\nmove down\nwait_one_tick\nnotebook\n";

    let first = run_semantic_script(&mut bound_app(), script, TerminalSize::new(80, 24)).unwrap();
    let second = run_semantic_script(&mut bound_app(), script, TerminalSize::new(80, 24)).unwrap();

    assert_eq!(first, second);
}

fn bound_app() -> TuiApp {
    let mut app = TuiApp::from_golden(fixtures::strongbox_001()).unwrap();
    app.bind_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();
    app
}
