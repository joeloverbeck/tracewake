mod container_item_move_001;
mod debug_attach_001;
mod debug_omniscience_excluded_001;
mod door_access_001;
mod expectation_contradiction_001;
mod food_unavailable_replan_001;
mod hidden_food_closed_container_001;
mod hidden_food_unknown_route_001;
mod hidden_route_edge_001;
mod hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001;
mod knowledge_blocker_accuse_001;
mod method_fallback_requires_new_trace_or_stuck_001;
mod no_hidden_truth_planning_001;
mod no_human_advance_001;
mod no_human_current_place_without_sleep_affordance_does_not_sleep_001;
mod no_human_day_001;
mod no_human_epistemic_check_001;
mod no_human_known_workplace_requires_provenance_001;
mod no_human_unseen_workplace_assignment_does_not_plan_work_001;
mod ordinary_workday_001;
mod planner_trace_001;
mod possession_does_not_reset_intention_001;
mod possession_parity_001;
mod prose_born_fact_rejected_001;
mod replay_item_location_001;
mod routine_blocked_diagnostic_001;
mod routine_no_teleport_001;
mod scheduler_cannot_rewrite_wait_reason_after_transaction_001;
mod sleep_eat_work_001;
mod sleep_rejects_current_place_without_sleep_affordance_001;
mod sound_uncertainty_001;
mod strongbox_001;
mod view_filtering_001;
mod view_model_local_actions_001;
mod workplace_assignment_provenance_001;

use tracewake_core::agent::{NeedKind, RoutineCondition, RoutineFamily, RoutineStep};
use tracewake_core::epistemics::observation::EPISTEMIC_RECORD_SCHEMA_V1;
use tracewake_core::epistemics::{
    Channel, Confidence, PrivacyScope, Proposition, SourceRef, Stance,
};
use tracewake_core::ids::{
    ActionId, ActorId, BeliefId, ContainerId, DoorId, EventId, FixtureId, FoodSupplyId, ItemId,
    PlaceId, RoutineTemplateId, SchemaVersion, SemanticActionId, SleepAffordanceId,
};
use tracewake_core::location::Location;
use tracewake_core::time::SimTick;

use crate::load::SourceFile;
use crate::schema::{
    ActionAffordanceSchema, ActorSchema, ContainerSchema, DayWindowSchema, DoorSchema,
    FixtureSchema, FixtureScope, FoodSupplySchema, HomeSchema, InitialBeliefSchema,
    InitialNeedSchema, ItemSchema, PlaceSchema, RoutineAssignmentSchema, RoutineTemplateSchema,
    SleepPlaceSchema, WorkplaceSchema,
};
use crate::serialization::serialize_fixture;

