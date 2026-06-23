use tracewake_core::events::log::EventLog;
use tracewake_core::projections::build_debug_event_log_view;
use tracewake_core::view_models::TypedActorKnownIntervalSummary;

pub fn convert_debug_view() -> TypedActorKnownIntervalSummary {
    let debug_view = build_debug_event_log_view(&EventLog::new());
    TypedActorKnownIntervalSummary::from(debug_view)
}
