use crate::epistemics::{Belief, Contradiction, Observation};
use crate::ids::{
    ActionId, ActorId, ContainerId, ContentManifestId, ControllerId, EventId, PlaceId, ProcessId,
    ProposalId, SchemaVersion, ValidationReportId,
};
use crate::scheduler::{OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId};
use crate::time::SimTick;

pub const EVENT_SCHEMA_V1: &str = EventSchemaVersion::V1.as_str();

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EventSchemaVersion {
    V1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventSchemaMigration {
    CurrentNoMigrationRequired,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EventSchemaRegistryEntry {
    pub version: EventSchemaVersion,
    pub migration: EventSchemaMigration,
}

pub const EVENT_SCHEMA_REGISTRY: &[EventSchemaRegistryEntry] = &[EventSchemaRegistryEntry {
    version: EventSchemaVersion::V1,
    migration: EventSchemaMigration::CurrentNoMigrationRequired,
}];

impl EventSchemaVersion {
    pub const fn all() -> &'static [EventSchemaVersion] {
        &[EventSchemaVersion::V1]
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            EventSchemaVersion::V1 => "event_schema_v1",
        }
    }

    pub fn to_schema_version(self) -> SchemaVersion {
        SchemaVersion::new(self.as_str()).expect("event schema registry uses valid stable ids")
    }

    pub fn from_schema_version(version: &SchemaVersion) -> Option<Self> {
        EVENT_SCHEMA_REGISTRY
            .iter()
            .find(|entry| entry.version.as_str() == version.as_str())
            .map(|entry| entry.version)
    }
}

pub fn event_schema_registry() -> &'static [EventSchemaRegistryEntry] {
    EVENT_SCHEMA_REGISTRY
}

