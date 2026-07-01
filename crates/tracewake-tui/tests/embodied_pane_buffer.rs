use ratatui::layout::Rect;
use tracewake_content::fixtures;
use tracewake_core::ids::ActorId;
use tracewake_tui::app::TuiApp;
use tracewake_tui::screen::{
    buffer_to_plain_text, build_embodied_screen_model, render_embodied_to_buffer,
    render_pane_region_bindings, EmbodiedScreenModel, RenderOptions, TerminalSize,
};

const GOLDEN_80X24: &str = include_str!("goldens/embodied_pane_80x24.txt");
const GOLDEN_100X30: &str = include_str!("goldens/embodied_pane_100x30.txt");
const GOLDEN_60X20: &str = include_str!("goldens/embodied_pane_60x20.txt");

#[test]
fn embodied_pane_text_goldens_match_fixed_sizes() {
    maybe_print_current_outputs();

    assert_eq!(pane_text_for_size(80, 24), GOLDEN_80X24.trim_end());
    assert_eq!(pane_text_for_size(100, 30), GOLDEN_100X30.trim_end());
    assert_eq!(pane_text_for_size(60, 20), GOLDEN_60X20.trim_end());
}

#[test]
fn embodied_pane_buffer_snapshots_match_fixed_sizes() {
    insta::assert_snapshot!("embodied_pane_buffer_80x24", buffer_text_for_size(80, 24));
    insta::assert_snapshot!("embodied_pane_buffer_100x30", buffer_text_for_size(100, 30));
    insta::assert_snapshot!("embodied_pane_buffer_60x20", buffer_text_for_size(60, 20));
}

#[test]
fn embodied_pane_buffer_is_byte_deterministic() {
    assert_eq!(buffer_text_for_size(100, 30), buffer_text_for_size(100, 30));
}

#[test]
fn embodied_pane_buffer_and_text_expose_same_actor_safe_content() {
    let pane_text = pane_text_for_size(100, 30);
    let buffer_text = buffer_text_for_size(100, 30);

    for expected in [
        "actor=actor_tomas mode=Embodied focus=place debug_available=false",
        "place:",
        "Tomas home (home_tomas)",
        "actions:",
        "4. Wait [wait.1_tick]",
        "status:",
        "Needs:",
        "actors:",
        "- none",
        "why_not:",
        "- none",
    ] {
        assert!(pane_text.contains(expected), "pane text missing {expected}");
        assert!(
            buffer_text.contains(expected),
            "buffer text missing {expected}"
        );
    }
}

fn pane_text_for_size(columns: u16, rows: u16) -> String {
    let screen = ordinary_workday_screen(columns, rows);
    render_pane_region_bindings(&screen)
        .into_iter()
        .flat_map(|binding| {
            let mut lines = vec![format!("REGION {}", binding.title)];
            lines.extend(binding.lines);
            lines
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn buffer_text_for_size(columns: u16, rows: u16) -> String {
    let screen = ordinary_workday_screen(columns, rows);
    let buffer = render_embodied_to_buffer(&screen, Rect::new(0, 0, columns, rows));
    buffer_to_plain_text(&buffer)
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

fn maybe_print_current_outputs() {
    if std::env::var_os("TRACEWAKE_PRINT_PANE_GOLDENS").is_none() {
        return;
    }

    println!("--- embodied_pane_80x24.txt ---");
    println!("{}", pane_text_for_size(80, 24));
    println!("--- embodied_pane_100x30.txt ---");
    println!("{}", pane_text_for_size(100, 30));
    println!("--- embodied_pane_60x20.txt ---");
    println!("{}", pane_text_for_size(60, 20));
}
