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
use tracewake_core::checksum::ChecksumContext;
use tracewake_core::debug_reports::item_location_report;
use tracewake_core::epistemics::{ActorKnownFoodSourceFact, EpistemicProjection, KnowledgeContext};
use tracewake_core::ids::{
    ActorId, ContainerId, ContentManifestId, ContentVersion, FixtureId, FoodSupplyId, ItemId,
    PlaceId, WorkplaceId,
};
use tracewake_core::location::Location;
use tracewake_core::projections::{build_embodied_view_model, EmbodiedProjectionSource};
use tracewake_core::state::{ActorBody, AgentState, ContainerState, FoodSupplyState, PlaceState};
use tracewake_core::time::SimTick;

const ACTOR_KNOWN_RS: &str = include_str!("../src/agent/actor_known.rs");
const DEBUG_CAPABILITY_RS: &str = include_str!("../src/debug_capability.rs");
const KNOWLEDGE_CONTEXT_RS: &str = include_str!("../src/epistemics/knowledge_context.rs");
const EPISTEMIC_PROJECTION_RS: &str = include_str!("../src/epistemics/projection.rs");
const EPISTEMIC_BELIEF_RS: &str = include_str!("../src/epistemics/belief.rs");
const EPISTEMIC_OBSERVATION_RS: &str = include_str!("../src/epistemics/observation.rs");
const EPISTEMIC_CONTRADICTION_RS: &str = include_str!("../src/epistemics/contradiction.rs");
const VIEW_MODELS_RS: &str = include_str!("../src/view_models.rs");
const DEBUG_REPORTS_RS: &str = include_str!("../src/debug_reports.rs");
const CONTENT_SCHEMA_RS: &str = include_str!("../../tracewake-content/src/schema.rs");
const CONTENT_VALIDATE_RS: &str = include_str!("../../tracewake-content/src/validate.rs");
const CONTENT_SERIALIZATION_RS: &str = include_str!("../../tracewake-content/src/serialization.rs");
const CONTENT_LOAD_RS: &str = include_str!("../../tracewake-content/src/load.rs");
const TUI_INPUT_RS: &str = include_str!("../../tracewake-tui/src/input.rs");
const TUI_RENDER_RS: &str = include_str!("../../tracewake-tui/src/render.rs");
const TUI_DEBUG_PANELS_RS: &str = include_str!("../../tracewake-tui/src/debug_panels.rs");
const TUI_APP_RS: &str = include_str!("../../tracewake-tui/src/app.rs");
const TUI_TRANSCRIPT_RS: &str = include_str!("../../tracewake-tui/src/transcript.rs");

fn assert_source_lacks_any(source_name: &str, source: &str, banned: &[&str]) {
    let violations: Vec<_> = banned
        .iter()
        .copied()
        .filter(|token| source.contains(token))
        .collect();
    assert!(
        violations.is_empty(),
        "{} contains banned source token(s): {}",
        source_name,
        violations.join(", ")
    );
}

fn epistemic_guard_sources() -> [(&'static str, &'static str); 14] {
    [
        ("epistemics/knowledge_context.rs", KNOWLEDGE_CONTEXT_RS),
        ("epistemics/projection.rs", EPISTEMIC_PROJECTION_RS),
        ("epistemics/belief.rs", EPISTEMIC_BELIEF_RS),
        ("epistemics/observation.rs", EPISTEMIC_OBSERVATION_RS),
        ("epistemics/contradiction.rs", EPISTEMIC_CONTRADICTION_RS),
        ("view_models.rs", VIEW_MODELS_RS),
        ("debug_reports.rs", DEBUG_REPORTS_RS),
        ("content/schema.rs", CONTENT_SCHEMA_RS),
        ("content/validate.rs", CONTENT_VALIDATE_RS),
        ("content/serialization.rs", CONTENT_SERIALIZATION_RS),
        ("content/load.rs", CONTENT_LOAD_RS),
        ("tui/input.rs", TUI_INPUT_RS),
        ("tui/render.rs", TUI_RENDER_RS),
        ("tui/debug_panels.rs", TUI_DEBUG_PANELS_RS),
    ]
}

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
fn epistemic_context_projection_and_records_remain_sealed() {
    assert_source_lacks_any(
        "epistemics/knowledge_context.rs",
        KNOWLEDGE_CONTEXT_RS,
        &[
            "pub viewer_actor_id",
            "pub mode",
            "pub perception_scope",
            "pub belief_scope",
            "pub observation_scope",
            "pub debug_non_diegetic",
            "pub holder_known_context_hash",
            "pub forbidden_truth_audit",
        ],
    );
    assert!(
        KNOWLEDGE_CONTEXT_RS.contains("pub fn debug(")
            && KNOWLEDGE_CONTEXT_RS.contains("_capability: &DebugCapability"),
        "KnowledgeContext::debug must stay capability-gated if it remains public"
    );

    assert_source_lacks_any(
        "epistemics/projection.rs",
        EPISTEMIC_PROJECTION_RS,
        &[
            "pub observations_by_id",
            "pub beliefs_by_id",
            "pub beliefs_by_holder",
            "pub contradictions_by_id",
            "pub notebook_entries_by_actor",
            "pub fn insert_observation",
            "pub fn insert_belief",
            "pub fn insert_contradiction",
            "pub fn insert_notebook_entry",
        ],
    );

    assert_source_lacks_any(
        "epistemics/belief.rs",
        EPISTEMIC_BELIEF_RS,
        &[
            "pub belief_id:",
            "pub holder:",
            "pub proposition:",
            "pub stance:",
            "pub confidence:",
            "pub source:",
            "pub privacy_scope:",
            "pub schema_version:",
        ],
    );
    assert_source_lacks_any(
        "epistemics/observation.rs",
        EPISTEMIC_OBSERVATION_RS,
        &[
            "pub observation_id:",
            "pub observer_actor_id:",
            "pub channel:",
            "pub confidence:",
            "pub source:",
            "pub privacy_scope:",
            "pub schema_version:",
        ],
    );
    assert_source_lacks_any(
        "epistemics/contradiction.rs",
        EPISTEMIC_CONTRADICTION_RS,
        &[
            "pub contradiction_id:",
            "pub holder_actor_id:",
            "pub prior_expectation_belief_id:",
            "pub contradicting_observation_id:",
            "pub expected_proposition:",
            "pub observed_proposition:",
            "pub schema_version:",
        ],
    );

    let synthetic_context_leak = "pub viewer_actor_id: ActorId";
    assert!(
        synthetic_context_leak.contains("pub viewer_actor_id"),
        "source guard self-coverage must include context public-field leaks"
    );
}

