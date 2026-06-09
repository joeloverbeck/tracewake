use tracewake_core::epistemics::{KnowledgeContext, ViewMode};
use tracewake_core::ids::ActorId;
use tracewake_core::time::SimTick;

pub fn mutate_context_mode() {
    let mut context =
        KnowledgeContext::embodied(ActorId::new("actor_tomas").unwrap(), SimTick::ZERO);
    context.mode = ViewMode::Debug;
}
