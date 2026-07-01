mod support;

use std::collections::{BTreeMap, BTreeSet};

use support::{AgentSeed, PhysicalSeed};
use tracewake_core::agent::{
    generate_candidate_goals_from_agent_state, ActorDecisionTransaction,
    ActorDecisionTransactionInput, ActorDecisionTransactionOutcome, CandidateGoalSource, GoalKind,
    Intention, IntentionSource, LiveCandidateGenerationInput, NeedChangeCause, NeedKind, NeedState,
    NoHumanActorKnownSurfaceBuilder, NoHumanActorKnownSurfaceRequest, RoutineExecution,
    RoutineFamily, RoutineStepStatus,
};
use tracewake_core::epistemics::EpistemicProjection;
use tracewake_core::events::EventKind;
use tracewake_core::ids::{
    ActorId, CandidateGoalId, ContentManifestId, DecisionTraceId, EventId, FoodSupplyId,
    IntentionId, PlaceId, RoutineExecutionId, RoutineTemplateId, WorkplaceId,
};
use tracewake_core::location::Location;
use tracewake_core::state::{ActorBody, FoodSupplyState, WorkplaceState};
use tracewake_core::time::SimTick;

const SCHEDULER_RS: &str = include_str!("../src/scheduler.rs");
const TRANSACTION_RS: &str = include_str!("../src/agent/transaction.rs");
const GENERATION_RS: &str = include_str!("../src/agent/generation.rs");
const TICK: SimTick = SimTick::new(8);

#[test]
fn a1_0059_scheduler_routine_active_intention_authority_workplace_temptation_eat() {
    let fixture = Fixture::new("routine_eat_meal", "eat")
        .with_food()
        .at_known_workplace()
        .build();

    let outcome = run_transaction(&fixture, Some(RoutineFamily::WorkBlock), true);

    assert_proposed_not_routine_duty_override(
        outcome,
        "eat",
        "routine_window_family_ignored_conflicts_with_active_intention",
    );
}

#[test]
fn a2_0059_scheduler_routine_active_intention_authority_workplace_temptation_sleep() {
    let fixture = Fixture::new("routine_sleep_night", "sleep")
        .with_sleep()
        .at_known_workplace()
        .build();

    let outcome = run_transaction(&fixture, Some(RoutineFamily::WorkBlock), true);

    assert_no_routine_duty_override(
        outcome,
        "routine_window_family_ignored_conflicts_with_active_intention",
    );
}

#[test]
fn a3_0059_scheduler_routine_active_intention_authority_inactive_execution_ignored() {
    let decoy = routine_execution(
        "routine_exec_inactive_work",
        "routine_work_block",
        RoutineFamily::WorkBlock,
        RoutineStepStatus::NotStarted,
        SimTick::new(12),
    );
    let fixture = Fixture::new("routine_eat_meal", "eat")
        .with_food()
        .with_execution(decoy)
        .build();

    let generated = generate_with_hint(&fixture, Some(GoalKind::Eat));

    assert_has_authorized_routine_duty(&generated, GoalKind::Eat);
    assert!(!generated.candidates.iter().any(|candidate| {
        candidate.source == CandidateGoalSource::RoutineDuty
            && candidate.goal_kind == GoalKind::PerformWorkBlock
    }));
}

#[test]
fn a4_0059_scheduler_routine_active_intention_authority_resolved_execution_ignored() {
    let decoy = routine_execution(
        "routine_exec_resolved_work",
        "routine_work_block",
        RoutineFamily::WorkBlock,
        RoutineStepStatus::Completed,
        SimTick::new(1),
    );
    let fixture = Fixture::new("routine_eat_meal", "eat")
        .with_food()
        .with_execution(decoy)
        .build();

    let outcome = run_transaction(&fixture, Some(RoutineFamily::WorkBlock), true);

    assert_proposed_not_routine_duty_override(
        outcome,
        "eat",
        "routine_window_family_ignored_conflicts_with_active_intention",
    );
}

