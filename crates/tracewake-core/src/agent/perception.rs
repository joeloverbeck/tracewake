use crate::epistemics::{Channel, Confidence};
use crate::events::log::EventLog;
use crate::events::{EventEnvelope, EventKind, PayloadField, EVENT_SCHEMA_V1};
use crate::ids::{ActionId, ActorId, ContentManifestId, EventId, PlaceId, ProcessId};
use crate::location::Location;
use crate::scheduler::{OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId};
use crate::state::PhysicalState;
use crate::time::SimTick;

pub fn record_current_place_perception(
    log: &mut EventLog,
    state: &PhysicalState,
    actor_id: &ActorId,
    decision_tick: SimTick,
    content_manifest_id: &ContentManifestId,
) -> Vec<EventEnvelope> {
    current_place_perception_events(state, actor_id, decision_tick, content_manifest_id)
        .into_iter()
        .map(|event| {
            log.append(event)
                .expect("current-place perception event is versioned")
        })
        .collect()
}

pub fn current_place_perception_events(
    state: &PhysicalState,
    actor_id: &ActorId,
    decision_tick: SimTick,
    content_manifest_id: &ContentManifestId,
) -> Vec<EventEnvelope> {
    let Some(actor) = state.actors().get(actor_id) else {
        return Vec::new();
    };
    let current_place_id = actor.current_place_id.clone();
    let mut observations = Vec::new();

    if let Some(place) = state.places().get(&current_place_id) {
        for adjacent_place_id in &place.adjacent_place_ids {
            if !is_visible_exit_target(state, adjacent_place_id) {
                continue;
            }
            observations.push(PerceivedThing {
                kind: "visible_exit",
                subject_id: current_place_id.as_str().to_string(),
                target_id: adjacent_place_id.as_str().to_string(),
                place_id: current_place_id.clone(),
            });
        }
    }

    for food in state.food_supplies().values().filter(|food| {
        matches!(&food.location, Location::AtPlace(place_id) if place_id == &current_place_id)
    }) {
        observations.push(PerceivedThing {
            kind: "visible_food_supply",
            subject_id: current_place_id.as_str().to_string(),
            target_id: food.food_supply_id.as_str().to_string(),
            place_id: current_place_id.clone(),
        });
    }

    for sleep_affordance in state
        .sleep_affordances()
        .values()
        .filter(|sleep_affordance| {
            sleep_affordance.access_open && sleep_affordance.place_id == current_place_id
        })
    {
        observations.push(PerceivedThing {
            kind: "visible_sleep_affordance",
            subject_id: current_place_id.as_str().to_string(),
            target_id: sleep_affordance.sleep_affordance_id.as_str().to_string(),
            place_id: current_place_id.clone(),
        });
    }

    observations
        .into_iter()
        .enumerate()
        .map(|(index, perceived)| {
            observation_event(
                actor_id,
                decision_tick,
                index as u64,
                content_manifest_id,
                perceived,
            )
        })
        .collect()
}

