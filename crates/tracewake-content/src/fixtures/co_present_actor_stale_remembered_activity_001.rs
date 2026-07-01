use crate::fixtures::*;

pub fn co_present_actor_stale_remembered_activity_001() -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id("co_present_actor_stale_remembered_activity_001"),
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
            3,
            true,
        )],
        routine_templates: vec![routine_template_schema(
            "routine_tomas_work_then_age",
            RoutineFamily::WorkBlock,
            vec![routine_step("start_work_block", "work_block")],
            &["workplace_closed", "need_blocked"],
        )],
        routine_assignments: vec![routine_assignment_schema(
            "actor_tomas",
            "routine_tomas_work_then_age",
            0,
            3,
        )],
        day_windows: vec![
            day_window_schema("actor_mara", 0, 12),
            day_window_schema("actor_tomas", 0, 12),
        ],
    };
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: "co_present_actor_stale_remembered_activity_001",
            purpose: "Provide an observable work activity setup whose actor-known row can later age stale.",
            setup: vec![
                "actor_mara and actor_tomas start in workshop_mara",
                "actor_tomas has a short work routine during ticks 0..3",
                "tests may observe the row early and then advance time before projecting it again",
            ],
            allowed_actions: vec![
                "observe actor_tomas during routine_tomas_work_then_age",
                "advance simulation time before rebuilding actor_mara's actor-known context",
            ],
            expected_events_or_reports: vec![
                "early visible_actor observation can carry work activity evidence",
                "later actor-known row can be marked stale without inventing fresh activity",
            ],
            acceptance_assertions: vec![
                "fixture authors no initial belief about actor_tomas activity",
                "staleness comes from observation tick age, not authored prose",
                "fixture canonicalizes without changing authored references",
            ],
        },
    }
}
