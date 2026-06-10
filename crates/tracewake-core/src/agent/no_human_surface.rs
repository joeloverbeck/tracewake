use std::collections::{BTreeMap, BTreeSet};

use crate::agent::{ActorKnownFact, ActorKnownPlanningContext, SourceEventIds};
use crate::epistemics::{ActorKnownProjectionRecord, EpistemicProjection, KnowledgeContext};
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

/// Event-log-backed actor-known surface builder for no-human decisions.
///
/// Raw convenience construction without source events is intentionally absent.
///
/// ```compile_fail
/// use tracewake_core::agent::NoHumanActorKnownSurfaceBuilder;
/// use tracewake_core::ids::{ActorId, PlaceId, WorkplaceId};
///
/// let builder = NoHumanActorKnownSurfaceBuilder::new(
///     ActorId::new("actor_tomas").unwrap(),
///     PlaceId::new("home_tomas").unwrap(),
///     None,
/// );
/// let _ = builder.with_role_assignment_notice(
///     WorkplaceId::new("workplace_tomas").unwrap(),
///     PlaceId::new("workshop_tomas").unwrap(),
///     "raw_assignment",
/// );
/// ```
///
/// ```compile_fail
/// use tracewake_core::agent::NoHumanActorKnownSurfaceBuilder;
/// use tracewake_core::ids::{ActorId, PlaceId};
///
/// let builder = NoHumanActorKnownSurfaceBuilder::new(
///     ActorId::new("actor_tomas").unwrap(),
///     PlaceId::new("home_tomas").unwrap(),
///     None,
/// );
/// let _ = builder.with_sleep_place_knowledge(
///     PlaceId::new("home_tomas").unwrap(),
///     "raw_sleep_truth",
/// );
/// ```
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