pub use container_item_move_001::container_item_move_001;
pub use debug_attach_001::debug_attach_001;
pub use debug_omniscience_excluded_001::debug_omniscience_excluded_001;
pub use door_access_001::door_access_001;
pub use expectation_contradiction_001::expectation_contradiction_001;
pub use food_unavailable_replan_001::food_unavailable_replan_001;
pub use hidden_food_closed_container_001::hidden_food_closed_container_001;
pub use hidden_food_unknown_route_001::hidden_food_unknown_route_001;
pub use hidden_route_edge_001::hidden_route_edge_001;
pub use hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001::hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001;
pub use knowledge_blocker_accuse_001::knowledge_blocker_accuse_001;
pub use method_fallback_requires_new_trace_or_stuck_001::method_fallback_requires_new_trace_or_stuck_001;
pub use no_hidden_truth_planning_001::no_hidden_truth_planning_001;
pub use no_human_advance_001::no_human_advance_001;
pub use no_human_current_place_without_sleep_affordance_does_not_sleep_001::no_human_current_place_without_sleep_affordance_does_not_sleep_001;
pub use no_human_day_001::no_human_day_001;
pub use no_human_epistemic_check_001::no_human_epistemic_check_001;
pub use no_human_known_workplace_requires_provenance_001::no_human_known_workplace_requires_provenance_001;
pub use no_human_unseen_workplace_assignment_does_not_plan_work_001::no_human_unseen_workplace_assignment_does_not_plan_work_001;
pub use ordinary_workday_001::ordinary_workday_001;
pub use planner_trace_001::planner_trace_001;
pub use possession_does_not_reset_intention_001::possession_does_not_reset_intention_001;
pub use possession_parity_001::possession_parity_001;
pub use prose_born_fact_rejected_001::prose_born_fact_rejected_001;
pub use replay_item_location_001::replay_item_location_001;
pub use routine_blocked_diagnostic_001::routine_blocked_diagnostic_001;
pub use routine_no_teleport_001::routine_no_teleport_001;
pub use scheduler_cannot_rewrite_wait_reason_after_transaction_001::scheduler_cannot_rewrite_wait_reason_after_transaction_001;
pub use sleep_eat_work_001::sleep_eat_work_001;
pub use sleep_rejects_current_place_without_sleep_affordance_001::sleep_rejects_current_place_without_sleep_affordance_001;
pub use sound_uncertainty_001::sound_uncertainty_001;
pub use strongbox_001::strongbox_001;
pub use view_filtering_001::view_filtering_001;
pub use view_model_local_actions_001::view_model_local_actions_001;
pub use workplace_assignment_provenance_001::workplace_assignment_provenance_001;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GoldenFixture {
    pub fixture: FixtureSchema,
    pub contract: FixtureContract,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FixtureContract {
    pub fixture_id: &'static str,
    pub purpose: &'static str,
    pub setup: Vec<&'static str>,
    pub allowed_actions: Vec<&'static str>,
    pub expected_events_or_reports: Vec<&'static str>,
    pub acceptance_assertions: Vec<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FixtureContractViolation {
    pub fixture_id: &'static str,
    pub code: &'static str,
    pub message: String,
}

impl GoldenFixture {
    pub fn source_file(&self) -> SourceFile {
        SourceFile {
            path: format!("{}.twf", self.fixture.fixture_id.as_str()),
            bytes: serialize_fixture(&self.fixture),
        }
    }
}

pub fn all() -> Vec<GoldenFixture> {
    vec![
        strongbox_001(),
        expectation_contradiction_001(),
        possession_parity_001(),
        view_filtering_001(),
        knowledge_blocker_accuse_001(),
        sound_uncertainty_001(),
        no_human_epistemic_check_001(),
        container_item_move_001(),
        door_access_001(),
        debug_attach_001(),
        no_human_advance_001(),
        replay_item_location_001(),
        view_model_local_actions_001(),
        ordinary_workday_001(),
        sleep_eat_work_001(),
        food_unavailable_replan_001(),
        hidden_food_closed_container_001(),
        hidden_food_unknown_route_001(),
        workplace_assignment_provenance_001(),
        hidden_route_edge_001(),
        debug_omniscience_excluded_001(),
        routine_blocked_diagnostic_001(),
        planner_trace_001(),
        routine_no_teleport_001(),
        possession_does_not_reset_intention_001(),
        no_hidden_truth_planning_001(),
        no_human_unseen_workplace_assignment_does_not_plan_work_001(),
        no_human_current_place_without_sleep_affordance_does_not_sleep_001(),
        sleep_rejects_current_place_without_sleep_affordance_001(),
        no_human_known_workplace_requires_provenance_001(),
        scheduler_cannot_rewrite_wait_reason_after_transaction_001(),
        method_fallback_requires_new_trace_or_stuck_001(),
        hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001(),
        no_human_day_001(),
    ]
}

fn hidden_truth_adversarial_fixture(
    fixture_id_value: &'static str,
    purpose: &'static str,
    setup: Vec<&'static str>,
    acceptance_assertions: Vec<&'static str>,
) -> GoldenFixture {
    let mut fixture = FixtureSchema {
        fixture_id: fixture_id(fixture_id_value),
        schema_version: schema_version(),
        fixture_scope: FixtureScope::Phase3AHistorical,
        actors: vec![actor_schema("actor_mara", "home_mara")],
        places: vec![
            place_schema("home_mara", "Mara home", &["hidden_workshop"]),
            place_schema("hidden_workshop", "Hidden workshop", &["home_mara"]),
        ],
        doors: Vec::new(),
        containers: vec![container_schema(
            "hidden_pantry",
            "home_mara",
            false,
            false,
            &[],
            false,
        )],
        items: Vec::new(),
        affordances: vec![
            affordance("eat", "food_hidden_pantry"),
            affordance("move", "hidden_workshop"),
            affordance("work_block", "workplace_hidden"),
        ],
        initial_beliefs: Vec::new(),
        initial_needs: vec![initial_need("actor_mara", NeedKind::Hunger, 880)],
        homes: vec![home_schema("actor_mara", "home_mara")],
        sleep_places: Vec::new(),
        food_supplies: vec![food_supply_in_container(
            "food_hidden_pantry",
            "hidden_pantry",
            1,
            220,
        )],
        workplaces: vec![workplace_schema(
            "workplace_hidden",
            "hidden_workshop",
            &["actor_mara"],
            4,
            true,
        )],
        routine_templates: vec![routine_template_schema(
            "routine_mara_hidden_truth_guard",
            RoutineFamily::EatMeal,
            vec![routine_step("consume_accessible_food", "eat")],
            &["food_missing", "food_inaccessible"],
        )],
        routine_assignments: vec![routine_assignment_schema(
            "actor_mara",
            "routine_mara_hidden_truth_guard",
            0,
            6,
        )],
        day_windows: vec![day_window_schema("actor_mara", 0, 8)],
    };
    fixture.canonicalize();
    GoldenFixture {
        fixture,
        contract: FixtureContract {
            fixture_id: fixture_id_value,
            purpose,
            setup,
            allowed_actions: vec![
                "construct actor-known context from modeled visible inputs",
                "run planner/transaction with actor-known provenance only",
                "reject hidden physical truth absent observation or belief",
            ],
            expected_events_or_reports: vec![
                "hidden physical facts are absent from actor-known context",
                "planner proof provenance is actor-known-only",
                "debug-only facts are not admitted to planning context",
            ],
            acceptance_assertions,
        },
    }
}

pub fn validate_fixture_contract_metadata(
    contract: &FixtureContract,
) -> Vec<FixtureContractViolation> {
    let mut violations = Vec::new();
    let is_no_human_contract = contract.fixture_id.contains("no_human");
    if !is_no_human_contract {
        return violations;
    }

    let has_structured_behavior_proof = contract
        .expected_events_or_reports
        .iter()
        .any(|entry| entry.starts_with("autonomous_no_human_event="));
    let has_log_metric = contract
        .expected_events_or_reports
        .iter()
        .any(|entry| entry.starts_with("log_derived_metric="));
    if !has_structured_behavior_proof || !has_log_metric {
        violations.push(FixtureContractViolation {
            fixture_id: contract.fixture_id,
            code: "text_only_behavior_proof",
            message: format!(
                "fixture {} expected events/reports must use autonomous event and log-derived metric rows",
                contract.fixture_id
            ),
        });
    }

    let rejects_manual_forcing = contract
        .acceptance_assertions
        .iter()
        .any(|entry| entry.contains("not manually forced"));
    if !rejects_manual_forcing {
        violations.push(FixtureContractViolation {
            fixture_id: contract.fixture_id,
            code: "manual_action_ambiguity",
            message: format!(
                "fixture {} must distinguish autonomous no-human events from manually forced action-unit events",
                contract.fixture_id
            ),
        });
    }

    violations
}

pub fn by_id(fixture_id: &str) -> Option<GoldenFixture> {
    all()
        .into_iter()
        .find(|golden| golden.fixture.fixture_id.as_str() == fixture_id)
}

fn fixture_id(value: &str) -> FixtureId {
    FixtureId::new(value).unwrap()
}

fn schema_version() -> SchemaVersion {
    SchemaVersion::new("schema_v1").unwrap()
}

fn actor(value: &str) -> ActorId {
    ActorId::new(value).unwrap()
}

fn place(value: &str) -> PlaceId {
    PlaceId::new(value).unwrap()
}

fn door(value: &str) -> DoorId {
    DoorId::new(value).unwrap()
}

fn container(value: &str) -> ContainerId {
    ContainerId::new(value).unwrap()
}

fn item(value: &str) -> ItemId {
    ItemId::new(value).unwrap()
}

fn action(value: &str) -> ActionId {
    ActionId::new(value).unwrap()
}

fn routine_template(value: &str) -> RoutineTemplateId {
    RoutineTemplateId::new(value).unwrap()
}

fn semantic_action(value: &str) -> SemanticActionId {
    SemanticActionId::new(value).unwrap()
}

fn actor_schema(actor_id: &str, current_place_id: &str) -> ActorSchema {
    ActorSchema {
        actor_id: actor(actor_id),
        current_place_id: place(current_place_id),
    }
}

fn place_schema(place_id: &str, display_label: &str, adjacent_place_ids: &[&str]) -> PlaceSchema {
    PlaceSchema {
        place_id: place(place_id),
        display_label: display_label.to_string(),
        adjacent_place_ids: adjacent_place_ids.iter().map(|id| place(id)).collect(),
    }
}

fn door_schema(
    door_id: &str,
    endpoint_a: &str,
    endpoint_b: &str,
    is_open: bool,
    is_locked: bool,
) -> DoorSchema {
    DoorSchema {
        door_id: door(door_id),
        endpoint_a: place(endpoint_a),
        endpoint_b: place(endpoint_b),
        is_open,
        is_locked,
    }
}

fn container_schema(
    container_id: &str,
    place_id: &str,
    is_open: bool,
    is_locked: bool,
    contents: &[&str],
    contents_visible_when_closed: bool,
) -> ContainerSchema {
    ContainerSchema {
        container_id: container(container_id),
        place_id: place(place_id),
        is_open,
        is_locked,
        contents: contents.iter().map(|id| item(id)).collect(),
        contents_visible_when_closed,
    }
}

fn item_in_container(item_id: &str, container_id: &str, portable: bool) -> ItemSchema {
    ItemSchema {
        item_id: item(item_id),
        portable,
        location: Location::InContainer(container(container_id)),
    }
}

fn item_at_place(item_id: &str, place_id: &str, portable: bool) -> ItemSchema {
    ItemSchema {
        item_id: item(item_id),
        portable,
        location: Location::AtPlace(place(place_id)),
    }
}

fn item_carried_by(item_id: &str, actor_id: &str, portable: bool) -> ItemSchema {
    ItemSchema {
        item_id: item(item_id),
        portable,
        location: Location::CarriedBy(actor(actor_id)),
    }
}

fn affordance(action_id: &str, target_id: &str) -> ActionAffordanceSchema {
    ActionAffordanceSchema {
        action_id: action(action_id),
        target_id: target_id.to_string(),
    }
}

fn initial_need(actor_id: &str, kind: NeedKind, value: u16) -> InitialNeedSchema {
    InitialNeedSchema {
        actor_id: actor(actor_id),
        kind,
        value,
    }
}

fn home_schema(actor_id: &str, place_id: &str) -> HomeSchema {
    HomeSchema {
        actor_id: actor(actor_id),
        place_id: place(place_id),
    }
}

fn sleep_place_schema(actor_id: &str, place_id: &str, sleep_place_id: &str) -> SleepPlaceSchema {
    SleepPlaceSchema {
        actor_id: actor(actor_id),
        place_id: place(place_id),
        sleep_place_id: SleepAffordanceId::new(sleep_place_id).unwrap(),
        access_open: true,
    }
}

fn food_supply_at_place(
    food_supply_id: &str,
    place_id: &str,
    servings: u32,
    hunger_reduction_per_serving: i32,
) -> FoodSupplySchema {
    FoodSupplySchema {
        food_supply_id: food_supply_id.parse().unwrap(),
        location: Location::AtPlace(place(place_id)),
        servings,
        hunger_reduction_per_serving,
    }
}

fn food_supply_in_container(
    food_supply_id: &str,
    container_id: &str,
    servings: u32,
    hunger_reduction_per_serving: i32,
) -> FoodSupplySchema {
    FoodSupplySchema {
        food_supply_id: FoodSupplyId::new(food_supply_id).unwrap(),
        location: Location::InContainer(container(container_id)),
        servings,
        hunger_reduction_per_serving,
    }
}

fn workplace_schema(
    workplace_id: &str,
    place_id: &str,
    assigned_actor_ids: &[&str],
    work_duration_ticks: u64,
    access_open: bool,
) -> WorkplaceSchema {
    WorkplaceSchema {
        workplace_id: workplace_id.parse().unwrap(),
        place_id: place(place_id),
        assigned_actor_ids: assigned_actor_ids.iter().map(|id| actor(id)).collect(),
        work_duration_ticks,
        fatigue_delta_per_tick: 8,
        hunger_delta_per_tick: 4,
        max_fatigue_to_start: 800,
        max_hunger_to_start: 850,
        access_open,
        output_tag: format!("{workplace_id}_ordinary_output"),
    }
}

fn routine_step(kind: &str, action_id: &str) -> RoutineStep {
    let action_id = semantic_action(action_id);
    match kind {
        "move_toward_place" => RoutineStep::MoveTowardPlace { action_id },
        "check_known_container" => RoutineStep::CheckKnownContainer { action_id },
        "consume_accessible_food" => RoutineStep::ConsumeAccessibleFood { action_id },
        "start_scheduled_sleep" => RoutineStep::StartScheduledSleep { action_id },
        "start_work_block" => RoutineStep::StartWorkBlock { action_id },
        "continue_current_step" => RoutineStep::ContinueCurrentStep { action_id },
        "fallback_to_find_food" => RoutineStep::FallbackToFindFood { action_id },
        _ => panic!("unsupported routine step kind {kind}"),
    }
}

fn wait_step(reason: &str) -> RoutineStep {
    RoutineStep::WaitUntil {
        reason: reason.to_string(),
    }
}

fn routine_template_schema(
    template_id: &str,
    family: RoutineFamily,
    steps: Vec<RoutineStep>,
    failure_modes: &[&str],
) -> RoutineTemplateSchema {
    RoutineTemplateSchema {
        template_id: routine_template(template_id),
        family,
        applicability_conditions: vec![RoutineCondition::FixtureAuthoredPossibility],
        preconditions: vec![RoutineCondition::SharedPipelinePreconditions],
        steps,
        min_duration_ticks: 1,
        max_duration_ticks: 12,
        interruption_points: vec![0],
        failure_modes: failure_modes.iter().map(|mode| mode.to_string()).collect(),
        fallback_rules: vec!["fallback_wait_with_reason".to_string()],
        debug_labels: vec![template_id.to_string()],
        reservable_resource: Some("body".to_string()),
    }
}

fn routine_assignment_schema(
    actor_id: &str,
    template_id: &str,
    start_tick: u64,
    end_tick: u64,
) -> RoutineAssignmentSchema {
    RoutineAssignmentSchema {
        actor_id: actor(actor_id),
        template_id: routine_template(template_id),
        start_tick: SimTick::new(start_tick),
        end_tick: SimTick::new(end_tick),
    }
}

fn day_window_schema(actor_id: &str, start_tick: u64, end_tick: u64) -> DayWindowSchema {
    DayWindowSchema {
        actor_id: actor(actor_id),
        start_tick: SimTick::new(start_tick),
        end_tick: SimTick::new(end_tick),
    }
}

fn tomas_coin_expectation_seed() -> InitialBeliefSchema {
    expectation_seed(
        "belief_tomas_expects_coin_stack_01_in_strongbox_tomas",
        "actor_tomas",
        "coin_stack_01",
        "strongbox_tomas",
        "prehistory_tomas_checked_strongbox_before_start",
    )
}

fn expectation_seed(
    belief_id: &str,
    holder_actor_id: &str,
    item_id: &str,
    container_id: &str,
    source_id: &str,
) -> InitialBeliefSchema {
    InitialBeliefSchema::new_expectation(
        BeliefId::new(belief_id).unwrap(),
        actor(holder_actor_id),
        Proposition::ItemLocatedInContainer {
            item_id: item(item_id),
            container_id: container(container_id),
        },
        Confidence::new(900).unwrap(),
        SourceRef::Event(EventId::new(source_id).unwrap()),
        SimTick::ZERO,
    )
}

fn sound_lead_seed(
    belief_id: &str,
    holder_actor_id: &str,
    place_id: &str,
    source_id: &str,
) -> InitialBeliefSchema {
    InitialBeliefSchema {
        belief_id: BeliefId::new(belief_id).unwrap(),
        holder_actor_id: actor(holder_actor_id),
        proposition: Proposition::SoundHeardNearPlace {
            place_id: place(place_id),
        },
        stance: Stance::Plausible,
        confidence: Confidence::new(250).unwrap(),
        source_kind: tracewake_core::events::InitialBeliefSourceKind::AuthoredPrehistory,
        source: SourceRef::Event(EventId::new(source_id).unwrap()),
        channel: Some(Channel::SimpleSound),
        acquired_tick: SimTick::ZERO,
        last_verified_tick: None,
        privacy_scope: PrivacyScope::ActorPrivate(actor(holder_actor_id)),
        schema_version: SchemaVersion::new(EPISTEMIC_RECORD_SCHEMA_V1).unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn by_id_returns_matching_golden() {
        let golden = by_id("strongbox_001").unwrap();

        assert_eq!(golden.fixture.fixture_id.as_str(), "strongbox_001");
    }

    #[test]
    fn by_id_returns_none_for_unknown_id() {
        assert!(by_id("missing_fixture").is_none());
    }
}
