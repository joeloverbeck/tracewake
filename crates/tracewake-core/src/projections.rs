use crate::actions::{
    validate_proposal, ActionRegistry, Proposal, ProposalOrigin, ProposalSource,
    ProposalSourceContext, ProposalValidationContext, ReasonCode, ReportStatus, ValidationReport,
};
use crate::agent::{BlockerCode, ResponsibleLayer};
use crate::epistemics::contradiction::{Contradiction, ContradictionKind};
use crate::epistemics::proposition::Proposition;
use crate::epistemics::{EpistemicProjection, KnowledgeContext, SourceRef};
use crate::events::log::EventLog;
use crate::events::{EventEnvelope, EventKind};
use crate::ids::{
    ActionId, ActorId, ContentManifestId, ControllerId, EventId, FoodSupplyId,
    HolderKnownContextId, ItemId, PlaceId, ProposalId, SemanticActionId, SleepAffordanceId,
    ViewModelId, WorkplaceId,
};
use crate::location::{visible_locality, Location};
use crate::scheduler::{OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId};
use crate::state::AgentState;
use crate::state::{ActorBody, DoorState, PhysicalState};
use crate::time::SimTick;
use crate::view_models::{
    ActionAvailability, ActionAvailabilityProvenance, ActionAvailabilityProvenanceKind,
    DebugEventLogView, DebugEventSummary, EmbodiedViewModel, NeedStatusEntry, NotebookBeliefEntry,
    NotebookContradictionEntry, NotebookLeadEntry, NotebookObservationEntry, NotebookView,
    Phase3AEmbodiedStatus, SemanticActionEntry, ViewMode, VisibleActor, VisibleContainer,
    VisibleDoor, VisibleExit, VisibleItem, VisibleItemSource, WhyNotView,
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ActorKnownWorkplaceSurface {
    workplace_id: WorkplaceId,
    place_id: PlaceId,
    believed_access_open: bool,
    source_event_ids: Vec<EventId>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ActorKnownFoodSourceSurface {
    food_supply_id: FoodSupplyId,
    believed_servings: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProjectionError {
    ActorNotFound(ActorId),
    PlaceNotFound(PlaceId),
}

pub struct EmbodiedProjectionSource<'a> {
    state: &'a PhysicalState,
    agent_state: Option<&'a AgentState>,
    knowledge_context_id: HolderKnownContextId,
    actor_known_food_sources: Vec<ActorKnownFoodSourceSurface>,
    actor_known_sleep_affordances: Vec<SleepAffordanceId>,
    actor_known_routes: Vec<PlaceId>,
    actor_known_workplaces: Vec<ActorKnownWorkplaceSurface>,
}

impl<'a> EmbodiedProjectionSource<'a> {
    pub fn from_sealed_context(
        context: &'a KnowledgeContext,
        state: &'a PhysicalState,
        agent_state: Option<&'a AgentState>,
    ) -> Self {
        let current_place_id = state
            .actors
            .get(context.viewer_actor_id())
            .map(|actor| actor.current_place_id.clone());
        let actor_known_food_sources = actor_known_food_sources_for_context(context);
        let actor_known_sleep_affordances = actor_known_sleep_affordances_for_context(context);
        let actor_known_routes = actor_known_routes_for_context(context, current_place_id.as_ref());
        let actor_known_workplaces = actor_known_workplaces_for_context(context);
        Self {
            state,
            agent_state,
            knowledge_context_id: context.holder_known_context_id().clone(),
            actor_known_food_sources,
            actor_known_sleep_affordances,
            actor_known_routes,
            actor_known_workplaces,
        }
    }
}

fn actor_known_food_sources_for_context(
    context: &KnowledgeContext,
) -> Vec<ActorKnownFoodSourceSurface> {
    let mut food_sources = context
        .actor_known_food_sources()
        .iter()
        .map(|fact| ActorKnownFoodSourceSurface {
            food_supply_id: fact.food_supply_id().clone(),
            believed_servings: fact.believed_servings(),
        })
        .collect::<Vec<_>>();
    food_sources.sort();
    food_sources.dedup();
    food_sources
}

fn actor_known_sleep_affordances_for_context(context: &KnowledgeContext) -> Vec<SleepAffordanceId> {
    let mut sleep_affordances = context
        .actor_known_sleep_affordances()
        .iter()
        .map(|fact| fact.sleep_affordance_id().clone())
        .collect::<Vec<_>>();
    sleep_affordances.sort();
    sleep_affordances.dedup();
    sleep_affordances
}

fn actor_known_routes_for_context(
    context: &KnowledgeContext,
    current_place_id: Option<&PlaceId>,
) -> Vec<PlaceId> {
    let mut routes = context
        .actor_known_routes()
        .iter()
        .filter(|fact| current_place_id.is_some_and(|place_id| fact.from_place_id() == place_id))
        .map(|fact| fact.to_place_id().clone())
        .collect::<Vec<_>>();
    routes.sort();
    routes.dedup();
    routes
}

fn actor_known_workplaces_for_context(
    context: &KnowledgeContext,
) -> Vec<ActorKnownWorkplaceSurface> {
    let mut workplaces = context
        .actor_known_workplaces()
        .iter()
        .map(|fact| ActorKnownWorkplaceSurface {
            workplace_id: fact.workplace_id().clone(),
            place_id: fact.place_id().clone(),
            believed_access_open: fact.believed_access_open(),
            source_event_ids: fact.source_event_ids().as_slice().to_vec(),
        })
        .collect::<Vec<_>>();
    workplaces.sort();
    workplaces.dedup();
    workplaces
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NoHumanDayMetrics {
    pub projection_version: String,
    pub events_per_day: usize,
    pub routine_event_count: usize,
    pub meals_completed: usize,
    pub meals_missed: usize,
    pub sleep_completed: usize,
    pub sleep_interrupted: usize,
    pub work_blocks_completed: usize,
    pub work_blocks_failed: usize,
    pub need_threshold_crossings: usize,
    pub routine_interruptions: usize,
    pub planner_failures: usize,
    pub stuck_actor_count: usize,
    pub run_duration_ticks: u64,
    pub replay_failure_count: usize,
    pub tui_action_coverage: usize,
    pub player_conditioned_event_count: usize,
    pub player_conditioned_event_rate_per_1000: u64,
}

impl NoHumanDayMetrics {
    pub fn serialize_canonical(&self) -> String {
        format!(
            "no_human_day_metrics_v1|events={}|routine_events={}|meals_completed={}|meals_missed={}|sleep_completed={}|sleep_interrupted={}|work_completed={}|work_failed={}|need_crossings={}|routine_interruptions={}|planner_failures={}|stuck_actors={}|run_duration_ticks={}|replay_failures={}|tui_action_coverage={}|player_conditioned_events={}|player_conditioned_rate_per_1000={}",
            self.events_per_day,
            self.routine_event_count,
            self.meals_completed,
            self.meals_missed,
            self.sleep_completed,
            self.sleep_interrupted,
            self.work_blocks_completed,
            self.work_blocks_failed,
            self.need_threshold_crossings,
            self.routine_interruptions,
            self.planner_failures,
            self.stuck_actor_count,
            self.run_duration_ticks,
            self.replay_failure_count,
            self.tui_action_coverage,
            self.player_conditioned_event_count,
            self.player_conditioned_event_rate_per_1000
        )
    }
}

pub fn no_human_day_metrics(log: &EventLog) -> NoHumanDayMetrics {
    let events = log.events();
    let player_conditioned_event_count = events
        .iter()
        .filter(|event| contains_player_conditioned_fact(event))
        .count();
    let player_conditioned_event_rate_per_1000 = if events.is_empty() {
        0
    } else {
        (player_conditioned_event_count as u64 * 1000) / events.len() as u64
    };
    let start_tick = events
        .iter()
        .find(|event| event.event_type == EventKind::NoHumanDayStarted)
        .map(|event| event.sim_tick)
        .unwrap_or(SimTick::ZERO);
    let end_tick = events
        .iter()
        .rev()
        .find(|event| event.event_type == EventKind::NoHumanDayCompleted)
        .map(|event| event.sim_tick)
        .unwrap_or(start_tick);
    let mut stuck_actor_ids = events
        .iter()
        .filter(|event| event.event_type == EventKind::StuckDiagnosticRecorded)
        .filter_map(|event| event.actor_id.clone())
        .collect::<Vec<_>>();
    stuck_actor_ids.sort();
    stuck_actor_ids.dedup();

    NoHumanDayMetrics {
        projection_version: "no_human_day_metrics_v1".to_string(),
        events_per_day: events.len(),
        routine_event_count: events
            .iter()
            .filter(|event| is_routine_event(event.event_type))
            .count(),
        meals_completed: count_kind(events, EventKind::FoodConsumed),
        meals_missed: count_kind(events, EventKind::EatFailed),
        sleep_completed: count_kind(events, EventKind::SleepCompleted),
        sleep_interrupted: count_kind(events, EventKind::SleepInterrupted),
        work_blocks_completed: count_kind(events, EventKind::WorkBlockCompleted),
        work_blocks_failed: count_kind(events, EventKind::WorkBlockFailed),
        need_threshold_crossings: count_kind(events, EventKind::NeedThresholdCrossed),
        routine_interruptions: count_kind(events, EventKind::IntentionInterrupted)
            + count_kind(events, EventKind::RoutineStepFailed),
        planner_failures: events
            .iter()
            .filter(|event| is_typed_planner_failure_event(event))
            .count(),
        stuck_actor_count: stuck_actor_ids.len(),
        run_duration_ticks: end_tick.value().saturating_sub(start_tick.value()),
        replay_failure_count: events
            .iter()
            .filter(|event| {
                event.event_type == EventKind::ReplayProjectionRebuilt
                    && event
                        .payload
                        .iter()
                        .any(|field| field.value.contains("failure"))
            })
            .count(),
        tui_action_coverage: unique_action_count(events),
        player_conditioned_event_count,
        player_conditioned_event_rate_per_1000,
    }
}

fn count_kind(events: &[EventEnvelope], kind: EventKind) -> usize {
    events
        .iter()
        .filter(|event| event.event_type == kind)
        .count()
}

fn is_typed_planner_failure_event(event: &EventEnvelope) -> bool {
    if !matches!(
        event.event_type,
        EventKind::DecisionTraceRecorded | EventKind::StuckDiagnosticRecorded
    ) {
        return false;
    }
    let responsible_layer = typed_responsible_layer(event);
    let blocker_code = typed_blocker_code(event);
    matches!(responsible_layer, Some(ResponsibleLayer::LocalPlanning))
        || matches!(
            blocker_code,
            Some(
                BlockerCode::PlannerBudgetExhausted
                    | BlockerCode::EmptyLocalPlan
                    | BlockerCode::LocalPlanFailed
            )
        )
}

fn typed_responsible_layer(event: &EventEnvelope) -> Option<ResponsibleLayer> {
    payload_value(event, "responsible_layer").and_then(|value| ResponsibleLayer::parse(value).ok())
}

fn typed_blocker_code(event: &EventEnvelope) -> Option<BlockerCode> {
    payload_value(event, "blocker_code").and_then(|value| BlockerCode::parse(value).ok())
}

fn payload_value<'a>(event: &'a EventEnvelope, key: &str) -> Option<&'a str> {
    event
        .payload
        .iter()
        .find(|field| field.key == key)
        .map(|field| field.value.as_str())
}

fn is_routine_event(kind: EventKind) -> bool {
    matches!(
        kind,
        EventKind::RoutineStepStarted
            | EventKind::RoutineStepCompleted
            | EventKind::RoutineStepFailed
            | EventKind::ContinueRoutineAccepted
            | EventKind::ContinueRoutineRejected
    )
}

fn unique_action_count(events: &[EventEnvelope]) -> usize {
    let mut action_ids = events
        .iter()
        .map(|event| event.ordering_key.action_id.clone())
        .collect::<Vec<_>>();
    action_ids.sort();
    action_ids.dedup();
    action_ids.len()
}

fn contains_player_conditioned_fact(event: &EventEnvelope) -> bool {
    event.participants.iter().any(|value| is_player_term(value))
        || event
            .payload
            .iter()
            .any(|field| is_player_term(&field.key) || is_player_term(&field.value))
        || is_player_term(&event.effects_summary)
}

fn is_player_term(value: &str) -> bool {
    let lowered = value.to_ascii_lowercase();
    lowered.contains("player") || lowered.contains("controller")
}

pub fn build_embodied_view_model(
    context: &KnowledgeContext,
    source: &EmbodiedProjectionSource<'_>,
    registry: &ActionRegistry,
    content_manifest_id: &ContentManifestId,
    last_rejection: Option<&ValidationReport>,
) -> Result<EmbodiedViewModel, ProjectionError> {
    let viewer_actor_id = context.viewer_actor_id();
    let sim_tick = context.current_tick();
    let state = source.state;
    let agent_state = source.agent_state;
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

    let mut visible_exits = source
        .actor_known_routes
        .iter()
        .cloned()
        .map(|destination_place_id| VisibleExit {
            blocker_summary: visible_exit_blocker_summary(
                state,
                &visible.connected_doors,
                &actor.current_place_id,
                &destination_place_id,
            ),
            destination_place_id,
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
            source: visible_item_source(&item.location),
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
            source: visible_item_source(&item.location),
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

    let fallback_agent_state = AgentState::default();
    let preflight_context = SemanticActionPreflightContext {
        state,
        agent_state: agent_state.unwrap_or(&fallback_agent_state),
        registry,
        content_manifest_id,
        viewer_actor_id,
        sim_tick,
        knowledge_context_id: context.holder_known_context_id().clone(),
        knowledge_context_frontier: context.event_frontier(),
    };
    let mut semantic_actions = semantic_actions(
        &preflight_context,
        &visible_exits,
        &visible_doors,
        &visible_containers,
        &visible_items,
        &carried_item_ids,
    );
    semantic_actions.extend(phase3a_semantic_actions(
        state,
        agent_state,
        source,
        actor,
        viewer_actor_id,
    ));
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
        phase3a_status: agent_state.map(|agent_state| phase3a_status(agent_state, viewer_actor_id)),
        last_rejection_summary: last_rejection.map(|report| report.actor_visible_summary.clone()),
        last_rejection_why_not: last_rejection.map(WhyNotView::from),
        holder_known_context_id: context.holder_known_context_id().clone(),
        holder_known_context_hash: context.holder_known_context_hash().clone(),
        holder_known_context_frontier: context.event_frontier(),
        holder_known_context_source_summary: format!(
            "allowed={} provenance={}",
            context.allowed_sources().len(),
            context.provenance_entries().len()
        ),
        notebook: None,
        debug_available: true,
    })
}

pub fn build_notebook_view(
    projection: &EpistemicProjection,
    context: &KnowledgeContext,
) -> NotebookView {
    let mut beliefs = projection
        .beliefs_for_context(context)
        .into_iter()
        .map(|belief| NotebookBeliefEntry {
            belief_id: belief.belief_id().as_str().to_string(),
            summary: belief.proposition().render(),
            source_summary: source_summary(belief.source()),
            confidence_label: belief.confidence().serialize_canonical(),
            acquired_tick: belief.acquired_tick().value(),
            contradiction_ids: belief
                .contradiction_ids()
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
            observation_id: observation.observation_id().as_str().to_string(),
            channel: observation.channel().stable_id().to_string(),
            summary: format!("{} observation", observation.channel().stable_id()),
            confidence_label: observation.confidence().serialize_canonical(),
            observed_tick: observation.observed_tick().value(),
        })
        .collect::<Vec<_>>();
    observations.sort();

    let mut contradictions = projection
        .contradictions_for_context(context)
        .into_iter()
        .map(|contradiction| NotebookContradictionEntry {
            contradiction_id: contradiction.contradiction_id().as_str().to_string(),
            summary: "Contradicts your earlier expectation.".to_string(),
        })
        .collect::<Vec<_>>();
    contradictions.sort();

    let mut typed_leads = projection
        .contradictions_for_context(context)
        .into_iter()
        .filter_map(|contradiction| typed_notebook_lead(projection, context, contradiction))
        .collect::<Vec<_>>();
    typed_leads.sort();
    let possible_leads = typed_leads
        .iter()
        .map(|lead| lead.summary.clone())
        .collect();

    NotebookView {
        viewer_actor_id: context.viewer_actor_id().clone(),
        source_bound_beliefs: beliefs,
        recent_observations: observations,
        known_contradictions: contradictions,
        typed_leads,
        possible_leads,
    }
}

fn typed_notebook_lead(
    projection: &EpistemicProjection,
    context: &KnowledgeContext,
    contradiction: &Contradiction,
) -> Option<NotebookLeadEntry> {
    if contradiction.kind() != ContradictionKind::ExpectedItemAbsentFromContainer {
        return None;
    }
    let Proposition::ItemMissingFromExpectedLocation {
        item_id,
        expected_location,
    } = &contradiction.observed_proposition()
    else {
        return None;
    };
    let observation = projection.observation(contradiction.contradicting_observation_id())?;
    let possible_next_actions = match expected_location {
        Location::InContainer(container_id) => {
            vec![format!("check.container.{}", container_id.as_str())]
        }
        Location::AtPlace(place_id) => vec![format!("inspect.place.{}", place_id.as_str())],
        Location::CarriedBy(actor_id) => vec![format!("ask.actor.{}", actor_id.as_str())],
    };

    Some(NotebookLeadEntry {
        lead_id: format!("lead.{}", contradiction.contradiction_id().as_str()),
        contradiction_id: contradiction.contradiction_id().as_str().to_string(),
        belief_id: contradiction
            .prior_expectation_belief_id()
            .as_str()
            .to_string(),
        observation_id: observation.observation_id().as_str().to_string(),
        source_kind: source_kind(observation.source()).to_string(),
        source_summary: source_summary(observation.source()),
        confidence_label: observation.confidence().serialize_canonical(),
        detected_tick: contradiction.detected_tick().value(),
        staleness_label: staleness_label(context, contradiction.detected_tick()),
        how_this_may_be_wrong: "The item may have moved through an unobserved ordinary event."
            .to_string(),
        possible_next_actions,
        summary: format!(
            "Source-bound lead from {}: {} missing from expected location",
            contradiction.contradiction_id().as_str(),
            item_id.as_str()
        ),
    })
}

fn source_kind(source: &SourceRef) -> &'static str {
    match source {
        SourceRef::Event(_) => "event",
        SourceRef::Action(_) => "action",
        SourceRef::Cause(_) => "cause",
    }
}

