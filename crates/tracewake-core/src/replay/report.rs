use crate::checksum::{compute_physical_checksum, ChecksumContext, PhysicalChecksum};
use crate::epistemics::EpistemicProjectionChecksum;
use crate::events::log::EventLog;
use crate::events::EventStream;
use crate::ids::{ContentManifestId, FixtureId};
use crate::replay::rebuild::{diff_physical_state, rebuild_projection};
use crate::state::PhysicalState;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplayReport {
    pub fixture_id: FixtureId,
    pub content_manifest_id: ContentManifestId,
    pub initial_checksum: PhysicalChecksum,
    pub event_count: usize,
    pub diagnostic_event_count: usize,
    pub unsupported_versions: Vec<String>,
    pub unsupported_epistemic_versions: Vec<String>,
    pub application_errors: Vec<String>,
    pub epistemic_application_errors: Vec<String>,
    pub final_checksum: PhysicalChecksum,
    pub final_epistemic_checksum: EpistemicProjectionChecksum,
    pub epistemic_projection_version: String,
    pub expected_checksum: Option<PhysicalChecksum>,
    pub matches_expected: bool,
    pub state_diff: Vec<String>,
}

pub fn run_replay(
    initial_state: &PhysicalState,
    log: &EventLog,
    context: &ChecksumContext,
    expected_final_state: Option<&PhysicalState>,
    expected_checksum: Option<PhysicalChecksum>,
) -> ReplayReport {
    let initial_checksum = compute_physical_checksum(initial_state, context).checksum;
    let rebuild = rebuild_projection(initial_state, log, context, expected_final_state);
    let diagnostic_event_count = log
        .events()
        .iter()
        .filter(|event| event.stream != EventStream::World)
        .count();
    let expected_state_diff = expected_final_state
        .map(|expected| diff_physical_state(expected, &rebuild.final_state))
        .unwrap_or_default();
    let state_diff = if expected_state_diff.is_empty() {
        rebuild.state_diff.clone()
    } else {
        expected_state_diff
    };
    let checksum_matches = expected_checksum
        .as_ref()
        .map(|expected| expected == &rebuild.final_checksum)
        .unwrap_or(true);
    let matches_expected = checksum_matches
        && state_diff.is_empty()
        && rebuild.unsupported_versions.is_empty()
        && rebuild.unsupported_epistemic_versions.is_empty()
        && rebuild.invariant_violations.is_empty()
        && rebuild.epistemic_application_errors.is_empty();
    let epistemic_projection_version = rebuild
        .final_epistemic_projection
        .projection_version
        .as_str()
        .to_string();

    ReplayReport {
        fixture_id: context.fixture_id.clone(),
        content_manifest_id: ContentManifestId::new(context.content_version.as_str()).unwrap(),
        initial_checksum,
        event_count: rebuild.event_count_applied,
        diagnostic_event_count,
        unsupported_versions: rebuild.unsupported_versions,
        unsupported_epistemic_versions: rebuild.unsupported_epistemic_versions,
        application_errors: rebuild.invariant_violations,
        epistemic_application_errors: rebuild.epistemic_application_errors,
        final_checksum: rebuild.final_checksum,
        final_epistemic_checksum: rebuild.final_epistemic_checksum,
        epistemic_projection_version,
        expected_checksum,
        matches_expected,
        state_diff,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actions::pipeline::{run_pipeline, PipelineContext};
    use crate::actions::proposal::{Proposal, ProposalOrigin};
    use crate::actions::registry::ActionRegistry;
    use crate::events::log::EventLog;
    use crate::ids::{
        ActionId, ActorId, ContentVersion, EventId, FixtureId, ItemId, PlaceId, ProposalId,
        SchemaVersion,
    };
    use crate::location::Location;
    use crate::scheduler::{OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId};
    use crate::state::{ActorBody, ContainerState, ItemState, PhysicalState, PlaceState};
    use crate::time::SimTick;

    fn actor_id() -> ActorId {
        ActorId::new("actor_tomas").unwrap()
    }

    fn item_id() -> ItemId {
        ItemId::new("coin_stack_01").unwrap()
    }

    fn container_id() -> crate::ids::ContainerId {
        crate::ids::ContainerId::new("strongbox_tomas").unwrap()
    }

    fn initial_state() -> PhysicalState {
        let mut state = PhysicalState::default();
        let shop = PlaceId::new("shop_front").unwrap();
        state
            .places
            .insert(shop.clone(), PlaceState::new(shop.clone(), "Shop front"));
        state
            .actors
            .insert(actor_id(), ActorBody::new(actor_id(), shop.clone()));
        let mut container = ContainerState::fixed_at_place(container_id(), shop);
        container.is_open = true;
        container.contents.insert(item_id());
        state.containers.insert(container_id(), container);
        state.items.insert(
            item_id(),
            ItemState::new(item_id(), Location::InContainer(container_id())),
        );
        state
    }

    fn ordering_key(action: &str) -> OrderingKey {
        OrderingKey::new(
            SimTick::ZERO,
            SchedulePhase::HumanCommand,
            SchedulerSourceId::Actor(actor_id()),
            ProposalSequence::new(0),
            ActionId::new(action).unwrap(),
            vec!["coin_stack_01".to_string()],
            "tie",
        )
    }

    fn context() -> ChecksumContext {
        ChecksumContext {
            fixture_id: FixtureId::new("replay_item_location_001").unwrap(),
            content_version: ContentVersion::new("phase1_manifest").unwrap(),
            sim_tick: SimTick::ZERO,
            world_stream_position_applied: 1,
        }
    }

    fn live_take_run() -> (PhysicalState, EventLog, PhysicalState) {
        let initial = initial_state();
        let mut live = initial.clone();
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
            state: &mut live,
            log: &mut log,
            controller_bindings: None,
            epistemic_projection: None,
            content_manifest_id: ContentManifestId::new("phase1_manifest").unwrap(),
            ordering_key: ordering_key("take"),
        };
        run_pipeline(&mut context, &proposal);
        (initial, log, live)
    }

    #[test]
    fn replay_clean_log_matches_expected() {
        let (initial, log, live) = live_take_run();
        let expected_checksum = compute_physical_checksum(&live, &context()).checksum;

        let report = run_replay(
            &initial,
            &log,
            &context(),
            Some(&live),
            Some(expected_checksum),
        );

        assert!(report.matches_expected);
        assert!(report.state_diff.is_empty());
        assert_eq!(report.event_count, 1);
    }

    #[test]
    fn missing_item_event_is_drift_not_warning() {
        let (initial, _log, live) = live_take_run();
        let empty_log = EventLog::new();
        let expected_checksum = compute_physical_checksum(&live, &context()).checksum;

        let report = run_replay(
            &initial,
            &empty_log,
            &context(),
            Some(&live),
            Some(expected_checksum),
        );

        assert!(!report.matches_expected);
        assert!(!report.state_diff.is_empty());
    }

    #[test]
    fn unsupported_version_is_reported_and_not_applied() {
        let (initial, log, live) = live_take_run();
        let mut event = log.events()[0].clone();
        event.event_id = EventId::new("event_bad_version").unwrap();
        event.event_schema_version = SchemaVersion::new("event_schema_v999").unwrap();
        let bad_log = event_log_from_events(vec![event]);

        let report = run_replay(&initial, &bad_log, &context(), Some(&live), None);

        assert!(!report.matches_expected);
        assert_eq!(report.unsupported_versions, ["event_schema_v999"]);
        assert_eq!(report.event_count, 0);
    }

    fn event_log_from_events(events: Vec<crate::events::EventEnvelope>) -> EventLog {
        EventLog::from_ordered_events_for_replay_tests(events)
    }
}
