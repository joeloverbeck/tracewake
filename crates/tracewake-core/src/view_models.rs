use crate::actions::report::ValidationReport;
use crate::events::{EventEnvelope, EventStream};
use crate::ids::{
    ActionId, ActorId, ContainerId, DoorId, ItemId, PlaceId, SemanticActionId, ViewModelId,
};
use crate::time::SimTick;

pub const DEBUG_EPISTEMICS_MARKER: &str = "DEBUG NON-DIEGETIC: Epistemics";

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
    pub carried_items: Vec<VisibleItem>,
    pub local_actors: Vec<VisibleActor>,
    pub semantic_actions: Vec<SemanticActionEntry>,
    pub phase3a_status: Option<Phase3AEmbodiedStatus>,
    pub last_rejection_summary: Option<String>,
    pub knowledge_context_id: Option<String>,
    pub notebook: Option<NotebookView>,
    pub debug_available: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Phase3AEmbodiedStatus {
    pub need_summaries: Vec<NeedStatusEntry>,
    pub intention_summary: Option<String>,
    pub routine_summary: Option<String>,
    pub salient_interruption: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NeedStatusEntry {
    pub kind: String,
    pub band_label: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NotebookView {
    pub viewer_actor_id: ActorId,
    pub source_bound_beliefs: Vec<NotebookBeliefEntry>,
    pub recent_observations: Vec<NotebookObservationEntry>,
    pub known_contradictions: Vec<NotebookContradictionEntry>,
    pub possible_leads: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NotebookBeliefEntry {
    pub belief_id: String,
    pub summary: String,
    pub source_summary: String,
    pub confidence_label: String,
    pub acquired_tick: u64,
    pub contradiction_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NotebookObservationEntry {
    pub observation_id: String,
    pub channel: String,
    pub summary: String,
    pub confidence_label: String,
    pub observed_tick: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NotebookContradictionEntry {
    pub contradiction_id: String,
    pub summary: String,
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
    pub source: VisibleItemSource,
    pub portable: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum VisibleItemSource {
    Place,
    Container(ContainerId),
    Carried,
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
    ActionRejection(Box<DebugActionRejectionView>),
    ProjectionRebuild(DebugProjectionRebuildView),
    ReplayReport(DebugReplayReportView),
    Epistemics(DebugEpistemicsView),
    Beliefs(DebugBeliefsView),
    Observations(DebugObservationsView),
    TruthBeliefMismatch(DebugTruthBeliefMismatchView),
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugEpistemicsView {
    pub debug_only: bool,
    pub non_diegetic_marker: String,
    pub context_mode: String,
    pub observations: Vec<DebugObservationEntry>,
    pub beliefs_by_holder: Vec<DebugHolderBeliefs>,
    pub contradictions: Vec<DebugContradictionEntry>,
    pub possession_metadata: Vec<String>,
    pub projection_summary: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugBeliefsView {
    pub debug_only: bool,
    pub non_diegetic_marker: String,
    pub holder_actor_id: ActorId,
    pub beliefs: Vec<DebugBeliefEntry>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugObservationsView {
    pub debug_only: bool,
    pub non_diegetic_marker: String,
    pub observer_actor_id: ActorId,
    pub observations: Vec<DebugObservationEntry>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugTruthBeliefMismatchView {
    pub debug_only: bool,
    pub non_diegetic_marker: String,
    pub item_id: ItemId,
    pub ground_truth_location: String,
    pub held_belief_summary: String,
    pub mismatch_summary: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DebugHolderBeliefs {
    pub holder_actor_id: ActorId,
    pub beliefs: Vec<DebugBeliefEntry>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DebugBeliefEntry {
    pub belief_id: String,
    pub proposition: String,
    pub stance: String,
    pub confidence: String,
    pub source: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DebugObservationEntry {
    pub observation_id: String,
    pub observer_actor_id: ActorId,
    pub channel: String,
    pub confidence: String,
    pub source: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DebugContradictionEntry {
    pub contradiction_id: String,
    pub holder_actor_id: ActorId,
    pub expectation_belief_id: String,
    pub observation_id: String,
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

    #[test]
    fn notebook_view_is_actor_scoped_and_source_bound() {
        let notebook = NotebookView {
            viewer_actor_id: ActorId::new("actor_tomas").unwrap(),
            source_bound_beliefs: vec![NotebookBeliefEntry {
                belief_id: "belief_tomas_missing_coin".to_string(),
                summary: "coin_stack_01 is missing from expected location".to_string(),
                source_summary: "event:event_observation".to_string(),
                confidence_label: "1000".to_string(),
                acquired_tick: 3,
                contradiction_ids: vec!["contradiction_tomas_missing_coin".to_string()],
            }],
            recent_observations: Vec::new(),
            known_contradictions: Vec::new(),
            possible_leads: vec!["Source-bound lead from belief_tomas_missing_coin".to_string()],
        };

        assert_eq!(notebook.viewer_actor_id.as_str(), "actor_tomas");
        assert!(!format!("{notebook:?}").contains("quest"));
    }

    #[test]
    fn debug_epistemics_view_is_non_diegetic_and_lists_all_holders() {
        let view = DebugEpistemicsView {
            debug_only: true,
            non_diegetic_marker: DEBUG_EPISTEMICS_MARKER.to_string(),
            context_mode: "debug".to_string(),
            observations: vec![DebugObservationEntry {
                observation_id: "obs_tomas_checked_strongbox".to_string(),
                observer_actor_id: ActorId::new("actor_tomas").unwrap(),
                channel: "touch_or_search".to_string(),
                confidence: "1000".to_string(),
                source: "event:event_observation".to_string(),
            }],
            beliefs_by_holder: vec![
                DebugHolderBeliefs {
                    holder_actor_id: ActorId::new("actor_tomas").unwrap(),
                    beliefs: vec![DebugBeliefEntry {
                        belief_id: "belief_tomas_missing_coin".to_string(),
                        proposition: "coin_stack_01 is missing from expected location".to_string(),
                        stance: "believes_true".to_string(),
                        confidence: "1000".to_string(),
                        source: "event:event_observation".to_string(),
                    }],
                },
                DebugHolderBeliefs {
                    holder_actor_id: ActorId::new("actor_elena").unwrap(),
                    beliefs: Vec::new(),
                },
            ],
            contradictions: vec![DebugContradictionEntry {
                contradiction_id: "contradiction_tomas_missing_coin".to_string(),
                holder_actor_id: ActorId::new("actor_tomas").unwrap(),
                expectation_belief_id: "belief_tomas_expected_coin".to_string(),
                observation_id: "obs_tomas_checked_strongbox".to_string(),
                summary: "expected item absent from container".to_string(),
            }],
            possession_metadata: vec!["controller_human->actor_tomas@2".to_string()],
            projection_summary: "epistemic_projection_v1".to_string(),
        };

        assert!(view.debug_only);
        assert_eq!(view.non_diegetic_marker, DEBUG_EPISTEMICS_MARKER);
        assert_eq!(view.beliefs_by_holder.len(), 2);
        assert_eq!(
            DebugViewModel::Epistemics(view.clone()),
            DebugViewModel::Epistemics(view)
        );
    }

    #[test]
    fn focused_debug_views_are_marked_non_diegetic() {
        let beliefs = DebugBeliefsView {
            debug_only: true,
            non_diegetic_marker: DEBUG_EPISTEMICS_MARKER.to_string(),
            holder_actor_id: ActorId::new("actor_tomas").unwrap(),
            beliefs: Vec::new(),
        };
        let observations = DebugObservationsView {
            debug_only: true,
            non_diegetic_marker: DEBUG_EPISTEMICS_MARKER.to_string(),
            observer_actor_id: ActorId::new("actor_tomas").unwrap(),
            observations: Vec::new(),
        };

        assert!(beliefs.debug_only);
        assert!(observations.debug_only);
        assert_eq!(beliefs.non_diegetic_marker, DEBUG_EPISTEMICS_MARKER);
        assert_eq!(observations.non_diegetic_marker, DEBUG_EPISTEMICS_MARKER);
    }

    #[test]
    fn truth_belief_mismatch_shows_truth_and_belief_side_by_side() {
        let mismatch = DebugTruthBeliefMismatchView {
            debug_only: true,
            non_diegetic_marker: DEBUG_EPISTEMICS_MARKER.to_string(),
            item_id: ItemId::new("coin_stack_01").unwrap(),
            ground_truth_location: "actor:actor_mara".to_string(),
            held_belief_summary: "coin_stack_01 is missing from expected location".to_string(),
            mismatch_summary: "truth and holder belief diverge".to_string(),
        };

        assert!(mismatch.debug_only);
        assert!(mismatch.ground_truth_location.contains("actor_mara"));
        assert!(mismatch.held_belief_summary.contains("missing"));
    }
}
