use crate::actions::report::ValidationReport;
use crate::events::{EventEnvelope, EventStream};
use crate::ids::{
    ActionId, ActorId, ContainerId, DoorId, ItemId, PlaceId, SemanticActionId, ViewModelId,
};
use crate::time::SimTick;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViewMode {
    Embodied,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EmbodiedViewModel {
    pub view_model_id: ViewModelId,
    pub mode: ViewMode,
    pub viewer_actor_id: ActorId,
    pub sim_tick: SimTick,
    pub place_id: PlaceId,
    pub place_label: String,
    pub visible_exits: Vec<VisibleExit>,
    pub visible_doors: Vec<VisibleDoor>,
    pub visible_containers: Vec<VisibleContainer>,
    pub visible_items: Vec<VisibleItem>,
    pub local_actors: Vec<VisibleActor>,
    pub semantic_actions: Vec<SemanticActionEntry>,
    pub last_rejection_summary: Option<String>,
    pub debug_available: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VisibleExit {
    pub destination_place_id: PlaceId,
    pub blocker_summary: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VisibleDoor {
    pub door_id: DoorId,
    pub endpoint_a: PlaceId,
    pub endpoint_b: PlaceId,
    pub is_open: bool,
    pub is_locked: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VisibleContainer {
    pub container_id: ContainerId,
    pub is_open: bool,
    pub is_locked: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VisibleItem {
    pub item_id: ItemId,
    pub portable: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VisibleActor {
    pub actor_id: ActorId,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemanticActionEntry {
    pub semantic_action_id: SemanticActionId,
    pub action_id: ActionId,
    pub target_ids: Vec<String>,
    pub label: String,
    pub enabled: bool,
    pub why_disabled: Option<String>,
}

impl SemanticActionEntry {
    pub fn new(
        semantic_action_id: SemanticActionId,
        action_id: ActionId,
        target_ids: Vec<String>,
        label: impl Into<String>,
        enabled: bool,
        why_disabled: Option<String>,
    ) -> Self {
        Self {
            semantic_action_id,
            action_id,
            target_ids,
            label: label.into(),
            enabled,
            why_disabled,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DebugViewModel {
    ControllerBinding(DebugControllerBindingView),
    EventLog(DebugEventLogView),
    ItemLocation(DebugItemLocationView),
    ActionRejection(DebugActionRejectionView),
    ProjectionRebuild(DebugProjectionRebuildView),
    ReplayReport(DebugReplayReportView),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugControllerBindingView {
    pub debug_only: bool,
    pub current_binding: Option<String>,
    pub binding_history: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugEventLogView {
    pub debug_only: bool,
    pub events: Vec<DebugEventSummary>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugEventSummary {
    pub stream: EventStream,
    pub stream_position: u64,
    pub global_order: u64,
    pub event_type: String,
    pub actor_or_process: Option<String>,
    pub participants: Vec<String>,
    pub checksum_after: Option<String>,
}

impl From<&EventEnvelope> for DebugEventSummary {
    fn from(event: &EventEnvelope) -> Self {
        Self {
            stream: event.stream,
            stream_position: event.stream_position,
            global_order: event.global_order,
            event_type: event.event_type.stable_id().to_string(),
            actor_or_process: event
                .actor_id
                .as_ref()
                .map(ToString::to_string)
                .or_else(|| event.process_id.as_ref().map(ToString::to_string)),
            participants: event.participants.clone(),
            checksum_after: event.checksum_after.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugItemLocationView {
    pub debug_only: bool,
    pub item_id: ItemId,
    pub location_summary: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugActionRejectionView {
    pub debug_only: bool,
    pub report: ValidationReport,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugProjectionRebuildView {
    pub debug_only: bool,
    pub summary: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugReplayReportView {
    pub debug_only: bool,
    pub summary: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semantic_action_id_is_stable_and_target_specific() {
        let entry = SemanticActionEntry::new(
            SemanticActionId::new("open.container.strongbox_tomas").unwrap(),
            ActionId::new("open").unwrap(),
            vec!["strongbox_tomas".to_string()],
            "Open strongbox",
            true,
            None,
        );

        assert_eq!(
            entry.semantic_action_id.as_str(),
            "open.container.strongbox_tomas"
        );
        assert_ne!(entry.semantic_action_id.as_str(), "0");
        assert_eq!(entry.target_ids, ["strongbox_tomas"]);
    }

    #[test]
    fn debug_and_embodied_view_models_are_distinct_types() {
        let debug = DebugViewModel::ProjectionRebuild(DebugProjectionRebuildView {
            debug_only: true,
            summary: "rebuilt".to_string(),
        });

        match debug {
            DebugViewModel::ProjectionRebuild(view) => assert!(view.debug_only),
            _ => panic!("wrong debug view variant"),
        }
    }
}