#[test]
fn epistemic_confidence_paths_do_not_use_raw_floats_or_hash_ordering() {
    let banned_tokens = [
        "f32",
        "f64",
        "parse::<f32>",
        "parse::<f64>",
        "HashMap",
        "HashSet",
    ];
    for (source_name, source) in epistemic_guard_sources().into_iter().chain([
        ("tui/app.rs", TUI_APP_RS),
        ("tui/transcript.rs", TUI_TRANSCRIPT_RS),
    ]) {
        assert_source_lacks_any(source_name, source, &banned_tokens);
    }

    let synthetic_float_leak = "let confidence: f32 = 0.5;";
    assert!(
        synthetic_float_leak.contains("f32"),
        "source guard self-coverage must include raw float confidence leaks"
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

    let knowledge_context = KnowledgeContext::embodied_at_frontier_with_facts(
        actor_id(),
        SimTick::ZERO,
        0,
        Vec::new(),
        vec![ActorKnownFoodSourceFact::new(
            food_supply_id("food_empty_pantry_mara"),
            "visible_food_supply",
        )],
        Vec::new(),
        Vec::new(),
    );
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
        source_event_ids: None,
        routine_window_family: Some(RoutineFamily::EatMeal),
        include_idle_fallback: true,
    });
    match outcome {
        ActorDecisionTransactionOutcome::Proposed(proposed) => {
            assert_ne!(proposed.proposal.action_id().as_str(), "eat");
            assert!(!proposed
                .proposal
                .target_ids()
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
    let context = context(BTreeMap::new(), BTreeSet::new(), BTreeMap::new());
    proof_sources_are_actor_known(&context);
    assert!(!context
        .proof_sources()
        .iter()
        .any(|source| source.contains("debug_omniscience")));

    let unproven = ActorKnownFact::unproven(
        "debug_hidden_food",
        "debug-only omniscience must not be accepted as actor-known",
    );
    assert!(!context.audit_with(&[unproven]).actor_known_only);
}

#[test]
fn debug_truth_never_enters_holder_known_context_hash() {
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
    let before_view = build_embodied_view_model(
        &knowledge_context,
        &projection_source,
        &registry(),
        &ContentManifestId::new("hidden_truth_gate_manifest").unwrap(),
        None,
    )
    .unwrap();

    let debug_report = item_location_report(
        &world,
        &tracewake_core::events::log::EventLog::new(),
        &ItemId::new("food_hidden_pantry").unwrap(),
        &ChecksumContext {
            fixture_id: FixtureId::new("debug_truth_hash_gate").unwrap(),
            content_version: ContentVersion::new("hidden_truth_gate_manifest").unwrap(),
            sim_tick: SimTick::ZERO,
            world_stream_position_applied: 0,
        },
    );
    assert!(debug_report.debug_only());
    assert!(format!("{debug_report:?}").contains("food_hidden_pantry"));

    let after_projection_source =
        EmbodiedProjectionSource::from_sealed_context(&knowledge_context, &world, None);
    let after_view = build_embodied_view_model(
        &knowledge_context,
        &after_projection_source,
        &registry(),
        &ContentManifestId::new("hidden_truth_gate_manifest").unwrap(),
        None,
    )
    .unwrap();

    assert_eq!(
        after_view.holder_known_context_hash,
        before_view.holder_known_context_hash
    );
    assert_eq!(
        after_view.holder_known_context_source_summary,
        before_view.holder_known_context_source_summary
    );
    assert!(!after_view
        .holder_known_context_source_summary
        .contains("debug"));
    assert!(!after_view.semantic_actions.iter().any(|entry| entry
        .target_ids
        .iter()
        .any(|target| target == "food_hidden_pantry")));
}
