use crate::actions::{Proposal, ProposalOrigin};
use crate::agent::{
    generate_candidate_goals_from_agent_state, plan_local_actions, select_goal_and_trace,
    select_phase3a_method, ActorKnownPlanningContext, ApplicabilityResult, BlockerCategory,
    BlockerCode, CandidateGoal, DecisionInput, DecisionTrace, DecisionTraceRecord, GoalKind,
    IntentionLifecycleEffect, LiveCandidateGenerationInput, LocalPlan, LocalPlanFailure,
    LocalPlanRequest, PlannerGoal, ResponsibleLayer, RoutineFamily, RoutineStep,
    StuckDiagnosticRecord, StuckResultingStatus, TypedDiagnosticFields,
};
use crate::ids::{ActorId, EventId, ProposalId, StuckDiagnosticId};
use crate::state::AgentState;
use crate::time::SimTick;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug)]
pub struct ActorDecisionTransactionInput<'a> {
    pub actor_id: ActorId,
    pub decision_tick: SimTick,
    pub agent_state: &'a AgentState,
    pub actor_known_context: &'a ActorKnownPlanningContext,
    pub source_event_ids: Option<&'a BTreeSet<EventId>>,
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
pub struct SelectedGoalBundle {
    pub candidate_goal_id: crate::ids::CandidateGoalId,
    pub decision_trace_id: crate::ids::DecisionTraceId,
    pub intention_transition_id: Option<String>,
    pub selected_method_id: crate::ids::RoutineTemplateId,
    pub local_plan_id: String,
    pub proposal_ancestry: Vec<String>,
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
        if let Some(available_source_event_ids) = input.source_event_ids {
            if let Some(diagnostic) = dangling_provenance_diagnostic(
                &input.actor_id,
                input.decision_tick,
                &actor_known_facts,
                available_source_event_ids,
            ) {
                return ActorDecisionTransactionOutcome::Stuck {
                    diagnostic: Box::new(diagnostic),
                };
            }
        }
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
        let mut candidates = if input.include_idle_fallback {
            generated.candidates
        } else {
            generated
                .candidates
                .into_iter()
                .filter(|candidate| candidate.goal_kind != GoalKind::IdleWithReason)
                .collect()
        };
        let mut last_method_failure: Option<(CandidateGoal, String)> = None;
        let (selection, method) = loop {
            let Some(selection) = select_goal_and_trace(DecisionInput {
                actor_id: input.actor_id.clone(),
                decision_tick: input.decision_tick,
                candidates: candidates.clone(),
                active_intention: active_intention.clone(),
                actor_known_inputs: generated.actor_known_inputs_used.clone(),
            }) else {
                return ActorDecisionTransactionOutcome::Stuck {
                    diagnostic: Box::new(match last_method_failure {
                        Some((goal, error)) => stuck_diagnostic(
                            &input.actor_id,
                            input.decision_tick,
                            Some(&goal),
                            None,
                            "no applicable method",
                            error,
                        ),
                        None => stuck_diagnostic(
                            &input.actor_id,
                            input.decision_tick,
                            None,
                            None,
                            "no applicable candidate",
                            "candidate generation produced no applicable goal",
                        ),
                    }),
                };
            };
            if !selection.trace.hidden_truth_audit_result.actor_known_only {
                return ActorDecisionTransactionOutcome::Stuck {
                    diagnostic: Box::new(stuck_diagnostic_for_hidden_truth_audit(
                        &input.actor_id,
                        input.decision_tick,
                        &selection.selected_goal,
                        &selection.trace,
                    )),
                };
            }

            match select_phase3a_method(
                &selection.selected_goal,
                input.actor_known_context,
                &actor_known_facts,
                input.decision_tick,
            ) {
                Ok(method) => break (selection, method),
                Err(error) => {
                    let failed_goal = selection.selected_goal.clone();
                    let error = format!("{error:?}");
                    for candidate in candidates.iter_mut().filter(|candidate| {
                        candidate.candidate_goal_id == failed_goal.candidate_goal_id
                    }) {
                        candidate.applicability = ApplicabilityResult::Inapplicable;
                        candidate.rejection_reason =
                            Some(format!("method_selection_rejected:{error}"));
                    }
                    last_method_failure = Some((failed_goal, error));
                }
            }
        };
        let method_goal = selection.selected_goal.clone();

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
        let bundle = SelectedGoalBundle {
            candidate_goal_id: method_goal.candidate_goal_id.clone(),
            decision_trace_id: selection.trace.trace_id.clone(),
            intention_transition_id: selection.lifecycle_effects.first().map(
                |effect| match effect {
                    IntentionLifecycleEffect::Continued { intention_id, .. } => {
                        format!("continued:{}", intention_id.as_str())
                    }
                    IntentionLifecycleEffect::Interrupted { intention_id, .. } => {
                        format!("interrupted:{}", intention_id.as_str())
                    }
                    IntentionLifecycleEffect::Adopted { intention, .. } => {
                        format!("adopted:{}", intention.intention_id.as_str())
                    }
                },
            ),
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
                if let Some(sleep_affordance_id) =
                    actor_known_sleep_affordance_id(input.actor_known_context)
                {
                    proposal
                        .parameters
                        .insert("sleep_affordance_id".to_string(), sleep_affordance_id);
                }
            }
        } else {
            proposal.target_ids = planned.target_ids.clone();
        }
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

        ActorDecisionTransactionOutcome::Proposed(Box::new(ActorDecisionProposalOutcome {
            proposal: SealedProposal::seal(proposal),
            decision_trace_record: DecisionTraceRecord::from_trace(&selection.trace),
            decision_trace: selection.trace,
            lifecycle_effects: selection.lifecycle_effects,
            local_plan,
        }))
    }
}

