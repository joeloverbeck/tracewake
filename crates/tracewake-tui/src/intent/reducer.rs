use tracewake_core::runtime::{ContinuedRuntimeReceipt, RuntimeActionReceipt};

use crate::app::{AppError, TuiApp};
use crate::run;
use crate::screen::model::FocusedPane;

use super::{Direction, PresentationState, UiIntent};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReduceOutcome {
    PresentationChanged,
    ActionSubmitted(Box<RuntimeActionReceipt>),
    Continued(Box<ContinuedRuntimeReceipt>),
    DebugRendered(String),
    Quit,
}

#[derive(Debug)]
pub enum ReduceError {
    App(Box<AppError>),
    WaitActionUnavailable,
    DebugRender(std::io::Error),
    DebugOutputNotUtf8(std::string::FromUtf8Error),
}

impl From<AppError> for ReduceError {
    fn from(value: AppError) -> Self {
        Self::App(Box::new(value))
    }
}

pub fn reduce(
    app: &mut TuiApp,
    state: &mut PresentationState,
    intent: UiIntent,
) -> Result<ReduceOutcome, ReduceError> {
    match intent {
        UiIntent::FocusNext => {
            state.set_focused_pane(next_pane(state.focused_pane()));
            Ok(ReduceOutcome::PresentationChanged)
        }
        UiIntent::FocusPane(focused_pane) => {
            state.set_focused_pane(focused_pane);
            Ok(ReduceOutcome::PresentationChanged)
        }
        UiIntent::MoveSelection(direction) => {
            move_selection(state, direction);
            Ok(ReduceOutcome::PresentationChanged)
        }
        UiIntent::ActivateSelection => activate_selection(app, state),
        UiIntent::SubmitSemanticAction(semantic_action_id) => app
            .submit_semantic_action(&semantic_action_id)
            .map(Box::new)
            .map(ReduceOutcome::ActionSubmitted)
            .map_err(ReduceError::from),
        UiIntent::SubmitDebugCommand(debug_command) => {
            render_debug(app, debug_command).map(ReduceOutcome::DebugRendered)
        }
        UiIntent::WaitOneTick => {
            let view = app.current_view()?;
            let semantic_action_id =
                run::semantic_id_for_wait_alias(&view).ok_or(ReduceError::WaitActionUnavailable)?;
            app.submit_semantic_action(&semantic_action_id)
                .map(Box::new)
                .map(ReduceOutcome::ActionSubmitted)
                .map_err(ReduceError::from)
        }
        UiIntent::ContinueUntil { max_ticks } => app
            .advance_until(max_ticks)
            .map(Box::new)
            .map(ReduceOutcome::Continued)
            .map_err(ReduceError::from),
        UiIntent::ToggleHelp => {
            state.set_help_open(!state.help_open());
            Ok(ReduceOutcome::PresentationChanged)
        }
        UiIntent::FocusNotebook => {
            state.set_focused_pane(FocusedPane::Notebook);
            Ok(ReduceOutcome::PresentationChanged)
        }
        UiIntent::Quit => Ok(ReduceOutcome::Quit),
    }
}

fn render_debug(
    app: &mut TuiApp,
    debug_command: crate::input::DebugCommand,
) -> Result<String, ReduceError> {
    let mut output = Vec::new();
    run::render_debug(app, debug_command, &mut output).map_err(ReduceError::DebugRender)?;
    String::from_utf8(output).map_err(ReduceError::DebugOutputNotUtf8)
}

fn move_selection(state: &mut PresentationState, direction: Direction) {
    let pane = state.focused_pane();
    let current = state.selection_index(pane);
    let next = match direction {
        Direction::Up => current.saturating_sub(1),
        Direction::Down => current.saturating_add(1),
    };
    state.set_selection_index(pane, next);
}

fn activate_selection(
    app: &mut TuiApp,
    state: &PresentationState,
) -> Result<ReduceOutcome, ReduceError> {
    if state.focused_pane() != FocusedPane::Actions {
        return Ok(ReduceOutcome::PresentationChanged);
    }
    let view = app.current_view()?;
    let Some(entry) = view
        .semantic_actions
        .get(state.selection_index(FocusedPane::Actions))
    else {
        return Ok(ReduceOutcome::PresentationChanged);
    };
    app.submit_semantic_action(&entry.semantic_action_id)
        .map(Box::new)
        .map(ReduceOutcome::ActionSubmitted)
        .map_err(ReduceError::from)
}

fn next_pane(current: FocusedPane) -> FocusedPane {
    let current_index = focused_panes()
        .iter()
        .position(|pane| *pane == current)
        .expect("focused pane is listed");
    focused_panes()[(current_index + 1) % focused_panes().len()]
}

