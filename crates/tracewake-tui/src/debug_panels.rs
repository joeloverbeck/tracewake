use tracewake_core::debug_reports::{
    ActionRejectionDebugReport, ControllerBindingDebugReport, ItemLocationDebugReport,
    ProjectionRebuildDebugReport, ReplayDebugReport,
};
use tracewake_core::view_models::DebugEventLogView;

const DEBUG_MARKER: &str = "DEBUG NON-DIEGETIC";

pub fn render_item_location_panel(report: &ItemLocationDebugReport) -> String {
    assert!(report.debug_only);
    lines(
        "Item Location",
        vec![
            format!("item={}", report.item_id.as_str()),
            format!("exists={}", report.exists),
            format!(
                "current_location={}",
                report.current_location.as_deref().unwrap_or("missing")
            ),
            format!(
                "last_location_event={}",
                report
                    .last_location_event_id
                    .as_ref()
                    .map(|id| id.as_str())
                    .unwrap_or("none")
            ),
            format!("checksum={}", report.physical_checksum.as_str()),
        ],
    )
}

pub fn render_action_rejection_panel(report: &ActionRejectionDebugReport) -> String {
    assert!(report.debug_only);
    lines(
        "Action Rejection",
        vec![
            format!("proposal={}", report.proposal_id.as_str()),
            format!("validation={}", report.validation_report_id.as_str()),
            format!("failed_stage={:?}", report.failed_stage),
            format!(
                "reason_codes={}",
                report
                    .reason_codes
                    .iter()
                    .map(|reason| reason.stable_id())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            format!("actor_summary={}", report.actor_visible_summary),
            format!("debug_summary={}", report.debug_summary),
            format!("mutation_attempted={}", report.mutation_attempted),
        ],
    )
}

pub fn render_event_log_panel(view: &DebugEventLogView) -> String {
    assert!(view.debug_only);
    let mut rows = vec![format!("event_count={}", view.events.len())];
    rows.extend(view.events.iter().map(|event| {
        format!(
            "{}:{}:{}:{:?}",
            event.global_order, event.stream_position, event.event_type, event.stream
        )
    }));
    lines("Event Log", rows)
}

pub fn render_controller_binding_panel(report: &ControllerBindingDebugReport) -> String {
    assert!(report.debug_only);
    let mut rows = vec![format!("binding_count={}", report.bindings.len())];
    rows.extend(report.bindings.iter().cloned());
    lines("Controller Binding", rows)
}

pub fn render_projection_rebuild_panel(report: &ProjectionRebuildDebugReport) -> String {
    assert!(report.debug_only);
    lines(
        "Projection Rebuild",
        vec![
            format!("events_applied={}", report.rebuild.event_count_applied),
            format!(
                "last_event={}",
                report
                    .rebuild
                    .last_event_id
                    .as_ref()
                    .map(|id| id.as_str())
                    .unwrap_or("none")
            ),
            format!("checksum={}", report.rebuild.final_checksum.as_str()),
            format!("diffs={}", report.rebuild.state_diff.len()),
        ],
    )
}

pub fn render_replay_panel(report: &ReplayDebugReport) -> String {
    assert!(report.debug_only);
    lines(
        "Replay",
        vec![
            format!("fixture={}", report.replay.fixture_id.as_str()),
            format!("events={}", report.replay.event_count),
            format!("diagnostic_events={}", report.replay.diagnostic_event_count),
            format!("matches_expected={}", report.replay.matches_expected),
            format!("final_checksum={}", report.replay.final_checksum.as_str()),
        ],
    )
}

fn lines(title: &str, rows: Vec<String>) -> String {
    let mut output = vec![format!("{DEBUG_MARKER}: {title}")];
    output.extend(rows);
    output.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::TuiApp;
    use tracewake_core::actions::ReportStatus;
    use tracewake_core::ids::{ActorId, ItemId, SemanticActionId};

    #[test]
    fn debug_panels_are_marked_non_diegetic_and_do_not_change_embodied_view_or_checksum() {
        let mut app = TuiApp::load_default().unwrap();
        app.bind_actor(ActorId::new("actor_tomas").unwrap())
            .unwrap();
        app.submit_semantic_action(
            &SemanticActionId::new("open.container.strongbox_tomas").unwrap(),
        )
        .unwrap();
        let before_view = app.render_current_view().unwrap();
        let before_checksum = app.physical_checksum();

        let item = app.render_debug_item_location_panel(&ItemId::new("coin_stack_01").unwrap());
        let event_log = app.render_debug_event_log_panel();
        let binding = app.render_debug_controller_binding_panel();
        let rebuild = app.render_debug_projection_rebuild_panel();
        let replay = app.render_debug_replay_panel();

        for panel in [item, event_log, binding, rebuild, replay] {
            assert!(panel.contains(DEBUG_MARKER));
        }
        assert_eq!(app.render_current_view().unwrap(), before_view);
        assert_eq!(app.physical_checksum(), before_checksum);
    }

    #[test]
    fn action_rejection_panel_renders_validation_report() {
        let mut app = TuiApp::from_golden(tracewake_content::fixtures::door_access_001()).unwrap();
        app.bind_actor(ActorId::new("actor_sena").unwrap()).unwrap();
        let result = app
            .submit_semantic_action(&SemanticActionId::new("move.to.back_room").unwrap())
            .unwrap();
        assert_eq!(result.report.status, ReportStatus::Rejected);

        let panel = app.render_debug_action_rejection_panel().unwrap();

        assert!(panel.contains(DEBUG_MARKER));
        assert!(panel.contains("Action Rejection"));
        assert!(panel.contains(result.report.proposal_id.as_str()));
    }
}
