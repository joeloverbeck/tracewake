use std::collections::BTreeSet;

use crate::checksum::{compute_holder_known_context_hash, HolderKnownContextHash};
use crate::debug_capability::DebugCapability;
use crate::epistemics::observation::{PrivacyScope, EPISTEMIC_RECORD_SCHEMA_V1};
use crate::ids::{
    ActorId, FoodSupplyId, HolderKnownContextId, PlaceId, SchemaVersion, SleepAffordanceId,
    WorkplaceId,
};
use crate::time::SimTick;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ViewMode {
    Embodied,
    Debug,
}

impl ViewMode {
    pub const fn stable_id(self) -> &'static str {
        match self {
            ViewMode::Embodied => "embodied",
            ViewMode::Debug => "debug",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AllowedKnowledgeSource {
    OwnDirectObservation,
    OwnSearchOrTouchObservation,
    OwnSoundObservation,
    OwnAbsenceMarker,
    OwnSourceBackedBelief,
}

impl AllowedKnowledgeSource {
    pub const fn stable_id(self) -> &'static str {
        match self {
            AllowedKnowledgeSource::OwnDirectObservation => "own_direct_observation",
            AllowedKnowledgeSource::OwnSearchOrTouchObservation => {
                "own_search_or_touch_observation"
            }
            AllowedKnowledgeSource::OwnSoundObservation => "own_sound_observation",
            AllowedKnowledgeSource::OwnAbsenceMarker => "own_absence_marker",
            AllowedKnowledgeSource::OwnSourceBackedBelief => "own_source_backed_belief",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ForbiddenKnowledgeSource {
    UnobservedEventLogTruth,
    HiddenItemLocation,
    OtherActorsPrivateBeliefs,
    HumanDebugNotes,
    PreviousPossessedActorKnowledge,
}

impl ForbiddenKnowledgeSource {
    pub const fn stable_id(self) -> &'static str {
        match self {
            ForbiddenKnowledgeSource::UnobservedEventLogTruth => "unobserved_event_log_truth",
            ForbiddenKnowledgeSource::HiddenItemLocation => "hidden_item_location",
            ForbiddenKnowledgeSource::OtherActorsPrivateBeliefs => "other_actors_private_beliefs",
            ForbiddenKnowledgeSource::HumanDebugNotes => "human_debug_notes",
            ForbiddenKnowledgeSource::PreviousPossessedActorKnowledge => {
                "previous_possessed_actor_knowledge"
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScopeFilter {
    ActorPrivate(ActorId),
    DebugAll,
}

impl ScopeFilter {
    fn canonical_key(&self) -> String {
        match self {
            ScopeFilter::ActorPrivate(actor_id) => {
                format!("actor_private:{}", actor_id.as_str())
            }
            ScopeFilter::DebugAll => "debug_all".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KnowledgeProvenanceKind {
    Perception,
    Memory,
    Observation,
    Record,
    Belief,
    Reservation,
    ActionAffordanceFact,
}

impl KnowledgeProvenanceKind {
    pub const fn stable_id(&self) -> &'static str {
        match self {
            KnowledgeProvenanceKind::Perception => "perception",
            KnowledgeProvenanceKind::Memory => "memory",
            KnowledgeProvenanceKind::Observation => "observation",
            KnowledgeProvenanceKind::Record => "record",
            KnowledgeProvenanceKind::Belief => "belief",
            KnowledgeProvenanceKind::Reservation => "reservation",
            KnowledgeProvenanceKind::ActionAffordanceFact => "action_affordance_fact",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KnowledgeProvenanceEntry {
    kind: KnowledgeProvenanceKind,
    source: AllowedKnowledgeSource,
    source_key: String,
}

impl KnowledgeProvenanceEntry {
    pub fn actor_known_source(
        kind: KnowledgeProvenanceKind,
        source: AllowedKnowledgeSource,
        source_key: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            source,
            source_key: source_key.into(),
        }
    }

    pub fn kind(&self) -> &KnowledgeProvenanceKind {
        &self.kind
    }

    pub fn source(&self) -> AllowedKnowledgeSource {
        self.source
    }

    pub fn source_key(&self) -> &str {
        &self.source_key
    }

    fn canonical_key(&self) -> String {
        format!(
            "{}:{}:{}",
            self.kind.stable_id(),
            self.source.stable_id(),
            self.source_key
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ActorKnownWorkplaceFact {
    workplace_id: WorkplaceId,
    place_id: PlaceId,
    believed_access_open: bool,
    source_key: String,
}

impl ActorKnownWorkplaceFact {
    pub fn new(
        workplace_id: WorkplaceId,
        place_id: PlaceId,
        believed_access_open: bool,
        source_key: impl Into<String>,
    ) -> Self {
        Self {
            workplace_id,
            place_id,
            believed_access_open,
            source_key: source_key.into(),
        }
    }

    pub fn workplace_id(&self) -> &WorkplaceId {
        &self.workplace_id
    }

    pub fn place_id(&self) -> &PlaceId {
        &self.place_id
    }

    pub fn believed_access_open(&self) -> bool {
        self.believed_access_open
    }

    pub fn source_key(&self) -> &str {
        &self.source_key
    }

    fn canonical_key(&self) -> String {
        format!(
            "{}@{}:access_open={}:{}",
            self.workplace_id.as_str(),
            self.place_id.as_str(),
            self.believed_access_open,
            self.source_key
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ActorKnownFoodSourceFact {
    food_supply_id: FoodSupplyId,
    believed_servings: Option<u32>,
    source_key: String,
}

impl ActorKnownFoodSourceFact {
    pub fn new(food_supply_id: FoodSupplyId, source_key: impl Into<String>) -> Self {
        Self::with_believed_servings(food_supply_id, None, source_key)
    }

    pub fn with_believed_servings(
        food_supply_id: FoodSupplyId,
        believed_servings: Option<u32>,
        source_key: impl Into<String>,
    ) -> Self {
        Self {
            food_supply_id,
            believed_servings,
            source_key: source_key.into(),
        }
    }

    pub fn food_supply_id(&self) -> &FoodSupplyId {
        &self.food_supply_id
    }

    pub fn believed_servings(&self) -> Option<u32> {
        self.believed_servings
    }

    pub fn source_key(&self) -> &str {
        &self.source_key
    }

    fn canonical_key(&self) -> String {
        format!(
            "{}:servings={}:{}",
            self.food_supply_id.as_str(),
            self.believed_servings
                .map(|servings| servings.to_string())
                .unwrap_or_else(|| "-".to_string()),
            self.source_key
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ActorKnownSleepAffordanceFact {
    sleep_affordance_id: SleepAffordanceId,
    place_id: PlaceId,
    source_key: String,
}

impl ActorKnownSleepAffordanceFact {
    pub fn new(
        sleep_affordance_id: SleepAffordanceId,
        place_id: PlaceId,
        source_key: impl Into<String>,
    ) -> Self {
        Self {
            sleep_affordance_id,
            place_id,
            source_key: source_key.into(),
        }
    }

    pub fn sleep_affordance_id(&self) -> &SleepAffordanceId {
        &self.sleep_affordance_id
    }

    pub fn place_id(&self) -> &PlaceId {
        &self.place_id
    }

    pub fn source_key(&self) -> &str {
        &self.source_key
    }

    fn canonical_key(&self) -> String {
        format!(
            "{}@{}:{}",
            self.sleep_affordance_id.as_str(),
            self.place_id.as_str(),
            self.source_key
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ActorKnownRouteFact {
    from_place_id: PlaceId,
    to_place_id: PlaceId,
    source_key: String,
}

impl ActorKnownRouteFact {
    pub fn new(
        from_place_id: PlaceId,
        to_place_id: PlaceId,
        source_key: impl Into<String>,
    ) -> Self {
        Self {
            from_place_id,
            to_place_id,
            source_key: source_key.into(),
        }
    }

    pub fn from_place_id(&self) -> &PlaceId {
        &self.from_place_id
    }

    pub fn to_place_id(&self) -> &PlaceId {
        &self.to_place_id
    }

    pub fn source_key(&self) -> &str {
        &self.source_key
    }

    fn canonical_key(&self) -> String {
        format!(
            "{}->{}:{}",
            self.from_place_id.as_str(),
            self.to_place_id.as_str(),
            self.source_key
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ForbiddenTruthAudit {
    excluded_sources: BTreeSet<ForbiddenKnowledgeSource>,
    passed: bool,
}

impl ForbiddenTruthAudit {
    fn passed_excluding(excluded_sources: BTreeSet<ForbiddenKnowledgeSource>) -> Self {
        Self {
            excluded_sources,
            passed: true,
        }
    }

    pub fn passed(&self) -> bool {
        self.passed
    }

    pub fn excluded_sources(&self) -> &BTreeSet<ForbiddenKnowledgeSource> {
        &self.excluded_sources
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KnowledgeContextStatus {
    Current,
    Stale,
    Invalid,
}

impl KnowledgeContextStatus {
    pub const fn stable_id(self) -> &'static str {
        match self {
            KnowledgeContextStatus::Current => "current",
            KnowledgeContextStatus::Stale => "stale",
            KnowledgeContextStatus::Invalid => "invalid",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KnowledgeContext {
    viewer_actor_id: ActorId,
    bound_actor_id: ActorId,
    mode: ViewMode,
    current_tick: SimTick,
    event_frontier: u64,
    allowed_sources: BTreeSet<AllowedKnowledgeSource>,
    forbidden_sources: BTreeSet<ForbiddenKnowledgeSource>,
    perception_scope: ScopeFilter,
    belief_scope: ScopeFilter,
    observation_scope: ScopeFilter,
    projection_schema_version: SchemaVersion,
    debug_non_diegetic: bool,
    holder_known_context_id: HolderKnownContextId,
    holder_known_context_hash: HolderKnownContextHash,
    provenance_entries: Vec<KnowledgeProvenanceEntry>,
    actor_known_workplaces: Vec<ActorKnownWorkplaceFact>,
    actor_known_food_sources: Vec<ActorKnownFoodSourceFact>,
    actor_known_sleep_affordances: Vec<ActorKnownSleepAffordanceFact>,
    actor_known_routes: Vec<ActorKnownRouteFact>,
    forbidden_truth_audit: ForbiddenTruthAudit,
    status: KnowledgeContextStatus,
}

impl KnowledgeContext {
    pub fn embodied(viewer_actor_id: ActorId, current_tick: SimTick) -> Self {
        Self::embodied_at_frontier(viewer_actor_id, current_tick, 0)
    }

    pub fn embodied_at_frontier(
        viewer_actor_id: ActorId,
        current_tick: SimTick,
        event_frontier: u64,
    ) -> Self {
        Self::embodied_at_frontier_with_facts(
            viewer_actor_id,
            current_tick,
            event_frontier,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
    }

    pub fn embodied_at_frontier_with_workplaces(
        viewer_actor_id: ActorId,
        current_tick: SimTick,
        event_frontier: u64,
        actor_known_workplaces: Vec<ActorKnownWorkplaceFact>,
    ) -> Self {
        Self::embodied_at_frontier_with_facts(
            viewer_actor_id,
            current_tick,
            event_frontier,
            actor_known_workplaces,
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
    }

    pub fn embodied_at_frontier_with_facts(
        viewer_actor_id: ActorId,
        current_tick: SimTick,
        event_frontier: u64,
        actor_known_workplaces: Vec<ActorKnownWorkplaceFact>,
        actor_known_food_sources: Vec<ActorKnownFoodSourceFact>,
        actor_known_sleep_affordances: Vec<ActorKnownSleepAffordanceFact>,
        actor_known_routes: Vec<ActorKnownRouteFact>,
    ) -> Self {
        let actor_scope = ScopeFilter::ActorPrivate(viewer_actor_id.clone());
        Self::seal(
            viewer_actor_id.clone(),
            viewer_actor_id,
            ViewMode::Embodied,
            current_tick,
            event_frontier,
            embodied_allowed_sources(),
            forbidden_sources(),
            actor_scope.clone(),
            actor_scope.clone(),
            actor_scope,
            false,
            baseline_embodied_provenance(),
            actor_known_workplaces,
            actor_known_food_sources,
            actor_known_sleep_affordances,
            actor_known_routes,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn seal(
        viewer_actor_id: ActorId,
        bound_actor_id: ActorId,
        mode: ViewMode,
        current_tick: SimTick,
        event_frontier: u64,
        allowed_sources: BTreeSet<AllowedKnowledgeSource>,
        forbidden_sources: BTreeSet<ForbiddenKnowledgeSource>,
        perception_scope: ScopeFilter,
        belief_scope: ScopeFilter,
        observation_scope: ScopeFilter,
        debug_non_diegetic: bool,
        mut provenance_entries: Vec<KnowledgeProvenanceEntry>,
        mut actor_known_workplaces: Vec<ActorKnownWorkplaceFact>,
        mut actor_known_food_sources: Vec<ActorKnownFoodSourceFact>,
        mut actor_known_sleep_affordances: Vec<ActorKnownSleepAffordanceFact>,
        mut actor_known_routes: Vec<ActorKnownRouteFact>,
    ) -> Self {
        provenance_entries.sort();
        provenance_entries.dedup();
        actor_known_workplaces.sort();
        actor_known_workplaces.dedup();
        actor_known_food_sources.sort();
        actor_known_food_sources.dedup();
        actor_known_sleep_affordances.sort();
        actor_known_sleep_affordances.dedup();
        actor_known_routes.sort();
        actor_known_routes.dedup();
        let projection_schema_version = SchemaVersion::new(EPISTEMIC_RECORD_SCHEMA_V1).unwrap();
        let holder_known_context_id = HolderKnownContextId::new(format!(
            "hkc.{}.{}.{}",
            viewer_actor_id.as_str(),
            current_tick.value(),
            event_frontier
        ))
        .unwrap();
        let status = KnowledgeContextStatus::Current;
        let forbidden_truth_audit =
            ForbiddenTruthAudit::passed_excluding(forbidden_sources.clone());
        let holder_known_context_hash = compute_holder_known_context_hash(canonical_hash_inputs(
            &holder_known_context_id,
            &viewer_actor_id,
            &bound_actor_id,
            mode,
            current_tick,
            event_frontier,
            &allowed_sources,
            &forbidden_sources,
            &perception_scope,
            &belief_scope,
            &observation_scope,
            &projection_schema_version,
            debug_non_diegetic,
            &provenance_entries,
            &actor_known_workplaces,
            &actor_known_food_sources,
            &actor_known_sleep_affordances,
            &actor_known_routes,
            &forbidden_truth_audit,
            status,
        ))
        .hash;

        Self {
            viewer_actor_id,
            bound_actor_id,
            mode,
            current_tick,
            event_frontier,
            allowed_sources,
            forbidden_sources,
            perception_scope,
            belief_scope,
            observation_scope,
            projection_schema_version,
            debug_non_diegetic,
            holder_known_context_id,
            holder_known_context_hash,
            provenance_entries,
            actor_known_workplaces,
            actor_known_food_sources,
            actor_known_sleep_affordances,
            actor_known_routes,
            forbidden_truth_audit,
            status,
        }
    }

    pub fn debug(
        viewer_actor_id: ActorId,
        current_tick: SimTick,
        _capability: &DebugCapability,
    ) -> Self {
        Self::seal(
            viewer_actor_id.clone(),
            viewer_actor_id,
            ViewMode::Debug,
            current_tick,
            0,
            embodied_allowed_sources(),
            forbidden_sources(),
            ScopeFilter::DebugAll,
            ScopeFilter::DebugAll,
            ScopeFilter::DebugAll,
            true,
            baseline_embodied_provenance(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
    }

    pub fn permits_scope(&self, privacy_scope: &PrivacyScope) -> bool {
        match (self.mode, privacy_scope) {
            (ViewMode::Debug, _) => true,
            (ViewMode::Embodied, PrivacyScope::ActorPrivate(actor_id)) => {
                actor_id == &self.viewer_actor_id
            }
            (ViewMode::Embodied, PrivacyScope::PublicPlaceholder) => true,
            (ViewMode::Embodied, PrivacyScope::InstitutionPlaceholder(_)) => false,
        }
    }

    pub fn viewer_actor_id(&self) -> &ActorId {
        &self.viewer_actor_id
    }

    pub fn bound_actor_id(&self) -> &ActorId {
        &self.bound_actor_id
    }

    pub fn mode(&self) -> ViewMode {
        self.mode
    }

    pub fn current_tick(&self) -> SimTick {
        self.current_tick
    }

    pub fn event_frontier(&self) -> u64 {
        self.event_frontier
    }

    pub fn allowed_sources(&self) -> &BTreeSet<AllowedKnowledgeSource> {
        &self.allowed_sources
    }

    pub fn forbidden_sources(&self) -> &BTreeSet<ForbiddenKnowledgeSource> {
        &self.forbidden_sources
    }

    pub fn perception_scope(&self) -> &ScopeFilter {
        &self.perception_scope
    }

    pub fn belief_scope(&self) -> &ScopeFilter {
        &self.belief_scope
    }

    pub fn observation_scope(&self) -> &ScopeFilter {
        &self.observation_scope
    }

    pub fn projection_schema_version(&self) -> &SchemaVersion {
        &self.projection_schema_version
    }

    pub fn debug_non_diegetic(&self) -> bool {
        self.debug_non_diegetic
    }

    pub fn holder_known_context_id(&self) -> &HolderKnownContextId {
        &self.holder_known_context_id
    }

    pub fn holder_known_context_hash(&self) -> &HolderKnownContextHash {
        &self.holder_known_context_hash
    }

    pub fn provenance_entries(&self) -> &[KnowledgeProvenanceEntry] {
        &self.provenance_entries
    }

    pub fn actor_known_workplaces(&self) -> &[ActorKnownWorkplaceFact] {
        &self.actor_known_workplaces
    }

    pub fn actor_known_food_sources(&self) -> &[ActorKnownFoodSourceFact] {
        &self.actor_known_food_sources
    }

    pub fn actor_known_sleep_affordances(&self) -> &[ActorKnownSleepAffordanceFact] {
        &self.actor_known_sleep_affordances
    }

    pub fn actor_known_routes(&self) -> &[ActorKnownRouteFact] {
        &self.actor_known_routes
    }

    pub fn forbidden_truth_audit(&self) -> &ForbiddenTruthAudit {
        &self.forbidden_truth_audit
    }

    pub fn status(&self) -> KnowledgeContextStatus {
        self.status
    }
}

fn embodied_allowed_sources() -> BTreeSet<AllowedKnowledgeSource> {
    BTreeSet::from([
        AllowedKnowledgeSource::OwnDirectObservation,
        AllowedKnowledgeSource::OwnSearchOrTouchObservation,
        AllowedKnowledgeSource::OwnSoundObservation,
        AllowedKnowledgeSource::OwnAbsenceMarker,
        AllowedKnowledgeSource::OwnSourceBackedBelief,
    ])
}

fn forbidden_sources() -> BTreeSet<ForbiddenKnowledgeSource> {
    BTreeSet::from([
        ForbiddenKnowledgeSource::UnobservedEventLogTruth,
        ForbiddenKnowledgeSource::HiddenItemLocation,
        ForbiddenKnowledgeSource::OtherActorsPrivateBeliefs,
        ForbiddenKnowledgeSource::HumanDebugNotes,
        ForbiddenKnowledgeSource::PreviousPossessedActorKnowledge,
    ])
}

fn baseline_embodied_provenance() -> Vec<KnowledgeProvenanceEntry> {
    vec![
        KnowledgeProvenanceEntry::actor_known_source(
            KnowledgeProvenanceKind::Perception,
            AllowedKnowledgeSource::OwnDirectObservation,
            "context_source_policy",
        ),
        KnowledgeProvenanceEntry::actor_known_source(
            KnowledgeProvenanceKind::Observation,
            AllowedKnowledgeSource::OwnSearchOrTouchObservation,
            "context_source_policy",
        ),
        KnowledgeProvenanceEntry::actor_known_source(
            KnowledgeProvenanceKind::Observation,
            AllowedKnowledgeSource::OwnSoundObservation,
            "context_source_policy",
        ),
        KnowledgeProvenanceEntry::actor_known_source(
            KnowledgeProvenanceKind::Record,
            AllowedKnowledgeSource::OwnAbsenceMarker,
            "context_source_policy",
        ),
        KnowledgeProvenanceEntry::actor_known_source(
            KnowledgeProvenanceKind::Belief,
            AllowedKnowledgeSource::OwnSourceBackedBelief,
            "context_source_policy",
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn canonical_hash_inputs(
    holder_known_context_id: &HolderKnownContextId,
    viewer_actor_id: &ActorId,
    bound_actor_id: &ActorId,
    mode: ViewMode,
    current_tick: SimTick,
    event_frontier: u64,
    allowed_sources: &BTreeSet<AllowedKnowledgeSource>,
    forbidden_sources: &BTreeSet<ForbiddenKnowledgeSource>,
    perception_scope: &ScopeFilter,
    belief_scope: &ScopeFilter,
    observation_scope: &ScopeFilter,
    projection_schema_version: &SchemaVersion,
    debug_non_diegetic: bool,
    provenance_entries: &[KnowledgeProvenanceEntry],
    actor_known_workplaces: &[ActorKnownWorkplaceFact],
    actor_known_food_sources: &[ActorKnownFoodSourceFact],
    actor_known_sleep_affordances: &[ActorKnownSleepAffordanceFact],
    actor_known_routes: &[ActorKnownRouteFact],
    forbidden_truth_audit: &ForbiddenTruthAudit,
    status: KnowledgeContextStatus,
) -> Vec<String> {
    let mut lines = vec![
        format!("context_id={}", holder_known_context_id.as_str()),
        format!("viewer_actor={}", viewer_actor_id.as_str()),
        format!("bound_actor={}", bound_actor_id.as_str()),
        format!("mode={}", mode.stable_id()),
        format!("tick={}", current_tick.value()),
        format!("event_frontier={event_frontier}"),
        format!("perception_scope={}", perception_scope.canonical_key()),
        format!("belief_scope={}", belief_scope.canonical_key()),
        format!("observation_scope={}", observation_scope.canonical_key()),
        format!("schema={}", projection_schema_version.as_str()),
        format!("debug_non_diegetic={debug_non_diegetic}"),
        format!("audit_passed={}", forbidden_truth_audit.passed()),
        format!("status={}", status.stable_id()),
    ];

    lines.extend(
        allowed_sources
            .iter()
            .map(|source| format!("allowed={}", source.stable_id())),
    );
    lines.extend(
        forbidden_sources
            .iter()
            .map(|source| format!("forbidden={}", source.stable_id())),
    );
    lines.extend(
        forbidden_truth_audit
            .excluded_sources()
            .iter()
            .map(|source| format!("audit_excluded={}", source.stable_id())),
    );
    lines.extend(
        provenance_entries
            .iter()
            .map(|entry| format!("provenance={}", entry.canonical_key())),
    );
    lines.extend(
        actor_known_workplaces
            .iter()
            .map(|fact| format!("actor_known_workplace={}", fact.canonical_key())),
    );
    lines.extend(
        actor_known_food_sources
            .iter()
            .map(|fact| format!("actor_known_food_source={}", fact.canonical_key())),
    );
    lines.extend(
        actor_known_sleep_affordances
            .iter()
            .map(|fact| format!("actor_known_sleep_affordance={}", fact.canonical_key())),
    );
    lines.extend(
        actor_known_routes
            .iter()
            .map(|fact| format!("actor_known_route={}", fact.canonical_key())),
    );
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    fn actor_id(value: &str) -> ActorId {
        ActorId::new(value).unwrap()
    }

    #[test]
    fn embodied_context_has_expected_allowed_and_forbidden_sources() {
        let context = KnowledgeContext::embodied(actor_id("actor_tomas"), SimTick::new(5));

        assert_eq!(context.mode(), ViewMode::Embodied);
        assert_eq!(context.viewer_actor_id(), &actor_id("actor_tomas"));
        assert_eq!(context.bound_actor_id(), &actor_id("actor_tomas"));
        assert_eq!(context.event_frontier(), 0);
        assert!(context
            .allowed_sources()
            .contains(&AllowedKnowledgeSource::OwnDirectObservation));
        assert!(context
            .allowed_sources()
            .contains(&AllowedKnowledgeSource::OwnSearchOrTouchObservation));
        assert!(context
            .allowed_sources()
            .contains(&AllowedKnowledgeSource::OwnSoundObservation));
        assert!(context
            .allowed_sources()
            .contains(&AllowedKnowledgeSource::OwnAbsenceMarker));
        assert!(context
            .allowed_sources()
            .contains(&AllowedKnowledgeSource::OwnSourceBackedBelief));

        assert!(context
            .forbidden_sources()
            .contains(&ForbiddenKnowledgeSource::UnobservedEventLogTruth));
        assert!(context
            .forbidden_sources()
            .contains(&ForbiddenKnowledgeSource::HiddenItemLocation));
        assert!(context
            .forbidden_sources()
            .contains(&ForbiddenKnowledgeSource::OtherActorsPrivateBeliefs));
        assert!(context
            .forbidden_sources()
            .contains(&ForbiddenKnowledgeSource::HumanDebugNotes));
        assert!(context
            .forbidden_sources()
            .contains(&ForbiddenKnowledgeSource::PreviousPossessedActorKnowledge));
    }

    #[test]
    fn embodied_context_seals_id_hash_provenance_frontier_and_audit() {
        let context =
            KnowledgeContext::embodied_at_frontier(actor_id("actor_tomas"), SimTick::new(5), 11);

        assert_eq!(
            context.holder_known_context_id().as_str(),
            "hkc.actor_tomas.5.11"
        );
        assert!(context
            .holder_known_context_hash()
            .as_str()
            .starts_with("hkc1-"));
        assert_eq!(context.bound_actor_id(), &actor_id("actor_tomas"));
        assert_eq!(context.current_tick(), SimTick::new(5));
        assert_eq!(context.event_frontier(), 11);
        assert_eq!(context.status(), KnowledgeContextStatus::Current);
        assert!(!context.provenance_entries().is_empty());
        assert!(context.provenance_entries().iter().any(|entry| {
            entry.kind() == &KnowledgeProvenanceKind::Belief
                && entry.source() == AllowedKnowledgeSource::OwnSourceBackedBelief
        }));
        assert!(context.actor_known_workplaces().is_empty());
        assert!(context.forbidden_truth_audit().passed());
        for source in context.forbidden_sources().iter() {
            assert!(context
                .forbidden_truth_audit()
                .excluded_sources()
                .contains(source));
        }
    }

    #[test]
    fn embodied_context_can_seal_actor_known_workplace_facts() {
        let context = KnowledgeContext::embodied_at_frontier_with_workplaces(
            actor_id("actor_tomas"),
            SimTick::new(5),
            11,
            vec![ActorKnownWorkplaceFact::new(
                WorkplaceId::new("workplace_tomas").unwrap(),
                PlaceId::new("workshop_tomas").unwrap(),
                true,
                "routine_assignment_notice",
            )],
        );

        assert_eq!(context.actor_known_workplaces().len(), 1);
        assert_eq!(
            context.actor_known_workplaces()[0].workplace_id().as_str(),
            "workplace_tomas"
        );
        assert!(context
            .holder_known_context_hash()
            .as_str()
            .starts_with("hkc1-"));
    }

    #[test]
    fn embodied_context_hash_is_deterministic_and_input_sensitive() {
        let first =
            KnowledgeContext::embodied_at_frontier(actor_id("actor_tomas"), SimTick::new(5), 11);
        let second =
            KnowledgeContext::embodied_at_frontier(actor_id("actor_tomas"), SimTick::new(5), 11);
        let changed_actor =
            KnowledgeContext::embodied_at_frontier(actor_id("actor_elena"), SimTick::new(5), 11);
        let changed_frontier =
            KnowledgeContext::embodied_at_frontier(actor_id("actor_tomas"), SimTick::new(5), 12);

        assert_eq!(
            first.holder_known_context_hash(),
            second.holder_known_context_hash()
        );
        assert_ne!(
            first.holder_known_context_hash(),
            changed_actor.holder_known_context_hash()
        );
        assert_ne!(
            first.holder_known_context_hash(),
            changed_frontier.holder_known_context_hash()
        );
    }

    #[test]
    fn embodied_scope_permits_only_viewer_private_and_public_records() {
        let context = KnowledgeContext::embodied(actor_id("actor_tomas"), SimTick::ZERO);

        assert!(context.permits_scope(&PrivacyScope::ActorPrivate(actor_id("actor_tomas"))));
        assert!(context.permits_scope(&PrivacyScope::PublicPlaceholder));
        assert!(!context.permits_scope(&PrivacyScope::ActorPrivate(actor_id("actor_mara"))));
        assert!(
            !context.permits_scope(&PrivacyScope::InstitutionPlaceholder(
                "ledger_placeholder".to_string()
            ))
        );
    }

    #[test]
    fn embodied_context_cannot_be_mutated_into_debug_scope_after_seal() {
        let context = KnowledgeContext::embodied(actor_id("actor_tomas"), SimTick::ZERO);

        assert_eq!(context.mode(), ViewMode::Embodied);
        assert!(!context.permits_scope(&PrivacyScope::ActorPrivate(actor_id("actor_mara"))));
        assert!(!context.debug_non_diegetic());
    }

    #[test]
    fn debug_context_is_non_diegetic_and_can_inspect_all_scopes() {
        let capability = DebugCapability::mint();
        let context = KnowledgeContext::debug(actor_id("actor_tomas"), SimTick::ZERO, &capability);

        assert_eq!(context.mode(), ViewMode::Debug);
        assert!(context.debug_non_diegetic());
        assert!(context.permits_scope(&PrivacyScope::ActorPrivate(actor_id("actor_mara"))));
        assert!(context.permits_scope(&PrivacyScope::InstitutionPlaceholder(
            "ledger_placeholder".to_string()
        )));
    }
}
