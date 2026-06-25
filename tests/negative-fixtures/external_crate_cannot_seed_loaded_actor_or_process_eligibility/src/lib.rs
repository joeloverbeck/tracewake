use tracewake_core::ids::{ActorId, ContentManifestId, ProcessId};
use tracewake_core::scheduler::DeterministicScheduler;
use tracewake_core::time::SimTick;

pub fn seed_scheduler(mut scheduler: DeterministicScheduler) {
    scheduler.schedule_loaded_actor_decision(
        ActorId::new("actor_mara").unwrap(),
        SimTick::new(1),
    );
    scheduler.register_cadenced_world_process(
        ProcessId::new("process_probe").unwrap(),
        SimTick::new(1),
        1,
        Vec::new(),
        ContentManifestId::new("manifest_probe").unwrap(),
        None,
    );
}
