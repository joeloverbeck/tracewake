use tracewake_content::fixtures;
use tracewake_core::ids::{ActorId, ItemId, SemanticActionId};

use crate::app::{AppError, TuiApp};
use crate::render::render_rejection;

pub fn capture_representative_transcript() -> Result<String, AppError> {
    let mut app = TuiApp::from_golden(fixtures::door_access_001())?;
    app.bind_actor(ActorId::new("actor_sena").unwrap())?;

    let mut sections = Vec::new();
    sections.push(section("view.initial", app.render_current_view()?));

    let rejected =
        app.submit_semantic_action(&SemanticActionId::new("move.to.back_room").unwrap())?;
    sections.push(section(
        "action.move.to.back_room",
        render_rejection(&rejected.report),
    ));
    sections.push(section("view.why_not", app.render_current_view()?));

    app.submit_semantic_action(&SemanticActionId::new("wait.1_tick").unwrap())?;
    sections.push(section("view.after_wait", app.render_current_view()?));

    sections.push(section(
        "debug.event_log",
        app.render_debug_event_log_panel(),
    ));
    sections.push(section(
        "debug.controller_binding",
        app.render_debug_controller_binding_panel(),
    ));
    sections.push(section(
        "debug.item_location",
        app.render_debug_item_location_panel(&ItemId::new("nonexistent_item").unwrap()),
    ));
    sections.push(section(
        "debug.action_rejection",
        app.render_debug_action_rejection_panel()
            .unwrap_or_else(|| "DEBUG NON-DIEGETIC: Action Rejection\nnone".to_string()),
    ));
    sections.push(section(
        "debug.projection_rebuild",
        app.render_debug_projection_rebuild_panel(),
    ));
    sections.push(section("debug.replay", app.render_debug_replay_panel()));

    Ok(sections.join("\n\n"))
}

fn section(name: &str, body: String) -> String {
    format!("== {name} ==\n{body}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn representative_transcript_is_deterministic() {
        let first = capture_representative_transcript().unwrap();
        let second = capture_representative_transcript().unwrap();

        assert_eq!(first, second);
        assert!(first.contains("== view.initial =="));
        assert!(first.contains("== action.move.to.back_room =="));
        assert!(first.contains("DEBUG NON-DIEGETIC"));
    }
}
