use crate::actions::defs::movement::build_move_event;
use crate::actions::defs::openclose::build_open_close_event;
use crate::actions::defs::takeplace::build_take_place_event;
use crate::actions::defs::ActionRejection;
use crate::actions::proposal::Proposal;
use crate::actions::registry::{ActionEffect, ActionRegistry};
use crate::actions::report::{CheckedFact, ReasonCode, ReportStatus, ValidationReport};
use crate::events::apply::apply_event;
use crate::events::log::EventLog;
use crate::events::{EventEnvelope, EventKind, PayloadField};
use crate::ids::{ContentManifestId, EventId, ValidationReportId};
use crate::scheduler::OrderingKey;
use crate::state::PhysicalState;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PipelineStage {
    OriginIntake,
    ControllerBindingCheck,
    ActionDefinitionLookup,
    ActorLookup,
    TargetBinding,
    LocalityReachabilityValidation,
    PhysicalPreconditionValidation,
    KnowledgePerceptionPlaceholder,
    SocialNormPlaceholder,
    CostDurationCheck,
    ReservationConflictCheck,
    PhaseBoundaryValidation,
    MutationPlanConstruction,
    ValidationReportCreation,
    EventEnvelopeConstruction,
    EventAppend,
    EventApplication,
    ProjectionViewUpdate,
    DebugLinkage,
}

impl PipelineStage {
    pub const fn all() -> &'static [PipelineStage] {
        &[
            PipelineStage::OriginIntake,
            PipelineStage::ControllerBindingCheck,
            PipelineStage::ActionDefinitionLookup,
            PipelineStage::ActorLookup,
            PipelineStage::TargetBinding,
            PipelineStage::LocalityReachabilityValidation,
            PipelineStage::PhysicalPreconditionValidation,
            PipelineStage::KnowledgePerceptionPlaceholder,
            PipelineStage::SocialNormPlaceholder,
            PipelineStage::CostDurationCheck,
            PipelineStage::ReservationConflictCheck,
            PipelineStage::PhaseBoundaryValidation,
            PipelineStage::MutationPlanConstruction,
            PipelineStage::ValidationReportCreation,
            PipelineStage::EventEnvelopeConstruction,
            PipelineStage::EventAppend,
            PipelineStage::EventApplication,
            PipelineStage::ProjectionViewUpdate,
            PipelineStage::DebugLinkage,
        ]
    }
}

pub struct PipelineContext<'a> {
    pub registry: &'a ActionRegistry,
    pub state: &'a mut PhysicalState,
    pub log: &'a mut EventLog,
    pub content_manifest_id: ContentManifestId,
    pub ordering_key: OrderingKey,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PipelineResult {
    pub report: ValidationReport,
    pub appended_events: Vec<EventEnvelope>,
}