fn actor_known_sleep_affordance_id(context: &ActorKnownPlanningContext) -> Option<String> {
    context
        .actor_known_facts()
        .iter()
        .find(|fact| fact.stable_id() == "actor_knows_sleep_affordance" && fact.is_actor_known())
        .map(|fact| fact.value().to_string())
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

fn dangling_provenance_diagnostic(
    actor_id: &ActorId,
    tick: SimTick,
    actor_known_facts: &[crate::agent::ActorKnownFact],
    available_source_event_ids: &BTreeSet<EventId>,
) -> Option<StuckDiagnosticRecord> {
    let dangling = actor_known_facts
        .iter()
        .flat_map(|fact| fact.source_event_ids())
        .find(|event_id| !available_source_event_ids.contains(*event_id))?;
    Some(
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
            None,
            None,
            None,
            None,
            None,
            None,
            BlockerCategory::Knowledge,
            "provenance dangling",
            "actor decision transaction rejected unresolved actor-known source event",
            format!("dangling_source_event_id={}", dangling.as_str()),
            "typed_stuck_diagnostic",
            StuckResultingStatus::Failed,
        )
        .with_typed_diagnostic(TypedDiagnosticFields {
            responsible_layer: ResponsibleLayer::CandidateGeneration,
            blocker_code: BlockerCode::ProvenanceDangling,
            input_source: "actor_known_context".to_string(),
            actual_source: "actor_decision_transaction".to_string(),
            hidden_truth_referenced: false,
            remediation_hint:
                "rebuild actor-known facts only from events present in the decision log frontier"
                    .to_string(),
        }),
    )
}

fn stuck_diagnostic_for_hidden_truth_audit(
    actor_id: &ActorId,
    tick: SimTick,
    active_goal: &CandidateGoal,
    trace: &DecisionTrace,
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
        None,
        None,
        BlockerCategory::Knowledge,
        "hidden truth input",
        "actor decision transaction rejected forbidden actor-known input provenance",
        trace.hidden_truth_audit_result.notes.clone(),
        "typed_stuck_diagnostic",
        StuckResultingStatus::Failed,
    )
    .with_typed_diagnostic(TypedDiagnosticFields {
        responsible_layer: ResponsibleLayer::CandidateGeneration,
        blocker_code: BlockerCode::HiddenTruthInput,
        input_source: "actor_known_context".to_string(),
        actual_source: "actor_decision_transaction".to_string(),
        hidden_truth_referenced: true,
        remediation_hint: "remove forbidden source classes from action-relevant actor-known inputs"
            .to_string(),
    })
}

fn stuck_diagnostic(
    actor_id: &ActorId,
    tick: SimTick,
    active_goal: Option<&CandidateGoal>,
    routine_step: Option<RoutineStep>,
    concrete_blocker: impl Into<String>,
    debug: impl Into<String>,
) -> StuckDiagnosticRecord {
    let concrete_blocker = concrete_blocker.into();
    let (responsible_layer, blocker_code) = match concrete_blocker.as_str() {
        "no applicable candidate" => (
            ResponsibleLayer::CandidateGeneration,
            BlockerCode::NoApplicableCandidate,
        ),
        "no applicable method" => (
            ResponsibleLayer::MethodSelection,
            BlockerCode::NoApplicableMethod,
        ),
        "empty local plan" => (ResponsibleLayer::LocalPlanning, BlockerCode::EmptyLocalPlan),
        _ => (
            ResponsibleLayer::LocalPlanning,
            BlockerCode::PlannerBudgetExhausted,
        ),
    };
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
    .with_typed_diagnostic(TypedDiagnosticFields {
        responsible_layer,
        blocker_code,
        input_source: "actor_known_context".to_string(),
        actual_source: "actor_decision_transaction".to_string(),
        hidden_truth_referenced: false,
        remediation_hint: "inspect candidate, method, and local-plan typed records".to_string(),
    })
}

