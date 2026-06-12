use std::cmp::Ordering;

use crate::ids::{ActorId, CandidateGoalId, DecisionTraceId, RoutineTemplateId};
use crate::time::SimTick;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GoalKind {
    Eat,
    FindFood,
    SleepOrRest,
    GoToWork,
    PerformWorkBlock,
    ReturnHome,
    ContinueCurrentIntention,
    IdleWithReason,
    LeaveUnsafePlace,
}

impl GoalKind {
    pub const fn stable_id(self) -> &'static str {
        match self {
            GoalKind::Eat => "eat",
            GoalKind::FindFood => "find_food",
            GoalKind::SleepOrRest => "sleep_or_rest",
            GoalKind::GoToWork => "go_to_work",
            GoalKind::PerformWorkBlock => "perform_work_block",
            GoalKind::ReturnHome => "return_home",
            GoalKind::ContinueCurrentIntention => "continue_current_intention",
            GoalKind::IdleWithReason => "idle_with_reason",
            GoalKind::LeaveUnsafePlace => "leave_unsafe_place",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CandidateGoalSource {
    NeedPressure,
    RoutineDuty,
    CurrentIntentionContinuation,
    SafetyInterruption,
    FixtureRoutineAssignment,
    Fallback,
}

impl CandidateGoalSource {
    pub const fn stable_id(self) -> &'static str {
        match self {
            CandidateGoalSource::NeedPressure => "need_pressure",
            CandidateGoalSource::RoutineDuty => "routine_duty",
            CandidateGoalSource::CurrentIntentionContinuation => "current_intention_continuation",
            CandidateGoalSource::SafetyInterruption => "safety_interruption",
            CandidateGoalSource::FixtureRoutineAssignment => "fixture_routine_assignment",
            CandidateGoalSource::Fallback => "fallback",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GoalPriority {
    SevereSafety,
    SevereHunger,
    SevereFatigue,
    ActiveIntentionContinuation,
    UrgentHungerOrFatigue,
    RoutineWindowDuty,
    ReturnHomeOrSleepWindow,
    IdleWithReason,
    MildNeedPressure,
}

impl GoalPriority {
    pub const fn stable_id(self) -> &'static str {
        match self {
            GoalPriority::SevereSafety => "severe_safety",
            GoalPriority::SevereHunger => "severe_hunger",
            GoalPriority::SevereFatigue => "severe_fatigue",
            GoalPriority::ActiveIntentionContinuation => "active_intention_continuation",
            GoalPriority::UrgentHungerOrFatigue => "urgent_hunger_or_fatigue",
            GoalPriority::RoutineWindowDuty => "routine_window_duty",
            GoalPriority::ReturnHomeOrSleepWindow => "return_home_or_sleep_window",
            GoalPriority::IdleWithReason => "idle_with_reason",
            GoalPriority::MildNeedPressure => "mild_need_pressure",
        }
    }

    pub const fn selection_rank(self) -> u8 {
        match self {
            GoalPriority::SevereSafety => 0,
            GoalPriority::SevereHunger => 1,
            GoalPriority::SevereFatigue => 2,
            GoalPriority::UrgentHungerOrFatigue => 3,
            GoalPriority::RoutineWindowDuty => 4,
            GoalPriority::ActiveIntentionContinuation => 5,
            GoalPriority::ReturnHomeOrSleepWindow => 6,
            GoalPriority::IdleWithReason => 7,
            GoalPriority::MildNeedPressure => 8,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ApplicabilityResult {
    Applicable,
    Inapplicable,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CandidateGoal {
    pub candidate_goal_id: CandidateGoalId,
    pub actor_id: ActorId,
    pub window_start_tick: SimTick,
    pub window_end_tick: SimTick,
    pub source: CandidateGoalSource,
    pub goal_kind: GoalKind,
    pub priority: GoalPriority,
    pub priority_reason: String,
    pub belief_inputs: Vec<String>,
    pub applicability: ApplicabilityResult,
    pub rejection_reason: Option<String>,
    pub selected_routine_method: Option<RoutineTemplateId>,
    pub trace_id: DecisionTraceId,
}

impl CandidateGoal {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        candidate_goal_id: CandidateGoalId,
        actor_id: ActorId,
        window_start_tick: SimTick,
        window_end_tick: SimTick,
        source: CandidateGoalSource,
        goal_kind: GoalKind,
        priority: GoalPriority,
        priority_reason: impl Into<String>,
        belief_inputs: Vec<String>,
        applicability: ApplicabilityResult,
        rejection_reason: Option<String>,
        selected_routine_method: Option<RoutineTemplateId>,
        trace_id: DecisionTraceId,
    ) -> Self {
        Self {
            candidate_goal_id,
            actor_id,
            window_start_tick,
            window_end_tick,
            source,
            goal_kind,
            priority,
            priority_reason: priority_reason.into(),
            belief_inputs,
            applicability,
            rejection_reason,
            selected_routine_method,
            trace_id,
        }
    }

    fn ordering_key(&self) -> (u8, &ActorId, &CandidateGoalId) {
        (
            self.priority.selection_rank(),
            &self.actor_id,
            &self.candidate_goal_id,
        )
    }
}

impl Ord for CandidateGoal {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ordering_key().cmp(&other.ordering_key())
    }
}

impl PartialOrd for CandidateGoal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: &str) -> CandidateGoalId {
        CandidateGoalId::new(value).unwrap()
    }

    fn actor(value: &str) -> ActorId {
        ActorId::new(value).unwrap()
    }

    fn trace(value: &str) -> DecisionTraceId {
        DecisionTraceId::new(value).unwrap()
    }

    fn candidate(
        candidate_goal_id: &str,
        actor_id: &str,
        priority: GoalPriority,
        goal_kind: GoalKind,
    ) -> CandidateGoal {
        CandidateGoal::new(
            id(candidate_goal_id),
            actor(actor_id),
            SimTick::new(10),
            SimTick::new(11),
            CandidateGoalSource::NeedPressure,
            goal_kind,
            priority,
            priority.stable_id(),
            vec!["belief:item_known_at_place".to_string()],
            ApplicabilityResult::Applicable,
            None,
            None,
            trace(&format!("trace_{candidate_goal_id}")),
        )
    }

    #[test]
    fn goal_and_source_stable_ids_cover_all_variants() {
        let goal_ids = [
            (GoalKind::Eat, "eat"),
            (GoalKind::FindFood, "find_food"),
            (GoalKind::SleepOrRest, "sleep_or_rest"),
            (GoalKind::GoToWork, "go_to_work"),
            (GoalKind::PerformWorkBlock, "perform_work_block"),
            (GoalKind::ReturnHome, "return_home"),
            (
                GoalKind::ContinueCurrentIntention,
                "continue_current_intention",
            ),
            (GoalKind::IdleWithReason, "idle_with_reason"),
            (GoalKind::LeaveUnsafePlace, "leave_unsafe_place"),
        ];
        let source_ids = [
            (CandidateGoalSource::NeedPressure, "need_pressure"),
            (CandidateGoalSource::RoutineDuty, "routine_duty"),
            (
                CandidateGoalSource::CurrentIntentionContinuation,
                "current_intention_continuation",
            ),
            (
                CandidateGoalSource::SafetyInterruption,
                "safety_interruption",
            ),
            (
                CandidateGoalSource::FixtureRoutineAssignment,
                "fixture_routine_assignment",
            ),
            (CandidateGoalSource::Fallback, "fallback"),
        ];

        assert_eq!(
            goal_ids
                .iter()
                .map(|(kind, _)| kind.stable_id())
                .collect::<Vec<_>>(),
            goal_ids
                .iter()
                .map(|(_, stable_id)| *stable_id)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            source_ids
                .iter()
                .map(|(source, _)| source.stable_id())
                .collect::<Vec<_>>(),
            source_ids
                .iter()
                .map(|(_, stable_id)| *stable_id)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn candidate_goal_order_is_total_and_stable() {
        let unsorted = vec![
            candidate(
                "goal_idle",
                "actor_mara",
                GoalPriority::IdleWithReason,
                GoalKind::IdleWithReason,
            ),
            candidate(
                "goal_severe_hunger",
                "actor_mara",
                GoalPriority::SevereHunger,
                GoalKind::Eat,
            ),
            candidate(
                "goal_continue",
                "actor_mara",
                GoalPriority::ActiveIntentionContinuation,
                GoalKind::ContinueCurrentIntention,
            ),
            candidate(
                "goal_severe_safety",
                "actor_mara",
                GoalPriority::SevereSafety,
                GoalKind::LeaveUnsafePlace,
            ),
            candidate(
                "goal_mild_need",
                "actor_mara",
                GoalPriority::MildNeedPressure,
                GoalKind::FindFood,
            ),
            candidate(
                "goal_severe_fatigue",
                "actor_mara",
                GoalPriority::SevereFatigue,
                GoalKind::SleepOrRest,
            ),
        ];

        let mut first = unsorted.clone();
        let mut second = unsorted;
        first.sort();
        second.sort();

        let ordered: Vec<_> = first
            .iter()
            .map(|candidate| candidate.candidate_goal_id.as_str())
            .collect();

        assert_eq!(first, second);
        assert_eq!(
            ordered,
            [
                "goal_severe_safety",
                "goal_severe_hunger",
                "goal_severe_fatigue",
                "goal_continue",
                "goal_idle",
                "goal_mild_need"
            ]
        );
    }

    #[test]
    fn active_intention_continuation_outranks_mild_need_pressure() {
        let mut candidates = [
            candidate(
                "goal_mild_hunger",
                "actor_mara",
                GoalPriority::MildNeedPressure,
                GoalKind::Eat,
            ),
            candidate(
                "goal_continue_work",
                "actor_mara",
                GoalPriority::ActiveIntentionContinuation,
                GoalKind::ContinueCurrentIntention,
            ),
        ];

        candidates.sort();

        assert_eq!(candidates[0].goal_kind, GoalKind::ContinueCurrentIntention);
    }

    #[test]
    fn stable_ids_terminate_ties() {
        let mut candidates = [
            candidate(
                "goal_02",
                "actor_mara",
                GoalPriority::RoutineWindowDuty,
                GoalKind::GoToWork,
            ),
            candidate(
                "goal_01",
                "actor_mara",
                GoalPriority::RoutineWindowDuty,
                GoalKind::PerformWorkBlock,
            ),
        ];

        candidates.sort();

        assert_eq!(candidates[0].candidate_goal_id.as_str(), "goal_01");
        assert_eq!(candidates[1].candidate_goal_id.as_str(), "goal_02");
    }

    #[test]
    fn rejected_candidates_carry_applicability_and_reason() {
        let candidate = CandidateGoal::new(
            id("goal_rejected_food"),
            actor("actor_mara"),
            SimTick::new(10),
            SimTick::new(11),
            CandidateGoalSource::NeedPressure,
            GoalKind::Eat,
            GoalPriority::UrgentHungerOrFatigue,
            "known food was not reachable",
            vec!["belief:pantry_known".to_string()],
            ApplicabilityResult::Inapplicable,
            Some("known food inaccessible from current place".to_string()),
            None,
            trace("trace_rejected_food"),
        );

        assert_eq!(candidate.applicability, ApplicabilityResult::Inapplicable);
        assert!(candidate.rejection_reason.is_some());
    }
}
