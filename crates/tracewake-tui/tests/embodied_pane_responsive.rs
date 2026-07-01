use ratatui::layout::Rect;
use tracewake_content::fixtures;
use tracewake_core::ids::ActorId;
use tracewake_tui::app::TuiApp;
use tracewake_tui::screen::{
    buffer_to_plain_text, build_embodied_screen_model, expanded_region_lines,
    render_embodied_to_buffer, visible_region_lines, EmbodiedScreenModel, PaneRegion,
    PaneRegionBinding, RenderOptions, TerminalSize, MINIMUM_PANE_COLUMNS, MINIMUM_PANE_ROWS,
};

#[test]
fn narrow_60x20_retains_why_not_actions_and_needs() {
    let text = buffer_text_for_size(60, 20);

    assert!(text.contains("[Details / Why-not]"));
    assert!(text.contains("why_not:"));
    assert!(text.contains("- none"));
    assert!(text.contains("[Actions / Affordances]"));
    assert!(text.contains("4. Wait [wait.1_tick]"));
    assert!(text.contains("[Self / Body / Routine]"));
    assert!(text.contains("Needs:"));
    assert!(text.contains("- fatigue band=comfortable cause=fixture_initial"));
    assert!(text.contains("- hunger band=comfortable cause=fixture_initial"));
    assert!(text.contains("- safety band=comfortable cause=fixture_initial"));
}

#[test]
fn overflow_emits_marker_and_expansion_recovers_elided_actions() {
    let binding = overflowing_actions_binding();

    let visible = visible_region_lines(&binding, 3).unwrap();

    assert_eq!(visible[0], "actions:");
    assert_eq!(visible[1], "1. Wait [wait.1_tick]");
    assert_eq!(visible[2], "... 2 more; focus details to inspect");
    assert_eq!(expanded_region_lines(&binding), binding.lines);
}

#[test]
fn non_vacuity_fails_when_actions_clip_to_no_visible_lines() {
    let err = visible_region_lines(&overflowing_actions_binding(), 0).unwrap_err();

    assert_eq!(err.region, PaneRegion::ActionsAffordances);
    assert_eq!(
        err.reason,
        "safety-critical region clipped to zero visible lines"
    );
}

#[test]
fn below_floor_rendering_yields_actor_safe_message() {
    let screen = ordinary_workday_screen(MINIMUM_PANE_COLUMNS - 1, MINIMUM_PANE_ROWS);
    let text = buffer_to_plain_text(&render_embodied_to_buffer(
        &screen,
        Rect::new(0, 0, MINIMUM_PANE_COLUMNS - 1, MINIMUM_PANE_ROWS),
    ));

    assert!(text.contains("Screen too small for embodied panes"));
    assert!(!text.contains("debug_only"));
}

fn overflowing_actions_binding() -> PaneRegionBinding {
    PaneRegionBinding {
        region: PaneRegion::ActionsAffordances,
        title: "Actions / Affordances",
        lines: vec![
            "actions:".to_string(),
            "1. Wait [wait.1_tick]".to_string(),
            "2. Sleep here [sleep.here]".to_string(),
            "3. Move [move.to.workshop]".to_string(),
        ],
    }
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
