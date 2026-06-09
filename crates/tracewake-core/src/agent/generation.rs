use crate::agent::{
    ActorKnownFact, ActorKnownInputRef, ApplicabilityResult, CandidateGoal, CandidateGoalSource,
    GoalKind, GoalPriority, Intention, NeedBand, NeedKind, NeedPressure, NeedState,
    NeedThresholdCrossing, ThresholdDirection,
};
use crate::ids::{ActorId, CandidateGoalId, DecisionTraceId};
use crate::state::AgentState;
use crate::time::SimTick;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CandidateGenerationInput {
    pub actor_id: ActorId,
    pub decision_tick: SimTick,
    pub needs: Vec<NeedState>,
    pub active_intention: Option<Intention>,
    pub actor_known_facts: Vec<ActorKnownFact>,
    pub routine_window_goal: Option<GoalKind>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CandidateGenerationOutput {
    pub candidates: Vec<CandidateGoal>,
    pub active_needs: Vec<NeedPressure>,
    pub actor_known_inputs_used: Vec<ActorKnownInputRef>,
}

pub struct LiveCandidateGenerationInput<'a> {
    pub actor_id: ActorId,
    pub decision_tick: SimTick,
    pub agent_state: &'a AgentState,
    pub active_intention: Option<Intention>,
    pub actor_known_facts: Vec<ActorKnownFact>,
    pub routine_window_goal: Option<GoalKind>,
}

pub fn generate_candidate_goals_from_agent_state(
    input: &LiveCandidateGenerationInput<'_>,
) -> CandidateGenerationOutput {
    let needs = input
        .agent_state
        .needs_by_actor
        .get(&input.actor_id)
        .map(|needs| needs.values().cloned().collect::<Vec<_>>())
        .unwrap_or_default();

    generate_candidate_goals(&CandidateGenerationInput {
        actor_id: input.actor_id.clone(),
        decision_tick: input.decision_tick,
        needs,
        active_intention: input.active_intention.clone(),
        actor_known_facts: input.actor_known_facts.clone(),
        routine_window_goal: input.routine_window_goal,
    })
}

pub const fn need_crossing_triggers_candidate_reevaluation(
    crossing: &NeedThresholdCrossing,
) -> bool {
    matches!(crossing.direction, ThresholdDirection::IncreasingPressure)
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
            actor_known_fact_notes(&input.actor_known_facts),
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
                actor_known_fact_notes(&input.actor_known_facts),
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
                actor_known_fact_notes(&input.actor_known_facts),
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
                actor_known_fact_notes(&input.actor_known_facts),
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
            actor_known_fact_notes(&input.actor_known_facts),
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
        actor_known_fact_notes(&input.actor_known_facts),
        ApplicabilityResult::Applicable,
        None,
    ));
    candidates.sort();

    CandidateGenerationOutput {
        candidates,
        active_needs,
        actor_known_inputs_used: actor_known_input_refs(&input.actor_known_facts),
    }
}

fn hunger_candidate(
    input: &CandidateGenerationInput,
    priority: GoalPriority,
    reason: &str,
) -> CandidateGoal {
    let knows_food = input
        .actor_known_facts
        .iter()
        .any(|fact| fact.stable_id() == "actor_knows_food_source" && fact.is_actor_known());
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
        actor_known_fact_notes(&input.actor_known_facts),
        ApplicabilityResult::Applicable,
        None,
    )
}

fn actor_known_fact_notes(actor_known_facts: &[ActorKnownFact]) -> Vec<String> {
    actor_known_facts
        .iter()
        .map(ActorKnownFact::proof_note)
        .collect()
}

