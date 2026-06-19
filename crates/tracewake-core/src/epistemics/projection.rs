use std::collections::{BTreeMap, BTreeSet};

use crate::epistemics::belief::{Belief, HolderKind};
use crate::epistemics::contradiction::Contradiction;
use crate::epistemics::knowledge_context::{KnowledgeContext, ViewMode};
use crate::epistemics::observation::{Observation, SourceRef, EPISTEMIC_RECORD_SCHEMA_V1};
use crate::ids::{
    ActorId, BeliefId, ContainerId, ContentManifestId, ContradictionId, DoorId,
    EpistemicProjectionVersion, EventId, ItemId, ObservationId, PlaceId, SchemaVersion,
    WorkplaceId,
};
use crate::location::Location;
use crate::time::SimTick;
use crate::view_models::{
    DebugBeliefEntry, DebugBeliefsView, DebugContradictionEntry, DebugEpistemicsView,
    DebugHolderBeliefs, DebugObservationEntry, DebugObservationsView,
};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProjectionEventRange {
    pub first_event_id: Option<EventId>,
    pub last_event_id: Option<EventId>,
    pub event_count: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NotebookEntry {
    pub actor_id: ActorId,
    pub source_belief_id: Option<BeliefId>,
    pub source_observation_id: Option<ObservationId>,
    pub summary: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ActorKnownProjectionSource {
    RoleAssignmentNotice,
    StartingBelief,
    CurrentPlace,
    CarriedItem,
    VisibleExit,
    VisibleFoodSupply,
    VisibleDoor,
    VisibleContainer,
    VisibleItem,
    VisibleActor,
    VisibleSleepAffordance,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ActorKnownProjectionRecord {
    CurrentPlace {
        actor_id: ActorId,
        place_id: PlaceId,
        display_label: String,
        source: ActorKnownProjectionSource,
        source_event_id: EventId,
        source_tick: SimTick,
    },
    CarriedItem {
        actor_id: ActorId,
        item_id: ItemId,
        place_id: PlaceId,
        source_location: Location,
        portable: bool,
        source: ActorKnownProjectionSource,
        source_event_id: EventId,
        source_tick: SimTick,
    },
    Route {
        actor_id: ActorId,
        from_place_id: PlaceId,
        to_place_id: PlaceId,
        source: ActorKnownProjectionSource,
        source_event_id: EventId,
        source_tick: SimTick,
    },
    FoodSource {
        actor_id: ActorId,
        food_source_id: String,
        place_id: Option<PlaceId>,
        believed_servings: Option<u32>,
        source: ActorKnownProjectionSource,
        source_event_id: EventId,
        source_tick: SimTick,
    },
    SleepPlace {
        actor_id: ActorId,
        place_id: PlaceId,
        sleep_affordance_id: Option<String>,
        source: ActorKnownProjectionSource,
        source_event_id: EventId,
        source_tick: SimTick,
    },
    Workplace {
        actor_id: ActorId,
        workplace_id: WorkplaceId,
        place_id: PlaceId,
        believed_access_open: bool,
        source: ActorKnownProjectionSource,
        source_event_id: EventId,
        source_tick: SimTick,
    },
    LocalDoor {
        actor_id: ActorId,
        door_id: DoorId,
        place_id: PlaceId,
        endpoint_a: PlaceId,
        endpoint_b: PlaceId,
        is_open: bool,
        is_locked: bool,
        blocks_movement_when_closed: bool,
        source: ActorKnownProjectionSource,
        source_event_id: EventId,
        source_tick: SimTick,
    },
    LocalContainer {
        actor_id: ActorId,
        container_id: ContainerId,
        place_id: PlaceId,
        is_open: bool,
        is_locked: bool,
        source: ActorKnownProjectionSource,
        source_event_id: EventId,
        source_tick: SimTick,
    },
    LocalItem {
        actor_id: ActorId,
        item_id: ItemId,
        place_id: PlaceId,
        source_location: Location,
        portable: bool,
        source: ActorKnownProjectionSource,
        source_event_id: EventId,
        source_tick: SimTick,
    },
    LocalActor {
        actor_id: ActorId,
        observed_actor_id: ActorId,
        place_id: PlaceId,
        source: ActorKnownProjectionSource,
        source_event_id: EventId,
        source_tick: SimTick,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActorKnownProjectionFreshness {
    CurrentlyPerceived,
    Remembered,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActorKnownProjectionPolicy {
    ReclassifyWhenStale,
    SupersedeNewestBySubject,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActorKnownProjectionEmbodiedScope {
    LatestCurrentPlaceOnly,
    CurrentPlaceOnly,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActorKnownProjectionAccessibilityScope {
    None,
    FromAnyPlace,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActorKnownProjectionKindPolicy {
    classification: ActorKnownProjectionPolicy,
    embodied_scope: ActorKnownProjectionEmbodiedScope,
    accessibility_scope: ActorKnownProjectionAccessibilityScope,
}

impl ActorKnownProjectionKindPolicy {
    pub fn classification(&self) -> ActorKnownProjectionPolicy {
        self.classification
    }

    pub fn embodied_scope(&self) -> ActorKnownProjectionEmbodiedScope {
        self.embodied_scope
    }

    pub fn accessibility_scope(&self) -> ActorKnownProjectionAccessibilityScope {
        self.accessibility_scope
    }

    pub fn includes_in_embodied_context(
        &self,
        current_place_record: bool,
        latest_current_place_record: bool,
    ) -> bool {
        match self.embodied_scope {
            ActorKnownProjectionEmbodiedScope::LatestCurrentPlaceOnly => {
                latest_current_place_record
            }
            ActorKnownProjectionEmbodiedScope::CurrentPlaceOnly => current_place_record,
        }
    }

    fn includes_classified_record(
        &self,
        record: &ActorKnownProjectionRecord,
        selected_workplaces: &BTreeMap<WorkplaceId, &ActorKnownProjectionRecord>,
    ) -> bool {
        match self.classification {
            ActorKnownProjectionPolicy::ReclassifyWhenStale => true,
            ActorKnownProjectionPolicy::SupersedeNewestBySubject => {
                let workplace_id = record.supersede_workplace_subject().unwrap_or_else(|| {
                    panic!(
                        "actor-known projection kind with SupersedeNewestBySubject lacks a subject extractor: {}",
                        record.kind()
                    )
                });
                selected_workplaces
                    .get(workplace_id)
                    .is_some_and(|selected| *selected == record)
            }
        }
    }

    fn freshness(
        &self,
        record: &ActorKnownProjectionRecord,
        latest_current_place_record: bool,
        context_tick: SimTick,
    ) -> ActorKnownProjectionFreshness {
        match self.classification {
            ActorKnownProjectionPolicy::ReclassifyWhenStale => {
                reclassifying_record_freshness(record, latest_current_place_record, context_tick)
            }
            ActorKnownProjectionPolicy::SupersedeNewestBySubject => {
                ActorKnownProjectionFreshness::Remembered
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClassifiedActorKnownProjectionRecord<'a> {
    record: &'a ActorKnownProjectionRecord,
    freshness: ActorKnownProjectionFreshness,
    current_place_record: bool,
    latest_current_place_record: bool,
}

impl<'a> ClassifiedActorKnownProjectionRecord<'a> {
    pub fn record(&self) -> &'a ActorKnownProjectionRecord {
        self.record
    }

    pub fn freshness(&self) -> ActorKnownProjectionFreshness {
        self.freshness
    }

    pub fn source_tick(&self) -> SimTick {
        self.record.source_tick()
    }

    pub fn is_latest_current_place_record(&self) -> bool {
        self.latest_current_place_record
    }

    pub fn is_current_place_record(&self) -> bool {
        self.current_place_record
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EpistemicProjection {
    observations_by_id: BTreeMap<ObservationId, Observation>,
    observations_by_actor: BTreeMap<ActorId, BTreeSet<ObservationId>>,
    beliefs_by_id: BTreeMap<BeliefId, Belief>,
    beliefs_by_holder: BTreeMap<ActorId, BTreeSet<BeliefId>>,
    contradictions_by_id: BTreeMap<ContradictionId, Contradiction>,
    contradictions_by_holder: BTreeMap<ActorId, BTreeSet<ContradictionId>>,
    notebook_entries_by_actor: BTreeMap<ActorId, BTreeSet<NotebookEntry>>,
    actor_known_records_by_actor: BTreeMap<ActorId, BTreeSet<ActorKnownProjectionRecord>>,
    projection_version: EpistemicProjectionVersion,
    projection_schema_version: SchemaVersion,
    event_range: ProjectionEventRange,
    content_manifest_id: ContentManifestId,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EpistemicProjectionChecksum(String);

impl EpistemicProjectionChecksum {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn from_canonical_lines(lines: &[String]) -> Self {
        let mut hash = 0xcbf2_9ce4_8422_2325_u64;
        for line in lines {
            for byte in line.as_bytes().iter().copied().chain([b'\n']) {
                hash ^= u64::from(byte);
                hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
            }
        }
        Self(format!("twe1-{hash:016x}"))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EpistemicProjectionChecksumReport {
    pub checksum: EpistemicProjectionChecksum,
    pub canonical_input: Vec<String>,
}

impl EpistemicProjection {
    pub fn new(content_manifest_id: ContentManifestId) -> Self {
        Self {
            observations_by_id: BTreeMap::new(),
            observations_by_actor: BTreeMap::new(),
            beliefs_by_id: BTreeMap::new(),
            beliefs_by_holder: BTreeMap::new(),
            contradictions_by_id: BTreeMap::new(),
            contradictions_by_holder: BTreeMap::new(),
            notebook_entries_by_actor: BTreeMap::new(),
            actor_known_records_by_actor: BTreeMap::new(),
            projection_version: EpistemicProjectionVersion::new("epistemic_projection_v1").unwrap(),
            projection_schema_version: SchemaVersion::new(EPISTEMIC_RECORD_SCHEMA_V1).unwrap(),
            event_range: ProjectionEventRange::default(),
            content_manifest_id,
        }
    }

    pub fn from_initial_beliefs(
        content_manifest_id: ContentManifestId,
        beliefs: impl IntoIterator<Item = Belief>,
    ) -> Self {
        let mut projection = Self::new(content_manifest_id);
        for belief in beliefs {
            projection.insert_belief(belief);
        }
        projection
    }

    pub fn record_applied_event(&mut self, event_id: EventId) {
        self.event_range.event_count += 1;
        if self.event_range.first_event_id.is_none() {
            self.event_range.first_event_id = Some(event_id.clone());
        }
        self.event_range.last_event_id = Some(event_id);
    }

    pub(crate) fn insert_observation(&mut self, observation: Observation) {
        let observation_id = observation.observation_id().clone();
        let actor_id = observation.observer_actor_id().clone();
        for record in actor_known_records_from_observation(&observation) {
            self.actor_known_records_by_actor
                .entry(actor_id.clone())
                .or_default()
                .insert(record);
        }
        self.observations_by_actor
            .entry(actor_id)
            .or_default()
            .insert(observation_id.clone());
        self.observations_by_id.insert(observation_id, observation);
    }

    pub(crate) fn insert_belief(&mut self, belief: Belief) {
        let belief_id = belief.belief_id().clone();
        if let HolderKind::Actor(actor_id) = &belief.holder() {
            self.beliefs_by_holder
                .entry(actor_id.clone())
                .or_default()
                .insert(belief_id.clone());
        }
        self.beliefs_by_id.insert(belief_id, belief);
    }

    pub(crate) fn insert_role_assignment_notice(
        &mut self,
        actor_id: ActorId,
        workplace_id: WorkplaceId,
        place_id: PlaceId,
        believed_access_open: bool,
        source_event_id: EventId,
        source_tick: SimTick,
    ) {
        self.actor_known_records_by_actor
            .entry(actor_id.clone())
            .or_default()
            .insert(ActorKnownProjectionRecord::Workplace {
                actor_id,
                workplace_id,
                place_id,
                believed_access_open,
                source: ActorKnownProjectionSource::RoleAssignmentNotice,
                source_event_id,
                source_tick,
            });
    }

    pub(crate) fn insert_starting_belief(
        &mut self,
        actor_id: ActorId,
        belief_kind: &str,
        subject_id: &str,
        value: &str,
        source_event_id: EventId,
        source_tick: SimTick,
    ) {
        let Some(record) = actor_known_record_from_starting_belief(
            actor_id,
            belief_kind,
            subject_id,
            value,
            source_event_id,
            source_tick,
        ) else {
            return;
        };
        let actor_id = record.actor_id().clone();
        self.actor_known_records_by_actor
            .entry(actor_id)
            .or_default()
            .insert(record);
    }

    pub(crate) fn insert_contradiction(&mut self, contradiction: Contradiction) {
        let contradiction_id = contradiction.contradiction_id().clone();
        let holder_actor_id = contradiction.holder_actor_id().clone();
        self.contradictions_by_holder
            .entry(holder_actor_id)
            .or_default()
            .insert(contradiction_id.clone());
        self.contradictions_by_id
            .insert(contradiction_id, contradiction);
    }

    pub(crate) fn observation(&self, observation_id: &ObservationId) -> Option<&Observation> {
        self.observations_by_id.get(observation_id)
    }

    pub fn has_belief(&self, belief_id: &BeliefId) -> bool {
        self.beliefs_by_id.contains_key(belief_id)
    }

    pub fn is_empty(&self) -> bool {
        self.observations_by_id.is_empty()
            && self.beliefs_by_id.is_empty()
            && self.contradictions_by_id.is_empty()
            && self.notebook_entries_by_actor.is_empty()
            && self.actor_known_records_by_actor.is_empty()
    }

    pub fn projection_version(&self) -> &EpistemicProjectionVersion {
        &self.projection_version
    }

    pub fn observations_for_context(&self, context: &KnowledgeContext) -> Vec<&Observation> {
        self.observations_by_actor
            .get(context.viewer_actor_id())
            .into_iter()
            .flat_map(|ids| ids.iter())
            .filter_map(|id| self.observations_by_id.get(id))
            .filter(|observation| context.permits_scope(observation.privacy_scope()))
            .collect()
    }

    pub fn beliefs_for_context(&self, context: &KnowledgeContext) -> Vec<&Belief> {
        match context.mode() {
            ViewMode::Debug => self
                .beliefs_by_id
                .values()
                .filter(|belief| context.permits_scope(belief.privacy_scope()))
                .collect(),
            ViewMode::Embodied => self
                .beliefs_by_holder
                .get(context.viewer_actor_id())
                .into_iter()
                .flat_map(|ids| ids.iter())
                .filter_map(|id| self.beliefs_by_id.get(id))
                .filter(|belief| context.permits_scope(belief.privacy_scope()))
                .collect(),
        }
    }

    pub fn contradictions_for_context(&self, context: &KnowledgeContext) -> Vec<&Contradiction> {
        match context.mode() {
            ViewMode::Debug => self.contradictions_by_id.values().collect(),
            ViewMode::Embodied => self
                .contradictions_by_holder
                .get(context.viewer_actor_id())
                .into_iter()
                .flat_map(|ids| ids.iter())
                .filter_map(|id| self.contradictions_by_id.get(id))
                .collect(),
        }
    }

    pub fn notebook_entries_for_context(&self, context: &KnowledgeContext) -> Vec<&NotebookEntry> {
        match context.mode() {
            ViewMode::Debug => self
                .notebook_entries_by_actor
                .values()
                .flat_map(|entries| entries.iter())
                .collect(),
            ViewMode::Embodied => self
                .notebook_entries_by_actor
                .get(context.viewer_actor_id())
                .into_iter()
                .flat_map(|entries| entries.iter())
                .collect(),
        }
    }

    pub fn classified_actor_known_records_for_context(
        &self,
        context: &KnowledgeContext,
        current_place_id: &PlaceId,
    ) -> Vec<ClassifiedActorKnownProjectionRecord<'_>> {
        let latest_current_place_tick = self
            .observations_for_context(context)
            .into_iter()
            .filter(|observation| observation.observer_place_id() == current_place_id)
            .filter(|observation| observation.observed_tick() <= context.current_tick())
            .map(|observation| observation.observed_tick())
            .max();

        let selected_workplaces = newest_workplace_records(
            self.actor_known_records_by_actor
                .get(context.viewer_actor_id()),
        );

        self.actor_known_records_by_actor
            .get(context.viewer_actor_id())
            .into_iter()
            .flat_map(|records| records.iter())
            .filter(|record| {
                record
                    .policy()
                    .includes_classified_record(record, &selected_workplaces)
            })
            .map(|record| {
                let current_place_record = record.relevant_place_id() == current_place_id;
                let latest_current_place_record = is_latest_current_place_record(
                    record,
                    current_place_id,
                    latest_current_place_tick,
                );
                ClassifiedActorKnownProjectionRecord {
                    record,
                    freshness: record.policy().freshness(
                        record,
                        latest_current_place_record,
                        context.current_tick(),
                    ),
                    current_place_record,
                    latest_current_place_record,
                }
            })
            .collect()
    }

    pub fn debug_epistemics_view(&self) -> DebugEpistemicsView {
        let observations = self
            .observations_by_id
            .values()
            .map(debug_observation_entry)
            .collect();
        let contradictions = self
            .contradictions_by_id
            .values()
            .map(debug_contradiction_entry)
            .collect();
        let beliefs_by_holder = self
            .beliefs_by_holder
            .iter()
            .map(|(holder_actor_id, belief_ids)| DebugHolderBeliefs {
                holder_actor_id: holder_actor_id.clone(),
                beliefs: belief_ids
                    .iter()
                    .filter_map(|belief_id| self.beliefs_by_id.get(belief_id))
                    .map(debug_belief_entry)
                    .collect(),
            })
            .collect();
        let checksum = self.compute_checksum().checksum;

        DebugEpistemicsView::new(
            "debug",
            observations,
            beliefs_by_holder,
            contradictions,
            Vec::new(),
            format!(
                "{} checksum={}",
                self.projection_version.as_str(),
                checksum.as_str()
            ),
        )
    }

    pub fn debug_beliefs_view(&self, actor_id: ActorId) -> DebugBeliefsView {
        let beliefs = self
            .beliefs_by_holder
            .get(&actor_id)
            .into_iter()
            .flat_map(|ids| ids.iter())
            .filter_map(|belief_id| self.beliefs_by_id.get(belief_id))
            .map(debug_belief_entry)
            .collect();
        DebugBeliefsView::new(actor_id, beliefs)
    }

    pub fn debug_observations_view(&self, actor_id: ActorId) -> DebugObservationsView {
        let observations = self
            .observations_by_actor
            .get(&actor_id)
            .into_iter()
            .flat_map(|ids| ids.iter())
            .filter_map(|observation_id| self.observations_by_id.get(observation_id))
            .map(debug_observation_entry)
            .collect();
        DebugObservationsView::new(actor_id, observations)
    }

    pub fn compute_checksum(&self) -> EpistemicProjectionChecksumReport {
        let mut lines = vec![
            format!("projection_version={}", self.projection_version.as_str()),
            format!(
                "projection_schema_version={}",
                self.projection_schema_version.as_str()
            ),
            format!("content_manifest_id={}", self.content_manifest_id.as_str()),
            format!(
                "event_range={}:{}:{}",
                self.event_range
                    .first_event_id
                    .as_ref()
                    .map(|id| id.as_str())
                    .unwrap_or(""),
                self.event_range
                    .last_event_id
                    .as_ref()
                    .map(|id| id.as_str())
                    .unwrap_or(""),
                self.event_range.event_count
            ),
        ];

        for (id, observation) in &self.observations_by_id {
            lines.push(format!(
                "observation|id={}|actor={}|channel={}|tick={}|place={}|confidence={}|source={:?}",
                id.as_str(),
                observation.observer_actor_id().as_str(),
                observation.channel().stable_id(),
                observation.observed_tick().value(),
                observation.observer_place_id().as_str(),
                observation.confidence().serialize_canonical(),
                observation.source(),
            ));
        }

        for (id, belief) in &self.beliefs_by_id {
            lines.push(format!(
                "belief|id={}|holder={}|stance={}|confidence={}|proposition={}|source={:?}",
                id.as_str(),
                holder_key(belief.holder()),
                belief.stance().stable_id(),
                belief.confidence().serialize_canonical(),
                belief.proposition().serialize_canonical(),
                belief.source(),
            ));
        }

        for (id, contradiction) in &self.contradictions_by_id {
            lines.push(format!(
                "contradiction|id={}|holder={}|kind={}|belief={}|observation={}|expected={}|observed={}|tick={}",
                id.as_str(),
                contradiction.holder_actor_id().as_str(),
                contradiction.kind().stable_id(),
                contradiction.prior_expectation_belief_id().as_str(),
                contradiction.contradicting_observation_id().as_str(),
                contradiction.expected_proposition().serialize_canonical(),
                contradiction.observed_proposition().serialize_canonical(),
                contradiction.detected_tick().value(),
            ));
        }

        for (actor_id, entries) in &self.notebook_entries_by_actor {
            for entry in entries {
                lines.push(format!(
                    "notebook|actor={}|belief={}|observation={}|summary={}",
                    actor_id.as_str(),
                    entry
                        .source_belief_id
                        .as_ref()
                        .map(|id| id.as_str())
                        .unwrap_or(""),
                    entry
                        .source_observation_id
                        .as_ref()
                        .map(|id| id.as_str())
                        .unwrap_or(""),
                    entry.summary
                ));
            }
        }

        for (actor_id, records) in &self.actor_known_records_by_actor {
            for record in records {
                lines.push(format!(
                    "actor_known|actor={}|record={}",
                    actor_id.as_str(),
                    record.serialize_canonical()
                ));
            }
        }

        let checksum = EpistemicProjectionChecksum::from_canonical_lines(&lines);
        EpistemicProjectionChecksumReport {
            checksum,
            canonical_input: lines,
        }
    }
}

impl ActorKnownProjectionSource {
    pub fn source_label(&self) -> &'static str {
        match self {
            Self::RoleAssignmentNotice => "evented_role_assignment_notice",
            Self::StartingBelief => "evented_starting_belief",
            Self::CurrentPlace => "evented_perception:current_place",
            Self::CarriedItem => "evented_perception:carried_item",
            Self::VisibleExit => "evented_perception:visible_exit",
            Self::VisibleFoodSupply => "evented_perception:visible_food_supply",
            Self::VisibleDoor => "evented_perception:visible_door",
            Self::VisibleContainer => "evented_perception:visible_container",
            Self::VisibleItem => "evented_perception:visible_item",
            Self::VisibleActor => "evented_perception:visible_actor",
            Self::VisibleSleepAffordance => "evented_perception:visible_sleep_affordance",
        }
    }

    fn stable_id(&self) -> &'static str {
        match self {
            Self::RoleAssignmentNotice => "role_assignment_notice",
            Self::StartingBelief => "starting_belief",
            Self::CurrentPlace => "current_place",
            Self::CarriedItem => "carried_item",
            Self::VisibleExit => "visible_exit",
            Self::VisibleFoodSupply => "visible_food_supply",
            Self::VisibleDoor => "visible_door",
            Self::VisibleContainer => "visible_container",
            Self::VisibleItem => "visible_item",
            Self::VisibleActor => "visible_actor",
            Self::VisibleSleepAffordance => "visible_sleep_affordance",
        }
    }
}

impl ActorKnownProjectionRecord {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::CurrentPlace { .. } => "current_place",
            Self::CarriedItem { .. } => "carried_item",
            Self::Route { .. } => "route",
            Self::FoodSource { .. } => "food_source",
            Self::LocalActor { .. } => "local_actor",
            Self::LocalContainer { .. } => "local_container",
            Self::LocalDoor { .. } => "local_door",
            Self::LocalItem { .. } => "local_item",
            Self::SleepPlace { .. } => "sleep_place",
            Self::Workplace { .. } => "workplace",
        }
    }

    fn supersede_workplace_subject(&self) -> Option<&WorkplaceId> {
        match self {
            Self::Workplace { workplace_id, .. } => Some(workplace_id),
            Self::Route { .. }
            | Self::CurrentPlace { .. }
            | Self::CarriedItem { .. }
            | Self::FoodSource { .. }
            | Self::SleepPlace { .. }
            | Self::LocalDoor { .. }
            | Self::LocalContainer { .. }
            | Self::LocalItem { .. }
            | Self::LocalActor { .. } => None,
        }
    }

    pub fn actor_id(&self) -> &ActorId {
        match self {
            Self::Route { actor_id, .. }
            | Self::CurrentPlace { actor_id, .. }
            | Self::CarriedItem { actor_id, .. }
            | Self::FoodSource { actor_id, .. }
            | Self::LocalActor { actor_id, .. }
            | Self::LocalContainer { actor_id, .. }
            | Self::LocalDoor { actor_id, .. }
            | Self::LocalItem { actor_id, .. }
            | Self::SleepPlace { actor_id, .. }
            | Self::Workplace { actor_id, .. } => actor_id,
        }
    }

    pub fn source(&self) -> &ActorKnownProjectionSource {
        match self {
            Self::Route { source, .. }
            | Self::CurrentPlace { source, .. }
            | Self::CarriedItem { source, .. }
            | Self::FoodSource { source, .. }
            | Self::LocalActor { source, .. }
            | Self::LocalContainer { source, .. }
            | Self::LocalDoor { source, .. }
            | Self::LocalItem { source, .. }
            | Self::SleepPlace { source, .. }
            | Self::Workplace { source, .. } => source,
        }
    }

    pub fn source_event_id(&self) -> &EventId {
        match self {
            Self::Route {
                source_event_id, ..
            }
            | Self::CurrentPlace {
                source_event_id, ..
            }
            | Self::CarriedItem {
                source_event_id, ..
            }
            | Self::FoodSource {
                source_event_id, ..
            }
            | Self::LocalActor {
                source_event_id, ..
            }
            | Self::LocalContainer {
                source_event_id, ..
            }
            | Self::LocalDoor {
                source_event_id, ..
            }
            | Self::LocalItem {
                source_event_id, ..
            }
            | Self::SleepPlace {
                source_event_id, ..
            }
            | Self::Workplace {
                source_event_id, ..
            } => source_event_id,
        }
    }

    pub fn source_tick(&self) -> SimTick {
        match self {
            Self::Route { source_tick, .. }
            | Self::CurrentPlace { source_tick, .. }
            | Self::CarriedItem { source_tick, .. }
            | Self::FoodSource { source_tick, .. }
            | Self::LocalActor { source_tick, .. }
            | Self::LocalContainer { source_tick, .. }
            | Self::LocalDoor { source_tick, .. }
            | Self::LocalItem { source_tick, .. }
            | Self::SleepPlace { source_tick, .. }
            | Self::Workplace { source_tick, .. } => *source_tick,
        }
    }

    pub fn policy(&self) -> ActorKnownProjectionKindPolicy {
        actor_known_projection_kind_policy(self.kind())
    }

    fn relevant_place_id(&self) -> &PlaceId {
        match self {
            Self::CurrentPlace { place_id, .. } | Self::CarriedItem { place_id, .. } => place_id,
            Self::Route { from_place_id, .. } => from_place_id,
            Self::FoodSource {
                place_id: Some(place_id),
                ..
            } => place_id,
            Self::FoodSource { place_id: None, .. } => {
                panic!("projection food-source records must carry source place")
            }
            Self::SleepPlace { place_id, .. }
            | Self::Workplace { place_id, .. }
            | Self::LocalDoor { place_id, .. }
            | Self::LocalContainer { place_id, .. }
            | Self::LocalItem { place_id, .. }
            | Self::LocalActor { place_id, .. } => place_id,
        }
    }

    fn serialize_canonical(&self) -> String {
        match self {
            Self::CurrentPlace {
                place_id,
                display_label,
                source,
                source_event_id,
                source_tick,
                ..
            } => format!(
                "current_place|place={}|label={}|source={}|event={}|tick={}",
                place_id.as_str(),
                display_label,
                source.stable_id(),
                source_event_id.as_str(),
                source_tick.value()
            ),
            Self::CarriedItem {
                item_id,
                place_id,
                source_location,
                portable,
                source,
                source_event_id,
                source_tick,
                ..
            } => format!(
                "carried_item|id={}|place={}|location={}|portable={}|source={}|event={}|tick={}",
                item_id.as_str(),
                place_id.as_str(),
                location_key(source_location),
                portable,
                source.stable_id(),
                source_event_id.as_str(),
                source_tick.value()
            ),
            Self::Route {
                from_place_id,
                to_place_id,
                source,
                source_event_id,
                source_tick,
                ..
            } => format!(
                "route|from={}|to={}|source={}|event={}|tick={}",
                from_place_id.as_str(),
                to_place_id.as_str(),
                source.stable_id(),
                source_event_id.as_str(),
                source_tick.value()
            ),
            Self::FoodSource {
                food_source_id,
                place_id,
                believed_servings,
                source,
                source_event_id,
                source_tick,
                ..
            } => format!(
                "food|id={food_source_id}|place={}|servings={}|source={}|event={}|tick={}",
                place_id.as_ref().map(|id| id.as_str()).unwrap_or(""),
                believed_servings
                    .map(|servings| servings.to_string())
                    .unwrap_or_else(|| "-".to_string()),
                source.stable_id(),
                source_event_id.as_str(),
                source_tick.value()
            ),
            Self::SleepPlace {
                place_id,
                sleep_affordance_id,
                source,
                source_event_id,
                source_tick,
                ..
            } => format!(
                "sleep|place={}|affordance={}|source={}|event={}|tick={}",
                place_id.as_str(),
                sleep_affordance_id.as_deref().unwrap_or(""),
                source.stable_id(),
                source_event_id.as_str(),
                source_tick.value()
            ),
            Self::Workplace {
                workplace_id,
                place_id,
                believed_access_open,
                source,
                source_event_id,
                source_tick,
                ..
            } => format!(
                "workplace|id={}|place={}|access_open={}|source={}|event={}|tick={}",
                workplace_id.as_str(),
                place_id.as_str(),
                believed_access_open,
                source.stable_id(),
                source_event_id.as_str(),
                source_tick.value()
            ),
            Self::LocalDoor {
                door_id,
                place_id,
                endpoint_a,
                endpoint_b,
                is_open,
                is_locked,
                blocks_movement_when_closed,
                source,
                source_event_id,
                source_tick,
                ..
            } => format!(
                "local_door|id={}|place={}|a={}|b={}|open={}|locked={}|blocks={}|source={}|event={}|tick={}",
                door_id.as_str(),
                place_id.as_str(),
                endpoint_a.as_str(),
                endpoint_b.as_str(),
                is_open,
                is_locked,
                blocks_movement_when_closed,
                source.stable_id(),
                source_event_id.as_str(),
                source_tick.value()
            ),
            Self::LocalContainer {
                container_id,
                place_id,
                is_open,
                is_locked,
                source,
                source_event_id,
                source_tick,
                ..
            } => format!(
                "local_container|id={}|place={}|open={}|locked={}|source={}|event={}|tick={}",
                container_id.as_str(),
                place_id.as_str(),
                is_open,
                is_locked,
                source.stable_id(),
                source_event_id.as_str(),
                source_tick.value()
            ),
            Self::LocalItem {
                item_id,
                place_id,
                source_location,
                portable,
                source,
                source_event_id,
                source_tick,
                ..
            } => format!(
                "local_item|id={}|place={}|location={}|portable={}|source={}|event={}|tick={}",
                item_id.as_str(),
                place_id.as_str(),
                location_key(source_location),
                portable,
                source.stable_id(),
                source_event_id.as_str(),
                source_tick.value()
            ),
            Self::LocalActor {
                observed_actor_id,
                place_id,
                source,
                source_event_id,
                source_tick,
                ..
            } => format!(
                "local_actor|id={}|place={}|source={}|event={}|tick={}",
                observed_actor_id.as_str(),
                place_id.as_str(),
                source.stable_id(),
                source_event_id.as_str(),
                source_tick.value()
            ),
        }
    }
}

pub fn actor_known_projection_policy_kinds(
) -> BTreeMap<&'static str, ActorKnownProjectionKindPolicy> {
    BTreeMap::from([
        (
            "current_place",
            ActorKnownProjectionKindPolicy {
                classification: ActorKnownProjectionPolicy::ReclassifyWhenStale,
                embodied_scope: ActorKnownProjectionEmbodiedScope::LatestCurrentPlaceOnly,
                accessibility_scope: ActorKnownProjectionAccessibilityScope::None,
            },
        ),
        (
            "carried_item",
            ActorKnownProjectionKindPolicy {
                classification: ActorKnownProjectionPolicy::ReclassifyWhenStale,
                embodied_scope: ActorKnownProjectionEmbodiedScope::LatestCurrentPlaceOnly,
                accessibility_scope: ActorKnownProjectionAccessibilityScope::None,
            },
        ),
        (
            "route",
            ActorKnownProjectionKindPolicy {
                classification: ActorKnownProjectionPolicy::ReclassifyWhenStale,
                embodied_scope: ActorKnownProjectionEmbodiedScope::LatestCurrentPlaceOnly,
                accessibility_scope: ActorKnownProjectionAccessibilityScope::None,
            },
        ),
        (
            "food_source",
            ActorKnownProjectionKindPolicy {
                classification: ActorKnownProjectionPolicy::ReclassifyWhenStale,
                embodied_scope: ActorKnownProjectionEmbodiedScope::LatestCurrentPlaceOnly,
                accessibility_scope: ActorKnownProjectionAccessibilityScope::FromAnyPlace,
            },
        ),
        (
            "local_actor",
            ActorKnownProjectionKindPolicy {
                classification: ActorKnownProjectionPolicy::ReclassifyWhenStale,
                embodied_scope: ActorKnownProjectionEmbodiedScope::LatestCurrentPlaceOnly,
                accessibility_scope: ActorKnownProjectionAccessibilityScope::None,
            },
        ),
        (
            "local_container",
            ActorKnownProjectionKindPolicy {
                classification: ActorKnownProjectionPolicy::ReclassifyWhenStale,
                embodied_scope: ActorKnownProjectionEmbodiedScope::LatestCurrentPlaceOnly,
                accessibility_scope: ActorKnownProjectionAccessibilityScope::None,
            },
        ),
        (
            "local_door",
            ActorKnownProjectionKindPolicy {
                classification: ActorKnownProjectionPolicy::ReclassifyWhenStale,
                embodied_scope: ActorKnownProjectionEmbodiedScope::LatestCurrentPlaceOnly,
                accessibility_scope: ActorKnownProjectionAccessibilityScope::None,
            },
        ),
        (
            "local_item",
            ActorKnownProjectionKindPolicy {
                classification: ActorKnownProjectionPolicy::ReclassifyWhenStale,
                embodied_scope: ActorKnownProjectionEmbodiedScope::LatestCurrentPlaceOnly,
                accessibility_scope: ActorKnownProjectionAccessibilityScope::None,
            },
        ),
        (
            "sleep_place",
            ActorKnownProjectionKindPolicy {
                classification: ActorKnownProjectionPolicy::ReclassifyWhenStale,
                embodied_scope: ActorKnownProjectionEmbodiedScope::LatestCurrentPlaceOnly,
                accessibility_scope: ActorKnownProjectionAccessibilityScope::FromAnyPlace,
            },
        ),
        (
            "workplace",
            ActorKnownProjectionKindPolicy {
                classification: ActorKnownProjectionPolicy::SupersedeNewestBySubject,
                embodied_scope: ActorKnownProjectionEmbodiedScope::CurrentPlaceOnly,
                accessibility_scope: ActorKnownProjectionAccessibilityScope::FromAnyPlace,
            },
        ),
    ])
}

pub fn actor_known_projection_kind_policy(kind: &str) -> ActorKnownProjectionKindPolicy {
    let mut policies = actor_known_projection_policy_kinds();
    policies
        .remove(kind)
        .unwrap_or_else(|| panic!("unknown actor-known projection kind: {kind}"))
}

fn newest_workplace_records<'a>(
    records: Option<&'a BTreeSet<ActorKnownProjectionRecord>>,
) -> BTreeMap<WorkplaceId, &'a ActorKnownProjectionRecord> {
    let mut newest: BTreeMap<WorkplaceId, &'a ActorKnownProjectionRecord> = BTreeMap::new();
    for record in records.into_iter().flat_map(|records| records.iter()) {
        let ActorKnownProjectionRecord::Workplace { workplace_id, .. } = record else {
            continue;
        };
        if newest
            .get(workplace_id)
            .is_none_or(|previous| workplace_record_is_newer(record, previous))
        {
            newest.insert(workplace_id.clone(), record);
        }
    }
    newest
}

fn workplace_record_is_newer(
    candidate: &ActorKnownProjectionRecord,
    previous: &ActorKnownProjectionRecord,
) -> bool {
    candidate.source_tick() > previous.source_tick()
        || (candidate.source_tick() == previous.source_tick()
            && candidate.source_event_id() > previous.source_event_id())
}

fn reclassifying_record_freshness(
    record: &ActorKnownProjectionRecord,
    latest_current_place_record: bool,
    context_tick: SimTick,
) -> ActorKnownProjectionFreshness {
    if !matches!(
        record.source(),
        ActorKnownProjectionSource::VisibleExit
            | ActorKnownProjectionSource::VisibleFoodSupply
            | ActorKnownProjectionSource::VisibleDoor
            | ActorKnownProjectionSource::VisibleContainer
            | ActorKnownProjectionSource::VisibleItem
            | ActorKnownProjectionSource::VisibleActor
            | ActorKnownProjectionSource::VisibleSleepAffordance
    ) {
        return ActorKnownProjectionFreshness::Remembered;
    }
    if !latest_current_place_record || record.source_tick() != context_tick {
        return ActorKnownProjectionFreshness::Remembered;
    }
    ActorKnownProjectionFreshness::CurrentlyPerceived
}

fn location_key(location: &Location) -> String {
    match location {
        Location::AtPlace(place_id) => format!("place:{}", place_id.as_str()),
        Location::InContainer(container_id) => format!("container:{}", container_id.as_str()),
        Location::CarriedBy(actor_id) => format!("carried:{}", actor_id.as_str()),
    }
}

fn is_latest_current_place_record(
    record: &ActorKnownProjectionRecord,
    current_place_id: &PlaceId,
    latest_current_place_tick: Option<SimTick>,
) -> bool {
    record.relevant_place_id() == current_place_id
        && Some(record.source_tick()) == latest_current_place_tick
}

fn actor_known_records_from_observation(
    observation: &Observation,
) -> Vec<ActorKnownProjectionRecord> {
    let Some(source_event_id) = source_event_id(observation.source()) else {
        return Vec::new();
    };
    let actor_id = observation.observer_actor_id().clone();
    match observation_payload_value(observation, "perceived_kind") {
        Some("current_place") => {
            let Some(place_id) = observation_payload_value(observation, "target_id")
                .and_then(|value| PlaceId::new(value).ok())
            else {
                return Vec::new();
            };
            let Some(display_label) = observation_payload_value(observation, "display_label")
            else {
                return Vec::new();
            };
            vec![ActorKnownProjectionRecord::CurrentPlace {
                actor_id,
                place_id,
                display_label: display_label.to_string(),
                source: ActorKnownProjectionSource::CurrentPlace,
                source_event_id,
                source_tick: observation.observed_tick(),
            }]
        }
        Some("carried_item") => {
            let Some(item_id) = observation_payload_value(observation, "target_id")
                .and_then(|value| ItemId::new(value).ok())
            else {
                return Vec::new();
            };
            let Some(source_location) = observation_item_location(observation) else {
                return Vec::new();
            };
            let Some(portable) = observation_payload_bool(observation, "portable") else {
                return Vec::new();
            };
            vec![ActorKnownProjectionRecord::CarriedItem {
                actor_id,
                item_id,
                place_id: observation.observer_place_id().clone(),
                source_location,
                portable,
                source: ActorKnownProjectionSource::CarriedItem,
                source_event_id,
                source_tick: observation.observed_tick(),
            }]
        }
        Some("visible_exit") => {
            let Some(from_place_id) = observation_payload_value(observation, "subject_id")
                .and_then(|value| PlaceId::new(value).ok())
            else {
                return Vec::new();
            };
            let Some(to_place_id) = observation_payload_value(observation, "target_id")
                .and_then(|value| PlaceId::new(value).ok())
            else {
                return Vec::new();
            };
            vec![ActorKnownProjectionRecord::Route {
                actor_id,
                from_place_id,
                to_place_id,
                source: ActorKnownProjectionSource::VisibleExit,
                source_event_id,
                source_tick: observation.observed_tick(),
            }]
        }
        Some("visible_food_supply") => {
            let Some(food_source_id) = observation_payload_value(observation, "target_id") else {
                return Vec::new();
            };
            vec![ActorKnownProjectionRecord::FoodSource {
                actor_id,
                food_source_id: food_source_id.to_string(),
                place_id: Some(observation.observer_place_id().clone()),
                believed_servings: observation_payload_value(observation, "servings")
                    .and_then(|value| value.parse::<u32>().ok()),
                source: ActorKnownProjectionSource::VisibleFoodSupply,
                source_event_id,
                source_tick: observation.observed_tick(),
            }]
        }
        Some("visible_door") => {
            let Some(door_id) = observation_payload_value(observation, "target_id")
                .and_then(|value| DoorId::new(value).ok())
            else {
                return Vec::new();
            };
            let Some(endpoint_a) = observation_payload_value(observation, "endpoint_a")
                .and_then(|value| PlaceId::new(value).ok())
            else {
                return Vec::new();
            };
            let Some(endpoint_b) = observation_payload_value(observation, "endpoint_b")
                .and_then(|value| PlaceId::new(value).ok())
            else {
                return Vec::new();
            };
            let Some(is_open) = observation_payload_bool(observation, "is_open") else {
                return Vec::new();
            };
            let Some(is_locked) = observation_payload_bool(observation, "is_locked") else {
                return Vec::new();
            };
            let Some(blocks_movement_when_closed) =
                observation_payload_bool(observation, "blocks_movement_when_closed")
            else {
                return Vec::new();
            };
            vec![ActorKnownProjectionRecord::LocalDoor {
                actor_id,
                door_id,
                place_id: observation.observer_place_id().clone(),
                endpoint_a,
                endpoint_b,
                is_open,
                is_locked,
                blocks_movement_when_closed,
                source: ActorKnownProjectionSource::VisibleDoor,
                source_event_id,
                source_tick: observation.observed_tick(),
            }]
        }
        Some("visible_container") => {
            let Some(container_id) = observation_payload_value(observation, "target_id")
                .and_then(|value| ContainerId::new(value).ok())
            else {
                return Vec::new();
            };
            let Some(is_open) = observation_payload_bool(observation, "is_open") else {
                return Vec::new();
            };
            let Some(is_locked) = observation_payload_bool(observation, "is_locked") else {
                return Vec::new();
            };
            vec![ActorKnownProjectionRecord::LocalContainer {
                actor_id,
                container_id,
                place_id: observation.observer_place_id().clone(),
                is_open,
                is_locked,
                source: ActorKnownProjectionSource::VisibleContainer,
                source_event_id,
                source_tick: observation.observed_tick(),
            }]
        }
        Some("visible_item") => {
            let Some(item_id) = observation_payload_value(observation, "target_id")
                .and_then(|value| ItemId::new(value).ok())
            else {
                return Vec::new();
            };
            let Some(source_location) = observation_item_location(observation) else {
                return Vec::new();
            };
            let Some(portable) = observation_payload_bool(observation, "portable") else {
                return Vec::new();
            };
            vec![ActorKnownProjectionRecord::LocalItem {
                actor_id,
                item_id,
                place_id: observation.observer_place_id().clone(),
                source_location,
                portable,
                source: ActorKnownProjectionSource::VisibleItem,
                source_event_id,
                source_tick: observation.observed_tick(),
            }]
        }
        Some("visible_actor") => {
            let Some(observed_actor_id) = observation_payload_value(observation, "target_id")
                .and_then(|value| ActorId::new(value).ok())
            else {
                return Vec::new();
            };
            vec![ActorKnownProjectionRecord::LocalActor {
                actor_id,
                observed_actor_id,
                place_id: observation.observer_place_id().clone(),
                source: ActorKnownProjectionSource::VisibleActor,
                source_event_id,
                source_tick: observation.observed_tick(),
            }]
        }
        Some("visible_sleep_affordance") => {
            let Some(place_id) = observation_payload_value(observation, "place_id")
                .and_then(|value| PlaceId::new(value).ok())
            else {
                return Vec::new();
            };
            vec![ActorKnownProjectionRecord::SleepPlace {
                actor_id,
                place_id,
                sleep_affordance_id: observation_payload_value(observation, "target_id")
                    .map(ToString::to_string),
                source: ActorKnownProjectionSource::VisibleSleepAffordance,
                source_event_id,
                source_tick: observation.observed_tick(),
            }]
        }
        _ => Vec::new(),
    }
}

fn actor_known_record_from_starting_belief(
    actor_id: ActorId,
    belief_kind: &str,
    subject_id: &str,
    value: &str,
    source_event_id: EventId,
    source_tick: SimTick,
) -> Option<ActorKnownProjectionRecord> {
    match belief_kind {
        "sleep_place" => Some(ActorKnownProjectionRecord::SleepPlace {
            actor_id,
            place_id: PlaceId::new(value).ok()?,
            sleep_affordance_id: Some(subject_id.to_string()),
            source: ActorKnownProjectionSource::StartingBelief,
            source_event_id,
            source_tick,
        }),
        "household_food_source" => {
            let place_value = value.strip_prefix("place:")?;
            let place_id = PlaceId::new(place_value).ok()?;
            Some(ActorKnownProjectionRecord::FoodSource {
                actor_id,
                food_source_id: subject_id.to_string(),
                place_id: Some(place_id),
                believed_servings: None,
                source: ActorKnownProjectionSource::StartingBelief,
                source_event_id,
                source_tick,
            })
        }
        _ => None,
    }
}

fn source_event_id(source: &SourceRef) -> Option<EventId> {
    match source {
        SourceRef::Event(event_id) => Some(event_id.clone()),
        SourceRef::Action(_) | SourceRef::Cause(_) => None,
    }
}

fn observation_payload_value<'a>(observation: &'a Observation, key: &str) -> Option<&'a str> {
    observation
        .raw_payload()
        .iter()
        .find(|field| field.key == key)
        .map(|field| field.value.as_str())
}

fn observation_payload_bool(observation: &Observation, key: &str) -> Option<bool> {
    observation_payload_value(observation, key).and_then(|value| value.parse::<bool>().ok())
}

fn observation_item_location(observation: &Observation) -> Option<Location> {
    match observation_payload_value(observation, "item_source_kind")? {
        "place" => observation_payload_value(observation, "item_source_place_id")
            .and_then(|value| PlaceId::new(value).ok())
            .map(Location::AtPlace),
        "container" => observation_payload_value(observation, "item_source_container_id")
            .and_then(|value| ContainerId::new(value).ok())
            .map(Location::InContainer),
        "carried" => observation_payload_value(observation, "item_source_actor_id")
            .and_then(|value| ActorId::new(value).ok())
            .map(Location::CarriedBy),
        _ => None,
    }
}

fn holder_key(holder: &HolderKind) -> String {
    match holder {
        HolderKind::Actor(actor_id) => format!("actor:{}", actor_id.as_str()),
        HolderKind::InstitutionPlaceholder(id) => format!("institution_placeholder:{id}"),
    }
}

fn debug_belief_entry(belief: &Belief) -> DebugBeliefEntry {
    DebugBeliefEntry {
        belief_id: belief.belief_id().as_str().to_string(),
        proposition: belief.proposition().render(),
        stance: belief.stance().stable_id().to_string(),
        confidence: belief.confidence().serialize_canonical(),
        source: source_summary(belief.source()),
    }
}

fn debug_observation_entry(observation: &Observation) -> DebugObservationEntry {
    DebugObservationEntry {
        observation_id: observation.observation_id().as_str().to_string(),
        observer_actor_id: observation.observer_actor_id().clone(),
        channel: observation.channel().stable_id().to_string(),
        confidence: observation.confidence().serialize_canonical(),
        source: source_summary(observation.source()),
    }
}

fn debug_contradiction_entry(contradiction: &Contradiction) -> DebugContradictionEntry {
    DebugContradictionEntry {
        contradiction_id: contradiction.contradiction_id().as_str().to_string(),
        holder_actor_id: contradiction.holder_actor_id().clone(),
        expectation_belief_id: contradiction
            .prior_expectation_belief_id()
            .as_str()
            .to_string(),
        observation_id: contradiction
            .contradicting_observation_id()
            .as_str()
            .to_string(),
        summary: format!(
            "{} -> {}",
            contradiction.expected_proposition().render(),
            contradiction.observed_proposition().render()
        ),
    }
}

fn source_summary(source: &SourceRef) -> String {
    match source {
        SourceRef::Event(event_id) => format!("event:{}", event_id.as_str()),
        SourceRef::Action(action_id) => format!("action:{}", action_id.as_str()),
        SourceRef::Cause(cause) => format!("cause:{cause:?}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::{
        current_place_knowledge_context, NoHumanActorKnownSurfaceBuilder,
        NoHumanActorKnownSurfaceRequest,
    };
    use crate::epistemics::belief::{Belief, HolderKind, Stance};
    use crate::epistemics::contradiction::{Contradiction, ContradictionKind};
    use crate::epistemics::knowledge_context::KnowledgeContext;
    use crate::epistemics::observation::{
        Channel, Confidence, Observation, ObservationSubject, ObservationTarget, SourceRef,
    };
    use crate::epistemics::proposition::Proposition;
    use crate::events::PayloadField;
    use crate::ids::{
        ActionId, ContainerId, ContradictionId, DoorId, EventId, ItemId, ObservationId,
    };
    use crate::location::Location;
    use crate::state::{ActorBody, AgentState, NeedModelState, PhysicalState};
    use crate::time::SimTick;

    fn actor_id(value: &str) -> ActorId {
        ActorId::new(value).unwrap()
    }

    fn belief_id(value: &str) -> BeliefId {
        BeliefId::new(value).unwrap()
    }

    fn container_id(value: &str) -> ContainerId {
        ContainerId::new(value).unwrap()
    }

    fn event_id(value: &str) -> EventId {
        EventId::new(value).unwrap()
    }

    fn place_id(value: &str) -> PlaceId {
        PlaceId::new(value).unwrap()
    }

    fn workplace_id(value: &str) -> WorkplaceId {
        WorkplaceId::new(value).unwrap()
    }

    fn item_id(value: &str) -> ItemId {
        ItemId::new(value).unwrap()
    }

    fn manifest_id() -> ContentManifestId {
        ContentManifestId::new("phase2a_manifest").unwrap()
    }

    fn proposition(item: &str, container: &str) -> Proposition {
        Proposition::ItemMissingFromExpectedLocation {
            item_id: item_id(item),
            expected_location: Location::InContainer(container_id(container)),
        }
    }

    fn belief(id: &str, actor: &str, item: &str) -> Belief {
        Belief::new(
            belief_id(id),
            HolderKind::Actor(actor_id(actor)),
            proposition(item, "strongbox_tomas"),
            Stance::BelievesTrue,
            Confidence::new(900).unwrap(),
            SourceRef::Event(event_id(id)),
            SimTick::new(3),
        )
    }

    fn visible_observation(kind: &str, target_id: &str, payload: Vec<PayloadField>) -> Observation {
        let mut raw_payload = vec![
            PayloadField::new("perceived_kind", kind),
            PayloadField::new("target_id", target_id),
        ];
        raw_payload.extend(payload);
        Observation::new(
            ObservationId::new(format!("observation_{kind}_{target_id}")).unwrap(),
            actor_id("actor_tomas"),
            Channel::DirectSight,
            SimTick::new(4),
            place_id("home_tomas"),
            ObservationSubject::Place(place_id("home_tomas")),
            ObservationTarget::Place(place_id("home_tomas")),
            Confidence::new(900).unwrap(),
            SourceRef::Event(event_id("event_visible_observation")),
        )
        .with_raw_payload(raw_payload)
    }

    fn contradiction() -> Contradiction {
        Contradiction::new(
            ContradictionId::new("contradiction_missing_coin").unwrap(),
            actor_id("actor_tomas"),
            ContradictionKind::ExpectedItemAbsentFromContainer,
            belief_id("belief_expected_coin"),
            ObservationId::new("observation_missing_coin").unwrap(),
            Proposition::ItemLocatedInContainer {
                item_id: item_id("coin_stack_01"),
                container_id: container_id("strongbox_tomas"),
            },
            proposition("coin_stack_01", "strongbox_tomas"),
            SimTick::new(5),
        )
    }

    #[test]
    fn checksum_and_projection_bookkeeping_are_canonical() {
        let checksum = EpistemicProjectionChecksum::from_canonical_lines(&["alpha".to_string()]);
        assert_eq!(checksum.as_str(), "twe1-bbd23ea491ed9813");

        let changed = EpistemicProjectionChecksum::from_canonical_lines(&["alphb".to_string()]);
        assert_ne!(checksum, changed);

        let mut projection = EpistemicProjection::new(manifest_id());
        assert_eq!(projection.event_range.event_count, 0);
        projection.record_applied_event(event_id("event_first"));
        projection.record_applied_event(event_id("event_second"));
        assert_eq!(projection.event_range.event_count, 2);
        assert_eq!(
            projection
                .event_range
                .first_event_id
                .as_ref()
                .map(EventId::as_str),
            Some("event_first")
        );
        assert_eq!(
            projection
                .event_range
                .last_event_id
                .as_ref()
                .map(EventId::as_str),
            Some("event_second")
        );
    }

    #[test]
    fn projection_presence_and_notebook_filters_are_observable() {
        let mut projection = EpistemicProjection::new(manifest_id());
        assert!(projection.is_empty());
        assert!(!projection.has_belief(&belief_id("belief_tomas_missing_coin")));

        projection.insert_belief(belief(
            "belief_tomas_missing_coin",
            "actor_tomas",
            "coin_stack_01",
        ));
        assert!(projection.has_belief(&belief_id("belief_tomas_missing_coin")));
        assert!(!projection.is_empty());

        for (label, mut candidate) in [
            ("observation", EpistemicProjection::new(manifest_id())),
            ("belief", EpistemicProjection::new(manifest_id())),
            ("contradiction", EpistemicProjection::new(manifest_id())),
            ("notebook", EpistemicProjection::new(manifest_id())),
            ("actor_known", EpistemicProjection::new(manifest_id())),
        ] {
            match label {
                "observation" => candidate.insert_observation(visible_observation(
                    "visible_actor",
                    "actor_mara",
                    Vec::new(),
                )),
                "belief" => candidate.insert_belief(belief(
                    "belief_candidate_missing_coin",
                    "actor_tomas",
                    "coin_stack_01",
                )),
                "contradiction" => candidate.insert_contradiction(contradiction()),
                "notebook" => {
                    candidate
                        .notebook_entries_by_actor
                        .entry(actor_id("actor_tomas"))
                        .or_default()
                        .insert(NotebookEntry {
                            actor_id: actor_id("actor_tomas"),
                            source_belief_id: Some(belief_id("belief_note")),
                            source_observation_id: None,
                            summary: "Tomas wrote a note".to_string(),
                        });
                }
                "actor_known" => candidate.insert_role_assignment_notice(
                    actor_id("actor_tomas"),
                    workplace_id("workplace_tomas"),
                    place_id("home_tomas"),
                    true,
                    event_id("event_role_notice"),
                    SimTick::new(4),
                ),
                other => panic!("unhandled candidate {other}"),
            }
            assert!(
                !candidate.is_empty(),
                "{label} should make projection non-empty"
            );
        }

        let mut notebook_projection = EpistemicProjection::new(manifest_id());
        for actor in ["actor_tomas", "actor_mara"] {
            notebook_projection
                .notebook_entries_by_actor
                .entry(actor_id(actor))
                .or_default()
                .insert(NotebookEntry {
                    actor_id: actor_id(actor),
                    source_belief_id: None,
                    source_observation_id: Some(
                        ObservationId::new(format!("observation_{actor}")).unwrap(),
                    ),
                    summary: format!("note for {actor}"),
                });
        }
        let embodied = KnowledgeContext::embodied(actor_id("actor_tomas"), SimTick::new(4));
        let embodied_notes: Vec<_> = notebook_projection
            .notebook_entries_for_context(&embodied)
            .iter()
            .map(|entry| entry.summary.as_str())
            .collect();
        assert_eq!(embodied_notes, ["note for actor_tomas"]);

        let capability = crate::debug_capability::DebugCapability::mint();
        let debug = KnowledgeContext::debug(actor_id("actor_tomas"), SimTick::new(4), &capability);
        assert_eq!(
            notebook_projection
                .notebook_entries_for_context(&debug)
                .len(),
            2
        );
    }

    #[test]
    fn actor_known_sources_and_canonical_records_are_stable() {
        let stable_ids: Vec<_> = [
            ActorKnownProjectionSource::RoleAssignmentNotice,
            ActorKnownProjectionSource::StartingBelief,
            ActorKnownProjectionSource::CurrentPlace,
            ActorKnownProjectionSource::CarriedItem,
            ActorKnownProjectionSource::VisibleExit,
            ActorKnownProjectionSource::VisibleFoodSupply,
            ActorKnownProjectionSource::VisibleDoor,
            ActorKnownProjectionSource::VisibleContainer,
            ActorKnownProjectionSource::VisibleItem,
            ActorKnownProjectionSource::VisibleActor,
            ActorKnownProjectionSource::VisibleSleepAffordance,
        ]
        .iter()
        .map(ActorKnownProjectionSource::stable_id)
        .collect();
        assert_eq!(
            stable_ids,
            [
                "role_assignment_notice",
                "starting_belief",
                "current_place",
                "carried_item",
                "visible_exit",
                "visible_food_supply",
                "visible_door",
                "visible_container",
                "visible_item",
                "visible_actor",
                "visible_sleep_affordance"
            ]
        );

        let canonical: Vec<_> = policy_behavior_records(actor_id("actor_tomas"))
            .iter()
            .map(ActorKnownProjectionRecord::serialize_canonical)
            .collect();
        assert!(canonical.iter().any(|line| line
            == "current_place|place=home_tomas|label=Tomas home|source=current_place|event=event_current_place|tick=4"));
        assert!(canonical.iter().any(|line| line
            == "carried_item|id=notebook_01|place=home_tomas|location=carried:actor_tomas|portable=true|source=carried_item|event=event_carried_item|tick=4"));
        assert!(canonical.iter().any(|line| line
            == "workplace|id=workplace_tomas|place=home_tomas|access_open=true|source=role_assignment_notice|event=event_role_notice|tick=4"));
    }

    #[test]
    fn actor_known_classification_helpers_preserve_freshness_keys_and_sources() {
        let actor = actor_id("actor_tomas");
        let previous = ActorKnownProjectionRecord::Workplace {
            actor_id: actor.clone(),
            workplace_id: workplace_id("workplace_tomas"),
            place_id: place_id("home_tomas"),
            believed_access_open: false,
            source: ActorKnownProjectionSource::RoleAssignmentNotice,
            source_event_id: event_id("event_role_notice_a"),
            source_tick: SimTick::new(4),
        };
        let later_event = ActorKnownProjectionRecord::Workplace {
            actor_id: actor.clone(),
            workplace_id: workplace_id("workplace_tomas"),
            place_id: place_id("home_tomas"),
            believed_access_open: true,
            source: ActorKnownProjectionSource::RoleAssignmentNotice,
            source_event_id: event_id("event_role_notice_b"),
            source_tick: SimTick::new(4),
        };
        let later_tick = ActorKnownProjectionRecord::Workplace {
            actor_id: actor.clone(),
            workplace_id: workplace_id("workplace_tomas"),
            place_id: place_id("home_tomas"),
            believed_access_open: true,
            source: ActorKnownProjectionSource::RoleAssignmentNotice,
            source_event_id: event_id("event_role_notice_old"),
            source_tick: SimTick::new(5),
        };
        assert!(workplace_record_is_newer(&later_event, &previous));
        assert!(workplace_record_is_newer(&later_tick, &previous));
        assert!(!workplace_record_is_newer(&previous, &later_event));
        assert!(!workplace_record_is_newer(&previous, &previous));

        let visible_actor = ActorKnownProjectionRecord::LocalActor {
            actor_id: actor.clone(),
            observed_actor_id: actor_id("actor_mara"),
            place_id: place_id("home_tomas"),
            source: ActorKnownProjectionSource::VisibleActor,
            source_event_id: event_id("event_visible_actor"),
            source_tick: SimTick::new(4),
        };
        assert_eq!(
            reclassifying_record_freshness(&visible_actor, true, SimTick::new(4)),
            ActorKnownProjectionFreshness::CurrentlyPerceived
        );
        assert_eq!(
            reclassifying_record_freshness(&visible_actor, false, SimTick::new(4)),
            ActorKnownProjectionFreshness::Remembered
        );
        assert_eq!(
            reclassifying_record_freshness(&previous, true, SimTick::new(4)),
            ActorKnownProjectionFreshness::Remembered
        );

        assert_eq!(
            location_key(&Location::AtPlace(place_id("home_tomas"))),
            "place:home_tomas"
        );
        assert_eq!(
            location_key(&Location::InContainer(container_id("strongbox_tomas"))),
            "container:strongbox_tomas"
        );
        assert_eq!(
            location_key(&Location::CarriedBy(actor.clone())),
            "carried:actor_tomas"
        );
        assert_eq!(holder_key(&HolderKind::Actor(actor)), "actor:actor_tomas");
        assert_eq!(
            source_summary(&SourceRef::Event(event_id("event_source"))),
            "event:event_source"
        );
        assert_eq!(
            source_summary(&SourceRef::Action(ActionId::new("look").unwrap())),
            "action:look"
        );
    }

    #[test]
    fn observations_materialize_actor_known_records_with_locations() {
        let actor_records = actor_known_records_from_observation(&visible_observation(
            "visible_actor",
            "actor_mara",
            Vec::new(),
        ));
        assert!(matches!(
            actor_records.as_slice(),
            [ActorKnownProjectionRecord::LocalActor {
                observed_actor_id,
                source: ActorKnownProjectionSource::VisibleActor,
                ..
            }] if observed_actor_id.as_str() == "actor_mara"
        ));

        let item_records = actor_known_records_from_observation(&visible_observation(
            "visible_item",
            "coin_stack_01",
            vec![
                PayloadField::new("item_source_kind", "place"),
                PayloadField::new("item_source_place_id", "home_tomas"),
                PayloadField::new("portable", "true"),
            ],
        ));
        assert!(matches!(
            item_records.as_slice(),
            [ActorKnownProjectionRecord::LocalItem {
                item_id,
                source_location: Location::AtPlace(place_id),
                source: ActorKnownProjectionSource::VisibleItem,
                ..
            }] if item_id.as_str() == "coin_stack_01" && place_id.as_str() == "home_tomas"
        ));

        let carried_records = actor_known_records_from_observation(&visible_observation(
            "carried_item",
            "notebook_01",
            vec![
                PayloadField::new("item_source_kind", "carried"),
                PayloadField::new("item_source_actor_id", "actor_tomas"),
                PayloadField::new("portable", "true"),
            ],
        ));
        assert!(matches!(
            carried_records.as_slice(),
            [ActorKnownProjectionRecord::CarriedItem {
                item_id,
                source_location: Location::CarriedBy(actor_id),
                portable: true,
                source: ActorKnownProjectionSource::CarriedItem,
                ..
            }] if item_id.as_str() == "notebook_01" && actor_id.as_str() == "actor_tomas"
        ));

        let container_item_records = actor_known_records_from_observation(&visible_observation(
            "visible_item",
            "coin_stack_02",
            vec![
                PayloadField::new("item_source_kind", "container"),
                PayloadField::new("item_source_container_id", "strongbox_tomas"),
                PayloadField::new("portable", "false"),
            ],
        ));
        assert!(matches!(
            container_item_records.as_slice(),
            [ActorKnownProjectionRecord::LocalItem {
                item_id,
                source_location: Location::InContainer(container_id),
                portable: false,
                source: ActorKnownProjectionSource::VisibleItem,
                ..
            }] if item_id.as_str() == "coin_stack_02" && container_id.as_str() == "strongbox_tomas"
        ));

        let door_records = actor_known_records_from_observation(&visible_observation(
            "visible_door",
            "door_home_market",
            vec![
                PayloadField::new("endpoint_a", "home_tomas"),
                PayloadField::new("endpoint_b", "market"),
                PayloadField::new("is_open", "true"),
                PayloadField::new("is_locked", "false"),
                PayloadField::new("blocks_movement_when_closed", "true"),
            ],
        ));
        assert!(matches!(
            door_records.as_slice(),
            [ActorKnownProjectionRecord::LocalDoor {
                door_id,
                endpoint_a,
                endpoint_b,
                is_open: true,
                is_locked: false,
                blocks_movement_when_closed: true,
                source: ActorKnownProjectionSource::VisibleDoor,
                ..
            }] if door_id.as_str() == "door_home_market"
                && endpoint_a.as_str() == "home_tomas"
                && endpoint_b.as_str() == "market"
        ));

        let container_records = actor_known_records_from_observation(&visible_observation(
            "visible_container",
            "strongbox_tomas",
            vec![
                PayloadField::new("is_open", "false"),
                PayloadField::new("is_locked", "true"),
            ],
        ));
        assert!(matches!(
            container_records.as_slice(),
            [ActorKnownProjectionRecord::LocalContainer {
                container_id,
                is_open: false,
                is_locked: true,
                source: ActorKnownProjectionSource::VisibleContainer,
                ..
            }] if container_id.as_str() == "strongbox_tomas"
        ));
    }

    #[test]
    fn starting_beliefs_materialize_actor_known_records() {
        let mut projection = EpistemicProjection::new(manifest_id());
        projection.insert_starting_belief(
            actor_id("actor_tomas"),
            "sleep_place",
            "sleep_affordance_bed",
            "home_tomas",
            event_id("event_starting_sleep"),
            SimTick::new(1),
        );
        projection.insert_starting_belief(
            actor_id("actor_tomas"),
            "household_food_source",
            "food_pantry",
            "place:kitchen",
            event_id("event_starting_food"),
            SimTick::new(2),
        );
        projection.insert_starting_belief(
            actor_id("actor_tomas"),
            "unknown_kind",
            "ignored",
            "ignored",
            event_id("event_ignored"),
            SimTick::new(3),
        );

        let records: Vec<_> = projection
            .actor_known_records_by_actor
            .get(&actor_id("actor_tomas"))
            .into_iter()
            .flat_map(|records| records.iter())
            .map(ActorKnownProjectionRecord::serialize_canonical)
            .collect();

        assert_eq!(records.len(), 2);
        assert!(records.iter().any(|record| record
            == "food|id=food_pantry|place=kitchen|servings=-|source=starting_belief|event=event_starting_food|tick=2"));
        assert!(records.iter().any(|record| record
            == "sleep|place=home_tomas|affordance=sleep_affordance_bed|source=starting_belief|event=event_starting_sleep|tick=1"));
    }

    #[test]
    fn actor_known_projection_policy_table_declares_every_record_kind() {
        let policies = actor_known_projection_policy_kinds();

        assert_eq!(
            policies.keys().copied().collect::<Vec<_>>(),
            [
                "carried_item",
                "current_place",
                "food_source",
                "local_actor",
                "local_container",
                "local_door",
                "local_item",
                "route",
                "sleep_place",
                "workplace"
            ]
        );
        for record in policy_behavior_records(actor_id("actor_tomas")) {
            assert!(
                policies.contains_key(record.kind()),
                "policy table lacks record kind {}",
                record.kind()
            );
        }
    }

    #[test]
    fn actor_known_projection_policy_table_drives_record_behavior() {
        let current_place = place_id("home_tomas");
        let policies = actor_known_projection_policy_kinds();

        let mismatches = policy_surface_mismatches(&policies, &current_place);

        assert!(
            mismatches.is_empty(),
            "policy table rows must drive emitted no-human and embodied surface behavior: {mismatches:?}"
        );
    }

    #[test]
    fn actor_known_projection_policy_table_detects_synthetic_row_mutations() {
        let current_place = place_id("home_tomas");
        let mut policies = actor_known_projection_policy_kinds();

        policies.get_mut("food_source").unwrap().accessibility_scope =
            ActorKnownProjectionAccessibilityScope::None;
        assert!(
            policy_surface_mismatches(&policies, &current_place)
                .iter()
                .any(|mismatch| mismatch.contains("food_source accessibility")),
            "accessibility-scope mutation must be caught by emitted no-human facts"
        );

        let mut policies = actor_known_projection_policy_kinds();
        policies.get_mut("workplace").unwrap().accessibility_scope =
            ActorKnownProjectionAccessibilityScope::None;
        assert!(
            policy_surface_mismatches(&policies, &current_place)
                .iter()
                .any(|mismatch| mismatch.contains("workplace accessibility")),
            "workplace accessibility-scope mutation must be caught by emitted no-human facts"
        );

        let mut policies = actor_known_projection_policy_kinds();
        policies.get_mut("workplace").unwrap().classification =
            ActorKnownProjectionPolicy::ReclassifyWhenStale;
        assert!(
            policy_surface_mismatches(&policies, &current_place)
                .iter()
                .any(|mismatch| mismatch.contains("workplace classification")),
            "classification mutation must be caught by superseded workplace surface output"
        );

        let mut policies = actor_known_projection_policy_kinds();
        policies.get_mut("workplace").unwrap().embodied_scope =
            ActorKnownProjectionEmbodiedScope::LatestCurrentPlaceOnly;
        assert!(
            stale_workplace_embodied_scope_mismatches(&policies)
                .iter()
                .any(|mismatch| mismatch.contains("workplace embodied")),
            "embodied-scope mutation must be caught by stale current-place workplace behavior"
        );
    }

    #[test]
    fn actor_known_projection_policy_truth_table_detects_predicate_inversion() {
        let current_place = place_id("home_tomas");
        let policies = actor_known_projection_policy_kinds();
        let actor = actor_id("actor_tomas");
        let projection = policy_behavior_projection(actor.clone());
        let filter_context = KnowledgeContext::embodied(actor, SimTick::new(4));

        let mismatches = inverted_embodied_predicate_mismatches(
            &policies,
            &projection,
            &filter_context,
            &current_place,
        );

        assert!(
            !mismatches.is_empty(),
            "synthetic inversion of includes_in_embodied_context must fail the policy oracle"
        );
    }

    #[test]
    fn supersede_newest_by_subject_requires_subject_extractor() {
        let policy = ActorKnownProjectionKindPolicy {
            classification: ActorKnownProjectionPolicy::SupersedeNewestBySubject,
            embodied_scope: ActorKnownProjectionEmbodiedScope::LatestCurrentPlaceOnly,
            accessibility_scope: ActorKnownProjectionAccessibilityScope::None,
        };
        let actor = actor_id("actor_tomas");
        let route_record = ActorKnownProjectionRecord::Route {
            actor_id: actor,
            from_place_id: place_id("home_tomas"),
            to_place_id: place_id("market"),
            source: ActorKnownProjectionSource::VisibleExit,
            source_event_id: event_id("event_visible_exit"),
            source_tick: SimTick::new(4),
        };

        let result = std::panic::catch_unwind(|| {
            policy.includes_classified_record(&route_record, &BTreeMap::new());
        });
        assert!(
            result.is_err(),
            "synthetic non-workplace supersede policy must fail loudly instead of dropping records"
        );
    }

    #[test]
    fn workplace_current_place_scope_drops_other_place_from_embodied_context() {
        let actor = actor_id("actor_tomas");
        let mut projection = EpistemicProjection::new(manifest_id());
        projection.insert_role_assignment_notice(
            actor.clone(),
            workplace_id("workplace_tomas"),
            place_id("home_tomas"),
            true,
            event_id("event_role_notice_home"),
            SimTick::new(4),
        );
        let context = KnowledgeContext::embodied(actor, SimTick::new(9));

        let classified =
            projection.classified_actor_known_records_for_context(&context, &place_id("market"));
        let workplace_record = classified
            .iter()
            .find(|classified| classified.record().kind() == "workplace")
            .expect("no-human classifier keeps remembered workplace record");

        assert!(!workplace_record.is_current_place_record());
        assert!(
            !workplace_record
                .record()
                .policy()
                .includes_in_embodied_context(
                    workplace_record.is_current_place_record(),
                    workplace_record.is_latest_current_place_record(),
                ),
            "workplace CurrentPlaceOnly policy must drop other-place records from embodied context"
        );
    }

    fn policy_behavior_records(actor: ActorId) -> Vec<ActorKnownProjectionRecord> {
        vec![
            ActorKnownProjectionRecord::CurrentPlace {
                actor_id: actor.clone(),
                place_id: place_id("home_tomas"),
                display_label: "Tomas home".to_string(),
                source: ActorKnownProjectionSource::CurrentPlace,
                source_event_id: event_id("event_current_place"),
                source_tick: SimTick::new(4),
            },
            ActorKnownProjectionRecord::CarriedItem {
                actor_id: actor.clone(),
                item_id: item_id("notebook_01"),
                place_id: place_id("home_tomas"),
                source_location: Location::CarriedBy(actor.clone()),
                portable: true,
                source: ActorKnownProjectionSource::CarriedItem,
                source_event_id: event_id("event_carried_item"),
                source_tick: SimTick::new(4),
            },
            ActorKnownProjectionRecord::Route {
                actor_id: actor.clone(),
                from_place_id: place_id("home_tomas"),
                to_place_id: place_id("market"),
                source: ActorKnownProjectionSource::VisibleExit,
                source_event_id: event_id("event_visible_exit"),
                source_tick: SimTick::new(4),
            },
            ActorKnownProjectionRecord::FoodSource {
                actor_id: actor.clone(),
                food_source_id: "food_stew".to_string(),
                place_id: Some(place_id("home_tomas")),
                believed_servings: Some(2),
                source: ActorKnownProjectionSource::VisibleFoodSupply,
                source_event_id: event_id("event_visible_food"),
                source_tick: SimTick::new(4),
            },
            ActorKnownProjectionRecord::LocalActor {
                actor_id: actor.clone(),
                observed_actor_id: actor_id("actor_mara"),
                place_id: place_id("home_tomas"),
                source: ActorKnownProjectionSource::VisibleActor,
                source_event_id: event_id("event_visible_actor"),
                source_tick: SimTick::new(4),
            },
            ActorKnownProjectionRecord::LocalContainer {
                actor_id: actor.clone(),
                container_id: container_id("strongbox_tomas"),
                place_id: place_id("home_tomas"),
                is_open: false,
                is_locked: false,
                source: ActorKnownProjectionSource::VisibleContainer,
                source_event_id: event_id("event_visible_container"),
                source_tick: SimTick::new(4),
            },
            ActorKnownProjectionRecord::LocalDoor {
                actor_id: actor.clone(),
                door_id: DoorId::new("door_home_market").unwrap(),
                place_id: place_id("home_tomas"),
                endpoint_a: place_id("home_tomas"),
                endpoint_b: place_id("market"),
                is_open: true,
                is_locked: false,
                blocks_movement_when_closed: true,
                source: ActorKnownProjectionSource::VisibleDoor,
                source_event_id: event_id("event_visible_door"),
                source_tick: SimTick::new(4),
            },
            ActorKnownProjectionRecord::LocalItem {
                actor_id: actor.clone(),
                item_id: item_id("coin_stack_01"),
                place_id: place_id("home_tomas"),
                source_location: Location::AtPlace(place_id("home_tomas")),
                portable: true,
                source: ActorKnownProjectionSource::VisibleItem,
                source_event_id: event_id("event_visible_item"),
                source_tick: SimTick::new(4),
            },
            ActorKnownProjectionRecord::SleepPlace {
                actor_id: actor.clone(),
                place_id: place_id("home_tomas"),
                sleep_affordance_id: Some("bed_tomas".to_string()),
                source: ActorKnownProjectionSource::VisibleSleepAffordance,
                source_event_id: event_id("event_visible_sleep"),
                source_tick: SimTick::new(4),
            },
            ActorKnownProjectionRecord::Workplace {
                actor_id: actor,
                workplace_id: workplace_id("workplace_tomas"),
                place_id: place_id("home_tomas"),
                believed_access_open: true,
                source: ActorKnownProjectionSource::RoleAssignmentNotice,
                source_event_id: event_id("event_role_notice"),
                source_tick: SimTick::new(4),
            },
        ]
    }

    fn policy_surface_mismatches(
        policies: &BTreeMap<&'static str, ActorKnownProjectionKindPolicy>,
        current_place: &PlaceId,
    ) -> Vec<String> {
        let actor = actor_id("actor_tomas");
        let projection = policy_behavior_projection(actor.clone());
        let filter_context = KnowledgeContext::embodied(actor.clone(), SimTick::new(4));
        let embodied_context =
            policy_behavior_embodied_context(&projection, actor.clone(), current_place.clone());
        let no_human_fact_values =
            policy_behavior_no_human_fact_values(&projection, actor, current_place.clone());
        let mut mismatches = Vec::new();

        for (kind, policy) in policies {
            let expected_embodied = expected_embodied_presence(
                kind,
                *policy,
                &projection,
                &filter_context,
                current_place,
            );
            let actual_embodied = embodied_surface_contains(&embodied_context, kind);
            if actual_embodied != expected_embodied {
                mismatches.push(format!(
                    "{kind} embodied expected {expected_embodied} observed {actual_embodied}"
                ));
            }

            if let Some(access_fact_id) = accessibility_fact_id(kind) {
                let expected_accessibility = policy.accessibility_scope()
                    == ActorKnownProjectionAccessibilityScope::FromAnyPlace;
                let actual_accessibility = no_human_fact_values.contains_key(access_fact_id);
                if actual_accessibility != expected_accessibility {
                    mismatches.push(format!(
                        "{kind} accessibility expected {expected_accessibility} observed {actual_accessibility}"
                    ));
                }
            }
        }

        let expected_workplace_values = match policies
            .get("workplace")
            .expect("policy table has workplace")
            .classification()
        {
            ActorKnownProjectionPolicy::SupersedeNewestBySubject => {
                vec!["workplace_tomas:true".to_string()]
            }
            ActorKnownProjectionPolicy::ReclassifyWhenStale => vec![
                "workplace_tomas:false".to_string(),
                "workplace_tomas:true".to_string(),
            ],
        };
        let actual_workplace_values = no_human_fact_values
            .get("workplace_believed_accessible")
            .cloned()
            .unwrap_or_default();
        if actual_workplace_values != expected_workplace_values {
            mismatches.push(format!(
                "workplace classification expected values {expected_workplace_values:?} observed {actual_workplace_values:?}"
            ));
        }

        mismatches
    }

    fn policy_behavior_projection(actor: ActorId) -> EpistemicProjection {
        let mut projection = EpistemicProjection::new(manifest_id());
        projection.insert_observation(Observation::new(
            ObservationId::new("observation_current_place_tick_4").unwrap(),
            actor.clone(),
            Channel::DirectSight,
            SimTick::new(4),
            place_id("home_tomas"),
            ObservationSubject::Place(place_id("home_tomas")),
            ObservationTarget::Place(place_id("home_tomas")),
            Confidence::new(900).unwrap(),
            SourceRef::Event(event_id("event_current_place_tick_4")),
        ));
        let mut records = policy_behavior_records(actor);
        records.push(ActorKnownProjectionRecord::Workplace {
            actor_id: actor_id("actor_tomas"),
            workplace_id: workplace_id("workplace_tomas"),
            place_id: place_id("home_tomas"),
            believed_access_open: false,
            source: ActorKnownProjectionSource::RoleAssignmentNotice,
            source_event_id: event_id("event_role_notice_old_closed"),
            source_tick: SimTick::new(3),
        });
        projection
            .actor_known_records_by_actor
            .entry(actor_id("actor_tomas"))
            .or_default()
            .extend(records);
        projection
    }

    fn policy_behavior_embodied_context(
        projection: &EpistemicProjection,
        actor: ActorId,
        current_place: PlaceId,
    ) -> KnowledgeContext {
        let mut state = PhysicalState::empty(NeedModelState::new(5, 3));
        state
            .actors
            .insert(actor.clone(), ActorBody::new(actor.clone(), current_place));
        current_place_knowledge_context(
            &state,
            Some(projection),
            &actor,
            SimTick::new(4),
            &manifest_id(),
            8,
        )
    }

    fn policy_behavior_no_human_fact_values(
        projection: &EpistemicProjection,
        actor: ActorId,
        current_place: PlaceId,
    ) -> BTreeMap<String, Vec<String>> {
        let surface =
            NoHumanActorKnownSurfaceBuilder::from_projection(NoHumanActorKnownSurfaceRequest {
                projection,
                agent_state: &AgentState::default(),
                actor_id: actor,
                current_place_id: current_place,
                decision_tick: SimTick::new(4),
                window_id: "policy_surface_window",
                window_end_tick: SimTick::new(5),
                current_place_witness_event_id: Some(event_id("event_current_place_witness")),
                needs_witness_event_id: None,
                frame_event_id: Some(event_id("event_policy_frame")),
            })
            .build(&AgentState::default());
        let mut values = BTreeMap::<String, Vec<String>>::new();
        for fact in surface.context().actor_known_facts() {
            values
                .entry(fact.stable_id().to_string())
                .or_default()
                .push(fact.value().to_string());
        }
        for fact_values in values.values_mut() {
            fact_values.sort();
            fact_values.dedup();
        }
        values
    }

    fn stale_workplace_embodied_scope_mismatches(
        policies: &BTreeMap<&'static str, ActorKnownProjectionKindPolicy>,
    ) -> Vec<String> {
        let actor = actor_id("actor_tomas");
        let current_place = place_id("home_tomas");
        let mut projection = EpistemicProjection::new(manifest_id());
        projection.insert_observation(Observation::new(
            ObservationId::new("observation_current_place_tick_4").unwrap(),
            actor.clone(),
            Channel::DirectSight,
            SimTick::new(4),
            current_place.clone(),
            ObservationSubject::Place(current_place.clone()),
            ObservationTarget::Place(current_place.clone()),
            Confidence::new(900).unwrap(),
            SourceRef::Event(event_id("event_current_place_tick_4")),
        ));
        projection.insert_role_assignment_notice(
            actor.clone(),
            workplace_id("workplace_tomas"),
            current_place.clone(),
            true,
            event_id("event_role_notice_stale"),
            SimTick::new(3),
        );
        let embodied_context =
            policy_behavior_embodied_context(&projection, actor, current_place.clone());
        let expected_embodied = policies
            .get("workplace")
            .expect("policy table has workplace")
            .includes_in_embodied_context(true, false);
        let actual_embodied = embodied_surface_contains(&embodied_context, "workplace");
        if actual_embodied == expected_embodied {
            Vec::new()
        } else {
            vec![format!(
                "workplace embodied expected {expected_embodied} observed {actual_embodied}"
            )]
        }
    }

    fn expected_embodied_presence(
        kind: &str,
        policy: ActorKnownProjectionKindPolicy,
        projection: &EpistemicProjection,
        context: &KnowledgeContext,
        current_place: &PlaceId,
    ) -> bool {
        projection
            .classified_actor_known_records_for_context(context, current_place)
            .iter()
            .find(|classified| classified.record().kind() == kind)
            .is_some_and(|classified| {
                expected_embodied_presence_from_truth_table(
                    policy.embodied_scope(),
                    classified.is_current_place_record(),
                    classified.is_latest_current_place_record(),
                )
            })
    }

    fn expected_embodied_presence_from_truth_table(
        embodied_scope: ActorKnownProjectionEmbodiedScope,
        current_place_record: bool,
        latest_current_place_record: bool,
    ) -> bool {
        match embodied_scope {
            ActorKnownProjectionEmbodiedScope::LatestCurrentPlaceOnly => {
                latest_current_place_record
            }
            ActorKnownProjectionEmbodiedScope::CurrentPlaceOnly => current_place_record,
        }
    }

    fn inverted_embodied_predicate_mismatches(
        policies: &BTreeMap<&'static str, ActorKnownProjectionKindPolicy>,
        projection: &EpistemicProjection,
        context: &KnowledgeContext,
        current_place: &PlaceId,
    ) -> Vec<String> {
        let classified_records =
            projection.classified_actor_known_records_for_context(context, current_place);
        policies
            .iter()
            .filter_map(|(kind, policy)| {
                let classified = classified_records
                    .iter()
                    .find(|classified| classified.record().kind() == *kind)?;
                let expected = expected_embodied_presence_from_truth_table(
                    policy.embodied_scope(),
                    classified.is_current_place_record(),
                    classified.is_latest_current_place_record(),
                );
                let inverted_actual = match policy.embodied_scope() {
                    ActorKnownProjectionEmbodiedScope::LatestCurrentPlaceOnly => {
                        !classified.is_latest_current_place_record()
                    }
                    ActorKnownProjectionEmbodiedScope::CurrentPlaceOnly => {
                        !classified.is_current_place_record()
                    }
                };
                (expected != inverted_actual).then(|| {
                    format!(
                        "{kind} embodied inversion expected {expected} observed {inverted_actual}"
                    )
                })
            })
            .collect()
    }

    fn embodied_surface_contains(context: &KnowledgeContext, kind: &str) -> bool {
        match kind {
            "current_place" => context
                .actor_known_current_places()
                .iter()
                .any(|fact| fact.place_id().as_str() == "home_tomas"),
            "carried_item" => context
                .actor_known_carried_items()
                .iter()
                .any(|fact| fact.item_id().as_str() == "notebook_01"),
            "route" => context
                .actor_known_routes()
                .iter()
                .any(|fact| fact.from_place_id().as_str() == "home_tomas"),
            "food_source" => context
                .actor_known_food_sources()
                .iter()
                .any(|fact| fact.food_supply_id().as_str() == "food_stew"),
            "local_actor" => context
                .actor_known_local_actors()
                .iter()
                .any(|fact| fact.actor_id().as_str() == "actor_mara"),
            "local_container" => context
                .actor_known_containers()
                .iter()
                .any(|fact| fact.container_id().as_str() == "strongbox_tomas"),
            "local_door" => context
                .actor_known_doors()
                .iter()
                .any(|fact| fact.door_id().as_str() == "door_home_market"),
            "local_item" => context
                .actor_known_items()
                .iter()
                .any(|fact| fact.item_id().as_str() == "coin_stack_01"),
            "sleep_place" => context
                .actor_known_sleep_affordances()
                .iter()
                .any(|fact| fact.sleep_affordance_id().as_str() == "bed_tomas"),
            "workplace" => context
                .actor_known_workplaces()
                .iter()
                .any(|fact| fact.workplace_id().as_str() == "workplace_tomas"),
            other => panic!("unhandled policy behavior kind {other}"),
        }
    }

    fn accessibility_fact_id(kind: &str) -> Option<&'static str> {
        match kind {
            "food_source" => Some("food_source_believed_accessible"),
            "sleep_place" => Some("sleep_place_believed_accessible"),
            "workplace" => Some("workplace_believed_accessible"),
            _ => None,
        }
    }

    #[test]
    fn classified_actor_known_records_supersede_workplaces_by_tick_then_event_id() {
        let actor = actor_id("actor_tomas");
        let mut projection = EpistemicProjection::new(manifest_id());
        projection.insert_role_assignment_notice(
            actor.clone(),
            workplace_id("workplace_tomas"),
            place_id("home_tomas"),
            false,
            event_id("event_role_notice_same_tick_closed"),
            SimTick::new(8),
        );
        projection.insert_role_assignment_notice(
            actor.clone(),
            workplace_id("workplace_tomas"),
            place_id("home_tomas"),
            true,
            event_id("event_role_notice_same_tick_open"),
            SimTick::new(8),
        );
        projection.insert_role_assignment_notice(
            actor.clone(),
            workplace_id("workplace_other"),
            place_id("market"),
            true,
            event_id("event_role_notice_other"),
            SimTick::new(7),
        );

        let context = KnowledgeContext::embodied(actor, SimTick::new(9));
        let workplace_records: Vec<_> = projection
            .classified_actor_known_records_for_context(&context, &place_id("home_tomas"))
            .into_iter()
            .filter_map(|classified| match classified.record() {
                ActorKnownProjectionRecord::Workplace {
                    workplace_id,
                    believed_access_open,
                    source_event_id,
                    ..
                } => Some((
                    workplace_id.as_str().to_string(),
                    *believed_access_open,
                    source_event_id.as_str().to_string(),
                )),
                _ => None,
            })
            .collect();

        assert_eq!(
            workplace_records,
            [
                (
                    "workplace_other".to_string(),
                    true,
                    "event_role_notice_other".to_string()
                ),
                (
                    "workplace_tomas".to_string(),
                    true,
                    "event_role_notice_same_tick_open".to_string()
                )
            ]
        );
    }

    #[test]
    fn embodied_context_does_not_return_other_actor_private_beliefs() {
        let mut projection = EpistemicProjection::new(manifest_id());
        projection.insert_belief(belief(
            "belief_tomas_missing_coin",
            "actor_tomas",
            "coin_stack_01",
        ));
        projection.insert_belief(belief(
            "belief_mara_hidden_coin",
            "actor_mara",
            "coin_stack_02",
        ));

        let context = KnowledgeContext::embodied(actor_id("actor_tomas"), SimTick::new(4));
        let visible_beliefs = projection.beliefs_for_context(&context);

        assert_eq!(visible_beliefs.len(), 1);
        assert_eq!(
            visible_beliefs[0].belief_id(),
            &belief_id("belief_tomas_missing_coin")
        );
    }

    #[test]
    fn debug_context_can_inspect_all_beliefs_non_diegetically() {
        let mut projection = EpistemicProjection::new(manifest_id());
        projection.insert_belief(belief(
            "belief_tomas_missing_coin",
            "actor_tomas",
            "coin_stack_01",
        ));
        projection.insert_belief(belief(
            "belief_mara_hidden_coin",
            "actor_mara",
            "coin_stack_02",
        ));

        let capability = crate::debug_capability::DebugCapability::mint();
        let context =
            KnowledgeContext::debug(actor_id("actor_tomas"), SimTick::new(4), &capability);
        let visible_beliefs = projection.beliefs_for_context(&context);
        let ordered_ids: Vec<_> = visible_beliefs
            .iter()
            .map(|belief| belief.belief_id().as_str())
            .collect();

        assert!(context.debug_non_diegetic());
        assert_eq!(
            ordered_ids,
            ["belief_mara_hidden_coin", "belief_tomas_missing_coin"]
        );

        let debug_view = projection.debug_epistemics_view();
        assert!(debug_view.debug_only());
        assert_eq!(debug_view.beliefs_by_holder.len(), 2);
        assert!(debug_view
            .beliefs_by_holder
            .iter()
            .any(|holder| holder.holder_actor_id == actor_id("actor_mara")));
        assert!(debug_view
            .beliefs_by_holder
            .iter()
            .any(|holder| holder.holder_actor_id == actor_id("actor_tomas")));
    }

    #[test]
    fn projection_indexes_iterate_in_stable_id_order() {
        let mut first = EpistemicProjection::new(manifest_id());
        first.insert_belief(belief("belief_10", "actor_tomas", "coin_stack_10"));
        first.insert_belief(belief("belief_01", "actor_tomas", "coin_stack_01"));
        first.insert_belief(belief("belief_02", "actor_tomas", "coin_stack_02"));

        let mut second = EpistemicProjection::new(manifest_id());
        second.insert_belief(belief("belief_02", "actor_tomas", "coin_stack_02"));
        second.insert_belief(belief("belief_10", "actor_tomas", "coin_stack_10"));
        second.insert_belief(belief("belief_01", "actor_tomas", "coin_stack_01"));

        let context = KnowledgeContext::embodied(actor_id("actor_tomas"), SimTick::new(4));
        let first_ids: Vec<_> = first
            .beliefs_for_context(&context)
            .iter()
            .map(|belief| belief.belief_id().as_str())
            .collect();
        let second_ids: Vec<_> = second
            .beliefs_for_context(&context)
            .iter()
            .map(|belief| belief.belief_id().as_str())
            .collect();

        assert_eq!(first_ids, ["belief_01", "belief_02", "belief_10"]);
        assert_eq!(second_ids, first_ids);
    }
}
