use crate::agent::{
    ActorKnownInputRef, DecisionTraceRecord, NoHumanActorKnownSurfaceBuilder,
    NoHumanActorKnownSurfaceRequest,
};
use crate::checksum::{
    compute_agent_state_checksum, compute_holder_known_context_hash, compute_physical_checksum,
    AgentStateChecksum, AgentStateChecksumReport, ChecksumContext, PhysicalChecksum,
};
use crate::epistemics::{EpistemicProjection, EpistemicProjectionChecksum};
use crate::events::apply::{apply_event_stream, EventApplicationContext, EventApplicationError};
use crate::events::log::EventLog;
use crate::events::{EventEnvelope, EventKind, EventStream};
use crate::ids::{ActorId, ContentManifestId, EventId, PlaceId};
use crate::state::{AgentState, PhysicalState};
use crate::time::SimTick;

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
    pub decision_context_hash_failures: Vec<Phase3AReplayFailure>,
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
    initial_agent_state: &AgentState,
    log: &EventLog,
    context: &ChecksumContext,
    expected_final_state: Option<&PhysicalState>,
) -> ProjectionRebuildReport {
    let mut rebuilt = initial_state.clone();
    let mut rebuilt_agent = initial_agent_state.clone();
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

    for issue in verify_event_ordering(log) {
        invariant_violations.push(issue);
    }
    for issue in crate::need_accounting::duplicate_duration_terminal_violations(log) {
        invariant_violations.push(issue);
    }

    for event in log.events() {
        if !invariant_violations.is_empty() {
            break;
        }
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

        let mut application_context = EventApplicationContext {
            physical_state: &mut rebuilt,
            agent_state: &mut rebuilt_agent,
            epistemic_projection: Some(&mut epistemic_projection),
        };
        match apply_event_stream(&mut application_context, event) {
            Ok(_) => match event.stream {
                EventStream::World => {
                    event_count_applied += 1;
                    last_event_id = Some(event.event_id.clone());
                    last_stream_position = Some(event.stream_position);
                }
                EventStream::Epistemic => {
                    epistemic_projection.record_applied_event(event.event_id.clone());
                }
                EventStream::Agent => {
                    last_event_id = Some(event.event_id.clone());
                    last_stream_position = Some(event.stream_position);
                }
                EventStream::Diagnostic | EventStream::Controller | EventStream::ReplayDebug => {}
            },
            Err(EventApplicationError::World(error)) => {
                invariant_violations.push(format!("{}: {:?}", event.event_id.as_str(), error));
                break;
            }
            Err(EventApplicationError::Epistemic(error)) => {
                epistemic_application_errors.push(format!(
                    "event_position={} event_id={}: {:?}",
                    event.global_order,
                    event.event_id.as_str(),
                    error
                ));
                break;
            }
            Err(EventApplicationError::Agent(error)) => {
                let mut failure = phase3a_failure(event, "agent_application_error");
                failure.issue = format!("agent_application_error:{error:?}");
                agent_application_errors.push(failure);
                break;
            }
        }
    }

    let final_checksum = compute_physical_checksum(&rebuilt, context).checksum;
    let final_epistemic_checksum = epistemic_projection.compute_checksum().checksum;
    let final_agent_checksum_report = compute_agent_state_checksum(&rebuilt_agent, context);
    let final_agent_checksum = final_agent_checksum_report.checksum.clone();
    let decision_context_hash_failures =
        rebuild_decision_context_hashes(initial_state, initial_agent_state, log);
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
        decision_context_hash_failures,
        state_diff,
    }
}

fn verify_event_ordering(log: &EventLog) -> Vec<String> {
    let mut issues = Vec::new();
    let mut stream_positions = std::collections::BTreeMap::<EventStream, u64>::new();
    for (index, event) in log.events().iter().enumerate() {
        let expected_global = index as u64;
        if event.global_order != expected_global {
            issues.push(format!(
                "event_order_mismatch:event_id={} expected_global_order={} actual_global_order={}",
                event.event_id.as_str(),
                expected_global,
                event.global_order
            ));
            break;
        }
        let expected_stream = stream_positions.entry(event.stream).or_insert(0);
        if event.stream_position != *expected_stream {
            issues.push(format!(
                "event_stream_position_mismatch:event_id={} expected_stream_position={} actual_stream_position={}",
                event.event_id.as_str(),
                *expected_stream,
                event.stream_position
            ));
            break;
        }
        *expected_stream += 1;
    }
    issues
}

