use std::collections::BTreeSet;

use crate::epistemics::observation::{PrivacyScope, EPISTEMIC_RECORD_SCHEMA_V1};
use crate::ids::{ActorId, SchemaVersion};
use crate::time::SimTick;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ViewMode {
    Embodied,
    Debug,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AllowedKnowledgeSource {
    OwnDirectObservation,
    OwnSearchOrTouchObservation,
    OwnSoundObservation,
    OwnAbsenceMarker,
    OwnSourceBackedBelief,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ForbiddenKnowledgeSource {
    UnobservedEventLogTruth,
    HiddenItemLocation,
    OtherActorsPrivateBeliefs,
    HumanDebugNotes,
    PreviousPossessedActorKnowledge,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScopeFilter {
    ActorPrivate(ActorId),
    DebugAll,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KnowledgeContext {
    pub viewer_actor_id: ActorId,
    pub mode: ViewMode,
    pub current_tick: SimTick,
    pub allowed_sources: BTreeSet<AllowedKnowledgeSource>,
    pub forbidden_sources: BTreeSet<ForbiddenKnowledgeSource>,
    pub perception_scope: ScopeFilter,
    pub belief_scope: ScopeFilter,
    pub observation_scope: ScopeFilter,
    pub projection_schema_version: SchemaVersion,
    pub debug_non_diegetic: bool,
}

impl KnowledgeContext {
    pub fn embodied(viewer_actor_id: ActorId, current_tick: SimTick) -> Self {
        let actor_scope = ScopeFilter::ActorPrivate(viewer_actor_id.clone());
        Self {
            viewer_actor_id,
            mode: ViewMode::Embodied,
            current_tick,
            allowed_sources: embodied_allowed_sources(),
            forbidden_sources: forbidden_sources(),
            perception_scope: actor_scope.clone(),
            belief_scope: actor_scope.clone(),
            observation_scope: actor_scope,
            projection_schema_version: SchemaVersion::new(EPISTEMIC_RECORD_SCHEMA_V1).unwrap(),
            debug_non_diegetic: false,
        }
    }

    pub fn debug(viewer_actor_id: ActorId, current_tick: SimTick) -> Self {
        Self {
            viewer_actor_id,
            mode: ViewMode::Debug,
            current_tick,
            allowed_sources: embodied_allowed_sources(),
            forbidden_sources: forbidden_sources(),
            perception_scope: ScopeFilter::DebugAll,
            belief_scope: ScopeFilter::DebugAll,
            observation_scope: ScopeFilter::DebugAll,
            projection_schema_version: SchemaVersion::new(EPISTEMIC_RECORD_SCHEMA_V1).unwrap(),
            debug_non_diegetic: true,
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn actor_id(value: &str) -> ActorId {
        ActorId::new(value).unwrap()
    }

    #[test]
    fn embodied_context_has_expected_allowed_and_forbidden_sources() {
        let context = KnowledgeContext::embodied(actor_id("actor_tomas"), SimTick::new(5));

        assert_eq!(context.mode, ViewMode::Embodied);
        assert!(context
            .allowed_sources
            .contains(&AllowedKnowledgeSource::OwnDirectObservation));
        assert!(context
            .allowed_sources
            .contains(&AllowedKnowledgeSource::OwnSearchOrTouchObservation));
        assert!(context
            .allowed_sources
            .contains(&AllowedKnowledgeSource::OwnSoundObservation));
        assert!(context
            .allowed_sources
            .contains(&AllowedKnowledgeSource::OwnAbsenceMarker));
        assert!(context
            .allowed_sources
            .contains(&AllowedKnowledgeSource::OwnSourceBackedBelief));

        assert!(context
            .forbidden_sources
            .contains(&ForbiddenKnowledgeSource::UnobservedEventLogTruth));
        assert!(context
            .forbidden_sources
            .contains(&ForbiddenKnowledgeSource::HiddenItemLocation));
        assert!(context
            .forbidden_sources
            .contains(&ForbiddenKnowledgeSource::OtherActorsPrivateBeliefs));
        assert!(context
            .forbidden_sources
            .contains(&ForbiddenKnowledgeSource::HumanDebugNotes));
        assert!(context
            .forbidden_sources
            .contains(&ForbiddenKnowledgeSource::PreviousPossessedActorKnowledge));
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
    fn debug_context_is_non_diegetic_and_can_inspect_all_scopes() {
        let context = KnowledgeContext::debug(actor_id("actor_tomas"), SimTick::ZERO);

        assert_eq!(context.mode, ViewMode::Debug);
        assert!(context.debug_non_diegetic);
        assert!(context.permits_scope(&PrivacyScope::ActorPrivate(actor_id("actor_mara"))));
        assert!(context.permits_scope(&PrivacyScope::InstitutionPlaceholder(
            "ledger_placeholder".to_string()
        )));
    }
}
