use crate::actions::defs::accuseprobe::validate_truthful_accuse_probe;
use crate::actions::defs::checkcontainer::{
    build_check_container_event, build_observation_recorded_event,
};
use crate::actions::defs::continue_routine::build_continue_routine_event;
use crate::actions::defs::eat::build_eat_events;
use crate::actions::defs::movement::build_move_event;
use crate::actions::defs::openclose::build_open_close_event;
use crate::actions::defs::sleep::build_sleep_start_event;
use crate::actions::defs::takeplace::{
    build_sound_belief_event, build_sound_observation_event, build_take_place_event,
};
use crate::actions::defs::wait::build_wait_events;
use crate::actions::defs::work::build_work_start_events;
use crate::actions::defs::ActionRejection;
use crate::actions::proposal::{Proposal, ProposalOrigin};
use crate::actions::registry::{ActionEffect, ActionRegistry};
use crate::actions::report::{CheckedFact, ReasonCode, ReportStatus, ValidationReport};
use crate::agent::current_place_knowledge_context;
use crate::controller::{ControllerBindings, ControllerError};
use crate::epistemics::{
    detect_expected_absences, Confidence, EpistemicProjection, KnowledgeContext,
};
use crate::events::apply::{
    apply_epistemic_event, apply_event, apply_event_stream, EventApplicationContext,
};
use crate::events::log::EventLog;
use crate::events::{EventEnvelope, EventKind, PayloadField, EVENT_SCHEMA_V1};
use crate::ids::{ActionId, ContainerId, ContentManifestId, EventId, ValidationReportId};
use crate::scheduler::OrderingKey;
use crate::state::{AgentState, PhysicalState};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PipelineStage {
    OriginIntake,
    ControllerBindingCheck,
    SourceContextValidation,
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
            PipelineStage::SourceContextValidation,
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
    pub agent_state: &'a mut AgentState,
    pub log: &'a mut EventLog,
    pub controller_bindings: Option<&'a ControllerBindings>,
    pub epistemic_projection: Option<&'a mut EpistemicProjection>,
    pub content_manifest_id: ContentManifestId,
    pub ordering_key: OrderingKey,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PipelineResult {
    pub report: ValidationReport,
    pub appended_events: Vec<EventEnvelope>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum PipelineDecision {
    Accepted {
        candidate_events: Vec<EventEnvelope>,
        checked_facts: Vec<CheckedFact>,
        would_mutate: bool,
    },
    Rejected(Box<ValidationReport>),
}

#[derive(Clone, Copy)]
struct PipelineReadContext<'a> {
    registry: &'a ActionRegistry,
    state: &'a PhysicalState,
    agent_state: &'a AgentState,
    controller_bindings: Option<&'a ControllerBindings>,
    epistemic_projection: Option<&'a EpistemicProjection>,
    content_manifest_id: &'a ContentManifestId,
    ordering_key: &'a OrderingKey,
    current_event_frontier: u64,
}

#[derive(Clone, Copy)]
pub struct ProposalValidationContext<'a> {
    pub registry: &'a ActionRegistry,
    pub state: &'a PhysicalState,
    pub agent_state: &'a AgentState,
    pub controller_bindings: Option<&'a ControllerBindings>,
    pub epistemic_projection: Option<&'a EpistemicProjection>,
    pub content_manifest_id: &'a ContentManifestId,
    pub ordering_key: &'a OrderingKey,
    pub current_event_frontier: u64,
}

pub fn validate_proposal(
    validation_context: ProposalValidationContext<'_>,
    proposal: &Proposal,
) -> ValidationReport {
    let context = PipelineReadContext {
        registry: validation_context.registry,
        state: validation_context.state,
        agent_state: validation_context.agent_state,
        controller_bindings: validation_context.controller_bindings,
        epistemic_projection: validation_context.epistemic_projection,
        content_manifest_id: validation_context.content_manifest_id,
        ordering_key: validation_context.ordering_key,
        current_event_frontier: validation_context.current_event_frontier,
    };
    match decide_proposal(context, proposal) {
        PipelineDecision::Accepted {
            candidate_events,
            checked_facts,
            would_mutate,
        } => accepted_report(proposal, checked_facts, would_mutate, candidate_events),
        PipelineDecision::Rejected(report) => *report,
    }
}

