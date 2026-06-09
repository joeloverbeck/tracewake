use tracewake_core::epistemics::{
    Belief, Confidence, HolderKind, PrivacyScope, Proposition, SourceRef, Stance,
};
use tracewake_core::ids::{ActorId, BeliefId, EventId, ItemId, PlaceId};
use tracewake_core::time::SimTick;

pub fn mutate_belief_provenance() {
    let actor_id = ActorId::new("actor_tomas").unwrap();
    let mut belief = Belief::new(
        BeliefId::new("belief_tomas_coin").unwrap(),
        HolderKind::Actor(actor_id.clone()),
        Proposition::ItemMissingFromExpectedLocation {
            item_id: ItemId::new("coin_stack_01").unwrap(),
            expected_location: tracewake_core::location::Location::AtPlace(
                PlaceId::new("tomas_room").unwrap(),
            ),
        },
        Stance::BelievesTrue,
        Confidence::new(900).unwrap(),
        SourceRef::Event(EventId::new("event_seed").unwrap()),
        SimTick::ZERO,
    );
    belief.source = SourceRef::Event(EventId::new("event_forged").unwrap());
    belief.privacy_scope = PrivacyScope::ActorPrivate(actor_id);
}