fn focused_panes() -> &'static [FocusedPane] {
    &[
        FocusedPane::Place,
        FocusedPane::Exits,
        FocusedPane::Doors,
        FocusedPane::Containers,
        FocusedPane::Items,
        FocusedPane::Inventory,
        FocusedPane::Actors,
        FocusedPane::Actions,
        FocusedPane::Status,
        FocusedPane::WhyNot,
        FocusedPane::Notebook,
        FocusedPane::ActorKnownInterval,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracewake_content::fixtures;
    use tracewake_core::actions::ReportStatus;
    use tracewake_core::ids::{ActorId, SemanticActionId};

    #[test]
    fn presentation_intents_mutate_only_presentation_state() {
        let mut app = bound_app();
        let event_count_before = app.event_count();
        let checksum_before = app.physical_checksum();
        let mut state = PresentationState::default();

        let outcome = reduce(
            &mut app,
            &mut state,
            UiIntent::FocusPane(FocusedPane::Actions),
        )
        .expect("focus intent reduces");
        assert_eq!(outcome, ReduceOutcome::PresentationChanged);
        assert_eq!(state.focused_pane(), FocusedPane::Actions);

        reduce(
            &mut app,
            &mut state,
            UiIntent::MoveSelection(Direction::Down),
        )
        .expect("move selection reduces");
        assert_eq!(state.selection_index(FocusedPane::Actions), 1);

        reduce(&mut app, &mut state, UiIntent::ToggleHelp).expect("help intent reduces");
        assert!(state.help_open());

        assert_eq!(app.event_count(), event_count_before);
        assert_eq!(app.physical_checksum(), checksum_before);
    }

    #[test]
    fn semantic_action_intent_matches_direct_authorized_submission() {
        let semantic_action_id = SemanticActionId::new("open.container.strongbox_tomas").unwrap();
        let mut direct_app = bound_app();
        let direct = direct_app
            .submit_semantic_action(&semantic_action_id)
            .expect("direct submit succeeds");

        let mut reduced_app = bound_app();
        let mut state = PresentationState::default();
        let reduced = reduce(
            &mut reduced_app,
            &mut state,
            UiIntent::SubmitSemanticAction(semantic_action_id),
        )
        .expect("reduced submit succeeds");

        let ReduceOutcome::ActionSubmitted(reduced) = reduced else {
            panic!("semantic action returns action receipt");
        };
        assert_eq!(reduced.report.status, direct.report.status);
        assert_eq!(reduced.report.action_id, direct.report.action_id);
        assert_eq!(
            reduced_app.physical_checksum(),
            direct_app.physical_checksum()
        );
    }

    #[test]
    fn activate_selection_submits_selected_action_through_authorized_path() {
        let mut direct_app = bound_app();
        let view = direct_app.current_view().unwrap();
        let selected_index = view
            .semantic_actions
            .iter()
            .position(|entry| entry.action_id.as_str() == "open")
            .expect("fixture exposes open action");
        let selected_action = view.semantic_actions[selected_index]
            .semantic_action_id
            .clone();
        let direct = direct_app
            .submit_semantic_action(&selected_action)
            .expect("direct submit succeeds");

        let mut reduced_app = bound_app();
        let mut state = PresentationState::default();
        state.set_focused_pane(FocusedPane::Actions);
        state.set_selection_index(FocusedPane::Actions, selected_index);
        let reduced = reduce(&mut reduced_app, &mut state, UiIntent::ActivateSelection)
            .expect("activate selection reduces");

        let ReduceOutcome::ActionSubmitted(reduced) = reduced else {
            panic!("activate selection submits selected action");
        };
        assert_eq!(reduced.report.status, direct.report.status);
        assert_eq!(reduced.report.action_id, direct.report.action_id);
        assert_eq!(
            reduced_app.physical_checksum(),
            direct_app.physical_checksum()
        );
    }

    #[test]
    fn time_control_intent_uses_authorized_runtime_path() {
        let mut app = bound_app();
        let mut state = PresentationState::default();

        let outcome = reduce(
            &mut app,
            &mut state,
            UiIntent::ContinueUntil { max_ticks: 1 },
        )
        .expect("continue intent reduces");

        let ReduceOutcome::Continued(_receipt) = outcome else {
            panic!("continue returns continued receipt");
        };
        assert!(app.event_count() > 0);
    }

    #[test]
    fn debug_intent_routes_through_debug_gate_without_embodied_output() {
        let mut app = TuiApp::from_golden_operator_debug(fixtures::strongbox_001()).unwrap();
        app.bind_debug_actor(ActorId::new("actor_tomas").unwrap())
            .unwrap();
        let mut state = PresentationState::default();

        let outcome = reduce(
            &mut app,
            &mut state,
            UiIntent::SubmitDebugCommand(crate::input::DebugCommand::Overlay),
        )
        .expect("debug overlay reduces");

        let ReduceOutcome::DebugRendered(rendered) = outcome else {
            panic!("debug intent returns rendered debug text");
        };
        assert!(rendered.contains("DEBUG NON-DIEGETIC: Embodied Overlay"));

        let embodied = app.render_current_view().unwrap();
        assert!(!embodied.contains("DEBUG NON-DIEGETIC"));
    }

    #[test]
    fn debug_intent_refuses_without_debug_authority() {
        let mut app = bound_app();
        let mut state = PresentationState::default();

        let outcome = reduce(
            &mut app,
            &mut state,
            UiIntent::SubmitDebugCommand(crate::input::DebugCommand::Overlay),
        )
        .expect("debug refusal is rendered by existing path");

        let ReduceOutcome::DebugRendered(rendered) = outcome else {
            panic!("debug intent returns rendered debug text");
        };
        assert_eq!(rendered.trim(), "Error: debug unavailable");
    }

    #[test]
    fn wait_one_tick_routes_to_current_wait_action() {
        let mut app = bound_app();
        let mut state = PresentationState::default();

        let outcome =
            reduce(&mut app, &mut state, UiIntent::WaitOneTick).expect("wait intent reduces");

        let ReduceOutcome::ActionSubmitted(receipt) = outcome else {
            panic!("wait returns action receipt");
        };
        assert_eq!(receipt.report.status, ReportStatus::Accepted);
        assert_eq!(receipt.report.action_id.as_str(), "wait");
    }

    fn bound_app() -> TuiApp {
        let mut app = TuiApp::load_default().unwrap();
        app.bind_actor(ActorId::new("actor_tomas").unwrap())
            .unwrap();
        app
    }
}