#[test]
fn a5_0059_scheduler_routine_active_intention_authority_foreign_execution_ignored() {
    let mut decoy = routine_execution(
        "routine_exec_foreign_work",
        "routine_work_block",
        RoutineFamily::WorkBlock,
        RoutineStepStatus::InProgress,
        SimTick::new(1),
    );
    decoy.actor_id = ActorId::new("actor_foreign").unwrap();
    let fixture = Fixture::new("routine_eat_meal", "eat")
        .with_food()
        .with_execution(decoy)
        .build();

    let outcome = run_transaction(&fixture, Some(RoutineFamily::WorkBlock), true);

    assert_proposed_not_routine_duty_override(
        outcome,
        "eat",
        "routine_window_family_ignored_conflicts_with_active_intention",
    );
}

#[test]
fn a6_0059_scheduler_routine_active_intention_authority_no_active_intention_fails_closed() {
    let fixture = Fixture::without_active().with_food().build();

    let outcome = run_transaction(&fixture, Some(RoutineFamily::WorkBlock), true);

    assert_proposed_not_routine_duty_override(
        outcome,
        "eat",
        "routine_window_family_ignored_without_active_intention",
    );
}

#[test]
fn a7_0059_scheduler_routine_active_intention_authority_malformed_chain_fails_closed() {
    let fixture = Fixture::malformed_active().with_food().build();

    let outcome = run_transaction(&fixture, Some(RoutineFamily::EatMeal), true);

    assert_no_routine_duty_override(
        outcome,
        "routine_window_family_ignored_malformed_active_intention",
    );
}

#[test]
fn a8_0059_scheduler_routine_active_intention_authority_work_route_issue_is_stuck_not_switch() {
    let fixture = Fixture::new("routine_work_block", "work_block")
        .with_workplace_truth_only()
        .without_framing_facts()
        .build();

    let outcome = run_transaction(&fixture, Some(RoutineFamily::GoToWork), false);

    let ObservedOutcome::Stuck {
        concrete_blocker,
        hidden_truth_referenced,
        ..
    } = outcome
    else {
        panic!("expected actor-known stuck outcome for missing route/workplace facts");
    };
    assert_eq!(concrete_blocker, "no applicable method");
    assert!(!hidden_truth_referenced);
}

#[test]
fn a9_0059_scheduler_routine_active_intention_authority_hidden_workplace_truth_only_never_selects_work(
) {
    let fixture = Fixture::new("routine_eat_meal", "eat")
        .with_food()
        .with_workplace_truth_only()
        .build();

    let outcome = run_transaction(&fixture, Some(RoutineFamily::WorkBlock), true);

    assert_proposed_not_routine_duty_override(
        outcome,
        "eat",
        "routine_window_family_ignored_conflicts_with_active_intention",
    );
}

#[test]
fn a10_0059_scheduler_routine_active_intention_authority_conflicting_hint_records_diagnostic() {
    let fixture = Fixture::new("routine_sleep_night", "sleep")
        .with_sleep()
        .build();

    let outcome = run_transaction(&fixture, Some(RoutineFamily::EatMeal), true);

    assert_no_routine_duty_override(
        outcome,
        "routine_window_family_ignored_conflicts_with_active_intention",
    );
}

#[test]
fn fail_closed_0059_scheduler_routine_active_intention_authority_records_typed_non_override_outcomes(
) {
    for fixture in [
        Fixture::without_active().with_food().build(),
        Fixture::missing_index().with_food().build(),
        Fixture::terminal_active().with_food().build(),
        Fixture::malformed_active().with_food().build(),
    ] {
        let outcome = run_transaction(&fixture, Some(RoutineFamily::WorkBlock), true);

        assert!(
            outcome
                .trace_notes()
                .contains("routine_window_family_ignored_"),
            "{outcome:#?}"
        );
        assert!(
            outcome
                .candidate_sources()
                .iter()
                .all(|source| *source != CandidateGoalSource::RoutineDuty.stable_id()),
            "{outcome:#?}"
        );
        assert!(outcome.hidden_truth_actor_known_only(), "{outcome:#?}");
    }
}