fn source_summary(source: &SourceRef) -> String {
    match source {
        SourceRef::Event(event_id) => format!("event:{}", event_id.as_str()),
        SourceRef::Action(action_id) => format!("action:{}", action_id.as_str()),
        SourceRef::Cause(cause) => format!("cause:{cause:?}"),
    }
}

fn staleness_label(context: &KnowledgeContext, detected_tick: SimTick) -> String {
    let elapsed = context
        .current_tick()
        .value()
        .saturating_sub(detected_tick.value());
    if elapsed == 0 {
        "current_tick".to_string()
    } else {
        format!("{} ticks old", elapsed)
    }
}

fn phase3a_salient_interruption(
    agent_state: &AgentState,
    viewer_actor_id: &ActorId,
) -> Option<String> {
    agent_state
        .ordinary_life_episodes()
        .values()
        .filter(|episode| episode.actor_id.as_ref() == Some(viewer_actor_id))
        .filter(|episode| {
            matches!(
                episode.event_kind.as_str(),
                "sleep_interrupted" | "work_block_failed"
            )
        })
        .max_by(|left, right| {
            left.sim_tick
                .cmp(&right.sim_tick)
                .then_with(|| left.event_id.cmp(&right.event_id))
        })
        .map(|episode| {
            format!(
                "{} at tick {}: {}",
                episode.event_kind,
                episode.sim_tick.value(),
                episode.summary
            )
        })
}

