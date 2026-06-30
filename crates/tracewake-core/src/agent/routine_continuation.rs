use crate::actions::{Proposal, ProposalOrigin};
use crate::agent::{
    plan_local_actions, ActorKnownFact, ActorKnownPlanningContext, CandidateGoal,
    DecisionSelection, IntentionLifecycleEffect, LocalPlan, LocalPlanFailure, LocalPlanRequest,
    MethodSelection, PlannerGoal, RoutineStep, SealedProposal, SelectedGoalBundle,
    DEFAULT_PLANNER_BUDGET,
};
use crate::ids::{ActionId, ActorId, ProposalId};
use crate::time::SimTick;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoutineContinuationResolution {
    pub proposal: SealedProposal,
    pub local_plan: LocalPlan,
    pub selected_goal_bundle: SelectedGoalBundle,
    pub routine_step: RoutineStep,
}

#[allow(clippy::result_large_err)]
pub fn resolve_routine_step_follow_on(
    actor_id: &ActorId,
    decision_tick: SimTick,
    actor_known_context: &ActorKnownPlanningContext,
    actor_known_facts: &[ActorKnownFact],
    selection: &DecisionSelection,
    method: &MethodSelection,
) -> Result<RoutineContinuationResolution, LocalPlanFailure> {
    let method_goal = selection.selected_goal.clone();
    let step = method
        .template
        .steps
        .first()
        .cloned()
        .unwrap_or_else(|| RoutineStep::WaitUntil {
            reason: "empty method".to_string(),
        });
    let request = LocalPlanRequest {
        routine_step: step.clone(),
        goal: planner_goal_for(&method_goal, actor_known_context),
        budget: DEFAULT_PLANNER_BUDGET,
        actor_known_facts: actor_known_facts.to_vec(),
    };
    let local_plan = plan_local_actions(actor_known_context, &request)?;
    let Some(planned) = local_plan.proposals.first() else {
        return Ok(RoutineContinuationResolution {
            proposal: SealedProposal::seal(empty_plan_proposal(actor_id, decision_tick)),
            local_plan,
            selected_goal_bundle: empty_plan_bundle(selection, method, &method_goal),
            routine_step: step,
        });
    };
    let bundle = SelectedGoalBundle {
        candidate_goal_id: method_goal.candidate_goal_id.clone(),
        decision_trace_id: selection.trace.trace_id.clone(),
        intention_transition_id: selection
            .lifecycle_effects
            .first()
            .map(|effect| match effect {
                IntentionLifecycleEffect::Continued { intention_id, .. } => {
                    format!("continued:{}", intention_id.as_str())
                }
                IntentionLifecycleEffect::Interrupted { intention_id, .. } => {
                    format!("interrupted:{}", intention_id.as_str())
                }
                IntentionLifecycleEffect::Adopted { intention, .. } => {
                    format!("adopted:{}", intention.intention_id.as_str())
                }
            }),
        selected_method_id: method.template.template_id.clone(),
        local_plan_id: format!(
            "local_plan_{}_{}",
            selection.trace.trace_id.as_str(),
            planned.action_id.as_str()
        ),
        proposal_ancestry: vec![
            selection.trace.trace_id.as_str().to_string(),
            method_goal.candidate_goal_id.as_str().to_string(),
            method.template.template_id.as_str().to_string(),
        ],
    };

    let mut proposal = Proposal::new(
        ProposalId::new(format!(
            "proposal_actor_decision_{}_{}_{}",
            actor_id.as_str(),
            decision_tick.value(),
            planned.action_id.as_str()
        ))
        .unwrap(),
        ProposalOrigin::Agent,
        Some(actor_id.clone()),
        planned.action_id.clone(),
        decision_tick,
    );
    assign_planned_targets_and_parameters(
        &mut proposal,
        actor_known_context,
        &planned.action_id,
        &planned.target_ids,
    );
    proposal.parameters.insert(
        "decision_trace_id".to_string(),
        bundle.decision_trace_id.as_str().to_string(),
    );
    proposal.parameters.insert(
        "hidden_truth_audit_actor_known_only".to_string(),
        selection
            .trace
            .hidden_truth_audit_result
            .actor_known_only
            .to_string(),
    );
    proposal.parameters.insert(
        "candidate_goal_id".to_string(),
        bundle.candidate_goal_id.as_str().to_string(),
    );
    proposal.parameters.insert(
        "routine_template_id".to_string(),
        bundle.selected_method_id.as_str().to_string(),
    );
    proposal.parameters.insert(
        "routine_execution_id".to_string(),
        method.execution.execution_id.as_str().to_string(),
    );
    proposal
        .parameters
        .insert("local_plan_id".to_string(), bundle.local_plan_id.clone());
    proposal.parameters.insert(
        "proposal_ancestry".to_string(),
        bundle.proposal_ancestry.join("\n"),
    );

    Ok(RoutineContinuationResolution {
        proposal: SealedProposal::seal(proposal),
        local_plan,
        selected_goal_bundle: bundle,
        routine_step: step,
    })
}

