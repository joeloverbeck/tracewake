use tracewake_core::epistemics::observation::EPISTEMIC_RECORD_SCHEMA_V1;
use tracewake_core::epistemics::{
    Belief, Channel, Confidence, HolderKind, PrivacyScope, Proposition, SourceRef, Stance,
};
use tracewake_core::events::InitialBeliefSourceKind;
use tracewake_core::ids::{
    ActionId, ActorId, BeliefId, ContainerId, DoorId, FixtureId, ItemId, PlaceId, SchemaVersion,
};
use tracewake_core::location::Location;
use tracewake_core::state::{
    ActorBody, ContainerState, DoorState, ItemState, PhysicalState, PlaceState,
};
use tracewake_core::time::SimTick;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FixtureSchema {
    pub fixture_id: FixtureId,
    pub schema_version: SchemaVersion,
    pub actors: Vec<ActorSchema>,
    pub places: Vec<PlaceSchema>,
    pub doors: Vec<DoorSchema>,
    pub containers: Vec<ContainerSchema>,
    pub items: Vec<ItemSchema>,
    pub affordances: Vec<ActionAffordanceSchema>,
    pub initial_beliefs: Vec<InitialBeliefSchema>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActorSchema {
    pub actor_id: ActorId,
    pub current_place_id: PlaceId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlaceSchema {
    pub place_id: PlaceId,
    pub display_label: String,
    pub adjacent_place_ids: Vec<PlaceId>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DoorSchema {
    pub door_id: DoorId,
    pub endpoint_a: PlaceId,
    pub endpoint_b: PlaceId,
    pub is_open: bool,
    pub is_locked: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContainerSchema {
    pub container_id: ContainerId,
    pub place_id: PlaceId,
    pub is_open: bool,
    pub is_locked: bool,
    pub contents: Vec<ItemId>,
    pub contents_visible_when_closed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ItemSchema {
    pub item_id: ItemId,
    pub portable: bool,
    pub location: Location,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionAffordanceSchema {
    pub action_id: ActionId,
    pub target_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InitialBeliefSchema {
    pub belief_id: BeliefId,
    pub holder_actor_id: ActorId,
    pub proposition: Proposition,
    pub stance: Stance,
    pub confidence: Confidence,
    pub source_kind: InitialBeliefSourceKind,
    pub source: SourceRef,
    pub channel: Option<Channel>,
    pub acquired_tick: SimTick,
    pub last_verified_tick: Option<SimTick>,
    pub privacy_scope: PrivacyScope,
    pub schema_version: SchemaVersion,
}

impl InitialBeliefSchema {
    pub fn to_belief(&self) -> Belief {
        let mut belief = Belief::new(
            self.belief_id.clone(),
            HolderKind::Actor(self.holder_actor_id.clone()),
            self.proposition.clone(),
            self.stance,
            self.confidence,
            self.source.clone(),
            self.acquired_tick,
        );
        belief.channel = self.channel;
        belief.last_verified_tick = self.last_verified_tick;
        belief.privacy_scope = self.privacy_scope.clone();
        belief.schema_version = self.schema_version.clone();
        belief
    }

    pub fn new_expectation(
        belief_id: BeliefId,
        holder_actor_id: ActorId,
        proposition: Proposition,
        confidence: Confidence,
        source: SourceRef,
        acquired_tick: SimTick,
    ) -> Self {
        Self {
            belief_id,
            privacy_scope: PrivacyScope::ActorPrivate(holder_actor_id.clone()),
            holder_actor_id,
            proposition,
            stance: Stance::ExpectsTrue,
            confidence,
            source_kind: InitialBeliefSourceKind::AuthoredPrehistory,
            source,
            channel: None,
            acquired_tick,
            last_verified_tick: None,
            schema_version: SchemaVersion::new(EPISTEMIC_RECORD_SCHEMA_V1).unwrap(),
        }
    }
}

impl FixtureSchema {
    pub fn canonicalize(&mut self) {
        self.actors
            .sort_by(|left, right| left.actor_id.cmp(&right.actor_id));
        self.places
            .sort_by(|left, right| left.place_id.cmp(&right.place_id));
        self.doors
            .sort_by(|left, right| left.door_id.cmp(&right.door_id));
        self.containers
            .sort_by(|left, right| left.container_id.cmp(&right.container_id));
        self.items
            .sort_by(|left, right| left.item_id.cmp(&right.item_id));
        self.affordances.sort_by(|left, right| {
            (&left.action_id, &left.target_id).cmp(&(&right.action_id, &right.target_id))
        });
        self.initial_beliefs
            .sort_by(|left, right| left.belief_id.cmp(&right.belief_id));
        for place in &mut self.places {
            place.adjacent_place_ids.sort();
        }
        for container in &mut self.containers {
            container.contents.sort();
        }
    }

    pub fn to_physical_state(&self) -> PhysicalState {
        let mut state = PhysicalState::default();

        for place in &self.places {
            let mut place_state =
                PlaceState::new(place.place_id.clone(), place.display_label.clone());
            place_state
                .adjacent_place_ids
                .extend(place.adjacent_place_ids.iter().cloned());
            state.places.insert(place.place_id.clone(), place_state);
        }

        for actor in &self.actors {
            state.actors.insert(
                actor.actor_id.clone(),
                ActorBody::new(actor.actor_id.clone(), actor.current_place_id.clone()),
            );
            if let Some(place) = state.places.get_mut(&actor.current_place_id) {
                place.local_actor_ids.insert(actor.actor_id.clone());
            }
        }

        for door in &self.doors {
            let mut door_state = DoorState::new(
                door.door_id.clone(),
                door.endpoint_a.clone(),
                door.endpoint_b.clone(),
            );
            door_state.is_open = door.is_open;
            door_state.is_locked = door.is_locked;
            if let Some(place) = state.places.get_mut(&door.endpoint_a) {
                place.connected_door_ids.insert(door.door_id.clone());
            }
            if let Some(place) = state.places.get_mut(&door.endpoint_b) {
                place.connected_door_ids.insert(door.door_id.clone());
            }
            state.doors.insert(door.door_id.clone(), door_state);
        }

        for container in &self.containers {
            let mut container_state = ContainerState::fixed_at_place(
                container.container_id.clone(),
                container.place_id.clone(),
            );
            container_state.is_open = container.is_open;
            container_state.is_locked = container.is_locked;
            container_state
                .contents
                .extend(container.contents.iter().cloned());
            container_state.contents_visible_when_closed = container.contents_visible_when_closed;
            if let Some(place) = state.places.get_mut(&container.place_id) {
                place
                    .local_container_ids
                    .insert(container.container_id.clone());
            }
            state
                .containers
                .insert(container.container_id.clone(), container_state);
        }

        for item in &self.items {
            let mut item_state = ItemState::new(item.item_id.clone(), item.location.clone());
            item_state.portable = item.portable;
            match &item.location {
                Location::AtPlace(place_id) => {
                    if let Some(place) = state.places.get_mut(place_id) {
                        place.local_item_ids.insert(item.item_id.clone());
                    }
                }
                Location::CarriedBy(actor_id) => {
                    if let Some(actor) = state.actors.get_mut(actor_id) {
                        actor.carried_item_ids.insert(item.item_id.clone());
                    }
                }
                Location::InContainer(_) => {}
            }
            state.items.insert(item.item_id.clone(), item_state);
        }

        state
    }
}
