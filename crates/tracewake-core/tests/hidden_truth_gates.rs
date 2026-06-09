use std::collections::{BTreeMap, BTreeSet};

mod support;

use support::{AgentSeed, PhysicalSeed};
use tracewake_core::actions::ActionRegistry;
use tracewake_core::agent::{
    build_actor_known_planning_state, plan_local_actions, ActorDecisionTransaction,
    ActorDecisionTransactionInput, ActorDecisionTransactionOutcome, ActorKnownFact,
    LocalPlanRequest, NeedChangeCause, NeedKind, NeedState, PlannerGoal, RoutineFamily,
    RoutineStep,
};
use tracewake_core::epistemics::EpistemicProjection;
use tracewake_core::epistemics::KnowledgeContext;
use tracewake_core::ids::{
    ActorId, ContainerId, ContentManifestId, FoodSupplyId, PlaceId, WorkplaceId,
};
use tracewake_core::location::Location;
use tracewake_core::projections::{build_embodied_view_model, EmbodiedProjectionSource};
use tracewake_core::state::{ActorBody, AgentState, ContainerState, FoodSupplyState, PlaceState};
use tracewake_core::time::SimTick;

const ACTOR_KNOWN_RS: &str = include_str!("../src/agent/actor_known.rs");
const DEBUG_CAPABILITY_RS: &str = include_str!("../src/debug_capability.rs");

fn actor_id() -> ActorId {
    ActorId::new("actor_mara").unwrap()
}

fn place_id(value: &str) -> PlaceId {
    PlaceId::new(value).unwrap()
}

fn workplace_id(value: &str) -> WorkplaceId {
    WorkplaceId::new(value).unwrap()
}

fn container_id(value: &str) -> ContainerId {
    ContainerId::new(value).unwrap()
}

fn food_supply_id(value: &str) -> FoodSupplyId {
    FoodSupplyId::new(value).unwrap()
}

fn registry() -> ActionRegistry {
    let mut registry = ActionRegistry::new();
    registry.register_phase1_inspect_wait();
    registry.register_phase3a_sleep();
    registry.register_phase3a_eat();
    registry.register_phase3a_work();
    registry.register_phase3a_continue_routine();
    registry
}

fn agent_state(hunger: u16) -> AgentState {
    let mut state = AgentSeed::default();
    state.needs_by_actor_mut().insert(
        actor_id(),
        BTreeMap::from([
            (
                NeedKind::Hunger,
                NeedState::initial(
                    NeedKind::Hunger,
                    i32::from(hunger),
                    NeedChangeCause::FixtureInitial,
                ),
            ),
            (
                NeedKind::Fatigue,
                NeedState::initial(NeedKind::Fatigue, 100, NeedChangeCause::FixtureInitial),
            ),
            (
                NeedKind::Safety,
                NeedState::initial(NeedKind::Safety, 100, NeedChangeCause::FixtureInitial),
            ),
        ]),
    );
    state.build()
}

fn context(
    known_edges: BTreeMap<PlaceId, BTreeSet<PlaceId>>,
    known_food_sources: BTreeSet<String>,
    known_workplaces: BTreeMap<WorkplaceId, PlaceId>,
) -> tracewake_core::agent::ActorKnownPlanningState {
    build_actor_known_planning_state(
        &actor_id(),
        &EpistemicProjection::new(ContentManifestId::new("hidden_truth_gate_manifest").unwrap()),
        &agent_state(880),
        &tracewake_core::agent::VisibleLocalPlanningState::new(
            place_id("home_mara"),
            known_edges,
            BTreeMap::new(),
            BTreeMap::new(),
            known_food_sources,
            BTreeSet::new(),
            known_workplaces,
        ),
    )
}

fn proof_sources_are_actor_known(context: &tracewake_core::agent::ActorKnownPlanningState) {
    assert!(
        context.audit_with(&[]).actor_known_only,
        "fixture gate must assert provenance, not display text"
    );
    assert!(!context
        .proof_sources()
        .iter()
        .any(|source| source.contains("debug_omniscience") || source.contains("unproven")));
}

