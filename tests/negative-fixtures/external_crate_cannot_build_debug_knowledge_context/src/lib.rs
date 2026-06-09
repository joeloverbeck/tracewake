use tracewake_core::epistemics::KnowledgeContext;
use tracewake_core::ids::ActorId;
use tracewake_core::time::SimTick;

pub fn build_debug_context_without_capability() -> KnowledgeContext {
    KnowledgeContext::debug(ActorId::new("actor_tomas").unwrap(), SimTick::ZERO)
}
