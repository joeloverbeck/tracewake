use tracewake_core::epistemics::projection::EpistemicProjection;
use tracewake_core::epistemics::SourceRef;
use tracewake_core::events::apply::apply_epistemic_event;
use tracewake_core::events::log::EventLog;
use tracewake_core::events::{
    EventEnvelope, EventKind, EventStream, PayloadField, EVENT_SCHEMA_V1,
};
use tracewake_core::ids::{
    ActionId, ActorId, ContentManifestId, ContentVersion, EventId, PlaceId, ProcessId,
};
use tracewake_core::runtime::LoadedWorldBootstrap;
use tracewake_core::scheduler::{OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId};
use tracewake_core::time::SimTick;

use crate::manifest::ContentManifest;
use crate::schema::{FixtureSchema, FixtureScope, WorkplaceSchema};
use crate::serialization::SerializationError;
use crate::validate::{validate_fixture_bytes, ContentValidationFailure};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LoadError {
    Serialization(SerializationError),
    Validation(ContentValidationFailure),
    EpistemicSeedEvent(String),
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
    pub epistemic_projection: EpistemicProjection,
    pub seed_event_log: EventLog,
}

impl LoadedFixture {
    pub fn into_runtime_bootstrap(
        self,
        registry: tracewake_core::actions::ActionRegistry,
    ) -> LoadedWorldBootstrap {
        LoadedWorldBootstrap::from_loaded_state(
            registry,
            self.canonical_world,
            self.canonical_agent_state,
            self.seed_event_log,
            self.epistemic_projection,
            self.manifest.manifest_id,
        )
    }
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
    let registry = registry_for_primary_bytes(&primary.bytes)?;
    let accepted_world = validate_fixture_bytes(&primary.bytes, &registry)?;
    let fixture = accepted_world.fixture;
    let mut manifest = ContentManifest::new(
        manifest_id,
        fixture.fixture_id.clone(),
        fixture.schema_version.clone(),
        content_version,
        files
            .iter()
            .map(|file| (file.path.clone(), file.bytes.clone()))
            .collect(),
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
    let validation_token = accepted_world.validation_token;
    let canonical_world = accepted_world.physical_state;
    let canonical_agent_state = fixture.to_agent_state(validation_token);
    let seed_event_log = seed_event_log(&fixture, manifest.manifest_id.clone());
    let epistemic_projection =
        seed_epistemic_projection(manifest.manifest_id.clone(), &seed_event_log)?;
    Ok(LoadedFixture {
        fixture,
        manifest,
        canonical_world,
        canonical_agent_state,
        epistemic_projection,
        seed_event_log,
    })
}

fn registry_for_primary_bytes(
    bytes: &[u8],
) -> Result<tracewake_core::actions::ActionRegistry, SerializationError> {
    let text = std::str::from_utf8(bytes)
        .map_err(|_| SerializationError::BadLine("non-utf8 fixture".to_string()))?;
    let scope = fixture_scope_from_raw_lines(text)?;
    Ok(registry_for_fixture_scope(scope))
}

fn fixture_scope_from_raw_lines(text: &str) -> Result<FixtureScope, SerializationError> {
    let mut scope = None;
    for line in text.lines() {
        let mut parts = line.split('|');
        match (parts.next(), parts.next(), parts.next()) {
            (Some("fixture_scope"), Some("phase1"), None) => {
                if scope.replace(FixtureScope::Phase1).is_some() {
                    return Err(SerializationError::BadLine(
                        "duplicate fixture_scope".to_string(),
                    ));
                }
            }
            (Some("fixture_scope"), Some("phase2a_historical"), None) => {
                if scope.replace(FixtureScope::Phase2AHistorical).is_some() {
                    return Err(SerializationError::BadLine(
                        "duplicate fixture_scope".to_string(),
                    ));
                }
            }
            (Some("fixture_scope"), Some("phase3a_historical"), None) => {
                if scope.replace(FixtureScope::Phase3AHistorical).is_some() {
                    return Err(SerializationError::BadLine(
                        "duplicate fixture_scope".to_string(),
                    ));
                }
            }
            (Some("fixture_scope"), ..) => {
                return Err(SerializationError::BadLine(line.to_string()));
            }
            _ => {}
        }
    }
    scope.ok_or(SerializationError::MissingField("fixture_scope"))
}

fn seed_epistemic_projection(
    manifest_id: ContentManifestId,
    seed_event_log: &EventLog,
) -> Result<EpistemicProjection, LoadError> {
    let mut projection = EpistemicProjection::new(manifest_id);
    for event in seed_event_log.events() {
        if event.stream == EventStream::Epistemic {
            apply_epistemic_event(&mut projection, event).map_err(|error| {
                LoadError::EpistemicSeedEvent(format!(
                    "fixture seed epistemic event {} failed: {error:?}",
                    event.event_id.as_str()
                ))
            })?;
            projection.record_applied_event(event.event_id.clone());
        }
    }
    Ok(projection)
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

    for edge in &fixture.known_food_sources {
        let food = fixture
            .food_supplies
            .iter()
            .find(|food| food.food_supply_id == edge.food_supply_id)
            .expect("known food source references are validated before seed log construction");
        append_starting_belief(
            &mut log,
            &mut sequence,
            manifest_id.clone(),
            edge.actor_id.clone(),
            "household_food_source",
            food.food_supply_id.as_str(),
            serialize_location(&food.location),
        );
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
            PayloadField::new("access_open", workplace.role_notice_access_open.to_string()),
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
        ActorSchema, ContainerSchema, FixtureSchema, FixtureScope, InitialNeedSchema, ItemSchema,
        NeedModelSchema, PlaceSchema,
    };
    use crate::serialization::serialize_fixture;
    use tracewake_core::agent::NeedKind;
    use tracewake_core::ids::{
        ActorId, ContainerId, ControllerId, FixtureId, ItemId, PlaceId, SchemaVersion,
    };
    use tracewake_core::location::Location;
    use tracewake_core::runtime::{LoadedWorldRuntime, RuntimeReceiptKind};
    use tracewake_core::scheduler::WorldAdvanceOrigin;
    use tracewake_core::state::VisibilityDefault;

    fn fixture() -> FixtureSchema {
        FixtureSchema {
            fixture_id: FixtureId::new("strongbox_001").unwrap(),
            schema_version: SchemaVersion::new("schema_v1").unwrap(),
            fixture_scope: FixtureScope::Phase1,
            need_model: NeedModelSchema {
                awake_hunger_delta_per_tick: 5,
                awake_fatigue_delta_per_tick: 3,
            },
            actors: vec![
                ActorSchema {
                    actor_id: ActorId::new("actor_tomas").unwrap(),
                    current_place_id: PlaceId::new("shop_front").unwrap(),
                },
                ActorSchema {
                    actor_id: ActorId::new("actor_elena").unwrap(),
                    current_place_id: PlaceId::new("shop_front").unwrap(),
                },
            ],
            places: vec![PlaceSchema {
                place_id: PlaceId::new("shop_front").unwrap(),
                display_label: "Shop front".to_string(),
                adjacent_place_ids: Vec::new(),
                visibility_default: VisibilityDefault::Visible,
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
            initial_needs: vec![
                InitialNeedSchema {
                    actor_id: ActorId::new("actor_tomas").unwrap(),
                    kind: NeedKind::Hunger,
                    value: 100,
                },
                InitialNeedSchema {
                    actor_id: ActorId::new("actor_tomas").unwrap(),
                    kind: NeedKind::Fatigue,
                    value: 100,
                },
                InitialNeedSchema {
                    actor_id: ActorId::new("actor_tomas").unwrap(),
                    kind: NeedKind::Safety,
                    value: 100,
                },
                InitialNeedSchema {
                    actor_id: ActorId::new("actor_elena").unwrap(),
                    kind: NeedKind::Hunger,
                    value: 100,
                },
                InitialNeedSchema {
                    actor_id: ActorId::new("actor_elena").unwrap(),
                    kind: NeedKind::Fatigue,
                    value: 100,
                },
                InitialNeedSchema {
                    actor_id: ActorId::new("actor_elena").unwrap(),
                    kind: NeedKind::Safety,
                    value: 100,
                },
            ],
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

    #[test]
    fn loaded_fixture_exports_scheduler_free_runtime_bootstrap() {
        let bytes = serialize_fixture(&fixture());
        let loaded = load_fixture_package(
            ContentManifestId::new("manifest_runtime_bootstrap").unwrap(),
            ContentVersion::new("content_v1").unwrap(),
            vec![SourceFile {
                path: "fixture.twf".to_string(),
                bytes,
            }],
        )
        .unwrap();
        let bootstrap =
            loaded.into_runtime_bootstrap(registry_for_fixture_scope(FixtureScope::Phase1));
        let mut runtime = LoadedWorldRuntime::from_bootstrap(bootstrap, SimTick::ZERO);

        let receipt = runtime
            .wait_one_tick(WorldAdvanceOrigin::Controller(
                ControllerId::new("controller_human").unwrap(),
            ))
            .unwrap();

        match receipt.kind() {
            RuntimeReceiptKind::OneTickAdvanced(result) => {
                assert_eq!(result.due_work_summary.actor_transactions_attempted, 2);
                assert_eq!(result.due_work_summary.world_processes_applied, 1);
            }
            _ => panic!("expected one-tick receipt"),
        }
    }
}