#[test]
fn actor_known_context_unforgeable_from_truth() {
    assert!(
        ACTOR_KNOWN_RS.contains("pub(crate) fn from_observed_parts"),
        "actor-known planning context construction must stay crate-private"
    );
    assert!(
        !ACTOR_KNOWN_RS.contains("pub fn from_observed_parts"),
        "actor-known planning context construction must not become public"
    );
    assert!(
        !ACTOR_KNOWN_RS.contains("impl From<PhysicalState> for ActorKnownPlanningContext")
            && !ACTOR_KNOWN_RS.contains("impl From<&PhysicalState> for ActorKnownPlanningContext")
            && !ACTOR_KNOWN_RS.contains("from_physical_state"),
        "actor-known planning context must not gain a privileged raw-truth constructor"
    );
    assert!(
        ACTOR_KNOWN_RS.contains("```compile_fail")
            && ACTOR_KNOWN_RS.contains("ActorKnownPlanningContext::from_observed_parts")
            && ACTOR_KNOWN_RS.contains("ActorKnownPlanningContext::from(PhysicalState::default())"),
        "actor-known unforgeability must be documented by compile-fail examples"
    );
    assert!(
        DEBUG_CAPABILITY_RS.contains("pub(crate) const fn mint"),
        "debug capability minting must stay crate-private"
    );
    assert!(
        !DEBUG_CAPABILITY_RS.contains("pub const fn mint")
            && !DEBUG_CAPABILITY_RS.contains("pub fn mint"),
        "debug capability minting must not become public"
    );
    assert!(
        DEBUG_CAPABILITY_RS.contains("DebugCapability::mint()")
            && DEBUG_CAPABILITY_RS.contains("```compile_fail"),
        "debug capability minting must be covered by compile-fail documentation"
    );
}

#[test]
fn hidden_food_closed_container_is_not_actor_known_food_source() {
    let context = context(BTreeMap::new(), BTreeSet::new(), BTreeMap::new());
    proof_sources_are_actor_known(&context);
    assert!(!context.known_food_sources().contains("food_hidden_pantry"));

    let failure = plan_local_actions(
        &context,
        &LocalPlanRequest {
            routine_step: RoutineStep::ConsumeAccessibleFood {
                action_id: "eat".parse().unwrap(),
            },
            goal: PlannerGoal::EatKnownFood("food_hidden_pantry".to_string()),
            budget: 1,
            actor_known_facts: Vec::new(),
        },
    )
    .unwrap_err();
    assert_eq!(failure.reason, "food source is not actor-known");
    assert!(failure.trace.hidden_truth_audit_result.actor_known_only);
}

#[test]
fn embodied_affordances_exclude_hidden_food_in_closed_container() {
    let mut world = PhysicalSeed::default();
    world.places_mut().insert(
        place_id("home_mara"),
        PlaceState::new(place_id("home_mara"), "Mara home"),
    );
    world.actors_mut().insert(
        actor_id(),
        ActorBody::new(actor_id(), place_id("home_mara")),
    );

    world.food_supplies_mut().insert(
        food_supply_id("food_empty_pantry_mara"),
        FoodSupplyState::new(
            food_supply_id("food_empty_pantry_mara"),
            Location::AtPlace(place_id("home_mara")),
            0,
            180,
        ),
    );
    let hidden_container_id = container_id("hidden_pantry");
    world.containers_mut().insert(
        hidden_container_id.clone(),
        ContainerState::fixed_at_place(hidden_container_id.clone(), place_id("home_mara")),
    );
    world.food_supplies_mut().insert(
        food_supply_id("food_hidden_pantry"),
        FoodSupplyState::new(
            food_supply_id("food_hidden_pantry"),
            Location::InContainer(hidden_container_id),
            1,
            220,
        ),
    );
    let world = world.build();

    let knowledge_context = KnowledgeContext::embodied(actor_id(), SimTick::ZERO);
    let projection_source =
        EmbodiedProjectionSource::from_sealed_context(&knowledge_context, &world, None);
    let view = build_embodied_view_model(
        &knowledge_context,
        &projection_source,
        &registry(),
        &ContentManifestId::new("hidden_truth_gate_manifest").unwrap(),
        None,
    )
    .unwrap();

    assert!(view
        .semantic_actions
        .iter()
        .any(|entry| { entry.semantic_action_id.as_str() == "eat.food.food_empty_pantry_mara" }));
    assert!(!view.semantic_actions.iter().any(|entry| {
        entry
            .semantic_action_id
            .as_str()
            .contains("food_hidden_pantry")
            || entry.label.contains("food_hidden_pantry")
            || entry
                .target_ids
                .iter()
                .any(|target| target == "food_hidden_pantry")
            || entry
                .availability
                .actor_safe_summary()
                .is_some_and(|why| why.contains("food_hidden_pantry"))
    }));
}

