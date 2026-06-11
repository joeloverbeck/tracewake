use std::collections::{BTreeMap, BTreeSet};

use tracewake_core::actions::{ActionEffect, ActionRegistry, ActionScope};
use tracewake_core::agent::need::NEED_MAX;
use tracewake_core::agent::{RoutineFamily, RoutineStepProposal};
use tracewake_core::epistemics::observation::EPISTEMIC_RECORD_SCHEMA_V1;
use tracewake_core::epistemics::{PrivacyScope, SourceRef};
use tracewake_core::events::InitialBeliefSourceKind;
use tracewake_core::ids::{ActionId, FoodSupplyId, PlaceId, WorkplaceId};
use tracewake_core::location::Location;
use tracewake_core::state::PhysicalState;

use crate::schema::{
    content_field_by_canonical_key, ActionAffordanceSchema, FixtureSchema, FixtureScope,
};
pub use crate::schema::{content_field_registry, ValidationPhase};
use crate::serialization::{deserialize_fixture, serialize_fixture, SerializationError};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InitialWorld {
    pub fixture: FixtureSchema,
    pub physical_state: PhysicalState,
    pub validation_token: FixtureValidationToken,
}

/// Proof that a fixture passed the content validation gate.
///
/// The token can be named outside the crate, but only `tracewake-content` can
/// mint it after validation succeeds.
///
/// ```compile_fail
/// use tracewake_content::validate::FixtureValidationToken;
///
/// let _token = FixtureValidationToken { private: () };
/// ```
///
/// ```compile_fail
/// use tracewake_content::validate::FixtureValidationToken;
///
/// let _token = FixtureValidationToken::mint();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FixtureValidationToken {
    private: (),
}

