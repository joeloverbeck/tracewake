//! Agent cognition substrate for ordinary-life simulation.

pub mod candidate;
pub mod intention;
pub mod need;

pub use candidate::{
    ApplicabilityResult, CandidateGoal, CandidateGoalSource, GoalKind, GoalPriority,
};
pub use intention::{
    ActorIntentions, Intention, IntentionSource, IntentionStatus, IntentionTransitionError,
};
pub use need::{
    NeedBand, NeedChangeCause, NeedKind, NeedParseError, NeedPressure, NeedState,
    NeedThresholdCrossing, ThresholdDirection,
};
