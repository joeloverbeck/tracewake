use tracewake_core::ids::{ActorId, ItemId, SemanticActionId};
use tracewake_core::view_models::EmbodiedViewModel;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UiCommand {
    Help,
    View,
    Notebook,
    BindActor(ActorId),
    SelectSemanticAction(SemanticActionId),
    SelectByMenuIndex(usize),
    WaitOneTick,
    OperatorProof(OperatorProofCommand),
    Debug(DebugCommand),
    Quit,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OperatorProofCommand {
    RunNoHumanDay,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DebugCommand {
    Overlay,
    EventLog,
    ControllerBindings,
    ItemLocation(ItemId),
    Rejection,
    ProjectionRebuild,
    Replay,
    Epistemics,
    Beliefs(ActorId),
    Observations(ActorId),
    Needs,
    Routines,
    Planner(ActorId),
    Stuck,
    NoHumanDay,
    Actor(ActorId),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InputError {
    Empty,
    UnknownCommand(String),
    BadActorId(String),
    BadSemanticActionId(String),
    BadMenuIndex(String),
    BadDebugCommand(String),
    BadItemId(String),
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
    if trimmed == "help" {
        return Ok(UiCommand::Help);
    }
    if trimmed == "view" {
        return Ok(UiCommand::View);
    }
    if trimmed == "notebook" {
        return Ok(UiCommand::Notebook);
    }
    if trimmed == "wait" || trimmed == "w" {
        return Ok(UiCommand::WaitOneTick);
    }
    if trimmed == "run no-human-day" {
        return Ok(UiCommand::OperatorProof(
            OperatorProofCommand::RunNoHumanDay,
        ));
    }
    if trimmed.chars().all(|ch| ch.is_ascii_digit()) {
        let one_based_selection = trimmed
            .parse::<usize>()
            .map_err(|_| InputError::BadMenuIndex(trimmed.to_string()))?;
        if one_based_selection == 0 {
            return Err(InputError::SelectionOutOfRange(one_based_selection));
        }
        return Ok(UiCommand::SelectByMenuIndex(one_based_selection));
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
    if let Some(debug_command) = trimmed.strip_prefix("debug ") {
        return parse_debug_command(debug_command).map(UiCommand::Debug);
    }
    Err(InputError::UnknownCommand(trimmed.to_string()))
}

fn parse_debug_command(input: &str) -> Result<DebugCommand, InputError> {
    let trimmed = input.trim();
    match trimmed {
        "overlay" => Ok(DebugCommand::Overlay),
        "log" => Ok(DebugCommand::EventLog),
        "bindings" => Ok(DebugCommand::ControllerBindings),
        "rejection" => Ok(DebugCommand::Rejection),
        "projection" => Ok(DebugCommand::ProjectionRebuild),
        "replay" => Ok(DebugCommand::Replay),
        "epistemics" => Ok(DebugCommand::Epistemics),
        "needs" => Ok(DebugCommand::Needs),
        "routines" => Ok(DebugCommand::Routines),
        "stuck" => Ok(DebugCommand::Stuck),
        "no-human-day" => Ok(DebugCommand::NoHumanDay),
        _ => {
            if let Some(item_id) = trimmed.strip_prefix("item ") {
                return ItemId::new(item_id.to_string())
                    .map(DebugCommand::ItemLocation)
                    .map_err(|_| InputError::BadItemId(item_id.to_string()));
            }
            if let Some(actor_id) = trimmed.strip_prefix("beliefs ") {
                return ActorId::new(actor_id.to_string())
                    .map(DebugCommand::Beliefs)
                    .map_err(|_| InputError::BadActorId(actor_id.to_string()));
            }
            if let Some(actor_id) = trimmed.strip_prefix("observations ") {
                return ActorId::new(actor_id.to_string())
                    .map(DebugCommand::Observations)
                    .map_err(|_| InputError::BadActorId(actor_id.to_string()));
            }
            if let Some(actor_id) = trimmed.strip_prefix("planner ") {
                return ActorId::new(actor_id.to_string())
                    .map(DebugCommand::Planner)
                    .map_err(|_| InputError::BadActorId(actor_id.to_string()));
            }
            if let Some(actor_id) = trimmed.strip_prefix("actor ") {
                return ActorId::new(actor_id.to_string())
                    .map(DebugCommand::Actor)
                    .map_err(|_| InputError::BadActorId(actor_id.to_string()));
            }
            Err(InputError::BadDebugCommand(trimmed.to_string()))
        }
    }
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
    use tracewake_core::epistemics::KnowledgeContext;
    use tracewake_core::ids::{ActionId, SemanticActionId, ViewModelId};
    use tracewake_core::time::SimTick;
    use tracewake_core::view_models::{SemanticActionEntry, ViewMode};

    fn view() -> EmbodiedViewModel {
        let context =
            KnowledgeContext::embodied(ActorId::new("actor_tomas").unwrap(), SimTick::ZERO);
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
            carried_items: Vec::new(),
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
                    SemanticActionId::new(format!("wait.{}_tick", 1)).unwrap(),
                    ActionId::new("wait").unwrap(),
                    vec!["1_tick".to_string()],
                    "Wait",
                    true,
                    None,
                ),
            ],
            phase3a_status: None,
            last_rejection_summary: None,
            last_rejection_why_not: None,
            holder_known_context_id: context.holder_known_context_id().clone(),
            holder_known_context_hash: context.holder_known_context_hash().clone(),
            holder_known_context_frontier: context.event_frontier(),
            holder_known_context_source_summary: "allowed=5 provenance=5".to_string(),
            notebook: None,
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
        let wait_action_id = format!("wait.{}_tick", 1);

        assert_eq!(
            parse_command(&format!("do {wait_action_id}")).unwrap(),
            UiCommand::SelectSemanticAction(SemanticActionId::new(wait_action_id).unwrap())
        );
    }

    #[test]
    fn parser_recognizes_help_view_and_wait_commands() {
        assert_eq!(parse_command("help").unwrap(), UiCommand::Help);
        assert_eq!(parse_command("view").unwrap(), UiCommand::View);
        assert_eq!(parse_command("notebook").unwrap(), UiCommand::Notebook);
        assert_eq!(parse_command("wait").unwrap(), UiCommand::WaitOneTick);
        assert_eq!(parse_command("w").unwrap(), UiCommand::WaitOneTick);
        assert_eq!(
            parse_command("run no-human-day").unwrap(),
            UiCommand::OperatorProof(OperatorProofCommand::RunNoHumanDay)
        );
    }

    #[test]
    fn parser_classifies_numeric_selection_as_menu_index() {
        assert_eq!(parse_command("1").unwrap(), UiCommand::SelectByMenuIndex(1));
        assert_ne!(
            parse_command("1").unwrap(),
            UiCommand::SelectSemanticAction(SemanticActionId::new("1").unwrap())
        );
        assert_eq!(parse_command("0"), Err(InputError::SelectionOutOfRange(0)));
    }

    #[test]
    fn parser_recognizes_debug_commands() {
        assert_eq!(
            parse_command("debug overlay").unwrap(),
            UiCommand::Debug(DebugCommand::Overlay)
        );
        assert_eq!(
            parse_command("debug log").unwrap(),
            UiCommand::Debug(DebugCommand::EventLog)
        );
        assert_eq!(
            parse_command("debug bindings").unwrap(),
            UiCommand::Debug(DebugCommand::ControllerBindings)
        );
        assert_eq!(
            parse_command("debug item coin_stack_01").unwrap(),
            UiCommand::Debug(DebugCommand::ItemLocation(
                ItemId::new("coin_stack_01").unwrap()
            ))
        );
        assert_eq!(
            parse_command("debug rejection").unwrap(),
            UiCommand::Debug(DebugCommand::Rejection)
        );
        assert_eq!(
            parse_command("debug projection").unwrap(),
            UiCommand::Debug(DebugCommand::ProjectionRebuild)
        );
        assert_eq!(
            parse_command("debug replay").unwrap(),
            UiCommand::Debug(DebugCommand::Replay)
        );
        assert_eq!(
            parse_command("debug epistemics").unwrap(),
            UiCommand::Debug(DebugCommand::Epistemics)
        );
        assert_eq!(
            parse_command("debug beliefs actor_tomas").unwrap(),
            UiCommand::Debug(DebugCommand::Beliefs(ActorId::new("actor_tomas").unwrap()))
        );
        assert_eq!(
            parse_command("debug observations actor_tomas").unwrap(),
            UiCommand::Debug(DebugCommand::Observations(
                ActorId::new("actor_tomas").unwrap()
            ))
        );
        assert_eq!(
            parse_command("debug needs").unwrap(),
            UiCommand::Debug(DebugCommand::Needs)
        );
        assert_eq!(
            parse_command("debug routines").unwrap(),
            UiCommand::Debug(DebugCommand::Routines)
        );
        assert_eq!(
            parse_command("debug planner actor_mara").unwrap(),
            UiCommand::Debug(DebugCommand::Planner(ActorId::new("actor_mara").unwrap()))
        );
        assert_eq!(
            parse_command("debug stuck").unwrap(),
            UiCommand::Debug(DebugCommand::Stuck)
        );
        assert_eq!(
            parse_command("debug no-human-day").unwrap(),
            UiCommand::Debug(DebugCommand::NoHumanDay)
        );
        assert_eq!(
            parse_command("debug actor actor_tomas").unwrap(),
            UiCommand::Debug(DebugCommand::Actor(ActorId::new("actor_tomas").unwrap()))
        );
    }

    #[test]
    fn parser_rejects_bad_debug_command() {
        assert_eq!(
            parse_command("debug bogus"),
            Err(InputError::BadDebugCommand("bogus".to_string()))
        );
        assert_eq!(
            parse_command("debug item BAD"),
            Err(InputError::BadItemId("BAD".to_string()))
        );
        assert_eq!(
            parse_command("debug beliefs BAD"),
            Err(InputError::BadActorId("BAD".to_string()))
        );
    }
}
