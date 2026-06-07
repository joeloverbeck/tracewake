//! Terminal user interface boundary for Tracewake.

pub mod app;
pub mod debug_panels;
pub mod input;
pub mod launch;
pub mod render;
pub mod run;
pub mod transcript;

pub fn startup_message() -> &'static str {
    "tracewake-tui ready"
}
