use tracewake_core::state::PhysicalState;

pub fn mutate_loaded_state(state: &mut PhysicalState) {
    let _items = state.seed_items_mut();
}
