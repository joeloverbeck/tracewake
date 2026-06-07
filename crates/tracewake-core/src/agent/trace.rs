use std::fmt;

use crate::agent::candidate::CandidateGoal;
use crate::agent::intention::Intention;
use crate::agent::need::{NeedKind, NeedPressure};
use crate::agent::routine::RoutineStep;
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
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HiddenTruthAudit {
    pub actor_known_only: bool,
    pub notes: String,
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
}

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
        }
    }

    pub fn serialize_canonical(&self) -> String {
        format!(
            "stuck_diagnostic_v1|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
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
        )
    }

    pub fn serialize_canonical_bytes(&self) -> Vec<u8> {
        self.serialize_canonical().into_bytes()
    }

    pub fn deserialize_canonical(value: &[u8]) -> Result<Self, StuckDiagnosticParseError> {
        let value =
            std::str::from_utf8(value).map_err(|_| StuckDiagnosticParseError::InvalidUtf8)?;
        let fields: Vec<_> = value.split('|').collect();
        if fields.len() != 18 || fields[0] != "stuck_diagnostic_v1" {
            return Err(StuckDiagnosticParseError::InvalidShape);
        }

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
            blocker_category: BlockerCategory::parse(fields[12])?,
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
                    diagnostic: "resource:known food absent".to_string(),
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
            assert_eq!(round_tripped.serialize_canonical_bytes(), bytes);
        }
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
