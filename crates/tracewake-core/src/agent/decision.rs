use crate::agent::{
    ApplicabilityResult, CandidateGoal, DecisionOutcome, DecisionTrace, HiddenTruthAudit,
    Intention, RejectedDecisionItem,
};
use crate::ids::{ActorId, CandidateGoalId, DecisionTraceId};
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
    let outcome = if selected.goal_kind == crate::agent::GoalKind::ContinueCurrentIntention {
        DecisionOutcome::Continued
    } else if input.active_intention.is_some() {
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
        input.active_intention,
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
    })
}

pub fn selected_goal_id(selection: &DecisionSelection) -> &CandidateGoalId {
    &selection.selected_goal.candidate_goal_id
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
            actor_known_facts: vec![ActorKnownFact::modeled(
                "actor_knows_food_source",
                "test:visible_food",
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
        assert!(selection.trace.hidden_truth_audit_result.actor_known_only);
    }

    #[test]
    fn severe_hunger_switches_and_emits_trace() {
        let selection = decide_for_hunger(800);

        assert_eq!(selection.selected_goal.goal_kind, GoalKind::Eat);
        assert_eq!(selection.trace.outcome, DecisionOutcome::Switched);
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
