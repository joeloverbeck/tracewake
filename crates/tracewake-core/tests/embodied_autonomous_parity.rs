mod support;

use support::{
    parity_execution, parity_surface, test_event_id, RoutineParityFixture, RoutineParityOutcome,
};
use tracewake_core::agent::{RoutineFamily, RoutineStepStatus};
use tracewake_core::ids::{ActorId, PlaceId, WorkplaceId};
use tracewake_core::time::SimTick;

#[test]
fn embodied_continue_and_autonomous_transaction_match_from_equivalent_actor_known_state() {
    let home = place("home_parity");
    let workshop = place("workshop_parity");
    let fixture = go_to_work_fixture(home.clone(), workshop.clone(), Vec::new());

    assert_matching_proposed(&fixture, "move", Some("routine_go_to_work"));
}

#[test]
fn embodied_autonomous_parity_uses_active_eat_not_known_workplace_shortcut() {
    let workshop = place("workshop_parity");
    let surface = base_surface(workshop.clone(), workshop.clone())
        .with_test_known_food_source("food_stew", test_event_id("event_parity_food_eat"));
    let fixture = RoutineParityFixture::new(
        RoutineFamily::EatMeal,
        "routine_eat_meal",
        RoutineStepStatus::InProgress,
        Vec::new(),
        workshop,
        surface,
    );

    assert_matching_proposed(&fixture, "eat", Some("routine_eat_meal"));
}

#[test]
fn embodied_autonomous_parity_does_not_select_hidden_workplace_truth() {
    let home = place("home_parity");
    let surface = parity_surface(home.clone());
    let fixture = RoutineParityFixture::new(
        RoutineFamily::WorkBlock,
        "routine_work_block",
        RoutineStepStatus::InProgress,
        Vec::new(),
        home,
        surface,
    );

    let embodied = fixture.run_embodied();
    let autonomous = fixture.run_autonomous();

    assert_eq!(embodied, autonomous);
    match embodied {
        RoutineParityOutcome::Stuck {
            concrete_blocker,
            hidden_truth_referenced,
        } => {
            assert_eq!(concrete_blocker, "no applicable method");
            assert!(!hidden_truth_referenced);
        }
        RoutineParityOutcome::Proposed(shape) => {
            assert_ne!(shape.action_id, "work_block");
            assert!(shape.hidden_truth_actor_known_only);
        }
    }
}

#[test]
fn embodied_autonomous_parity_ignores_inactive_resolved_and_other_actor_executions() {
    let home = place("home_parity");
    let workshop = place("workshop_parity");
    let other_actor = ActorId::new("actor_parity_other").unwrap();
    let decoys = vec![
        parity_execution(
            "routine_exec_future_inactive_work",
            ActorId::new("actor_parity").unwrap(),
            "routine_work_block",
            RoutineFamily::WorkBlock,
            RoutineStepStatus::NotStarted,
            SimTick::new(12),
        ),
        parity_execution(
            "routine_exec_resolved_sleep",
            ActorId::new("actor_parity").unwrap(),
            "routine_sleep_night",
            RoutineFamily::SleepNight,
            RoutineStepStatus::Completed,
            SimTick::new(1),
        ),
        parity_execution(
            "routine_exec_other_actor_work",
            other_actor,
            "routine_work_block",
            RoutineFamily::WorkBlock,
            RoutineStepStatus::InProgress,
            SimTick::new(1),
        ),
    ];
    let fixture = go_to_work_fixture(home, workshop, decoys);

    assert_matching_proposed(&fixture, "move", Some("routine_go_to_work"));
}

fn assert_matching_proposed(
    fixture: &RoutineParityFixture,
    expected_action_id: &str,
    expected_template_id: Option<&str>,
) {
    let embodied = fixture.run_embodied();
    let autonomous = fixture.run_autonomous();

    assert_eq!(embodied, autonomous);
    let RoutineParityOutcome::Proposed(shape) = embodied else {
        panic!("expected matching proposed outcome, got {autonomous:?}");
    };
    assert_eq!(shape.action_id, expected_action_id);
    assert_eq!(
        shape.routine_template_id.as_deref(),
        expected_template_id,
        "{shape:#?}"
    );
    assert_eq!(shape.actor_id.as_ref(), Some(&fixture.actor_id));
    assert!(!shape.target_ids.is_empty(), "{shape:#?}");
    assert!(shape.routine_execution_id.is_some(), "{shape:#?}");
    assert!(shape.local_plan_id_present, "{shape:#?}");
    assert_eq!(shape.proposal_ancestry_len, 3, "{shape:#?}");
    assert!(shape.hidden_truth_actor_known_only, "{shape:#?}");
}

fn go_to_work_fixture(
    home: PlaceId,
    workshop: PlaceId,
    scheduler_decoys: Vec<tracewake_core::agent::RoutineExecution>,
) -> RoutineParityFixture {
    let surface = base_surface(home.clone(), workshop.clone());
    RoutineParityFixture::new(
        RoutineFamily::GoToWork,
        "routine_go_to_work",
        RoutineStepStatus::InProgress,
        scheduler_decoys,
        home,
        surface,
    )
}

fn base_surface(
    current_place: PlaceId,
    workshop: PlaceId,
) -> tracewake_core::agent::NoHumanActorKnownSurfaceBuilder {
    parity_surface(current_place.clone())
        .with_test_known_route(
            current_place.clone(),
            workshop.clone(),
            test_event_id("event_parity_route"),
        )
        .with_test_known_workplace(
            WorkplaceId::new("workplace_parity").unwrap(),
            workshop,
            true,
            test_event_id("event_parity_workplace"),
        )
        .with_test_known_sleep_place(
            current_place,
            Some("sleep_affordance_parity"),
            test_event_id("event_parity_sleep"),
        )
}

fn place(value: &str) -> PlaceId {
    PlaceId::new(value).unwrap()
}