fn is_visible_exit_target(state: &PhysicalState, place_id: &PlaceId) -> bool {
    let Some(place) = state.places().get(place_id) else {
        return false;
    };
    !place.place_id.as_str().contains("hidden")
        && !place.display_label.to_lowercase().contains("hidden")
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PerceivedThing {
    kind: &'static str,
    subject_id: String,
    target_id: String,
    place_id: PlaceId,
}

fn observation_event(
    actor_id: &ActorId,
    decision_tick: SimTick,
    index: u64,
    content_manifest_id: &ContentManifestId,
    perceived: PerceivedThing,
) -> EventEnvelope {
    let event_id = EventId::new(format!(
        "event.perception.{}.{}.{}.{}",
        actor_id.as_str(),
        decision_tick.value(),
        perceived.kind,
        perceived.target_id
    ))
    .expect("perception event id is built from stable ids");
    let observation_id = format!("obs.perception.{}", event_id.as_str());
    let ordering_key = OrderingKey::new(
        decision_tick,
        SchedulePhase::NoHumanProcess,
        SchedulerSourceId::Actor(actor_id.clone()),
        ProposalSequence::new(index),
        ActionId::new("perceive_current_place").unwrap(),
        vec![perceived.kind.to_string(), perceived.target_id.clone()],
        format!("perception_{index:04}"),
    );
    let mut event = EventEnvelope::new_v1(
        event_id.clone(),
        EventKind::ObservationRecorded,
        0,
        0,
        decision_tick,
        ordering_key,
        content_manifest_id.clone(),
    );
    event.actor_id = Some(actor_id.clone());
    event.place_id = Some(perceived.place_id.clone());
    event.process_id = Some(ProcessId::new("current_place_perception").unwrap());
    event.participants = vec![
        actor_id.to_string(),
        perceived.place_id.to_string(),
        perceived.target_id.clone(),
    ];
    event.payload = vec![
        PayloadField::new("schema_version", EVENT_SCHEMA_V1),
        PayloadField::new("observation_id", observation_id),
        PayloadField::new("observer_actor_id", actor_id.as_str()),
        PayloadField::new("observer_place_id", perceived.place_id.as_str()),
        PayloadField::new("place_id", perceived.place_id.as_str()),
        PayloadField::new("channel", Channel::DirectSight.stable_id()),
        PayloadField::new("observed_tick", decision_tick.value().to_string()),
        PayloadField::new(
            "confidence",
            Confidence::MAX.serialize_canonical().to_string(),
        ),
        PayloadField::new("source_event_id", event_id.as_str()),
        PayloadField::new("perceived_kind", perceived.kind),
        PayloadField::new("subject_id", perceived.subject_id),
        PayloadField::new("target_id", perceived.target_id.clone()),
    ];
    event.effects_summary = format!(
        "current-place perception recorded {} {}",
        perceived.kind, perceived.target_id
    );
    event
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::ids::{FoodSupplyId, SleepAffordanceId};
    use crate::state::{ActorBody, FoodSupplyState, PlaceState, SleepAffordanceState};

    fn actor_id(value: &str) -> ActorId {
        ActorId::new(value).unwrap()
    }

    fn place_id(value: &str) -> PlaceId {
        PlaceId::new(value).unwrap()
    }

    fn manifest_id() -> ContentManifestId {
        ContentManifestId::new("manifest_perception_test").unwrap()
    }

    fn state_with_visible_current_place_surfaces() -> PhysicalState {
        let actor = actor_id("actor_tomas");
        let home = place_id("home_tomas");
        let workshop = place_id("workshop_tomas");
        let mut home_state = PlaceState::new(home.clone(), "Tomas home");
        home_state.adjacent_place_ids.insert(workshop.clone());
        let workshop_state = PlaceState::new(workshop.clone(), "Workshop");

        let mut actors = BTreeMap::new();
        actors.insert(actor.clone(), ActorBody::new(actor, home.clone()));
        let mut places = BTreeMap::new();
        places.insert(home.clone(), home_state);
        places.insert(workshop.clone(), workshop_state);

        let food_supply_id = FoodSupplyId::new("food_breakfast_tomas").unwrap();
        let mut food_supplies = BTreeMap::new();
        food_supplies.insert(
            food_supply_id.clone(),
            FoodSupplyState::new(food_supply_id, Location::AtPlace(home.clone()), 2, 220),
        );

        let sleep_affordance_id = SleepAffordanceId::new("bed_tomas").unwrap();
        let mut sleep_affordances = BTreeMap::new();
        sleep_affordances.insert(
            sleep_affordance_id.clone(),
            SleepAffordanceState::new(sleep_affordance_id, home),
        );

        PhysicalState::from_seed_parts(
            actors,
            places,
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
            food_supplies,
            BTreeMap::new(),
            sleep_affordances,
        )
    }

    fn state_without_visible_surfaces() -> PhysicalState {
        let actor = actor_id("actor_tomas");
        let home = place_id("home_tomas");
        let mut actors = BTreeMap::new();
        actors.insert(actor.clone(), ActorBody::new(actor, home.clone()));
        let mut places = BTreeMap::new();
        places.insert(home.clone(), PlaceState::new(home, "Tomas home"));
        PhysicalState::from_seed_parts(
            actors,
            places,
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
        )
    }

    #[test]
    fn current_place_perception_emits_window_keyed_observation_events() {
        let events = current_place_perception_events(
            &state_with_visible_current_place_surfaces(),
            &actor_id("actor_tomas"),
            SimTick::new(4),
            &manifest_id(),
        );

        let perceived_kinds = events
            .iter()
            .filter_map(|event| {
                event
                    .payload
                    .iter()
                    .find(|field| field.key == "perceived_kind")
                    .map(|field| field.value.as_str())
            })
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(
            perceived_kinds,
            std::collections::BTreeSet::from([
                "visible_exit",
                "visible_food_supply",
                "visible_sleep_affordance"
            ])
        );
        assert!(events.iter().all(|event| {
            event.event_type == EventKind::ObservationRecorded
                && event.ordering_key.sim_tick == SimTick::new(4)
                && matches!(event.ordering_key.source_id, SchedulerSourceId::Actor(_))
                && event.payload.iter().any(|field| {
                    field.key == "source_event_id" && field.value == event.event_id.as_str()
                })
        }));
    }

    #[test]
    fn current_place_perception_emits_no_absence_facts() {
        let events = current_place_perception_events(
            &state_without_visible_surfaces(),
            &actor_id("actor_tomas"),
            SimTick::new(4),
            &manifest_id(),
        );

        assert!(events.is_empty());
    }

    #[test]
    fn current_place_perception_appends_byte_identically() {
        let state = state_with_visible_current_place_surfaces();
        let mut first = EventLog::new();
        let mut second = EventLog::new();

        record_current_place_perception(
            &mut first,
            &state,
            &actor_id("actor_tomas"),
            SimTick::new(4),
            &manifest_id(),
        );
        record_current_place_perception(
            &mut second,
            &state,
            &actor_id("actor_tomas"),
            SimTick::new(4),
            &manifest_id(),
        );

        assert_eq!(first.serialize_canonical(), second.serialize_canonical());
    }
}