fn phase3a_status(agent_state: &AgentState, viewer_actor_id: &ActorId) -> Phase3AEmbodiedStatus {
    let mut need_summaries = agent_state
        .needs_by_actor
        .get(viewer_actor_id)
        .into_iter()
        .flat_map(|needs| needs.values())
        .map(|need| NeedStatusEntry {
            kind: need.kind().stable_id().to_string(),
            band_label: need.band().stable_id().to_string(),
            last_cause: need.last_change_cause().stable_id().to_string(),
        })
        .collect::<Vec<_>>();
    need_summaries.sort();

    let intention = agent_state
        .active_intention_by_actor
        .get(viewer_actor_id)
        .and_then(|intention_id| agent_state.intentions.get(intention_id));
    let intention_summary = intention.map(|intention| {
        format!(
            "active:{}:{}",
            intention
                .selected_routine_method
                .as_ref()
                .map(|id| id.as_str())
                .unwrap_or("routine_unknown"),
            intention.current_step.as_deref().unwrap_or("step_pending")
        )
    });
    let routine_summary = intention
        .and_then(|intention| intention.selected_routine_method.as_ref())
        .map(|template_id| format!("routine:{}", template_id.as_str()));

    Phase3AEmbodiedStatus {
        need_summaries,
        intention_summary,
        routine_summary,
        salient_interruption: phase3a_salient_interruption(agent_state, viewer_actor_id),
    }
}

fn phase3a_semantic_actions(
    _state: &PhysicalState,
    agent_state: Option<&AgentState>,
    source: &EmbodiedProjectionSource<'_>,
    actor: &ActorBody,
    viewer_actor_id: &ActorId,
) -> Vec<SemanticActionEntry> {
    let mut actions = Vec::new();
    if let Some(sleep_affordance_id) = visible_open_sleep_affordance(source) {
        actions.push(SemanticActionEntry::with_availability(
            SemanticActionId::new("sleep.here").unwrap(),
            ActionId::new("sleep").unwrap(),
            vec![sleep_affordance_id.to_string()],
            "Sleep here",
            ActionAvailability::available(),
        ));
    }

    for food_source in &source.actor_known_food_sources {
        let availability = match food_source.believed_servings {
            Some(0) => ActionAvailability::disabled(
                vec![ReasonCode::KnowledgePreconditionNotMet],
                "You know that food source is empty.",
                vec![ActionAvailabilityProvenance::new(
                    ActionAvailabilityProvenanceKind::HolderKnownContext,
                    source.knowledge_context_id.as_str(),
                )],
                Vec::new(),
            ),
            _ => ActionAvailability::available(),
        };
        actions.push(SemanticActionEntry::with_availability(
            SemanticActionId::new(format!("eat.food.{}", food_source.food_supply_id.as_str()))
                .unwrap(),
            ActionId::new("eat").unwrap(),
            vec![food_source.food_supply_id.to_string()],
            format!("Eat {}", food_source.food_supply_id.as_str()),
            availability,
        ));
    }

    for workplace in &source.actor_known_workplaces {
        let at_workplace = workplace.place_id == actor.current_place_id;
        let provenance_refs = workplace_availability_provenance_refs(
            &source.knowledge_context_id,
            &workplace.source_event_ids,
        );
        let availability = if !at_workplace {
            ActionAvailability::disabled(
                vec![ReasonCode::ActorNotAtRequiredPlace],
                "You are not at that workplace.",
                provenance_refs,
                Vec::new(),
            )
        } else if !workplace.believed_access_open {
            ActionAvailability::disabled(
                vec![ReasonCode::KnowledgePreconditionNotMet],
                "You know that workplace access is closed.",
                provenance_refs,
                Vec::new(),
            )
        } else {
            ActionAvailability::available()
        };
        actions.push(SemanticActionEntry::with_availability(
            SemanticActionId::new(format!("work.block.{}", workplace.workplace_id.as_str()))
                .unwrap(),
            ActionId::new("work_block").unwrap(),
            vec![workplace.workplace_id.to_string()],
            format!("Work at {}", workplace.workplace_id.as_str()),
            availability,
        ));
    }

    if let Some((intention_id, intention)) = agent_state.and_then(|agent_state| {
        agent_state
            .active_intention_by_actor
            .get(viewer_actor_id)
            .and_then(|intention_id| {
                agent_state
                    .intentions
                    .get(intention_id)
                    .map(|intention| (intention_id, intention))
            })
    }) {
        actions.push(SemanticActionEntry::with_availability(
            SemanticActionId::new(format!("continue.routine.{}", intention_id.as_str())).unwrap(),
            ActionId::new("continue_routine").unwrap(),
            vec![
                intention_id.to_string(),
                intention
                    .current_step
                    .clone()
                    .unwrap_or_else(|| "wait".to_string()),
            ],
            "Continue routine",
            ActionAvailability::available(),
        ));
    }

    actions
}

fn workplace_availability_provenance_refs(
    context_id: &HolderKnownContextId,
    source_event_ids: &[EventId],
) -> Vec<ActionAvailabilityProvenance> {
    let mut refs = vec![ActionAvailabilityProvenance::new(
        ActionAvailabilityProvenanceKind::HolderKnownContext,
        context_id.as_str(),
    )];
    refs.extend(source_event_ids.iter().map(|event_id| {
        ActionAvailabilityProvenance::new(
            ActionAvailabilityProvenanceKind::SourceEvent,
            event_id.as_str(),
        )
    }));
    refs
}

pub fn build_debug_event_log_view(log: &EventLog) -> DebugEventLogView {
    DebugEventLogView::new(log.events().iter().map(DebugEventSummary::from).collect())
}

fn visible_item_source(location: &Location) -> VisibleItemSource {
    match location {
        Location::AtPlace(_) => VisibleItemSource::Place,
        Location::InContainer(container_id) => VisibleItemSource::Container(container_id.clone()),
        Location::CarriedBy(_) => VisibleItemSource::Carried,
    }
}

fn visible_exit_blocker_summary(
    state: &PhysicalState,
    connected_doors: &std::collections::BTreeSet<crate::ids::DoorId>,
    from_place_id: &PlaceId,
    destination_place_id: &PlaceId,
) -> Option<String> {
    connected_doors
        .iter()
        .filter_map(|door_id| state.doors.get(door_id))
        .find(|door| door_connects_edge(door, from_place_id, destination_place_id))
        .and_then(|door| {
            if door.is_locked && !door.is_open {
                Some(format!(
                    "door {} is closed and locked",
                    door.door_id.as_str()
                ))
            } else if door.is_locked {
                Some(format!("door {} is locked", door.door_id.as_str()))
            } else if !door.is_open && door.blocks_movement_when_closed {
                Some(format!("door {} is closed", door.door_id.as_str()))
            } else {
                None
            }
        })
}

fn door_connects_edge(door: &DoorState, from_place_id: &PlaceId, to_place_id: &PlaceId) -> bool {
    (&door.endpoint_a == from_place_id && &door.endpoint_b == to_place_id)
        || (&door.endpoint_b == from_place_id && &door.endpoint_a == to_place_id)
}

