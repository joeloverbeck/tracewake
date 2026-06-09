use crate::ids::{ActionId, ActorId, EventId, ProposalId, ValidationReportId};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReportStatus {
    Accepted,
    Rejected,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReasonCode {
    UnknownActionId,
    PhaseUnsupportedAction,
    ControllerUnbound,
    ControllerActorMismatch,
    ActorNotFound,
    TargetNotFound,
    UnsupportedTargetKind,
    ActorNotAtRequiredPlace,
    NotAdjacent,
    DoorClosedBlocksMovement,
    DoorLocked,
    ContainerClosed,
    ContainerLocked,
    TargetNotVisible,
    TargetNotReachable,
    ItemNotPortable,
    ItemNotAtSource,
    ItemNotCarried,
    CarryCapacityExceeded,
    DestinationNotOpen,
    AlreadyOpen,
    AlreadyClosed,
    InvalidParameter,
    WorldStateMismatch,
    TargetReserved,
    KnowledgePreconditionNotMet,
    ProposalSourceMissing,
    ProposalSourceActorMismatch,
    ProposalSourceContextMismatch,
    ProposalSourceStale,
    ProposalSourceForged,
    NoCurrentIntention,
    IntentionTerminal,
    RoutineStepBlocked,
    ReservationConflict,
    NoSleepAffordance,
}

impl ReasonCode {
    pub const fn stable_id(self) -> &'static str {
        match self {
            ReasonCode::UnknownActionId => "unknown_action_id",
            ReasonCode::PhaseUnsupportedAction => "phase_unsupported_action",
            ReasonCode::ControllerUnbound => "controller_unbound",
            ReasonCode::ControllerActorMismatch => "controller_actor_mismatch",
            ReasonCode::ActorNotFound => "actor_not_found",
            ReasonCode::TargetNotFound => "target_not_found",
            ReasonCode::UnsupportedTargetKind => "unsupported_target_kind",
            ReasonCode::ActorNotAtRequiredPlace => "actor_not_at_required_place",
            ReasonCode::NotAdjacent => "not_adjacent",
            ReasonCode::DoorClosedBlocksMovement => "door_closed_blocks_movement",
            ReasonCode::DoorLocked => "door_locked",
            ReasonCode::ContainerClosed => "container_closed",
            ReasonCode::ContainerLocked => "container_locked",
            ReasonCode::TargetNotVisible => "target_not_visible",
            ReasonCode::TargetNotReachable => "target_not_reachable",
            ReasonCode::ItemNotPortable => "item_not_portable",
            ReasonCode::ItemNotAtSource => "item_not_at_source",
            ReasonCode::ItemNotCarried => "item_not_carried",
            ReasonCode::CarryCapacityExceeded => "carry_capacity_exceeded",
            ReasonCode::DestinationNotOpen => "destination_not_open",
            ReasonCode::AlreadyOpen => "already_open",
            ReasonCode::AlreadyClosed => "already_closed",
            ReasonCode::InvalidParameter => "invalid_parameter",
            ReasonCode::WorldStateMismatch => "world_state_mismatch",
            ReasonCode::TargetReserved => "target_reserved",
            ReasonCode::KnowledgePreconditionNotMet => "knowledge_precondition_not_met",
            ReasonCode::ProposalSourceMissing => "proposal_source_missing",
            ReasonCode::ProposalSourceActorMismatch => "proposal_source_actor_mismatch",
            ReasonCode::ProposalSourceContextMismatch => "proposal_source_context_mismatch",
            ReasonCode::ProposalSourceStale => "proposal_source_stale",
            ReasonCode::ProposalSourceForged => "proposal_source_forged",
            ReasonCode::NoCurrentIntention => "no_current_intention",
            ReasonCode::IntentionTerminal => "intention_terminal",
            ReasonCode::RoutineStepBlocked => "routine_step_blocked",
            ReasonCode::ReservationConflict => "reservation_conflict",
            ReasonCode::NoSleepAffordance => "no_sleep_affordance",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CheckedFactKey {
    ActionId,
    ActorId,
    BodyExclusive,
    ContainerId,
    DoorId,
    DurationTicks,
    FromPlaceId,
    ItemId,
    NeedKind,
    PipelineSlot,
    PlaceId,
    Reason,
    SleepAffordanceId,
    TargetId,
    TickCount,
    ToPlaceId,
    Unsupported(String),
}

impl CheckedFactKey {
    pub fn from_stable_key(key: impl Into<String>) -> Self {
        let key = key.into();
        match key.as_str() {
            "action_id" => Self::ActionId,
            "actor_id" => Self::ActorId,
            "body_exclusive" => Self::BodyExclusive,
            "container_id" => Self::ContainerId,
            "door_id" => Self::DoorId,
            "duration_ticks" => Self::DurationTicks,
            "from_place_id" => Self::FromPlaceId,
            "item_id" => Self::ItemId,
            "need_kind" => Self::NeedKind,
            "pipeline_slots_9_11" => Self::PipelineSlot,
            "place_id" => Self::PlaceId,
            "reason" => Self::Reason,
            "sleep_affordance_id" => Self::SleepAffordanceId,
            "target_id" => Self::TargetId,
            "ticks" => Self::TickCount,
            "to_place_id" => Self::ToPlaceId,
            _ => Self::Unsupported(key),
        }
    }

    pub fn stable_key(&self) -> &str {
        match self {
            Self::ActionId => "action_id",
            Self::ActorId => "actor_id",
            Self::BodyExclusive => "body_exclusive",
            Self::ContainerId => "container_id",
            Self::DoorId => "door_id",
            Self::DurationTicks => "duration_ticks",
            Self::FromPlaceId => "from_place_id",
            Self::ItemId => "item_id",
            Self::NeedKind => "need_kind",
            Self::PipelineSlot => "pipeline_slots_9_11",
            Self::PlaceId => "place_id",
            Self::Reason => "reason",
            Self::SleepAffordanceId => "sleep_affordance_id",
            Self::TargetId => "target_id",
            Self::TickCount => "ticks",
            Self::ToPlaceId => "to_place_id",
            Self::Unsupported(key) => key.as_str(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CheckedFactSource {
    Validation,
}

impl CheckedFactSource {
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::Validation => "validation",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CheckedFact {
    key: CheckedFactKey,
    value: String,
    source: CheckedFactSource,
}

impl CheckedFact {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: CheckedFactKey::from_stable_key(key),
            value: value.into(),
            source: CheckedFactSource::Validation,
        }
    }

    pub const fn key(&self) -> &CheckedFactKey {
        &self.key
    }

    pub fn stable_key(&self) -> &str {
        self.key.stable_key()
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub const fn source(&self) -> CheckedFactSource {
        self.source
    }

    pub fn render_pair(&self) -> String {
        format!("{}={}", self.stable_key(), self.value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationReport {
    pub validation_report_id: ValidationReportId,
    pub proposal_id: ProposalId,
    pub actor_id: Option<ActorId>,
    pub action_id: ActionId,
    pub target_ids: Vec<String>,
    pub status: ReportStatus,
    pub failed_stage: Option<crate::actions::pipeline::PipelineStage>,
    pub reason_codes: Vec<ReasonCode>,
    pub checked_facts: Vec<CheckedFact>,
    pub actor_visible_facts: Vec<CheckedFact>,
    pub debug_only_facts: Vec<CheckedFact>,
    pub actor_visible_summary: String,
    pub debug_summary: String,
    pub would_mutate: bool,
    pub event_ids: Vec<EventId>,
    pub checksum_before: Option<String>,
    pub checksum_after: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reason_code_stable_ids_match_spec() {
        assert_eq!(ReasonCode::UnknownActionId.stable_id(), "unknown_action_id");
        assert_eq!(ReasonCode::TargetReserved.stable_id(), "target_reserved");
        assert_eq!(
            ReasonCode::DoorClosedBlocksMovement.stable_id(),
            "door_closed_blocks_movement"
        );
        assert_eq!(
            ReasonCode::KnowledgePreconditionNotMet.stable_id(),
            "knowledge_precondition_not_met"
        );
        assert_eq!(
            ReasonCode::NoCurrentIntention.stable_id(),
            "no_current_intention"
        );
        assert_eq!(
            ReasonCode::ProposalSourceStale.stable_id(),
            "proposal_source_stale"
        );
        assert_eq!(
            ReasonCode::ReservationConflict.stable_id(),
            "reservation_conflict"
        );
    }

    #[test]
    fn checked_fact_preserves_typed_key_and_render_pair() {
        let fact = CheckedFact::new("actor_id", "actor_tomas");

        assert_eq!(fact.key(), &CheckedFactKey::ActorId);
        assert_eq!(fact.stable_key(), "actor_id");
        assert_eq!(fact.value(), "actor_tomas");
        assert_eq!(fact.source().stable_id(), "validation");
        assert_eq!(fact.render_pair(), "actor_id=actor_tomas");
    }

    #[test]
    fn checked_fact_key_classifies_debug_only_validator_fact() {
        let fact = CheckedFact::new("holder_known_context_hash", "hkc_hash_001");

        assert_eq!(
            fact.key(),
            &CheckedFactKey::Unsupported("holder_known_context_hash".to_string())
        );
    }
}
