use tracewake_core::debug_capability::DebugCapability;
use tracewake_core::debug_reports::ControllerBindingDebugReport;
use tracewake_core::ids::DebugReportId;

pub fn forge_debug_report() -> ControllerBindingDebugReport {
    ControllerBindingDebugReport {
        report_id: DebugReportId::new("debug.forged").unwrap(),
        bindings: Vec::new(),
        debug_capability: DebugCapability::mint(),
    }
}
