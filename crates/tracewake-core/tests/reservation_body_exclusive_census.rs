mod support;

use support::generative::{
    actor_id, away_place_id, content_manifest_id, food_supply_id, initial_agent_state,
    initial_world, registry, sleep_affordance_id, workplace_id,
};
use tracewake_core::actions::{
    run_pipeline, ActionEffect, ActionRegistry, PipelineContext, PipelineStage, Proposal,
    ProposalOrigin, ProposalSource, ProposalSourceContext, ReasonCode, ReportStatus,
};
use tracewake_core::agent::current_place_knowledge_context;
use tracewake_core::controller::ControllerBindings;
use tracewake_core::events::log::EventLog;
use tracewake_core::events::{EventCause, EventEnvelope, EventKind, PayloadField, EVENT_SCHEMA_V1};
use tracewake_core::ids::{
    ActionId, ControllerId, EventId, ProcessId, ProposalId, SemanticActionId, ViewModelId,
};
use tracewake_core::scheduler::{OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId};
use tracewake_core::state::{ControllerMode, PhysicalState};
use tracewake_core::time::SimTick;

fn body_using_action_ids(registry: &ActionRegistry) -> Vec<ActionId> {
    registry
        .definitions()
        .filter(|definition| {
            matches!(
                definition.effect,
                ActionEffect::Move
                    | ActionEffect::Eat
                    | ActionEffect::Wait
                    | ActionEffect::Sleep
                    | ActionEffect::Work
            )
        })
        .map(|definition| definition.action_id.clone())
        .collect()
}

fn proposal_for(action_id: &ActionId, origin: ProposalOrigin) -> Proposal {
    let actor = actor_id();
    let mut proposal = Proposal::new(
        ProposalId::new(format!(
            "proposal_reservation_census_{}_{}",
            action_id.as_str(),
            match origin {
                ProposalOrigin::Human => "human",
                ProposalOrigin::Scheduler => "scheduler",
                ProposalOrigin::Test => "test",
                ProposalOrigin::Agent => "agent",
                ProposalOrigin::Debug => "debug",
            }
        ))
        .unwrap(),
        origin,
        Some(actor),
        action_id.clone(),
        SimTick::new(1),
    );
    match action_id.as_str() {
        "move" => proposal.target_ids.push(away_place_id().to_string()),
        "eat" => proposal.target_ids.push(food_supply_id().to_string()),
        "wait" => {
            proposal
                .parameters
                .insert("reason".to_string(), "reservation census wait".to_string());
            proposal
                .parameters
                .insert("ticks".to_string(), "1".to_string());
        }
        "sleep" => {
            proposal.parameters.insert(
                "sleep_affordance_id".to_string(),
                sleep_affordance_id().to_string(),
            );
            proposal
                .parameters
                .insert("duration_ticks".to_string(), "2".to_string());
        }
        "work_block" => proposal.target_ids.push(workplace_id().to_string()),
        other => panic!("unhandled body-using action in census: {other}"),
    }
    if proposal.origin == ProposalOrigin::Agent {
        proposal.parameters.insert(
            "hidden_truth_audit_actor_known_only".to_string(),
            "true".to_string(),
        );
    }
    proposal
}

fn semantic_action_id(action_id: &ActionId) -> SemanticActionId {
    let semantic = match action_id.as_str() {
        "work_block" => "work.block",
        other => other,
    };
    SemanticActionId::new(semantic).unwrap()
}

fn attach_human_source_and_binding(
    proposal: &mut Proposal,
    state: &PhysicalState,
) -> ControllerBindings {
    let controller_id = ControllerId::new("controller_reservation_census").unwrap();
    let content_manifest_id = content_manifest_id(0x6006);
    let source_context = current_place_knowledge_context(
        state,
        None,
        &actor_id(),
        proposal.requested_tick,
        &content_manifest_id,
        1,
    );
    let source_view_model_id = ViewModelId::new(format!(
        "view.reservation_census.{}",
        proposal.action_id.as_str()
    ))
    .unwrap();
    proposal.source_view_model_id = Some(source_view_model_id.clone());
    proposal.source = Some(ProposalSource::TuiSemanticAction(ProposalSourceContext {
        source_view_model_id,
        holder_known_context_id: source_context.holder_known_context_id().clone(),
        holder_known_context_hash: source_context.holder_known_context_hash().clone(),
        holder_known_context_frontier: source_context.event_frontier(),
        context_tick: proposal.requested_tick,
        actor_id: actor_id(),
        semantic_action_id: semantic_action_id(&proposal.action_id),
        provenance_ancestry: Vec::new(),
    }));
    proposal
        .parameters
        .insert("controller_id".to_string(), controller_id.to_string());

    let mut bindings = ControllerBindings::new();
    let mut binding_log = EventLog::new();
    bindings.attach(
        controller_id,
        actor_id(),
        ControllerMode::Embodied,
        SimTick::ZERO,
        &mut binding_log,
        content_manifest_id,
    );
    bindings
}