fn stuck_diagnostic_for_plan_failure(
    actor_id: &ActorId,
    tick: SimTick,
    active_goal: &CandidateGoal,
    routine_step: Option<RoutineStep>,
    failure: LocalPlanFailure,
) -> StuckDiagnosticRecord {
    let blocker_code = failure.blocker.blocker_code();
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
    .with_typed_diagnostic(TypedDiagnosticFields {
        responsible_layer: ResponsibleLayer::LocalPlanning,
        blocker_code,
        input_source: "actor_known_context".to_string(),
        actual_source: "actor_decision_transaction".to_string(),
        hidden_truth_referenced: false,
        remediation_hint: "inspect local plan trace for actor-known blocker".to_string(),
    })
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
            vec![
                ActorKnownFact::observed_now(
                    actor_id(),
                    "actor_knows_food_source",
                    "food_stew",
                    "test:visible_food",
                    None,
                    test_source(),
                ),
                ActorKnownFact::observed_now(
                    actor_id(),
                    "actor_knows_sleep_place",
                    home.as_str(),
                    "test:visible_sleep_place",
                    None,
                    test_source(),
                ),
                ActorKnownFact::observed_now(
                    actor_id(),
                    "known_route_surface",
                    format!("{}->{}", home.as_str(), workshop.as_str()),
                    "test:visible_route",
                    None,
                    test_source(),
                ),
            ],
        )
    }

    fn context_with_forbidden_input() -> ActorKnownPlanningContext {
        let home = place("home_tomas");
        let workshop = place("workshop_tomas");
        ActorKnownPlanningContext::from_observed_parts(
            actor_id(),
            home.clone(),
            BTreeMap::from([(home.clone(), BTreeSet::from([workshop]))]),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeSet::from(["food_stew".to_string()]),
            BTreeSet::from([home]),
            BTreeMap::new(),
            vec![
                ActorKnownFact::observed_now(
                    actor_id(),
                    "actor_knows_food_source",
                    "food_stew",
                    "test:visible_food",
                    None,
                    test_source(),
                ),
                ActorKnownFact::unproven("audit_probe", "typed_source_class_only"),
            ],
        )
    }

    fn test_source() -> crate::agent::SourceEventIds {
        crate::agent::SourceEventIds::checked(vec![crate::ids::EventId::new(
            "event_test_actor_known",
        )
        .unwrap()])
        .unwrap()
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

    fn agent_state_with_severe_safety_and_hunger() -> AgentState {
        let mut state = AgentState::default();
        state.needs_by_actor.insert(
            actor_id(),
            BTreeMap::from([
                (
                    NeedKind::Safety,
                    NeedState::initial(NeedKind::Safety, 950, NeedChangeCause::TickDelta),
                ),
                (
                    NeedKind::Hunger,
                    NeedState::initial(NeedKind::Hunger, 900, NeedChangeCause::TickDelta),
                ),
            ]),
        );
        state
    }

    fn agent_state_with_fatigue(value: i32) -> AgentState {
        let mut state = AgentState::default();
        state.needs_by_actor.insert(
            actor_id(),
            BTreeMap::from([(
                NeedKind::Fatigue,
                NeedState::initial(NeedKind::Fatigue, value, NeedChangeCause::TickDelta),
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
            source_event_ids: None,
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
            source_event_ids: None,
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

    #[test]
    fn forbidden_actor_known_input_fails_closed_before_proposal() {
        let agent_state = agent_state_with_hunger(900);
        let context = context_with_forbidden_input();

        let outcome = ActorDecisionTransaction::run(ActorDecisionTransactionInput {
            actor_id: actor_id(),
            decision_tick: SimTick::new(15),
            agent_state: &agent_state,
            actor_known_context: &context,
            source_event_ids: None,
            routine_window_family: None,
            include_idle_fallback: true,
        });

        let ActorDecisionTransactionOutcome::Stuck { diagnostic } = outcome else {
            panic!("expected hidden-truth audit stuck diagnostic");
        };

        assert_eq!(diagnostic.concrete_blocker, "hidden truth input");
        assert_eq!(diagnostic.routine_template_id, None);
        assert_eq!(
            diagnostic.typed_diagnostic.responsible_layer,
            ResponsibleLayer::CandidateGeneration
        );
        assert_eq!(
            diagnostic.typed_diagnostic.blocker_code,
            BlockerCode::HiddenTruthInput
        );
        assert_eq!(
            diagnostic.typed_diagnostic.input_source,
            "actor_known_context"
        );
        assert!(diagnostic.typed_diagnostic.hidden_truth_referenced);
        assert!(!diagnostic.debug_only_details.contains("food_stew"));
    }

    #[test]
    fn dangling_actor_known_source_event_fails_closed_before_proposal() {
        let agent_state = agent_state_with_hunger(900);
        let context = known_context();
        let source_event_ids = BTreeSet::new();

        let outcome = ActorDecisionTransaction::run(ActorDecisionTransactionInput {
            actor_id: actor_id(),
            decision_tick: SimTick::new(17),
            agent_state: &agent_state,
            actor_known_context: &context,
            source_event_ids: Some(&source_event_ids),
            routine_window_family: None,
            include_idle_fallback: true,
        });

        let ActorDecisionTransactionOutcome::Stuck { diagnostic } = outcome else {
            panic!("expected dangling provenance stuck diagnostic");
        };

        assert_eq!(diagnostic.concrete_blocker, "provenance dangling");
        assert_eq!(
            diagnostic.typed_diagnostic.blocker_code,
            BlockerCode::ProvenanceDangling
        );
        assert!(diagnostic
            .debug_only_details
            .contains("event_test_actor_known"));
    }

    #[test]
    fn method_fallback_reruns_selection_with_coherent_trace_and_candidate() {
        let agent_state = agent_state_with_severe_safety_and_hunger();
        let context = known_context();

        let outcome = ActorDecisionTransaction::run(ActorDecisionTransactionInput {
            actor_id: actor_id(),
            decision_tick: SimTick::new(16),
            agent_state: &agent_state,
            actor_known_context: &context,
            source_event_ids: None,
            routine_window_family: None,
            include_idle_fallback: true,
        });

        let ActorDecisionTransactionOutcome::Proposed(proposed) = outcome else {
            panic!("expected coherent fallback proposal");
        };

        let selected_goal_id = proposed
            .decision_trace
            .selected_goal_id
            .as_ref()
            .expect("fallback trace selects a goal")
            .as_str()
            .to_string();

        assert!(selected_goal_id.ends_with("_eat"));
        assert_eq!(
            proposed.proposal.parameters().get("candidate_goal_id"),
            Some(&selected_goal_id)
        );
        assert_eq!(
            proposed.proposal.parameters().get("decision_trace_id"),
            Some(&proposed.decision_trace.trace_id.as_str().to_string())
        );
        assert!(
            proposed
                .decision_trace
                .trace_id
                .as_str()
                .contains(&selected_goal_id),
            "rerun trace id must be specific to the selected fallback candidate"
        );
        assert!(proposed
            .decision_trace
            .rejected_goals
            .iter()
            .any(
                |rejected| rejected.stable_ref.ends_with("_leave_unsafe_place")
                    && rejected.reason.starts_with("method_selection_rejected:")
            ));
    }

    #[test]
    fn duplicate_failed_candidates_are_retired_together() {
        let agent_state = agent_state_with_fatigue(900);
        let context = known_context();

        let outcome = ActorDecisionTransaction::run(ActorDecisionTransactionInput {
            actor_id: actor_id(),
            decision_tick: SimTick::new(18),
            agent_state: &agent_state,
            actor_known_context: &context,
            source_event_ids: None,
            routine_window_family: Some(RoutineFamily::SleepNight),
            include_idle_fallback: true,
        });

        let ActorDecisionTransactionOutcome::Stuck { diagnostic } = outcome else {
            panic!("expected fail-closed diagnostic after duplicate method misses");
        };

        assert_eq!(diagnostic.concrete_blocker, "no applicable method");
        assert_eq!(
            diagnostic.typed_diagnostic.responsible_layer,
            ResponsibleLayer::MethodSelection
        );
        assert_eq!(
            diagnostic.typed_diagnostic.blocker_code,
            BlockerCode::NoApplicableMethod
        );
    }
}
