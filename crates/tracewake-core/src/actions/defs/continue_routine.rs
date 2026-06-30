use crate::actions::defs::ActionRejection;
use crate::actions::pipeline::PipelineStage;
use crate::actions::proposal::Proposal;
use crate::actions::report::{CheckedFact, ReasonCode};
use crate::agent::IntentionStatus;
use crate::events::{EventCause, EventEnvelope, EventKind, PayloadField, EVENT_SCHEMA_V1};
use crate::ids::{ActionId, ActorId, ContentManifestId, EventId, IntentionId, RoutineExecutionId};
use crate::scheduler::OrderingKey;
use crate::state::{AgentState, PhysicalState};

pub fn build_continue_routine_event(
    state: &PhysicalState,
    agent_state: &AgentState,
    proposal: &Proposal,
    ordering_key: &OrderingKey,
    content_manifest_id: &ContentManifestId,
) -> Result<EventEnvelope, ActionRejection> {
    let actor_id = proposal.actor_id.clone().ok_or_else(actor_missing)?;
    if !state.actors.contains_key(&actor_id) {
        return Err(actor_missing());
    }

    let intention_id = parse_intention_id(proposal)?;
    validate_authoritative_intention(agent_state, &actor_id, &intention_id)?;

    let next_action_id = parse_next_action_id(proposal)?;
    let routine_execution_id = proposal
        .parameters
        .get("routine_execution_id")
        .map(|value| RoutineExecutionId::new(value.clone()))
        .transpose()
        .map_err(|_| {
            reject(
                PipelineStage::TargetBinding,
                ReasonCode::RoutineStepBlocked,
                "routine_execution_id",
                proposal
                    .parameters
                    .get("routine_execution_id")
                    .map(String::as_str)
                    .unwrap_or(""),
                "That routine step is invalid.",
                "continue_routine routine_execution_id was not stable",
            )
        })?;

    let mut event = EventEnvelope::new_caused_v1(
        EventId::new(format!(
            "event.continue_routine_proposed.{}",
            proposal.proposal_id.as_str()
        ))
        .unwrap(),
        EventKind::ContinueRoutineProposed,
        0,
        0,
        proposal.requested_tick,
        ordering_key.clone(),
        content_manifest_id.clone(),
        vec![EventCause::Proposal(proposal.proposal_id.clone())],
    )
    .unwrap();
    event.actor_id = Some(actor_id.clone());
    event.proposal_id = Some(proposal.proposal_id.clone());
    event.participants = vec![actor_id.to_string(), intention_id.to_string()];
    event.payload = vec![
        PayloadField::new("schema_version", EVENT_SCHEMA_V1),
        PayloadField::new("payload_schema_version", "1"),
        PayloadField::new("actor_id", actor_id.as_str()),
        PayloadField::new("intention_id", intention_id.as_str()),
        PayloadField::new("next_action_id", next_action_id.as_str()),
        PayloadField::new("routes_through_shared_pipeline", "true"),
        PayloadField::new("intention_mutated", "false"),
        PayloadField::new("behavioral_progress", "false"),
    ];
    if let Some(routine_execution_id) = routine_execution_id {
        event.payload.push(PayloadField::new(
            "routine_execution_id",
            routine_execution_id.as_str(),
        ));
    }
    event.effects_summary =
        "continue routine marker only; behavioral progress requires next ordinary action"
            .to_string();
    Ok(event)
}

