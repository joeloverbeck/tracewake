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
    NoCurrentIntention,
    IntentionTerminal,
    RoutineStepBlocked,
    ReservationConflict,
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
            ReasonCode::NoCurrentIntention => "no_current_intention",
            ReasonCode::IntentionTerminal => "intention_terminal",
            ReasonCode::RoutineStepBlocked => "routine_step_blocked",
            ReasonCode::ReservationConflict => "reservation_conflict",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CheckedFact {
    pub key: String,
    pub value: String,
}

impl CheckedFact {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
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
            ReasonCode::ReservationConflict.stable_id(),
            "reservation_conflict"
        );
    }
}
