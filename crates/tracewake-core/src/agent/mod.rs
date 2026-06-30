//! Agent cognition substrate for ordinary-life simulation.

pub mod actor_known;
pub mod candidate;
pub mod decision;
pub mod generation;
pub mod htn;
pub mod intention;
pub mod methods;
pub mod need;
pub mod no_human_surface;
pub mod perception;
pub mod planner;
pub mod routine;
pub mod routine_continuation;
pub mod trace;
pub mod transaction;

pub use actor_known::{
    ActorKnownFact, ActorKnownPlanningContext, ActorKnownProvenance, SourceEventIds,
    SourceEventIdsError,
};
pub use candidate::{
    ApplicabilityResult, CandidateGoal, CandidateGoalSource, GoalKind, GoalPriority,
};
pub use decision::{
    select_goal_and_trace, ActorKnownInputRef, ActorKnownInputSourceClass, DecisionInput,
    DecisionSelection, IntentionLifecycleEffect,
};
pub use generation::{
    generate_candidate_goals, generate_candidate_goals_from_agent_state,
    need_crossing_triggers_candidate_reevaluation, CandidateGenerationInput,
    CandidateGenerationOutput, LiveCandidateGenerationInput,
};
pub use htn::{
    mark_mid_method_failure, select_method_from_templates, select_phase3a_method, MethodSelection,
    MethodSelectionFailure,
};
pub use intention::{
    ActorIntentions, Intention, IntentionSource, IntentionStatus, IntentionTransitionError,
};
pub use methods::{all_steps_are_proposals, family_for_goal, phase3a_routine_templates};
pub use need::{
    NeedBand, NeedChangeCause, NeedKind, NeedParseError, NeedPressure, NeedState,
    NeedThresholdCrossing, ThresholdDirection,
};
pub use no_human_surface::{
    NoHumanActorKnownSurfaceBuilder, NoHumanActorKnownSurfaceRequest, SealedActorKnownSurface,
};
pub use perception::{current_place_knowledge_context, current_place_perception_events};
pub(crate) use perception::{
    record_current_place_perception, record_current_place_perception_and_project,
};
pub use planner::{
    derive_hidden_truth_audit, plan_local_actions, ActorKnownPlanningState, LocalPlan,
    LocalPlanFailure, LocalPlanRequest, LocalPlanTrace, PlannedProposal, PlannerGoal,
    DEFAULT_PLANNER_BUDGET,
};
pub use routine::{
    RoutineCondition, RoutineExecution, RoutineFamily, RoutineStep, RoutineStepParseError,
    RoutineStepProposal, RoutineStepStatus, RoutineTemplate, RoutineTemplateError,
};
pub use routine_continuation::{resolve_routine_step_follow_on, RoutineContinuationResolution};
pub use trace::{
    BlockerCategory, BlockerCode, DecisionOutcome, DecisionTrace, DecisionTraceRecord,
    DecisionTraceRecordParseError, HiddenTruthAudit, RejectedDecisionItem, ResponsibleLayer,
    StuckDiagnostic, StuckDiagnosticParseError, StuckDiagnosticRecord, StuckResultingStatus,
    TypedDiagnosticFields,
};
pub use transaction::{
    ActorDecisionProposalOutcome, ActorDecisionTransaction, ActorDecisionTransactionInput,
    ActorDecisionTransactionOutcome, SealedProposal, SelectedGoalBundle,
};