pub struct NoHumanActorKnownSurfaceRequest<'a> {
    pub projection: &'a EpistemicProjection,
    pub agent_state: &'a AgentState,
    pub actor_id: ActorId,
    pub current_place_id: PlaceId,
    pub decision_tick: SimTick,
    pub window_id: &'a str,
    pub window_end_tick: SimTick,
    pub frame_event_id: Option<EventId>,
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

    pub fn from_projection(request: NoHumanActorKnownSurfaceRequest<'_>) -> Self {
        let mut builder = Self::new(
            request.actor_id,
            request.current_place_id,
            Some(request.decision_tick),
        );
        builder.consume_projection_records(request.projection);
        builder.add_window_framing_facts(
            request.agent_state,
            request.window_id,
            request.window_end_tick,
            request.frame_event_id,
        );
        builder
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

    fn consume_projection_records(&mut self, projection: &EpistemicProjection) {
        let context = KnowledgeContext::embodied(
            self.actor_id.clone(),
            self.decision_tick.unwrap_or(SimTick::ZERO),
        );
        for record in projection.actor_known_records_for_context(&context) {
            self.consume_projection_record(record);
        }
    }

    fn consume_projection_record(&mut self, record: &ActorKnownProjectionRecord) {
        match record {
            ActorKnownProjectionRecord::Route {
                from_place_id,
                to_place_id,
                source,
                source_event_id,
                ..
            } => {
                self.known_edges
                    .entry(from_place_id.clone())
                    .or_default()
                    .insert(to_place_id.clone());
                self.facts.push(ActorKnownFact::observed_now(
                    self.actor_id.clone(),
                    "known_route_surface",
                    format!("{}->{}", from_place_id.as_str(), to_place_id.as_str()),
                    source.source_label(),
                    self.decision_tick,
                    SourceEventIds::checked(vec![source_event_id.clone()])
                        .expect("projection record source ids are non-empty"),
                ));
            }
            ActorKnownProjectionRecord::FoodSource {
                food_source_id,
                place_id,
                source,
                source_event_id,
                ..
            } => {
                if place_id
                    .as_ref()
                    .is_some_and(|place_id| place_id != &self.current_place_id)
                {
                    return;
                }
                self.add_food_source_knowledge(
                    food_source_id,
                    source.source_label(),
                    vec![source_event_id.clone()],
                );
            }
            ActorKnownProjectionRecord::SleepPlace {
                place_id,
                sleep_affordance_id,
                source,
                source_event_id,
                ..
            } => {
                self.add_sleep_place_knowledge(
                    place_id.clone(),
                    sleep_affordance_id.as_deref(),
                    source.source_label(),
                    vec![source_event_id.clone()],
                );
            }
            ActorKnownProjectionRecord::Workplace {
                workplace_id,
                place_id,
                source,
                source_event_id,
                ..
            } => {
                self.add_role_assignment_notice(
                    workplace_id.clone(),
                    place_id.clone(),
                    source.source_label(),
                    vec![source_event_id.clone()],
                );
            }
        }
    }

    fn add_role_assignment_notice(
        &mut self,
        workplace_id: WorkplaceId,
        place_id: PlaceId,
        source: impl Into<String>,
        source_event_ids: Vec<EventId>,
    ) {
        let source_event_ids = SourceEventIds::checked(source_event_ids)
            .expect("role notice source ids are non-empty");
        let source = source.into();
        self.known_workplaces
            .insert(workplace_id.clone(), place_id.clone());
        self.facts.push(ActorKnownFact::routine_assignment(
            self.actor_id.clone(),
            "actor_knows_workplace",
            format!("{}@{}", workplace_id.as_str(), place_id.as_str()),
            format!("role_assignment_notice:{source}"),
            self.decision_tick,
            source_event_ids.clone(),
        ));
        self.facts.push(ActorKnownFact::routine_assignment(
            self.actor_id.clone(),
            "workplace_assignment_active",
            workplace_id.as_str(),
            format!("role_assignment_notice:{source}"),
            self.decision_tick,
            source_event_ids.clone(),
        ));
        if place_id == self.current_place_id {
            for stable_id in [
                "actor_at_workplace",
                "assigned_workplace_known",
                "at_workplace",
            ] {
                self.facts.push(ActorKnownFact::observed_now(
                    self.actor_id.clone(),
                    stable_id,
                    place_id.as_str(),
                    "evented_perception:actor_at_noticed_workplace",
                    self.decision_tick,
                    source_event_ids.clone(),
                ));
            }
        }
    }

    fn add_food_source_knowledge(
        &mut self,
        food_source: &str,
        source: impl Into<String>,
        source_event_ids: Vec<EventId>,
    ) {
        let source_event_ids =
            SourceEventIds::checked(source_event_ids).expect("food source ids are non-empty");
        let source = source.into();
        self.known_food_sources.insert(food_source.to_string());
        self.facts.push(ActorKnownFact::observed_now(
            self.actor_id.clone(),
            "actor_knows_food_source",
            food_source,
            source.clone(),
            self.decision_tick,
            source_event_ids.clone(),
        ));
        self.facts.push(ActorKnownFact::observed_now(
            self.actor_id.clone(),
            "food_source_believed_accessible",
            food_source,
            source,
            self.decision_tick,
            source_event_ids,
        ));
    }

    fn add_sleep_place_knowledge(
        &mut self,
        place_id: PlaceId,
        sleep_affordance_id: Option<&str>,
        source: impl Into<String>,
        source_event_ids: Vec<EventId>,
    ) {
        let source_event_ids =
            SourceEventIds::checked(source_event_ids).expect("sleep source ids are non-empty");
        let source = source.into();
        self.known_sleep_places.insert(place_id.clone());
        if let Some(sleep_affordance_id) = sleep_affordance_id {
            self.facts.push(ActorKnownFact::observed_now(
                self.actor_id.clone(),
                "actor_knows_sleep_affordance",
                sleep_affordance_id,
                source.clone(),
                self.decision_tick,
                source_event_ids.clone(),
            ));
        }
        self.facts.push(ActorKnownFact::remembered_belief(
            self.actor_id.clone(),
            "actor_knows_sleep_place",
            place_id.as_str(),
            source.clone(),
            self.decision_tick,
            source_event_ids.clone(),
        ));
        if place_id == self.current_place_id {
            self.facts.push(ActorKnownFact::observed_now(
                self.actor_id.clone(),
                "sleep_place_believed_accessible",
                place_id.as_str(),
                source,
                self.decision_tick,
                source_event_ids,
            ));
        }
    }

    fn add_window_framing_facts(
        &mut self,
        agent_state: &AgentState,
        window_id: &str,
        window_end_tick: SimTick,
        source_event_id: Option<EventId>,
    ) {
        let Some(source_event_ids) = source_event_id
            .map(|id| SourceEventIds::checked(vec![id]).expect("frame event id is non-empty"))
        else {
            return;
        };
        self.facts.push(ActorKnownFact::observed_now(
            self.actor_id.clone(),
            "actor_current_place_visible",
            self.current_place_id.as_str(),
            "evented_frame:current_place",
            self.decision_tick,
            source_event_ids.clone(),
        ));
        if agent_state.needs_by_actor().contains_key(&self.actor_id) {
            self.facts.push(ActorKnownFact::remembered_belief(
                self.actor_id.clone(),
                "agent_needs_present",
                "needs_present",
                "agent_state:needs_present",
                self.decision_tick,
                source_event_ids.clone(),
            ));
        }
        self.facts.push(ActorKnownFact::remembered_belief(
            self.actor_id.clone(),
            "actor_belief_projection_limitation",
            "not_supplied",
            "no_human_day:typed_projection_limitation",
            self.decision_tick,
            source_event_ids.clone(),
        ));
        self.facts.push(ActorKnownFact::remembered_belief(
            self.actor_id.clone(),
            "modeled_wait_reason",
            "bounded_idle",
            format!("window_id={window_id}:bounded_idle"),
            self.decision_tick,
            source_event_ids.clone(),
        ));
        self.facts.push(ActorKnownFact::remembered_belief(
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
            source_event_ids.clone(),
        ));
        if has_active_intention(agent_state, &self.actor_id) {
            self.facts.push(ActorKnownFact::remembered_belief(
                self.actor_id.clone(),
                "active_intention_present",
                "active",
                "agent_state:active_intention",
                self.decision_tick,
                source_event_ids.clone(),
            ));
            self.facts.push(ActorKnownFact::remembered_belief(
                self.actor_id.clone(),
                "next_step_available",
                "modeled_next_step",
                "agent_state:active_intention_next_step",
                self.decision_tick,
                source_event_ids,
            ));
        }
    }
}

