use tracewake_core::epistemics::KnowledgeContext;
use tracewake_core::ids::ActorId;
use tracewake_core::time::SimTick;

pub fn mutate_context_viewer() {
    let mut context =
        KnowledgeContext::embodied(ActorId::new("actor_tomas").unwrap(), SimTick::ZERO);
    context.viewer_actor_id = ActorId::new("actor_mara").unwrap();
}
