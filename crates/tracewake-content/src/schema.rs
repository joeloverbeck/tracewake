use std::collections::BTreeMap;

use tracewake_core::agent::{
    Intention, IntentionSource, NeedChangeCause, NeedKind, NeedState, RoutineCondition,
    RoutineExecution, RoutineFamily, RoutineStep, RoutineStepProposal,
};
use tracewake_core::epistemics::observation::EPISTEMIC_RECORD_SCHEMA_V1;
use tracewake_core::epistemics::{
    Belief, Channel, Confidence, HolderKind, PrivacyScope, Proposition, SourceRef, Stance,
};
use tracewake_core::events::InitialBeliefSourceKind;
use tracewake_core::ids::{
    ActionId, ActorId, BeliefId, CandidateGoalId, ContainerId, DecisionTraceId, DoorId, FixtureId,
    FoodSupplyId, IntentionId, ItemId, PlaceId, RoutineExecutionId, RoutineTemplateId,
    SchemaVersion, SleepAffordanceId, WorkplaceId,
};
use tracewake_core::location::Location;
use tracewake_core::state::{
    ActorBody, AgentState, ContainerState, DoorState, FoodSupplyState, ItemState, NeedModelState,
    PhysicalState, PlaceState, SleepAffordanceState, VisibilityDefault, WorkplaceState,
};
use tracewake_core::time::SimTick;

