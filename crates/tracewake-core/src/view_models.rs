use crate::actions::report::{ReasonCode, ValidationReport};
use crate::checksum::HolderKnownContextHash;
use crate::debug_capability::DebugCapability;
use crate::events::{EventEnvelope, EventStream};
use crate::ids::{
    ActionId, ActorId, ContainerId, DoorId, HolderKnownContextId, ItemId, PlaceId,
    SemanticActionId, ViewModelId,
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
    pub last_rejection_why_not: Option<WhyNotView>,
    pub holder_known_context_id: HolderKnownContextId,
    pub holder_known_context_hash: HolderKnownContextHash,
    pub holder_known_context_frontier: u64,
    pub holder_known_context_source_summary: String,
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
    pub last_cause: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WhyNotView {
    pub failure_kind: WhyNotFailureKind,
    pub actor_known_summary: String,
    pub reason_codes: Vec<String>,
    pub actor_visible_facts: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WhyNotFailureKind {
    ActorKnownUncertainty,
    GroundTruthValidationFailure,
    Access,
    ReservationOrBodyConstraint,
    AuthoredContentInvalidity,
    UnsupportedAction,
}

impl WhyNotFailureKind {
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::ActorKnownUncertainty => "actor_known_uncertainty",
            Self::GroundTruthValidationFailure => "ground_truth_validation_failure",
            Self::Access => "access",
            Self::ReservationOrBodyConstraint => "reservation_or_body_constraint",
            Self::AuthoredContentInvalidity => "authored_content_invalidity",
            Self::UnsupportedAction => "unsupported_action",
        }
    }
}

