use crate::actions::pipeline::PipelineStage;
use crate::actions::report::{CheckedFact, ReasonCode, ValidationReport};
use crate::checksum::{compute_physical_checksum, ChecksumContext, PhysicalChecksum};
use crate::controller::ControllerBindings;
use crate::debug_capability::DebugCapability;
use crate::events::log::EventLog;
use crate::events::{EventEnvelope, EventKind, EventStream};
use crate::ids::{ActorId, DebugReportId, EventId, ItemId, ProposalId, ValidationReportId};
use crate::location::Location;
use crate::projections::{no_human_day_metrics, NoHumanDayMetrics};
use crate::replay::{ProjectionRebuildReport, ReplayReport};
use crate::state::{AgentState, PhysicalState};
use crate::time::SimTick;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ItemLocationDebugReport {
    pub report_id: DebugReportId,
    pub item_id: ItemId,
    pub exists: bool,
    pub current_location: Option<String>,
    pub location_chain: Vec<String>,
    pub current_projection_position: Option<u64>,
    pub last_location_event_id: Option<EventId>,
    pub location_event_chain: Vec<EventId>,
    pub relevant_events: Vec<DebugEventSummary>,
    pub inconsistencies: Vec<String>,
    pub physical_checksum: PhysicalChecksum,
    debug_capability: DebugCapability,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionRejectionDebugReport {
    pub report_id: DebugReportId,
    pub proposal_id: ProposalId,
    pub validation_report_id: ValidationReportId,
    pub failed_stage: Option<PipelineStage>,
    pub reason_codes: Vec<ReasonCode>,
    pub actor_visible_summary: String,
    pub debug_summary: String,
    pub actor_visible_facts: Vec<CheckedFact>,
    pub debug_only_facts: Vec<CheckedFact>,
    pub precondition_trace: Vec<String>,
    pub events_created: Vec<EventId>,
    pub mutation_attempted: bool,
    pub checksum_before: PhysicalChecksum,
    pub checksum_after: PhysicalChecksum,
    debug_capability: DebugCapability,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProjectionRebuildDebugReport {
    pub report_id: DebugReportId,
    pub rebuild: ProjectionRebuildReport,
    debug_capability: DebugCapability,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplayDebugReport {
    pub report_id: DebugReportId,
    pub replay: ReplayReport,
    debug_capability: DebugCapability,
}

/// Privileged controller-binding report.
///
/// ```compile_fail
/// use tracewake_core::debug_reports::ControllerBindingDebugReport;
/// use tracewake_core::ids::DebugReportId;
///
/// let _report = ControllerBindingDebugReport {
///     report_id: DebugReportId::new("debug.controller").unwrap(),
///     bindings: Vec::new(),
/// };
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ControllerBindingDebugReport {
    pub report_id: DebugReportId,
    pub bindings: Vec<String>,
    debug_capability: DebugCapability,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Phase3ADebugReport {
    pub report_id: DebugReportId,
    pub title: String,
    pub rows: Vec<String>,
    pub decision_traces: Vec<Phase3ADecisionTraceView>,
    pub stuck_diagnostics: Vec<Phase3AStuckDiagnosticView>,
    debug_capability: DebugCapability,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NoHumanDayDebugReport {
    pub report_id: DebugReportId,
    pub metrics: NoHumanDayMetrics,
    pub canonical_summary: String,
    debug_capability: DebugCapability,
}

impl ItemLocationDebugReport {
    pub fn debug_only(&self) -> bool {
        self.debug_capability.debug_only()
    }
}

impl ActionRejectionDebugReport {
    pub fn debug_only(&self) -> bool {
        self.debug_capability.debug_only()
    }
}

impl ProjectionRebuildDebugReport {
    pub fn debug_only(&self) -> bool {
        self.debug_capability.debug_only()
    }
}

impl ReplayDebugReport {
    pub fn debug_only(&self) -> bool {
        self.debug_capability.debug_only()
    }
}

impl ControllerBindingDebugReport {
    pub fn debug_only(&self) -> bool {
        self.debug_capability.debug_only()
    }
}

impl Phase3ADebugReport {
    pub fn debug_only(&self) -> bool {
        self.debug_capability.debug_only()
    }
}

impl NoHumanDayDebugReport {
    pub fn debug_only(&self) -> bool {
        self.debug_capability.debug_only()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Phase3ADecisionTraceView {
    pub trace_id: String,
    pub actor_id: ActorId,
    pub window_start_tick: SimTick,
    pub window_end_tick: SimTick,
    pub outcome: String,
    pub candidate_goal_count: usize,
    pub hidden_truth_actor_known_only: bool,
    pub hidden_truth_notes: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Phase3AStuckDiagnosticView {
    pub diagnostic_id: String,
    pub actor_id: ActorId,
    pub window_start_tick: SimTick,
    pub window_end_tick: SimTick,
    pub active_need: Option<String>,
    pub active_goal_id: Option<String>,
    pub active_intention_id: Option<String>,
    pub routine_template_id: Option<String>,
    pub routine_execution_id: Option<String>,
    pub routine_step: Option<String>,
    pub attempted_action: Option<String>,
    pub blocker_category: String,
    pub concrete_blocker: String,
    pub actor_known_explanation: String,
    pub debug_only_details: String,
    pub retry_abandon_fallback_outcome: String,
    pub resulting_status: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugEventSummary {
    pub event_id: EventId,
    pub event_type: EventKind,
    pub stream: EventStream,
    pub stream_position: u64,
}

pub fn item_location_report(
    state: &PhysicalState,
    log: &EventLog,
    item_id: &ItemId,
    checksum_context: &ChecksumContext,
) -> ItemLocationDebugReport {
    let physical_checksum = compute_physical_checksum(state, checksum_context).checksum;
    let item = state.items.get(item_id);
    let location_chain = item
        .map(|item| location_chain(state, &item.location))
        .unwrap_or_default();
    let current_location = item.map(|item| location_summary(state, &item.location));
    let relevant_events = log
        .events()
        .iter()
        .filter(|event| is_item_location_event(event, item_id))
        .map(|event| DebugEventSummary {
            event_id: event.event_id.clone(),
            event_type: event.event_type,
            stream: event.stream,
            stream_position: event.stream_position,
        })
        .collect::<Vec<_>>();
    let location_event_chain = relevant_events
        .iter()
        .map(|event| event.event_id.clone())
        .collect::<Vec<_>>();
    let last_location_event_id = location_event_chain
        .last()
        .cloned()
        .or_else(|| Some(EventId::new(format!("fixture_origin.{}", item_id.as_str())).unwrap()));
    let current_projection_position = log
        .events()
        .iter()
        .filter(|event| event.stream == EventStream::World)
        .map(|event| event.stream_position)
        .max();

    ItemLocationDebugReport {
        report_id: DebugReportId::new(format!("debug.item_location.{}", item_id.as_str())).unwrap(),
        item_id: item_id.clone(),
        exists: item.is_some(),
        current_location,
        location_chain,
        current_projection_position,
        last_location_event_id,
        location_event_chain,
        relevant_events,
        inconsistencies: Vec::new(),
        physical_checksum,
        debug_capability: DebugCapability::mint(),
    }
}

pub fn action_rejection_report(
    validation_report: &ValidationReport,
    state: &PhysicalState,
    checksum_context: &ChecksumContext,
) -> ActionRejectionDebugReport {
    let checksum = compute_physical_checksum(state, checksum_context).checksum;
    ActionRejectionDebugReport {
        report_id: DebugReportId::new(format!(
            "debug.action_rejection.{}",
            validation_report.proposal_id.as_str()
        ))
        .unwrap(),
        proposal_id: validation_report.proposal_id.clone(),
        validation_report_id: validation_report.validation_report_id.clone(),
        failed_stage: validation_report.failed_stage,
        reason_codes: validation_report.reason_codes.clone(),
        actor_visible_summary: validation_report.actor_visible_summary.clone(),
        debug_summary: validation_report.debug_summary.clone(),
        actor_visible_facts: validation_report.actor_visible_facts.clone(),
        debug_only_facts: validation_report.debug_only_facts.clone(),
        precondition_trace: validation_report
            .debug_only_facts
            .iter()
            .map(CheckedFact::render_pair)
            .collect(),
        events_created: validation_report.event_ids.clone(),
        mutation_attempted: false,
        checksum_before: checksum.clone(),
        checksum_after: checksum,
        debug_capability: DebugCapability::mint(),
    }
}

pub fn projection_rebuild_debug_report(
    report_id: DebugReportId,
    rebuild: ProjectionRebuildReport,
) -> ProjectionRebuildDebugReport {
    ProjectionRebuildDebugReport {
        report_id,
        rebuild,
        debug_capability: DebugCapability::mint(),
    }
}

pub fn replay_debug_report(report_id: DebugReportId, replay: ReplayReport) -> ReplayDebugReport {
    ReplayDebugReport {
        report_id,
        replay,
        debug_capability: DebugCapability::mint(),
    }
}

pub fn controller_binding_report(
    report_id: DebugReportId,
    bindings: &ControllerBindings,
) -> ControllerBindingDebugReport {
    let bindings = bindings
        .debug_bindings()
        .into_iter()
        .map(|binding| {
            format!(
                "{}->{:?}@{}",
                binding.binding.controller_id.as_str(),
                binding.binding.bound_actor_id,
                binding.binding.binding_sequence
            )
        })
        .collect();
    ControllerBindingDebugReport {
        report_id,
        bindings,
        debug_capability: DebugCapability::mint(),
    }
}

pub fn phase3a_needs_report(agent_state: &AgentState) -> Phase3ADebugReport {
    let mut rows = Vec::new();
    rows.push(format!(
        "actor_count={}",
        agent_state.needs_by_actor.keys().count()
    ));
    for (actor_id, needs) in &agent_state.needs_by_actor {
        for need in needs.values() {
            rows.push(format!(
                "actor={} need={} value={} band={} cause={}",
                actor_id.as_str(),
                need.kind().stable_id(),
                need.value(),
                need.band().stable_id(),
                need.last_change_cause().stable_id()
            ));
        }
    }
    phase3a_report("debug.phase3a.needs", "Needs", rows, Vec::new(), Vec::new())
}

pub fn phase3a_routines_report(agent_state: &AgentState) -> Phase3ADebugReport {
    let mut rows = vec![
        format!("intention_count={}", agent_state.intentions.len()),
        format!(
            "active_intention_count={}",
            agent_state.active_intention_by_actor.len()
        ),
        format!("routine_count={}", agent_state.routine_executions.len()),
    ];
    rows.extend(
        agent_state
            .active_intention_by_actor
            .iter()
            .map(|(actor_id, intention_id)| {
                format!(
                    "active actor={} intention={}",
                    actor_id.as_str(),
                    intention_id.as_str()
                )
            }),
    );
    rows.extend(agent_state.intentions.values().map(|intention| {
        format!(
            "intention={} actor={} status={:?} goal={} method={} step={}",
            intention.intention_id.as_str(),
            intention.actor_id.as_str(),
            intention.status,
            intention.selected_goal_id.as_str(),
            intention
                .selected_routine_method
                .as_ref()
                .map(|id| id.as_str())
                .unwrap_or("none"),
            intention.current_step.as_deref().unwrap_or("none")
        )
    }));
    rows.extend(agent_state.routine_executions.values().map(|execution| {
        format!(
            "routine={} actor={} template={} family={} step_index={} status={:?} next_tick={} deadline={} fallback_attempts={}",
            execution.execution_id.as_str(),
            execution.actor_id.as_str(),
            execution.template_id.as_str(),
            execution.family.stable_id(),
            execution.current_step_index,
            execution.step_status,
            execution
                .expected_next_progress_tick
                .map(|tick| tick.value().to_string())
                .unwrap_or_else(|| "none".to_string()),
            execution
                .deadline_tick
                .map(|tick| tick.value().to_string())
                .unwrap_or_else(|| "none".to_string()),
            execution.fallback_attempts
        )
    }));
    phase3a_report(
        "debug.phase3a.routines",
        "Routines",
        rows,
        Vec::new(),
        Vec::new(),
    )
}

pub fn phase3a_actor_report(agent_state: &AgentState, actor_id: &ActorId) -> Phase3ADebugReport {
    let mut rows = vec![format!("actor={}", actor_id.as_str())];
    if let Some(needs) = agent_state.needs_by_actor.get(actor_id) {
        rows.extend(needs.values().map(|need| {
            format!(
                "need={} value={} band={}",
                need.kind().stable_id(),
                need.value(),
                need.band().stable_id()
            )
        }));
    } else {
        rows.push("needs=none".to_string());
    }
    let active_intention_id = agent_state.active_intention_by_actor.get(actor_id);
    rows.push(format!(
        "active_intention={}",
        active_intention_id.map(|id| id.as_str()).unwrap_or("none")
    ));
    rows.extend(
        agent_state
            .routine_executions
            .values()
            .filter(|execution| execution.actor_id == *actor_id)
            .map(|execution| {
                format!(
                    "routine={} template={} status={:?}",
                    execution.execution_id.as_str(),
                    execution.template_id.as_str(),
                    execution.step_status
                )
            }),
    );
    let decision_traces = decision_trace_views_for_actor(agent_state, actor_id);
    rows.extend(decision_traces.iter().map(render_decision_trace_row));
    let stuck_diagnostics = stuck_views_for_actor(agent_state, actor_id);
    rows.extend(stuck_diagnostics.iter().map(render_stuck_row));
    phase3a_report(
        &format!("debug.phase3a.actor.{}", actor_id.as_str()),
        "Actor",
        rows,
        decision_traces,
        stuck_diagnostics,
    )
}

pub fn phase3a_planner_report(agent_state: &AgentState, actor_id: &ActorId) -> Phase3ADebugReport {
    let mut rows = vec![
        format!("actor={}", actor_id.as_str()),
        "candidate_goals=typed_decision_trace_records".to_string(),
        "selected_method=typed_decision_trace_records".to_string(),
        "rejected_reasons=typed_decision_trace_records".to_string(),
        "blocked_preconditions=typed_stuck_diagnostic_records".to_string(),
        "hidden_truth_audit=typed_decision_trace_records".to_string(),
        "omniscient_comparison=debug_only_non_authoritative".to_string(),
    ];
    let decision_traces = decision_trace_views_for_actor(agent_state, actor_id);
    if decision_traces.is_empty() {
        rows.push("trace=none".to_string());
    } else {
        rows.extend(decision_traces.iter().map(render_decision_trace_row));
    }
    let stuck_diagnostics = stuck_views_for_actor(agent_state, actor_id);
    if stuck_diagnostics.is_empty() {
        rows.push("blocked=none".to_string());
    } else {
        rows.extend(stuck_diagnostics.iter().map(render_stuck_row));
    }
    phase3a_report(
        &format!("debug.phase3a.planner.{}", actor_id.as_str()),
        "Planner",
        rows,
        decision_traces,
        stuck_diagnostics,
    )
}

pub fn phase3a_stuck_report(agent_state: &AgentState) -> Phase3ADebugReport {
    let mut rows = vec![format!(
        "stuck_diagnostic_count={}",
        agent_state.stuck_diagnostics.len()
    )];
    let mut stuck_diagnostics = stuck_views(agent_state);
    rows.extend(stuck_diagnostics.iter().map(render_stuck_row));
    phase3a_report(
        "debug.phase3a.stuck",
        "Stuck",
        rows,
        Vec::new(),
        std::mem::take(&mut stuck_diagnostics),
    )
}

pub fn no_human_day_debug_report(log: &EventLog) -> NoHumanDayDebugReport {
    let metrics = no_human_day_metrics(log);
    let canonical_summary = metrics.serialize_canonical();
    NoHumanDayDebugReport {
        report_id: DebugReportId::new("debug.phase3a.no_human_day").unwrap(),
        metrics,
        canonical_summary,
        debug_capability: DebugCapability::mint(),
    }
}

fn is_item_location_event(event: &EventEnvelope, item_id: &ItemId) -> bool {
    matches!(
        event.event_type,
        EventKind::ItemRemovedFromContainer
            | EventKind::ItemTakenFromPlace
            | EventKind::ItemPlacedInContainer
            | EventKind::ItemPlacedInPlace
    ) && event
        .payload
        .iter()
        .any(|field| field.key == "item_id" && field.value == item_id.as_str())
}

fn phase3a_report(
    report_id: &str,
    title: &str,
    rows: Vec<String>,
    decision_traces: Vec<Phase3ADecisionTraceView>,
    stuck_diagnostics: Vec<Phase3AStuckDiagnosticView>,
) -> Phase3ADebugReport {
    Phase3ADebugReport {
        report_id: DebugReportId::new(report_id).unwrap(),
        title: title.to_string(),
        rows,
        decision_traces,
        stuck_diagnostics,
        debug_capability: DebugCapability::mint(),
    }
}

fn decision_trace_views_for_actor(
    agent_state: &AgentState,
    actor_id: &ActorId,
) -> Vec<Phase3ADecisionTraceView> {
    agent_state
        .decision_traces
        .values()
        .filter(|trace| &trace.actor_id == actor_id)
        .map(decision_trace_view)
        .collect()
}

fn decision_trace_view(trace: &crate::agent::DecisionTraceRecord) -> Phase3ADecisionTraceView {
    Phase3ADecisionTraceView {
        trace_id: trace.trace_id.as_str().to_string(),
        actor_id: trace.actor_id.clone(),
        window_start_tick: trace.window_start_tick,
        window_end_tick: trace.window_end_tick,
        outcome: trace.outcome.stable_id().to_string(),
        candidate_goal_count: trace.candidate_goal_count,
        hidden_truth_actor_known_only: trace.hidden_truth_audit_result.actor_known_only,
        hidden_truth_notes: trace.hidden_truth_audit_result.notes.clone(),
    }
}

fn stuck_views(agent_state: &AgentState) -> Vec<Phase3AStuckDiagnosticView> {
    agent_state
        .stuck_diagnostics
        .values()
        .map(stuck_view)
        .collect()
}

fn stuck_views_for_actor(
    agent_state: &AgentState,
    actor_id: &ActorId,
) -> Vec<Phase3AStuckDiagnosticView> {
    agent_state
        .stuck_diagnostics
        .values()
        .filter(|diagnostic| &diagnostic.actor_id == actor_id)
        .map(stuck_view)
        .collect()
}

fn stuck_view(diagnostic: &crate::agent::StuckDiagnosticRecord) -> Phase3AStuckDiagnosticView {
    Phase3AStuckDiagnosticView {
        diagnostic_id: diagnostic.diagnostic_id.as_str().to_string(),
        actor_id: diagnostic.actor_id.clone(),
        window_start_tick: diagnostic.window_start_tick,
        window_end_tick: diagnostic.window_end_tick,
        active_need: diagnostic
            .active_need
            .map(|need| need.stable_id().to_string()),
        active_goal_id: diagnostic
            .active_goal_id
            .as_ref()
            .map(|id| id.as_str().to_string()),
        active_intention_id: diagnostic
            .active_intention_id
            .as_ref()
            .map(|id| id.as_str().to_string()),
        routine_template_id: diagnostic
            .routine_template_id
            .as_ref()
            .map(|id| id.as_str().to_string()),
        routine_execution_id: diagnostic
            .routine_execution_id
            .as_ref()
            .map(|id| id.as_str().to_string()),
        routine_step: diagnostic
            .routine_step
            .as_ref()
            .map(crate::agent::RoutineStep::serialize_canonical),
        attempted_action: diagnostic
            .attempted_action
            .as_ref()
            .map(|id| id.as_str().to_string()),
        blocker_category: diagnostic.blocker_category.stable_id().to_string(),
        concrete_blocker: diagnostic.concrete_blocker.clone(),
        actor_known_explanation: diagnostic.actor_known_explanation.clone(),
        debug_only_details: diagnostic.debug_only_details.clone(),
        retry_abandon_fallback_outcome: diagnostic.retry_abandon_fallback_outcome.clone(),
        resulting_status: diagnostic.resulting_status.stable_id().to_string(),
    }
}

fn render_decision_trace_row(trace: &Phase3ADecisionTraceView) -> String {
    format!(
        "trace={} actor={} outcome={} candidates={} actor_known_only={} hidden_truth_notes={}",
        trace.trace_id,
        trace.actor_id.as_str(),
        trace.outcome,
        trace.candidate_goal_count,
        trace.hidden_truth_actor_known_only,
        trace.hidden_truth_notes
    )
}

fn render_stuck_row(diagnostic: &Phase3AStuckDiagnosticView) -> String {
    format!(
        "stuck={} actor={} blocker_category={} outcome={} concrete_blocker={} actor_known={} debug_detail={}",
        diagnostic.diagnostic_id,
        diagnostic.actor_id.as_str(),
        diagnostic.blocker_category,
        diagnostic.resulting_status,
        diagnostic.concrete_blocker,
        diagnostic.actor_known_explanation,
        diagnostic.debug_only_details
    )
}

fn location_summary(state: &PhysicalState, location: &Location) -> String {
    location_chain(state, location).join(" -> ")
}

fn location_chain(state: &PhysicalState, location: &Location) -> Vec<String> {
    match location {
        Location::AtPlace(place_id) => vec![format!("place:{}", place_id.as_str())],
        Location::CarriedBy(actor_id) => {
            let mut chain = vec![format!("actor:{}", actor_id.as_str())];
            if let Some(actor) = state.actors.get(actor_id) {
                chain.push(format!("place:{}", actor.current_place_id.as_str()));
            }
            chain
        }
        Location::InContainer(container_id) => {
            let mut chain = vec![format!("container:{}", container_id.as_str())];
            if let Some(container) = state.containers.get(container_id) {
                chain.extend(location_chain(state, &container.location));
            }
            chain
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actions::pipeline::{run_pipeline, PipelineContext};
    use crate::actions::proposal::{Proposal, ProposalOrigin};
    use crate::actions::registry::ActionRegistry;
    use crate::actions::report::ReportStatus;
    use crate::agent::{
        Intention, IntentionSource, NeedChangeCause, NeedKind, NeedState, RoutineExecution,
        RoutineFamily,
    };
    use crate::events::PayloadField;
    use crate::ids::{
        ActionId, ActorId, CandidateGoalId, ContainerId, ContentManifestId, ContentVersion,
        ControllerId, DecisionTraceId, FixtureId, IntentionId, PlaceId, ProposalId,
        RoutineExecutionId, RoutineTemplateId, SemanticActionId, StuckDiagnosticId,
    };
    use crate::replay::{rebuild_projection, run_replay};
    use crate::scheduler::{OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId};
    use crate::state::{
        ActorBody, AgentState, ContainerState, ControllerMode, ItemState, PhysicalState, PlaceState,
    };
    use crate::time::SimTick;

    fn actor_id() -> ActorId {
        ActorId::new("actor_tomas").unwrap()
    }

    fn item_id() -> ItemId {
        ItemId::new("coin_stack_01").unwrap()
    }

    fn container_id() -> ContainerId {
        ContainerId::new("strongbox_tomas").unwrap()
    }

    fn content_manifest_id() -> ContentManifestId {
        ContentManifestId::new("phase1_manifest").unwrap()
    }

    fn checksum_context() -> ChecksumContext {
        ChecksumContext {
            fixture_id: FixtureId::new("debug_attach_001").unwrap(),
            content_version: ContentVersion::new("phase1_manifest").unwrap(),
            sim_tick: SimTick::ZERO,
            world_stream_position_applied: 1,
        }
    }

    fn ordering_key(action: &str) -> OrderingKey {
        OrderingKey::new(
            SimTick::ZERO,
            SchedulePhase::HumanCommand,
            SchedulerSourceId::Actor(actor_id()),
            ProposalSequence::new(0),
            ActionId::new(action).unwrap(),
            Vec::new(),
            "debug-report-test",
        )
    }

    fn event(id: &str, kind: EventKind, payload: Vec<PayloadField>) -> EventEnvelope {
        let mut event = EventEnvelope::new_v1(
            EventId::new(id).unwrap(),
            kind,
            99,
            99,
            SimTick::ZERO,
            ordering_key("debug_event"),
            content_manifest_id(),
        );
        event.payload = payload;
        event
    }

    fn state() -> PhysicalState {
        let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
        let place_id = PlaceId::new("shop_front").unwrap();
        state.places.insert(
            place_id.clone(),
            PlaceState::new(place_id.clone(), "Shop front"),
        );
        state
            .actors
            .insert(actor_id(), ActorBody::new(actor_id(), place_id.clone()));
        let mut container = ContainerState::fixed_at_place(container_id(), place_id);
        container.is_open = true;
        container.contents.insert(item_id());
        state.containers.insert(container_id(), container);
        state.items.insert(
            item_id(),
            ItemState::new(item_id(), Location::InContainer(container_id())),
        );
        state
    }

    #[test]
    fn debug_report_channel_routing_rejects_forged_non_debug_capability() {
        let state = state();
        let log = EventLog::new();
        let agent_state = AgentState::default();
        let rebuild = rebuild_projection(&state, &agent_state, &log, &checksum_context(), None);
        let replay = run_replay(
            &state,
            &agent_state,
            &log,
            &checksum_context(),
            Some(&state),
            Some(compute_physical_checksum(&state, &checksum_context()).checksum),
            None,
        );
        let mut bindings = ControllerBindings::new();
        bindings.attach(
            ControllerId::new("controller_human").unwrap(),
            actor_id(),
            ControllerMode::Embodied,
            SimTick::ZERO,
            &mut EventLog::new(),
            content_manifest_id(),
        );

        let routed = vec![
            debug_report_channel(&DebugReportRoute::ItemLocation(item_location_report(
                &state,
                &log,
                &item_id(),
                &checksum_context(),
            ))),
            debug_report_channel(&DebugReportRoute::ActionRejection(action_rejection_report(
                &ValidationReport {
                    validation_report_id: ValidationReportId::new("report_route").unwrap(),
                    proposal_id: ProposalId::new("proposal_route").unwrap(),
                    actor_id: Some(actor_id()),
                    action_id: ActionId::new("wait").unwrap(),
                    target_ids: Vec::new(),
                    status: ReportStatus::Rejected,
                    failed_stage: Some(PipelineStage::PhysicalPreconditionValidation),
                    reason_codes: vec![ReasonCode::WorldStateMismatch],
                    checked_facts: Vec::new(),
                    actor_visible_summary: "Action rejected.".to_string(),
                    debug_summary: "debug-only validator fact".to_string(),
                    actor_visible_facts: vec![CheckedFact::new("actor_id", actor_id().as_str())],
                    debug_only_facts: vec![CheckedFact::new("debug_only", "validator_fact")],
                    would_mutate: false,
                    event_ids: Vec::new(),
                    checksum_before: None,
                    checksum_after: None,
                },
                &state,
                &checksum_context(),
            ))),
            debug_report_channel(&DebugReportRoute::ProjectionRebuild(Box::new(
                projection_rebuild_debug_report(
                    DebugReportId::new("debug.rebuild.route").unwrap(),
                    rebuild,
                ),
            ))),
            debug_report_channel(&DebugReportRoute::Replay(Box::new(replay_debug_report(
                DebugReportId::new("debug.replay.route").unwrap(),
                replay,
            )))),
            debug_report_channel(&DebugReportRoute::ControllerBinding(
                controller_binding_report(
                    DebugReportId::new("debug.controller.route").unwrap(),
                    &bindings,
                ),
            )),
            debug_report_channel(&DebugReportRoute::Phase3A(phase3a_needs_report(
                &agent_state,
            ))),
            debug_report_channel(&DebugReportRoute::NoHumanDay(no_human_day_debug_report(
                &log,
            ))),
        ];
        assert_eq!(
            routed,
            vec![
                Some("debug:item_location"),
                Some("debug:action_rejection"),
                Some("debug:projection_rebuild"),
                Some("debug:replay"),
                Some("debug:controller_binding"),
                Some("debug:phase3a"),
                Some("debug:no_human_day"),
            ]
        );

        for forged in forged_non_debug_reports(&state, &agent_state, &log) {
            assert_eq!(
                debug_report_channel(&forged),
                None,
                "forged non-debug report must not route: {forged:?}"
            );
        }
    }

    #[test]
    fn item_location_report_filters_exact_item_location_events() {
        let state = state();
        let mut log = EventLog::new();
        log.append(event(
            "event_unrelated_wait",
            EventKind::ActorWaited,
            vec![PayloadField::new("item_id", item_id().as_str())],
        ))
        .unwrap();
        log.append(event(
            "event_wrong_item_take",
            EventKind::ItemTakenFromPlace,
            vec![PayloadField::new("item_id", "coin_stack_02")],
        ))
        .unwrap();
        log.append(event(
            "event_right_item_take",
            EventKind::ItemTakenFromPlace,
            vec![PayloadField::new("item_id", item_id().as_str())],
        ))
        .unwrap();

        let report = item_location_report(&state, &log, &item_id(), &checksum_context());

        assert_eq!(
            item_location_event_route(&report),
            vec!["item_event:event_right_item_take:ItemTakenFromPlace"]
        );
        assert_eq!(
            report.last_location_event_id.unwrap().as_str(),
            "event_right_item_take"
        );
        assert_eq!(report.current_projection_position, Some(2));
    }

    #[test]
    fn item_location_report_names_last_location_event() {
        let mut state = state();
        let mut log = EventLog::new();
        let mut registry = ActionRegistry::new();
        registry.register_phase1_take_place();
        let mut proposal = Proposal::new(
            ProposalId::new("proposal_take").unwrap(),
            ProposalOrigin::Test,
            Some(actor_id()),
            ActionId::new("take").unwrap(),
            SimTick::ZERO,
        );
        proposal.target_ids = vec!["coin_stack_01".to_string(), "strongbox_tomas".to_string()];
        let mut context = PipelineContext {
            registry: &registry,
            state: &mut state,
            agent_state: Box::leak(Box::new(crate::state::AgentState::default())),
            log: &mut log,
            controller_bindings: None,
            epistemic_projection: None,
            content_manifest_id: content_manifest_id(),
            ordering_key: crate::scheduler::OrderingKey::new(
                SimTick::ZERO,
                crate::scheduler::SchedulePhase::HumanCommand,
                crate::scheduler::SchedulerSourceId::Actor(actor_id()),
                crate::scheduler::ProposalSequence::new(0),
                ActionId::new("take").unwrap(),
                proposal.target_ids.clone(),
                "tie",
            ),
        };
        run_pipeline(&mut context, &proposal);

        let report = item_location_report(&state, &log, &item_id(), &checksum_context());

        assert!(report.debug_only());
        assert_eq!(
            report.current_location.unwrap(),
            "actor:actor_tomas -> place:shop_front"
        );
        assert_eq!(
            report.last_location_event_id.unwrap().as_str(),
            "event.item_removed_from_container.proposal_take"
        );
        assert_eq!(report.location_event_chain.len(), 1);
    }

    #[test]
    fn rejection_report_preserves_failed_stage_and_mutates_nothing() {
        let state = state();
        let before_actor = state.actors[&actor_id()].clone();
        let before_checksum = compute_physical_checksum(&state, &checksum_context()).checksum;
        let mut log = EventLog::new();
        let registry = ActionRegistry::new();
        let proposal = Proposal::new(
            ProposalId::new("proposal_bad").unwrap(),
            ProposalOrigin::Test,
            Some(actor_id()),
            ActionId::new("unknown_action").unwrap(),
            SimTick::ZERO,
        );
        let mut mutable_state = state.clone();
        let mut context = PipelineContext {
            registry: &registry,
            state: &mut mutable_state,
            agent_state: Box::leak(Box::new(crate::state::AgentState::default())),
            log: &mut log,
            controller_bindings: None,
            epistemic_projection: None,
            content_manifest_id: content_manifest_id(),
            ordering_key: crate::scheduler::OrderingKey::new(
                SimTick::ZERO,
                crate::scheduler::SchedulePhase::HumanCommand,
                crate::scheduler::SchedulerSourceId::Actor(actor_id()),
                crate::scheduler::ProposalSequence::new(0),
                ActionId::new("unknown_action").unwrap(),
                Vec::new(),
                "tie",
            ),
        };
        let result = run_pipeline(&mut context, &proposal);
        let report = action_rejection_report(&result.report, &mutable_state, &checksum_context());
        let after_checksum =
            compute_physical_checksum(&mutable_state, &checksum_context()).checksum;

        assert!(report.debug_only());
        assert_eq!(
            report.failed_stage,
            Some(crate::actions::pipeline::PipelineStage::ActionDefinitionLookup)
        );
        assert!(!report.actor_visible_facts.is_empty());
        assert!(report.debug_only_facts.is_empty());
        assert!(report.precondition_trace.is_empty());
        assert!(!report.mutation_attempted);
        assert_eq!(report.checksum_before, report.checksum_after);
        assert_eq!(mutable_state.actors[&actor_id()], before_actor);
        assert_eq!(before_checksum, after_checksum);
    }

    #[test]
    fn fixture_origin_item_report_is_debug_only_and_read_only() {
        let state = state();
        let before = state.clone();
        let log = EventLog::new();
        let report = item_location_report(&state, &log, &item_id(), &checksum_context());

        assert!(report.debug_only());
        assert_eq!(
            report.last_location_event_id.unwrap().as_str(),
            "fixture_origin.coin_stack_01"
        );
        assert_eq!(state, before);
    }

    #[test]
    fn debug_reports_phase3a_surfaces_are_deterministic_and_read_only() {
        let actor_mara = ActorId::new("actor_mara").unwrap();
        let trace_id = DecisionTraceId::new("trace_mara_food").unwrap();
        let intention_id = IntentionId::new("intention_mara_eat").unwrap();
        let routine_execution_id = RoutineExecutionId::new("routine_exec_mara_eat").unwrap();
        let routine_template_id = RoutineTemplateId::new("routine_mara_food_unavailable").unwrap();
        let mut agent_state = AgentState::default();
        agent_state.needs_by_actor.insert(
            actor_mara.clone(),
            [(
                NeedKind::Hunger,
                NeedState::initial(NeedKind::Hunger, 810, NeedChangeCause::FixtureInitial),
            )]
            .into(),
        );
        agent_state.intentions.insert(
            intention_id.clone(),
            Intention::adopt(
                intention_id.clone(),
                actor_mara.clone(),
                IntentionSource::RoutineDuty,
                CandidateGoalId::new("goal_mara_eat").unwrap(),
                Some(routine_template_id.clone()),
                Some("consume_accessible_food".to_string()),
                3,
                SimTick::new(4),
                trace_id.clone(),
            ),
        );
        agent_state
            .active_intention_by_actor
            .insert(actor_mara.clone(), intention_id);
        agent_state.routine_executions.insert(
            routine_execution_id.clone(),
            RoutineExecution::new(
                routine_execution_id,
                actor_mara.clone(),
                routine_template_id,
                RoutineFamily::EatMeal,
                SimTick::new(4),
                Some(SimTick::new(5)),
                Some(SimTick::new(10)),
                None,
                trace_id.clone(),
            ),
        );
        agent_state.decision_traces.insert(
            trace_id.clone(),
            crate::agent::DecisionTraceRecord {
                trace_id,
                actor_id: actor_mara.clone(),
                window_start_tick: SimTick::new(4),
                window_end_tick: SimTick::new(10),
                outcome: crate::agent::DecisionOutcome::Failed,
                candidate_goal_count: 2,
                actor_known_context_hash: Some(crate::checksum::HolderKnownContextHash::from_canonical_lines(&[])),
                actor_known_inputs: Vec::new(),
                local_plan_id: None,
                proposal_ancestry: Vec::new(),
                hidden_truth_audit_result: crate::agent::HiddenTruthAudit {
                    actor_known_only: true,
                    notes: "candidate_goals=eat,find_food;selected_method=none;rejected_reasons=empty_pantry;hidden_truth_audit=actor_known_only".to_string(),
                },
                typed_diagnostic: crate::agent::TypedDiagnosticFields::decision_default(false),
            },
        );
        agent_state.stuck_diagnostics.insert(
            StuckDiagnosticId::new("stuck_mara_food").unwrap(),
            crate::agent::StuckDiagnostic::new(
                StuckDiagnosticId::new("stuck_mara_food").unwrap(),
                actor_mara.clone(),
                SimTick::new(4),
                SimTick::new(10),
                Some(NeedKind::Hunger),
                Some(CandidateGoalId::new("goal_mara_eat").unwrap()),
                Some(IntentionId::new("intention_mara_eat").unwrap()),
                Some(RoutineTemplateId::new("routine_mara_food_unavailable").unwrap()),
                Some(RoutineExecutionId::new("routine_exec_mara_eat").unwrap()),
                None,
                Some(SemanticActionId::new("eat.food_empty_pantry_mara").unwrap()),
                crate::agent::BlockerCategory::Resource,
                "empty_pantry",
                "Mara knows no reachable food",
                "blocked precondition: no actor-known serving",
                "fallback_to_find_food",
                crate::agent::StuckResultingStatus::Replanning,
            ),
        );
        let before = agent_state.clone();

        let needs = phase3a_needs_report(&agent_state);
        let routines = phase3a_routines_report(&agent_state);
        let planner = phase3a_planner_report(&agent_state, &actor_mara);
        let stuck = phase3a_stuck_report(&agent_state);
        let actor = phase3a_actor_report(&agent_state, &actor_mara);
        let actor_tomas = phase3a_actor_report(&agent_state, &actor_id());

        for report in [&needs, &routines, &planner, &stuck, &actor] {
            assert!(report.debug_only());
        }
        assert!(needs
            .rows
            .iter()
            .any(|row| row.contains("need=hunger value=810 band=severe")));
        assert!(routines
            .rows
            .iter()
            .any(|row| row.contains("routine_exec_mara_eat")));
        assert!(planner
            .rows
            .iter()
            .any(|row| row.contains("candidate_goals")));
        assert_eq!(planner.decision_traces.len(), 1);
        assert_eq!(planner.decision_traces[0].trace_id, "trace_mara_food");
        assert_eq!(planner.decision_traces[0].outcome, "failed");
        assert!(planner.decision_traces[0].hidden_truth_actor_known_only);
        assert!(planner
            .rows
            .iter()
            .any(|row| row.contains("typed_decision_trace_records")));
        assert!(stuck
            .rows
            .iter()
            .any(|row| row.contains("blocker_category=resource")));
        assert_eq!(stuck.stuck_diagnostics.len(), 1);
        assert_eq!(stuck.stuck_diagnostics[0].blocker_category, "resource");
        assert_eq!(
            stuck.stuck_diagnostics[0].actor_known_explanation,
            "Mara knows no reachable food"
        );
        assert!(stuck
            .rows
            .iter()
            .any(|row| row.contains("outcome=replanning")));
        assert!(actor
            .rows
            .iter()
            .any(|row| row.contains("actor=actor_mara")));
        assert_eq!(
            actor_debug_route(&actor),
            vec![
                "row:actor=actor_mara".to_string(),
                "trace:trace_mara_food:actor_mara:failed:2:true:candidate_goals=eat,find_food;selected_method=none;rejected_reasons=empty_pantry;hidden_truth_audit=actor_known_only".to_string(),
                "stuck:stuck_mara_food:actor_mara:resource:replanning:empty_pantry:Mara knows no reachable food:blocked precondition: no actor-known serving".to_string(),
            ]
        );
        assert!(actor.rows.iter().any(|row| {
            row == "trace=trace_mara_food actor=actor_mara outcome=failed candidates=2 actor_known_only=true hidden_truth_notes=candidate_goals=eat,find_food;selected_method=none;rejected_reasons=empty_pantry;hidden_truth_audit=actor_known_only"
        }));
        assert!(actor.rows.iter().any(|row| {
            row == "routine=routine_exec_mara_eat template=routine_mara_food_unavailable status=NotStarted"
        }));
        assert!(actor_tomas
            .rows
            .iter()
            .any(|row| row == "actor=actor_tomas"));
        assert!(
            actor_debug_route(&actor_tomas)
                .iter()
                .all(|row| !row.contains("trace_mara_food") && !row.contains("stuck_mara_food")),
            "actor report for Tomas must exclude Mara-only trace/stuck diagnostics: {actor_tomas:?}"
        );
        assert!(
            actor_tomas
                .rows
                .iter()
                .all(|row| !row.contains("routine_exec_mara_eat")),
            "actor report for Tomas must exclude Mara-only routine rows: {actor_tomas:?}"
        );
        assert!(actor_tomas.decision_traces.is_empty());
        assert!(actor_tomas.stuck_diagnostics.is_empty());
        assert_eq!(agent_state, before);
        assert_eq!(phase3a_planner_report(&agent_state, &actor_mara), planner);

        let report_source = include_str!("debug_reports.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("module source has non-test section");
        for forbidden in [
            ".apply_delta(",
            ".insert_belief(",
            ".insert_observation(",
            ".append(",
            "run_pipeline(",
        ] {
            assert!(
                !report_source.contains(forbidden),
                "debug report generation must not call {forbidden}"
            );
        }
    }

    #[test]
    fn debug_reports_no_human_day_summary_is_debug_only_and_read_only() {
        let log = EventLog::new();
        let before = log.clone();

        let report = no_human_day_debug_report(&log);

        assert!(report.debug_only());
        assert_eq!(report.metrics.projection_version, "no_human_day_metrics_v1");
        assert!(report.canonical_summary.contains("no_human_day_metrics_v1"));
        assert_eq!(log, before);
    }

    #[derive(Debug)]
    enum DebugReportRoute {
        ItemLocation(ItemLocationDebugReport),
        ActionRejection(ActionRejectionDebugReport),
        ProjectionRebuild(Box<ProjectionRebuildDebugReport>),
        Replay(Box<ReplayDebugReport>),
        ControllerBinding(ControllerBindingDebugReport),
        Phase3A(Phase3ADebugReport),
        NoHumanDay(NoHumanDayDebugReport),
    }

    fn debug_report_channel(report: &DebugReportRoute) -> Option<&'static str> {
        match report {
            DebugReportRoute::ItemLocation(report) if report.debug_only() => {
                Some("debug:item_location")
            }
            DebugReportRoute::ActionRejection(report) if report.debug_only() => {
                Some("debug:action_rejection")
            }
            DebugReportRoute::ProjectionRebuild(report) if report.debug_only() => {
                Some("debug:projection_rebuild")
            }
            DebugReportRoute::Replay(report) if report.debug_only() => Some("debug:replay"),
            DebugReportRoute::ControllerBinding(report) if report.debug_only() => {
                Some("debug:controller_binding")
            }
            DebugReportRoute::Phase3A(report) if report.debug_only() => Some("debug:phase3a"),
            DebugReportRoute::NoHumanDay(report) if report.debug_only() => {
                Some("debug:no_human_day")
            }
            _ => None,
        }
    }

    fn forged_non_debug_reports(
        state: &PhysicalState,
        agent_state: &AgentState,
        log: &EventLog,
    ) -> Vec<DebugReportRoute> {
        let non_debug = DebugCapability::test_non_debug;
        let checksum = compute_physical_checksum(state, &checksum_context()).checksum;
        let rebuild = rebuild_projection(state, agent_state, log, &checksum_context(), None);
        let replay = run_replay(
            state,
            agent_state,
            log,
            &checksum_context(),
            Some(state),
            Some(checksum.clone()),
            None,
        );
        vec![
            DebugReportRoute::ItemLocation(ItemLocationDebugReport {
                report_id: DebugReportId::new("debug.item_location.forged").unwrap(),
                item_id: item_id(),
                exists: true,
                current_location: Some("container:strongbox_tomas".to_string()),
                location_chain: Vec::new(),
                current_projection_position: None,
                last_location_event_id: None,
                location_event_chain: Vec::new(),
                relevant_events: Vec::new(),
                inconsistencies: Vec::new(),
                physical_checksum: checksum.clone(),
                debug_capability: non_debug(),
            }),
            DebugReportRoute::ActionRejection(ActionRejectionDebugReport {
                report_id: DebugReportId::new("debug.action_rejection.forged").unwrap(),
                proposal_id: ProposalId::new("proposal_forged").unwrap(),
                validation_report_id: ValidationReportId::new("report_forged").unwrap(),
                failed_stage: Some(PipelineStage::PhysicalPreconditionValidation),
                reason_codes: vec![ReasonCode::WorldStateMismatch],
                actor_visible_summary: "forged".to_string(),
                debug_summary: "forged".to_string(),
                actor_visible_facts: Vec::new(),
                debug_only_facts: Vec::new(),
                precondition_trace: Vec::new(),
                events_created: Vec::new(),
                mutation_attempted: false,
                checksum_before: checksum.clone(),
                checksum_after: checksum.clone(),
                debug_capability: non_debug(),
            }),
            DebugReportRoute::ProjectionRebuild(Box::new(ProjectionRebuildDebugReport {
                report_id: DebugReportId::new("debug.rebuild.forged").unwrap(),
                rebuild,
                debug_capability: non_debug(),
            })),
            DebugReportRoute::Replay(Box::new(ReplayDebugReport {
                report_id: DebugReportId::new("debug.replay.forged").unwrap(),
                replay,
                debug_capability: non_debug(),
            })),
            DebugReportRoute::ControllerBinding(ControllerBindingDebugReport {
                report_id: DebugReportId::new("debug.controller.forged").unwrap(),
                bindings: vec!["controller_human->actor_tomas@0".to_string()],
                debug_capability: non_debug(),
            }),
            DebugReportRoute::Phase3A(Phase3ADebugReport {
                report_id: DebugReportId::new("debug.phase3a.forged").unwrap(),
                title: "Forged".to_string(),
                rows: vec!["forged".to_string()],
                decision_traces: Vec::new(),
                stuck_diagnostics: Vec::new(),
                debug_capability: non_debug(),
            }),
            DebugReportRoute::NoHumanDay(NoHumanDayDebugReport {
                report_id: DebugReportId::new("debug.no_human_day.forged").unwrap(),
                metrics: no_human_day_metrics(log),
                canonical_summary: "forged".to_string(),
                debug_capability: non_debug(),
            }),
        ]
    }

    fn item_location_event_route(report: &ItemLocationDebugReport) -> Vec<String> {
        report
            .relevant_events
            .iter()
            .map(|event| {
                format!(
                    "item_event:{}:{:?}",
                    event.event_id.as_str(),
                    event.event_type
                )
            })
            .collect()
    }

    fn actor_debug_route(report: &Phase3ADebugReport) -> Vec<String> {
        let mut routed = report
            .rows
            .iter()
            .filter(|row| row.starts_with("actor="))
            .map(|row| format!("row:{row}"))
            .collect::<Vec<_>>();
        routed.extend(report.decision_traces.iter().map(|trace| {
            format!(
                "trace:{}:{}:{}:{}:{}:{}",
                trace.trace_id,
                trace.actor_id.as_str(),
                trace.outcome,
                trace.candidate_goal_count,
                trace.hidden_truth_actor_known_only,
                trace.hidden_truth_notes
            )
        }));
        routed.extend(report.stuck_diagnostics.iter().map(|diagnostic| {
            format!(
                "stuck:{}:{}:{}:{}:{}:{}:{}",
                diagnostic.diagnostic_id,
                diagnostic.actor_id.as_str(),
                diagnostic.blocker_category,
                diagnostic.resulting_status,
                diagnostic.concrete_blocker,
                diagnostic.actor_known_explanation,
                diagnostic.debug_only_details
            )
        }));
        routed
    }
}