use crate::validate::FixtureValidationToken;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ValidationPhase {
    ParseSchema = 1,
    Canonicalization = 2,
    Id = 3,
    Referential = 4,
    Location = 5,
    PhysicalTopology = 6,
    State = 7,
    ActionRegistryParity = 8,
    SemanticView = 9,
    NoPlayer = 10,
    NoScript = 11,
    EpistemicSeed = 12,
    DeterminismHazard = 13,
    FixtureContract = 14,
    Serialization = 15,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ForbiddenConstructPolicy {
    StableIdentity,
    TypedPhysicalState,
    TypedAffordance,
    ActorKnownProvenance,
    TypedNeedSeed,
    TypedRoutineContext,
    TypedResourceState,
    NoAuthoredOutcomeChain,
    TypedSchedule,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContentFieldRegistration {
    pub schema_field: &'static str,
    pub canonical_serialization_key: &'static str,
    pub validation_phase: ValidationPhase,
    pub forbidden_construct_policy: ForbiddenConstructPolicy,
    pub diagnostic_code: &'static str,
}

pub const CONTENT_FIELD_REGISTRY: &[ContentFieldRegistration] = &[
    ContentFieldRegistration {
        schema_field: "fixture_id",
        canonical_serialization_key: "fixture",
        validation_phase: ValidationPhase::Id,
        forbidden_construct_policy: ForbiddenConstructPolicy::StableIdentity,
        diagnostic_code: "missing_stable_id",
    },
    ContentFieldRegistration {
        schema_field: "schema_version",
        canonical_serialization_key: "schema",
        validation_phase: ValidationPhase::ParseSchema,
        forbidden_construct_policy: ForbiddenConstructPolicy::StableIdentity,
        diagnostic_code: "missing_field",
    },
    ContentFieldRegistration {
        schema_field: "fixture_scope",
        canonical_serialization_key: "fixture_scope",
        validation_phase: ValidationPhase::ParseSchema,
        forbidden_construct_policy: ForbiddenConstructPolicy::TypedAffordance,
        diagnostic_code: "missing_field",
    },
    ContentFieldRegistration {
        schema_field: "need_model",
        canonical_serialization_key: "need_model",
        validation_phase: ValidationPhase::State,
        forbidden_construct_policy: ForbiddenConstructPolicy::TypedNeedSeed,
        diagnostic_code: "missing_field",
    },
    ContentFieldRegistration {
        schema_field: "actors",
        canonical_serialization_key: "actor",
        validation_phase: ValidationPhase::Referential,
        forbidden_construct_policy: ForbiddenConstructPolicy::TypedPhysicalState,
        diagnostic_code: "bad_reference",
    },
    ContentFieldRegistration {
        schema_field: "places",
        canonical_serialization_key: "place",
        validation_phase: ValidationPhase::PhysicalTopology,
        forbidden_construct_policy: ForbiddenConstructPolicy::TypedPhysicalState,
        diagnostic_code: "door_adjacency_inconsistency",
    },
    ContentFieldRegistration {
        schema_field: "doors",
        canonical_serialization_key: "door",
        validation_phase: ValidationPhase::State,
        forbidden_construct_policy: ForbiddenConstructPolicy::TypedPhysicalState,
        diagnostic_code: "locked_open_state",
    },
    ContentFieldRegistration {
        schema_field: "containers",
        canonical_serialization_key: "container",
        validation_phase: ValidationPhase::Location,
        forbidden_construct_policy: ForbiddenConstructPolicy::TypedPhysicalState,
        diagnostic_code: "container_item_mismatch",
    },
    ContentFieldRegistration {
        schema_field: "items",
        canonical_serialization_key: "item",
        validation_phase: ValidationPhase::Location,
        forbidden_construct_policy: ForbiddenConstructPolicy::TypedPhysicalState,
        diagnostic_code: "container_item_mismatch",
    },
    ContentFieldRegistration {
        schema_field: "affordances",
        canonical_serialization_key: "affordance",
        validation_phase: ValidationPhase::ActionRegistryParity,
        forbidden_construct_policy: ForbiddenConstructPolicy::TypedAffordance,
        diagnostic_code: "unknown_action",
    },
    ContentFieldRegistration {
        schema_field: "initial_beliefs",
        canonical_serialization_key: "initial_belief",
        validation_phase: ValidationPhase::EpistemicSeed,
        forbidden_construct_policy: ForbiddenConstructPolicy::ActorKnownProvenance,
        diagnostic_code: "unsupported_source_kind",
    },
    ContentFieldRegistration {
        schema_field: "initial_needs",
        canonical_serialization_key: "initial_need",
        validation_phase: ValidationPhase::State,
        forbidden_construct_policy: ForbiddenConstructPolicy::TypedNeedSeed,
        diagnostic_code: "bad_reference",
    },
    ContentFieldRegistration {
        schema_field: "homes",
        canonical_serialization_key: "home",
        validation_phase: ValidationPhase::Referential,
        forbidden_construct_policy: ForbiddenConstructPolicy::TypedRoutineContext,
        diagnostic_code: "bad_reference",
    },
    ContentFieldRegistration {
        schema_field: "sleep_places",
        canonical_serialization_key: "sleep_place",
        validation_phase: ValidationPhase::Referential,
        forbidden_construct_policy: ForbiddenConstructPolicy::TypedRoutineContext,
        diagnostic_code: "bad_reference",
    },
    ContentFieldRegistration {
        schema_field: "food_supplies",
        canonical_serialization_key: "food_supply",
        validation_phase: ValidationPhase::Referential,
        forbidden_construct_policy: ForbiddenConstructPolicy::TypedResourceState,
        diagnostic_code: "bad_reference",
    },
    ContentFieldRegistration {
        schema_field: "known_food_sources",
        canonical_serialization_key: "known_food_source",
        validation_phase: ValidationPhase::EpistemicSeed,
        forbidden_construct_policy: ForbiddenConstructPolicy::ActorKnownProvenance,
        diagnostic_code: "bad_reference",
    },
    ContentFieldRegistration {
        schema_field: "workplaces",
        canonical_serialization_key: "workplace",
        validation_phase: ValidationPhase::Referential,
        forbidden_construct_policy: ForbiddenConstructPolicy::TypedResourceState,
        diagnostic_code: "bad_reference",
    },
    ContentFieldRegistration {
        schema_field: "routine_templates",
        canonical_serialization_key: "routine_template",
        validation_phase: ValidationPhase::NoScript,
        forbidden_construct_policy: ForbiddenConstructPolicy::NoAuthoredOutcomeChain,
        diagnostic_code: "authored_outcome_chain",
    },
    ContentFieldRegistration {
        schema_field: "routine_assignments",
        canonical_serialization_key: "routine_assignment",
        validation_phase: ValidationPhase::State,
        forbidden_construct_policy: ForbiddenConstructPolicy::TypedSchedule,
        diagnostic_code: "bad_tick_order",
    },
    ContentFieldRegistration {
        schema_field: "day_windows",
        canonical_serialization_key: "day_window",
        validation_phase: ValidationPhase::State,
        forbidden_construct_policy: ForbiddenConstructPolicy::TypedSchedule,
        diagnostic_code: "bad_tick_order",
    },
];

pub fn content_field_registry() -> &'static [ContentFieldRegistration] {
    CONTENT_FIELD_REGISTRY
}

pub fn content_field_by_schema_field(
    schema_field: &str,
) -> Option<&'static ContentFieldRegistration> {
    CONTENT_FIELD_REGISTRY
        .iter()
        .find(|registration| registration.schema_field == schema_field)
}

