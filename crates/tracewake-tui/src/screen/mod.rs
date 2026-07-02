pub mod buffer_render;
pub mod model;
pub mod pane_bindings;
pub mod pane_layout;
pub mod struct_dump;
pub mod text_dump;

pub use buffer_render::{
    buffer_to_plain_text, render_embodied_to_buffer, MINIMUM_PANE_COLUMNS, MINIMUM_PANE_ROWS,
};
pub use model::{
    build_embodied_screen_model, EmbodiedScreenModel, FocusedPane, RenderOptions, ScreenMetadata,
    TerminalSize,
};
pub use pane_bindings::{
    expanded_region_lines, render_pane_region_bindings, visible_region_lines, PaneRegionBinding,
};
pub use pane_layout::{embodied_pane_layout, PaneRegion, PaneRegionLayout, ScreenPaneRef};
pub use struct_dump::{build_embodied_screen_dump, ScreenDump, ScreenPaneDump};
pub use text_dump::render_embodied_screen_dump;
