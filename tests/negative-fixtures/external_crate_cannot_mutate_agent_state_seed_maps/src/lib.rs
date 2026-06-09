use tracewake_core::state::AgentState;

pub fn mutate_loaded_agent_state(state: &mut AgentState) {
    let _intentions = state.seed_intentions_mut();
}
