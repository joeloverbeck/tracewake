use std::fmt;

use crate::agent::candidate::CandidateGoal;
use crate::agent::intention::Intention;
use crate::agent::need::{NeedKind, NeedPressure};
use crate::agent::routine::RoutineStep;
use crate::checksum::{compute_holder_known_context_hash, HolderKnownContextHash};
use crate::ids::{
    ActorId, CandidateGoalId, DecisionTraceId, IntentionId, RoutineExecutionId, RoutineTemplateId,
    SemanticActionId, StuckDiagnosticId,
};
use crate::time::SimTick;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DecisionOutcome {
    Continued,
    Switched,
    Waited,
    Replanned,
    Failed,
    Completed,
}

impl DecisionOutcome {
    pub const fn stable_id(self) -> &'static str {
        match self {
            DecisionOutcome::Continued => "continued",
            DecisionOutcome::Switched => "switched",
            DecisionOutcome::Waited => "waited",
            DecisionOutcome::Replanned => "replanned",
            DecisionOutcome::Failed => "failed",
            DecisionOutcome::Completed => "completed",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "continued" => Some(Self::Continued),
            "switched" => Some(Self::Switched),
            "waited" => Some(Self::Waited),
            "replanned" => Some(Self::Replanned),
            "failed" => Some(Self::Failed),
            "completed" => Some(Self::Completed),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HiddenTruthAudit {
    pub actor_known_only: bool,
    pub notes: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ResponsibleLayer {
    CandidateGeneration,
    IntentionLifecycle,
    MethodSelection,
    LocalPlanning,
    ProposalConstruction,
    Scheduler,
    ActionValidation,
    Projection,
    ViewModel,
    TestOracle,
    DebugQuarantine,
}

impl ResponsibleLayer {
    pub const ALL: [Self; 11] = [
        Self::CandidateGeneration,
        Self::IntentionLifecycle,
        Self::MethodSelection,
        Self::LocalPlanning,
        Self::ProposalConstruction,
        Self::Scheduler,
        Self::ActionValidation,
        Self::Projection,
        Self::ViewModel,
        Self::TestOracle,
        Self::DebugQuarantine,
    ];

    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::CandidateGeneration => "candidate_generation",
            Self::IntentionLifecycle => "intention_lifecycle",
            Self::MethodSelection => "method_selection",
            Self::LocalPlanning => "local_planning",
            Self::ProposalConstruction => "proposal_construction",
            Self::Scheduler => "scheduler",
            Self::ActionValidation => "action_validation",
            Self::Projection => "projection",
            Self::ViewModel => "view_model",
            Self::TestOracle => "test_oracle",
            Self::DebugQuarantine => "debug_quarantine",
        }
    }

    pub fn parse(value: &str) -> Result<Self, DiagnosticFieldParseError> {
        match value {
            "candidate_generation" => Ok(Self::CandidateGeneration),
            "intention_lifecycle" => Ok(Self::IntentionLifecycle),
            "method_selection" => Ok(Self::MethodSelection),
            "local_planning" => Ok(Self::LocalPlanning),
            "proposal_construction" => Ok(Self::ProposalConstruction),
            "scheduler" => Ok(Self::Scheduler),
            "action_validation" => Ok(Self::ActionValidation),
            "projection" => Ok(Self::Projection),
            "view_model" => Ok(Self::ViewModel),
            "test_oracle" => Ok(Self::TestOracle),
            "debug_quarantine" => Ok(Self::DebugQuarantine),
            _ => Err(DiagnosticFieldParseError::InvalidResponsibleLayer),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlockerCode {
    None,
    Physical,
    Access,
    Knowledge,
    Resource,
    SocialNormPlaceholder,
    SchedulingReservation,
    UnsupportedAction,
    PlannerBudgetExhausted,
    FixtureAuthoringError,
    NoApplicableCandidate,
    NoApplicableMethod,
    EmptyLocalPlan,
    LocalPlanFailed,
    HiddenTruthInput,
    ProvenanceDangling,
    ProvenanceClassMismatch,
    ProvenanceWitnessMismatch,
}

impl BlockerCode {
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Physical => "physical",
            Self::Access => "access",
            Self::Knowledge => "knowledge",
            Self::Resource => "resource",
            Self::SocialNormPlaceholder => "social_norm_placeholder",
            Self::SchedulingReservation => "scheduling_reservation",
            Self::UnsupportedAction => "unsupported_action",
            Self::PlannerBudgetExhausted => "planner_budget_exhausted",
            Self::FixtureAuthoringError => "fixture_authoring_error",
            Self::NoApplicableCandidate => "no_applicable_candidate",
            Self::NoApplicableMethod => "no_applicable_method",
            Self::EmptyLocalPlan => "empty_local_plan",
            Self::LocalPlanFailed => "local_plan_failed",
            Self::HiddenTruthInput => "hidden_truth_input",
            Self::ProvenanceDangling => "provenance_dangling",
            Self::ProvenanceClassMismatch => "provenance_class_mismatch",
            Self::ProvenanceWitnessMismatch => "provenance_witness_mismatch",
        }
    }

    pub fn parse(value: &str) -> Result<Self, DiagnosticFieldParseError> {
        match value {
            "none" => Ok(Self::None),
            "physical" => Ok(Self::Physical),
            "access" => Ok(Self::Access),
            "knowledge" => Ok(Self::Knowledge),
            "resource" => Ok(Self::Resource),
            "social_norm_placeholder" => Ok(Self::SocialNormPlaceholder),
            "scheduling_reservation" => Ok(Self::SchedulingReservation),
            "unsupported_action" => Ok(Self::UnsupportedAction),
            "planner_budget_exhausted" => Ok(Self::PlannerBudgetExhausted),
            "fixture_authoring_error" => Ok(Self::FixtureAuthoringError),
            "no_applicable_candidate" => Ok(Self::NoApplicableCandidate),
            "no_applicable_method" => Ok(Self::NoApplicableMethod),
            "empty_local_plan" => Ok(Self::EmptyLocalPlan),
            "local_plan_failed" => Ok(Self::LocalPlanFailed),
            "hidden_truth_input" => Ok(Self::HiddenTruthInput),
            "provenance_dangling" => Ok(Self::ProvenanceDangling),
            "provenance_class_mismatch" => Ok(Self::ProvenanceClassMismatch),
            "provenance_witness_mismatch" => Ok(Self::ProvenanceWitnessMismatch),
            _ => Err(DiagnosticFieldParseError::InvalidBlockerCode),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypedDiagnosticFields {
    pub responsible_layer: ResponsibleLayer,
    pub blocker_code: BlockerCode,
    pub input_source: String,
    pub actual_source: String,
    pub hidden_truth_referenced: bool,
    pub remediation_hint: String,
}

impl TypedDiagnosticFields {
    pub fn decision_default(hidden_truth_referenced: bool) -> Self {
        Self {
            responsible_layer: ResponsibleLayer::CandidateGeneration,
            blocker_code: BlockerCode::None,
            input_source: "actor_known_context".to_string(),
            actual_source: "actor_decision_transaction".to_string(),
            hidden_truth_referenced,
            remediation_hint: "inspect decision trace typed fields".to_string(),
        }
    }

    pub fn from_decision_outcome(outcome: DecisionOutcome, hidden_truth_referenced: bool) -> Self {
        let (responsible_layer, blocker_code, remediation_hint) = match outcome {
            DecisionOutcome::Failed => (
                ResponsibleLayer::LocalPlanning,
                BlockerCode::LocalPlanFailed,
                "inspect failed decision trace outcome and rejected items",
            ),
            DecisionOutcome::Waited => (
                ResponsibleLayer::MethodSelection,
                BlockerCode::NoApplicableMethod,
                "inspect waited decision trace outcome and fallback reason",
            ),
            DecisionOutcome::Replanned => (
                ResponsibleLayer::IntentionLifecycle,
                BlockerCode::NoApplicableCandidate,
                "inspect replanned decision trace outcome and candidate rejection",
            ),
            DecisionOutcome::Continued | DecisionOutcome::Switched | DecisionOutcome::Completed => {
                (
                    ResponsibleLayer::CandidateGeneration,
                    BlockerCode::None,
                    "inspect decision trace outcome and selected candidate",
                )
            }
        };
        Self {
            responsible_layer,
            blocker_code,
            input_source: "actor_known_context".to_string(),
            actual_source: format!("decision_outcome:{}", outcome.stable_id()),
            hidden_truth_referenced,
            remediation_hint: remediation_hint.to_string(),
        }
    }

    pub fn stuck_default() -> Self {
        Self {
            responsible_layer: ResponsibleLayer::Scheduler,
            blocker_code: BlockerCode::SchedulingReservation,
            input_source: "legacy_event_payload".to_string(),
            actual_source: "legacy_event_payload".to_string(),
            hidden_truth_referenced: false,
            remediation_hint: "legacy diagnostic had no typed fields".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiagnosticFieldParseError {
    InvalidResponsibleLayer,
    InvalidBlockerCode,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DecisionTraceRecord {
    pub trace_id: DecisionTraceId,
    pub actor_id: ActorId,
    pub window_start_tick: SimTick,
    pub window_end_tick: SimTick,
    pub outcome: DecisionOutcome,
    pub candidate_goal_count: usize,
    pub actor_known_context_hash: Option<HolderKnownContextHash>,
    pub actor_known_inputs: Vec<String>,
    pub hidden_truth_audit_result: HiddenTruthAudit,
    pub typed_diagnostic: TypedDiagnosticFields,
}

impl DecisionTraceRecord {
    pub fn from_trace(trace: &DecisionTrace) -> Self {
        let actor_known_inputs = trace.beliefs_perceptions_known_places_used.clone();
        let actor_known_context_hash =
            compute_holder_known_context_hash(actor_known_inputs.clone()).hash;
        Self {
            trace_id: trace.trace_id.clone(),
            actor_id: trace.actor_id.clone(),
            window_start_tick: trace.window_start_tick,
            window_end_tick: trace.window_end_tick,
            outcome: trace.outcome,
            candidate_goal_count: trace.candidate_goals_considered.len(),
            actor_known_context_hash: Some(actor_known_context_hash),
            actor_known_inputs,
            hidden_truth_audit_result: trace.hidden_truth_audit_result.clone(),
            typed_diagnostic: TypedDiagnosticFields::from_decision_outcome(
                trace.outcome,
                !trace.hidden_truth_audit_result.actor_known_only,
            ),
        }
    }

    pub fn serialize_canonical(&self) -> String {
        format!(
            "decision_trace_v1|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.trace_id.serialize_canonical(),
            self.actor_id.serialize_canonical(),
            self.window_start_tick.value(),
            self.window_end_tick.value(),
            self.outcome.stable_id(),
            self.candidate_goal_count,
            self.actor_known_context_hash
                .as_ref()
                .map(HolderKnownContextHash::as_str)
                .unwrap_or("-"),
            encode_text_payload(&self.actor_known_inputs.join("\n")),
            encode_bool(self.hidden_truth_audit_result.actor_known_only),
            encode_text_payload(&self.hidden_truth_audit_result.notes),
            self.typed_diagnostic.responsible_layer.stable_id(),
            self.typed_diagnostic.blocker_code.stable_id(),
            encode_text_payload(&self.typed_diagnostic.input_source),
            encode_text_payload(&self.typed_diagnostic.actual_source),
            encode_bool(self.typed_diagnostic.hidden_truth_referenced),
            encode_text_payload(&self.typed_diagnostic.remediation_hint)
        )
    }

    pub fn deserialize_canonical(value: &[u8]) -> Result<Self, DecisionTraceRecordParseError> {
        let value =
            std::str::from_utf8(value).map_err(|_| DecisionTraceRecordParseError::InvalidUtf8)?;
        let fields: Vec<_> = value.split('|').collect();
        if !matches!(fields.len(), 9 | 11 | 15 | 17) || fields[0] != "decision_trace_v1" {
            return Err(DecisionTraceRecordParseError::InvalidShape);
        }
        let (actor_known_context_hash, actor_known_inputs, audit_index, typed_index) =
            if matches!(fields.len(), 11 | 17) {
                let actor_known_inputs = decode_text_payload(fields[8])?
                    .lines()
                    .filter(|line| !line.is_empty())
                    .map(ToString::to_string)
                    .collect::<Vec<_>>();
                let actor_known_context_hash =
                    compute_holder_known_context_hash(actor_known_inputs.clone()).hash;
                // This checks trace serialization integrity. Replay derivability from the event
                // log is enforced by the replay context-hash rebuild gate.
                if fields[7] != actor_known_context_hash.as_str() {
                    return Err(DecisionTraceRecordParseError::InvalidContextHash);
                }
                (
                    Some(actor_known_context_hash),
                    actor_known_inputs,
                    9,
                    (fields.len() == 17).then_some(11),
                )
            } else {
                (None, Vec::new(), 7, (fields.len() == 15).then_some(9))
            };
        let actor_known_only =
            decode_bool(fields[audit_index]).ok_or(DecisionTraceRecordParseError::InvalidBool)?;
        Ok(Self {
            trace_id: DecisionTraceId::new(fields[1])
                .map_err(DecisionTraceRecordParseError::InvalidId)?,
            actor_id: ActorId::new(fields[2]).map_err(DecisionTraceRecordParseError::InvalidId)?,
            window_start_tick: SimTick::new(
                fields[3]
                    .parse()
                    .map_err(|_| DecisionTraceRecordParseError::InvalidTick)?,
            ),
            window_end_tick: SimTick::new(
                fields[4]
                    .parse()
                    .map_err(|_| DecisionTraceRecordParseError::InvalidTick)?,
            ),
            outcome: DecisionOutcome::parse(fields[5])
                .ok_or(DecisionTraceRecordParseError::InvalidOutcome)?,
            candidate_goal_count: fields[6]
                .parse()
                .map_err(|_| DecisionTraceRecordParseError::InvalidCount)?,
            actor_known_context_hash,
            actor_known_inputs,
            hidden_truth_audit_result: HiddenTruthAudit {
                actor_known_only,
                notes: decode_text_payload(fields[audit_index + 1])?,
            },
            typed_diagnostic: if let Some(typed_index) = typed_index {
                TypedDiagnosticFields {
                    responsible_layer: ResponsibleLayer::parse(fields[typed_index])?,
                    blocker_code: BlockerCode::parse(fields[typed_index + 1])?,
                    input_source: decode_text_payload(fields[typed_index + 2])?,
                    actual_source: decode_text_payload(fields[typed_index + 3])?,
                    hidden_truth_referenced: decode_bool(fields[typed_index + 4])
                        .ok_or(DecisionTraceRecordParseError::InvalidBool)?,
                    remediation_hint: decode_text_payload(fields[typed_index + 5])?,
                }
            } else {
                TypedDiagnosticFields::from_decision_outcome(
                    DecisionOutcome::parse(fields[5])
                        .ok_or(DecisionTraceRecordParseError::InvalidOutcome)?,
                    !actor_known_only,
                )
            },
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecisionTraceRecordParseError {
    InvalidUtf8,
    InvalidShape,
    InvalidTick,
    InvalidOutcome,
    InvalidCount,
    InvalidBool,
    InvalidTextPayload,
    InvalidContextHash,
    InvalidId(crate::ids::IdError),
    InvalidResponsibleLayer,
    InvalidBlockerCode,
}

impl From<DiagnosticFieldParseError> for DecisionTraceRecordParseError {
    fn from(value: DiagnosticFieldParseError) -> Self {
        match value {
            DiagnosticFieldParseError::InvalidResponsibleLayer => Self::InvalidResponsibleLayer,
            DiagnosticFieldParseError::InvalidBlockerCode => Self::InvalidBlockerCode,
        }
    }
}

impl From<StuckDiagnosticParseError> for DecisionTraceRecordParseError {
    fn from(value: StuckDiagnosticParseError) -> Self {
        match value {
            StuckDiagnosticParseError::InvalidTextPayload
            | StuckDiagnosticParseError::InvalidUtf8 => Self::InvalidTextPayload,
            _ => Self::InvalidShape,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RejectedDecisionItem {
    pub stable_ref: String,
    pub reason: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DecisionTrace {
    pub trace_id: DecisionTraceId,
    pub actor_id: ActorId,
    pub window_start_tick: SimTick,
    pub window_end_tick: SimTick,
    pub active_needs: Vec<NeedPressure>,
    pub current_intention_before_decision: Option<Intention>,
    pub candidate_goals_considered: Vec<CandidateGoal>,
    pub selected_goal_id: Option<CandidateGoalId>,
    pub selected_method_id: Option<RoutineTemplateId>,
    pub routine_execution_id: Option<RoutineExecutionId>,
    pub rejected_goals: Vec<RejectedDecisionItem>,
    pub rejected_methods: Vec<RejectedDecisionItem>,
    pub rejected_actions: Vec<RejectedDecisionItem>,
    pub beliefs_perceptions_known_places_used: Vec<String>,
    pub action_proposal_attempted: Option<SemanticActionId>,
    pub action_validation_result: Option<String>,
    pub fallback_considered: Option<String>,
    pub hidden_truth_audit_result: HiddenTruthAudit,
    pub outcome: DecisionOutcome,
    pub debug_only_details: String,
}

impl DecisionTrace {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        trace_id: DecisionTraceId,
        actor_id: ActorId,
        window_start_tick: SimTick,
        window_end_tick: SimTick,
        active_needs: Vec<NeedPressure>,
        current_intention_before_decision: Option<Intention>,
        candidate_goals_considered: Vec<CandidateGoal>,
        selected_goal_id: Option<CandidateGoalId>,
        selected_method_id: Option<RoutineTemplateId>,
        routine_execution_id: Option<RoutineExecutionId>,
        rejected_goals: Vec<RejectedDecisionItem>,
        rejected_methods: Vec<RejectedDecisionItem>,
        rejected_actions: Vec<RejectedDecisionItem>,
        beliefs_perceptions_known_places_used: Vec<String>,
        action_proposal_attempted: Option<SemanticActionId>,
        action_validation_result: Option<String>,
        fallback_considered: Option<String>,
        hidden_truth_audit_result: HiddenTruthAudit,
        outcome: DecisionOutcome,
        debug_only_details: impl Into<String>,
    ) -> Self {
        Self {
            trace_id,
            actor_id,
            window_start_tick,
            window_end_tick,
            active_needs,
            current_intention_before_decision,
            candidate_goals_considered,
            selected_goal_id,
            selected_method_id,
            routine_execution_id,
            rejected_goals,
            rejected_methods,
            rejected_actions,
            beliefs_perceptions_known_places_used,
            action_proposal_attempted,
            action_validation_result,
            fallback_considered,
            hidden_truth_audit_result,
            outcome,
            debug_only_details: debug_only_details.into(),
        }
    }

    pub fn render_summary(&self) -> String {
        format!(
            "trace={} actor={} outcome={} candidates={} actor_known_only={}",
            self.trace_id.serialize_canonical(),
            self.actor_id.serialize_canonical(),
            self.outcome.stable_id(),
            self.candidate_goals_considered.len(),
            self.hidden_truth_audit_result.actor_known_only
        )
    }

    pub fn serialize_canonical(&self) -> String {
        format!(
            "decision_trace_v1|{}|{}|{}|{}|{}|{}|{}|{}",
            self.trace_id.serialize_canonical(),
            self.actor_id.serialize_canonical(),
            self.window_start_tick.value(),
            self.window_end_tick.value(),
            self.outcome.stable_id(),
            self.candidate_goals_considered.len(),
            encode_bool(self.hidden_truth_audit_result.actor_known_only),
            encode_text_payload(&self.hidden_truth_audit_result.notes)
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlockerCategory {
    Physical,
    Access,
    Knowledge,
    Resource,
    SocialNormPlaceholder,
    SchedulingReservation,
    UnsupportedAction,
    PlannerBudgetExhausted,
    FixtureAuthoringError,
}

impl BlockerCategory {
    pub const ALL: [Self; 9] = [
        Self::Physical,
        Self::Access,
        Self::Knowledge,
        Self::Resource,
        Self::SocialNormPlaceholder,
        Self::SchedulingReservation,
        Self::UnsupportedAction,
        Self::PlannerBudgetExhausted,
        Self::FixtureAuthoringError,
    ];

    pub const fn stable_id(self) -> &'static str {
        match self {
            BlockerCategory::Physical => "physical",
            BlockerCategory::Access => "access",
            BlockerCategory::Knowledge => "knowledge",
            BlockerCategory::Resource => "resource",
            BlockerCategory::SocialNormPlaceholder => "social_norm_placeholder",
            BlockerCategory::SchedulingReservation => "scheduling_reservation",
            BlockerCategory::UnsupportedAction => "unsupported_action",
            BlockerCategory::PlannerBudgetExhausted => "planner_budget_exhausted",
            BlockerCategory::FixtureAuthoringError => "fixture_authoring_error",
        }
    }

    fn parse(value: &str) -> Result<Self, StuckDiagnosticParseError> {
        match value {
            "physical" => Ok(Self::Physical),
            "access" => Ok(Self::Access),
            "knowledge" => Ok(Self::Knowledge),
            "resource" => Ok(Self::Resource),
            "social_norm_placeholder" => Ok(Self::SocialNormPlaceholder),
            "scheduling_reservation" => Ok(Self::SchedulingReservation),
            "unsupported_action" => Ok(Self::UnsupportedAction),
            "planner_budget_exhausted" => Ok(Self::PlannerBudgetExhausted),
            "fixture_authoring_error" => Ok(Self::FixtureAuthoringError),
            _ => Err(StuckDiagnosticParseError::InvalidBlockerCategory),
        }
    }

    pub const fn blocker_code(self) -> BlockerCode {
        match self {
            Self::Physical => BlockerCode::Physical,
            Self::Access => BlockerCode::Access,
            Self::Knowledge => BlockerCode::Knowledge,
            Self::Resource => BlockerCode::Resource,
            Self::SocialNormPlaceholder => BlockerCode::SocialNormPlaceholder,
            Self::SchedulingReservation => BlockerCode::SchedulingReservation,
            Self::UnsupportedAction => BlockerCode::UnsupportedAction,
            Self::PlannerBudgetExhausted => BlockerCode::PlannerBudgetExhausted,
            Self::FixtureAuthoringError => BlockerCode::FixtureAuthoringError,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StuckResultingStatus {
    Idle,
    Waiting,
    Replanning,
    Failed,
    Suspended,
    Completed,
}

impl StuckResultingStatus {
    pub const fn stable_id(self) -> &'static str {
        match self {
            StuckResultingStatus::Idle => "idle",
            StuckResultingStatus::Waiting => "waiting",
            StuckResultingStatus::Replanning => "replanning",
            StuckResultingStatus::Failed => "failed",
            StuckResultingStatus::Suspended => "suspended",
            StuckResultingStatus::Completed => "completed",
        }
    }

    fn parse(value: &str) -> Result<Self, StuckDiagnosticParseError> {
        match value {
            "idle" => Ok(Self::Idle),
            "waiting" => Ok(Self::Waiting),
            "replanning" => Ok(Self::Replanning),
            "failed" => Ok(Self::Failed),
            "suspended" => Ok(Self::Suspended),
            "completed" => Ok(Self::Completed),
            _ => Err(StuckDiagnosticParseError::InvalidResultingStatus),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StuckDiagnostic {
    pub diagnostic_id: StuckDiagnosticId,
    pub actor_id: ActorId,
    pub window_start_tick: SimTick,
    pub window_end_tick: SimTick,
    pub active_need: Option<NeedKind>,
    pub active_goal_id: Option<CandidateGoalId>,
    pub active_intention_id: Option<IntentionId>,
    pub routine_template_id: Option<RoutineTemplateId>,
    pub routine_execution_id: Option<RoutineExecutionId>,
    pub routine_step: Option<RoutineStep>,
    pub attempted_action: Option<SemanticActionId>,
    pub blocker_category: BlockerCategory,
    pub concrete_blocker: String,
    pub actor_known_explanation: String,
    pub debug_only_details: String,
    pub retry_abandon_fallback_outcome: String,
    pub resulting_status: StuckResultingStatus,
    pub typed_diagnostic: TypedDiagnosticFields,
}

pub type StuckDiagnosticRecord = StuckDiagnostic;

impl StuckDiagnostic {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        diagnostic_id: StuckDiagnosticId,
        actor_id: ActorId,
        window_start_tick: SimTick,
        window_end_tick: SimTick,
        active_need: Option<NeedKind>,
        active_goal_id: Option<CandidateGoalId>,
        active_intention_id: Option<IntentionId>,
        routine_template_id: Option<RoutineTemplateId>,
        routine_execution_id: Option<RoutineExecutionId>,
        routine_step: Option<RoutineStep>,
        attempted_action: Option<SemanticActionId>,
        blocker_category: BlockerCategory,
        concrete_blocker: impl Into<String>,
        actor_known_explanation: impl Into<String>,
        debug_only_details: impl Into<String>,
        retry_abandon_fallback_outcome: impl Into<String>,
        resulting_status: StuckResultingStatus,
    ) -> Self {
        Self {
            diagnostic_id,
            actor_id,
            window_start_tick,
            window_end_tick,
            active_need,
            active_goal_id,
            active_intention_id,
            routine_template_id,
            routine_execution_id,
            routine_step,
            attempted_action,
            blocker_category,
            concrete_blocker: concrete_blocker.into(),
            actor_known_explanation: actor_known_explanation.into(),
            debug_only_details: debug_only_details.into(),
            retry_abandon_fallback_outcome: retry_abandon_fallback_outcome.into(),
            resulting_status,
            typed_diagnostic: TypedDiagnosticFields {
                responsible_layer: ResponsibleLayer::LocalPlanning,
                blocker_code: blocker_category.blocker_code(),
                input_source: "actor_known_context".to_string(),
                actual_source: "actor_decision_transaction".to_string(),
                hidden_truth_referenced: false,
                remediation_hint: "inspect stuck diagnostic typed fields".to_string(),
            },
        }
    }

    pub fn with_typed_diagnostic(mut self, typed_diagnostic: TypedDiagnosticFields) -> Self {
        self.typed_diagnostic = typed_diagnostic;
        self
    }

    pub fn serialize_canonical(&self) -> String {
        format!(
            "stuck_diagnostic_v1|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.diagnostic_id.serialize_canonical(),
            self.actor_id.serialize_canonical(),
            self.window_start_tick.value(),
            self.window_end_tick.value(),
            encode_opt_need(self.active_need),
            encode_opt_id(self.active_goal_id.as_ref().map(CandidateGoalId::as_str)),
            encode_opt_id(self.active_intention_id.as_ref().map(IntentionId::as_str)),
            encode_opt_id(
                self.routine_template_id
                    .as_ref()
                    .map(RoutineTemplateId::as_str)
            ),
            encode_opt_id(
                self.routine_execution_id
                    .as_ref()
                    .map(RoutineExecutionId::as_str)
            ),
            encode_opt_text(
                self.routine_step
                    .as_ref()
                    .map(RoutineStep::serialize_canonical)
                    .as_deref()
            ),
            encode_opt_id(self.attempted_action.as_ref().map(SemanticActionId::as_str)),
            self.blocker_category.stable_id(),
            encode_text_payload(&self.concrete_blocker),
            encode_text_payload(&self.actor_known_explanation),
            encode_text_payload(&self.debug_only_details),
            encode_text_payload(&self.retry_abandon_fallback_outcome),
            self.resulting_status.stable_id(),
            self.typed_diagnostic.responsible_layer.stable_id(),
            self.typed_diagnostic.blocker_code.stable_id(),
            encode_text_payload(&self.typed_diagnostic.input_source),
            encode_text_payload(&self.typed_diagnostic.actual_source),
            encode_bool(self.typed_diagnostic.hidden_truth_referenced),
            encode_text_payload(&self.typed_diagnostic.remediation_hint),
        )
    }

    pub fn serialize_canonical_bytes(&self) -> Vec<u8> {
        self.serialize_canonical().into_bytes()
    }

    pub fn deserialize_canonical(value: &[u8]) -> Result<Self, StuckDiagnosticParseError> {
        let value =
            std::str::from_utf8(value).map_err(|_| StuckDiagnosticParseError::InvalidUtf8)?;
        let fields: Vec<_> = value.split('|').collect();
        if !matches!(fields.len(), 18 | 24) || fields[0] != "stuck_diagnostic_v1" {
            return Err(StuckDiagnosticParseError::InvalidShape);
        }
        let blocker_category = BlockerCategory::parse(fields[12])?;

        Ok(Self {
            diagnostic_id: StuckDiagnosticId::new(fields[1])
                .map_err(StuckDiagnosticParseError::InvalidId)?,
            actor_id: ActorId::new(fields[2]).map_err(StuckDiagnosticParseError::InvalidId)?,
            window_start_tick: SimTick::new(
                fields[3]
                    .parse()
                    .map_err(|_| StuckDiagnosticParseError::InvalidTick)?,
            ),
            window_end_tick: SimTick::new(
                fields[4]
                    .parse()
                    .map_err(|_| StuckDiagnosticParseError::InvalidTick)?,
            ),
            active_need: decode_opt_need(fields[5])?,
            active_goal_id: decode_opt_id(fields[6], |value| CandidateGoalId::new(value))?,
            active_intention_id: decode_opt_id(fields[7], |value| IntentionId::new(value))?,
            routine_template_id: decode_opt_id(fields[8], |value| RoutineTemplateId::new(value))?,
            routine_execution_id: decode_opt_id(fields[9], |value| RoutineExecutionId::new(value))?,
            routine_step: decode_opt_text(fields[10])?
                .map(|step| RoutineStep::deserialize_canonical(step.as_bytes()))
                .transpose()
                .map_err(|_| StuckDiagnosticParseError::InvalidRoutineStep)?,
            attempted_action: decode_opt_id(fields[11], |value| SemanticActionId::new(value))?,
            blocker_category,
            concrete_blocker: decode_text_payload(fields[13])?,
            actor_known_explanation: decode_text_payload(fields[14])?,
            debug_only_details: decode_text_payload(fields[15])?,
            retry_abandon_fallback_outcome: decode_text_payload(fields[16])?,
            resulting_status: StuckResultingStatus::parse(
                fields
                    .get(17)
                    .copied()
                    .ok_or(StuckDiagnosticParseError::InvalidShape)?,
            )?,
            typed_diagnostic: if fields.len() == 24 {
                TypedDiagnosticFields {
                    responsible_layer: ResponsibleLayer::parse(fields[18])?,
                    blocker_code: BlockerCode::parse(fields[19])?,
                    input_source: decode_text_payload(fields[20])?,
                    actual_source: decode_text_payload(fields[21])?,
                    hidden_truth_referenced: decode_bool(fields[22])
                        .ok_or(StuckDiagnosticParseError::InvalidBool)?,
                    remediation_hint: decode_text_payload(fields[23])?,
                }
            } else {
                let mut typed = TypedDiagnosticFields::stuck_default();
                typed.blocker_code = blocker_category.blocker_code();
                typed
            },
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StuckDiagnosticParseError {
    InvalidUtf8,
    InvalidShape,
    InvalidTick,
    InvalidNeed,
    InvalidTextPayload,
    InvalidBlockerCategory,
    InvalidResultingStatus,
    InvalidRoutineStep,
    InvalidId(crate::ids::IdError),
    InvalidBool,
    InvalidResponsibleLayer,
    InvalidBlockerCode,
}

impl From<DiagnosticFieldParseError> for StuckDiagnosticParseError {
    fn from(value: DiagnosticFieldParseError) -> Self {
        match value {
            DiagnosticFieldParseError::InvalidResponsibleLayer => Self::InvalidResponsibleLayer,
            DiagnosticFieldParseError::InvalidBlockerCode => Self::InvalidBlockerCode,
        }
    }
}

impl fmt::Display for StuckDiagnosticParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StuckDiagnosticParseError::InvalidUtf8 => {
                write!(f, "canonical stuck diagnostic bytes must be UTF-8")
            }
            StuckDiagnosticParseError::InvalidShape => write!(f, "invalid stuck diagnostic shape"),
            StuckDiagnosticParseError::InvalidTick => write!(f, "invalid diagnostic tick"),
            StuckDiagnosticParseError::InvalidNeed => write!(f, "invalid diagnostic need"),
            StuckDiagnosticParseError::InvalidTextPayload => {
                write!(f, "invalid diagnostic text payload")
            }
            StuckDiagnosticParseError::InvalidBlockerCategory => {
                write!(f, "invalid blocker category")
            }
            StuckDiagnosticParseError::InvalidResultingStatus => {
                write!(f, "invalid resulting status")
            }
            StuckDiagnosticParseError::InvalidRoutineStep => write!(f, "invalid routine step"),
            StuckDiagnosticParseError::InvalidId(err) => write!(f, "invalid diagnostic ID: {err}"),
            StuckDiagnosticParseError::InvalidBool => write!(f, "invalid diagnostic bool"),
            StuckDiagnosticParseError::InvalidResponsibleLayer => {
                write!(f, "invalid responsible layer")
            }
            StuckDiagnosticParseError::InvalidBlockerCode => write!(f, "invalid blocker code"),
        }
    }
}

impl std::error::Error for StuckDiagnosticParseError {}

fn encode_bool(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

fn decode_bool(value: &str) -> Option<bool> {
    match value {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

fn encode_opt_need(value: Option<NeedKind>) -> &'static str {
    match value {
        Some(NeedKind::Hunger) => "hunger",
        Some(NeedKind::Fatigue) => "fatigue",
        Some(NeedKind::Safety) => "safety",
        None => "-",
    }
}

fn decode_opt_need(value: &str) -> Result<Option<NeedKind>, StuckDiagnosticParseError> {
    match value {
        "-" => Ok(None),
        "hunger" => Ok(Some(NeedKind::Hunger)),
        "fatigue" => Ok(Some(NeedKind::Fatigue)),
        "safety" => Ok(Some(NeedKind::Safety)),
        _ => Err(StuckDiagnosticParseError::InvalidNeed),
    }
}

fn encode_opt_id(value: Option<&str>) -> String {
    value.unwrap_or("-").to_string()
}

fn decode_opt_id<T>(
    value: &str,
    parse: impl FnOnce(&str) -> Result<T, crate::ids::IdError>,
) -> Result<Option<T>, StuckDiagnosticParseError> {
    if value == "-" {
        Ok(None)
    } else {
        Ok(Some(
            parse(value).map_err(StuckDiagnosticParseError::InvalidId)?,
        ))
    }
}

fn encode_opt_text(value: Option<&str>) -> String {
    value
        .map(encode_text_payload)
        .unwrap_or_else(|| "-".to_string())
}

fn decode_opt_text(value: &str) -> Result<Option<String>, StuckDiagnosticParseError> {
    if value == "-" {
        Ok(None)
    } else {
        Ok(Some(decode_text_payload(value)?))
    }
}

fn encode_text_payload(value: &str) -> String {
    value
        .as_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn decode_text_payload(value: &str) -> Result<String, StuckDiagnosticParseError> {
    if !value.len().is_multiple_of(2) {
        return Err(StuckDiagnosticParseError::InvalidTextPayload);
    }

    let bytes = value
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let hex =
                std::str::from_utf8(pair).map_err(|_| StuckDiagnosticParseError::InvalidUtf8)?;
            u8::from_str_radix(hex, 16).map_err(|_| StuckDiagnosticParseError::InvalidTextPayload)
        })
        .collect::<Result<Vec<_>, _>>()?;

    String::from_utf8(bytes).map_err(|_| StuckDiagnosticParseError::InvalidTextPayload)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::candidate::{
        ApplicabilityResult, CandidateGoalSource, GoalKind, GoalPriority,
    };
    use crate::agent::need::{NeedChangeCause, NeedState};
    use crate::agent::routine::RoutineDiagnosticKind;

    fn action(value: &str) -> SemanticActionId {
        SemanticActionId::new(value).unwrap()
    }

    fn candidate(value: &str, priority: GoalPriority) -> CandidateGoal {
        CandidateGoal::new(
            CandidateGoalId::new(value).unwrap(),
            ActorId::new("actor_mara").unwrap(),
            SimTick::new(10),
            SimTick::new(11),
            CandidateGoalSource::NeedPressure,
            GoalKind::Eat,
            priority,
            priority.stable_id(),
            vec!["belief:pantry_has_bread".to_string()],
            ApplicabilityResult::Applicable,
            None,
            Some(RoutineTemplateId::new("routine_eat_meal").unwrap()),
            DecisionTraceId::new("trace_candidate").unwrap(),
        )
    }

    #[test]
    fn decision_trace_exposes_food_failure_fields() {
        let state = NeedState::initial(NeedKind::Hunger, 760, NeedChangeCause::TickDelta);
        let pressure = state.derive_pressure(
            ActorId::new("actor_mara").unwrap(),
            None,
            "I am very hungry",
            "hunger crossed severe before breakfast",
        );
        let selected = candidate("goal_eat_known_food", GoalPriority::SevereHunger);
        let rejected = candidate("goal_wait_for_food", GoalPriority::IdleWithReason);

        let trace = DecisionTrace::new(
            DecisionTraceId::new("trace_food_unavailable").unwrap(),
            ActorId::new("actor_mara").unwrap(),
            SimTick::new(10),
            SimTick::new(11),
            vec![pressure],
            None,
            vec![selected.clone(), rejected.clone()],
            Some(selected.candidate_goal_id.clone()),
            Some(RoutineTemplateId::new("routine_eat_meal").unwrap()),
            None,
            vec![RejectedDecisionItem {
                stable_ref: rejected.candidate_goal_id.serialize_canonical().to_string(),
                reason: "waiting would not reduce severe hunger".to_string(),
            }],
            Vec::new(),
            vec![RejectedDecisionItem {
                stable_ref: "eat.food.bread".to_string(),
                reason: "validator found no accessible serving".to_string(),
            }],
            vec!["belief:pantry_has_bread".to_string()],
            Some(action("eat.food.bread")),
            Some("rejected: resource missing".to_string()),
            Some("fallback_find_food".to_string()),
            HiddenTruthAudit {
                actor_known_only: true,
                notes: "planner used belief, validator checked truth".to_string(),
            },
            DecisionOutcome::Failed,
            "debug: pantry was empty",
        );

        assert_eq!(trace.candidate_goals_considered.len(), 2);
        assert_eq!(
            trace.selected_method_id.as_ref().unwrap().as_str(),
            "routine_eat_meal"
        );
        assert_eq!(
            trace.rejected_goals[0].reason,
            "waiting would not reduce severe hunger"
        );
        assert!(trace.hidden_truth_audit_result.actor_known_only);
        assert_eq!(trace.outcome, DecisionOutcome::Failed);
        assert!(trace.render_summary().contains("outcome=failed"));
        assert!(trace
            .serialize_canonical()
            .starts_with("decision_trace_v1|"));
    }

    #[test]
    fn decision_trace_record_derives_typed_diagnostic_from_failed_outcome() {
        let trace = DecisionTrace::new(
            DecisionTraceId::new("trace_failed_outcome").unwrap(),
            ActorId::new("actor_mara").unwrap(),
            SimTick::new(10),
            SimTick::new(11),
            Vec::new(),
            None,
            Vec::new(),
            None,
            None,
            None,
            Vec::new(),
            Vec::new(),
            vec![RejectedDecisionItem {
                stable_ref: "eat.food_missing".to_string(),
                reason: "validator rejected target".to_string(),
            }],
            vec!["fact:remembered_food".to_string()],
            Some(action("eat.food_missing")),
            Some("rejected: target absent".to_string()),
            None,
            HiddenTruthAudit {
                actor_known_only: true,
                notes: "failed through validator".to_string(),
            },
            DecisionOutcome::Failed,
            "",
        );

        let record = DecisionTraceRecord::from_trace(&trace);

        assert!(record.actor_known_context_hash.is_some());
        assert_eq!(
            record.typed_diagnostic.responsible_layer,
            ResponsibleLayer::LocalPlanning
        );
        assert_eq!(
            record.typed_diagnostic.blocker_code,
            BlockerCode::LocalPlanFailed
        );
        assert_eq!(
            record.typed_diagnostic.actual_source,
            "decision_outcome:failed"
        );
    }

    #[test]
    fn legacy_decision_trace_record_keeps_absent_context_hash_typed_absent() {
        let legacy = format!(
            "decision_trace_v1|{}|{}|10|11|failed|0|true|{}",
            DecisionTraceId::new("trace_legacy_failed")
                .unwrap()
                .serialize_canonical(),
            ActorId::new("actor_mara").unwrap().serialize_canonical(),
            encode_text_payload("legacy without context hash")
        );

        let record = DecisionTraceRecord::deserialize_canonical(legacy.as_bytes()).unwrap();

        assert_eq!(record.actor_known_context_hash, None);
        assert!(record.actor_known_inputs.is_empty());
        assert_eq!(
            record.typed_diagnostic.responsible_layer,
            ResponsibleLayer::LocalPlanning
        );
        assert_eq!(
            record.typed_diagnostic.blocker_code,
            BlockerCode::LocalPlanFailed
        );
        assert_eq!(
            record.typed_diagnostic.actual_source,
            "decision_outcome:failed"
        );

        let legacy_hidden_truth = legacy.replace("|true|", "|false|");
        let record =
            DecisionTraceRecord::deserialize_canonical(legacy_hidden_truth.as_bytes()).unwrap();
        assert!(record.typed_diagnostic.hidden_truth_referenced);
    }

    #[test]
    fn every_blocker_category_serializes_and_round_trips() {
        for category in BlockerCategory::ALL {
            let diagnostic = StuckDiagnostic::new(
                StuckDiagnosticId::new(format!("diagnostic_{}", category.stable_id())).unwrap(),
                ActorId::new("actor_mara").unwrap(),
                SimTick::new(20),
                SimTick::new(21),
                Some(NeedKind::Hunger),
                Some(CandidateGoalId::new("goal_find_food").unwrap()),
                Some(IntentionId::new("intention_find_food").unwrap()),
                Some(RoutineTemplateId::new("routine_find_food").unwrap()),
                Some(RoutineExecutionId::new("routine_exec_find_food").unwrap()),
                Some(RoutineStep::FailWithTypedDiagnostic {
                    diagnostic: RoutineDiagnosticKind::NoSleepAffordance,
                }),
                Some(action("find_food.public_market")),
                category,
                "known food source unavailable",
                "I cannot find food I know how to reach",
                "debug-only fixture pantry empty",
                "fallback exhausted",
                StuckResultingStatus::Failed,
            );

            let bytes = diagnostic.serialize_canonical_bytes();
            let round_tripped = StuckDiagnostic::deserialize_canonical(&bytes).unwrap();

            assert_eq!(round_tripped, diagnostic);
            assert_eq!(
                round_tripped.typed_diagnostic.responsible_layer,
                ResponsibleLayer::LocalPlanning
            );
            assert_eq!(
                round_tripped.typed_diagnostic.blocker_code,
                category.blocker_code()
            );
            assert!(!round_tripped.typed_diagnostic.hidden_truth_referenced);
            assert_eq!(round_tripped.serialize_canonical_bytes(), bytes);
        }
    }

    #[test]
    fn responsible_layer_vocabulary_matches_spec() {
        let ids = ResponsibleLayer::ALL.map(ResponsibleLayer::stable_id);
        assert_eq!(
            ids,
            [
                "candidate_generation",
                "intention_lifecycle",
                "method_selection",
                "local_planning",
                "proposal_construction",
                "scheduler",
                "action_validation",
                "projection",
                "view_model",
                "test_oracle",
                "debug_quarantine",
            ]
        );
    }

    #[test]
    fn decision_trace_vocabulary_parses_every_canonical_token() {
        let outcomes = [
            DecisionOutcome::Continued,
            DecisionOutcome::Switched,
            DecisionOutcome::Waited,
            DecisionOutcome::Replanned,
            DecisionOutcome::Failed,
            DecisionOutcome::Completed,
        ];
        for outcome in outcomes {
            assert_eq!(DecisionOutcome::parse(outcome.stable_id()), Some(outcome));
        }
        assert_eq!(DecisionOutcome::parse("completed "), None);

        for layer in ResponsibleLayer::ALL {
            assert_eq!(ResponsibleLayer::parse(layer.stable_id()).unwrap(), layer);
        }
        assert_eq!(
            ResponsibleLayer::parse("debug-quarantine"),
            Err(DiagnosticFieldParseError::InvalidResponsibleLayer)
        );

        let blocker_codes = [
            BlockerCode::None,
            BlockerCode::Physical,
            BlockerCode::Access,
            BlockerCode::Knowledge,
            BlockerCode::Resource,
            BlockerCode::SocialNormPlaceholder,
            BlockerCode::SchedulingReservation,
            BlockerCode::UnsupportedAction,
            BlockerCode::PlannerBudgetExhausted,
            BlockerCode::FixtureAuthoringError,
            BlockerCode::NoApplicableCandidate,
            BlockerCode::NoApplicableMethod,
            BlockerCode::EmptyLocalPlan,
            BlockerCode::LocalPlanFailed,
            BlockerCode::HiddenTruthInput,
            BlockerCode::ProvenanceDangling,
            BlockerCode::ProvenanceClassMismatch,
            BlockerCode::ProvenanceWitnessMismatch,
        ];
        for blocker_code in blocker_codes {
            assert_eq!(
                BlockerCode::parse(blocker_code.stable_id()).unwrap(),
                blocker_code
            );
        }
        assert_eq!(
            BlockerCode::parse("local plan failed"),
            Err(DiagnosticFieldParseError::InvalidBlockerCode)
        );
    }

    #[test]
    fn stuck_diagnostic_vocabulary_parses_every_canonical_token() {
        for category in BlockerCategory::ALL {
            assert_eq!(
                BlockerCategory::parse(category.stable_id()).unwrap(),
                category
            );
        }
        assert_eq!(
            BlockerCategory::parse("social norm placeholder"),
            Err(StuckDiagnosticParseError::InvalidBlockerCategory)
        );

        let statuses = [
            StuckResultingStatus::Idle,
            StuckResultingStatus::Waiting,
            StuckResultingStatus::Replanning,
            StuckResultingStatus::Failed,
            StuckResultingStatus::Suspended,
            StuckResultingStatus::Completed,
        ];
        for status in statuses {
            assert_eq!(
                StuckResultingStatus::parse(status.stable_id()).unwrap(),
                status
            );
        }
        assert_eq!(
            StuckResultingStatus::parse("complete"),
            Err(StuckDiagnosticParseError::InvalidResultingStatus)
        );

        assert_eq!(decode_opt_need("-").unwrap(), None);
        assert_eq!(decode_opt_need("hunger").unwrap(), Some(NeedKind::Hunger));
        assert_eq!(decode_opt_need("fatigue").unwrap(), Some(NeedKind::Fatigue));
        assert_eq!(decode_opt_need("safety").unwrap(), Some(NeedKind::Safety));
        assert_eq!(
            decode_opt_need("fatigue "),
            Err(StuckDiagnosticParseError::InvalidNeed)
        );
    }

    #[test]
    fn canonical_trace_deserializers_reject_bad_prefix_and_field_count() {
        let record = DecisionTraceRecord {
            trace_id: DecisionTraceId::new("trace_vocab_shape").unwrap(),
            actor_id: ActorId::new("actor_mara").unwrap(),
            window_start_tick: SimTick::new(30),
            window_end_tick: SimTick::new(31),
            outcome: DecisionOutcome::Waited,
            candidate_goal_count: 0,
            actor_known_context_hash: Some(compute_holder_known_context_hash(Vec::new()).hash),
            actor_known_inputs: Vec::new(),
            hidden_truth_audit_result: HiddenTruthAudit {
                actor_known_only: true,
                notes: "no applicable method".to_string(),
            },
            typed_diagnostic: TypedDiagnosticFields {
                responsible_layer: ResponsibleLayer::MethodSelection,
                blocker_code: BlockerCode::NoApplicableMethod,
                input_source: "actor_known_context".to_string(),
                actual_source: "decision_outcome:waited".to_string(),
                hidden_truth_referenced: false,
                remediation_hint: "inspect method fallback".to_string(),
            },
        };
        let canonical = record.serialize_canonical();
        assert_eq!(
            DecisionTraceRecord::deserialize_canonical(canonical.as_bytes())
                .unwrap()
                .typed_diagnostic
                .blocker_code,
            BlockerCode::NoApplicableMethod
        );
        let bad_prefix = canonical.replacen("decision_trace_v1", "decision_trace_v2", 1);
        assert!(matches!(
            DecisionTraceRecord::deserialize_canonical(bad_prefix.as_bytes()),
            Err(DecisionTraceRecordParseError::InvalidShape)
        ));
        let missing_field = canonical.rsplit_once('|').unwrap().0;
        assert!(matches!(
            DecisionTraceRecord::deserialize_canonical(missing_field.as_bytes()),
            Err(DecisionTraceRecordParseError::InvalidShape)
        ));

        let diagnostic = StuckDiagnostic::new(
            StuckDiagnosticId::new("diagnostic_vocab_shape").unwrap(),
            ActorId::new("actor_mara").unwrap(),
            SimTick::new(30),
            SimTick::new(31),
            Some(NeedKind::Fatigue),
            None,
            None,
            None,
            None,
            None,
            None,
            BlockerCategory::PlannerBudgetExhausted,
            "planner budget exhausted",
            "I cannot find a local plan",
            "debug only",
            "replan",
            StuckResultingStatus::Suspended,
        )
        .with_typed_diagnostic(TypedDiagnosticFields {
            responsible_layer: ResponsibleLayer::LocalPlanning,
            blocker_code: BlockerCode::PlannerBudgetExhausted,
            input_source: "actor_known_context".to_string(),
            actual_source: "routine_stuck_diagnostic".to_string(),
            hidden_truth_referenced: false,
            remediation_hint: "inspect local planner".to_string(),
        });
        let canonical = diagnostic.serialize_canonical();
        assert_eq!(
            StuckDiagnostic::deserialize_canonical(canonical.as_bytes())
                .unwrap()
                .resulting_status,
            StuckResultingStatus::Suspended
        );
        let bad_prefix = canonical.replacen("stuck_diagnostic_v1", "stuck_diagnostic_v2", 1);
        assert!(matches!(
            StuckDiagnostic::deserialize_canonical(bad_prefix.as_bytes()),
            Err(StuckDiagnosticParseError::InvalidShape)
        ));
        let missing_field = canonical.rsplit_once('|').unwrap().0;
        assert!(matches!(
            StuckDiagnostic::deserialize_canonical(missing_field.as_bytes()),
            Err(StuckDiagnosticParseError::InvalidShape)
        ));
    }

    #[test]
    fn stuck_parse_errors_map_and_display_as_typed_failures() {
        assert_eq!(
            DecisionTraceRecordParseError::from(StuckDiagnosticParseError::InvalidTextPayload),
            DecisionTraceRecordParseError::InvalidTextPayload
        );
        assert_eq!(
            DecisionTraceRecordParseError::from(StuckDiagnosticParseError::InvalidUtf8),
            DecisionTraceRecordParseError::InvalidTextPayload
        );
        assert_eq!(
            DecisionTraceRecordParseError::from(StuckDiagnosticParseError::InvalidNeed),
            DecisionTraceRecordParseError::InvalidShape
        );

        let errors = [
            StuckDiagnosticParseError::InvalidUtf8,
            StuckDiagnosticParseError::InvalidShape,
            StuckDiagnosticParseError::InvalidTick,
            StuckDiagnosticParseError::InvalidNeed,
            StuckDiagnosticParseError::InvalidTextPayload,
            StuckDiagnosticParseError::InvalidBlockerCategory,
            StuckDiagnosticParseError::InvalidResultingStatus,
            StuckDiagnosticParseError::InvalidRoutineStep,
            StuckDiagnosticParseError::InvalidBool,
            StuckDiagnosticParseError::InvalidResponsibleLayer,
            StuckDiagnosticParseError::InvalidBlockerCode,
        ];
        for error in errors {
            let rendered = error.to_string();
            assert!(!rendered.is_empty());
        }
        assert!(StuckDiagnosticParseError::InvalidTextPayload
            .to_string()
            .contains("text payload"));
        assert!(StuckDiagnosticParseError::InvalidResponsibleLayer
            .to_string()
            .contains("responsible layer"));
    }

    #[test]
    fn trace_records_have_no_actor_state_mutator_surface() {
        let source = include_str!("trace.rs");
        let production_source = source.split("#[cfg(test)]").next().unwrap();

        assert!(!production_source.contains("NeedState"));
        assert!(!production_source.contains("WorldState"));
        assert!(!production_source.contains("BeliefDraft"));
        assert!(!production_source.contains("&mut Need"));
        assert!(!production_source.contains("&mut Belief"));
    }
}