impl From<&ValidationReport> for WhyNotView {
    fn from(report: &ValidationReport) -> Self {
        Self {
            failure_kind: classify_why_not(report),
            actor_known_summary: report.actor_visible_summary.clone(),
            reason_codes: report
                .reason_codes
                .iter()
                .map(|reason| reason.stable_id().to_string())
                .collect(),
            actor_visible_facts: report
                .actor_visible_facts
                .iter()
                .map(crate::actions::CheckedFact::render_pair)
                .collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NotebookView {
    pub viewer_actor_id: ActorId,
    pub source_bound_beliefs: Vec<NotebookBeliefEntry>,
    pub recent_observations: Vec<NotebookObservationEntry>,
    pub known_contradictions: Vec<NotebookContradictionEntry>,
    pub typed_leads: Vec<NotebookLeadEntry>,
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
pub struct NotebookLeadEntry {
    pub lead_id: String,
    pub contradiction_id: String,
    pub belief_id: String,
    pub observation_id: String,
    pub source_kind: String,
    pub source_summary: String,
    pub confidence_label: String,
    pub detected_tick: u64,
    pub staleness_label: String,
    pub how_this_may_be_wrong: String,
    pub possible_next_actions: Vec<String>,
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
    pub availability: ActionAvailability,
}

impl SemanticActionEntry {
    pub fn new(
        semantic_action_id: SemanticActionId,
        action_id: ActionId,
        target_ids: Vec<String>,
        label: impl Into<String>,
        enabled: bool,
        unavailable_summary: Option<String>,
    ) -> Self {
        let availability = if enabled {
            ActionAvailability::Available
        } else {
            ActionAvailability::disabled(
                vec![ReasonCode::WorldStateMismatch],
                unavailable_summary
                    .unwrap_or_else(|| "That is not currently possible.".to_string()),
                Vec::new(),
                Vec::new(),
            )
        };
        Self::with_availability(
            semantic_action_id,
            action_id,
            target_ids,
            label,
            availability,
        )
    }

    pub fn with_availability(
        semantic_action_id: SemanticActionId,
        action_id: ActionId,
        target_ids: Vec<String>,
        label: impl Into<String>,
        availability: ActionAvailability,
    ) -> Self {
        let enabled = availability.is_available();
        Self {
            semantic_action_id,
            action_id,
            target_ids,
            label: label.into(),
            enabled,
            availability,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ActionAvailability {
    Available,
    Disabled {
        reason_codes: Vec<ReasonCode>,
        actor_safe_summary: String,
        provenance_refs: Vec<ActionAvailabilityProvenance>,
        debug_only_diagnostics: Vec<String>,
    },
}

impl ActionAvailability {
    pub const fn available() -> Self {
        Self::Available
    }

    pub fn disabled(
        reason_codes: Vec<ReasonCode>,
        actor_safe_summary: impl Into<String>,
        provenance_refs: Vec<ActionAvailabilityProvenance>,
        debug_only_diagnostics: Vec<String>,
    ) -> Self {
        Self::Disabled {
            reason_codes,
            actor_safe_summary: actor_safe_summary.into(),
            provenance_refs,
            debug_only_diagnostics,
        }
    }

    pub const fn is_available(&self) -> bool {
        matches!(self, Self::Available)
    }

    pub fn reason_codes(&self) -> &[ReasonCode] {
        match self {
            Self::Available => &[],
            Self::Disabled { reason_codes, .. } => reason_codes,
        }
    }

    pub fn actor_safe_summary(&self) -> Option<&str> {
        match self {
            Self::Available => None,
            Self::Disabled {
                actor_safe_summary, ..
            } => Some(actor_safe_summary),
        }
    }

    pub fn provenance_refs(&self) -> &[ActionAvailabilityProvenance] {
        match self {
            Self::Available => &[],
            Self::Disabled {
                provenance_refs, ..
            } => provenance_refs,
        }
    }

    pub fn debug_only_diagnostics(&self) -> &[String] {
        match self {
            Self::Available => &[],
            Self::Disabled {
                debug_only_diagnostics,
                ..
            } => debug_only_diagnostics,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ActionAvailabilityProvenance {
    pub kind: ActionAvailabilityProvenanceKind,
    pub reference: String,
}

impl ActionAvailabilityProvenance {
    pub fn new(kind: ActionAvailabilityProvenanceKind, reference: impl Into<String>) -> Self {
        Self {
            kind,
            reference: reference.into(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ActionAvailabilityProvenanceKind {
    HolderKnownContext,
    SourceEvent,
    ValidationReport,
    ValidatorFact,
}

impl ActionAvailabilityProvenanceKind {
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::HolderKnownContext => "holder_known_context",
            Self::SourceEvent => "source_event",
            Self::ValidationReport => "validation_report",
            Self::ValidatorFact => "validator_fact",
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
    debug_capability: DebugCapability,
    pub current_binding: Option<String>,
    pub binding_history: Vec<String>,
}

/// Privileged event-log view.
///
/// ```compile_fail
/// use tracewake_core::view_models::DebugEventLogView;
///
/// let _view = DebugEventLogView { events: Vec::new() };
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugEventLogView {
    debug_capability: DebugCapability,
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
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugItemLocationView {
    debug_capability: DebugCapability,
    pub item_id: ItemId,
    pub location_summary: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugActionRejectionView {
    debug_capability: DebugCapability,
    pub report: ValidationReport,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugProjectionRebuildView {
    debug_capability: DebugCapability,
    pub summary: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugReplayReportView {
    debug_capability: DebugCapability,
    pub summary: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugEpistemicsView {
    debug_capability: DebugCapability,
    pub context_mode: String,
    pub observations: Vec<DebugObservationEntry>,
    pub beliefs_by_holder: Vec<DebugHolderBeliefs>,
    pub contradictions: Vec<DebugContradictionEntry>,
    pub possession_metadata: Vec<String>,
    pub projection_summary: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugBeliefsView {
    debug_capability: DebugCapability,
    pub holder_actor_id: ActorId,
    pub beliefs: Vec<DebugBeliefEntry>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugObservationsView {
    debug_capability: DebugCapability,
    pub observer_actor_id: ActorId,
    pub observations: Vec<DebugObservationEntry>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugTruthBeliefMismatchView {
    debug_capability: DebugCapability,
    pub item_id: ItemId,
    pub ground_truth_location: String,
    pub held_belief_summary: String,
    pub mismatch_summary: String,
}

impl DebugControllerBindingView {
    pub fn new(current_binding: Option<String>, binding_history: Vec<String>) -> Self {
        Self {
            debug_capability: DebugCapability::mint(),
            current_binding,
            binding_history,
        }
    }

    pub fn debug_only(&self) -> bool {
        self.debug_capability.debug_only()
    }
}

impl DebugEventLogView {
    pub fn new(events: Vec<DebugEventSummary>) -> Self {
        Self {
            debug_capability: DebugCapability::mint(),
            events,
        }
    }

    pub fn debug_only(&self) -> bool {
        self.debug_capability.debug_only()
    }
}

impl DebugItemLocationView {
    pub fn new(item_id: ItemId, location_summary: impl Into<String>) -> Self {
        Self {
            debug_capability: DebugCapability::mint(),
            item_id,
            location_summary: location_summary.into(),
        }
    }

    pub fn debug_only(&self) -> bool {
        self.debug_capability.debug_only()
    }
}

impl DebugActionRejectionView {
    pub fn new(report: ValidationReport) -> Self {
        Self {
            debug_capability: DebugCapability::mint(),
            report,
        }
    }

    pub fn debug_only(&self) -> bool {
        self.debug_capability.debug_only()
    }
}

impl DebugProjectionRebuildView {
    pub fn new(summary: impl Into<String>) -> Self {
        Self {
            debug_capability: DebugCapability::mint(),
            summary: summary.into(),
        }
    }

    pub fn debug_only(&self) -> bool {
        self.debug_capability.debug_only()
    }
}

impl DebugReplayReportView {
    pub fn new(summary: impl Into<String>) -> Self {
        Self {
            debug_capability: DebugCapability::mint(),
            summary: summary.into(),
        }
    }

    pub fn debug_only(&self) -> bool {
        self.debug_capability.debug_only()
    }
}

impl DebugEpistemicsView {
    pub fn new(
        context_mode: impl Into<String>,
        observations: Vec<DebugObservationEntry>,
        beliefs_by_holder: Vec<DebugHolderBeliefs>,
        contradictions: Vec<DebugContradictionEntry>,
        possession_metadata: Vec<String>,
        projection_summary: impl Into<String>,
    ) -> Self {
        Self {
            debug_capability: DebugCapability::mint(),
            context_mode: context_mode.into(),
            observations,
            beliefs_by_holder,
            contradictions,
            possession_metadata,
            projection_summary: projection_summary.into(),
        }
    }

    pub fn debug_only(&self) -> bool {
        self.debug_capability.debug_only()
    }

    pub fn non_diegetic_marker(&self) -> &'static str {
        DEBUG_EPISTEMICS_MARKER
    }
}

impl DebugBeliefsView {
    pub fn new(holder_actor_id: ActorId, beliefs: Vec<DebugBeliefEntry>) -> Self {
        Self {
            debug_capability: DebugCapability::mint(),
            holder_actor_id,
            beliefs,
        }
    }

    pub fn debug_only(&self) -> bool {
        self.debug_capability.debug_only()
    }

    pub fn non_diegetic_marker(&self) -> &'static str {
        DEBUG_EPISTEMICS_MARKER
    }
}

impl DebugObservationsView {
    pub fn new(observer_actor_id: ActorId, observations: Vec<DebugObservationEntry>) -> Self {
        Self {
            debug_capability: DebugCapability::mint(),
            observer_actor_id,
            observations,
        }
    }

    pub fn debug_only(&self) -> bool {
        self.debug_capability.debug_only()
    }

    pub fn non_diegetic_marker(&self) -> &'static str {
        DEBUG_EPISTEMICS_MARKER
    }
}

impl DebugTruthBeliefMismatchView {
    pub fn new(
        item_id: ItemId,
        ground_truth_location: impl Into<String>,
        held_belief_summary: impl Into<String>,
        mismatch_summary: impl Into<String>,
    ) -> Self {
        Self {
            debug_capability: DebugCapability::mint(),
            item_id,
            ground_truth_location: ground_truth_location.into(),
            held_belief_summary: held_belief_summary.into(),
            mismatch_summary: mismatch_summary.into(),
        }
    }

    pub fn debug_only(&self) -> bool {
        self.debug_capability.debug_only()
    }

    pub fn non_diegetic_marker(&self) -> &'static str {
        DEBUG_EPISTEMICS_MARKER
    }
}

fn classify_why_not(report: &ValidationReport) -> WhyNotFailureKind {
    if report.reason_codes.iter().any(|reason| {
        matches!(
            reason,
            ReasonCode::KnowledgePreconditionNotMet
                | ReasonCode::TargetNotVisible
                | ReasonCode::TargetNotReachable
                | ReasonCode::NoCurrentIntention
                | ReasonCode::IntentionTerminal
                | ReasonCode::RoutineStepBlocked
        )
    }) {
        return WhyNotFailureKind::ActorKnownUncertainty;
    }
    if report.reason_codes.iter().any(|reason| {
        matches!(
            reason,
            ReasonCode::DoorClosedBlocksMovement
                | ReasonCode::DoorLocked
                | ReasonCode::ContainerClosed
                | ReasonCode::ContainerLocked
                | ReasonCode::DestinationNotOpen
                | ReasonCode::ActorNotAtRequiredPlace
        )
    }) {
        return WhyNotFailureKind::Access;
    }
    if report.reason_codes.iter().any(|reason| {
        matches!(
            reason,
            ReasonCode::TargetReserved | ReasonCode::ReservationConflict
        )
    }) {
        return WhyNotFailureKind::ReservationOrBodyConstraint;
    }
    if report.reason_codes.iter().any(|reason| {
        matches!(
            reason,
            ReasonCode::InvalidParameter
                | ReasonCode::ControllerUnbound
                | ReasonCode::ControllerActorMismatch
        )
    }) {
        return WhyNotFailureKind::AuthoredContentInvalidity;
    }
    if report.reason_codes.iter().any(|reason| {
        matches!(
            reason,
            ReasonCode::UnknownActionId
                | ReasonCode::PhaseUnsupportedAction
                | ReasonCode::UnsupportedTargetKind
        )
    }) {
        return WhyNotFailureKind::UnsupportedAction;
    }
    WhyNotFailureKind::GroundTruthValidationFailure
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
    use crate::actions::pipeline::PipelineStage;
    use crate::actions::{ReasonCode, ReportStatus, ValidationReport};
    use crate::ids::{ProposalId, ValidationReportId};

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
        assert!(entry.availability.is_available());
    }

    #[test]
    fn action_availability_carries_typed_reason_and_provenance() {
        let entry = SemanticActionEntry::with_availability(
            SemanticActionId::new("check.container.strongbox_tomas").unwrap(),
            ActionId::new("check_container").unwrap(),
            vec!["strongbox_tomas".to_string()],
            "Check strongbox",
            ActionAvailability::disabled(
                vec![ReasonCode::ContainerClosed],
                "The container is closed.",
                vec![ActionAvailabilityProvenance::new(
                    ActionAvailabilityProvenanceKind::HolderKnownContext,
                    "knowledge.actor_tomas.0",
                )],
                vec!["container_id=strongbox_tomas".to_string()],
            ),
        );

        assert!(!entry.enabled);
        assert_eq!(
            entry.availability.reason_codes(),
            &[ReasonCode::ContainerClosed]
        );
        assert_eq!(
            entry.availability.actor_safe_summary(),
            Some("The container is closed.")
        );
        assert_eq!(
            entry.availability.provenance_refs()[0].kind,
            ActionAvailabilityProvenanceKind::HolderKnownContext
        );
    }

    #[test]
    fn action_availability_reason_codes_do_not_depend_on_display_text() {
        let original = ActionAvailability::disabled(
            vec![ReasonCode::DoorClosedBlocksMovement],
            "The door is closed.",
            Vec::new(),
            Vec::new(),
        );
        let reworded = ActionAvailability::disabled(
            vec![ReasonCode::DoorClosedBlocksMovement],
            "You cannot pass through the closed door.",
            Vec::new(),
            Vec::new(),
        );

        assert_eq!(original.reason_codes(), reworded.reason_codes());
        assert_ne!(original.actor_safe_summary(), reworded.actor_safe_summary());
    }

    #[test]
    fn debug_and_embodied_view_models_are_distinct_types() {
        let debug = DebugViewModel::ProjectionRebuild(DebugProjectionRebuildView::new("rebuilt"));

        match debug {
            DebugViewModel::ProjectionRebuild(view) => assert!(view.debug_only()),
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
            typed_leads: vec![NotebookLeadEntry {
                lead_id: "lead.contradiction_tomas_missing_coin".to_string(),
                contradiction_id: "contradiction_tomas_missing_coin".to_string(),
                belief_id: "belief_tomas_missing_coin".to_string(),
                observation_id: "obs_tomas_checked_strongbox".to_string(),
                source_kind: "event".to_string(),
                source_summary: "event:event_observation".to_string(),
                confidence_label: "1000".to_string(),
                detected_tick: 3,
                staleness_label: "1 ticks old".to_string(),
                how_this_may_be_wrong:
                    "The item may have moved through an unobserved ordinary event.".to_string(),
                possible_next_actions: vec!["check.container.strongbox_tomas".to_string()],
                summary: "Source-bound lead from contradiction_tomas_missing_coin".to_string(),
            }],
            possible_leads: vec!["Source-bound lead from belief_tomas_missing_coin".to_string()],
        };

        assert_eq!(notebook.viewer_actor_id.as_str(), "actor_tomas");
        assert!(!format!("{notebook:?}").contains("quest"));
    }

    #[test]
    fn debug_epistemics_view_is_non_diegetic_and_lists_all_holders() {
        let view = DebugEpistemicsView::new(
            "debug",
            vec![DebugObservationEntry {
                observation_id: "obs_tomas_checked_strongbox".to_string(),
                observer_actor_id: ActorId::new("actor_tomas").unwrap(),
                channel: "touch_or_search".to_string(),
                confidence: "1000".to_string(),
                source: "event:event_observation".to_string(),
            }],
            vec![
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
            vec![DebugContradictionEntry {
                contradiction_id: "contradiction_tomas_missing_coin".to_string(),
                holder_actor_id: ActorId::new("actor_tomas").unwrap(),
                expectation_belief_id: "belief_tomas_expected_coin".to_string(),
                observation_id: "obs_tomas_checked_strongbox".to_string(),
                summary: "expected item absent from container".to_string(),
            }],
            vec!["controller_human->actor_tomas@2".to_string()],
            "epistemic_projection_v1",
        );

        assert!(view.debug_only());
        assert_eq!(view.non_diegetic_marker(), DEBUG_EPISTEMICS_MARKER);
        assert_eq!(view.beliefs_by_holder.len(), 2);
        assert_eq!(
            DebugViewModel::Epistemics(view.clone()),
            DebugViewModel::Epistemics(view)
        );
    }

    #[test]
    fn focused_debug_views_are_marked_non_diegetic() {
        let beliefs = DebugBeliefsView::new(ActorId::new("actor_tomas").unwrap(), Vec::new());
        let observations =
            DebugObservationsView::new(ActorId::new("actor_tomas").unwrap(), Vec::new());

        assert!(beliefs.debug_only());
        assert!(observations.debug_only());
        assert_eq!(beliefs.non_diegetic_marker(), DEBUG_EPISTEMICS_MARKER);
        assert_eq!(observations.non_diegetic_marker(), DEBUG_EPISTEMICS_MARKER);
    }

    #[test]
    fn truth_belief_mismatch_shows_truth_and_belief_side_by_side() {
        let mismatch = DebugTruthBeliefMismatchView::new(
            ItemId::new("coin_stack_01").unwrap(),
            "actor:actor_mara",
            "coin_stack_01 is missing from expected location",
            "truth and holder belief diverge",
        );

        assert!(mismatch.debug_only());
        assert!(mismatch.ground_truth_location.contains("actor_mara"));
        assert!(mismatch.held_belief_summary.contains("missing"));
    }

    #[test]
    fn debug_view_channel_routing_rejects_forged_non_debug_capability() {
        let routed = minted_debug_views()
            .iter()
            .map(debug_channel_route)
            .collect::<Vec<_>>();
        assert_eq!(
            routed,
            vec![
                Some("debug:controller_binding"),
                Some("debug:event_log"),
                Some("debug:item_location"),
                Some("debug:action_rejection"),
                Some("debug:projection_rebuild"),
                Some("debug:replay_report"),
                Some("debug:epistemics"),
                Some("debug:beliefs"),
                Some("debug:observations"),
                Some("debug:truth_belief_mismatch"),
            ]
        );

        for forged in forged_non_debug_views() {
            assert_eq!(
                debug_channel_route(&forged),
                None,
                "forged non-debug view must not route: {forged:?}"
            );
        }
    }

    #[test]
    fn availability_and_why_not_snapshots_preserve_typed_debug_values() {
        let availability = ActionAvailability::disabled(
            vec![ReasonCode::TargetNotReachable],
            "That target is not reachable.",
            vec![
                ActionAvailabilityProvenance::new(
                    ActionAvailabilityProvenanceKind::HolderKnownContext,
                    "hkc.actor_tomas.7",
                ),
                ActionAvailabilityProvenance::new(
                    ActionAvailabilityProvenanceKind::SourceEvent,
                    "event_visible_route",
                ),
                ActionAvailabilityProvenance::new(
                    ActionAvailabilityProvenanceKind::ValidationReport,
                    "report.move.back_room",
                ),
                ActionAvailabilityProvenance::new(
                    ActionAvailabilityProvenanceKind::ValidatorFact,
                    "door_id=door_shop_back",
                ),
            ],
            vec![
                "validator_fact:door_id=door_shop_back".to_string(),
                "debug_only:blocked_by_closed_door".to_string(),
            ],
        );

        let route_snapshot = disabled_action_debug_route(&availability);
        assert_eq!(
            route_snapshot,
            vec![
                "reason:target_not_reachable".to_string(),
                "provenance:holder_known_context:hkc.actor_tomas.7".to_string(),
                "provenance:source_event:event_visible_route".to_string(),
                "provenance:validation_report:report.move.back_room".to_string(),
                "provenance:validator_fact:door_id=door_shop_back".to_string(),
                "diagnostic:validator_fact:door_id=door_shop_back".to_string(),
                "diagnostic:debug_only:blocked_by_closed_door".to_string(),
            ]
        );

        let report = validation_report(
            "proposal_actor_known_route",
            "You do not know a reachable route.",
            vec![ReasonCode::TargetNotReachable],
        );
        let why_not = WhyNotView::from(&report);
        assert_eq!(
            why_not_channel_route(&why_not),
            "why_not:actor_known_uncertainty:target_not_reachable"
        );
    }

    #[test]
    fn why_not_view_distinguishes_actor_known_from_ground_truth_failures() {
        let actor_known = validation_report(
            "proposal_actor_known",
            "You do not know a reachable food source.",
            vec![ReasonCode::KnowledgePreconditionNotMet],
        );
        let actor_known_view = WhyNotView::from(&actor_known);

        assert_eq!(
            actor_known_view.failure_kind,
            WhyNotFailureKind::ActorKnownUncertainty
        );
        assert_eq!(
            actor_known_view.reason_codes,
            ["knowledge_precondition_not_met"]
        );
        assert_eq!(
            actor_known_view.actor_known_summary,
            "You do not know a reachable food source."
        );

        let ground_truth = validation_report(
            "proposal_world_state",
            "That is not possible from here.",
            vec![ReasonCode::WorldStateMismatch],
        );
        assert_eq!(
            WhyNotView::from(&ground_truth).failure_kind,
            WhyNotFailureKind::GroundTruthValidationFailure
        );
    }

    fn validation_report(
        proposal_id: &str,
        actor_visible_summary: &str,
        reason_codes: Vec<ReasonCode>,
    ) -> ValidationReport {
        ValidationReport {
            validation_report_id: ValidationReportId::new(format!("validation_{proposal_id}"))
                .unwrap(),
            proposal_id: ProposalId::new(proposal_id).unwrap(),
            actor_id: None,
            action_id: ActionId::new("eat").unwrap(),
            target_ids: Vec::new(),
            status: ReportStatus::Rejected,
            failed_stage: Some(PipelineStage::PhysicalPreconditionValidation),
            reason_codes,
            checked_facts: Vec::new(),
            actor_visible_facts: Vec::new(),
            debug_only_facts: Vec::new(),
            actor_visible_summary: actor_visible_summary.to_string(),
            debug_summary: "debug detail".to_string(),
            would_mutate: false,
            event_ids: Vec::new(),
            checksum_before: None,
            checksum_after: None,
        }
    }

    fn debug_channel_route(view: &DebugViewModel) -> Option<&'static str> {
        match view {
            DebugViewModel::ControllerBinding(view) if view.debug_only() => {
                Some("debug:controller_binding")
            }
            DebugViewModel::EventLog(view) if view.debug_only() => Some("debug:event_log"),
            DebugViewModel::ItemLocation(view) if view.debug_only() => Some("debug:item_location"),
            DebugViewModel::ActionRejection(view) if view.debug_only() => {
                Some("debug:action_rejection")
            }
            DebugViewModel::ProjectionRebuild(view) if view.debug_only() => {
                Some("debug:projection_rebuild")
            }
            DebugViewModel::ReplayReport(view) if view.debug_only() => Some("debug:replay_report"),
            DebugViewModel::Epistemics(view)
                if view.debug_only() && view.non_diegetic_marker() == DEBUG_EPISTEMICS_MARKER =>
            {
                Some("debug:epistemics")
            }
            DebugViewModel::Beliefs(view)
                if view.debug_only() && view.non_diegetic_marker() == DEBUG_EPISTEMICS_MARKER =>
            {
                Some("debug:beliefs")
            }
            DebugViewModel::Observations(view)
                if view.debug_only() && view.non_diegetic_marker() == DEBUG_EPISTEMICS_MARKER =>
            {
                Some("debug:observations")
            }
            DebugViewModel::TruthBeliefMismatch(view)
                if view.debug_only() && view.non_diegetic_marker() == DEBUG_EPISTEMICS_MARKER =>
            {
                Some("debug:truth_belief_mismatch")
            }
            _ => None,
        }
    }

    fn minted_debug_views() -> Vec<DebugViewModel> {
        vec![
            DebugViewModel::ControllerBinding(DebugControllerBindingView::new(
                Some("controller_human->actor_tomas".to_string()),
                vec!["bound@0".to_string()],
            )),
            DebugViewModel::EventLog(DebugEventLogView::new(vec![DebugEventSummary {
                stream: EventStream::World,
                stream_position: 0,
                global_order: 0,
                event_type: "actor_waited".to_string(),
                actor_or_process: Some("actor_tomas".to_string()),
                participants: vec!["actor_tomas".to_string()],
            }])),
            DebugViewModel::ItemLocation(DebugItemLocationView::new(
                ItemId::new("coin_stack_01").unwrap(),
                "container:strongbox_tomas",
            )),
            DebugViewModel::ActionRejection(Box::new(DebugActionRejectionView::new(
                validation_report(
                    "proposal_debug_rejection",
                    "The door is closed.",
                    vec![ReasonCode::DoorClosedBlocksMovement],
                ),
            ))),
            DebugViewModel::ProjectionRebuild(DebugProjectionRebuildView::new(
                "projection rebuild matched",
            )),
            DebugViewModel::ReplayReport(DebugReplayReportView::new("replay matched")),
            DebugViewModel::Epistemics(DebugEpistemicsView::new(
                "debug",
                Vec::new(),
                Vec::new(),
                Vec::new(),
                vec!["controller_human->actor_tomas@2".to_string()],
                "epistemic_projection_v1",
            )),
            DebugViewModel::Beliefs(DebugBeliefsView::new(
                ActorId::new("actor_tomas").unwrap(),
                Vec::new(),
            )),
            DebugViewModel::Observations(DebugObservationsView::new(
                ActorId::new("actor_tomas").unwrap(),
                Vec::new(),
            )),
            DebugViewModel::TruthBeliefMismatch(DebugTruthBeliefMismatchView::new(
                ItemId::new("coin_stack_01").unwrap(),
                "container:strongbox_tomas",
                "holder believes coin is missing",
                "truth and belief diverge",
            )),
        ]
    }

    fn forged_non_debug_views() -> Vec<DebugViewModel> {
        let non_debug = DebugCapability::test_non_debug;
        vec![
            DebugViewModel::ControllerBinding(DebugControllerBindingView {
                debug_capability: non_debug(),
                current_binding: None,
                binding_history: Vec::new(),
            }),
            DebugViewModel::EventLog(DebugEventLogView {
                debug_capability: non_debug(),
                events: Vec::new(),
            }),
            DebugViewModel::ItemLocation(DebugItemLocationView {
                debug_capability: non_debug(),
                item_id: ItemId::new("coin_stack_01").unwrap(),
                location_summary: "hidden".to_string(),
            }),
            DebugViewModel::ActionRejection(Box::new(DebugActionRejectionView {
                debug_capability: non_debug(),
                report: validation_report(
                    "proposal_forged_debug_rejection",
                    "debug only",
                    vec![ReasonCode::WorldStateMismatch],
                ),
            })),
            DebugViewModel::ProjectionRebuild(DebugProjectionRebuildView {
                debug_capability: non_debug(),
                summary: "forged".to_string(),
            }),
            DebugViewModel::ReplayReport(DebugReplayReportView {
                debug_capability: non_debug(),
                summary: "forged".to_string(),
            }),
            DebugViewModel::Epistemics(DebugEpistemicsView {
                debug_capability: non_debug(),
                context_mode: "debug".to_string(),
                observations: Vec::new(),
                beliefs_by_holder: Vec::new(),
                contradictions: Vec::new(),
                possession_metadata: Vec::new(),
                projection_summary: "forged".to_string(),
            }),
            DebugViewModel::Beliefs(DebugBeliefsView {
                debug_capability: non_debug(),
                holder_actor_id: ActorId::new("actor_tomas").unwrap(),
                beliefs: Vec::new(),
            }),
            DebugViewModel::Observations(DebugObservationsView {
                debug_capability: non_debug(),
                observer_actor_id: ActorId::new("actor_tomas").unwrap(),
                observations: Vec::new(),
            }),
            DebugViewModel::TruthBeliefMismatch(DebugTruthBeliefMismatchView {
                debug_capability: non_debug(),
                item_id: ItemId::new("coin_stack_01").unwrap(),
                ground_truth_location: "container:strongbox_tomas".to_string(),
                held_belief_summary: "holder believes coin is missing".to_string(),
                mismatch_summary: "truth and belief diverge".to_string(),
            }),
        ]
    }

    fn disabled_action_debug_route(availability: &ActionAvailability) -> Vec<String> {
        let mut routed = availability
            .reason_codes()
            .iter()
            .map(|reason| format!("reason:{}", reason.stable_id()))
            .collect::<Vec<_>>();
        routed.extend(availability.provenance_refs().iter().map(|provenance| {
            format!(
                "provenance:{}:{}",
                provenance.kind.stable_id(),
                provenance.reference
            )
        }));
        routed.extend(
            availability
                .debug_only_diagnostics()
                .iter()
                .map(|diagnostic| format!("diagnostic:{diagnostic}")),
        );
        routed
    }

    fn why_not_channel_route(why_not: &WhyNotView) -> String {
        format!(
            "why_not:{}:{}",
            why_not.failure_kind.stable_id(),
            why_not.reason_codes.join(",")
        )
    }
}