#[test]
fn mutation_guard_0059_scheduler_world_step_and_routine_authority_predicates_are_live() {
    let due_loaded = function_body_if_present(SCHEDULER_RS, "fn due_loaded_actor_ids")
        .expect("due_loaded_actor_ids source body must be visible to mutation guard");
    for required in [
        "**next_decision_tick <= target_tick",
        "&& state.actors().contains_key(*actor_id)",
        "&& agent_state.needs_by_actor().contains_key(*actor_id)",
        ".map(|(actor_id, _)| actor_id.clone())",
    ] {
        assert!(
            due_loaded.contains(required),
            "0059 focused mutation guard requires due-loaded predicate snippet: {required}"
        );
    }

    let world_step = function_body_if_present(SCHEDULER_RS, "pub fn transact_world_one_tick")
        .expect("transact_world_one_tick source body must be visible to mutation guard");
    for required in [
        "request.advance.expected_tick != self.current_tick",
        "proposal.action_id.as_str() == \"wait\"",
        "rejected && proposal.origin == ProposalOrigin::Human",
        "if proposal.origin == ProposalOrigin::Human\n                && controlled_pipeline_results",
        "payload_value(event, \"accounting_phase\") == Some(\"world_step\")",
        "world_process_markers_observed += 1",
        "proposal.origin == ProposalOrigin::Human",
        ".is_some_and(|result| result.report.status == ReportStatus::Rejected)",
        ".filter(|actor_id| !controlled_actor_ids.contains(actor_id))",
        ".filter(|actor_id| !deferred_reserved_actor_ids.contains(actor_id))",
        "actor_transactions_attempted += 1",
        "routine_window_family: None",
        "include_idle_fallback: true",
        "} else if !scratch_agent_state.needs_by_actor().contains_key(&actor_id) {",
    ] {
        assert!(
            world_step.contains(required),
            "0059 focused mutation guard requires world-step authority snippet: {required}"
        );
    }
    assert_eq!(
        world_step
            .matches("result.report.status == ReportStatus::Rejected")
            .count(),
        2,
        "0059 focused mutation guard requires both controlled rejection predicates"
    );

    let routine_family = function_body_if_present(SCHEDULER_RS, "fn routine_window_family")
        .expect("routine_window_family source body must be visible to mutation guard");
    for required in [
        "let active = active_intention_for_actor(agent_state, actor_id)?;",
        "if active.actor_id != *actor_id || active.status.is_terminal()",
        "let selected_method = active.selected_routine_method.as_ref()?;",
        ".filter(|execution| &execution.actor_id == actor_id)",
        ".filter(|execution| &execution.template_id == selected_method)",
        "if !execution.step_status.is_resolved()",
        "execution.start_tick <= window.start_tick",
        "execution.start_tick <= window.start_tick\n                    && execution",
        ".is_none_or(|deadline| window.start_tick < deadline)",
        "if family == RoutineFamily::WorkBlock",
        "&& !actor_known_state",
        ".any(|place_id| place_id == actor_known_state.current_place_id())",
    ] {
        assert!(
            routine_family.contains(required),
            "0059 focused mutation guard requires routine-family authority snippet: {required}"
        );
    }

    let family_from_template =
        function_body_if_present(SCHEDULER_RS, "fn routine_family_from_template_id")
            .expect("routine_family_from_template_id source body must be visible");
    for required in [
        "template_id.contains(\"go_to_work\")",
        "template_id.contains(\"work_block\")",
        "template_id.contains(\"eat\")",
        "template_id.contains(\"sleep\")",
    ] {
        assert!(
            family_from_template.contains(required),
            "0059 focused mutation guard requires template family mapping: {required}"
        );
    }

    let eligible =
        function_body_if_present(SCHEDULER_RS, "fn eligible_routine_execution_for_actor")
            .expect("eligible_routine_execution_for_actor source body must be visible");
    for required in [
        ".filter(|(_, execution)| &execution.actor_id == actor_id)",
        "execution.start_tick <= window.start_tick",
        "execution.start_tick <= window.start_tick\n                    && execution",
        ".is_none_or(|deadline| window.start_tick < deadline)",
        "!matches!(",
        "RoutineStepStatus::Completed",
        ".min_by(|(_, left), (_, right)|",
        "left.execution_id.cmp(&right.execution_id)",
    ] {
        assert!(
            eligible.contains(required),
            "0059 focused mutation guard requires eligible-execution bookkeeping snippet: {required}"
        );
    }
}

