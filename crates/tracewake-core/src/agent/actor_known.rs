use std::collections::{BTreeMap, BTreeSet};

use crate::agent::HiddenTruthAudit;
use crate::epistemics::EpistemicProjection;
use crate::ids::{ActorId, ContainerId, PlaceId, WorkplaceId};
use crate::state::AgentState;
use crate::time::SimTick;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActorKnownFact {
    stable_id: String,
    semantic_kind: String,
    value: String,
    tick: Option<SimTick>,
    actor_id: ActorId,
    provenance: ActorKnownProvenance,
}

impl ActorKnownFact {
    pub fn observed_now(
        actor_id: ActorId,
        stable_id: impl Into<String>,
        value: impl Into<String>,
        source: impl Into<String>,
        tick: Option<SimTick>,
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
        )
    }

    pub fn remembered_belief(
        actor_id: ActorId,
        stable_id: impl Into<String>,
        value: impl Into<String>,
        source: impl Into<String>,
        tick: Option<SimTick>,
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
        )
    }

    pub fn routine_assignment(
        actor_id: ActorId,
        stable_id: impl Into<String>,
        value: impl Into<String>,
        source: impl Into<String>,
        tick: Option<SimTick>,
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
        )
    }

    pub fn fixture_possibility(
        actor_id: ActorId,
        stable_id: impl Into<String>,
        value: impl Into<String>,
        source: impl Into<String>,
        tick: Option<SimTick>,
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
        )
    }

    pub fn unproven(stable_id: impl Into<String>, note: impl Into<String>) -> Self {
        Self::new(
            ActorId::new("actor_unknown").unwrap(),
            stable_id,
            "unproven",
            "",
            None,
            ActorKnownProvenance::UnprovenPhysicalTruth { note: note.into() },
        )
    }

    fn new(
        actor_id: ActorId,
        stable_id: impl Into<String>,
        semantic_kind: impl Into<String>,
        value: impl Into<String>,
        tick: Option<SimTick>,
        provenance: ActorKnownProvenance,
    ) -> Self {
        Self {
            stable_id: stable_id.into(),
            semantic_kind: semantic_kind.into(),
            value: value.into(),
            tick,
            actor_id,
            provenance,
        }
    }

    pub fn stable_id(&self) -> &str {
        &self.stable_id
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
    #[allow(clippy::too_many_arguments)]
    pub fn from_observed_parts(
        actor_id: ActorId,
        current_place_id: PlaceId,
        known_edges: BTreeMap<PlaceId, BTreeSet<PlaceId>>,
        known_closed_doors: BTreeMap<(PlaceId, PlaceId), String>,
        known_containers_by_place: BTreeMap<PlaceId, BTreeSet<ContainerId>>,
        known_food_sources: BTreeSet<String>,
        known_sleep_places: BTreeSet<PlaceId>,
        known_workplaces: BTreeMap<WorkplaceId, PlaceId>,
        facts: Vec<ActorKnownFact>,
    ) -> Self {
        let mut context = Self {
            actor_id,
            current_place_id,
            known_edges,
            known_closed_doors,
            known_containers_by_place,
            known_food_sources,
            known_sleep_places,
            known_workplaces,
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

    pub fn add_actor_known_fact(&mut self, fact: ActorKnownFact) {
        self.facts.push(fact);
        self.sort_facts();
    }

    pub fn extend_actor_known_facts(&mut self, facts: impl IntoIterator<Item = ActorKnownFact>) {
        self.facts.extend(facts);
        self.sort_facts();
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
        proof_notes.sort();
        proof_notes.dedup();
        let actor_known_only = self
            .facts
            .iter()
            .chain(request_facts.iter())
            .all(ActorKnownFact::is_actor_known);
        HiddenTruthAudit {
            actor_known_only,
            notes: format!("planner proof_sources={}", proof_notes.join(",")),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VisibleLocalPlanningState {
    current_place_id: PlaceId,
    visible_edges: BTreeMap<PlaceId, BTreeSet<PlaceId>>,
    visible_closed_doors: BTreeMap<(PlaceId, PlaceId), String>,
    visible_containers_by_place: BTreeMap<PlaceId, BTreeSet<ContainerId>>,
    visible_food_sources: BTreeSet<String>,
    visible_sleep_places: BTreeSet<PlaceId>,
    visible_workplaces: BTreeMap<WorkplaceId, PlaceId>,
}

impl VisibleLocalPlanningState {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        current_place_id: PlaceId,
        visible_edges: BTreeMap<PlaceId, BTreeSet<PlaceId>>,
        visible_closed_doors: BTreeMap<(PlaceId, PlaceId), String>,
        visible_containers_by_place: BTreeMap<PlaceId, BTreeSet<ContainerId>>,
        visible_food_sources: BTreeSet<String>,
        visible_sleep_places: BTreeSet<PlaceId>,
        visible_workplaces: BTreeMap<WorkplaceId, PlaceId>,
    ) -> Self {
        Self {
            current_place_id,
            visible_edges,
            visible_closed_doors,
            visible_containers_by_place,
            visible_food_sources,
            visible_sleep_places,
            visible_workplaces,
        }
    }
}

pub fn build_actor_known_planning_state(
    actor_id: &ActorId,
    epistemic_projection: &EpistemicProjection,
    agent_state: &AgentState,
    visible_local: &VisibleLocalPlanningState,
) -> ActorKnownPlanningContext {
    observe_visible_local(
        actor_id,
        Some(epistemic_projection),
        agent_state,
        visible_local,
    )
}

pub fn build_actor_known_planning_state_with_projection_limitation(
    actor_id: &ActorId,
    agent_state: &AgentState,
    visible_local: &VisibleLocalPlanningState,
) -> ActorKnownPlanningContext {
    observe_visible_local(actor_id, None, agent_state, visible_local)
}

pub fn observe_visible_local(
    actor_id: &ActorId,
    epistemic_projection: Option<&EpistemicProjection>,
    agent_state: &AgentState,
    visible_local: &VisibleLocalPlanningState,
) -> ActorKnownPlanningContext {
    let mut facts = vec![ActorKnownFact::observed_now(
        actor_id.clone(),
        "actor_current_place_visible",
        visible_local.current_place_id.as_str(),
        format!(
            "visible_local:current_place:{}",
            visible_local.current_place_id.as_str()
        ),
        None,
    )];
    if agent_state.needs_by_actor.contains_key(actor_id) {
        facts.push(ActorKnownFact::remembered_belief(
            actor_id.clone(),
            "agent_needs_present",
            "needs_present",
            "agent_state:needs_present",
            None,
        ));
    }
    if let Some(epistemic_projection) = epistemic_projection {
        let actor_belief_count = epistemic_projection
            .beliefs_by_holder
            .get(actor_id)
            .map_or(0, BTreeSet::len);
        facts.push(ActorKnownFact::remembered_belief(
            actor_id.clone(),
            "actor_belief_projection",
            actor_belief_count.to_string(),
            format!("epistemic_projection:actor_beliefs:{actor_belief_count}"),
            None,
        ));
    } else {
        facts.push(ActorKnownFact::remembered_belief(
            actor_id.clone(),
            "actor_belief_projection_limitation",
            "not_supplied",
            "no_human_day:typed_projection_limitation",
            None,
        ));
    }
    for (from, tos) in &visible_local.visible_edges {
        for to in tos {
            facts.push(ActorKnownFact::observed_now(
                actor_id.clone(),
                "known_route_surface",
                format!("{}->{}", from.as_str(), to.as_str()),
                format!("visible_local:edge:{}->{}", from.as_str(), to.as_str()),
                None,
            ));
        }
    }
    for food_source in &visible_local.visible_food_sources {
        facts.push(ActorKnownFact::observed_now(
            actor_id.clone(),
            "actor_knows_food_source",
            food_source,
            format!("visible_local:food:{food_source}"),
            None,
        ));
    }
    for sleep_place in &visible_local.visible_sleep_places {
        facts.push(ActorKnownFact::observed_now(
            actor_id.clone(),
            "actor_knows_sleep_place",
            sleep_place.as_str(),
            format!("visible_local:sleep_place:{}", sleep_place.as_str()),
            None,
        ));
    }
    for (workplace_id, place_id) in &visible_local.visible_workplaces {
        facts.push(ActorKnownFact::observed_now(
            actor_id.clone(),
            "actor_knows_workplace",
            format!("{}@{}", workplace_id.as_str(), place_id.as_str()),
            format!(
                "visible_local:workplace:{}@{}",
                workplace_id.as_str(),
                place_id.as_str()
            ),
            None,
        ));
        facts.push(ActorKnownFact::routine_assignment(
            actor_id.clone(),
            "workplace_assignment_active",
            workplace_id.as_str(),
            format!(
                "visible_local:workplace_assignment:{}",
                workplace_id.as_str()
            ),
            None,
        ));
    }
    let mut context = ActorKnownPlanningContext {
        actor_id: actor_id.clone(),
        current_place_id: visible_local.current_place_id.clone(),
        known_edges: visible_local.visible_edges.clone(),
        known_closed_doors: visible_local.visible_closed_doors.clone(),
        known_containers_by_place: visible_local.visible_containers_by_place.clone(),
        known_food_sources: visible_local.visible_food_sources.clone(),
        known_sleep_places: visible_local.visible_sleep_places.clone(),
        known_workplaces: visible_local.visible_workplaces.clone(),
        facts,
    };
    context.sort_facts();
    context
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
    use crate::ids::ContentManifestId;

    fn actor_id() -> ActorId {
        ActorId::new("actor_tomas").unwrap()
    }

    fn place_id(value: &str) -> PlaceId {
        PlaceId::new(value).unwrap()
    }

    fn visible_local() -> VisibleLocalPlanningState {
        VisibleLocalPlanningState::new(
            place_id("home"),
            BTreeMap::from([(place_id("home"), BTreeSet::from([place_id("street")]))]),
            BTreeMap::new(),
            BTreeMap::new(),
            BTreeSet::from(["food_soup".to_string()]),
            BTreeSet::from([place_id("home")]),
            BTreeMap::new(),
        )
    }

    #[test]
    fn visible_local_observation_stamps_every_planner_fact_with_provenance() {
        let actor_id = actor_id();
        let epistemic_projection =
            EpistemicProjection::new(ContentManifestId::new("manifest_test").unwrap());
        let context = observe_visible_local(
            &actor_id,
            Some(&epistemic_projection),
            &AgentState::default(),
            &visible_local(),
        );

        assert!(!context.actor_known_facts().is_empty());
        assert!(context
            .actor_known_facts()
            .iter()
            .all(ActorKnownFact::is_actor_known));
        assert!(context
            .proof_sources()
            .iter()
            .any(|source| source.contains("observed_now:visible_local:food:food_soup")));
    }

    #[test]
    fn hidden_truth_audit_reads_provenance_graph_not_stored_boolean() {
        let epistemic_projection =
            EpistemicProjection::new(ContentManifestId::new("manifest_test").unwrap());
        let mut context = observe_visible_local(
            &actor_id(),
            Some(&epistemic_projection),
            &AgentState::default(),
            &visible_local(),
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
        context.add_actor_known_fact(unproven);
        let audit = context.audit_with(&[]);
        assert!(!audit.actor_known_only);
        assert!(audit
            .notes
            .contains("unproven:caller supplied physical-only food"));
    }
}