#[derive(Clone)]
struct SemanticActionPreflightContext<'a> {
    state: &'a PhysicalState,
    agent_state: &'a AgentState,
    registry: &'a ActionRegistry,
    content_manifest_id: &'a ContentManifestId,
    viewer_actor_id: &'a ActorId,
    sim_tick: SimTick,
    knowledge_context_id: HolderKnownContextId,
    knowledge_context_frontier: u64,
}

fn semantic_actions(
    preflight: &SemanticActionPreflightContext<'_>,
    visible_exits: &[VisibleExit],
    visible_doors: &[VisibleDoor],
    visible_containers: &[VisibleContainer],
    visible_items: &[VisibleItem],
    carried_items: &[ItemId],
) -> Vec<SemanticActionEntry> {
    let mut actions = Vec::new();
    actions.push(with_validator_availability(
        SemanticActionEntry::new(
            SemanticActionId::new("wait.1_tick").unwrap(),
            ActionId::new("wait").unwrap(),
            vec!["1_tick".to_string()],
            "Wait",
            true,
            None,
        ),
        preflight,
    ));

    for exit in visible_exits {
        actions.push(with_validator_availability(
            SemanticActionEntry::new(
                SemanticActionId::new(format!("move.to.{}", exit.destination_place_id.as_str()))
                    .unwrap(),
                ActionId::new("move").unwrap(),
                vec![exit.destination_place_id.to_string()],
                format!("Move to {}", exit.destination_place_id.as_str()),
                true,
                None,
            ),
            preflight,
        ));
    }

    for door in visible_doors {
        let action = if door.is_open { "close" } else { "open" };
        actions.push(with_validator_availability(
            SemanticActionEntry::new(
                SemanticActionId::new(format!("{action}.door.{}", door.door_id.as_str())).unwrap(),
                ActionId::new(action).unwrap(),
                vec![door.door_id.to_string()],
                format!("{action} {}", door.door_id.as_str()),
                true,
                None,
            ),
            preflight,
        ));
    }

    for container in visible_containers {
        actions.push(with_validator_availability(
            SemanticActionEntry::new(
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
            ),
            preflight,
        ));
        let action = if container.is_open { "close" } else { "open" };
        actions.push(with_validator_availability(
            SemanticActionEntry::new(
                SemanticActionId::new(format!(
                    "{action}.container.{}",
                    container.container_id.as_str()
                ))
                .unwrap(),
                ActionId::new(action).unwrap(),
                vec![container.container_id.to_string()],
                format!("{action} {}", container.container_id.as_str()),
                true,
                None,
            ),
            preflight,
        ));
    }

    for item in visible_items {
        let source_label = match &item.source {
            VisibleItemSource::Place => "place".to_string(),
            VisibleItemSource::Container(container_id) => container_id.to_string(),
            VisibleItemSource::Carried => "carried".to_string(),
        };
        actions.push(with_validator_availability(
            SemanticActionEntry::new(
                SemanticActionId::new(format!(
                    "take.item.{}.from.{source_label}",
                    item.item_id.as_str()
                ))
                .unwrap(),
                ActionId::new("take").unwrap(),
                vec![item.item_id.to_string()],
                format!("Take {}", item.item_id.as_str()),
                true,
                None,
            ),
            preflight,
        ));
        actions.push(with_validator_availability(
            SemanticActionEntry::new(
                SemanticActionId::new(format!("inspect.item.{}", item.item_id.as_str())).unwrap(),
                ActionId::new("inspect_entity").unwrap(),
                vec![item.item_id.to_string()],
                format!("Inspect {}", item.item_id.as_str()),
                true,
                None,
            ),
            preflight,
        ));
    }

    for item_id in carried_items {
        actions.push(with_validator_availability(
            SemanticActionEntry::new(
                SemanticActionId::new(format!("place.item.{}.at.place", item_id.as_str())).unwrap(),
                ActionId::new("place").unwrap(),
                vec![item_id.to_string()],
                format!("Place {}", item_id.as_str()),
                true,
                None,
            ),
            preflight,
        ));
    }

    actions
}

fn with_validator_availability(
    entry: SemanticActionEntry,
    preflight: &SemanticActionPreflightContext<'_>,
) -> SemanticActionEntry {
    let proposal = proposal_from_semantic_action_entry(
        ProposalId::new(format!(
            "proposal.preflight.{}",
            entry.semantic_action_id.as_str()
        ))
        .unwrap(),
        ProposalOrigin::Test,
        Some(preflight.viewer_actor_id.clone()),
        preflight.sim_tick,
        &entry,
        None,
        None,
    );
    let ordering_key = OrderingKey::new(
        preflight.sim_tick,
        SchedulePhase::HumanCommand,
        SchedulerSourceId::Actor(preflight.viewer_actor_id.clone()),
        ProposalSequence::new(0),
        proposal.action_id.clone(),
        proposal.target_ids.clone(),
        proposal.proposal_id.as_str().to_string(),
    );
    let report = validate_proposal(
        ProposalValidationContext {
            registry: preflight.registry,
            state: preflight.state,
            agent_state: preflight.agent_state,
            controller_bindings: None,
            epistemic_projection: None,
            content_manifest_id: preflight.content_manifest_id,
            ordering_key: &ordering_key,
            current_event_frontier: preflight.knowledge_context_frontier,
        },
        &proposal,
    );
    let enabled = report.status == ReportStatus::Accepted;
    let availability = if enabled {
        ActionAvailability::available()
    } else {
        let mut provenance_refs = vec![
            ActionAvailabilityProvenance::new(
                ActionAvailabilityProvenanceKind::HolderKnownContext,
                preflight.knowledge_context_id.as_str(),
            ),
            ActionAvailabilityProvenance::new(
                ActionAvailabilityProvenanceKind::ValidationReport,
                report.validation_report_id.as_str(),
            ),
        ];
        provenance_refs.extend(report.checked_facts.iter().map(|fact| {
            ActionAvailabilityProvenance::new(
                ActionAvailabilityProvenanceKind::ValidatorFact,
                fact.render_pair(),
            )
        }));
        ActionAvailability::disabled(
            report.reason_codes.clone(),
            report.actor_visible_summary,
            provenance_refs,
            Vec::new(),
        )
    };
    SemanticActionEntry::with_availability(
        entry.semantic_action_id,
        entry.action_id,
        entry.target_ids,
        entry.label,
        availability,
    )
}

pub fn proposal_from_semantic_action_entry(
    proposal_id: ProposalId,
    origin: ProposalOrigin,
    actor_id: Option<ActorId>,
    requested_tick: SimTick,
    entry: &SemanticActionEntry,
    source_view: Option<&EmbodiedViewModel>,
    controller_id: Option<&ControllerId>,
) -> Proposal {
    assert!(
        origin != ProposalOrigin::Human || source_view.is_some(),
        "human semantic-action proposal construction requires a current embodied view source context"
    );
    let mut proposal = Proposal::new(
        proposal_id,
        origin,
        actor_id,
        entry.action_id.clone(),
        requested_tick,
    );
    proposal.target_ids = entry.target_ids.clone();
    if let Some(view) = source_view {
        proposal.source_view_model_id = Some(view.view_model_id.clone());
        proposal.source = Some(ProposalSource::TuiSemanticAction(ProposalSourceContext {
            source_view_model_id: view.view_model_id.clone(),
            holder_known_context_id: view.holder_known_context_id.clone(),
            holder_known_context_hash: view.holder_known_context_hash.clone(),
            holder_known_context_frontier: view.holder_known_context_frontier,
            context_tick: view.sim_tick,
            actor_id: view.viewer_actor_id.clone(),
            semantic_action_id: entry.semantic_action_id.clone(),
            provenance_ancestry: entry
                .availability
                .provenance_refs()
                .iter()
                .map(|source| format!("{}:{}", source.kind.stable_id(), source.reference))
                .collect(),
        }));
    }
    if let Some(controller_id) = controller_id {
        proposal
            .parameters
            .insert("controller_id".to_string(), controller_id.to_string());
    }
    if entry.semantic_action_id.as_str() == "wait.1_tick" {
        proposal
            .parameters
            .insert("ticks".to_string(), "1".to_string());
        proposal
            .parameters
            .insert("reason".to_string(), "actor selected wait".to_string());
    }
    if entry.action_id.as_str() == "sleep" {
        if let Some(sleep_affordance_id) = entry.target_ids.first() {
            proposal.parameters.insert(
                "sleep_affordance_id".to_string(),
                sleep_affordance_id.clone(),
            );
        }
    }
    if entry.action_id.as_str() == "continue_routine" {
        if let Some(intention_id) = entry.target_ids.first() {
            proposal
                .parameters
                .insert("active_intention_id".to_string(), intention_id.clone());
        }
        if let Some(next_action_id) = entry.target_ids.get(1) {
            proposal
                .parameters
                .insert("next_action_id".to_string(), next_action_id.clone());
        }
        proposal
            .parameters
            .insert("intention_status".to_string(), "active".to_string());
    }
    proposal
}