#[test]
fn mutation_guard_0059_transaction_and_generation_predicates_are_live() {
    let transaction_run = function_body_if_present(TRANSACTION_RS, "pub fn run")
        .expect("ActorDecisionTransaction::run source body must be visible");
    for required in [
        ".filter(|candidate| candidate.goal_kind != GoalKind::IdleWithReason)",
        "routine_window_goal_from_hint(input.routine_window_family, active_intention.as_ref())",
        "routine_window_hint.goal_kind",
        "annotate_ignored_routine_window_hint(&mut selection.trace, diagnostic)",
    ] {
        assert!(
            transaction_run.contains(required),
            "0059 focused mutation guard requires transaction snippet: {required}"
        );
    }

    let generation = function_body_if_present(GENERATION_RS, "pub fn generate_candidate_goals(")
        .expect("generate_candidate_goals source body must be visible");
    for required in [
        "(NeedKind::Hunger, NeedBand::Rising)",
        "(NeedKind::Hunger, NeedBand::Urgent)",
        "(NeedKind::Hunger, NeedBand::Severe)",
        "(NeedKind::Fatigue, NeedBand::Urgent)",
        "(NeedKind::Fatigue, NeedBand::Severe)",
        "(NeedKind::Safety, NeedBand::Severe)",
        "CandidateGoalSource::RoutineDuty",
        "routine_window_goal_matches_active_intention(active, *goal_kind)",
    ] {
        assert!(
            generation.contains(required),
            "0059 focused mutation guard requires candidate-generation snippet: {required}"
        );
    }

    let compatibility = function_body_if_present(
        GENERATION_RS,
        "pub(crate) fn routine_window_goal_matches_active_intention",
    )
    .expect("routine_window_goal_matches_active_intention source body must be visible");
    for required in [
        "Some(active_goal) if active_goal == hint_goal => true",
        "Some(GoalKind::PerformWorkBlock) if hint_goal == GoalKind::GoToWork => true",
        "_ => false",
    ] {
        assert!(
            compatibility.contains(required),
            "0059 focused mutation guard requires routine-window compatibility snippet: {required}"
        );
    }
}

#[derive(Clone)]
struct Fixture {
    active: ActiveShape,
    food_known: bool,
    sleep_known: bool,
    at_known_workplace: bool,
    workplace_truth_only: bool,
    framing_facts: bool,
    executions: Vec<RoutineExecution>,
}

#[derive(Clone)]
enum ActiveShape {
    Live {
        template_id: &'static str,
        current_step: &'static str,
        terminal: bool,
        indexed: bool,
    },
    MissingRecord,
    None,
}

impl Fixture {
    fn new(template_id: &'static str, current_step: &'static str) -> Self {
        Self {
            active: ActiveShape::Live {
                template_id,
                current_step,
                terminal: false,
                indexed: true,
            },
            food_known: false,
            sleep_known: false,
            at_known_workplace: false,
            workplace_truth_only: false,
            framing_facts: true,
            executions: Vec::new(),
        }
    }

    fn without_active() -> Self {
        Self {
            active: ActiveShape::None,
            food_known: false,
            sleep_known: false,
            at_known_workplace: false,
            workplace_truth_only: false,
            framing_facts: true,
            executions: Vec::new(),
        }
    }

    fn missing_index() -> Self {
        Self {
            active: ActiveShape::MissingRecord,
            food_known: false,
            sleep_known: false,
            at_known_workplace: false,
            workplace_truth_only: false,
            framing_facts: true,
            executions: Vec::new(),
        }
    }

    fn terminal_active() -> Self {
        Self {
            active: ActiveShape::Live {
                template_id: "routine_eat_meal",
                current_step: "eat",
                terminal: true,
                indexed: true,
            },
            food_known: false,
            sleep_known: false,
            at_known_workplace: false,
            workplace_truth_only: false,
            framing_facts: true,
            executions: Vec::new(),
        }
    }

    fn malformed_active() -> Self {
        Self::new("routine_unknown", "unknown_step")
    }

    fn with_food(mut self) -> Self {
        self.food_known = true;
        self
    }

    fn with_sleep(mut self) -> Self {
        self.sleep_known = true;
        self
    }

    fn at_known_workplace(mut self) -> Self {
        self.at_known_workplace = true;
        self
    }

