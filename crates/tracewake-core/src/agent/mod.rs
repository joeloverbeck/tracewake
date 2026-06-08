//! Agent cognition substrate for ordinary-life simulation.

pub mod actor_known;
pub mod candidate;
pub mod decision;
pub mod generation;
pub mod htn;
pub mod intention;
pub mod methods;
pub mod need;
pub mod planner;
pub mod routine;
pub mod trace;

pub use actor_known::{
    ActorKnownFact, ActorKnownPlanningContext, ActorKnownProvenance, VisibleLocalPlanningState,
};
pub use candidate::{
    ApplicabilityResult, CandidateGoal, CandidateGoalSource, GoalKind, GoalPriority,
};
pub use decision::{
    select_goal_and_trace, DecisionInput, DecisionSelection, IntentionLifecycleEffect,
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
pub use planner::{
    build_actor_known_planning_state, derive_hidden_truth_audit, plan_local_actions,
    ActorKnownPlanningState, LocalPlan, LocalPlanFailure, LocalPlanRequest, LocalPlanTrace,
    PlannedProposal, PlannerGoal, DEFAULT_PLANNER_BUDGET,
};
pub use routine::{
    RoutineCondition, RoutineExecution, RoutineFamily, RoutineStep, RoutineStepParseError,
    RoutineStepProposal, RoutineStepStatus, RoutineTemplate, RoutineTemplateError,
};
pub use trace::{
    BlockerCategory, DecisionOutcome, DecisionTrace, DecisionTraceRecord,
    DecisionTraceRecordParseError, HiddenTruthAudit, RejectedDecisionItem, StuckDiagnostic,
    StuckDiagnosticParseError, StuckDiagnosticRecord, StuckResultingStatus,
};
