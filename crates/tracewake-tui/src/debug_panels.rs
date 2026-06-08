use tracewake_core::debug_reports::{
    ActionRejectionDebugReport, ControllerBindingDebugReport, ItemLocationDebugReport,
    NoHumanDayDebugReport, Phase3ADebugReport, ProjectionRebuildDebugReport, ReplayDebugReport,
};
use tracewake_core::view_models::DebugEventLogView;
use tracewake_core::view_models::{
    DebugBeliefsView, DebugEpistemicsView, DebugObservationsView, DEBUG_EPISTEMICS_MARKER,
};

const DEBUG_MARKER: &str = "DEBUG NON-DIEGETIC";

pub fn render_item_location_panel(report: &ItemLocationDebugReport) -> String {
    assert!(report.debug_only());
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
    assert!(report.debug_only());
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
    assert!(view.debug_only());
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
    assert!(report.debug_only());
    let mut rows = vec![format!("binding_count={}", report.bindings.len())];
    rows.extend(report.bindings.iter().cloned());
    lines("Controller Binding", rows)
}

pub fn render_projection_rebuild_panel(report: &ProjectionRebuildDebugReport) -> String {
    assert!(report.debug_only());
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
    assert!(report.debug_only());
    lines(
        "Replay",
        vec![
            format!("fixture={}", report.replay.fixture_id.as_str()),
            format!("events={}", report.replay.event_count),
            format!("diagnostic_events={}", report.replay.diagnostic_event_count),
            format!("matches_expected={}", report.replay.matches_expected),
            format!(
                "agent_checksum_matches={}",
                report.replay.agent_checksum_matches
            ),
            format!("final_checksum={}", report.replay.final_checksum.as_str()),
            format!(
                "final_agent_checksum={}",
                report.replay.final_agent_checksum.as_str()
            ),
        ],
    )
}

pub fn render_debug_epistemics_panel(view: &DebugEpistemicsView) -> String {
    assert!(view.debug_only());
    assert_eq!(view.non_diegetic_marker(), DEBUG_EPISTEMICS_MARKER);
    let mut rows = vec![
        format!("context_mode={}", view.context_mode),
        format!("projection={}", view.projection_summary),
        format!("observation_count={}", view.observations.len()),
        format!("holder_count={}", view.beliefs_by_holder.len()),
        format!("contradiction_count={}", view.contradictions.len()),
    ];
    rows.extend(view.observations.iter().map(|observation| {
        format!(
            "observation={} actor={} channel={} confidence={} source={}",
            observation.observation_id,
            observation.observer_actor_id.as_str(),
            observation.channel,
            observation.confidence,
            observation.source
        )
    }));
    rows.extend(view.beliefs_by_holder.iter().map(|holder| {
        format!(
            "holder={} belief_count={}",
            holder.holder_actor_id.as_str(),
            holder.beliefs.len()
        )
    }));
    rows.extend(view.contradictions.iter().map(|contradiction| {
        format!(
            "contradiction={} holder={} expectation={} observation={} summary={}",
            contradiction.contradiction_id,
            contradiction.holder_actor_id.as_str(),
            contradiction.expectation_belief_id,
            contradiction.observation_id,
            contradiction.summary
        )
    }));
    lines("Epistemics", rows)
}

pub fn render_debug_beliefs_panel(view: &DebugBeliefsView) -> String {
    assert!(view.debug_only());
    assert_eq!(view.non_diegetic_marker(), DEBUG_EPISTEMICS_MARKER);
    let mut rows = vec![
        format!("actor={}", view.holder_actor_id.as_str()),
        format!("belief_count={}", view.beliefs.len()),
    ];
    rows.extend(view.beliefs.iter().map(|belief| {
        format!(
            "belief={} stance={} confidence={} source={} :: {}",
            belief.belief_id, belief.stance, belief.confidence, belief.source, belief.proposition
        )
    }));
    lines("Beliefs", rows)
}

pub fn render_debug_observations_panel(view: &DebugObservationsView) -> String {
    assert!(view.debug_only());
    assert_eq!(view.non_diegetic_marker(), DEBUG_EPISTEMICS_MARKER);
    let mut rows = vec![
        format!("actor={}", view.observer_actor_id.as_str()),
        format!("observation_count={}", view.observations.len()),
    ];
    rows.extend(view.observations.iter().map(|observation| {
        format!(
            "observation={} channel={} confidence={} source={}",
            observation.observation_id,
            observation.channel,
            observation.confidence,
            observation.source
        )
    }));
    lines("Observations", rows)
}

pub fn render_phase3a_debug_panel(report: &Phase3ADebugReport) -> String {
    assert!(report.debug_only());
    let mut rows = vec![
        format!("typed_decision_traces={}", report.decision_traces.len()),
        format!("typed_stuck_diagnostics={}", report.stuck_diagnostics.len()),
    ];
    rows.extend(report.rows.clone());
    lines(&report.title, rows)
}

pub fn render_no_human_day_panel(report: &NoHumanDayDebugReport) -> String {
    assert!(report.debug_only());
    lines(
        "No Human Day",
        vec![
            format!("projection={}", report.metrics.projection_version),
            format!("events={}", report.metrics.events_per_day),
            format!("routine_events={}", report.metrics.routine_event_count),
            format!("meals_completed={}", report.metrics.meals_completed),
            format!("meals_missed={}", report.metrics.meals_missed),
            format!("sleep_completed={}", report.metrics.sleep_completed),
            format!("sleep_interrupted={}", report.metrics.sleep_interrupted),
            format!("work_completed={}", report.metrics.work_blocks_completed),
            format!("work_failed={}", report.metrics.work_blocks_failed),
            format!("need_crossings={}", report.metrics.need_threshold_crossings),
            format!(
                "routine_interruptions={}",
                report.metrics.routine_interruptions
            ),
            format!("planner_failures={}", report.metrics.planner_failures),
            format!("stuck_actors={}", report.metrics.stuck_actor_count),
            format!("replay_failures={}", report.metrics.replay_failure_count),
            format!("canonical={}", report.canonical_summary),
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
