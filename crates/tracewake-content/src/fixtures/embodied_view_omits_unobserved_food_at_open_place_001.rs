use crate::fixtures::*;

pub fn embodied_view_omits_unobserved_food_at_open_place_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("embodied_view_omits_unobserved_food_at_open_place_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase3AHistorical,
        need_model: need_model_schema(),
        actors: vec![actor_schema("actor_tomas", "shop_front")],
        places: vec![place_schema("shop_front", "Shop front", &[])],
        doors: Vec::new(),
        containers: Vec::new(),
        items: Vec::new(),
        affordances: Vec::new(),
        initial_beliefs: Vec::new(),
        initial_needs: vec![initial_need("actor_tomas", NeedKind::Hunger, 850)],
        homes: vec![home_schema("actor_tomas", "shop_front")],
        sleep_places: Vec::new(),
        food_supplies: vec![food_supply_at_place(
            "food_visible_truth",
            "shop_front",
            1,
            200,
        )],
        workplaces: Vec::new(),
        routine_templates: Vec::new(),
        routine_assignments: Vec::new(),
        day_windows: vec![day_window_schema("actor_tomas", 0, 4)],
    };
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "embodied_view_omits_unobserved_food_at_open_place_001",
            purpose: "Adversarial fixture: raw local food is omitted from embodied actions without a sealed context fact.",
            setup: vec!["food exists at the actor's open current place"],
            allowed_actions: vec!["eat only when a holder-known food fact exists"],
            expected_events_or_reports: vec!["embodied output omits food_visible_truth"],
            acceptance_assertions: vec![
                "raw food_supplies are not an embodied affordance source",
                "debug may compare truth non-diegetically",
            ],
        },
    }
}
