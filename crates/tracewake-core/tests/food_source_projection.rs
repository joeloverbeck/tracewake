mod support;

use tracewake_core::actions::ActionRegistry;
use tracewake_core::epistemics::{
    ActorKnownCurrentPlaceFact, ActorKnownFoodSourceFact, KnowledgeContext,
};
use tracewake_core::ids::{ActorId, ContentManifestId, FoodSupplyId, PlaceId};
use tracewake_core::location::Location;
use tracewake_core::projections::{
    build_embodied_view_model, EmbodiedPreflightSource, EmbodiedProjectionSource,
    EmbodiedTruthSnapshot,
};
use tracewake_core::state::{ActorBody, FoodSupplyState, PlaceState};
use tracewake_core::time::SimTick;
use tracewake_core::view_models::SemanticActionEntry;

use support::PhysicalSeed;

fn actor_id() -> ActorId {
    ActorId::new("actor_tomas").unwrap()
}

fn place_id() -> PlaceId {
    PlaceId::new("shop_front").unwrap()
}

fn food_id() -> FoodSupplyId {
    FoodSupplyId::new("food_stew").unwrap()
}

fn food_fact(believed_servings: Option<u32>, source_key: &str) -> ActorKnownFoodSourceFact {
    ActorKnownFoodSourceFact::with_believed_servings(food_id(), believed_servings, source_key)
}

fn state() -> tracewake_core::state::PhysicalState {
    let actor_id = actor_id();
    let place_id = place_id();
    let food_id = food_id();
    let mut seed = PhysicalSeed::default();
    seed.places_mut().insert(
        place_id.clone(),
        PlaceState::new(place_id.clone(), "Shop Front"),
    );
    seed.actors_mut()
        .insert(actor_id.clone(), ActorBody::new(actor_id, place_id.clone()));
    seed.food_supplies_mut().insert(
        food_id.clone(),
        FoodSupplyState::new(food_id, Location::AtPlace(place_id), 3, 30),
    );
    seed.build()
}

fn eat_action_for(facts: Vec<ActorKnownFoodSourceFact>) -> SemanticActionEntry {
    let state = state();
    let context = KnowledgeContext::embodied_at_frontier_with_all_facts_and_observations(
        actor_id(),
        SimTick::new(4),
        17,
        Vec::new(),
        vec![ActorKnownCurrentPlaceFact::new(
            place_id(),
            "Shop Front",
            "source:current_place",
        )],
        Vec::new(),
        facts,
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    );
    let snapshot = EmbodiedTruthSnapshot::from_physical_state(&context, &state);
    let source = EmbodiedProjectionSource::from_sealed_context(&context, snapshot, None);
    let mut registry = ActionRegistry::new();
    registry.register_phase3a_eat();
    let manifest_id = ContentManifestId::new("manifest_food_source_projection").unwrap();
    let preflight = EmbodiedPreflightSource::new(&state, &registry, &manifest_id);
    let view = build_embodied_view_model(&context, &source, &preflight, None).unwrap();
    view.semantic_actions
        .into_iter()
        .find(|entry| entry.semantic_action_id.as_str() == "eat.food.food_stew")
        .expect("known food source produces an eat action")
}

#[test]
fn source_bearing_empty_food_supersedes_source_less_food() {
    let action = eat_action_for(vec![
        food_fact(None, "source:unknown:a"),
        food_fact(Some(0), "source:empty:z"),
    ]);

    assert!(
        !action.enabled,
        "source-bearing empty food must supersede source-less availability"
    );
}

#[test]
fn source_less_food_does_not_replace_source_bearing_empty_food() {
    let action = eat_action_for(vec![
        food_fact(Some(0), "source:empty:z"),
        food_fact(None, "source:unknown:a"),
    ]);

    assert!(
        !action.enabled,
        "source-less food must not replace source-bearing empty knowledge"
    );
}

#[test]
fn source_key_order_is_deterministic_for_same_serving_class() {
    let earlier_source_action = eat_action_for(vec![
        food_fact(Some(0), "source:z"),
        food_fact(Some(3), "source:a"),
    ]);
    assert!(
        earlier_source_action.enabled,
        "lexically earlier source key should supersede within the same serving class"
    );

    let equal_source_action = eat_action_for(vec![
        food_fact(Some(0), "source:same"),
        food_fact(Some(3), "source:same"),
    ]);
    assert!(
        !equal_source_action.enabled,
        "equal source keys should keep the first selected fact"
    );
}
