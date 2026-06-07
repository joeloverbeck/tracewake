use crate::actions::defs::ActionRejection;
use crate::actions::pipeline::PipelineStage;
use crate::actions::proposal::Proposal;
use crate::actions::report::{CheckedFact, ReasonCode};
use crate::events::{EventCause, EventEnvelope, EventKind, PayloadField, EVENT_SCHEMA_V1};
use crate::ids::{ActionId, ContentManifestId, EventId, IntentionId, RoutineExecutionId};
use crate::scheduler::OrderingKey;
use crate::state::PhysicalState;

pub fn build_continue_routine_event(
    state: &PhysicalState,
    proposal: &Proposal,
    ordering_key: &OrderingKey,
    content_manifest_id: &ContentManifestId,
) -> Result<EventEnvelope, ActionRejection> {
    let actor_id = proposal.actor_id.clone().ok_or_else(actor_missing)?;
    if !state.actors.contains_key(&actor_id) {
        return Err(actor_missing());
    }

    let intention_id = parse_intention_id(proposal)?;
    let status = proposal
        .parameters
        .get("intention_status")
        .map(String::as_str)
        .unwrap_or("active");
    if matches!(status, "completed" | "failed" | "abandoned" | "interrupted") {
        return Err(reject(
            PipelineStage::ReservationConflictCheck,
            ReasonCode::IntentionTerminal,
            "intention_status",
            status,
            "That routine is no longer active.",
            "continue_routine was requested for a terminal intention",
        ));
    }

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
        PayloadField::new("actor_id", actor_id.as_str()),
        PayloadField::new("intention_id", intention_id.as_str()),
        PayloadField::new("next_action_id", next_action_id.as_str()),
        PayloadField::new("routes_through_shared_pipeline", "true"),
        PayloadField::new("intention_mutated", "false"),
    ];
    if let Some(routine_execution_id) = routine_execution_id {
        event.payload.push(PayloadField::new(
            "routine_execution_id",
            routine_execution_id.as_str(),
        ));
    }
    event.effects_summary = "continue routine proposed next ordinary action".to_string();
    Ok(event)
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
    use crate::actions::proposal::ProposalOrigin;
    use crate::actions::registry::ActionRegistry;
    use crate::actions::report::ReportStatus;
    use crate::controller::ControllerBindings;
    use crate::events::log::EventLog;
    use crate::events::EventKind;
    use crate::ids::{ActorId, ControllerId, PlaceId, ProposalId};
    use crate::scheduler::{ProposalSequence, SchedulePhase, SchedulerSourceId};
    use crate::state::{ActorBody, ControllerMode};
    use crate::time::SimTick;

    fn actor_id() -> ActorId {
        ActorId::new("actor_tomas").unwrap()
    }

    fn state() -> PhysicalState {
        let mut state = PhysicalState::default();
        state.actors.insert(
            actor_id(),
            ActorBody::new(actor_id(), PlaceId::new("office").unwrap()),
        );
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
            &proposal,
            &ordering_key(),
            &ContentManifestId::new("phase3a_manifest").unwrap(),
        )
        .unwrap_err();

        assert_eq!(rejection.reason_code, ReasonCode::NoCurrentIntention);
    }

    #[test]
    fn continue_terminal_intention_rejects_with_summary_reason() {
        let mut proposal = proposal(ProposalOrigin::Test);
        proposal
            .parameters
            .insert("intention_status".to_string(), "completed".to_string());
        let rejection = build_continue_routine_event(
            &state(),
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
    }

    #[test]
    fn human_and_scheduler_continue_produce_same_next_action() {
        let human = build_continue_routine_event(
            &state(),
            &proposal(ProposalOrigin::Human),
            &ordering_key(),
            &ContentManifestId::new("phase3a_manifest").unwrap(),
        )
        .unwrap();
        let scheduler = build_continue_routine_event(
            &state(),
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
                log: &mut log,
                controller_bindings: bindings.as_ref(),
                epistemic_projection: None,
                content_manifest_id: ContentManifestId::new("phase3a_manifest").unwrap(),
                ordering_key: ordering_key(),
            },
            &proposal,
        )
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