    fn with_workplace_truth_only(mut self) -> Self {
        self.workplace_truth_only = true;
        self
    }

    fn without_framing_facts(mut self) -> Self {
        self.framing_facts = false;
        self
    }

    fn with_execution(mut self, execution: RoutineExecution) -> Self {
        self.executions.push(execution);
        self
    }

    fn build(self) -> BuiltFixture {
        let actor_id = actor();
        let home = place("home_authority");
        let workshop = place("workshop_authority");
        let current_place = if self.at_known_workplace {
            workshop.clone()
        } else {
            home.clone()
        };

        let mut physical = PhysicalSeed::default();
        physical.actors_mut().insert(
            actor_id.clone(),
            ActorBody::new(actor_id.clone(), current_place.clone()),
        );
        physical.food_supplies_mut().insert(
            FoodSupplyId::new("food_authority").unwrap(),
            FoodSupplyState {
                food_supply_id: FoodSupplyId::new("food_authority").unwrap(),
                location: Location::AtPlace(current_place.clone()),
                servings: 1,
                hunger_reduction_per_serving: 120,
            },
        );
        if self.at_known_workplace || self.workplace_truth_only {
            let workplace_id = WorkplaceId::new("workplace_authority").unwrap();
            let mut workplace = WorkplaceState::new(
                workplace_id.clone(),
                workshop.clone(),
                1,
                1,
                1,
                900,
                900,
                "output_0059",
            );
            workplace.assigned_actor_ids = BTreeSet::from([actor_id.clone()]);
            physical.workplaces_mut().insert(workplace_id, workplace);
        }
        let physical_state = physical.build();

        let mut seed = AgentSeed::default();
        seed.needs_by_actor_mut().insert(
            actor_id.clone(),
            BTreeMap::from([(
                NeedKind::Hunger,
                NeedState::initial(NeedKind::Hunger, 900, NeedChangeCause::FixtureInitial),
            )]),
        );
        match self.active {
            ActiveShape::Live {
                template_id,
                current_step,
                terminal,
                indexed,
            } => {
                let mut intention = active_intention(template_id, current_step);
                if terminal {
                    intention
                        .complete("test terminal active intention")
                        .unwrap();
                }
                let intention_id = intention.intention_id.clone();
                seed.intentions_mut()
                    .insert(intention_id.clone(), intention);
                if indexed {
                    seed.active_intention_by_actor_mut()
                        .insert(actor_id.clone(), intention_id);
                }
            }
            ActiveShape::MissingRecord => {
                seed.active_intention_by_actor_mut().insert(
                    actor_id.clone(),
                    IntentionId::new("intention_missing_record").unwrap(),
                );
            }
            ActiveShape::None => {}
        }
        for execution in self.executions {
            seed.routine_executions_mut()
                .insert(execution.execution_id.clone(), execution);
        }
        let agent_state = seed.build();

        let mut surface = if self.framing_facts {
            NoHumanActorKnownSurfaceBuilder::from_projection(NoHumanActorKnownSurfaceRequest {
                projection: &EpistemicProjection::new(
                    ContentManifestId::new("manifest_0059_authority").unwrap(),
                ),
                agent_state: &agent_state,
                actor_id: actor_id.clone(),
                current_place_id: current_place.clone(),
                decision_tick: TICK,
                window_id: "0059_authority",
                window_end_tick: TICK.advance_by(1),
                current_place_witness_event_id: Some(event("event_0059_current_place")),
                needs_witness_event_id: Some(event("event_0059_needs")),
                frame_event_id: Some(event("event_0059_frame")),
            })
        } else {
            NoHumanActorKnownSurfaceBuilder::new(
                actor_id.clone(),
                current_place.clone(),
                Some(TICK),
            )
        };
        if self.food_known {
            surface = surface
                .with_test_known_food_source("food_authority", event("event_0059_food_known"));
        }
        if self.sleep_known {
            surface = surface.with_test_known_sleep_place(
                current_place.clone(),
                Some("sleep_0059"),
                event("event_0059_sleep_known"),
            );
        }
        if self.at_known_workplace {
            surface = surface.with_test_known_workplace(
                WorkplaceId::new("workplace_authority").unwrap(),
                workshop,
                true,
                event("event_0059_work_known"),
            );
        }
        let actor_known_context = surface.build(&agent_state).into_context();

        BuiltFixture {
            actor_id,
            agent_state,
            actor_known_context,
            physical_state,
        }
    }
}