pub fn rebuild_decision_context_hashes(
    initial_state: &PhysicalState,
    initial_agent_state: &AgentState,
    log: &EventLog,
) -> Vec<Phase3AReplayFailure> {
    log.events()
        .iter()
        .filter(|event| event.event_type == EventKind::DecisionTraceRecorded)
        .filter_map(|event| {
            rebuild_decision_context_hash(initial_state, initial_agent_state, log, event)
                .err()
                .map(|failure| *failure)
        })
        .collect()
}

fn rebuild_decision_context_hash(
    initial_state: &PhysicalState,
    initial_agent_state: &AgentState,
    log: &EventLog,
    trace_event: &EventEnvelope,
) -> Result<(), Box<Phase3AReplayFailure>> {
    let record = decision_trace_record(trace_event)
        .map_err(|issue| Box::new(phase3a_failure(trace_event, issue)))?;
    let window_id = payload_value(trace_event, "window_id")
        .ok_or_else(|| Box::new(phase3a_failure(trace_event, "missing_window_id")))?;
    let window_end_tick = payload_value(trace_event, "window_end_tick")
        .and_then(|value| value.parse::<u64>().ok())
        .map(SimTick::new)
        .ok_or_else(|| Box::new(phase3a_failure(trace_event, "missing_window_end_tick")))?;
    let ordinary_event_id = payload_value(trace_event, "ordinary_event_id")
        .and_then(|value| EventId::new(value).ok())
        .or_else(|| {
            trace_event.causes.iter().find_map(|cause| match cause {
                crate::events::EventCause::Event(event_id) => Some(event_id.clone()),
                _ => None,
            })
        })
        .ok_or_else(|| Box::new(phase3a_failure(trace_event, "missing_ordinary_event_id")))?;
    let ordinary_index = log
        .events()
        .iter()
        .position(|event| event.event_id == ordinary_event_id)
        .ok_or_else(|| Box::new(phase3a_failure(trace_event, "ordinary_event_not_found")))?;
    let trace_index = log
        .events()
        .iter()
        .position(|event| event.event_id == trace_event.event_id)
        .ok_or_else(|| {
            Box::new(phase3a_failure(
                trace_event,
                "decision_trace_event_not_found",
            ))
        })?;
    if ordinary_index >= trace_index {
        return Err(Box::new(phase3a_failure(
            trace_event,
            "ordinary_event_after_trace",
        )));
    }
    let (prefix_log, prefix_state, prefix_agent_state, prefix_epistemic_projection) =
        replay_prefix(
            initial_state,
            initial_agent_state,
            &log.events()[..ordinary_index],
        )
        .map_err(|issue| Box::new(phase3a_failure(trace_event, issue)))?;
    let actor = prefix_state.actors.get(&record.actor_id).ok_or_else(|| {
        Box::new(phase3a_failure(
            trace_event,
            "actor_missing_at_decision_frontier",
        ))
    })?;
    let surface =
        NoHumanActorKnownSurfaceBuilder::from_projection(NoHumanActorKnownSurfaceRequest {
            projection: &prefix_epistemic_projection,
            agent_state: &prefix_agent_state,
            actor_id: record.actor_id.clone(),
            current_place_id: actor.current_place_id.clone(),
            decision_tick: record.window_start_tick,
            window_id,
            window_end_tick,
            current_place_witness_event_id: latest_current_place_perception_event_id(
                &prefix_log,
                &record.actor_id,
                record.window_start_tick,
                &actor.current_place_id,
            ),
            needs_witness_event_id: latest_need_event_id(&prefix_log, &record.actor_id),
            frame_event_id: latest_frame_event_id(&prefix_log),
        })
        .build(&prefix_agent_state);
    let rebuilt_inputs = surface
        .context()
        .actor_known_facts()
        .iter()
        .map(ActorKnownInputRef::from_fact)
        .map(|input| input.render_for_trace())
        .collect::<Vec<_>>();
    let rebuilt_hash = compute_holder_known_context_hash(rebuilt_inputs.clone()).hash;
    let Some(recorded_hash) = record.actor_known_context_hash.as_ref() else {
        return Ok(());
    };
    if rebuilt_hash != *recorded_hash {
        let recorded_inputs = record
            .actor_known_inputs
            .iter()
            .collect::<std::collections::BTreeSet<_>>();
        let rebuilt_input_set = rebuilt_inputs
            .iter()
            .collect::<std::collections::BTreeSet<_>>();
        let missing_rebuilt = recorded_inputs
            .difference(&rebuilt_input_set)
            .next()
            .map(|value| value.as_str())
            .unwrap_or("-");
        let extra_rebuilt = rebuilt_input_set
            .difference(&recorded_inputs)
            .next()
            .map(|value| value.as_str())
            .unwrap_or("-");
        return Err(Box::new(phase3a_failure(
            trace_event,
            format!(
                "decision_context_hash_mismatch:recorded={} rebuilt={} recorded_count={} rebuilt_count={} missing_rebuilt={} extra_rebuilt={}",
                recorded_hash.as_str(),
                rebuilt_hash.as_str(),
                record.actor_known_inputs.len(),
                rebuilt_inputs.len(),
                missing_rebuilt,
                extra_rebuilt
            ),
        )));
    }
    Ok(())
}

