use crate::actions::{
    validate_proposal, ActionRegistry, Proposal, ProposalOrigin, ProposalValidationContext,
    ReportStatus, ValidationReport,
};
use crate::epistemics::{EpistemicProjection, KnowledgeContext, SourceRef};
use crate::events::log::EventLog;
use crate::events::{EventEnvelope, EventKind};
use crate::ids::{
    ActionId, ActorId, ContentManifestId, ControllerId, ItemId, PlaceId, ProposalId,
    SemanticActionId, ViewModelId,
};
use crate::location::{visible_locality, Location};
use crate::scheduler::{OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId};
use crate::state::AgentState;
use crate::state::{ActorBody, PhysicalState};
use crate::time::SimTick;
use crate::view_models::{
    DebugEventLogView, DebugEventSummary, EmbodiedViewModel, NeedStatusEntry, NotebookBeliefEntry,
    NotebookContradictionEntry, NotebookObservationEntry, NotebookView, Phase3AEmbodiedStatus,
    SemanticActionEntry, ViewMode, VisibleActor, VisibleContainer, VisibleDoor, VisibleExit,
    VisibleItem, VisibleItemSource, WhyNotView,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProjectionError {
    ActorNotFound(ActorId),
    PlaceNotFound(PlaceId),
}

pub struct EmbodiedProjectionSource<'a> {
    state: &'a PhysicalState,
    agent_state: Option<&'a AgentState>,
}

impl<'a> EmbodiedProjectionSource<'a> {
    pub fn from_sealed_context(
        _context: &'a KnowledgeContext,
        state: &'a PhysicalState,
        agent_state: Option<&'a AgentState>,
    ) -> Self {
        Self { state, agent_state }
    }
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
            .filter(|event| {
                event.event_type == EventKind::DecisionTraceRecorded
                    && event.payload.iter().any(|field| {
                        field.value.contains("planner_budget_exhausted")
                            || (field.value.contains("planner") && field.value.contains("failed"))
                    })
            })
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
    let viewer_actor_id = &context.viewer_actor_id;
    let sim_tick = context.current_tick;
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
        holder_known_context_frontier: context.event_frontier,
        holder_known_context_source_summary: format!(
            "allowed={} provenance={}",
            context.allowed_sources.len(),
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

fn phase3a_status(agent_state: &AgentState, viewer_actor_id: &ActorId) -> Phase3AEmbodiedStatus {
    let mut need_summaries = agent_state
        .needs_by_actor
        .get(viewer_actor_id)
        .into_iter()
        .flat_map(|needs| needs.values())
        .map(|need| NeedStatusEntry {
            kind: need.kind().stable_id().to_string(),
            value: need.value(),
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
        salient_interruption: None,
    }
}

fn phase3a_semantic_actions(
    state: &PhysicalState,
    agent_state: Option<&AgentState>,
    actor: &ActorBody,
    viewer_actor_id: &ActorId,
) -> Vec<SemanticActionEntry> {
    let mut actions = Vec::new();
    actions.push(SemanticActionEntry::new(
        SemanticActionId::new("sleep.here").unwrap(),
        ActionId::new("sleep").unwrap(),
        vec![actor.current_place_id.to_string()],
        "Sleep here",
        true,
        None,
    ));

    for food in state.food_supplies.values() {
        let reachable = match &food.location {
            Location::AtPlace(place_id) => place_id == &actor.current_place_id,
            Location::CarriedBy(actor_id) => actor_id == viewer_actor_id,
            Location::InContainer(container_id) => {
                state.containers.get(container_id).is_some_and(|container| {
                    container.location == Location::AtPlace(actor.current_place_id.clone())
                        && container.is_open
                })
            }
        };
        if reachable {
            actions.push(SemanticActionEntry::new(
                SemanticActionId::new(format!("eat.food.{}", food.food_supply_id.as_str()))
                    .unwrap(),
                ActionId::new("eat").unwrap(),
                vec![food.food_supply_id.to_string()],
                format!("Eat {}", food.food_supply_id.as_str()),
                !food.is_empty(),
                food.is_empty()
                    .then_some("No servings are available here.".to_string()),
            ));
        }
    }

    for workplace in state.workplaces.values() {
        let assigned = workplace.assigned_actor_ids.is_empty()
            || workplace.assigned_actor_ids.contains(viewer_actor_id);
        if assigned {
            let at_workplace = workplace.place_id == actor.current_place_id;
            let enabled = at_workplace && workplace.access_open;
            let why_disabled = if !at_workplace {
                Some("You are not at that workplace.".to_string())
            } else if !workplace.access_open {
                Some("That workplace is not available.".to_string())
            } else {
                None
            };
            actions.push(SemanticActionEntry::new(
                SemanticActionId::new(format!("work.block.{}", workplace.workplace_id.as_str()))
                    .unwrap(),
                ActionId::new("work_block").unwrap(),
                vec![workplace.workplace_id.to_string()],
                format!("Work at {}", workplace.workplace_id.as_str()),
                enabled,
                why_disabled,
            ));
        }
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
        actions.push(SemanticActionEntry::new(
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
            true,
            None,
        ));
    }

    actions
}

pub fn build_debug_event_log_view(log: &EventLog) -> DebugEventLogView {
    DebugEventLogView {
        debug_only: true,
        events: log.events().iter().map(DebugEventSummary::from).collect(),
    }
}

fn visible_item_source(location: &Location) -> VisibleItemSource {
    match location {
        Location::AtPlace(_) => VisibleItemSource::Place,
        Location::InContainer(container_id) => VisibleItemSource::Container(container_id.clone()),
        Location::CarriedBy(_) => VisibleItemSource::Carried,
    }
}

#[derive(Clone, Copy)]
struct SemanticActionPreflightContext<'a> {
    state: &'a PhysicalState,
    agent_state: &'a AgentState,
    registry: &'a ActionRegistry,
    content_manifest_id: &'a ContentManifestId,
    viewer_actor_id: &'a ActorId,
    sim_tick: SimTick,
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
        },
        &proposal,
    );
    let enabled = report.status == ReportStatus::Accepted;
    SemanticActionEntry::new(
        entry.semantic_action_id,
        entry.action_id,
        entry.target_ids,
        entry.label,
        enabled,
        (!enabled).then_some(report.actor_visible_summary),
    )
}

