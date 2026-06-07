use std::fmt;

use crate::ids::{
    ActorId, DecisionTraceId, RoutineExecutionId, RoutineTemplateId, SemanticActionId,
};
use crate::time::SimTick;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RoutineFamily {
    MorningWake,
    EatMeal,
    GoToWork,
    WorkBlock,
    ReturnHome,
    SleepNight,
    FindFood,
    ContinueCurrentIntention,
    Wait,
    IdleWithReason,
}

impl RoutineFamily {
    pub const fn stable_id(self) -> &'static str {
        match self {
            RoutineFamily::MorningWake => "morning_wake",
            RoutineFamily::EatMeal => "eat_meal",
            RoutineFamily::GoToWork => "go_to_work",
            RoutineFamily::WorkBlock => "work_block",
            RoutineFamily::ReturnHome => "return_home",
            RoutineFamily::SleepNight => "sleep_night",
            RoutineFamily::FindFood => "find_food",
            RoutineFamily::ContinueCurrentIntention => "continue_current_intention",
            RoutineFamily::Wait => "wait",
            RoutineFamily::IdleWithReason => "idle_with_reason",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RoutineStep {
    SelectKnownPlace { action_id: SemanticActionId },
    MoveTowardPlace { action_id: SemanticActionId },
    OpenKnownContainer { action_id: SemanticActionId },
    CheckKnownContainer { action_id: SemanticActionId },
    ConsumeAccessibleFood { action_id: SemanticActionId },
    StartScheduledSleep { action_id: SemanticActionId },
    CompleteScheduledSleep { action_id: SemanticActionId },
    StartWorkBlock { action_id: SemanticActionId },
    CompleteWorkBlock { action_id: SemanticActionId },
    WaitUntil { reason: String },
    ContinueCurrentStep { action_id: SemanticActionId },
    FallbackToFindFood { action_id: SemanticActionId },
    FailWithTypedDiagnostic { diagnostic: String },
}

impl RoutineStep {
    pub fn proposed(&self) -> RoutineStepProposal<'_> {
        match self {
            RoutineStep::SelectKnownPlace { action_id }
            | RoutineStep::MoveTowardPlace { action_id }
            | RoutineStep::OpenKnownContainer { action_id }
            | RoutineStep::CheckKnownContainer { action_id }
            | RoutineStep::ConsumeAccessibleFood { action_id }
            | RoutineStep::StartScheduledSleep { action_id }
            | RoutineStep::CompleteScheduledSleep { action_id }
            | RoutineStep::StartWorkBlock { action_id }
            | RoutineStep::CompleteWorkBlock { action_id }
            | RoutineStep::ContinueCurrentStep { action_id }
            | RoutineStep::FallbackToFindFood { action_id } => {
                RoutineStepProposal::Action(action_id)
            }
            RoutineStep::WaitUntil { reason } => RoutineStepProposal::Wait(reason),
            RoutineStep::FailWithTypedDiagnostic { diagnostic } => {
                RoutineStepProposal::Diagnostic(diagnostic)
            }
        }
    }

    pub fn serialize_canonical(&self) -> String {
        match self {
            RoutineStep::SelectKnownPlace { action_id } => {
                encode_action_step("select_known_place", action_id)
            }
            RoutineStep::MoveTowardPlace { action_id } => {
                encode_action_step("move_toward_place", action_id)
            }
            RoutineStep::OpenKnownContainer { action_id } => {
                encode_action_step("open_known_container", action_id)
            }
            RoutineStep::CheckKnownContainer { action_id } => {
                encode_action_step("check_known_container", action_id)
            }
            RoutineStep::ConsumeAccessibleFood { action_id } => {
                encode_action_step("consume_accessible_food", action_id)
            }
            RoutineStep::StartScheduledSleep { action_id } => {
                encode_action_step("start_scheduled_sleep", action_id)
            }
            RoutineStep::CompleteScheduledSleep { action_id } => {
                encode_action_step("complete_scheduled_sleep", action_id)
            }
            RoutineStep::StartWorkBlock { action_id } => {
                encode_action_step("start_work_block", action_id)
            }
            RoutineStep::CompleteWorkBlock { action_id } => {
                encode_action_step("complete_work_block", action_id)
            }
            RoutineStep::WaitUntil { reason } => encode_text_step("wait_until", reason),
            RoutineStep::ContinueCurrentStep { action_id } => {
                encode_action_step("continue_current_step", action_id)
            }
            RoutineStep::FallbackToFindFood { action_id } => {
                encode_action_step("fallback_to_find_food", action_id)
            }
            RoutineStep::FailWithTypedDiagnostic { diagnostic } => {
                encode_text_step("fail_with_typed_diagnostic", diagnostic)
            }
        }
    }

    pub fn serialize_canonical_bytes(&self) -> Vec<u8> {
        self.serialize_canonical().into_bytes()
    }

    pub fn deserialize_canonical(value: &[u8]) -> Result<Self, RoutineStepParseError> {
        let value = std::str::from_utf8(value).map_err(|_| RoutineStepParseError::InvalidUtf8)?;
        let mut fields = value.split('|');
        let version = fields.next().ok_or(RoutineStepParseError::InvalidShape)?;
        let kind = fields.next().ok_or(RoutineStepParseError::InvalidShape)?;
        let payload = fields.next().ok_or(RoutineStepParseError::InvalidShape)?;
        if fields.next().is_some() || version != "routine_step_v1" {
            return Err(RoutineStepParseError::InvalidShape);
        }

        let action = || {
            SemanticActionId::new(payload).map_err(RoutineStepParseError::InvalidSemanticActionId)
        };
        let text = || decode_text_payload(payload);

        match kind {
            "select_known_place" => Ok(Self::SelectKnownPlace {
                action_id: action()?,
            }),
            "move_toward_place" => Ok(Self::MoveTowardPlace {
                action_id: action()?,
            }),
            "open_known_container" => Ok(Self::OpenKnownContainer {
                action_id: action()?,
            }),
            "check_known_container" => Ok(Self::CheckKnownContainer {
                action_id: action()?,
            }),
            "consume_accessible_food" => Ok(Self::ConsumeAccessibleFood {
                action_id: action()?,
            }),
            "start_scheduled_sleep" => Ok(Self::StartScheduledSleep {
                action_id: action()?,
            }),
            "complete_scheduled_sleep" => Ok(Self::CompleteScheduledSleep {
                action_id: action()?,
            }),
            "start_work_block" => Ok(Self::StartWorkBlock {
                action_id: action()?,
            }),
            "complete_work_block" => Ok(Self::CompleteWorkBlock {
                action_id: action()?,
            }),
            "wait_until" => Ok(Self::WaitUntil { reason: text()? }),
            "continue_current_step" => Ok(Self::ContinueCurrentStep {
                action_id: action()?,
            }),
            "fallback_to_find_food" => Ok(Self::FallbackToFindFood {
                action_id: action()?,
            }),
            "fail_with_typed_diagnostic" => Ok(Self::FailWithTypedDiagnostic {
                diagnostic: text()?,
            }),
            _ => Err(RoutineStepParseError::InvalidStepKind),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoutineStepProposal<'a> {
    Action(&'a SemanticActionId),
    Wait(&'a str),
    Diagnostic(&'a str),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RoutineCondition {
    ActorKnowsFoodSource,
    ActorKnowsWorkplace,
    WorkplaceAssignmentActive,
    ActorKnowsHome,
    ActorKnowsSleepPlace,
    ActorHasFoodSearchKnowledge,
    ActiveIntentionPresent,
    SleepStateCanEnd,
    FoodSourceBelievedAccessible,
    ActorAtWorkplace,
    SleepPlaceBelievedAccessible,
    SearchSurfaceActorKnown,
    NextStepAvailable,
    KnownRouteSurface,
    ModeledWaitReason,
    ReevaluationWindowKnown,
    FixtureAuthoredPossibility,
    SharedPipelinePreconditions,
    AssignedWorkplaceKnown,
    AtWorkplace,
}

impl RoutineCondition {
    pub const fn stable_id(&self) -> &'static str {
        match self {
            Self::ActorKnowsFoodSource => "actor_knows_food_source",
            Self::ActorKnowsWorkplace => "actor_knows_workplace",
            Self::WorkplaceAssignmentActive => "workplace_assignment_active",
            Self::ActorKnowsHome => "actor_knows_home",
            Self::ActorKnowsSleepPlace => "actor_knows_sleep_place",
            Self::ActorHasFoodSearchKnowledge => "actor_has_food_search_knowledge",
            Self::ActiveIntentionPresent => "active_intention_present",
            Self::SleepStateCanEnd => "sleep_state_can_end",
            Self::FoodSourceBelievedAccessible => "food_source_believed_accessible",
            Self::ActorAtWorkplace => "actor_at_workplace",
            Self::SleepPlaceBelievedAccessible => "sleep_place_believed_accessible",
            Self::SearchSurfaceActorKnown => "search_surface_actor_known",
            Self::NextStepAvailable => "next_step_available",
            Self::KnownRouteSurface => "known_route_surface",
            Self::ModeledWaitReason => "modeled_wait_reason",
            Self::ReevaluationWindowKnown => "reevaluation_window_known",
            Self::FixtureAuthoredPossibility => "fixture_authored_possibility",
            Self::SharedPipelinePreconditions => "shared_pipeline_preconditions",
            Self::AssignedWorkplaceKnown => "assigned_workplace_known",
            Self::AtWorkplace => "at_workplace",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "actor_knows_food_source" => Some(Self::ActorKnowsFoodSource),
            "actor_knows_workplace" => Some(Self::ActorKnowsWorkplace),
            "workplace_assignment_active" => Some(Self::WorkplaceAssignmentActive),
            "actor_knows_home" => Some(Self::ActorKnowsHome),
            "actor_knows_sleep_place" => Some(Self::ActorKnowsSleepPlace),
            "actor_has_food_search_knowledge" => Some(Self::ActorHasFoodSearchKnowledge),
            "active_intention_present" => Some(Self::ActiveIntentionPresent),
            "sleep_state_can_end" => Some(Self::SleepStateCanEnd),
            "food_source_believed_accessible" => Some(Self::FoodSourceBelievedAccessible),
            "actor_at_workplace" => Some(Self::ActorAtWorkplace),
            "sleep_place_believed_accessible" => Some(Self::SleepPlaceBelievedAccessible),
            "search_surface_actor_known" => Some(Self::SearchSurfaceActorKnown),
            "next_step_available" => Some(Self::NextStepAvailable),
            "known_route_surface" => Some(Self::KnownRouteSurface),
            "modeled_wait_reason" => Some(Self::ModeledWaitReason),
            "reevaluation_window_known" => Some(Self::ReevaluationWindowKnown),
            "fixture_authored_possibility" => Some(Self::FixtureAuthoredPossibility),
            "shared_pipeline_preconditions" => Some(Self::SharedPipelinePreconditions),
            "assigned_workplace_known" => Some(Self::AssignedWorkplaceKnown),
            "at_workplace" => Some(Self::AtWorkplace),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoutineTemplate {
    pub template_id: RoutineTemplateId,
    pub family: RoutineFamily,
    pub applicability_conditions: Vec<RoutineCondition>,
    pub preconditions: Vec<RoutineCondition>,
    pub steps: Vec<RoutineStep>,
    pub min_duration_ticks: u64,
    pub max_duration_ticks: u64,
    pub interruption_points: Vec<usize>,
    pub failure_modes: Vec<String>,
    pub fallback_rules: Vec<String>,
    pub debug_labels: Vec<String>,
    pub reservable_resource: Option<String>,
}

impl RoutineTemplate {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        template_id: RoutineTemplateId,
        family: RoutineFamily,
        applicability_conditions: Vec<RoutineCondition>,
        preconditions: Vec<RoutineCondition>,
        steps: Vec<RoutineStep>,
        min_duration_ticks: u64,
        max_duration_ticks: u64,
        interruption_points: Vec<usize>,
        failure_modes: Vec<String>,
        fallback_rules: Vec<String>,
        debug_labels: Vec<String>,
        reservable_resource: Option<String>,
    ) -> Result<Self, RoutineTemplateError> {
        if steps.is_empty() {
            return Err(RoutineTemplateError::MissingSteps);
        }
        if failure_modes.is_empty() {
            return Err(RoutineTemplateError::MissingFailureModes);
        }
        if interruption_points.is_empty() {
            return Err(RoutineTemplateError::MissingInterruptionPoints);
        }
        if min_duration_ticks == 0 || max_duration_ticks == 0 {
            return Err(RoutineTemplateError::NonPositiveDuration);
        }
        if min_duration_ticks > max_duration_ticks {
            return Err(RoutineTemplateError::InvalidDurationRange);
        }
        if interruption_points
            .iter()
            .any(|point| *point >= steps.len())
        {
            return Err(RoutineTemplateError::InvalidInterruptionPoint);
        }

        Ok(Self {
            template_id,
            family,
            applicability_conditions,
            preconditions,
            steps,
            min_duration_ticks,
            max_duration_ticks,
            interruption_points,
            failure_modes,
            fallback_rules,
            debug_labels,
            reservable_resource,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RoutineStepStatus {
    NotStarted,
    InProgress,
    Waiting,
    Completed,
    Failed,
    Interrupted,
    Suspended,
    Abandoned,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoutineExecution {
    pub execution_id: RoutineExecutionId,
    pub actor_id: ActorId,
    pub template_id: RoutineTemplateId,
    pub family: RoutineFamily,
    pub current_step_index: usize,
    pub step_status: RoutineStepStatus,
    pub start_tick: SimTick,
    pub last_progress_tick: SimTick,
    pub expected_next_progress_tick: Option<SimTick>,
    pub deadline_tick: Option<SimTick>,
    pub concrete_action_ancestry: Vec<SemanticActionId>,
    pub reserved_resource: Option<String>,
    pub fallback_attempts: u32,
    pub failure_interruption_reason: Option<String>,
    pub trace_id: DecisionTraceId,
}

impl RoutineExecution {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        execution_id: RoutineExecutionId,
        actor_id: ActorId,
        template_id: RoutineTemplateId,
        family: RoutineFamily,
        start_tick: SimTick,
        expected_next_progress_tick: Option<SimTick>,
        deadline_tick: Option<SimTick>,
        reserved_resource: Option<String>,
        trace_id: DecisionTraceId,
    ) -> Self {
        Self {
            execution_id,
            actor_id,
            template_id,
            family,
            current_step_index: 0,
            step_status: RoutineStepStatus::NotStarted,
            start_tick,
            last_progress_tick: start_tick,
            expected_next_progress_tick,
            deadline_tick,
            concrete_action_ancestry: Vec::new(),
            reserved_resource,
            fallback_attempts: 0,
            failure_interruption_reason: None,
            trace_id,
        }
    }

    pub fn start_step(&mut self, tick: SimTick, action_id: SemanticActionId) {
        self.step_status = RoutineStepStatus::InProgress;
        self.last_progress_tick = tick;
        self.concrete_action_ancestry.push(action_id);
    }

    pub fn wait(&mut self, tick: SimTick) {
        self.step_status = RoutineStepStatus::Waiting;
        self.last_progress_tick = tick;
    }

    pub fn complete_step(&mut self, tick: SimTick) {
        self.step_status = RoutineStepStatus::Completed;
        self.last_progress_tick = tick;
        self.failure_interruption_reason = None;
    }

    pub fn fail(&mut self, tick: SimTick, reason: impl Into<String>) {
        self.step_status = RoutineStepStatus::Failed;
        self.last_progress_tick = tick;
        self.failure_interruption_reason = Some(reason.into());
    }

    pub fn interrupt(&mut self, tick: SimTick, reason: impl Into<String>) {
        self.step_status = RoutineStepStatus::Interrupted;
        self.last_progress_tick = tick;
        self.failure_interruption_reason = Some(reason.into());
    }

    pub fn suspend(&mut self, tick: SimTick, reason: impl Into<String>) {
        self.step_status = RoutineStepStatus::Suspended;
        self.last_progress_tick = tick;
        self.failure_interruption_reason = Some(reason.into());
    }

    pub fn resume(&mut self, tick: SimTick) {
        self.step_status = RoutineStepStatus::InProgress;
        self.last_progress_tick = tick;
        self.failure_interruption_reason = None;
    }

    pub fn abandon(&mut self, tick: SimTick, reason: impl Into<String>) {
        self.step_status = RoutineStepStatus::Abandoned;
        self.last_progress_tick = tick;
        self.failure_interruption_reason = Some(reason.into());
    }

    pub fn record_fallback_attempt(&mut self) {
        self.fallback_attempts = self.fallback_attempts.saturating_add(1);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RoutineTemplateError {
    MissingSteps,
    MissingFailureModes,
    MissingInterruptionPoints,
    NonPositiveDuration,
    InvalidDurationRange,
    InvalidInterruptionPoint,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RoutineStepParseError {
    InvalidUtf8,
    InvalidShape,
    InvalidStepKind,
    InvalidTextPayload,
    InvalidSemanticActionId(crate::ids::IdError),
}

impl fmt::Display for RoutineStepParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RoutineStepParseError::InvalidUtf8 => {
                write!(f, "canonical routine step bytes must be UTF-8")
            }
            RoutineStepParseError::InvalidShape => write!(f, "invalid routine step shape"),
            RoutineStepParseError::InvalidStepKind => write!(f, "invalid routine step kind"),
            RoutineStepParseError::InvalidTextPayload => write!(f, "invalid routine step text"),
            RoutineStepParseError::InvalidSemanticActionId(err) => {
                write!(f, "invalid routine step action ID: {err}")
            }
        }
    }
}

impl std::error::Error for RoutineStepParseError {}

fn encode_action_step(kind: &str, action_id: &SemanticActionId) -> String {
    format!(
        "routine_step_v1|{}|{}",
        kind,
        action_id.serialize_canonical()
    )
}

fn encode_text_step(kind: &str, value: &str) -> String {
    format!("routine_step_v1|{}|{}", kind, encode_text_payload(value))
}

fn encode_text_payload(value: &str) -> String {
    value
        .as_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn decode_text_payload(value: &str) -> Result<String, RoutineStepParseError> {
    if !value.len().is_multiple_of(2) {
        return Err(RoutineStepParseError::InvalidTextPayload);
    }

    let bytes = value
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let hex = std::str::from_utf8(pair).map_err(|_| RoutineStepParseError::InvalidUtf8)?;
            u8::from_str_radix(hex, 16).map_err(|_| RoutineStepParseError::InvalidTextPayload)
        })
        .collect::<Result<Vec<_>, _>>()?;

    String::from_utf8(bytes).map_err(|_| RoutineStepParseError::InvalidTextPayload)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn action(value: &str) -> SemanticActionId {
        SemanticActionId::new(value).unwrap()
    }

    fn template(
        failure_modes: Vec<String>,
        interruption_points: Vec<usize>,
        min_duration_ticks: u64,
        max_duration_ticks: u64,
    ) -> Result<RoutineTemplate, RoutineTemplateError> {
        RoutineTemplate::new(
            RoutineTemplateId::new("routine_eat_meal").unwrap(),
            RoutineFamily::EatMeal,
            vec![RoutineCondition::ActorKnowsFoodSource],
            vec![RoutineCondition::FoodSourceBelievedAccessible],
            vec![RoutineStep::CheckKnownContainer {
                action_id: action("check_container.pantry"),
            }],
            min_duration_ticks,
            max_duration_ticks,
            interruption_points,
            failure_modes,
            vec!["fallback_find_food".to_string()],
            vec!["debug_eat_meal".to_string()],
            Some("resource_pantry".to_string()),
        )
    }

    fn execution() -> RoutineExecution {
        RoutineExecution::new(
            RoutineExecutionId::new("routine_exec_mara_breakfast").unwrap(),
            ActorId::new("actor_mara").unwrap(),
            RoutineTemplateId::new("routine_eat_meal").unwrap(),
            RoutineFamily::EatMeal,
            SimTick::new(10),
            Some(SimTick::new(11)),
            Some(SimTick::new(20)),
            Some("resource_pantry".to_string()),
            DecisionTraceId::new("trace_routine_breakfast").unwrap(),
        )
    }

    #[test]
    fn routine_template_rejects_non_defeasible_or_invalid_duration_shapes() {
        assert_eq!(
            template(Vec::new(), vec![0], 1, 4).unwrap_err(),
            RoutineTemplateError::MissingFailureModes
        );
        assert_eq!(
            template(vec!["food_missing".to_string()], Vec::new(), 1, 4).unwrap_err(),
            RoutineTemplateError::MissingInterruptionPoints
        );
        assert_eq!(
            template(vec!["food_missing".to_string()], vec![0], 0, 4).unwrap_err(),
            RoutineTemplateError::NonPositiveDuration
        );
        assert_eq!(
            template(vec!["food_missing".to_string()], vec![0], 5, 4).unwrap_err(),
            RoutineTemplateError::InvalidDurationRange
        );
        assert_eq!(
            template(vec!["food_missing".to_string()], vec![1], 1, 4).unwrap_err(),
            RoutineTemplateError::InvalidInterruptionPoint
        );
        assert!(template(vec!["food_missing".to_string()], vec![0], 1, 4).is_ok());
    }

    #[test]
    fn routine_execution_transitions_record_ticks_reasons_and_fallbacks() {
        let mut execution = execution();
        assert_eq!(execution.family, RoutineFamily::EatMeal);

        execution.start_step(SimTick::new(11), action("check_container.pantry"));
        assert_eq!(execution.step_status, RoutineStepStatus::InProgress);
        assert_eq!(execution.last_progress_tick, SimTick::new(11));
        assert_eq!(execution.concrete_action_ancestry.len(), 1);

        execution.record_fallback_attempt();
        execution.record_fallback_attempt();
        assert_eq!(execution.fallback_attempts, 2);

        execution.interrupt(SimTick::new(12), "severe safety pressure");
        assert_eq!(execution.step_status, RoutineStepStatus::Interrupted);
        assert_eq!(execution.last_progress_tick, SimTick::new(12));
        assert_eq!(
            execution.failure_interruption_reason.as_deref(),
            Some("severe safety pressure")
        );

        execution.resume(SimTick::new(13));
        assert_eq!(execution.step_status, RoutineStepStatus::InProgress);
        assert!(execution.failure_interruption_reason.is_none());

        execution.fail(SimTick::new(14), "known food absent");
        assert_eq!(execution.step_status, RoutineStepStatus::Failed);
        assert_eq!(
            execution.failure_interruption_reason.as_deref(),
            Some("known food absent")
        );
    }

    #[test]
    fn routine_steps_round_trip_canonically_and_expose_proposals() {
        let steps = [
            RoutineStep::SelectKnownPlace {
                action_id: action("select.place.home"),
            },
            RoutineStep::MoveTowardPlace {
                action_id: action("move.home.to.market"),
            },
            RoutineStep::OpenKnownContainer {
                action_id: action("open.container.pantry"),
            },
            RoutineStep::CheckKnownContainer {
                action_id: action("check_container.pantry"),
            },
            RoutineStep::ConsumeAccessibleFood {
                action_id: action("eat.food.bread"),
            },
            RoutineStep::StartScheduledSleep {
                action_id: action("sleep.start.bed"),
            },
            RoutineStep::CompleteScheduledSleep {
                action_id: action("sleep.complete.bed"),
            },
            RoutineStep::StartWorkBlock {
                action_id: action("work_block.start.office"),
            },
            RoutineStep::CompleteWorkBlock {
                action_id: action("work_block.complete.office"),
            },
            RoutineStep::WaitUntil {
                reason: "window opens".to_string(),
            },
            RoutineStep::ContinueCurrentStep {
                action_id: action("continue_routine.current"),
            },
            RoutineStep::FallbackToFindFood {
                action_id: action("find_food.public_market"),
            },
            RoutineStep::FailWithTypedDiagnostic {
                diagnostic: "resource:known food absent".to_string(),
            },
        ];

        for step in steps {
            let bytes = step.serialize_canonical_bytes();
            let round_tripped = RoutineStep::deserialize_canonical(&bytes).unwrap();

            assert_eq!(round_tripped, step);
            assert_eq!(round_tripped.serialize_canonical_bytes(), bytes);
            match round_tripped.proposed() {
                RoutineStepProposal::Action(action_id) => assert!(!action_id.as_str().is_empty()),
                RoutineStepProposal::Wait(reason) => assert!(!reason.is_empty()),
                RoutineStepProposal::Diagnostic(diagnostic) => assert!(!diagnostic.is_empty()),
            }
        }
    }

    #[test]
    fn routine_step_vocabulary_has_no_direct_state_mutation_variants() {
        let variant_names = [
            "SelectKnownPlace",
            "MoveTowardPlace",
            "OpenKnownContainer",
            "CheckKnownContainer",
            "ConsumeAccessibleFood",
            "StartScheduledSleep",
            "CompleteScheduledSleep",
            "StartWorkBlock",
            "CompleteWorkBlock",
            "WaitUntil",
            "ContinueCurrentStep",
            "FallbackToFindFood",
            "FailWithTypedDiagnostic",
        ];

        for variant in variant_names {
            assert!(!variant.starts_with("Set"));
            assert!(!variant.contains("Direct"));
            assert_ne!(variant, "SetLocation");
            assert_ne!(variant, "SetNeed");
            assert_ne!(variant, "MoveItemDirect");
        }
    }
}