fn actor_known_input_refs(actor_known_facts: &[ActorKnownFact]) -> Vec<ActorKnownInputRef> {
    actor_known_facts
        .iter()
        .map(ActorKnownInputRef::from_fact)
        .collect()
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
    use crate::state::AgentState;
    use std::collections::BTreeMap;

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

    fn known_food_fact() -> ActorKnownFact {
        ActorKnownFact::observed_now(
            actor_id(),
            "actor_knows_food_source",
            "food_soup",
            "test:visible_food",
            None,
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
            actor_known_facts: vec![known_food_fact()],
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
            actor_known_facts: vec![ActorKnownFact::observed_now(
                actor_id(),
                "actor_knows_sleep_place",
                "home_tomas",
                "test:visible_sleep_place",
                None,
            )],
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
            actor_known_facts: vec![ActorKnownFact::observed_now(
                actor_id(),
                "actor_current_place_visible",
                "home_tomas",
                "test:visible_place",
                None,
            )],
            routine_window_goal: None,
        });

        assert!(!output
            .actor_known_inputs_used
            .iter()
            .any(|input| input.render_for_trace().contains("hidden_food_pantry")));
        assert!(output
            .candidates
            .iter()
            .any(|candidate| candidate.goal_kind == GoalKind::FindFood));
        assert!(!output
            .candidates
            .iter()
            .any(|candidate| candidate.goal_kind == GoalKind::Eat));
    }

    #[test]
    fn unproven_food_fact_does_not_make_eat_applicable() {
        let output = generate_candidate_goals(&CandidateGenerationInput {
            actor_id: actor_id(),
            decision_tick: SimTick::new(12),
            needs: vec![NeedState::initial(
                NeedKind::Hunger,
                800,
                NeedChangeCause::TickDelta,
            )],
            active_intention: None,
            actor_known_facts: vec![ActorKnownFact::unproven(
                "actor_knows_food_source",
                "caller supplied no modeled source",
            )],
            routine_window_goal: None,
        });

        assert!(output
            .candidates
            .iter()
            .any(|candidate| candidate.goal_kind == GoalKind::FindFood));
        assert!(!output
            .candidates
            .iter()
            .any(|candidate| candidate.goal_kind == GoalKind::Eat));
    }

    #[test]
    fn live_agent_state_hunger_generates_need_candidate_without_supplied_need_vector() {
        let mut agent_state = AgentState::default();
        agent_state.needs_by_actor.insert(
            actor_id(),
            BTreeMap::from([(
                NeedKind::Hunger,
                NeedState::initial(NeedKind::Hunger, 760, NeedChangeCause::TickDelta),
            )]),
        );

        let output = generate_candidate_goals_from_agent_state(&LiveCandidateGenerationInput {
            actor_id: actor_id(),
            decision_tick: SimTick::new(20),
            agent_state: &agent_state,
            active_intention: None,
            actor_known_facts: vec![known_food_fact()],
            routine_window_goal: None,
        });

        assert!(output
            .active_needs
            .iter()
            .any(|pressure| pressure.need_kind == NeedKind::Hunger
                && pressure.band == NeedBand::Severe
                && pressure.source_ancestry == NeedChangeCause::TickDelta));
        assert!(output
            .candidates
            .iter()
            .any(|candidate| candidate.goal_kind == GoalKind::Eat
                && candidate.source == CandidateGoalSource::NeedPressure
                && candidate.priority == GoalPriority::SevereHunger));
    }

    #[test]
    fn increasing_threshold_crossing_requests_candidate_reevaluation() {
        let mut hunger = NeedState::initial(NeedKind::Hunger, 490, NeedChangeCause::FixtureInitial);
        let crossing = hunger.apply_delta(25, NeedChangeCause::TickDelta).unwrap();

        assert!(need_crossing_triggers_candidate_reevaluation(&crossing));
        assert_eq!(crossing.from, NeedBand::Rising);
        assert_eq!(crossing.to, NeedBand::Urgent);

        let recovery =
            hunger.apply_delta(-300, NeedChangeCause::ActionEffect("eat".parse().unwrap()));
        assert!(!need_crossing_triggers_candidate_reevaluation(
            &recovery.unwrap()
        ));
    }
}
