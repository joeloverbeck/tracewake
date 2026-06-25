use tracewake_core::projections::IntervalStopReason;
use tracewake_core::time::SimTick;
use tracewake_core::view_models::TypedActorKnownIntervalSummary;

pub fn forge_interval_summary() -> TypedActorKnownIntervalSummary {
    TypedActorKnownIntervalSummary {
        start_tick: SimTick::ZERO,
        stop_tick: SimTick::new(99),
        start_frontier: 0,
        stop_frontier: 99,
        stop_reason: IntervalStopReason::ControllerSafetyBound,
        notices: Vec::new(),
        no_new_actor_known_information: true,
    }
}