fn decision_trace_record(event: &EventEnvelope) -> Result<DecisionTraceRecord, &'static str> {
    let canonical = payload_value(event, "trace_canonical").ok_or("missing_trace_canonical")?;
    DecisionTraceRecord::deserialize_canonical(canonical.as_bytes())
        .map_err(|_| "invalid_trace_canonical")
}

fn replay_prefix(
    initial_state: &PhysicalState,
    initial_agent_state: &AgentState,
    events: &[EventEnvelope],
) -> Result<(EventLog, PhysicalState, AgentState, EpistemicProjection), String> {
    let mut prefix_log = EventLog::new();
    let mut state = initial_state.clone();
    let mut agent_state = initial_agent_state.clone();
    let mut epistemic_projection = EpistemicProjection::new(
        events
            .first()
            .map(|event| event.content_manifest_id.clone())
            .unwrap_or_else(|| ContentManifestId::new("empty_prefix").unwrap()),
    );
    for event in events {
        let appended = prefix_log
            .append(event.clone())
            .map_err(|error| format!("prefix_append_error:{error:?}"))?;
        let mut application_context = EventApplicationContext {
            physical_state: &mut state,
            agent_state: &mut agent_state,
            epistemic_projection: Some(&mut epistemic_projection),
        };
        apply_event_stream(&mut application_context, &appended)
            .map_err(|error| format!("prefix_apply_error:{error:?}"))?;
    }
    Ok((prefix_log, state, agent_state, epistemic_projection))
}

fn latest_frame_event_id(log: &EventLog) -> Option<EventId> {
    log.events()
        .iter()
        .rev()
        .find(|event| {
            matches!(
                event.event_type,
                EventKind::NoHumanDayStarted | EventKind::NoHumanAdvanceStarted
            )
        })
        .map(|event| event.event_id.clone())
}

fn latest_current_place_perception_event_id(
    log: &EventLog,
    actor_id: &ActorId,
    tick: SimTick,
    place_id: &PlaceId,
) -> Option<EventId> {
    log.events()
        .iter()
        .rev()
        .find(|event| {
            event.event_type == EventKind::ObservationRecorded
                && event.actor_id.as_ref() == Some(actor_id)
                && event.sim_tick == tick
                && event.place_id.as_ref() == Some(place_id)
        })
        .map(|event| event.event_id.clone())
}

fn latest_need_event_id(log: &EventLog, actor_id: &ActorId) -> Option<EventId> {
    log.events()
        .iter()
        .rev()
        .find(|event| {
            event.event_type == EventKind::NeedDeltaApplied
                && (event.actor_id.as_ref() == Some(actor_id)
                    || payload_value(event, "actor_id") == Some(actor_id.as_str()))
        })
        .map(|event| event.event_id.clone())
}

