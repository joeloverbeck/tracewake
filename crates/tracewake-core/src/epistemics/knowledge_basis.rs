use crate::epistemics::knowledge_context::KnowledgeContext;
use crate::epistemics::projection::EpistemicProjection;
use crate::epistemics::proposition::Proposition;

pub fn actor_has_source_backed_support(
    context: &KnowledgeContext,
    projection: &EpistemicProjection,
    proposition: &Proposition,
) -> bool {
    projection
        .beliefs_for_context(context)
        .into_iter()
        .any(|belief| {
            belief.proposition() == proposition
                && matches!(
                    belief.stance(),
                    crate::epistemics::Stance::BelievesTrue
                        | crate::epistemics::Stance::ExpectsTrue
                        | crate::epistemics::Stance::Plausible
                )
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::epistemics::{Belief, Confidence, HolderKind, SourceRef, Stance};
    use crate::ids::{ActorId, BeliefId, ContentManifestId, EventId, PlaceId};
    use crate::time::SimTick;

    fn actor_id(value: &str) -> ActorId {
        ActorId::new(value).unwrap()
    }

    fn place_id(value: &str) -> PlaceId {
        PlaceId::new(value).unwrap()
    }

    #[test]
    fn support_reads_only_context_visible_holder_beliefs() {
        let proposition = Proposition::ActorWasNearPlace {
            actor_id: actor_id("actor_mara"),
            place_id: place_id("shop_front"),
        };
        let mut projection =
            EpistemicProjection::new(ContentManifestId::new("phase2a_manifest").unwrap());
        projection.insert_belief(Belief::new(
            BeliefId::new("belief_mara_near_shop").unwrap(),
            HolderKind::Actor(actor_id("actor_elena")),
            proposition.clone(),
            Stance::BelievesTrue,
            Confidence::new(900).unwrap(),
            SourceRef::Event(EventId::new("event_elena_observed").unwrap()),
            SimTick::ZERO,
        ));

        let tomas_context = KnowledgeContext::embodied(actor_id("actor_tomas"), SimTick::ZERO);
        let elena_context = KnowledgeContext::embodied(actor_id("actor_elena"), SimTick::ZERO);

        assert!(!actor_has_source_backed_support(
            &tomas_context,
            &projection,
            &proposition
        ));
        assert!(actor_has_source_backed_support(
            &elena_context,
            &projection,
            &proposition
        ));
    }
}
