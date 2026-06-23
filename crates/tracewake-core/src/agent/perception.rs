use crate::epistemics::{
    ActorKnownCarriedItemFact, ActorKnownContainerFact, ActorKnownCurrentPlaceFact,
    ActorKnownDoorFact, ActorKnownFoodSourceFact, ActorKnownItemFact, ActorKnownLocalActorFact,
    ActorKnownProjectionRecord, ActorKnownRouteFact, ActorKnownSleepAffordanceFact,
    ActorKnownWorkplaceFact, AllowedKnowledgeSource, Channel, Confidence, EpistemicProjection,
    KnowledgeContext, KnowledgeProvenanceEntry, KnowledgeProvenanceKind, Observation,
    ObservationSubject, ObservationTarget, SourceRef,
};
use crate::events::log::EventLog;
use crate::events::{EventCause, EventEnvelope, EventKind, PayloadField, EVENT_SCHEMA_V1};
use crate::ids::{
    ActionId, ActorId, ContentManifestId, EventId, FoodSupplyId, ObservationId, PlaceId, ProcessId,
    SleepAffordanceId,
};
use crate::location::Location;
use crate::scheduler::{OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId};
use crate::state::{AgentState, PhysicalState};
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

pub fn record_current_place_perception_and_project(
    log: &mut EventLog,
    state: &mut PhysicalState,
    _agent_state: &mut AgentState,
    epistemic_projection: &mut EpistemicProjection,
    actor_id: &ActorId,
    decision_tick: SimTick,
    content_manifest_id: &ContentManifestId,
) -> Vec<EventEnvelope> {
    let events =
        record_current_place_perception(log, state, actor_id, decision_tick, content_manifest_id);
    for event in &events {
        project_perception_event(epistemic_projection, event);
    }
    events
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
        observations.push(PerceivedThing {
            kind: "current_place",
            subject_id: current_place_id.as_str().to_string(),
            target_id: current_place_id.as_str().to_string(),
            place_id: current_place_id.clone(),
            servings: None,
            payload: vec![PayloadField::new(
                "display_label",
                place.display_label.clone(),
            )],
        });

        for adjacent_place_id in &place.adjacent_place_ids {
            if !is_visible_exit_target(state, adjacent_place_id) {
                continue;
            }
            observations.push(PerceivedThing {
                kind: "visible_exit",
                subject_id: current_place_id.as_str().to_string(),
                target_id: adjacent_place_id.as_str().to_string(),
                place_id: current_place_id.clone(),
                servings: None,
                payload: Vec::new(),
            });
        }
    }

    for item_id in &actor.carried_item_ids {
        let Some(item) = state.items().get(item_id) else {
            continue;
        };
        observations.push(PerceivedThing {
            kind: "carried_item",
            subject_id: current_place_id.as_str().to_string(),
            target_id: item.item_id.as_str().to_string(),
            place_id: current_place_id.clone(),
            servings: None,
            payload: vec![
                PayloadField::new("item_source_kind", "carried"),
                PayloadField::new("item_source_actor_id", actor.actor_id.as_str()),
                PayloadField::new("portable", item.portable.to_string()),
            ],
        });
    }

    for door in state
        .doors()
        .values()
        .filter(|door| door.connects_place(&current_place_id))
    {
        observations.push(PerceivedThing {
            kind: "visible_door",
            subject_id: current_place_id.as_str().to_string(),
            target_id: door.door_id.as_str().to_string(),
            place_id: current_place_id.clone(),
            servings: None,
            payload: vec![
                PayloadField::new("endpoint_a", door.endpoint_a.as_str()),
                PayloadField::new("endpoint_b", door.endpoint_b.as_str()),
                PayloadField::new("is_open", door.is_open.to_string()),
                PayloadField::new("is_locked", door.is_locked.to_string()),
                PayloadField::new(
                    "blocks_movement_when_closed",
                    door.blocks_movement_when_closed.to_string(),
                ),
            ],
        });
    }

    for container in state.containers().values().filter(|container| {
        matches!(&container.location, Location::AtPlace(place_id) if place_id == &current_place_id)
    }) {
        observations.push(PerceivedThing {
            kind: "visible_container",
            subject_id: current_place_id.as_str().to_string(),
            target_id: container.container_id.as_str().to_string(),
            place_id: current_place_id.clone(),
            servings: None,
            payload: vec![
                PayloadField::new("is_open", container.is_open.to_string()),
                PayloadField::new("is_locked", container.is_locked.to_string()),
            ],
        });
    }

    for item in state.items().values() {
        let Some(payload) = visible_item_payload(state, &current_place_id, &item.location) else {
            continue;
        };
        observations.push(PerceivedThing {
            kind: "visible_item",
            subject_id: current_place_id.as_str().to_string(),
            target_id: item.item_id.as_str().to_string(),
            place_id: current_place_id.clone(),
            servings: None,
            payload: payload
                .into_iter()
                .chain([PayloadField::new("portable", item.portable.to_string())])
                .collect(),
        });
    }

    for local_actor_id in state.actors().keys().filter(|local_actor_id| {
        *local_actor_id != actor_id
            && state
                .actors()
                .get(*local_actor_id)
                .is_some_and(|local_actor| local_actor.current_place_id == current_place_id)
    }) {
        observations.push(PerceivedThing {
            kind: "visible_actor",
            subject_id: current_place_id.as_str().to_string(),
            target_id: local_actor_id.as_str().to_string(),
            place_id: current_place_id.clone(),
            servings: None,
            payload: Vec::new(),
        });
    }

    for food in state.food_supplies().values().filter(|food| {
        matches!(&food.location, Location::AtPlace(place_id) if place_id == &current_place_id)
    }) {
        observations.push(PerceivedThing {
            kind: "visible_food_supply",
            subject_id: current_place_id.as_str().to_string(),
            target_id: food.food_supply_id.as_str().to_string(),
            place_id: current_place_id.clone(),
            servings: Some(food.servings),
            payload: Vec::new(),
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
            servings: None,
            payload: Vec::new(),
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

pub fn current_place_knowledge_context(
    state: &PhysicalState,
    epistemic_projection: Option<&EpistemicProjection>,
    actor_id: &ActorId,
    decision_tick: SimTick,
    _content_manifest_id: &ContentManifestId,
    event_frontier: u64,
) -> KnowledgeContext {
    let mut food_sources = Vec::new();
    let mut current_places = Vec::new();
    let mut carried_items = Vec::new();
    let mut sleep_affordances = Vec::new();
    let mut routes = Vec::new();
    let mut workplaces = Vec::new();
    let mut doors = Vec::new();
    let mut containers = Vec::new();
    let mut items = Vec::new();
    let mut local_actors = Vec::new();
    let mut provenance = Vec::new();

    let Some(actor) = state.actors().get(actor_id) else {
        return KnowledgeContext::embodied_at_frontier_with_all_facts(
            actor_id.clone(),
            decision_tick,
            event_frontier,
            Vec::new(),
            food_sources,
            sleep_affordances,
            routes,
            doors,
            containers,
            items,
            local_actors,
        );
    };
    let Some(epistemic_projection) = epistemic_projection else {
        return KnowledgeContext::embodied_at_frontier_with_all_facts(
            actor_id.clone(),
            decision_tick,
            event_frontier,
            Vec::new(),
            food_sources,
            sleep_affordances,
            routes,
            doors,
            containers,
            items,
            local_actors,
        );
    };
    let filter_context =
        KnowledgeContext::embodied_at_frontier(actor_id.clone(), decision_tick, event_frontier);
    for classified in epistemic_projection
        .classified_actor_known_records_for_context(&filter_context, &actor.current_place_id)
    {
        let source_key = format!(
            "epistemic_projection:{}",
            classified.record().source_event_id().as_str()
        );
        let source_event_key = classified.record().source_event_id().as_str().to_string();
        let included_by_policy = classified.record().policy().includes_in_embodied_context(
            classified.is_current_place_record(),
            classified.is_latest_current_place_record(),
        );
        match classified.record() {
            ActorKnownProjectionRecord::Route {
                from_place_id,
                to_place_id,
                ..
            } => {
                if !included_by_policy {
                    continue;
                }
                provenance.push(provenance_for_projection_record(
                    classified.record(),
                    source_event_key,
                ));
                routes.push(ActorKnownRouteFact::new(
                    from_place_id.clone(),
                    to_place_id.clone(),
                    source_key,
                ));
            }
            ActorKnownProjectionRecord::CurrentPlace {
                place_id,
                display_label,
                ..
            } => {
                if included_by_policy {
                    provenance.push(provenance_for_projection_record(
                        classified.record(),
                        source_event_key,
                    ));
                    current_places.push(ActorKnownCurrentPlaceFact::new(
                        place_id.clone(),
                        display_label.clone(),
                        source_key,
                    ));
                }
            }
            ActorKnownProjectionRecord::CarriedItem {
                item_id,
                source_location,
                portable,
                ..
            } => {
                if included_by_policy {
                    provenance.push(provenance_for_projection_record(
                        classified.record(),
                        source_event_key,
                    ));
                    carried_items.push(ActorKnownCarriedItemFact::new(
                        item_id.clone(),
                        source_location.clone(),
                        *portable,
                        source_key,
                    ));
                }
            }
            ActorKnownProjectionRecord::FoodSource {
                food_source_id,
                believed_servings,
                ..
            } => {
                if !included_by_policy {
                    continue;
                }
                let Some(food_supply_id) = FoodSupplyId::new(food_source_id).ok() else {
                    continue;
                };
                provenance.push(provenance_for_projection_record(
                    classified.record(),
                    source_event_key,
                ));
                food_sources.push(ActorKnownFoodSourceFact::with_believed_servings(
                    food_supply_id,
                    *believed_servings,
                    source_key,
                ));
            }
            ActorKnownProjectionRecord::SleepPlace {
                place_id,
                sleep_affordance_id,
                ..
            } => {
                if !included_by_policy {
                    continue;
                }
                let Some(sleep_affordance_id) = sleep_affordance_id
                    .as_deref()
                    .and_then(|value| SleepAffordanceId::new(value).ok())
                else {
                    continue;
                };
                provenance.push(provenance_for_projection_record(
                    classified.record(),
                    source_event_key,
                ));
                sleep_affordances.push(ActorKnownSleepAffordanceFact::new(
                    sleep_affordance_id,
                    place_id.clone(),
                    source_key,
                ));
            }
            ActorKnownProjectionRecord::Workplace {
                workplace_id,
                place_id,
                believed_access_open,
                ..
            } => {
                if included_by_policy {
                    provenance.push(provenance_for_projection_record(
                        classified.record(),
                        source_event_key,
                    ));
                    workplaces.push(ActorKnownWorkplaceFact::new(
                        workplace_id.clone(),
                        place_id.clone(),
                        *believed_access_open,
                        source_key,
                        crate::agent::SourceEventIds::checked(vec![classified
                            .record()
                            .source_event_id()
                            .clone()])
                        .expect("projection workplace records carry a source event"),
                        classified.source_tick(),
                    ));
                }
            }
            ActorKnownProjectionRecord::LocalDoor {
                door_id,
                endpoint_a,
                endpoint_b,
                is_open,
                is_locked,
                blocks_movement_when_closed,
                ..
            } => {
                if included_by_policy {
                    provenance.push(provenance_for_projection_record(
                        classified.record(),
                        source_event_key,
                    ));
                    doors.push(ActorKnownDoorFact::new(
                        door_id.clone(),
                        endpoint_a.clone(),
                        endpoint_b.clone(),
                        *is_open,
                        *is_locked,
                        *blocks_movement_when_closed,
                        source_key,
                    ));
                }
            }
            ActorKnownProjectionRecord::LocalContainer {
                container_id,
                is_open,
                is_locked,
                ..
            } => {
                if included_by_policy {
                    provenance.push(provenance_for_projection_record(
                        classified.record(),
                        source_event_key,
                    ));
                    containers.push(ActorKnownContainerFact::new(
                        container_id.clone(),
                        *is_open,
                        *is_locked,
                        source_key,
                    ));
                }
            }
            ActorKnownProjectionRecord::LocalItem {
                item_id,
                source_location,
                portable,
                ..
            } => {
                if included_by_policy {
                    provenance.push(provenance_for_projection_record(
                        classified.record(),
                        source_event_key,
                    ));
                    items.push(ActorKnownItemFact::new(
                        item_id.clone(),
                        source_location.clone(),
                        *portable,
                        source_key,
                    ));
                }
            }
            ActorKnownProjectionRecord::LocalActor {
                observed_actor_id, ..
            } => {
                if included_by_policy {
                    provenance.push(provenance_for_projection_record(
                        classified.record(),
                        source_event_key,
                    ));
                    local_actors.push(ActorKnownLocalActorFact::new(
                        observed_actor_id.clone(),
                        source_key,
                    ));
                }
            }
        }
    }

    KnowledgeContext::embodied_at_frontier_with_all_facts_observations_and_provenance(
        actor_id.clone(),
        decision_tick,
        event_frontier,
        workplaces,
        current_places,
        carried_items,
        food_sources,
        sleep_affordances,
        routes,
        doors,
        containers,
        items,
        local_actors,
        provenance,
    )
}

fn provenance_for_projection_record(
    record: &ActorKnownProjectionRecord,
    source_event_key: String,
) -> KnowledgeProvenanceEntry {
    match record {
        ActorKnownProjectionRecord::Workplace { .. } => {
            KnowledgeProvenanceEntry::actor_known_source(
                KnowledgeProvenanceKind::Record,
                AllowedKnowledgeSource::OwnSourceBackedBelief,
                source_event_key,
            )
        }
        ActorKnownProjectionRecord::LocalActor { .. } => {
            KnowledgeProvenanceEntry::actor_known_source(
                KnowledgeProvenanceKind::Observation,
                AllowedKnowledgeSource::OwnDirectObservation,
                source_event_key,
            )
        }
        ActorKnownProjectionRecord::CurrentPlace { .. }
        | ActorKnownProjectionRecord::CarriedItem { .. }
        | ActorKnownProjectionRecord::Route { .. }
        | ActorKnownProjectionRecord::FoodSource { .. }
        | ActorKnownProjectionRecord::SleepPlace { .. }
        | ActorKnownProjectionRecord::LocalDoor { .. }
        | ActorKnownProjectionRecord::LocalContainer { .. }
        | ActorKnownProjectionRecord::LocalItem { .. } => {
            KnowledgeProvenanceEntry::actor_known_source(
                KnowledgeProvenanceKind::Perception,
                AllowedKnowledgeSource::OwnDirectObservation,
                source_event_key,
            )
        }
    }
}

fn project_perception_event(projection: &mut EpistemicProjection, event: &EventEnvelope) {
    let Some(actor_id) = event.actor_id.clone() else {
        return;
    };
    let Some(place_id) = event.place_id.clone() else {
        return;
    };
    let Some(observation_id) =
        payload_value(event, "observation_id").and_then(|value| ObservationId::new(value).ok())
    else {
        return;
    };
    let Some(observed_tick) = payload_value(event, "observed_tick")
        .and_then(|value| value.parse::<u64>().ok())
        .map(SimTick::new)
    else {
        return;
    };
    let observation = Observation::new(
        observation_id,
        actor_id,
        Channel::DirectSight,
        observed_tick,
        place_id.clone(),
        ObservationSubject::Place(place_id.clone()),
        ObservationTarget::Place(place_id),
        Confidence::MAX,
        SourceRef::Event(event.event_id.clone()),
    )
    .with_raw_payload(event.payload.clone());
    projection.insert_observation(observation);
    projection.record_applied_event(event.event_id.clone());
}

fn is_visible_exit_target(state: &PhysicalState, place_id: &PlaceId) -> bool {
    let Some(place) = state.places().get(place_id) else {
        return false;
    };
    place.visibility_default.is_visible()
}

fn payload_value<'a>(event: &'a EventEnvelope, key: &str) -> Option<&'a str> {
    event
        .payload
        .iter()
        .find(|field| field.key == key)
        .map(|field| field.value.as_str())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PerceivedThing {
    kind: &'static str,
    subject_id: String,
    target_id: String,
    place_id: PlaceId,
    servings: Option<u32>,
    payload: Vec<PayloadField>,
}

fn visible_item_payload(
    state: &PhysicalState,
    current_place_id: &PlaceId,
    location: &Location,
) -> Option<Vec<PayloadField>> {
    match location {
        Location::AtPlace(place_id) if place_id == current_place_id => Some(vec![
            PayloadField::new("item_source_kind", "place"),
            PayloadField::new("item_source_place_id", place_id.as_str()),
        ]),
        Location::InContainer(container_id) => {
            let container = state.containers().get(container_id)?;
            if !matches!(&container.location, Location::AtPlace(place_id) if place_id == current_place_id)
            {
                return None;
            }
            if !container.is_open && !container.contents_visible_when_closed {
                return None;
            }
            Some(vec![
                PayloadField::new("item_source_kind", "container"),
                PayloadField::new("item_source_container_id", container_id.as_str()),
            ])
        }
        Location::CarriedBy(_) | Location::AtPlace(_) => None,
    }
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
    let process_id = ProcessId::new("current_place_perception").unwrap();
    let mut event = EventEnvelope::new_caused_v1(
        event_id.clone(),
        EventKind::ObservationRecorded,
        0,
        0,
        decision_tick,
        ordering_key,
        content_manifest_id.clone(),
        vec![EventCause::Process(process_id.clone())],
    )
    .expect("perception observations carry process ancestry");
    event.actor_id = Some(actor_id.clone());
    event.place_id = Some(perceived.place_id.clone());
    event.process_id = Some(process_id);
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
    if let Some(servings) = perceived.servings {
        event
            .payload
            .push(PayloadField::new("servings", servings.to_string()));
    }
    event.payload.extend(perceived.payload);
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
    use crate::agent::{NoHumanActorKnownSurfaceBuilder, NoHumanActorKnownSurfaceRequest};
    use crate::epistemics::{ActorKnownProjectionFreshness, EpistemicProjection};
    use crate::ids::{ContainerId, FoodSupplyId, ItemId, SleepAffordanceId, WorkplaceId};
    use crate::state::AgentState;
    use crate::state::{
        ActorBody, ContainerState, FoodSupplyState, ItemState, PlaceState, SleepAffordanceState,
        VisibilityDefault, WorkplaceState,
    };

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
            SleepAffordanceState::new(sleep_affordance_id, home, 4, 20, 2),
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
            crate::state::NeedModelState::new(5, 3),
        )
    }

    fn state_with_current_place_workplace() -> PhysicalState {
        let mut state = state_with_visible_current_place_surfaces();
        let workplace_id = WorkplaceId::new("workplace_tomas").unwrap();
        let mut workplace = WorkplaceState::new(
            workplace_id.clone(),
            place_id("home_tomas"),
            4,
            8,
            4,
            900,
            900,
            "repair_output",
        );
        workplace.assigned_actor_ids.insert(actor_id("actor_tomas"));
        state.workplaces.insert(workplace_id, workplace);
        state
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
            crate::state::NeedModelState::new(5, 3),
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
                "current_place",
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

    fn item_id(value: &str) -> ItemId {
        ItemId::new(value).unwrap()
    }

    fn container_id(value: &str) -> ContainerId {
        ContainerId::new(value).unwrap()
    }

    fn visible_item_events(state: &PhysicalState, actor: &ActorId) -> Vec<EventEnvelope> {
        current_place_perception_events(state, actor, SimTick::new(4), &manifest_id())
            .into_iter()
            .filter(|event| payload_value(event, "perceived_kind") == Some("visible_item"))
            .collect()
    }

    #[test]
    fn visible_item_perception_follows_current_place_match_not_id_prose() {
        let actor = actor_id("actor_tomas");
        let home = place_id("home_tomas");
        let elsewhere = place_id("workshop_tomas");

        let mut actors = BTreeMap::new();
        actors.insert(actor.clone(), ActorBody::new(actor.clone(), home.clone()));
        let mut places = BTreeMap::new();
        places.insert(home.clone(), PlaceState::new(home.clone(), "Tomas home"));
        places.insert(
            elsewhere.clone(),
            PlaceState::new(elsewhere.clone(), "Workshop"),
        );

        let here = item_id("item_here");
        let there = item_id("item_elsewhere");
        let mut items = BTreeMap::new();
        items.insert(
            here.clone(),
            ItemState::new(here.clone(), Location::AtPlace(home.clone())),
        );
        items.insert(
            there.clone(),
            ItemState::new(there.clone(), Location::AtPlace(elsewhere)),
        );

        let state = PhysicalState::from_seed_parts(
            actors,
            places,
            BTreeMap::new(),
            BTreeMap::new(),
            items,
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
            crate::state::NeedModelState::new(5, 3),
        );

        let events = visible_item_events(&state, &actor);

        // The item co-located with the actor is perceived and keyed as a place source.
        let here_event = events
            .iter()
            .find(|event| payload_value(event, "target_id") == Some(here.as_str()))
            .expect("item at the actor's current place is perceived");
        assert_eq!(payload_value(here_event, "item_source_kind"), Some("place"));
        assert_eq!(
            payload_value(here_event, "item_source_place_id"),
            Some(home.as_str())
        );
        // An item at a different place is not perceived from here.
        assert!(events
            .iter()
            .all(|event| payload_value(event, "target_id") != Some(there.as_str())));
    }

    #[test]
    fn visible_item_perception_reveals_open_container_without_closed_visibility() {
        let actor = actor_id("actor_tomas");
        let home = place_id("home_tomas");

        let mut actors = BTreeMap::new();
        actors.insert(actor.clone(), ActorBody::new(actor.clone(), home.clone()));
        let mut places = BTreeMap::new();
        places.insert(home.clone(), PlaceState::new(home.clone(), "Tomas home"));

        let chest_id = container_id("chest_tomas");
        let stored = item_id("item_in_chest");
        let mut chest = ContainerState::fixed_at_place(chest_id.clone(), home);
        // Open, but contents are *not* visible while closed: visibility must hinge
        // on the container being open, not on both conditions holding.
        chest.is_open = true;
        chest.contents_visible_when_closed = false;
        chest.contents.insert(stored.clone());
        let mut containers = BTreeMap::new();
        containers.insert(chest_id.clone(), chest);

        let mut items = BTreeMap::new();
        items.insert(
            stored.clone(),
            ItemState::new(stored.clone(), Location::InContainer(chest_id.clone())),
        );

        let state = PhysicalState::from_seed_parts(
            actors,
            places,
            BTreeMap::new(),
            containers,
            items,
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
            crate::state::NeedModelState::new(5, 3),
        );

        let events = visible_item_events(&state, &actor);

        let stored_event = events
            .iter()
            .find(|event| payload_value(event, "target_id") == Some(stored.as_str()))
            .expect("item in an open container at the current place is perceived");
        assert_eq!(
            payload_value(stored_event, "item_source_kind"),
            Some("container")
        );
        assert_eq!(
            payload_value(stored_event, "item_source_container_id"),
            Some(chest_id.as_str())
        );
    }

    #[test]
    fn visible_exit_perception_follows_typed_visibility_not_id_or_label_prose() {
        let actor = actor_id("actor_tomas");
        let home = place_id("home_tomas");
        let misleading_visible = place_id("hidden_annex");
        let concealed_plain = place_id("quiet_annex");

        let mut home_state = PlaceState::new(home.clone(), "Tomas home");
        home_state
            .adjacent_place_ids
            .insert(misleading_visible.clone());
        home_state
            .adjacent_place_ids
            .insert(concealed_plain.clone());
        let mut visible_state = PlaceState::new(misleading_visible.clone(), "Hidden annex");
        visible_state.visibility_default = VisibilityDefault::Visible;
        let mut concealed_state = PlaceState::new(concealed_plain.clone(), "Quiet annex");
        concealed_state.visibility_default = VisibilityDefault::Concealed;

        let mut actors = BTreeMap::new();
        actors.insert(actor.clone(), ActorBody::new(actor.clone(), home.clone()));
        let mut places = BTreeMap::new();
        places.insert(home, home_state);
        places.insert(misleading_visible.clone(), visible_state);
        places.insert(concealed_plain.clone(), concealed_state);
        let state = PhysicalState::from_seed_parts(
            actors,
            places,
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
            crate::state::NeedModelState::new(5, 3),
        );

        let visible_exit_targets =
            current_place_perception_events(&state, &actor, SimTick::new(4), &manifest_id())
                .into_iter()
                .filter(|event| {
                    event
                        .payload
                        .iter()
                        .any(|field| field.key == "perceived_kind" && field.value == "visible_exit")
                })
                .filter_map(|event| {
                    event
                        .payload
                        .iter()
                        .find(|field| field.key == "target_id")
                        .map(|field| field.value.clone())
                })
                .collect::<std::collections::BTreeSet<_>>();

        assert!(visible_exit_targets.contains(misleading_visible.as_str()));
        assert!(!visible_exit_targets.contains(concealed_plain.as_str()));
        assert!(is_visible_exit_target(&state, &misleading_visible));
        assert!(!is_visible_exit_target(&state, &concealed_plain));
        assert!(!is_visible_exit_target(
            &state,
            &place_id("missing_visible_named_place")
        ));
    }

    #[test]
    fn current_place_perception_emits_only_current_place_without_visible_surfaces() {
        let events = current_place_perception_events(
            &state_without_visible_surfaces(),
            &actor_id("actor_tomas"),
            SimTick::new(4),
            &manifest_id(),
        );

        assert_eq!(events.len(), 1);
        assert_eq!(
            payload_value(&events[0], "perceived_kind"),
            Some("current_place")
        );
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

    #[test]
    fn current_place_knowledge_context_uses_latest_projection_window_not_live_truth() {
        let actor = actor_id("actor_tomas");
        let mut state = state_with_visible_current_place_surfaces();
        let manifest_id = manifest_id();
        let mut projection = EpistemicProjection::new(manifest_id.clone());

        for event in current_place_perception_events(&state, &actor, SimTick::new(4), &manifest_id)
        {
            project_perception_event(&mut projection, &event);
        }

        let stale_context = current_place_knowledge_context(
            &state,
            Some(&projection),
            &actor,
            SimTick::new(5),
            &manifest_id,
            3,
        );
        assert!(stale_context
            .actor_known_food_sources()
            .iter()
            .any(|fact| fact.food_supply_id().as_str() == "food_breakfast_tomas"));

        state.food_supplies.clear();
        let replacement_food_id = FoodSupplyId::new("food_lunch_tomas").unwrap();
        state.food_supplies.insert(
            replacement_food_id.clone(),
            FoodSupplyState::new(
                replacement_food_id,
                Location::AtPlace(place_id("home_tomas")),
                1,
                180,
            ),
        );

        let stale_after_truth_change = current_place_knowledge_context(
            &state,
            Some(&projection),
            &actor,
            SimTick::new(6),
            &manifest_id,
            3,
        );
        assert!(stale_after_truth_change
            .actor_known_food_sources()
            .iter()
            .any(|fact| fact.food_supply_id().as_str() == "food_breakfast_tomas"));
        assert!(!stale_after_truth_change
            .actor_known_food_sources()
            .iter()
            .any(|fact| fact.food_supply_id().as_str() == "food_lunch_tomas"));

        for event in current_place_perception_events(&state, &actor, SimTick::new(7), &manifest_id)
        {
            project_perception_event(&mut projection, &event);
        }

        let refreshed_context = current_place_knowledge_context(
            &state,
            Some(&projection),
            &actor,
            SimTick::new(8),
            &manifest_id,
            6,
        );
        assert!(!refreshed_context
            .actor_known_food_sources()
            .iter()
            .any(|fact| fact.food_supply_id().as_str() == "food_breakfast_tomas"));
        assert!(refreshed_context
            .actor_known_food_sources()
            .iter()
            .any(|fact| fact.food_supply_id().as_str() == "food_lunch_tomas"));
    }

    #[test]
    fn current_place_knowledge_context_uses_latest_projection_window_for_sleep_affordances() {
        let actor = actor_id("actor_tomas");
        let mut state = state_with_visible_current_place_surfaces();
        let manifest_id = manifest_id();
        let mut projection = EpistemicProjection::new(manifest_id.clone());

        for event in current_place_perception_events(&state, &actor, SimTick::new(4), &manifest_id)
        {
            project_perception_event(&mut projection, &event);
        }

        let stale_context = current_place_knowledge_context(
            &state,
            Some(&projection),
            &actor,
            SimTick::new(5),
            &manifest_id,
            3,
        );
        assert!(stale_context
            .actor_known_sleep_affordances()
            .iter()
            .any(|fact| fact.sleep_affordance_id().as_str() == "bed_tomas"));

        state.sleep_affordances.clear();
        let replacement_sleep_id = SleepAffordanceId::new("cot_tomas").unwrap();
        state.sleep_affordances.insert(
            replacement_sleep_id.clone(),
            SleepAffordanceState::new(replacement_sleep_id, place_id("home_tomas"), 4, 20, 2),
        );

        let stale_after_truth_change = current_place_knowledge_context(
            &state,
            Some(&projection),
            &actor,
            SimTick::new(6),
            &manifest_id,
            3,
        );
        assert!(stale_after_truth_change
            .actor_known_sleep_affordances()
            .iter()
            .any(|fact| fact.sleep_affordance_id().as_str() == "bed_tomas"));
        assert!(!stale_after_truth_change
            .actor_known_sleep_affordances()
            .iter()
            .any(|fact| fact.sleep_affordance_id().as_str() == "cot_tomas"));

        for event in current_place_perception_events(&state, &actor, SimTick::new(7), &manifest_id)
        {
            project_perception_event(&mut projection, &event);
        }

        let refreshed_context = current_place_knowledge_context(
            &state,
            Some(&projection),
            &actor,
            SimTick::new(8),
            &manifest_id,
            6,
        );
        assert!(!refreshed_context
            .actor_known_sleep_affordances()
            .iter()
            .any(|fact| fact.sleep_affordance_id().as_str() == "bed_tomas"));
        assert!(refreshed_context
            .actor_known_sleep_affordances()
            .iter()
            .any(|fact| fact.sleep_affordance_id().as_str() == "cot_tomas"));
    }

    #[test]
    fn embodied_context_filters_reclassified_records_by_declared_scope_but_no_human_keeps_memory() {
        let actor = actor_id("actor_tomas");
        let mut state = state_with_visible_current_place_surfaces();
        let manifest_id = manifest_id();
        let mut projection = EpistemicProjection::new(manifest_id.clone());

        for event in current_place_perception_events(&state, &actor, SimTick::new(4), &manifest_id)
        {
            project_perception_event(&mut projection, &event);
        }

        let workshop = place_id("workshop_tomas");
        state
            .actors
            .get_mut(&actor)
            .expect("test actor exists")
            .current_place_id = workshop.clone();

        let filter_context =
            KnowledgeContext::embodied_at_frontier(actor.clone(), SimTick::new(9), 4);
        let classified =
            projection.classified_actor_known_records_for_context(&filter_context, &workshop);
        let sleep_record = classified
            .iter()
            .find(|record| {
                matches!(
                    record.record(),
                    ActorKnownProjectionRecord::SleepPlace { sleep_affordance_id, .. }
                        if sleep_affordance_id.as_deref() == Some("bed_tomas")
                )
            })
            .expect("shared classifier keeps remembered sleep place");
        assert!(
            !sleep_record.record().policy().includes_in_embodied_context(
                sleep_record.is_current_place_record(),
                sleep_record.is_latest_current_place_record(),
            )
        );

        let agent_state = AgentState::default();
        let no_human_surface =
            NoHumanActorKnownSurfaceBuilder::from_projection(NoHumanActorKnownSurfaceRequest {
                projection: &projection,
                agent_state: &agent_state,
                actor_id: actor.clone(),
                current_place_id: workshop.clone(),
                decision_tick: SimTick::new(9),
                window_id: "morning",
                window_end_tick: SimTick::new(12),
                current_place_witness_event_id: None,
                needs_witness_event_id: None,
                frame_event_id: Some(EventId::new("event_frame").unwrap()),
            })
            .build(&agent_state);
        assert!(no_human_surface
            .context()
            .known_sleep_places()
            .contains(&place_id("home_tomas")));

        let embodied_context = current_place_knowledge_context(
            &state,
            Some(&projection),
            &actor,
            SimTick::new(9),
            &manifest_id,
            4,
        );
        assert!(embodied_context.actor_known_sleep_affordances().is_empty());
    }

    #[test]
    fn current_place_knowledge_context_applies_freshness_to_workplace_notices() {
        let actor = actor_id("actor_tomas");
        let state = state_with_current_place_workplace();
        let manifest_id = manifest_id();
        let mut projection = EpistemicProjection::new(manifest_id.clone());

        for event in current_place_perception_events(&state, &actor, SimTick::new(4), &manifest_id)
        {
            project_perception_event(&mut projection, &event);
        }
        projection.insert_role_assignment_notice(
            actor.clone(),
            WorkplaceId::new("workplace_tomas").unwrap(),
            place_id("home_tomas"),
            false,
            EventId::new("event_role_notice_old_closed").unwrap(),
            SimTick::new(4),
        );

        for event in current_place_perception_events(&state, &actor, SimTick::new(7), &manifest_id)
        {
            project_perception_event(&mut projection, &event);
        }
        projection.insert_role_assignment_notice(
            actor.clone(),
            WorkplaceId::new("workplace_tomas").unwrap(),
            place_id("home_tomas"),
            true,
            EventId::new("event_role_notice_new_open").unwrap(),
            SimTick::new(7),
        );

        let context = current_place_knowledge_context(
            &state,
            Some(&projection),
            &actor,
            SimTick::new(8),
            &manifest_id,
            8,
        );

        assert_eq!(context.actor_known_workplaces().len(), 1);
        let workplace = &context.actor_known_workplaces()[0];
        assert!(workplace.believed_access_open());
        assert_eq!(workplace.acquired_tick(), SimTick::new(7));
        assert_eq!(
            workplace.source_event_ids().as_slice(),
            &[EventId::new("event_role_notice_new_open").unwrap()]
        );
    }

    #[test]
    fn current_place_knowledge_context_keeps_remembered_workplace_notice_after_newer_perception() {
        let actor = actor_id("actor_tomas");
        let state = state_with_current_place_workplace();
        let manifest_id = manifest_id();
        let mut projection = EpistemicProjection::new(manifest_id.clone());

        for event in current_place_perception_events(&state, &actor, SimTick::new(4), &manifest_id)
        {
            project_perception_event(&mut projection, &event);
        }
        projection.insert_role_assignment_notice(
            actor.clone(),
            WorkplaceId::new("workplace_tomas").unwrap(),
            place_id("home_tomas"),
            true,
            EventId::new("event_role_notice_remembered_open").unwrap(),
            SimTick::new(4),
        );

        for event in current_place_perception_events(&state, &actor, SimTick::new(9), &manifest_id)
        {
            project_perception_event(&mut projection, &event);
        }

        let filter_context =
            KnowledgeContext::embodied_at_frontier(actor.clone(), SimTick::new(10), 8);
        let classified = projection
            .classified_actor_known_records_for_context(&filter_context, &place_id("home_tomas"));
        assert!(classified.iter().any(|record| {
            matches!(
                record.record(),
                ActorKnownProjectionRecord::Workplace { workplace_id, .. }
                    if workplace_id.as_str() == "workplace_tomas"
            ) && record.freshness() == ActorKnownProjectionFreshness::Remembered
                && !record.is_latest_current_place_record()
        }));

        let context = current_place_knowledge_context(
            &state,
            Some(&projection),
            &actor,
            SimTick::new(10),
            &manifest_id,
            8,
        );

        assert_eq!(context.actor_known_workplaces().len(), 1);
        let workplace = &context.actor_known_workplaces()[0];
        assert_eq!(workplace.workplace_id().as_str(), "workplace_tomas");
        assert_eq!(workplace.place_id().as_str(), "home_tomas");
        assert!(workplace.believed_access_open());
        assert_eq!(workplace.acquired_tick(), SimTick::new(4));
        assert_eq!(
            workplace.source_event_ids().as_slice(),
            &[EventId::new("event_role_notice_remembered_open").unwrap()]
        );
    }

    #[test]
    fn workplace_same_tick_tie_breaks_on_source_event_id_for_both_surfaces() {
        let actor = actor_id("actor_tomas");
        let state = state_with_current_place_workplace();
        let manifest_id = manifest_id();
        let mut projection = EpistemicProjection::new(manifest_id.clone());
        projection.insert_role_assignment_notice(
            actor.clone(),
            WorkplaceId::new("workplace_tomas").unwrap(),
            place_id("home_tomas"),
            false,
            EventId::new("event_role_notice_same_tick_closed").unwrap(),
            SimTick::new(8),
        );
        projection.insert_role_assignment_notice(
            actor.clone(),
            WorkplaceId::new("workplace_tomas").unwrap(),
            place_id("home_tomas"),
            true,
            EventId::new("event_role_notice_same_tick_open").unwrap(),
            SimTick::new(8),
        );

        let embodied = current_place_knowledge_context(
            &state,
            Some(&projection),
            &actor,
            SimTick::new(9),
            &manifest_id,
            8,
        );
        let agent_state = AgentState::default();
        let no_human =
            NoHumanActorKnownSurfaceBuilder::from_projection(NoHumanActorKnownSurfaceRequest {
                projection: &projection,
                agent_state: &agent_state,
                actor_id: actor,
                current_place_id: place_id("home_tomas"),
                decision_tick: SimTick::new(9),
                window_id: "morning",
                window_end_tick: SimTick::new(12),
                current_place_witness_event_id: None,
                needs_witness_event_id: None,
                frame_event_id: Some(EventId::new("event_role_notice_same_tick_open").unwrap()),
            })
            .build(&agent_state);

        let embodied_workplace = &embodied.actor_known_workplaces()[0];
        let no_human_access_fact = no_human
            .context()
            .actor_known_facts()
            .iter()
            .find(|fact| fact.stable_id() == "workplace_believed_accessible")
            .expect("no-human workplace access fact should be present");

        assert_eq!(embodied.actor_known_workplaces().len(), 1);
        assert!(embodied_workplace.believed_access_open());
        assert_eq!(
            embodied_workplace.source_event_ids().as_slice(),
            &[EventId::new("event_role_notice_same_tick_open").unwrap()]
        );
        assert_eq!(no_human_access_fact.value(), "workplace_tomas:true");
        assert_eq!(
            no_human_access_fact.source_event_ids(),
            &[EventId::new("event_role_notice_same_tick_open").unwrap()]
        );
    }
}
