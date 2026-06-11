use std::str::FromStr;
use tracewake_core::events::log::{EventLog, EventLogError};

use tracewake_core::agent::{NeedKind, RoutineCondition, RoutineFamily, RoutineStep};
use tracewake_core::epistemics::{
    Channel, Confidence, PrivacyScope, Proposition, SourceRef, Stance,
};
use tracewake_core::events::InitialBeliefSourceKind;
use tracewake_core::ids::{
    ActionId, ActorId, BeliefId, ContainerId, DoorId, EventId, FixtureId, FoodSupplyId, ItemId,
    PlaceId, RoutineTemplateId, SchemaVersion, SleepAffordanceId, WorkplaceId,
};
use tracewake_core::location::Location;
use tracewake_core::time::SimTick;

use crate::schema::{
    canonical_key_for_schema_field, ActionAffordanceSchema, ActorSchema, ContainerSchema,
    DayWindowSchema, DoorSchema, FixtureSchema, FixtureScope, FoodSupplySchema, HomeSchema,
    InitialBeliefSchema, InitialNeedSchema, ItemSchema, KnownFoodSourceSchema, NeedModelSchema,
    PlaceSchema, RoutineAssignmentSchema, RoutineTemplateSchema, SleepPlaceSchema, WorkplaceSchema,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SerializationError {
    MissingField(&'static str),
    BadLine(String),
    BadBool(String),
    BadU64(String),
    Id(tracewake_core::ids::IdError),
    EventLog(EventLogError),
    Proposition(tracewake_core::epistemics::proposition::PropositionParseError),
    Confidence(tracewake_core::epistemics::observation::ConfidenceError),
}

impl From<tracewake_core::ids::IdError> for SerializationError {
    fn from(value: tracewake_core::ids::IdError) -> Self {
        Self::Id(value)
    }
}

impl From<tracewake_core::epistemics::proposition::PropositionParseError> for SerializationError {
    fn from(value: tracewake_core::epistemics::proposition::PropositionParseError) -> Self {
        Self::Proposition(value)
    }
}

impl From<tracewake_core::epistemics::observation::ConfidenceError> for SerializationError {
    fn from(value: tracewake_core::epistemics::observation::ConfidenceError) -> Self {
        Self::Confidence(value)
    }
}

pub fn serialize_fixture(fixture: &FixtureSchema) -> Vec<u8> {
    let mut fixture = fixture.clone();
    fixture.canonicalize();
    let mut lines = vec![
        format!(
            "{}|{}",
            canonical_key_for_schema_field("fixture_id"),
            fixture.fixture_id.as_str()
        ),
        format!(
            "{}|{}",
            canonical_key_for_schema_field("schema_version"),
            fixture.schema_version.as_str()
        ),
        format!(
            "{}|{}",
            canonical_key_for_schema_field("fixture_scope"),
            fixture.fixture_scope.stable_id()
        ),
        format!(
            "{}|{}|{}",
            canonical_key_for_schema_field("need_model"),
            fixture.need_model.awake_hunger_delta_per_tick,
            fixture.need_model.awake_fatigue_delta_per_tick
        ),
    ];
    for actor in fixture.actors {
        lines.push(format!(
            "{}|{}|{}",
            canonical_key_for_schema_field("actors"),
            actor.actor_id.as_str(),
            actor.current_place_id.as_str()
        ));
    }
    for place in fixture.places {
        lines.push(format!(
            "{}|{}|{}|{}",
            canonical_key_for_schema_field("places"),
            place.place_id.as_str(),
            encode(&place.display_label),
            join(place.adjacent_place_ids.iter().map(|id| id.as_str()))
        ));
    }
    for door in fixture.doors {
        lines.push(format!(
            "{}|{}|{}|{}|{}|{}",
            canonical_key_for_schema_field("doors"),
            door.door_id.as_str(),
            door.endpoint_a.as_str(),
            door.endpoint_b.as_str(),
            door.is_open,
            door.is_locked
        ));
    }
    for container in fixture.containers {
        lines.push(format!(
            "{}|{}|{}|{}|{}|{}|{}",
            canonical_key_for_schema_field("containers"),
            container.container_id.as_str(),
            container.place_id.as_str(),
            container.is_open,
            container.is_locked,
            container.contents_visible_when_closed,
            join(container.contents.iter().map(|id| id.as_str()))
        ));
    }
    for item in fixture.items {
        lines.push(format!(
            "{}|{}|{}|{}",
            canonical_key_for_schema_field("items"),
            item.item_id.as_str(),
            item.portable,
            serialize_location(&item.location)
        ));
    }
    for affordance in fixture.affordances {
        lines.push(format!(
            "{}|{}|{}",
            canonical_key_for_schema_field("affordances"),
            affordance.action_id.as_str(),
            affordance.target_id
        ));
    }
    for belief in fixture.initial_beliefs {
        lines.push(format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            canonical_key_for_schema_field("initial_beliefs"),
            belief.belief_id.as_str(),
            belief.holder_actor_id.as_str(),
            encode(&belief.proposition.serialize_canonical()),
            belief.stance.stable_id(),
            belief.confidence.serialize_canonical(),
            belief.source_kind.stable_id(),
            source_id(&belief.source),
            belief.channel.map(channel_id).unwrap_or(""),
            belief.acquired_tick.value(),
            belief
                .last_verified_tick
                .map(|tick| tick.value().to_string())
                .unwrap_or_default(),
            serialize_privacy_scope(&belief.privacy_scope),
            belief.schema_version.as_str(),
        ));
    }
    for need in fixture.initial_needs {
        lines.push(format!(
            "{}|{}|{}|{}",
            canonical_key_for_schema_field("initial_needs"),
            need.actor_id.as_str(),
            need.kind.stable_id(),
            need.value
        ));
    }
    for home in fixture.homes {
        lines.push(format!(
            "{}|{}|{}",
            canonical_key_for_schema_field("homes"),
            home.actor_id.as_str(),
            home.place_id.as_str()
        ));
    }
    for sleep_place in fixture.sleep_places {
        lines.push(format!(
            "{}|{}|{}|{}|{}|{}|{}|{}",
            canonical_key_for_schema_field("sleep_places"),
            sleep_place.actor_id.as_str(),
            sleep_place.place_id.as_str(),
            sleep_place.sleep_place_id.as_str(),
            sleep_place.access_open,
            sleep_place.duration_ticks,
            sleep_place.fatigue_recovery_per_tick,
            sleep_place.hunger_rise_per_tick
        ));
    }
    for food in fixture.food_supplies {
        lines.push(format!(
            "{}|{}|{}|{}|{}",
            canonical_key_for_schema_field("food_supplies"),
            food.food_supply_id.as_str(),
            serialize_location(&food.location),
            food.servings,
            food.hunger_reduction_per_serving
        ));
    }
    for edge in fixture.known_food_sources {
        lines.push(format!(
            "{}|{}|{}",
            canonical_key_for_schema_field("known_food_sources"),
            edge.actor_id.as_str(),
            edge.food_supply_id.as_str()
        ));
    }
    for workplace in fixture.workplaces {
        lines.push(format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            canonical_key_for_schema_field("workplaces"),
            workplace.workplace_id.as_str(),
            workplace.place_id.as_str(),
            join(workplace.assigned_actor_ids.iter().map(|id| id.as_str())),
            workplace.work_duration_ticks,
            workplace.fatigue_delta_per_tick,
            workplace.hunger_delta_per_tick,
            workplace.max_fatigue_to_start,
            workplace.max_hunger_to_start,
            workplace.access_open,
            workplace.role_notice_access_open,
            encode(&workplace.output_tag)
        ));
    }
    for template in fixture.routine_templates {
        lines.push(format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            canonical_key_for_schema_field("routine_templates"),
            template.template_id.as_str(),
            template.family.stable_id(),
            join_encoded(
                &template
                    .applicability_conditions
                    .iter()
                    .map(|condition| condition.stable_id().to_string())
                    .collect::<Vec<_>>()
            ),
            join_encoded(
                &template
                    .preconditions
                    .iter()
                    .map(|condition| condition.stable_id().to_string())
                    .collect::<Vec<_>>()
            ),
            join_encoded(
                &template
                    .steps
                    .iter()
                    .map(RoutineStep::serialize_canonical)
                    .collect::<Vec<_>>()
            ),
            template.min_duration_ticks,
            template.max_duration_ticks,
            join_usize(&template.interruption_points),
            join_encoded(&template.failure_modes),
            join_encoded(&template.fallback_rules),
            join_encoded(&template.debug_labels),
            template
                .reservable_resource
                .as_ref()
                .map(|value| encode(value))
                .unwrap_or_default()
        ));
    }
    for assignment in fixture.routine_assignments {
        lines.push(format!(
            "{}|{}|{}|{}|{}",
            canonical_key_for_schema_field("routine_assignments"),
            assignment.actor_id.as_str(),
            assignment.template_id.as_str(),
            assignment.start_tick.value(),
            assignment.end_tick.value()
        ));
    }
    for window in fixture.day_windows {
        lines.push(format!(
            "{}|{}|{}|{}",
            canonical_key_for_schema_field("day_windows"),
            window.actor_id.as_str(),
            window.start_tick.value(),
            window.end_tick.value()
        ));
    }
    lines.join("\n").into_bytes()
}