pub fn run_pipeline(context: &mut PipelineContext<'_>, proposal: &Proposal) -> PipelineResult {
    let mut checked_facts = vec![
        CheckedFact::new("origin", format!("{:?}", proposal.origin)),
        CheckedFact::new("action_id", proposal.action_id.as_str()),
    ];

    // Stages 8-11 are deliberately inert architectural slots for later
    // epistemic, norm, cost/duration, and reservation systems.
    let definition = match context.registry.get(&proposal.action_id) {
        Some(definition) => definition,
        None => {
            return reject(
                context,
                proposal,
                PipelineStage::ActionDefinitionLookup,
                vec![ReasonCode::UnknownActionId],
                checked_facts,
                "That action is not available.",
                "action definition was not present in the ordinary registry",
            )
        }
    };

    if let Some(actor_id) = &proposal.actor_id {
        checked_facts.push(CheckedFact::new("actor_id", actor_id.as_str()));
        match context.state.actors.get(actor_id) {
            Some(actor) if actor.enabled => {}
            _ => {
                return reject(
                    context,
                    proposal,
                    PipelineStage::ActorLookup,
                    vec![ReasonCode::ActorNotFound],
                    checked_facts,
                    "That actor cannot act.",
                    "actor was missing or disabled",
                )
            }
        }
    }

    if !definition.phase1_implemented {
        return reject(
            context,
            proposal,
            PipelineStage::PhaseBoundaryValidation,
            vec![ReasonCode::PhaseUnsupportedAction],
            checked_facts,
            "That action is not part of this phase.",
            "registry action exists but is not implemented for Phase 1",
        );
    }

    checked_facts.push(CheckedFact::new("pipeline_slots_8_11", "inert"));

    let mut appended_events = Vec::new();
    let would_mutate = !matches!(definition.effect, ActionEffect::QueryOnly);
    if would_mutate {
        let event_result = match definition.effect {
            ActionEffect::Move => build_move_event(
                context.state,
                proposal,
                &context.ordering_key,
                &context.content_manifest_id,
            ),
            ActionEffect::Open => build_open_close_event(
                context.state,
                proposal,
                &context.ordering_key,
                &context.content_manifest_id,
                true,
            ),
            ActionEffect::Close => build_open_close_event(
                context.state,
                proposal,
                &context.ordering_key,
                &context.content_manifest_id,
                false,
            ),
            ActionEffect::Take => build_take_place_event(
                context.state,
                proposal,
                &context.ordering_key,
                &context.content_manifest_id,
                true,
            ),
            ActionEffect::Place => build_take_place_event(
                context.state,
                proposal,
                &context.ordering_key,
                &context.content_manifest_id,
                false,
            ),
            ActionEffect::QueryOnly => unreachable!("would_mutate checked above"),
        };

        let event = match event_result {
            Ok(event) => event,
            Err(rejection) => return reject_action(context, proposal, rejection),
        };

        let mut dry_run = context.state.clone();
        if apply_event(&mut dry_run, &event).is_err() {
            return reject(
                context,
                proposal,
                PipelineStage::EventApplication,
                vec![ReasonCode::WorldStateMismatch],
                checked_facts,
                "The world state did not match that action.",
                "dry-run event application rejected the constructed event",
            );
        }

        let appended = match context.log.append(event) {
            Ok(appended) => appended,
            Err(_) => {
                return reject(
                    context,
                    proposal,
                    PipelineStage::EventAppend,
                    vec![ReasonCode::WorldStateMismatch],
                    checked_facts,
                    "The event could not be recorded.",
                    "append-only log rejected the constructed event",
                )
            }
        };
        if apply_event(context.state, &appended).is_err() {
            return reject(
                context,
                proposal,
                PipelineStage::EventApplication,
                vec![ReasonCode::WorldStateMismatch],
                checked_facts,
                "The world state did not match that action.",
                "event application rejected an event that passed dry-run",
            );
        }
        appended_events.push(appended);
    }
    let event_ids = appended_events
        .iter()
        .map(|event| event.event_id.clone())
        .collect();
    let report = ValidationReport {
        validation_report_id: report_id(proposal),
        proposal_id: proposal.proposal_id.clone(),
        actor_id: proposal.actor_id.clone(),
        action_id: proposal.action_id.clone(),
        target_ids: proposal.target_ids.clone(),
        status: ReportStatus::Accepted,
        failed_stage: None,
        reason_codes: Vec::new(),
        checked_facts,
        actor_visible_summary: "Accepted.".to_string(),
        debug_summary: "proposal accepted by shared pipeline".to_string(),
        would_mutate,
        event_ids,
        checksum_before: None,
        checksum_after: None,
    };

    PipelineResult {
        report,
        appended_events,
    }
}

fn reject_action(
    context: &mut PipelineContext<'_>,
    proposal: &Proposal,
    rejection: ActionRejection,
) -> PipelineResult {
    reject(
        context,
        proposal,
        rejection.failed_stage,
        vec![rejection.reason_code],
        rejection.checked_facts,
        rejection.actor_visible_summary,
        rejection.debug_summary,
    )
}