fn validate_authoritative_intention(
    agent_state: &AgentState,
    actor_id: &ActorId,
    intention_id: &IntentionId,
) -> Result<(), ActionRejection> {
    let Some(active_intention_id) = agent_state.active_intention_by_actor.get(actor_id) else {
        return Err(reject(
            PipelineStage::TargetBinding,
            ReasonCode::NoCurrentIntention,
            "active_intention_id",
            "",
            "There is no active routine to continue.",
            "continue_routine actor has no authoritative active intention",
        ));
    };
    if active_intention_id != intention_id {
        return Err(reject(
            PipelineStage::TargetBinding,
            ReasonCode::NoCurrentIntention,
            "active_intention_id",
            intention_id.as_str(),
            "There is no active routine to continue.",
            "continue_routine proposal did not match authoritative active intention",
        ));
    }
    let Some(intention) = agent_state.intentions.get(intention_id) else {
        return Err(reject(
            PipelineStage::TargetBinding,
            ReasonCode::NoCurrentIntention,
            "active_intention_id",
            intention_id.as_str(),
            "There is no active routine to continue.",
            "continue_routine authoritative intention record was missing",
        ));
    };
    if intention.status != IntentionStatus::Active {
        return Err(reject(
            PipelineStage::ReservationConflictCheck,
            ReasonCode::IntentionTerminal,
            "authoritative_intention_status",
            intention.status.stable_id(),
            "That routine is no longer active.",
            "continue_routine was requested for a terminal intention",
        ));
    }
    Ok(())
}

fn parse_intention_id(proposal: &Proposal) -> Result<IntentionId, ActionRejection> {
    let Some(value) = proposal.parameters.get("active_intention_id") else {
        return Err(reject(
            PipelineStage::TargetBinding,
            ReasonCode::NoCurrentIntention,
            "active_intention_id",
            "",
            "There is no active routine to continue.",
            "continue_routine proposal did not name an active intention",
        ));
    };
    IntentionId::new(value.clone()).map_err(|_| {
        reject(
            PipelineStage::TargetBinding,
            ReasonCode::NoCurrentIntention,
            "active_intention_id",
            value,
            "There is no active routine to continue.",
            "continue_routine active_intention_id was not stable",
        )
    })
}

fn parse_next_action_id(proposal: &Proposal) -> Result<ActionId, ActionRejection> {
    let Some(value) = proposal.parameters.get("next_action_id") else {
        return Err(reject(
            PipelineStage::TargetBinding,
            ReasonCode::RoutineStepBlocked,
            "next_action_id",
            "",
            "That routine step cannot produce an action yet.",
            "continue_routine proposal did not include next_action_id",
        ));
    };
    ActionId::new(value.clone()).map_err(|_| {
        reject(
            PipelineStage::TargetBinding,
            ReasonCode::RoutineStepBlocked,
            "next_action_id",
            value,
            "That routine step cannot produce an action yet.",
            "continue_routine next_action_id was not stable",
        )
    })
}

fn actor_missing() -> ActionRejection {
    ActionRejection::new(
        PipelineStage::ActorLookup,
        ReasonCode::ActorNotFound,
        Vec::new(),
        "That actor cannot continue a routine.",
        "actor was missing",
    )
}

