pub mod key_script;
pub mod reducer;
pub mod state;

pub use state::PresentationState;

use crate::input::DebugCommand;
use crate::screen::model::FocusedPane;
use tracewake_core::ids::SemanticActionId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UiIntent {
    FocusNext,
    FocusPane(FocusedPane),
    MoveSelection(Direction),
    ActivateSelection,
    SubmitSemanticAction(SemanticActionId),
    SubmitDebugCommand(DebugCommand),
    WaitOneTick,
    ContinueUntil { max_ticks: u64 },
    ToggleHelp,
    FocusNotebook,
    Quit,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intent_types_are_constructible_for_dependents() {
        let semantic_action_id = SemanticActionId::new("wait.1_tick").unwrap();
        let intents = vec![
            UiIntent::FocusNext,
            UiIntent::FocusPane(FocusedPane::Actions),
            UiIntent::MoveSelection(Direction::Up),
            UiIntent::MoveSelection(Direction::Down),
            UiIntent::ActivateSelection,
            UiIntent::SubmitSemanticAction(semantic_action_id),
            UiIntent::SubmitDebugCommand(DebugCommand::Overlay),
            UiIntent::WaitOneTick,
            UiIntent::ContinueUntil { max_ticks: 64 },
            UiIntent::ToggleHelp,
            UiIntent::FocusNotebook,
            UiIntent::Quit,
        ];

        assert_eq!(intents.len(), 12);

        let state = PresentationState::default();
        assert_eq!(state.focused_pane(), FocusedPane::Place);
        assert_eq!(state.selection_index(FocusedPane::Actions), 0);
        assert_eq!(state.scroll_offset(FocusedPane::Actions), 0);
        assert!(!state.help_open());
    }
}