fn has_active_intention(agent_state: &AgentState, actor_id: &ActorId) -> bool {
    agent_state
        .active_intention_by_actor()
        .contains_key(actor_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::apply::apply_epistemic_event;
    use crate::events::log::EventLog;
    use crate::events::{EventEnvelope, EventKind, PayloadField, EVENT_SCHEMA_V1};
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

    fn projection_from_log(log: &EventLog) -> EpistemicProjection {
        let mut projection = EpistemicProjection::new(content_manifest_id());
        for event in log.events() {
            apply_epistemic_event(&mut projection, event).unwrap();
        }
        projection
    }

    fn build_surface(
        projection: &EpistemicProjection,
        actor_id: ActorId,
        current_place_id: PlaceId,
        frame_event_id: Option<EventId>,
    ) -> SealedActorKnownSurface {
        let agent_state = AgentState::default();
        NoHumanActorKnownSurfaceBuilder::from_projection(NoHumanActorKnownSurfaceRequest {
            projection,
            agent_state: &agent_state,
            actor_id,
            current_place_id,
            decision_tick: SimTick::ZERO,
            window_id: "morning",
            window_end_tick: SimTick::new(4),
            frame_event_id,
        })
        .build(&agent_state)
    }

    #[test]
    fn raw_workplace_assignment_is_not_actor_known_without_notice() {
        let actor_id = actor_id();
        let home = place_id("home_tomas");
        let log = EventLog::new();
        let projection = projection_from_log(&log);

        let surface = build_surface(&projection, actor_id, home, None);

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
        let projection = projection_from_log(&log);

        let surface = build_surface(&projection, actor_id, home, None);

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
                PayloadField::new("schema_version", EVENT_SCHEMA_V1),
                PayloadField::new("actor_id", actor_id.as_str()),
                PayloadField::new("workplace_id", workplace_id().as_str()),
                PayloadField::new("place_id", workshop.as_str()),
            ],
        ))
        .unwrap();
        let projection = projection_from_log(&log);

        let surface = build_surface(
            &projection,
            actor_id,
            workshop.clone(),
            Some(EventId::new(event_id).unwrap()),
        );

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
                PayloadField::new("schema_version", EVENT_SCHEMA_V1),
                PayloadField::new("actor_id", actor_id.as_str()),
                PayloadField::new("observer_actor_id", actor_id.as_str()),
                PayloadField::new("observer_place_id", kitchen.as_str()),
                PayloadField::new("observation_id", "observation.visible_food.actor_tomas"),
                PayloadField::new("observed_tick", "0"),
                PayloadField::new("source_event_id", event_id),
                PayloadField::new("channel", "direct_sight"),
                PayloadField::new("place_id", kitchen.as_str()),
                PayloadField::new("confidence", "1000"),
                PayloadField::new("perceived_kind", "visible_food_supply"),
                PayloadField::new("subject_id", kitchen.as_str()),
                PayloadField::new("target_id", "visible_meal"),
            ],
        ))
        .unwrap();
        let projection = projection_from_log(&log);

        let surface = build_surface(
            &projection,
            actor_id,
            kitchen,
            Some(EventId::new(event_id).unwrap()),
        );

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
