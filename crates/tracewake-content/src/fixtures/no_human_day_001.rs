use crate::fixtures::*;

pub fn no_human_day_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("no_human_day_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase3AHistorical,
        need_model: need_model_schema(),
        actors: vec![
            actor_schema("actor_anna", "office_anna"),
            actor_schema("actor_elena", "home_elena"),
            actor_schema("actor_mara", "home_mara"),
            actor_schema("actor_tomas", "home_tomas"),
        ],
        places: vec![
            place_schema(
                "commons",
                "Commons",
                &[
                    "home_elena",
                    "home_mara",
                    "home_tomas",
                    "office_anna",
                    "workshop_tomas",
                ],
            ),
            place_schema("home_elena", "Elena home", &["commons"]),
            place_schema("home_mara", "Mara home", &["commons"]),
            place_schema("home_tomas", "Tomas home", &["commons"]),
            place_schema("office_anna", "Anna office", &["commons"]),
            place_schema("workshop_tomas", "Tomas workshop", &["commons"]),
        ],
        doors: Vec::new(),
        containers: Vec::new(),
        items: Vec::new(),
        affordances: vec![
            affordance("eat", "food_empty_pantry_mara"),
            affordance("eat", "food_stew_home_tomas"),
            affordance("move", "commons"),
            affordance("move", "home_elena"),
            affordance("move", "home_tomas"),
            affordance("move", "office_anna"),
            affordance("move", "workshop_tomas"),
            affordance("sleep", "home_elena"),
            affordance("work_block", "workplace_anna_closed"),
            affordance("work_block", "workplace_tomas"),
        ],
        initial_beliefs: Vec::new(),
        initial_needs: vec![
            initial_need("actor_anna", NeedKind::Fatigue, 220),
            initial_need("actor_anna", NeedKind::Hunger, 300),
            initial_need("actor_elena", NeedKind::Fatigue, 720),
            initial_need("actor_elena", NeedKind::Hunger, 300),
            initial_need("actor_mara", NeedKind::Hunger, 900),
            initial_need("actor_tomas", NeedKind::Fatigue, 260),
            initial_need("actor_tomas", NeedKind::Hunger, 520),
            initial_need("actor_anna", NeedKind::Safety, 100),
            initial_need("actor_elena", NeedKind::Safety, 100),
            initial_need("actor_mara", NeedKind::Fatigue, 100),
            initial_need("actor_mara", NeedKind::Safety, 100),
            initial_need("actor_tomas", NeedKind::Safety, 100),
        ],
        homes: vec![
            home_schema("actor_anna", "office_anna"),
            home_schema("actor_elena", "home_elena"),
            home_schema("actor_mara", "home_mara"),
            home_schema("actor_tomas", "home_tomas"),
        ],
        sleep_places: vec![sleep_place_schema("actor_elena", "home_elena", "bed_elena")],
        food_supplies: vec![
            food_supply_at_place("food_empty_pantry_mara", "home_mara", 0, 200),
            food_supply_at_place("food_stew_home_tomas", "home_tomas", 2, 240),
        ],
        known_food_sources: Vec::new(),
        workplaces: vec![
            workplace_schema(
                "workplace_anna_closed",
                "office_anna",
                &["actor_anna"],
                4,
                false,
            ),
            workplace_schema(
                "workplace_tomas",
                "workshop_tomas",
                &["actor_tomas"],
                4,
                true,
            ),
        ],
        routine_templates: vec![
            routine_template_schema(
                "routine_anna_blocked_office",
                RoutineFamily::WorkBlock,
                vec![routine_step("start_work_block", "work_block")],
                &["workplace_closed", "access"],
            ),
            routine_template_schema(
                "routine_elena_sleep",
                RoutineFamily::SleepNight,
                vec![routine_step("start_scheduled_sleep", "sleep")],
                &["sleep_place_blocked"],
            ),
            routine_template_schema(
                "routine_mara_food_unavailable",
                RoutineFamily::EatMeal,
                vec![
                    routine_step("consume_accessible_food", "eat"),
                    wait_step("food unavailable replan"),
                ],
                &["food_missing", "food_inaccessible"],
            ),
            routine_template_schema(
                "routine_tomas_go_work",
                RoutineFamily::GoToWork,
                vec![routine_step("move_toward_place", "move")],
                &["route_blocked"],
            ),
            routine_template_schema(
                "routine_tomas_work",
                RoutineFamily::WorkBlock,
                vec![routine_step("start_work_block", "work_block")],
                &["workplace_closed", "need_blocked"],
            ),
        ],
        routine_assignments: vec![
            routine_assignment_schema("actor_anna", "routine_anna_blocked_office", 10, 18),
            routine_assignment_schema("actor_elena", "routine_elena_sleep", 24, 32),
            routine_assignment_schema("actor_mara", "routine_mara_food_unavailable", 4, 10),
            routine_assignment_schema("actor_tomas", "routine_tomas_go_work", 8, 18),
            routine_assignment_schema("actor_tomas", "routine_tomas_work", 18, 32),
        ],
        day_windows: vec![
            day_window_schema("actor_anna", 0, 32),
            day_window_schema("actor_elena", 0, 32),
            day_window_schema("actor_mara", 0, 32),
            day_window_schema("actor_tomas", 0, 32),
        ],
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
            fixture_id: "no_human_day_001",
            purpose: "Canonical Phase 3A no-human ordinary-day fixture.",
            setup: vec![
                "roster:actor_anna,actor_elena,actor_mara,actor_tomas",
                "homes, day windows, initial needs, food, sleep, work, routes, and routines are authored",
                "Mara has an empty food source and Anna has closed workplace access",
            ],
            allowed_actions: vec![
                "run no-human day without controller binding",
                "move Tomas through commons to workshop_tomas",
                "eat, sleep, work, and fail modeled blockers through shared pipeline",
            ],
            expected_events_or_reports: vec![
                "expected_roster=actor_anna,actor_elena,actor_mara,actor_tomas",
                "log_derived_metric=no_human_day_metrics_v1",
                "autonomous_no_human_event=NoHumanDayStarted",
                "autonomous_no_human_event=NoHumanDayCompleted",
                "autonomous_no_human_event=FoodConsumed|EatFailed",
                "autonomous_no_human_event=SleepCompleted",
                "routine_execution_status=routine_exec_tomas_work:Completed",
                "canonical_mara_recovery_resolution=fail_only_empty_food_source",
            ],
            acceptance_assertions: vec![
                "no emitted event references player or controller identity",
                "every roster actor advances during no-human run",
                "autonomous no-human events are not manually forced action-unit events",
                "movement uses ActorMoved events before workplace presence",
                "Mara's empty food source records typed failure",
                "Mara's canonical recovery variant is recorded as fail-only on no_human_day_001",
                "Anna's closed workplace records an access blocker",
                "metrics derive from the event log and have nonzero activity",
            ],
        },
    }
}