fn assign_planned_targets_and_parameters(
    proposal: &mut Proposal,
    actor_known_context: &ActorKnownPlanningContext,
    action_id: &ActionId,
    target_ids: &[String],
) {
    if action_id.as_str() == "wait" {
        let reason = target_ids
            .first()
            .cloned()
            .unwrap_or_else(|| "actor_decision_idle".to_string());
        proposal.parameters.insert("reason".to_string(), reason);
    } else if action_id.as_str() == "sleep" {
        proposal.target_ids = target_ids.to_vec();
        if let Some(sleep_place_id) = target_ids.first() {
            proposal
                .parameters
                .insert("sleep_place_id".to_string(), sleep_place_id.clone());
            if let Some(sleep_affordance_id) = actor_known_sleep_affordance_id(actor_known_context)
            {
                proposal
                    .parameters
                    .insert("sleep_affordance_id".to_string(), sleep_affordance_id);
            }
        }
    } else {
        proposal.target_ids = target_ids.to_vec();
    }
}

fn empty_plan_proposal(actor_id: &ActorId, decision_tick: SimTick) -> Proposal {
    Proposal::new(
        ProposalId::new(format!(
            "proposal_actor_decision_{}_{}_empty_plan",
            actor_id.as_str(),
            decision_tick.value()
        ))
        .unwrap(),
        ProposalOrigin::Agent,
        Some(actor_id.clone()),
        ActionId::new("wait").unwrap(),
        decision_tick,
    )
}

fn empty_plan_bundle(
    selection: &DecisionSelection,
    method: &MethodSelection,
    method_goal: &CandidateGoal,
) -> SelectedGoalBundle {
    SelectedGoalBundle {
        candidate_goal_id: method_goal.candidate_goal_id.clone(),
        decision_trace_id: selection.trace.trace_id.clone(),
        intention_transition_id: None,
        selected_method_id: method.template.template_id.clone(),
        local_plan_id: format!("local_plan_{}_empty", selection.trace.trace_id.as_str()),
        proposal_ancestry: vec![
            selection.trace.trace_id.as_str().to_string(),
            method_goal.candidate_goal_id.as_str().to_string(),
            method.template.template_id.as_str().to_string(),
        ],
    }
}

fn actor_known_sleep_affordance_id(context: &ActorKnownPlanningContext) -> Option<String> {
    context
        .actor_known_facts()
        .iter()
        .find(|fact| fact.stable_id() == "actor_knows_sleep_affordance" && fact.is_actor_known())
        .map(|fact| fact.value().to_string())
}

