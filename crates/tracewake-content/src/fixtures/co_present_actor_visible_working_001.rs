use crate::fixtures::*;

pub fn co_present_actor_visible_working_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("co_present_actor_visible_working_001"),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase3AHistorical,
        need_model: need_model_schema(),
        actors: vec![
            actor_schema("actor_mara", "workshop_mara"),
            actor_schema("actor_tomas", "workshop_mara"),
        ],
        places: vec![place_schema("workshop_mara", "Mara workshop", &[])],
        doors: Vec::new(),
        containers: Vec::new(),
        items: Vec::new(),
        affordances: vec![affordance("work_block", "workplace_tomas_bench")],
        initial_beliefs: Vec::new(),
        initial_needs: vec![
            initial_need("actor_mara", NeedKind::Hunger, 120),
            initial_need("actor_mara", NeedKind::Fatigue, 120),
            initial_need("actor_mara", NeedKind::Safety, 100),
            initial_need("actor_tomas", NeedKind::Hunger, 160),
            initial_need("actor_tomas", NeedKind::Fatigue, 180),
            initial_need("actor_tomas", NeedKind::Safety, 100),
        ],
        homes: Vec::new(),
        sleep_places: Vec::new(),
        food_supplies: Vec::new(),
        known_food_sources: Vec::new(),
        workplaces: vec![workplace_schema(
            "workplace_tomas_bench",
            "workshop_mara",
            &["actor_tomas"],
            4,
            true,
        )],
        routine_templates: vec![routine_template_schema(
            "routine_tomas_work_visible",
            RoutineFamily::WorkBlock,
            vec![routine_step("start_work_block", "work_block")],
            &["workplace_closed", "need_blocked"],
        )],
        routine_assignments: vec![routine_assignment_schema(
            "actor_tomas",
            "routine_tomas_work_visible",
            0,
            6,
        )],
        day_windows: vec![
            day_window_schema("actor_mara", 0, 8),
            day_window_schema("actor_tomas", 0, 8),
        ],
    };
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "co_present_actor_visible_working_001",
            purpose:
                "Provide co-present ordinary-work substrate for actor-known observed activity.",
            setup: vec![
                "actor_mara and actor_tomas start in workshop_mara",
                "actor_tomas is assigned to routine_tomas_work_visible during ticks 0..6",
                "workplace_tomas_bench is open and located in workshop_mara",
            ],
            allowed_actions: vec![
                "observe co-present actor_tomas from actor_mara",
                "start or continue routine_tomas_work_visible through ordinary work semantics",
            ],
            expected_events_or_reports: vec![
                "visible_actor observation can cite actor_tomas as co-present",
                "work activity evidence comes from modeled routine/work state, not prose",
            ],
            acceptance_assertions: vec![
                "fixture authors no initial belief about actor_tomas activity",
                "activity evidence is available only through modeled work substrate",
                "fixture canonicalizes without changing authored references",
            ],
        },
    }
}
