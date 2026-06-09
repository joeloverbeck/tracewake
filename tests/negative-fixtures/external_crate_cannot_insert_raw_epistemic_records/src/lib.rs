use tracewake_core::epistemics::{
    Belief, Confidence, EpistemicProjection, HolderKind, Proposition, SourceRef, Stance,
};
use tracewake_core::ids::{ActorId, BeliefId, ContentManifestId, EventId, ItemId, PlaceId};
use tracewake_core::time::SimTick;

pub fn insert_record_directly() {
    let mut projection =
        EpistemicProjection::new(ContentManifestId::new("manifest_phase2a").unwrap());
    let belief = Belief::new(
        BeliefId::new("belief_tomas_coin").unwrap(),
        HolderKind::Actor(ActorId::new("actor_tomas").unwrap()),
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
    projection.insert_belief(belief);
}
