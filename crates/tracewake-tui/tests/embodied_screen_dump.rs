use tracewake_content::fixtures;
use tracewake_core::debug_capability::DEBUG_NON_DIEGETIC_MARKER;
use tracewake_core::ids::ActorId;
use tracewake_tui::app::TuiApp;
use tracewake_tui::screen::{
    build_embodied_screen_dump, build_embodied_screen_model, render_embodied_screen_dump,
    EmbodiedScreenModel, RenderOptions, ScreenDump, TerminalSize,
};

const GOLDEN_80X24: &str = include_str!("goldens/embodied_screen_80x24.txt");
const GOLDEN_100X30: &str = include_str!("goldens/embodied_screen_100x30.txt");
const GOLDEN_60X20: &str = include_str!("goldens/embodied_screen_60x20.txt");
const SNAPSHOT: &str = include_str!("goldens/embodied_screen_dump.snapshot.txt");

#[test]
fn embodied_screen_plain_text_goldens_match_fixed_sizes() {
    maybe_print_current_outputs();

    assert_eq!(dump_for_size(80, 24), GOLDEN_80X24.trim_end());
    assert_eq!(dump_for_size(100, 30), GOLDEN_100X30.trim_end());
    assert_eq!(dump_for_size(60, 20), GOLDEN_60X20.trim_end());
}

#[test]
fn embodied_screen_structured_snapshot_matches() {
    let screen = ordinary_workday_screen(80, 24);
    let snapshot = render_structured_snapshot(&build_embodied_screen_dump(&screen));

    assert_eq!(snapshot, SNAPSHOT.trim_end());
}

#[test]
fn embodied_screen_dumps_never_carry_debug_non_diegetic_marker() {
    let screen = ordinary_workday_screen(80, 24);
    let plain = render_embodied_screen_dump(&screen);
    let structured = build_embodied_screen_dump(&screen);

    assert!(!plain.contains(DEBUG_NON_DIEGETIC_MARKER));
    assert!(!structured.debug_marker_present);
    assert!(!structured
        .panes
        .iter()
        .flat_map(|pane| pane.lines.iter())
        .any(|line| line.contains(DEBUG_NON_DIEGETIC_MARKER)));
}

#[test]
fn embodied_screen_build_and_dump_are_byte_deterministic() {
    let first = ordinary_workday_screen(100, 30);
    let second = ordinary_workday_screen(100, 30);

    assert_eq!(first, second);
    assert_eq!(
        render_embodied_screen_dump(&first),
        render_embodied_screen_dump(&second)
    );
    assert_eq!(
        build_embodied_screen_dump(&first),
        build_embodied_screen_dump(&second)
    );
}

fn dump_for_size(columns: u16, rows: u16) -> String {
    render_embodied_screen_dump(&ordinary_workday_screen(columns, rows))
}

fn ordinary_workday_screen(columns: u16, rows: u16) -> EmbodiedScreenModel {
    let mut app = TuiApp::from_golden(fixtures::ordinary_workday_001()).unwrap();
    app.bind_actor(ActorId::new("actor_tomas").unwrap())
        .unwrap();
    let view = app.current_view().unwrap();
    build_embodied_screen_model(
        &view,
        RenderOptions {
            terminal_size: TerminalSize::new(columns, rows),
            ..RenderOptions::default()
        },
    )
}

fn render_structured_snapshot(dump: &ScreenDump) -> String {
    let mut lines = Vec::new();
    lines.push(format!("mode={:?}", dump.mode));
    lines.push(format!(
        "terminal_size={}x{}",
        dump.terminal_size.columns, dump.terminal_size.rows
    ));
    lines.push(format!("focused_pane={:?}", dump.focused_pane));
    lines.push(format!("view_model_id={}", dump.view_model_id));
    lines.push(format!(
        "holder_known_context_hash={}",
        dump.holder_known_context_hash
    ));
    lines.push(format!(
        "debug_marker_present={}",
        dump.debug_marker_present
    ));
    lines.push(format!("action_refs={}", dump.action_refs.join(",")));
    for pane in &dump.panes {
        lines.push(format!("pane={}", pane.name));
        for line in &pane.lines {
            lines.push(format!("  {line}"));
        }
    }
    lines.join("\n")
}

fn maybe_print_current_outputs() {
    if std::env::var_os("TRACEWAKE_PRINT_SCREEN_GOLDENS").is_none() {
        return;
    }

    println!("--- embodied_screen_80x24.txt ---");
    println!("{}", dump_for_size(80, 24));
    println!("--- embodied_screen_100x30.txt ---");
    println!("{}", dump_for_size(100, 30));
    println!("--- embodied_screen_60x20.txt ---");
    println!("{}", dump_for_size(60, 20));
    println!("--- embodied_screen_dump.snapshot.txt ---");
    let screen = ordinary_workday_screen(80, 24);
    println!(
        "{}",
        render_structured_snapshot(&build_embodied_screen_dump(&screen))
    );
}