pub fn content_field_by_canonical_key(
    canonical_key: &str,
) -> Option<&'static ContentFieldRegistration> {
    CONTENT_FIELD_REGISTRY
        .iter()
        .find(|registration| registration.canonical_serialization_key == canonical_key)
}

pub fn canonical_key_for_schema_field(schema_field: &str) -> &'static str {
    content_field_by_schema_field(schema_field)
        .unwrap_or_else(|| panic!("FixtureSchema field {schema_field} must be registered"))
        .canonical_serialization_key
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FixtureSchema {
    pub fixture_id: FixtureId,
    pub schema_version: SchemaVersion,
    pub fixture_scope: FixtureScope,
    pub need_model: NeedModelSchema,
    pub actors: Vec<ActorSchema>,
    pub places: Vec<PlaceSchema>,
    pub doors: Vec<DoorSchema>,
    pub containers: Vec<ContainerSchema>,
    pub items: Vec<ItemSchema>,
    pub affordances: Vec<ActionAffordanceSchema>,
    pub initial_beliefs: Vec<InitialBeliefSchema>,
    pub initial_needs: Vec<InitialNeedSchema>,
    pub homes: Vec<HomeSchema>,
    pub sleep_places: Vec<SleepPlaceSchema>,
    pub food_supplies: Vec<FoodSupplySchema>,
    pub known_food_sources: Vec<KnownFoodSourceSchema>,
    pub workplaces: Vec<WorkplaceSchema>,
    pub routine_templates: Vec<RoutineTemplateSchema>,
    pub routine_assignments: Vec<RoutineAssignmentSchema>,
    pub day_windows: Vec<DayWindowSchema>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NeedModelSchema {
    pub awake_hunger_delta_per_tick: i32,
    pub awake_fatigue_delta_per_tick: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FixtureScope {
    Phase1,
    Phase2AHistorical,
    Phase3AHistorical,
}

impl FixtureScope {
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::Phase1 => "phase1",
            Self::Phase2AHistorical => "phase2a_historical",
            Self::Phase3AHistorical => "phase3a_historical",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActorSchema {
    pub actor_id: ActorId,
    pub current_place_id: PlaceId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlaceSchema {
    pub place_id: PlaceId,
    pub display_label: String,
    pub adjacent_place_ids: Vec<PlaceId>,
    pub visibility_default: VisibilityDefault,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DoorSchema {
    pub door_id: DoorId,
    pub endpoint_a: PlaceId,
    pub endpoint_b: PlaceId,
    pub is_open: bool,
    pub is_locked: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContainerSchema {
    pub container_id: ContainerId,
    pub place_id: PlaceId,
    pub is_open: bool,
    pub is_locked: bool,
    pub contents: Vec<ItemId>,
    pub contents_visible_when_closed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ItemSchema {
    pub item_id: ItemId,
    pub portable: bool,
    pub location: Location,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionAffordanceSchema {
    pub action_id: ActionId,
    pub target_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InitialBeliefSchema {
    pub belief_id: BeliefId,
    pub holder_actor_id: ActorId,
    pub proposition: Proposition,
    pub stance: Stance,
    pub confidence: Confidence,
    pub source_kind: InitialBeliefSourceKind,
    pub source: SourceRef,
    pub channel: Option<Channel>,
    pub acquired_tick: SimTick,
    pub last_verified_tick: Option<SimTick>,
    pub privacy_scope: PrivacyScope,
    pub schema_version: SchemaVersion,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InitialNeedSchema {
    pub actor_id: ActorId,
    pub kind: NeedKind,
    pub value: u16,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HomeSchema {
    pub actor_id: ActorId,
    pub place_id: PlaceId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SleepPlaceSchema {
    pub actor_id: ActorId,
    pub place_id: PlaceId,
    pub sleep_place_id: SleepAffordanceId,
    pub access_open: bool,
    pub duration_ticks: u64,
    pub fatigue_recovery_per_tick: i32,
    pub hunger_rise_per_tick: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FoodSupplySchema {
    pub food_supply_id: FoodSupplyId,
    pub location: Location,
    pub servings: u32,
    pub hunger_reduction_per_serving: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KnownFoodSourceSchema {
    pub actor_id: ActorId,
    pub food_supply_id: FoodSupplyId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkplaceSchema {
    pub workplace_id: WorkplaceId,
    pub place_id: PlaceId,
    pub assigned_actor_ids: Vec<ActorId>,
    pub work_duration_ticks: u64,
    pub fatigue_delta_per_tick: i32,
    pub hunger_delta_per_tick: i32,
    pub max_fatigue_to_start: i32,
    pub max_hunger_to_start: i32,
    pub access_open: bool,
    pub role_notice_access_open: bool,
    pub output_tag: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoutineTemplateSchema {
    pub template_id: RoutineTemplateId,
    pub family: RoutineFamily,
    pub applicability_conditions: Vec<RoutineCondition>,
    pub preconditions: Vec<RoutineCondition>,
    pub steps: Vec<RoutineStep>,
    pub min_duration_ticks: u64,
    pub max_duration_ticks: u64,
    pub interruption_points: Vec<usize>,
    pub failure_modes: Vec<String>,
    pub fallback_rules: Vec<String>,
    pub debug_labels: Vec<String>,
    pub reservable_resource: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoutineAssignmentSchema {
    pub actor_id: ActorId,
    pub template_id: RoutineTemplateId,
    pub start_tick: SimTick,
    pub end_tick: SimTick,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DayWindowSchema {
    pub actor_id: ActorId,
    pub start_tick: SimTick,
    pub end_tick: SimTick,
}

impl InitialBeliefSchema {
    pub fn to_belief(&self) -> Belief {
        let belief = Belief::new(
            self.belief_id.clone(),
            HolderKind::Actor(self.holder_actor_id.clone()),
            self.proposition.clone(),
            self.stance,
            self.confidence,
            self.source.clone(),
            self.acquired_tick,
        )
        .with_last_verified_tick(self.last_verified_tick);
        match self.channel {
            Some(channel) => belief.with_channel(channel),
            None => belief,
        }
    }

    pub fn new_expectation(
        belief_id: BeliefId,
        holder_actor_id: ActorId,
        proposition: Proposition,
        confidence: Confidence,
        source: SourceRef,
        acquired_tick: SimTick,
    ) -> Self {
        Self {
            belief_id,
            privacy_scope: PrivacyScope::ActorPrivate(holder_actor_id.clone()),
            holder_actor_id,
            proposition,
            stance: Stance::ExpectsTrue,
            confidence,
            source_kind: InitialBeliefSourceKind::AuthoredPrehistory,
            source,
            channel: None,
            acquired_tick,
            last_verified_tick: None,
            schema_version: SchemaVersion::new(EPISTEMIC_RECORD_SCHEMA_V1).unwrap(),
        }
    }
}

impl FixtureSchema {
    pub fn canonicalize(&mut self) {
        self.actors
            .sort_by(|left, right| left.actor_id.cmp(&right.actor_id));
        self.places
            .sort_by(|left, right| left.place_id.cmp(&right.place_id));
        self.doors
            .sort_by(|left, right| left.door_id.cmp(&right.door_id));
        self.containers
            .sort_by(|left, right| left.container_id.cmp(&right.container_id));
        self.items
            .sort_by(|left, right| left.item_id.cmp(&right.item_id));
        self.affordances.sort_by(|left, right| {
            (&left.action_id, &left.target_id).cmp(&(&right.action_id, &right.target_id))
        });
        self.initial_beliefs
            .sort_by(|left, right| left.belief_id.cmp(&right.belief_id));
        self.initial_needs
            .sort_by(|left, right| (&left.actor_id, left.kind).cmp(&(&right.actor_id, right.kind)));
        self.homes
            .sort_by(|left, right| left.actor_id.cmp(&right.actor_id));
        self.sleep_places.sort_by(|left, right| {
            (&left.actor_id, &left.sleep_place_id).cmp(&(&right.actor_id, &right.sleep_place_id))
        });
        self.food_supplies
            .sort_by(|left, right| left.food_supply_id.cmp(&right.food_supply_id));
        self.known_food_sources.sort_by(|left, right| {
            (&left.actor_id, &left.food_supply_id).cmp(&(&right.actor_id, &right.food_supply_id))
        });
        self.workplaces
            .sort_by(|left, right| left.workplace_id.cmp(&right.workplace_id));
        self.routine_templates
            .sort_by(|left, right| left.template_id.cmp(&right.template_id));
        self.routine_assignments.sort_by(|left, right| {
            (&left.actor_id, &left.template_id, left.start_tick).cmp(&(
                &right.actor_id,
                &right.template_id,
                right.start_tick,
            ))
        });
        self.day_windows.sort_by(|left, right| {
            (&left.actor_id, left.start_tick, left.end_tick).cmp(&(
                &right.actor_id,
                right.start_tick,
                right.end_tick,
            ))
        });
        for place in &mut self.places {
            place.adjacent_place_ids.sort();
        }
        for container in &mut self.containers {
            container.contents.sort();
        }
        for workplace in &mut self.workplaces {
            workplace.assigned_actor_ids.sort();
        }
        for template in &mut self.routine_templates {
            template.applicability_conditions.sort();
            template.preconditions.sort();
            template.failure_modes.sort();
            template.fallback_rules.sort();
            template.debug_labels.sort();
            template.interruption_points.sort();
        }
    }

    pub fn populate_known_food_sources_for_all_actors(&mut self) {
        self.known_food_sources = self
            .actors
            .iter()
            .flat_map(|actor| {
                self.food_supplies
                    .iter()
                    .map(move |food| KnownFoodSourceSchema {
                        actor_id: actor.actor_id.clone(),
                        food_supply_id: food.food_supply_id.clone(),
                    })
            })
            .collect();
        self.known_food_sources.sort_by(|left, right| {
            (&left.actor_id, &left.food_supply_id).cmp(&(&right.actor_id, &right.food_supply_id))
        });
    }

    pub fn to_physical_state(&self) -> PhysicalState {
        let mut actors = BTreeMap::new();
        let mut places = BTreeMap::new();
        let mut doors = BTreeMap::new();
        let mut containers = BTreeMap::new();
        let mut items = BTreeMap::new();
        let mut food_supplies = BTreeMap::new();
        let mut workplaces = BTreeMap::new();
        let mut sleep_affordances = BTreeMap::new();

        for place in &self.places {
            let mut place_state =
                PlaceState::new(place.place_id.clone(), place.display_label.clone());
            place_state.visibility_default = place.visibility_default.clone();
            place_state
                .adjacent_place_ids
                .extend(place.adjacent_place_ids.iter().cloned());
            places.insert(place.place_id.clone(), place_state);
        }

        for actor in &self.actors {
            actors.insert(
                actor.actor_id.clone(),
                ActorBody::new(actor.actor_id.clone(), actor.current_place_id.clone()),
            );
            if let Some(place) = places.get_mut(&actor.current_place_id) {
                place.local_actor_ids.insert(actor.actor_id.clone());
            }
        }

        for door in &self.doors {
            let mut door_state = DoorState::new(
                door.door_id.clone(),
                door.endpoint_a.clone(),
                door.endpoint_b.clone(),
            );
            door_state.is_open = door.is_open;
            door_state.is_locked = door.is_locked;
            if let Some(place) = places.get_mut(&door.endpoint_a) {
                place.connected_door_ids.insert(door.door_id.clone());
            }
            if let Some(place) = places.get_mut(&door.endpoint_b) {
                place.connected_door_ids.insert(door.door_id.clone());
            }
            doors.insert(door.door_id.clone(), door_state);
        }

        for container in &self.containers {
            let mut container_state = ContainerState::fixed_at_place(
                container.container_id.clone(),
                container.place_id.clone(),
            );
            container_state.is_open = container.is_open;
            container_state.is_locked = container.is_locked;
            container_state
                .contents
                .extend(container.contents.iter().cloned());
            container_state.contents_visible_when_closed = container.contents_visible_when_closed;
            if let Some(place) = places.get_mut(&container.place_id) {
                place
                    .local_container_ids
                    .insert(container.container_id.clone());
            }
            containers.insert(container.container_id.clone(), container_state);
        }

        for item in &self.items {
            let mut item_state = ItemState::new(item.item_id.clone(), item.location.clone());
            item_state.portable = item.portable;
            match &item.location {
                Location::AtPlace(place_id) => {
                    if let Some(place) = places.get_mut(place_id) {
                        place.local_item_ids.insert(item.item_id.clone());
                    }
                }
                Location::CarriedBy(actor_id) => {
                    if let Some(actor) = actors.get_mut(actor_id) {
                        actor.carried_item_ids.insert(item.item_id.clone());
                    }
                }
                Location::InContainer(_) => {}
            }
            items.insert(item.item_id.clone(), item_state);
        }

        for food in &self.food_supplies {
            food_supplies.insert(
                food.food_supply_id.clone(),
                FoodSupplyState::new(
                    food.food_supply_id.clone(),
                    food.location.clone(),
                    food.servings,
                    food.hunger_reduction_per_serving,
                ),
            );
        }

        for workplace in &self.workplaces {
            let mut workplace_state = WorkplaceState::new(
                workplace.workplace_id.clone(),
                workplace.place_id.clone(),
                workplace.work_duration_ticks,
                workplace.fatigue_delta_per_tick,
                workplace.hunger_delta_per_tick,
                workplace.max_fatigue_to_start,
                workplace.max_hunger_to_start,
                workplace.output_tag.clone(),
            );
            workplace_state
                .assigned_actor_ids
                .extend(workplace.assigned_actor_ids.iter().cloned());
            workplace_state.access_open = workplace.access_open;
            workplaces.insert(workplace.workplace_id.clone(), workplace_state);
        }

        for sleep_place in &self.sleep_places {
            let sleep_affordance_id = sleep_place.sleep_place_id.clone();
            let mut sleep_affordance = SleepAffordanceState::new(
                sleep_affordance_id.clone(),
                sleep_place.place_id.clone(),
                sleep_place.duration_ticks,
                sleep_place.fatigue_recovery_per_tick,
                sleep_place.hunger_rise_per_tick,
            );
            sleep_affordance.access_open = sleep_place.access_open;
            sleep_affordances.insert(sleep_affordance_id, sleep_affordance);
        }

        PhysicalState::from_seed_parts(
            actors,
            places,
            doors,
            containers,
            items,
            food_supplies,
            workplaces,
            sleep_affordances,
            NeedModelState {
                awake_hunger_delta_per_tick: self.need_model.awake_hunger_delta_per_tick,
                awake_fatigue_delta_per_tick: self.need_model.awake_fatigue_delta_per_tick,
            },
        )
    }

    /// Materializes validated fixture-authored agent state.
    ///
    /// Callers must obtain the validation token from the content validation
    /// gate before authored need seeds can become authoritative agent state.
    ///
    /// ```compile_fail
    /// use tracewake_content::schema::FixtureSchema;
    ///
    /// let fixture: FixtureSchema = todo!();
    /// let _agent_state = fixture.to_agent_state();
    /// ```
    pub fn to_agent_state(&self, _validation: FixtureValidationToken) -> AgentState {
        let mut needs_by_actor: BTreeMap<_, BTreeMap<_, _>> = BTreeMap::new();
        let mut intentions = BTreeMap::new();
        let mut active_intention_by_actor = BTreeMap::new();
        let mut active_intention_candidates: BTreeMap<ActorId, (SimTick, IntentionId)> =
            BTreeMap::new();
        let mut routine_executions = BTreeMap::new();
        for need in &self.initial_needs {
            needs_by_actor
                .entry(need.actor_id.clone())
                .or_default()
                .insert(
                    need.kind,
                    NeedState::initial(
                        need.kind,
                        i32::from(need.value),
                        NeedChangeCause::FixtureInitial,
                    ),
                );
        }
        for actor in &self.actors {
            let needs = needs_by_actor.entry(actor.actor_id.clone()).or_default();
            for kind in [NeedKind::Hunger, NeedKind::Fatigue, NeedKind::Safety] {
                needs.entry(kind).or_insert_with(|| {
                    NeedState::initial(kind, 100, NeedChangeCause::FixtureInitial)
                });
            }
        }
        for assignment in &self.routine_assignments {
            let template = self
                .routine_templates
                .iter()
                .find(|template| template.template_id == assignment.template_id)
                .expect("validated routine assignment references existing template");
            let actor_suffix = routine_assignment_actor_suffix(&assignment.actor_id);
            let family_suffix = routine_family_assignment_suffix(template.family);
            let intention_id =
                IntentionId::new(format!("intention_{actor_suffix}_{family_suffix}")).unwrap();
            let routine_execution_id =
                RoutineExecutionId::new(format!("routine_exec_{actor_suffix}_{family_suffix}"))
                    .unwrap();
            let trace_id =
                DecisionTraceId::new(format!("trace_{actor_suffix}_{family_suffix}")).unwrap();
            let goal_id =
                CandidateGoalId::new(format!("goal_{actor_suffix}_{family_suffix}")).unwrap();
            let current_step = template.steps.first().map(|step| match step.proposed() {
                RoutineStepProposal::Action(action_id) => action_id.as_str().to_string(),
                RoutineStepProposal::Wait(reason) => reason.to_string(),
                RoutineStepProposal::Diagnostic(diagnostic) => diagnostic.to_string(),
            });
            let intention = Intention::adopt(
                intention_id.clone(),
                assignment.actor_id.clone(),
                IntentionSource::FixtureRoutineAssignment,
                goal_id,
                Some(template.template_id.clone()),
                current_step,
                8,
                assignment.start_tick,
                trace_id.clone(),
            );
            active_intention_candidates
                .entry(assignment.actor_id.clone())
                .and_modify(|(start_tick, active_intention_id)| {
                    if assignment.start_tick < *start_tick {
                        *start_tick = assignment.start_tick;
                        *active_intention_id = intention_id.clone();
                    }
                })
                .or_insert((assignment.start_tick, intention_id.clone()));
            assert!(
                intentions.insert(intention_id, intention).is_none(),
                "validated routine assignments produce unique intention IDs"
            );
            assert!(
                routine_executions
                    .insert(
                        routine_execution_id.clone(),
                        RoutineExecution::new(
                            routine_execution_id,
                            assignment.actor_id.clone(),
                            template.template_id.clone(),
                            template.family,
                            assignment.start_tick,
                            Some(assignment.start_tick.next()),
                            Some(assignment.end_tick),
                            template.reservable_resource.clone(),
                            trace_id,
                        ),
                    )
                    .is_none(),
                "validated routine assignments produce unique routine execution IDs"
            );
        }
        for (actor_id, (_start_tick, intention_id)) in active_intention_candidates {
            active_intention_by_actor.insert(actor_id, intention_id);
        }
        AgentState::from_seed_parts(
            needs_by_actor,
            intentions,
            active_intention_by_actor,
            routine_executions,
            BTreeMap::new(),
            BTreeMap::new(),
        )
    }
}

pub(crate) fn routine_assignment_actor_suffix(actor_id: &ActorId) -> &str {
    actor_id
        .as_str()
        .strip_prefix("actor_")
        .unwrap_or(actor_id.as_str())
}

pub(crate) fn routine_family_assignment_suffix(family: RoutineFamily) -> &'static str {
    match family {
        RoutineFamily::MorningWake => "wake",
        RoutineFamily::EatMeal => "eat",
        RoutineFamily::GoToWork => "go_work",
        RoutineFamily::WorkBlock => "work",
        RoutineFamily::ReturnHome => "return_home",
        RoutineFamily::SleepNight => "sleep",
        RoutineFamily::FindFood => "find_food",
        RoutineFamily::LeaveUnsafePlace => "leave_unsafe",
        RoutineFamily::ContinueCurrentIntention => "continue",
        RoutineFamily::Wait | RoutineFamily::IdleWithReason => "wait",
    }
}

impl RoutineTemplateSchema {
    pub fn to_template(
        &self,
    ) -> Result<tracewake_core::agent::RoutineTemplate, tracewake_core::agent::RoutineTemplateError>
    {
        tracewake_core::agent::RoutineTemplate::new(
            self.template_id.clone(),
            self.family,
            self.applicability_conditions.clone(),
            self.preconditions.clone(),
            self.steps.clone(),
            self.min_duration_ticks,
            self.max_duration_ticks,
            self.interruption_points.clone(),
            self.failure_modes.clone(),
            self.fallback_rules.clone(),
            self.debug_labels.clone(),
            self.reservable_resource.clone(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracewake_core::epistemics::proposition::Proposition;

    fn actor_id(value: &str) -> ActorId {
        ActorId::new(value).unwrap()
    }

    fn belief_id(value: &str) -> BeliefId {
        BeliefId::new(value).unwrap()
    }

    fn container_id(value: &str) -> ContainerId {
        ContainerId::new(value).unwrap()
    }

    fn event_id(value: &str) -> tracewake_core::ids::EventId {
        tracewake_core::ids::EventId::new(value).unwrap()
    }

    fn item_id(value: &str) -> ItemId {
        ItemId::new(value).unwrap()
    }

    #[test]
    fn to_belief_uses_builder_preserving_provenance() {
        let holder = actor_id("actor_tomas");
        let source = SourceRef::Event(event_id("event_seed"));
        let mut schema = InitialBeliefSchema::new_expectation(
            belief_id("belief_tomas_expected_coin"),
            holder.clone(),
            Proposition::ItemLocatedInContainer {
                item_id: item_id("coin_stack_01"),
                container_id: container_id("strongbox_tomas"),
            },
            Confidence::new(900).unwrap(),
            source.clone(),
            SimTick::new(2),
        );
        schema.channel = Some(Channel::DirectSight);
        schema.last_verified_tick = Some(SimTick::new(3));

        let belief = schema.to_belief();

        assert_eq!(belief.holder(), &HolderKind::Actor(holder.clone()));
        assert_eq!(belief.source(), &source);
        assert_eq!(belief.channel(), Some(Channel::DirectSight));
        assert_eq!(belief.last_verified_tick(), Some(SimTick::new(3)));
        assert_eq!(belief.privacy_scope(), &PrivacyScope::ActorPrivate(holder));
        assert_eq!(belief.schema_version().as_str(), EPISTEMIC_RECORD_SCHEMA_V1);
    }
}
