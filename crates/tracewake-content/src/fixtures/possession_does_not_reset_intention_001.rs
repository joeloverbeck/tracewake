use crate::fixtures::*;

pub fn possession_does_not_reset_intention_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("possession_does_not_reset_intention_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase3AHistorical,
        need_model: need_model_schema(),
        actors: vec![actor_schema("actor_mara", "workshop_mara")],
        places: vec![place_schema("workshop_mara", "Mara workshop", &[])],
        doors: Vec::new(),
        containers: Vec::new(),
        items: vec![item_carried_by("tool_roll_mara", "actor_mara", true)],
        affordances: vec![affordance("work_block", "workplace_mara")],
        initial_beliefs: Vec::new(),
        initial_needs: vec![
            initial_need("actor_mara", NeedKind::Hunger, 320),
            initial_need("actor_mara", NeedKind::Fatigue, 280),
        ],
        homes: vec![home_schema("actor_mara", "workshop_mara")],
        sleep_places: Vec::new(),
        food_supplies: Vec::new(),
        known_food_sources: Vec::new(),
        workplaces: vec![workplace_schema(
            "workplace_mara",
            "workshop_mara",
            &["actor_mara"],
            4,
            true,
        )],
        routine_templates: vec![routine_template_schema(
            "routine_mara_active_work",
            RoutineFamily::WorkBlock,
            vec![routine_step("continue_current_step", "continue_routine")],
            &["no_current_intention", "step_blocked"],
        )],
        routine_assignments: vec![routine_assignment_schema(
            "actor_mara",
            "routine_mara_active_work",
            0,
            8,
        )],
        day_windows: vec![day_window_schema("actor_mara", 0, 8)],
    };
    #[expect(
        clippy::disallowed_methods,
        reason = "legacy fixture blanket food-source seeding is pinned by fixtures_load census; new fixtures must author per-actor known_food_sources edges"
    )]
    fixture.populate_known_food_sources_for_all_actors();
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "possession_does_not_reset_intention_001",
            purpose: "Prove controller bind/unbind does not reset needs, intention, or routine execution.",
            setup: vec![
                "actor_mara starts as an ordinary worker",
                "the test seeds an active work intention and routine execution",
                "controller attach/detach is metadata only",
            ],
            allowed_actions: vec![
                "attach controller_human",
                "detach controller_human",
                "continue_routine through shared pipeline",
            ],
            expected_events_or_reports: vec![
                "ControllerAttached and ControllerDetached",
                "ContinueRoutineProposed",
                "unchanged active intention and need maps",
            ],
            acceptance_assertions: vec![
                "possession does not copy or reset intention state",
                "needs remain unchanged by binding metadata",
                "continue_routine resumes the preexisting intention",
            ],
        },
    }
}