fn payload_value<'a>(event: &'a EventEnvelope, key: &str) -> Option<&'a str> {
    event
        .payload
        .iter()
        .find(|field| field.key == key)
        .map(|field| field.value.as_str())
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
    if expected.sleep_affordances != actual.sleep_affordances {
        diffs.push(format!(
            "sleep_affordances expected={:?} actual={:?}",
            expected.sleep_affordances, actual.sleep_affordances
        ));
    }
    if expected.need_model != actual.need_model {
        diffs.push(format!(
            "need_model expected={:?} actual={:?}",
            expected.need_model, actual.need_model
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
        let mut state = PhysicalState::empty(crate::state::NeedModelState::new(5, 3));
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
            0,
            0,
            SimTick::new(3),
            ordering_key("check_container"),
            ContentManifestId::new("phase2a_manifest").unwrap(),
        );
        event.payload = payload;
        if kind.requires_cause() {
            event.causes = vec![EventCause::Process(
                ProcessId::new("process_epistemic").unwrap(),
            )];
        }
        event
    }

    fn agent_event(id: &str, kind: EventKind, payload: Vec<PayloadField>) -> EventEnvelope {
        let mut event = EventEnvelope::new_caused_v1(
            EventId::new(id).unwrap(),
            kind,
            0,
            0,
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

    fn ordered_test_log(mut events: Vec<EventEnvelope>) -> EventLog {
        let mut stream_positions = std::collections::BTreeMap::<EventStream, u64>::new();
        for (index, event) in events.iter_mut().enumerate() {
            event.global_order = index as u64;
            let stream_position = stream_positions.entry(event.stream).or_insert(0);
            event.stream_position = *stream_position;
            *stream_position += 1;
        }
        EventLog::from_ordered_events_for_replay_tests(events)
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
            agent_state: Box::leak(Box::new(crate::state::AgentState::default())),
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
            agent_state: Box::leak(Box::new(crate::state::AgentState::default())),
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
        let report = rebuild_projection(
            &initial,
            &crate::state::AgentState::default(),
            &log,
            &context(),
            Some(&live),
        );

        assert_eq!(report.final_state, live);
        assert_eq!(report.event_count_applied, 2);
        assert!(report.state_diff.is_empty());
    }

    #[test]
    fn reordered_in_memory_log_is_detected_before_rebuild() {
        let (initial, log, _) = live_run();
        let mut events = log.events().to_vec();
        events.swap(0, 1);
        let reordered = EventLog::from_ordered_events_for_replay_tests(events);

        let report = rebuild_projection(
            &initial,
            &crate::state::AgentState::default(),
            &reordered,
            &context(),
            None,
        );

        assert_eq!(report.event_count_applied, 0);
        assert!(report
            .invariant_violations
            .iter()
            .any(|issue| issue.contains("event_order_mismatch")));
    }

    #[test]
    fn corrupt_midstream_agent_event_poisons_rebuild() {
        let initial = initial_state();
        let mut log = EventLog::new();
        let mut corrupt = agent_event(
            "event.threshold.corrupt",
            EventKind::NeedThresholdCrossed,
            vec![
                PayloadField::new("need_kind", "hunger"),
                PayloadField::new("from_value", "249"),
                PayloadField::new("to_value", "254"),
                PayloadField::new("from_band", "comfortable"),
                PayloadField::new("to_band", "rising"),
            ],
        );
        corrupt.actor_id = Some(actor_id());
        log.append(corrupt).unwrap();
        let valid = agent_event(
            "event.threshold.valid",
            EventKind::NeedThresholdCrossed,
            vec![
                PayloadField::new("actor_id", "actor_tomas"),
                PayloadField::new("need_kind", "hunger"),
                PayloadField::new("from_value", "254"),
                PayloadField::new("to_value", "500"),
                PayloadField::new("from_band", "rising"),
                PayloadField::new("to_band", "urgent"),
            ],
        );
        log.append(valid).unwrap();

        let report = rebuild_projection(
            &initial,
            &crate::state::AgentState::default(),
            &log,
            &context(),
            None,
        );

        assert_eq!(report.agent_application_errors.len(), 1);
        assert_eq!(report.final_agent_state.need_threshold_crossings().len(), 0);
    }

    #[test]
    fn epistemic_rebuild_is_deterministic_across_repeated_runs() {
        let initial = initial_state();
        let log = ordered_test_log(vec![epistemic_event(
            "event_belief_updated",
            EventKind::BeliefUpdated,
            belief_payload(),
        )]);

        let first = rebuild_projection(
            &initial,
            &crate::state::AgentState::default(),
            &log,
            &context(),
            None,
        );
        let second = rebuild_projection(
            &initial,
            &crate::state::AgentState::default(),
            &log,
            &context(),
            None,
        );

        assert_eq!(
            first.final_epistemic_checksum,
            second.final_epistemic_checksum
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
        let log = ordered_test_log(vec![event]);

        let report = rebuild_projection(
            &initial,
            &crate::state::AgentState::default(),
            &log,
            &context(),
            None,
        );

        assert!(report.final_epistemic_projection.is_empty());
        assert_eq!(
            report.unsupported_epistemic_versions,
            ["event_position=0 event_id=event_bad_epistemic_version version=event_schema_v999"]
        );
    }

    #[test]
    fn unsupported_event_schema_version_replay_fails_loudly() {
        let initial = initial_state();
        let mut event = EventEnvelope::new_v1(
            EventId::new("event_unknown_world_version").unwrap(),
            EventKind::ActorMoved,
            0,
            0,
            SimTick::new(3),
            ordering_key("move"),
            ContentManifestId::new("phase1_manifest").unwrap(),
        );
        event.payload = vec![
            PayloadField::new("actor_id", "actor_tomas"),
            PayloadField::new("from_place_id", "shop_front"),
            PayloadField::new("to_place_id", "back_room"),
        ];
        event.event_schema_version = SchemaVersion::new("event_schema_v999").unwrap();

        let mut live_state = initial.clone();
        let live_result = crate::events::apply::apply_event(&mut live_state, &event);
        assert_eq!(
            live_result,
            Err(crate::events::apply::ApplyError::UnsupportedSchemaVersion(
                "event_schema_v999".to_string()
            ))
        );
        assert_eq!(live_state, initial);

        let log = ordered_test_log(vec![event]);
        let report = rebuild_projection(
            &initial,
            &crate::state::AgentState::default(),
            &log,
            &context(),
            None,
        );

        assert_eq!(report.event_count_applied, 0);
        assert_eq!(report.final_state, initial);
        assert_eq!(report.unsupported_versions, ["event_schema_v999"]);
        assert!(report.invariant_violations.is_empty());
    }

    #[test]
    fn agent_state_rebuild_is_deterministic_and_catches_need_delta_divergence() {
        let initial = initial_state();
        let log = ordered_test_log(vec![
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
        let changed_log = ordered_test_log(vec![
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

        let first = rebuild_projection(
            &initial,
            &crate::state::AgentState::default(),
            &log,
            &context(),
            None,
        );
        let second = rebuild_projection(
            &initial,
            &crate::state::AgentState::default(),
            &log,
            &context(),
            None,
        );
        let changed = rebuild_projection(
            &initial,
            &crate::state::AgentState::default(),
            &changed_log,
            &context(),
            None,
        );

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
        let log = ordered_test_log(vec![event]);

        let report = rebuild_projection(
            &initial,
            &crate::state::AgentState::default(),
            &log,
            &context(),
            None,
        );

        assert!(report.final_agent_state.routine_executions.is_empty());
        assert_eq!(report.unsupported_agent_versions.len(), 1);
        let failure = &report.unsupported_agent_versions[0];
        assert_eq!(failure.event_position, 0);
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
        let unbatched = ordered_test_log(vec![
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
        let batched = ordered_test_log(vec![
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

        let unbatched_report = rebuild_projection(
            &initial,
            &crate::state::AgentState::default(),
            &unbatched,
            &context(),
            None,
        );
        let batched_report = rebuild_projection(
            &initial,
            &crate::state::AgentState::default(),
            &batched,
            &context(),
            None,
        );

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