fn reject(
    context: &mut PipelineContext<'_>,
    proposal: &Proposal,
    failed_stage: PipelineStage,
    reason_codes: Vec<ReasonCode>,
    checked_facts: Vec<CheckedFact>,
    actor_visible_summary: impl Into<String>,
    debug_summary: impl Into<String>,
) -> PipelineResult {
    let report_id = report_id(proposal);
    let event_id = rejection_event_id(proposal);
    let report = ValidationReport {
        validation_report_id: report_id.clone(),
        proposal_id: proposal.proposal_id.clone(),
        actor_id: proposal.actor_id.clone(),
        action_id: proposal.action_id.clone(),
        target_ids: proposal.target_ids.clone(),
        status: ReportStatus::Rejected,
        failed_stage: Some(failed_stage),
        reason_codes: reason_codes.clone(),
        checked_facts,
        actor_visible_summary: actor_visible_summary.into(),
        debug_summary: debug_summary.into(),
        would_mutate: false,
        event_ids: vec![event_id.clone()],
        checksum_before: None,
        checksum_after: None,
    };

    let mut event = EventEnvelope::new_v1(
        event_id,
        EventKind::ActionRejected,
        0,
        0,
        proposal.requested_tick,
        context.ordering_key.clone(),
        context.content_manifest_id.clone(),
    );
    event.actor_id = proposal.actor_id.clone();
    event.proposal_id = Some(proposal.proposal_id.clone());
    event.validation_report_id = Some(report_id);
    event.payload = reason_codes
        .iter()
        .map(|code| PayloadField::new("reason_code", code.stable_id()))
        .collect();
    event.effects_summary = "rejected before world mutation".to_string();

    let _ = context.log.append(event.clone());

    PipelineResult {
        report,
        appended_events: vec![event],
    }
}

fn report_id(proposal: &Proposal) -> ValidationReportId {
    ValidationReportId::new(format!("report.{}", proposal.proposal_id.as_str())).unwrap()
}

