use std::collections::{BTreeMap, BTreeSet};

use crate::agent::{ActorKnownFact, ActorKnownPlanningContext, SourceEventIds};
use crate::epistemics::{
    ActorKnownProjectionAccessibilityScope, ActorKnownProjectionFreshness,
    ActorKnownProjectionRecord, EpistemicProjection, KnowledgeContext,
};
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
    current_place_witness_event_id: Option<EventId>,
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
    pub current_place_witness_event_id: Option<EventId>,
    pub needs_witness_event_id: Option<EventId>,
    pub frame_event_id: Option<EventId>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ProjectionFactPolicy {
    freshness: ActorKnownProjectionFreshness,
    source_tick: SimTick,
    accessibility_scope: ActorKnownProjectionAccessibilityScope,
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
            current_place_witness_event_id: None,
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
        builder.current_place_witness_event_id = request.current_place_witness_event_id.clone();
        builder.consume_projection_records(request.projection);
        builder.add_window_framing_facts(
            request.agent_state,
            request.window_id,
            request.window_end_tick,
            request.current_place_witness_event_id,
            request.needs_witness_event_id,
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
        for classified in
            projection.classified_actor_known_records_for_context(&context, &self.current_place_id)
        {
            self.consume_projection_record(
                classified.record(),
                classified.freshness(),
                classified.source_tick(),
            );
        }
    }

    fn consume_projection_record(
        &mut self,
        record: &ActorKnownProjectionRecord,
        freshness: ActorKnownProjectionFreshness,
        source_tick: SimTick,
    ) {
        let policy = record.policy();
        let fact_policy = ProjectionFactPolicy {
            freshness,
            source_tick,
            accessibility_scope: policy.accessibility_scope(),
        };
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
                self.push_projection_fact(
                    freshness,
                    "known_route_surface",
                    format!("{}->{}", from_place_id.as_str(), to_place_id.as_str()),
                    source.source_label(),
                    source_tick,
                    SourceEventIds::checked(vec![source_event_id.clone()])
                        .expect("projection record source ids are non-empty"),
                );
            }
            ActorKnownProjectionRecord::FoodSource {
                food_source_id,
                place_id: _,
                believed_servings: _,
                source,
                source_event_id,
                ..
            } => {
                self.add_food_source_knowledge(
                    food_source_id,
                    source.source_label(),
                    vec![source_event_id.clone()],
                    fact_policy,
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
                    fact_policy,
                );
            }
            ActorKnownProjectionRecord::Workplace {
                workplace_id,
                place_id,
                believed_access_open,
                source,
                source_event_id,
                ..
            } => {
                self.add_role_assignment_notice(
                    workplace_id.clone(),
                    place_id.clone(),
                    *believed_access_open,
                    source.source_label(),
                    vec![source_event_id.clone()],
                    fact_policy,
                );
            }
            ActorKnownProjectionRecord::CurrentPlace { .. }
            | ActorKnownProjectionRecord::CarriedItem { .. }
            | ActorKnownProjectionRecord::LocalDoor { .. }
            | ActorKnownProjectionRecord::LocalContainer { .. }
            | ActorKnownProjectionRecord::LocalItem { .. }
            | ActorKnownProjectionRecord::LocalActor { .. } => {}
        }
    }

    fn push_projection_fact(
        &mut self,
        freshness: ActorKnownProjectionFreshness,
        stable_id: impl Into<String>,
        value: impl Into<String>,
        source: impl Into<String>,
        source_tick: SimTick,
        source_event_ids: SourceEventIds,
    ) {
        match freshness {
            ActorKnownProjectionFreshness::CurrentlyPerceived => {
                self.facts.push(ActorKnownFact::observed_now(
                    self.actor_id.clone(),
                    stable_id,
                    value,
                    source,
                    Some(source_tick),
                    source_event_ids,
                ));
            }
            ActorKnownProjectionFreshness::Remembered => {
                self.facts.push(ActorKnownFact::remembered_belief(
                    self.actor_id.clone(),
                    stable_id,
                    value,
                    source,
                    Some(source_tick),
                    source_event_ids,
                ));
            }
        }
    }

    fn add_role_assignment_notice(
        &mut self,
        workplace_id: WorkplaceId,
        place_id: PlaceId,
        believed_access_open: bool,
        source: impl Into<String>,
        source_event_ids: Vec<EventId>,
        policy: ProjectionFactPolicy,
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
            Some(policy.source_tick),
            source_event_ids.clone(),
        ));
        self.facts.push(ActorKnownFact::routine_assignment(
            self.actor_id.clone(),
            "workplace_assignment_active",
            workplace_id.as_str(),
            format!("role_assignment_notice:{source}"),
            Some(policy.source_tick),
            source_event_ids.clone(),
        ));
        if policy.accessibility_scope == ActorKnownProjectionAccessibilityScope::FromAnyPlace {
            self.facts.push(ActorKnownFact::routine_assignment(
                self.actor_id.clone(),
                "workplace_believed_accessible",
                format!("{}:{}", workplace_id.as_str(), believed_access_open),
                format!("role_assignment_notice:{source}"),
                Some(policy.source_tick),
                source_event_ids.clone(),
            ));
        }
        if place_id == self.current_place_id {
            if let Some(current_place_witness_event_id) = &self.current_place_witness_event_id {
                let current_place_source_event_ids =
                    SourceEventIds::checked(vec![current_place_witness_event_id.clone()])
                        .expect("place witness id is non-empty");
                for stable_id in [
                    "actor_at_workplace",
                    "assigned_workplace_known",
                    "at_workplace",
                ] {
                    self.facts.push(ActorKnownFact::observed_now(
                        self.actor_id.clone(),
                        stable_id,
                        place_id.as_str(),
                        "evented_perception:actor_at_workplace",
                        self.decision_tick,
                        current_place_source_event_ids.clone(),
                    ));
                }
            }
        }
    }

    fn add_food_source_knowledge(
        &mut self,
        food_source: &str,
        source: impl Into<String>,
        source_event_ids: Vec<EventId>,
        policy: ProjectionFactPolicy,
    ) {
        let source_event_ids =
            SourceEventIds::checked(source_event_ids).expect("food source ids are non-empty");
        let source = source.into();
        self.known_food_sources.insert(food_source.to_string());
        self.push_projection_fact(
            policy.freshness,
            "actor_knows_food_source",
            food_source,
            source.clone(),
            policy.source_tick,
            source_event_ids.clone(),
        );
        if policy.accessibility_scope == ActorKnownProjectionAccessibilityScope::FromAnyPlace {
            self.push_projection_fact(
                policy.freshness,
                "food_source_believed_accessible",
                food_source,
                source,
                policy.source_tick,
                source_event_ids,
            );
        }
    }

    fn add_sleep_place_knowledge(
        &mut self,
        place_id: PlaceId,
        sleep_affordance_id: Option<&str>,
        source: impl Into<String>,
        source_event_ids: Vec<EventId>,
        policy: ProjectionFactPolicy,
    ) {
        let source_event_ids =
            SourceEventIds::checked(source_event_ids).expect("sleep source ids are non-empty");
        let source = source.into();
        self.known_sleep_places.insert(place_id.clone());
        if let Some(sleep_affordance_id) = sleep_affordance_id {
            self.push_projection_fact(
                policy.freshness,
                "actor_knows_sleep_affordance",
                sleep_affordance_id,
                source.clone(),
                policy.source_tick,
                source_event_ids.clone(),
            );
        }
        self.push_projection_fact(
            policy.freshness,
            "actor_knows_sleep_place",
            place_id.as_str(),
            source.clone(),
            policy.source_tick,
            source_event_ids.clone(),
        );
        if policy.accessibility_scope == ActorKnownProjectionAccessibilityScope::FromAnyPlace {
            self.push_projection_fact(
                policy.freshness,
                "sleep_place_believed_accessible",
                place_id.as_str(),
                source,
                policy.source_tick,
                source_event_ids,
            );
        }
    }

    fn add_window_framing_facts(
        &mut self,
        agent_state: &AgentState,
        window_id: &str,
        window_end_tick: SimTick,
        current_place_witness_event_id: Option<EventId>,
        needs_witness_event_id: Option<EventId>,
        frame_event_id: Option<EventId>,
    ) {
        let Some(frame_source_event_ids) = frame_event_id
            .map(|id| SourceEventIds::checked(vec![id]).expect("frame event id is non-empty"))
        else {
            return;
        };
        if let Some(current_place_source_event_ids) = current_place_witness_event_id
            .map(|id| SourceEventIds::checked(vec![id]).expect("place witness id is non-empty"))
        {
            self.facts.push(ActorKnownFact::observed_now(
                self.actor_id.clone(),
                "actor_current_place_visible",
                self.current_place_id.as_str(),
                "evented_perception:current_place",
                self.decision_tick,
                current_place_source_event_ids,
            ));
        }
        if agent_state.needs_by_actor().contains_key(&self.actor_id) {
            if let Some(needs_source_event_ids) = needs_witness_event_id
                .map(|id| SourceEventIds::checked(vec![id]).expect("needs witness id is non-empty"))
            {
                self.facts.push(ActorKnownFact::remembered_belief(
                    self.actor_id.clone(),
                    "agent_needs_present",
                    "needs_present",
                    "agent_state:needs_present",
                    self.decision_tick,
                    needs_source_event_ids,
                ));
            }
        }
        self.facts.push(ActorKnownFact::remembered_belief(
            self.actor_id.clone(),
            "actor_belief_projection_limitation",
            "not_supplied",
            "no_human_day:typed_projection_limitation",
            self.decision_tick,
            frame_source_event_ids.clone(),
        ));
        self.facts.push(ActorKnownFact::remembered_belief(
            self.actor_id.clone(),
            "modeled_wait_reason",
            "bounded_idle",
            format!("window_id={window_id}:bounded_idle"),
            self.decision_tick,
            frame_source_event_ids.clone(),
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
            frame_source_event_ids.clone(),
        ));
        if has_active_intention(agent_state, &self.actor_id) {
            self.facts.push(ActorKnownFact::remembered_belief(
                self.actor_id.clone(),
                "active_intention_present",
                "active",
                "agent_state:active_intention",
                self.decision_tick,
                frame_source_event_ids.clone(),
            ));
            self.facts.push(ActorKnownFact::remembered_belief(
                self.actor_id.clone(),
                "next_step_available",
                "modeled_next_step",
                "agent_state:active_intention_next_step",
                self.decision_tick,
                frame_source_event_ids,
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
    use crate::agent::htn::{resolve_condition, ConditionResolution};
    use crate::agent::{
        plan_local_actions, LocalPlanRequest, PlannerGoal, RoutineCondition, RoutineStep,
        DEFAULT_PLANNER_BUDGET,
    };
    use crate::events::apply::apply_epistemic_event;
    use crate::events::log::EventLog;
    use crate::events::{EventCause, EventEnvelope, EventKind, PayloadField, EVENT_SCHEMA_V1};
    use crate::ids::{ActionId, ContentManifestId, IntentionId, SemanticActionId};
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
        if kind.requires_cause() {
            event.causes = vec![EventCause::Process(
                crate::ids::ProcessId::new("process_no_human_surface_test").unwrap(),
            )];
        }
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
        build_surface_at(
            projection,
            actor_id,
            current_place_id,
            SimTick::ZERO,
            frame_event_id,
        )
    }

    fn build_surface_at(
        projection: &EpistemicProjection,
        actor_id: ActorId,
        current_place_id: PlaceId,
        decision_tick: SimTick,
        frame_event_id: Option<EventId>,
    ) -> SealedActorKnownSurface {
        let agent_state = AgentState::default();
        NoHumanActorKnownSurfaceBuilder::from_projection(NoHumanActorKnownSurfaceRequest {
            projection,
            agent_state: &agent_state,
            actor_id,
            current_place_id,
            decision_tick,
            window_id: "morning",
            window_end_tick: SimTick::new(4),
            current_place_witness_event_id: frame_event_id.clone(),
            needs_witness_event_id: None,
            frame_event_id,
        })
        .build(&agent_state)
    }

    fn fact_stable_ids(surface: &SealedActorKnownSurface) -> Vec<&str> {
        surface
            .context()
            .actor_known_facts()
            .iter()
            .map(|fact| fact.stable_id())
            .collect()
    }

    #[test]
    fn active_intention_presence_uses_actor_specific_index() {
        let actor_id = actor_id();
        let other_actor_id = ActorId::new("actor_elena").unwrap();
        let mut agent_state = AgentState::default();

        assert!(!has_active_intention(&agent_state, &actor_id));

        agent_state.active_intention_by_actor.insert(
            other_actor_id,
            IntentionId::new("intention_other_actor").unwrap(),
        );
        assert!(!has_active_intention(&agent_state, &actor_id));

        agent_state.active_intention_by_actor.insert(
            actor_id.clone(),
            IntentionId::new("intention_actor_tomas").unwrap(),
        );
        assert!(has_active_intention(&agent_state, &actor_id));
    }

    #[test]
    fn food_source_accessible_fact_requires_from_any_place_scope() {
        let actor_id = actor_id();
        let kitchen = place_id("kitchen");
        let event_id = EventId::new("event.food_scope.actor_tomas").unwrap();
        let policy_from_any_place = ProjectionFactPolicy {
            freshness: ActorKnownProjectionFreshness::CurrentlyPerceived,
            source_tick: SimTick::new(4),
            accessibility_scope: ActorKnownProjectionAccessibilityScope::FromAnyPlace,
        };
        let policy_not_accessible = ProjectionFactPolicy {
            freshness: ActorKnownProjectionFreshness::CurrentlyPerceived,
            source_tick: SimTick::new(4),
            accessibility_scope: ActorKnownProjectionAccessibilityScope::None,
        };

        let mut reachable_builder =
            NoHumanActorKnownSurfaceBuilder::new(actor_id.clone(), kitchen.clone(), None);
        reachable_builder.add_food_source_knowledge(
            "kitchen_stew",
            "evented_perception:visible_food_supply",
            vec![event_id.clone()],
            policy_from_any_place,
        );
        let reachable_surface = reachable_builder.build(&AgentState::default());
        let reachable_fact_ids = fact_stable_ids(&reachable_surface);
        assert!(reachable_fact_ids.contains(&"actor_knows_food_source"));
        assert!(reachable_fact_ids.contains(&"food_source_believed_accessible"));
        assert!(reachable_surface
            .context()
            .known_food_sources()
            .contains("kitchen_stew"));

        let mut current_place_builder =
            NoHumanActorKnownSurfaceBuilder::new(actor_id, kitchen, None);
        current_place_builder.add_food_source_knowledge(
            "table_bread",
            "evented_perception:visible_food_supply",
            vec![event_id],
            policy_not_accessible,
        );
        let current_place_surface = current_place_builder.build(&AgentState::default());
        let current_place_fact_ids = fact_stable_ids(&current_place_surface);
        assert!(current_place_fact_ids.contains(&"actor_knows_food_source"));
        assert!(!current_place_fact_ids.contains(&"food_source_believed_accessible"));
        assert!(current_place_surface
            .context()
            .known_food_sources()
            .contains("table_bread"));
    }

    #[test]
    fn workplace_accessible_fact_requires_from_any_place_scope() {
        let actor_id = actor_id();
        let workshop = place_id("workshop_tomas");
        let event_id = EventId::new("event.workplace_scope.actor_tomas").unwrap();
        let policy_from_any_place = ProjectionFactPolicy {
            freshness: ActorKnownProjectionFreshness::CurrentlyPerceived,
            source_tick: SimTick::new(4),
            accessibility_scope: ActorKnownProjectionAccessibilityScope::FromAnyPlace,
        };
        let policy_not_accessible = ProjectionFactPolicy {
            freshness: ActorKnownProjectionFreshness::CurrentlyPerceived,
            source_tick: SimTick::new(4),
            accessibility_scope: ActorKnownProjectionAccessibilityScope::None,
        };

        let mut reachable_builder =
            NoHumanActorKnownSurfaceBuilder::new(actor_id.clone(), workshop.clone(), None);
        reachable_builder.add_role_assignment_notice(
            workplace_id(),
            workshop.clone(),
            true,
            "role_assignment_notice:evented",
            vec![event_id.clone()],
            policy_from_any_place,
        );
        let reachable_surface = reachable_builder.build(&AgentState::default());
        let reachable_fact_ids = fact_stable_ids(&reachable_surface);
        assert!(reachable_fact_ids.contains(&"actor_knows_workplace"));
        assert!(reachable_fact_ids.contains(&"workplace_assignment_active"));
        assert!(reachable_fact_ids.contains(&"workplace_believed_accessible"));
        assert_eq!(reachable_surface.context().known_workplaces().len(), 1);

        let mut current_place_builder =
            NoHumanActorKnownSurfaceBuilder::new(actor_id, workshop.clone(), None);
        current_place_builder.add_role_assignment_notice(
            workplace_id(),
            workshop,
            true,
            "role_assignment_notice:evented",
            vec![event_id],
            policy_not_accessible,
        );
        let current_place_surface = current_place_builder.build(&AgentState::default());
        let current_place_fact_ids = fact_stable_ids(&current_place_surface);
        assert!(current_place_fact_ids.contains(&"actor_knows_workplace"));
        assert!(current_place_fact_ids.contains(&"workplace_assignment_active"));
        assert!(!current_place_fact_ids.contains(&"workplace_believed_accessible"));
        assert_eq!(current_place_surface.context().known_workplaces().len(), 1);
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
                PayloadField::new("access_open", "true"),
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
    fn workplace_notices_supersede_before_no_human_facts_are_minted() {
        let actor_id = actor_id();
        let workshop = place_id("workshop_tomas");
        let old_event_id = "event.role_notice.actor_tomas.closed";
        let new_event_id = "event.role_notice.actor_tomas.open";
        let mut log = EventLog::new();
        log.append(event(
            old_event_id,
            &actor_id,
            EventKind::RoleAssignmentNoticeRecorded,
            SimTick::new(4),
            "role_assignment_notice",
            vec![
                PayloadField::new("schema_version", EVENT_SCHEMA_V1),
                PayloadField::new("actor_id", actor_id.as_str()),
                PayloadField::new("workplace_id", workplace_id().as_str()),
                PayloadField::new("place_id", workshop.as_str()),
                PayloadField::new("access_open", "false"),
            ],
        ))
        .unwrap();
        log.append(event(
            new_event_id,
            &actor_id,
            EventKind::RoleAssignmentNoticeRecorded,
            SimTick::new(9),
            "role_assignment_notice",
            vec![
                PayloadField::new("schema_version", EVENT_SCHEMA_V1),
                PayloadField::new("actor_id", actor_id.as_str()),
                PayloadField::new("workplace_id", workplace_id().as_str()),
                PayloadField::new("place_id", workshop.as_str()),
                PayloadField::new("access_open", "true"),
            ],
        ))
        .unwrap();
        let projection = projection_from_log(&log);

        let surface = build_surface_at(
            &projection,
            actor_id,
            workshop,
            SimTick::new(10),
            Some(EventId::new(new_event_id).unwrap()),
        );
        let access_facts: Vec<_> = surface
            .context()
            .actor_known_facts()
            .iter()
            .filter(|fact| fact.stable_id() == "workplace_believed_accessible")
            .collect();

        assert_eq!(access_facts.len(), 1);
        assert_eq!(
            access_facts[0].value(),
            format!("{}:true", workplace_id().as_str())
        );
        assert_eq!(access_facts[0].source_event_ids().len(), 1);
        assert_eq!(access_facts[0].source_event_ids()[0].as_str(), new_event_id);
    }

    #[test]
    fn same_tick_workplace_notice_tie_uses_source_event_id_on_no_human_surface() {
        let actor_id = actor_id();
        let workshop = place_id("workshop_tomas");
        let lower_event_id = "event.role_notice.actor_tomas.same_tick.closed";
        let higher_event_id = "event.role_notice.actor_tomas.same_tick.open";
        let mut log = EventLog::new();
        for (event_id, access_open) in [(lower_event_id, "false"), (higher_event_id, "true")] {
            log.append(event(
                event_id,
                &actor_id,
                EventKind::RoleAssignmentNoticeRecorded,
                SimTick::new(9),
                "role_assignment_notice",
                vec![
                    PayloadField::new("schema_version", EVENT_SCHEMA_V1),
                    PayloadField::new("actor_id", actor_id.as_str()),
                    PayloadField::new("workplace_id", workplace_id().as_str()),
                    PayloadField::new("place_id", workshop.as_str()),
                    PayloadField::new("access_open", access_open),
                ],
            ))
            .unwrap();
        }
        let projection = projection_from_log(&log);

        let surface = build_surface_at(
            &projection,
            actor_id,
            workshop,
            SimTick::new(10),
            Some(EventId::new(higher_event_id).unwrap()),
        );
        let access_fact = surface
            .context()
            .actor_known_facts()
            .iter()
            .find(|fact| fact.stable_id() == "workplace_believed_accessible")
            .expect("superseded workplace notice should mint one access fact");

        assert_eq!(
            access_fact.value(),
            format!("{}:true", workplace_id().as_str())
        );
        assert_eq!(access_fact.source_event_ids()[0].as_str(), higher_event_id);
    }

    #[test]
    fn workplace_presence_facts_cite_current_place_observation_not_role_notice() {
        let actor_id = actor_id();
        let workshop = place_id("workshop_tomas");
        let notice_event_id = "event.role_notice.actor_tomas";
        let observation_event_id = "event.observation.actor_tomas.workshop";
        let mut log = EventLog::new();
        log.append(event(
            notice_event_id,
            &actor_id,
            EventKind::RoleAssignmentNoticeRecorded,
            SimTick::ZERO,
            "role_assignment_notice",
            vec![
                PayloadField::new("schema_version", EVENT_SCHEMA_V1),
                PayloadField::new("actor_id", actor_id.as_str()),
                PayloadField::new("workplace_id", workplace_id().as_str()),
                PayloadField::new("place_id", workshop.as_str()),
                PayloadField::new("access_open", "true"),
            ],
        ))
        .unwrap();
        log.append(event(
            observation_event_id,
            &actor_id,
            EventKind::ObservationRecorded,
            SimTick::ZERO,
            "record_observation",
            vec![
                PayloadField::new("schema_version", EVENT_SCHEMA_V1),
                PayloadField::new("actor_id", actor_id.as_str()),
                PayloadField::new("observer_actor_id", actor_id.as_str()),
                PayloadField::new("observer_place_id", workshop.as_str()),
                PayloadField::new("observation_id", "observation.actor_tomas.workshop"),
                PayloadField::new("observed_tick", "0"),
                PayloadField::new("source_event_id", observation_event_id),
                PayloadField::new("channel", "direct_sight"),
                PayloadField::new("place_id", workshop.as_str()),
                PayloadField::new("confidence", "1000"),
                PayloadField::new("perceived_kind", "current_place"),
                PayloadField::new("subject_id", actor_id.as_str()),
                PayloadField::new("target_id", workshop.as_str()),
            ],
        ))
        .unwrap();
        let projection = projection_from_log(&log);
        let agent_state = AgentState::default();

        let surface =
            NoHumanActorKnownSurfaceBuilder::from_projection(NoHumanActorKnownSurfaceRequest {
                projection: &projection,
                agent_state: &agent_state,
                actor_id,
                current_place_id: workshop,
                decision_tick: SimTick::ZERO,
                window_id: "morning",
                window_end_tick: SimTick::new(4),
                current_place_witness_event_id: Some(EventId::new(observation_event_id).unwrap()),
                needs_witness_event_id: None,
                frame_event_id: Some(EventId::new(notice_event_id).unwrap()),
            })
            .build(&agent_state);

        for stable_id in [
            "actor_at_workplace",
            "assigned_workplace_known",
            "at_workplace",
        ] {
            let fact = surface
                .context()
                .actor_known_facts()
                .iter()
                .find(|fact| fact.stable_id() == stable_id)
                .expect("same-place workplace fact should be minted from current-place witness");
            assert_eq!(fact.semantic_kind(), "observed_now");
            assert_eq!(fact.source_event_ids().len(), 1);
            assert_eq!(fact.source_event_ids()[0].as_str(), observation_event_id);
        }
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

    #[test]
    fn aged_food_record_surfaces_as_remembered_belief_not_observation() {
        let actor_id = actor_id();
        let kitchen = place_id("kitchen");
        let event_id = "event.visible_food.actor_tomas.old";
        let mut log = EventLog::new();
        log.append(event(
            event_id,
            &actor_id,
            EventKind::ObservationRecorded,
            SimTick::new(4),
            "record_observation",
            vec![
                PayloadField::new("schema_version", EVENT_SCHEMA_V1),
                PayloadField::new("actor_id", actor_id.as_str()),
                PayloadField::new("observer_actor_id", actor_id.as_str()),
                PayloadField::new("observer_place_id", kitchen.as_str()),
                PayloadField::new("observation_id", "observation.visible_food.actor_tomas.old"),
                PayloadField::new("observed_tick", "4"),
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
        let agent_state = AgentState::default();

        let surface =
            NoHumanActorKnownSurfaceBuilder::from_projection(NoHumanActorKnownSurfaceRequest {
                projection: &projection,
                agent_state: &agent_state,
                actor_id,
                current_place_id: kitchen,
                decision_tick: SimTick::new(9),
                window_id: "morning",
                window_end_tick: SimTick::new(12),
                current_place_witness_event_id: Some(EventId::new(event_id).unwrap()),
                needs_witness_event_id: None,
                frame_event_id: Some(EventId::new(event_id).unwrap()),
            })
            .build(&agent_state);

        assert!(surface
            .context()
            .known_food_sources()
            .contains("visible_meal"));
        let food_fact = surface
            .context()
            .actor_known_facts()
            .iter()
            .find(|fact| fact.stable_id() == "actor_knows_food_source")
            .expect("stale food observation remains actor-known memory");
        assert_eq!(food_fact.semantic_kind(), "remembered_belief");
        assert_eq!(food_fact.tick(), Some(SimTick::new(4)));
        assert!(food_fact
            .proof_note()
            .contains("remembered_belief:evented_perception:visible_food_supply"));
    }

    #[test]
    fn food_record_from_other_place_surfaces_as_remembered_find_food_input() {
        let actor_id = actor_id();
        let kitchen = place_id("kitchen");
        let square = place_id("village_square");
        let event_id = "event.visible_food.actor_tomas.kitchen";
        let mut log = EventLog::new();
        log.append(event(
            event_id,
            &actor_id,
            EventKind::ObservationRecorded,
            SimTick::new(4),
            "record_observation",
            vec![
                PayloadField::new("schema_version", EVENT_SCHEMA_V1),
                PayloadField::new("actor_id", actor_id.as_str()),
                PayloadField::new("observer_actor_id", actor_id.as_str()),
                PayloadField::new("observer_place_id", kitchen.as_str()),
                PayloadField::new(
                    "observation_id",
                    "observation.visible_food.actor_tomas.kitchen",
                ),
                PayloadField::new("observed_tick", "4"),
                PayloadField::new("source_event_id", event_id),
                PayloadField::new("channel", "direct_sight"),
                PayloadField::new("place_id", kitchen.as_str()),
                PayloadField::new("confidence", "1000"),
                PayloadField::new("perceived_kind", "visible_food_supply"),
                PayloadField::new("subject_id", kitchen.as_str()),
                PayloadField::new("target_id", "kitchen_stew"),
            ],
        ))
        .unwrap();
        let projection = projection_from_log(&log);

        let surface = build_surface_at(
            &projection,
            actor_id,
            square,
            SimTick::new(9),
            Some(EventId::new(event_id).unwrap()),
        );
        let context = surface.context();
        let food_fact = context
            .actor_known_facts()
            .iter()
            .find(|fact| fact.stable_id() == "actor_knows_food_source")
            .expect("remembered food source should survive after actor moves");

        assert!(context.known_food_sources().contains("kitchen_stew"));
        assert_eq!(food_fact.semantic_kind(), "remembered_belief");
        assert_eq!(food_fact.tick(), Some(SimTick::new(4)));
        assert!(matches!(
            resolve_condition(
                &RoutineCondition::ActorKnowsFoodSource,
                context,
                context.actor_known_facts()
            ),
            ConditionResolution::Satisfied { .. }
        ));
    }

    #[test]
    fn sleep_record_from_other_place_surfaces_as_remembered_reachable_sleep_input() {
        let actor_id = actor_id();
        let kitchen = place_id("kitchen");
        let square = place_id("village_square");
        let sleep_event_id = "event.visible_sleep.actor_tomas.kitchen";
        let route_event_id = "event.visible_exit.actor_tomas.square_to_kitchen";
        let mut log = EventLog::new();
        log.append(event(
            sleep_event_id,
            &actor_id,
            EventKind::ObservationRecorded,
            SimTick::new(4),
            "record_observation",
            vec![
                PayloadField::new("schema_version", EVENT_SCHEMA_V1),
                PayloadField::new("actor_id", actor_id.as_str()),
                PayloadField::new("observer_actor_id", actor_id.as_str()),
                PayloadField::new("observer_place_id", kitchen.as_str()),
                PayloadField::new(
                    "observation_id",
                    "observation.visible_sleep.actor_tomas.kitchen",
                ),
                PayloadField::new("observed_tick", "4"),
                PayloadField::new("source_event_id", sleep_event_id),
                PayloadField::new("channel", "direct_sight"),
                PayloadField::new("place_id", kitchen.as_str()),
                PayloadField::new("confidence", "1000"),
                PayloadField::new("perceived_kind", "visible_sleep_affordance"),
                PayloadField::new("subject_id", kitchen.as_str()),
                PayloadField::new("target_id", "bed_tomas"),
            ],
        ))
        .unwrap();
        log.append(event(
            route_event_id,
            &actor_id,
            EventKind::ObservationRecorded,
            SimTick::new(5),
            "record_observation",
            vec![
                PayloadField::new("schema_version", EVENT_SCHEMA_V1),
                PayloadField::new("actor_id", actor_id.as_str()),
                PayloadField::new("observer_actor_id", actor_id.as_str()),
                PayloadField::new("observer_place_id", square.as_str()),
                PayloadField::new(
                    "observation_id",
                    "observation.visible_exit.actor_tomas.square_to_kitchen",
                ),
                PayloadField::new("observed_tick", "5"),
                PayloadField::new("source_event_id", route_event_id),
                PayloadField::new("channel", "direct_sight"),
                PayloadField::new("place_id", square.as_str()),
                PayloadField::new("confidence", "1000"),
                PayloadField::new("perceived_kind", "visible_exit"),
                PayloadField::new("subject_id", square.as_str()),
                PayloadField::new("target_id", kitchen.as_str()),
            ],
        ))
        .unwrap();
        let projection = projection_from_log(&log);

        let surface = build_surface_at(
            &projection,
            actor_id,
            square,
            SimTick::new(9),
            Some(EventId::new(route_event_id).unwrap()),
        );
        let context = surface.context();
        let sleep_fact = context
            .actor_known_facts()
            .iter()
            .find(|fact| fact.stable_id() == "actor_knows_sleep_place")
            .expect("remembered sleep place should survive after actor moves");

        assert!(context.known_sleep_places().contains(&kitchen));
        assert_eq!(sleep_fact.semantic_kind(), "remembered_belief");
        assert_eq!(sleep_fact.tick(), Some(SimTick::new(4)));
        for condition in [
            RoutineCondition::ActorKnowsSleepPlace,
            RoutineCondition::SleepPlaceBelievedAccessible,
        ] {
            assert!(matches!(
                resolve_condition(&condition, context, context.actor_known_facts()),
                ConditionResolution::Satisfied { .. }
            ));
        }

        let plan = plan_local_actions(
            context,
            &LocalPlanRequest {
                routine_step: RoutineStep::MoveTowardPlace {
                    action_id: SemanticActionId::new("move_toward_place").unwrap(),
                },
                goal: PlannerGoal::ReachPlace(kitchen),
                budget: DEFAULT_PLANNER_BUDGET,
                actor_known_facts: context.actor_known_facts().to_vec(),
            },
        )
        .expect("remembered sleep place and current route should plan movement toward bed");

        assert_eq!(plan.proposals[0].action_id.as_str(), "move");
        assert_eq!(plan.proposals[0].target_ids, ["kitchen".to_string()]);
    }
}
