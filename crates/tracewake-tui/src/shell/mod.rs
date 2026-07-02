//! Fullscreen terminal shell adapter.

pub mod event_loop;
pub mod key_map;
pub mod terminal;

pub use terminal::{CrosstermTerminal, TerminalGuard};
