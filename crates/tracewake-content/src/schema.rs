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
    SchemaVersion, WorkplaceId,
};
use tracewake_core::location::Location;
use tracewake_core::state::{
    ActorBody, AgentState, ContainerState, DoorState, FoodSupplyState, ItemState, PhysicalState,
    PlaceState, WorkplaceState,
};
use tracewake_core::time::SimTick;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FixtureSchema {
    pub fixture_id: FixtureId,
    pub schema_version: SchemaVersion,
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
    pub workplaces: Vec<WorkplaceSchema>,
    pub routine_templates: Vec<RoutineTemplateSchema>,
    pub routine_assignments: Vec<RoutineAssignmentSchema>,
    pub day_windows: Vec<DayWindowSchema>,
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
    pub sleep_place_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FoodSupplySchema {
    pub food_supply_id: FoodSupplyId,
    pub location: Location,
    pub servings: u32,
    pub hunger_reduction_per_serving: i32,
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
        let mut belief = Belief::new(
            self.belief_id.clone(),
            HolderKind::Actor(self.holder_actor_id.clone()),
            self.proposition.clone(),
            self.stance,
            self.confidence,
            self.source.clone(),
            self.acquired_tick,
        );
        belief.channel = self.channel;
        belief.last_verified_tick = self.last_verified_tick;
        belief.privacy_scope = self.privacy_scope.clone();
        belief.schema_version = self.schema_version.clone();
        belief
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

    pub fn to_physical_state(&self) -> PhysicalState {
        let mut actors = BTreeMap::new();
        let mut places = BTreeMap::new();
        let mut doors = BTreeMap::new();
        let mut containers = BTreeMap::new();
        let mut items = BTreeMap::new();
        let mut food_supplies = BTreeMap::new();
        let mut workplaces = BTreeMap::new();

        for place in &self.places {
            let mut place_state =
                PlaceState::new(place.place_id.clone(), place.display_label.clone());
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
                workplace.output_tag.clone(),
            );
            workplace_state
                .assigned_actor_ids
                .extend(workplace.assigned_actor_ids.iter().cloned());
            workplace_state.work_duration_ticks = workplace.work_duration_ticks;
            workplace_state.fatigue_delta_per_tick = workplace.fatigue_delta_per_tick;
            workplace_state.hunger_delta_per_tick = workplace.hunger_delta_per_tick;
            workplace_state.max_fatigue_to_start = workplace.max_fatigue_to_start;
            workplace_state.max_hunger_to_start = workplace.max_hunger_to_start;
            workplace_state.access_open = workplace.access_open;
            workplaces.insert(workplace.workplace_id.clone(), workplace_state);
        }

        PhysicalState::from_seed_parts(
            actors,
            places,
            doors,
            containers,
            items,
            food_supplies,
            workplaces,
        )
    }

    pub fn to_agent_state(&self) -> AgentState {
        let mut needs_by_actor: BTreeMap<_, BTreeMap<_, _>> = BTreeMap::new();
        let mut intentions = BTreeMap::new();
        let mut active_intention_by_actor = BTreeMap::new();
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
            let Some(template) = self
                .routine_templates
                .iter()
                .find(|template| template.template_id == assignment.template_id)
            else {
                continue;
            };
            let actor_suffix = assignment
                .actor_id
                .as_str()
                .strip_prefix("actor_")
                .unwrap_or(assignment.actor_id.as_str());
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
            active_intention_by_actor
                .entry(assignment.actor_id.clone())
                .or_insert_with(|| intention_id.clone());
            intentions.entry(intention_id).or_insert(intention);
            routine_executions
                .entry(routine_execution_id.clone())
                .or_insert_with(|| {
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
                    )
                });
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

fn routine_family_assignment_suffix(family: RoutineFamily) -> &'static str {
    match family {
        RoutineFamily::MorningWake => "wake",
        RoutineFamily::EatMeal => "eat",
        RoutineFamily::GoToWork => "go_work",
        RoutineFamily::WorkBlock => "work",
        RoutineFamily::ReturnHome => "return_home",
        RoutineFamily::SleepNight => "sleep",
        RoutineFamily::FindFood => "find_food",
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
