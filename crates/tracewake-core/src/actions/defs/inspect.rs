use crate::ids::{ActorId, ContainerId, ItemId};
use crate::location::visible_locality;
use crate::state::PhysicalState;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InspectResponse {
    pub visible_containers: Vec<ContainerId>,
    pub visible_items: Vec<ItemId>,
}

pub fn inspect_visible(state: &PhysicalState, actor_id: &ActorId) -> Option<InspectResponse> {
    let actor = state.actors.get(actor_id)?;
    let visible = visible_locality(
        actor,
        &state.actors,
        &state.doors,
        &state.containers,
        &state.items,
    );
    Some(InspectResponse {
        visible_containers: visible.visible_containers.into_iter().collect(),
        visible_items: visible.visible_items.into_iter().collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actions::pipeline::{run_pipeline, PipelineContext};
    use crate::actions::proposal::{Proposal, ProposalOrigin};
    use crate::actions::registry::ActionRegistry;
    use crate::events::log::EventLog;
    use crate::ids::{ActionId, ContentManifestId, ProposalId};
    use crate::location::Location;
    use crate::scheduler::{OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId};
    use crate::state::PlaceState;
    use crate::state::{ActorBody, ContainerState, ItemState};
    use crate::time::SimTick;

    fn actor_id() -> ActorId {
        ActorId::new("actor_tomas").unwrap()
    }

    fn place_id() -> crate::ids::PlaceId {
        crate::ids::PlaceId::new("shop_front").unwrap()
    }

    fn container_id() -> ContainerId {
        ContainerId::new("strongbox_tomas").unwrap()
    }

    fn item_id() -> ItemId {
        ItemId::new("coin_stack_01").unwrap()
    }

    fn state() -> PhysicalState {
        let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
        state
            .places
            .insert(place_id(), PlaceState::new(place_id(), "Shop front"));
        state
            .actors
            .insert(actor_id(), ActorBody::new(actor_id(), place_id()));
        let mut container = ContainerState::fixed_at_place(container_id(), place_id());
        container.is_open = false;
        container.contents_visible_when_closed = false;
        container.contents.insert(item_id());
        state.containers.insert(container_id(), container);
        state.items.insert(
            item_id(),
            ItemState::new(item_id(), Location::InContainer(container_id())),
        );
        state
    }

    fn ordering_key() -> OrderingKey {
        OrderingKey::new(
            SimTick::ZERO,
            SchedulePhase::HumanCommand,
            SchedulerSourceId::Actor(actor_id()),
            ProposalSequence::new(0),
            ActionId::new("inspect_entity").unwrap(),
            vec!["strongbox_tomas".to_string()],
            "tie",
        )
    }

    #[test]
    fn inspect_closed_opaque_container_excludes_hidden_contents() {
        let response = inspect_visible(&state(), &actor_id()).unwrap();

        assert!(response.visible_containers.contains(&container_id()));
        assert!(!response.visible_items.contains(&item_id()));
    }

    #[test]
    fn accepted_inspect_appends_no_world_event() {
        let mut registry = ActionRegistry::new();
        registry.register_phase1_inspect_wait();
        let mut state = state();
        let before_actor = state.actors[&actor_id()].clone();
        let mut log = EventLog::new();
        let mut context = PipelineContext {
            registry: &registry,
            state: &mut state,
            agent_state: Box::leak(Box::new(crate::state::AgentState::default())),
            log: &mut log,
            controller_bindings: None,
            epistemic_projection: None,
            content_manifest_id: ContentManifestId::new("phase1_manifest").unwrap(),
            ordering_key: ordering_key(),
        };
        let proposal = Proposal::new(
            ProposalId::new("proposal_inspect").unwrap(),
            ProposalOrigin::Test,
            Some(actor_id()),
            ActionId::new("inspect_entity").unwrap(),
            SimTick::ZERO,
        );

        let result = run_pipeline(&mut context, &proposal);

        assert!(result.appended_events.is_empty());
        assert!(log.events().is_empty());
        assert_eq!(state.actors[&actor_id()], before_actor);
    }
}
