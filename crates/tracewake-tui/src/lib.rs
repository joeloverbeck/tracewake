//! Terminal user interface boundary for Tracewake.

pub mod app;
pub mod input;
pub mod render;

pub fn startup_message() -> &'static str {
    "tracewake-tui ready"
}