fn visible_open_sleep_affordance(
    source: &EmbodiedProjectionSource<'_>,
) -> Option<SleepAffordanceId> {
    source.actor_known_sleep_affordances.first().cloned()
}

pub fn proposal_from_current_view_semantic_action(
    proposal_id: ProposalId,
    actor_id: ActorId,
    requested_tick: SimTick,
    entry: &SemanticActionEntry,
    source_view: &EmbodiedViewModel,
    controller_id: &ControllerId,
) -> Proposal {
    proposal_from_semantic_action_entry(
        proposal_id,
        ProposalOrigin::Human,
        Some(actor_id),
        requested_tick,
        entry,
        Some(source_view),
        Some(controller_id),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::{Intention, IntentionSource, NeedChangeCause, NeedKind, NeedState};
    use crate::epistemics::{
        Belief, Channel, Confidence, Contradiction, ContradictionKind, HolderKind, Observation,
        ObservationSubject, ObservationTarget, Proposition, SourceRef, Stance,
    };
    use crate::events::PayloadField;
    use crate::ids::{
        CandidateGoalId, ContainerId, DecisionTraceId, DoorId, FoodSupplyId, IntentionId,
        ProcessId, RoutineTemplateId, SleepAffordanceId, WorkplaceId,
    };
    use crate::state::{
        ActorBody, ContainerState, DoorState, ItemState, OrdinaryLifeEpisodeRecord, PhysicalState,
        PlaceState,
    };
    use crate::state::{AgentState, FoodSupplyState, SleepAffordanceState, WorkplaceState};

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

    fn content_manifest_id() -> ContentManifestId {
        ContentManifestId::new("phase2a_manifest").unwrap()
    }

    fn metric_event(kind: EventKind, sequence: u64, tick: u64) -> EventEnvelope {
        EventEnvelope::new_v1(
            event_id(&format!("event_metrics_{sequence}")),
            kind,
            99,
            99,
            SimTick::new(tick),
            OrderingKey::new(
                SimTick::new(tick),
                SchedulePhase::NoHumanProcess,
                SchedulerSourceId::Process(ProcessId::new("process_metrics").unwrap()),
                ProposalSequence::new(sequence),
                ActionId::new(format!("action_metrics_{sequence}")).unwrap(),
                vec![format!("target_{sequence}")],
                format!("metrics_tie_{sequence}"),
            ),
            content_manifest_id(),
        )
    }

    fn append_metric_event(log: &mut EventLog, kind: EventKind, sequence: u64, tick: u64) {
        log.append(metric_event(kind, sequence, tick)).unwrap();
    }

    fn registry() -> ActionRegistry {
        let mut registry = ActionRegistry::new();
        registry.register_phase1_movement_open_close();
        registry.register_phase1_take_place();
        registry.register_phase1_inspect_wait();
        registry.register_phase2a_epistemics();
        registry
    }

    fn view_for(state: &PhysicalState) -> EmbodiedViewModel {
        view_for_known_route(state, place_id("back_room"))
    }

    fn view_for_known_route(
        state: &PhysicalState,
        destination_place_id: PlaceId,
    ) -> EmbodiedViewModel {
        let context = KnowledgeContext::embodied_at_frontier_with_facts(
            actor_id("actor_tomas"),
            SimTick::new(1),
            0,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            vec![crate::epistemics::ActorKnownRouteFact::new(
                place_id("shop_front"),
                destination_place_id,
                "visible_exit",
            )],
        );
        let source = EmbodiedProjectionSource::from_sealed_context(&context, state, None);
        build_embodied_view_model(&context, &source, &registry(), &content_manifest_id(), None)
            .unwrap()
    }

    fn state() -> PhysicalState {
        let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
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
        let view = view_for(&state());

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
        let first = view_for(&state());
        let second = view_for(&state());

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
    fn embodied_view_omits_unobserved_food_at_open_place() {
        let mut state = state();
        state.food_supplies.insert(
            FoodSupplyId::new("food_visible_truth").unwrap(),
            FoodSupplyState::new(
                FoodSupplyId::new("food_visible_truth").unwrap(),
                Location::AtPlace(place_id("shop_front")),
                1,
                200,
            ),
        );
        let context = KnowledgeContext::embodied(actor_id("actor_tomas"), SimTick::new(1));
        let source = EmbodiedProjectionSource::from_sealed_context(&context, &state, None);
        let view =
            build_embodied_view_model(&context, &source, &registry(), &content_manifest_id(), None)
                .unwrap();

        assert!(!view.semantic_actions.iter().any(|entry| {
            entry.semantic_action_id.as_str() == "eat.food.food_visible_truth"
                || entry.target_ids.iter().any(|id| id == "food_visible_truth")
        }));
    }

    #[test]
    fn embodied_view_omits_unknown_sleep_affordance() {
        let base = state();
        let mut sleep_affordances = base.sleep_affordances().clone();
        sleep_affordances.insert(
            SleepAffordanceId::new("bed_tomas").unwrap(),
            SleepAffordanceState::new(
                SleepAffordanceId::new("bed_tomas").unwrap(),
                place_id("shop_front"),
                4,
                20,
                2,
            ),
        );
        let state = PhysicalState::from_seed_parts(
            base.actors().clone(),
            base.places().clone(),
            base.doors().clone(),
            base.containers().clone(),
            base.items().clone(),
            base.food_supplies().clone(),
            base.workplaces().clone(),
            sleep_affordances,
            crate::state::NeedModelState::new(5, 3),
        );
        let context = KnowledgeContext::embodied(actor_id("actor_tomas"), SimTick::new(1));
        let source = EmbodiedProjectionSource::from_sealed_context(&context, &state, None);
        let view =
            build_embodied_view_model(&context, &source, &registry(), &content_manifest_id(), None)
                .unwrap();

        assert!(!view
            .semantic_actions
            .iter()
            .any(|entry| entry.semantic_action_id.as_str() == "sleep.here"));
    }

    #[test]
    fn embodied_exits_require_perceived_or_known_route() {
        let state = state();
        let context = KnowledgeContext::embodied(actor_id("actor_tomas"), SimTick::new(1));
        let source = EmbodiedProjectionSource::from_sealed_context(&context, &state, None);
        let view =
            build_embodied_view_model(&context, &source, &registry(), &content_manifest_id(), None)
                .unwrap();

        assert!(view.visible_exits.is_empty());
        assert!(!view
            .semantic_actions
            .iter()
            .any(|entry| entry.semantic_action_id.as_str() == "move.to.back_room"));
    }

    #[test]
    fn embodied_exit_surfaces_perceived_closed_locked_door_blocker() {
        let mut state = state();
        let door = state
            .doors
            .get_mut(&DoorId::new("door_shop_back").unwrap())
            .unwrap();
        door.is_open = false;
        door.is_locked = true;

        let view = view_for(&state);

        assert_eq!(view.visible_exits.len(), 1);
        assert_eq!(
            view.visible_exits[0].blocker_summary.as_deref(),
            Some("door door_shop_back is closed and locked")
        );
    }

    #[test]
    fn embodied_exit_omits_unperceived_door_blocker_on_known_route() {
        let mut state = state();
        state.places.insert(
            place_id("remote_room"),
            PlaceState::new(place_id("remote_room"), "Remote room"),
        );
        let mut remote_door = DoorState::new(
            DoorId::new("door_back_remote").unwrap(),
            place_id("back_room"),
            place_id("remote_room"),
        );
        remote_door.is_open = false;
        remote_door.is_locked = true;
        state.doors.insert(remote_door.door_id.clone(), remote_door);

        let view = view_for_known_route(&state, place_id("remote_room"));

        assert_eq!(view.visible_exits.len(), 1);
        assert_eq!(
            view.visible_exits[0].destination_place_id,
            place_id("remote_room")
        );
        assert_eq!(view.visible_exits[0].blocker_summary, None);
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

        let view = view_for(&state);

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

    #[test]
    fn container_sourced_visible_item_uses_container_take_id() {
        let mut state = state();
        state
            .containers
            .get_mut(&container_id("strongbox_tomas"))
            .unwrap()
            .is_open = true;

        let view = view_for(&state);

        assert!(view
            .semantic_actions
            .iter()
            .any(|entry| entry.semantic_action_id.as_str()
                == "take.item.coin_stack_01.from.strongbox_tomas"));
        assert!(
            !view
                .semantic_actions
                .iter()
                .any(|entry| entry.semantic_action_id.as_str()
                    == "take.item.coin_stack_01.from.place")
        );
        assert!(
            view.semantic_actions
                .iter()
                .any(|entry| entry.semantic_action_id.as_str()
                    == "take.item.loose_coin_01.from.place")
        );
    }

    #[test]
    fn closed_opaque_container_check_is_disabled_with_validator_reason() {
        let view = view_for(&state());
        let entry = view
            .semantic_actions
            .iter()
            .find(|entry| entry.semantic_action_id.as_str() == "check.container.strongbox_tomas")
            .unwrap();

        assert!(!entry.enabled);
        assert_eq!(
            entry.availability.reason_codes(),
            &[ReasonCode::ContainerClosed]
        );
        assert_eq!(
            entry.availability.actor_safe_summary(),
            Some("The container is closed.")
        );
        assert!(entry
            .availability
            .provenance_refs()
            .iter()
            .any(|source| { source.kind == ActionAvailabilityProvenanceKind::HolderKnownContext }));
    }

    #[test]
    fn closed_blocking_door_move_is_disabled_with_validator_reason() {
        let mut state = state();
        state
            .doors
            .get_mut(&DoorId::new("door_shop_back").unwrap())
            .unwrap()
            .is_open = false;

        let view = view_for(&state);
        let entry = view
            .semantic_actions
            .iter()
            .find(|entry| entry.semantic_action_id.as_str() == "move.to.back_room")
            .unwrap();

        assert!(!entry.enabled);
        assert_eq!(
            entry.availability.reason_codes(),
            &[ReasonCode::DoorClosedBlocksMovement]
        );
        assert_eq!(
            entry.availability.actor_safe_summary(),
            Some("The door is closed.")
        );
    }

    #[test]
    fn locked_container_check_is_disabled_with_validator_reason() {
        let mut state = state();
        state
            .containers
            .get_mut(&container_id("strongbox_tomas"))
            .unwrap()
            .is_locked = true;

        let view = view_for(&state);
        let entry = view
            .semantic_actions
            .iter()
            .find(|entry| entry.semantic_action_id.as_str() == "check.container.strongbox_tomas")
            .unwrap();

        assert!(!entry.enabled);
        assert_eq!(
            entry.availability.reason_codes(),
            &[ReasonCode::ContainerLocked]
        );
        assert_eq!(
            entry.availability.actor_safe_summary(),
            Some("The container is locked.")
        );
    }

    fn projection_with_missing_coin_belief() -> EpistemicProjection {
        projection_with_missing_coin_variants(
            Proposition::ItemMissingFromExpectedLocation {
                item_id: item_id("coin_stack_01"),
                expected_location: Location::InContainer(container_id("strongbox_tomas")),
            },
            Proposition::ItemMissingFromExpectedLocation {
                item_id: item_id("coin_stack_01"),
                expected_location: Location::InContainer(container_id("strongbox_tomas")),
            },
        )
    }

    fn projection_with_missing_coin_variants(
        missing_belief_proposition: Proposition,
        contradiction_observed_proposition: Proposition,
    ) -> EpistemicProjection {
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
            contradiction_observed_proposition,
            SimTick::new(3),
        ));
        projection.insert_belief(
            Belief::new(
                crate::ids::BeliefId::new("belief_tomas_missing_coin").unwrap(),
                HolderKind::Actor(actor_id("actor_tomas")),
                missing_belief_proposition,
                Stance::BelievesTrue,
                Confidence::new(1000).unwrap(),
                SourceRef::Event(event_id("event_observation")),
                SimTick::new(3),
            )
            .with_observation(observation_id)
            .with_contradiction(contradiction_id),
        );

        let elena_belief = Belief::new(
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
        assert_eq!(notebook.typed_leads.len(), 1);
        let lead = &notebook.typed_leads[0];
        assert_eq!(lead.contradiction_id, "contradiction_tomas_missing_coin");
        assert_eq!(lead.observation_id, "obs_tomas_checked_strongbox");
        assert_eq!(lead.source_kind, "event");
        assert_eq!(lead.source_summary, "event:event_observation");
        assert_eq!(lead.confidence_label, "1000");
        assert_eq!(lead.staleness_label, "1 ticks old");
        assert_eq!(
            lead.possible_next_actions,
            vec!["check.container.strongbox_tomas".to_string()]
        );
        assert!(!rendered.contains("actor_mara"));
        assert!(!rendered.contains("culprit"));
        assert!(!rendered.contains("previous"));
        assert!(!rendered.contains("debug note"));
    }

    #[test]
    fn notebook_leads_come_from_typed_contradictions_not_belief_wording() {
        let projection = projection_with_missing_coin_belief();
        let context = KnowledgeContext::embodied(actor_id("actor_tomas"), SimTick::new(4));
        let baseline = build_notebook_view(&projection, &context);
        assert_eq!(baseline.typed_leads.len(), 1);

        let reworded_projection = projection_with_missing_coin_variants(
            Proposition::SoundHeardNearPlace {
                place_id: place_id("shop_front"),
            },
            Proposition::ItemMissingFromExpectedLocation {
                item_id: item_id("coin_stack_01"),
                expected_location: Location::InContainer(container_id("strongbox_tomas")),
            },
        );
        let reworded = build_notebook_view(&reworded_projection, &context);
        assert_eq!(reworded.typed_leads.len(), 1);

        let changed_contradiction_projection = projection_with_missing_coin_variants(
            Proposition::ItemMissingFromExpectedLocation {
                item_id: item_id("coin_stack_01"),
                expected_location: Location::InContainer(container_id("strongbox_tomas")),
            },
            Proposition::SoundHeardNearPlace {
                place_id: place_id("shop_front"),
            },
        );
        let typed_kind_changed = build_notebook_view(&changed_contradiction_projection, &context);
        assert!(typed_kind_changed.typed_leads.is_empty());
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
        let context = KnowledgeContext::embodied(actor_id("actor_tomas"), SimTick::new(4));
        let state = state();
        let source = EmbodiedProjectionSource::from_sealed_context(&context, &state, None);
        let mut view =
            build_embodied_view_model(&context, &source, &registry(), &content_manifest_id(), None)
                .unwrap();
        view.notebook = Some(build_notebook_view(&projection, &context));

        assert_eq!(
            view.holder_known_context_id,
            context.holder_known_context_id().clone()
        );
        assert_eq!(
            view.holder_known_context_hash,
            context.holder_known_context_hash().clone()
        );
        assert_eq!(
            view.notebook.unwrap().source_bound_beliefs[0].acquired_tick,
            3
        );
    }

    #[test]
    fn view_models_embodied_phase3a_status_is_viewer_scoped_and_actor_known() {
        let mut state = state();
        state
            .actors
            .get_mut(&actor_id("actor_mara"))
            .unwrap()
            .current_place_id = place_id("back_room");
        state.food_supplies.insert(
            FoodSupplyId::new("food_soup_pot").unwrap(),
            FoodSupplyState::new(
                FoodSupplyId::new("food_soup_pot").unwrap(),
                Location::AtPlace(place_id("shop_front")),
                1,
                200,
            ),
        );
        state.food_supplies.insert(
            FoodSupplyId::new("food_hidden_back_room").unwrap(),
            FoodSupplyState::new(
                FoodSupplyId::new("food_hidden_back_room").unwrap(),
                Location::AtPlace(place_id("back_room")),
                1,
                200,
            ),
        );
        let hidden_container_id = container_id("hidden_pantry");
        state.containers.insert(
            hidden_container_id.clone(),
            ContainerState::fixed_at_place(hidden_container_id.clone(), place_id("shop_front")),
        );
        state.food_supplies.insert(
            FoodSupplyId::new("food_hidden_pantry").unwrap(),
            FoodSupplyState::new(
                FoodSupplyId::new("food_hidden_pantry").unwrap(),
                Location::InContainer(hidden_container_id),
                1,
                200,
            ),
        );
        let mut workplace = WorkplaceState::new(
            WorkplaceId::new("workplace_tomas").unwrap(),
            place_id("shop_front"),
            4,
            8,
            4,
            900,
            900,
            "repair_output",
        );
        workplace.assigned_actor_ids.insert(actor_id("actor_tomas"));
        state
            .workplaces
            .insert(WorkplaceId::new("workplace_tomas").unwrap(), workplace);

        let mut agent_state = AgentState::default();
        agent_state
            .needs_by_actor
            .entry(actor_id("actor_tomas"))
            .or_default()
            .insert(
                NeedKind::Hunger,
                NeedState::initial(NeedKind::Hunger, 620, NeedChangeCause::FixtureInitial),
            );
        agent_state
            .needs_by_actor
            .entry(actor_id("actor_mara"))
            .or_default()
            .insert(
                NeedKind::Hunger,
                NeedState::initial(NeedKind::Hunger, 900, NeedChangeCause::FixtureInitial),
            );
        let intention_id = IntentionId::new("intention_tomas_work").unwrap();
        agent_state
            .active_intention_by_actor
            .insert(actor_id("actor_tomas"), intention_id.clone());
        agent_state.intentions.insert(
            intention_id.clone(),
            Intention::adopt(
                intention_id,
                actor_id("actor_tomas"),
                IntentionSource::RoutineDuty,
                CandidateGoalId::new("goal_tomas_work").unwrap(),
                Some(RoutineTemplateId::new("routine_tomas_work").unwrap()),
                Some("work_block".to_string()),
                8,
                SimTick::new(1),
                DecisionTraceId::new("trace_tomas_work").unwrap(),
            ),
        );

        let context = KnowledgeContext::embodied_at_frontier_with_facts(
            actor_id("actor_tomas"),
            SimTick::new(2),
            0,
            vec![crate::epistemics::ActorKnownWorkplaceFact::new(
                WorkplaceId::new("workplace_tomas").unwrap(),
                place_id("shop_front"),
                true,
                "routine_assignment_notice",
                crate::agent::SourceEventIds::checked(vec![EventId::new(
                    "event_role_notice_actor_tomas_workplace_tomas",
                )
                .unwrap()])
                .unwrap(),
                SimTick::new(0),
            )],
            vec![crate::epistemics::ActorKnownFoodSourceFact::new(
                FoodSupplyId::new("food_soup_pot").unwrap(),
                "visible_food_supply",
            )],
            Vec::new(),
            Vec::new(),
        );
        let source =
            EmbodiedProjectionSource::from_sealed_context(&context, &state, Some(&agent_state));
        let view =
            build_embodied_view_model(&context, &source, &registry(), &content_manifest_id(), None)
                .unwrap();

        let status = view.phase3a_status.as_ref().unwrap();
        assert_eq!(status.need_summaries.len(), 1);
        assert_eq!(status.need_summaries[0].kind, "hunger");
        assert_eq!(status.need_summaries[0].band_label, "urgent");
        assert_eq!(status.need_summaries[0].last_cause, "fixture_initial");
        assert!(status
            .intention_summary
            .as_deref()
            .unwrap()
            .contains("routine_tomas_work"));
        assert!(view
            .semantic_actions
            .iter()
            .any(|entry| entry.semantic_action_id.as_str() == "eat.food.food_soup_pot"));
        assert!(!view
            .semantic_actions
            .iter()
            .any(|entry| entry.semantic_action_id.as_str() == "eat.food.food_hidden_back_room"));
        assert!(!view.semantic_actions.iter().any(|entry| {
            entry
                .semantic_action_id
                .as_str()
                .contains("food_hidden_pantry")
                || entry.label.contains("food_hidden_pantry")
                || entry
                    .target_ids
                    .iter()
                    .any(|target| target == "food_hidden_pantry")
                || entry
                    .availability
                    .actor_safe_summary()
                    .is_some_and(|why| why.contains("food_hidden_pantry"))
        }));
        assert!(view
            .semantic_actions
            .iter()
            .any(|entry| entry.semantic_action_id.as_str() == "work.block.workplace_tomas"));
        assert!(view
            .semantic_actions
            .iter()
            .any(|entry| entry.semantic_action_id.as_str()
                == "continue.routine.intention_tomas_work"));
        let rendered = format!("{view:?}");
        assert!(!rendered.contains("actor_mara"));
        assert!(!rendered.contains("900"));
    }

    #[test]
    fn view_models_embodied_phase3a_salient_interruption_is_viewer_scoped() {
        let state = state();
        let mut agent_state = AgentState::default();
        agent_state.ordinary_life_episodes.insert(
            event_id("event_mara_sleep_interrupted"),
            OrdinaryLifeEpisodeRecord {
                event_id: event_id("event_mara_sleep_interrupted"),
                event_kind: "sleep_interrupted".to_string(),
                actor_id: Some(actor_id("actor_mara")),
                proposal_id: None,
                caused_event_ids: Vec::new(),
                sim_tick: SimTick::new(9),
                payload_fields: vec![("reason".to_string(), "door_noise".to_string())],
                summary: "Mara woke because of door noise".to_string(),
            },
        );
        agent_state.ordinary_life_episodes.insert(
            event_id("event_tomas_work_block_failed"),
            OrdinaryLifeEpisodeRecord {
                event_id: event_id("event_tomas_work_block_failed"),
                event_kind: "work_block_failed".to_string(),
                actor_id: Some(actor_id("actor_tomas")),
                proposal_id: None,
                caused_event_ids: Vec::new(),
                sim_tick: SimTick::new(4),
                payload_fields: vec![("reason".to_string(), "workplace_closed".to_string())],
                summary: "Work blocked by closed workshop".to_string(),
            },
        );
        agent_state.ordinary_life_episodes.insert(
            event_id("event_tomas_sleep_interrupted"),
            OrdinaryLifeEpisodeRecord {
                event_id: event_id("event_tomas_sleep_interrupted"),
                event_kind: "sleep_interrupted".to_string(),
                actor_id: Some(actor_id("actor_tomas")),
                proposal_id: None,
                caused_event_ids: Vec::new(),
                sim_tick: SimTick::new(8),
                payload_fields: vec![("reason".to_string(), "hunger".to_string())],
                summary: "Sleep interrupted by hunger".to_string(),
            },
        );

        let context = KnowledgeContext::embodied(actor_id("actor_tomas"), SimTick::new(10));
        let source =
            EmbodiedProjectionSource::from_sealed_context(&context, &state, Some(&agent_state));
        let view =
            build_embodied_view_model(&context, &source, &registry(), &content_manifest_id(), None)
                .unwrap();

        let interruption = view
            .phase3a_status
            .as_ref()
            .unwrap()
            .salient_interruption
            .as_deref()
            .unwrap();
        assert_eq!(
            interruption,
            "sleep_interrupted at tick 8: Sleep interrupted by hunger"
        );
        assert!(!interruption.contains("Mara"));
        assert!(!interruption.contains("closed workshop"));
    }

    #[test]
    fn no_human_day_metrics_are_independent_event_log_counts() {
        let mut log = EventLog::new();
        append_metric_event(&mut log, EventKind::NoHumanDayStarted, 0, 0);
        append_metric_event(&mut log, EventKind::RoutineStepStarted, 1, 1);
        append_metric_event(&mut log, EventKind::RoutineStepCompleted, 2, 2);
        append_metric_event(&mut log, EventKind::FoodConsumed, 3, 3);
        append_metric_event(&mut log, EventKind::EatFailed, 4, 4);
        append_metric_event(&mut log, EventKind::SleepCompleted, 5, 5);
        append_metric_event(&mut log, EventKind::SleepInterrupted, 6, 6);
        append_metric_event(&mut log, EventKind::WorkBlockCompleted, 7, 7);
        append_metric_event(&mut log, EventKind::WorkBlockFailed, 8, 8);
        append_metric_event(&mut log, EventKind::NeedThresholdCrossed, 9, 9);
        append_metric_event(&mut log, EventKind::IntentionInterrupted, 10, 10);
        append_metric_event(&mut log, EventKind::RoutineStepFailed, 11, 11);
        let mut text_only_planner_failure = metric_event(EventKind::DecisionTraceRecorded, 12, 12);
        text_only_planner_failure.payload = vec![PayloadField::new(
            "trace_canonical",
            "decision_trace_v1|planner_budget_exhausted",
        )];
        log.append(text_only_planner_failure).unwrap();
        let mut typed_planner_failure = metric_event(EventKind::StuckDiagnosticRecorded, 13, 13);
        typed_planner_failure.payload = vec![
            PayloadField::new("responsible_layer", "local_planning"),
            PayloadField::new("blocker_code", "local_plan_failed"),
        ];
        log.append(typed_planner_failure).unwrap();
        let mut stuck_first = metric_event(EventKind::StuckDiagnosticRecorded, 14, 14);
        stuck_first.actor_id = Some(actor_id("actor_tomas"));
        log.append(stuck_first).unwrap();
        let mut stuck_duplicate = metric_event(EventKind::StuckDiagnosticRecorded, 15, 15);
        stuck_duplicate.actor_id = Some(actor_id("actor_tomas"));
        log.append(stuck_duplicate).unwrap();
        let mut replay_failure = metric_event(EventKind::ReplayProjectionRebuilt, 16, 16);
        replay_failure.payload = vec![PayloadField::new("status", "failure")];
        log.append(replay_failure).unwrap();
        let mut player_conditioned = metric_event(EventKind::ActionStarted, 17, 17);
        player_conditioned.effects_summary = "conditioned on player choice".to_string();
        log.append(player_conditioned).unwrap();
        append_metric_event(&mut log, EventKind::NoHumanDayCompleted, 18, 20);

        let metrics = no_human_day_metrics(&log);
        let events = log.events();

        assert_eq!(metrics.projection_version, "no_human_day_metrics_v1");
        assert_eq!(metrics.events_per_day, events.len());
        assert_eq!(
            metrics.routine_event_count,
            events
                .iter()
                .filter(|event| matches!(
                    event.event_type,
                    EventKind::RoutineStepStarted
                        | EventKind::RoutineStepCompleted
                        | EventKind::RoutineStepFailed
                        | EventKind::ContinueRoutineAccepted
                        | EventKind::ContinueRoutineRejected
                ))
                .count()
        );
        assert_eq!(
            metrics.meals_completed,
            events
                .iter()
                .filter(|event| event.event_type == EventKind::FoodConsumed)
                .count()
        );
        assert_eq!(
            metrics.meals_missed,
            events
                .iter()
                .filter(|event| event.event_type == EventKind::EatFailed)
                .count()
        );
        assert_eq!(
            metrics.sleep_completed,
            events
                .iter()
                .filter(|event| event.event_type == EventKind::SleepCompleted)
                .count()
        );
        assert_eq!(
            metrics.sleep_interrupted,
            events
                .iter()
                .filter(|event| event.event_type == EventKind::SleepInterrupted)
                .count()
        );
        assert_eq!(
            metrics.work_blocks_completed,
            events
                .iter()
                .filter(|event| event.event_type == EventKind::WorkBlockCompleted)
                .count()
        );
        assert_eq!(
            metrics.work_blocks_failed,
            events
                .iter()
                .filter(|event| event.event_type == EventKind::WorkBlockFailed)
                .count()
        );
        assert_eq!(
            metrics.need_threshold_crossings,
            events
                .iter()
                .filter(|event| event.event_type == EventKind::NeedThresholdCrossed)
                .count()
        );
        assert_eq!(metrics.routine_interruptions, 2);
        assert_eq!(metrics.planner_failures, 1);
        assert_eq!(metrics.stuck_actor_count, 1);
        assert_eq!(metrics.run_duration_ticks, 20);
        assert_eq!(metrics.replay_failure_count, 1);
        assert_eq!(metrics.tui_action_coverage, events.len());
        assert_eq!(metrics.player_conditioned_event_count, 1);
        assert_eq!(
            metrics.player_conditioned_event_rate_per_1000,
            (metrics.player_conditioned_event_count as u64 * 1000) / metrics.events_per_day as u64
        );
    }

    #[test]
    fn continue_routine_marker_alone_counts_as_no_behavioral_progress() {
        let mut log = EventLog::new();
        append_metric_event(&mut log, EventKind::NoHumanDayStarted, 0, 0);
        append_metric_event(&mut log, EventKind::ContinueRoutineProposed, 1, 1);
        append_metric_event(&mut log, EventKind::NoHumanDayCompleted, 2, 2);

        let metrics = no_human_day_metrics(&log);

        assert_eq!(metrics.routine_event_count, 0);
        assert_eq!(metrics.meals_completed, 0);
        assert_eq!(metrics.sleep_completed, 0);
        assert_eq!(metrics.work_blocks_completed, 0);
    }

    #[test]
    fn continue_routine_follow_on_progress_survives_replay_from_event_chain() {
        let mut log = EventLog::new();
        append_metric_event(&mut log, EventKind::NoHumanDayStarted, 0, 0);
        append_metric_event(&mut log, EventKind::ContinueRoutineProposed, 1, 1);
        append_metric_event(&mut log, EventKind::RoutineStepCompleted, 2, 2);
        append_metric_event(&mut log, EventKind::FoodConsumed, 3, 3);
        append_metric_event(&mut log, EventKind::NoHumanDayCompleted, 4, 4);

        let first = no_human_day_metrics(&log);
        let replayed = EventLog::deserialize_canonical(&log.serialize_canonical()).unwrap();
        let second = no_human_day_metrics(&replayed);

        assert_eq!(first.routine_event_count, 1);
        assert_eq!(first.meals_completed, 1);
        assert_eq!(first.serialize_canonical(), second.serialize_canonical());
    }

    #[test]
    fn no_human_day_metrics_survive_replay_byte_identically() {
        let mut log = EventLog::new();
        append_metric_event(&mut log, EventKind::NoHumanDayStarted, 0, 0);
        append_metric_event(&mut log, EventKind::RoutineStepStarted, 1, 1);
        append_metric_event(&mut log, EventKind::RoutineStepCompleted, 2, 2);
        append_metric_event(&mut log, EventKind::FoodConsumed, 3, 3);
        append_metric_event(&mut log, EventKind::NeedThresholdCrossed, 4, 4);
        append_metric_event(&mut log, EventKind::NoHumanDayCompleted, 5, 24);

        let first = no_human_day_metrics(&log);
        let replayed = EventLog::deserialize_canonical(&log.serialize_canonical()).unwrap();
        let second = no_human_day_metrics(&replayed);

        assert_eq!(first.serialize_canonical(), second.serialize_canonical());
    }

    #[test]
    fn populated_no_human_day_metrics_have_activity_without_player_conditioning() {
        let mut log = EventLog::new();
        append_metric_event(&mut log, EventKind::NoHumanDayStarted, 0, 0);
        append_metric_event(&mut log, EventKind::RoutineStepStarted, 1, 1);
        append_metric_event(&mut log, EventKind::RoutineStepCompleted, 2, 2);
        append_metric_event(&mut log, EventKind::FoodConsumed, 3, 3);
        append_metric_event(&mut log, EventKind::NeedThresholdCrossed, 4, 4);
        append_metric_event(&mut log, EventKind::NoHumanDayCompleted, 5, 24);

        let metrics = no_human_day_metrics(&log);

        assert!(metrics.routine_event_count > 0);
        assert!(metrics.meals_completed > 0);
        assert!(metrics.need_threshold_crossings > 0);
        assert_eq!(metrics.player_conditioned_event_count, 0);
        assert_eq!(metrics.player_conditioned_event_rate_per_1000, 0);
    }

    fn door_between(id: &str, a: &str, b: &str) -> DoorState {
        DoorState::new(DoorId::new(id).unwrap(), place_id(a), place_id(b))
    }

    fn blocker_summary_for(door: DoorState, from: &str, to: &str) -> Option<String> {
        let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
        let door_id = door.door_id.clone();
        state.doors.insert(door_id.clone(), door);
        let mut connected = std::collections::BTreeSet::new();
        connected.insert(door_id);
        visible_exit_blocker_summary(&state, &connected, &place_id(from), &place_id(to))
    }

    #[test]
    fn door_connects_edge_requires_the_full_endpoint_pair() {
        let door = door_between("door_ab", "place_a", "place_b");

        // A door connects its edge in both traversal directions. The reverse case
        // pins the `endpoint_b == from && endpoint_a == to` comparisons (kills the
        // `== -> !=` mutants on that clause).
        assert!(door_connects_edge(
            &door,
            &place_id("place_a"),
            &place_id("place_b")
        ));
        assert!(door_connects_edge(
            &door,
            &place_id("place_b"),
            &place_id("place_a")
        ));

        // Exactly one endpoint matching must NOT connect the edge. These kill the
        // `&& -> ||` mutants on both clauses, which would otherwise let a single
        // matching endpoint satisfy the predicate.
        assert!(!door_connects_edge(
            &door,
            &place_id("place_a"),
            &place_id("place_c")
        ));
        assert!(!door_connects_edge(
            &door,
            &place_id("place_c"),
            &place_id("place_a")
        ));
        assert!(!door_connects_edge(
            &door,
            &place_id("place_c"),
            &place_id("place_b")
        ));

        // A wholly unrelated edge does not connect.
        assert!(!door_connects_edge(
            &door,
            &place_id("place_c"),
            &place_id("place_d")
        ));
    }

    #[test]
    fn visible_exit_blocker_summary_distinguishes_blocker_branches() {
        let edge = ("place_a", "place_b");

        // Locked + closed -> "closed and locked".
        let mut locked_closed = door_between("door_lc", edge.0, edge.1);
        locked_closed.is_open = false;
        locked_closed.is_locked = true;
        assert_eq!(
            blocker_summary_for(locked_closed, edge.0, edge.1),
            Some("door door_lc is closed and locked".to_string())
        );

        // Locked + OPEN -> "locked" (not "closed and locked"). Kills the
        // `is_locked && !is_open -> is_locked || !is_open` mutant, which would
        // misreport an open-but-locked door as closed.
        let mut locked_open = door_between("door_lo", edge.0, edge.1);
        locked_open.is_open = true;
        locked_open.is_locked = true;
        assert_eq!(
            blocker_summary_for(locked_open, edge.0, edge.1),
            Some("door door_lo is locked".to_string())
        );

        // Unlocked + closed + blocks-when-closed -> "closed".
        let mut closed_blocking = door_between("door_cb", edge.0, edge.1);
        closed_blocking.is_open = false;
        closed_blocking.is_locked = false;
        closed_blocking.blocks_movement_when_closed = true;
        assert_eq!(
            blocker_summary_for(closed_blocking, edge.0, edge.1),
            Some("door door_cb is closed".to_string())
        );

        // Unlocked + OPEN + blocks-when-closed -> no blocker. Kills the
        // `delete !` mutant on `!door.is_open`, which would treat an open door
        // as closed.
        let mut open_passable = door_between("door_op", edge.0, edge.1);
        open_passable.is_open = true;
        open_passable.is_locked = false;
        open_passable.blocks_movement_when_closed = true;
        assert_eq!(blocker_summary_for(open_passable, edge.0, edge.1), None);

        // Unlocked + closed + does NOT block when closed -> no blocker. Kills the
        // `!is_open && blocks_movement_when_closed -> ... || ...` mutant, which
        // would report a passable closed door as a blocker.
        let mut closed_non_blocking = door_between("door_cn", edge.0, edge.1);
        closed_non_blocking.is_open = false;
        closed_non_blocking.is_locked = false;
        closed_non_blocking.blocks_movement_when_closed = false;
        assert_eq!(
            blocker_summary_for(closed_non_blocking, edge.0, edge.1),
            None
        );
    }
}