struct BuiltFixture {
    actor_id: ActorId,
    agent_state: tracewake_core::state::AgentState,
    actor_known_context: tracewake_core::agent::ActorKnownPlanningContext,
    #[allow(dead_code)]
    physical_state: tracewake_core::state::PhysicalState,
}

#[derive(Debug)]
enum ObservedOutcome {
    Proposed {
        action_id: String,
        target_ids: Vec<String>,
        candidate_sources: Vec<&'static str>,
        routine_duty_goals: Vec<GoalKind>,
        notes: String,
        hidden_truth_actor_known_only: bool,
    },
    Stuck {
        concrete_blocker: String,
        candidate_sources: Vec<&'static str>,
        notes: String,
        hidden_truth_referenced: bool,
    },
}

impl ObservedOutcome {
    fn trace_notes(&self) -> &str {
        match self {
            Self::Proposed { notes, .. } | Self::Stuck { notes, .. } => notes,
        }
    }

    fn candidate_sources(&self) -> &[&'static str] {
        match self {
            Self::Proposed {
                candidate_sources, ..
            }
            | Self::Stuck {
                candidate_sources, ..
            } => candidate_sources,
        }
    }

    fn hidden_truth_actor_known_only(&self) -> bool {
        match self {
            Self::Proposed {
                hidden_truth_actor_known_only,
                ..
            } => *hidden_truth_actor_known_only,
            Self::Stuck {
                hidden_truth_referenced,
                ..
            } => !hidden_truth_referenced,
        }
    }
}

fn run_transaction(
    fixture: &BuiltFixture,
    routine_window_family: Option<RoutineFamily>,
    include_idle_fallback: bool,
) -> ObservedOutcome {
    match ActorDecisionTransaction::run(ActorDecisionTransactionInput {
        actor_id: fixture.actor_id.clone(),
        decision_tick: TICK,
        agent_state: &fixture.agent_state,
        actor_known_context: &fixture.actor_known_context,
        source_event_ids: None,
        source_event_kinds: None,
        routine_window_family,
        include_idle_fallback,
    }) {
        ActorDecisionTransactionOutcome::Proposed(proposed) => {
            let candidate_sources = proposed
                .decision_trace
                .candidate_goals_considered
                .iter()
                .map(|candidate| candidate.source.stable_id())
                .collect::<Vec<_>>();
            let routine_duty_goals = proposed
                .decision_trace
                .candidate_goals_considered
                .iter()
                .filter(|candidate| candidate.source == CandidateGoalSource::RoutineDuty)
                .map(|candidate| candidate.goal_kind)
                .collect::<Vec<_>>();
            ObservedOutcome::Proposed {
                action_id: proposed.proposal.action_id().as_str().to_string(),
                target_ids: proposed.proposal.target_ids().to_vec(),
                candidate_sources,
                routine_duty_goals,
                notes: proposed.decision_trace.hidden_truth_audit_result.notes,
                hidden_truth_actor_known_only: proposed
                    .decision_trace
                    .hidden_truth_audit_result
                    .actor_known_only,
            }
        }
        ActorDecisionTransactionOutcome::Stuck { diagnostic } => ObservedOutcome::Stuck {
            concrete_blocker: diagnostic.concrete_blocker,
            candidate_sources: Vec::new(),
            notes: diagnostic.debug_only_details,
            hidden_truth_referenced: diagnostic.typed_diagnostic.hidden_truth_referenced,
        },
    }
}

fn generate_with_hint(
    fixture: &BuiltFixture,
    routine_window_goal: Option<GoalKind>,
) -> tracewake_core::agent::CandidateGenerationOutput {
    let active = fixture
        .agent_state
        .active_intention_by_actor()
        .get(&fixture.actor_id)
        .and_then(|id| fixture.agent_state.intentions().get(id))
        .cloned();
    generate_candidate_goals_from_agent_state(&LiveCandidateGenerationInput {
        actor_id: fixture.actor_id.clone(),
        decision_tick: TICK,
        agent_state: &fixture.agent_state,
        active_intention: active,
        actor_known_facts: fixture.actor_known_context.actor_known_facts().to_vec(),
        routine_window_goal,
    })
}