pub fn event_schema_registry_entry(
    version: &SchemaVersion,
) -> Option<&'static EventSchemaRegistryEntry> {
    EVENT_SCHEMA_REGISTRY
        .iter()
        .find(|entry| entry.version.as_str() == version.as_str())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EventStream {
    World,
    Agent,
    Epistemic,
    Diagnostic,
    Controller,
    ReplayDebug,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EventKind {
    ControllerAttached,
    ControllerDetached,
    ActorMoved,
    DoorOpened,
    DoorClosed,
    ContainerOpened,
    ContainerClosed,
    ItemRemovedFromContainer,
    ItemTakenFromPlace,
    ItemPlacedInContainer,
    ItemPlacedInPlace,
    ActorWaited,
    TimeAdvanced,
    DeclaredWorldProcessApplied,
    ActionStarted,
    ActionFailed,
    ActionRejected,
    NoHumanAdvanceStarted,
    NoHumanAdvanceCompleted,
    InitialBeliefSeeded,
    RoleAssignmentNoticeRecorded,
    StartingBeliefRecorded,
    ObservationRecorded,
    BeliefUpdated,
    ExpectationContradicted,
    ContainerChecked,
    NeedDeltaApplied,
    NeedThresholdCrossed,
    CandidateGoalsEvaluated,
    IntentionStarted,
    IntentionContinued,
    IntentionSuspended,
    IntentionResumed,
    IntentionCompleted,
    IntentionFailed,
    IntentionAbandoned,
    IntentionInterrupted,
    RoutineStepStarted,
    RoutineStepCompleted,
    RoutineStepFailed,
    DecisionTraceRecorded,
    SleepStarted,
    SleepCompleted,
    SleepInterrupted,
    FoodConsumed,
    FoodServiceUsed,
    EatFailed,
    WorkBlockStarted,
    WorkBlockCompleted,
    WorkBlockFailed,
    ContinueRoutineProposed,
    ContinueRoutineAccepted,
    ContinueRoutineRejected,
    StuckDiagnosticRecorded,
    NoHumanDayStarted,
    NoHumanDayCompleted,
    ReplayProjectionRebuilt,
}

impl EventKind {
    pub const fn all() -> &'static [EventKind] {
        &[
            EventKind::ControllerAttached,
            EventKind::ControllerDetached,
            EventKind::ActorMoved,
            EventKind::DoorOpened,
            EventKind::DoorClosed,
            EventKind::ContainerOpened,
            EventKind::ContainerClosed,
            EventKind::ItemRemovedFromContainer,
            EventKind::ItemTakenFromPlace,
            EventKind::ItemPlacedInContainer,
            EventKind::ItemPlacedInPlace,
            EventKind::ActorWaited,
            EventKind::TimeAdvanced,
            EventKind::DeclaredWorldProcessApplied,
            EventKind::ActionStarted,
            EventKind::ActionFailed,
            EventKind::ActionRejected,
            EventKind::NoHumanAdvanceStarted,
            EventKind::NoHumanAdvanceCompleted,
            EventKind::InitialBeliefSeeded,
            EventKind::RoleAssignmentNoticeRecorded,
            EventKind::StartingBeliefRecorded,
            EventKind::ObservationRecorded,
            EventKind::BeliefUpdated,
            EventKind::ExpectationContradicted,
            EventKind::ContainerChecked,
            EventKind::NeedDeltaApplied,
            EventKind::NeedThresholdCrossed,
            EventKind::CandidateGoalsEvaluated,
            EventKind::IntentionStarted,
            EventKind::IntentionContinued,
            EventKind::IntentionSuspended,
            EventKind::IntentionResumed,
            EventKind::IntentionCompleted,
            EventKind::IntentionFailed,
            EventKind::IntentionAbandoned,
            EventKind::IntentionInterrupted,
            EventKind::RoutineStepStarted,
            EventKind::RoutineStepCompleted,
            EventKind::RoutineStepFailed,
            EventKind::DecisionTraceRecorded,
            EventKind::SleepStarted,
            EventKind::SleepCompleted,
            EventKind::SleepInterrupted,
            EventKind::FoodConsumed,
            EventKind::FoodServiceUsed,
            EventKind::EatFailed,
            EventKind::WorkBlockStarted,
            EventKind::WorkBlockCompleted,
            EventKind::WorkBlockFailed,
            EventKind::ContinueRoutineProposed,
            EventKind::ContinueRoutineAccepted,
            EventKind::ContinueRoutineRejected,
            EventKind::StuckDiagnosticRecorded,
            EventKind::NoHumanDayStarted,
            EventKind::NoHumanDayCompleted,
            EventKind::ReplayProjectionRebuilt,
        ]
    }

    pub fn registry() -> Vec<EventKindMetadata> {
        Self::all().iter().map(|kind| kind.metadata()).collect()
    }

    pub const fn metadata(self) -> EventKindMetadata {
        EventKindMetadata {
            kind: self,
            stream: self.stream(),
            schema_version: EventSchemaVersion::V1,
            physical_mutating: self.physical_mutating(),
            cause_required: self.cause_required(),
            replay_handling: EventReplayHandling::for_stream(self.stream()),
        }
    }

    pub const fn stream(self) -> EventStream {
        match self {
            EventKind::ControllerAttached | EventKind::ControllerDetached => {
                EventStream::Controller
            }
            EventKind::ActionStarted
            | EventKind::ActionFailed
            | EventKind::ActionRejected
            | EventKind::NoHumanAdvanceStarted
            | EventKind::NoHumanAdvanceCompleted => EventStream::Diagnostic,
            EventKind::InitialBeliefSeeded
            | EventKind::RoleAssignmentNoticeRecorded
            | EventKind::StartingBeliefRecorded
            | EventKind::ObservationRecorded
            | EventKind::BeliefUpdated
            | EventKind::ExpectationContradicted
            | EventKind::ContainerChecked => EventStream::Epistemic,
            EventKind::NeedDeltaApplied
            | EventKind::NeedThresholdCrossed
            | EventKind::CandidateGoalsEvaluated
            | EventKind::IntentionStarted
            | EventKind::IntentionContinued
            | EventKind::IntentionSuspended
            | EventKind::IntentionResumed
            | EventKind::IntentionCompleted
            | EventKind::IntentionFailed
            | EventKind::IntentionAbandoned
            | EventKind::IntentionInterrupted
            | EventKind::RoutineStepStarted
            | EventKind::RoutineStepCompleted
            | EventKind::RoutineStepFailed
            | EventKind::DecisionTraceRecorded
            | EventKind::SleepStarted
            | EventKind::SleepCompleted
            | EventKind::SleepInterrupted
            | EventKind::FoodServiceUsed
            | EventKind::EatFailed
            | EventKind::WorkBlockStarted
            | EventKind::WorkBlockCompleted
            | EventKind::WorkBlockFailed
            | EventKind::ContinueRoutineProposed
            | EventKind::ContinueRoutineAccepted
            | EventKind::ContinueRoutineRejected
            | EventKind::StuckDiagnosticRecorded
            | EventKind::NoHumanDayStarted
            | EventKind::NoHumanDayCompleted => EventStream::Agent,
            EventKind::ReplayProjectionRebuilt => EventStream::ReplayDebug,
            EventKind::ActorMoved
            | EventKind::DoorOpened
            | EventKind::DoorClosed
            | EventKind::ContainerOpened
            | EventKind::ContainerClosed
            | EventKind::ItemRemovedFromContainer
            | EventKind::ItemTakenFromPlace
            | EventKind::ItemPlacedInContainer
            | EventKind::ItemPlacedInPlace
            | EventKind::FoodConsumed
            | EventKind::ActorWaited
            | EventKind::TimeAdvanced
            | EventKind::DeclaredWorldProcessApplied => EventStream::World,
        }
    }

    pub const fn physical_mutating(self) -> bool {
        matches!(
            self,
            EventKind::ActorMoved
                | EventKind::DoorOpened
                | EventKind::DoorClosed
                | EventKind::ContainerOpened
                | EventKind::ContainerClosed
                | EventKind::ItemRemovedFromContainer
                | EventKind::ItemTakenFromPlace
                | EventKind::ItemPlacedInContainer
                | EventKind::ItemPlacedInPlace
                | EventKind::FoodConsumed
                | EventKind::ActorWaited
                | EventKind::TimeAdvanced
                | EventKind::DeclaredWorldProcessApplied
        )
    }

    pub const fn stable_id(self) -> &'static str {
        match self {
            EventKind::ControllerAttached => "controller_attached",
            EventKind::ControllerDetached => "controller_detached",
            EventKind::ActorMoved => "actor_moved",
            EventKind::DoorOpened => "door_opened",
            EventKind::DoorClosed => "door_closed",
            EventKind::ContainerOpened => "container_opened",
            EventKind::ContainerClosed => "container_closed",
            EventKind::ItemRemovedFromContainer => "item_removed_from_container",
            EventKind::ItemTakenFromPlace => "item_taken_from_place",
            EventKind::ItemPlacedInContainer => "item_placed_in_container",
            EventKind::ItemPlacedInPlace => "item_placed_in_place",
            EventKind::ActorWaited => "actor_waited",
            EventKind::TimeAdvanced => "time_advanced",
            EventKind::DeclaredWorldProcessApplied => "declared_world_process_applied",
            EventKind::ActionStarted => "action_started",
            EventKind::ActionFailed => "action_failed",
            EventKind::ActionRejected => "action_rejected",
            EventKind::NoHumanAdvanceStarted => "no_human_advance_started",
            EventKind::NoHumanAdvanceCompleted => "no_human_advance_completed",
            EventKind::InitialBeliefSeeded => "initial_belief_seeded",
            EventKind::RoleAssignmentNoticeRecorded => "role_assignment_notice_recorded",
            EventKind::StartingBeliefRecorded => "starting_belief_recorded",
            EventKind::ObservationRecorded => "observation_recorded",
            EventKind::BeliefUpdated => "belief_updated",
            EventKind::ExpectationContradicted => "expectation_contradicted",
            EventKind::ContainerChecked => "container_checked",
            EventKind::NeedDeltaApplied => "need_delta_applied",
            EventKind::NeedThresholdCrossed => "need_threshold_crossed",
            EventKind::CandidateGoalsEvaluated => "candidate_goals_evaluated",
            EventKind::IntentionStarted => "intention_started",
            EventKind::IntentionContinued => "intention_continued",
            EventKind::IntentionSuspended => "intention_suspended",
            EventKind::IntentionResumed => "intention_resumed",
            EventKind::IntentionCompleted => "intention_completed",
            EventKind::IntentionFailed => "intention_failed",
            EventKind::IntentionAbandoned => "intention_abandoned",
            EventKind::IntentionInterrupted => "intention_interrupted",
            EventKind::RoutineStepStarted => "routine_step_started",
            EventKind::RoutineStepCompleted => "routine_step_completed",
            EventKind::RoutineStepFailed => "routine_step_failed",
            EventKind::DecisionTraceRecorded => "decision_trace_recorded",
            EventKind::SleepStarted => "sleep_started",
            EventKind::SleepCompleted => "sleep_completed",
            EventKind::SleepInterrupted => "sleep_interrupted",
            EventKind::FoodConsumed => "food_consumed",
            EventKind::FoodServiceUsed => "food_service_used",
            EventKind::EatFailed => "eat_failed",
            EventKind::WorkBlockStarted => "work_block_started",
            EventKind::WorkBlockCompleted => "work_block_completed",
            EventKind::WorkBlockFailed => "work_block_failed",
            EventKind::ContinueRoutineProposed => "continue_routine_proposed",
            EventKind::ContinueRoutineAccepted => "continue_routine_accepted",
            EventKind::ContinueRoutineRejected => "continue_routine_rejected",
            EventKind::StuckDiagnosticRecorded => "stuck_diagnostic_recorded",
            EventKind::NoHumanDayStarted => "no_human_day_started",
            EventKind::NoHumanDayCompleted => "no_human_day_completed",
            EventKind::ReplayProjectionRebuilt => "replay_projection_rebuilt",
        }
    }

    fn from_stable_id(value: &str) -> Option<Self> {
        Self::all()
            .iter()
            .copied()
            .find(|kind| kind.stable_id() == value)
    }
}

