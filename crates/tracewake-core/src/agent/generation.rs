use crate::agent::{
    ApplicabilityResult, CandidateGoal, CandidateGoalSource, GoalKind, GoalPriority, Intention,
    NeedBand, NeedKind, NeedPressure, NeedState,
};
use crate::ids::{ActorId, CandidateGoalId, DecisionTraceId};
use crate::time::SimTick;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CandidateGenerationInput {
    pub actor_id: ActorId,
    pub decision_tick: SimTick,
    pub needs: Vec<NeedState>,
    pub active_intention: Option<Intention>,
    pub actor_known_inputs: Vec<String>,
    pub routine_window_goal: Option<GoalKind>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CandidateGenerationOutput {
    pub candidates: Vec<CandidateGoal>,
    pub active_needs: Vec<NeedPressure>,
    pub actor_known_inputs_used: Vec<String>,
}

pub fn generate_candidate_goals(input: &CandidateGenerationInput) -> CandidateGenerationOutput {
    let mut candidates = Vec::new();
    let active_needs = input
        .needs
        .iter()
        .map(|need| pressure_for_need(&input.actor_id, need))
        .collect::<Vec<_>>();

    if input.active_intention.is_some() {
        candidates.push(candidate(
            input,
            "continue_current_intention",
            CandidateGoalSource::CurrentIntentionContinuation,
            GoalKind::ContinueCurrentIntention,
            GoalPriority::ActiveIntentionContinuation,
            "active intention remains durable",
            input.actor_known_inputs.clone(),
            ApplicabilityResult::Applicable,
            None,
        ));
    }

    for need in &input.needs {
        match (need.kind(), need.band()) {
            (NeedKind::Hunger, NeedBand::Rising) => candidates.push(hunger_candidate(
                input,
                GoalPriority::MildNeedPressure,
                "hunger is rising",
            )),
            (NeedKind::Hunger, NeedBand::Urgent) => candidates.push(hunger_candidate(
                input,
                GoalPriority::UrgentHungerOrFatigue,
                "hunger is urgent",
            )),
            (NeedKind::Hunger, NeedBand::Severe) => candidates.push(hunger_candidate(
                input,
                GoalPriority::SevereHunger,
                "hunger is severe",
            )),
            (NeedKind::Fatigue, NeedBand::Urgent) => candidates.push(candidate(
                input,
                "sleep_or_rest",
                CandidateGoalSource::NeedPressure,
                GoalKind::SleepOrRest,
                GoalPriority::UrgentHungerOrFatigue,
                "fatigue is urgent",
                input.actor_known_inputs.clone(),
                ApplicabilityResult::Applicable,
                None,
            )),
            (NeedKind::Fatigue, NeedBand::Severe) => candidates.push(candidate(
                input,
                "sleep_or_rest",
                CandidateGoalSource::NeedPressure,
                GoalKind::SleepOrRest,
                GoalPriority::SevereFatigue,
                "fatigue is severe",
                input.actor_known_inputs.clone(),
                ApplicabilityResult::Applicable,
                None,
            )),
            (NeedKind::Safety, NeedBand::Severe) => candidates.push(candidate(
                input,
                "leave_unsafe_place",
                CandidateGoalSource::SafetyInterruption,
                GoalKind::LeaveUnsafePlace,
                GoalPriority::SevereSafety,
                "safety is severe",
                input.actor_known_inputs.clone(),
                ApplicabilityResult::Applicable,
                None,
            )),
            _ => {}
        }
    }

    if let Some(goal_kind) = input.routine_window_goal {
        candidates.push(candidate(
            input,
            goal_kind.stable_id(),
            CandidateGoalSource::RoutineDuty,
            goal_kind,
            GoalPriority::RoutineWindowDuty,
            "routine window is active",
            input.actor_known_inputs.clone(),
            ApplicabilityResult::Applicable,
            None,
        ));
    }

    candidates.push(candidate(
        input,
        "idle_with_reason",
        CandidateGoalSource::Fallback,
        GoalKind::IdleWithReason,
        GoalPriority::IdleWithReason,
        "no higher priority candidate selected",
        input.actor_known_inputs.clone(),
        ApplicabilityResult::Applicable,
        None,
    ));
    candidates.sort();

    CandidateGenerationOutput {
        candidates,
        active_needs,
        actor_known_inputs_used: input.actor_known_inputs.clone(),
    }
}

fn hunger_candidate(
    input: &CandidateGenerationInput,
    priority: GoalPriority,
    reason: &str,
) -> CandidateGoal {
    let knows_food = input
        .actor_known_inputs
        .iter()
        .any(|input| input.starts_with("known_food:"));
    candidate(
        input,
        if knows_food { "eat" } else { "find_food" },
        CandidateGoalSource::NeedPressure,
        if knows_food {
            GoalKind::Eat
        } else {
            GoalKind::FindFood
        },
        priority,
        reason,
        input.actor_known_inputs.clone(),
        ApplicabilityResult::Applicable,
        None,
    )
}

#[allow(clippy::too_many_arguments)]
fn candidate(
    input: &CandidateGenerationInput,
    suffix: &str,
    source: CandidateGoalSource,
    goal_kind: GoalKind,
    priority: GoalPriority,
    reason: &str,
    belief_inputs: Vec<String>,
    applicability: ApplicabilityResult,
    rejection_reason: Option<String>,
) -> CandidateGoal {
    CandidateGoal::new(
        CandidateGoalId::new(format!(
            "goal_{}_{}_{}",
            input.actor_id.as_str(),
            input.decision_tick.value(),
            suffix
        ))
        .unwrap(),
        input.actor_id.clone(),
        input.decision_tick,
        input.decision_tick.advance_by(1),
        source,
        goal_kind,
        priority,
        reason,
        belief_inputs,
        applicability,
        rejection_reason,
        None,
        DecisionTraceId::new(format!(
            "trace_{}_{}_{}",
            input.actor_id.as_str(),
            input.decision_tick.value(),
            suffix
        ))
        .unwrap(),
    )
}

fn pressure_for_need(actor_id: &ActorId, need: &NeedState) -> NeedPressure {
    NeedPressure {
        actor_id: actor_id.clone(),
        need_kind: need.kind(),
        value: need.value(),
        band: need.band(),
        threshold_crossing: None,
        source_ancestry: need.last_change_cause().clone(),
        interrupt_eligible: matches!(need.band(), NeedBand::Urgent | NeedBand::Severe),
        actor_known_explanation: format!(
            "{} is {}",
            need.kind().stable_id(),
            need.band().stable_id()
        ),
        debug_detail: "candidate generation pressure".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::{IntentionSource, NeedChangeCause};
    use crate::ids::{CandidateGoalId, DecisionTraceId, IntentionId, RoutineTemplateId};

    fn actor_id() -> ActorId {
        ActorId::new("actor_tomas").unwrap()
    }

    fn active_intention() -> Intention {
        Intention::adopt(
            IntentionId::new("intention_work").unwrap(),
            actor_id(),
            IntentionSource::RoutineDuty,
            CandidateGoalId::new("goal_work").unwrap(),
            Some(RoutineTemplateId::new("routine_work").unwrap()),
            Some("work_block".to_string()),
            8,
            SimTick::new(10),
            DecisionTraceId::new("trace_existing").unwrap(),
        )
    }

    #[test]
    fn rising_hunger_adds_eat_candidate_without_erasing_active_intention() {
        let output = generate_candidate_goals(&CandidateGenerationInput {
            actor_id: actor_id(),
            decision_tick: SimTick::new(12),
            needs: vec![NeedState::initial(
                NeedKind::Hunger,
                300,
                NeedChangeCause::TickDelta,
            )],
            active_intention: Some(active_intention()),
            actor_known_inputs: vec!["known_food:food_soup_pot".to_string()],
            routine_window_goal: None,
        });

        assert_eq!(
            output.candidates[0].goal_kind,
            GoalKind::ContinueCurrentIntention
        );
        assert!(output
            .candidates
            .iter()
            .any(|candidate| candidate.goal_kind == GoalKind::Eat));
    }

    #[test]
    fn candidate_generation_is_deterministic() {
        let input = CandidateGenerationInput {
            actor_id: actor_id(),
            decision_tick: SimTick::new(12),
            needs: vec![NeedState::initial(
                NeedKind::Fatigue,
                800,
                NeedChangeCause::TickDelta,
            )],
            active_intention: None,
            actor_known_inputs: vec!["known_sleep_place:bed_tomas".to_string()],
            routine_window_goal: Some(GoalKind::GoToWork),
        };

        assert_eq!(
            generate_candidate_goals(&input),
            generate_candidate_goals(&input)
        );
    }

    #[test]
    fn hidden_true_food_location_is_not_used_without_actor_belief() {
        let output = generate_candidate_goals(&CandidateGenerationInput {
            actor_id: actor_id(),
            decision_tick: SimTick::new(12),
            needs: vec![NeedState::initial(
                NeedKind::Hunger,
                800,
                NeedChangeCause::TickDelta,
            )],
            active_intention: None,
            actor_known_inputs: vec!["known_place:kitchen".to_string()],
            routine_window_goal: None,
        });

        assert!(!output
            .actor_known_inputs_used
            .iter()
            .any(|input| input.contains("hidden_food_pantry")));
        assert!(output
            .candidates
            .iter()
            .any(|candidate| candidate.goal_kind == GoalKind::FindFood));
    }
}