fn reject(
    failed_stage: PipelineStage,
    reason_code: ReasonCode,
    key: &'static str,
    value: &str,
    actor_visible_summary: impl Into<String>,
    debug_summary: impl Into<String>,
) -> ActionRejection {
    ActionRejection::new(
        failed_stage,
        reason_code,
        vec![CheckedFact::new(key, value)],
        actor_visible_summary,
        debug_summary,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actions::pipeline::{run_pipeline, PipelineContext};
    use crate::actions::proposal::{ProposalOrigin, ProposalSource, ProposalSourceContext};
    use crate::actions::registry::ActionRegistry;
    use crate::actions::report::ReportStatus;
    use crate::agent::{current_place_knowledge_context, Intention, IntentionSource};
    use crate::controller::ControllerBindings;
    use crate::events::log::EventLog;
    use crate::events::EventKind;
    use crate::ids::{
        ActorId, CandidateGoalId, ControllerId, DecisionTraceId, PlaceId, ProposalId,
        RoutineTemplateId, SemanticActionId, ViewModelId,
    };
    use crate::scheduler::{ProposalSequence, SchedulePhase, SchedulerSourceId};
    use crate::state::{ActorBody, ControllerMode, PlaceState};
    use crate::time::SimTick;

    fn actor_id() -> ActorId {
        ActorId::new("actor_tomas").unwrap()
    }

    fn state() -> PhysicalState {
        let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
        let office_id = PlaceId::new("office").unwrap();
        let hallway_id = PlaceId::new("hallway").unwrap();
        let mut office = PlaceState::new(office_id.clone(), "Office");
        office.adjacent_place_ids.insert(hallway_id.clone());
        let mut hallway = PlaceState::new(hallway_id.clone(), "Hallway");
        hallway.adjacent_place_ids.insert(office_id.clone());
        state.places.insert(office_id.clone(), office);
        state.places.insert(hallway_id, hallway);
        state
            .actors
            .insert(actor_id(), ActorBody::new(actor_id(), office_id));
        state
    }

    fn proposal(origin: ProposalOrigin) -> Proposal {
        let mut proposal = Proposal::new(
            ProposalId::new("proposal_continue").unwrap(),
            origin,
            Some(actor_id()),
            ActionId::new("continue_routine").unwrap(),
            SimTick::new(5),
        );
        proposal.parameters.insert(
            "active_intention_id".to_string(),
            "intention_workday".to_string(),
        );
        proposal
            .parameters
            .insert("next_action_id".to_string(), "work_block".to_string());
        proposal
    }

    fn agent_state_with_intention(status: IntentionStatus) -> AgentState {
        let mut agent_state = AgentState::default();
        let intention_id = IntentionId::new("intention_workday").unwrap();
        let mut intention = Intention::adopt(
            intention_id.clone(),
            actor_id(),
            IntentionSource::FixtureRoutineAssignment,
            CandidateGoalId::new("goal_workday").unwrap(),
            Some(RoutineTemplateId::new("routine_workday").unwrap()),
            Some("work_block".to_string()),
            5,
            SimTick::new(1),
            DecisionTraceId::new("trace_workday").unwrap(),
        );
        intention.status = status;
        agent_state
            .active_intention_by_actor
            .insert(actor_id(), intention_id.clone());
        agent_state.intentions.insert(intention_id, intention);
        agent_state
    }

    fn agent_state() -> AgentState {
        agent_state_with_intention(IntentionStatus::Active)
    }

    fn attach_tui_source(proposal: &mut Proposal, state: &PhysicalState) {
        let content_manifest_id = ContentManifestId::new("phase3a_manifest").unwrap();
        let context = current_place_knowledge_context(
            state,
            None,
            &actor_id(),
            proposal.requested_tick,
            &content_manifest_id,
            0,
        );
        let source_view_model_id = ViewModelId::new("view.actor_tomas.5").unwrap();
        proposal.source_view_model_id = Some(source_view_model_id.clone());
        proposal.source = Some(ProposalSource::TuiSemanticAction(ProposalSourceContext {
            source_view_model_id,
            holder_known_context_id: context.holder_known_context_id().clone(),
            holder_known_context_hash: context.holder_known_context_hash().clone(),
            holder_known_context_frontier: context.event_frontier(),
            context_tick: proposal.requested_tick,
            actor_id: actor_id(),
            semantic_action_id: SemanticActionId::new("continue.routine.intention_workday")
                .unwrap(),
            provenance_ancestry: Vec::new(),
        }));
    }

    fn ordering_key() -> OrderingKey {
        OrderingKey::new(
            SimTick::new(5),
            SchedulePhase::HumanCommand,
            SchedulerSourceId::Actor(actor_id()),
            ProposalSequence::new(0),
            ActionId::new("continue_routine").unwrap(),
            Vec::new(),
            "continue_routine",
        )
    }

    #[test]
    fn continue_without_active_intention_rejects_with_reason() {
        let mut proposal = proposal(ProposalOrigin::Test);
        proposal.parameters.remove("active_intention_id");
        let rejection = build_continue_routine_event(
            &state(),
            &agent_state(),
            &proposal,
            &ordering_key(),
            &ContentManifestId::new("phase3a_manifest").unwrap(),
        )
        .unwrap_err();

        assert_eq!(rejection.reason_code, ReasonCode::NoCurrentIntention);
    }

    #[test]
    fn continue_terminal_intention_rejects_with_summary_reason() {
        let proposal = proposal(ProposalOrigin::Test);
        let rejection = build_continue_routine_event(
            &state(),
            &agent_state_with_intention(IntentionStatus::Completed),
            &proposal,
            &ordering_key(),
            &ContentManifestId::new("phase3a_manifest").unwrap(),
        )
        .unwrap_err();

        assert_eq!(rejection.reason_code, ReasonCode::IntentionTerminal);
    }

    #[test]
    fn continue_produces_next_action_without_mutating_intention() {
        let event = build_continue_routine_event(
            &state(),
            &agent_state(),
            &proposal(ProposalOrigin::Test),
            &ordering_key(),
            &ContentManifestId::new("phase3a_manifest").unwrap(),
        )
        .unwrap();

        assert_eq!(event.event_type, EventKind::ContinueRoutineProposed);
        assert!(event
            .payload
            .iter()
            .any(|field| field.key == "next_action_id" && field.value == "work_block"));
        assert!(event
            .payload
            .iter()
            .any(|field| field.key == "intention_mutated" && field.value == "false"));
        assert!(event
            .payload
            .iter()
            .any(|field| field.key == "behavioral_progress" && field.value == "false"));
    }

    #[test]
    fn continue_marker_payload_contract_stays_non_progress() {
        let event = build_continue_routine_event(
            &state(),
            &agent_state(),
            &proposal(ProposalOrigin::Test),
            &ordering_key(),
            &ContentManifestId::new("phase3a_manifest").unwrap(),
        )
        .unwrap();
        let payload_value = |key: &str| {
            event
                .payload
                .iter()
                .find(|field| field.key == key)
                .map(|field| field.value.as_str())
        };

        assert_eq!(event.event_type, EventKind::ContinueRoutineProposed);
        assert_eq!(payload_value("intention_mutated"), Some("false"));
        assert_eq!(payload_value("behavioral_progress"), Some("false"));
        assert_eq!(
            payload_value("routes_through_shared_pipeline"),
            Some("true")
        );
        assert_eq!(payload_value("next_action_id"), Some("work_block"));
        assert_eq!(
            event.effects_summary,
            "continue routine marker only; behavioral progress requires next ordinary action"
        );
        assert!(event
            .causes
            .iter()
            .all(|cause| matches!(cause, EventCause::Proposal(_))));
    }

    #[test]
    fn human_and_scheduler_continue_produce_same_next_action() {
        let human = build_continue_routine_event(
            &state(),
            &agent_state(),
            &proposal(ProposalOrigin::Human),
            &ordering_key(),
            &ContentManifestId::new("phase3a_manifest").unwrap(),
        )
        .unwrap();
        let scheduler = build_continue_routine_event(
            &state(),
            &agent_state(),
            &proposal(ProposalOrigin::Scheduler),
            &ordering_key(),
            &ContentManifestId::new("phase3a_manifest").unwrap(),
        )
        .unwrap();

        let next = |event: &EventEnvelope| {
            event
                .payload
                .iter()
                .find(|field| field.key == "next_action_id")
                .map(|field| field.value.clone())
        };
        assert_eq!(next(&human), next(&scheduler));
    }

    fn run_continue_pipeline(origin: ProposalOrigin) -> crate::actions::PipelineResult {
        let mut state = state();
        let mut log = EventLog::new();
        let mut registry = ActionRegistry::new();
        registry.register_phase3a_continue_routine();
        let mut proposal = proposal(origin.clone());
        let mut bindings = None;
        if origin == ProposalOrigin::Human {
            let controller_id = ControllerId::new("controller_human").unwrap();
            attach_tui_source(&mut proposal, &state);
            proposal
                .parameters
                .insert("controller_id".to_string(), controller_id.to_string());
            let mut human_bindings = ControllerBindings::new();
            let mut binding_log = EventLog::new();
            human_bindings.attach(
                controller_id,
                actor_id(),
                ControllerMode::Embodied,
                SimTick::new(5),
                &mut binding_log,
                ContentManifestId::new("phase3a_manifest").unwrap(),
            );
            bindings = Some(human_bindings);
        }
        run_pipeline(
            &mut PipelineContext {
                registry: &registry,
                state: &mut state,
                agent_state: Box::leak(Box::new(agent_state())),
                log: &mut log,
                controller_bindings: bindings.as_ref(),
                epistemic_projection: None,
                content_manifest_id: ContentManifestId::new("phase3a_manifest").unwrap(),
                ordering_key: ordering_key(),
            },
            &proposal,
        )
    }

    fn run_move_pipeline(
        state: &mut PhysicalState,
        log: &mut EventLog,
    ) -> crate::actions::PipelineResult {
        let mut registry = ActionRegistry::new();
        registry.register_phase1_movement_open_close();
        let mut proposal = Proposal::new(
            ProposalId::new("proposal_continue_follow_on_move").unwrap(),
            ProposalOrigin::Scheduler,
            Some(actor_id()),
            ActionId::new("move").unwrap(),
            SimTick::new(6),
        );
        proposal.target_ids.push("hallway".to_string());
        run_pipeline(
            &mut PipelineContext {
                registry: &registry,
                state,
                agent_state: Box::leak(Box::new(crate::state::AgentState::default())),
                log,
                controller_bindings: None,
                epistemic_projection: None,
                content_manifest_id: ContentManifestId::new("phase3a_manifest").unwrap(),
                ordering_key: OrderingKey::new(
                    SimTick::new(6),
                    SchedulePhase::NoHumanProcess,
                    SchedulerSourceId::Actor(actor_id()),
                    ProposalSequence::new(1),
                    ActionId::new("move").unwrap(),
                    vec!["hallway".to_string()],
                    "continue_follow_on_move",
                ),
            },
            &proposal,
        )
    }

    #[test]
    fn continuation_success_requires_follow_on_ordinary_action_ancestry() {
        let mut state = state();
        let mut log = EventLog::new();
        let mut registry = ActionRegistry::new();
        registry.register_phase3a_continue_routine();
        let continue_result = run_pipeline(
            &mut PipelineContext {
                registry: &registry,
                state: &mut state,
                agent_state: Box::leak(Box::new(agent_state())),
                log: &mut log,
                controller_bindings: None,
                epistemic_projection: None,
                content_manifest_id: ContentManifestId::new("phase3a_manifest").unwrap(),
                ordering_key: ordering_key(),
            },
            &proposal(ProposalOrigin::Scheduler),
        );

        assert_eq!(continue_result.report.status, ReportStatus::Accepted);
        assert!(continue_result
            .appended_events
            .iter()
            .any(|event| event.event_type == EventKind::ContinueRoutineProposed));
        assert!(!log
            .events()
            .iter()
            .any(|event| event.event_type == EventKind::ActorMoved));

        let move_result = run_move_pipeline(&mut state, &mut log);

        assert_eq!(move_result.report.status, ReportStatus::Accepted);
        assert!(move_result
            .appended_events
            .iter()
            .any(|event| event.event_type == EventKind::ActorMoved));
        assert!(log
            .events()
            .iter()
            .any(|event| event.event_type == EventKind::ActorMoved));
    }

    #[test]
    fn human_and_scheduler_continue_share_pipeline_result() {
        let human = run_continue_pipeline(ProposalOrigin::Human);
        let scheduler = run_continue_pipeline(ProposalOrigin::Scheduler);

        assert_eq!(human.report.status, ReportStatus::Accepted);
        assert_eq!(human.report.status, scheduler.report.status);
        assert_eq!(
            human.appended_events[0].event_type,
            scheduler.appended_events[0].event_type
        );
    }
}