pub fn proposal_from_semantic_action_entry(
    proposal_id: ProposalId,
    origin: ProposalOrigin,
    actor_id: Option<ActorId>,
    requested_tick: SimTick,
    entry: &SemanticActionEntry,
    source_view_model_id: Option<ViewModelId>,
    controller_id: Option<&ControllerId>,
) -> Proposal {
    let mut proposal = Proposal::new(
        proposal_id,
        origin,
        actor_id,
        entry.action_id.clone(),
        requested_tick,
    );
    proposal.target_ids = entry.target_ids.clone();
    proposal.source_view_model_id = source_view_model_id;
    if let Some(controller_id) = controller_id {
        proposal
            .parameters
            .insert("controller_id".to_string(), controller_id.to_string());
    }
    if entry.semantic_action_id.as_str() == "wait.1_tick" {
        proposal
            .parameters
            .insert("ticks".to_string(), "1".to_string());
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::{Intention, IntentionSource, NeedChangeCause, NeedKind, NeedState};
    use crate::epistemics::{
        Belief, Channel, Confidence, Contradiction, ContradictionKind, HolderKind, Observation,
        ObservationSubject, ObservationTarget, PrivacyScope, Proposition, SourceRef, Stance,
    };
    use crate::events::PayloadField;
    use crate::ids::{
        CandidateGoalId, ContainerId, DecisionTraceId, DoorId, FoodSupplyId, IntentionId,
        ProcessId, RoutineTemplateId, WorkplaceId,
    };
    use crate::state::{
        ActorBody, ContainerState, DoorState, ItemState, PhysicalState, PlaceState,
    };
    use crate::state::{AgentState, FoodSupplyState, WorkplaceState};

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
        let context = KnowledgeContext::embodied(actor_id("actor_tomas"), SimTick::new(1));
        let source = EmbodiedProjectionSource::from_sealed_context(&context, state, None);
        build_embodied_view_model(&context, &source, &registry(), &content_manifest_id(), None)
            .unwrap()
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
            entry.why_disabled.as_deref(),
            Some("The container is closed.")
        );
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
        assert_eq!(entry.why_disabled.as_deref(), Some("The door is closed."));
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
            entry.why_disabled.as_deref(),
            Some("The container is locked.")
        );
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
        let mut workplace = WorkplaceState::new(
            WorkplaceId::new("workplace_tomas").unwrap(),
            place_id("shop_front"),
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

        let context = KnowledgeContext::embodied(actor_id("actor_tomas"), SimTick::new(2));
        let source =
            EmbodiedProjectionSource::from_sealed_context(&context, &state, Some(&agent_state));
        let view =
            build_embodied_view_model(&context, &source, &registry(), &content_manifest_id(), None)
                .unwrap();

        let status = view.phase3a_status.as_ref().unwrap();
        assert_eq!(status.need_summaries.len(), 1);
        assert_eq!(status.need_summaries[0].kind, "hunger");
        assert_eq!(status.need_summaries[0].value, 620);
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
        let mut planner_failure = metric_event(EventKind::DecisionTraceRecorded, 12, 12);
        planner_failure.payload = vec![PayloadField::new(
            "trace_canonical",
            "decision_trace_v1|planner_budget_exhausted",
        )];
        log.append(planner_failure).unwrap();
        let mut stuck_first = metric_event(EventKind::StuckDiagnosticRecorded, 13, 13);
        stuck_first.actor_id = Some(actor_id("actor_tomas"));
        log.append(stuck_first).unwrap();
        let mut stuck_duplicate = metric_event(EventKind::StuckDiagnosticRecorded, 14, 14);
        stuck_duplicate.actor_id = Some(actor_id("actor_tomas"));
        log.append(stuck_duplicate).unwrap();
        let mut replay_failure = metric_event(EventKind::ReplayProjectionRebuilt, 15, 15);
        replay_failure.payload = vec![PayloadField::new("status", "failure")];
        log.append(replay_failure).unwrap();
        let mut player_conditioned = metric_event(EventKind::ActionStarted, 16, 16);
        player_conditioned.effects_summary = "conditioned on player choice".to_string();
        log.append(player_conditioned).unwrap();
        append_metric_event(&mut log, EventKind::NoHumanDayCompleted, 17, 20);

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
}
