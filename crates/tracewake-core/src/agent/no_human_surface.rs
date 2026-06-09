use std::collections::{BTreeMap, BTreeSet};

use crate::agent::{ActorKnownFact, ActorKnownPlanningContext};
use crate::events::log::EventLog;
use crate::events::{EventEnvelope, EventKind};
use crate::ids::{ActorId, EventId, PlaceId, WorkplaceId};
use crate::state::AgentState;
use crate::time::SimTick;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SealedActorKnownSurface {
    context: ActorKnownPlanningContext,
}

impl SealedActorKnownSurface {
    fn new(context: ActorKnownPlanningContext) -> Self {
        Self { context }
    }

    pub fn context(&self) -> &ActorKnownPlanningContext {
        &self.context
    }

    pub fn into_context(self) -> ActorKnownPlanningContext {
        self.context
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NoHumanActorKnownSurfaceBuilder {
    actor_id: ActorId,
    current_place_id: PlaceId,
    decision_tick: Option<SimTick>,
    known_edges: BTreeMap<PlaceId, BTreeSet<PlaceId>>,
    known_food_sources: BTreeSet<String>,
    known_sleep_places: BTreeSet<PlaceId>,
    known_workplaces: BTreeMap<WorkplaceId, PlaceId>,
    facts: Vec<ActorKnownFact>,
}

impl NoHumanActorKnownSurfaceBuilder {
    pub fn new(
        actor_id: ActorId,
        current_place_id: PlaceId,
        decision_tick: Option<SimTick>,
    ) -> Self {
        Self {
            actor_id,
            current_place_id,
            decision_tick,
            known_edges: BTreeMap::new(),
            known_food_sources: BTreeSet::new(),
            known_sleep_places: BTreeSet::new(),
            known_workplaces: BTreeMap::new(),
            facts: Vec::new(),
        }
    }

    pub fn from_event_log(
        log: &EventLog,
        agent_state: &AgentState,
        actor_id: ActorId,
        current_place_id: PlaceId,
        decision_tick: SimTick,
        window_id: &str,
        window_end_tick: SimTick,
    ) -> Self {
        let frame_event_id = latest_frame_event_id(log);
        let mut builder = Self::new(actor_id, current_place_id, Some(decision_tick));
        builder.consume_events(log);
        builder.add_window_framing_facts(agent_state, window_id, window_end_tick, frame_event_id);
        builder
    }

    pub fn with_role_assignment_notice(
        mut self,
        workplace_id: WorkplaceId,
        place_id: PlaceId,
        source: impl Into<String>,
    ) -> Self {
        self.add_role_assignment_notice(workplace_id, place_id, source, Vec::new());
        self
    }

    pub fn with_sleep_place_knowledge(
        mut self,
        sleep_place_id: PlaceId,
        source: impl Into<String>,
    ) -> Self {
        let source = source.into();
        self.known_sleep_places.insert(sleep_place_id.clone());
        self.push_fact(
            ActorKnownFact::remembered_belief(
                self.actor_id.clone(),
                "actor_knows_sleep_place",
                sleep_place_id.as_str(),
                format!("sleep_place_knowledge:{source}"),
                self.decision_tick,
            ),
            Vec::new(),
        );
        if sleep_place_id == self.current_place_id {
            self.push_fact(
                ActorKnownFact::observed_now(
                    self.actor_id.clone(),
                    "sleep_place_believed_accessible",
                    sleep_place_id.as_str(),
                    "modeled_observation:sleep_place_accessible",
                    self.decision_tick,
                ),
                Vec::new(),
            );
        }
        self
    }

    pub fn build(self, _agent_state: &AgentState) -> SealedActorKnownSurface {
        let context = ActorKnownPlanningContext::from_observed_parts(
            self.actor_id,
            self.current_place_id,
            self.known_edges,
            BTreeMap::new(),
            BTreeMap::new(),
            self.known_food_sources,
            self.known_sleep_places,
            self.known_workplaces,
            self.facts,
        );
        SealedActorKnownSurface::new(context)
    }

    fn consume_events(&mut self, log: &EventLog) {
        for event in log.events() {
            match event.event_type {
                EventKind::RoleAssignmentNoticeRecorded => {
                    self.consume_role_assignment_notice(event);
                }
                EventKind::StartingBeliefRecorded => {
                    self.consume_starting_belief(event);
                }
                EventKind::ObservationRecorded => {
                    self.consume_observation(event);
                }
                _ => {}
            }
        }
    }

    fn consume_role_assignment_notice(&mut self, event: &EventEnvelope) {
        if payload_value(event, "actor_id") != Some(self.actor_id.as_str()) {
            return;
        }
        let Some(workplace_id) =
            payload_value(event, "workplace_id").and_then(|value| WorkplaceId::new(value).ok())
        else {
            return;
        };
        let Some(place_id) =
            payload_value(event, "place_id").and_then(|value| PlaceId::new(value).ok())
        else {
            return;
        };
        self.add_role_assignment_notice(
            workplace_id,
            place_id,
            "evented_role_assignment_notice",
            vec![event.event_id.clone()],
        );
    }

    fn consume_starting_belief(&mut self, event: &EventEnvelope) {
        if payload_value(event, "actor_id") != Some(self.actor_id.as_str()) {
            return;
        }
        match payload_value(event, "belief_kind") {
            Some("sleep_place") => {
                let Some(place_id) =
                    payload_value(event, "value").and_then(|value| PlaceId::new(value).ok())
                else {
                    return;
                };
                self.add_sleep_place_knowledge(
                    place_id,
                    payload_value(event, "subject_id"),
                    "evented_starting_belief",
                    vec![event.event_id.clone()],
                );
            }
            Some("household_food_source") => {
                let current_place_value = format!("place:{}", self.current_place_id.as_str());
                if payload_value(event, "value") != Some(current_place_value.as_str()) {
                    return;
                }
                let Some(food_source) = payload_value(event, "subject_id") else {
                    return;
                };
                self.add_food_source_knowledge(
                    food_source,
                    "evented_starting_belief",
                    vec![event.event_id.clone()],
                );
            }
            _ => {}
        }
    }

    fn consume_observation(&mut self, event: &EventEnvelope) {
        if event.actor_id.as_ref() != Some(&self.actor_id) {
            return;
        }
        match payload_value(event, "perceived_kind") {
            Some("visible_exit") => {
                let Some(target_id) =
                    payload_value(event, "target_id").and_then(|value| PlaceId::new(value).ok())
                else {
                    return;
                };
                self.known_edges
                    .entry(self.current_place_id.clone())
                    .or_default()
                    .insert(target_id.clone());
                self.push_fact(
                    ActorKnownFact::observed_now(
                        self.actor_id.clone(),
                        "known_route_surface",
                        format!("{}->{}", self.current_place_id.as_str(), target_id.as_str()),
                        "evented_perception:visible_exit",
                        self.decision_tick,
                    ),
                    vec![event.event_id.clone()],
                );
            }
            Some("visible_food_supply") => {
                let Some(food_source) = payload_value(event, "target_id") else {
                    return;
                };
                self.add_food_source_knowledge(
                    food_source,
                    "evented_perception:visible_food_supply",
                    vec![event.event_id.clone()],
                );
            }
            Some("visible_sleep_affordance") => {
                self.add_sleep_place_knowledge(
                    self.current_place_id.clone(),
                    payload_value(event, "target_id"),
                    "evented_perception:visible_sleep_affordance",
                    vec![event.event_id.clone()],
                );
            }
            _ => {}
        }
    }

    fn add_role_assignment_notice(
        &mut self,
        workplace_id: WorkplaceId,
        place_id: PlaceId,
        source: impl Into<String>,
        source_event_ids: Vec<EventId>,
    ) {
        let source = source.into();
        self.known_workplaces
            .insert(workplace_id.clone(), place_id.clone());
        self.push_fact(
            ActorKnownFact::routine_assignment(
                self.actor_id.clone(),
                "actor_knows_workplace",
                format!("{}@{}", workplace_id.as_str(), place_id.as_str()),
                format!("role_assignment_notice:{source}"),
                self.decision_tick,
            ),
            source_event_ids.clone(),
        );
        self.push_fact(
            ActorKnownFact::routine_assignment(
                self.actor_id.clone(),
                "workplace_assignment_active",
                workplace_id.as_str(),
                format!("role_assignment_notice:{source}"),
                self.decision_tick,
            ),
            source_event_ids.clone(),
        );
        if place_id == self.current_place_id {
            for stable_id in [
                "actor_at_workplace",
                "assigned_workplace_known",
                "at_workplace",
            ] {
                self.push_fact(
                    ActorKnownFact::observed_now(
                        self.actor_id.clone(),
                        stable_id,
                        place_id.as_str(),
                        "evented_perception:actor_at_noticed_workplace",
                        self.decision_tick,
                    ),
                    source_event_ids.clone(),
                );
            }
        }
    }

    fn add_food_source_knowledge(
        &mut self,
        food_source: &str,
        source: impl Into<String>,
        source_event_ids: Vec<EventId>,
    ) {
        let source = source.into();
        self.known_food_sources.insert(food_source.to_string());
        self.push_fact(
            ActorKnownFact::observed_now(
                self.actor_id.clone(),
                "actor_knows_food_source",
                food_source,
                source.clone(),
                self.decision_tick,
            ),
            source_event_ids.clone(),
        );
        self.push_fact(
            ActorKnownFact::observed_now(
                self.actor_id.clone(),
                "food_source_believed_accessible",
                food_source,
                source,
                self.decision_tick,
            ),
            source_event_ids,
        );
    }

    fn add_sleep_place_knowledge(
        &mut self,
        place_id: PlaceId,
        sleep_affordance_id: Option<&str>,
        source: impl Into<String>,
        source_event_ids: Vec<EventId>,
    ) {
        let source = source.into();
        self.known_sleep_places.insert(place_id.clone());
        if let Some(sleep_affordance_id) = sleep_affordance_id {
            self.push_fact(
                ActorKnownFact::observed_now(
                    self.actor_id.clone(),
                    "actor_knows_sleep_affordance",
                    sleep_affordance_id,
                    source.clone(),
                    self.decision_tick,
                ),
                source_event_ids.clone(),
            );
        }
        self.push_fact(
            ActorKnownFact::remembered_belief(
                self.actor_id.clone(),
                "actor_knows_sleep_place",
                place_id.as_str(),
                source.clone(),
                self.decision_tick,
            ),
            source_event_ids.clone(),
        );
        if place_id == self.current_place_id {
            self.push_fact(
                ActorKnownFact::observed_now(
                    self.actor_id.clone(),
                    "sleep_place_believed_accessible",
                    place_id.as_str(),
                    source,
                    self.decision_tick,
                ),
                source_event_ids,
            );
        }
    }

    fn add_window_framing_facts(
        &mut self,
        agent_state: &AgentState,
        window_id: &str,
        window_end_tick: SimTick,
        source_event_id: Option<EventId>,
    ) {
        let source_event_ids = source_event_id.into_iter().collect::<Vec<_>>();
        self.push_fact(
            ActorKnownFact::observed_now(
                self.actor_id.clone(),
                "actor_current_place_visible",
                self.current_place_id.as_str(),
                "evented_frame:current_place",
                self.decision_tick,
            ),
            source_event_ids.clone(),
        );
        if agent_state.needs_by_actor().contains_key(&self.actor_id) {
            self.push_fact(
                ActorKnownFact::remembered_belief(
                    self.actor_id.clone(),
                    "agent_needs_present",
                    "needs_present",
                    "agent_state:needs_present",
                    self.decision_tick,
                ),
                source_event_ids.clone(),
            );
        }
        self.push_fact(
            ActorKnownFact::remembered_belief(
                self.actor_id.clone(),
                "actor_belief_projection_limitation",
                "not_supplied",
                "no_human_day:typed_projection_limitation",
                self.decision_tick,
            ),
            source_event_ids.clone(),
        );
        self.push_fact(
            ActorKnownFact::remembered_belief(
                self.actor_id.clone(),
                "modeled_wait_reason",
                "bounded_idle",
                format!("window_id={window_id}:bounded_idle"),
                self.decision_tick,
            ),
            source_event_ids.clone(),
        );
        self.push_fact(
            ActorKnownFact::remembered_belief(
                self.actor_id.clone(),
                "reevaluation_window_known",
                window_id,
                format!(
                    "window_id={}:{}..{}",
                    window_id,
                    self.decision_tick.map_or(0, |tick| tick.value()),
                    window_end_tick.value()
                ),
                self.decision_tick,
            ),
            source_event_ids.clone(),
        );
        if has_active_intention(agent_state, &self.actor_id) {
            self.push_fact(
                ActorKnownFact::remembered_belief(
                    self.actor_id.clone(),
                    "active_intention_present",
                    "active",
                    "agent_state:active_intention",
                    self.decision_tick,
                ),
                source_event_ids.clone(),
            );
            self.push_fact(
                ActorKnownFact::remembered_belief(
                    self.actor_id.clone(),
                    "next_step_available",
                    "modeled_next_step",
                    "agent_state:active_intention_next_step",
                    self.decision_tick,
                ),
                source_event_ids,
            );
        }
    }

    fn push_fact(&mut self, fact: ActorKnownFact, source_event_ids: Vec<EventId>) {
        self.facts
            .push(fact.with_source_event_ids(source_event_ids));
    }
}

fn latest_frame_event_id(log: &EventLog) -> Option<EventId> {
    log.events()
        .iter()
        .rev()
        .find(|event| {
            matches!(
                event.event_type,
                EventKind::NoHumanDayStarted | EventKind::NoHumanAdvanceStarted
            )
        })
        .map(|event| event.event_id.clone())
        .or_else(|| log.events().first().map(|event| event.event_id.clone()))
}

fn payload_value<'a>(event: &'a EventEnvelope, key: &str) -> Option<&'a str> {
    event
        .payload
        .iter()
        .find(|field| field.key == key)
        .map(|field| field.value.as_str())
}

