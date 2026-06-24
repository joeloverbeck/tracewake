use tracewake_core::view_models::EmbodiedViewModel;

pub fn mutate_embodied_temporal_fields(mut view: EmbodiedViewModel) -> EmbodiedViewModel {
    view.holder_known_context_frontier = 99;
    view.debug_available = true;
    view
}