fn rejection_event_id(proposal: &Proposal) -> EventId {
    EventId::new(format!("event.rejection.{}", proposal.proposal_id.as_str())).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actions::{ActionDefinition, ProposalOrigin};
    use crate::events::apply::apply_event;
    use crate::ids::{ActionId, ActorId, ProposalId};
    use crate::scheduler::{ProposalSequence, SchedulePhase, SchedulerSourceId};
    use crate::state::{ActorBody, DoorState, PlaceState};
    use crate::time::SimTick;

    fn action_id(value: &str) -> ActionId {
        ActionId::new(value).unwrap()
    }

    fn actor_id(value: &str) -> ActorId {
        ActorId::new(value).unwrap()
    }

    fn proposal(origin: ProposalOrigin) -> Proposal {
        Proposal::new(
            ProposalId::new("proposal_001").unwrap(),
            origin,
            Some(actor_id("actor_tomas")),
            action_id("look"),
            SimTick::ZERO,
        )
    }

    fn ordering_key() -> OrderingKey {
        OrderingKey::new(
            SimTick::ZERO,
            SchedulePhase::HumanCommand,
            SchedulerSourceId::Actor(actor_id("actor_tomas")),
            ProposalSequence::new(0),
            action_id("look"),
            Vec::new(),
            "tie",
        )
    }

    fn state() -> PhysicalState {
        let mut state = PhysicalState::default();
        state.actors.insert(
            actor_id("actor_tomas"),
            ActorBody::new(
                actor_id("actor_tomas"),
                crate::ids::PlaceId::new("shop_front").unwrap(),
            ),
        );
        state
    }

    fn door_state() -> PhysicalState {
        let mut state = PhysicalState::default();
        let actor_id = actor_id("actor_tomas");
        let shop = crate::ids::PlaceId::new("shop_front").unwrap();
        let back = crate::ids::PlaceId::new("back_room").unwrap();
        let mut shop_state = PlaceState::new(shop.clone(), "Shop front");
        shop_state.adjacent_place_ids.insert(back.clone());
        state.places.insert(shop.clone(), shop_state);
        state
            .places
            .insert(back.clone(), PlaceState::new(back.clone(), "Back room"));
        state.actors.insert(
            actor_id.clone(),
            ActorBody::new(actor_id.clone(), shop.clone()),
        );
        let door_id = crate::ids::DoorId::new("door_shop_back").unwrap();
        state
            .doors
            .insert(door_id.clone(), DoorState::new(door_id, shop, back));
        state
    }

    #[test]
    fn same_proposal_validates_same_for_human_and_scheduler_origin() {
        let mut registry = ActionRegistry::new();
        registry.register(ActionDefinition::query_only(action_id("look")));

        let mut human_state = state();
        let mut human_log = EventLog::new();
        let mut human_context = PipelineContext {
            registry: &registry,
            state: &mut human_state,
            log: &mut human_log,
            content_manifest_id: ContentManifestId::new("phase1_manifest").unwrap(),
            ordering_key: ordering_key(),
        };
        let human = run_pipeline(&mut human_context, &proposal(ProposalOrigin::Human));

        let mut scheduler_state = state();
        let mut scheduler_log = EventLog::new();
        let mut scheduler_context = PipelineContext {
            registry: &registry,
            state: &mut scheduler_state,
            log: &mut scheduler_log,
            content_manifest_id: ContentManifestId::new("phase1_manifest").unwrap(),
            ordering_key: ordering_key(),
        };
        let scheduler = run_pipeline(&mut scheduler_context, &proposal(ProposalOrigin::Scheduler));

        assert_eq!(human.report.status, scheduler.report.status);
        assert_eq!(human.report.reason_codes, scheduler.report.reason_codes);
        assert_eq!(human.report.would_mutate, scheduler.report.would_mutate);
        assert_eq!(human.appended_events.len(), scheduler.appended_events.len());
        assert_eq!(human_state, scheduler_state);
    }

    #[test]
    fn rejected_proposal_is_structured_and_mutates_no_world_state() {
        let registry = ActionRegistry::new();
        let mut state = state();
        let before = state.clone();
        let mut log = EventLog::new();
        let mut context = PipelineContext {
            registry: &registry,
            state: &mut state,
            log: &mut log,
            content_manifest_id: ContentManifestId::new("phase1_manifest").unwrap(),
            ordering_key: ordering_key(),
        };

        let result = run_pipeline(&mut context, &proposal(ProposalOrigin::Test));

        assert_eq!(result.report.status, ReportStatus::Rejected);
        assert_eq!(
            result.report.failed_stage,
            Some(PipelineStage::ActionDefinitionLookup)
        );
        assert_eq!(
            result.report.reason_codes,
            vec![ReasonCode::UnknownActionId]
        );
        assert!(!result.report.checked_facts.is_empty());
        assert!(!result.report.actor_visible_summary.is_empty());
        assert!(!result.report.debug_summary.is_empty());
        assert!(!result.report.would_mutate);
        assert_eq!(
            result.appended_events[0].event_type,
            EventKind::ActionRejected
        );
        assert_eq!(state, before);
    }

    #[test]
    fn pipeline_contains_inert_slots_8_to_11() {
        let stages = PipelineStage::all();
        assert!(stages.contains(&PipelineStage::KnowledgePerceptionPlaceholder));
        assert!(stages.contains(&PipelineStage::SocialNormPlaceholder));
        assert!(stages.contains(&PipelineStage::CostDurationCheck));
        assert!(stages.contains(&PipelineStage::ReservationConflictCheck));
        assert_eq!(stages.len(), 19);
    }

    #[test]
    fn open_then_move_log_replays_to_same_state() {
        let mut registry = ActionRegistry::new();
        registry.register_phase1_movement_open_close();
        let initial = door_state();
        let mut live_state = initial.clone();
        let mut live_log = EventLog::new();

        let mut open = Proposal::new(
            ProposalId::new("proposal_open").unwrap(),
            ProposalOrigin::Test,
            Some(actor_id("actor_tomas")),
            action_id("open"),
            SimTick::ZERO,
        );
        open.target_ids.push("door_shop_back".to_string());
        let mut open_context = PipelineContext {
            registry: &registry,
            state: &mut live_state,
            log: &mut live_log,
            content_manifest_id: ContentManifestId::new("phase1_manifest").unwrap(),
            ordering_key: ordering_key(),
        };
        let open_result = run_pipeline(&mut open_context, &open);
        assert_eq!(open_result.report.status, ReportStatus::Accepted);
        assert_eq!(
            open_result.appended_events[0].event_type,
            EventKind::DoorOpened
        );

        let mut move_proposal = Proposal::new(
            ProposalId::new("proposal_move").unwrap(),
            ProposalOrigin::Test,
            Some(actor_id("actor_tomas")),
            action_id("move"),
            SimTick::ZERO,
        );
        move_proposal.target_ids.push("back_room".to_string());
        let mut move_context = PipelineContext {
            registry: &registry,
            state: &mut live_state,
            log: &mut live_log,
            content_manifest_id: ContentManifestId::new("phase1_manifest").unwrap(),
            ordering_key: ordering_key(),
        };
        let move_result = run_pipeline(&mut move_context, &move_proposal);
        assert_eq!(move_result.report.status, ReportStatus::Accepted);
        assert_eq!(
            move_result.appended_events[0].event_type,
            EventKind::ActorMoved
        );

        let mut replay_state = initial;
        for event in live_log.events() {
            apply_event(&mut replay_state, event).unwrap();
        }
        assert_eq!(replay_state, live_state);
    }
}