fn has_active_intention(agent_state: &AgentState, actor_id: &ActorId) -> bool {
    agent_state
        .active_intention_by_actor()
        .contains_key(actor_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::PayloadField;
    use crate::ids::{ActionId, ContentManifestId};
    use crate::scheduler::{OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId};

    fn actor_id() -> ActorId {
        ActorId::new("actor_tomas").unwrap()
    }

    fn place_id(value: &str) -> PlaceId {
        PlaceId::new(value).unwrap()
    }

    fn workplace_id() -> WorkplaceId {
        WorkplaceId::new("workplace_tomas").unwrap()
    }

    fn content_manifest_id() -> ContentManifestId {
        ContentManifestId::new("phase3a_manifest").unwrap()
    }

    fn ordering_key(actor_id: &ActorId, tick: SimTick, action_id: &str) -> OrderingKey {
        OrderingKey::new(
            tick,
            SchedulePhase::NoHumanProcess,
            SchedulerSourceId::Actor(actor_id.clone()),
            ProposalSequence::new(0),
            ActionId::new(action_id).unwrap(),
            vec![actor_id.as_str().to_string()],
            format!("{action_id}:{}", actor_id.as_str()),
        )
    }

    fn event(
        event_id: &str,
        actor_id: &ActorId,
        kind: EventKind,
        tick: SimTick,
        action_id: &str,
        payload: Vec<PayloadField>,
    ) -> EventEnvelope {
        let mut event = EventEnvelope::new_v1(
            EventId::new(event_id).unwrap(),
            kind,
            0,
            0,
            tick,
            ordering_key(actor_id, tick, action_id),
            content_manifest_id(),
        );
        event.actor_id = Some(actor_id.clone());
        event.participants = vec![actor_id.as_str().to_string()];
        event.payload = payload;
        event
    }

    #[test]
    fn raw_workplace_assignment_is_not_actor_known_without_notice() {
        let actor_id = actor_id();
        let home = place_id("home_tomas");
        let log = EventLog::new();

        let surface = NoHumanActorKnownSurfaceBuilder::from_event_log(
            &log,
            &AgentState::default(),
            actor_id,
            home,
            SimTick::ZERO,
            "morning",
            SimTick::new(4),
        )
        .build(&AgentState::default());

        assert!(surface.context().known_workplaces().is_empty());
        assert!(!surface
            .context()
            .actor_known_facts()
            .iter()
            .any(|fact| fact.stable_id() == "actor_knows_workplace"));
    }

    #[test]
    fn current_place_is_not_implicit_sleep_knowledge() {
        let actor_id = actor_id();
        let home = place_id("home_tomas");
        let log = EventLog::new();

        let surface = NoHumanActorKnownSurfaceBuilder::from_event_log(
            &log,
            &AgentState::default(),
            actor_id,
            home,
            SimTick::ZERO,
            "morning",
            SimTick::new(4),
        )
        .build(&AgentState::default());

        assert!(surface.context().known_sleep_places().is_empty());
        assert!(!surface
            .context()
            .actor_known_facts()
            .iter()
            .any(|fact| fact.stable_id() == "sleep_place_believed_accessible"));
    }

    #[test]
    fn workplace_becomes_known_only_from_modeled_notice() {
        let actor_id = actor_id();
        let workshop = place_id("workshop_tomas");
        let event_id = "event.role_notice.actor_tomas";
        let mut log = EventLog::new();
        log.append(event(
            event_id,
            &actor_id,
            EventKind::RoleAssignmentNoticeRecorded,
            SimTick::ZERO,
            "role_assignment_notice",
            vec![
                PayloadField::new("actor_id", actor_id.as_str()),
                PayloadField::new("workplace_id", workplace_id().as_str()),
                PayloadField::new("place_id", workshop.as_str()),
            ],
        ))
        .unwrap();

        let surface = NoHumanActorKnownSurfaceBuilder::from_event_log(
            &log,
            &AgentState::default(),
            actor_id,
            workshop.clone(),
            SimTick::ZERO,
            "work_window",
            SimTick::new(4),
        )
        .build(&AgentState::default());

        assert_eq!(surface.context().known_workplaces().len(), 1);
        let workplace_fact = surface
            .context()
            .actor_known_facts()
            .iter()
            .find(|fact| fact.stable_id() == "actor_knows_workplace")
            .expect("role notice should produce actor-known workplace");
        assert!(workplace_fact
            .source_event_ids()
            .iter()
            .any(|id| id.as_str() == event_id));
    }

    #[test]
    fn visible_food_observation_still_requires_local_visibility() {
        let actor_id = actor_id();
        let kitchen = place_id("kitchen");
        let event_id = "event.visible_food.actor_tomas";
        let mut log = EventLog::new();
        log.append(event(
            event_id,
            &actor_id,
            EventKind::ObservationRecorded,
            SimTick::ZERO,
            "record_observation",
            vec![
                PayloadField::new("actor_id", actor_id.as_str()),
                PayloadField::new("perceived_kind", "visible_food_supply"),
                PayloadField::new("target_id", "visible_meal"),
            ],
        ))
        .unwrap();

        let surface = NoHumanActorKnownSurfaceBuilder::from_event_log(
            &log,
            &AgentState::default(),
            actor_id,
            kitchen,
            SimTick::ZERO,
            "morning",
            SimTick::new(4),
        )
        .build(&AgentState::default());

        assert!(surface
            .context()
            .known_food_sources()
            .contains("visible_meal"));
        assert!(!surface
            .context()
            .known_food_sources()
            .contains("hidden_meal"));
        let food_fact = surface
            .context()
            .actor_known_facts()
            .iter()
            .find(|fact| fact.stable_id() == "actor_knows_food_source")
            .expect("visible food observation should produce actor-known food");
        assert_eq!(food_fact.source_event_ids().len(), 1);
        assert_eq!(food_fact.source_event_ids()[0].as_str(), event_id);
    }
}
