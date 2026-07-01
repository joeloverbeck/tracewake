use crate::screen::model::FocusedPane;

const PANE_COUNT: usize = 12;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PresentationState {
    focused_pane: FocusedPane,
    selection_indices: [usize; PANE_COUNT],
    scroll_offsets: [usize; PANE_COUNT],
    help_open: bool,
}

impl PresentationState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn focused_pane(&self) -> FocusedPane {
        self.focused_pane
    }

    pub fn set_focused_pane(&mut self, focused_pane: FocusedPane) {
        self.focused_pane = focused_pane;
    }

    pub fn selection_index(&self, pane: FocusedPane) -> usize {
        self.selection_indices[pane_index(pane)]
    }

    pub fn set_selection_index(&mut self, pane: FocusedPane, selection_index: usize) {
        self.selection_indices[pane_index(pane)] = selection_index;
    }

    pub fn scroll_offset(&self, pane: FocusedPane) -> usize {
        self.scroll_offsets[pane_index(pane)]
    }

    pub fn set_scroll_offset(&mut self, pane: FocusedPane, scroll_offset: usize) {
        self.scroll_offsets[pane_index(pane)] = scroll_offset;
    }

    pub fn help_open(&self) -> bool {
        self.help_open
    }

    pub fn set_help_open(&mut self, help_open: bool) {
        self.help_open = help_open;
    }
}

impl Default for PresentationState {
    fn default() -> Self {
        Self {
            focused_pane: FocusedPane::Place,
            selection_indices: [0; PANE_COUNT],
            scroll_offsets: [0; PANE_COUNT],
            help_open: false,
        }
    }
}

fn pane_index(pane: FocusedPane) -> usize {
    match pane {
        FocusedPane::Place => 0,
        FocusedPane::Exits => 1,
        FocusedPane::Doors => 2,
        FocusedPane::Containers => 3,
        FocusedPane::Items => 4,
        FocusedPane::Inventory => 5,
        FocusedPane::Actors => 6,
        FocusedPane::Actions => 7,
        FocusedPane::Status => 8,
        FocusedPane::WhyNot => 9,
        FocusedPane::Notebook => 10,
        FocusedPane::ActorKnownInterval => 11,
    }
}