pub fn deserialize_fixture(bytes: &[u8]) -> Result<FixtureSchema, SerializationError> {
    let text = std::str::from_utf8(bytes)
        .map_err(|_| SerializationError::BadLine("non-utf8 fixture".to_string()))?;
    let mut fixture_id = None;
    let mut schema_version = None;
    let mut fixture_scope = None;
    let mut need_model = None;
    let mut actors = Vec::new();
    let mut places = Vec::new();
    let mut doors = Vec::new();
    let mut containers = Vec::new();
    let mut items = Vec::new();
    let mut affordances = Vec::new();
    let mut initial_beliefs = Vec::new();
    let mut initial_needs = Vec::new();
    let mut homes = Vec::new();
    let mut sleep_places = Vec::new();
    let mut food_supplies = Vec::new();
    let mut known_food_sources = Vec::new();
    let mut workplaces = Vec::new();
    let mut routine_templates = Vec::new();
    let mut routine_assignments = Vec::new();
    let mut day_windows = Vec::new();

    for line in text.lines() {
        let parts = line.split('|').collect::<Vec<_>>();
        match parts.as_slice() {
            ["fixture", id] => fixture_id = Some(FixtureId::new(*id)?),
            ["schema", version] => schema_version = Some(SchemaVersion::new(*version)?),
            ["fixture_scope", scope] => fixture_scope = Some(parse_fixture_scope(scope)?),
            ["need_model", awake_hunger_delta_per_tick, awake_fatigue_delta_per_tick] => {
                need_model = Some(NeedModelSchema {
                    awake_hunger_delta_per_tick: parse_i32(awake_hunger_delta_per_tick)?,
                    awake_fatigue_delta_per_tick: parse_i32(awake_fatigue_delta_per_tick)?,
                })
            }
            ["actor", actor_id, place_id] => actors.push(ActorSchema {
                actor_id: ActorId::new(*actor_id)?,
                current_place_id: PlaceId::new(*place_id)?,
            }),
            ["place", place_id, label, adjacent] => places.push(PlaceSchema {
                place_id: PlaceId::new(*place_id)?,
                display_label: decode(label)?,
                adjacent_place_ids: split_ids(adjacent, |part| PlaceId::new(part))?,
            }),
            ["door", door_id, endpoint_a, endpoint_b, is_open, is_locked] => {
                doors.push(DoorSchema {
                    door_id: DoorId::new(*door_id)?,
                    endpoint_a: PlaceId::new(*endpoint_a)?,
                    endpoint_b: PlaceId::new(*endpoint_b)?,
                    is_open: parse_bool(is_open)?,
                    is_locked: parse_bool(is_locked)?,
                })
            }
            ["container", container_id, place_id, is_open, is_locked, contents_visible_when_closed, contents] => {
                containers.push(ContainerSchema {
                    container_id: ContainerId::new(*container_id)?,
                    place_id: PlaceId::new(*place_id)?,
                    is_open: parse_bool(is_open)?,
                    is_locked: parse_bool(is_locked)?,
                    contents_visible_when_closed: parse_bool(contents_visible_when_closed)?,
                    contents: split_ids(contents, |part| ItemId::new(part))?,
                })
            }
            ["item", item_id, portable, location] => items.push(ItemSchema {
                item_id: ItemId::new(*item_id)?,
                portable: parse_bool(portable)?,
                location: deserialize_location(location)?,
            }),
            ["affordance", action_id, target_id] => affordances.push(ActionAffordanceSchema {
                action_id: ActionId::new(*action_id)?,
                target_id: (*target_id).to_string(),
            }),
            ["initial_belief", belief_id, holder_actor_id, proposition, stance, confidence, source_kind, source_id, channel, acquired_tick, last_verified_tick, privacy_scope, schema_version] => {
                initial_beliefs.push(InitialBeliefSchema {
                    belief_id: BeliefId::new(*belief_id)?,
                    holder_actor_id: ActorId::new(*holder_actor_id)?,
                    proposition: Proposition::from_str(&decode(proposition)?)?,
                    stance: parse_stance(stance)?,
                    confidence: parse_confidence(confidence)?,
                    source_kind: parse_source_kind(source_kind)?,
                    source: parse_source(source_kind, source_id)?,
                    channel: parse_optional_channel(channel)?,
                    acquired_tick: parse_tick(acquired_tick)?,
                    last_verified_tick: parse_optional_tick(last_verified_tick)?,
                    privacy_scope: parse_privacy_scope(privacy_scope)?,
                    schema_version: SchemaVersion::new(*schema_version)?,
                })
            }
            ["initial_need", actor_id, kind, value] => initial_needs.push(InitialNeedSchema {
                actor_id: ActorId::new(*actor_id)?,
                kind: parse_need_kind(kind)?,
                value: parse_u16(value)?,
            }),
            ["home", actor_id, place_id] => homes.push(HomeSchema {
                actor_id: ActorId::new(*actor_id)?,
                place_id: PlaceId::new(*place_id)?,
            }),
            ["sleep_place", actor_id, place_id, sleep_place_id, access_open, duration_ticks, fatigue_recovery_per_tick, hunger_rise_per_tick] => {
                sleep_places.push(SleepPlaceSchema {
                    actor_id: ActorId::new(*actor_id)?,
                    place_id: PlaceId::new(*place_id)?,
                    sleep_place_id: SleepAffordanceId::new(*sleep_place_id)?,
                    access_open: parse_bool(access_open)?,
                    duration_ticks: parse_u64(duration_ticks)?,
                    fatigue_recovery_per_tick: parse_i32(fatigue_recovery_per_tick)?,
                    hunger_rise_per_tick: parse_i32(hunger_rise_per_tick)?,
                })
            }
            ["food_supply", food_supply_id, location, servings, hunger_reduction_per_serving] => {
                food_supplies.push(FoodSupplySchema {
                    food_supply_id: FoodSupplyId::new(*food_supply_id)?,
                    location: deserialize_location(location)?,
                    servings: parse_u32(servings)?,
                    hunger_reduction_per_serving: parse_i32(hunger_reduction_per_serving)?,
                })
            }
            ["known_food_source", actor_id, food_supply_id] => {
                known_food_sources.push(KnownFoodSourceSchema {
                    actor_id: ActorId::new(*actor_id)?,
                    food_supply_id: FoodSupplyId::new(*food_supply_id)?,
                })
            }
            ["workplace", workplace_id, place_id, assigned_actor_ids, work_duration_ticks, fatigue_delta_per_tick, hunger_delta_per_tick, max_fatigue_to_start, max_hunger_to_start, access_open, role_notice_access_open, output_tag] => {
                workplaces.push(WorkplaceSchema {
                    workplace_id: WorkplaceId::new(*workplace_id)?,
                    place_id: PlaceId::new(*place_id)?,
                    assigned_actor_ids: split_ids(assigned_actor_ids, |part| ActorId::new(part))?,
                    work_duration_ticks: parse_u64(work_duration_ticks)?,
                    fatigue_delta_per_tick: parse_i32(fatigue_delta_per_tick)?,
                    hunger_delta_per_tick: parse_i32(hunger_delta_per_tick)?,
                    max_fatigue_to_start: parse_i32(max_fatigue_to_start)?,
                    max_hunger_to_start: parse_i32(max_hunger_to_start)?,
                    access_open: parse_bool(access_open)?,
                    role_notice_access_open: parse_bool(role_notice_access_open)?,
                    output_tag: decode(output_tag)?,
                })
            }
            ["routine_template", template_id, family, applicability_conditions, preconditions, steps, min_duration_ticks, max_duration_ticks, interruption_points, failure_modes, fallback_rules, debug_labels, reservable_resource] => {
                routine_templates.push(RoutineTemplateSchema {
                    template_id: RoutineTemplateId::new(*template_id)?,
                    family: parse_routine_family(family)?,
                    applicability_conditions: split_encoded(applicability_conditions)?
                        .into_iter()
                        .map(parse_routine_condition)
                        .collect::<Result<Vec<_>, _>>()?,
                    preconditions: split_encoded(preconditions)?
                        .into_iter()
                        .map(parse_routine_condition)
                        .collect::<Result<Vec<_>, _>>()?,
                    steps: split_encoded(steps)?
                        .into_iter()
                        .map(|step| {
                            RoutineStep::deserialize_canonical(step.as_bytes())
                                .map_err(|error| SerializationError::BadLine(format!("{error}")))
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                    min_duration_ticks: parse_u64(min_duration_ticks)?,
                    max_duration_ticks: parse_u64(max_duration_ticks)?,
                    interruption_points: split_usize(interruption_points)?,
                    failure_modes: split_encoded(failure_modes)?,
                    fallback_rules: split_encoded(fallback_rules)?,
                    debug_labels: split_encoded(debug_labels)?,
                    reservable_resource: if reservable_resource.is_empty() {
                        None
                    } else {
                        Some(decode(reservable_resource)?)
                    },
                })
            }
            ["routine_assignment", actor_id, template_id, start_tick, end_tick] => {
                routine_assignments.push(RoutineAssignmentSchema {
                    actor_id: ActorId::new(*actor_id)?,
                    template_id: RoutineTemplateId::new(*template_id)?,
                    start_tick: parse_tick(start_tick)?,
                    end_tick: parse_tick(end_tick)?,
                })
            }
            ["day_window", actor_id, start_tick, end_tick] => day_windows.push(DayWindowSchema {
                actor_id: ActorId::new(*actor_id)?,
                start_tick: parse_tick(start_tick)?,
                end_tick: parse_tick(end_tick)?,
            }),
            _ => return Err(SerializationError::BadLine(line.to_string())),
        }
    }

    let mut fixture = FixtureSchema {
        fixture_id: fixture_id.ok_or(SerializationError::MissingField("fixture"))?,
        schema_version: schema_version.ok_or(SerializationError::MissingField("schema"))?,
        fixture_scope: fixture_scope.ok_or(SerializationError::MissingField("fixture_scope"))?,
        need_model: need_model.ok_or(SerializationError::MissingField("need_model"))?,
        actors,
        places,
        doors,
        containers,
        items,
        affordances,
        initial_beliefs,
        initial_needs,
        homes,
        sleep_places,
        food_supplies,
        known_food_sources,
        workplaces,
        routine_templates,
        routine_assignments,
        day_windows,
    };
    fixture.canonicalize();
    Ok(fixture)
}

fn parse_fixture_scope(value: &str) -> Result<FixtureScope, SerializationError> {
    match value {
        "phase1" => Ok(FixtureScope::Phase1),
        "phase2a_historical" => Ok(FixtureScope::Phase2AHistorical),
        "phase3a_historical" => Ok(FixtureScope::Phase3AHistorical),
        _ => Err(SerializationError::BadLine(format!(
            "bad fixture scope {value}"
        ))),
    }
}

pub fn serialize_event_log(log: &EventLog) -> Vec<u8> {
    log.serialize_canonical()
}

pub fn deserialize_event_log(bytes: &[u8]) -> Result<EventLog, SerializationError> {
    EventLog::deserialize_canonical(bytes).map_err(SerializationError::EventLog)
}

fn serialize_location(location: &Location) -> String {
    match location {
        Location::AtPlace(id) => format!("at:{}", id.as_str()),
        Location::InContainer(id) => format!("in:{}", id.as_str()),
        Location::CarriedBy(id) => format!("carried:{}", id.as_str()),
    }
}

fn deserialize_location(value: &str) -> Result<Location, SerializationError> {
    let (kind, id) = value
        .split_once(':')
        .ok_or_else(|| SerializationError::BadLine(value.to_string()))?;
    match kind {
        "at" => Ok(Location::AtPlace(PlaceId::new(id)?)),
        "in" => Ok(Location::InContainer(ContainerId::new(id)?)),
        "carried" => Ok(Location::CarriedBy(ActorId::new(id)?)),
        _ => Err(SerializationError::BadLine(value.to_string())),
    }
}

fn channel_id(channel: Channel) -> &'static str {
    channel.stable_id()
}

fn parse_optional_channel(value: &str) -> Result<Option<Channel>, SerializationError> {
    if value.is_empty() {
        Ok(None)
    } else {
        parse_channel(value).map(Some)
    }
}

fn parse_channel(value: &str) -> Result<Channel, SerializationError> {
    match value {
        "direct_sight" => Ok(Channel::DirectSight),
        "touch_or_search" => Ok(Channel::TouchOrSearch),
        "simple_sound" => Ok(Channel::SimpleSound),
        "absence_marker" => Ok(Channel::AbsenceMarker),
        "reading_placeholder_schema_only" => Ok(Channel::ReadingPlaceholderSchemaOnly),
        _ => Err(SerializationError::BadLine(format!("bad channel {value}"))),
    }
}

fn parse_stance(value: &str) -> Result<Stance, SerializationError> {
    match value {
        "believes_true" => Ok(Stance::BelievesTrue),
        "believes_false" => Ok(Stance::BelievesFalse),
        "expects_true" => Ok(Stance::ExpectsTrue),
        "plausible" => Ok(Stance::Plausible),
        "doubts" => Ok(Stance::Doubts),
        "unknown_or_unresolved" => Ok(Stance::UnknownOrUnresolved),
        _ => Err(SerializationError::BadLine(format!("bad stance {value}"))),
    }
}

fn parse_confidence(value: &str) -> Result<Confidence, SerializationError> {
    Confidence::new(
        value
            .parse::<u16>()
            .map_err(|_| SerializationError::BadU64(value.to_string()))?,
    )
    .map_err(Into::into)
}

fn parse_tick(value: &str) -> Result<SimTick, SerializationError> {
    parse_u64(value).map(SimTick::new)
}

fn parse_optional_tick(value: &str) -> Result<Option<SimTick>, SerializationError> {
    if value.is_empty() {
        Ok(None)
    } else {
        parse_tick(value).map(Some)
    }
}

fn parse_u16(value: &str) -> Result<u16, SerializationError> {
    value
        .parse::<u16>()
        .map_err(|_| SerializationError::BadU64(value.to_string()))
}

fn parse_u32(value: &str) -> Result<u32, SerializationError> {
    value
        .parse::<u32>()
        .map_err(|_| SerializationError::BadU64(value.to_string()))
}

fn parse_u64(value: &str) -> Result<u64, SerializationError> {
    value
        .parse::<u64>()
        .map_err(|_| SerializationError::BadU64(value.to_string()))
}

fn parse_i32(value: &str) -> Result<i32, SerializationError> {
    value
        .parse::<i32>()
        .map_err(|_| SerializationError::BadU64(value.to_string()))
}

fn parse_need_kind(value: &str) -> Result<NeedKind, SerializationError> {
    match value {
        "hunger" => Ok(NeedKind::Hunger),
        "fatigue" => Ok(NeedKind::Fatigue),
        "safety" => Ok(NeedKind::Safety),
        _ => Err(SerializationError::BadLine(format!(
            "bad need kind {value}"
        ))),
    }
}

fn parse_routine_family(value: &str) -> Result<RoutineFamily, SerializationError> {
    match value {
        "morning_wake" => Ok(RoutineFamily::MorningWake),
        "eat_meal" => Ok(RoutineFamily::EatMeal),
        "go_to_work" => Ok(RoutineFamily::GoToWork),
        "work_block" => Ok(RoutineFamily::WorkBlock),
        "return_home" => Ok(RoutineFamily::ReturnHome),
        "sleep_night" => Ok(RoutineFamily::SleepNight),
        "find_food" => Ok(RoutineFamily::FindFood),
        "leave_unsafe_place" => Ok(RoutineFamily::LeaveUnsafePlace),
        "continue_current_intention" => Ok(RoutineFamily::ContinueCurrentIntention),
        "wait" => Ok(RoutineFamily::Wait),
        "idle_with_reason" => Ok(RoutineFamily::IdleWithReason),
        _ => Err(SerializationError::BadLine(format!(
            "bad routine family {value}"
        ))),
    }
}

fn parse_routine_condition(value: String) -> Result<RoutineCondition, SerializationError> {
    RoutineCondition::parse(&value)
        .ok_or_else(|| SerializationError::BadLine(format!("bad routine condition {value}")))
}

fn source_id(source: &SourceRef) -> String {
    match source {
        SourceRef::Event(event_id) => event_id.as_str().to_string(),
        SourceRef::Action(action_id) => action_id.as_str().to_string(),
        SourceRef::Cause(cause) => format!("{cause:?}"),
    }
}

fn parse_source(kind: &str, id: &str) -> Result<SourceRef, SerializationError> {
    if id.is_empty() {
        return Err(SerializationError::BadLine(
            "initial_belief source_id must not be empty".to_string(),
        ));
    }
    match kind {
        "authored_prehistory" => Ok(SourceRef::Event(EventId::new(id)?)),
        _ => Err(SerializationError::BadLine(format!(
            "bad source kind {kind}"
        ))),
    }
}

fn parse_source_kind(value: &str) -> Result<InitialBeliefSourceKind, SerializationError> {
    match value {
        "authored_prehistory" => Ok(InitialBeliefSourceKind::AuthoredPrehistory),
        _ => Err(SerializationError::BadLine(format!(
            "bad source kind {value}"
        ))),
    }
}

fn serialize_privacy_scope(scope: &PrivacyScope) -> String {
    match scope {
        PrivacyScope::ActorPrivate(actor_id) => format!("actor:{}", actor_id.as_str()),
        PrivacyScope::PublicPlaceholder => "public".to_string(),
        PrivacyScope::InstitutionPlaceholder(value) => format!("institution:{}", encode(value)),
    }
}

fn parse_privacy_scope(value: &str) -> Result<PrivacyScope, SerializationError> {
    if value == "public" {
        return Ok(PrivacyScope::PublicPlaceholder);
    }
    let (kind, id) = value
        .split_once(':')
        .ok_or_else(|| SerializationError::BadLine(value.to_string()))?;
    match kind {
        "actor" => Ok(PrivacyScope::ActorPrivate(ActorId::new(id)?)),
        "institution" => Ok(PrivacyScope::InstitutionPlaceholder(decode(id)?)),
        _ => Err(SerializationError::BadLine(value.to_string())),
    }
}

fn split_ids<T>(
    value: &str,
    parse: impl Fn(&str) -> Result<T, tracewake_core::ids::IdError>,
) -> Result<Vec<T>, SerializationError> {
    if value.is_empty() {
        Ok(Vec::new())
    } else {
        value
            .split(',')
            .map(|part| parse(part).map_err(Into::into))
            .collect()
    }
}

fn join<'a>(values: impl Iterator<Item = &'a str>) -> String {
    values.collect::<Vec<_>>().join(",")
}

