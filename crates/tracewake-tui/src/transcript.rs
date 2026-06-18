use tracewake_content::fixtures;
use tracewake_core::ids::{ActorId, ItemId, SemanticActionId};

use crate::app::{AppError, TuiApp};
use crate::render::render_rejection;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TranscriptSection {
    pub name: String,
    pub body: String,
}

pub fn capture_representative_transcript() -> Result<String, AppError> {
    Ok(render_transcript_sections(
        &capture_representative_transcript_sections()?,
    ))
}

pub fn capture_representative_transcript_sections() -> Result<Vec<TranscriptSection>, AppError> {
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
    sections.push(section(
        "notebook.actor_sena",
        crate::render::render_notebook(&app.notebook_view()?),
    ));

    let wait_action_id = app
        .current_view()?
        .semantic_actions
        .iter()
        .find(|action| action.action_id.as_str() == "wait")
        .map(|action| action.semantic_action_id.clone())
        .ok_or_else(|| AppError::SemanticActionNotFound("wait".to_string()))?;
    app.submit_semantic_action(&wait_action_id)?;
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
    sections.push(section(
        "debug.epistemics",
        crate::debug_panels::render_debug_epistemics_panel(&app.debug_epistemics_view()),
    ));
    sections.push(section(
        "debug.beliefs.actor_sena",
        crate::debug_panels::render_debug_beliefs_panel(
            &app.debug_beliefs_view(&ActorId::new("actor_sena").unwrap())?,
        ),
    ));
    sections.push(section(
        "debug.observations.actor_sena",
        crate::debug_panels::render_debug_observations_panel(
            &app.debug_observations_view(&ActorId::new("actor_sena").unwrap())?,
        ),
    ));

    Ok(sections)
}

pub fn render_transcript_sections(sections: &[TranscriptSection]) -> String {
    sections
        .iter()
        .map(|section| format!("== {} ==\n{}", section.name, section.body))
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn section(name: &str, body: String) -> TranscriptSection {
    TranscriptSection {
        name: name.to_string(),
        body,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn representative_transcript_is_deterministic() {
        let first_sections = capture_representative_transcript_sections().unwrap();
        let second_sections = capture_representative_transcript_sections().unwrap();
        let first = render_transcript_sections(&first_sections);
        let second = render_transcript_sections(&second_sections);

        assert_eq!(first_sections, second_sections);
        assert_eq!(first, second);
        assert_eq!(
            first_sections
                .iter()
                .map(|section| section.name.as_str())
                .collect::<Vec<_>>(),
            vec![
                "view.initial",
                "action.move.to.back_room",
                "view.why_not",
                "notebook.actor_sena",
                "view.after_wait",
                "debug.event_log",
                "debug.controller_binding",
                "debug.item_location",
                "debug.action_rejection",
                "debug.projection_rebuild",
                "debug.replay",
                "debug.epistemics",
                "debug.beliefs.actor_sena",
                "debug.observations.actor_sena",
            ]
        );
        let after_wait = first_sections
            .iter()
            .find(|section| section.name == "view.after_wait")
            .expect("representative transcript includes post-wait view");
        assert!(after_wait.body.contains("Tick: 1"));
        assert!(first.contains("== view.initial =="));
        assert!(first.contains("== action.move.to.back_room =="));
        assert!(first.contains("DEBUG NON-DIEGETIC"));
    }
}
