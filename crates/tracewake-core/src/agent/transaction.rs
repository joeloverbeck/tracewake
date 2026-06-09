use crate::actions::{Proposal, ProposalOrigin};
use crate::agent::{
    generate_candidate_goals_from_agent_state, plan_local_actions, select_goal_and_trace,
    select_phase3a_method, ActorKnownPlanningContext, BlockerCategory, CandidateGoal,
    DecisionInput, DecisionTrace, DecisionTraceRecord, GoalKind, IntentionLifecycleEffect,
    LiveCandidateGenerationInput, LocalPlan, LocalPlanFailure, LocalPlanRequest, PlannerGoal,
    RoutineFamily, RoutineStep, StuckDiagnosticRecord, StuckResultingStatus,
};
use crate::ids::{ActorId, ProposalId, StuckDiagnosticId};
use crate::state::AgentState;
use crate::time::SimTick;
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct ActorDecisionTransactionInput<'a> {
    pub actor_id: ActorId,
    pub decision_tick: SimTick,
    pub agent_state: &'a AgentState,
    pub actor_known_context: &'a ActorKnownPlanningContext,
    pub routine_window_family: Option<RoutineFamily>,
    pub include_idle_fallback: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActorDecisionTransactionOutcome {
    Proposed(Box<ActorDecisionProposalOutcome>),
    Stuck {
        diagnostic: Box<StuckDiagnosticRecord>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActorDecisionProposalOutcome {
    pub proposal: SealedProposal,
    pub decision_trace: DecisionTrace,
    pub decision_trace_record: DecisionTraceRecord,
    pub lifecycle_effects: Vec<IntentionLifecycleEffect>,
    pub local_plan: LocalPlan,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SealedProposal {
    proposal: Proposal,
}

impl SealedProposal {
    pub fn seal(proposal: Proposal) -> Self {
        Self { proposal }
    }

    pub fn proposal_id(&self) -> &ProposalId {
        &self.proposal.proposal_id
    }

    pub fn action_id(&self) -> &crate::ids::ActionId {
        &self.proposal.action_id
    }

    pub fn target_ids(&self) -> &[String] {
        &self.proposal.target_ids
    }

    pub fn parameters(&self) -> &BTreeMap<String, String> {
        &self.proposal.parameters
    }

    pub fn into_proposal(self) -> Proposal {
        self.proposal
    }
}

pub struct ActorDecisionTransaction;

impl ActorDecisionTransaction {
    pub fn run(input: ActorDecisionTransactionInput<'_>) -> ActorDecisionTransactionOutcome {
        let active_intention = active_intention_for_actor(input.agent_state, &input.actor_id);
        let actor_known_facts = input.actor_known_context.actor_known_facts().to_vec();
        let generated = generate_candidate_goals_from_agent_state(&LiveCandidateGenerationInput {
            actor_id: input.actor_id.clone(),
            decision_tick: input.decision_tick,
            agent_state: input.agent_state,
            active_intention: active_intention.clone(),
            actor_known_facts: actor_known_facts.clone(),
            routine_window_goal: input
                .routine_window_family
                .and_then(goal_for_routine_family),
        });
        let candidates = if input.include_idle_fallback {
            generated.candidates
        } else {
            generated
                .candidates
                .into_iter()
                .filter(|candidate| candidate.goal_kind != GoalKind::IdleWithReason)
                .collect()
        };
        let candidate_fallbacks = candidates.clone();
        let Some(selection) = select_goal_and_trace(DecisionInput {
            actor_id: input.actor_id.clone(),
            decision_tick: input.decision_tick,
            candidates,
            active_intention,
            actor_known_inputs: generated.actor_known_inputs_used,
        }) else {
            return ActorDecisionTransactionOutcome::Stuck {
                diagnostic: Box::new(stuck_diagnostic(
                    &input.actor_id,
                    input.decision_tick,
                    None,
                    None,
                    "no applicable candidate",
                    "candidate generation produced no applicable goal",
                )),
            };
        };

        let selected_goal = selection.selected_goal.clone();
        let method_goal = match select_phase3a_method(
            &selected_goal,
            input.actor_known_context,
            &actor_known_facts,
            input.decision_tick,
        ) {
            Ok(method) => (selected_goal, method),
            Err(error) => {
                let Some((fallback_candidate, method)) = candidate_fallbacks
                    .iter()
                    .filter(|candidate| {
                        candidate.candidate_goal_id != selection.selected_goal.candidate_goal_id
                    })
                    .find_map(|candidate| {
                        select_phase3a_method(
                            candidate,
                            input.actor_known_context,
                            &actor_known_facts,
                            input.decision_tick,
                        )
                        .ok()
                        .map(|method| (candidate.clone(), method))
                    })
                else {
                    return ActorDecisionTransactionOutcome::Stuck {
                        diagnostic: Box::new(stuck_diagnostic(
                            &input.actor_id,
                            input.decision_tick,
                            Some(&selection.selected_goal),
                            None,
                            "no applicable method",
                            format!("{error:?}"),
                        )),
                    };
                };
                (fallback_candidate, method)
            }
        };
        let (method_goal, method) = method_goal;

        let step =
            method
                .template
                .steps
                .first()
                .cloned()
                .unwrap_or_else(|| RoutineStep::WaitUntil {
                    reason: "empty method".to_string(),
                });
        let request = LocalPlanRequest {
            routine_step: step.clone(),
            goal: planner_goal_for(&method_goal, input.actor_known_context),
            budget: crate::agent::DEFAULT_PLANNER_BUDGET,
            actor_known_facts: actor_known_facts.clone(),
        };
        let local_plan = match plan_local_actions(input.actor_known_context, &request) {
            Ok(plan) => plan,
            Err(failure) => {
                return ActorDecisionTransactionOutcome::Stuck {
                    diagnostic: Box::new(stuck_diagnostic_for_plan_failure(
                        &input.actor_id,
                        input.decision_tick,
                        &method_goal,
                        Some(step),
                        failure,
                    )),
                };
            }
        };
        let Some(planned) = local_plan.proposals.first() else {
            return ActorDecisionTransactionOutcome::Stuck {
                diagnostic: Box::new(stuck_diagnostic(
                    &input.actor_id,
                    input.decision_tick,
                    Some(&method_goal),
                    Some(step),
                    "empty local plan",
                    "selected method produced no concrete proposal",
                )),
            };
        };

        let mut proposal = Proposal::new(
            ProposalId::new(format!(
                "proposal_actor_decision_{}_{}_{}",
                input.actor_id.as_str(),
                input.decision_tick.value(),
                planned.action_id.as_str()
            ))
            .unwrap(),
            ProposalOrigin::Agent,
            Some(input.actor_id.clone()),
            planned.action_id.clone(),
            input.decision_tick,
        );
        if planned.action_id.as_str() == "wait" {
            let reason = planned
                .target_ids
                .first()
                .cloned()
                .unwrap_or_else(|| "actor_decision_idle".to_string());
            proposal.parameters.insert("reason".to_string(), reason);
        } else if planned.action_id.as_str() == "sleep" {
            proposal.target_ids = planned.target_ids.clone();
            if let Some(sleep_place_id) = planned.target_ids.first() {
                proposal
                    .parameters
                    .insert("sleep_place_id".to_string(), sleep_place_id.clone());
            }
        } else {
            proposal.target_ids = planned.target_ids.clone();
        }
        proposal.parameters.insert(
            "decision_trace_id".to_string(),
            selection.trace.trace_id.as_str().to_string(),
        );
        proposal.parameters.insert(
            "candidate_goal_id".to_string(),
            method_goal.candidate_goal_id.as_str().to_string(),
        );
        proposal.parameters.insert(
            "routine_template_id".to_string(),
            method.template.template_id.as_str().to_string(),
        );
        proposal.parameters.insert(
            "routine_execution_id".to_string(),
            method.execution.execution_id.as_str().to_string(),
        );

        ActorDecisionTransactionOutcome::Proposed(Box::new(ActorDecisionProposalOutcome {
            proposal: SealedProposal::seal(proposal),
            decision_trace_record: DecisionTraceRecord::from_trace(&selection.trace),
            decision_trace: selection.trace,
            lifecycle_effects: selection.lifecycle_effects,
            local_plan,
        }))
    }
}

fn active_intention_for_actor(
    agent_state: &AgentState,
    actor_id: &ActorId,
) -> Option<crate::agent::Intention> {
    let intention_id = agent_state.active_intention_by_actor.get(actor_id)?;
    agent_state.intentions.get(intention_id).cloned()
}

fn planner_goal_for(
    selected_goal: &CandidateGoal,
    actor_known_context: &ActorKnownPlanningContext,
) -> PlannerGoal {
    match selected_goal.goal_kind {
        GoalKind::Eat | GoalKind::FindFood => actor_known_context
            .known_food_sources()
            .iter()
            .next()
            .cloned()
            .map(PlannerGoal::EatKnownFood)
            .unwrap_or_else(|| PlannerGoal::WaitWithReason("no_actor_known_food".to_string())),
        GoalKind::GoToWork => actor_known_context
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
        GoalKind::PerformWorkBlock => actor_known_context
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
        GoalKind::SleepOrRest => actor_known_context
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
        GoalKind::ReturnHome => actor_known_context
            .known_sleep_places()
            .iter()
            .next()
            .cloned()
            .map(PlannerGoal::ReachPlace)
            .unwrap_or_else(|| {
                PlannerGoal::WaitWithReason("no_actor_known_sleep_place".to_string())
            }),
        GoalKind::ContinueCurrentIntention
        | GoalKind::IdleWithReason
        | GoalKind::LeaveUnsafePlace => {
            PlannerGoal::WaitWithReason("actor_decision_reevaluation".to_string())
        }
    }
}

fn goal_for_routine_family(family: RoutineFamily) -> Option<GoalKind> {
    match family {
        RoutineFamily::SleepNight => Some(GoalKind::SleepOrRest),
        RoutineFamily::EatMeal => Some(GoalKind::Eat),
        RoutineFamily::FindFood => Some(GoalKind::FindFood),
        RoutineFamily::GoToWork => Some(GoalKind::GoToWork),
        RoutineFamily::WorkBlock => Some(GoalKind::PerformWorkBlock),
        RoutineFamily::ReturnHome => Some(GoalKind::ReturnHome),
        RoutineFamily::ContinueCurrentIntention => Some(GoalKind::ContinueCurrentIntention),
        RoutineFamily::MorningWake | RoutineFamily::Wait | RoutineFamily::IdleWithReason => None,
    }
}

fn stuck_diagnostic(
    actor_id: &ActorId,
    tick: SimTick,
    active_goal: Option<&CandidateGoal>,
    routine_step: Option<RoutineStep>,
    concrete_blocker: impl Into<String>,
    debug: impl Into<String>,
) -> StuckDiagnosticRecord {
    StuckDiagnosticRecord::new(
        StuckDiagnosticId::new(format!(
            "stuck_actor_decision_{}_{}",
            actor_id.as_str(),
            tick.value()
        ))
        .unwrap(),
        actor_id.clone(),
        tick,
        tick.advance_by(1),
        None,
        active_goal.map(|goal| goal.candidate_goal_id.clone()),
        None,
        active_goal.and_then(|goal| goal.selected_routine_method.clone()),
        None,
        routine_step,
        None,
        BlockerCategory::PlannerBudgetExhausted,
        concrete_blocker,
        "actor decision transaction failed closed from actor-known inputs",
        debug,
        "typed_stuck_diagnostic",
        StuckResultingStatus::Failed,
    )
}

fn stuck_diagnostic_for_plan_failure(
    actor_id: &ActorId,
    tick: SimTick,
    active_goal: &CandidateGoal,
    routine_step: Option<RoutineStep>,
    failure: LocalPlanFailure,
) -> StuckDiagnosticRecord {
    StuckDiagnosticRecord::new(
        StuckDiagnosticId::new(format!(
            "stuck_actor_decision_{}_{}",
            actor_id.as_str(),
            tick.value()
        ))
        .unwrap(),
        actor_id.clone(),
        tick,
        tick.advance_by(1),
        None,
        Some(active_goal.candidate_goal_id.clone()),
        None,
        active_goal.selected_routine_method.clone(),
        None,
        routine_step,
        None,
        failure.blocker,
        failure.reason,
        "local plan failed from actor-known inputs",
        format!("{:?}", failure.trace),
        "typed_stuck_diagnostic",
        StuckResultingStatus::Failed,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::{ActorKnownFact, IntentionSource, NeedChangeCause, NeedKind, NeedState};
    use crate::ids::PlaceId;
    use std::collections::{BTreeMap, BTreeSet};

    fn actor_id() -> ActorId {
        ActorId::new("actor_tomas").unwrap()
    }

    fn place(value: &str) -> PlaceId {
        PlaceId::new(value).unwrap()
    }

    fn known_context() -> ActorKnownPlanningContext {
        let home = place("home_tomas");
        let workshop = place("workshop_tomas");
        ActorKnownPlanningContext::from_observed_parts(
            actor_id(),
            home.clone(),
            BTreeMap::from([(home.clone(), BTreeSet::from([workshop.clone()]))]),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeSet::from(["food_stew".to_string()]),
            BTreeSet::from([home.clone()]),
            BTreeMap::new(),
            vec![ActorKnownFact::observed_now(
                actor_id(),
                "actor_knows_food_source",
                "food_stew",
                "test:visible_food",
                None,
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
    fn transaction_links_candidate_intention_method_plan_and_proposal() {
        let agent_state = agent_state_with_hunger(800);
        let context = known_context();

        let outcome = ActorDecisionTransaction::run(ActorDecisionTransactionInput {
            actor_id: actor_id(),
            decision_tick: SimTick::new(12),
            agent_state: &agent_state,
            actor_known_context: &context,
            routine_window_family: None,
            include_idle_fallback: true,
        });

        let ActorDecisionTransactionOutcome::Proposed(proposed) = outcome else {
            panic!("expected proposal");
        };
        let ActorDecisionProposalOutcome {
            proposal,
            decision_trace,
            decision_trace_record,
            lifecycle_effects,
            local_plan,
        } = *proposed;

        assert_eq!(decision_trace_record.trace_id, decision_trace.trace_id);
        assert_eq!(local_plan.trace.selected_plan[0].action_id.as_str(), "eat");
        let selected_goal_id = decision_trace.selected_goal_id.clone().unwrap();
        assert_eq!(
            proposal.parameters().get("candidate_goal_id"),
            Some(&selected_goal_id.as_str().to_string())
        );
        assert_eq!(
            proposal.parameters().get("decision_trace_id"),
            Some(&decision_trace.trace_id.as_str().to_string())
        );
        assert!(proposal.parameters().contains_key("routine_template_id"));
        assert!(proposal.parameters().contains_key("routine_execution_id"));
        assert_eq!(proposal.action_id().as_str(), "eat");
        assert_eq!(proposal.target_ids(), ["food_stew".to_string()]);
        assert!(matches!(
            &lifecycle_effects[..],
            [IntentionLifecycleEffect::Adopted { intention, .. }]
                if intention.source == IntentionSource::NeedPressure
                    && intention.selected_goal_id == selected_goal_id
                    && intention.trace_ancestry == vec![decision_trace.trace_id.clone()]
        ));
    }

    #[test]
    fn empty_candidate_set_yields_typed_stuck_record() {
        let agent_state = AgentState::default();
        let context = known_context();

        let outcome = ActorDecisionTransaction::run(ActorDecisionTransactionInput {
            actor_id: actor_id(),
            decision_tick: SimTick::new(14),
            agent_state: &agent_state,
            actor_known_context: &context,
            routine_window_family: None,
            include_idle_fallback: false,
        });

        let ActorDecisionTransactionOutcome::Stuck { diagnostic } = outcome else {
            panic!("expected stuck diagnostic");
        };

        assert_eq!(diagnostic.actor_id, actor_id());
        assert_eq!(diagnostic.concrete_blocker, "no applicable candidate");
        assert_eq!(diagnostic.resulting_status, StuckResultingStatus::Failed);
    }
}
