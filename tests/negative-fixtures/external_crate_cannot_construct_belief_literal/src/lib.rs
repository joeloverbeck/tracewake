use std::collections::BTreeSet;

use tracewake_core::epistemics::{
    Belief, Confidence, HolderKind, PrivacyScope, Proposition, SourceRef, Stance,
};
use tracewake_core::epistemics::observation::EPISTEMIC_RECORD_SCHEMA_V1;
use tracewake_core::ids::{ActorId, BeliefId, EventId, ItemId, PlaceId, SchemaVersion};
use tracewake_core::time::SimTick;

pub fn construct_belief_literal() -> Belief {
    let actor_id = ActorId::new("actor_tomas").unwrap();
    Belief {
        belief_id: BeliefId::new("belief_tomas_coin").unwrap(),
        holder: HolderKind::Actor(actor_id.clone()),
        proposition: Proposition::ItemMissingFromExpectedLocation {
            item_id: ItemId::new("coin_stack_01").unwrap(),
            expected_location: tracewake_core::location::Location::AtPlace(
                PlaceId::new("tomas_room").unwrap(),
            ),
        },
        stance: Stance::BelievesTrue,
        confidence: Confidence::new(900).unwrap(),
        source: SourceRef::Event(EventId::new("event_seed").unwrap()),
        channel: None,
        acquired_tick: SimTick::ZERO,
        last_verified_tick: None,
        stale_after_tick: None,
        observation_ids: BTreeSet::new(),
        contradiction_ids: BTreeSet::new(),
        schema_version: SchemaVersion::new(EPISTEMIC_RECORD_SCHEMA_V1).unwrap(),
        privacy_scope: PrivacyScope::ActorPrivate(actor_id),
    }
}
