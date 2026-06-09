use tracewake_core::epistemics::SourceRef;
use tracewake_core::events::log::EventLog;
use tracewake_core::events::{EventEnvelope, EventKind, PayloadField, EVENT_SCHEMA_V1};
use tracewake_core::ids::{
    ActionId, ActorId, ContentManifestId, ContentVersion, EventId, PlaceId, ProcessId,
};
use tracewake_core::scheduler::{OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId};
use tracewake_core::time::SimTick;

use crate::manifest::ContentManifest;
use crate::schema::{FixtureSchema, FixtureScope, WorkplaceSchema};
use crate::serialization::{deserialize_fixture, serialize_fixture, SerializationError};
use crate::validate::{validate_fixture, ContentValidationFailure};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LoadError {
    Serialization(SerializationError),
    Validation(ContentValidationFailure),
}

impl From<SerializationError> for LoadError {
    fn from(value: SerializationError) -> Self {
        Self::Serialization(value)
    }
}

impl From<ContentValidationFailure> for LoadError {
    fn from(value: ContentValidationFailure) -> Self {
        Self::Validation(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceFile {
    pub path: String,
    pub bytes: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoadedFixture {
    pub fixture: FixtureSchema,
    pub manifest: ContentManifest,
    pub canonical_world: tracewake_core::state::PhysicalState,
    pub canonical_agent_state: tracewake_core::state::AgentState,
    pub seed_event_log: EventLog,
}

pub fn load_fixture_package(
    manifest_id: ContentManifestId,
    content_version: ContentVersion,
    mut files: Vec<SourceFile>,
) -> Result<LoadedFixture, LoadError> {
    files.sort_by(|left, right| left.path.cmp(&right.path));
    let primary = files
        .first()
        .ok_or(SerializationError::MissingField("source_file"))?;
    let mut fixture = deserialize_fixture(&primary.bytes)?;
    fixture.canonicalize();
    let registry = registry_for_fixture_scope(fixture.fixture_scope);
    let accepted_world = validate_fixture(&fixture, &registry)?;
    let canonical_bytes = serialize_fixture(&fixture);
    let mut manifest = ContentManifest::new(
        manifest_id,
        fixture.fixture_id.clone(),
        fixture.schema_version.clone(),
        content_version,
        files.iter().map(|file| file.path.clone()).collect(),
        &canonical_bytes,
    );
    manifest.actor_roster = fixture
        .actors
        .iter()
        .map(|actor| actor.actor_id.as_str().to_string())
        .collect();
    manifest.no_human_day_windows = fixture
        .day_windows
        .iter()
        .map(|window| {
            format!(
                "{}:{}-{}",
                window.actor_id.as_str(),
                window.start_tick.value(),
                window.end_tick.value()
            )
        })
        .collect();
    manifest.actor_roster.sort();
    manifest.no_human_day_windows.sort();
    let canonical_world = accepted_world.physical_state;
    let canonical_agent_state = fixture.to_agent_state();
    let seed_event_log = seed_event_log(&fixture, manifest.manifest_id.clone());
    Ok(LoadedFixture {
        fixture,
        manifest,
        canonical_world,
        canonical_agent_state,
        seed_event_log,
    })
}

fn seed_event_log(fixture: &FixtureSchema, manifest_id: ContentManifestId) -> EventLog {
    let mut log = EventLog::new();
    let mut sequence = 0_u64;

    for belief in &fixture.initial_beliefs {
        let event_id = EventId::new(format!(
            "event_seed_initial_belief_{}",
            belief.belief_id.as_str()
        ))
        .expect("fixture belief ids form valid seed event ids");
        let source_event_id = match &belief.source {
            SourceRef::Event(event_id) => event_id.as_str().to_string(),
            SourceRef::Action(action_id) => action_id.as_str().to_string(),
            SourceRef::Cause(_) => "cause_authored_prehistory".to_string(),
        };
        let mut event = seed_event(
            event_id,
            EventKind::InitialBeliefSeeded,
            belief.holder_actor_id.clone(),
            None,
            sequence,
            manifest_id.clone(),
        );
        event.payload = vec![
            PayloadField::new("schema_version", EVENT_SCHEMA_V1),
            PayloadField::new("source_kind", belief.source_kind.stable_id()),
            PayloadField::new("belief_id", belief.belief_id.as_str()),
            PayloadField::new("holder_actor_id", belief.holder_actor_id.as_str()),
            PayloadField::new("proposition", belief.proposition.serialize_canonical()),
            PayloadField::new("stance", belief.stance.stable_id()),
            PayloadField::new("confidence", belief.confidence.serialize_canonical()),
            PayloadField::new("source_event_id", source_event_id),
            PayloadField::new("acquired_tick", belief.acquired_tick.value().to_string()),
        ];
        if let Some(channel) = belief.channel {
            event
                .payload
                .push(PayloadField::new("channel", channel.stable_id()));
        }
        event.effects_summary = format!(
            "authored prehistory seeded belief {} for {}",
            belief.belief_id.as_str(),
            belief.holder_actor_id.as_str()
        );
        log.append(event).expect("seed belief event is versioned");
        sequence += 1;
    }

    for home in &fixture.homes {
        append_starting_belief(
            &mut log,
            &mut sequence,
            manifest_id.clone(),
            home.actor_id.clone(),
            "home_place",
            home.place_id.as_str(),
            home.place_id.as_str(),
        );
    }

    for sleep_place in &fixture.sleep_places {
        append_starting_belief(
            &mut log,
            &mut sequence,
            manifest_id.clone(),
            sleep_place.actor_id.clone(),
            "sleep_place",
            sleep_place.sleep_place_id.as_str(),
            sleep_place.place_id.as_str(),
        );
    }

    for actor in &fixture.actors {
        for food in &fixture.food_supplies {
            append_starting_belief(
                &mut log,
                &mut sequence,
                manifest_id.clone(),
                actor.actor_id.clone(),
                "household_food_source",
                food.food_supply_id.as_str(),
                serialize_location(&food.location),
            );
        }
    }

    for workplace in &fixture.workplaces {
        append_role_assignment_notices(&mut log, &mut sequence, manifest_id.clone(), workplace);
    }

    log
}

fn serialize_location(location: &tracewake_core::location::Location) -> String {
    match location {
        tracewake_core::location::Location::AtPlace(place_id) => {
            format!("place:{}", place_id.as_str())
        }
        tracewake_core::location::Location::InContainer(container_id) => {
            format!("container:{}", container_id.as_str())
        }
        tracewake_core::location::Location::CarriedBy(actor_id) => {
            format!("actor:{}", actor_id.as_str())
        }
    }
}

fn append_starting_belief(
    log: &mut EventLog,
    sequence: &mut u64,
    manifest_id: ContentManifestId,
    actor_id: ActorId,
    belief_kind: &str,
    subject_id: &str,
    value: impl Into<String>,
) {
    let value = value.into();
    let mut event = seed_event(
        EventId::new(format!(
            "event_seed_starting_belief_{}_{}_{}",
            actor_id.as_str(),
            belief_kind,
            subject_id
        ))
        .expect("fixture ids form valid starting-belief event ids"),
        EventKind::StartingBeliefRecorded,
        actor_id.clone(),
        None,
        *sequence,
        manifest_id,
    );
    event.payload = vec![
        PayloadField::new("schema_version", EVENT_SCHEMA_V1),
        PayloadField::new("source_kind", "authored_prehistory"),
        PayloadField::new("actor_id", actor_id.as_str()),
        PayloadField::new("belief_kind", belief_kind),
        PayloadField::new("subject_id", subject_id),
        PayloadField::new("value", value),
    ];
    event.effects_summary = format!(
        "authored prehistory recorded {belief_kind} for {}",
        actor_id.as_str()
    );
    log.append(event)
        .expect("starting belief seed event is versioned");
    *sequence += 1;
}

fn append_role_assignment_notices(
    log: &mut EventLog,
    sequence: &mut u64,
    manifest_id: ContentManifestId,
    workplace: &WorkplaceSchema,
) {
    for actor_id in &workplace.assigned_actor_ids {
        let mut event = seed_event(
            EventId::new(format!(
                "event_seed_role_assignment_notice_{}_{}",
                actor_id.as_str(),
                workplace.workplace_id.as_str()
            ))
            .expect("fixture ids form valid role-assignment event ids"),
            EventKind::RoleAssignmentNoticeRecorded,
            actor_id.clone(),
            Some(workplace.place_id.clone()),
            *sequence,
            manifest_id.clone(),
        );
        event.payload = vec![
            PayloadField::new("schema_version", EVENT_SCHEMA_V1),
            PayloadField::new("source_kind", "authored_prehistory"),
            PayloadField::new("actor_id", actor_id.as_str()),
            PayloadField::new("workplace_id", workplace.workplace_id.as_str()),
            PayloadField::new("place_id", workplace.place_id.as_str()),
        ];
        event.effects_summary = format!(
            "authored prehistory recorded workplace assignment {} for {}",
            workplace.workplace_id.as_str(),
            actor_id.as_str()
        );
        log.append(event)
            .expect("role assignment seed event is versioned");
        *sequence += 1;
    }
}

fn seed_event(
    event_id: EventId,
    event_type: EventKind,
    actor_id: ActorId,
    place_id: Option<PlaceId>,
    sequence: u64,
    manifest_id: ContentManifestId,
) -> EventEnvelope {
    let mut event = EventEnvelope::new_v1(
        event_id,
        event_type,
        0,
        sequence,
        SimTick::ZERO,
        OrderingKey::new(
            SimTick::ZERO,
            SchedulePhase::NoHumanProcess,
            SchedulerSourceId::Process(ProcessId::new("content_seed").unwrap()),
            ProposalSequence::new(sequence),
            ActionId::new("content_seed").unwrap(),
            vec![actor_id.as_str().to_string()],
            format!("seed_{sequence:04}"),
        ),
        manifest_id,
    );
    event.actor_id = Some(actor_id.clone());
    event.participants = vec![actor_id.to_string()];
    event.place_id = place_id;
    event
}

pub fn registry_for_fixture_scope(scope: FixtureScope) -> tracewake_core::actions::ActionRegistry {
    let mut registry = tracewake_core::actions::ActionRegistry::new();
    registry.register_phase1_movement_open_close();
    registry.register_phase1_take_place();
    registry.register_phase1_inspect_wait();
    match scope {
        FixtureScope::Phase1 => {}
        FixtureScope::Phase2AHistorical => {
            registry.register_phase2a_epistemics();
        }
        FixtureScope::Phase3AHistorical => {
            registry.register_phase2a_epistemics();
            registry.register_phase3a_sleep();
            registry.register_phase3a_eat();
            registry.register_phase3a_work();
            registry.register_phase3a_continue_routine();
        }
    }
    registry
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{
        ActorSchema, ContainerSchema, FixtureSchema, FixtureScope, ItemSchema, PlaceSchema,
    };
    use crate::serialization::serialize_fixture;
    use tracewake_core::ids::{ActorId, ContainerId, FixtureId, ItemId, PlaceId, SchemaVersion};
    use tracewake_core::location::Location;

    fn fixture() -> FixtureSchema {
        FixtureSchema {
            fixture_id: FixtureId::new("strongbox_001").unwrap(),
            schema_version: SchemaVersion::new("schema_v1").unwrap(),
            fixture_scope: FixtureScope::Phase1,
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
            workplaces: Vec::new(),
            routine_templates: Vec::new(),
            routine_assignments: Vec::new(),
            day_windows: Vec::new(),
        }
    }

    #[test]
    fn shuffled_input_paths_load_to_same_canonical_world() {
        let bytes = serialize_fixture(&fixture());
        let first = load_fixture_package(
            ContentManifestId::new("manifest_strongbox").unwrap(),
            ContentVersion::new("content_v1").unwrap(),
            vec![
                SourceFile {
                    path: "fixture.twf".to_string(),
                    bytes: bytes.clone(),
                },
                SourceFile {
                    path: "z_note.ignored".to_string(),
                    bytes: b"ignored".to_vec(),
                },
            ],
        )
        .unwrap();
        let second = load_fixture_package(
            ContentManifestId::new("manifest_strongbox").unwrap(),
            ContentVersion::new("content_v1").unwrap(),
            vec![
                SourceFile {
                    path: "z_note.ignored".to_string(),
                    bytes: b"ignored".to_vec(),
                },
                SourceFile {
                    path: "fixture.twf".to_string(),
                    bytes,
                },
            ],
        )
        .unwrap();

        assert_eq!(first.canonical_world, second.canonical_world);
        assert_eq!(
            first.manifest.content_fingerprint,
            second.manifest.content_fingerprint
        );
    }
}