fn planner_goal_for(
    selected_goal: &CandidateGoal,
    actor_known_context: &ActorKnownPlanningContext,
) -> PlannerGoal {
    match selected_goal.goal_kind {
        crate::agent::GoalKind::Eat | crate::agent::GoalKind::FindFood => {
            actor_known_food_source_for_goal(actor_known_context)
                .map(PlannerGoal::EatKnownFood)
                .unwrap_or_else(|| PlannerGoal::WaitWithReason("no_actor_known_food".to_string()))
        }
        crate::agent::GoalKind::GoToWork => actor_known_context
            .known_workplaces()
            .values()
            .next()
            .cloned()
            .map(|workplace_place_id| {
                if actor_known_context
                    .known_edges()
                    .get(actor_known_context.current_place_id())
                    .is_some_and(|edges| edges.contains(&workplace_place_id))
                {
                    PlannerGoal::ReachPlace(workplace_place_id)
                } else {
                    actor_known_context
                        .known_edges()
                        .get(actor_known_context.current_place_id())
                        .and_then(|edges| edges.iter().next().cloned())
                        .map(PlannerGoal::ReachPlace)
                        .unwrap_or(PlannerGoal::ReachPlace(workplace_place_id))
                }
            })
            .unwrap_or_else(|| PlannerGoal::WaitWithReason("no_actor_known_workplace".to_string())),
        crate::agent::GoalKind::PerformWorkBlock => actor_known_context
            .known_workplaces()
            .iter()
            .next()
            .map(|(workplace_id, place_id)| {
                if place_id == actor_known_context.current_place_id() {
                    PlannerGoal::StartWorkBlock(workplace_id.as_str().to_string())
                } else {
                    PlannerGoal::ReachPlace(place_id.clone())
                }
            })
            .unwrap_or_else(|| PlannerGoal::WaitWithReason("no_actor_known_workplace".to_string())),
        crate::agent::GoalKind::SleepOrRest => actor_known_context
            .known_sleep_places()
            .iter()
            .next()
            .cloned()
            .map(|sleep_place_id| {
                if &sleep_place_id == actor_known_context.current_place_id() {
                    PlannerGoal::StartSleep(sleep_place_id)
                } else {
                    PlannerGoal::ReachPlace(sleep_place_id)
                }
            })
            .unwrap_or_else(|| {
                PlannerGoal::WaitWithReason("no_actor_known_sleep_place".to_string())
            }),
        crate::agent::GoalKind::ReturnHome => actor_known_context
            .known_sleep_places()
            .iter()
            .next()
            .cloned()
            .map(PlannerGoal::ReachPlace)
            .unwrap_or_else(|| {
                PlannerGoal::WaitWithReason("no_actor_known_sleep_place".to_string())
            }),
        crate::agent::GoalKind::ContinueCurrentIntention
        | crate::agent::GoalKind::IdleWithReason => {
            PlannerGoal::WaitWithReason("actor_decision_reevaluation".to_string())
        }
        crate::agent::GoalKind::LeaveUnsafePlace => PlannerGoal::LeaveUnsafePlace,
    }
}