#[test]
fn hidden_food_unknown_route_does_not_become_transaction_target() {
    let agent_state = agent_state(920);
    let context = context(BTreeMap::new(), BTreeSet::new(), BTreeMap::new());
    let outcome = ActorDecisionTransaction::run(ActorDecisionTransactionInput {
        actor_id: actor_id(),
        decision_tick: SimTick::ZERO,
        agent_state: &agent_state,
        actor_known_context: &context,
        routine_window_family: Some(RoutineFamily::EatMeal),
        include_idle_fallback: true,
    });
    match outcome {
        ActorDecisionTransactionOutcome::Proposed(proposed) => {
            assert_ne!(proposed.proposal.action_id.as_str(), "eat");
            assert!(!proposed
                .proposal
                .target_ids
                .iter()
                .any(|target| target == "food_hidden_pantry"));
            assert!(
                proposed
                    .decision_trace_record
                    .hidden_truth_audit_result
                    .actor_known_only
            );
        }
        ActorDecisionTransactionOutcome::Stuck { diagnostic } => {
            assert!(!format!("{diagnostic:?}").contains("food_hidden_pantry"));
        }
    }
}

#[test]
fn workplace_requires_assignment_or_observation_provenance() {
    let hidden_workplace = workplace_id("workplace_hidden");
    let hidden_place = place_id("hidden_workshop");
    let context_without_workplace = context(BTreeMap::new(), BTreeSet::new(), BTreeMap::new());
    assert!(!context_without_workplace
        .known_workplaces()
        .contains_key(&hidden_workplace));

    let failure = plan_local_actions(
        &context_without_workplace,
        &LocalPlanRequest {
            routine_step: RoutineStep::StartWorkBlock {
                action_id: "work_block".parse().unwrap(),
            },
            goal: PlannerGoal::StartWorkBlock(hidden_workplace.as_str().to_string()),
            budget: 1,
            actor_known_facts: Vec::new(),
        },
    )
    .unwrap_err();
    assert_eq!(
        failure.reason,
        "workplace is not actor-known at current place"
    );

    let context_with_assignment = context(
        BTreeMap::new(),
        BTreeSet::new(),
        BTreeMap::from([(hidden_workplace.clone(), hidden_place)]),
    );
    assert!(context_with_assignment
        .proof_sources()
        .iter()
        .any(|source| source.contains("routine_assignment")));
}

#[test]
fn hidden_route_edge_absent_from_actor_known_edges_blocks_route_plan() {
    let context = context(BTreeMap::new(), BTreeSet::new(), BTreeMap::new());
    proof_sources_are_actor_known(&context);
    assert!(!context
        .known_edges()
        .get(&place_id("home_mara"))
        .is_some_and(|edges| edges.contains(&place_id("hidden_workshop"))));

    let failure = plan_local_actions(
        &context,
        &LocalPlanRequest {
            routine_step: RoutineStep::MoveTowardPlace {
                action_id: "move".parse().unwrap(),
            },
            goal: PlannerGoal::ReachPlace(place_id("hidden_workshop")),
            budget: 1,
            actor_known_facts: Vec::new(),
        },
    )
    .unwrap_err();
    assert_eq!(failure.reason, "no actor-known route to target");
}

#[test]
fn debug_omniscience_facts_are_excluded_from_planner_context() {
    let mut context = context(BTreeMap::new(), BTreeSet::new(), BTreeMap::new());
    proof_sources_are_actor_known(&context);
    assert!(!context
        .proof_sources()
        .iter()
        .any(|source| source.contains("debug_omniscience")));

    context.add_actor_known_fact(ActorKnownFact::unproven(
        "debug_hidden_food",
        "debug-only omniscience must not be accepted as actor-known",
    ));
    assert!(!context.audit_with(&[]).actor_known_only);
}
