pub mod model;
pub mod text_dump;

pub use model::{
    build_embodied_screen_model, EmbodiedScreenModel, FocusedPane, RenderOptions, ScreenMetadata,
    TerminalSize,
};
pub use text_dump::render_embodied_screen_dump;
