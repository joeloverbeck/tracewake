use tracewake_core::ids::{ActorId, SemanticActionId};
use tracewake_core::view_models::EmbodiedViewModel;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UiCommand {
    BindActor(ActorId),
    SelectSemanticAction(SemanticActionId),
    WaitOneTick,
    Quit,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InputError {
    Empty,
    UnknownCommand(String),
    BadActorId(String),
    BadSemanticActionId(String),
    SelectionOutOfRange(usize),
}

pub fn parse_command(input: &str) -> Result<UiCommand, InputError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(InputError::Empty);
    }
    if trimmed == "quit" || trimmed == "q" {
        return Ok(UiCommand::Quit);
    }
    if trimmed == "wait" || trimmed == "w" {
        return Ok(UiCommand::WaitOneTick);
    }
    if let Some(actor_id) = trimmed.strip_prefix("bind ") {
        return ActorId::new(actor_id.to_string())
            .map(UiCommand::BindActor)
            .map_err(|_| InputError::BadActorId(actor_id.to_string()));
    }
    if let Some(semantic_action_id) = trimmed.strip_prefix("do ") {
        return SemanticActionId::new(semantic_action_id.to_string())
            .map(UiCommand::SelectSemanticAction)
            .map_err(|_| InputError::BadSemanticActionId(semantic_action_id.to_string()));
    }
    Err(InputError::UnknownCommand(trimmed.to_string()))
}

pub fn semantic_id_for_selection(
    view: &EmbodiedViewModel,
    one_based_selection: usize,
) -> Result<SemanticActionId, InputError> {
    if one_based_selection == 0 {
        return Err(InputError::SelectionOutOfRange(one_based_selection));
    }
    view.semantic_actions
        .get(one_based_selection - 1)
        .map(|entry| entry.semantic_action_id.clone())
        .ok_or(InputError::SelectionOutOfRange(one_based_selection))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracewake_core::ids::{ActionId, SemanticActionId, ViewModelId};
    use tracewake_core::time::SimTick;
    use tracewake_core::view_models::{SemanticActionEntry, ViewMode};

    fn view() -> EmbodiedViewModel {
        EmbodiedViewModel {
            view_model_id: ViewModelId::new("view.actor_tomas.0").unwrap(),
            mode: ViewMode::Embodied,
            viewer_actor_id: ActorId::new("actor_tomas").unwrap(),
            sim_tick: SimTick::ZERO,
            place_id: "house_tomas".parse().unwrap(),
            place_label: "House".to_string(),
            visible_exits: Vec::new(),
            visible_doors: Vec::new(),
            visible_containers: Vec::new(),
            visible_items: Vec::new(),
            local_actors: Vec::new(),
            semantic_actions: vec![
                SemanticActionEntry::new(
                    SemanticActionId::new("open.container.strongbox_tomas").unwrap(),
                    ActionId::new("open").unwrap(),
                    vec!["strongbox_tomas".to_string()],
                    "Open strongbox",
                    true,
                    None,
                ),
                SemanticActionEntry::new(
                    SemanticActionId::new("wait.1_tick").unwrap(),
                    ActionId::new("wait").unwrap(),
                    vec!["1_tick".to_string()],
                    "Wait",
                    true,
                    None,
                ),
            ],
            last_rejection_summary: None,
            debug_available: true,
        }
    }

    #[test]
    fn selection_returns_semantic_action_id_not_menu_index() {
        let selected = semantic_id_for_selection(&view(), 1).unwrap();

        assert_eq!(selected.as_str(), "open.container.strongbox_tomas");
        assert_ne!(selected.as_str(), "1");
    }

    #[test]
    fn typed_command_selects_stable_semantic_id() {
        assert_eq!(
            parse_command("do wait.1_tick").unwrap(),
            UiCommand::SelectSemanticAction(SemanticActionId::new("wait.1_tick").unwrap())
        );
    }
}