fn join_encoded(values: &[String]) -> String {
    values
        .iter()
        .map(|value| encode(value))
        .collect::<Vec<_>>()
        .join(",")
}

fn split_encoded(value: &str) -> Result<Vec<String>, SerializationError> {
    if value.is_empty() {
        Ok(Vec::new())
    } else {
        value.split(',').map(decode).collect()
    }
}

fn join_usize(values: &[usize]) -> String {
    values
        .iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

fn split_usize(value: &str) -> Result<Vec<usize>, SerializationError> {
    if value.is_empty() {
        Ok(Vec::new())
    } else {
        value
            .split(',')
            .map(|part| {
                part.parse::<usize>()
                    .map_err(|_| SerializationError::BadU64(part.to_string()))
            })
            .collect()
    }
}

fn parse_bool(value: &str) -> Result<bool, SerializationError> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(SerializationError::BadBool(value.to_string())),
    }
}

fn encode(value: &str) -> String {
    value
        .as_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn decode(value: &str) -> Result<String, SerializationError> {
    if !value.len().is_multiple_of(2) {
        return Err(SerializationError::BadLine(value.to_string()));
    }
    let mut bytes = Vec::new();
    for chunk in value.as_bytes().chunks_exact(2) {
        let hex = std::str::from_utf8(chunk)
            .map_err(|_| SerializationError::BadLine(value.to_string()))?;
        bytes.push(
            u8::from_str_radix(hex, 16)
                .map_err(|_| SerializationError::BadLine(value.to_string()))?,
        );
    }
    String::from_utf8(bytes).map_err(|_| SerializationError::BadLine(value.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracewake_core::events::log::EventLog;
    use tracewake_core::events::{EventEnvelope, EventKind};
    use tracewake_core::ids::{ActionId, ContentManifestId, EventId};
    use tracewake_core::scheduler::{
        OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId,
    };
    use tracewake_core::time::SimTick;

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
            places: vec![PlaceSchema {
                place_id: PlaceId::new("shop_front").unwrap(),
                display_label: "Shop front".to_string(),
                adjacent_place_ids: Vec::new(),
            }],
            doors: Vec::new(),
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
            affordances: Vec::new(),
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

    #[test]
    fn fixture_serialization_round_trips_byte_identically() {
        let first = serialize_fixture(&fixture());
        let parsed = deserialize_fixture(&first).unwrap();
        let second = serialize_fixture(&parsed);

        assert_eq!(first, second);
    }

    #[test]
    fn event_log_serialization_round_trips_byte_identically() {
        let mut log = EventLog::new();
        let event = EventEnvelope::new_v1(
            EventId::new("event_0001").unwrap(),
            EventKind::ActorWaited,
            0,
            0,
            SimTick::ZERO,
            OrderingKey::new(
                SimTick::ZERO,
                SchedulePhase::HumanCommand,
                SchedulerSourceId::Actor(ActorId::new("actor_tomas").unwrap()),
                ProposalSequence::new(0),
                ActionId::new("wait").unwrap(),
                vec!["1_tick".to_string()],
                "tie",
            ),
            ContentManifestId::new("phase1_manifest").unwrap(),
        );
        log.append(event).unwrap();

        let first = serialize_event_log(&log);
        let second = serialize_event_log(&deserialize_event_log(&first).unwrap());

        assert_eq!(first, second);
    }
}
