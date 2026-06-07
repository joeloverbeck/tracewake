use crate::agent::{
    family_for_goal, phase3a_routine_templates, CandidateGoal, DecisionOutcome, DecisionTrace,
    HiddenTruthAudit, RoutineExecution, RoutineFamily, RoutineTemplate,
};
use crate::ids::{DecisionTraceId, RoutineExecutionId};
use crate::time::SimTick;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MethodSelectionFailure {
    NoApplicableMethod {
        family: RoutineFamily,
        reason: String,
    },
    PreconditionFailed {
        template_id: String,
        reason: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MethodSelection {
    pub template: RoutineTemplate,
    pub execution: RoutineExecution,
    pub trace: DecisionTrace,
}

pub fn select_phase3a_method(
    selected_goal: &CandidateGoal,
    actor_known_inputs: &[String],
    tick: SimTick,
) -> Result<MethodSelection, MethodSelectionFailure> {
    select_method_from_templates(
        selected_goal,
        actor_known_inputs,
        tick,
        &phase3a_routine_templates(),
    )
}

pub fn select_method_from_templates(
    selected_goal: &CandidateGoal,
    actor_known_inputs: &[String],
    tick: SimTick,
    templates: &[RoutineTemplate],
) -> Result<MethodSelection, MethodSelectionFailure> {
    let family = family_for_goal(selected_goal.goal_kind);
    let mut candidates = templates
        .iter()
        .filter(|template| template.family == family)
        .cloned()
        .collect::<Vec<_>>();
    candidates.sort_by(|left, right| left.template_id.cmp(&right.template_id));
    let Some(template) = candidates
        .into_iter()
        .find(|template| template_applicable(template, actor_known_inputs))
    else {
        return Err(MethodSelectionFailure::NoApplicableMethod {
            family,
            reason: format!("no actor-known method inputs for {}", family.stable_id()),
        });
    };
    if let Some(failed) = template
        .preconditions
        .iter()
        .find(|precondition| precondition.starts_with("fail:"))
    {
        return Err(MethodSelectionFailure::PreconditionFailed {
            template_id: template.template_id.to_string(),
            reason: failed.trim_start_matches("fail:").to_string(),
        });
    }

    let trace_id = DecisionTraceId::new(format!(
        "trace_method_{}_{}_{}",
        selected_goal.actor_id.as_str(),
        tick.value(),
        template.family.stable_id()
    ))
    .unwrap();
    let execution = RoutineExecution::new(
        RoutineExecutionId::new(format!(
            "routine_exec_{}_{}_{}",
            selected_goal.actor_id.as_str(),
            tick.value(),
            template.family.stable_id()
        ))
        .unwrap(),
        selected_goal.actor_id.clone(),
        template.template_id.clone(),
        tick,
        Some(tick.advance_by(template.min_duration_ticks)),
        Some(tick.advance_by(template.max_duration_ticks)),
        template.reservable_resource.clone(),
        trace_id.clone(),
    );
    let trace = DecisionTrace::new(
        trace_id,
        selected_goal.actor_id.clone(),
        tick,
        tick.advance_by(1),
        Vec::new(),
        None,
        vec![selected_goal.clone()],
        Some(selected_goal.candidate_goal_id.clone()),
        Some(template.template_id.clone()),
        Some(execution.execution_id.clone()),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        actor_known_inputs.to_vec(),
        None,
        Some("method_selected".to_string()),
        template.fallback_rules.first().cloned(),
        HiddenTruthAudit {
            actor_known_only: true,
            notes: "method selected from actor-known applicability inputs".to_string(),
        },
        DecisionOutcome::Replanned,
        "deterministic htn method selection".to_string(),
    );
    Ok(MethodSelection {
        template,
        execution,
        trace,
    })
}

pub fn mark_mid_method_failure(
    mut execution: RoutineExecution,
    tick: SimTick,
    reason: &str,
) -> RoutineExecution {
    execution.record_fallback_attempt();
    execution.interrupt(tick, reason);
    execution
}

fn template_applicable(template: &RoutineTemplate, actor_known_inputs: &[String]) -> bool {
    template.applicability_conditions.iter().all(|condition| {
        condition.starts_with("actor_has_")
            || condition.starts_with("active_intention")
            || condition.starts_with("reason_available")
            || actor_known_inputs
                .iter()
                .any(|input| input.contains(condition))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::{CandidateGoalSource, GoalKind, GoalPriority};
    use crate::ids::{ActorId, CandidateGoalId};

    fn actor_id() -> ActorId {
        ActorId::new("actor_tomas").unwrap()
    }

    fn candidate(goal_kind: GoalKind) -> CandidateGoal {
        CandidateGoal::new(
            CandidateGoalId::new(format!("goal_{}", goal_kind.stable_id())).unwrap(),
            actor_id(),
            SimTick::new(10),
            SimTick::new(11),
            CandidateGoalSource::NeedPressure,
            goal_kind,
            GoalPriority::RoutineWindowDuty,
            "test",
            vec!["actor_knows_food_source".to_string()],
            crate::agent::ApplicabilityResult::Applicable,
            None,
            None,
            DecisionTraceId::new(format!("trace_{}", goal_kind.stable_id())).unwrap(),
        )
    }

    #[test]
    fn each_required_family_selects_applicable_method() {
        for goal_kind in [
            GoalKind::Eat,
            GoalKind::FindFood,
            GoalKind::SleepOrRest,
            GoalKind::GoToWork,
            GoalKind::PerformWorkBlock,
            GoalKind::ReturnHome,
            GoalKind::ContinueCurrentIntention,
            GoalKind::IdleWithReason,
        ] {
            let inputs = vec![
                "actor_knows_food_source".to_string(),
                "actor_knows_workplace".to_string(),
                "workplace_assignment_active".to_string(),
                "actor_knows_home".to_string(),
                "actor_knows_sleep_place".to_string(),
                "active_intention_present".to_string(),
                "reason_available".to_string(),
            ];
            let selection =
                select_phase3a_method(&candidate(goal_kind), &inputs, SimTick::new(10)).unwrap();
            assert_eq!(selection.template.family, family_for_goal(goal_kind));
            assert!(selection.trace.hidden_truth_audit_result.actor_known_only);
        }
    }

    #[test]
    fn no_applicable_method_returns_typed_failure() {
        let err =
            select_method_from_templates(&candidate(GoalKind::Eat), &[], SimTick::new(10), &[])
                .unwrap_err();

        assert!(matches!(
            err,
            MethodSelectionFailure::NoApplicableMethod { .. }
        ));
    }

    #[test]
    fn mid_method_failure_records_interruption_and_fallback_attempt() {
        let selection = select_phase3a_method(
            &candidate(GoalKind::Eat),
            &["actor_knows_food_source".to_string()],
            SimTick::new(10),
        )
        .unwrap();
        let failed = mark_mid_method_failure(selection.execution, SimTick::new(11), "food missing");

        assert_eq!(
            failed.step_status,
            crate::agent::RoutineStepStatus::Interrupted
        );
        assert_eq!(failed.fallback_attempts, 1);
        assert_eq!(
            failed.failure_interruption_reason,
            Some("food missing".to_string())
        );
    }

    #[test]
    fn selection_is_deterministic() {
        let goal = candidate(GoalKind::Eat);
        let inputs = vec!["actor_knows_food_source".to_string()];
        let first = select_phase3a_method(&goal, &inputs, SimTick::new(10)).unwrap();
        let second = select_phase3a_method(&goal, &inputs, SimTick::new(10)).unwrap();

        assert_eq!(first.template.template_id, second.template.template_id);
        assert_eq!(
            first.trace.serialize_canonical(),
            second.trace.serialize_canonical()
        );
    }
}
