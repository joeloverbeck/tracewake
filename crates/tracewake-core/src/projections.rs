use crate::actions::report::ValidationReport;
use crate::epistemics::{EpistemicProjection, KnowledgeContext, SourceRef};
use crate::events::log::EventLog;
use crate::ids::{ActionId, ActorId, ItemId, PlaceId, SemanticActionId, ViewModelId};
use crate::location::visible_locality;
use crate::state::PhysicalState;
use crate::time::SimTick;
use crate::view_models::{
    DebugEventLogView, DebugEventSummary, EmbodiedViewModel, NotebookBeliefEntry,
    NotebookContradictionEntry, NotebookObservationEntry, NotebookView, SemanticActionEntry,
    ViewMode, VisibleActor, VisibleContainer, VisibleDoor, VisibleExit, VisibleItem,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProjectionError {
    ActorNotFound(ActorId),
    PlaceNotFound(PlaceId),
}

pub fn build_embodied_view_model(
    state: &PhysicalState,
    viewer_actor_id: &ActorId,
    sim_tick: SimTick,
    last_rejection: Option<&ValidationReport>,
) -> Result<EmbodiedViewModel, ProjectionError> {
    let actor = state
        .actors
        .get(viewer_actor_id)
        .ok_or_else(|| ProjectionError::ActorNotFound(viewer_actor_id.clone()))?;
    let place = state
        .places
        .get(&actor.current_place_id)
        .ok_or_else(|| ProjectionError::PlaceNotFound(actor.current_place_id.clone()))?;
    let visible = visible_locality(
        actor,
        &state.actors,
        &state.doors,
        &state.containers,
        &state.items,
    );

    let mut visible_exits = place
        .adjacent_place_ids
        .iter()
        .cloned()
        .map(|destination_place_id| VisibleExit {
            destination_place_id,
            blocker_summary: None,
        })
        .collect::<Vec<_>>();
    visible_exits.sort();

    let mut visible_doors = visible
        .connected_doors
        .iter()
        .filter_map(|door_id| state.doors.get(door_id))
        .map(|door| VisibleDoor {
            door_id: door.door_id.clone(),
            endpoint_a: door.endpoint_a.clone(),
            endpoint_b: door.endpoint_b.clone(),
            is_open: door.is_open,
            is_locked: door.is_locked,
        })
        .collect::<Vec<_>>();
    visible_doors.sort();

    let mut visible_containers = visible
        .visible_containers
        .iter()
        .filter_map(|container_id| state.containers.get(container_id))
        .map(|container| VisibleContainer {
            container_id: container.container_id.clone(),
            is_open: container.is_open,
            is_locked: container.is_locked,
        })
        .collect::<Vec<_>>();
    visible_containers.sort();

    let mut visible_items = visible
        .visible_items
        .iter()
        .filter_map(|item_id| state.items.get(item_id))
        .map(|item| VisibleItem {
            item_id: item.item_id.clone(),
            portable: item.portable,
        })
        .collect::<Vec<_>>();
    visible_items.sort();

    let mut carried_items = visible
        .carried_items
        .iter()
        .filter_map(|item_id| state.items.get(item_id))
        .map(|item| VisibleItem {
            item_id: item.item_id.clone(),
            portable: item.portable,
        })
        .collect::<Vec<_>>();
    carried_items.sort();
    let carried_item_ids = carried_items
        .iter()
        .map(|item| item.item_id.clone())
        .collect::<Vec<_>>();

    let mut local_actors = visible
        .co_located_actors
        .iter()
        .cloned()
        .map(|actor_id| VisibleActor { actor_id })
        .collect::<Vec<_>>();
    local_actors.sort();

    let mut semantic_actions = semantic_actions(
        &visible_exits,
        &visible_doors,
        &visible_containers,
        &visible_items,
        &carried_item_ids,
    );
    semantic_actions.sort();

    Ok(EmbodiedViewModel {
        view_model_id: ViewModelId::new(format!(
            "view.{}.{}",
            viewer_actor_id.as_str(),
            sim_tick.value()
        ))
        .unwrap(),
        mode: ViewMode::Embodied,
        viewer_actor_id: viewer_actor_id.clone(),
        sim_tick,
        place_id: actor.current_place_id.clone(),
        place_label: place.display_label.clone(),
        visible_exits,
        visible_doors,
        visible_containers,
        visible_items,
        carried_items,
        local_actors,
        semantic_actions,
        last_rejection_summary: last_rejection.map(|report| report.actor_visible_summary.clone()),
        knowledge_context_id: None,
        notebook: None,
        debug_available: true,
    })
}

pub fn build_embodied_view_model_with_notebook(
    state: &PhysicalState,
    viewer_actor_id: &ActorId,
    sim_tick: SimTick,
    last_rejection: Option<&ValidationReport>,
    projection: &EpistemicProjection,
) -> Result<EmbodiedViewModel, ProjectionError> {
    let mut view = build_embodied_view_model(state, viewer_actor_id, sim_tick, last_rejection)?;
    let context = KnowledgeContext::embodied(viewer_actor_id.clone(), sim_tick);
    view.knowledge_context_id = Some(format!(
        "knowledge.{}.{}",
        viewer_actor_id.as_str(),
        sim_tick.value()
    ));
    view.notebook = Some(build_notebook_view(projection, &context));
    Ok(view)
}

pub fn build_notebook_view(
    projection: &EpistemicProjection,
    context: &KnowledgeContext,
) -> NotebookView {
    let mut beliefs = projection
        .beliefs_for_context(context)
        .into_iter()
        .map(|belief| NotebookBeliefEntry {
            belief_id: belief.belief_id.as_str().to_string(),
            summary: belief.proposition.render(),
            source_summary: source_summary(&belief.source),
            confidence_label: belief.confidence.serialize_canonical(),
            acquired_tick: belief.acquired_tick.value(),
            contradiction_ids: belief
                .contradiction_ids
                .iter()
                .map(|id| id.as_str().to_string())
                .collect(),
        })
        .collect::<Vec<_>>();
    beliefs.sort();

    let mut observations = projection
        .observations_for_context(context)
        .into_iter()
        .map(|observation| NotebookObservationEntry {
            observation_id: observation.observation_id.as_str().to_string(),
            channel: observation.channel.stable_id().to_string(),
            summary: format!("{} observation", observation.channel.stable_id()),
            confidence_label: observation.confidence.serialize_canonical(),
            observed_tick: observation.observed_tick.value(),
        })
        .collect::<Vec<_>>();
    observations.sort();

    let mut contradictions = projection
        .contradictions_for_context(context)
        .into_iter()
        .map(|contradiction| NotebookContradictionEntry {
            contradiction_id: contradiction.contradiction_id.as_str().to_string(),
            summary: "Contradicts your earlier expectation.".to_string(),
        })
        .collect::<Vec<_>>();
    contradictions.sort();

    let possible_leads = beliefs
        .iter()
        .filter(|belief| belief.summary.contains("missing from expected location"))
        .map(|belief| format!("Source-bound lead from {}", belief.belief_id))
        .collect();

    NotebookView {
        viewer_actor_id: context.viewer_actor_id.clone(),
        source_bound_beliefs: beliefs,
        recent_observations: observations,
        known_contradictions: contradictions,
        possible_leads,
    }
}

fn source_summary(source: &SourceRef) -> String {
    match source {
        SourceRef::Event(event_id) => format!("event:{}", event_id.as_str()),
        SourceRef::Action(action_id) => format!("action:{}", action_id.as_str()),
        SourceRef::Cause(cause) => format!("cause:{cause:?}"),
    }
}

pub fn build_debug_event_log_view(log: &EventLog) -> DebugEventLogView {
    DebugEventLogView {
        debug_only: true,
        events: log.events().iter().map(DebugEventSummary::from).collect(),
    }
}

fn semantic_actions(
    visible_exits: &[VisibleExit],
    visible_doors: &[VisibleDoor],
    visible_containers: &[VisibleContainer],
    visible_items: &[VisibleItem],
    carried_items: &[ItemId],
) -> Vec<SemanticActionEntry> {
    let mut actions = Vec::new();
    actions.push(SemanticActionEntry::new(
        SemanticActionId::new("wait.1_tick").unwrap(),
        ActionId::new("wait").unwrap(),
        vec!["1_tick".to_string()],
        "Wait",
        true,
        None,
    ));

    for exit in visible_exits {
        actions.push(SemanticActionEntry::new(
            SemanticActionId::new(format!("move.to.{}", exit.destination_place_id.as_str()))
                .unwrap(),
            ActionId::new("move").unwrap(),
            vec![exit.destination_place_id.to_string()],
            format!("Move to {}", exit.destination_place_id.as_str()),
            true,
            None,
        ));
    }

    for door in visible_doors {
        let action = if door.is_open { "close" } else { "open" };
        actions.push(SemanticActionEntry::new(
            SemanticActionId::new(format!("{action}.door.{}", door.door_id.as_str())).unwrap(),
            ActionId::new(action).unwrap(),
            vec![door.door_id.to_string()],
            format!("{action} {}", door.door_id.as_str()),
            !door.is_locked,
            door.is_locked.then(|| "locked".to_string()),
        ));
    }

    for container in visible_containers {
        actions.push(SemanticActionEntry::new(
            SemanticActionId::new(format!(
                "check.container.{}",
                container.container_id.as_str()
            ))
            .unwrap(),
            ActionId::new("check_container").unwrap(),
            vec![container.container_id.to_string()],
            format!("Check {}", container.container_id.as_str()),
            true,
            None,
        ));
        let action = if container.is_open { "close" } else { "open" };
        actions.push(SemanticActionEntry::new(
            SemanticActionId::new(format!(
                "{action}.container.{}",
                container.container_id.as_str()
            ))
            .unwrap(),
            ActionId::new(action).unwrap(),
            vec![container.container_id.to_string()],
            format!("{action} {}", container.container_id.as_str()),
            !container.is_locked,
            container.is_locked.then(|| "locked".to_string()),
        ));
    }

    for item in visible_items {
        actions.push(SemanticActionEntry::new(
            SemanticActionId::new(format!("take.item.{}.from.place", item.item_id.as_str()))
                .unwrap(),
            ActionId::new("take").unwrap(),
            vec![item.item_id.to_string()],
            format!("Take {}", item.item_id.as_str()),
            item.portable,
            (!item.portable).then(|| "not portable".to_string()),
        ));
        actions.push(SemanticActionEntry::new(
            SemanticActionId::new(format!("inspect.item.{}", item.item_id.as_str())).unwrap(),
            ActionId::new("inspect_entity").unwrap(),
            vec![item.item_id.to_string()],
            format!("Inspect {}", item.item_id.as_str()),
            true,
            None,
        ));
    }

    for item_id in carried_items {
        actions.push(SemanticActionEntry::new(
            SemanticActionId::new(format!("place.item.{}.at.place", item_id.as_str())).unwrap(),
            ActionId::new("place").unwrap(),
            vec![item_id.to_string()],
            format!("Place {}", item_id.as_str()),
            true,
            None,
        ));
    }

    actions
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::epistemics::{
        Belief, Channel, Confidence, Contradiction, ContradictionKind, HolderKind, Observation,
        ObservationSubject, ObservationTarget, PrivacyScope, Proposition, SourceRef, Stance,
    };
    use crate::ids::{ContainerId, DoorId};
    use crate::location::Location;
    use crate::state::{
        ActorBody, ContainerState, DoorState, ItemState, PhysicalState, PlaceState,
    };

    fn actor_id(value: &str) -> ActorId {
        ActorId::new(value).unwrap()
    }

    fn place_id(value: &str) -> PlaceId {
        PlaceId::new(value).unwrap()
    }

    fn item_id(value: &str) -> ItemId {
        ItemId::new(value).unwrap()
    }

    fn container_id(value: &str) -> ContainerId {
        ContainerId::new(value).unwrap()
    }

    fn event_id(value: &str) -> crate::ids::EventId {
        crate::ids::EventId::new(value).unwrap()
    }

    fn state() -> PhysicalState {
        let mut state = PhysicalState::default();
        let mut shop = PlaceState::new(place_id("shop_front"), "Shop front");
        shop.adjacent_place_ids.insert(place_id("back_room"));
        state.places.insert(place_id("shop_front"), shop);
        state.places.insert(
            place_id("back_room"),
            PlaceState::new(place_id("back_room"), "Back room"),
        );
        state.actors.insert(
            actor_id("actor_tomas"),
            ActorBody::new(actor_id("actor_tomas"), place_id("shop_front")),
        );
        state.actors.insert(
            actor_id("actor_mara"),
            ActorBody::new(actor_id("actor_mara"), place_id("shop_front")),
        );
        let mut container =
            ContainerState::fixed_at_place(container_id("strongbox_tomas"), place_id("shop_front"));
        container.is_open = false;
        container.contents_visible_when_closed = false;
        container.contents.insert(item_id("coin_stack_01"));
        state
            .containers
            .insert(container_id("strongbox_tomas"), container);
        state.items.insert(
            item_id("coin_stack_01"),
            ItemState::new(
                item_id("coin_stack_01"),
                Location::InContainer(container_id("strongbox_tomas")),
            ),
        );
        state.items.insert(
            item_id("loose_coin_01"),
            ItemState::new(
                item_id("loose_coin_01"),
                Location::AtPlace(place_id("shop_front")),
            ),
        );
        state.items.insert(
            item_id("ledger_01"),
            ItemState::new(
                item_id("ledger_01"),
                Location::AtPlace(place_id("back_room")),
            ),
        );
        let mut door = DoorState::new(
            DoorId::new("door_shop_back").unwrap(),
            place_id("shop_front"),
            place_id("back_room"),
        );
        door.is_open = true;
        state.doors.insert(door.door_id.clone(), door);
        state
    }

    #[test]
    fn embodied_projection_excludes_hidden_and_debug_truth() {
        let view =
            build_embodied_view_model(&state(), &actor_id("actor_tomas"), SimTick::new(1), None)
                .unwrap();

        assert!(view
            .visible_containers
            .iter()
            .any(|container| container.container_id == container_id("strongbox_tomas")));
        assert!(view
            .visible_items
            .iter()
            .any(|item| item.item_id == item_id("loose_coin_01")));
        assert!(!view
            .visible_items
            .iter()
            .any(|item| item.item_id == item_id("coin_stack_01")));
        assert!(!view
            .visible_items
            .iter()
            .any(|item| item.item_id == item_id("ledger_01")));
        assert_eq!(view.mode, ViewMode::Embodied);
        assert!(view.debug_available);
    }

    #[test]
    fn semantic_actions_are_target_specific_and_deterministic() {
        let first =
            build_embodied_view_model(&state(), &actor_id("actor_tomas"), SimTick::new(1), None)
                .unwrap();
        let second =
            build_embodied_view_model(&state(), &actor_id("actor_tomas"), SimTick::new(1), None)
                .unwrap();

        assert_eq!(first.semantic_actions, second.semantic_actions);
        assert!(first
            .semantic_actions
            .iter()
            .any(|entry| entry.semantic_action_id.as_str() == "move.to.back_room"));
        assert!(first
            .semantic_actions
            .iter()
            .all(|entry| entry.semantic_action_id.as_str() != "0"));
    }

    #[test]
    fn embodied_projection_separates_carried_items_from_reachable_items() {
        let mut state = state();
        state
            .actors
            .get_mut(&actor_id("actor_tomas"))
            .unwrap()
            .carried_item_ids
            .insert(item_id("coin_stack_01"));
        state
            .items
            .get_mut(&item_id("coin_stack_01"))
            .unwrap()
            .location = Location::CarriedBy(actor_id("actor_tomas"));
        state
            .containers
            .get_mut(&container_id("strongbox_tomas"))
            .unwrap()
            .contents
            .remove(&item_id("coin_stack_01"));

        let view =
            build_embodied_view_model(&state, &actor_id("actor_tomas"), SimTick::new(1), None)
                .unwrap();

        assert!(!view
            .visible_items
            .iter()
            .any(|item| item.item_id == item_id("coin_stack_01")));
        assert!(view
            .carried_items
            .iter()
            .any(|item| item.item_id == item_id("coin_stack_01")));
        assert!(
            !view
                .semantic_actions
                .iter()
                .any(|entry| entry.semantic_action_id.as_str()
                    == "take.item.coin_stack_01.from.place")
        );
        assert!(view
            .semantic_actions
            .iter()
            .any(|entry| entry.semantic_action_id.as_str() == "place.item.coin_stack_01.at.place"));
    }

    fn projection_with_missing_coin_belief() -> EpistemicProjection {
        let mut projection = EpistemicProjection::new(
            crate::ids::ContentManifestId::new("phase2a_manifest").unwrap(),
        );
        let observation_id = crate::ids::ObservationId::new("obs_tomas_checked_strongbox").unwrap();
        let contradiction_id =
            crate::ids::ContradictionId::new("contradiction_tomas_missing_coin").unwrap();
        projection.insert_observation(Observation::new(
            observation_id.clone(),
            actor_id("actor_tomas"),
            Channel::AbsenceMarker,
            SimTick::new(3),
            place_id("shop_front"),
            ObservationSubject::Container(container_id("strongbox_tomas")),
            ObservationTarget::Container(container_id("strongbox_tomas")),
            Confidence::new(1000).unwrap(),
            SourceRef::Event(event_id("event_observation")),
        ));
        projection.insert_contradiction(Contradiction::new(
            contradiction_id.clone(),
            actor_id("actor_tomas"),
            ContradictionKind::ExpectedItemAbsentFromContainer,
            crate::ids::BeliefId::new("belief_tomas_expected_coin").unwrap(),
            observation_id.clone(),
            Proposition::ItemLocatedInContainer {
                item_id: item_id("coin_stack_01"),
                container_id: container_id("strongbox_tomas"),
            },
            Proposition::ItemMissingFromExpectedLocation {
                item_id: item_id("coin_stack_01"),
                expected_location: Location::InContainer(container_id("strongbox_tomas")),
            },
            SimTick::new(3),
        ));
        projection.insert_belief(
            Belief::new(
                crate::ids::BeliefId::new("belief_tomas_missing_coin").unwrap(),
                HolderKind::Actor(actor_id("actor_tomas")),
                Proposition::ItemMissingFromExpectedLocation {
                    item_id: item_id("coin_stack_01"),
                    expected_location: Location::InContainer(container_id("strongbox_tomas")),
                },
                Stance::BelievesTrue,
                Confidence::new(1000).unwrap(),
                SourceRef::Event(event_id("event_observation")),
                SimTick::new(3),
            )
            .with_observation(observation_id)
            .with_contradiction(contradiction_id),
        );

        let mut elena_belief = Belief::new(
            crate::ids::BeliefId::new("belief_elena_private_sound").unwrap(),
            HolderKind::Actor(actor_id("actor_elena")),
            Proposition::SoundHeardNearPlace {
                place_id: place_id("shop_front"),
            },
            Stance::Plausible,
            Confidence::new(250).unwrap(),
            SourceRef::Event(event_id("event_elena_sound")),
            SimTick::new(2),
        );
        elena_belief.privacy_scope = PrivacyScope::ActorPrivate(actor_id("actor_elena"));
        projection.insert_belief(elena_belief);
        projection
    }

    #[test]
    fn notebook_for_tomas_shows_missing_belief_without_culprit_leak() {
        let projection = projection_with_missing_coin_belief();
        let context = KnowledgeContext::embodied(actor_id("actor_tomas"), SimTick::new(4));

        let notebook = build_notebook_view(&projection, &context);

        assert_eq!(notebook.viewer_actor_id, actor_id("actor_tomas"));
        assert_eq!(notebook.source_bound_beliefs.len(), 1);
        let rendered = format!("{notebook:?}");
        assert!(rendered.contains("coin_stack_01 is missing from expected location"));
        assert!(rendered.contains("Contradicts your earlier expectation."));
        assert!(rendered.contains("1000"));
        assert!(rendered.contains("event:event_observation"));
        assert!(!rendered.contains("actor_mara"));
        assert!(!rendered.contains("culprit"));
        assert!(!rendered.contains("previous"));
        assert!(!rendered.contains("debug note"));
    }

    #[test]
    fn notebook_excludes_other_actor_private_beliefs() {
        let projection = projection_with_missing_coin_belief();
        let context = KnowledgeContext::embodied(actor_id("actor_tomas"), SimTick::new(4));

        let notebook = build_notebook_view(&projection, &context);
        let rendered = format!("{notebook:?}");

        assert!(!rendered.contains("belief_elena_private_sound"));
        assert!(!rendered.contains("sound was heard"));
    }

    #[test]
    fn embodied_view_can_carry_actor_known_notebook() {
        let projection = projection_with_missing_coin_belief();
        let view = build_embodied_view_model_with_notebook(
            &state(),
            &actor_id("actor_tomas"),
            SimTick::new(4),
            None,
            &projection,
        )
        .unwrap();

        assert!(view.knowledge_context_id.is_some());
        assert_eq!(
            view.notebook.unwrap().source_bound_beliefs[0].acquired_tick,
            3
        );
    }
}