pub fn run_pipeline(context: &mut PipelineContext<'_>, proposal: &Proposal) -> PipelineResult {
    let read_context = PipelineReadContext {
        registry: context.registry,
        state: context.state,
        agent_state: context.agent_state,
        controller_bindings: context.controller_bindings,
        epistemic_projection: context.epistemic_projection.as_deref(),
        content_manifest_id: &context.content_manifest_id,
        ordering_key: &context.ordering_key,
        current_event_frontier: context.log.events().len() as u64,
    };
    let decision = decide_proposal(read_context, proposal);
    let (candidate_events, checked_facts, would_mutate) = match decision {
        PipelineDecision::Accepted {
            candidate_events,
            checked_facts,
            would_mutate,
        } => (candidate_events, checked_facts, would_mutate),
        PipelineDecision::Rejected(report) => {
            let report = *report;
            let event = append_rejection_event(context, proposal, &report);
            return PipelineResult {
                report,
                appended_events: vec![event],
            };
        }
    };

    let definition = context
        .registry
        .get(&proposal.action_id)
        .expect("accepted decision must have an action definition");

    if let Some(active_event_id) =
        body_exclusive_reservation_conflict(context, proposal, &candidate_events)
    {
        return reject_committed(
            context,
            proposal,
            PipelineStage::ReservationConflictCheck,
            vec![ReasonCode::ReservationConflict],
            checked_facts,
            "That actor is already in a body-exclusive action.",
            format!(
                "scheduling/reservation conflict with active body-exclusive event {}",
                active_event_id.as_str()
            ),
        );
    }

    let mut appended_events = Vec::new();
    for event in candidate_events {
        let appended = match context.log.append(event) {
            Ok(appended) => appended,
            Err(_) => {
                return reject_committed(
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
        let mut application_context = EventApplicationContext {
            physical_state: context.state,
            agent_state: context.agent_state,
            epistemic_projection: context.epistemic_projection.as_deref_mut(),
        };
        if apply_event_stream(&mut application_context, &appended).is_err() {
            return reject_committed(
                context,
                proposal,
                PipelineStage::EventApplication,
                vec![ReasonCode::WorldStateMismatch],
                checked_facts,
                "The world state did not match that action.",
                "event application rejected an event that passed dry-run",
            );
        }
        if definition.effect == ActionEffect::CheckContainer {
            let observation_event = build_observation_recorded_event(
                &appended,
                &context.ordering_key,
                &context.content_manifest_id,
            );
            let appended_observation = match context.log.append(observation_event) {
                Ok(appended) => appended,
                Err(_) => {
                    return reject_committed(
                        context,
                        proposal,
                        PipelineStage::KnowledgePerceptionPlaceholder,
                        vec![ReasonCode::WorldStateMismatch],
                        checked_facts,
                        "The observation could not be recorded.",
                        "append-only log rejected the epistemic observation event",
                    )
                }
            };
            appended_events.push(appended.clone());
            appended_events.push(appended_observation.clone());
            if let Some(projection) = context.epistemic_projection.as_deref_mut() {
                if apply_epistemic_event(projection, &appended_observation).is_err() {
                    return reject_committed(
                        context,
                        proposal,
                        PipelineStage::KnowledgePerceptionPlaceholder,
                        vec![ReasonCode::WorldStateMismatch],
                        checked_facts,
                        "The observation could not be applied.",
                        "epistemic projection rejected the observation event",
                    );
                }
                for event in build_absence_detection_events(
                    projection,
                    context.state,
                    &appended,
                    &appended_observation,
                    &context.ordering_key,
                    &context.content_manifest_id,
                ) {
                    let appended_detection = match context.log.append(event) {
                        Ok(appended) => appended,
                        Err(_) => {
                            return reject_committed(
                                context,
                                proposal,
                                PipelineStage::KnowledgePerceptionPlaceholder,
                                vec![ReasonCode::WorldStateMismatch],
                                checked_facts,
                                "The derived belief could not be recorded.",
                                "append-only log rejected a contradiction/belief event",
                            )
                        }
                    };
                    if apply_epistemic_event(projection, &appended_detection).is_err() {
                        return reject_committed(
                            context,
                            proposal,
                            PipelineStage::KnowledgePerceptionPlaceholder,
                            vec![ReasonCode::WorldStateMismatch],
                            checked_facts,
                            "The derived belief could not be applied.",
                            "epistemic projection rejected a contradiction/belief event",
                        );
                    }
                    appended_events.push(appended_detection);
                }
            }
        } else if matches!(definition.effect, ActionEffect::Take | ActionEffect::Place) {
            appended_events.push(appended.clone());
            if let Some(sound_observation) = build_sound_observation_event(
                context.state,
                &appended,
                &context.ordering_key,
                &context.content_manifest_id,
            ) {
                let appended_observation = match context.log.append(sound_observation) {
                    Ok(appended) => appended,
                    Err(_) => {
                        return reject_committed(
                            context,
                            proposal,
                            PipelineStage::KnowledgePerceptionPlaceholder,
                            vec![ReasonCode::WorldStateMismatch],
                            checked_facts,
                            "The observation could not be recorded.",
                            "append-only log rejected the sound observation event",
                        )
                    }
                };
                if let Some(projection) = context.epistemic_projection.as_deref_mut() {
                    if apply_epistemic_event(projection, &appended_observation).is_err() {
                        return reject_committed(
                            context,
                            proposal,
                            PipelineStage::KnowledgePerceptionPlaceholder,
                            vec![ReasonCode::WorldStateMismatch],
                            checked_facts,
                            "The observation could not be applied.",
                            "epistemic projection rejected the sound observation event",
                        );
                    }
                }
                appended_events.push(appended_observation.clone());
                if let Some(sound_belief) = build_sound_belief_event(
                    &appended_observation,
                    &context.ordering_key,
                    &context.content_manifest_id,
                ) {
                    let appended_belief = match context.log.append(sound_belief) {
                        Ok(appended) => appended,
                        Err(_) => {
                            return reject_committed(
                                context,
                                proposal,
                                PipelineStage::KnowledgePerceptionPlaceholder,
                                vec![ReasonCode::WorldStateMismatch],
                                checked_facts,
                                "The derived belief could not be recorded.",
                                "append-only log rejected the sound belief event",
                            )
                        }
                    };
                    if let Some(projection) = context.epistemic_projection.as_deref_mut() {
                        if apply_epistemic_event(projection, &appended_belief).is_err() {
                            return reject_committed(
                                context,
                                proposal,
                                PipelineStage::KnowledgePerceptionPlaceholder,
                                vec![ReasonCode::WorldStateMismatch],
                                checked_facts,
                                "The derived belief could not be applied.",
                                "epistemic projection rejected the sound belief event",
                            );
                        }
                    }
                    appended_events.push(appended_belief);
                }
            }
        } else {
            appended_events.push(appended);
        }
    }

    let report = accepted_report(
        proposal,
        checked_facts,
        would_mutate,
        appended_events.clone(),
    );

    PipelineResult {
        report,
        appended_events,
    }
}

fn body_exclusive_reservation_conflict(
    context: &PipelineContext<'_>,
    proposal: &Proposal,
    candidate_events: &[EventEnvelope],
) -> Option<EventId> {
    if is_lifecycle_control(&proposal.action_id) || candidate_events.is_empty() {
        return None;
    }
    let actor_id = proposal.actor_id.as_ref()?;

    let open_starts = crate::need_accounting::open_body_exclusive_starts(
        context.log,
        actor_id,
        context.ordering_key.sim_tick,
    )
    .expect("duplicate duration terminals are rejected before reservation checks");
    open_starts.into_iter().next()
}

fn is_lifecycle_control(action_id: &ActionId) -> bool {
    matches!(action_id.as_str(), "continue_routine")
}

#[cfg(test)]
fn is_body_exclusive_start(event: &EventEnvelope) -> bool {
    matches!(
        event.event_type,
        EventKind::SleepStarted | EventKind::WorkBlockStarted
    ) && event
        .payload
        .iter()
        .any(|field| field.key == "body_exclusive" && field.value == "true")
}

fn decide_proposal(context: PipelineReadContext<'_>, proposal: &Proposal) -> PipelineDecision {
    let mut checked_facts = vec![CheckedFact::new("action_id", proposal.action_id.as_str())];

    if let Some(result) = controller_binding_check(context, proposal, checked_facts.clone()) {
        return result;
    }
    if let Some(result) = source_context_check(context, proposal, checked_facts.clone()) {
        return result;
    }

    // Later placeholder stages remain explicit so new mechanics can activate
    // the shared pipeline without creating side mutation paths.
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

    if !context.registry.scope_is_active(definition.scope) {
        return reject(
            context,
            proposal,
            PipelineStage::PhaseBoundaryValidation,
            vec![ReasonCode::PhaseUnsupportedAction],
            checked_facts,
            "That action is not part of this phase.",
            "registry action exists but is outside the active action scope set",
        );
    }

    checked_facts.push(CheckedFact::new(
        "knowledge_perception_slot",
        if matches!(definition.effect, ActionEffect::CheckContainer) {
            "active"
        } else if proposal.action_id.as_str() == "truthful_accuse_probe" {
            "query_validation"
        } else {
            "inert"
        },
    ));
    checked_facts.push(CheckedFact::new("pipeline_slots_9_11", "inert"));

    let would_mutate = !matches!(definition.effect, ActionEffect::QueryOnly);
    if definition.effect == ActionEffect::QueryOnly
        && proposal.action_id.as_str() == "truthful_accuse_probe"
    {
        if let Err(rejection) =
            validate_truthful_accuse_probe(context.state, context.epistemic_projection, proposal)
        {
            return reject_action(context, proposal, rejection);
        }
    }
    if would_mutate {
        let event_result = match definition.effect {
            ActionEffect::Move => build_move_event(
                context.state,
                proposal,
                context.ordering_key,
                context.content_manifest_id,
            ),
            ActionEffect::Open => build_open_close_event(
                context.state,
                proposal,
                context.ordering_key,
                context.content_manifest_id,
                true,
            ),
            ActionEffect::Close => build_open_close_event(
                context.state,
                proposal,
                context.ordering_key,
                context.content_manifest_id,
                false,
            ),
            ActionEffect::Take => build_take_place_event(
                context.state,
                proposal,
                context.ordering_key,
                context.content_manifest_id,
                true,
            ),
            ActionEffect::Place => build_take_place_event(
                context.state,
                proposal,
                context.ordering_key,
                context.content_manifest_id,
                false,
            ),
            ActionEffect::Wait => {
                let events = match build_wait_events(
                    context.state,
                    context.agent_state,
                    proposal,
                    context.ordering_key,
                    context.content_manifest_id,
                ) {
                    Ok(events) => events,
                    Err(rejection) => return reject_action(context, proposal, rejection),
                };

                let mut dry_run = context.state.clone();
                for event in &events {
                    if apply_event(&mut dry_run, event).is_err() {
                        return reject(
                            context,
                            proposal,
                            PipelineStage::EventApplication,
                            vec![ReasonCode::WorldStateMismatch],
                            checked_facts,
                            "The world state did not match that action.",
                            "dry-run event application rejected a constructed wait event",
                        );
                    }
                }
                return PipelineDecision::Accepted {
                    candidate_events: events,
                    checked_facts,
                    would_mutate,
                };
            }
            ActionEffect::CheckContainer => build_check_container_event(
                context.state,
                proposal,
                context.ordering_key,
                context.content_manifest_id,
            ),
            ActionEffect::Sleep => build_sleep_start_event(
                context.state,
                proposal,
                context.ordering_key,
                context.content_manifest_id,
            ),
            ActionEffect::Eat => {
                let events = match build_eat_events(
                    context.state,
                    context.agent_state,
                    proposal,
                    context.ordering_key,
                    context.content_manifest_id,
                ) {
                    Ok(events) => events,
                    Err(rejection) => return reject_action(context, proposal, rejection),
                };

                let mut dry_run = context.state.clone();
                for event in &events {
                    if apply_event(&mut dry_run, event).is_err() {
                        return reject(
                            context,
                            proposal,
                            PipelineStage::EventApplication,
                            vec![ReasonCode::WorldStateMismatch],
                            checked_facts,
                            "The world state did not match that action.",
                            "dry-run event application rejected a constructed eat event",
                        );
                    }
                }
                return PipelineDecision::Accepted {
                    candidate_events: events,
                    checked_facts,
                    would_mutate,
                };
            }
            ActionEffect::Work => {
                let events = match build_work_start_events(
                    context.state,
                    context.agent_state,
                    proposal,
                    context.ordering_key,
                    context.content_manifest_id,
                ) {
                    Ok(events) => events,
                    Err(rejection) => return reject_action(context, proposal, rejection),
                };

                let mut dry_run = context.state.clone();
                for event in &events {
                    if apply_event(&mut dry_run, event).is_err() {
                        return reject(
                            context,
                            proposal,
                            PipelineStage::EventApplication,
                            vec![ReasonCode::WorldStateMismatch],
                            checked_facts,
                            "The world state did not match that action.",
                            "dry-run event application rejected a constructed work event",
                        );
                    }
                }
                return PipelineDecision::Accepted {
                    candidate_events: events,
                    checked_facts,
                    would_mutate,
                };
            }
            ActionEffect::ContinueRoutine => build_continue_routine_event(
                context.state,
                context.agent_state,
                proposal,
                context.ordering_key,
                context.content_manifest_id,
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
        return PipelineDecision::Accepted {
            candidate_events: vec![event],
            checked_facts,
            would_mutate,
        };
    }

    PipelineDecision::Accepted {
        candidate_events: Vec::new(),
        checked_facts,
        would_mutate,
    }
}

fn accepted_report(
    proposal: &Proposal,
    checked_facts: Vec<CheckedFact>,
    would_mutate: bool,
    events: Vec<EventEnvelope>,
) -> ValidationReport {
    let event_ids = events.iter().map(|event| event.event_id.clone()).collect();
    ValidationReport {
        validation_report_id: report_id(proposal),
        proposal_id: proposal.proposal_id.clone(),
        actor_id: proposal.actor_id.clone(),
        action_id: proposal.action_id.clone(),
        target_ids: proposal.target_ids.clone(),
        status: ReportStatus::Accepted,
        failed_stage: None,
        reason_codes: Vec::new(),
        actor_visible_facts: actor_visible_facts(&checked_facts),
        debug_only_facts: debug_only_facts(&checked_facts),
        checked_facts,
        actor_visible_summary: "Accepted.".to_string(),
        debug_summary: "proposal accepted by shared pipeline".to_string(),
        would_mutate,
        event_ids,
        checksum_before: None,
        checksum_after: None,
    }
}

fn controller_binding_check(
    context: PipelineReadContext<'_>,
    proposal: &Proposal,
    checked_facts: Vec<CheckedFact>,
) -> Option<PipelineDecision> {
    if proposal.origin != ProposalOrigin::Human {
        return None;
    }

    let Some(actor_id) = proposal.actor_id.as_ref() else {
        return Some(reject(
            context,
            proposal,
            PipelineStage::ControllerBindingCheck,
            vec![ReasonCode::ControllerUnbound],
            checked_facts,
            "No controller is bound to that actor.",
            "human-origin proposal did not name an actor for authorization",
        ));
    };

    let Some(controller_id_value) = proposal.parameters.get("controller_id") else {
        return Some(reject(
            context,
            proposal,
            PipelineStage::ControllerBindingCheck,
            vec![ReasonCode::ControllerUnbound],
            checked_facts,
            "No controller is bound to that actor.",
            "human-origin proposal did not include controller_id metadata",
        ));
    };
    let Ok(controller_id) = crate::ids::ControllerId::new(controller_id_value.clone()) else {
        return Some(reject(
            context,
            proposal,
            PipelineStage::ControllerBindingCheck,
            vec![ReasonCode::ControllerUnbound],
            checked_facts,
            "No controller is bound to that actor.",
            "human-origin controller_id was not a stable ID",
        ));
    };

    match context
        .controller_bindings
        .map(|bindings| bindings.authorize(&controller_id, actor_id))
    {
        Some(Ok(())) => None,
        Some(Err(ControllerError::ControllerActorMismatch { .. })) => Some(reject(
            context,
            proposal,
            PipelineStage::ControllerBindingCheck,
            vec![ReasonCode::ControllerActorMismatch],
            checked_facts,
            "That controller is bound to another actor.",
            "human-origin controller binding did not match proposal actor",
        )),
        Some(Err(ControllerError::ControllerUnbound(_))) | None => Some(reject(
            context,
            proposal,
            PipelineStage::ControllerBindingCheck,
            vec![ReasonCode::ControllerUnbound],
            checked_facts,
            "No controller is bound to that actor.",
            "human-origin controller was not bound",
        )),
    }
}

fn source_context_check(
    context: PipelineReadContext<'_>,
    proposal: &Proposal,
    mut checked_facts: Vec<CheckedFact>,
) -> Option<PipelineDecision> {
    if proposal.origin == ProposalOrigin::Agent {
        if proposal
            .parameters
            .get("hidden_truth_audit_actor_known_only")
            .is_none_or(|value| value != "true")
        {
            return Some(reject(
                context,
                proposal,
                PipelineStage::SourceContextValidation,
                vec![ReasonCode::HiddenTruthInput],
                checked_facts,
                "That action was blocked by the actor-known audit.",
                "agent-origin proposal lacked a clean hidden-truth audit stamp",
            ));
        }
        return None;
    }

    if proposal.origin != ProposalOrigin::Human {
        return None;
    }

    let Some(actor_id) = proposal.actor_id.as_ref() else {
        return Some(reject(
            context,
            proposal,
            PipelineStage::SourceContextValidation,
            vec![ReasonCode::ProposalSourceMissing],
            checked_facts,
            "That action does not have a current view source.",
            "human-origin proposal did not name an actor for source validation",
        ));
    };

    let Some(proposal_source) = proposal.source.as_ref() else {
        return Some(reject(
            context,
            proposal,
            PipelineStage::SourceContextValidation,
            vec![ReasonCode::ProposalSourceMissing],
            checked_facts,
            "That action does not have a current view source.",
            "human-origin proposal did not include a TUI semantic-action source packet",
        ));
    };

    let Some(source) = proposal_source.tui_context() else {
        return Some(reject(
            context,
            proposal,
            PipelineStage::SourceContextValidation,
            vec![ReasonCode::ProposalSourceMissing],
            checked_facts,
            "That action does not have a current view source.",
            "human-origin proposal did not include a TUI semantic-action source packet",
        ));
    };

    checked_facts.push(CheckedFact::new("source_kind", proposal_source.stable_id()));
    checked_facts.push(CheckedFact::new(
        "semantic_action_id",
        source.semantic_action_id.as_str(),
    ));
    checked_facts.push(CheckedFact::new(
        "holder_known_context_id",
        source.holder_known_context_id.as_str(),
    ));
    checked_facts.push(CheckedFact::new(
        "holder_known_context_hash",
        source.holder_known_context_hash.as_str(),
    ));
    checked_facts.push(CheckedFact::new(
        "holder_known_context_frontier",
        source.holder_known_context_frontier.to_string(),
    ));

    if source.actor_id != *actor_id {
        return Some(reject(
            context,
            proposal,
            PipelineStage::SourceContextValidation,
            vec![ReasonCode::ProposalSourceActorMismatch],
            checked_facts,
            "That action belongs to another actor's view.",
            "proposal source actor did not match proposal actor",
        ));
    }
    if source.context_tick != proposal.requested_tick
        || source.context_tick != context.ordering_key.sim_tick
    {
        return Some(reject(
            context,
            proposal,
            PipelineStage::SourceContextValidation,
            vec![ReasonCode::ProposalSourceStale],
            checked_facts,
            "That action is from a stale view.",
            "proposal source tick did not match requested/current ordering tick",
        ));
    }
    if source.holder_known_context_frontier != context.current_event_frontier {
        return Some(reject(
            context,
            proposal,
            PipelineStage::SourceContextValidation,
            vec![ReasonCode::ProposalSourceStale],
            checked_facts,
            "That action is from a stale view.",
            "proposal source frontier did not match current event frontier",
        ));
    }
    if source.source_view_model_id
        != proposal
            .source_view_model_id
            .clone()
            .unwrap_or_else(|| source.source_view_model_id.clone())
    {
        return Some(reject(
            context,
            proposal,
            PipelineStage::SourceContextValidation,
            vec![ReasonCode::ProposalSourceContextMismatch],
            checked_facts,
            "That action does not match its view source.",
            "proposal source view model did not match legacy source view field",
        ));
    }

    let expected_context = current_place_knowledge_context(
        context.state,
        context.epistemic_projection,
        actor_id,
        source.context_tick,
        context.content_manifest_id,
        context.current_event_frontier,
    );
    if source.holder_known_context_id != *expected_context.holder_known_context_id()
        || source.holder_known_context_hash != *expected_context.holder_known_context_hash()
    {
        return Some(reject(
            context,
            proposal,
            PipelineStage::SourceContextValidation,
            vec![ReasonCode::ProposalSourceContextMismatch],
            checked_facts,
            "That action does not match the actor's current knowledge context.",
            "proposal source context id/hash did not match reconstructed sealed context",
        ));
    }

    if !semantic_action_matches_action(&source.semantic_action_id, &proposal.action_id) {
        return Some(reject(
            context,
            proposal,
            PipelineStage::SourceContextValidation,
            vec![ReasonCode::ProposalSourceForged],
            checked_facts,
            "That action token does not match the requested action.",
            "semantic action id did not match proposal action id",
        ));
    }

    None
}

fn semantic_action_matches_action(
    semantic_action_id: &crate::ids::SemanticActionId,
    action_id: &crate::ids::ActionId,
) -> bool {
    let semantic = semantic_action_id.as_str();
    let action = action_id.as_str();
    if semantic == action || semantic.starts_with(&format!("{action}.")) {
        return true;
    }
    let dotted_action = action.replace('_', ".");
    semantic == dotted_action || semantic.starts_with(&format!("{dotted_action}."))
}

fn reject_action(
    context: PipelineReadContext<'_>,
    proposal: &Proposal,
    rejection: ActionRejection,
) -> PipelineDecision {
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
    _context: PipelineReadContext<'_>,
    proposal: &Proposal,
    failed_stage: PipelineStage,
    reason_codes: Vec<ReasonCode>,
    checked_facts: Vec<CheckedFact>,
    actor_visible_summary: impl Into<String>,
    debug_summary: impl Into<String>,
) -> PipelineDecision {
    PipelineDecision::Rejected(Box::new(rejection_report(
        proposal,
        failed_stage,
        reason_codes,
        checked_facts,
        actor_visible_summary,
        debug_summary,
    )))
}

fn actor_visible_facts(checked_facts: &[CheckedFact]) -> Vec<CheckedFact> {
    checked_facts
        .iter()
        .filter(|fact| is_actor_visible_fact(fact))
        .cloned()
        .collect()
}

fn debug_only_facts(checked_facts: &[CheckedFact]) -> Vec<CheckedFact> {
    checked_facts
        .iter()
        .filter(|fact| !is_actor_visible_fact(fact))
        .cloned()
        .collect()
}

fn is_actor_visible_fact(fact: &CheckedFact) -> bool {
    matches!(
        fact.key(),
        crate::actions::report::CheckedFactKey::ActionId
            | crate::actions::report::CheckedFactKey::ActorId
            | crate::actions::report::CheckedFactKey::Reason
            | crate::actions::report::CheckedFactKey::TickCount
    )
}

fn reject_committed(
    context: &mut PipelineContext<'_>,
    proposal: &Proposal,
    failed_stage: PipelineStage,
    reason_codes: Vec<ReasonCode>,
    checked_facts: Vec<CheckedFact>,
    actor_visible_summary: impl Into<String>,
    debug_summary: impl Into<String>,
) -> PipelineResult {
    let report = rejection_report(
        proposal,
        failed_stage,
        reason_codes,
        checked_facts,
        actor_visible_summary,
        debug_summary,
    );
    let event = append_rejection_event(context, proposal, &report);
    PipelineResult {
        report,
        appended_events: vec![event],
    }
}

fn rejection_report(
    proposal: &Proposal,
    failed_stage: PipelineStage,
    reason_codes: Vec<ReasonCode>,
    checked_facts: Vec<CheckedFact>,
    actor_visible_summary: impl Into<String>,
    debug_summary: impl Into<String>,
) -> ValidationReport {
    let report_id = report_id(proposal);
    let event_id = rejection_event_id(proposal);
    ValidationReport {
        validation_report_id: report_id.clone(),
        proposal_id: proposal.proposal_id.clone(),
        actor_id: proposal.actor_id.clone(),
        action_id: proposal.action_id.clone(),
        target_ids: proposal.target_ids.clone(),
        status: ReportStatus::Rejected,
        failed_stage: Some(failed_stage),
        reason_codes: reason_codes.clone(),
        actor_visible_facts: actor_visible_facts(&checked_facts),
        debug_only_facts: debug_only_facts(&checked_facts),
        checked_facts,
        actor_visible_summary: actor_visible_summary.into(),
        debug_summary: debug_summary.into(),
        would_mutate: false,
        event_ids: vec![event_id.clone()],
        checksum_before: None,
        checksum_after: None,
    }
}

fn append_rejection_event(
    context: &mut PipelineContext<'_>,
    proposal: &Proposal,
    report: &ValidationReport,
) -> EventEnvelope {
    let event_id = rejection_event_id(proposal);

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
    event.validation_report_id = Some(report.validation_report_id.clone());
    event.payload = report
        .reason_codes
        .iter()
        .map(|code| PayloadField::new("reason_code", code.stable_id()))
        .collect();
    event.effects_summary = "rejected before world mutation".to_string();

    let _ = context.log.append(event.clone());
    event
}

fn report_id(proposal: &Proposal) -> ValidationReportId {
    ValidationReportId::new(format!("report.{}", proposal.proposal_id.as_str())).unwrap()
}

fn rejection_event_id(proposal: &Proposal) -> EventId {
    EventId::new(format!("event.rejection.{}", proposal.proposal_id.as_str())).unwrap()
}

fn build_absence_detection_events(
    projection: &EpistemicProjection,
    state: &PhysicalState,
    check_event: &EventEnvelope,
    observation_event: &EventEnvelope,
    ordering_key: &OrderingKey,
    content_manifest_id: &ContentManifestId,
) -> Vec<EventEnvelope> {
    let Some(actor_id) = check_event.actor_id.as_ref() else {
        return Vec::new();
    };
    let Some(container_id) =
        payload_value(check_event, "container_id").and_then(|value| ContainerId::new(value).ok())
    else {
        return Vec::new();
    };
    let Some(observation_id) = payload_value(observation_event, "observation_id")
        .and_then(|value| crate::ids::ObservationId::new(value).ok())
    else {
        return Vec::new();
    };
    let observed_item_ids = state
        .containers
        .get(&container_id)
        .map(|container| container.contents.clone())
        .unwrap_or_default();
    let knowledge_context =
        KnowledgeContext::embodied(actor_id.clone(), observation_event.sim_tick);
    let expectation_beliefs = projection.beliefs_for_context(&knowledge_context);
    let detections = detect_expected_absences(
        actor_id,
        &container_id,
        &observed_item_ids,
        &expectation_beliefs,
        &observation_id,
        &observation_event.event_id,
        observation_event.sim_tick,
        Confidence::new(1000).unwrap(),
    );
    let mut events = Vec::new();
    for detection in detections {
        let mut contradiction_event = EventEnvelope::new_v1(
            EventId::new(format!(
                "event.expectation_contradicted.{}",
                detection.contradiction.contradiction_id().as_str()
            ))
            .unwrap(),
            EventKind::ExpectationContradicted,
            0,
            0,
            observation_event.sim_tick,
            ordering_key.clone(),
            content_manifest_id.clone(),
        );
        contradiction_event.actor_id = Some(actor_id.clone());
        contradiction_event.proposal_id = check_event.proposal_id.clone();
        contradiction_event.causes = vec![crate::events::EventCause::Event(
            observation_event.event_id.clone(),
        )];
        contradiction_event.payload = vec![
            PayloadField::new("schema_version", EVENT_SCHEMA_V1),
            PayloadField::new(
                "contradiction_id",
                detection.contradiction.contradiction_id().as_str(),
            ),
            PayloadField::new("holder_actor_id", actor_id.as_str()),
            PayloadField::new(
                "prior_expectation_belief_id",
                detection
                    .contradiction
                    .prior_expectation_belief_id()
                    .as_str(),
            ),
            PayloadField::new(
                "contradicting_observation_id",
                detection
                    .contradiction
                    .contradicting_observation_id()
                    .as_str(),
            ),
            PayloadField::new(
                "expected_proposition",
                detection
                    .contradiction
                    .expected_proposition()
                    .serialize_canonical(),
            ),
            PayloadField::new(
                "observed_proposition",
                detection
                    .contradiction
                    .observed_proposition()
                    .serialize_canonical(),
            ),
            PayloadField::new(
                "detected_tick",
                observation_event.sim_tick.value().to_string(),
            ),
        ];
        contradiction_event.effects_summary = "expectation contradicted by check".to_string();

        let mut belief_event = EventEnvelope::new_v1(
            EventId::new(format!(
                "event.belief_updated.{}",
                detection.missing_belief.belief_id().as_str()
            ))
            .unwrap(),
            EventKind::BeliefUpdated,
            0,
            0,
            observation_event.sim_tick,
            ordering_key.clone(),
            content_manifest_id.clone(),
        );
        belief_event.actor_id = Some(actor_id.clone());
        belief_event.proposal_id = check_event.proposal_id.clone();
        belief_event.causes = vec![
            crate::events::EventCause::Event(observation_event.event_id.clone()),
            crate::events::EventCause::Event(contradiction_event.event_id.clone()),
        ];
        belief_event.payload = vec![
            PayloadField::new("schema_version", EVENT_SCHEMA_V1),
            PayloadField::new("belief_id", detection.missing_belief.belief_id().as_str()),
            PayloadField::new("holder_actor_id", actor_id.as_str()),
            PayloadField::new(
                "proposition",
                detection.missing_belief.proposition().serialize_canonical(),
            ),
            PayloadField::new("stance", detection.missing_belief.stance().stable_id()),
            PayloadField::new(
                "confidence",
                detection
                    .missing_belief
                    .confidence()
                    .parts_per_thousand()
                    .to_string(),
            ),
            PayloadField::new("source_event_id", observation_event.event_id.as_str()),
            PayloadField::new(
                "acquired_tick",
                observation_event.sim_tick.value().to_string(),
            ),
            PayloadField::new("channel", "absence_marker"),
            PayloadField::new(
                "contradiction_ids",
                detection
                    .missing_belief
                    .contradiction_ids()
                    .iter()
                    .map(|id| id.as_str())
                    .collect::<Vec<_>>()
                    .join(","),
            ),
        ];
        belief_event.effects_summary = "missing expected item belief updated".to_string();

        events.push(contradiction_event);
        events.push(belief_event);
    }
    events
}

fn payload_value<'a>(event: &'a EventEnvelope, key: &str) -> Option<&'a str> {
    event
        .payload
        .iter()
        .find(|field| field.key == key)
        .map(|field| field.value.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actions::{
        ActionDefinition, ActionScope, ProposalOrigin, ProposalSource, ProposalSourceContext,
    };
    use crate::controller::ControllerBindings;
    use crate::epistemics::KnowledgeContext;
    use crate::events::apply::apply_event;
    use crate::events::{EventCause, EventKind, PayloadField, EVENT_SCHEMA_V1};
    use crate::ids::{
        ActionId, ActorId, EventId, ProposalId, SemanticActionId, SleepAffordanceId, ViewModelId,
    };
    use crate::scheduler::{ProposalSequence, SchedulePhase, SchedulerSourceId};
    use crate::state::{
        ActorBody, AgentState, ContainerState, DoorState, PlaceState, SleepAffordanceState,
    };
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

    fn attach_tui_source(proposal: &mut Proposal, semantic_action_id: &str, frontier: u64) {
        let actor_id = proposal.actor_id.clone().unwrap();
        let context = KnowledgeContext::embodied_at_frontier(
            actor_id.clone(),
            proposal.requested_tick,
            frontier,
        );
        let source_view_model_id = ViewModelId::new(format!(
            "view.{}.{}",
            actor_id.as_str(),
            proposal.requested_tick.value()
        ))
        .unwrap();
        proposal.source_view_model_id = Some(source_view_model_id.clone());
        proposal.source = Some(ProposalSource::TuiSemanticAction(ProposalSourceContext {
            source_view_model_id,
            holder_known_context_id: context.holder_known_context_id().clone(),
            holder_known_context_hash: context.holder_known_context_hash().clone(),
            holder_known_context_frontier: context.event_frontier(),
            context_tick: proposal.requested_tick,
            actor_id,
            semantic_action_id: SemanticActionId::new(semantic_action_id).unwrap(),
            provenance_ancestry: Vec::new(),
        }));
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
        let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
        state.actors.insert(
            actor_id("actor_tomas"),
            ActorBody::new(
                actor_id("actor_tomas"),
                crate::ids::PlaceId::new("shop_front").unwrap(),
            ),
        );
        state
    }

    fn sleep_state() -> PhysicalState {
        let mut state = state();
        let place_id = crate::ids::PlaceId::new("shop_front").unwrap();
        state.places.insert(
            place_id.clone(),
            PlaceState::new(place_id.clone(), "Shop front"),
        );
        let sleep_affordance_id = SleepAffordanceId::new("sleep_mat_shop").unwrap();
        state.sleep_affordances.insert(
            sleep_affordance_id.clone(),
            SleepAffordanceState::new(sleep_affordance_id, place_id, 4, 20, 2),
        );
        state
    }

    fn open_sleep_start_event() -> EventEnvelope {
        let actor = actor_id("actor_tomas");
        let mut event = EventEnvelope::new_caused_v1(
            EventId::new("event_sleep_started_open").unwrap(),
            EventKind::SleepStarted,
            0,
            0,
            SimTick::new(9),
            OrderingKey::new(
                SimTick::new(9),
                SchedulePhase::HumanCommand,
                SchedulerSourceId::Actor(actor.clone()),
                ProposalSequence::new(0),
                action_id("sleep"),
                Vec::new(),
                "sleep_open",
            ),
            content_manifest_id(),
            vec![EventCause::Proposal(
                ProposalId::new("proposal_sleep_open").unwrap(),
            )],
        )
        .unwrap();
        event.actor_id = Some(actor.clone());
        event.proposal_id = Some(ProposalId::new("proposal_sleep_open").unwrap());
        event.payload = vec![
            PayloadField::new("schema_version", EVENT_SCHEMA_V1),
            PayloadField::new("actor_id", actor.as_str()),
            PayloadField::new("expected_completion_tick", "13"),
            PayloadField::new("body_exclusive", "true"),
            PayloadField::new("sleep_affordance_id", "sleep_mat_shop"),
            PayloadField::new("fatigue_recovery_per_tick", "20"),
            PayloadField::new("hunger_rise_per_tick", "2"),
        ];
        event
    }

    fn run_pipeline_with_open_sleep(
        registry: &ActionRegistry,
        state: &mut PhysicalState,
        agent_state: &mut AgentState,
        proposal: &Proposal,
    ) -> PipelineResult {
        let mut log = EventLog::new();
        log.append(open_sleep_start_event()).unwrap();
        let controller_bindings =
            matches!(proposal.origin, ProposalOrigin::Human).then(human_bindings);
        run_pipeline(
            &mut PipelineContext {
                registry,
                state,
                agent_state,
                log: &mut log,
                controller_bindings: controller_bindings.as_ref(),
                epistemic_projection: None,
                content_manifest_id: content_manifest_id(),
                ordering_key: OrderingKey::new(
                    proposal.requested_tick,
                    SchedulePhase::HumanCommand,
                    SchedulerSourceId::Actor(actor_id("actor_tomas")),
                    ProposalSequence::new(1),
                    proposal.action_id.clone(),
                    proposal.target_ids.clone(),
                    proposal.proposal_id.as_str().to_string(),
                ),
            },
            proposal,
        )
    }

    fn check_state() -> PhysicalState {
        let mut state = state();
        let place_id = crate::ids::PlaceId::new("shop_front").unwrap();
        state.places.insert(
            place_id.clone(),
            PlaceState::new(place_id.clone(), "Shop front"),
        );
        let mut container = ContainerState::fixed_at_place(
            crate::ids::ContainerId::new("strongbox_tomas").unwrap(),
            place_id,
        );
        container.is_open = true;
        state.containers.insert(
            crate::ids::ContainerId::new("strongbox_tomas").unwrap(),
            container,
        );
        state
    }

    fn door_state() -> PhysicalState {
        let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
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

    fn content_manifest_id() -> ContentManifestId {
        ContentManifestId::new("phase2a_manifest").unwrap()
    }

    fn phase2a_registry() -> ActionRegistry {
        let mut registry = ActionRegistry::new();
        registry.register_phase1_movement_open_close();
        registry.register_phase1_inspect_wait();
        registry.register_phase2a_epistemics();
        registry
    }

    fn human_bindings() -> ControllerBindings {
        let mut bindings = ControllerBindings::new();
        let mut binding_log = EventLog::new();
        bindings.attach(
            crate::ids::ControllerId::new("controller_human").unwrap(),
            actor_id("actor_tomas"),
            crate::state::ControllerMode::Embodied,
            SimTick::ZERO,
            &mut binding_log,
            content_manifest_id(),
        );
        bindings
    }

    fn human_validation_report(
        registry: &ActionRegistry,
        state: &PhysicalState,
        proposal: &Proposal,
        bindings: &ControllerBindings,
        current_event_frontier: u64,
    ) -> ValidationReport {
        let ordering_key = OrderingKey::new(
            proposal.requested_tick,
            SchedulePhase::HumanCommand,
            SchedulerSourceId::Controller(
                crate::ids::ControllerId::new("controller_human").unwrap(),
            ),
            ProposalSequence::new(0),
            proposal.action_id.clone(),
            proposal.target_ids.clone(),
            "source-test",
        );
        validate_proposal(
            ProposalValidationContext {
                registry,
                state,
                agent_state: &AgentState::default(),
                controller_bindings: Some(bindings),
                epistemic_projection: None,
                content_manifest_id: &content_manifest_id(),
                ordering_key: &ordering_key,
                current_event_frontier,
            },
            proposal,
        )
    }

    fn check_container_proposal(proposal_id: &str) -> Proposal {
        let mut proposal = Proposal::new(
            ProposalId::new(proposal_id).unwrap(),
            ProposalOrigin::Test,
            Some(actor_id("actor_tomas")),
            action_id("check_container"),
            SimTick::ZERO,
        );
        proposal.target_ids.push("strongbox_tomas".to_string());
        proposal
    }

    fn move_proposal(proposal_id: &str) -> Proposal {
        let mut proposal = Proposal::new(
            ProposalId::new(proposal_id).unwrap(),
            ProposalOrigin::Test,
            Some(actor_id("actor_tomas")),
            action_id("move"),
            SimTick::ZERO,
        );
        proposal.target_ids.push("back_room".to_string());
        proposal
    }

    fn committed_report_for(
        registry: &ActionRegistry,
        state: &mut PhysicalState,
        proposal: &Proposal,
    ) -> ValidationReport {
        let mut log = EventLog::new();
        let mut projection = EpistemicProjection::new(content_manifest_id());
        let mut context = PipelineContext {
            registry,
            state,
            agent_state: Box::leak(Box::new(crate::state::AgentState::default())),
            log: &mut log,
            controller_bindings: None,
            epistemic_projection: Some(&mut projection),
            content_manifest_id: content_manifest_id(),
            ordering_key: ordering_key(),
        };
        run_pipeline(&mut context, proposal).report
    }

    fn has_fact(facts: &[CheckedFact], stable_key: &str) -> bool {
        facts.iter().any(|fact| fact.stable_key() == stable_key)
    }

    fn fact_value<'a>(facts: &'a [CheckedFact], stable_key: &str) -> Option<&'a str> {
        facts
            .iter()
            .find(|fact| fact.stable_key() == stable_key)
            .map(CheckedFact::value)
    }

    fn wait_proposal(origin: ProposalOrigin, proposal_id: &str) -> Proposal {
        let is_human = origin == ProposalOrigin::Human;
        let mut proposal = Proposal::new(
            ProposalId::new(proposal_id).unwrap(),
            origin,
            Some(actor_id("actor_tomas")),
            action_id("wait"),
            SimTick::new(10),
        );
        proposal
            .parameters
            .insert("reason".to_string(), "wait during open sleep".to_string());
        if is_human {
            proposal
                .parameters
                .insert("controller_id".to_string(), "controller_human".to_string());
            attach_tui_source(&mut proposal, "wait", 1);
        }
        proposal
    }

    #[test]
    fn reservation_rejects_human_wait_during_open_body_exclusive_duration() {
        let mut registry = ActionRegistry::new();
        registry.register_phase1_inspect_wait();
        let mut state = sleep_state();
        let mut agent_state = AgentState::default();
        let proposal = wait_proposal(ProposalOrigin::Human, "proposal_human_wait_asleep");

        let result =
            run_pipeline_with_open_sleep(&registry, &mut state, &mut agent_state, &proposal);

        assert_eq!(result.report.status, ReportStatus::Rejected);
        assert_eq!(
            result.report.failed_stage,
            Some(PipelineStage::ReservationConflictCheck)
        );
        assert_eq!(
            result.report.reason_codes,
            vec![ReasonCode::ReservationConflict]
        );
        assert!(result
            .appended_events
            .iter()
            .all(|event| event.event_type != EventKind::ActorWaited));
    }

    #[test]
    fn reservation_rejects_scheduler_wait_during_open_body_exclusive_duration() {
        let mut registry = ActionRegistry::new();
        registry.register_phase1_inspect_wait();
        let mut state = sleep_state();
        let mut agent_state = AgentState::default();
        let proposal = wait_proposal(ProposalOrigin::Scheduler, "proposal_scheduler_wait_asleep");

        let result =
            run_pipeline_with_open_sleep(&registry, &mut state, &mut agent_state, &proposal);

        assert_eq!(result.report.status, ReportStatus::Rejected);
        assert_eq!(
            result.report.failed_stage,
            Some(PipelineStage::ReservationConflictCheck)
        );
        assert_eq!(
            result.report.reason_codes,
            vec![ReasonCode::ReservationConflict]
        );
        assert!(result
            .appended_events
            .iter()
            .all(|event| event.event_type != EventKind::ActorWaited));
    }

    #[test]
    fn reservation_still_rejects_second_sleep_during_open_body_exclusive_duration() {
        let mut registry = ActionRegistry::new();
        registry.register_phase3a_sleep();
        let mut state = sleep_state();
        let mut agent_state = AgentState::default();
        let mut proposal = Proposal::new(
            ProposalId::new("proposal_second_sleep_asleep").unwrap(),
            ProposalOrigin::Scheduler,
            Some(actor_id("actor_tomas")),
            action_id("sleep"),
            SimTick::new(10),
        );
        proposal.parameters.insert(
            "sleep_affordance_id".to_string(),
            "sleep_mat_shop".to_string(),
        );

        let result =
            run_pipeline_with_open_sleep(&registry, &mut state, &mut agent_state, &proposal);

        assert_eq!(result.report.status, ReportStatus::Rejected);
        assert_eq!(
            result.report.failed_stage,
            Some(PipelineStage::ReservationConflictCheck)
        );
        assert_eq!(
            result.report.reason_codes,
            vec![ReasonCode::ReservationConflict]
        );
    }

    #[test]
    fn human_source_context_missing_rejects_before_action_lookup() {
        let registry = ActionRegistry::new();
        let state = state();
        let bindings = human_bindings();
        let mut proposal = Proposal::new(
            ProposalId::new("proposal_missing_source").unwrap(),
            ProposalOrigin::Human,
            Some(actor_id("actor_tomas")),
            action_id("unknown_action"),
            SimTick::ZERO,
        );
        proposal
            .parameters
            .insert("controller_id".to_string(), "controller_human".to_string());

        let report = human_validation_report(&registry, &state, &proposal, &bindings, 0);

        assert_eq!(report.status, ReportStatus::Rejected);
        assert_eq!(
            report.failed_stage,
            Some(PipelineStage::SourceContextValidation)
        );
        assert_eq!(report.reason_codes, vec![ReasonCode::ProposalSourceMissing]);
        assert!(!report.actor_visible_summary.contains("unknown_action"));
    }

    #[test]
    fn human_source_context_fresh_view_passes_source_validation() {
        let mut registry = ActionRegistry::new();
        registry.register(ActionDefinition::query_only(
            action_id("look"),
            ActionScope::Phase1,
        ));
        let state = state();
        let bindings = human_bindings();
        let mut proposal = proposal(ProposalOrigin::Human);
        attach_tui_source(&mut proposal, "look", 0);
        proposal
            .parameters
            .insert("controller_id".to_string(), "controller_human".to_string());

        let report = human_validation_report(&registry, &state, &proposal, &bindings, 0);

        assert_eq!(report.status, ReportStatus::Accepted);
    }

    #[test]
    fn human_source_context_stale_frontier_rejects_before_action_validation() {
        let mut registry = ActionRegistry::new();
        registry.register(ActionDefinition::query_only(
            action_id("look"),
            ActionScope::Phase1,
        ));
        let state = state();
        let bindings = human_bindings();
        let mut proposal = proposal(ProposalOrigin::Human);
        attach_tui_source(&mut proposal, "look", 0);
        proposal
            .parameters
            .insert("controller_id".to_string(), "controller_human".to_string());

        let report = human_validation_report(&registry, &state, &proposal, &bindings, 1);

        assert_eq!(report.status, ReportStatus::Rejected);
        assert_eq!(
            report.failed_stage,
            Some(PipelineStage::SourceContextValidation)
        );
        assert_eq!(report.reason_codes, vec![ReasonCode::ProposalSourceStale]);
        assert!(!report.actor_visible_summary.contains("hkc."));
        assert!(has_fact(&report.actor_visible_facts, "action_id"));
        assert!(has_fact(&report.debug_only_facts, "source_kind"));
        assert_eq!(
            fact_value(&report.debug_only_facts, "source_kind"),
            Some("tui_semantic_action")
        );
        assert_eq!(
            fact_value(&report.debug_only_facts, "semantic_action_id"),
            Some("look")
        );
        assert!(has_fact(
            &report.debug_only_facts,
            "holder_known_context_id"
        ));
        assert!(has_fact(
            &report.debug_only_facts,
            "holder_known_context_hash"
        ));
        assert!(has_fact(
            &report.debug_only_facts,
            "holder_known_context_frontier"
        ));
        assert!(!has_fact(&report.actor_visible_facts, "source_kind"));
        assert!(!has_fact(
            &report.actor_visible_facts,
            "holder_known_context_id"
        ));
        assert!(!has_fact(
            &report.actor_visible_facts,
            "holder_known_context_hash"
        ));
    }

    #[test]
    fn human_source_context_cross_actor_rejects() {
        let mut registry = ActionRegistry::new();
        registry.register(ActionDefinition::query_only(
            action_id("look"),
            ActionScope::Phase1,
        ));
        let state = state();
        let bindings = human_bindings();
        let mut proposal = proposal(ProposalOrigin::Human);
        attach_tui_source(&mut proposal, "look", 0);
        if let Some(ProposalSource::TuiSemanticAction(source)) = proposal.source.as_mut() {
            source.actor_id = actor_id("actor_elena");
        }
        proposal
            .parameters
            .insert("controller_id".to_string(), "controller_human".to_string());

        let report = human_validation_report(&registry, &state, &proposal, &bindings, 0);

        assert_eq!(report.status, ReportStatus::Rejected);
        assert_eq!(
            report.failed_stage,
            Some(PipelineStage::SourceContextValidation)
        );
        assert_eq!(
            report.reason_codes,
            vec![ReasonCode::ProposalSourceActorMismatch]
        );
        assert!(!report.actor_visible_summary.contains("actor_elena"));
    }

    #[test]
    fn human_source_context_forged_action_token_rejects() {
        let mut registry = ActionRegistry::new();
        registry.register(ActionDefinition::query_only(
            action_id("look"),
            ActionScope::Phase1,
        ));
        let state = state();
        let bindings = human_bindings();
        let mut proposal = proposal(ProposalOrigin::Human);
        attach_tui_source(&mut proposal, "move.to.hidden_room", 0);
        proposal
            .parameters
            .insert("controller_id".to_string(), "controller_human".to_string());

        let report = human_validation_report(&registry, &state, &proposal, &bindings, 0);

        assert_eq!(report.status, ReportStatus::Rejected);
        assert_eq!(
            report.failed_stage,
            Some(PipelineStage::SourceContextValidation)
        );
        assert_eq!(report.reason_codes, vec![ReasonCode::ProposalSourceForged]);
        assert!(!report.actor_visible_summary.contains("hidden_room"));
        assert!(has_fact(&report.debug_only_facts, "source_kind"));
        assert!(has_fact(&report.debug_only_facts, "semantic_action_id"));
        assert!(!has_fact(&report.actor_visible_facts, "source_kind"));
        assert!(!has_fact(&report.actor_visible_facts, "semantic_action_id"));
    }

    #[test]
    fn human_source_context_matches_exact_and_dotted_semantic_actions() {
        let mut registry = ActionRegistry::new();
        registry.register(ActionDefinition::query_only(
            action_id("look"),
            ActionScope::Phase1,
        ));
        registry.register(ActionDefinition::query_only(
            action_id("inspect_place"),
            ActionScope::Phase1,
        ));
        let state = state();
        let bindings = human_bindings();

        let mut exact = proposal(ProposalOrigin::Human);
        attach_tui_source(&mut exact, "look", 0);
        exact
            .parameters
            .insert("controller_id".to_string(), "controller_human".to_string());
        let exact_report = human_validation_report(&registry, &state, &exact, &bindings, 0);
        assert_eq!(exact_report.status, ReportStatus::Accepted);

        let mut exact_prefix = proposal(ProposalOrigin::Human);
        attach_tui_source(&mut exact_prefix, "look.around", 0);
        exact_prefix
            .parameters
            .insert("controller_id".to_string(), "controller_human".to_string());
        let exact_prefix_report =
            human_validation_report(&registry, &state, &exact_prefix, &bindings, 0);
        assert_eq!(exact_prefix_report.status, ReportStatus::Accepted);

        let mut dotted = Proposal::new(
            ProposalId::new("proposal_inspect_place").unwrap(),
            ProposalOrigin::Human,
            Some(actor_id("actor_tomas")),
            action_id("inspect_place"),
            SimTick::ZERO,
        );
        attach_tui_source(&mut dotted, "inspect.place.shop_front", 0);
        dotted
            .parameters
            .insert("controller_id".to_string(), "controller_human".to_string());
        let dotted_report = human_validation_report(&registry, &state, &dotted, &bindings, 0);
        assert_eq!(dotted_report.status, ReportStatus::Accepted);

        let mut mismatched = proposal(ProposalOrigin::Human);
        attach_tui_source(&mut mismatched, "look.hidden", 0);
        mismatched.action_id = action_id("inspect_place");
        mismatched
            .parameters
            .insert("controller_id".to_string(), "controller_human".to_string());
        let mismatch_report = human_validation_report(&registry, &state, &mismatched, &bindings, 0);
        assert_eq!(mismatch_report.status, ReportStatus::Rejected);
        assert_eq!(
            mismatch_report.reason_codes,
            vec![ReasonCode::ProposalSourceForged]
        );
    }

    #[test]
    fn semantic_action_match_accepts_exact_and_prefix_forms() {
        assert!(semantic_action_matches_action(
            &SemanticActionId::new("look").unwrap(),
            &action_id("look")
        ));
        assert!(semantic_action_matches_action(
            &SemanticActionId::new("look.around").unwrap(),
            &action_id("look")
        ));
        assert!(semantic_action_matches_action(
            &SemanticActionId::new("inspect_place.shop_front").unwrap(),
            &action_id("inspect_place")
        ));
        assert!(semantic_action_matches_action(
            &SemanticActionId::new("inspect.place.shop_front").unwrap(),
            &action_id("inspect_place")
        ));
        assert!(!semantic_action_matches_action(
            &SemanticActionId::new("look.hidden").unwrap(),
            &action_id("inspect_place")
        ));
    }

    #[test]
    fn query_only_look_keeps_knowledge_slot_inert() {
        let mut registry = ActionRegistry::new();
        registry.register(ActionDefinition::query_only(
            action_id("look"),
            ActionScope::Phase1,
        ));
        let report = validate_proposal(
            ProposalValidationContext {
                registry: &registry,
                state: &state(),
                agent_state: &AgentState::default(),
                controller_bindings: None,
                epistemic_projection: None,
                content_manifest_id: &content_manifest_id(),
                ordering_key: &ordering_key(),
                current_event_frontier: 0,
            },
            &proposal(ProposalOrigin::Test),
        );

        assert_eq!(report.status, ReportStatus::Accepted);
        assert_eq!(
            fact_value(&report.checked_facts, "knowledge_perception_slot"),
            Some("inert")
        );
    }

    #[test]
    fn human_source_context_rejects_each_stale_tick_side() {
        let mut registry = ActionRegistry::new();
        registry.register(ActionDefinition::query_only(
            action_id("look"),
            ActionScope::Phase1,
        ));
        let state = state();
        let bindings = human_bindings();

        let mut requested_tick_mismatch = proposal(ProposalOrigin::Human);
        attach_tui_source(&mut requested_tick_mismatch, "look", 0);
        requested_tick_mismatch.requested_tick = SimTick::new(1);
        requested_tick_mismatch
            .parameters
            .insert("controller_id".to_string(), "controller_human".to_string());
        let requested_report =
            human_validation_report(&registry, &state, &requested_tick_mismatch, &bindings, 0);
        assert_eq!(requested_report.status, ReportStatus::Rejected);
        assert_eq!(
            requested_report.reason_codes,
            vec![ReasonCode::ProposalSourceStale]
        );

        let mut ordering_tick_mismatch = proposal(ProposalOrigin::Human);
        attach_tui_source(&mut ordering_tick_mismatch, "look", 0);
        ordering_tick_mismatch.requested_tick = SimTick::new(1);
        if let Some(ProposalSource::TuiSemanticAction(source)) =
            ordering_tick_mismatch.source.as_mut()
        {
            source.context_tick = SimTick::new(1);
        }
        ordering_tick_mismatch
            .parameters
            .insert("controller_id".to_string(), "controller_human".to_string());
        let zero_ordering_key = ordering_key();
        let ordering_report = validate_proposal(
            ProposalValidationContext {
                registry: &registry,
                state: &state,
                agent_state: &AgentState::default(),
                controller_bindings: Some(&bindings),
                epistemic_projection: None,
                content_manifest_id: &content_manifest_id(),
                ordering_key: &zero_ordering_key,
                current_event_frontier: 0,
            },
            &ordering_tick_mismatch,
        );
        assert_eq!(ordering_report.status, ReportStatus::Rejected);
        assert_eq!(
            ordering_report.reason_codes,
            vec![ReasonCode::ProposalSourceStale]
        );
    }

    #[test]
    fn human_controller_binding_is_required_before_other_validation() {
        let mut registry = ActionRegistry::new();
        registry.register(ActionDefinition::query_only(
            action_id("look"),
            ActionScope::Phase1,
        ));
        let state = state();
        let bindings = human_bindings();
        let mut proposal = proposal(ProposalOrigin::Human);
        attach_tui_source(&mut proposal, "look", 0);

        let report = human_validation_report(&registry, &state, &proposal, &bindings, 0);

        assert_eq!(report.status, ReportStatus::Rejected);
        assert_eq!(
            report.failed_stage,
            Some(PipelineStage::ControllerBindingCheck)
        );
        assert_eq!(report.reason_codes, vec![ReasonCode::ControllerUnbound]);
    }

    #[test]
    fn disabled_actor_rejects_before_query_acceptance() {
        let mut registry = ActionRegistry::new();
        registry.register(ActionDefinition::query_only(
            action_id("look"),
            ActionScope::Phase1,
        ));
        let mut state = state();
        state
            .actors
            .get_mut(&actor_id("actor_tomas"))
            .unwrap()
            .enabled = false;
        let report = validate_proposal(
            ProposalValidationContext {
                registry: &registry,
                state: &state,
                agent_state: &AgentState::default(),
                controller_bindings: None,
                epistemic_projection: None,
                content_manifest_id: &content_manifest_id(),
                ordering_key: &ordering_key(),
                current_event_frontier: 0,
            },
            &proposal(ProposalOrigin::Test),
        );

        assert_eq!(report.status, ReportStatus::Rejected);
        assert_eq!(report.failed_stage, Some(PipelineStage::ActorLookup));
        assert_eq!(report.reason_codes, vec![ReasonCode::ActorNotFound]);
    }

    #[test]
    fn inactive_action_scope_rejects_registered_action() {
        let mut registry = ActionRegistry::new_with_active_scopes([ActionScope::Phase1]);
        registry.register(ActionDefinition::query_only(
            action_id("truthful_accuse_probe"),
            ActionScope::Phase2AHistorical,
        ));
        let mut proposal = Proposal::new(
            ProposalId::new("proposal_inactive_scope").unwrap(),
            ProposalOrigin::Test,
            Some(actor_id("actor_tomas")),
            action_id("truthful_accuse_probe"),
            SimTick::ZERO,
        );
        proposal.target_ids.push("actor_mara".to_string());
        let state = state();

        let report = validate_proposal(
            ProposalValidationContext {
                registry: &registry,
                state: &state,
                agent_state: &AgentState::default(),
                controller_bindings: None,
                epistemic_projection: None,
                content_manifest_id: &content_manifest_id(),
                ordering_key: &ordering_key(),
                current_event_frontier: 0,
            },
            &proposal,
        );

        assert_eq!(report.status, ReportStatus::Rejected);
        assert_eq!(
            report.failed_stage,
            Some(PipelineStage::PhaseBoundaryValidation)
        );
        assert_eq!(
            report.reason_codes,
            vec![ReasonCode::PhaseUnsupportedAction]
        );
    }

    #[test]
    fn body_exclusive_start_requires_kind_and_payload_flag() {
        let mut work = EventEnvelope::new_v1(
            EventId::new("event_work_started").unwrap(),
            EventKind::WorkBlockStarted,
            0,
            0,
            SimTick::ZERO,
            ordering_key(),
            content_manifest_id(),
        );
        work.payload
            .push(PayloadField::new("body_exclusive", "true"));
        assert!(is_body_exclusive_start(&work));

        let missing_flag = EventEnvelope::new_v1(
            EventId::new("event_work_started_missing_flag").unwrap(),
            EventKind::WorkBlockStarted,
            0,
            0,
            SimTick::ZERO,
            ordering_key(),
            content_manifest_id(),
        );
        assert!(!is_body_exclusive_start(&missing_flag));

        let mut false_flag = EventEnvelope::new_v1(
            EventId::new("event_work_started_false_flag").unwrap(),
            EventKind::WorkBlockStarted,
            0,
            0,
            SimTick::ZERO,
            ordering_key(),
            content_manifest_id(),
        );
        false_flag
            .payload
            .push(PayloadField::new("body_exclusive", "false"));
        assert!(!is_body_exclusive_start(&false_flag));

        let mut wrong_key = EventEnvelope::new_v1(
            EventId::new("event_work_started_wrong_key").unwrap(),
            EventKind::WorkBlockStarted,
            0,
            0,
            SimTick::ZERO,
            ordering_key(),
            content_manifest_id(),
        );
        wrong_key
            .payload
            .push(PayloadField::new("not_body_exclusive", "true"));
        assert!(!is_body_exclusive_start(&wrong_key));

        let mut wrong_kind = EventEnvelope::new_v1(
            EventId::new("event_actor_moved_body_flag").unwrap(),
            EventKind::ActorMoved,
            0,
            0,
            SimTick::ZERO,
            ordering_key(),
            content_manifest_id(),
        );
        wrong_kind
            .payload
            .push(PayloadField::new("body_exclusive", "true"));
        assert!(!is_body_exclusive_start(&wrong_kind));
    }

    #[test]
    fn agent_source_context_rejects_failed_hidden_truth_audit() {
        let registry = phase2a_registry();
        let mut state = state();
        let mut agent_state = AgentState::default();
        let mut log = EventLog::new();
        let mut proposal = proposal(ProposalOrigin::Agent);
        proposal.source = Some(ProposalSource::Agent);
        proposal.parameters.insert(
            "hidden_truth_audit_actor_known_only".to_string(),
            "false".to_string(),
        );
        let mut context = PipelineContext {
            registry: &registry,
            state: &mut state,
            agent_state: &mut agent_state,
            log: &mut log,
            controller_bindings: None,
            epistemic_projection: None,
            content_manifest_id: content_manifest_id(),
            ordering_key: ordering_key(),
        };

        let result = run_pipeline(&mut context, &proposal);

        assert_eq!(result.report.status, ReportStatus::Rejected);
        assert_eq!(
            result.report.failed_stage,
            Some(PipelineStage::SourceContextValidation)
        );
        assert!(result
            .report
            .reason_codes
            .contains(&ReasonCode::HiddenTruthInput));
        assert!(result.appended_events.iter().any(|event| {
            event.event_type == EventKind::ActionRejected
                && payload_value(event, "reason_code") == Some("hidden_truth_input")
        }));
    }

    #[test]
    fn agent_source_context_rejects_absent_hidden_truth_audit_stamp() {
        let registry = phase2a_registry();
        let mut state = state();
        let mut agent_state = AgentState::default();
        let mut log = EventLog::new();
        let mut proposal = proposal(ProposalOrigin::Agent);
        proposal.source = Some(ProposalSource::Agent);
        let mut context = PipelineContext {
            registry: &registry,
            state: &mut state,
            agent_state: &mut agent_state,
            log: &mut log,
            controller_bindings: None,
            epistemic_projection: None,
            content_manifest_id: content_manifest_id(),
            ordering_key: ordering_key(),
        };

        let result = run_pipeline(&mut context, &proposal);

        assert_eq!(result.report.status, ReportStatus::Rejected);
        assert_eq!(
            result.report.failed_stage,
            Some(PipelineStage::SourceContextValidation)
        );
        assert_eq!(
            result.report.reason_codes,
            vec![ReasonCode::HiddenTruthInput]
        );
        assert!(result
            .report
            .debug_summary
            .contains("lacked a clean hidden-truth audit stamp"));
    }

    #[test]
    fn same_proposal_validates_same_for_human_and_scheduler_origin() {
        let mut registry = ActionRegistry::new();
        registry.register(ActionDefinition::query_only(
            action_id("look"),
            ActionScope::Phase1,
        ));

        let mut human_state = state();
        let mut human_log = EventLog::new();
        let mut bindings = ControllerBindings::new();
        let mut binding_log = EventLog::new();
        bindings.attach(
            crate::ids::ControllerId::new("controller_human").unwrap(),
            actor_id("actor_tomas"),
            crate::state::ControllerMode::Embodied,
            SimTick::ZERO,
            &mut binding_log,
            ContentManifestId::new("phase1_manifest").unwrap(),
        );
        let mut human_proposal = proposal(ProposalOrigin::Human);
        attach_tui_source(&mut human_proposal, "look", 0);
        human_proposal
            .parameters
            .insert("controller_id".to_string(), "controller_human".to_string());
        let mut human_context = PipelineContext {
            registry: &registry,
            state: &mut human_state,
            agent_state: Box::leak(Box::new(crate::state::AgentState::default())),
            log: &mut human_log,
            controller_bindings: Some(&bindings),
            epistemic_projection: None,
            content_manifest_id: ContentManifestId::new("phase1_manifest").unwrap(),
            ordering_key: ordering_key(),
        };
        let human = run_pipeline(&mut human_context, &human_proposal);

        let mut scheduler_state = state();
        let mut scheduler_log = EventLog::new();
        let mut scheduler_context = PipelineContext {
            registry: &registry,
            state: &mut scheduler_state,
            agent_state: Box::leak(Box::new(crate::state::AgentState::default())),
            log: &mut scheduler_log,
            controller_bindings: None,
            epistemic_projection: None,
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
    fn validate_proposal_matches_committed_rejection_for_closed_container() {
        let registry = phase2a_registry();
        let state = {
            let mut state = check_state();
            state
                .containers
                .get_mut(&crate::ids::ContainerId::new("strongbox_tomas").unwrap())
                .unwrap()
                .is_open = false;
            state
        };
        let proposal = check_container_proposal("proposal_closed_container");

        let preflight = validate_proposal(
            ProposalValidationContext {
                registry: &registry,
                state: &state,
                agent_state: &AgentState::default(),
                controller_bindings: None,
                epistemic_projection: None,
                content_manifest_id: &content_manifest_id(),
                ordering_key: &ordering_key(),
                current_event_frontier: 0,
            },
            &proposal,
        );
        let committed = committed_report_for(&registry, &mut state.clone(), &proposal);

        assert_eq!(preflight.status, ReportStatus::Rejected);
        assert_eq!(preflight.reason_codes, vec![ReasonCode::ContainerClosed]);
        assert_eq!(preflight.actor_visible_summary, "The container is closed.");
        assert_eq!(preflight, committed);
    }

    #[test]
    fn validate_proposal_matches_committed_rejection_for_closed_door() {
        let registry = phase2a_registry();
        let state = door_state();
        let proposal = move_proposal("proposal_closed_door");

        let preflight = validate_proposal(
            ProposalValidationContext {
                registry: &registry,
                state: &state,
                agent_state: &AgentState::default(),
                controller_bindings: None,
                epistemic_projection: None,
                content_manifest_id: &content_manifest_id(),
                ordering_key: &ordering_key(),
                current_event_frontier: 0,
            },
            &proposal,
        );
        let committed = committed_report_for(&registry, &mut state.clone(), &proposal);

        assert_eq!(preflight.status, ReportStatus::Rejected);
        assert_eq!(
            preflight.reason_codes,
            vec![ReasonCode::DoorClosedBlocksMovement]
        );
        assert_eq!(preflight.actor_visible_summary, "The door is closed.");
        assert_eq!(preflight, committed);
    }

    #[test]
    fn validate_proposal_does_not_commit_or_mutate_state_log_or_epistemics() {
        let registry = phase2a_registry();
        let state = check_state();
        let log = EventLog::new();
        let projection = EpistemicProjection::new(content_manifest_id());
        let before_state = state.clone();
        let before_log_len = log.events().len();
        let before_projection = projection.clone();

        let accepted_check = check_container_proposal("proposal_accept_check");
        let mut accepted_wait = Proposal::new(
            ProposalId::new("proposal_accept_wait").unwrap(),
            ProposalOrigin::Test,
            Some(actor_id("actor_tomas")),
            action_id("wait"),
            SimTick::ZERO,
        );
        accepted_wait
            .parameters
            .insert("reason".to_string(), "validation wait".to_string());
        let mut rejected_state = state.clone();
        rejected_state
            .containers
            .get_mut(&crate::ids::ContainerId::new("strongbox_tomas").unwrap())
            .unwrap()
            .is_open = false;
        let rejected_check = check_container_proposal("proposal_reject_check");

        let accepted_check_report = validate_proposal(
            ProposalValidationContext {
                registry: &registry,
                state: &state,
                agent_state: &AgentState::default(),
                controller_bindings: None,
                epistemic_projection: Some(&projection),
                content_manifest_id: &content_manifest_id(),
                ordering_key: &ordering_key(),
                current_event_frontier: 0,
            },
            &accepted_check,
        );
        let accepted_wait_report = validate_proposal(
            ProposalValidationContext {
                registry: &registry,
                state: &state,
                agent_state: &AgentState::default(),
                controller_bindings: None,
                epistemic_projection: Some(&projection),
                content_manifest_id: &content_manifest_id(),
                ordering_key: &ordering_key(),
                current_event_frontier: 0,
            },
            &accepted_wait,
        );
        let rejected_check_report = validate_proposal(
            ProposalValidationContext {
                registry: &registry,
                state: &rejected_state,
                agent_state: &AgentState::default(),
                controller_bindings: None,
                epistemic_projection: Some(&projection),
                content_manifest_id: &content_manifest_id(),
                ordering_key: &ordering_key(),
                current_event_frontier: 0,
            },
            &rejected_check,
        );

        assert_eq!(accepted_check_report.status, ReportStatus::Accepted);
        assert_eq!(accepted_wait_report.status, ReportStatus::Accepted);
        assert_eq!(rejected_check_report.status, ReportStatus::Rejected);
        assert_eq!(state, before_state);
        assert_eq!(log.events().len(), before_log_len);
        assert_eq!(projection, before_projection);
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
            agent_state: Box::leak(Box::new(crate::state::AgentState::default())),
            log: &mut log,
            controller_bindings: None,
            epistemic_projection: None,
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
    fn pipeline_contains_epistemic_slot_and_later_inert_slots() {
        let stages = PipelineStage::all();
        assert!(stages.contains(&PipelineStage::SourceContextValidation));
        assert!(stages.contains(&PipelineStage::KnowledgePerceptionPlaceholder));
        assert!(stages.contains(&PipelineStage::SocialNormPlaceholder));
        assert!(stages.contains(&PipelineStage::CostDurationCheck));
        assert!(stages.contains(&PipelineStage::ReservationConflictCheck));
        assert_eq!(stages.len(), 20);
    }

    #[test]
    fn scheduler_origin_check_container_commits_without_controller_binding() {
        let mut registry = ActionRegistry::new();
        registry.register_phase2a_epistemics();
        let mut state = check_state();
        let mut log = EventLog::new();
        let mut proposal = Proposal::new(
            ProposalId::new("proposal_scheduler_check").unwrap(),
            ProposalOrigin::Scheduler,
            Some(actor_id("actor_tomas")),
            action_id("check_container"),
            SimTick::ZERO,
        );
        proposal.target_ids.push("strongbox_tomas".to_string());
        let mut context = PipelineContext {
            registry: &registry,
            state: &mut state,
            agent_state: Box::leak(Box::new(crate::state::AgentState::default())),
            log: &mut log,
            controller_bindings: None,
            epistemic_projection: None,
            content_manifest_id: ContentManifestId::new("phase2a_manifest").unwrap(),
            ordering_key: ordering_key(),
        };

        let result = run_pipeline(&mut context, &proposal);

        assert_eq!(result.report.status, ReportStatus::Accepted);
        assert_eq!(result.appended_events.len(), 2);
        assert_eq!(
            result.appended_events[0].event_type,
            EventKind::ContainerChecked
        );
        assert_eq!(
            result.appended_events[1].event_type,
            EventKind::ObservationRecorded
        );
        assert_eq!(log.events().len(), 2);
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
            agent_state: Box::leak(Box::new(crate::state::AgentState::default())),
            log: &mut live_log,
            controller_bindings: None,
            epistemic_projection: None,
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
            agent_state: Box::leak(Box::new(crate::state::AgentState::default())),
            log: &mut live_log,
            controller_bindings: None,
            epistemic_projection: None,
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
