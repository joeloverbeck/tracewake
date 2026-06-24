use tracewake_core::events::EventEnvelope;
use tracewake_core::ids::{ContentManifestId, ControllerId};
use tracewake_core::scheduler::{
    WorldAdvanceOrigin, WorldAdvanceRequest, WorldStepTransactionRequest,
};
use tracewake_core::time::SimTick;

pub fn caller_supplies_raw_world_process_events() -> WorldStepTransactionRequest {
    WorldStepTransactionRequest {
        advance: WorldAdvanceRequest {
            expected_tick: SimTick::new(1),
            origin: WorldAdvanceOrigin::Controller(ControllerId::new("controller.test").unwrap()),
            content_manifest_id: ContentManifestId::new("manifest.test").unwrap(),
            authorized_sleep_interruptions: Vec::new(),
        },
        controlled_proposals: Vec::new(),
        world_process_events: Vec::<EventEnvelope>::new(),
        actor_known_interval_actor_id: None,
    }
}