impl FixtureValidationToken {
    pub(crate) const fn mint() -> Self {
        Self { private: () }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContentValidationReport {
    pub status: ValidationStatus,
    pub errors: Vec<ContentValidationError>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValidationStatus {
    Accepted,
    Rejected,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContentValidationError {
    pub phase: ValidationPhase,
    pub path: String,
    pub code: &'static str,
    pub message: String,
}

impl ContentValidationError {
    fn new(
        phase: ValidationPhase,
        path: impl Into<String>,
        code: &'static str,
        message: impl Into<String>,
    ) -> Self {
        Self {
            phase,
            path: path.into(),
            code,
            message: message.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContentValidationFailure {
    pub report: ContentValidationReport,
}

impl ContentValidationFailure {
    fn from_errors(mut errors: Vec<ContentValidationError>) -> Self {
        errors.sort_by(|left, right| {
            (left.phase, &left.path, left.code, &left.message).cmp(&(
                right.phase,
                &right.path,
                right.code,
                &right.message,
            ))
        });
        Self {
            report: ContentValidationReport {
                status: ValidationStatus::Rejected,
                errors,
            },
        }
    }
}

pub fn validate_fixture_bytes(
    bytes: &[u8],
    registry: &ActionRegistry,
) -> Result<InitialWorld, ContentValidationFailure> {
    let mut errors = validate_raw_lines(bytes);
    let fixture = match deserialize_fixture(bytes) {
        Ok(fixture) => fixture,
        Err(error) => {
            errors.push(serialization_error(error));
            return Err(ContentValidationFailure::from_errors(errors));
        }
    };
    errors.extend(validate_fixture_errors(&fixture, registry));
    if errors.is_empty() {
        Ok(accepted_world(fixture))
    } else {
        Err(ContentValidationFailure::from_errors(errors))
    }
}

pub fn validate_fixture(
    fixture: &FixtureSchema,
    registry: &ActionRegistry,
) -> Result<InitialWorld, ContentValidationFailure> {
    let errors = validate_fixture_errors(fixture, registry);
    if errors.is_empty() {
        Ok(accepted_world(fixture.clone()))
    } else {
        Err(ContentValidationFailure::from_errors(errors))
    }
}

fn accepted_world(mut fixture: FixtureSchema) -> InitialWorld {
    fixture.canonicalize();
    let physical_state = fixture.to_physical_state();
    InitialWorld {
        fixture,
        physical_state,
        validation_token: FixtureValidationToken::mint(),
    }
}

fn validate_fixture_errors(
    fixture: &FixtureSchema,
    registry: &ActionRegistry,
) -> Vec<ContentValidationError> {
    let mut errors = Vec::new();
    validate_ids(fixture, &mut errors);
    validate_references(fixture, &mut errors);
    validate_locations(fixture, &mut errors);
    validate_topology(fixture, &mut errors);
    validate_state(fixture, &mut errors);
    validate_action_registry_parity(fixture, registry, &mut errors);
    validate_routine_rules(fixture, registry, &mut errors);
    validate_phase3a_sleep_surface_contract(fixture, &mut errors);
    validate_semantic_ids(fixture, &mut errors);
    validate_no_player(fixture, &mut errors);
    validate_no_script(fixture, &mut errors);
    validate_phase3a_no_shortcuts(fixture, &mut errors);
    validate_epistemic_seeds(fixture, registry, &mut errors);
    validate_determinism(fixture, &mut errors);
    validate_fixture_contract(fixture, &mut errors);
    validate_serialization_roundtrip(fixture, &mut errors);
    errors
}

fn validate_raw_lines(bytes: &[u8]) -> Vec<ContentValidationError> {
    let mut errors = Vec::new();
    let text = match std::str::from_utf8(bytes) {
        Ok(text) => text,
        Err(_) => {
            errors.push(ContentValidationError::new(
                ValidationPhase::ParseSchema,
                "fixture",
                "non_utf8",
                "fixture source must be UTF-8",
            ));
            return errors;
        }
    };

    for (index, line) in text.lines().enumerate() {
        let line_no = index + 1;
        let tag = line.split('|').next().unwrap_or_default();
        if is_forbidden_key(tag) {
            let phase = if is_player_key(tag) {
                ValidationPhase::NoPlayer
            } else {
                ValidationPhase::NoScript
            };
            errors.push(ContentValidationError::new(
                phase,
                format!("line[{line_no}].{tag}"),
                "forbidden_form",
                format!("forbidden content form {tag}"),
            ));
        } else if content_field_by_canonical_key(tag).is_none() {
            errors.push(ContentValidationError::new(
                ValidationPhase::ParseSchema,
                format!("line[{line_no}].{tag}"),
                "unknown_field",
                format!("unknown content section {tag}"),
            ));
            if contains_prose_born_fact_marker(line) {
                errors.push(ContentValidationError::new(
                    ValidationPhase::NoScript,
                    format!("line[{line_no}].{tag}"),
                    "prose_born_fact",
                    "prose or notes must not imply culprit, outcome, or hidden truth facts",
                ));
            }
            if tag.starts_with("routine_") && contains_direct_state_or_script_marker(line) {
                errors.push(ContentValidationError::new(
                    ValidationPhase::NoScript,
                    format!("line[{line_no}].{tag}"),
                    "authored_shortcut_effect",
                    format!("unknown routine content section {tag} contains behavior shortcut"),
                ));
            }
        }

        let parts = line.split('|').collect::<Vec<_>>();
        if parts.get(1).is_some_and(|value| value.is_empty()) {
            if let Some(registration) = content_field_by_canonical_key(tag) {
                errors.push(ContentValidationError::new(
                    registration.validation_phase,
                    format!("line[{line_no}].{tag}.id"),
                    registration.diagnostic_code,
                    "stable ID field must not be empty",
                ));
            }
        }
    }
    errors
}

fn validate_ids(fixture: &FixtureSchema, errors: &mut Vec<ContentValidationError>) {
    let mut seen: BTreeMap<String, Vec<String>> = BTreeMap::new();
    seen.entry(fixture.fixture_id.as_str().to_string())
        .or_default()
        .push("fixture.fixture_id".to_string());
    seen.entry(fixture.schema_version.as_str().to_string())
        .or_default()
        .push("fixture.schema_version".to_string());

    for (index, actor) in fixture.actors.iter().enumerate() {
        push_id(
            &mut seen,
            actor.actor_id.as_str(),
            format!("actors[{index}].actor_id"),
        );
        reject_reserved_or_display(
            actor.actor_id.as_str(),
            format!("actors[{index}].actor_id"),
            errors,
        );
    }
    for (index, place) in fixture.places.iter().enumerate() {
        push_id(
            &mut seen,
            place.place_id.as_str(),
            format!("places[{index}].place_id"),
        );
        reject_reserved_or_display(
            place.place_id.as_str(),
            format!("places[{index}].place_id"),
            errors,
        );
    }
    for (index, door) in fixture.doors.iter().enumerate() {
        push_id(
            &mut seen,
            door.door_id.as_str(),
            format!("doors[{index}].door_id"),
        );
        reject_reserved_or_display(
            door.door_id.as_str(),
            format!("doors[{index}].door_id"),
            errors,
        );
    }
    for (index, container) in fixture.containers.iter().enumerate() {
        push_id(
            &mut seen,
            container.container_id.as_str(),
            format!("containers[{index}].container_id"),
        );
        reject_reserved_or_display(
            container.container_id.as_str(),
            format!("containers[{index}].container_id"),
            errors,
        );
    }
    for (index, item) in fixture.items.iter().enumerate() {
        push_id(
            &mut seen,
            item.item_id.as_str(),
            format!("items[{index}].item_id"),
        );
        reject_reserved_or_display(
            item.item_id.as_str(),
            format!("items[{index}].item_id"),
            errors,
        );
    }
    for (index, belief) in fixture.initial_beliefs.iter().enumerate() {
        push_id(
            &mut seen,
            belief.belief_id.as_str(),
            format!("initial_beliefs[{index}].belief_id"),
        );
        reject_reserved_or_display(
            belief.belief_id.as_str(),
            format!("initial_beliefs[{index}].belief_id"),
            errors,
        );
    }
    for (index, food) in fixture.food_supplies.iter().enumerate() {
        push_id(
            &mut seen,
            food.food_supply_id.as_str(),
            format!("food_supplies[{index}].food_supply_id"),
        );
        reject_reserved_or_display(
            food.food_supply_id.as_str(),
            format!("food_supplies[{index}].food_supply_id"),
            errors,
        );
    }
    for (index, workplace) in fixture.workplaces.iter().enumerate() {
        push_id(
            &mut seen,
            workplace.workplace_id.as_str(),
            format!("workplaces[{index}].workplace_id"),
        );
        reject_reserved_or_display(
            workplace.workplace_id.as_str(),
            format!("workplaces[{index}].workplace_id"),
            errors,
        );
    }
    for (index, sleep_place) in fixture.sleep_places.iter().enumerate() {
        push_id(
            &mut seen,
            sleep_place.sleep_place_id.as_str(),
            format!("sleep_places[{index}].sleep_place_id"),
        );
        reject_reserved_or_display(
            sleep_place.sleep_place_id.as_str(),
            format!("sleep_places[{index}].sleep_place_id"),
            errors,
        );
    }
    for (index, template) in fixture.routine_templates.iter().enumerate() {
        push_id(
            &mut seen,
            template.template_id.as_str(),
            format!("routine_templates[{index}].template_id"),
        );
        reject_reserved_or_display(
            template.template_id.as_str(),
            format!("routine_templates[{index}].template_id"),
            errors,
        );
    }

    let mut initial_need_keys = BTreeSet::new();
    for (index, need) in fixture.initial_needs.iter().enumerate() {
        if !initial_need_keys.insert((need.actor_id.clone(), need.kind)) {
            errors.push(ContentValidationError::new(
                ValidationPhase::Id,
                format!("initial_needs[{index}]"),
                "duplicate_id",
                format!(
                    "duplicate initial need {} for {}",
                    need.kind.stable_id(),
                    need.actor_id.as_str()
                ),
            ));
        }
    }

    let mut known_food_source_keys = BTreeSet::new();
    for (index, edge) in fixture.known_food_sources.iter().enumerate() {
        if !known_food_source_keys.insert((edge.actor_id.clone(), edge.food_supply_id.clone())) {
            errors.push(ContentValidationError::new(
                ValidationPhase::Id,
                format!("known_food_sources[{index}]"),
                "duplicate_id",
                format!(
                    "duplicate known food source edge {} -> {}",
                    edge.actor_id.as_str(),
                    edge.food_supply_id.as_str()
                ),
            ));
        }
    }

    for (id, paths) in seen {
        if paths.len() > 1 {
            errors.push(ContentValidationError::new(
                ValidationPhase::Id,
                paths.join(","),
                "duplicate_id",
                format!("duplicate stable ID {id} at {}", paths.join(",")),
            ));
        }
    }
}

fn push_id(seen: &mut BTreeMap<String, Vec<String>>, id: &str, path: String) {
    seen.entry(id.to_string()).or_default().push(path);
}

fn reject_reserved_or_display(id: &str, path: String, errors: &mut Vec<ContentValidationError>) {
    if matches!(
        id,
        "player" | "protagonist" | "quest" | "objective" | "reward" | "culprit" | "director"
    ) {
        errors.push(ContentValidationError::new(
            ValidationPhase::NoPlayer,
            path,
            "reserved_player_or_story_id",
            format!("reserved player/story ID {id} is forbidden"),
        ));
    } else if id.contains(' ') {
        errors.push(ContentValidationError::new(
            ValidationPhase::Id,
            path,
            "display_name_as_id",
            "stable IDs must not be display names",
        ));
    }
}

fn validate_references(fixture: &FixtureSchema, errors: &mut Vec<ContentValidationError>) {
    let actors = fixture
        .actors
        .iter()
        .map(|actor| actor.actor_id.clone())
        .collect::<BTreeSet<_>>();
    let places = fixture
        .places
        .iter()
        .map(|place| place.place_id.clone())
        .collect::<BTreeSet<_>>();
    let containers = fixture
        .containers
        .iter()
        .map(|container| container.container_id.clone())
        .collect::<BTreeSet<_>>();
    let items = fixture
        .items
        .iter()
        .map(|item| item.item_id.clone())
        .collect::<BTreeSet<_>>();
    let food_supplies = fixture
        .food_supplies
        .iter()
        .map(|food| food.food_supply_id.clone())
        .collect::<BTreeSet<_>>();
    let workplaces = fixture
        .workplaces
        .iter()
        .map(|workplace| workplace.workplace_id.clone())
        .collect::<BTreeSet<_>>();
    let routine_templates = fixture
        .routine_templates
        .iter()
        .map(|template| template.template_id.clone())
        .collect::<BTreeSet<_>>();

    for (index, actor) in fixture.actors.iter().enumerate() {
        if !places.contains(&actor.current_place_id) {
            missing(
                errors,
                ValidationPhase::Referential,
                format!("actors[{index}].current_place_id"),
                actor.current_place_id.as_str(),
                "place",
            );
        }
    }
    for (index, place) in fixture.places.iter().enumerate() {
        for adjacent in &place.adjacent_place_ids {
            if !places.contains(adjacent) {
                missing(
                    errors,
                    ValidationPhase::Referential,
                    format!("places[{index}].adjacent_place_ids"),
                    adjacent.as_str(),
                    "place",
                );
            }
        }
    }
    for (index, container) in fixture.containers.iter().enumerate() {
        if !places.contains(&container.place_id) {
            missing(
                errors,
                ValidationPhase::Referential,
                format!("containers[{index}].place_id"),
                container.place_id.as_str(),
                "place",
            );
        }
        for item_id in &container.contents {
            if !items.contains(item_id) {
                missing(
                    errors,
                    ValidationPhase::Referential,
                    format!("containers[{index}].contents"),
                    item_id.as_str(),
                    "item",
                );
            }
        }
    }
    for (index, item) in fixture.items.iter().enumerate() {
        match &item.location {
            Location::AtPlace(place_id) if !places.contains(place_id) => missing(
                errors,
                ValidationPhase::Referential,
                format!("items[{index}].location"),
                place_id.as_str(),
                "place",
            ),
            Location::InContainer(container_id) if !containers.contains(container_id) => missing(
                errors,
                ValidationPhase::Referential,
                format!("items[{index}].location"),
                container_id.as_str(),
                "container",
            ),
            Location::CarriedBy(actor_id) if !actors.contains(actor_id) => missing(
                errors,
                ValidationPhase::Referential,
                format!("items[{index}].location"),
                actor_id.as_str(),
                "actor",
            ),
            _ => {}
        }
    }

    for (index, belief) in fixture.initial_beliefs.iter().enumerate() {
        if !actors.contains(&belief.holder_actor_id) {
            missing(
                errors,
                ValidationPhase::Referential,
                format!("initial_beliefs[{index}].holder_actor_id"),
                belief.holder_actor_id.as_str(),
                "actor",
            );
        }
        if let Err(error) =
            belief
                .proposition
                .validate_references(&actors, &places, &containers, &items)
        {
            errors.push(ContentValidationError::new(
                ValidationPhase::EpistemicSeed,
                format!("initial_beliefs[{index}].proposition"),
                "bad_reference",
                format!("{error}"),
            ));
        }
    }
    for (index, need) in fixture.initial_needs.iter().enumerate() {
        if !actors.contains(&need.actor_id) {
            missing(
                errors,
                ValidationPhase::Referential,
                format!("initial_needs[{index}].actor_id"),
                need.actor_id.as_str(),
                "actor",
            );
        }
    }
    for (index, home) in fixture.homes.iter().enumerate() {
        if !actors.contains(&home.actor_id) {
            missing(
                errors,
                ValidationPhase::Referential,
                format!("homes[{index}].actor_id"),
                home.actor_id.as_str(),
                "actor",
            );
        }
        if !places.contains(&home.place_id) {
            missing(
                errors,
                ValidationPhase::Referential,
                format!("homes[{index}].place_id"),
                home.place_id.as_str(),
                "place",
            );
        }
    }
    for (index, sleep_place) in fixture.sleep_places.iter().enumerate() {
        if !actors.contains(&sleep_place.actor_id) {
            missing(
                errors,
                ValidationPhase::Referential,
                format!("sleep_places[{index}].actor_id"),
                sleep_place.actor_id.as_str(),
                "actor",
            );
        }
        if !places.contains(&sleep_place.place_id) {
            missing(
                errors,
                ValidationPhase::Referential,
                format!("sleep_places[{index}].place_id"),
                sleep_place.place_id.as_str(),
                "place",
            );
        }
    }
    for (index, food) in fixture.food_supplies.iter().enumerate() {
        validate_location_reference(
            &food.location,
            &actors,
            &places,
            &containers,
            errors,
            format!("food_supplies[{index}].location"),
        );
    }
    for (index, edge) in fixture.known_food_sources.iter().enumerate() {
        if !actors.contains(&edge.actor_id) {
            missing(
                errors,
                ValidationPhase::Referential,
                format!("known_food_sources[{index}].actor_id"),
                edge.actor_id.as_str(),
                "actor",
            );
        }
        if !food_supplies.contains(&edge.food_supply_id) {
            missing(
                errors,
                ValidationPhase::Referential,
                format!("known_food_sources[{index}].food_supply_id"),
                edge.food_supply_id.as_str(),
                "food_supply",
            );
        }
    }
    for (index, workplace) in fixture.workplaces.iter().enumerate() {
        if !places.contains(&workplace.place_id) {
            missing(
                errors,
                ValidationPhase::Referential,
                format!("workplaces[{index}].place_id"),
                workplace.place_id.as_str(),
                "place",
            );
        }
        for actor_id in &workplace.assigned_actor_ids {
            if !actors.contains(actor_id) {
                missing(
                    errors,
                    ValidationPhase::Referential,
                    format!("workplaces[{index}].assigned_actor_ids"),
                    actor_id.as_str(),
                    "actor",
                );
            }
        }
    }
    for (index, template) in fixture.routine_templates.iter().enumerate() {
        if let Err(error) = template.to_template() {
            errors.push(ContentValidationError::new(
                ValidationPhase::State,
                format!("routine_templates[{index}]"),
                "invalid_routine_template",
                format!("{error:?}"),
            ));
        }
    }
    for (index, assignment) in fixture.routine_assignments.iter().enumerate() {
        if !actors.contains(&assignment.actor_id) {
            missing(
                errors,
                ValidationPhase::Referential,
                format!("routine_assignments[{index}].actor_id"),
                assignment.actor_id.as_str(),
                "actor",
            );
        }
        if !routine_templates.contains(&assignment.template_id) {
            missing(
                errors,
                ValidationPhase::Referential,
                format!("routine_assignments[{index}].template_id"),
                assignment.template_id.as_str(),
                "routine_template",
            );
        }
        if assignment.start_tick >= assignment.end_tick {
            errors.push(ContentValidationError::new(
                ValidationPhase::State,
                format!("routine_assignments[{index}]"),
                "bad_tick_order",
                "routine assignment start_tick must precede end_tick",
            ));
        }
    }
    for (index, window) in fixture.day_windows.iter().enumerate() {
        if !actors.contains(&window.actor_id) {
            missing(
                errors,
                ValidationPhase::Referential,
                format!("day_windows[{index}].actor_id"),
                window.actor_id.as_str(),
                "actor",
            );
        }
        if window.start_tick >= window.end_tick {
            errors.push(ContentValidationError::new(
                ValidationPhase::State,
                format!("day_windows[{index}]"),
                "bad_tick_order",
                "day window start_tick must precede end_tick",
            ));
        }
    }

    for (index, affordance) in fixture.affordances.iter().enumerate() {
        if let Ok(food_supply_id) = FoodSupplyId::new(affordance.target_id.as_str()) {
            if affordance.action_id.as_str() == "eat" && !food_supplies.contains(&food_supply_id) {
                missing(
                    errors,
                    ValidationPhase::Referential,
                    format!("affordances[{index}].target_id"),
                    affordance.target_id.as_str(),
                    "food_supply",
                );
            }
        }
        if let Ok(workplace_id) = WorkplaceId::new(affordance.target_id.as_str()) {
            if affordance.action_id.as_str() == "work_block" && !workplaces.contains(&workplace_id)
            {
                missing(
                    errors,
                    ValidationPhase::Referential,
                    format!("affordances[{index}].target_id"),
                    affordance.target_id.as_str(),
                    "workplace",
                );
            }
        }
    }
}

fn validate_location_reference(
    location: &Location,
    actors: &BTreeSet<tracewake_core::ids::ActorId>,
    places: &BTreeSet<PlaceId>,
    containers: &BTreeSet<tracewake_core::ids::ContainerId>,
    errors: &mut Vec<ContentValidationError>,
    path: String,
) {
    match location {
        Location::AtPlace(place_id) if !places.contains(place_id) => missing(
            errors,
            ValidationPhase::Referential,
            path,
            place_id.as_str(),
            "place",
        ),
        Location::InContainer(container_id) if !containers.contains(container_id) => missing(
            errors,
            ValidationPhase::Referential,
            path,
            container_id.as_str(),
            "container",
        ),
        Location::CarriedBy(actor_id) if !actors.contains(actor_id) => missing(
            errors,
            ValidationPhase::Referential,
            path,
            actor_id.as_str(),
            "actor",
        ),
        _ => {}
    }
}

fn validate_locations(fixture: &FixtureSchema, errors: &mut Vec<ContentValidationError>) {
    let item_locations = fixture
        .items
        .iter()
        .map(|item| (item.item_id.clone(), item.location.clone()))
        .collect::<BTreeMap<_, _>>();
    let mut listed_contents = BTreeSet::new();
    for (container_index, container) in fixture.containers.iter().enumerate() {
        let mut local_contents = BTreeSet::new();
        for item_id in &container.contents {
            if !local_contents.insert(item_id.clone()) {
                errors.push(ContentValidationError::new(
                    ValidationPhase::DeterminismHazard,
                    format!("containers[{container_index}].contents"),
                    "duplicate_contents",
                    format!("container contents duplicate item {}", item_id.as_str()),
                ));
            }
            if !listed_contents.insert(item_id.clone()) {
                errors.push(ContentValidationError::new(
                    ValidationPhase::Location,
                    format!("containers[{container_index}].contents"),
                    "item_double_location",
                    format!(
                        "item {} is listed by more than one container",
                        item_id.as_str()
                    ),
                ));
            }
            if item_locations.get(item_id)
                != Some(&Location::InContainer(container.container_id.clone()))
            {
                errors.push(ContentValidationError::new(
                    ValidationPhase::Location,
                    format!("containers[{container_index}].contents"),
                    "container_item_mismatch",
                    format!(
                        "container {} lists item {} but item holder disagrees",
                        container.container_id.as_str(),
                        item_id.as_str()
                    ),
                ));
            }
        }
    }

    for (item_index, item) in fixture.items.iter().enumerate() {
        if let Location::InContainer(container_id) = &item.location {
            let Some(container) = fixture
                .containers
                .iter()
                .find(|container| &container.container_id == container_id)
            else {
                continue;
            };
            if !container.contents.contains(&item.item_id) {
                errors.push(ContentValidationError::new(
                    ValidationPhase::Location,
                    format!("items[{item_index}].location"),
                    "container_item_mismatch",
                    format!(
                        "item {} is in container {} but container contents omit it",
                        item.item_id.as_str(),
                        container_id.as_str()
                    ),
                ));
            }
        }
    }
}

fn validate_topology(fixture: &FixtureSchema, errors: &mut Vec<ContentValidationError>) {
    let places = fixture
        .places
        .iter()
        .map(|place| place.place_id.clone())
        .collect::<BTreeSet<_>>();
    let adjacency = fixture
        .places
        .iter()
        .map(|place| (place.place_id.clone(), place.adjacent_place_ids.clone()))
        .collect::<BTreeMap<_, _>>();
    for (index, door) in fixture.doors.iter().enumerate() {
        if !places.contains(&door.endpoint_a) {
            missing(
                errors,
                ValidationPhase::PhysicalTopology,
                format!("doors[{index}].endpoint_a"),
                door.endpoint_a.as_str(),
                "place",
            );
        }
        if !places.contains(&door.endpoint_b) {
            missing(
                errors,
                ValidationPhase::PhysicalTopology,
                format!("doors[{index}].endpoint_b"),
                door.endpoint_b.as_str(),
                "place",
            );
        }
        let a_to_b = adjacency
            .get(&door.endpoint_a)
            .is_some_and(|ids| ids.contains(&door.endpoint_b));
        let b_to_a = adjacency
            .get(&door.endpoint_b)
            .is_some_and(|ids| ids.contains(&door.endpoint_a));
        if a_to_b != b_to_a {
            errors.push(ContentValidationError::new(
                ValidationPhase::PhysicalTopology,
                format!("doors[{index}]"),
                "door_adjacency_inconsistency",
                format!(
                    "door {} endpoints must have symmetric adjacency",
                    door.door_id.as_str()
                ),
            ));
        }
    }
}

fn validate_state(fixture: &FixtureSchema, errors: &mut Vec<ContentValidationError>) {
    validate_numeric_i32(
        fixture.need_model.awake_hunger_delta_per_tick,
        "need_model.awake_hunger_delta_per_tick",
        NumericFieldPolicy::PressureNonnegative,
        errors,
    );
    validate_numeric_i32(
        fixture.need_model.awake_fatigue_delta_per_tick,
        "need_model.awake_fatigue_delta_per_tick",
        NumericFieldPolicy::PressureNonnegative,
        errors,
    );
    for (index, need) in fixture.initial_needs.iter().enumerate() {
        validate_need_band_u16(need.value, format!("initial_needs[{index}].value"), errors);
    }
    for (index, door) in fixture.doors.iter().enumerate() {
        if door.is_locked && door.is_open {
            errors.push(ContentValidationError::new(
                ValidationPhase::State,
                format!("doors[{index}]"),
                "locked_open_state",
                "locked doors must not start open in Phase 1 fixtures",
            ));
        }
    }
    for (index, container) in fixture.containers.iter().enumerate() {
        if container.is_locked && container.is_open {
            errors.push(ContentValidationError::new(
                ValidationPhase::State,
                format!("containers[{index}]"),
                "locked_open_state",
                "locked containers must not start open in Phase 1 fixtures",
            ));
        }
    }
    for (index, sleep_place) in fixture.sleep_places.iter().enumerate() {
        validate_duration_u64(
            sleep_place.duration_ticks,
            format!("sleep_places[{index}].duration_ticks"),
            errors,
        );
        validate_numeric_i32(
            sleep_place.fatigue_recovery_per_tick,
            format!("sleep_places[{index}].fatigue_recovery_per_tick"),
            NumericFieldPolicy::ReliefPositive,
            errors,
        );
        validate_numeric_i32(
            sleep_place.hunger_rise_per_tick,
            format!("sleep_places[{index}].hunger_rise_per_tick"),
            NumericFieldPolicy::PressureNonnegative,
            errors,
        );
    }
    for (index, food) in fixture.food_supplies.iter().enumerate() {
        validate_numeric_i32(
            food.hunger_reduction_per_serving,
            format!("food_supplies[{index}].hunger_reduction_per_serving"),
            NumericFieldPolicy::ReliefPositive,
            errors,
        );
    }
    for (index, workplace) in fixture.workplaces.iter().enumerate() {
        validate_duration_u64(
            workplace.work_duration_ticks,
            format!("workplaces[{index}].work_duration_ticks"),
            errors,
        );
        validate_numeric_i32(
            workplace.fatigue_delta_per_tick,
            format!("workplaces[{index}].fatigue_delta_per_tick"),
            NumericFieldPolicy::PressureNonnegative,
            errors,
        );
        validate_numeric_i32(
            workplace.hunger_delta_per_tick,
            format!("workplaces[{index}].hunger_delta_per_tick"),
            NumericFieldPolicy::PressureNonnegative,
            errors,
        );
        validate_numeric_i32(
            workplace.max_fatigue_to_start,
            format!("workplaces[{index}].max_fatigue_to_start"),
            NumericFieldPolicy::NeedBandI32,
            errors,
        );
        validate_numeric_i32(
            workplace.max_hunger_to_start,
            format!("workplaces[{index}].max_hunger_to_start"),
            NumericFieldPolicy::NeedBandI32,
            errors,
        );
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum NumericFieldPolicy {
    PressureNonnegative,
    ReliefPositive,
    DurationAtLeastOne,
    NeedBandI32,
    NeedBandU16,
    CountNonnegative,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct NumericFieldRegistration {
    field: &'static str,
    policy: NumericFieldPolicy,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ContentNegativePolicy {
    Numeric(NumericFieldPolicy),
    StringScan(StringScanPolicy),
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ContentNegativeProof {
    policy: ContentNegativePolicy,
    proving_test: &'static str,
    rejection_code: &'static str,
    rejection_path: &'static str,
}

#[cfg(test)]
const NUMERIC_FIELD_REGISTRY: &[NumericFieldRegistration] = &[
    NumericFieldRegistration {
        field: "NeedModelSchema.awake_hunger_delta_per_tick",
        policy: NumericFieldPolicy::PressureNonnegative,
    },
    NumericFieldRegistration {
        field: "NeedModelSchema.awake_fatigue_delta_per_tick",
        policy: NumericFieldPolicy::PressureNonnegative,
    },
    NumericFieldRegistration {
        field: "InitialNeedSchema.value",
        policy: NumericFieldPolicy::NeedBandU16,
    },
    NumericFieldRegistration {
        field: "SleepPlaceSchema.duration_ticks",
        policy: NumericFieldPolicy::DurationAtLeastOne,
    },
    NumericFieldRegistration {
        field: "SleepPlaceSchema.fatigue_recovery_per_tick",
        policy: NumericFieldPolicy::ReliefPositive,
    },
    NumericFieldRegistration {
        field: "SleepPlaceSchema.hunger_rise_per_tick",
        policy: NumericFieldPolicy::PressureNonnegative,
    },
    NumericFieldRegistration {
        field: "FoodSupplySchema.servings",
        policy: NumericFieldPolicy::CountNonnegative,
    },
    NumericFieldRegistration {
        field: "FoodSupplySchema.hunger_reduction_per_serving",
        policy: NumericFieldPolicy::ReliefPositive,
    },
    NumericFieldRegistration {
        field: "WorkplaceSchema.work_duration_ticks",
        policy: NumericFieldPolicy::DurationAtLeastOne,
    },
    NumericFieldRegistration {
        field: "WorkplaceSchema.fatigue_delta_per_tick",
        policy: NumericFieldPolicy::PressureNonnegative,
    },
    NumericFieldRegistration {
        field: "WorkplaceSchema.hunger_delta_per_tick",
        policy: NumericFieldPolicy::PressureNonnegative,
    },
    NumericFieldRegistration {
        field: "WorkplaceSchema.max_fatigue_to_start",
        policy: NumericFieldPolicy::NeedBandI32,
    },
    NumericFieldRegistration {
        field: "WorkplaceSchema.max_hunger_to_start",
        policy: NumericFieldPolicy::NeedBandI32,
    },
    NumericFieldRegistration {
        field: "RoutineTemplateSchema.min_duration_ticks",
        policy: NumericFieldPolicy::DurationAtLeastOne,
    },
    NumericFieldRegistration {
        field: "RoutineTemplateSchema.max_duration_ticks",
        policy: NumericFieldPolicy::DurationAtLeastOne,
    },
    NumericFieldRegistration {
        field: "RoutineTemplateSchema.interruption_points",
        policy: NumericFieldPolicy::CountNonnegative,
    },
];

#[cfg(test)]
const CONTENT_NEGATIVE_PROOFS: &[ContentNegativeProof] = &[
    ContentNegativeProof {
        policy: ContentNegativePolicy::Numeric(NumericFieldPolicy::PressureNonnegative),
        proving_test: "negative_need_tuning_direction_is_rejected",
        rejection_code: "invalid_tuning_direction",
        rejection_path: "need_model.awake_hunger_delta_per_tick",
    },
    ContentNegativeProof {
        policy: ContentNegativePolicy::Numeric(NumericFieldPolicy::ReliefPositive),
        proving_test: "fixture_relief_zero_and_need_gate_out_of_band_are_rejected_001",
        rejection_code: "invalid_relief_magnitude",
        rejection_path: "food_supplies[0].hunger_reduction_per_serving",
    },
    ContentNegativeProof {
        policy: ContentNegativePolicy::Numeric(NumericFieldPolicy::DurationAtLeastOne),
        proving_test: "fixture_workplace_zero_duration_rejected_001",
        rejection_code: "invalid_duration",
        rejection_path: "workplaces[0].work_duration_ticks",
    },
    ContentNegativeProof {
        policy: ContentNegativePolicy::Numeric(NumericFieldPolicy::NeedBandI32),
        proving_test: "fixture_relief_zero_and_need_gate_out_of_band_are_rejected_001",
        rejection_code: "invalid_need_band",
        rejection_path: "workplaces[0].max_fatigue_to_start",
    },
    ContentNegativeProof {
        policy: ContentNegativePolicy::Numeric(NumericFieldPolicy::NeedBandU16),
        proving_test: "fixture_initial_need_out_of_band_rejected_001",
        rejection_code: "invalid_need_band",
        rejection_path: "initial_needs[0].value",
    },
    ContentNegativeProof {
        policy: ContentNegativePolicy::Numeric(NumericFieldPolicy::CountNonnegative),
        proving_test: "fixture_food_supply_negative_servings_rejected_001",
        rejection_code: "bad_number",
        rejection_path: "fixture.number",
    },
    ContentNegativeProof {
        policy: ContentNegativePolicy::StringScan(StringScanPolicy::Union),
        proving_test: "fixture_output_tag_script_marker_rejected_001",
        rejection_code: "authored_outcome_chain",
        rejection_path: "workplaces[0].output_tag",
    },
];

fn validate_numeric_i32(
    value: i32,
    path: impl Into<String>,
    policy: NumericFieldPolicy,
    errors: &mut Vec<ContentValidationError>,
) {
    let path = path.into();
    match policy {
        NumericFieldPolicy::PressureNonnegative => {
            if value < 0 {
                errors.push(ContentValidationError::new(
                    ValidationPhase::State,
                    path,
                    "invalid_tuning_direction",
                    "need and routine tuning values must be nonnegative in their modeled direction",
                ));
            } else if value > i32::from(NEED_MAX) {
                errors.push(ContentValidationError::new(
                    ValidationPhase::State,
                    path,
                    "invalid_tuning_magnitude",
                    format!(
                        "need and routine tuning values must not exceed {NEED_MAX} per tick or serving"
                    ),
                ));
            }
        }
        NumericFieldPolicy::ReliefPositive => {
            if value <= 0 {
                errors.push(ContentValidationError::new(
                    ValidationPhase::State,
                    path,
                    "invalid_relief_magnitude",
                    "relief-direction tuning values must be greater than zero",
                ));
            } else if value > i32::from(NEED_MAX) {
                errors.push(ContentValidationError::new(
                    ValidationPhase::State,
                    path,
                    "invalid_tuning_magnitude",
                    format!(
                        "need and routine tuning values must not exceed {NEED_MAX} per tick or serving"
                    ),
                ));
            }
        }
        NumericFieldPolicy::NeedBandI32 => {
            if !(0..=i32::from(NEED_MAX)).contains(&value) {
                errors.push(ContentValidationError::new(
                    ValidationPhase::State,
                    path,
                    "invalid_need_band",
                    format!("need-band values must be between 0 and {NEED_MAX}"),
                ));
            }
        }
        NumericFieldPolicy::DurationAtLeastOne
        | NumericFieldPolicy::NeedBandU16
        | NumericFieldPolicy::CountNonnegative => {
            unreachable!("non-i32 numeric policy used for i32 field")
        }
    }
}

fn validate_duration_u64(
    value: u64,
    path: impl Into<String>,
    errors: &mut Vec<ContentValidationError>,
) {
    if value == 0 {
        errors.push(ContentValidationError::new(
            ValidationPhase::State,
            path,
            "invalid_duration",
            "duration values must be greater than zero",
        ));
    }
}

fn validate_need_band_u16(
    value: u16,
    path: impl Into<String>,
    errors: &mut Vec<ContentValidationError>,
) {
    if value > NEED_MAX {
        errors.push(ContentValidationError::new(
            ValidationPhase::State,
            path,
            "invalid_need_band",
            format!("need-band values must be between 0 and {NEED_MAX}"),
        ));
    }
}

fn validate_action_registry_parity(
    fixture: &FixtureSchema,
    registry: &ActionRegistry,
    errors: &mut Vec<ContentValidationError>,
) {
    for (index, affordance) in fixture.affordances.iter().enumerate() {
        let Some(definition) = registry.get(&affordance.action_id) else {
            let (code, message) = if let Some(scope) = known_action_scope(&affordance.action_id) {
                (
                    "phase_unsupported_action",
                    format!(
                        "action {} belongs to {:?} and is outside this fixture scope",
                        affordance.action_id.as_str(),
                        scope
                    ),
                )
            } else {
                (
                    "unknown_action",
                    format!("action {} is not registered", affordance.action_id.as_str()),
                )
            };
            errors.push(ContentValidationError::new(
                ValidationPhase::ActionRegistryParity,
                format!("affordances[{index}].action_id"),
                code,
                message,
            ));
            continue;
        };
        if target_kind(fixture, affordance.target_id.as_str()).is_none() {
            errors.push(ContentValidationError::new(
                ValidationPhase::Referential,
                format!("affordances[{index}].target_id"),
                "bad_reference",
                format!("affordance target {} does not exist", affordance.target_id),
            ));
            continue;
        }
        if !supports_target_kind(definition.effect.clone(), fixture, affordance) {
            errors.push(ContentValidationError::new(
                ValidationPhase::ActionRegistryParity,
                format!("affordances[{index}].target_id"),
                "unsupported_action_target",
                format!(
                    "action {} does not support target {}",
                    affordance.action_id.as_str(),
                    affordance.target_id
                ),
            ));
        }
    }
}

fn validate_routine_rules(
    fixture: &FixtureSchema,
    registry: &ActionRegistry,
    errors: &mut Vec<ContentValidationError>,
) {
    for (template_index, template) in fixture.routine_templates.iter().enumerate() {
        let has_explicit_diagnostic = template.steps.iter().any(|step| {
            matches!(
                step.proposed(),
                RoutineStepProposal::Diagnostic(diagnostic) if !diagnostic.is_empty()
            )
        });
        if template.fallback_rules.is_empty() && !has_explicit_diagnostic {
            errors.push(ContentValidationError::new(
                ValidationPhase::State,
                format!("routine_templates[{template_index}].fallback_rules"),
                "missing_fallback_or_diagnostic",
                "routine templates must declare fallback rules or an explicit diagnostic failure",
            ));
        }

        for (step_index, step) in template.steps.iter().enumerate() {
            let RoutineStepProposal::Action(semantic_action_id) = step.proposed() else {
                continue;
            };
            if contains_direct_state_or_script_marker(semantic_action_id.as_str()) {
                errors.push(ContentValidationError::new(
                    ValidationPhase::NoScript,
                    format!("routine_templates[{template_index}].steps[{step_index}]"),
                    "authored_shortcut_effect",
                    format!(
                        "routine step action {} contains a direct state/script operation",
                        semantic_action_id.as_str()
                    ),
                ));
                continue;
            }
            let Some(action_id) = semantic_action_base(semantic_action_id.as_str()) else {
                errors.push(ContentValidationError::new(
                    ValidationPhase::ActionRegistryParity,
                    format!("routine_templates[{template_index}].steps[{step_index}]"),
                    "unknown_action",
                    format!(
                        "routine step action {} has no stable base action",
                        semantic_action_id.as_str()
                    ),
                ));
                continue;
            };
            if registry.get(&action_id).is_none() {
                let (code, message) = if let Some(scope) = known_action_scope(&action_id) {
                    (
                        "phase_unsupported_action",
                        format!(
                            "routine step action {} maps to {} in {:?}, outside this fixture scope",
                            semantic_action_id.as_str(),
                            action_id.as_str(),
                            scope
                        ),
                    )
                } else {
                    (
                        "unknown_action",
                        format!(
                            "routine step action {} maps to unregistered action {}",
                            semantic_action_id.as_str(),
                            action_id.as_str()
                        ),
                    )
                };
                errors.push(ContentValidationError::new(
                    ValidationPhase::ActionRegistryParity,
                    format!("routine_templates[{template_index}].steps[{step_index}]"),
                    code,
                    message,
                ));
            }
        }

        for (mode_index, failure_mode) in template.failure_modes.iter().enumerate() {
            if !is_known_routine_failure_mode(failure_mode) {
                errors.push(ContentValidationError::new(
                    ValidationPhase::State,
                    format!("routine_templates[{template_index}].failure_modes[{mode_index}]"),
                    "unknown_failure_mode",
                    format!("unknown routine failure mode {failure_mode}"),
                ));
            }
        }
        for (rule_index, fallback_rule) in template.fallback_rules.iter().enumerate() {
            if !is_known_routine_fallback_rule(fallback_rule) {
                errors.push(ContentValidationError::new(
                    ValidationPhase::State,
                    format!("routine_templates[{template_index}].fallback_rules[{rule_index}]"),
                    "unknown_fallback_rule",
                    format!("unknown routine fallback rule {fallback_rule}"),
                ));
            }
        }
    }
}

fn validate_phase3a_sleep_surface_contract(
    fixture: &FixtureSchema,
    errors: &mut Vec<ContentValidationError>,
) {
    if fixture.fixture_scope != FixtureScope::Phase3AHistorical {
        return;
    }
    if !has_sleep_routine_template(fixture) || !fixture.sleep_places.is_empty() {
        return;
    }
    if has_explicit_no_sleep_diagnostic(fixture) {
        return;
    }
    errors.push(ContentValidationError::new(
        ValidationPhase::State,
        "sleep_places",
        "missing_sleep_surface",
        "Phase 3A sleep-routine fixtures must author a sleep surface or an explicit no-sleep diagnostic",
    ));
}

fn has_sleep_routine_template(fixture: &FixtureSchema) -> bool {
    fixture
        .routine_templates
        .iter()
        .any(|template| template.family == RoutineFamily::SleepNight)
}

fn has_explicit_no_sleep_diagnostic(fixture: &FixtureSchema) -> bool {
    fixture.routine_templates.iter().any(|template| {
        template.steps.iter().any(|step| {
            matches!(
                step.proposed(),
                RoutineStepProposal::Diagnostic(diagnostic)
                    if diagnostic.contains("no_sleep")
                        || diagnostic.contains("no_sleep_affordance")
                        || diagnostic.contains("NoSleepAffordance")
            )
        })
    })
}

fn known_action_scope(action_id: &ActionId) -> Option<ActionScope> {
    match action_id.as_str() {
        "move" | "open" | "close" | "take" | "place" | "look" | "inspect_place"
        | "inspect_entity" | "wait" => Some(ActionScope::Phase1),
        "check_container" | "truthful_accuse_probe" => Some(ActionScope::Phase2AHistorical),
        "sleep" | "eat" | "work_block" | "continue_routine" => Some(ActionScope::Phase3AHistorical),
        _ => None,
    }
}

fn semantic_action_base(value: &str) -> Option<ActionId> {
    let base = value.split('.').next().unwrap_or(value);
    ActionId::new(base).ok()
}

fn contains_direct_state_or_script_marker(value: &str) -> bool {
    value
        .split(|character: char| !character.is_ascii_alphanumeric() && character != '_')
        .any(|token| is_script_key(token) || is_phase3a_shortcut_marker(token))
        || PHASE3A_SHORTCUT_MARKERS
            .iter()
            .any(|marker| value.contains(marker))
}

fn contains_prose_born_fact_marker(value: &str) -> bool {
    let normalized = value.to_ascii_lowercase();
    let has_prose_field = normalized.starts_with("note|")
        || normalized.starts_with("notes|")
        || normalized.starts_with("prose|")
        || normalized.starts_with("description|")
        || normalized.starts_with("flavor_text|");
    let implies_simulation_fact = [
        "culprit",
        "true culprit",
        "hidden truth",
        "hidden_true",
        "did it",
        "stole",
        "scripted outcome",
        "outcome",
        "final event",
        "must happen",
        "really happened",
    ]
    .iter()
    .any(|marker| normalized.contains(marker));
    has_prose_field && implies_simulation_fact
}

fn is_known_routine_failure_mode(value: &str) -> bool {
    matches!(
        value,
        "access"
            | "actor_not_at_workplace"
            | "food_inaccessible"
            | "food_missing"
            | "need_blocked"
            | "no_current_intention"
            | "no_known_food_sources"
            | "route_blocked"
            | "search_blocked"
            | "sleep_place_blocked"
            | "step_blocked"
            | "workplace_closed"
    )
}

fn is_known_routine_fallback_rule(value: &str) -> bool {
    matches!(value, "fallback_wait_with_reason" | "wait")
}

fn supports_target_kind(
    effect: ActionEffect,
    fixture: &FixtureSchema,
    affordance: &ActionAffordanceSchema,
) -> bool {
    match effect {
        ActionEffect::Move => {
            parse_place_target(&affordance.target_id).is_some()
                && fixture
                    .places
                    .iter()
                    .any(|place| place.place_id.as_str() == affordance.target_id)
        }
        ActionEffect::Open | ActionEffect::Close => {
            fixture
                .doors
                .iter()
                .any(|door| door.door_id.as_str() == affordance.target_id)
                || fixture
                    .containers
                    .iter()
                    .any(|container| container.container_id.as_str() == affordance.target_id)
        }
        ActionEffect::CheckContainer => fixture
            .containers
            .iter()
            .any(|container| container.container_id.as_str() == affordance.target_id),
        ActionEffect::Eat => fixture
            .food_supplies
            .iter()
            .any(|food| food.food_supply_id.as_str() == affordance.target_id),
        ActionEffect::Sleep => fixture
            .places
            .iter()
            .any(|place| place.place_id.as_str() == affordance.target_id),
        ActionEffect::Work => fixture
            .workplaces
            .iter()
            .any(|workplace| workplace.workplace_id.as_str() == affordance.target_id),
        ActionEffect::ContinueRoutine => fixture
            .routine_templates
            .iter()
            .any(|template| template.template_id.as_str() == affordance.target_id),
        ActionEffect::Take | ActionEffect::Place => fixture
            .items
            .iter()
            .any(|item| item.item_id.as_str() == affordance.target_id),
        ActionEffect::QueryOnly => {
            affordance.action_id.as_str() == "inspect_entity"
                || affordance.action_id.as_str() == "inspect_place"
                || affordance.action_id.as_str() == "look"
                || affordance.action_id.as_str() == "truthful_accuse_probe"
        }
        ActionEffect::Wait => false,
    }
}

fn parse_place_target(value: &str) -> Option<PlaceId> {
    PlaceId::new(value).ok()
}

fn target_kind(fixture: &FixtureSchema, target_id: &str) -> Option<&'static str> {
    if fixture
        .actors
        .iter()
        .any(|actor| actor.actor_id.as_str() == target_id)
    {
        Some("actor")
    } else if fixture
        .places
        .iter()
        .any(|place| place.place_id.as_str() == target_id)
    {
        Some("place")
    } else if fixture
        .doors
        .iter()
        .any(|door| door.door_id.as_str() == target_id)
    {
        Some("door")
    } else if fixture
        .containers
        .iter()
        .any(|container| container.container_id.as_str() == target_id)
    {
        Some("container")
    } else if fixture
        .items
        .iter()
        .any(|item| item.item_id.as_str() == target_id)
    {
        Some("item")
    } else if fixture
        .food_supplies
        .iter()
        .any(|food| food.food_supply_id.as_str() == target_id)
    {
        Some("food_supply")
    } else if fixture
        .workplaces
        .iter()
        .any(|workplace| workplace.workplace_id.as_str() == target_id)
    {
        Some("workplace")
    } else if fixture
        .routine_templates
        .iter()
        .any(|template| template.template_id.as_str() == target_id)
    {
        Some("routine_template")
    } else {
        None
    }
}

fn validate_semantic_ids(fixture: &FixtureSchema, errors: &mut Vec<ContentValidationError>) {
    for (index, affordance) in fixture.affordances.iter().enumerate() {
        if affordance.action_id.as_str().parse::<u64>().is_ok() {
            errors.push(ContentValidationError::new(
                ValidationPhase::SemanticView,
                format!("affordances[{index}].action_id"),
                "menu_position_action_id",
                "fixture action IDs must be semantic IDs, not menu positions",
            ));
        }
    }
}

fn validate_no_player(fixture: &FixtureSchema, errors: &mut Vec<ContentValidationError>) {
    for (index, affordance) in fixture.affordances.iter().enumerate() {
        if is_player_key(affordance.action_id.as_str()) {
            errors.push(ContentValidationError::new(
                ValidationPhase::NoPlayer,
                format!("affordances[{index}].action_id"),
                "player_only_verb",
                format!(
                    "player-only action {} is forbidden",
                    affordance.action_id.as_str()
                ),
            ));
        }
    }
}

fn validate_no_script(fixture: &FixtureSchema, errors: &mut Vec<ContentValidationError>) {
    for (index, place) in fixture.places.iter().enumerate() {
        reject_text_by_policy(
            &place.display_label,
            format!("places[{index}].display_label"),
            StringScanPolicy::Union,
            errors,
        );
    }
    for (index, affordance) in fixture.affordances.iter().enumerate() {
        if is_script_key(affordance.action_id.as_str()) {
            errors.push(ContentValidationError::new(
                ValidationPhase::NoScript,
                format!("affordances[{index}].action_id"),
                "authored_outcome_chain",
                format!(
                    "script action {} is forbidden",
                    affordance.action_id.as_str()
                ),
            ));
        }
        reject_text_by_policy(
            &affordance.target_id,
            format!("affordances[{index}].target_id"),
            StringScanPolicy::Union,
            errors,
        );
    }
    for (index, belief) in fixture.initial_beliefs.iter().enumerate() {
        let planner_intended = planner_intended_seed_text(belief).any(contains_planner_seed_marker);
        if planner_intended && belief.channel.is_none() {
            errors.push(ContentValidationError::new(
                ValidationPhase::NoScript,
                format!("initial_beliefs[{index}]"),
                "missing_actor_known_provenance",
                "planner-intended actor-known initial beliefs must declare an observation channel",
            ));
        }
    }
    for (template_index, template) in fixture.routine_templates.iter().enumerate() {
        for (index, value) in template.failure_modes.iter().enumerate() {
            reject_text_by_policy(
                value,
                format!("routine_templates[{template_index}].failure_modes[{index}]"),
                StringScanPolicy::Union,
                errors,
            );
        }
        for (index, value) in template.debug_labels.iter().enumerate() {
            reject_text_by_policy(
                value,
                format!("routine_templates[{template_index}].debug_labels[{index}]"),
                StringScanPolicy::Union,
                errors,
            );
        }
        if let Some(value) = &template.reservable_resource {
            reject_text_by_policy(
                value,
                format!("routine_templates[{template_index}].reservable_resource"),
                StringScanPolicy::Union,
                errors,
            );
        }
        for (index, step) in template.steps.iter().enumerate() {
            match step.proposed() {
                RoutineStepProposal::Action(action_id) => reject_text_by_policy(
                    action_id.as_str(),
                    format!("routine_templates[{template_index}].steps[{index}]"),
                    StringScanPolicy::Union,
                    errors,
                ),
                RoutineStepProposal::Diagnostic(diagnostic) => reject_text_by_policy(
                    diagnostic,
                    format!("routine_templates[{template_index}].steps[{index}]"),
                    StringScanPolicy::Union,
                    errors,
                ),
                RoutineStepProposal::Wait(reason) => reject_text_by_policy(
                    reason,
                    format!("routine_templates[{template_index}].steps[{index}]"),
                    StringScanPolicy::Union,
                    errors,
                ),
            }
        }
        for (index, fallback) in template.fallback_rules.iter().enumerate() {
            reject_text_by_policy(
                fallback,
                format!("routine_templates[{template_index}].fallback_rules[{index}]"),
                StringScanPolicy::Union,
                errors,
            );
        }
    }
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct StringFieldScanRegistration {
    field: &'static str,
    policy: StringScanPolicy,
    rationale: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum StringScanPolicy {
    Union,
}

#[cfg(test)]
const SCANNED_STRING_FIELDS: &[StringFieldScanRegistration] = &[
    StringFieldScanRegistration {
        field: "PlaceSchema.display_label",
        policy: StringScanPolicy::Union,
        rationale: "display text can carry authored-outcome or shortcut markers",
    },
    StringFieldScanRegistration {
        field: "ActionAffordanceSchema.target_id",
        policy: StringScanPolicy::Union,
        rationale: "free-form target text can smuggle script or shortcut markers",
    },
    StringFieldScanRegistration {
        field: "WorkplaceSchema.output_tag",
        policy: StringScanPolicy::Union,
        rationale: "output tags are free-form fixture text and must not encode outcomes",
    },
    StringFieldScanRegistration {
        field: "RoutineTemplateSchema.failure_modes",
        policy: StringScanPolicy::Union,
        rationale: "routine labels can otherwise become authored outcomes",
    },
    StringFieldScanRegistration {
        field: "RoutineTemplateSchema.fallback_rules",
        policy: StringScanPolicy::Union,
        rationale: "fallback labels can otherwise become direct behavior scripts",
    },
    StringFieldScanRegistration {
        field: "RoutineTemplateSchema.debug_labels",
        policy: StringScanPolicy::Union,
        rationale: "debug labels must not smuggle acceptance or shortcut markers",
    },
    StringFieldScanRegistration {
        field: "RoutineTemplateSchema.reservable_resource",
        policy: StringScanPolicy::Union,
        rationale: "reservable-resource text participates in routine policy",
    },
];

fn planner_intended_seed_text(
    belief: &crate::schema::InitialBeliefSchema,
) -> impl Iterator<Item = String> + '_ {
    [
        belief.belief_id.as_str().to_string(),
        belief.proposition.serialize_canonical(),
        match &belief.source {
            SourceRef::Event(event_id) => event_id.as_str().to_string(),
            SourceRef::Action(action_id) => action_id.as_str().to_string(),
            SourceRef::Cause(cause) => format!("{cause:?}"),
        },
    ]
    .into_iter()
}

fn contains_planner_seed_marker(value: String) -> bool {
    let markers = [
        "actor_known",
        "planner_visible",
        "known_food",
        "known_route",
        "known_workplace",
        "food_source",
        "route_edge",
        "workplace_access",
    ];
    markers.iter().any(|marker| value.contains(marker))
}

fn reject_text_by_policy(
    value: &str,
    path: String,
    policy: StringScanPolicy,
    errors: &mut Vec<ContentValidationError>,
) {
    match policy {
        StringScanPolicy::Union => {
            reject_script_marker_text(value, path.clone(), errors);
            reject_shortcut_text(value, path, errors);
        }
    }
}

fn reject_script_marker_text(value: &str, path: String, errors: &mut Vec<ContentValidationError>) {
    if contains_authored_outcome_marker(value) {
        errors.push(ContentValidationError::new(
            ValidationPhase::NoScript,
            path,
            "authored_outcome_chain",
            format!("authored outcome-chain marker {value} is forbidden"),
        ));
    }
}

fn contains_authored_outcome_marker(value: &str) -> bool {
    let markers = [
        "guaranteed_success",
        "always_succeeds",
        "success_without_action",
        "without_pipeline",
        "synthetic_action_chain",
        "acceptance_marker",
        "debug_acceptance",
        "expected_final_event",
        "player_conditioned",
        "protagonist_conditioned",
        "player_only",
        "protagonist_only",
    ];
    markers.iter().any(|marker| value.contains(marker))
        || value
            .split(|character: char| !character.is_ascii_alphanumeric() && character != '_')
            .any(|token| is_script_key(token) || is_player_key(token))
}

fn validate_phase3a_no_shortcuts(
    fixture: &FixtureSchema,
    errors: &mut Vec<ContentValidationError>,
) {
    for (index, sleep_place) in fixture.sleep_places.iter().enumerate() {
        reject_shortcut_text(
            sleep_place.sleep_place_id.as_str(),
            format!("sleep_places[{index}].sleep_place_id"),
            errors,
        );
    }
    for (index, workplace) in fixture.workplaces.iter().enumerate() {
        reject_text_by_policy(
            &workplace.output_tag,
            format!("workplaces[{index}].output_tag"),
            StringScanPolicy::Union,
            errors,
        );
    }
    for (template_index, template) in fixture.routine_templates.iter().enumerate() {
        for (index, value) in template.applicability_conditions.iter().enumerate() {
            reject_shortcut_text(
                value.stable_id(),
                format!("routine_templates[{template_index}].applicability_conditions[{index}]"),
                errors,
            );
        }
        for (index, value) in template.preconditions.iter().enumerate() {
            reject_shortcut_text(
                value.stable_id(),
                format!("routine_templates[{template_index}].preconditions[{index}]"),
                errors,
            );
        }
        for (index, value) in template.failure_modes.iter().enumerate() {
            reject_shortcut_text(
                value,
                format!("routine_templates[{template_index}].failure_modes[{index}]"),
                errors,
            );
        }
        for (index, value) in template.fallback_rules.iter().enumerate() {
            reject_shortcut_text(
                value,
                format!("routine_templates[{template_index}].fallback_rules[{index}]"),
                errors,
            );
        }
        for (index, value) in template.debug_labels.iter().enumerate() {
            reject_shortcut_text(
                value,
                format!("routine_templates[{template_index}].debug_labels[{index}]"),
                errors,
            );
        }
        if let Some(value) = &template.reservable_resource {
            reject_shortcut_text(
                value,
                format!("routine_templates[{template_index}].reservable_resource"),
                errors,
            );
        }
    }
}

fn reject_shortcut_text(value: &str, path: String, errors: &mut Vec<ContentValidationError>) {
    if value
        .split(|character: char| !character.is_ascii_alphanumeric() && character != '_')
        .any(is_phase3a_shortcut_marker)
    {
        errors.push(ContentValidationError::new(
            ValidationPhase::NoScript,
            path,
            "authored_shortcut_effect",
            format!("Phase 3A shortcut marker {value} is forbidden"),
        ));
    }
}

fn validate_epistemic_seeds(
    fixture: &FixtureSchema,
    registry: &ActionRegistry,
    errors: &mut Vec<ContentValidationError>,
) {
    for (index, belief) in fixture.initial_beliefs.iter().enumerate() {
        if belief.schema_version.as_str() != EPISTEMIC_RECORD_SCHEMA_V1 {
            errors.push(ContentValidationError::new(
                ValidationPhase::EpistemicSeed,
                format!("initial_beliefs[{index}].schema_version"),
                "unsupported_epistemic_schema_version",
                format!(
                    "unsupported epistemic schema version {}",
                    belief.schema_version.as_str()
                ),
            ));
        }
        if !(0..=1000).contains(&belief.confidence.parts_per_thousand()) {
            errors.push(ContentValidationError::new(
                ValidationPhase::EpistemicSeed,
                format!("initial_beliefs[{index}].confidence"),
                "bad_confidence",
                "belief confidence must be 0..=1000",
            ));
        }
        match &belief.privacy_scope {
            PrivacyScope::ActorPrivate(actor_id) if actor_id == &belief.holder_actor_id => {}
            PrivacyScope::ActorPrivate(actor_id) => errors.push(ContentValidationError::new(
                ValidationPhase::EpistemicSeed,
                format!("initial_beliefs[{index}].privacy_scope"),
                "cross_actor_seed_scope",
                format!(
                    "belief holder {} must not receive actor-private scope for {}",
                    belief.holder_actor_id.as_str(),
                    actor_id.as_str()
                ),
            )),
            PrivacyScope::PublicPlaceholder | PrivacyScope::InstitutionPlaceholder(_) => {
                errors.push(ContentValidationError::new(
                    ValidationPhase::EpistemicSeed,
                    format!("initial_beliefs[{index}].privacy_scope"),
                    "unsupported_seed_scope",
                    "initial belief seeds must be actor-private in Phase 2A",
                ));
            }
        }
        if let Some(last_verified_tick) = belief.last_verified_tick {
            if last_verified_tick < belief.acquired_tick {
                errors.push(ContentValidationError::new(
                    ValidationPhase::EpistemicSeed,
                    format!("initial_beliefs[{index}].last_verified_tick"),
                    "bad_tick_order",
                    "last_verified_tick must not precede acquired_tick",
                ));
            }
        }
        match &belief.source {
            SourceRef::Event(event_id)
                if belief.source_kind == InitialBeliefSourceKind::AuthoredPrehistory =>
            {
                if is_forbidden_key(event_id.as_str())
                    || contains_direct_state_or_script_marker(event_id.as_str())
                {
                    errors.push(ContentValidationError::new(
                        ValidationPhase::NoScript,
                        format!("initial_beliefs[{index}].source_id"),
                        "shortcut_truth_field",
                        format!("forbidden shortcut-truth source {}", event_id.as_str()),
                    ));
                }
            }
            SourceRef::Event(_) | SourceRef::Action(_) | SourceRef::Cause(_) => {
                errors.push(ContentValidationError::new(
                    ValidationPhase::EpistemicSeed,
                    format!("initial_beliefs[{index}].source_kind"),
                    "unsupported_source_kind",
                    "initial belief seeds must use authored_prehistory event source references",
                ))
            }
        }
        if let SourceRef::Action(action_id) = &belief.source {
            if registry.get(action_id).is_none() {
                errors.push(ContentValidationError::new(
                    ValidationPhase::EpistemicSeed,
                    format!("initial_beliefs[{index}].source_id"),
                    "unknown_action_source",
                    format!(
                        "belief source action {} is not registered",
                        action_id.as_str()
                    ),
                ));
            }
        }
        if matches!(&belief.source, SourceRef::Cause(_)) {
            errors.push(ContentValidationError::new(
                ValidationPhase::EpistemicSeed,
                format!("initial_beliefs[{index}].source_kind"),
                "unsupported_source_kind",
                "initial belief seeds must use authored_prehistory event source references",
            ));
        }

        let _belief = belief.to_belief();
    }
}

fn validate_determinism(fixture: &FixtureSchema, errors: &mut Vec<ContentValidationError>) {
    for (index, place) in fixture.places.iter().enumerate() {
        if !is_sorted_unique(&place.adjacent_place_ids) {
            errors.push(ContentValidationError::new(
                ValidationPhase::DeterminismHazard,
                format!("places[{index}].adjacent_place_ids"),
                "non_canonical_ordering",
                "adjacent place IDs must be canonical sorted without duplicates",
            ));
        }
    }
    for (index, container) in fixture.containers.iter().enumerate() {
        if !is_sorted_unique(&container.contents) {
            errors.push(ContentValidationError::new(
                ValidationPhase::DeterminismHazard,
                format!("containers[{index}].contents"),
                "non_canonical_ordering",
                "container contents must be canonical sorted without duplicates",
            ));
        }
    }
    if !fixture
        .initial_beliefs
        .windows(2)
        .all(|window| window[0].belief_id < window[1].belief_id)
    {
        errors.push(ContentValidationError::new(
            ValidationPhase::DeterminismHazard,
            "initial_beliefs",
            "non_canonical_ordering",
            "initial belief seeds must be canonical sorted without duplicates",
        ));
    }
}

fn is_sorted_unique<T: Ord>(values: &[T]) -> bool {
    values.windows(2).all(|window| window[0] < window[1])
}

fn validate_fixture_contract(fixture: &FixtureSchema, errors: &mut Vec<ContentValidationError>) {
    if fixture.actors.is_empty() || fixture.places.is_empty() {
        errors.push(ContentValidationError::new(
            ValidationPhase::FixtureContract,
            "fixture",
            "missing_golden_assertions",
            "Phase 1 fixtures must declare at least one actor and one place",
        ));
    }
}

fn validate_serialization_roundtrip(
    fixture: &FixtureSchema,
    errors: &mut Vec<ContentValidationError>,
) {
    let bytes = serialize_fixture(fixture);
    match deserialize_fixture(&bytes) {
        Ok(roundtrip)
            if roundtrip == {
                let mut expected = fixture.clone();
                expected.canonicalize();
                expected
            } => {}
        Ok(_) => errors.push(ContentValidationError::new(
            ValidationPhase::Serialization,
            "fixture",
            "serialization_drift",
            "fixture serialization changed canonical content",
        )),
        Err(error) => errors.push(ContentValidationError::new(
            ValidationPhase::Serialization,
            "fixture",
            "serialization_error",
            format!("fixture serialization failed: {error:?}"),
        )),
    }
}

fn missing(
    errors: &mut Vec<ContentValidationError>,
    phase: ValidationPhase,
    path: String,
    value: &str,
    expected: &str,
) {
    errors.push(ContentValidationError::new(
        phase,
        path,
        "bad_reference",
        format!("missing {expected} reference {value}"),
    ));
}

fn serialization_error(error: SerializationError) -> ContentValidationError {
    let (path, code, message) = match error {
        SerializationError::MissingField(field) => (
            format!("fixture.{field}"),
            "missing_field",
            format!("missing required field {field}"),
        ),
        SerializationError::BadLine(line) => (
            "fixture.line".to_string(),
            "bad_line",
            format!("bad fixture line {line}"),
        ),
        SerializationError::BadBool(value) => (
            "fixture.bool".to_string(),
            "bad_bool",
            format!("bad bool value {value}"),
        ),
        SerializationError::BadU64(value) => (
            "fixture.number".to_string(),
            "bad_number",
            format!("bad numeric value {value}"),
        ),
        SerializationError::Id(error) => (
            "fixture.id".to_string(),
            "bad_stable_id",
            format!("{error}"),
        ),
        SerializationError::EventLog(error) => (
            "fixture.event_log".to_string(),
            "event_log_error",
            format!("{error:?}"),
        ),
        SerializationError::Proposition(error) => (
            "fixture.initial_belief.proposition".to_string(),
            "bad_proposition",
            format!("{error}"),
        ),
        SerializationError::Confidence(error) => (
            "fixture.initial_belief.confidence".to_string(),
            "bad_confidence",
            format!("{error:?}"),
        ),
    };
    ContentValidationError::new(ValidationPhase::ParseSchema, path, code, message)
}

fn is_forbidden_key(value: &str) -> bool {
    is_player_key(value) || is_script_key(value)
}

fn is_player_key(value: &str) -> bool {
    matches!(
        value,
        "player"
            | "player_character"
            | "player_only"
            | "protagonist"
            | "quest"
            | "objective"
            | "reward"
    )
}

fn is_script_key(value: &str) -> bool {
    matches!(
        value,
        "on_enter"
            | "on_open"
            | "on_tick"
            | "force_event"
            | "complete_objective"
            | "appear_at"
            | "force_location_at_tick"
            | "scripted_absence"
            | "teleport_actor"
            | "move_item_to"
            | "set_need"
            | "set_hunger"
            | "set_fatigue"
            | "hunger_refill_without_food"
            | "instant_sleep_refill"
            | "work_always_succeeds"
            | "hidden_true_item_location"
            | "hidden_planner_input"
            | "actor_known_hidden_input"
            | "actor_knows_hidden_food"
            | "story_beat"
            | "final_event"
            | "expected_final_event"
            | "assert_final_event"
            | "scripted_outcome"
            | "golden_event_log"
            | "director"
            | "culprit"
            | "true_culprit"
            | "stolen_flag"
            | "npc_knows_truth"
            | "knows_mara_did_it"
            | "truth_alias"
            | "nested_culprit_hint"
            | "renamed_stolen_state"
            | "quest_state"
            | "player_memory"
            | "branch"
            | "selector"
            | "llm_prompt"
    )
}

const PHASE3A_SHORTCUT_MARKERS: &[&str] = &[
    "appear_at",
    "force_location_at_tick",
    "scripted_absence",
    "teleport_actor",
    "move_item_to",
    "set_need",
    "set_hunger",
    "set_fatigue",
    "hunger_refill_without_food",
    "instant_sleep_refill",
    "work_always_succeeds",
    "hidden_true_item_location",
];

fn is_phase3a_shortcut_marker(value: &str) -> bool {
    PHASE3A_SHORTCUT_MARKERS.contains(&value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{
        ActionAffordanceSchema, ActorSchema, ContainerSchema, DoorSchema, FixtureScope,
        FoodSupplySchema, InitialNeedSchema, ItemSchema, NeedModelSchema, PlaceSchema,
        RoutineAssignmentSchema, RoutineTemplateSchema, SleepPlaceSchema, WorkplaceSchema,
    };
    use tracewake_core::agent::{NeedKind, RoutineCondition, RoutineFamily, RoutineStep};
    use tracewake_core::ids::{
        ActionId, ActorId, ContainerId, DoorId, FixtureId, FoodSupplyId, ItemId, PlaceId,
        RoutineTemplateId, SchemaVersion, SemanticActionId, SleepAffordanceId, WorkplaceId,
    };
    use tracewake_core::time::SimTick;

    fn registry() -> ActionRegistry {
        let mut registry = ActionRegistry::new();
        registry.register_phase1_movement_open_close();
        registry.register_phase1_take_place();
        registry.register_phase1_inspect_wait();
        registry.register_phase2a_epistemics();
        registry.register_phase3a_sleep();
        registry.register_phase3a_eat();
        registry.register_phase3a_work();
        registry.register_phase3a_continue_routine();
        registry
    }

    fn fixture() -> FixtureSchema {
        FixtureSchema {
            fixture_id: FixtureId::new("strongbox_001").unwrap(),
            schema_version: SchemaVersion::new("schema_v1").unwrap(),
            fixture_scope: FixtureScope::Phase1,
            need_model: NeedModelSchema {
                awake_hunger_delta_per_tick: 5,
                awake_fatigue_delta_per_tick: 3,
            },
            actors: vec![ActorSchema {
                actor_id: ActorId::new("actor_tomas").unwrap(),
                current_place_id: PlaceId::new("shop_front").unwrap(),
            }],
            places: vec![
                PlaceSchema {
                    place_id: PlaceId::new("back_room").unwrap(),
                    display_label: "Back room".to_string(),
                    adjacent_place_ids: vec![PlaceId::new("shop_front").unwrap()],
                },
                PlaceSchema {
                    place_id: PlaceId::new("shop_front").unwrap(),
                    display_label: "Shop front".to_string(),
                    adjacent_place_ids: vec![PlaceId::new("back_room").unwrap()],
                },
            ],
            doors: vec![DoorSchema {
                door_id: DoorId::new("door_shop_back").unwrap(),
                endpoint_a: PlaceId::new("shop_front").unwrap(),
                endpoint_b: PlaceId::new("back_room").unwrap(),
                is_open: false,
                is_locked: false,
            }],
            containers: vec![ContainerSchema {
                container_id: ContainerId::new("strongbox_tomas").unwrap(),
                place_id: PlaceId::new("shop_front").unwrap(),
                is_open: false,
                is_locked: false,
                contents: vec![ItemId::new("coin_stack_01").unwrap()],
                contents_visible_when_closed: false,
            }],
            items: vec![ItemSchema {
                item_id: ItemId::new("coin_stack_01").unwrap(),
                portable: true,
                location: Location::InContainer(ContainerId::new("strongbox_tomas").unwrap()),
            }],
            affordances: vec![
                ActionAffordanceSchema {
                    action_id: ActionId::new("open").unwrap(),
                    target_id: "strongbox_tomas".to_string(),
                },
                ActionAffordanceSchema {
                    action_id: ActionId::new("move").unwrap(),
                    target_id: "back_room".to_string(),
                },
            ],
            initial_beliefs: Vec::new(),
            initial_needs: Vec::new(),
            homes: Vec::new(),
            sleep_places: Vec::new(),
            food_supplies: Vec::new(),
            known_food_sources: Vec::new(),
            workplaces: Vec::new(),
            routine_templates: Vec::new(),
            routine_assignments: Vec::new(),
            day_windows: Vec::new(),
        }
    }

    fn phase3a_fixture() -> FixtureSchema {
        let mut fixture = fixture();
        fixture.fixture_scope = FixtureScope::Phase3AHistorical;
        fixture.places.push(PlaceSchema {
            place_id: PlaceId::new("workshop").unwrap(),
            display_label: "Workshop".to_string(),
            adjacent_place_ids: vec![PlaceId::new("shop_front").unwrap()],
        });
        fixture.initial_needs.push(InitialNeedSchema {
            actor_id: ActorId::new("actor_tomas").unwrap(),
            kind: NeedKind::Hunger,
            value: 350,
        });
        fixture.sleep_places.push(SleepPlaceSchema {
            actor_id: ActorId::new("actor_tomas").unwrap(),
            place_id: PlaceId::new("shop_front").unwrap(),
            sleep_place_id: SleepAffordanceId::new("bed_tomas").unwrap(),
            access_open: true,
            duration_ticks: 4,
            fatigue_recovery_per_tick: 20,
            hunger_rise_per_tick: 2,
        });
        fixture.food_supplies.push(FoodSupplySchema {
            food_supply_id: FoodSupplyId::new("food_soup_pot").unwrap(),
            location: Location::AtPlace(PlaceId::new("shop_front").unwrap()),
            servings: 2,
            hunger_reduction_per_serving: 100,
        });
        fixture.workplaces.push(WorkplaceSchema {
            workplace_id: WorkplaceId::new("workplace_shop").unwrap(),
            place_id: PlaceId::new("workshop").unwrap(),
            assigned_actor_ids: vec![ActorId::new("actor_tomas").unwrap()],
            work_duration_ticks: 4,
            fatigue_delta_per_tick: 8,
            hunger_delta_per_tick: 4,
            max_fatigue_to_start: 800,
            max_hunger_to_start: 850,
            access_open: true,
            role_notice_access_open: true,
            output_tag: "service_completed_placeholder".to_string(),
        });
        fixture.routine_templates.push(valid_routine_template());
        fixture.routine_assignments.push(RoutineAssignmentSchema {
            actor_id: ActorId::new("actor_tomas").unwrap(),
            template_id: RoutineTemplateId::new("routine_work_shift").unwrap(),
            start_tick: SimTick::new(10),
            end_tick: SimTick::new(20),
        });
        fixture.canonicalize();
        fixture
    }

    fn valid_routine_template() -> RoutineTemplateSchema {
        RoutineTemplateSchema {
            template_id: RoutineTemplateId::new("routine_work_shift").unwrap(),
            family: RoutineFamily::WorkBlock,
            applicability_conditions: vec![RoutineCondition::AssignedWorkplaceKnown],
            preconditions: vec![RoutineCondition::AtWorkplace],
            steps: vec![RoutineStep::StartWorkBlock {
                action_id: SemanticActionId::new("work_block.workplace_shop").unwrap(),
            }],
            min_duration_ticks: 4,
            max_duration_ticks: 6,
            interruption_points: vec![0],
            failure_modes: vec!["access".to_string()],
            fallback_rules: vec!["wait".to_string()],
            debug_labels: vec!["phase3a_schema_sample".to_string()],
            reservable_resource: Some("body".to_string()),
        }
    }

    #[test]
    fn valid_fixture_produces_initial_world() {
        let world = validate_fixture(&fixture(), &registry()).unwrap();
        assert!(world
            .physical_state
            .places()
            .contains_key(&PlaceId::new("shop_front").unwrap()));
    }

    #[test]
    fn negative_need_tuning_direction_is_rejected() {
        let mut fixture = fixture();
        fixture.need_model.awake_hunger_delta_per_tick = -50;

        let errors = validate_fixture(&fixture, &registry()).unwrap_err();

        assert!(errors.report.errors.iter().any(|error| {
            error.code == "invalid_tuning_direction"
                && error.path == "need_model.awake_hunger_delta_per_tick"
        }));
    }

    #[test]
    fn excessive_need_tuning_magnitude_is_rejected_against_need_max() {
        let mut fixture = phase3a_fixture();
        fixture.need_model.awake_hunger_delta_per_tick = i32::from(NEED_MAX) + 1;

        let errors = validate_fixture(&fixture, &registry()).unwrap_err();

        assert!(errors.report.errors.iter().any(|error| {
            error.code == "invalid_tuning_magnitude"
                && error.path == "need_model.awake_hunger_delta_per_tick"
        }));
    }

    #[test]
    fn fixture_workplace_zero_duration_rejected_001() {
        let mut fixture = phase3a_fixture();
        fixture.workplaces[0].work_duration_ticks = 0;

        let errors = validate_fixture(&fixture, &registry()).unwrap_err();

        assert!(errors.report.errors.iter().any(|error| {
            error.code == "invalid_duration" && error.path == "workplaces[0].work_duration_ticks"
        }));
    }

    #[test]
    fn fixture_initial_need_out_of_band_rejected_001() {
        let mut fixture = phase3a_fixture();
        fixture.initial_needs[0].value = NEED_MAX + 1;

        let errors = validate_fixture(&fixture, &registry()).unwrap_err();

        assert!(errors.report.errors.iter().any(|error| {
            error.code == "invalid_need_band" && error.path == "initial_needs[0].value"
        }));
    }

    #[test]
    fn fixture_food_supply_negative_servings_rejected_001() {
        let raw = b"fixture|phase3a_bad_servings_001\nschema|schema_v1\nfixture_scope|phase3a_historical\nneed_model|5|3\nactor|actor_tomas|home_tomas\nplace|home_tomas|546f6d617320686f6d65|\nfood_supply|food_soup_pot|at:home_tomas|-1|180";

        let errors = validate_fixture_bytes(raw, &registry()).unwrap_err();

        assert!(errors
            .report
            .errors
            .iter()
            .any(|error| { error.code == "bad_number" && error.path == "fixture.number" }));
    }

    #[test]
    fn fixture_relief_zero_and_need_gate_out_of_band_are_rejected_001() {
        let mut fixture = phase3a_fixture();
        fixture.food_supplies[0].hunger_reduction_per_serving = 0;
        fixture.sleep_places[0].fatigue_recovery_per_tick = 0;
        fixture.workplaces[0].max_fatigue_to_start = -1;
        fixture.workplaces[0].max_hunger_to_start = i32::from(NEED_MAX) + 1;

        let report = validate_fixture(&fixture, &registry()).unwrap_err().report;

        for (code, path) in [
            (
                "invalid_relief_magnitude",
                "food_supplies[0].hunger_reduction_per_serving",
            ),
            (
                "invalid_relief_magnitude",
                "sleep_places[0].fatigue_recovery_per_tick",
            ),
            ("invalid_need_band", "workplaces[0].max_fatigue_to_start"),
            ("invalid_need_band", "workplaces[0].max_hunger_to_start"),
        ] {
            assert!(
                report
                    .errors
                    .iter()
                    .any(|error| error.code == code && error.path == path),
                "missing {code} at {path}: {report:?}"
            );
        }
    }

    #[test]
    fn display_label_script_marker_is_rejected() {
        let mut fixture = fixture();
        fixture.places[0].display_label = "expected_final_event hidden truth".to_string();

        let errors = validate_fixture(&fixture, &registry()).unwrap_err();

        assert!(errors.report.errors.iter().any(|error| {
            error.code == "authored_outcome_chain" && error.path == "places[0].display_label"
        }));
    }

    #[test]
    fn fixture_output_tag_script_marker_rejected_001() {
        let mut fixture = phase3a_fixture();
        fixture.workplaces[0].output_tag = "expected_final_event".to_string();

        let errors = validate_fixture(&fixture, &registry()).unwrap_err();

        assert!(errors.report.errors.iter().any(|error| {
            error.code == "authored_outcome_chain" && error.path == "workplaces[0].output_tag"
        }));
    }

    #[test]
    fn content_negative_registry_covers_validation_policy_variants_and_tests() {
        let expected_numeric_policies = BTreeSet::from([
            NumericFieldPolicy::PressureNonnegative,
            NumericFieldPolicy::ReliefPositive,
            NumericFieldPolicy::DurationAtLeastOne,
            NumericFieldPolicy::NeedBandI32,
            NumericFieldPolicy::NeedBandU16,
            NumericFieldPolicy::CountNonnegative,
        ]);
        let expected_string_policies = BTreeSet::from([StringScanPolicy::Union]);
        let actual_numeric_policies = CONTENT_NEGATIVE_PROOFS
            .iter()
            .filter_map(|proof| match proof.policy {
                ContentNegativePolicy::Numeric(policy) => Some(policy),
                ContentNegativePolicy::StringScan(_) => None,
            })
            .collect::<BTreeSet<_>>();
        let actual_string_policies = CONTENT_NEGATIVE_PROOFS
            .iter()
            .filter_map(|proof| match proof.policy {
                ContentNegativePolicy::Numeric(_) => None,
                ContentNegativePolicy::StringScan(policy) => Some(policy),
            })
            .collect::<BTreeSet<_>>();

        assert_eq!(actual_numeric_policies, expected_numeric_policies);
        assert_eq!(actual_string_policies, expected_string_policies);
        for proof in CONTENT_NEGATIVE_PROOFS {
            assert!(
                proof.proving_test.ends_with("_rejected_001")
                    || proof.proving_test == "negative_need_tuning_direction_is_rejected",
                "{proof:?} must point to a named negative rejection test"
            );
            assert!(!proof.rejection_code.is_empty(), "{proof:?}");
            assert!(!proof.rejection_path.is_empty(), "{proof:?}");
        }
        assert!(CONTENT_NEGATIVE_PROOFS.iter().any(|proof| {
            proof.proving_test == "fixture_workplace_zero_duration_rejected_001"
                && proof.policy
                    == ContentNegativePolicy::Numeric(NumericFieldPolicy::DurationAtLeastOne)
        }));
        assert!(CONTENT_NEGATIVE_PROOFS.iter().any(|proof| {
            proof.proving_test == "fixture_initial_need_out_of_band_rejected_001"
                && proof.policy == ContentNegativePolicy::Numeric(NumericFieldPolicy::NeedBandU16)
        }));
        assert!(CONTENT_NEGATIVE_PROOFS.iter().any(|proof| {
            proof.proving_test == "fixture_output_tag_script_marker_rejected_001"
                && proof.policy == ContentNegativePolicy::StringScan(StringScanPolicy::Union)
        }));
    }

    #[test]
    fn schema_numeric_fields_are_classified_for_validation_policy() {
        let schema = include_str!("schema.rs");
        let discovered = discover_schema_fields(schema, |field_type| {
            matches!(field_type, "i32" | "u16" | "u32" | "u64" | "Vec<usize>")
        });

        let registered = NUMERIC_FIELD_REGISTRY
            .iter()
            .map(|registration| registration.field.to_string())
            .collect::<BTreeSet<_>>();
        assert_eq!(discovered, registered);
        assert!(NUMERIC_FIELD_REGISTRY.iter().any(|registration| {
            registration.field == "InitialNeedSchema.value"
                && registration.policy == NumericFieldPolicy::NeedBandU16
        }));
        assert!(NUMERIC_FIELD_REGISTRY.iter().any(|registration| {
            registration.field == "FoodSupplySchema.hunger_reduction_per_serving"
                && registration.policy == NumericFieldPolicy::ReliefPositive
        }));
    }

    #[test]
    fn schema_string_fields_are_classified_for_script_scanning() {
        let schema = include_str!("schema.rs");
        let discovered = discover_schema_fields(schema, |field_type| {
            matches!(field_type, "String" | "Option<String>" | "Vec<String>")
        });

        let registered = SCANNED_STRING_FIELDS
            .iter()
            .map(|registration| registration.field.to_string())
            .collect::<BTreeSet<_>>();
        assert_eq!(discovered, registered);
        for registration in SCANNED_STRING_FIELDS {
            assert_eq!(registration.policy, StringScanPolicy::Union);
            assert!(
                !registration.rationale.is_empty(),
                "{} needs a narrower-policy rationale or union policy rationale",
                registration.field
            );
        }
    }

    fn discover_schema_fields(
        schema: &str,
        include_type: impl Fn(&str) -> bool,
    ) -> BTreeSet<String> {
        let mut current_struct = None;
        let mut discovered = BTreeSet::new();
        for line in schema.lines() {
            let trimmed = line.trim();
            if let Some(rest) = trimmed.strip_prefix("pub struct ") {
                current_struct = rest
                    .split_whitespace()
                    .next()
                    .map(|name| name.trim_end_matches('{').to_string());
                continue;
            }
            let Some(struct_name) = current_struct.as_ref() else {
                continue;
            };
            if trimmed == "}" {
                current_struct = None;
                continue;
            }
            if let Some(rest) = trimmed.strip_prefix("pub ") {
                let Some((field, field_type)) = rest.split_once(':') else {
                    continue;
                };
                let normalized_type = field_type.trim().trim_end_matches(',');
                if include_type(normalized_type) {
                    discovered.insert(format!("{struct_name}.{field}"));
                }
            }
        }
        discovered
    }

    #[test]
    fn missing_duplicate_bad_reference_wrong_target_and_double_location_are_rejected() {
        let mut fixture = fixture();
        fixture.actors[0].current_place_id = PlaceId::new("missing_place").unwrap();
        fixture.places.push(PlaceSchema {
            place_id: PlaceId::new("shop_front").unwrap(),
            display_label: "Duplicate".to_string(),
            adjacent_place_ids: Vec::new(),
        });
        fixture.affordances.push(ActionAffordanceSchema {
            action_id: ActionId::new("open").unwrap(),
            target_id: "coin_stack_01".to_string(),
        });
        fixture.containers.push(ContainerSchema {
            container_id: ContainerId::new("crate_01").unwrap(),
            place_id: PlaceId::new("shop_front").unwrap(),
            is_open: true,
            is_locked: false,
            contents: vec![ItemId::new("coin_stack_01").unwrap()],
            contents_visible_when_closed: true,
        });

        let report = validate_fixture(&fixture, &registry()).unwrap_err().report;
        let codes = report
            .errors
            .iter()
            .map(|error| error.code)
            .collect::<BTreeSet<_>>();
        assert!(codes.contains("duplicate_id"));
        assert!(codes.contains("bad_reference"));
        assert!(codes.contains("unsupported_action_target"));
        assert!(codes.contains("item_double_location"));
    }

    #[test]
    fn forbidden_player_and_script_forms_are_rejected_from_raw_content() {
        let raw = b"fixture|bad_fixture\nschema|schema_v1\nplayer|actor_tomas\non_open|force_event";
        let report = validate_fixture_bytes(raw, &registry()).unwrap_err().report;
        assert!(report
            .errors
            .iter()
            .any(|error| error.phase == ValidationPhase::NoPlayer));
        assert!(report
            .errors
            .iter()
            .any(|error| error.phase == ValidationPhase::NoScript));
    }

    #[test]
    fn deterministic_ordering_hazards_sort_errors() {
        let mut fixture = fixture();
        fixture.places[0].adjacent_place_ids = vec![
            PlaceId::new("shop_front").unwrap(),
            PlaceId::new("back_room").unwrap(),
        ];
        fixture.containers[0]
            .contents
            .push(ItemId::new("coin_stack_01").unwrap());

        let report = validate_fixture(&fixture, &registry()).unwrap_err().report;
        let sorted = {
            let mut errors = report.errors.clone();
            errors.sort_by(|left, right| {
                (left.phase, &left.path, left.code, &left.message).cmp(&(
                    right.phase,
                    &right.path,
                    right.code,
                    &right.message,
                ))
            });
            errors
        };
        assert_eq!(report.errors, sorted);
        assert!(report
            .errors
            .iter()
            .any(|error| error.code == "non_canonical_ordering"));
    }

    #[test]
    fn action_affordances_must_resolve_to_known_action_and_supported_target() {
        let mut fixture = fixture();
        fixture.affordances.push(ActionAffordanceSchema {
            action_id: ActionId::new("steal").unwrap(),
            target_id: "coin_stack_01".to_string(),
        });

        let report = validate_fixture(&fixture, &registry()).unwrap_err().report;
        assert!(report
            .errors
            .iter()
            .any(|error| error.code == "unknown_action"));
    }

    #[test]
    fn phase3a_valid_routine_content_is_accepted() {
        validate_fixture(&phase3a_fixture(), &registry()).unwrap();
    }

    #[test]
    fn phase3a_sleep_routine_requires_authored_sleep_surface_or_diagnostic() {
        let mut fixture = phase3a_fixture();
        fixture.sleep_places.clear();
        fixture.routine_templates[0].family = RoutineFamily::SleepNight;
        fixture.routine_templates[0].steps = vec![RoutineStep::StartScheduledSleep {
            action_id: SemanticActionId::new("sleep").unwrap(),
        }];

        let report = validate_fixture(&fixture, &registry()).unwrap_err().report;

        assert!(report
            .errors
            .iter()
            .any(|error| error.code == "missing_sleep_surface"));
    }

    #[test]
    fn phase3a_routine_structure_failures_are_rejected() {
        let mut fixture = phase3a_fixture();
        fixture.routine_templates[0].failure_modes.clear();
        fixture.routine_templates[0].min_duration_ticks = 0;
        fixture.routine_templates[0].steps = vec![RoutineStep::StartWorkBlock {
            action_id: SemanticActionId::new("unknown_action.workplace_shop").unwrap(),
        }];
        fixture.routine_assignments[0].template_id =
            RoutineTemplateId::new("routine_missing").unwrap();

        let report = validate_fixture(&fixture, &registry()).unwrap_err().report;
        let codes = report
            .errors
            .iter()
            .map(|error| error.code)
            .collect::<BTreeSet<_>>();
        assert!(codes.contains("invalid_routine_template"));
        assert!(codes.contains("unknown_action"));
        assert!(codes.contains("bad_reference"));
    }

    #[test]
    fn phase3a_routine_typed_modes_windows_and_direct_ops_are_rejected() {
        let mut fixture = phase3a_fixture();
        fixture.routine_templates[0]
            .failure_modes
            .push("story_beats_complete".to_string());
        fixture.routine_templates[0]
            .fallback_rules
            .push("teleport_to_workplace".to_string());
        fixture.routine_templates[0].steps = vec![RoutineStep::ContinueCurrentStep {
            action_id: SemanticActionId::new("set_need.hunger").unwrap(),
        }];
        fixture.routine_assignments[0].start_tick = SimTick::new(20);
        fixture.routine_assignments[0].end_tick = SimTick::new(20);
        fixture.day_windows.push(crate::schema::DayWindowSchema {
            actor_id: ActorId::new("actor_tomas").unwrap(),
            start_tick: SimTick::new(5),
            end_tick: SimTick::new(5),
        });

        let report = validate_fixture(&fixture, &registry()).unwrap_err().report;
        let codes = report
            .errors
            .iter()
            .map(|error| error.code)
            .collect::<BTreeSet<_>>();

        assert!(codes.contains("unknown_failure_mode"));
        assert!(codes.contains("unknown_fallback_rule"));
        assert!(codes.contains("authored_shortcut_effect"));
        assert!(codes.contains("bad_tick_order"));
    }

    #[test]
    fn phase3a_routine_shortcut_effects_are_rejected() {
        let mut fixture = phase3a_fixture();
        fixture.routine_templates[0]
            .fallback_rules
            .push("hunger_refill_without_food".to_string());
        fixture.routine_templates[0]
            .debug_labels
            .push("instant_sleep_refill".to_string());
        fixture.workplaces[0].output_tag = "work_always_succeeds".to_string();
        fixture.sleep_places[0].sleep_place_id = SleepAffordanceId::new("always_succeeds").unwrap();

        let report = validate_fixture(&fixture, &registry()).unwrap_err().report;
        for marker in [
            "hunger_refill_without_food",
            "instant_sleep_refill",
            "work_always_succeeds",
            "always_succeeds",
        ] {
            assert!(
                report
                    .errors
                    .iter()
                    .any(|error| error.code == "authored_shortcut_effect"
                        && error.message.contains(marker)),
                "missing shortcut rejection for {marker}: {report:?}"
            );
        }
    }
}
