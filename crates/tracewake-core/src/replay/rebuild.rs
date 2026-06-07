use crate::checksum::{
    compute_agent_state_checksum, compute_physical_checksum, AgentStateChecksum,
    AgentStateChecksumReport, ChecksumContext, PhysicalChecksum,
};
use crate::epistemics::{EpistemicProjection, EpistemicProjectionChecksum};
use crate::events::apply::{apply_agent_event, apply_epistemic_event, apply_event};
use crate::events::log::EventLog;
use crate::events::{EventEnvelope, EventKind, EventStream};
use crate::ids::{ContentManifestId, EventId};
use crate::state::{AgentState, PhysicalState};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProjectionRebuildReport {
    pub final_state: PhysicalState,
    pub event_count_applied: usize,
    pub last_event_id: Option<EventId>,
    pub last_stream_position: Option<u64>,
    pub content_manifest_id: ContentManifestId,
    pub final_checksum: PhysicalChecksum,
    pub final_epistemic_projection: EpistemicProjection,
    pub final_epistemic_checksum: EpistemicProjectionChecksum,
    pub final_agent_state: AgentState,
    pub final_agent_checksum: AgentStateChecksum,
    pub final_agent_checksum_report: AgentStateChecksumReport,
    pub unsupported_versions: Vec<String>,
    pub unsupported_epistemic_versions: Vec<String>,
    pub unsupported_agent_versions: Vec<Phase3AReplayFailure>,
    pub invariant_violations: Vec<String>,
    pub epistemic_application_errors: Vec<String>,
    pub agent_application_errors: Vec<Phase3AReplayFailure>,
    pub state_diff: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Phase3AReplayFailure {
    pub event_position: u64,
    pub event_kind: EventKind,
    pub schema_version: String,
    pub issue: String,
    pub actor_id: Option<String>,
    pub routine_id: Option<String>,
    pub trace_id: Option<String>,
}

pub fn rebuild_projection(
    initial_state: &PhysicalState,
    log: &EventLog,
    context: &ChecksumContext,
    expected_final_state: Option<&PhysicalState>,
) -> ProjectionRebuildReport {
    let mut rebuilt = initial_state.clone();
    let mut rebuilt_agent = AgentState::default();
    let mut event_count_applied = 0;
    let mut last_event_id = None;
    let mut last_stream_position = None;
    let mut unsupported_versions = Vec::new();
    let mut unsupported_epistemic_versions = Vec::new();
    let mut unsupported_agent_versions = Vec::new();
    let mut invariant_violations = Vec::new();
    let mut epistemic_application_errors = Vec::new();
    let mut agent_application_errors = Vec::new();
    let content_manifest_id = ContentManifestId::new(context.content_version.as_str()).unwrap();
    let mut epistemic_projection = EpistemicProjection::new(content_manifest_id.clone());

    for event in log.events() {
        if !event.has_supported_schema_version() {
            let message = format!(
                "event_position={} event_id={} version={}",
                event.global_order,
                event.event_id.as_str(),
                event.event_schema_version.as_str()
            );
            match event.stream {
                EventStream::Epistemic => unsupported_epistemic_versions.push(message),
                EventStream::Agent => unsupported_agent_versions
                    .push(phase3a_failure(event, "unsupported_schema_version")),
                EventStream::World => {
                    unsupported_versions.push(event.event_schema_version.as_str().to_string())
                }
                EventStream::Diagnostic | EventStream::Controller | EventStream::ReplayDebug => {}
            }
            continue;
        }

        match event.stream {
            EventStream::World => match apply_event(&mut rebuilt, event) {
                Ok(_) => {
                    event_count_applied += 1;
                    last_event_id = Some(event.event_id.clone());
                    last_stream_position = Some(event.stream_position);
                }
                Err(error) => {
                    invariant_violations.push(format!("{}: {:?}", event.event_id.as_str(), error))
                }
            },
            EventStream::Epistemic => match apply_epistemic_event(&mut epistemic_projection, event)
            {
                Ok(_) => {
                    epistemic_projection.event_range.event_count += 1;
                    if epistemic_projection.event_range.first_event_id.is_none() {
                        epistemic_projection.event_range.first_event_id =
                            Some(event.event_id.clone());
                    }
                    epistemic_projection.event_range.last_event_id = Some(event.event_id.clone());
                }
                Err(error) => {
                    epistemic_application_errors.push(format!(
                        "event_position={} event_id={}: {:?}",
                        event.global_order,
                        event.event_id.as_str(),
                        error
                    ));
                }
            },
            EventStream::Agent => match apply_agent_event(&mut rebuilt_agent, event) {
                Ok(_) => {
                    last_event_id = Some(event.event_id.clone());
                    last_stream_position = Some(event.stream_position);
                }
                Err(error) => {
                    let mut failure = phase3a_failure(event, "agent_application_error");
                    failure.issue = format!("agent_application_error:{error:?}");
                    agent_application_errors.push(failure);
                }
            },
            EventStream::Diagnostic | EventStream::Controller | EventStream::ReplayDebug => {}
        }
    }

    let final_checksum = compute_physical_checksum(&rebuilt, context).checksum;
    let final_epistemic_checksum = epistemic_projection.compute_checksum().checksum;
    let final_agent_checksum_report = compute_agent_state_checksum(&rebuilt_agent, context);
    let final_agent_checksum = final_agent_checksum_report.checksum.clone();
    let state_diff = expected_final_state
        .map(|expected| diff_physical_state(expected, &rebuilt))
        .unwrap_or_default();

    ProjectionRebuildReport {
        final_state: rebuilt,
        event_count_applied,
        last_event_id,
        last_stream_position,
        content_manifest_id,
        final_checksum,
        final_epistemic_projection: epistemic_projection,
        final_epistemic_checksum,
        final_agent_state: rebuilt_agent,
        final_agent_checksum,
        final_agent_checksum_report,
        unsupported_versions,
        unsupported_epistemic_versions,
        unsupported_agent_versions,
        invariant_violations,
        epistemic_application_errors,
        agent_application_errors,
        state_diff,
    }
}

fn phase3a_failure(event: &EventEnvelope, issue: impl Into<String>) -> Phase3AReplayFailure {
    let payload = event
        .payload
        .iter()
        .map(|field| (field.key.as_str(), field.value.as_str()))
        .collect::<std::collections::BTreeMap<_, _>>();
    Phase3AReplayFailure {
        event_position: event.global_order,
        event_kind: event.event_type,
        schema_version: event.event_schema_version.as_str().to_string(),
        issue: issue.into(),
        actor_id: event
            .actor_id
            .as_ref()
            .map(|id| id.as_str().to_string())
            .or_else(|| payload.get("actor_id").map(|value| (*value).to_string())),
        routine_id: payload
            .get("routine_execution_id")
            .or_else(|| payload.get("routine_id"))
            .map(|value| (*value).to_string()),
        trace_id: payload
            .get("trace_id")
            .or_else(|| payload.get("decision_trace_id"))
            .map(|value| (*value).to_string()),
    }
}

pub fn diff_physical_state(expected: &PhysicalState, actual: &PhysicalState) -> Vec<String> {
    let mut diffs = Vec::new();
    if expected.actors != actual.actors {
        diffs.push(format!(
            "actors expected={:?} actual={:?}",
            expected.actors, actual.actors
        ));
    }
    if expected.places != actual.places {
        diffs.push(format!(
            "places expected={:?} actual={:?}",
            expected.places, actual.places
        ));
    }
    if expected.doors != actual.doors {
        diffs.push(format!(
            "doors expected={:?} actual={:?}",
            expected.doors, actual.doors
        ));
    }
    if expected.containers != actual.containers {
        diffs.push(format!(
            "containers expected={:?} actual={:?}",
            expected.containers, actual.containers
        ));
    }
    if expected.items != actual.items {
        diffs.push(format!(
            "items expected={:?} actual={:?}",
            expected.items, actual.items
        ));
    }
    if expected.food_supplies != actual.food_supplies {
        diffs.push(format!(
            "food_supplies expected={:?} actual={:?}",
            expected.food_supplies, actual.food_supplies
        ));
    }
    if expected.workplaces != actual.workplaces {
        diffs.push(format!(
            "workplaces expected={:?} actual={:?}",
            expected.workplaces, actual.workplaces
        ));
    }
    diffs.sort();
    diffs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actions::pipeline::{run_pipeline, PipelineContext};
    use crate::actions::proposal::{Proposal, ProposalOrigin};
    use crate::actions::registry::ActionRegistry;
    use crate::events::log::EventLog;
    use crate::events::{EventCause, EventEnvelope, EventKind, PayloadField, EVENT_SCHEMA_V1};
    use crate::ids::{
        ActionId, ActorId, ContentManifestId, ContentVersion, EventId, FixtureId, PlaceId,
        ProcessId, ProposalId, SchemaVersion,
    };
    use crate::scheduler::{OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId};
    use crate::state::{ActorBody, DoorState, PhysicalState, PlaceState};
    use crate::time::SimTick;

    fn actor_id() -> ActorId {
        ActorId::new("actor_tomas").unwrap()
    }

    fn initial_state() -> PhysicalState {
        let mut state = PhysicalState::default();
        let shop = PlaceId::new("shop_front").unwrap();
        let back = PlaceId::new("back_room").unwrap();
        let mut shop_state = PlaceState::new(shop.clone(), "Shop front");
        shop_state.adjacent_place_ids.insert(back.clone());
        state.places.insert(shop.clone(), shop_state);
        state
            .places
            .insert(back.clone(), PlaceState::new(back.clone(), "Back room"));
        state
            .actors
            .insert(actor_id(), ActorBody::new(actor_id(), shop.clone()));
        let door_id = crate::ids::DoorId::new("door_shop_back").unwrap();
        state
            .doors
            .insert(door_id.clone(), DoorState::new(door_id, shop, back));
        state
    }

    fn ordering_key(action: &str) -> OrderingKey {
        OrderingKey::new(
            SimTick::ZERO,
            SchedulePhase::HumanCommand,
            SchedulerSourceId::Actor(actor_id()),
            ProposalSequence::new(0),
            ActionId::new(action).unwrap(),
            Vec::new(),
            "tie",
        )
    }

    fn epistemic_event(id: &str, kind: EventKind, payload: Vec<PayloadField>) -> EventEnvelope {
        let mut event = EventEnvelope::new_v1(
            EventId::new(id).unwrap(),
            kind,
            99,
            99,
            SimTick::new(3),
            ordering_key("check_container"),
            ContentManifestId::new("phase2a_manifest").unwrap(),
        );
        event.payload = payload;
        event
    }

    fn agent_event(id: &str, kind: EventKind, payload: Vec<PayloadField>) -> EventEnvelope {
        let mut event = EventEnvelope::new_caused_v1(
            EventId::new(id).unwrap(),
            kind,
            99,
            99,
            SimTick::new(3),
            ordering_key("continue_routine"),
            ContentManifestId::new("phase3a_manifest").unwrap(),
            vec![EventCause::Process(
                ProcessId::new("process_agent").unwrap(),
            )],
        )
        .unwrap();
        event.actor_id = Some(actor_id());
        event.payload = payload;
        event
    }

    fn hunger_delta_payload(delta: i32, cause_kind: &str) -> Vec<PayloadField> {
        vec![
            PayloadField::new("actor_id", "actor_tomas"),
            PayloadField::new("need_kind", "hunger"),
            PayloadField::new("delta", delta.to_string()),
            PayloadField::new("cause_kind", cause_kind),
        ]
    }

    fn missing_coin_proposition() -> String {
        crate::epistemics::Proposition::ItemMissingFromExpectedLocation {
            item_id: crate::ids::ItemId::new("coin_stack_01").unwrap(),
            expected_location: crate::location::Location::InContainer(
                crate::ids::ContainerId::new("strongbox_tomas").unwrap(),
            ),
        }
        .serialize_canonical()
    }

    fn belief_payload() -> Vec<PayloadField> {
        vec![
            PayloadField::new("schema_version", EVENT_SCHEMA_V1),
            PayloadField::new("belief_id", "belief_tomas_missing_coin"),
            PayloadField::new("holder_actor_id", "actor_tomas"),
            PayloadField::new("proposition", missing_coin_proposition()),
            PayloadField::new("stance", "believes_true"),
            PayloadField::new("confidence", "900"),
            PayloadField::new("source_event_id", "event_obs_absence"),
            PayloadField::new("acquired_tick", "3"),
            PayloadField::new("channel", "absence_marker"),
        ]
    }

    fn context() -> ChecksumContext {
        ChecksumContext {
            fixture_id: FixtureId::new("door_access_001").unwrap(),
            content_version: ContentVersion::new("phase1_manifest").unwrap(),
            sim_tick: SimTick::ZERO,
            world_stream_position_applied: 2,
        }
    }

    fn live_run() -> (PhysicalState, EventLog, PhysicalState) {
        let initial = initial_state();
        let mut live = initial.clone();
        let mut log = EventLog::new();
        let mut registry = ActionRegistry::new();
        registry.register_phase1_movement_open_close();

        let mut open = Proposal::new(
            ProposalId::new("proposal_open").unwrap(),
            ProposalOrigin::Test,
            Some(actor_id()),
            ActionId::new("open").unwrap(),
            SimTick::ZERO,
        );
        open.target_ids.push("door_shop_back".to_string());
        let mut open_context = PipelineContext {
            registry: &registry,
            state: &mut live,
            log: &mut log,
            controller_bindings: None,
            epistemic_projection: None,
            content_manifest_id: ContentManifestId::new("phase1_manifest").unwrap(),
            ordering_key: ordering_key("open"),
        };
        run_pipeline(&mut open_context, &open);

        let mut move_proposal = Proposal::new(
            ProposalId::new("proposal_move").unwrap(),
            ProposalOrigin::Test,
            Some(actor_id()),
            ActionId::new("move").unwrap(),
            SimTick::ZERO,
        );
        move_proposal.target_ids.push("back_room".to_string());
        let mut move_context = PipelineContext {
            registry: &registry,
            state: &mut live,
            log: &mut log,
            controller_bindings: None,
            epistemic_projection: None,
            content_manifest_id: ContentManifestId::new("phase1_manifest").unwrap(),
            ordering_key: ordering_key("move"),
        };
        run_pipeline(&mut move_context, &move_proposal);

        (initial, log, live)
    }

    #[test]
    fn rebuild_from_fixture_and_events_equals_live_run() {
        let (initial, log, live) = live_run();
        let report = rebuild_projection(&initial, &log, &context(), Some(&live));

        assert_eq!(report.final_state, live);
        assert_eq!(report.event_count_applied, 2);
        assert!(report.state_diff.is_empty());
    }

    #[test]
    fn epistemic_rebuild_is_deterministic_across_repeated_runs() {
        let initial = initial_state();
        let log = EventLog::from_ordered_events_for_replay_tests(vec![epistemic_event(
            "event_belief_updated",
            EventKind::BeliefUpdated,
            belief_payload(),
        )]);

        let first = rebuild_projection(&initial, &log, &context(), None);
        let second = rebuild_projection(&initial, &log, &context(), None);

        assert_eq!(
            first.final_epistemic_checksum,
            second.final_epistemic_checksum
        );
        assert_eq!(
            first.final_epistemic_projection.beliefs_by_id,
            second.final_epistemic_projection.beliefs_by_id
        );
        assert!(first.epistemic_application_errors.is_empty());
    }

    #[test]
    fn unsupported_epistemic_event_schema_reports_position_and_version() {
        let initial = initial_state();
        let mut event = epistemic_event(
            "event_bad_epistemic_version",
            EventKind::BeliefUpdated,
            belief_payload(),
        );
        event.event_schema_version = SchemaVersion::new("event_schema_v999").unwrap();
        let log = EventLog::from_ordered_events_for_replay_tests(vec![event]);

        let report = rebuild_projection(&initial, &log, &context(), None);

        assert!(report.final_epistemic_projection.beliefs_by_id.is_empty());
        assert_eq!(
            report.unsupported_epistemic_versions,
            ["event_position=99 event_id=event_bad_epistemic_version version=event_schema_v999"]
        );
    }

    #[test]
    fn agent_state_rebuild_is_deterministic_and_catches_need_delta_divergence() {
        let initial = initial_state();
        let log = EventLog::from_ordered_events_for_replay_tests(vec![
            agent_event(
                "event_hunger_initial",
                EventKind::NeedDeltaApplied,
                hunger_delta_payload(490, "fixture_initial"),
            ),
            agent_event(
                "event_hunger_tick",
                EventKind::NeedDeltaApplied,
                hunger_delta_payload(40, "tick_delta"),
            ),
        ]);
        let changed_log = EventLog::from_ordered_events_for_replay_tests(vec![
            agent_event(
                "event_hunger_initial",
                EventKind::NeedDeltaApplied,
                hunger_delta_payload(490, "fixture_initial"),
            ),
            agent_event(
                "event_hunger_tick",
                EventKind::NeedDeltaApplied,
                hunger_delta_payload(41, "tick_delta"),
            ),
        ]);

        let first = rebuild_projection(&initial, &log, &context(), None);
        let second = rebuild_projection(&initial, &log, &context(), None);
        let changed = rebuild_projection(&initial, &changed_log, &context(), None);

        assert_eq!(first.final_agent_checksum, second.final_agent_checksum);
        assert_eq!(
            first.final_agent_checksum_report.canonical_input,
            second.final_agent_checksum_report.canonical_input
        );
        assert_ne!(first.final_agent_checksum, changed.final_agent_checksum);
        assert!(first.agent_application_errors.is_empty());
    }

    #[test]
    fn unsupported_agent_event_schema_reports_position_kind_and_ids() {
        let initial = initial_state();
        let mut event = agent_event(
            "event_bad_agent_version",
            EventKind::RoutineStepStarted,
            vec![
                PayloadField::new("actor_id", "actor_tomas"),
                PayloadField::new("routine_execution_id", "routine_exec_breakfast"),
                PayloadField::new("trace_id", "trace_breakfast"),
                PayloadField::new("progress_tick", "3"),
                PayloadField::new("action_id", "check_container.pantry"),
            ],
        );
        event.event_schema_version = SchemaVersion::new("event_schema_v999").unwrap();
        let log = EventLog::from_ordered_events_for_replay_tests(vec![event]);

        let report = rebuild_projection(&initial, &log, &context(), None);

        assert!(report.final_agent_state.routine_executions.is_empty());
        assert_eq!(report.unsupported_agent_versions.len(), 1);
        let failure = &report.unsupported_agent_versions[0];
        assert_eq!(failure.event_position, 99);
        assert_eq!(failure.event_kind, EventKind::RoutineStepStarted);
        assert_eq!(failure.schema_version, "event_schema_v999");
        assert_eq!(failure.actor_id.as_deref(), Some("actor_tomas"));
        assert_eq!(
            failure.routine_id.as_deref(),
            Some("routine_exec_breakfast")
        );
        assert_eq!(failure.trace_id.as_deref(), Some("trace_breakfast"));
    }

    #[test]
    fn windowed_need_delta_batching_rebuilds_same_final_agent_state() {
        let initial = initial_state();
        let unbatched = EventLog::from_ordered_events_for_replay_tests(vec![
            agent_event(
                "event_hunger_initial",
                EventKind::NeedDeltaApplied,
                hunger_delta_payload(490, "fixture_initial"),
            ),
            agent_event(
                "event_hunger_tick_a",
                EventKind::NeedDeltaApplied,
                hunger_delta_payload(10, "tick_delta"),
            ),
            agent_event(
                "event_hunger_tick_b",
                EventKind::NeedDeltaApplied,
                hunger_delta_payload(20, "tick_delta"),
            ),
        ]);
        let batched = EventLog::from_ordered_events_for_replay_tests(vec![
            agent_event(
                "event_hunger_initial",
                EventKind::NeedDeltaApplied,
                hunger_delta_payload(490, "fixture_initial"),
            ),
            agent_event(
                "event_hunger_tick_window",
                EventKind::NeedDeltaApplied,
                hunger_delta_payload(30, "tick_delta"),
            ),
        ]);

        let unbatched_report = rebuild_projection(&initial, &unbatched, &context(), None);
        let batched_report = rebuild_projection(&initial, &batched, &context(), None);

        assert_eq!(
            unbatched_report.final_agent_checksum,
            batched_report.final_agent_checksum
        );
        assert_eq!(
            unbatched_report.final_agent_checksum_report.canonical_input,
            batched_report.final_agent_checksum_report.canonical_input
        );
    }
}