fn open_sleep_start_event() -> EventEnvelope {
    let actor = actor_id();
    let process = ProcessId::new("reservation_census").unwrap();
    let mut event = EventEnvelope::new_caused_v1(
        EventId::new("event.reservation_census.sleep_started").unwrap(),
        EventKind::SleepStarted,
        0,
        0,
        SimTick::ZERO,
        OrderingKey::new(
            SimTick::ZERO,
            SchedulePhase::HumanCommand,
            SchedulerSourceId::Actor(actor.clone()),
            ProposalSequence::new(0),
            ActionId::new("sleep").unwrap(),
            vec![sleep_affordance_id().to_string()],
            "reservation_census_sleep",
        ),
        content_manifest_id(0x6006),
        vec![EventCause::Process(process)],
    )
    .unwrap();
    event.actor_id = Some(actor.clone());
    event.proposal_id = Some(ProposalId::new("proposal_reservation_census_open_sleep").unwrap());
    event.payload = vec![
        PayloadField::new("schema_version", EVENT_SCHEMA_V1),
        PayloadField::new("actor_id", actor.as_str()),
        PayloadField::new("expected_completion_tick", "4"),
        PayloadField::new("body_exclusive", "true"),
        PayloadField::new("sleep_affordance_id", sleep_affordance_id().to_string()),
        PayloadField::new("fatigue_recovery_per_tick", "20"),
        PayloadField::new("hunger_rise_per_tick", "2"),
    ];
    event
}

fn run_with_open_reservation(
    action_id: &ActionId,
    origin: ProposalOrigin,
) -> tracewake_core::actions::PipelineResult {
    let mut world = initial_world(0x6006);
    let mut agent = initial_agent_state(0x6006);
    let registry = registry();
    let mut proposal = proposal_for(action_id, origin);
    let mut log = EventLog::new();
    log.append(open_sleep_start_event()).unwrap();
    let human_bindings = if proposal.origin == ProposalOrigin::Human {
        Some(attach_human_source_and_binding(&mut proposal, &world))
    } else {
        None
    };
    run_pipeline(
        &mut PipelineContext {
            registry: &registry,
            state: &mut world,
            agent_state: &mut agent,
            log: &mut log,
            controller_bindings: human_bindings.as_ref(),
            epistemic_projection: None,
            content_manifest_id: content_manifest_id(0x6006),
            ordering_key: OrderingKey::new(
                proposal.requested_tick,
                SchedulePhase::HumanCommand,
                SchedulerSourceId::Actor(actor_id()),
                ProposalSequence::new(1),
                proposal.action_id.clone(),
                proposal.target_ids.clone(),
                proposal.proposal_id.as_str().to_string(),
            ),
        },
        &proposal,
    )
}

#[test]
fn derived_body_exclusive_reservation_census_rejects_body_using_actions() {
    let registry = registry();
    let action_ids = body_using_action_ids(&registry);
    assert_eq!(
        action_ids.iter().map(|id| id.as_str()).collect::<Vec<_>>(),
        vec!["eat", "move", "sleep", "wait", "work_block"]
    );

    for action_id in &action_ids {
        for origin in [
            ProposalOrigin::Human,
            ProposalOrigin::Scheduler,
            ProposalOrigin::Agent,
        ] {
            let result = run_with_open_reservation(action_id, origin.clone());
            assert_eq!(
                result.report.status,
                ReportStatus::Rejected,
                "action={} origin={:?}",
                action_id.as_str(),
                origin
            );
            assert_eq!(
                result.report.failed_stage,
                Some(PipelineStage::ReservationConflictCheck),
                "action={}",
                action_id.as_str()
            );
            assert_eq!(
                result.report.reason_codes,
                vec![ReasonCode::ReservationConflict],
                "action={}",
                action_id.as_str()
            );
        }
    }
}
