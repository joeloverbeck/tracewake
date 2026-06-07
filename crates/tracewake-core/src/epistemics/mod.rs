//! Epistemic substrate types for actor-relative claims and beliefs.

pub mod belief;
pub mod contradiction;
pub mod knowledge_context;
pub mod observation;
pub mod projection;
pub mod proposition;

pub use belief::{Belief, BeliefBuildError, BeliefDraft, HolderKind, Stance};
pub use contradiction::{Contradiction, ContradictionKind};
pub use knowledge_context::{
    AllowedKnowledgeSource, ForbiddenKnowledgeSource, KnowledgeContext, ScopeFilter, ViewMode,
};
pub use observation::{
    Channel, Confidence, ConfidenceError, Observation, ObservationSubject, ObservationTarget,
    PrivacyScope, SourceRef, TickWindow,
};
pub use projection::{
    EpistemicProjection, EpistemicProjectionChecksum, EpistemicProjectionChecksumReport,
    NotebookEntry, ProjectionEventRange,
};
pub use proposition::{Proposition, PropositionReferenceError};
