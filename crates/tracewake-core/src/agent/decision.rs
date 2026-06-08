use crate::agent::{
    ApplicabilityResult, CandidateGoal, DecisionOutcome, DecisionTrace, HiddenTruthAudit,
    Intention, IntentionSource, RejectedDecisionItem,
};
use crate::ids::{ActorId, CandidateGoalId, DecisionTraceId, IntentionId};
use crate::time::SimTick;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DecisionInput {
    pub actor_id: ActorId,
    pub decision_tick: SimTick,
    pub candidates: Vec<CandidateGoal>,
    pub active_intention: Option<Intention>,
    pub actor_known_inputs: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DecisionSelection {
    pub selected_goal: CandidateGoal,
    pub trace: DecisionTrace,
    pub lifecycle_effects: Vec<IntentionLifecycleEffect>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IntentionLifecycleEffect {
    Continued {
        intention_id: IntentionId,
        reason: String,
    },
    Interrupted {
        intention_id: IntentionId,
        reason: String,
    },
    Adopted {
        intention: Intention,
        reason: String,
    },
}

pub fn select_goal_and_trace(input: DecisionInput) -> Option<DecisionSelection> {
    let mut applicable = input
        .candidates
        .iter()
        .filter(|candidate| candidate.applicability == ApplicabilityResult::Applicable)
        .cloned()
        .collect::<Vec<_>>();
    applicable.sort();
    let selected = applicable.into_iter().next()?;
    let active_intention = input.active_intention.clone();
    let outcome = if selected.goal_kind == crate::agent::GoalKind::ContinueCurrentIntention {
        DecisionOutcome::Continued
    } else if active_intention.is_some() {
        DecisionOutcome::Switched
    } else {
        DecisionOutcome::Continued
    };

    let trace_id = DecisionTraceId::new(format!(
        "trace_decision_{}_{}",
        input.actor_id.as_str(),
        input.decision_tick.value()
    ))
    .unwrap();
    let lifecycle_effects = lifecycle_effects(
        &input.actor_id,
        input.decision_tick,
        &selected,
        active_intention.as_ref(),
        &trace_id,
    );
    let rejected_goals = input
        .candidates
        .iter()
        .filter(|candidate| candidate.candidate_goal_id != selected.candidate_goal_id)
        .map(|candidate| RejectedDecisionItem {
            stable_ref: candidate.candidate_goal_id.to_string(),
            reason: candidate
                .rejection_reason
                .clone()
                .unwrap_or_else(|| "lower_priority".to_string()),
        })
        .collect();
    let trace = DecisionTrace::new(
        trace_id,
        input.actor_id,
        input.decision_tick,
        input.decision_tick.advance_by(1),
        Vec::new(),
        active_intention,
        input.candidates,
        Some(selected.candidate_goal_id.clone()),
        selected.selected_routine_method.clone(),
        None,
        rejected_goals,
        Vec::new(),
        Vec::new(),
        input.actor_known_inputs,
        None,
        Some("selected_by_priority_order".to_string()),
        None,
        HiddenTruthAudit {
            actor_known_only: true,
            notes: "candidate inputs supplied by actor-known generation boundary".to_string(),
        },
        outcome,
        "deterministic candidate selection".to_string(),
    );
    Some(DecisionSelection {
        selected_goal: selected,
        trace,
        lifecycle_effects,
    })
}

pub fn selected_goal_id(selection: &DecisionSelection) -> &CandidateGoalId {
    &selection.selected_goal.candidate_goal_id
}

fn lifecycle_effects(
    actor_id: &ActorId,
    decision_tick: SimTick,
    selected: &CandidateGoal,
    active_intention: Option<&Intention>,
    trace_id: &DecisionTraceId,
) -> Vec<IntentionLifecycleEffect> {
    if selected.goal_kind == crate::agent::GoalKind::ContinueCurrentIntention {
        return active_intention
            .map(|intention| {
                vec![IntentionLifecycleEffect::Continued {
                    intention_id: intention.intention_id.clone(),
                    reason: "selected active intention continuation candidate".to_string(),
                }]
            })
            .unwrap_or_default();
    }

    let mut effects = Vec::new();
    if let Some(intention) = active_intention {
        effects.push(IntentionLifecycleEffect::Interrupted {
            intention_id: intention.intention_id.clone(),
            reason: format!(
                "interrupted by {} candidate {}",
                selected.priority.stable_id(),
                selected.candidate_goal_id.as_str()
            ),
        });
    }

    effects.push(IntentionLifecycleEffect::Adopted {
        intention: Intention::adopt(
            IntentionId::new(format!(
                "intention_{}_{}_{}",
                actor_id.as_str(),
                decision_tick.value(),
                selected.goal_kind.stable_id()
            ))
            .unwrap(),
            actor_id.clone(),
            intention_source_for(selected),
            selected.candidate_goal_id.clone(),
            selected.selected_routine_method.clone(),
            Some(selected.goal_kind.stable_id().to_string()),
            durability_level_for(selected),
            decision_tick,
            trace_id.clone(),
        ),
        reason: "selected candidate becomes active intention".to_string(),
    });
    effects
}

fn intention_source_for(selected: &CandidateGoal) -> IntentionSource {
    match selected.source {
        crate::agent::CandidateGoalSource::NeedPressure => IntentionSource::NeedPressure,
        crate::agent::CandidateGoalSource::RoutineDuty
        | crate::agent::CandidateGoalSource::FixtureRoutineAssignment => {
            IntentionSource::RoutineDuty
        }
        crate::agent::CandidateGoalSource::SafetyInterruption => {
            IntentionSource::SafetyInterruption
        }
        crate::agent::CandidateGoalSource::CurrentIntentionContinuation => {
            IntentionSource::CandidateGoalSelection
        }
        crate::agent::CandidateGoalSource::Fallback => IntentionSource::Fallback,
    }
}

fn durability_level_for(selected: &CandidateGoal) -> u8 {
    match selected.priority {
        crate::agent::GoalPriority::SevereSafety
        | crate::agent::GoalPriority::SevereHunger
        | crate::agent::GoalPriority::SevereFatigue => 9,
        crate::agent::GoalPriority::ActiveIntentionContinuation => 8,
        crate::agent::GoalPriority::UrgentHungerOrFatigue => 7,
        crate::agent::GoalPriority::RoutineWindowDuty
        | crate::agent::GoalPriority::ReturnHomeOrSleepWindow => 6,
        crate::agent::GoalPriority::IdleWithReason
        | crate::agent::GoalPriority::MildNeedPressure => 4,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::generation::{generate_candidate_goals, CandidateGenerationInput};
    use crate::agent::{
        ActorKnownFact, GoalKind, Intention, IntentionSource, NeedChangeCause, NeedKind, NeedState,
    };
    use crate::ids::{CandidateGoalId, IntentionId, RoutineTemplateId};

    fn actor_id() -> ActorId {
        ActorId::new("actor_tomas").unwrap()
    }

    fn active_work_intention() -> Intention {
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

    fn decide_for_hunger(value: i32) -> DecisionSelection {
        let active = active_work_intention();
        let generated = generate_candidate_goals(&CandidateGenerationInput {
            actor_id: actor_id(),
            decision_tick: SimTick::new(12),
            needs: vec![NeedState::initial(
                NeedKind::Hunger,
                value,
                NeedChangeCause::TickDelta,
            )],
            active_intention: Some(active.clone()),
            actor_known_facts: vec![ActorKnownFact::observed_now(
                actor_id(),
                "actor_knows_food_source",
                "food_soup",
                "test:visible_food",
                None,
            )],
            routine_window_goal: None,
        });
        select_goal_and_trace(DecisionInput {
            actor_id: actor_id(),
            decision_tick: SimTick::new(12),
            candidates: generated.candidates,
            active_intention: Some(active),
            actor_known_inputs: generated.actor_known_inputs_used,
        })
        .unwrap()
    }

    #[test]
    fn mild_hunger_continues_active_intention() {
        let selection = decide_for_hunger(300);

        assert_eq!(
            selection.selected_goal.goal_kind,
            GoalKind::ContinueCurrentIntention
        );
        assert_eq!(selection.trace.outcome, DecisionOutcome::Continued);
        assert_eq!(
            selection.lifecycle_effects,
            vec![IntentionLifecycleEffect::Continued {
                intention_id: IntentionId::new("intention_work").unwrap(),
                reason: "selected active intention continuation candidate".to_string(),
            }]
        );
        assert!(selection.trace.hidden_truth_audit_result.actor_known_only);
    }

    #[test]
    fn severe_hunger_switches_and_emits_trace() {
        let selection = decide_for_hunger(800);

        assert_eq!(selection.selected_goal.goal_kind, GoalKind::Eat);
        assert_eq!(selection.trace.outcome, DecisionOutcome::Switched);
        assert!(matches!(
            &selection.lifecycle_effects[..],
            [
                IntentionLifecycleEffect::Interrupted {
                    intention_id,
                    reason
                },
                IntentionLifecycleEffect::Adopted {
                    intention,
                    reason: adoption_reason
                }
            ] if intention_id.as_str() == "intention_work"
                && reason.contains("severe_hunger")
                && intention.source == IntentionSource::NeedPressure
                && intention.selected_goal_id == selection.selected_goal.candidate_goal_id
                && intention.trace_ancestry == vec![selection.trace.trace_id.clone()]
                && adoption_reason == "selected candidate becomes active intention"
        ));
        assert_eq!(
            selection.trace.selected_goal_id,
            Some(selection.selected_goal.candidate_goal_id.clone())
        );
    }

    #[test]
    fn repeated_selection_is_deterministic() {
        let first = decide_for_hunger(800);
        let second = decide_for_hunger(800);

        assert_eq!(selected_goal_id(&first), selected_goal_id(&second));
        assert_eq!(
            first.trace.serialize_canonical(),
            second.trace.serialize_canonical()
        );
    }
}
