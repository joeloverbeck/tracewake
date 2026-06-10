use std::collections::{BTreeMap, BTreeSet};

use crate::epistemics::belief::{Belief, HolderKind};
use crate::epistemics::contradiction::Contradiction;
use crate::epistemics::knowledge_context::{KnowledgeContext, ViewMode};
use crate::epistemics::observation::{Observation, SourceRef, EPISTEMIC_RECORD_SCHEMA_V1};
use crate::ids::{
    ActorId, BeliefId, ContentManifestId, ContradictionId, EpistemicProjectionVersion, EventId,
    ObservationId, SchemaVersion,
};
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EpistemicProjection {
    observations_by_id: BTreeMap<ObservationId, Observation>,
    observations_by_actor: BTreeMap<ActorId, BTreeSet<ObservationId>>,
    beliefs_by_id: BTreeMap<BeliefId, Belief>,
    beliefs_by_holder: BTreeMap<ActorId, BTreeSet<BeliefId>>,
    contradictions_by_id: BTreeMap<ContradictionId, Contradiction>,
    contradictions_by_holder: BTreeMap<ActorId, BTreeSet<ContradictionId>>,
    notebook_entries_by_actor: BTreeMap<ActorId, BTreeSet<NotebookEntry>>,
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

    pub(crate) fn belief_count_for_actor(&self, actor_id: &ActorId) -> usize {
        self.beliefs_by_holder
            .get(actor_id)
            .map_or(0, BTreeSet::len)
    }

    pub fn has_belief(&self, belief_id: &BeliefId) -> bool {
        self.beliefs_by_id.contains_key(belief_id)
    }

    pub fn is_empty(&self) -> bool {
        self.observations_by_id.is_empty()
            && self.beliefs_by_id.is_empty()
            && self.contradictions_by_id.is_empty()
            && self.notebook_entries_by_actor.is_empty()
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

        let checksum = EpistemicProjectionChecksum::from_canonical_lines(&lines);
        EpistemicProjectionChecksumReport {
            checksum,
            canonical_input: lines,
        }
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
    use crate::epistemics::belief::{Belief, HolderKind, Stance};
    use crate::epistemics::knowledge_context::KnowledgeContext;
    use crate::epistemics::observation::{Confidence, SourceRef};
    use crate::epistemics::proposition::Proposition;
    use crate::ids::{ContainerId, EventId, ItemId};
    use crate::location::Location;
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
