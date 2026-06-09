use crate::fixtures::*;

pub fn scheduler_cannot_rewrite_wait_reason_after_transaction_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("scheduler_cannot_rewrite_wait_reason_after_transaction_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase3AHistorical,
        need_model: need_model_schema(),
        actors: vec![actor_schema("actor_tomas", "home_tomas")],
        places: vec![place_schema("home_tomas", "Tomas home", &[])],
        doors: Vec::new(),
        containers: Vec::new(),
        items: Vec::new(),
        affordances: Vec::new(),
        initial_beliefs: Vec::new(),
        initial_needs: vec![
            initial_need("actor_tomas", NeedKind::Hunger, 100),
            initial_need("actor_tomas", NeedKind::Fatigue, 100),
        ],
        homes: vec![home_schema("actor_tomas", "home_tomas")],
        sleep_places: Vec::new(),
        food_supplies: Vec::new(),
        workplaces: Vec::new(),
        routine_templates: Vec::new(),
        routine_assignments: Vec::new(),
        day_windows: vec![day_window_schema("actor_tomas", 0, 4)],
    };
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "scheduler_cannot_rewrite_wait_reason_after_transaction_001",
            purpose: "Prove scheduler does not rewrite actor-visible wait proposal reason after the actor decision transaction.",
            setup: vec![
                "actor_tomas has no urgent need, food, workplace, or sleep surface",
                "the no-human window id is scheduler metadata only",
            ],
            allowed_actions: vec!["transaction-authored wait"],
            expected_events_or_reports: vec![
                "ActorWaited reason remains actor_decision_reevaluation",
                "no actor-visible proposal parameter contains no_human_day:<window>",
            ],
            acceptance_assertions: vec![
                "transaction proposal is sealed before scheduler handoff",
                "no-human window id is not written into actor-visible wait reason",
            ],
        },
    }
}
