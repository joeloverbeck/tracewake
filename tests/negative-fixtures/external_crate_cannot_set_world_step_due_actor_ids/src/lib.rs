use tracewake_core::ids::{ActorId, ContentManifestId, ControllerId};
use tracewake_core::scheduler::{
    WorldAdvanceOrigin, WorldAdvanceRequest, WorldStepTransactionRequest,
};
use tracewake_core::time::SimTick;

pub fn caller_supplies_due_actor_ids() -> WorldStepTransactionRequest {
    WorldStepTransactionRequest {
        advance: WorldAdvanceRequest {
            expected_tick: SimTick::new(1),
            origin: WorldAdvanceOrigin::Controller(ControllerId::new("controller.test").unwrap()),
            content_manifest_id: ContentManifestId::new("manifest.test").unwrap(),
            authorized_sleep_interruptions: Vec::new(),
        },
        controlled_proposals: Vec::new(),
        due_actor_ids: vec![ActorId::new("actor.test").unwrap()],
        actor_known_interval_actor_id: None,
    }
}
