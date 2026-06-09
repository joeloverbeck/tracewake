use std::collections::BTreeSet;

use tracewake_core::epistemics::{
    Channel, Confidence, Observation, ObservationSubject, ObservationTarget, PrivacyScope,
    TickWindow,
};
use tracewake_core::epistemics::observation::EPISTEMIC_RECORD_SCHEMA_V1;
use tracewake_core::ids::{ActorId, ObservationId, PlaceId, SchemaVersion};
use tracewake_core::time::SimTick;

pub fn construct_observation_without_source() -> Observation {
    let actor_id = ActorId::new("actor_tomas").unwrap();
    Observation {
        observation_id: ObservationId::new("obs_tomas_room").unwrap(),
        observer_actor_id: actor_id.clone(),
        channel: Channel::DirectSight,
        observed_tick: SimTick::ZERO,
        tick_window: TickWindow::at(SimTick::ZERO),
        observer_place_id: PlaceId::new("tomas_room").unwrap(),
        subject: ObservationSubject::Place(PlaceId::new("tomas_room").unwrap()),
        target: ObservationTarget::Place(PlaceId::new("tomas_room").unwrap()),
        raw_payload: Vec::new(),
        confidence: Confidence::new(900).unwrap(),
        alternatives: BTreeSet::new(),
        schema_version: SchemaVersion::new(EPISTEMIC_RECORD_SCHEMA_V1).unwrap(),
        privacy_scope: PrivacyScope::ActorPrivate(actor_id),
    }
}
