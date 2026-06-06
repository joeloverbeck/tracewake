use crate::checksum::{compute_physical_checksum, ChecksumContext, PhysicalChecksum};
use crate::events::apply::apply_event;
use crate::events::log::EventLog;
use crate::events::EventStream;
use crate::ids::{ContentManifestId, EventId};
use crate::state::PhysicalState;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProjectionRebuildReport {
    pub final_state: PhysicalState,
    pub event_count_applied: usize,
    pub last_event_id: Option<EventId>,
    pub last_stream_position: Option<u64>,
    pub content_manifest_id: ContentManifestId,
    pub final_checksum: PhysicalChecksum,
    pub unsupported_versions: Vec<String>,
    pub invariant_violations: Vec<String>,
    pub state_diff: Vec<String>,
}

pub fn rebuild_projection(
    initial_state: &PhysicalState,
    log: &EventLog,
    context: &ChecksumContext,
    expected_final_state: Option<&PhysicalState>,
) -> ProjectionRebuildReport {
    let mut rebuilt = initial_state.clone();
    let mut event_count_applied = 0;
    let mut last_event_id = None;
    let mut last_stream_position = None;
    let mut unsupported_versions = Vec::new();
    let mut invariant_violations = Vec::new();

    for event in log
        .events()
        .iter()
        .filter(|event| event.stream == EventStream::World)
    {
        if !event.has_supported_schema_version() {
            unsupported_versions.push(event.event_schema_version.as_str().to_string());
            continue;
        }

        match apply_event(&mut rebuilt, event) {
            Ok(_) => {
                event_count_applied += 1;
                last_event_id = Some(event.event_id.clone());
                last_stream_position = Some(event.stream_position);
            }
            Err(error) => {
                invariant_violations.push(format!("{}: {:?}", event.event_id.as_str(), error))
            }
        }
    }

    let final_checksum = compute_physical_checksum(&rebuilt, context).checksum;
    let state_diff = expected_final_state
        .map(|expected| diff_physical_state(expected, &rebuilt))
        .unwrap_or_default();

    ProjectionRebuildReport {
        final_state: rebuilt,
        event_count_applied,
        last_event_id,
        last_stream_position,
        content_manifest_id: ContentManifestId::new(context.content_version.as_str()).unwrap(),
        final_checksum,
        unsupported_versions,
        invariant_violations,
        state_diff,
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
    use crate::ids::{
        ActionId, ActorId, ContentManifestId, ContentVersion, FixtureId, PlaceId, ProposalId,
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
}
