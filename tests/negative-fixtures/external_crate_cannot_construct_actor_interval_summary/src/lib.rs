use tracewake_core::projections::IntervalStopReason;
use tracewake_core::runtime::{RuntimeReceipt, RuntimeReceiptKind};
use tracewake_core::time::SimTick;
use tracewake_core::view_models::{
    EmbodiedViewModel, NotebookView, TypedActorKnownIntervalSummary,
};

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

pub fn assemble_interval_summary(
    delta: tracewake_core::projections::ActorKnownIntervalDelta,
) -> TypedActorKnownIntervalSummary {
    TypedActorKnownIntervalSummary::from_actor_known_delta(delta)
}

pub fn read_exact_interval_fields(summary: &TypedActorKnownIntervalSummary) {
    let _ = summary.start_tick();
    let _ = summary.stop_tick();
    let _ = summary.start_frontier();
    let _ = summary.stop_frontier();
    let _ = summary.stop_reason();
}

pub fn mutate_embodied_view(mut view: EmbodiedViewModel, notebook: NotebookView) {
    view.set_notebook(Some(notebook));
    view.set_debug_available(true);
    view.set_actor_known_interval_summary(None);
}

pub fn extract_raw_continue_result(receipt: &RuntimeReceipt) {
    if let RuntimeReceiptKind::Continued(result) = receipt.kind() {
        let _ = result.actor_known_interval_delta;
    }
}
