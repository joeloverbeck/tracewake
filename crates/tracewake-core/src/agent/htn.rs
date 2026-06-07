use crate::agent::{
    family_for_goal, phase3a_routine_templates, ActorKnownPlanningState, CandidateGoal,
    DecisionOutcome, DecisionTrace, HiddenTruthAudit, RejectedDecisionItem, RoutineCondition,
    RoutineExecution, RoutineFamily, RoutineTemplate,
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
    actor_known_state: &ActorKnownPlanningState,
    actor_known_inputs: &[String],
    tick: SimTick,
) -> Result<MethodSelection, MethodSelectionFailure> {
    select_method_from_templates(
        selected_goal,
        actor_known_state,
        actor_known_inputs,
        tick,
        &phase3a_routine_templates(),
    )
}

pub fn select_method_from_templates(
    selected_goal: &CandidateGoal,
    actor_known_state: &ActorKnownPlanningState,
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
    let mut rejected_methods = Vec::new();
    let mut selected = None;
    let mut selected_proofs = Vec::new();
    for template in candidates {
        let resolution =
            resolve_template_conditions(&template, actor_known_state, actor_known_inputs);
        if resolution.rejected_conditions.is_empty() && resolution.unknown_conditions.is_empty() {
            selected_proofs = resolution.proof_sources;
            selected = Some(template);
            break;
        }
        rejected_methods.extend(resolution.rejected_items(&template));
    }

    let Some(template) = selected else {
        return Err(MethodSelectionFailure::NoApplicableMethod {
            family,
            reason: format!("no actor-known method inputs for {}", family.stable_id()),
        });
    };

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
        rejected_methods,
        Vec::new(),
        selected_proofs.clone(),
        None,
        Some("method_selected".to_string()),
        template.fallback_rules.first().cloned(),
        HiddenTruthAudit {
            actor_known_only: true,
            notes: format!(
                "method selected from actor-known condition proofs={}",
                selected_proofs.join(",")
            ),
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConditionResolution {
    Satisfied { proof_source: String },
    Rejected { reason: String },
    Unknown { reason: String },
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct TemplateConditionResolution {
    proof_sources: Vec<String>,
    rejected_conditions: Vec<(RoutineCondition, String)>,
    unknown_conditions: Vec<(RoutineCondition, String)>,
}

impl TemplateConditionResolution {
    fn rejected_items(&self, template: &RoutineTemplate) -> Vec<RejectedDecisionItem> {
        self.rejected_conditions
            .iter()
            .chain(self.unknown_conditions.iter())
            .map(|(condition, reason)| RejectedDecisionItem {
                stable_ref: format!(
                    "{}:{}",
                    template.template_id.as_str(),
                    condition.stable_id()
                ),
                reason: reason.clone(),
            })
            .collect()
    }
}

fn resolve_template_conditions(
    template: &RoutineTemplate,
    actor_known_state: &ActorKnownPlanningState,
    actor_known_inputs: &[String],
) -> TemplateConditionResolution {
    let mut resolution = TemplateConditionResolution::default();
    for condition in template
        .applicability_conditions
        .iter()
        .chain(template.preconditions.iter())
    {
        match resolve_condition(condition, actor_known_state, actor_known_inputs) {
            ConditionResolution::Satisfied { proof_source } => {
                resolution
                    .proof_sources
                    .push(format!("{}={proof_source}", condition.stable_id()));
            }
            ConditionResolution::Rejected { reason } => {
                resolution.rejected_conditions.push((*condition, reason));
            }
            ConditionResolution::Unknown { reason } => {
                resolution.unknown_conditions.push((*condition, reason));
            }
        }
    }
    resolution.proof_sources.sort();
    resolution
}

pub fn resolve_condition(
    condition: &RoutineCondition,
    actor_known_state: &ActorKnownPlanningState,
    actor_known_inputs: &[String],
) -> ConditionResolution {
    match condition {
        RoutineCondition::ActorKnowsFoodSource => {
            if actor_known_state.known_food_sources.is_empty() {
                return unknown(condition, "no actor-known food source");
            }
            satisfied(condition, "actor_known_state:known_food_sources")
        }
        RoutineCondition::FoodSourceBelievedAccessible => {
            if actor_known_state.known_food_sources.is_empty() {
                return rejected(condition, "food source is not actor-known accessible");
            }
            satisfied(condition, "actor_known_state:known_food_sources")
        }
        RoutineCondition::ActorHasFoodSearchKnowledge => {
            if actor_known_state.known_food_sources.is_empty()
                && !has_exact_input(actor_known_inputs, "food_search_knowledge:local_visible")
            {
                return unknown(condition, "no modeled actor-known food search source");
            }
            satisfied(condition, "actor_known_state:food_search_source")
        }
        RoutineCondition::SearchSurfaceActorKnown => {
            if actor_known_state.known_food_sources.is_empty()
                && actor_known_state.known_containers_by_place.is_empty()
                && !has_exact_input(actor_known_inputs, "food_search_knowledge:local_visible")
            {
                return unknown(condition, "no actor-known search surface");
            }
            satisfied(condition, "actor_known_state:search_surface")
        }
        RoutineCondition::RoutePlannerAvailable => {
            if actor_known_state.known_edges.is_empty()
                && !has_exact_input(actor_known_inputs, condition.stable_id())
            {
                return unknown(condition, "no actor-known route surface");
            }
            satisfied(condition, "actor_known_state:known_edges")
        }
        RoutineCondition::ActorKnowsWorkplace
        | RoutineCondition::WorkplaceAssignmentActive
        | RoutineCondition::ActorKnowsHome
        | RoutineCondition::ActorKnowsSleepPlace
        | RoutineCondition::ActiveIntentionPresent
        | RoutineCondition::ReasonAvailable
        | RoutineCondition::SleepStateCanEnd
        | RoutineCondition::ActorAtWorkplace
        | RoutineCondition::SleepPlaceBelievedAccessible
        | RoutineCondition::NextStepAvailable
        | RoutineCondition::ReevaluationScheduled
        | RoutineCondition::AssignedWorkplaceKnown
        | RoutineCondition::AtWorkplace => {
            if has_exact_input(actor_known_inputs, condition.stable_id()) {
                satisfied(condition, "actor_known_input:exact")
            } else {
                unknown(
                    condition,
                    format!("missing actor-known input {}", condition.stable_id()),
                )
            }
        }
        RoutineCondition::FixtureAuthoredPossibility
        | RoutineCondition::SharedPipelinePreconditions => {
            satisfied(condition, "fixture_seed:validation_allowed")
        }
    }
}

fn has_exact_input(actor_known_inputs: &[String], expected: &str) -> bool {
    actor_known_inputs.iter().any(|input| input == expected)
}

fn satisfied(condition: &RoutineCondition, proof_source: &str) -> ConditionResolution {
    ConditionResolution::Satisfied {
        proof_source: format!("{}:{proof_source}", condition.stable_id()),
    }
}

fn rejected(condition: &RoutineCondition, reason: impl Into<String>) -> ConditionResolution {
    ConditionResolution::Rejected {
        reason: format!("{} rejected: {}", condition.stable_id(), reason.into()),
    }
}

fn unknown(condition: &RoutineCondition, reason: impl Into<String>) -> ConditionResolution {
    ConditionResolution::Unknown {
        reason: format!("{} unknown: {}", condition.stable_id(), reason.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::{CandidateGoalSource, GoalKind, GoalPriority, RoutineStep};
    use crate::ids::{ActorId, CandidateGoalId, PlaceId, RoutineTemplateId};
    use std::collections::{BTreeMap, BTreeSet};

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

    fn planning_state(known_food: &[&str]) -> ActorKnownPlanningState {
        ActorKnownPlanningState {
            actor_id: actor_id(),
            current_place_id: PlaceId::new("home_tomas").unwrap(),
            known_edges: BTreeMap::from([(
                PlaceId::new("home_tomas").unwrap(),
                BTreeSet::from([PlaceId::new("market_square").unwrap()]),
            )]),
            known_closed_doors: BTreeMap::new(),
            known_containers_by_place: BTreeMap::new(),
            known_food_sources: known_food
                .iter()
                .map(|food| (*food).to_string())
                .collect::<BTreeSet<_>>(),
            proof_sources: vec!["test:actor_known_state".to_string()],
        }
    }

    fn wait_step() -> RoutineStep {
        RoutineStep::WaitUntil {
            reason: "test wait".to_string(),
        }
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
                "actor_at_workplace".to_string(),
                "actor_knows_home".to_string(),
                "actor_knows_sleep_place".to_string(),
                "sleep_place_believed_accessible".to_string(),
                "active_intention_present".to_string(),
                "next_step_available".to_string(),
                "reason_available".to_string(),
                "reevaluation_scheduled".to_string(),
            ];
            let selection = select_phase3a_method(
                &candidate(goal_kind),
                &planning_state(&["food_soup"]),
                &inputs,
                SimTick::new(10),
            )
            .unwrap();
            assert_eq!(selection.template.family, family_for_goal(goal_kind));
            assert!(selection.trace.hidden_truth_audit_result.actor_known_only);
            assert!(!selection
                .trace
                .beliefs_perceptions_known_places_used
                .is_empty());
        }
    }

    #[test]
    fn no_applicable_method_returns_typed_failure() {
        let err = select_method_from_templates(
            &candidate(GoalKind::Eat),
            &planning_state(&[]),
            &[],
            SimTick::new(10),
            &[],
        )
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
            &planning_state(&["food_soup"]),
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
        let state = planning_state(&["food_soup"]);
        let first = select_phase3a_method(&goal, &state, &inputs, SimTick::new(10)).unwrap();
        let second = select_phase3a_method(&goal, &state, &inputs, SimTick::new(10)).unwrap();

        assert_eq!(first.template.template_id, second.template.template_id);
        assert_eq!(
            first.trace.serialize_canonical(),
            second.trace.serialize_canonical()
        );
    }

    #[test]
    fn spoofed_actor_has_condition_without_modeled_source_is_rejected() {
        let template = RoutineTemplate::new(
            RoutineTemplateId::new("routine_spoof_find_food").unwrap(),
            RoutineFamily::FindFood,
            vec![RoutineCondition::ActorHasFoodSearchKnowledge],
            vec![RoutineCondition::SearchSurfaceActorKnown],
            vec![wait_step()],
            1,
            1,
            vec![0],
            vec!["no_known_food_sources".to_string()],
            vec!["fallback_wait".to_string()],
            vec!["test".to_string()],
            None,
        )
        .unwrap();
        let err = select_method_from_templates(
            &candidate(GoalKind::FindFood),
            &planning_state(&[]),
            &["actor_has_food_search_knowledge".to_string()],
            SimTick::new(10),
            &[template],
        )
        .unwrap_err();

        let MethodSelectionFailure::NoApplicableMethod { reason, .. } = err else {
            panic!("expected no applicable method");
        };
        assert!(reason.contains("find_food"));
    }

    #[test]
    fn rejected_method_conditions_are_recorded_before_selected_method() {
        let rejected = RoutineTemplate::new(
            RoutineTemplateId::new("routine_a_rejected_find_food").unwrap(),
            RoutineFamily::FindFood,
            vec![RoutineCondition::ActorHasFoodSearchKnowledge],
            vec![RoutineCondition::SearchSurfaceActorKnown],
            vec![wait_step()],
            1,
            1,
            vec![0],
            vec!["no_known_food_sources".to_string()],
            vec!["fallback_wait".to_string()],
            vec!["test".to_string()],
            None,
        )
        .unwrap();
        let selected = RoutineTemplate::new(
            RoutineTemplateId::new("routine_b_selected_wait").unwrap(),
            RoutineFamily::FindFood,
            vec![RoutineCondition::FixtureAuthoredPossibility],
            vec![RoutineCondition::SharedPipelinePreconditions],
            vec![wait_step()],
            1,
            1,
            vec![0],
            vec!["wait".to_string()],
            vec!["fallback_wait".to_string()],
            vec!["test".to_string()],
            None,
        )
        .unwrap();

        let selection = select_method_from_templates(
            &candidate(GoalKind::FindFood),
            &planning_state(&[]),
            &[],
            SimTick::new(10),
            &[selected, rejected],
        )
        .unwrap();

        assert_eq!(
            selection.template.template_id.as_str(),
            "routine_b_selected_wait"
        );
        assert!(selection
            .trace
            .rejected_methods
            .iter()
            .any(|item| item.stable_ref
                == "routine_a_rejected_find_food:actor_has_food_search_knowledge"
                && item.reason.contains("unknown")));
        assert!(selection
            .trace
            .beliefs_perceptions_known_places_used
            .iter()
            .any(|proof| proof.contains("fixture_seed")));
    }
}
