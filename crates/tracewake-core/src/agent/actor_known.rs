use std::collections::{BTreeMap, BTreeSet};

use crate::agent::HiddenTruthAudit;
use crate::ids::{ActorId, ContainerId, EventId, PlaceId, WorkplaceId};
use crate::time::SimTick;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceEventIds {
    ids: Vec<EventId>,
}

impl SourceEventIds {
    pub fn from_event(event: &crate::events::EventEnvelope) -> Self {
        Self {
            ids: vec![event.event_id.clone()],
        }
    }

    pub fn checked(mut ids: Vec<EventId>) -> Result<Self, SourceEventIdsError> {
        ids.sort();
        ids.dedup();
        if ids.is_empty() {
            Err(SourceEventIdsError::Empty)
        } else {
            Ok(Self { ids })
        }
    }

    pub fn as_slice(&self) -> &[EventId] {
        &self.ids
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceEventIdsError {
    Empty,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActorKnownFact {
    stable_id: String,
    semantic_kind: String,
    value: String,
    tick: Option<SimTick>,
    actor_id: ActorId,
    provenance: ActorKnownProvenance,
    source_event_ids: SourceEventIds,
}

impl ActorKnownFact {
    /// Actor-known facts require typed source-event witnesses.
    ///
    /// ```compile_fail
    /// use tracewake_core::agent::ActorKnownFact;
    /// use tracewake_core::ids::ActorId;
    ///
    /// let _fact = ActorKnownFact::observed_now(
    ///     ActorId::new("actor_tomas").unwrap(),
    ///     "actor_knows_food_source",
    ///     "food_stew",
    ///     "test",
    ///     None,
    ///     Vec::new(),
    /// );
    /// ```
    pub fn observed_now(
        actor_id: ActorId,
        stable_id: impl Into<String>,
        value: impl Into<String>,
        source: impl Into<String>,
        tick: Option<SimTick>,
        source_event_ids: SourceEventIds,
    ) -> Self {
        Self::new(
            actor_id,
            stable_id,
            "observed_now",
            value,
            tick,
            ActorKnownProvenance::ObservedNow {
                source: source.into(),
            },
            source_event_ids,
        )
    }

    pub fn remembered_belief(
        actor_id: ActorId,
        stable_id: impl Into<String>,
        value: impl Into<String>,
        source: impl Into<String>,
        tick: Option<SimTick>,
        source_event_ids: SourceEventIds,
    ) -> Self {
        Self::new(
            actor_id,
            stable_id,
            "remembered_belief",
            value,
            tick,
            ActorKnownProvenance::RememberedBelief {
                source: source.into(),
            },
            source_event_ids,
        )
    }

    pub fn routine_assignment(
        actor_id: ActorId,
        stable_id: impl Into<String>,
        value: impl Into<String>,
        source: impl Into<String>,
        tick: Option<SimTick>,
        source_event_ids: SourceEventIds,
    ) -> Self {
        Self::new(
            actor_id,
            stable_id,
            "routine_assignment",
            value,
            tick,
            ActorKnownProvenance::RoutineAssignment {
                source: source.into(),
            },
            source_event_ids,
        )
    }

    pub fn fixture_possibility(
        actor_id: ActorId,
        stable_id: impl Into<String>,
        value: impl Into<String>,
        source: impl Into<String>,
        tick: Option<SimTick>,
        source_event_ids: SourceEventIds,
    ) -> Self {
        Self::new(
            actor_id,
            stable_id,
            "fixture_possibility",
            value,
            tick,
            ActorKnownProvenance::FixturePossibility {
                source: source.into(),
            },
            source_event_ids,
        )
    }

    pub fn unproven(stable_id: impl Into<String>, note: impl Into<String>) -> Self {
        let source_event_ids = SourceEventIds::checked(vec![EventId::new(
            "event_unproven_physical_truth_rejected_test_only",
        )
        .unwrap()])
        .unwrap();
        Self::new(
            ActorId::new("actor_unknown").unwrap(),
            stable_id,
            "unproven",
            "",
            None,
            ActorKnownProvenance::UnprovenPhysicalTruth { note: note.into() },
            source_event_ids,
        )
    }

    fn new(
        actor_id: ActorId,
        stable_id: impl Into<String>,
        semantic_kind: impl Into<String>,
        value: impl Into<String>,
        tick: Option<SimTick>,
        provenance: ActorKnownProvenance,
        source_event_ids: SourceEventIds,
    ) -> Self {
        Self {
            stable_id: stable_id.into(),
            semantic_kind: semantic_kind.into(),
            value: value.into(),
            tick,
            actor_id,
            provenance,
            source_event_ids,
        }
    }

    pub fn stable_id(&self) -> &str {
        &self.stable_id
    }

    pub fn semantic_kind(&self) -> &str {
        &self.semantic_kind
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn tick(&self) -> Option<SimTick> {
        self.tick
    }

    pub fn actor_id(&self) -> &ActorId {
        &self.actor_id
    }

    pub fn is_actor_known(&self) -> bool {
        self.provenance.actor_known()
    }

    pub fn proof_note(&self) -> String {
        format!("{}={}", self.stable_id, self.provenance.note())
    }

    pub fn provenance(&self) -> &ActorKnownProvenance {
        &self.provenance
    }

    pub fn source_event_ids(&self) -> &[EventId] {
        self.source_event_ids.as_slice()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActorKnownProvenance {
    ObservedNow { source: String },
    RememberedBelief { source: String },
    RoutineAssignment { source: String },
    FixturePossibility { source: String },
    PipelineValidationTruth(RestrictedProvenance),
    DebugOmniscience(RestrictedProvenance),
    UnprovenPhysicalTruth { note: String },
}

impl ActorKnownProvenance {
    fn actor_known(&self) -> bool {
        matches!(
            self,
            Self::ObservedNow { .. }
                | Self::RememberedBelief { .. }
                | Self::RoutineAssignment { .. }
                | Self::FixturePossibility { .. }
        )
    }

    fn note(&self) -> String {
        match self {
            Self::ObservedNow { source } => format!("observed_now:{source}"),
            Self::RememberedBelief { source } => format!("remembered_belief:{source}"),
            Self::RoutineAssignment { source } => format!("routine_assignment:{source}"),
            Self::FixturePossibility { source } => format!("fixture_possibility:{source}"),
            Self::PipelineValidationTruth(source) => {
                format!("pipeline_validation_truth:{}", source.note)
            }
            Self::DebugOmniscience(source) => format!("debug_omniscience:{}", source.note),
            Self::UnprovenPhysicalTruth { note } => format!("unproven:{note}"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RestrictedProvenance {
    note: String,
}

/// Holder-specific planning context for ordinary actor cognition.
///
/// External crates can read this context through accessors, but cannot
/// construct it from raw authoritative truth.
///
/// ```compile_fail
/// use tracewake_core::agent::ActorKnownPlanningContext;
/// use tracewake_core::ids::{ActorId, PlaceId};
///
/// let _context = ActorKnownPlanningContext::from_observed_parts(
///     ActorId::new("actor_mara").unwrap(),
///     PlaceId::new("home_mara").unwrap(),
///     Default::default(),
///     Default::default(),
///     Default::default(),
///     Default::default(),
///     Default::default(),
///     Default::default(),
///     Vec::new(),
/// );
/// ```
///
/// ```compile_fail
/// use tracewake_core::agent::ActorKnownPlanningContext;
/// use tracewake_core::state::{NeedModelState, PhysicalState};
///
/// let _context = ActorKnownPlanningContext::from(PhysicalState::empty(NeedModelState::new(5, 3)));
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActorKnownPlanningContext {
    actor_id: ActorId,
    current_place_id: PlaceId,
    known_edges: BTreeMap<PlaceId, BTreeSet<PlaceId>>,
    known_closed_doors: BTreeMap<(PlaceId, PlaceId), String>,
    known_containers_by_place: BTreeMap<PlaceId, BTreeSet<ContainerId>>,
    known_food_sources: BTreeSet<String>,
    known_sleep_places: BTreeSet<PlaceId>,
    known_workplaces: BTreeMap<WorkplaceId, PlaceId>,
    facts: Vec<ActorKnownFact>,
}

impl ActorKnownPlanningContext {
    #[allow(dead_code)]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_observed_parts(
        actor_id: ActorId,
        current_place_id: PlaceId,
        _known_edges: BTreeMap<PlaceId, BTreeSet<PlaceId>>,
        _known_closed_doors: BTreeMap<(PlaceId, PlaceId), String>,
        _known_containers_by_place: BTreeMap<PlaceId, BTreeSet<ContainerId>>,
        _known_food_sources: BTreeSet<String>,
        _known_sleep_places: BTreeSet<PlaceId>,
        _known_workplaces: BTreeMap<WorkplaceId, PlaceId>,
        facts: Vec<ActorKnownFact>,
    ) -> Self {
        let derived = DerivedActorKnownFields::from_facts(&facts);
        let mut context = Self {
            actor_id,
            current_place_id,
            known_edges: derived.known_edges,
            known_closed_doors: derived.known_closed_doors,
            known_containers_by_place: derived.known_containers_by_place,
            known_food_sources: derived.known_food_sources,
            known_sleep_places: derived.known_sleep_places,
            known_workplaces: derived.known_workplaces,
            facts,
        };
        context.sort_facts();
        context
    }

    pub fn actor_id(&self) -> &ActorId {
        &self.actor_id
    }

    pub fn current_place_id(&self) -> &PlaceId {
        &self.current_place_id
    }

    pub fn known_edges(&self) -> &BTreeMap<PlaceId, BTreeSet<PlaceId>> {
        &self.known_edges
    }

    pub fn known_closed_doors(&self) -> &BTreeMap<(PlaceId, PlaceId), String> {
        &self.known_closed_doors
    }

    pub fn known_containers_by_place(&self) -> &BTreeMap<PlaceId, BTreeSet<ContainerId>> {
        &self.known_containers_by_place
    }

    pub fn known_food_sources(&self) -> &BTreeSet<String> {
        &self.known_food_sources
    }

    pub fn known_sleep_places(&self) -> &BTreeSet<PlaceId> {
        &self.known_sleep_places
    }

    pub fn known_workplaces(&self) -> &BTreeMap<WorkplaceId, PlaceId> {
        &self.known_workplaces
    }

    pub fn actor_known_facts(&self) -> &[ActorKnownFact] {
        &self.facts
    }

    pub fn proof_sources(&self) -> Vec<String> {
        self.facts.iter().map(ActorKnownFact::proof_note).collect()
    }

    fn sort_facts(&mut self) {
        self.facts.sort_by(|left, right| {
            left.stable_id()
                .cmp(right.stable_id())
                .then_with(|| left.proof_note().cmp(&right.proof_note()))
        });
    }

    pub fn audit_with(&self, request_facts: &[ActorKnownFact]) -> HiddenTruthAudit {
        let mut proof_notes = self.proof_sources();
        proof_notes.extend(request_facts.iter().map(ActorKnownFact::proof_note));
        let structured_gaps = self.structured_fact_gaps();
        proof_notes.extend(
            structured_gaps
                .iter()
                .map(|gap| format!("structured_context_without_fact:{gap}")),
        );
        proof_notes.sort();
        proof_notes.dedup();
        let actor_known_only = self
            .facts
            .iter()
            .chain(request_facts.iter())
            .all(ActorKnownFact::is_actor_known)
            && structured_gaps.is_empty();
        HiddenTruthAudit {
            actor_known_only,
            notes: format!("planner proof_sources={}", proof_notes.join(",")),
        }
    }

    fn structured_fact_gaps(&self) -> Vec<String> {
        let mut gaps = Vec::new();
        for food_source in &self.known_food_sources {
            if !self.has_fact("actor_knows_food_source", food_source) {
                gaps.push(format!("known_food_sources:{food_source}"));
            }
        }
        for sleep_place in &self.known_sleep_places {
            if !self.has_fact("actor_knows_sleep_place", sleep_place.as_str()) {
                gaps.push(format!("known_sleep_places:{}", sleep_place.as_str()));
            }
        }
        for (workplace_id, place_id) in &self.known_workplaces {
            let value = format!("{}@{}", workplace_id.as_str(), place_id.as_str());
            if !self.has_fact("actor_knows_workplace", &value) {
                gaps.push(format!("known_workplaces:{value}"));
            }
        }
        for (from, tos) in &self.known_edges {
            for to in tos {
                let value = format!("{}->{}", from.as_str(), to.as_str());
                if !self.has_fact("known_route_surface", &value) {
                    gaps.push(format!("known_edges:{value}"));
                }
            }
        }
        for ((from, to), door_id) in &self.known_closed_doors {
            let edge_value = format!("{}->{}", from.as_str(), to.as_str());
            let door_value = format!("{edge_value}@{door_id}");
            if !self.has_fact("known_closed_door_surface", &door_value) {
                gaps.push(format!("known_closed_doors:{door_value}"));
            }
        }
        for (place_id, containers) in &self.known_containers_by_place {
            for container_id in containers {
                let value = format!("{}@{}", container_id.as_str(), place_id.as_str());
                if !self.has_fact("known_container_surface", &value) {
                    gaps.push(format!("known_containers_by_place:{value}"));
                }
            }
        }
        gaps.sort();
        gaps
    }

    fn has_fact(&self, stable_id: &str, value: &str) -> bool {
        self.facts.iter().any(|fact| {
            fact.stable_id() == stable_id && fact.value() == value && fact.is_actor_known()
        })
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct DerivedActorKnownFields {
    known_edges: BTreeMap<PlaceId, BTreeSet<PlaceId>>,
    known_closed_doors: BTreeMap<(PlaceId, PlaceId), String>,
    known_containers_by_place: BTreeMap<PlaceId, BTreeSet<ContainerId>>,
    known_food_sources: BTreeSet<String>,
    known_sleep_places: BTreeSet<PlaceId>,
    known_workplaces: BTreeMap<WorkplaceId, PlaceId>,
}

impl DerivedActorKnownFields {
    fn from_facts(facts: &[ActorKnownFact]) -> Self {
        let mut derived = Self::default();
        for fact in facts.iter().filter(|fact| fact.is_actor_known()) {
            match fact.stable_id() {
                "known_route_surface" => {
                    if let Some((from, to)) =
                        fact.value().split_once("->").and_then(|(from, to)| {
                            Some((PlaceId::new(from).ok()?, PlaceId::new(to).ok()?))
                        })
                    {
                        derived.known_edges.entry(from).or_default().insert(to);
                    }
                }
                "known_closed_door_surface" => {
                    if let Some((edge, door_id)) = fact.value().split_once('@') {
                        if let Some((from, to)) = edge.split_once("->").and_then(|(from, to)| {
                            Some((PlaceId::new(from).ok()?, PlaceId::new(to).ok()?))
                        }) {
                            derived
                                .known_closed_doors
                                .insert((from, to), door_id.to_string());
                        }
                    }
                }
                "known_container_surface" => {
                    if let Some((container_id, place_id)) = fact.value().split_once('@') {
                        if let (Ok(container_id), Ok(place_id)) =
                            (ContainerId::new(container_id), PlaceId::new(place_id))
                        {
                            derived
                                .known_containers_by_place
                                .entry(place_id)
                                .or_default()
                                .insert(container_id);
                        }
                    }
                }
                "actor_knows_food_source" => {
                    derived.known_food_sources.insert(fact.value().to_string());
                }
                "actor_knows_sleep_place" => {
                    if let Ok(place_id) = PlaceId::new(fact.value()) {
                        derived.known_sleep_places.insert(place_id);
                    }
                }
                "actor_knows_workplace" => {
                    if let Some((workplace_id, place_id)) = fact.value().split_once('@') {
                        if let (Ok(workplace_id), Ok(place_id)) =
                            (WorkplaceId::new(workplace_id), PlaceId::new(place_id))
                        {
                            derived.known_workplaces.insert(workplace_id, place_id);
                        }
                    }
                }
                _ => {}
            }
        }
        derived
    }
}

pub fn derive_hidden_truth_audit(
    context: &ActorKnownPlanningContext,
    request_facts: &[ActorKnownFact],
) -> HiddenTruthAudit {
    context.audit_with(request_facts)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn actor_id() -> ActorId {
        ActorId::new("actor_tomas").unwrap()
    }

    fn place_id(value: &str) -> PlaceId {
        PlaceId::new(value).unwrap()
    }

    fn source() -> SourceEventIds {
        SourceEventIds::checked(vec![EventId::new("event_test_source").unwrap()]).unwrap()
    }

    fn observed_fact(stable_id: &str, value: &str) -> ActorKnownFact {
        ActorKnownFact::observed_now(actor_id(), stable_id, value, "test", None, source())
    }

    #[test]
    fn hidden_truth_audit_reads_provenance_graph_not_stored_boolean() {
        let context = ActorKnownPlanningContext::from_observed_parts(
            actor_id(),
            place_id("home"),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeSet::new(),
            BTreeSet::new(),
            BTreeMap::new(),
            vec![ActorKnownFact::observed_now(
                actor_id(),
                "actor_current_place_visible",
                "home",
                "test:current_place",
                None,
                source(),
            )],
        );
        assert!(context.audit_with(&[]).actor_known_only);

        let unproven = ActorKnownFact::unproven(
            "actor_knows_food_source",
            "caller supplied physical-only food",
        );
        assert!(
            !context
                .audit_with(std::slice::from_ref(&unproven))
                .actor_known_only
        );
        let context_with_unproven = ActorKnownPlanningContext::from_observed_parts(
            actor_id(),
            place_id("home"),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeSet::new(),
            BTreeSet::new(),
            BTreeMap::new(),
            vec![unproven],
        );
        let audit = context_with_unproven.audit_with(&[]);
        assert!(!audit.actor_known_only);
        assert!(audit
            .notes
            .contains("unproven:caller supplied physical-only food"));
    }

    #[test]
    fn structured_actor_known_fields_are_derived_from_facts() {
        let actor_id = actor_id();
        let source = source();
        let context = ActorKnownPlanningContext::from_observed_parts(
            actor_id.clone(),
            place_id("home"),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeSet::new(),
            BTreeSet::new(),
            BTreeMap::new(),
            vec![
                ActorKnownFact::observed_now(
                    actor_id.clone(),
                    "known_route_surface",
                    "home->market",
                    "test:route",
                    None,
                    source.clone(),
                ),
                ActorKnownFact::observed_now(
                    actor_id.clone(),
                    "known_container_surface",
                    "pantry@home",
                    "test:container",
                    None,
                    source.clone(),
                ),
                ActorKnownFact::observed_now(
                    actor_id.clone(),
                    "actor_knows_food_source",
                    "food_stew",
                    "test:food",
                    None,
                    source.clone(),
                ),
                ActorKnownFact::remembered_belief(
                    actor_id.clone(),
                    "actor_knows_sleep_place",
                    "home",
                    "test:sleep",
                    None,
                    source.clone(),
                ),
                ActorKnownFact::routine_assignment(
                    actor_id,
                    "actor_knows_workplace",
                    "workshop@market",
                    "test:workplace",
                    None,
                    source,
                ),
            ],
        );

        assert!(context
            .known_edges()
            .get(&place_id("home"))
            .is_some_and(|edges| edges.contains(&place_id("market"))));
        assert!(context
            .known_containers_by_place()
            .get(&place_id("home"))
            .is_some_and(|containers| containers.contains(&ContainerId::new("pantry").unwrap())));
        assert!(context.known_food_sources().contains("food_stew"));
        assert!(context.known_sleep_places().contains(&place_id("home")));
        assert_eq!(
            context
                .known_workplaces()
                .get(&WorkplaceId::new("workshop").unwrap()),
            Some(&place_id("market"))
        );
        assert!(context.audit_with(&[]).actor_known_only);
    }

    #[test]
    fn actor_known_facts_are_sorted_by_stable_id_then_proof_note() {
        let context = ActorKnownPlanningContext::from_observed_parts(
            actor_id(),
            place_id("home"),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeSet::new(),
            BTreeSet::new(),
            BTreeMap::new(),
            vec![
                ActorKnownFact::observed_now(
                    actor_id(),
                    "known_route_surface",
                    "home->market",
                    "source:z",
                    None,
                    source(),
                ),
                ActorKnownFact::remembered_belief(
                    actor_id(),
                    "actor_knows_food_source",
                    "food_stew",
                    "source:b",
                    None,
                    source(),
                ),
                ActorKnownFact::observed_now(
                    actor_id(),
                    "actor_knows_food_source",
                    "food_stew",
                    "source:a",
                    None,
                    source(),
                ),
            ],
        );

        let ordered = context
            .actor_known_facts()
            .iter()
            .map(ActorKnownFact::proof_note)
            .collect::<Vec<_>>();
        assert_eq!(
            ordered,
            vec![
                "actor_knows_food_source=observed_now:source:a",
                "actor_knows_food_source=remembered_belief:source:b",
                "known_route_surface=observed_now:source:z",
            ]
        );
    }

    #[test]
    fn structured_fact_gaps_require_exact_actor_known_stable_id_and_value() {
        let mut context = ActorKnownPlanningContext::from_observed_parts(
            actor_id(),
            place_id("home"),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeSet::new(),
            BTreeSet::new(),
            BTreeMap::new(),
            vec![
                observed_fact("actor_knows_food_source", "food_other"),
                observed_fact("actor_knows_sleep_place", "food_hidden"),
                ActorKnownFact::unproven("actor_knows_food_source", "food_hidden"),
            ],
        );
        context.known_food_sources.insert("food_hidden".to_string());

        let gaps = context.structured_fact_gaps();
        let audit = context.audit_with(&[]);

        assert_eq!(gaps, vec!["known_food_sources:food_hidden".to_string()]);
        assert!(!audit.actor_known_only);
        assert!(audit
            .notes
            .contains("structured_context_without_fact:known_food_sources:food_hidden"));
    }

    #[test]
    fn structured_context_without_matching_fact_is_not_constructed() {
        let context = ActorKnownPlanningContext::from_observed_parts(
            actor_id(),
            place_id("home"),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeSet::from(["food_hidden".to_string()]),
            BTreeSet::new(),
            BTreeMap::new(),
            Vec::new(),
        );

        let audit = context.audit_with(&[]);

        assert!(context.known_food_sources().is_empty());
        assert!(audit.actor_known_only);
        assert!(!audit.notes.contains("food_hidden"));
    }
}