pub const fn is_duration_terminal(kind: EventKind) -> bool {
    match kind {
        EventKind::SleepCompleted
        | EventKind::SleepInterrupted
        | EventKind::WorkBlockCompleted
        | EventKind::WorkBlockFailed => true,
        EventKind::ControllerAttached
        | EventKind::ControllerDetached
        | EventKind::ActorMoved
        | EventKind::DoorOpened
        | EventKind::DoorClosed
        | EventKind::ContainerOpened
        | EventKind::ContainerClosed
        | EventKind::ItemRemovedFromContainer
        | EventKind::ItemTakenFromPlace
        | EventKind::ItemPlacedInContainer
        | EventKind::ItemPlacedInPlace
        | EventKind::ActorWaited
        | EventKind::TimeAdvanced
        | EventKind::DeclaredWorldProcessApplied
        | EventKind::ActionStarted
        | EventKind::ActionFailed
        | EventKind::ActionRejected
        | EventKind::NoHumanAdvanceStarted
        | EventKind::NoHumanAdvanceCompleted
        | EventKind::InitialBeliefSeeded
        | EventKind::RoleAssignmentNoticeRecorded
        | EventKind::StartingBeliefRecorded
        | EventKind::ObservationRecorded
        | EventKind::BeliefUpdated
        | EventKind::ExpectationContradicted
        | EventKind::ContainerChecked
        | EventKind::NeedDeltaApplied
        | EventKind::NeedThresholdCrossed
        | EventKind::CandidateGoalsEvaluated
        | EventKind::IntentionStarted
        | EventKind::IntentionContinued
        | EventKind::IntentionSuspended
        | EventKind::IntentionResumed
        | EventKind::IntentionCompleted
        | EventKind::IntentionFailed
        | EventKind::IntentionAbandoned
        | EventKind::IntentionInterrupted
        | EventKind::RoutineStepStarted
        | EventKind::RoutineStepCompleted
        | EventKind::RoutineStepFailed
        | EventKind::DecisionTraceRecorded
        | EventKind::SleepStarted
        | EventKind::FoodConsumed
        | EventKind::FoodServiceUsed
        | EventKind::EatFailed
        | EventKind::WorkBlockStarted
        | EventKind::ContinueRoutineProposed
        | EventKind::ContinueRoutineAccepted
        | EventKind::ContinueRoutineRejected
        | EventKind::StuckDiagnosticRecorded
        | EventKind::NoHumanDayStarted
        | EventKind::NoHumanDayCompleted
        | EventKind::ReplayProjectionRebuilt => false,
    }
}

impl EventKind {
    pub const fn requires_cause(self) -> bool {
        self.metadata().cause_required
    }