fn actor_known_food_source_for_goal(context: &ActorKnownPlanningContext) -> Option<String> {
    context
        .actor_known_facts()
        .iter()
        .find(|fact| {
            fact.stable_id() == "actor_knows_food_source"
                && fact.semantic_kind() == "observed_now"
                && fact.is_actor_known()
        })
        .map(|fact| fact.value().to_string())
        .or_else(|| context.known_food_sources().iter().next().cloned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::{
        generate_candidate_goals_from_agent_state, select_goal_and_trace, select_phase3a_method,
        ActorDecisionTransaction, ActorDecisionTransactionInput, ActorDecisionTransactionOutcome,
        ActorKnownFact, ApplicabilityResult, BlockerCategory, CandidateGoal, CandidateGoalSource,
        DecisionInput, DecisionSelection, DecisionTrace, GoalKind, GoalPriority, HiddenTruthAudit,
        LiveCandidateGenerationInput, MethodSelection, NeedChangeCause, NeedKind, NeedState,
        RoutineExecution, RoutineFamily, RoutineTemplate, SourceEventIds,
    };
    use crate::ids::{
        ActorId, CandidateGoalId, DecisionTraceId, EventId, PlaceId, RoutineExecutionId,
        RoutineTemplateId, SemanticActionId,
    };
    use crate::state::AgentState;
    use std::collections::{BTreeMap, BTreeSet};

    fn actor_id() -> ActorId {
        ActorId::new("actor_tomas").unwrap()
    }

    fn place(value: &str) -> PlaceId {
        PlaceId::new(value).unwrap()
    }

    fn source() -> SourceEventIds {
        SourceEventIds::checked(vec![EventId::new("event_test_actor_known").unwrap()]).unwrap()
    }

    fn fact(stable_id: &str, value: &str) -> ActorKnownFact {
        ActorKnownFact::observed_now(
            actor_id(),
            stable_id,
            value,
            "test:resolver",
            Some(SimTick::new(4)),
            source(),
        )
    }

    fn remembered_fact(stable_id: &str, value: &str) -> ActorKnownFact {
        ActorKnownFact::remembered_belief(
            actor_id(),
            stable_id,
            value,
            "test:resolver",
            Some(SimTick::new(4)),
            source(),
        )
    }

    fn decision_selection(goal_kind: GoalKind) -> DecisionSelection {
        let selected_goal = CandidateGoal::new(
            CandidateGoalId::new("candidate_actor_tomas_go_to_work").unwrap(),
            actor_id(),
            SimTick::new(4),
            SimTick::new(5),
            CandidateGoalSource::RoutineDuty,
            goal_kind,
            GoalPriority::RoutineWindowDuty,
            "test routine duty",
            vec!["actor_knows_workplace".to_string()],
            ApplicabilityResult::Applicable,
            None,
            Some(RoutineTemplateId::new("routine_go_to_work").unwrap()),
            DecisionTraceId::new("trace_actor_tomas_go_to_work").unwrap(),
        );
        DecisionSelection {
            selected_goal: selected_goal.clone(),
            trace: DecisionTrace::new(
                DecisionTraceId::new("trace_actor_tomas_go_to_work").unwrap(),
                actor_id(),
                SimTick::new(4),
                SimTick::new(5),
                Vec::new(),
                None,
                vec![selected_goal.clone()],
                Some(selected_goal.candidate_goal_id.clone()),
                Some(RoutineTemplateId::new("routine_go_to_work").unwrap()),
                None,
                Vec::new(),
                Vec::new(),
                Vec::new(),
                vec!["actor_knows_workplace".to_string()],
                Some(SemanticActionId::new("move").unwrap()),
                None,
                None,
                HiddenTruthAudit {
                    actor_known_only: true,
                    notes: "actor-known only".to_string(),
                },
                crate::agent::DecisionOutcome::Switched,
                "test resolver selection",
            ),
            lifecycle_effects: Vec::new(),
        }
    }

    fn method(step: RoutineStep) -> MethodSelection {
        MethodSelection {
            template: RoutineTemplate::new(
                RoutineTemplateId::new("routine_go_to_work").unwrap(),
                RoutineFamily::GoToWork,
                Vec::new(),
                Vec::new(),
                vec![step],
                1,
                4,
                vec![0],
                vec!["blocked".to_string()],
                Vec::new(),
                vec!["test".to_string()],
                None,
            )
            .unwrap(),
            execution: RoutineExecution::new(
                RoutineExecutionId::new("routine_exec_actor_tomas_go_to_work").unwrap(),
                actor_id(),
                RoutineTemplateId::new("routine_go_to_work").unwrap(),
                RoutineFamily::GoToWork,
                SimTick::new(4),
                Some(SimTick::new(5)),
                None,
                None,
                DecisionTraceId::new("trace_method_actor_tomas_go_to_work").unwrap(),
            ),
            trace: DecisionTrace::new(
                DecisionTraceId::new("trace_method_actor_tomas_go_to_work").unwrap(),
                actor_id(),
                SimTick::new(4),
                SimTick::new(5),
                Vec::new(),
                None,
                Vec::new(),
                None,
                Some(RoutineTemplateId::new("routine_go_to_work").unwrap()),
                Some(RoutineExecutionId::new("routine_exec_actor_tomas_go_to_work").unwrap()),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                vec!["actor_knows_workplace".to_string()],
                Some(SemanticActionId::new("move").unwrap()),
                None,
                None,
                HiddenTruthAudit {
                    actor_known_only: true,
                    notes: "actor-known only".to_string(),
                },
                crate::agent::DecisionOutcome::Switched,
                "test resolver method",
            ),
        }
    }

    fn work_context(include_workplace: bool) -> ActorKnownPlanningContext {
        let home = place("home_tomas");
        let workshop = place("workshop_tomas");
        let mut facts = Vec::new();
        if include_workplace {
            facts.push(fact(
                "actor_knows_workplace",
                "workplace_tomas@workshop_tomas",
            ));
            facts.push(fact(
                "known_route_surface",
                &format!("{}->{}", home.as_str(), workshop.as_str()),
            ));
        }
        ActorKnownPlanningContext::from_observed_parts(
            actor_id(),
            home,
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeSet::new(),
            BTreeSet::new(),
            BTreeMap::new(),
            facts,
        )
    }

    fn work_context_without_route() -> ActorKnownPlanningContext {
        let home = place("home_tomas");
        ActorKnownPlanningContext::from_observed_parts(
            actor_id(),
            home,
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeSet::new(),
            BTreeSet::new(),
            BTreeMap::new(),
            vec![fact(
                "actor_knows_workplace",
                "workplace_tomas@workshop_tomas",
            )],
        )
    }

    fn agent_state_with_hunger(value: i32) -> AgentState {
        let mut state = AgentState::default();
        state.needs_by_actor.insert(
            actor_id(),
            BTreeMap::from([(
                NeedKind::Hunger,
                NeedState::initial(NeedKind::Hunger, value, NeedChangeCause::TickDelta),
            )]),
        );
        state
    }

    #[test]
    fn shared_resolver_matches_autonomous_transaction_resolution() {
        let context = ActorKnownPlanningContext::from_observed_parts(
            actor_id(),
            place("home_tomas"),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeSet::new(),
            BTreeSet::new(),
            BTreeMap::new(),
            vec![fact("actor_knows_food_source", "food_stew_home_tomas")],
        );
        let agent_state = agent_state_with_hunger(800);
        let actor_known_facts = context.actor_known_facts().to_vec();
        let generated = generate_candidate_goals_from_agent_state(&LiveCandidateGenerationInput {
            actor_id: actor_id(),
            decision_tick: SimTick::new(4),
            agent_state: &agent_state,
            active_intention: None,
            actor_known_facts: actor_known_facts.clone(),
            routine_window_goal: None,
        });
        let selection = select_goal_and_trace(DecisionInput {
            actor_id: actor_id(),
            decision_tick: SimTick::new(4),
            candidates: generated.candidates,
            active_intention: None,
            actor_known_inputs: generated.actor_known_inputs_used,
        })
        .expect("hunger should select an eat goal");
        let method = select_phase3a_method(
            &selection.selected_goal,
            &context,
            &actor_known_facts,
            SimTick::new(4),
        )
        .expect("actor-known food should select eat method");

        let resolved = resolve_routine_step_follow_on(
            &actor_id(),
            SimTick::new(4),
            &context,
            &actor_known_facts,
            &selection,
            &method,
        )
        .expect("actor-known food should resolve through shared resolver");
        let transaction_outcome = ActorDecisionTransaction::run(ActorDecisionTransactionInput {
            actor_id: actor_id(),
            decision_tick: SimTick::new(4),
            agent_state: &agent_state,
            actor_known_context: &context,
            source_event_ids: None,
            source_event_kinds: None,
            routine_window_family: None,
            include_idle_fallback: true,
        });
        let ActorDecisionTransactionOutcome::Proposed(transaction_proposed) = transaction_outcome
        else {
            panic!("expected autonomous transaction proposal");
        };

        assert_eq!(
            transaction_proposed.proposal.action_id(),
            resolved.proposal.action_id()
        );
        assert_eq!(
            transaction_proposed.proposal.target_ids(),
            resolved.proposal.target_ids()
        );
        assert_eq!(
            transaction_proposed.proposal.parameters(),
            resolved.proposal.parameters()
        );
        assert_eq!(
            transaction_proposed.local_plan.proposals.first().unwrap(),
            resolved.local_plan.proposals.first().unwrap()
        );
        assert_eq!(
            transaction_proposed.selected_goal_bundle,
            resolved.selected_goal_bundle
        );
    }

    #[test]
    fn shared_resolver_resolves_actor_known_go_to_work_move() {
        let context = work_context(true);
        let actor_known_facts = context.actor_known_facts().to_vec();
        let selection = decision_selection(GoalKind::GoToWork);
        let method = method(RoutineStep::MoveTowardPlace {
            action_id: SemanticActionId::new("move").unwrap(),
        });

        let resolved = resolve_routine_step_follow_on(
            &actor_id(),
            SimTick::new(4),
            &context,
            &actor_known_facts,
            &selection,
            &method,
        )
        .expect("actor-known workplace should resolve to move");

        assert_eq!(resolved.proposal.action_id().as_str(), "move");
        assert_eq!(
            resolved.proposal.target_ids(),
            ["workshop_tomas".to_string()]
        );
        assert_eq!(
            resolved
                .local_plan
                .proposals
                .first()
                .unwrap()
                .action_id
                .as_str(),
            resolved.proposal.action_id().as_str()
        );
        assert_eq!(
            resolved.local_plan.proposals.first().unwrap().target_ids,
            resolved.proposal.target_ids()
        );
        assert_eq!(
            resolved.proposal.parameters().get("candidate_goal_id"),
            Some(
                &selection
                    .selected_goal
                    .candidate_goal_id
                    .as_str()
                    .to_string()
            )
        );
        assert_eq!(
            resolved.proposal.parameters().get("routine_template_id"),
            Some(&method.template.template_id.as_str().to_string())
        );
    }

    #[test]
    fn hidden_only_route_yields_knowledge_blocker_not_truth_target() {
        let context = work_context_without_route();
        let actor_known_facts = context.actor_known_facts().to_vec();
        let selection = decision_selection(GoalKind::GoToWork);
        let method = method(RoutineStep::MoveTowardPlace {
            action_id: SemanticActionId::new("move").unwrap(),
        });

        let failure = resolve_routine_step_follow_on(
            &actor_id(),
            SimTick::new(4),
            &context,
            &actor_known_facts,
            &selection,
            &method,
        )
        .expect_err("missing actor-known route should fail closed");

        assert_eq!(failure.blocker, BlockerCategory::Knowledge);
        assert!(failure.reason.contains("no actor-known route"));
        assert!(failure.trace.selected_plan.is_empty());
        assert!(!failure.trace.selected_plan.iter().any(|proposal| proposal
            .target_ids
            .iter()
            .any(|target| target == "workshop_tomas")));
    }

    #[test]
    fn actor_known_food_source_requires_food_source_stable_id() {
        // The food-source fact and a distinct non-food fact are both actor-known
        // and observed-now. The resolver must select the value of the
        // `actor_knows_food_source` fact specifically; a `stable_id != ...`
        // mutant would instead surface the unrelated fact's value, so the two
        // values are kept deliberately different.
        let context = ActorKnownPlanningContext::from_observed_parts(
            actor_id(),
            place("home_tomas"),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeSet::new(),
            BTreeSet::new(),
            BTreeMap::new(),
            vec![
                fact("actor_knows_food_source", "food_stew_home_tomas"),
                fact("actor_knows_sleep_place", "home_tomas"),
            ],
        );

        assert_eq!(
            actor_known_food_source_for_goal(&context),
            Some("food_stew_home_tomas".to_string())
        );
    }

    #[test]
    fn actor_known_food_source_requires_observed_now_semantic_kind() {
        // Two `actor_knows_food_source` facts that differ only by semantic kind
        // and value, both actor-known. The resolver must prefer the
        // `observed_now` fact; a `semantic_kind == "observed_now"` -> `!=` mutant
        // would instead surface the remembered-belief fact's value. (A fast,
        // standalone witness so this is caught before the slower full-sim
        // scenarios that would otherwise only flag it via a timeout.)
        let context = ActorKnownPlanningContext::from_observed_parts(
            actor_id(),
            place("home_tomas"),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeSet::new(),
            BTreeSet::new(),
            BTreeMap::new(),
            vec![
                fact("actor_knows_food_source", "food_observed_home_tomas"),
                remembered_fact("actor_knows_food_source", "food_remembered_home_tomas"),
            ],
        );

        assert_eq!(
            actor_known_food_source_for_goal(&context),
            Some("food_observed_home_tomas".to_string())
        );
    }
}
