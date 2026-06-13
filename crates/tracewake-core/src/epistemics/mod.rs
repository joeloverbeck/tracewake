//! Epistemic substrate types for actor-relative claims and beliefs.

pub mod belief;
pub mod contradiction;
pub mod knowledge_basis;
pub mod knowledge_context;
pub mod observation;
pub mod projection;
pub mod proposition;

pub use belief::{Belief, BeliefBuildError, BeliefDraft, HolderKind, Stance};
pub use contradiction::{
    detect_expected_absences, Contradiction, ContradictionKind, ExpectedAbsenceDetection,
};
pub use knowledge_basis::actor_has_source_backed_support;
pub use knowledge_context::{
    ActorKnownCarriedItemFact, ActorKnownContainerFact, ActorKnownCurrentPlaceFact,
    ActorKnownDoorFact, ActorKnownFoodSourceFact, ActorKnownItemFact, ActorKnownLocalActorFact,
    ActorKnownRouteFact, ActorKnownSleepAffordanceFact, ActorKnownWorkplaceFact,
    AllowedKnowledgeSource, ForbiddenKnowledgeSource, ForbiddenTruthAudit, KnowledgeContext,
    KnowledgeContextStatus, KnowledgeProvenanceEntry, KnowledgeProvenanceKind, ScopeFilter,
    ViewMode,
};
pub use observation::{
    Channel, Confidence, ConfidenceError, Observation, ObservationSubject, ObservationTarget,
    PrivacyScope, SourceRef, TickWindow,
};
pub use projection::{
    ActorKnownProjectionAccessibilityScope, ActorKnownProjectionEmbodiedScope,
    ActorKnownProjectionFreshness, ActorKnownProjectionKindPolicy, ActorKnownProjectionPolicy,
    ActorKnownProjectionRecord, ActorKnownProjectionSource, ClassifiedActorKnownProjectionRecord,
    EpistemicProjection, EpistemicProjectionChecksum, EpistemicProjectionChecksumReport,
    NotebookEntry, ProjectionEventRange,
};
pub use proposition::{Proposition, PropositionReferenceError};