    const fn cause_required(self) -> bool {
        match self {
            EventKind::ActorMoved
            | EventKind::DoorOpened
            | EventKind::DoorClosed
            | EventKind::ContainerOpened
            | EventKind::ContainerClosed
            | EventKind::ItemRemovedFromContainer
            | EventKind::ItemTakenFromPlace
            | EventKind::ItemPlacedInContainer
            | EventKind::ItemPlacedInPlace
            | EventKind::ActorWaited
            | EventKind::TimeAdvanced
            | EventKind::DeclaredWorldProcessApplied
            | EventKind::ObservationRecorded
            | EventKind::BeliefUpdated
            | EventKind::ContainerChecked
            | EventKind::NeedDeltaApplied
            | EventKind::NeedThresholdCrossed
            | EventKind::CandidateGoalsEvaluated
            | EventKind::IntentionStarted
            | EventKind::IntentionContinued
            | EventKind::IntentionSuspended
            | EventKind::IntentionResumed
            | EventKind::IntentionCompleted
            | EventKind::IntentionFailed
            | EventKind::IntentionAbandoned
            | EventKind::IntentionInterrupted
            | EventKind::RoutineStepStarted
            | EventKind::RoutineStepCompleted
            | EventKind::RoutineStepFailed
            | EventKind::DecisionTraceRecorded
            | EventKind::SleepStarted
            | EventKind::SleepCompleted
            | EventKind::SleepInterrupted
            | EventKind::FoodConsumed
            | EventKind::FoodServiceUsed
            | EventKind::EatFailed
            | EventKind::WorkBlockStarted
            | EventKind::WorkBlockCompleted
            | EventKind::WorkBlockFailed
            | EventKind::ContinueRoutineProposed
            | EventKind::ContinueRoutineAccepted
            | EventKind::ContinueRoutineRejected
            | EventKind::StuckDiagnosticRecorded
            | EventKind::NoHumanDayStarted
            | EventKind::NoHumanDayCompleted => true,
            EventKind::ControllerAttached
            | EventKind::ControllerDetached
            | EventKind::ActionStarted
            | EventKind::ActionFailed
            | EventKind::ActionRejected
            | EventKind::NoHumanAdvanceStarted
            | EventKind::NoHumanAdvanceCompleted
            | EventKind::InitialBeliefSeeded
            | EventKind::RoleAssignmentNoticeRecorded
            | EventKind::StartingBeliefRecorded
            | EventKind::ExpectationContradicted
            | EventKind::ReplayProjectionRebuilt => false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EventKindMetadata {
    pub kind: EventKind,
    pub stream: EventStream,
    pub schema_version: EventSchemaVersion,
    pub physical_mutating: bool,
    pub cause_required: bool,
    pub replay_handling: EventReplayHandling,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EventReplayHandling {
    ApplyWorld,
    ApplyAgent,
    ApplyEpistemicProjection,
    NonMutatingNoOp,
}

impl EventReplayHandling {
    pub const fn for_stream(stream: EventStream) -> Self {
        match stream {
            EventStream::World => EventReplayHandling::ApplyWorld,
            EventStream::Agent => EventReplayHandling::ApplyAgent,
            EventStream::Epistemic => EventReplayHandling::ApplyEpistemicProjection,
            EventStream::Diagnostic | EventStream::Controller | EventStream::ReplayDebug => {
                EventReplayHandling::NonMutatingNoOp
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InitialBeliefSourceKind {
    AuthoredPrehistory,
}

impl InitialBeliefSourceKind {
    pub const fn stable_id(self) -> &'static str {
        match self {
            InitialBeliefSourceKind::AuthoredPrehistory => "authored_prehistory",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InitialBeliefSeededPayload {
    pub schema_version: SchemaVersion,
    pub source_kind: InitialBeliefSourceKind,
    pub belief: Belief,
}

impl InitialBeliefSeededPayload {
    pub fn new_v1(belief: Belief) -> Self {
        Self {
            schema_version: SchemaVersion::new(EVENT_SCHEMA_V1).unwrap(),
            source_kind: InitialBeliefSourceKind::AuthoredPrehistory,
            belief,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoleAssignmentNoticeRecordedPayload {
    pub schema_version: SchemaVersion,
    pub source_kind: InitialBeliefSourceKind,
    pub actor_id: ActorId,
    pub workplace_id: String,
    pub place_id: PlaceId,
}

impl RoleAssignmentNoticeRecordedPayload {
    pub fn new_v1(actor_id: ActorId, workplace_id: impl Into<String>, place_id: PlaceId) -> Self {
        Self {
            schema_version: SchemaVersion::new(EVENT_SCHEMA_V1).unwrap(),
            source_kind: InitialBeliefSourceKind::AuthoredPrehistory,
            actor_id,
            workplace_id: workplace_id.into(),
            place_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StartingBeliefRecordedPayload {
    pub schema_version: SchemaVersion,
    pub source_kind: InitialBeliefSourceKind,
    pub actor_id: ActorId,
    pub belief_kind: String,
    pub subject_id: String,
    pub value: String,
}

impl StartingBeliefRecordedPayload {
    pub fn new_v1(
        actor_id: ActorId,
        belief_kind: impl Into<String>,
        subject_id: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self {
            schema_version: SchemaVersion::new(EVENT_SCHEMA_V1).unwrap(),
            source_kind: InitialBeliefSourceKind::AuthoredPrehistory,
            actor_id,
            belief_kind: belief_kind.into(),
            subject_id: subject_id.into(),
            value: value.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObservationRecordedPayload {
    pub schema_version: SchemaVersion,
    pub observation: Observation,
}

impl ObservationRecordedPayload {
    pub fn new_v1(observation: Observation) -> Self {
        Self {
            schema_version: SchemaVersion::new(EVENT_SCHEMA_V1).unwrap(),
            observation,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BeliefUpdatedPayload {
    pub schema_version: SchemaVersion,
    pub belief: Belief,
}

impl BeliefUpdatedPayload {
    pub fn new_v1(belief: Belief) -> Self {
        Self {
            schema_version: SchemaVersion::new(EVENT_SCHEMA_V1).unwrap(),
            belief,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExpectationContradictedPayload {
    pub schema_version: SchemaVersion,
    pub contradiction: Contradiction,
}

impl ExpectationContradictedPayload {
    pub fn new_v1(contradiction: Contradiction) -> Self {
        Self {
            schema_version: SchemaVersion::new(EVENT_SCHEMA_V1).unwrap(),
            contradiction,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContainerCheckedPayload {
    pub schema_version: SchemaVersion,
    pub actor_id: ActorId,
    pub container_id: ContainerId,
    pub source_action_id: ActionId,
    pub observed_tick: SimTick,
}

impl ContainerCheckedPayload {
    pub fn new_v1(
        actor_id: ActorId,
        container_id: ContainerId,
        source_action_id: ActionId,
        observed_tick: SimTick,
    ) -> Self {
        Self {
            schema_version: SchemaVersion::new(EVENT_SCHEMA_V1).unwrap(),
            actor_id,
            container_id,
            source_action_id,
            observed_tick,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EventCause {
    Event(EventId),
    Proposal(ProposalId),
    ValidationReport(ValidationReportId),
    Process(ProcessId),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RandomDrawRef {
    pub scope: String,
    pub draw_id: String,
    pub value: String,
}

impl RandomDrawRef {
    pub fn new(
        scope: impl Into<String>,
        draw_id: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self {
            scope: scope.into(),
            draw_id: draw_id.into(),
            value: value.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PayloadField {
    pub key: String,
    pub value: String,
}

impl PayloadField {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventEnvelope {
    pub event_id: EventId,
    pub event_type: EventKind,
    pub event_schema_version: SchemaVersion,
    pub stream: EventStream,
    pub stream_position: u64,
    pub global_order: u64,
    pub sim_tick: SimTick,
    pub ordering_key: OrderingKey,
    pub actor_id: Option<ActorId>,
    pub process_id: Option<ProcessId>,
    pub participants: Vec<String>,
    pub place_id: Option<PlaceId>,
    pub causes: Vec<EventCause>,
    pub proposal_id: Option<ProposalId>,
    pub validation_report_id: Option<ValidationReportId>,
    pub random_draws: Vec<RandomDrawRef>,
    pub payload: Vec<PayloadField>,
    pub effects_summary: String,
    pub content_manifest_id: ContentManifestId,
}

impl EventEnvelope {
    pub fn new_v1(
        event_id: EventId,
        event_type: EventKind,
        stream_position: u64,
        global_order: u64,
        sim_tick: SimTick,
        ordering_key: OrderingKey,
        content_manifest_id: ContentManifestId,
    ) -> Self {
        Self {
            event_id,
            event_type,
            event_schema_version: EventSchemaVersion::V1.to_schema_version(),
            stream: event_type.stream(),
            stream_position,
            global_order,
            sim_tick,
            ordering_key,
            actor_id: None,
            process_id: None,
            participants: Vec::new(),
            place_id: None,
            causes: Vec::new(),
            proposal_id: None,
            validation_report_id: None,
            random_draws: Vec::new(),
            payload: Vec::new(),
            effects_summary: String::new(),
            content_manifest_id,
        }
    }

    pub fn has_supported_schema_version(&self) -> bool {
        event_schema_registry_entry(&self.event_schema_version).is_some()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_caused_v1(
        event_id: EventId,
        event_type: EventKind,
        stream_position: u64,
        global_order: u64,
        sim_tick: SimTick,
        ordering_key: OrderingKey,
        content_manifest_id: ContentManifestId,
        causes: Vec<EventCause>,
    ) -> Result<Self, EventEnvelopeBuildError> {
        if event_type.requires_cause() && causes.is_empty() {
            return Err(EventEnvelopeBuildError::MissingRequiredCause);
        }

        let mut envelope = Self::new_v1(
            event_id,
            event_type,
            stream_position,
            global_order,
            sim_tick,
            ordering_key,
            content_manifest_id,
        );
        envelope.causes = causes;
        Ok(envelope)
    }

    pub fn serialize_canonical(&self) -> Vec<u8> {
        let fields = [
            ("event_id", encode(self.event_id.as_str())),
            ("event_type", encode(self.event_type.stable_id())),
            (
                "event_schema_version",
                encode(self.event_schema_version.as_str()),
            ),
            ("stream", encode(stream_id(self.stream))),
            ("stream_position", self.stream_position.to_string()),
            ("global_order", self.global_order.to_string()),
            ("sim_tick", self.sim_tick.value().to_string()),
            (
                "ordering_key",
                encode(&serialize_ordering_key(&self.ordering_key)),
            ),
            (
                "actor_id",
                encode_opt(self.actor_id.as_ref().map(ActorId::as_str)),
            ),
            (
                "process_id",
                encode_opt(self.process_id.as_ref().map(ProcessId::as_str)),
            ),
            ("participants", encode_vec(&self.participants)),
            (
                "place_id",
                encode_opt(self.place_id.as_ref().map(PlaceId::as_str)),
            ),
            (
                "causes",
                encode_vec(&self.causes.iter().map(serialize_cause).collect::<Vec<_>>()),
            ),
            (
                "proposal_id",
                encode_opt(self.proposal_id.as_ref().map(ProposalId::as_str)),
            ),
            (
                "validation_report_id",
                encode_opt(
                    self.validation_report_id
                        .as_ref()
                        .map(ValidationReportId::as_str),
                ),
            ),
            (
                "random_draws",
                encode_vec(
                    &self
                        .random_draws
                        .iter()
                        .map(serialize_random_draw)
                        .collect::<Vec<_>>(),
                ),
            ),
            (
                "payload",
                encode_vec(
                    &self
                        .payload
                        .iter()
                        .map(serialize_payload)
                        .collect::<Vec<_>>(),
                ),
            ),
            ("effects_summary", encode(&self.effects_summary)),
            (
                "content_manifest_id",
                encode(self.content_manifest_id.as_str()),
            ),
        ];

        fields
            .into_iter()
            .map(|(key, value)| format!("{key}={value}"))
            .collect::<Vec<_>>()
            .join("\n")
            .into_bytes()
    }

    pub fn deserialize_canonical(bytes: &[u8]) -> Result<Self, EventEnvelopeParseError> {
        let text = std::str::from_utf8(bytes).map_err(|_| EventEnvelopeParseError::InvalidUtf8)?;
        let mut map = std::collections::BTreeMap::new();
        for line in text.lines() {
            let (key, value) = line
                .split_once('=')
                .ok_or(EventEnvelopeParseError::MalformedField)?;
            if map.insert(key, value).is_some() {
                return Err(EventEnvelopeParseError::DuplicateField(key.to_string()));
            }
        }

        let event_type = EventKind::from_stable_id(&decode(required(&map, "event_type")?)?)
            .ok_or(EventEnvelopeParseError::UnknownEventKind)?;
        let event_schema_version =
            SchemaVersion::new(decode(required(&map, "event_schema_version")?)?)?;
        if event_schema_registry_entry(&event_schema_version).is_none() {
            return Err(EventEnvelopeParseError::UnsupportedSchemaVersion(
                event_schema_version.as_str().to_string(),
            ));
        }
        let ordering_key = deserialize_ordering_key(&decode(required(&map, "ordering_key")?)?)?;

        Ok(Self {
            event_id: EventId::new(decode(required(&map, "event_id")?)?)?,
            event_type,
            event_schema_version,
            stream: stream_from_id(&decode(required(&map, "stream")?)?)
                .ok_or(EventEnvelopeParseError::UnknownStream)?,
            stream_position: required(&map, "stream_position")?.parse()?,
            global_order: required(&map, "global_order")?.parse()?,
            sim_tick: SimTick::new(required(&map, "sim_tick")?.parse()?),
            ordering_key,
            actor_id: decode_opt(required(&map, "actor_id")?)?
                .map(ActorId::new)
                .transpose()?,
            process_id: decode_opt(required(&map, "process_id")?)?
                .map(ProcessId::new)
                .transpose()?,
            participants: decode_vec(required(&map, "participants")?)?,
            place_id: decode_opt(required(&map, "place_id")?)?
                .map(PlaceId::new)
                .transpose()?,
            causes: decode_vec(required(&map, "causes")?)?
                .into_iter()
                .map(|value| deserialize_cause(&value))
                .collect::<Result<_, _>>()?,
            proposal_id: decode_opt(required(&map, "proposal_id")?)?
                .map(ProposalId::new)
                .transpose()?,
            validation_report_id: decode_opt(required(&map, "validation_report_id")?)?
                .map(ValidationReportId::new)
                .transpose()?,
            random_draws: decode_vec(required(&map, "random_draws")?)?
                .into_iter()
                .map(|value| deserialize_random_draw(&value))
                .collect::<Result<_, _>>()?,
            payload: decode_vec(required(&map, "payload")?)?
                .into_iter()
                .map(|value| deserialize_payload(&value))
                .collect::<Result<_, _>>()?,
            effects_summary: decode(required(&map, "effects_summary")?)?,
            content_manifest_id: ContentManifestId::new(decode(required(
                &map,
                "content_manifest_id",
            )?)?)?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EventEnvelopeBuildError {
    MissingRequiredCause,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EventEnvelopeParseError {
    InvalidUtf8,
    MalformedField,
    DuplicateField(String),
    MissingField(&'static str),
    UnsupportedSchemaVersion(String),
    BadHex,
    UnknownEventKind,
    UnknownStream,
    UnknownSchedulePhase,
    UnknownSchedulerSource,
    UnknownCause,
    InvalidTuple,
    Id(crate::ids::IdError),
    Number,
}

impl From<crate::ids::IdError> for EventEnvelopeParseError {
    fn from(value: crate::ids::IdError) -> Self {
        Self::Id(value)
    }
}

impl From<std::num::ParseIntError> for EventEnvelopeParseError {
    fn from(_: std::num::ParseIntError) -> Self {
        Self::Number
    }
}

fn required<'a>(
    map: &'a std::collections::BTreeMap<&str, &str>,
    key: &'static str,
) -> Result<&'a str, EventEnvelopeParseError> {
    map.get(key)
        .copied()
        .ok_or(EventEnvelopeParseError::MissingField(key))
}

fn stream_id(stream: EventStream) -> &'static str {
    match stream {
        EventStream::World => "world",
        EventStream::Agent => "agent",
        EventStream::Epistemic => "epistemic",
        EventStream::Diagnostic => "diagnostic",
        EventStream::Controller => "controller",
        EventStream::ReplayDebug => "replay_debug",
    }
}

fn stream_from_id(value: &str) -> Option<EventStream> {
    match value {
        "world" => Some(EventStream::World),
        "agent" => Some(EventStream::Agent),
        "epistemic" => Some(EventStream::Epistemic),
        "diagnostic" => Some(EventStream::Diagnostic),
        "controller" => Some(EventStream::Controller),
        "replay_debug" => Some(EventStream::ReplayDebug),
        _ => None,
    }
}

fn serialize_ordering_key(key: &OrderingKey) -> String {
    [
        key.sim_tick.value().to_string(),
        schedule_phase_id(&key.phase).to_string(),
        serialize_source_id(&key.source_id),
        key.proposal_sequence.value().to_string(),
        key.action_id.as_str().to_string(),
        key.target_ids.join(","),
        key.final_tie_breaker.clone(),
    ]
    .into_iter()
    .map(|part| encode(&part))
    .collect::<Vec<_>>()
    .join("|")
}

fn deserialize_ordering_key(value: &str) -> Result<OrderingKey, EventEnvelopeParseError> {
    let parts = value
        .split('|')
        .map(decode)
        .collect::<Result<Vec<_>, _>>()?;
    if parts.len() != 7 {
        return Err(EventEnvelopeParseError::InvalidTuple);
    }

    Ok(OrderingKey::new(
        SimTick::new(parts[0].parse()?),
        schedule_phase_from_id(&parts[1]).ok_or(EventEnvelopeParseError::UnknownSchedulePhase)?,
        deserialize_source_id(&parts[2])?,
        ProposalSequence::new(parts[3].parse()?),
        ActionId::new(parts[4].clone())?,
        if parts[5].is_empty() {
            Vec::new()
        } else {
            parts[5].split(',').map(ToString::to_string).collect()
        },
        parts[6].clone(),
    ))
}

fn schedule_phase_id(phase: &SchedulePhase) -> &'static str {
    match phase {
        SchedulePhase::HumanCommand => "human_command",
        SchedulePhase::NoHumanProcess => "no_human_process",
        SchedulePhase::DeferredProcess => "deferred_process",
        SchedulePhase::Replay => "replay",
    }
}

fn schedule_phase_from_id(value: &str) -> Option<SchedulePhase> {
    match value {
        "human_command" => Some(SchedulePhase::HumanCommand),
        "no_human_process" => Some(SchedulePhase::NoHumanProcess),
        "deferred_process" => Some(SchedulePhase::DeferredProcess),
        "replay" => Some(SchedulePhase::Replay),
        _ => None,
    }
}

fn serialize_source_id(source_id: &SchedulerSourceId) -> String {
    match source_id {
        SchedulerSourceId::Actor(id) => format!("actor:{}", id.as_str()),
        SchedulerSourceId::Controller(id) => format!("controller:{}", id.as_str()),
        SchedulerSourceId::Process(id) => format!("process:{}", id.as_str()),
    }
}

fn deserialize_source_id(value: &str) -> Result<SchedulerSourceId, EventEnvelopeParseError> {
    let (kind, value) = value
        .split_once(':')
        .ok_or(EventEnvelopeParseError::UnknownSchedulerSource)?;
    match kind {
        "actor" => Ok(SchedulerSourceId::Actor(ActorId::new(value)?)),
        "controller" => Ok(SchedulerSourceId::Controller(ControllerId::new(value)?)),
        "process" => Ok(SchedulerSourceId::Process(ProcessId::new(value)?)),
        _ => Err(EventEnvelopeParseError::UnknownSchedulerSource),
    }
}

fn serialize_cause(cause: &EventCause) -> String {
    match cause {
        EventCause::Event(id) => format!("event:{}", id.as_str()),
        EventCause::Proposal(id) => format!("proposal:{}", id.as_str()),
        EventCause::ValidationReport(id) => format!("validation_report:{}", id.as_str()),
        EventCause::Process(id) => format!("process:{}", id.as_str()),
    }
}

fn deserialize_cause(value: &str) -> Result<EventCause, EventEnvelopeParseError> {
    let (kind, value) = value
        .split_once(':')
        .ok_or(EventEnvelopeParseError::UnknownCause)?;
    match kind {
        "event" => Ok(EventCause::Event(EventId::new(value)?)),
        "proposal" => Ok(EventCause::Proposal(ProposalId::new(value)?)),
        "validation_report" => Ok(EventCause::ValidationReport(ValidationReportId::new(
            value,
        )?)),
        "process" => Ok(EventCause::Process(ProcessId::new(value)?)),
        _ => Err(EventEnvelopeParseError::UnknownCause),
    }
}

fn serialize_random_draw(draw: &RandomDrawRef) -> String {
    [draw.scope.clone(), draw.draw_id.clone(), draw.value.clone()]
        .into_iter()
        .map(|part| encode(&part))
        .collect::<Vec<_>>()
        .join(":")
}

fn deserialize_random_draw(value: &str) -> Result<RandomDrawRef, EventEnvelopeParseError> {
    let parts = value
        .split(':')
        .map(decode)
        .collect::<Result<Vec<_>, _>>()?;
    if parts.len() != 3 {
        return Err(EventEnvelopeParseError::InvalidTuple);
    }
    Ok(RandomDrawRef::new(&parts[0], &parts[1], &parts[2]))
}

fn serialize_payload(field: &PayloadField) -> String {
    format!("{}:{}", encode(&field.key), encode(&field.value))
}

fn deserialize_payload(value: &str) -> Result<PayloadField, EventEnvelopeParseError> {
    let (key, value) = value
        .split_once(':')
        .ok_or(EventEnvelopeParseError::InvalidTuple)?;
    Ok(PayloadField::new(decode(key)?, decode(value)?))
}

fn encode_opt(value: Option<&str>) -> String {
    value.map(encode).unwrap_or_default()
}

fn decode_opt(value: &str) -> Result<Option<String>, EventEnvelopeParseError> {
    if value.is_empty() {
        Ok(None)
    } else {
        Ok(Some(decode(value)?))
    }
}

fn encode_vec(values: &[String]) -> String {
    values
        .iter()
        .map(|value| encode(value))
        .collect::<Vec<_>>()
        .join(";")
}

fn decode_vec(value: &str) -> Result<Vec<String>, EventEnvelopeParseError> {
    if value.is_empty() {
        Ok(Vec::new())
    } else {
        value.split(';').map(decode).collect()
    }
}

fn encode(value: &str) -> String {
    value
        .as_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn decode(value: &str) -> Result<String, EventEnvelopeParseError> {
    if !value.len().is_multiple_of(2) {
        return Err(EventEnvelopeParseError::BadHex);
    }

    let mut bytes = Vec::with_capacity(value.len() / 2);
    let chars: Vec<_> = value.as_bytes().chunks_exact(2).collect();
    for chunk in chars {
        let hex = std::str::from_utf8(chunk).map_err(|_| EventEnvelopeParseError::BadHex)?;
        let byte = u8::from_str_radix(hex, 16).map_err(|_| EventEnvelopeParseError::BadHex)?;
        bytes.push(byte);
    }

    String::from_utf8(bytes).map_err(|_| EventEnvelopeParseError::InvalidUtf8)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::epistemics::{
        Belief, Channel, Confidence, Contradiction, ContradictionKind, HolderKind, Observation,
        ObservationSubject, ObservationTarget, SourceRef, Stance,
    };
    use crate::ids::{BeliefId, ContradictionId, ItemId, ObservationId};
    use crate::location::Location;

    fn actor_id(value: &str) -> ActorId {
        ActorId::new(value).unwrap()
    }

    fn action_id(value: &str) -> ActionId {
        ActionId::new(value).unwrap()
    }

    fn ordering_key() -> OrderingKey {
        OrderingKey::new(
            SimTick::new(3),
            SchedulePhase::NoHumanProcess,
            SchedulerSourceId::Actor(actor_id("actor_tomas")),
            ProposalSequence::new(7),
            action_id("continue_routine"),
            vec!["phase3a".to_string()],
            "tie_phase3a",
        )
    }

    fn belief_id(value: &str) -> BeliefId {
        BeliefId::new(value).unwrap()
    }

    fn container_id(value: &str) -> ContainerId {
        ContainerId::new(value).unwrap()
    }

    fn contradiction_id(value: &str) -> ContradictionId {
        ContradictionId::new(value).unwrap()
    }

    #[test]
    fn event_schema_registry_has_one_current_version_with_migration_proof() {
        let registry = event_schema_registry();
        assert_eq!(registry.len(), 1);
        assert_eq!(registry[0].version, EventSchemaVersion::V1);
        assert_eq!(
            registry[0].migration,
            EventSchemaMigration::CurrentNoMigrationRequired
        );
        assert_eq!(EventSchemaVersion::V1.as_str(), EVENT_SCHEMA_V1);
        assert_eq!(
            EventSchemaVersion::from_schema_version(&SchemaVersion::new(EVENT_SCHEMA_V1).unwrap()),
            Some(EventSchemaVersion::V1)
        );
    }

    #[test]
    fn unknown_event_schema_version_is_not_registry_supported() {
        let unknown = SchemaVersion::new("event_schema_v999").unwrap();
        assert!(event_schema_registry_entry(&unknown).is_none());
        assert_eq!(EventSchemaVersion::from_schema_version(&unknown), None);

        let mut event = EventEnvelope::new_v1(
            event_id("event_unknown_schema"),
            EventKind::ActorMoved,
            0,
            0,
            SimTick::new(3),
            ordering_key(),
            ContentManifestId::new("phase1_manifest").unwrap(),
        );
        event.event_schema_version = unknown;
        assert!(!event.has_supported_schema_version());
    }

    fn event_id(value: &str) -> EventId {
        EventId::new(value).unwrap()
    }

    fn item_id(value: &str) -> ItemId {
        ItemId::new(value).unwrap()
    }

    fn observation_id(value: &str) -> ObservationId {
        ObservationId::new(value).unwrap()
    }

    fn place_id(value: &str) -> PlaceId {
        PlaceId::new(value).unwrap()
    }

    fn expected_proposition() -> crate::epistemics::Proposition {
        crate::epistemics::Proposition::ItemLocatedInContainer {
            item_id: item_id("coin_stack_01"),
            container_id: container_id("strongbox_tomas"),
        }
    }

    fn missing_proposition() -> crate::epistemics::Proposition {
        crate::epistemics::Proposition::ItemMissingFromExpectedLocation {
            item_id: item_id("coin_stack_01"),
            expected_location: Location::InContainer(container_id("strongbox_tomas")),
        }
    }

    fn belief() -> Belief {
        Belief::new(
            belief_id("belief_tomas_expected_coin"),
            HolderKind::Actor(actor_id("actor_tomas")),
            expected_proposition(),
            Stance::ExpectsTrue,
            Confidence::new(900).unwrap(),
            SourceRef::Event(event_id("event_initial_belief")),
            SimTick::ZERO,
        )
    }

    fn observation() -> Observation {
        Observation::new(
            observation_id("obs_tomas_checked_strongbox"),
            actor_id("actor_tomas"),
            Channel::AbsenceMarker,
            SimTick::new(3),
            place_id("tomas_room"),
            ObservationSubject::Container(container_id("strongbox_tomas")),
            ObservationTarget::Container(container_id("strongbox_tomas")),
            Confidence::new(950).unwrap(),
            SourceRef::Event(event_id("event_container_checked")),
        )
    }

    fn contradiction() -> Contradiction {
        Contradiction::new(
            contradiction_id("contradiction_tomas_missing_coin"),
            actor_id("actor_tomas"),
            ContradictionKind::ExpectedItemAbsentFromContainer,
            belief_id("belief_tomas_expected_coin"),
            observation_id("obs_tomas_checked_strongbox"),
            expected_proposition(),
            missing_proposition(),
            SimTick::new(3),
        )
    }

    #[test]
    fn epistemic_event_kinds_classify_to_epistemic_stream() {
        let kinds = [
            EventKind::InitialBeliefSeeded,
            EventKind::RoleAssignmentNoticeRecorded,
            EventKind::StartingBeliefRecorded,
            EventKind::ObservationRecorded,
            EventKind::BeliefUpdated,
            EventKind::ExpectationContradicted,
            EventKind::ContainerChecked,
        ];

        for kind in kinds {
            assert_eq!(kind.stream(), EventStream::Epistemic);
            assert!(!kind.physical_mutating());
            assert_eq!(EventKind::from_stable_id(kind.stable_id()), Some(kind));
        }
    }

    #[test]
    fn epistemic_stream_label_round_trips() {
        assert_eq!(stream_id(EventStream::Epistemic), "epistemic");
        assert_eq!(stream_from_id("epistemic"), Some(EventStream::Epistemic));
    }

    #[test]
    fn phase_3a_event_kinds_are_agent_stream_and_round_trip() {
        let phase_3a_kinds = EventKind::all()
            .iter()
            .copied()
            .filter(|kind| kind.stream() == EventStream::Agent)
            .collect::<Vec<_>>();

        assert_eq!(phase_3a_kinds.len(), 29);
        for kind in phase_3a_kinds {
            assert!(kind.requires_cause());
            assert!(!kind.physical_mutating());
            assert_eq!(EventKind::from_stable_id(kind.stable_id()), Some(kind));

            let envelope = EventEnvelope::new_caused_v1(
                event_id("event_phase3a"),
                kind,
                0,
                0,
                SimTick::new(3),
                ordering_key(),
                ContentManifestId::new("phase3a_manifest").unwrap(),
                vec![EventCause::Process(
                    ProcessId::new("process_agent").unwrap(),
                )],
            )
            .unwrap();
            let round_tripped =
                EventEnvelope::deserialize_canonical(&envelope.serialize_canonical()).unwrap();
            assert_eq!(round_tripped.event_type, kind);
            assert_eq!(round_tripped.stream, EventStream::Agent);
        }
    }

    #[test]
    fn food_consumed_is_physical_world_event() {
        assert_eq!(EventKind::FoodConsumed.stream(), EventStream::World);
        assert!(EventKind::FoodConsumed.physical_mutating());
        assert!(EventKind::FoodConsumed.requires_cause());
        assert_eq!(
            EventKind::from_stable_id(EventKind::FoodConsumed.stable_id()),
            Some(EventKind::FoodConsumed)
        );
    }

    #[test]
    fn caused_constructor_rejects_causeless_phase_3a_event() {
        assert_eq!(
            EventEnvelope::new_caused_v1(
                event_id("event_need_delta"),
                EventKind::NeedDeltaApplied,
                0,
                0,
                SimTick::new(3),
                ordering_key(),
                ContentManifestId::new("phase3a_manifest").unwrap(),
                Vec::new(),
            )
            .unwrap_err(),
            EventEnvelopeBuildError::MissingRequiredCause
        );
    }

    #[test]
    fn agent_stream_label_round_trips() {
        assert_eq!(stream_id(EventStream::Agent), "agent");
        assert_eq!(stream_from_id("agent"), Some(EventStream::Agent));
    }

    #[test]
    fn epistemic_payloads_are_versioned_and_can_hold_unsupported_schema() {
        let mut observation_payload = ObservationRecordedPayload::new_v1(observation());
        let belief_payload = BeliefUpdatedPayload::new_v1(belief());
        let initial_payload = InitialBeliefSeededPayload::new_v1(belief());
        let role_assignment_payload = RoleAssignmentNoticeRecordedPayload::new_v1(
            actor_id("actor_tomas"),
            "workplace_tomas",
            place_id("workshop_tomas"),
        );
        let starting_belief_payload = StartingBeliefRecordedPayload::new_v1(
            actor_id("actor_tomas"),
            "sleep_place",
            "bed_tomas",
            "home_tomas",
        );
        let contradiction_payload = ExpectationContradictedPayload::new_v1(contradiction());
        let container_checked_payload = ContainerCheckedPayload::new_v1(
            actor_id("actor_tomas"),
            container_id("strongbox_tomas"),
            action_id("check_container"),
            SimTick::new(3),
        );

        assert_eq!(observation_payload.schema_version.as_str(), EVENT_SCHEMA_V1);
        assert_eq!(belief_payload.schema_version.as_str(), EVENT_SCHEMA_V1);
        assert_eq!(initial_payload.schema_version.as_str(), EVENT_SCHEMA_V1);
        assert_eq!(
            initial_payload.source_kind,
            InitialBeliefSourceKind::AuthoredPrehistory
        );
        assert_eq!(
            initial_payload.source_kind.stable_id(),
            "authored_prehistory"
        );
        assert_eq!(
            role_assignment_payload.schema_version.as_str(),
            EVENT_SCHEMA_V1
        );
        assert_eq!(
            role_assignment_payload.source_kind,
            InitialBeliefSourceKind::AuthoredPrehistory
        );
        assert_eq!(
            role_assignment_payload.source_kind.stable_id(),
            "authored_prehistory"
        );
        assert_eq!(
            starting_belief_payload.schema_version.as_str(),
            EVENT_SCHEMA_V1
        );
        assert_eq!(
            starting_belief_payload.source_kind,
            InitialBeliefSourceKind::AuthoredPrehistory
        );
        assert_eq!(
            starting_belief_payload.source_kind.stable_id(),
            "authored_prehistory"
        );
        assert_eq!(
            contradiction_payload.schema_version.as_str(),
            EVENT_SCHEMA_V1
        );
        assert_eq!(
            container_checked_payload.schema_version.as_str(),
            EVENT_SCHEMA_V1
        );

        observation_payload.schema_version = SchemaVersion::new("event_schema_v999").unwrap();
        assert_eq!(observation_payload.observation, observation());
        assert_eq!(
            observation_payload.schema_version.as_str(),
            "event_schema_v999"
        );
    }
}
