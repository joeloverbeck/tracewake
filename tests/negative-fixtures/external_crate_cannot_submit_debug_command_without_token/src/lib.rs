use tracewake_core::debug_capability::DebugSessionAuthority;
use tracewake_core::runtime::RuntimeCommand;
use tracewake_core::view_models::{DebugEventLogView, DebugProjectionRebuildView};

pub fn submit_debug_without_runtime_authority() {
    let _command = RuntimeCommand::run_no_human_day();
}

pub fn forge_debug_authority() {
    let _authority = DebugSessionAuthority::mint();
}

pub fn construct_debug_views_without_authority() {
    let _event_log = DebugEventLogView::new(Vec::new());
    let _rebuild = DebugProjectionRebuildView::new("rebuilt");
}
