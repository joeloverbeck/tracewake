use crate::actions::pipeline::PipelineStage;
use crate::actions::report::{CheckedFact, ReasonCode, ValidationReport};
use crate::checksum::{compute_physical_checksum, ChecksumContext, PhysicalChecksum};
use crate::controller::ControllerBindings;
use crate::events::log::EventLog;
use crate::events::{EventEnvelope, EventKind, EventStream};
use crate::ids::{DebugReportId, EventId, ItemId, ProposalId, ValidationReportId};
use crate::location::Location;
use crate::replay::{ProjectionRebuildReport, ReplayReport};
use crate::state::PhysicalState;

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
    pub debug_only: bool,
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
    pub checked_facts: Vec<CheckedFact>,
    pub precondition_trace: Vec<String>,
    pub events_created: Vec<EventId>,
    pub mutation_attempted: bool,
    pub checksum_before: PhysicalChecksum,
    pub checksum_after: PhysicalChecksum,
    pub debug_only: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProjectionRebuildDebugReport {
    pub report_id: DebugReportId,
    pub rebuild: ProjectionRebuildReport,
    pub debug_only: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplayDebugReport {
    pub report_id: DebugReportId,
    pub replay: ReplayReport,
    pub debug_only: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ControllerBindingDebugReport {
    pub report_id: DebugReportId,
    pub bindings: Vec<String>,
    pub debug_only: bool,
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
        debug_only: true,
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
        checked_facts: validation_report.checked_facts.clone(),
        precondition_trace: validation_report
            .checked_facts
            .iter()
            .map(|fact| format!("{}={}", fact.key, fact.value))
            .collect(),
        events_created: validation_report.event_ids.clone(),
        mutation_attempted: false,
        checksum_before: checksum.clone(),
        checksum_after: checksum,
        debug_only: true,
    }
}

pub fn projection_rebuild_debug_report(
    report_id: DebugReportId,
    rebuild: ProjectionRebuildReport,
) -> ProjectionRebuildDebugReport {
    ProjectionRebuildDebugReport {
        report_id,
        rebuild,
        debug_only: true,
    }
}

pub fn replay_debug_report(report_id: DebugReportId, replay: ReplayReport) -> ReplayDebugReport {
    ReplayDebugReport {
        report_id,
        replay,
        debug_only: true,
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
        debug_only: true,
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
    use crate::ids::{
        ActionId, ActorId, ContainerId, ContentManifestId, ContentVersion, FixtureId, PlaceId,
        ProposalId,
    };
    use crate::state::{ActorBody, ContainerState, ItemState, PhysicalState, PlaceState};
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

    fn state() -> PhysicalState {
        let mut state = PhysicalState::default();
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
            log: &mut log,
            controller_bindings: None,
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

        assert!(report.debug_only);
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
            log: &mut log,
            controller_bindings: None,
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

        assert!(report.debug_only);
        assert_eq!(
            report.failed_stage,
            Some(crate::actions::pipeline::PipelineStage::ActionDefinitionLookup)
        );
        assert!(!report.checked_facts.is_empty());
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

        assert!(report.debug_only);
        assert_eq!(
            report.last_location_event_id.unwrap().as_str(),
            "fixture_origin.coin_stack_01"
        );
        assert_eq!(state, before);
    }
}