fn assert_has_authorized_routine_duty(
    generated: &tracewake_core::agent::CandidateGenerationOutput,
    goal_kind: GoalKind,
) {
    assert!(generated.candidates.iter().any(|candidate| {
        candidate.source == CandidateGoalSource::RoutineDuty && candidate.goal_kind == goal_kind
    }));
}

fn assert_proposed_not_routine_duty_override(
    outcome: ObservedOutcome,
    expected_action_id: &str,
    expected_note: &str,
) {
    let ObservedOutcome::Proposed {
        action_id,
        target_ids,
        routine_duty_goals,
        notes,
        hidden_truth_actor_known_only,
        ..
    } = outcome
    else {
        panic!("expected proposed outcome, got {outcome:#?}");
    };
    assert_eq!(action_id, expected_action_id);
    assert!(!target_ids.is_empty());
    assert!(routine_duty_goals.is_empty(), "{routine_duty_goals:?}");
    assert!(notes.contains(expected_note), "{notes}");
    assert!(hidden_truth_actor_known_only);
}

fn assert_no_routine_duty_override(outcome: ObservedOutcome, expected_note: &str) {
    match outcome {
        ObservedOutcome::Proposed {
            routine_duty_goals,
            notes,
            hidden_truth_actor_known_only,
            ..
        } => {
            assert!(routine_duty_goals.is_empty(), "{routine_duty_goals:?}");
            assert!(notes.contains(expected_note), "{notes}");
            assert!(hidden_truth_actor_known_only);
        }
        ObservedOutcome::Stuck {
            concrete_blocker,
            hidden_truth_referenced,
            ..
        } => {
            assert_eq!(concrete_blocker, "no applicable method");
            assert!(!hidden_truth_referenced);
        }
    }
}

fn active_intention(template_id: &str, current_step: &str) -> Intention {
    Intention::adopt(
        IntentionId::new("intention_0059_active").unwrap(),
        actor(),
        IntentionSource::RoutineDuty,
        CandidateGoalId::new("goal_0059_active").unwrap(),
        Some(RoutineTemplateId::new(template_id).unwrap()),
        Some(current_step.to_string()),
        8,
        TICK,
        DecisionTraceId::new("trace_0059_active").unwrap(),
    )
}

fn routine_execution(
    id: &str,
    template_id: &str,
    family: RoutineFamily,
    status: RoutineStepStatus,
    start_tick: SimTick,
) -> RoutineExecution {
    let mut execution = RoutineExecution::new(
        RoutineExecutionId::new(id).unwrap(),
        actor(),
        RoutineTemplateId::new(template_id).unwrap(),
        family,
        start_tick,
        Some(start_tick.advance_by(1)),
        Some(start_tick.advance_by(8)),
        None,
        DecisionTraceId::new(format!("trace_{id}")).unwrap(),
    );
    execution.step_status = status;
    execution
}

fn actor() -> ActorId {
    ActorId::new("actor_0059_authority").unwrap()
}

fn place(value: &str) -> PlaceId {
    PlaceId::new(value).unwrap()
}

fn event(value: &str) -> EventId {
    EventId::new(value).unwrap()
}

fn function_body_if_present<'a>(source: &'a str, marker: &str) -> Option<&'a str> {
    source
        .split(marker)
        .nth(1)
        .and_then(|after_marker| after_marker.find('{').map(|start| (after_marker, start)))
        .and_then(|(after_marker, start)| {
            let mut depth = 0_i32;
            for (offset, byte) in after_marker[start..].bytes().enumerate() {
                match byte {
                    b'{' => depth += 1,
                    b'}' => {
                        depth -= 1;
                        if depth == 0 {
                            return Some(&after_marker[start..start + offset + 1]);
                        }
                    }
                    _ => {}
                }
            }
            None
        })
}

#[test]
fn event_kind_import_keeps_fail_closed_surface_explicit_for_0059_scheduler_routine_active_intention_authority(
) {
    assert_eq!(
        EventKind::StuckDiagnosticRecorded.stable_id(),
        "stuck_diagnostic_recorded"
    );
}
