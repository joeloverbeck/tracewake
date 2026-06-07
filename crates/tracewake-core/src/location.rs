use std::collections::{BTreeMap, BTreeSet};

use crate::ids::{ActorId, ContainerId, DoorId, ItemId, PlaceId};
use crate::state::{ActorBody, ContainerState, DoorState, ItemState};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Location {
    AtPlace(PlaceId),
    InContainer(ContainerId),
    CarriedBy(ActorId),
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LocationAssertion {
    pub at_place: Option<PlaceId>,
    pub in_container: Option<ContainerId>,
    pub carried_by: Option<ActorId>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LocationError {
    MissingHolder,
    MultipleHolders { count: usize },
}

impl LocationAssertion {
    pub fn into_single_location(self) -> Result<Location, LocationError> {
        let count = usize::from(self.at_place.is_some())
            + usize::from(self.in_container.is_some())
            + usize::from(self.carried_by.is_some());

        match count {
            0 => Err(LocationError::MissingHolder),
            1 => {
                if let Some(place_id) = self.at_place {
                    Ok(Location::AtPlace(place_id))
                } else if let Some(container_id) = self.in_container {
                    Ok(Location::InContainer(container_id))
                } else {
                    Ok(Location::CarriedBy(
                        self.carried_by.expect("count checked above"),
                    ))
                }
            }
            count => Err(LocationError::MultipleHolders { count }),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VisibleLocality {
    pub current_place_id: PlaceId,
    pub connected_doors: BTreeSet<DoorId>,
    pub visible_containers: BTreeSet<ContainerId>,
    pub visible_items: BTreeSet<ItemId>,
    pub carried_items: BTreeSet<ItemId>,
    pub co_located_actors: BTreeSet<ActorId>,
}

pub fn visible_locality(
    actor: &ActorBody,
    actors: &BTreeMap<ActorId, ActorBody>,
    doors: &BTreeMap<DoorId, DoorState>,
    containers: &BTreeMap<ContainerId, ContainerState>,
    items: &BTreeMap<ItemId, ItemState>,
) -> VisibleLocality {
    let current_place_id = actor.current_place_id.clone();
    let mut connected_doors = BTreeSet::new();
    let mut visible_containers = BTreeSet::new();
    let mut visible_items = BTreeSet::new();
    let carried_items = actor.carried_item_ids.clone();
    let mut co_located_actors = BTreeSet::new();

    for (door_id, door) in doors {
        if door.connects_place(&current_place_id) {
            connected_doors.insert(door_id.clone());
        }
    }

    for (container_id, container) in containers {
        if container.location == Location::AtPlace(current_place_id.clone()) {
            visible_containers.insert(container_id.clone());
            if container.is_open || container.contents_visible_when_closed {
                visible_items.extend(container.contents.iter().filter_map(|item_id| {
                    items
                        .get(item_id)
                        .filter(|item| item.location == Location::InContainer(container_id.clone()))
                        .map(|item| item.item_id.clone())
                }));
            }
        }
    }

    for (item_id, item) in items {
        match &item.location {
            Location::AtPlace(place_id) if place_id == &current_place_id => {
                visible_items.insert(item_id.clone());
            }
            _ => {}
        }
    }

    for (other_actor_id, other_actor) in actors {
        if other_actor_id != &actor.actor_id && other_actor.current_place_id == current_place_id {
            co_located_actors.insert(other_actor_id.clone());
        }
    }

    VisibleLocality {
        current_place_id,
        connected_doors,
        visible_containers,
        visible_items,
        carried_items,
        co_located_actors,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{ActorBody, ContainerState, DoorState, ItemState};

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

    fn door_id(value: &str) -> DoorId {
        DoorId::new(value).unwrap()
    }

    #[test]
    fn location_assertion_accepts_exactly_one_holder() {
        assert_eq!(
            LocationAssertion {
                at_place: Some(place_id("shop_front")),
                in_container: None,
                carried_by: None,
            }
            .into_single_location()
            .unwrap(),
            Location::AtPlace(place_id("shop_front"))
        );
        assert_eq!(
            LocationAssertion {
                at_place: None,
                in_container: Some(container_id("strongbox_tomas")),
                carried_by: None,
            }
            .into_single_location()
            .unwrap(),
            Location::InContainer(container_id("strongbox_tomas"))
        );
        assert_eq!(
            LocationAssertion {
                at_place: None,
                in_container: None,
                carried_by: Some(actor_id("actor_tomas")),
            }
            .into_single_location()
            .unwrap(),
            Location::CarriedBy(actor_id("actor_tomas"))
        );
    }

    #[test]
    fn location_assertion_rejects_missing_or_multiple_holders() {
        assert_eq!(
            LocationAssertion::default().into_single_location(),
            Err(LocationError::MissingHolder)
        );
        assert_eq!(
            LocationAssertion {
                at_place: Some(place_id("shop_front")),
                in_container: Some(container_id("strongbox_tomas")),
                carried_by: None,
            }
            .into_single_location(),
            Err(LocationError::MultipleHolders { count: 2 })
        );
    }

    #[test]
    fn visibility_excludes_closed_opaque_contents_and_other_places() {
        let tomas = ActorBody::new(actor_id("actor_tomas"), place_id("shop_front"));
        let mara = ActorBody::new(actor_id("actor_mara"), place_id("shop_front"));
        let distant_actor = ActorBody::new(actor_id("actor_ren"), place_id("back_room"));
        let mut actors = BTreeMap::new();
        actors.insert(tomas.actor_id.clone(), tomas.clone());
        actors.insert(mara.actor_id.clone(), mara);
        actors.insert(distant_actor.actor_id.clone(), distant_actor);

        let door = DoorState::new(
            door_id("door_shop_back"),
            place_id("shop_front"),
            place_id("back_room"),
        );
        let mut doors = BTreeMap::new();
        doors.insert(door.door_id.clone(), door);

        let mut strongbox =
            ContainerState::fixed_at_place(container_id("strongbox_tomas"), place_id("shop_front"));
        strongbox.is_open = false;
        strongbox.contents_visible_when_closed = false;
        strongbox.contents.insert(item_id("coin_stack_01"));
        let mut containers = BTreeMap::new();
        containers.insert(strongbox.container_id.clone(), strongbox);

        let visible_coin = ItemState::new(
            item_id("loose_coin_01"),
            Location::AtPlace(place_id("shop_front")),
        );
        let hidden_coin = ItemState::new(
            item_id("coin_stack_01"),
            Location::InContainer(container_id("strongbox_tomas")),
        );
        let other_place_item = ItemState::new(
            item_id("ledger_01"),
            Location::AtPlace(place_id("back_room")),
        );
        let mut items = BTreeMap::new();
        items.insert(visible_coin.item_id.clone(), visible_coin);
        items.insert(hidden_coin.item_id.clone(), hidden_coin);
        items.insert(other_place_item.item_id.clone(), other_place_item);

        let visible = visible_locality(&tomas, &actors, &doors, &containers, &items);

        assert!(visible.connected_doors.contains(&door_id("door_shop_back")));
        assert!(visible
            .visible_containers
            .contains(&container_id("strongbox_tomas")));
        assert!(visible.visible_items.contains(&item_id("loose_coin_01")));
        assert!(!visible.visible_items.contains(&item_id("coin_stack_01")));
        assert!(!visible.visible_items.contains(&item_id("ledger_01")));
        assert!(visible.co_located_actors.contains(&actor_id("actor_mara")));
        assert!(!visible.co_located_actors.contains(&actor_id("actor_ren")));
    }

    #[test]
    fn visibility_keeps_carried_items_out_of_place_reachable_items() {
        let mut tomas = ActorBody::new(actor_id("actor_tomas"), place_id("shop_front"));
        tomas.carried_item_ids.insert(item_id("coin_stack_01"));
        let mut actors = BTreeMap::new();
        actors.insert(tomas.actor_id.clone(), tomas.clone());

        let doors = BTreeMap::new();
        let containers = BTreeMap::new();
        let mut items = BTreeMap::new();
        items.insert(
            item_id("coin_stack_01"),
            ItemState::new(
                item_id("coin_stack_01"),
                Location::CarriedBy(actor_id("actor_tomas")),
            ),
        );
        items.insert(
            item_id("loose_coin_01"),
            ItemState::new(
                item_id("loose_coin_01"),
                Location::AtPlace(place_id("shop_front")),
            ),
        );

        let visible = visible_locality(&tomas, &actors, &doors, &containers, &items);

        assert!(visible.visible_items.contains(&item_id("loose_coin_01")));
        assert!(!visible.visible_items.contains(&item_id("coin_stack_01")));
        assert!(visible.carried_items.contains(&item_id("coin_stack_01")));
        assert!(!visible.carried_items.contains(&item_id("loose_coin_01")));
    }
}
