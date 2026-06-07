//! Epistemic substrate types for actor-relative claims and beliefs.

pub mod belief;
pub mod contradiction;
pub mod observation;
pub mod proposition;

pub use belief::{Belief, BeliefBuildError, BeliefDraft, HolderKind, Stance};
pub use contradiction::{Contradiction, ContradictionKind};
pub use observation::{
    Channel, Confidence, ConfidenceError, Observation, ObservationSubject, ObservationTarget,
    PrivacyScope, SourceRef, TickWindow,
};
pub use proposition::{Proposition, PropositionReferenceError};
