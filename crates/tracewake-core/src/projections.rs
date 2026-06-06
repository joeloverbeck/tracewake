use crate::actions::report::ValidationReport;
use crate::events::log::EventLog;
use crate::ids::{ActionId, ActorId, ItemId, PlaceId, SemanticActionId, ViewModelId};
use crate::location::visible_locality;
use crate::state::PhysicalState;
use crate::time::SimTick;
use crate::view_models::{
    DebugEventLogView, DebugEventSummary, EmbodiedViewModel, SemanticActionEntry, ViewMode,
    VisibleActor, VisibleContainer, VisibleDoor, VisibleExit, VisibleItem,
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
        &visible.carried_items.into_iter().collect::<Vec<_>>(),
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
        local_actors,
        semantic_actions,
        last_rejection_summary: last_rejection.map(|report| report.actor_visible_summary.clone()),
        debug_available: true,
    })
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
}
