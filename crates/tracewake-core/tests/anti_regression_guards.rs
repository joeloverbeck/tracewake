mod support;

use std::collections::{BTreeMap, BTreeSet};
use std::fs;

use support::{AgentSeed, PhysicalSeed};
use tracewake_core::events::log::EventLog;
use tracewake_core::events::{EventCause, EventEnvelope, EventKind, PayloadField, EVENT_SCHEMA_V1};
use tracewake_core::ids::{ActionId, ActorId, ContentManifestId, EventId, ProposalId};
use tracewake_core::need_accounting::{
    classify_actor_tick_regimes, classify_actor_tick_regimes_with_start, TickRegimeCounts,
};
use tracewake_core::scheduler::{OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId};
use tracewake_core::time::SimTick;

const SCHEDULER_RS: &str = include_str!("../src/scheduler.rs");
const ACTOR_KNOWN_RS: &str = include_str!("../src/agent/actor_known.rs");
const GENERATION_RS: &str = include_str!("../src/agent/generation.rs");
const NO_HUMAN_SURFACE_RS: &str = include_str!("../src/agent/no_human_surface.rs");
const PERCEPTION_RS: &str = include_str!("../src/agent/perception.rs");
const TRANSACTION_RS: &str = include_str!("../src/agent/transaction.rs");
const ROUTINE_CONTINUATION_RS: &str = include_str!("../src/agent/routine_continuation.rs");
const DECISION_RS: &str = include_str!("../src/agent/decision.rs");
const PIPELINE_RS: &str = include_str!("../src/actions/pipeline.rs");
const HTN_RS: &str = include_str!("../src/agent/htn.rs");
const PLANNER_RS: &str = include_str!("../src/agent/planner.rs");
const STATE_RS: &str = include_str!("../src/state.rs");
const CHECKSUM_RS: &str = include_str!("../src/checksum.rs");
const EVENTS_MOD_RS: &str = include_str!("../src/events/mod.rs");
const EVENTS_APPLY_RS: &str = include_str!("../src/events/apply.rs");
const EVENTS_ENVELOPE_RS: &str = include_str!("../src/events/envelope.rs");
const EVENTS_MUTATION_RS: &str = include_str!("../src/events/mutation.rs");
const EPISTEMIC_PROJECTION_RS: &str = include_str!("../src/epistemics/projection.rs");
const EAT_RS: &str = include_str!("../src/actions/defs/eat.rs");
const CONTINUE_ROUTINE_RS: &str = include_str!("../src/actions/defs/continue_routine.rs");
const MOVEMENT_RS: &str = include_str!("../src/actions/defs/movement.rs");
const WAIT_RS: &str = include_str!("../src/actions/defs/wait.rs");
const OPENCLOSE_RS: &str = include_str!("../src/actions/defs/openclose.rs");
const CHECKCONTAINER_RS: &str = include_str!("../src/actions/defs/checkcontainer.rs");
const TAKEPLACE_RS: &str = include_str!("../src/actions/defs/takeplace.rs");
const SLEEP_RS: &str = include_str!("../src/actions/defs/sleep.rs");
const TIME_RS: &str = include_str!("../src/time.rs");
const WORK_RS: &str = include_str!("../src/actions/defs/work.rs");
const ACTIONS_REGISTRY_RS: &str = include_str!("../src/actions/registry.rs");
const ACTIONS_REPORT_RS: &str = include_str!("../src/actions/report.rs");
const PROJECTIONS_RS: &str = include_str!("../src/projections.rs");
const RUNTIME_SESSION_RS: &str = include_str!("../src/runtime/session.rs");
const VIEW_MODELS_RS: &str = include_str!("../src/view_models.rs");
const REBUILD_RS: &str = include_str!("../src/replay/rebuild.rs");
const REPORT_RS: &str = include_str!("../src/replay/report.rs");
const GENERATIVE_LOCK_RS: &str = include_str!("generative_lock.rs");
const HIDDEN_TRUTH_GATES_RS: &str = include_str!("hidden_truth_gates.rs");
const ANTI_REGRESSION_GUARDS_RS: &str = include_str!("anti_regression_guards.rs");
const NEGATIVE_FIXTURE_RUNNER_RS: &str = include_str!("negative_fixture_runner.rs");
const SUPPORT_ACCEPTANCE_STATUS_MANIFEST_RS: &str =
    include_str!("support/acceptance_status_manifest.rs");
const SUPPORT_GENERATIVE_RS: &str = include_str!("support/generative.rs");
const SUPPORT_MOD_RS: &str = include_str!("support/mod.rs");
const CONTENT_LOAD_RS: &str = include_str!("../src/content/load.rs");
const CONTENT_FIXTURE_HIDDEN_FOOD_CLOSED_CONTAINER_RS: &str =
    include_str!("../../tracewake-content/src/fixtures/hidden_food_closed_container_001.rs");
const CONTENT_FIXTURE_HIDDEN_ROUTE_EDGE_RS: &str =
    include_str!("../../tracewake-content/src/fixtures/hidden_route_edge_001.rs");
const CONTENT_GOLDEN_FIXTURES_RUN_RS: &str =
    include_str!("../../tracewake-content/tests/golden_fixtures_run.rs");
const TUI_APP_RS: &str = include_str!("../../tracewake-tui/src/app.rs");
const TUI_RENDER_RS: &str = include_str!("../../tracewake-tui/src/render.rs");
const MUTANTS_TOML: &str = include_str!("../../../.cargo/mutants.toml");
const MUTANTS_BASELINE_MISSES: &str = include_str!("../../../.cargo/mutants-baseline-misses.txt");
const MUTANTS_BASELINE_LEDGER: &str =
    include_str!("../../../archive/reports/0020_mutants_baseline_disposition.md");
const ACCEPTANCE_0021_REPORT: &str =
    include_str!("../../../archive/reports/0021_ord_life_cert_scoped_acceptance.md");
const ACCEPTANCE_0022_REPORT: &str =
    include_str!("../../../archive/reports/0022_ord_life_cert_scoped_acceptance.md");
const ACCEPTANCE_0023_REPORT: &str =
    include_str!("../../../archive/reports/0023_ord_life_cert_scoped_acceptance.md");
const ACCEPTANCE_0024_REPORT: &str =
    include_str!("../../../archive/reports/0024_ord_life_cert_scoped_acceptance.md");
const ACCEPTANCE_0025_REPORT: &str =
    include_str!("../../../archive/reports/0025_ord_life_cert_scoped_acceptance.md");
const CI_YML: &str = include_str!("../../../.github/workflows/ci.yml");
const CI_WORKFLOW_GUARDS_RS: &str = include_str!("ci_workflow_guards.rs");

struct BannedApiToken {
    token: &'static str,
    reason: &'static str,
}

struct NondeterminismAllowlistEntry {
    path: &'static str,
    token: &'static str,
    rationale: &'static str,
    responsible_layer: &'static str,
}

struct SchedulerMarkerAllowlistEntry {
    snippet: &'static str,
    rationale: &'static str,
    responsible_layer: &'static str,
}

struct TruthAccessorAllowlistEntry {
    path: &'static str,
    token: &'static str,
    rationale: &'static str,
    responsible_layer: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WorkspaceSourceClass {
    GuardedLayer,
    Exempt { rationale: &'static str },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct WorkspaceSourceClassification {
    path: &'static str,
    class: WorkspaceSourceClass,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct DependencyEntry {
    manifest_path: String,
    section: String,
    dependency: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct TypedColumnClosureExemption {
    map_name: &'static str,
    anchor: &'static str,
    typed_columns: &'static [&'static str],
    rationale: &'static str,
}

struct EmbodiedSurfaceFieldProducer {
    struct_name: &'static str,
    field_name: &'static str,
    source_path: &'static str,
    producer_snippet: &'static str,
    cite: &'static str,
    rationale: &'static str,
}

struct PanicAllowlistEntry {
    path: &'static str,
    token: &'static str,
    rationale: &'static str,
}

struct ApplyMutatorAllowlistEntry {
    path: &'static str,
    rationale: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct EmbodiedSurfaceField {
    struct_name: String,
    field_name: String,
    type_name: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MetaLockRouting {
    SharedScan,
    BehaviorAssertion,
    TicketOwnedDebt { ticket: &'static str },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct MetaLockRegistryEntry {
    lock_id: &'static str,
    negative_id: &'static str,
    routing: MetaLockRouting,
    witness_min: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct MetaLockCensusExemption {
    test_name: &'static str,
    rationale: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct MutationBaselineDelta {
    from_count: usize,
    from_hash: u64,
    to_count: usize,
    to_hash: u64,
}

const EMBODIED_SURFACE_FIELD_PRODUCERS: &[EmbodiedSurfaceFieldProducer] =
    &[
        EmbodiedSurfaceFieldProducer {
            struct_name: "EmbodiedViewModel",
            field_name: "notebook",
            source_path: "tracewake-core/src/runtime/session.rs",
            producer_snippet: "view.set_notebook(Some(build_notebook_view",
            cite: "docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md",
            rationale: "The core runtime builds the embodied shell and attaches the actor-known notebook from the same sealed view context.",
        },
        EmbodiedSurfaceFieldProducer {
            struct_name: "EmbodiedViewModel",
            field_name: "debug_available",
            source_path: "tracewake-core/src/runtime/session.rs",
            producer_snippet: "view.set_debug_available(self.controller_debug_available_for(controller_id, actor_id));",
            cite: "specs/0021_PHASE_3A_POSSESSION_REBIND_HYGIENE_GUARD_VACUITY_CLOSURE_HARNESS_PROVENANCE_FIDELITY_AND_REJECT_LOUDLY_REPLAY_POSTURE_HARDENING_SPEC.md",
            rationale: "Core derives debug availability from the live controller binding for the viewed actor.",
        },
        EmbodiedSurfaceFieldProducer {
            struct_name: "EmbodiedViewModel",
            field_name: "actor_known_interval_summary",
            source_path: "tracewake-core/src/runtime/session.rs",
            producer_snippet: "view.set_actor_known_interval_summary(actor_known_interval_summary);",
            cite: "archive/specs/0047_TUI_AUTHORITATIVE_WORLD_ADVANCE_DURATION_COMPLETION_AND_ACTOR_KNOWN_INTERVAL_SUMMARIES_SPEC.md",
            rationale: "Core builds the sealed embodied shell and attaches the last completed advance summary constructed from source-bearing interval inputs.",
        },
        EmbodiedSurfaceFieldProducer {
            struct_name: "ActionAvailability",
            field_name: "debug_only_diagnostics",
            source_path: "tracewake-tui/src/render.rs",
            producer_snippet: "availability.debug_only_diagnostics()",
            cite: "specs/0021_PHASE_3A_POSSESSION_REBIND_HYGIENE_GUARD_VACUITY_CLOSURE_HARNESS_PROVENANCE_FIDELITY_AND_REJECT_LOUDLY_REPLAY_POSTURE_HARDENING_SPEC.md",
            rationale: "Current production action availability keeps actor-safe provenance populated; the TUI render boundary consumes debug-only diagnostics through the accessor, proving the field is not orphaned at its definition site.",
        },
    ];

const TYPED_COLUMN_CLOSURE_EXEMPTIONS: &[TypedColumnClosureExemption] = &[
    TypedColumnClosureExemption {
        map_name: "needs_by_actor",
        anchor: "apply_need_delta",
        typed_columns: &[
            "actor_id",
            "need_kind",
            "delta",
            "elapsed_ticks",
            "cause_kind",
            "cause_action_id",
        ],
        rationale: "NeedDeltaApplied materializes typed need columns and does not retain arbitrary semantic payload fields.",
    },
    TypedColumnClosureExemption {
        map_name: "need_tick_charges",
        anchor: "assert_single_tick_delta_charge",
        typed_columns: &["actor_id", "need_kind", "elapsed_ticks", "cause_kind"],
        rationale: "Need tick charges derive from typed elapsed-tick accounting columns; no additional payload fields are materialized.",
    },
    TypedColumnClosureExemption {
        map_name: "intentions",
        anchor: "apply_intention_started",
        typed_columns: &[
            "intention_id",
            "actor_id",
            "status",
            "source",
            "candidate_goal_id",
            "selected_goal_id",
            "selected_routine_method",
            "routine_template_id",
            "current_step",
            "durability_level",
            "start_tick",
            "decision_trace_id",
            "trace_id",
        ],
        rationale: "IntentionStarted builds the typed Intention record from closed columns rather than retaining event payload fields.",
    },
    TypedColumnClosureExemption {
        map_name: "intentions",
        anchor: "apply_intention_transition",
        typed_columns: &["intention_id", "status", "reason", "progress_tick", "current_step"],
        rationale: "Intention transition events update typed Intention lifecycle fields; unsupported free payload fields are not materialized.",
    },
    TypedColumnClosureExemption {
        map_name: "active_intention_by_actor",
        anchor: "apply_intention_started",
        typed_columns: &[
            "intention_id",
            "actor_id",
            "status",
            "source",
            "candidate_goal_id",
            "selected_goal_id",
            "selected_routine_method",
            "routine_template_id",
            "current_step",
            "durability_level",
            "start_tick",
            "decision_trace_id",
            "trace_id",
        ],
        rationale: "The active-intention index is a typed pointer derived from IntentionStarted columns.",
    },
    TypedColumnClosureExemption {
        map_name: "routine_executions",
        anchor: "apply_routine_step_transition",
        typed_columns: &[
            "routine_execution_id",
            "action_id",
            "progress_tick",
            "reason",
            "fallback_attempts",
        ],
        rationale: "Routine step transitions mutate typed RoutineExecution lifecycle columns and do not retain arbitrary semantic payload fields.",
    },
];

const PANIC_ALLOWLIST: &[PanicAllowlistEntry] = &[
    PanicAllowlistEntry {
        path: "crates/tracewake-core/src/events/apply.rs",
        token: ".expect(\"actor checked\")",
        rationale: "Payload location was matched against actor ownership before removing from the actor inventory.",
    },
    PanicAllowlistEntry {
        path: "crates/tracewake-core/src/events/apply.rs",
        token: ".expect(\"item location checked\")",
        rationale: "The item location variant was matched immediately before mutating its containing collection.",
    },
    PanicAllowlistEntry {
        path: "crates/tracewake-core/src/events/apply.rs",
        token: ".expect(\"place checked\")",
        rationale: "The place existence check returned earlier with ApplyError before mutating the place collection.",
    },
    PanicAllowlistEntry {
        path: "crates/tracewake-core/src/events/apply.rs",
        token: ".expect(\"container checked\")",
        rationale: "The container existence check returned earlier with ApplyError before mutating the container collection.",
    },
    PanicAllowlistEntry {
        path: "crates/tracewake-core/src/actions/defs/work.rs",
        token: ".expect(\"actor checked\")",
        rationale: "The work event builder validates actor presence before constructing actor-scoped event payloads.",
    },
    PanicAllowlistEntry {
        path: "crates/tracewake-core/src/scheduler.rs",
        token: ".expect(\"agent event is appendable\")",
        rationale: "Internal scheduler agent events are constructed with the current schema before append; log-derived failures use the typed try path.",
    },
    PanicAllowlistEntry {
        path: "crates/tracewake-core/src/scheduler.rs",
        token: ".expect(\"agent event applies to live state\")",
        rationale: "Internal lifecycle events are generated from live agent state; completion and stuck-diagnostic log-derived paths use typed error collection.",
    },
    PanicAllowlistEntry {
        path: "crates/tracewake-core/src/scheduler.rs",
        token: ".expect(\"no-human marker is versioned\")",
        rationale: "No-human process markers are static current-schema diagnostic events.",
    },
    PanicAllowlistEntry {
        path: "crates/tracewake-core/src/scheduler.rs",
        token: ".expect(\"stuck diagnostic is versioned\")",
        rationale: "Stuck diagnostic events are constructed with the current schema before typed agent application.",
    },
    PanicAllowlistEntry {
        path: "crates/tracewake-core/src/scheduler.rs",
        token: "EventId::new(format!(",
        rationale: "Scheduler event ids are deterministic strings built from typed ids, stable kinds, and ticks before append.",
    },
    PanicAllowlistEntry {
        path: "crates/tracewake-core/src/scheduler.rs",
        token: "EventEnvelope::new_caused_v1(",
        rationale: "Scheduler envelopes are built with current schema constants and typed causes before append/application.",
    },
    PanicAllowlistEntry {
        path: "crates/tracewake-core/src/scheduler.rs",
        token: "ProcessId::new(\"no_human_day\").unwrap()",
        rationale: "The no-human process id is a static literal used to stamp scheduler-originated events.",
    },
    PanicAllowlistEntry {
        path: "crates/tracewake-core/src/scheduler.rs",
        token: "ProcessId::new(\"no_human_advance\").unwrap()",
        rationale: "The no-human advance process id is a static literal used to stamp scheduler-originated process markers.",
    },
    PanicAllowlistEntry {
        path: "crates/tracewake-core/src/scheduler.rs",
        token: "ActionId::new(\"passive_need_delta\").unwrap()",
        rationale: "The passive-need action id is a static internal scheduler event label.",
    },
    PanicAllowlistEntry {
        path: "crates/tracewake-core/src/scheduler.rs",
        token: "ActionId::new(\"sleep_completed\").unwrap()",
        rationale: "The sleep-completion action id is a static internal scheduler lifecycle label.",
    },
    PanicAllowlistEntry {
        path: "crates/tracewake-core/src/scheduler.rs",
        token: "ActionId::new(\"work_block_completed\").unwrap()",
        rationale: "The work-completion action id is a static internal scheduler lifecycle label.",
    },
    PanicAllowlistEntry {
        path: "crates/tracewake-core/src/scheduler.rs",
        token: "ActionId::new(\"routine_step_completed\").unwrap()",
        rationale: "The routine-step-completed action id is a static scheduler lifecycle label.",
    },
    PanicAllowlistEntry {
        path: "crates/tracewake-core/src/scheduler.rs",
        token: "ActionId::new(\"decision_trace_recorded\").unwrap()",
        rationale: "The decision-trace action id is a static scheduler lifecycle label.",
    },
    PanicAllowlistEntry {
        path: "crates/tracewake-core/src/scheduler.rs",
        token: "ActionId::new(\"intention_started\").unwrap()",
        rationale: "The intention-started action id is a static scheduler lifecycle label.",
    },
    PanicAllowlistEntry {
        path: "crates/tracewake-core/src/scheduler.rs",
        token: "ActionId::new(\"intention_continued\").unwrap()",
        rationale: "The intention-continued action id is a static scheduler lifecycle label.",
    },
    PanicAllowlistEntry {
        path: "crates/tracewake-core/src/scheduler.rs",
        token: "ActionId::new(\"stuck_diagnostic_recorded\").unwrap()",
        rationale: "The stuck-diagnostic action id is a static scheduler diagnostic label.",
    },
    PanicAllowlistEntry {
        path: "crates/tracewake-core/src/scheduler.rs",
        token: "ActionId::new(kind.stable_id()).unwrap()",
        rationale: "No-human process marker kinds are closed enum variants with stable ids.",
    },
    PanicAllowlistEntry {
        path: "crates/tracewake-core/src/scheduler.rs",
        token: "ActionId::new(label).unwrap()",
        rationale: "Routine step event labels are internal constants selected by the scheduler call site.",
    },
    PanicAllowlistEntry {
        path: "crates/tracewake-core/src/scheduler.rs",
        token: "SemanticActionId::new(proposal.action_id.as_str()).unwrap()",
        rationale: "Routine step semantic ids are converted from an already-typed proposal ActionId.",
    },
    PanicAllowlistEntry {
        path: "crates/tracewake-core/src/scheduler.rs",
        token: "IntentionId::new(format!(",
        rationale: "Scheduler intention ids are deterministic strings built from typed actor/window/action material.",
    },
    PanicAllowlistEntry {
        path: "crates/tracewake-core/src/scheduler.rs",
        token: "CandidateGoalId::new(format!(",
        rationale: "Scheduler candidate-goal ids are deterministic strings built from typed actor/tick/action material.",
    },
    PanicAllowlistEntry {
        path: "crates/tracewake-core/src/scheduler.rs",
        token: "StuckDiagnosticId::new(format!(",
        rationale: "Scheduler stuck-diagnostic ids are deterministic strings built from closed diagnostic kinds and typed ids.",
    },
    PanicAllowlistEntry {
        path: "crates/tracewake-core/src/actions/defs/sleep.rs",
        token: "EventId::new(format!(",
        rationale: "Sleep completion and need event ids are deterministic strings built from the accepted sleep-start event id.",
    },
    PanicAllowlistEntry {
        path: "crates/tracewake-core/src/actions/defs/sleep.rs",
        token: "EventEnvelope::new_caused_v1(",
        rationale: "Sleep envelopes are built with current schema constants and typed proposal/event causes.",
    },
    PanicAllowlistEntry {
        path: "crates/tracewake-core/src/actions/defs/work.rs",
        token: "EventId::new(format!(",
        rationale: "Work completion and need event ids are deterministic strings built from the accepted work-start event id.",
    },
    PanicAllowlistEntry {
        path: "crates/tracewake-core/src/actions/defs/work.rs",
        token: "EventEnvelope::new_caused_v1(",
        rationale: "Work envelopes are built with current schema constants and typed proposal/event causes.",
    },
];

const BANNED_NONDETERMINISM_TOKENS: &[BannedApiToken] = &[
    BannedApiToken {
        token: "HashMap",
        reason: "randomized hash seeding can alter outcome iteration order; use BTreeMap",
    },
    BannedApiToken {
        token: "HashSet",
        reason: "randomized hash seeding can alter outcome iteration order; use BTreeSet",
    },
    BannedApiToken {
        token: "SystemTime",
        reason: "wall-clock time cannot be replayed; use SimTick and event material",
    },
    BannedApiToken {
        token: "Instant",
        reason: "wall-clock time cannot be replayed; use SimTick and event material",
    },
    BannedApiToken {
        token: "rand::",
        reason: "randomness must be seedable, scoped, recorded, and replayable",
    },
    BannedApiToken {
        token: "std::env::var",
        reason: "environment state is outside replay material; use content/config material",
    },
    BannedApiToken {
        token: "thread::spawn",
        reason: "thread scheduling is nondeterministic for outcome paths",
    },
    BannedApiToken {
        token: "std::thread::spawn",
        reason: "thread scheduling is nondeterministic for outcome paths",
    },
    BannedApiToken {
        token: "std::fs::",
        reason: "outcome paths must consume validated content, not ad hoc filesystem reads",
    },
    BannedApiToken {
        token: "std::fs::write",
        reason: "outcome paths must emit replayable events, not ad hoc filesystem writes",
    },
    BannedApiToken {
        token: "OpenOptions",
        reason: "outcome paths must not perform ad hoc filesystem reads or writes",
    },
    BannedApiToken {
        token: "File::open",
        reason: "outcome paths must consume validated content, not ad hoc filesystem reads",
    },
    BannedApiToken {
        token: "std::net::",
        reason: "network timing and responses cannot influence replay",
    },
    BannedApiToken {
        token: "TcpStream",
        reason: "network timing and responses cannot influence replay",
    },
    BannedApiToken {
        token: "UdpSocket",
        reason: "network timing and responses cannot influence replay",
    },
    BannedApiToken {
        token: "Command::new",
        reason: "process execution cannot influence deterministic outcomes",
    },
];

const NONDETERMINISM_ALLOWLIST: &[NondeterminismAllowlistEntry] = &[];

const SCHEDULER_MARKER_EVENT_ALLOWLIST: &[SchedulerMarkerAllowlistEntry] = &[
    SchedulerMarkerAllowlistEntry {
        snippet: "build_sleep_completion_events",
        rationale: "scheduler may complete previously accepted duration actions; initial sleep start still goes through the action pipeline",
        responsible_layer: "core/scheduler",
    },
    SchedulerMarkerAllowlistEntry {
        snippet: "build_work_completion_events",
        rationale: "scheduler may complete previously accepted duration actions; initial work start still goes through the action pipeline",
        responsible_layer: "core/scheduler",
    },
    SchedulerMarkerAllowlistEntry {
        snippet: "append_marker",
        rationale: "no-human process start/completion diagnostics are marker events, not primitive action dispatch",
        responsible_layer: "core/scheduler",
    },
    SchedulerMarkerAllowlistEntry {
        snippet: "append_and_apply_agent_event",
        rationale: "agent-stream routine, need, trace, and diagnostic records are replayable scheduler diagnostics, not physical primitive dispatch",
        responsible_layer: "core/scheduler",
    },
];

const APPLY_MUTATOR_ALLOWLIST: &[ApplyMutatorAllowlistEntry] = &[
    ApplyMutatorAllowlistEntry {
        path: "crates/tracewake-core/src/events/apply.rs",
        rationale: "defines the authoritative apply surface and may dispatch between world, agent, and epistemic event streams",
    },
    ApplyMutatorAllowlistEntry {
        path: "crates/tracewake-core/src/replay/rebuild.rs",
        rationale: "rebuilds canonical state from committed event logs during replay",
    },
    ApplyMutatorAllowlistEntry {
        path: "crates/tracewake-core/src/actions/pipeline.rs",
        rationale: "applies already accepted action events and performs cloned-state dry-run validation",
    },
    ApplyMutatorAllowlistEntry {
        path: "crates/tracewake-core/src/scheduler.rs",
        rationale: "applies scheduler-owned agent diagnostics and replay checks for no-human event streams",
    },
    ApplyMutatorAllowlistEntry {
        path: "crates/tracewake-core/src/content/load.rs",
        rationale: "materializes validated fixture seed events and epistemic projection inside the core-owned content validation authority",
    },
];

const COGNITION_TRUTH_ACCESSOR_TOKENS: &[&str] = &[
    "state.workplaces",
    "state.food_supplies",
    "state.sleep_affordances",
    ".workplaces()",
    ".food_supplies()",
    ".sleep_affordances()",
];

const TRUTH_ACCESSOR_ALLOWLIST: &[TruthAccessorAllowlistEntry] = &[
    TruthAccessorAllowlistEntry {
        path: "crates/tracewake-core/src/actions/defs/eat.rs",
        token: "state.food_supplies",
        rationale: "eat validator reads authoritative food supply truth at commit time",
        responsible_layer: "action_validation",
    },
    TruthAccessorAllowlistEntry {
        path: "crates/tracewake-core/src/actions/defs/sleep.rs",
        token: "state.sleep_affordances",
        rationale: "sleep validator reads authoritative sleep affordance truth at commit time",
        responsible_layer: "action_validation",
    },
    TruthAccessorAllowlistEntry {
        path: "crates/tracewake-core/src/actions/defs/work.rs",
        token: "state.workplaces",
        rationale: "work validator reads authoritative workplace truth at commit time",
        responsible_layer: "action_validation",
    },
    TruthAccessorAllowlistEntry {
        path: "crates/tracewake-core/src/agent/perception.rs",
        token: "state.food_supplies",
        rationale: "perception derives evented observations from visible current-place truth",
        responsible_layer: "projection",
    },
    TruthAccessorAllowlistEntry {
        path: "crates/tracewake-core/src/agent/perception.rs",
        token: ".food_supplies()",
        rationale: "perception derives evented observations from visible current-place truth",
        responsible_layer: "projection",
    },
    TruthAccessorAllowlistEntry {
        path: "crates/tracewake-core/src/agent/perception.rs",
        token: ".sleep_affordances()",
        rationale: "perception derives evented observations from visible current-place truth",
        responsible_layer: "projection",
    },
    TruthAccessorAllowlistEntry {
        path: "crates/tracewake-core/src/projections.rs",
        token: ".food_supplies()",
        rationale: "view projections render current physical truth, not actor cognition inputs",
        responsible_layer: "view_model",
    },
    TruthAccessorAllowlistEntry {
        path: "crates/tracewake-core/src/projections.rs",
        token: ".sleep_affordances()",
        rationale: "view projections render current physical truth, not actor cognition inputs",
        responsible_layer: "view_model",
    },
    TruthAccessorAllowlistEntry {
        path: "crates/tracewake-core/src/projections.rs",
        token: ".workplaces()",
        rationale: "view projections render current physical truth, not actor cognition inputs",
        responsible_layer: "view_model",
    },
    TruthAccessorAllowlistEntry {
        path: "crates/tracewake-core/src/checksum.rs",
        token: "state.food_supplies",
        rationale: "checksum code serializes authoritative state for replay verification",
        responsible_layer: "replay_checksum",
    },
    TruthAccessorAllowlistEntry {
        path: "crates/tracewake-core/src/checksum.rs",
        token: "state.workplaces",
        rationale: "checksum code serializes authoritative state for replay verification",
        responsible_layer: "replay_checksum",
    },
    TruthAccessorAllowlistEntry {
        path: "crates/tracewake-core/src/checksum.rs",
        token: "state.sleep_affordances",
        rationale: "checksum code serializes authoritative state for replay verification",
        responsible_layer: "replay_checksum",
    },
    TruthAccessorAllowlistEntry {
        path: "crates/tracewake-content/src/fixtures/no_human_unseen_workplace_assignment_does_not_plan_work_001.rs",
        token: "state.workplaces",
        rationale: "fixture contract text names the forbidden raw assignment surface being tested",
        responsible_layer: "content_fixture_contract",
    },
];

const LATER_PHASE_REGISTRATION_CALLS: &[&str] = &[
    "register_phase2a_",
    "register_phase3a_",
    "register_phase2a_epistemics",
    "register_phase3a_sleep",
    "register_phase3a_eat",
    "register_phase3a_work",
    "register_phase3a_continue_routine",
];

const NO_HUMAN_SURFACE_FACT_STABLE_IDS: &[&str] = &[
    "active_intention_present",
    "actor_at_workplace",
    "actor_belief_projection_limitation",
    "actor_current_place_visible",
    "actor_knows_food_source",
    "actor_knows_sleep_affordance",
    "actor_knows_sleep_place",
    "actor_knows_workplace",
    "agent_needs_present",
    "assigned_workplace_known",
    "at_workplace",
    "food_source_believed_accessible",
    "known_route_surface",
    "modeled_wait_reason",
    "next_step_available",
    "reevaluation_window_known",
    "sleep_place_believed_accessible",
    "workplace_assignment_active",
    "workplace_believed_accessible",
];

fn production(source: &str) -> String {
    let mut output = String::new();
    let lines = source.lines().collect::<Vec<_>>();
    let mut index = 0;

    while index < lines.len() {
        if lines[index].trim_start().starts_with("#[cfg(test)]") {
            index += 1;
            while index < lines.len() && lines[index].trim().is_empty() {
                index += 1;
            }
            let mut depth = 0_i32;
            let mut saw_brace = false;
            while index < lines.len() {
                let line = lines[index];
                for byte in line.bytes() {
                    match byte {
                        b'{' => {
                            saw_brace = true;
                            depth += 1;
                        }
                        b'}' => depth -= 1,
                        _ => {}
                    }
                }
                index += 1;
                if saw_brace && depth <= 0 {
                    break;
                }
                if !saw_brace && line.trim_end().ends_with(';') {
                    break;
                }
            }
            continue;
        }
        output.push_str(lines[index]);
        output.push('\n');
        index += 1;
    }

    output
}

fn assert_absent(haystack: impl AsRef<str>, needle: &str) {
    assert!(
        !haystack.as_ref().contains(needle),
        "forbidden shortcut reintroduced: {needle}"
    );
}

fn source_without_comments(source: &str) -> String {
    source
        .lines()
        .filter(|line| {
            let trimmed = line.trim_start();
            !(trimmed.starts_with("//") || trimmed.starts_with("///"))
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn normalized_source(source: &str) -> String {
    source.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn fabricated_planning_event_id_violations(sources: &[(String, String)]) -> Vec<String> {
    sources
        .iter()
        .filter(|(_, source)| source.contains("event_visible_local_planning_state"))
        .map(|(path, _)| path.clone())
        .collect()
}

fn hidden_truth_harness_provenance_violations(source: &str) -> Vec<String> {
    let banned_builders = [
        "build_actor_known_planning_state",
        "build_actor_known_planning_state_with_projection_limitation",
        "observe_visible_local",
        "event_visible_local_planning_state",
    ];
    let mut violations = Vec::new();
    for banned in banned_builders {
        if source.contains(banned) {
            violations.push(format!(
                "hidden_truth_gates.rs uses fabricated builder {banned}"
            ));
        }
    }
    if !source.contains("apply_epistemic_event") {
        violations.push(
            "hidden_truth_gates.rs must build gate contexts by applying epistemic events"
                .to_string(),
        );
    }
    for dead_parameter in ["_known_edges", "_known_food_sources"] {
        if source.contains(dead_parameter) {
            violations.push(format!(
                "hidden_truth_gates.rs leaves dead adversarial parameter {dead_parameter}"
            ));
        }
    }
    if !source.contains("planner_hidden_truth_fixture_witness_errors") {
        violations.push(
            "hidden_truth_gates.rs lacks the planner adversarial fixture witness".to_string(),
        );
    }
    violations
}

fn actor_known_context_constructor_violations(sources: &[(String, String)]) -> Vec<String> {
    let allowed_constructor_sites = [
        "crates/tracewake-core/src/agent/actor_known.rs",
        "crates/tracewake-core/src/agent/no_human_surface.rs",
    ];
    let banned_public_builders = [
        "pub fn build_actor_known_planning_state",
        "pub fn build_actor_known_planning_state_with_projection_limitation",
        "pub fn observe_visible_local",
    ];
    let mut violations = Vec::new();

    for (path, source) in sources {
        let uncommented = source_without_comments(&production(source));
        for banned in banned_public_builders {
            if uncommented.contains(banned) {
                violations.push(format!("{path} exposes retired context producer {banned}"));
            }
        }
        if !allowed_constructor_sites.contains(&path.as_str())
            && uncommented.contains("ActorKnownPlanningContext::from_observed_parts(")
        {
            violations.push(format!(
                "{path} constructs ActorKnownPlanningContext outside the builder/classifier surface"
            ));
        }
        if !allowed_constructor_sites.contains(&path.as_str()) {
            for line in uncommented.lines() {
                if line.contains("pub fn ") && line.contains("-> ActorKnownPlanningContext") {
                    violations.push(format!(
                        "{path} exposes public ActorKnownPlanningContext producer: {}",
                        line.trim()
                    ));
                }
            }
        }
    }

    violations
}

fn embodied_surface_fields(view_models_source: &str) -> Vec<EmbodiedSurfaceField> {
    let mut fields = Vec::new();
    for struct_name in embodied_struct_census() {
        let marker = format!("pub struct {struct_name} {{");
        let start = view_models_source
            .find(&marker)
            .unwrap_or_else(|| panic!("embodied surface census entry {struct_name} is a struct"));
        let body_start = start + marker.len();
        let body = view_models_source[body_start..]
            .lines()
            .take_while(|line| line.trim() != "}")
            .collect::<Vec<_>>()
            .join("\n");
        for line in body.lines() {
            let trimmed = line.trim();
            let Some(field) = trimmed.strip_prefix("pub ") else {
                continue;
            };
            let Some((field_name, type_name)) = field.trim_end_matches(',').split_once(": ") else {
                continue;
            };
            if type_name.starts_with("Option<")
                || type_name.starts_with("Vec<")
                || embodied_scalar_field_is_in_census(struct_name, field_name)
                || *struct_name == "NotebookLeadEntry"
            {
                fields.push(EmbodiedSurfaceField {
                    struct_name: struct_name.to_string(),
                    field_name: field_name.to_string(),
                    type_name: type_name.to_string(),
                });
            }
        }
    }
    let action_availability_marker = "pub enum ActionAvailability {";
    assert!(
        view_models_source.contains(action_availability_marker),
        "embodied surface census entry ActionAvailability is an enum"
    );
    fields.push(EmbodiedSurfaceField {
        struct_name: "ActionAvailability".to_string(),
        field_name: "debug_only_diagnostics".to_string(),
        type_name: "Vec<String>".to_string(),
    });
    fields.sort_by(|left, right| {
        (&left.struct_name, &left.field_name).cmp(&(&right.struct_name, &right.field_name))
    });
    fields
}

fn embodied_struct_census() -> &'static [&'static str] {
    &[
        "EmbodiedViewModel",
        "Phase3AEmbodiedStatus",
        "WhyNotView",
        "NotebookView",
        "NotebookBeliefEntry",
        "NotebookObservationEntry",
        "NotebookContradictionEntry",
        "NotebookLeadEntry",
        "VisibleExit",
        "VisibleDoor",
        "VisibleContainer",
        "VisibleItem",
        "SemanticActionEntry",
    ]
}

fn embodied_scalar_field_is_in_census(struct_name: &str, field_name: &str) -> bool {
    matches!(
        (struct_name, field_name),
        ("EmbodiedViewModel", "debug_available")
            | ("VisibleDoor", "endpoint_a")
            | ("VisibleDoor", "endpoint_b")
            | ("VisibleItem", "source")
    )
}

fn embodied_surface_dead_field_errors(
    view_models_source: &str,
    producer_sources: &[(&str, &str)],
    consumer_sources: &[(&str, &str)],
) -> Vec<String> {
    let fields = embodied_surface_fields(view_models_source);
    let mut errors = Vec::new();
    for field in fields {
        let has_producer = embodied_field_has_registered_producer(&field, producer_sources)
            || producer_sources.iter().any(|(_, source)| {
                source_has_non_default_struct_field_producer(
                    source,
                    &field.struct_name,
                    &field.field_name,
                )
            });
        if !has_producer {
            errors.push(format!(
                "{}.{} ({}) has no non-default embodied producer or cited external producer",
                field.struct_name, field.field_name, field.type_name
            ));
        }
        if !consumer_sources
            .iter()
            .any(|(_, source)| source_has_field_consumer(source, &field.field_name))
        {
            errors.push(format!(
                "{}.{} ({}) has no embodied render/app consumer or cited deferral",
                field.struct_name, field.field_name, field.type_name
            ));
        }
    }
    errors
}

fn embodied_field_has_registered_producer(
    field: &EmbodiedSurfaceField,
    producer_sources: &[(&str, &str)],
) -> bool {
    EMBODIED_SURFACE_FIELD_PRODUCERS.iter().any(|entry| {
        entry.struct_name == field.struct_name
            && entry.field_name == field.field_name
            && !entry.cite.is_empty()
            && !entry.rationale.is_empty()
            && (entry.producer_snippet.is_empty()
                || !producer_snippet_is_constant_literal(entry.producer_snippet))
            && producer_sources.iter().any(|(path, source)| {
                *path == entry.source_path
                    && if entry.producer_snippet.is_empty() {
                        source_has_deferral_write_or_consume_site(source, entry.field_name)
                    } else {
                        source.contains(entry.producer_snippet)
                    }
            })
    })
}

fn source_has_deferral_write_or_consume_site(source: &str, field_name: &str) -> bool {
    source.contains(&format!(".{field_name}()"))
        || source.contains(&format!("{field_name}()"))
        || source.contains(&format!("{field_name} = Some("))
        || source.contains(&format!("{field_name} = true"))
}

fn source_has_non_default_struct_field_producer(
    source: &str,
    struct_name: &str,
    field_name: &str,
) -> bool {
    let marker = format!("{struct_name} {{");
    let mut search_start = 0;
    while let Some(relative_index) = source[search_start..].find(&marker) {
        let marker_index = search_start + relative_index;
        let body = body_from_open_brace(&source[marker_index + struct_name.len()..]);
        if struct_body_has_non_default_field(body, field_name) {
            return true;
        }
        search_start = marker_index + marker.len();
    }
    false
}

fn struct_body_has_non_default_field(body: &str, field_name: &str) -> bool {
    let explicit = format!("{field_name}:");
    body.lines().any(|line| {
        let trimmed = line.trim();
        if !trimmed.starts_with(&explicit) {
            return false;
        }
        let value = trimmed[explicit.len()..].trim();
        !(value.starts_with("None")
            || value.starts_with("Vec::new()")
            || value.starts_with("false")
            || value.starts_with("true")
            || value
                .chars()
                .next()
                .is_some_and(|first| first.is_ascii_digit()))
    }) || body
        .lines()
        .any(|line| line.trim() == format!("{field_name},"))
}

fn producer_snippet_is_constant_literal(snippet: &str) -> bool {
    let normalized = normalized_source(snippet);
    normalized.contains("= true")
        || normalized.contains(": true")
        || normalized.contains("= false")
        || normalized.contains(": false")
}

fn source_has_field_consumer(source: &str, field_name: &str) -> bool {
    source.contains(&format!(".{field_name}"))
        || source.contains(&format!("{field_name}()"))
        || source.contains(&format!("{field_name} = Some("))
        || source.contains(&format!("{field_name} = true"))
        || identifier_occurrences(source, field_name) >= 2
}

fn identifier_occurrences(source: &str, needle: &str) -> usize {
    source
        .split(|character: char| !(character == '_' || character.is_ascii_alphanumeric()))
        .filter(|token| *token == needle)
        .count()
}

fn body_from_open_brace(after_name: &str) -> &str {
    let start = after_name
        .find('{')
        .expect("struct construction starts with opening brace");
    let mut depth = 0_i32;
    let mut end = None;
    for (offset, byte) in after_name[start..].bytes().enumerate() {
        match byte {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    end = Some(start + offset + 1);
                    break;
                }
            }
            _ => {}
        }
    }
    &after_name[start..end.expect("struct construction body closes")]
}

fn body_after_marker<'a>(source: &'a str, marker: &str) -> &'a str {
    let after_marker = source
        .split(marker)
        .nth(1)
        .unwrap_or_else(|| panic!("{marker} is present"));
    let start = after_marker
        .find('{')
        .unwrap_or_else(|| panic!("{marker} body starts with an opening brace"));
    let mut depth = 0_i32;
    let mut end = None;
    for (offset, byte) in after_marker[start..].bytes().enumerate() {
        match byte {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    end = Some(start + offset + 1);
                    break;
                }
            }
            _ => {}
        }
    }
    &after_marker[start..end.unwrap_or_else(|| panic!("{marker} body closes"))]
}

fn phase1_loader_later_phase_registration_violations(source: &str) -> Vec<String> {
    let mut violations = Vec::new();
    let load_body = body_after_marker(source, "pub fn load_fixture_package");
    let phase1_body = body_after_marker(source, "FixtureScope::Phase1 =>");
    for token in LATER_PHASE_REGISTRATION_CALLS {
        if load_body.contains(token) {
            violations.push(format!("load_fixture_package directly calls {token}"));
        }
        if phase1_body.contains(token) {
            violations.push(format!("FixtureScope::Phase1 arm calls {token}"));
        }
    }
    violations
}

#[allow(
    clippy::disallowed_methods,
    reason = "anti-regression test scans source files; this helper is not simulation outcome code"
)]
fn production_sources() -> Vec<(String, String)> {
    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("tracewake-core is under workspace crates directory");
    production_sources_from_roots(
        vec![
            workspace_root.join("crates/tracewake-core/src"),
            workspace_root.join("crates/tracewake-content/src"),
            workspace_root.join("crates/tracewake-tui/src"),
        ],
        workspace_root,
    )
}

fn core_production_sources() -> Vec<(String, String)> {
    production_sources()
        .into_iter()
        .filter(|(path, _)| path.starts_with("crates/tracewake-core/src/"))
        .collect()
}

fn apply_mutator_tokens_from(apply_source: &str) -> BTreeSet<String> {
    production(apply_source)
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim_start();
            let after_pub_fn = trimmed.strip_prefix("pub fn ")?;
            let name = after_pub_fn.split_once('(')?.0.trim();
            name.starts_with("apply_").then(|| format!("{name}("))
        })
        .collect()
}

fn apply_mutator_allowed_path(path: &str) -> bool {
    APPLY_MUTATOR_ALLOWLIST
        .iter()
        .any(|entry| entry.path == path && !entry.rationale.trim().is_empty())
}

fn apply_mutator_perimeter_errors(
    apply_source: &str,
    sources: &[(String, String)],
    scanned_tokens: &BTreeSet<String>,
) -> Vec<String> {
    let public_tokens = apply_mutator_tokens_from(apply_source);
    let mut errors = Vec::new();

    for token in public_tokens.difference(scanned_tokens) {
        errors.push(format!(
            "public apply mutator {token} is absent from the mutation perimeter scan"
        ));
    }
    for token in scanned_tokens.difference(&public_tokens) {
        errors.push(format!(
            "mutation perimeter scan lists non-public apply mutator {token}"
        ));
    }
    for entry in APPLY_MUTATOR_ALLOWLIST {
        if entry.rationale.trim().is_empty() {
            errors.push(format!(
                "apply mutator allowlist entry {} lacks rationale",
                entry.path
            ));
        }
    }

    for (path, source) in sources {
        let present_tokens = scanned_tokens
            .iter()
            .filter(|token| source.contains(token.as_str()))
            .cloned()
            .collect::<Vec<_>>();
        if !present_tokens.is_empty() && !apply_mutator_allowed_path(path) {
            errors.push(format!(
                "{} contains direct apply mutator call(s): {}",
                path,
                present_tokens.join(", ")
            ));
        }
    }

    let allowlist_paths = APPLY_MUTATOR_ALLOWLIST
        .iter()
        .map(|entry| entry.path)
        .collect::<BTreeSet<_>>();
    let live_paths = sources
        .iter()
        .map(|(path, _)| path.as_str())
        .filter(|path| allowlist_paths.contains(path))
        .collect::<BTreeSet<_>>();
    for missing_path in allowlist_paths.difference(&live_paths) {
        errors.push(format!(
            "apply mutator allowlist path {} is not a core production source",
            missing_path
        ));
    }

    errors
}

fn apply_mutator_live_witness_count(sources: &[(String, String)]) -> usize {
    let scanned_tokens = apply_mutator_tokens_from(EVENTS_APPLY_RS);
    APPLY_MUTATOR_ALLOWLIST
        .iter()
        .filter(|entry| {
            sources
                .iter()
                .find(|(path, _)| path == entry.path)
                .is_some_and(|(_, source)| {
                    scanned_tokens
                        .iter()
                        .any(|token| source.contains(token.as_str()))
                })
        })
        .count()
}

#[allow(
    clippy::disallowed_methods,
    reason = "anti-regression test scans source files; this helper is not simulation outcome code"
)]
fn production_sources_from_roots(
    roots: Vec<std::path::PathBuf>,
    strip_prefix: &std::path::Path,
) -> Vec<(String, String)> {
    let mut stack = roots;
    let mut sources = Vec::new();
    while let Some(path) = stack.pop() {
        for entry in std::fs::read_dir(path).expect("source directory is readable") {
            let entry = entry.expect("source directory entry is readable");
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                let relative = path
                    .strip_prefix(strip_prefix)
                    .expect("source path is under requested source root")
                    .display()
                    .to_string();
                let source = std::fs::read_to_string(&path).expect("source file is readable");
                sources.push((relative, production(&source)));
            }
        }
    }
    sources.sort_by(|left, right| left.0.cmp(&right.0));
    sources
}

#[allow(
    clippy::disallowed_methods,
    reason = "anti-regression test scans Cargo manifests; this helper is not simulation outcome code"
)]
fn workspace_dependency_entries() -> std::collections::BTreeSet<DependencyEntry> {
    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("tracewake-core is under workspace crates directory");
    [
        "Cargo.toml",
        "crates/tracewake-core/Cargo.toml",
        "crates/tracewake-content/Cargo.toml",
        "crates/tracewake-tui/Cargo.toml",
    ]
    .into_iter()
    .flat_map(|manifest_path| {
        let source = std::fs::read_to_string(workspace_root.join(manifest_path))
            .expect("Cargo manifest is readable");
        dependency_entries_from_manifest(manifest_path, &source)
    })
    .collect()
}

fn dependency_entries_from_manifest(manifest_path: &str, source: &str) -> Vec<DependencyEntry> {
    let mut section = None;
    let mut entries = Vec::new();
    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            let section_name = trimmed.trim_matches(['[', ']']);
            section = matches!(
                section_name,
                "dependencies"
                    | "dev-dependencies"
                    | "build-dependencies"
                    | "workspace.dependencies"
            )
            .then_some(section_name.to_string());
            continue;
        }
        let Some(current_section) = section.as_ref() else {
            continue;
        };
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let dependency = trimmed
            .split_once('=')
            .map_or(trimmed, |(name, _)| name)
            .trim()
            .trim_matches('"');
        if dependency.is_empty() {
            continue;
        }
        entries.push(DependencyEntry {
            manifest_path: manifest_path.to_string(),
            section: current_section.clone(),
            dependency: dependency.to_string(),
        });
    }
    entries
}

const CORE_FOUNDATION_RATIONALE: &str =
    "core foundation/replay/event/state infrastructure outside current ORD-HARD guarded cognition perimeter";
const CORE_ACTION_RATIONALE: &str =
    "core action validation/registry code outside the current scheduled ratchet is covered by targeted action and pipeline behavior gates";
const CORE_ACTION_MUTATION_PERIMETER_RATIONALE: &str =
    "core action code is inside the guarded mutation perimeter and must be covered by scheduled and in-diff cargo-mutants filters";
const CORE_EPISTEMIC_RATIONALE: &str =
    "epistemic data model is covered by capability, provenance, and projection tests";
const CONTENT_RATIONALE: &str =
    "content crate fixture/schema/loader code is not the actor cognition or scheduler lock layer";
const TUI_RATIONALE: &str =
    "tui crate is a boundary/presentation layer, not authoritative simulation outcome logic";

const WORKSPACE_DEPENDENCY_ALLOWLIST: &[(&str, &str, &str)] = &[
    (
        "crates/tracewake-content/Cargo.toml",
        "dependencies",
        "tracewake-core",
    ),
    (
        "crates/tracewake-core/Cargo.toml",
        "dev-dependencies",
        "tracewake-core",
    ),
    (
        "crates/tracewake-tui/Cargo.toml",
        "dependencies",
        "tracewake-content",
    ),
    (
        "crates/tracewake-tui/Cargo.toml",
        "dependencies",
        "tracewake-core",
    ),
    (
        "crates/tracewake-tui/Cargo.toml",
        "dev-dependencies",
        "tracewake-core",
    ),
];

const WORKSPACE_SOURCE_CLASSIFICATIONS: &[WorkspaceSourceClassification] = &[
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/aged_food_record_surfaces_as_remembered_belief_not_observation_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/competing_food_source_facts_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/container_item_move_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/debug_attach_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/debug_omniscience_excluded_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/door_access_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/embodied_continue_hidden_workplace_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/embodied_exits_require_perceived_or_known_route_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/embodied_menu_lags_truth_change_without_perception_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/embodied_view_omits_raw_assignment_without_context_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/embodied_view_omits_unknown_sleep_affordance_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/embodied_view_omits_unobserved_food_at_open_place_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/embodied_workplace_availability_reflects_belief_not_truth_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/embodied_workplace_believed_open_truth_closed_commit_fails_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/expectation_contradiction_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/food_unavailable_replan_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/forbidden_provenance_input_fails_closed_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/hidden_food_closed_container_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/hidden_food_unknown_route_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/hidden_route_edge_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/hidden_truth_audit_rejects_typed_unproven_fact_without_banned_words_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/knowledge_blocker_accuse_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/method_fallback_requires_new_trace_or_stuck_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/mod.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/no_hidden_truth_planning_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/no_human_advance_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/no_human_current_place_without_sleep_affordance_does_not_sleep_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/no_human_day_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/no_human_epistemic_check_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/no_human_known_workplace_requires_provenance_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/no_human_metrics_require_typed_responsible_layer_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/no_human_observation_facts_cite_log_events_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/no_human_sleep_knowledge_requires_observation_or_record_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/no_human_unseen_workplace_assignment_does_not_plan_work_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/no_human_workplace_knowledge_requires_notice_event_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/ordinary_workday_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/partial_food_source_knowledge_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/place_carried_item_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/planner_trace_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/possession_does_not_reset_intention_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/possession_parity_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/prose_born_fact_rejected_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/replay_item_location_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/routine_blocked_diagnostic_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/routine_no_teleport_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/scheduler_cannot_rewrite_wait_reason_after_transaction_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/seeded_food_source_unknown_to_all_actors_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/severe_safety_with_known_exit_produces_move_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/severe_safety_without_known_exit_waits_with_knowledge_blocker_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/sleep_eat_work_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/sleep_interrupted_by_severe_need_prorates_recovery_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/sleep_rejects_current_place_without_sleep_affordance_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/sleep_spanning_window_boundary_charges_each_tick_once_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/sound_uncertainty_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/stale_workplace_notice_superseded_by_newer_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/strongbox_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/view_filtering_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/view_model_local_actions_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/wait_then_window_passive_charges_each_tick_once_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/work_block_failed_then_sleep_succeeds_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/work_completion_fails_when_actor_displaced_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/workplace_assignment_provenance_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/lib.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/load.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/manifest.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/schema.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/serialization.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/validate.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/actions/defs/accuseprobe.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_ACTION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/actions/defs/checkcontainer.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_ACTION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/actions/defs/continue_routine.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_ACTION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/actions/defs/eat.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_ACTION_MUTATION_PERIMETER_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/actions/defs/inspect.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_ACTION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/actions/defs/mod.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_ACTION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/actions/defs/movement.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_ACTION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/actions/defs/need_events.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_ACTION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/actions/defs/openclose.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_ACTION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/actions/defs/sleep.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_ACTION_MUTATION_PERIMETER_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/actions/defs/takeplace.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_ACTION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/actions/defs/wait.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_ACTION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/actions/defs/work.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_ACTION_MUTATION_PERIMETER_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/actions/mod.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_ACTION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/actions/pipeline.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_ACTION_MUTATION_PERIMETER_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/actions/proposal.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_ACTION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/actions/registry.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_ACTION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/actions/report.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_ACTION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/agent/actor_known.rs", class: WorkspaceSourceClass::GuardedLayer },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/agent/candidate.rs", class: WorkspaceSourceClass::GuardedLayer },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/agent/decision.rs", class: WorkspaceSourceClass::GuardedLayer },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/agent/generation.rs", class: WorkspaceSourceClass::GuardedLayer },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/agent/htn.rs", class: WorkspaceSourceClass::GuardedLayer },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/agent/intention.rs", class: WorkspaceSourceClass::GuardedLayer },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/agent/methods.rs", class: WorkspaceSourceClass::GuardedLayer },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/agent/mod.rs", class: WorkspaceSourceClass::GuardedLayer },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/agent/need.rs", class: WorkspaceSourceClass::GuardedLayer },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/agent/no_human_surface.rs", class: WorkspaceSourceClass::GuardedLayer },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/agent/perception.rs", class: WorkspaceSourceClass::GuardedLayer },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/agent/planner.rs", class: WorkspaceSourceClass::GuardedLayer },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/agent/routine.rs", class: WorkspaceSourceClass::GuardedLayer },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/agent/routine_continuation.rs", class: WorkspaceSourceClass::GuardedLayer },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/agent/trace.rs", class: WorkspaceSourceClass::GuardedLayer },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/agent/transaction.rs", class: WorkspaceSourceClass::GuardedLayer },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/checksum.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_FOUNDATION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/content/load.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/content/manifest.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/content/mod.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/content/schema.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/content/serialization.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/content/validate.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/controller.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_FOUNDATION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/debug_capability.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_FOUNDATION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/debug_reports.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_FOUNDATION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/epistemics/belief.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_EPISTEMIC_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/epistemics/contradiction.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_EPISTEMIC_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/epistemics/knowledge_basis.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_EPISTEMIC_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/epistemics/knowledge_context.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_EPISTEMIC_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/epistemics/mod.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_EPISTEMIC_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/epistemics/observation.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_EPISTEMIC_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/epistemics/projection.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_EPISTEMIC_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/epistemics/proposition.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_EPISTEMIC_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/events/apply.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_FOUNDATION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/events/envelope.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_FOUNDATION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/events/log.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_FOUNDATION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/events/mod.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_FOUNDATION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/events/mutation.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_FOUNDATION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/ids.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_FOUNDATION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/lib.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_FOUNDATION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/location.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_FOUNDATION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/need_accounting.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_FOUNDATION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/projections.rs", class: WorkspaceSourceClass::GuardedLayer },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/replay/mod.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_FOUNDATION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/replay/rebuild.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_FOUNDATION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/replay/report.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_FOUNDATION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/replay/temporal.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_FOUNDATION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/runtime/command.rs", class: WorkspaceSourceClass::GuardedLayer },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/runtime/mod.rs", class: WorkspaceSourceClass::GuardedLayer },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/runtime/receipt.rs", class: WorkspaceSourceClass::GuardedLayer },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/runtime/session.rs", class: WorkspaceSourceClass::GuardedLayer },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/scheduler.rs", class: WorkspaceSourceClass::GuardedLayer },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/state.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_FOUNDATION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/time.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_FOUNDATION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/view_models.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_FOUNDATION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-tui/src/app.rs", class: WorkspaceSourceClass::Exempt { rationale: TUI_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-tui/src/debug_panels.rs", class: WorkspaceSourceClass::Exempt { rationale: TUI_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-tui/src/input.rs", class: WorkspaceSourceClass::Exempt { rationale: TUI_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-tui/src/launch.rs", class: WorkspaceSourceClass::Exempt { rationale: TUI_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-tui/src/lib.rs", class: WorkspaceSourceClass::Exempt { rationale: TUI_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-tui/src/main.rs", class: WorkspaceSourceClass::Exempt { rationale: TUI_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-tui/src/render.rs", class: WorkspaceSourceClass::Exempt { rationale: TUI_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-tui/src/run.rs", class: WorkspaceSourceClass::Exempt { rationale: TUI_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-tui/src/screen/mod.rs", class: WorkspaceSourceClass::Exempt { rationale: TUI_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-tui/src/screen/model.rs", class: WorkspaceSourceClass::Exempt { rationale: TUI_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-tui/src/screen/struct_dump.rs", class: WorkspaceSourceClass::Exempt { rationale: TUI_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-tui/src/screen/text_dump.rs", class: WorkspaceSourceClass::Exempt { rationale: TUI_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-tui/src/transcript.rs", class: WorkspaceSourceClass::Exempt { rationale: TUI_RATIONALE } },
];

#[derive(Clone, Copy)]
enum GuardedLayer {
    Agent,
    Scheduler,
    Projections,
}

fn guarded_layer_sources() -> Vec<(String, String)> {
    production_sources()
        .into_iter()
        .filter(|(path, _)| is_guarded_layer_source(path))
        .collect()
}

fn guarded_sources_for(layer: GuardedLayer) -> Vec<(String, String)> {
    guarded_layer_sources()
        .into_iter()
        .filter(|(path, _)| match layer {
            GuardedLayer::Agent => path.starts_with("crates/tracewake-core/src/agent/"),
            GuardedLayer::Scheduler => path.starts_with("crates/tracewake-core/src/scheduler"),
            GuardedLayer::Projections => path.starts_with("crates/tracewake-core/src/projections"),
        })
        .collect()
}

fn guarded_source(path: &str) -> String {
    let legacy_core_path = format!("crates/tracewake-core/{path}");
    guarded_layer_sources()
        .into_iter()
        .find_map(|(source_path, source)| {
            (source_path == path || source_path == legacy_core_path).then_some(source)
        })
        .unwrap_or_else(|| panic!("{path} is part of the guarded layer census"))
}

fn guarded_layer_source_paths() -> Vec<String> {
    production_sources()
        .into_iter()
        .map(|(path, _)| path)
        .filter(|path| is_guarded_layer_source(path))
        .collect()
}

fn is_guarded_layer_source(path: &str) -> bool {
    WORKSPACE_SOURCE_CLASSIFICATIONS.iter().any(|entry| {
        entry.path == path && matches!(entry.class, WorkspaceSourceClass::GuardedLayer)
    })
}

const MUTATION_PERIMETER_EXAMINE_GLOBS: &[&str] = &[
    "crates/tracewake-core/src/agent/**",
    "crates/tracewake-core/src/runtime/**",
    "crates/tracewake-core/src/scheduler.rs",
    "crates/tracewake-core/src/projections.rs",
    "crates/tracewake-core/src/actions/pipeline.rs",
    "crates/tracewake-core/src/actions/defs/eat.rs",
    "crates/tracewake-core/src/actions/defs/sleep.rs",
    "crates/tracewake-core/src/actions/defs/work.rs",
    "crates/tracewake-core/src/events/**",
    "crates/tracewake-core/src/replay/**",
    "crates/tracewake-core/src/checksum.rs",
    "crates/tracewake-core/src/state.rs",
    "crates/tracewake-core/src/actions/proposal.rs",
    "crates/tracewake-core/src/actions/report.rs",
    "crates/tracewake-core/src/view_models.rs",
    "crates/tracewake-core/src/debug_capability.rs",
    "crates/tracewake-core/src/controller.rs",
    "crates/tracewake-core/src/debug_reports.rs",
    "crates/tracewake-core/src/epistemics/knowledge_context.rs",
    "crates/tracewake-core/src/epistemics/projection.rs",
    "crates/tracewake-core/src/content/manifest.rs",
    "crates/tracewake-core/src/content/load.rs",
    "crates/tracewake-core/src/content/schema.rs",
    "crates/tracewake-core/src/content/serialization.rs",
    "crates/tracewake-core/src/content/validate.rs",
    "crates/tracewake-tui/src/app.rs",
    "crates/tracewake-tui/src/debug_panels.rs",
    "crates/tracewake-tui/src/render.rs",
    "crates/tracewake-tui/src/transcript.rs",
];

const MUTATION_PERIMETER_CANARY_PATHS: &[&str] = &[
    "crates/tracewake-core/src/agent/transaction.rs",
    "crates/tracewake-core/src/runtime/session.rs",
    "crates/tracewake-core/src/scheduler.rs",
    "crates/tracewake-core/src/projections.rs",
    "crates/tracewake-core/src/actions/pipeline.rs",
    "crates/tracewake-core/src/actions/defs/eat.rs",
    "crates/tracewake-core/src/actions/defs/sleep.rs",
    "crates/tracewake-core/src/actions/defs/work.rs",
    "crates/tracewake-core/src/events/mod.rs",
    "crates/tracewake-core/src/events/apply.rs",
    "crates/tracewake-core/src/events/envelope.rs",
    "crates/tracewake-core/src/replay/mod.rs",
    "crates/tracewake-core/src/replay/rebuild.rs",
    "crates/tracewake-core/src/replay/report.rs",
    "crates/tracewake-core/src/checksum.rs",
    "crates/tracewake-core/src/state.rs",
    "crates/tracewake-core/src/actions/proposal.rs",
    "crates/tracewake-core/src/actions/report.rs",
    "crates/tracewake-core/src/view_models.rs",
    "crates/tracewake-core/src/debug_capability.rs",
    "crates/tracewake-core/src/controller.rs",
    "crates/tracewake-core/src/debug_reports.rs",
    "crates/tracewake-core/src/epistemics/knowledge_context.rs",
    "crates/tracewake-core/src/epistemics/projection.rs",
    "crates/tracewake-core/src/content/manifest.rs",
    "crates/tracewake-core/src/content/load.rs",
    "crates/tracewake-core/src/content/schema.rs",
    "crates/tracewake-core/src/content/serialization.rs",
    "crates/tracewake-core/src/content/validate.rs",
    "crates/tracewake-tui/src/app.rs",
    "crates/tracewake-tui/src/debug_panels.rs",
    "crates/tracewake-tui/src/render.rs",
    "crates/tracewake-tui/src/transcript.rs",
];

const MUTANTS_BASELINE_NORMALIZED_COUNT: usize = 0;
const MUTANTS_BASELINE_NORMALIZED_FNV1A64: u64 = 0xcbf2_9ce4_8422_2325;
const MUTANTS_BASELINE_GENESIS_COUNT: usize = 143;
const MUTANTS_BASELINE_GENESIS_FNV1A64: u64 = 0xbd18_55a5_ee82_b428;
const MUTATION_LEDGER_MAX_IDENTICAL_RATIONALES: usize = 20;
const RECORDED_GENERATIVE_MASK_DIVERSITY: usize = 7;
const RECORDED_GENERATIVE_SEQUENCE_LENGTH_DIVERSITY: usize = 4;
const RECORDED_GENERATIVE_MULTI_SEED_CONTRIBUTORS: &[(&str, usize)] = &[
    ("actor_waited", 10),
    ("need_delta", 12),
    ("food_consumed", 6),
    ("sleep_started", 6),
    ("work_started", 7),
    ("sleep_completed", 4),
    ("sleep_interrupted", 2),
    ("work_completed", 4),
    ("work_failed", 3),
];
const META_LOCK_REGISTRY_MIN_ENTRIES: usize = 45;

const META_LOCK_REGISTRY: &[MetaLockRegistryEntry] = &[
    MetaLockRegistryEntry {
        lock_id: "meta_lock_registry_census",
        negative_id: "synthetic_meta_lock_without_negative",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "mutation_baseline_misses_are_pinned_and_ledgered",
        negative_id: "synthetic_unrecorded_mutation_baseline_shrink",
        routing: MetaLockRouting::SharedScan,
        witness_min: 0,
    },
    MetaLockRegistryEntry {
        lock_id: "mutation_perimeter_logical_line_swallow_scan",
        negative_id: "synthetic_multiline_mutants_swallow_suffix",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "mutation_baseline_non_empty_entries_require_ledger_dispositions",
        negative_id: "synthetic_unledgered_non_empty_mutation_baseline",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "ci_workflow_guards_cover_workflow_integrity",
        negative_id: "synthetic_ci_workflow_guard_removed",
        routing: MetaLockRouting::SharedScan,
        witness_min: 6,
    },
    MetaLockRegistryEntry {
        lock_id: "generative_lock_two_sided_floor_ratchets",
        negative_id: "synthetic_unrecorded_generative_floor_raise",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "hidden_truth_context_discrimination_witness",
        negative_id: "synthetic_context_hidden_food_injection",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "log_derived_panic_guard_scans_unwrap",
        negative_id: "synthetic_log_derived_unwrap_payload",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "generative_support_constructs_zero_event_envelopes",
        negative_id: "synthetic_support_event_envelope_construction",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "acceptance_artifact_0023_maps_spec_section_7_items_to_report_anchors",
        negative_id: "synthetic_0023_missing_acceptance_anchor",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "acceptance_artifact_0024_maps_spec_section_7_items_to_report_anchors",
        negative_id: "synthetic_0024_missing_acceptance_anchor",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "acceptance_artifact_0025_maps_spec_section_7_items_to_report_anchors",
        negative_id: "synthetic_0025_missing_acceptance_anchor",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "generative_support_bans_bare_event_envelope_token",
        negative_id: "synthetic_support_event_envelope_default",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "physical_mutating_event_kinds_have_explicit_world_apply_arms",
        negative_id: "synthetic_missing_arm_catch_all",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "agent_stream_event_kinds_have_explicit_agent_apply_arms",
        negative_id: "synthetic_missing_agent_stream_apply_arm",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "action_emitted_event_kinds_have_cause_disposition",
        negative_id: "synthetic_action_emitted_kind_without_cause_required",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "event_kind_cause_required_exhaustive_match",
        negative_id: "synthetic_cause_required_match_without_default",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "scheduler_apply_and_completion_paths_do_not_panic_on_log_derived_data",
        negative_id: "synthetic_log_derived_expect",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "embodied_projection_never_uses_unfiltered_checked_facts_as_actor_provenance",
        negative_id: "synthetic_unfiltered_checked_facts_iter",
        routing: MetaLockRouting::TicketOwnedDebt {
            ticket: "0022PHA3ABASTRI-009",
        },
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "embodied_view_option_and_collection_fields_have_reachable_producers",
        negative_id: "synthetic_orphaned_deferral_embodied_surface_producer",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "scheduler_never_direct_dispatches_primitive_action",
        negative_id: "synthetic_direct_scheduler_dispatch",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "no_direct_apply_event_outside_event_replay_or_pipeline",
        negative_id: "synthetic_direct_apply_event_call",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 4,
    },
    MetaLockRegistryEntry {
        lock_id: "event_apply_remains_only_post_seed_mutation_path",
        negative_id: "synthetic_post_seed_mutation_path_bypass",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_006_scheduler_has_no_direct_routine_or_need_proposal_bypass",
        negative_id: "synthetic_direct_scheduler_proposal_bypass",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_006_duration_need_deltas_route_through_shared_emitter",
        negative_id: "synthetic_direct_duration_need_delta_construction",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_006_scheduler_does_not_fabricate_empty_epistemic_projection",
        negative_id: "synthetic_empty_epistemic_projection",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_018_actor_known_facts_require_source_event_witness",
        negative_id: "synthetic_actor_known_fact_without_witness",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_018_witness_kind_no_human_fact_stable_ids_have_explicit_arms",
        negative_id: "synthetic_missing_witness_kind_stable_id_arm",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_014_no_human_cognition_surface_does_not_read_raw_assignment_or_sleep_truth",
        negative_id: "synthetic_raw_assignment_or_sleep_truth_read",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_015_ord_hard_008_cognition_channel_stays_evented_and_sealed",
        negative_id: "synthetic_unsealed_cognition_channel",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_014_embodied_projection_workplaces_are_context_backed",
        negative_id: "synthetic_contextless_workplace_projection",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_014_embodied_projection_source_has_no_physical_state_field",
        negative_id: "synthetic_embodied_projection_source_physical_state_field",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_014_phase3a_semantic_actions_do_not_use_literal_true_availability",
        negative_id: "synthetic_literal_true_action_availability",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_014_no_human_metrics_do_not_scan_display_text",
        negative_id: "synthetic_display_text_metric_scan",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_014_perception_visibility_uses_typed_place_visibility",
        negative_id: "synthetic_prose_visibility_branch",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_014_perception_visibility_detects_display_label_binding_laundering",
        negative_id: "synthetic_display_label_binding_laundering",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_014_perception_visibility_detects_bare_display_label_substrings",
        negative_id: "synthetic_bare_display_label_starts_with",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_014_perception_visibility_other_emission_paths",
        negative_id: "synthetic_prose_branch_in_other_emission_path",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "typed_column_closure_oblique_payload_helper_calls",
        negative_id: "synthetic_oblique_payload_helper_call",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "typed_column_closure_payload_receiver_helper_calls",
        negative_id: "synthetic_payload_receiver_helper_call",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "typed_column_closure_payload_alias_helper_calls",
        negative_id: "synthetic_payload_alias_helper_call",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "typed_column_closure_exemptions_are_rationale_bearing_and_live",
        negative_id: "synthetic_payload_fields_outside_gated_materialized_site",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_011_no_human_day_runner_only_evidence",
        negative_id: "synthetic_runner_only_work_completion_ancestry_removed",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_014_sleep_validation_requires_modeled_affordance",
        negative_id: "synthetic_sleep_validation_without_affordance",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_0021_actor_known_projection_policy_table_has_production_callers",
        negative_id: "synthetic_projection_policy_without_caller",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_015_ordinary_life_tuning_comes_from_authored_state",
        negative_id: "synthetic_ordinary_life_tuning_literal",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_006_scheduler_has_no_routine_family_to_primitive_dispatch",
        negative_id: "synthetic_routine_family_to_primitive_dispatch",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_014_scheduler_cannot_rewrite_transaction_proposals_after_cognition",
        negative_id: "synthetic_scheduler_rewrites_transaction_proposal",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_014_transaction_has_no_silent_method_fallback_scan",
        negative_id: "synthetic_silent_method_fallback_scan",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_015_hidden_truth_audit_fails_closed_in_transaction",
        negative_id: "synthetic_hidden_truth_audit_open_transaction",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_014_decision_hidden_truth_audit_uses_typed_input_refs",
        negative_id: "synthetic_hidden_truth_audit_string_tag",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id:
            "guard_003_work_eat_sleep_validators_do_not_read_need_values_from_proposal_parameters",
        negative_id: "synthetic_need_value_parameter_read",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "agent_world_noop_allowlist_is_explicit_and_excludes_materialized_episode_state",
        negative_id: "synthetic_agent_world_noop_allows_materialized_event",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "materialized_agent_payload_records_keep_payload_fields",
        negative_id: "synthetic_materialized_agent_record_without_payload_fields",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "materialized_agent_apply_arms_require_payload_schema_version",
        negative_id: "synthetic_materialized_agent_apply_arm_without_payload_schema_version",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_002_agent_state_keeps_typed_trace_and_diagnostic_records",
        negative_id: "synthetic_untyped_agent_trace_record",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_001_authoritative_state_fields_are_not_publicly_mutable",
        negative_id: "synthetic_public_authoritative_state_field",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_001_mutation_capability_is_private_to_event_application",
        negative_id: "synthetic_public_mutation_capability",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_001_no_production_seed_mutation_outside_state_definition",
        negative_id: "synthetic_seed_mutation_outside_state",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_001_no_direct_state_collection_insert_outside_event_application",
        negative_id: "synthetic_direct_state_collection_insert",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_001_actor_known_context_has_no_public_arbitrary_constructor",
        negative_id: "synthetic_public_actor_known_context_constructor",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "adding_event_schema_version_requires_migrator_registration",
        negative_id: "synthetic_event_schema_without_migrator",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "event_envelope_duplicate_fields_reject_loudly",
        negative_id: "synthetic_duplicate_content_manifest_id_field",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "event_envelope_direct_decode_rejects_unsupported_schema_version",
        negative_id: "synthetic_direct_decode_event_schema_v999",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "event_envelope_has_no_hollow_checksum_after_field",
        negative_id: "synthetic_envelope_checksum_after_field",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "non_world_stream_cannot_change_physical_checksum",
        negative_id: "synthetic_non_world_physical_checksum_change",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "checksum_coverage_is_total_for_authoritative_state",
        negative_id: "synthetic_checksum_coverage_gap",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "new_authoritative_field_without_checksum_registry_fails",
        negative_id: "synthetic_authoritative_field_without_registry",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "new_authoritative_field_without_canonical_checksum_line_fails",
        negative_id: "synthetic_authoritative_field_without_checksum_line",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_0021_actor_known_context_producers_are_projection_backed",
        negative_id: "synthetic_unbacked_actor_known_context_producer",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_0021_hidden_truth_gates_use_event_log_provenance",
        negative_id: "synthetic_hidden_truth_gate_without_event_log",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "hidden_food_closed_container_is_not_actor_known_food_source",
        negative_id: "synthetic_empty_hidden_food_adversarial_fixture",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "hidden_route_edge_absent_from_actor_context_blocks_route_plan",
        negative_id: "synthetic_empty_hidden_route_adversarial_fixture",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "planner_hidden_truth_fixture_witness_fails_on_empty_adversarial_context",
        negative_id: "synthetic_empty_planner_hidden_truth_fixture",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "actor_known_projection_policy_table_drives_record_behavior",
        negative_id: "synthetic_policy_table_behavior_drift",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "workplace_current_place_scope_drops_other_place_from_embodied_context",
        negative_id: "synthetic_workplace_embodied_scope_removed",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "supersede_newest_by_subject_requires_subject_extractor",
        negative_id: "synthetic_non_workplace_supersede_subject",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_0021_fabricated_visible_local_event_id_is_retired",
        negative_id: "synthetic_fabricated_visible_local_event_id",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_001_hidden_truth_audit_is_derived_from_provenance_not_tags",
        negative_id: "synthetic_hidden_truth_audit_tag_match",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_006_continue_routine_marker_alone_is_not_behavioral_progress",
        negative_id: "synthetic_continue_marker_as_progress",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_0057_embodied_continue_non_proposed_outcome_is_typed_stuck",
        negative_id: "synthetic_embodied_continue_stuck_falls_back_to_marker",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_0057_continue_routine_progress_of_record_is_follow_on",
        negative_id: "synthetic_continue_marker_counts_as_progress",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 2,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_0058_embodied_routine_family_has_no_pre_intention_workplace_selector",
        negative_id: "synthetic_workplace_before_active_intention",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_0058_embodied_continue_time_advancing_follow_on_is_gated",
        negative_id: "synthetic_direct_wait_follow_on",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_0058_embodied_continue_success_path_not_current_stuck",
        negative_id: "synthetic_success_prefixed_current_stuck",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_0058_tui_continue_routine_forwards_only",
        negative_id: "synthetic_tui_routine_selection",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_0059_scheduler_routine_family_authority_cannot_bypass_active_intention",
        negative_id: "synthetic_0059_window_keyed_routine_family",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id:
            "guard_0059_no_clock_keyed_routine_family_selector_without_active_intention_binding",
        negative_id: "synthetic_0059_eligible_execution_min_by_start",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_0059_synthetic_negative_census_is_live",
        negative_id: "synthetic_0059_routine_window_family_without_active_intention",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 5,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_007_mutation_efficacy_notes_cover_high_risk_shortcuts",
        negative_id: "synthetic_missing_mutation_efficacy_note",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_008_action_registry_uses_typed_scopes_not_phase1_boolean",
        negative_id: "synthetic_action_registry_phase1_boolean",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_008_phase1_loader_does_not_register_later_phase_actions",
        negative_id: "synthetic_phase1_loader_registers_later_phase",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_008_phase1_loader_source_guard_has_mutation_self_coverage",
        negative_id: "synthetic_phase1_loader_guard_mutation",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "workspace_dependency_posture_matches_allowlist",
        negative_id: "synthetic_unlisted_workspace_dependency",
        routing: MetaLockRouting::SharedScan,
        witness_min: 1,
    },
];

const META_LOCK_CENSUS_EXEMPTIONS: &[MetaLockCensusExemption] = &[
    MetaLockCensusExemption {
        test_name: "cognition_inputs_are_context_backed",
        rationale: "Product-surface regression test with direct behavioral assertions; not a structural lock family.",
    },
    MetaLockCensusExemption {
        test_name: "guarded_layer_source_census_matches_module_tree",
        rationale: "Workspace inventory census that feeds other source guards; not a structural lock with a synthetic negative.",
    },
    MetaLockCensusExemption {
        test_name: "mutation_perimeter_matches_duration_action_rationale_and_ci_filters",
        rationale: "Mutation perimeter governance test covered separately by lock_id mutation_perimeter_logical_line_swallow_scan and sibling mutation-CI meta-lock entries.",
    },
    MetaLockCensusExemption {
        test_name: "need_accounting_duration_body_start_is_exclusive_and_terminal_is_inclusive",
        rationale: "0043ORDLIFCER-002 ticket-owned mutation witness for the need_accounting.rs duration-boundary survivor.",
    },
    MetaLockCensusExemption {
        test_name: "need_accounting_current_start_requires_subject_actor_and_absent_log_identity",
        rationale: "0043ORDLIFCER-002 ticket-owned mutation witness for subject actor and duplicate-current-start ownership survivors.",
    },
    MetaLockCensusExemption {
        test_name: "need_accounting_current_start_identity_ignores_unrelated_log_events",
        rationale: "0043ORDLIFCER-002 ticket-owned mutation witness for event-id membership over unrelated log events.",
    },
    MetaLockCensusExemption {
        test_name: "movement_door_endpoint_mismatch_rejects_partial_connections",
        rationale: "0043ORDLIFCER-006 ticket-owned mutation witness for exact unordered movement-door endpoint matching.",
    },
    MetaLockCensusExemption {
        test_name: "user_origin_wait_keeps_candidate_goal_reevaluation_false",
        rationale: "0043ORDLIFCER-007 ticket-owned mutation witness for user-origin wait autonomy classification; scan-shaped payload checks are covered by lock_id guard_006_scheduler_has_no_direct_routine_or_need_proposal_bypass.",
    },
    MetaLockCensusExemption {
        test_name: "mutation_baseline_misses_are_pinned_and_ledgered",
        rationale: "Empty-baseline governance entry is enrolled under a zero-minimum registry exemption.",
    },
    MetaLockCensusExemption {
        test_name: "meta_lock_registry_covers_structural_locks_and_negatives",
        rationale: "Self-reflexive meta-lock census test is represented by lock_id meta_lock_registry_census.",
    },
    MetaLockCensusExemption {
        test_name: "generative_lock_source_uses_two_sided_recorded_floors",
        rationale: "Generative ratchet test is represented by lock_id generative_lock_two_sided_floor_ratchets.",
    },
    MetaLockCensusExemption {
        test_name: "acceptance_artifact_0021_maps_spec_section_7_items_to_report_anchors",
        rationale: "Historical acceptance-artifact anchor audit; not a standing structural lock family.",
    },
    MetaLockCensusExemption {
        test_name: "acceptance_artifact_0022_maps_spec_section_7_items_to_report_anchors",
        rationale: "Historical acceptance-artifact anchor audit; not a standing structural lock family.",
    },
    MetaLockCensusExemption {
        test_name: "workspace_source_classification_census_matches_production_tree",
        rationale: "Workspace inventory parity test, not a structural anti-regression lock with its own negative.",
    },
    MetaLockCensusExemption {
        test_name: "guarded_layer_entries_are_exactly_the_workspace_guarded_classifications",
        rationale: "Workspace classification parity test feeding mutation perimeter checks.",
    },
    MetaLockCensusExemption {
        test_name: "nondeterminism_api_gate",
        rationale: "Broad source hygiene gate with its own allowlist proof, not a spec-0022 structural lock family.",
    },
    MetaLockCensusExemption {
        test_name: "source_guard_discovers_new_nested_production_file",
        rationale: "Source-census helper self-test, not a standing structural lock.",
    },
    MetaLockCensusExemption {
        test_name: "source_guard_does_not_silently_skip_production_after_cfg_test",
        rationale: "Source-census helper self-test, not a standing structural lock.",
    },
    MetaLockCensusExemption {
        test_name: "forged_or_stale_source_context_rejected_by_reason_code",
        rationale: "Behavioral validation regression, not a structural lock family.",
    },
    MetaLockCensusExemption {
        test_name: "diagnostics_never_assert_display_label_as_authority",
        rationale: "Product diagnostic regression test, not a structural lock family.",
    },
    MetaLockCensusExemption {
        test_name: "validation_report_keeps_typed_provenance_and_actor_debug_split",
        rationale: "Validation report behavior test, not a structural source-scan lock.",
    },
    MetaLockCensusExemption {
        test_name: "privileged_tui_proposal_requires_current_view_source_context",
        rationale: "Behavioral possession/TUI regression, not a structural lock family.",
    },
    MetaLockCensusExemption {
        test_name: "accepted_action_appends_before_authoritative_apply",
        rationale: "Pipeline ordering behavior test, not a structural source-scan lock.",
    },
    MetaLockCensusExemption {
        test_name: "event_kind_metadata_is_total",
        rationale: "Typed registry parity test; downstream structural locks cover apply/cause/checksum consequences.",
    },
    MetaLockCensusExemption {
        test_name: "replay_physical_state_diff_covers_checksum_families",
        rationale: "Replay report parity companion to checksum coverage, not a separate structural lock family.",
    },
    MetaLockCensusExemption {
        test_name: "generative_lock_cannot_fabricate_duration_terminals",
        rationale: "Generative harness behavior proof represented by lock_id generative_support_bans_bare_event_envelope_token and sibling support/generative guard entries.",
    },
    MetaLockCensusExemption {
        test_name: "standing_barrier_negative_fixture_runner_keeps_all_feature_boundary_lane",
        rationale: "Secondary topology alarm for the load-bearing negative_fixture_runner all-feature compile-fail lane, not a separate production source-scan lock.",
    },
];

fn yaml_step_run_block<'a>(ci_yml: &'a str, step_name: &str) -> Option<&'a str> {
    let marker = format!("- name: {step_name}");
    let start = ci_yml.find(&marker)?;
    let after = &ci_yml[start + marker.len()..];
    let run_start = after.find("run: |")? + "run: |".len();
    let body = &after[run_start..];
    let end = body.find("\n      - name:").unwrap_or(body.len());
    Some(&body[..end])
}

fn non_comment_lines(source: &str) -> Vec<&str> {
    source
        .lines()
        .filter(|line| {
            let trimmed = line.trim_start();
            !trimmed.is_empty() && !trimmed.starts_with('#')
        })
        .collect()
}

fn logical_shell_lines(source: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    for line in non_comment_lines(source) {
        let trimmed_end = line.trim_end();
        let continued = trimmed_end.ends_with('\\');
        let segment = trimmed_end.trim_end_matches('\\').trim_end();
        if current.is_empty() {
            current.push_str(segment.trim_start());
        } else {
            current.push(' ');
            current.push_str(segment.trim_start());
        }
        if !continued {
            lines.push(current);
            current = String::new();
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}

fn simple_glob_matches(pattern: &str, path: &str) -> bool {
    if pattern == path {
        return true;
    }
    if let Some(prefix) = pattern.strip_suffix("/**") {
        return path.starts_with(&format!("{prefix}/"));
    }
    if let Some(prefix) = pattern.strip_suffix('*') {
        return path.starts_with(prefix);
    }
    false
}

fn recognized_mutants_glob(pattern: &str) -> bool {
    !pattern.contains('*')
        || pattern.ends_with("/**")
        || (pattern.ends_with('*') && !pattern[..pattern.len() - 1].contains('*'))
}

fn parse_mutants_toml_top_level_keys(mutants_toml: &str) -> Vec<String> {
    mutants_toml
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with('[') {
                return None;
            }
            trimmed
                .split_once('=')
                .map(|(key, _)| key.trim().to_string())
        })
        .collect()
}

fn parse_mutants_glob_array(mutants_toml: &str, key: &str) -> Vec<String> {
    let marker = format!("{key} = [");
    let Some(after_marker) = mutants_toml.split(&marker).nth(1) else {
        return Vec::new();
    };
    let Some(block) = after_marker.split(']').next() else {
        return Vec::new();
    };
    block
        .lines()
        .filter_map(|line| {
            line.trim()
                .trim_end_matches(',')
                .trim_matches('"')
                .split_once('#')
                .map_or_else(
                    || {
                        Some(
                            line.trim()
                                .trim_end_matches(',')
                                .trim_matches('"')
                                .to_string(),
                        )
                    },
                    |(value, _)| {
                        Some(
                            value
                                .trim()
                                .trim_end_matches(',')
                                .trim_matches('"')
                                .to_string(),
                        )
                    },
                )
        })
        .filter(|value| !value.is_empty())
        .collect()
}

fn parse_examine_globs(mutants_toml: &str) -> Vec<String> {
    parse_mutants_glob_array(mutants_toml, "examine_globs")
}

fn parse_exclude_globs(mutants_toml: &str) -> Vec<String> {
    parse_mutants_glob_array(mutants_toml, "exclude_globs")
}

fn yaml_concurrency_block(ci_yml: &str) -> Option<&str> {
    let start = ci_yml.find("\nconcurrency:\n")? + "\nconcurrency:\n".len();
    let body = &ci_yml[start..];
    let end = body.find("\n\nenv:").unwrap_or(body.len());
    Some(&body[..end])
}

fn yaml_step_blocks_with_cargo_mutants(ci_yml: &str) -> Vec<&str> {
    ci_yml
        .split("\n      - name:")
        .filter(|block| {
            non_comment_lines(block)
                .iter()
                .any(|line| line.contains("cargo mutants"))
        })
        .collect()
}

fn in_diff_filter_matches_path(filter_line: &str, required_path: &str) -> bool {
    if required_path.starts_with("crates/tracewake-core/src/agent/") {
        filter_line.contains("crates/tracewake-core/src/agent/|")
    } else if required_path.starts_with("crates/tracewake-core/src/runtime/") {
        filter_line.contains("crates/tracewake-core/src/runtime/")
    } else if required_path.starts_with("crates/tracewake-core/src/events/") {
        filter_line.contains("crates/tracewake-core/src/events/")
    } else if required_path.starts_with("crates/tracewake-core/src/replay/") {
        filter_line.contains("crates/tracewake-core/src/replay/")
    } else if required_path == "crates/tracewake-core/src/scheduler.rs" {
        filter_line.contains("crates/tracewake-core/src/scheduler\\.rs")
    } else if required_path == "crates/tracewake-core/src/projections.rs" {
        filter_line.contains("crates/tracewake-core/src/projections\\.rs")
    } else if required_path == "crates/tracewake-core/src/actions/pipeline.rs" {
        filter_line.contains("crates/tracewake-core/src/actions/pipeline\\.rs")
    } else if required_path.starts_with("crates/tracewake-core/src/actions/defs/") {
        grouped_regex_contains_stem(filter_line, "actions/defs/(", ")\\.rs", required_path)
    } else if matches!(
        required_path,
        "crates/tracewake-core/src/actions/proposal.rs"
            | "crates/tracewake-core/src/actions/report.rs"
    ) {
        grouped_regex_contains_stem(filter_line, "actions/(", ")\\.rs", required_path)
    } else if matches!(
        required_path,
        "crates/tracewake-core/src/epistemics/knowledge_context.rs"
            | "crates/tracewake-core/src/epistemics/projection.rs"
    ) {
        filter_line.contains("crates/tracewake-core/src/epistemics/")
            || grouped_regex_contains_stem(filter_line, "epistemics/(", ")\\.rs", required_path)
    } else if required_path.starts_with("crates/tracewake-core/src/content/") {
        grouped_regex_contains_stem(filter_line, "content/(", ")\\.rs", required_path)
    } else if required_path.starts_with("crates/tracewake-content/src/") {
        grouped_regex_contains_stem(
            filter_line,
            "tracewake-content/src/(",
            ")\\.rs",
            required_path,
        )
    } else if required_path.starts_with("crates/tracewake-tui/src/") {
        grouped_regex_contains_stem(filter_line, "tracewake-tui/src/(", ")\\.rs", required_path)
    } else if required_path.ends_with(".rs") {
        let escaped_path = required_path.replace('.', "\\.");
        filter_line.contains(&escaped_path)
    } else {
        false
    }
}

fn grouped_regex_contains_stem(
    filter_line: &str,
    prefix: &str,
    suffix: &str,
    required_path: &str,
) -> bool {
    let stem = required_path
        .rsplit('/')
        .next()
        .and_then(|file_name| file_name.strip_suffix(".rs"))
        .unwrap_or(required_path);
    filter_line
        .split(prefix)
        .nth(1)
        .and_then(|tail| tail.split(suffix).next())
        .is_some_and(|group| group.split('|').any(|entry| entry == stem))
}

fn mutation_rationale_violations(classifications: &[WorkspaceSourceClassification]) -> Vec<String> {
    classifications
        .iter()
        .filter_map(|entry| match entry.class {
            WorkspaceSourceClass::GuardedLayer => None,
            WorkspaceSourceClass::Exempt { rationale }
                if rationale.contains("mutation")
                    && !MUTATION_PERIMETER_CANARY_PATHS.contains(&entry.path) =>
            {
                Some(format!(
                    "{} claims mutation coverage but is outside the mutation perimeter",
                    entry.path
                ))
            }
            WorkspaceSourceClass::Exempt { .. } => None,
        })
        .collect()
}

fn anti_regression_test_names(source: &str) -> BTreeSet<String> {
    let mut tests = BTreeSet::new();
    let mut previous_was_test_attr = false;
    for line in source.lines() {
        let trimmed = line.trim_start();
        if trimmed == "#[test]" {
            previous_was_test_attr = true;
            continue;
        }
        if previous_was_test_attr {
            if let Some(name) = trimmed
                .strip_prefix("fn ")
                .and_then(|tail| tail.split_once('('))
                .map(|(name, _)| name.to_string())
            {
                tests.insert(name);
            }
            previous_was_test_attr = false;
        }
    }
    tests
}

fn meta_lock_census_exemption_rationale(test_name: &str) -> Option<&'static str> {
    META_LOCK_CENSUS_EXEMPTIONS
        .iter()
        .find(|entry| entry.test_name == test_name)
        .map(|entry| entry.rationale)
}

fn meta_lock_census_exemption_errors(
    test_name: &str,
    rationale: &str,
    lock_ids: &BTreeSet<&str>,
    anti_regression_source: &str,
) -> Vec<String> {
    let mut errors = Vec::new();
    if rationale.trim().is_empty() {
        errors.push(format!("exemption {test_name} has empty rationale"));
    }
    let body = function_body_if_present(anti_regression_source, &format!("fn {test_name}"))
        .unwrap_or_default();
    if !test_body_has_structural_scan_shape(body) {
        return errors;
    }
    if historical_acceptance_anchor_audit_exemption(test_name) {
        if !body.contains("acceptance_checklist_anchor_errors(") {
            errors.push(format!(
                "historical-audit exemption {test_name} does not run the acceptance checklist anchor checker"
            ));
        }
        return errors;
    }
    let Some(covering_lock_id) = rationale
        .split("lock_id ")
        .nth(1)
        .and_then(|tail| tail.split_whitespace().next())
        .map(|token| token.trim_matches(|ch: char| ch == '.' || ch == ',' || ch == '`'))
    else {
        errors.push(format!(
            "scan-shaped exemption {test_name} lacks covering lock_id"
        ));
        return errors;
    };
    if !lock_ids.contains(covering_lock_id) {
        errors.push(format!(
            "scan-shaped exemption {test_name} names missing covering lock_id {covering_lock_id}"
        ));
    }
    errors
}

fn historical_acceptance_anchor_audit_exemption(test_name: &str) -> bool {
    matches!(
        test_name,
        "acceptance_artifact_0021_maps_spec_section_7_items_to_report_anchors"
            | "acceptance_artifact_0022_maps_spec_section_7_items_to_report_anchors"
    )
}

fn test_body_has_structural_scan_shape(body: &str) -> bool {
    body.contains("_errors(")
        || body.contains("_violations(")
        || body.contains("_count(")
        || (body.contains(".filter(") && body.contains(".count()"))
        || (body.contains(".matches(") && body.contains(").count()"))
}

fn function_body_if_present<'a>(source: &'a str, marker: &str) -> Option<&'a str> {
    source
        .split(marker)
        .nth(1)
        .and_then(|after_marker| after_marker.find('{').map(|start| (after_marker, start)))
        .and_then(|(after_marker, start)| {
            let mut depth = 0_i32;
            for (offset, byte) in after_marker[start..].bytes().enumerate() {
                match byte {
                    b'{' => depth += 1,
                    b'}' => {
                        depth -= 1;
                        if depth == 0 {
                            return Some(&after_marker[start..start + offset + 1]);
                        }
                    }
                    _ => {}
                }
            }
            None
        })
}

fn meta_lock_live_witness_count(
    entry: &MetaLockRegistryEntry,
    anti_regression_source: &str,
) -> usize {
    if entry.lock_id == "event_apply_remains_only_post_seed_mutation_path" {
        return usize::from(
            anti_regression_source
                .contains("no_direct_apply_event_outside_event_replay_or_pipeline();"),
        ) + usize::from(
            anti_regression_source
                .contains("accepted_action_appends_before_authoritative_apply();"),
        );
    }
    if entry.lock_id == "no_direct_apply_event_outside_event_replay_or_pipeline" {
        return apply_mutator_live_witness_count(&core_production_sources());
    }
    if entry.lock_id == "guard_014_embodied_projection_source_has_no_physical_state_field" {
        return non_empty_production_sites(&[production(PROJECTIONS_RS).as_str()]);
    }
    if entry.lock_id == "typed_column_closure_exemptions_are_rationale_bearing_and_live" {
        return TYPED_COLUMN_CLOSURE_EXEMPTIONS.len();
    }
    if entry.lock_id == "materialized_agent_payload_records_keep_payload_fields" {
        return usize::from(STATE_RS.contains("pub payload_fields: Vec<(String, String)>"))
            + EVENTS_APPLY_RS
                .matches("payload_fields: payload_fields(event)")
                .count();
    }
    if matches!(
        entry.routing,
        MetaLockRouting::BehaviorAssertion | MetaLockRouting::TicketOwnedDebt { .. }
    ) {
        return behavior_assertion_inspected_site_count(entry);
    }
    if entry.lock_id == "meta_lock_registry_census" {
        return anti_regression_test_names(anti_regression_source).len();
    }
    if entry.lock_id == "mutation_baseline_misses_are_pinned_and_ledgered" {
        return normalized_mutant_misses(MUTANTS_BASELINE_MISSES).len();
    }
    if entry.lock_id == "generative_lock_two_sided_floor_ratchets" {
        return RECORDED_GENERATIVE_MULTI_SEED_CONTRIBUTORS
            .iter()
            .filter(|(_, count)| *count > 0)
            .count();
    }
    if entry.lock_id == "physical_mutating_event_kinds_have_explicit_world_apply_arms" {
        if !anti_regression_source
            .contains("body_after_marker(EVENTS_APPLY_RS, \"fn apply_event_with_capability\")")
        {
            return 0;
        }
        let Some(apply_body) =
            function_body_if_present(EVENTS_APPLY_RS, "fn apply_event_with_capability")
        else {
            return 0;
        };
        return tracewake_core::events::EventKind::all()
            .iter()
            .filter(|kind| kind.metadata().physical_mutating)
            .filter(|kind| apply_body.contains(&format!("EventKind::{kind:?}")))
            .count();
    }
    if entry.lock_id == "agent_stream_event_kinds_have_explicit_agent_apply_arms" {
        use tracewake_core::events::apply::AGENT_WORLD_NOOP_ALLOWLIST;
        use tracewake_core::events::{EventKind, EventStream};

        if !anti_regression_source.contains(
            "body_after_marker(EVENTS_APPLY_RS, \"fn apply_agent_event_with_capability\")",
        ) {
            return 0;
        }
        let Some(apply_body) =
            function_body_if_present(EVENTS_APPLY_RS, "fn apply_agent_event_with_capability")
        else {
            return 0;
        };
        return EventKind::all()
            .iter()
            .filter(|kind| kind.metadata().stream == EventStream::Agent)
            .filter(|kind| !AGENT_WORLD_NOOP_ALLOWLIST.contains(kind))
            .filter(|kind| apply_body.contains(&format!("EventKind::{kind:?}")))
            .count();
    }
    if entry.lock_id == "action_emitted_event_kinds_have_cause_disposition" {
        return [
            MOVEMENT_RS,
            WAIT_RS,
            OPENCLOSE_RS,
            CHECKCONTAINER_RS,
            TAKEPLACE_RS,
            PERCEPTION_RS,
            EAT_RS,
            SLEEP_RS,
            WORK_RS,
        ]
        .iter()
        .filter(|source| source.contains("EventEnvelope::new_caused_v1"))
        .count();
    }
    if entry.lock_id == "scheduler_apply_and_completion_paths_do_not_panic_on_log_derived_data" {
        return [
            production(SCHEDULER_RS),
            production(EVENTS_APPLY_RS),
            production(SLEEP_RS),
            production(WORK_RS),
        ]
        .iter()
        .filter(|source| source.contains(".expect(") || source.contains("assert!("))
        .count();
    }
    if entry.lock_id == "embodied_view_option_and_collection_fields_have_reachable_producers" {
        return embodied_surface_fields(&production(VIEW_MODELS_RS)).len();
    }
    if entry.lock_id == "guard_014_perception_visibility_uses_typed_place_visibility"
        || entry.lock_id == "guard_014_perception_visibility_other_emission_paths"
    {
        return perception_prose_scan_inspected_line_count(PERCEPTION_RS);
    }
    if entry.lock_id == "typed_column_closure_oblique_payload_helper_calls" {
        return typed_column_closure_oblique_inspected_site_count(
            EVENTS_APPLY_RS,
            TYPED_COLUMN_CLOSURE_EXEMPTIONS,
        );
    }
    if entry.lock_id == "guard_011_no_human_day_runner_only_evidence" {
        return no_human_day_runner_only_inspected_site_count(CONTENT_GOLDEN_FIXTURES_RUN_RS);
    }
    if entry.lock_id == "guard_0021_actor_known_context_producers_are_projection_backed" {
        return ACTOR_KNOWN_RS.matches("from_observed_parts").count();
    }
    if entry.lock_id == "guard_0021_hidden_truth_gates_use_event_log_provenance" {
        return function_body_if_present(
            anti_regression_source,
            "fn guard_0021_hidden_truth_gates_use_event_log_provenance",
        )
        .map_or(0, |body| {
            usize::from(body.contains("hidden_truth_harness_provenance_violations"))
        });
    }
    if entry.lock_id == "guard_0021_fabricated_visible_local_event_id_is_retired" {
        return function_body_if_present(
            anti_regression_source,
            "fn guard_0021_fabricated_visible_local_event_id_is_retired",
        )
        .map_or(0, |body| {
            usize::from(body.contains("fabricated_planning_event_id_violations"))
        });
    }
    if entry.lock_id == "guard_008_phase1_loader_does_not_register_later_phase_actions" {
        return usize::from(CONTENT_LOAD_RS.contains("FixtureScope::Phase1"));
    }
    if entry.lock_id == "guard_008_phase1_loader_source_guard_has_mutation_self_coverage" {
        return phase1_loader_later_phase_registration_violations(&CONTENT_LOAD_RS.replace(
            "FixtureScope::Phase1 => {}",
            "FixtureScope::Phase1 => { registry.register_phase2a_epistemics(); }",
        ))
        .len();
    }
    if entry.lock_id == "mutation_perimeter_logical_line_swallow_scan" {
        return logical_shell_lines(CI_YML)
            .into_iter()
            .filter(|line| line.contains("cargo mutants"))
            .count();
    }
    if entry.lock_id == "mutation_baseline_non_empty_entries_require_ledger_dispositions" {
        return mutation_baseline_governance_errors(
            "synthetic/path.rs:1:1: replace x -> y",
            MUTANTS_BASELINE_LEDGER,
        )
        .len();
    }
    if entry.lock_id == "ci_workflow_guards_cover_workflow_integrity" {
        return [
            "synthetic masked gate step must fail",
            "synthetic unpinned third-party action must fail",
            "synthetic missing permissions must fail",
            "synthetic target cache key without toolchain/manifests must fail",
            "synthetic missing verbatim gate command must fail",
            "synthetic undocumented workflow job must fail",
        ]
        .iter()
        .filter(|needle| CI_WORKFLOW_GUARDS_RS.contains(**needle))
        .count();
    }
    if entry.lock_id == "hidden_truth_context_discrimination_witness" {
        return hidden_truth_context_discrimination_witness_count();
    }
    if entry.lock_id == "log_derived_panic_guard_scans_unwrap" {
        return log_derived_panic_violations(&[(
            "synthetic.rs",
            "fn f() { value.unwrap(); }".to_string(),
        )])
        .len();
    }
    if entry.lock_id == "generative_support_constructs_zero_event_envelopes"
        || entry.lock_id == "generative_support_bans_bare_event_envelope_token"
    {
        return generative_duration_terminal_fabricator_errors(&[(
            "support/generative.rs",
            "fn helper() { let _event = EventEnvelope::default(); }",
        )])
        .len();
    }
    if entry.lock_id == "event_kind_cause_required_exhaustive_match" {
        return usize::from(
            body_after_marker(EVENTS_ENVELOPE_RS, "const fn cause_required").contains("match self"),
        );
    }
    if entry.lock_id == "event_envelope_duplicate_fields_reject_loudly" {
        return usize::from(EVENTS_ENVELOPE_RS.contains("DuplicateField"))
            + usize::from(
                EVENTS_MOD_RS.contains("fn envelope_deserialization_rejects_duplicate_fields"),
            );
    }
    if entry.lock_id == "event_envelope_direct_decode_rejects_unsupported_schema_version" {
        return usize::from(EVENTS_ENVELOPE_RS.contains("UnsupportedSchemaVersion"))
            + usize::from(
                EVENTS_MOD_RS
                    .contains("fn envelope_direct_decode_rejects_unsupported_schema_version"),
            );
    }
    if entry.lock_id == "event_envelope_has_no_hollow_checksum_after_field" {
        return usize::from(!EVENTS_ENVELOPE_RS.contains("checksum_after"));
    }
    if entry.lock_id == "guard_014_perception_visibility_detects_display_label_binding_laundering" {
        return perception_visibility_prose_branch_violations(
            "fn is_visible_exit_target(place: &PlaceState) { let visible = place.display_label != \"Hidden\"; }",
        )
        .len();
    }
    if entry.lock_id == "guard_014_perception_visibility_detects_bare_display_label_substrings" {
        return perception_visibility_prose_branch_violations(
            "fn is_visible_exit_target(place: &PlaceState) { place.display_label.starts_with(\"hid\"); }",
        )
        .len();
    }
    if entry.lock_id == "typed_column_closure_payload_receiver_helper_calls"
        || entry.lock_id == "typed_column_closure_payload_alias_helper_calls"
    {
        return typed_column_payload_helper_inspected_site_count(entry.lock_id);
    }
    if entry.lock_id == "hidden_food_closed_container_is_not_actor_known_food_source" {
        return usize::from(
            CONTENT_FIXTURE_HIDDEN_FOOD_CLOSED_CONTAINER_RS
                .contains("hidden_food_closed_container"),
        );
    }
    if entry.lock_id == "hidden_route_edge_absent_from_actor_context_blocks_route_plan" {
        return usize::from(CONTENT_FIXTURE_HIDDEN_ROUTE_EDGE_RS.contains("hidden_route_edge"));
    }
    if entry.lock_id == "planner_hidden_truth_fixture_witness_fails_on_empty_adversarial_context" {
        return function_body_if_present(
            HIDDEN_TRUTH_GATES_RS,
            "fn planner_hidden_truth_fixture_witness_fails_on_empty_adversarial_context",
        )
        .map_or(0, |body| {
            body.matches("planner_hidden_truth_fixture_witness_errors")
                .count()
        });
    }
    if entry.lock_id == "actor_known_projection_policy_table_drives_record_behavior" {
        return production(EPISTEMIC_PROJECTION_RS)
            .matches("includes_in_embodied_context(")
            .count();
    }
    if entry.lock_id == "workplace_current_place_scope_drops_other_place_from_embodied_context" {
        return production(EPISTEMIC_PROJECTION_RS)
            .matches("is_latest_current_place_record")
            .count();
    }
    if entry.lock_id == "supersede_newest_by_subject_requires_subject_extractor" {
        return production(EPISTEMIC_PROJECTION_RS)
            .matches("supersede_workplace_subject")
            .count();
    }
    if entry.lock_id == "workspace_dependency_posture_matches_allowlist" {
        return WORKSPACE_DEPENDENCY_ALLOWLIST.len();
    }
    if entry.lock_id == "checksum_coverage_is_total_for_authoritative_state" {
        return tracewake_core::checksum::PHYSICAL_STATE_CHECKSUM_COVERAGE.len()
            + tracewake_core::checksum::AGENT_STATE_CHECKSUM_COVERAGE.len();
    }
    if entry.lock_id == "guard_006_duration_need_deltas_route_through_shared_emitter" {
        return direct_duration_need_delta_construction_violations(&[(
            "crates/tracewake-core/src/actions/defs/work.rs".to_string(),
            "fn build_synthetic_delta() { EventEnvelope::new_caused_v1(event_id, EventKind::NeedDeltaApplied, 0, 0, tick, ordering_key, content_manifest_id, causes); }"
                .to_string(),
        )])
        .len();
    }
    if entry.lock_id == "acceptance_artifact_0023_maps_spec_section_7_items_to_report_anchors" {
        return acceptance_checklist_anchor_errors(
            ACCEPTANCE_0023_REPORT,
            &[AcceptanceChecklistAnchor {
                item: 99,
                anchors: &["synthetic_witness_missing_acceptance_anchor"],
            }],
        )
        .len();
    }
    if entry.lock_id == "acceptance_artifact_0024_maps_spec_section_7_items_to_report_anchors" {
        return acceptance_checklist_anchor_errors(
            ACCEPTANCE_0024_REPORT,
            &[AcceptanceChecklistAnchor {
                item: 99,
                anchors: &["synthetic_witness_missing_acceptance_anchor"],
            }],
        )
        .len();
    }
    if entry.lock_id == "acceptance_artifact_0025_maps_spec_section_7_items_to_report_anchors" {
        return acceptance_checklist_anchor_errors(
            ACCEPTANCE_0025_REPORT,
            &[AcceptanceChecklistAnchor {
                item: 99,
                anchors: &["synthetic_witness_missing_acceptance_anchor"],
            }],
        )
        .len();
    }
    0
}

fn non_empty_production_sites(sources: &[&str]) -> usize {
    sources
        .iter()
        .filter(|source| !source.trim().is_empty())
        .count()
}

fn witness_kind_arm_inspected_sites(transaction_source: &str) -> usize {
    function_body_if_present(transaction_source, "fn witness_kind_allowed")
        .map_or(0, |body| usize::from(!body.trim().is_empty()))
        * NO_HUMAN_SURFACE_FACT_STABLE_IDS.len()
}

fn perception_prose_scan_inspected_line_count(source: &str) -> usize {
    source_without_comments(source)
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count()
}

fn witness_shape_ban_errors(witness_body: &str) -> Vec<String> {
    [
        "test_names.contains(entry.lock_id)",
        "anti_regression_source.contains(entry.negative_id)",
        "matches(\"assert",
    ]
    .iter()
    .filter(|fragment| witness_body.contains(*fragment))
    .map(|fragment| format!("banned witness shape: {fragment}"))
    .collect()
}

fn behavior_assertion_inspected_site_count(entry: &MetaLockRegistryEntry) -> usize {
    use tracewake_core::checksum::{
        AGENT_STATE_CHECKSUM_COVERAGE, PHYSICAL_STATE_CHECKSUM_COVERAGE,
    };

    match entry.lock_id {
        "scheduler_never_direct_dispatches_primitive_action"
        | "guard_006_scheduler_has_no_routine_family_to_primitive_dispatch"
        | "guard_006_continue_routine_marker_alone_is_not_behavioral_progress" => {
            non_empty_production_sites(&[production(SCHEDULER_RS).as_str()])
        }
        "guard_0057_embodied_continue_non_proposed_outcome_is_typed_stuck" => {
            non_empty_production_sites(&[production(RUNTIME_SESSION_RS).as_str()])
        }
        "guard_0057_continue_routine_progress_of_record_is_follow_on" => {
            non_empty_production_sites(&[
                production(CONTINUE_ROUTINE_RS).as_str(),
                production(RUNTIME_SESSION_RS).as_str(),
                production(SCHEDULER_RS).as_str(),
            ])
        }
        "guard_0058_embodied_routine_family_has_no_pre_intention_workplace_selector"
        | "guard_0058_embodied_continue_time_advancing_follow_on_is_gated"
        | "guard_0058_embodied_continue_success_path_not_current_stuck" => {
            non_empty_production_sites(&[production(RUNTIME_SESSION_RS).as_str()])
        }
        "guard_0058_tui_continue_routine_forwards_only" => {
            non_empty_production_sites(&[production(TUI_APP_RS).as_str()])
        }
        "guard_0059_scheduler_routine_family_authority_cannot_bypass_active_intention"
        | "guard_0059_no_clock_keyed_routine_family_selector_without_active_intention_binding" => {
            non_empty_production_sites(&[production(SCHEDULER_RS).as_str()])
        }
        "guard_0059_synthetic_negative_census_is_live" => {
            synthetic_0059_negative_ids()
                .iter()
                .filter(|negative_id| ANTI_REGRESSION_GUARDS_RS.contains(**negative_id))
                .count()
        }
        "guard_006_scheduler_has_no_direct_routine_or_need_proposal_bypass" => {
            guarded_sources_for(GuardedLayer::Scheduler).len()
        }
        "guard_006_scheduler_does_not_fabricate_empty_epistemic_projection" => {
            non_empty_production_sites(&[guarded_source("src/scheduler.rs").as_str()])
        }
        "guard_018_actor_known_facts_require_source_event_witness" => {
            non_empty_production_sites(&[
                production(ACTOR_KNOWN_RS).as_str(),
                production(NO_HUMAN_SURFACE_RS).as_str(),
                production(TRANSACTION_RS).as_str(),
            ])
        }
        "guard_018_witness_kind_no_human_fact_stable_ids_have_explicit_arms" => {
            witness_kind_arm_inspected_sites(&production(TRANSACTION_RS))
        }
        "guard_014_no_human_cognition_surface_does_not_read_raw_assignment_or_sleep_truth" => {
            guarded_sources_for(GuardedLayer::Scheduler).len()
                + non_empty_production_sites(&[
                    guarded_source("src/agent/no_human_surface.rs").as_str()
                ])
        }
        "guard_015_ord_hard_008_cognition_channel_stays_evented_and_sealed" => {
            guarded_sources_for(GuardedLayer::Scheduler).len()
                + guarded_sources_for(GuardedLayer::Agent).len()
        }
        "guard_014_embodied_projection_workplaces_are_context_backed"
        | "guard_014_no_human_metrics_do_not_scan_display_text" => {
            guarded_sources_for(GuardedLayer::Projections).len()
        }
        "guard_014_phase3a_semantic_actions_do_not_use_literal_true_availability" => {
            non_empty_production_sites(&[production(PROJECTIONS_RS).as_str()])
        }
        "guard_014_sleep_validation_requires_modeled_affordance" => non_empty_production_sites(&[
            production(SLEEP_RS).as_str(),
            production(PROJECTIONS_RS).as_str(),
            production(TRANSACTION_RS).as_str(),
            production(NO_HUMAN_SURFACE_RS).as_str(),
        ]),
        "guard_0021_actor_known_projection_policy_table_has_production_callers" => {
            production(EPISTEMIC_PROJECTION_RS)
                .matches(".policy()")
                .count()
        }
        "guard_015_ordinary_life_tuning_comes_from_authored_state" => non_empty_production_sites(
            &[production(SLEEP_RS).as_str(), production(TIME_RS).as_str()],
        ),
        "guard_014_scheduler_cannot_rewrite_transaction_proposals_after_cognition" => {
            non_empty_production_sites(&[
                production(SCHEDULER_RS).as_str(),
                production(TRANSACTION_RS).as_str(),
            ])
        }
        "guard_014_transaction_has_no_silent_method_fallback_scan" => {
            non_empty_production_sites(&[production(TRANSACTION_RS).as_str()])
        }
        "guard_015_hidden_truth_audit_fails_closed_in_transaction" => non_empty_production_sites(
            &[
                production(TRANSACTION_RS).as_str(),
                production(PIPELINE_RS).as_str(),
            ],
        ),
        "guard_014_decision_hidden_truth_audit_uses_typed_input_refs" => {
            non_empty_production_sites(&[production(DECISION_RS).as_str()])
        }
        "guard_003_work_eat_sleep_validators_do_not_read_need_values_from_proposal_parameters" => {
            non_empty_production_sites(&[
                production(EAT_RS).as_str(),
                production(SLEEP_RS).as_str(),
                production(WORK_RS).as_str(),
            ])
        }
        "agent_world_noop_allowlist_is_explicit_and_excludes_materialized_episode_state" => {
            tracewake_core::events::apply::AGENT_WORLD_NOOP_ALLOWLIST.len()
        }
        "materialized_agent_apply_arms_require_payload_schema_version" => {
            AGENT_STATE_CHECKSUM_COVERAGE.len()
        }
        "guard_002_agent_state_keeps_typed_trace_and_diagnostic_records" => {
            non_empty_production_sites(&[STATE_RS])
        }
        "guard_001_authoritative_state_fields_are_not_publicly_mutable" => {
            PHYSICAL_STATE_CHECKSUM_COVERAGE.len() + AGENT_STATE_CHECKSUM_COVERAGE.len()
        }
        "guard_001_mutation_capability_is_private_to_event_application" => {
            non_empty_production_sites(&[EVENTS_MOD_RS, EVENTS_MUTATION_RS, EVENTS_APPLY_RS])
        }
        "guard_001_no_production_seed_mutation_outside_state_definition"
        | "guard_001_no_direct_state_collection_insert_outside_event_application" => {
            core_production_sources().len()
        }
        "guard_001_actor_known_context_has_no_public_arbitrary_constructor" => {
            non_empty_production_sites(&[production(ACTOR_KNOWN_RS).as_str()])
        }
        "adding_event_schema_version_requires_migrator_registration" => {
            tracewake_core::events::event_schema_registry().len()
        }
        "non_world_stream_cannot_change_physical_checksum" => {
            tracewake_core::events::EventKind::all()
                .iter()
                .filter(|kind| {
                    kind.metadata().stream != tracewake_core::events::EventStream::World
                })
                .count()
        }
        "new_authoritative_field_without_checksum_registry_fails"
        | "new_authoritative_field_without_canonical_checksum_line_fails" => {
            checksum_parity_errors(
                "pub struct PhysicalState {\n    pub(crate) actors: BTreeMap<ActorId, ActorBody>,\n    pub(crate) places: BTreeMap<PlaceId, PlaceState>,\n}\n\npub struct AgentState {\n    pub(crate) needs_by_actor: BTreeMap<ActorId, BTreeMap<NeedKind, NeedState>>,\n}\n",
                "lines.push(format!(\"actor|id={}\", actor_id.as_str()));\nlines.push(format!(\"need|actor={}\", actor_id.as_str()));\n",
                &[("actors", "actor")],
                &[("needs_by_actor", "need")],
            )
            .len()
        }
        "guard_001_hidden_truth_audit_is_derived_from_provenance_not_tags" => {
            non_empty_production_sites(&[
                production(ACTOR_KNOWN_RS).as_str(),
                production(DECISION_RS).as_str(),
                production(HTN_RS).as_str(),
                production(PLANNER_RS).as_str(),
            ])
        }
        "guard_007_mutation_efficacy_notes_cover_high_risk_shortcuts" => META_LOCK_REGISTRY
            .iter()
            .filter(|registered| {
                [
                    "guard_006_scheduler_has_no_routine_family_to_primitive_dispatch",
                    "guard_003_work_eat_sleep_validators_do_not_read_need_values_from_proposal_parameters",
                    "guard_002_agent_state_keeps_typed_trace_and_diagnostic_records",
                ]
                .contains(&registered.lock_id)
            })
            .count(),
        "guard_008_action_registry_uses_typed_scopes_not_phase1_boolean" => {
            non_empty_production_sites(&[production(ACTIONS_REGISTRY_RS).as_str()])
        }
        "embodied_projection_never_uses_unfiltered_checked_facts_as_actor_provenance" => {
            production(PROJECTIONS_RS)
                .matches("report.actor_visible_facts.iter()")
                .count()
        }
        _ => 0,
    }
}

fn typed_column_closure_oblique_inspected_site_count(
    source: &str,
    exemptions: &[TypedColumnClosureExemption],
) -> usize {
    exemptions
        .iter()
        .filter(|exemption| !payload_helper_calls_for_anchor(source, exemption.anchor).is_empty())
        .count()
}

fn payload_helper_calls_for_anchor(source: &str, anchor: &str) -> BTreeSet<String> {
    let scan_source = normalized_source(source).replace(" .", ".");
    let Some(body) = function_body(&scan_source, anchor) else {
        return BTreeSet::new();
    };
    let payload_bindings = payload_binding_aliases(body);
    payload_helper_calls(body, &payload_bindings)
}

fn typed_column_payload_helper_inspected_site_count(lock_id: &str) -> usize {
    if lock_id == "typed_column_closure_payload_alias_helper_calls" {
        return function_body_if_present(
            ANTI_REGRESSION_GUARDS_RS,
            "fn typed_column_closure_exemptions_are_rationale_bearing_and_live",
        )
        .map_or(0, |body| {
            body.matches("synthetic_payload_alias_source").count()
        });
    }
    let source = normalized_source(EVENTS_APPLY_RS).replace(" .", ".");
    let mut count = 0;
    for exemption in TYPED_COLUMN_CLOSURE_EXEMPTIONS {
        let Some(body) = function_body(&source, exemption.anchor) else {
            continue;
        };
        let payload_bindings = payload_binding_aliases(body);
        let helper_calls = payload_helper_calls(body, &payload_bindings);
        count += helper_calls
            .iter()
            .filter(|call| match lock_id {
                "typed_column_closure_payload_receiver_helper_calls" => {
                    body.contains(&format!(".{call}("))
                }
                "typed_column_closure_payload_alias_helper_calls" => {
                    body.contains(&format!("let {call} = &payload"))
                        || body.contains(&format!("let {call} = payload"))
                }
                _ => false,
            })
            .count();
    }
    count
}

fn hidden_truth_context_discrimination_witness_count() -> usize {
    function_body_if_present(
        HIDDEN_TRUTH_GATES_RS,
        "fn context_rejects_hidden_counterpart_injection",
    )
    .map_or(0, |body| {
        body.matches("synthetic_context_hidden_food_injection")
            .count()
            + body
                .matches("synthetic_context_hidden_route_injection")
                .count()
    })
}

fn no_human_day_runner_only_inspected_site_count(source: &str) -> usize {
    source
        .lines()
        .filter(|line| {
            line.contains("has_no_human_event_for_actor")
                || line.contains("EventKind::EatFailed")
                || line.contains("\"actor_mara\"")
        })
        .count()
}

fn meta_lock_registry_errors(
    registry: &[MetaLockRegistryEntry],
    anti_regression_source: &str,
) -> Vec<String> {
    let mut errors = Vec::new();
    let mut lock_ids = BTreeSet::new();
    let mut negative_ids = BTreeSet::new();

    if registry.len() < META_LOCK_REGISTRY_MIN_ENTRIES {
        errors.push(format!(
            "meta-lock registry enumerates too few locks: expected at least {}, got {}",
            META_LOCK_REGISTRY_MIN_ENTRIES,
            registry.len()
        ));
    }

    for entry in registry {
        if !lock_ids.insert(entry.lock_id) {
            errors.push(format!(
                "meta-lock registry duplicates lock {}",
                entry.lock_id
            ));
        }
        if entry.negative_id.trim().is_empty() {
            errors.push(format!(
                "meta-lock {} lacks a registered negative",
                entry.lock_id
            ));
        } else if !negative_ids.insert(entry.negative_id) {
            errors.push(format!(
                "meta-lock registry duplicates negative {}",
                entry.negative_id
            ));
        }
        match entry.routing {
            MetaLockRouting::SharedScan | MetaLockRouting::BehaviorAssertion => {}
            MetaLockRouting::TicketOwnedDebt { ticket } => {
                if !ticket_exists(ticket) {
                    errors.push(format!(
                        "meta-lock {} routes artifact-shaped negative to missing ticket {}",
                        entry.lock_id, ticket
                    ));
                }
            }
        }
        if entry.witness_min == 0
            && entry.lock_id != "mutation_baseline_misses_are_pinned_and_ledgered"
        {
            errors.push(format!(
                "meta-lock {} has a zero witness minimum",
                entry.lock_id
            ));
        }
        let witness_count = meta_lock_live_witness_count(entry, anti_regression_source);
        if witness_count < entry.witness_min {
            errors.push(format!(
                "meta-lock {} witness count {} is below minimum {}",
                entry.lock_id, witness_count, entry.witness_min
            ));
        }
    }

    if !lock_ids.contains("meta_lock_registry_census") {
        errors.push("meta-lock registry does not list its own census lock".to_string());
    }
    if !negative_ids.contains("synthetic_meta_lock_without_negative") {
        errors.push("meta-lock registry does not list its reflexive negative".to_string());
    }

    for test_name in anti_regression_test_names(anti_regression_source) {
        if lock_ids.contains(test_name.as_str()) {
            continue;
        }
        match meta_lock_census_exemption_rationale(&test_name) {
            Some(rationale) => errors.extend(meta_lock_census_exemption_errors(
                &test_name,
                rationale,
                &lock_ids,
                anti_regression_source,
            )),
            _ => {
                errors.push(format!(
                    "anti-regression test {test_name} is missing from meta-lock registry or rationale-bearing exemption"
            ));
            }
        }
    }

    errors
}

fn mutation_perimeter_consistency_violations(mutants_toml: &str, ci_yml: &str) -> Vec<String> {
    let mut violations = Vec::new();

    for key in parse_mutants_toml_top_level_keys(mutants_toml) {
        if !matches!(
            key.as_str(),
            "additional_cargo_args" | "test_workspace" | "examine_globs" | "exclude_globs"
        ) {
            violations.push(format!("mutants.toml uses unsupported key {key}"));
        }
    }

    if !mutants_toml.contains(r#"additional_cargo_args = ["--locked"]"#) {
        violations.push("mutants.toml omits locked cargo args".to_string());
    }
    if mutants_toml.contains(r#"additional_cargo_args = ["--workspace", "--locked"]"#)
        || mutants_toml.contains(r#"additional_cargo_args = ["--locked", "--workspace"]"#)
    {
        violations.push(
            "mutants.toml duplicates cargo test workspace args; test_workspace supplies workspace testing"
                .to_string(),
        );
    }
    if !mutants_toml.contains("test_workspace = true") {
        violations.push("mutants.toml omits test_workspace = true".to_string());
    }

    let scheduled_baseline_block =
        yaml_step_run_block(ci_yml, "Run unmutated baseline and canonical census")
            .unwrap_or_default();
    let scheduled_baseline_effective_lines = non_comment_lines(scheduled_baseline_block).join("\n");
    let scheduled_shard_block =
        yaml_step_run_block(ci_yml, "Run supervised mutation shard").unwrap_or_default();
    let scheduled_shard_effective_lines = non_comment_lines(scheduled_shard_block).join("\n");
    let scheduled_reconcile_block =
        yaml_step_run_block(ci_yml, "Reconcile shard union").unwrap_or_default();
    let in_diff_block = yaml_step_run_block(ci_yml, "Check guarded-layer diff").unwrap_or_default();
    let in_diff_filter_lines = non_comment_lines(in_diff_block)
        .into_iter()
        .filter(|line| {
            line.contains("git diff --name-only")
                && line.contains("grep -E")
                && line.contains("actions/defs/")
        })
        .collect::<Vec<_>>();
    if in_diff_filter_lines.len() != 1 {
        violations.push(format!(
            "in-diff mutation guarded-path filter must have exactly one git diff --name-only grep line, found {}",
            in_diff_filter_lines.len()
        ));
    }
    let in_diff_filter_line = in_diff_filter_lines.first().copied().unwrap_or_default();

    if !scheduled_baseline_effective_lines.contains("cargo mutants --workspace --no-shuffle --list")
        || !scheduled_shard_effective_lines.contains("cargo mutants --workspace --no-shuffle")
    {
        violations.push(
            "scheduled mutation lane does not use checked-in cargo-mutants config command"
                .to_string(),
        );
    }
    if scheduled_baseline_effective_lines
        .lines()
        .chain(scheduled_shard_effective_lines.lines())
        .filter(|line| line.contains("cargo mutants"))
        .any(|line| {
            line.contains(" -f ") || line.contains(" --file ") || line.contains("--no-config")
        })
    {
        violations.push("scheduled mutation lane declares a divergent file perimeter".to_string());
    }

    let examine_globs = parse_examine_globs(mutants_toml);
    for required_glob in MUTATION_PERIMETER_EXAMINE_GLOBS {
        if !examine_globs.iter().any(|glob| glob == required_glob) {
            violations.push(format!(
                "mutants.toml examine_globs omits required standing glob {required_glob}"
            ));
        }
    }
    for glob in &examine_globs {
        if !recognized_mutants_glob(glob) {
            violations.push(format!(
                "mutants.toml examine_globs entry {glob} uses unsupported glob shape"
            ));
        }
    }
    for required_path in MUTATION_PERIMETER_CANARY_PATHS {
        if !examine_globs
            .iter()
            .any(|glob| simple_glob_matches(glob, required_path))
        {
            violations.push(format!(
                "mutants.toml examine_globs do not cover mutation perimeter path {required_path}"
            ));
        }
    }

    for excluded in parse_exclude_globs(mutants_toml) {
        if !recognized_mutants_glob(&excluded) {
            violations.push(format!(
                "mutants.toml exclude_globs entry {excluded} uses unsupported glob shape"
            ));
        }
        for required_path in MUTATION_PERIMETER_CANARY_PATHS {
            if simple_glob_matches(&excluded, required_path) {
                violations.push(format!(
                    "mutants.toml exclude_globs entry {excluded} covers mutation perimeter path {required_path}"
                ));
            }
        }
    }

    for required_path in MUTATION_PERIMETER_CANARY_PATHS {
        if !in_diff_filter_matches_path(in_diff_filter_line, required_path) {
            violations.push(format!(
                "in-diff mutation guarded-path filter omits {required_path}"
            ));
        }

        if !WORKSPACE_SOURCE_CLASSIFICATIONS
            .iter()
            .any(|entry| entry.path == *required_path)
        {
            violations.push(format!(
                "{required_path} missing from source classification table"
            ));
        }
    }

    violations.extend(mutation_rationale_violations(
        WORKSPACE_SOURCE_CLASSIFICATIONS,
    ));

    for line in logical_shell_lines(ci_yml)
        .into_iter()
        .filter(|line| line.contains("cargo mutants"))
    {
        if line.contains("||") || line.contains("&&") {
            violations.push("cargo-mutants invocation has shell suffix".to_string());
        }
    }
    for block in yaml_step_blocks_with_cargo_mutants(ci_yml) {
        let effective_lines = non_comment_lines(block);
        if effective_lines
            .iter()
            .any(|line| line.contains("cargo mutants --in-diff"))
            && !effective_lines
                .iter()
                .any(|line| line.contains("mutants_status=$?"))
        {
            violations.push(
                "in-diff cargo-mutants step does not capture status in the same step block"
                    .to_string(),
            );
        }
        if effective_lines
            .iter()
            .any(|line| line.contains("tools/supervise-command.sh"))
            && !effective_lines
                .iter()
                .any(|line| line.contains("supervisor_status=$?"))
        {
            violations.push(
                "supervised cargo-mutants shard does not capture supervisor status in the same step block"
                    .to_string(),
            );
        }
    }
    if !ci_yml.contains("\"$mutants_status\" -ne 0")
        || !ci_yml.contains("\"$mutants_status\" -ne 2")
        || !ci_yml.contains("\"$mutants_status\" -ne 3")
    {
        violations.push(
            "in-diff cargo-mutants status handling does not separate tool failure from misses/timeouts"
                .to_string(),
        );
    }
    for required in [
        "mutants.out/missed.txt",
        "mutants.out/timeout.txt",
        "new_survivors",
    ] {
        if !ci_yml.contains(required) {
            violations.push(format!(
                "in-diff cargo-mutants survivor reconciliation omits {required}"
            ));
        }
    }
    if !ci_yml.contains("[ ! -d mutants.out ]") {
        violations.push("in-diff mutation job does not require output artifacts".to_string());
    }
    if !scheduled_shard_block.contains("exit \"$supervisor_status\"") {
        violations
            .push("scheduled cargo-mutants shard status is not exposed to job status".to_string());
    }
    if !ci_yml.contains("mutants-lock-layer-reconcile:")
        || !scheduled_reconcile_block.contains("python3 tools/merge-mutation-shards.py")
    {
        violations.push("scheduled mutation reconciliation job is missing".to_string());
    }
    if !ci_yml.contains("github.event_name != 'schedule'")
        || !ci_yml.contains("github.event_name != 'workflow_dispatch'")
    {
        violations.push(
            "scheduled/manual mutation runs are not exempt from concurrency cancellation"
                .to_string(),
        );
    }
    let concurrency_group_line = yaml_concurrency_block(ci_yml)
        .unwrap_or_default()
        .lines()
        .find(|line| line.trim_start().starts_with("group:"))
        .unwrap_or_default();
    if !concurrency_group_line.contains("github.event_name") {
        violations.push("concurrency group does not isolate github.event_name".to_string());
    }
    if !ci_yml.contains("github.event_name == 'pull_request' || github.event_name == 'push'") {
        violations.push("mutation in-diff job does not run on guarded direct pushes".to_string());
    }
    if !ci_yml.contains("HEAD^..HEAD") {
        violations.push("push mutation diff does not compare against HEAD^".to_string());
    }

    violations
}

fn normalized_mutant_misses(source: &str) -> std::collections::BTreeSet<String> {
    source
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.splitn(4, ':');
            let Some(path) = parts.next() else {
                return line.to_string();
            };
            let Some(_line_number) = parts.next() else {
                return line.to_string();
            };
            let Some(_column_number) = parts.next() else {
                return line.to_string();
            };
            let Some(rest) = parts.next() else {
                return line.to_string();
            };
            format!("{path}: {}", rest.trim_start())
        })
        .collect()
}

fn canonical_lines_hash(lines: &std::collections::BTreeSet<String>) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for line in lines {
        for byte in line.bytes().chain(std::iter::once(b'\n')) {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    hash
}

fn ledgered_mutant_misses(ledger: &str) -> std::collections::BTreeSet<String> {
    ledger
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim_start();
            if !trimmed.starts_with("- `") {
                return None;
            }
            trimmed
                .split_once('`')
                .and_then(|(_, tail)| tail.split_once('`'))
                .map(|(entry, _)| entry.to_string())
        })
        .collect()
}

fn parse_mutation_baseline_delta(line: &str) -> Option<MutationBaselineDelta> {
    let (_, tail) = line.split_once("baseline-delta:")?;
    let mut from_count = None;
    let mut from_hash = None;
    let mut to_count = None;
    let mut to_hash = None;

    for token in tail
        .split(|ch: char| ch.is_whitespace() || matches!(ch, ',' | ';'))
        .filter(|token| !token.is_empty() && *token != "->")
    {
        let Some((key, value)) = token.split_once('=') else {
            continue;
        };
        match key {
            "from-count" => from_count = value.parse::<usize>().ok(),
            "from-fnv1a64" => from_hash = u64::from_str_radix(value, 16).ok(),
            "to-count" => to_count = value.parse::<usize>().ok(),
            "to-fnv1a64" => to_hash = u64::from_str_radix(value, 16).ok(),
            _ => {}
        }
    }

    Some(MutationBaselineDelta {
        from_count: from_count?,
        from_hash: from_hash?,
        to_count: to_count?,
        to_hash: to_hash?,
    })
}

fn mutation_baseline_change_log_records(ledger: &str, count: usize, hash: u64) -> bool {
    let deltas = ledger
        .lines()
        .filter(|line| line.contains("baseline-delta:"))
        .filter_map(parse_mutation_baseline_delta)
        .collect::<Vec<_>>();
    let Some(genesis) = deltas.first() else {
        return false;
    };
    if genesis.from_count != MUTANTS_BASELINE_GENESIS_COUNT
        || genesis.from_hash != MUTANTS_BASELINE_GENESIS_FNV1A64
    {
        return false;
    }
    if deltas.len() < 2 {
        return false;
    }
    let Some(head) = deltas.last() else {
        return false;
    };
    if head.to_count != count || head.to_hash != hash {
        return false;
    }
    deltas
        .windows(2)
        .all(|pair| pair[0].to_count == pair[1].from_count && pair[0].to_hash == pair[1].from_hash)
}

fn ticket_exists(ticket_id: &str) -> bool {
    let active = format!("tickets/{ticket_id}.md");
    let archived = format!("archive/tickets/{ticket_id}.md");
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .map(|workspace_root| {
            workspace_root.join(active).exists() || workspace_root.join(archived).exists()
        })
        .unwrap_or(false)
}

fn mutation_baseline_governance_errors(baseline: &str, ledger: &str) -> Vec<String> {
    let mut errors = Vec::new();
    let normalized = normalized_mutant_misses(baseline);
    let hash = canonical_lines_hash(&normalized);
    if normalized.len() != MUTANTS_BASELINE_NORMALIZED_COUNT {
        errors.push(format!(
            "normalized mutation baseline count changed: expected {}, got {}",
            MUTANTS_BASELINE_NORMALIZED_COUNT,
            normalized.len()
        ));
    }
    if hash != MUTANTS_BASELINE_NORMALIZED_FNV1A64 {
        errors.push(format!(
            "normalized mutation baseline hash changed: expected {MUTANTS_BASELINE_NORMALIZED_FNV1A64:016x}, got {hash:016x}"
        ));
    }
    if !mutation_baseline_change_log_records(ledger, normalized.len(), hash) {
        errors.push(format!(
            "mutation baseline lacks recorded baseline change log for normalized-count={} fnv1a64={hash:016x}",
            normalized.len()
        ));
    }

    let ledgered = ledgered_mutant_misses(ledger);
    if !normalized.is_empty() && !normalized.is_subset(&ledgered) {
        errors
            .push("non-empty mutation baseline lacks ledgered per-entry dispositions".to_string());
    }
    for miss in normalized.difference(&ledgered) {
        errors.push(format!(
            "mutation baseline miss lacks ledger disposition: {miss}"
        ));
    }
    for miss in ledgered.difference(&normalized) {
        errors.push(format!("mutation baseline ledger entry is stale: {miss}"));
    }

    let mut rationale_counts = std::collections::BTreeMap::<String, usize>::new();
    for line in ledger
        .lines()
        .filter(|line| line.trim_start().starts_with("- `"))
    {
        if !line.contains(" — ") {
            errors.push(format!(
                "mutation baseline ledger entry lacks rationale: {line}"
            ));
            continue;
        }
        let Some((_, disposition)) = line.split_once(" — ") else {
            continue;
        };
        let Some((tag, rationale)) = disposition.split_once(": ") else {
            errors.push(format!(
                "mutation baseline ledger entry lacks closed tag: {line}"
            ));
            continue;
        };
        let rationale = rationale.trim();
        if rationale.is_empty() {
            errors.push(format!(
                "mutation baseline ledger entry lacks rationale: {line}"
            ));
        }
        let normalized_rationale = mutation_ledger_rationale_suffix(rationale);
        *rationale_counts.entry(normalized_rationale).or_default() += 1;
        if tag == "justified-baseline" {
            let rationale_lower = rationale.to_ascii_lowercase();
            if rationale_lower.contains("warrants a")
                || rationale_lower.contains("future")
                || rationale_lower.contains("follow-up")
            {
                errors.push(format!(
                    "mutation baseline justified-baseline entry encodes deferred test debt: {line}"
                ));
            }
            continue;
        }
        let Some(ticket_id) = tag.strip_prefix("warrants-test:") else {
            errors.push(format!(
                "mutation baseline ledger entry uses unsupported disposition tag {tag}"
            ));
            continue;
        };
        if !ticket_exists(ticket_id) {
            errors.push(format!(
                "mutation baseline warrants-test tag references missing ticket {ticket_id}"
            ));
        }
    }
    for (rationale, count) in rationale_counts {
        if count > MUTATION_LEDGER_MAX_IDENTICAL_RATIONALES {
            errors.push(format!(
                "mutation baseline ledger repeats one rationale {count} times: {rationale}"
            ));
        }
    }

    errors
}

fn mutation_ledger_rationale_suffix(rationale: &str) -> String {
    let lower = rationale.to_ascii_lowercase();
    for marker in [
        "coverage remains assigned for ",
        "coverage remains accepted because ",
        "baseline residence remains accepted because ",
    ] {
        if let Some(index) = lower.find(marker) {
            return rationale[index + marker.len()..].trim().to_string();
        }
    }
    rationale.trim().to_string()
}

fn generative_floor_ratchet_errors(source: &str) -> Vec<String> {
    let mut errors = Vec::new();
    let expected_constants = [
        (
            "RECORDED_GENERATIVE_MASK_DIVERSITY",
            RECORDED_GENERATIVE_MASK_DIVERSITY,
        ),
        (
            "RECORDED_GENERATIVE_SEQUENCE_LENGTH_DIVERSITY",
            RECORDED_GENERATIVE_SEQUENCE_LENGTH_DIVERSITY,
        ),
    ];
    for (name, expected) in expected_constants {
        let needle = format!("const {name}: usize = {expected};");
        if !source.contains(&needle) {
            errors.push(format!(
                "generative corpus floor {name} does not match recorded value {expected}"
            ));
        }
    }
    for (flag, expected) in RECORDED_GENERATIVE_MULTI_SEED_CONTRIBUTORS {
        let needle = format!("(\"{flag}\", {expected})");
        if !source.contains(&needle) {
            errors.push(format!(
                "generative corpus contributor floor {flag} does not match recorded value {expected}"
            ));
        }
    }
    for forbidden in [
        "masks.len() >=",
        "sequence_lengths.len() >=",
        "seeds.len() >=",
    ] {
        if source.contains(forbidden) {
            errors.push(format!(
                "generative corpus floor remains one-sided: {forbidden}"
            ));
        }
    }
    for required in [
        "assert_eq!(\n        masks.len(),\n        RECORDED_GENERATIVE_MASK_DIVERSITY",
        "assert_eq!(\n        sequence_lengths.len(),\n        RECORDED_GENERATIVE_SEQUENCE_LENGTH_DIVERSITY",
        "assert_eq!(\n        actual_counts, expected_counts,",
    ] {
        if !source.contains(required) {
            errors.push(format!(
                "generative corpus floor assertion is not two-sided: {required}"
            ));
        }
    }
    errors
}

#[derive(Clone, Copy)]
struct AcceptanceChecklistAnchor {
    item: u8,
    anchors: &'static [&'static str],
}

const ACCEPTANCE_0021_CHECKLIST_ANCHORS: &[AcceptanceChecklistAnchor] = &[
    AcceptanceChecklistAnchor {
        item: 1,
        anchors: &["Rebind-After-Rejection", "ORD-HARD-066"],
    },
    AcceptanceChecklistAnchor {
        item: 2,
        anchors: &[
            "Spec §7 item 2 scheduled mutation job record",
            "ORD-HARD-102",
        ],
    },
    AcceptanceChecklistAnchor {
        item: 3,
        anchors: &[
            "Hidden-Truth Gate Rebuild And Visibility Demotion",
            "ORD-HARD-068",
        ],
    },
    AcceptanceChecklistAnchor {
        item: 4,
        anchors: &["Per-Arm Census", "ORD-HARD-069"],
    },
    AcceptanceChecklistAnchor {
        item: 5,
        anchors: &["Mutation Perimeter And Rationale Split", "ORD-HARD-072"],
    },
    AcceptanceChecklistAnchor {
        item: 6,
        anchors: &["Baseline Triage", "ORD-HARD-099"],
    },
    AcceptanceChecklistAnchor {
        item: 7,
        anchors: &["Policy Dispatch And Sleep Accessibility", "ORD-HARD-074"],
    },
    AcceptanceChecklistAnchor {
        item: 8,
        anchors: &[
            "Shared Crossing Emitter And Corrupt-History Rejection",
            "ORD-HARD-076",
        ],
    },
    AcceptanceChecklistAnchor {
        item: 9,
        anchors: &["Typed Place Concealment", "ORD-HARD-078"],
    },
    AcceptanceChecklistAnchor {
        item: 10,
        anchors: &["Event/State Perimeter", "ORD-HARD-079"],
    },
    AcceptanceChecklistAnchor {
        item: 11,
        anchors: &["Content Integrity And Contract Prose", "ORD-HARD-081"],
    },
    AcceptanceChecklistAnchor {
        item: 12,
        anchors: &["Embodied Provenance And Dead-Surface Sweep", "ORD-HARD-082"],
    },
    AcceptanceChecklistAnchor {
        item: 13,
        anchors: &["Generative Tier Deltas", "ORD-HARD-084"],
    },
    AcceptanceChecklistAnchor {
        item: 14,
        anchors: &["Low-Group Closures And Deferrals", "ORD-HARD-095"],
    },
    AcceptanceChecklistAnchor {
        item: 15,
        anchors: &["Risk Register And Conformance Index", "R-29"],
    },
    AcceptanceChecklistAnchor {
        item: 16,
        anchors: &["EMERGE-OBS Refresh", "emerge_obs_v1"],
    },
    AcceptanceChecklistAnchor {
        item: 17,
        anchors: &[
            "Explicit Non-Certification Boundary",
            "not full-project certification",
        ],
    },
];

const ACCEPTANCE_0022_CHECKLIST_ANCHORS: &[AcceptanceChecklistAnchor] = &[
    AcceptanceChecklistAnchor {
        item: 1,
        anchors: &["Real Baseline Triage", "ORD-HARD-099"],
    },
    AcceptanceChecklistAnchor {
        item: 2,
        anchors: &[
            "Concurrency And Fail-Closed Mutation Config",
            "ORD-HARD-100",
            "ORD-HARD-101",
            "ORD-HARD-102",
        ],
    },
    AcceptanceChecklistAnchor {
        item: 3,
        anchors: &["Embodied Render Split", "ORD-HARD-103", "ORD-HARD-104"],
    },
    AcceptanceChecklistAnchor {
        item: 4,
        anchors: &["Planner-Gate Adversarial Restoration", "ORD-HARD-105"],
    },
    AcceptanceChecklistAnchor {
        item: 5,
        anchors: &["Policy Behavioral Table", "ORD-HARD-106", "ORD-HARD-115"],
    },
    AcceptanceChecklistAnchor {
        item: 6,
        anchors: &["Eat Crossing Proof", "ORD-HARD-107"],
    },
    AcceptanceChecklistAnchor {
        item: 7,
        anchors: &["Scheduler Typed Errors", "ORD-HARD-108"],
    },
    AcceptanceChecklistAnchor {
        item: 8,
        anchors: &[
            "Census And Totality Guard Widening",
            "ORD-HARD-109",
            "ORD-HARD-110",
            "ORD-HARD-112",
            "ORD-HARD-113",
        ],
    },
    AcceptanceChecklistAnchor {
        item: 9,
        anchors: &[
            "Runner-Only Canonical-Day Proof",
            "ORD-HARD-111",
            "ORD-HARD-120",
        ],
    },
    AcceptanceChecklistAnchor {
        item: 10,
        anchors: &[
            "Content And Generative Hygiene",
            "ORD-HARD-116",
            "ORD-HARD-117",
            "ORD-HARD-118",
        ],
    },
    AcceptanceChecklistAnchor {
        item: 11,
        anchors: &["Meta-Lock Tier", "bijection census", "nonzero-witness"],
    },
    AcceptanceChecklistAnchor {
        item: 12,
        anchors: &[
            "0021 Report Corrections And Checklist Parity",
            "ORD-HARD-119",
            "acceptance_artifact_0022_maps_spec_section_7_items_to_report_anchors",
        ],
    },
    AcceptanceChecklistAnchor {
        item: 13,
        anchors: &["Risk Register And Conformance Index Diffs", "R-29"],
    },
    AcceptanceChecklistAnchor {
        item: 14,
        anchors: &["EMERGE-OBS Refresh", "emerge_obs_v1"],
    },
    AcceptanceChecklistAnchor {
        item: 15,
        anchors: &[
            "Explicit Non-Certification Statement",
            "not full-project certification",
            "not Phase 4 entry",
            "not `FIRST-PROOF-CERT`",
        ],
    },
];

const ACCEPTANCE_0023_CHECKLIST_ANCHORS: &[AcceptanceChecklistAnchor] = &[
    AcceptanceChecklistAnchor {
        item: 1,
        anchors: &["Embodied Locality Migration", "ORD-HARD-121"],
    },
    AcceptanceChecklistAnchor {
        item: 2,
        anchors: &[
            "Meta-Lock Registry And Witness Repair",
            "ORD-HARD-122",
            "ORD-HARD-123",
            "ORD-HARD-124",
            "ORD-HARD-129",
        ],
    },
    AcceptanceChecklistAnchor {
        item: 3,
        anchors: &["Debug Overlay Wiring", "ORD-HARD-125", "ORD-HARD-135"],
    },
    AcceptanceChecklistAnchor {
        item: 4,
        anchors: &["Policy Surface-Driven Lock", "ORD-HARD-126"],
    },
    AcceptanceChecklistAnchor {
        item: 5,
        anchors: &[
            "Scan-Evasion Closures",
            "ORD-HARD-127",
            "ORD-HARD-128",
            "ORD-HARD-130",
            "ORD-HARD-132",
            "ORD-HARD-136",
        ],
    },
    AcceptanceChecklistAnchor {
        item: 6,
        anchors: &["In-Context Witness And Panic Closure", "ORD-HARD-131"],
    },
    AcceptanceChecklistAnchor {
        item: 7,
        anchors: &[
            "Canonical Intent And Sleep Positive",
            "ORD-HARD-137",
            "ORD-HARD-138",
        ],
    },
    AcceptanceChecklistAnchor {
        item: 8,
        anchors: &[
            "Cause Disposition And Baseline Governance",
            "ORD-HARD-139",
            "ORD-HARD-134",
        ],
    },
    AcceptanceChecklistAnchor {
        item: 9,
        anchors: &[
            "0022 Evidence-Honesty Correction",
            "ORD-HARD-133",
            "acceptance_artifact_0023_maps_spec_section_7_items_to_report_anchors",
        ],
    },
    AcceptanceChecklistAnchor {
        item: 10,
        anchors: &["Premise-Held Confirmations", "ORD-HARD-121", "ORD-HARD-139"],
    },
    AcceptanceChecklistAnchor {
        item: 11,
        anchors: &[
            "Risk Register And Conformance Diffs",
            "docs/3-reference/01_DESIGN_RISK_REGISTER.md",
            "docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md",
        ],
    },
    AcceptanceChecklistAnchor {
        item: 12,
        anchors: &[
            "EMERGE-OBS Derivation And Scheduled Run Status",
            "emerge_obs_v1",
        ],
    },
    AcceptanceChecklistAnchor {
        item: 13,
        anchors: &[
            "EMERGE-OBS Derivation And Scheduled Run Status",
            "scheduled mutation still pending",
        ],
    },
    AcceptanceChecklistAnchor {
        item: 14,
        anchors: &[
            "Explicit Non-Certification Statement",
            "not full-project certification",
            "not Phase 4 entry",
            "not `FIRST-PROOF-CERT`",
        ],
    },
];

const ACCEPTANCE_0024_CHECKLIST_ANCHORS: &[AcceptanceChecklistAnchor] = &[
    AcceptanceChecklistAnchor {
        item: 1,
        anchors: &["Fixture Schema-Version Gate", "ORD-HARD-140"],
    },
    AcceptanceChecklistAnchor {
        item: 2,
        anchors: &[
            "Meta-Witness Completion",
            "ORD-HARD-141",
            "ORD-HARD-142",
            "ORD-HARD-145",
            "ORD-HARD-146",
            "ORD-HARD-155",
        ],
    },
    AcceptanceChecklistAnchor {
        item: 3,
        anchors: &["Truth-Access Removal", "ORD-HARD-143", "ORD-HARD-154"],
    },
    AcceptanceChecklistAnchor {
        item: 4,
        anchors: &["Derived Apply Perimeter", "ORD-HARD-144"],
    },
    AcceptanceChecklistAnchor {
        item: 5,
        anchors: &[
            "Content Loader Closures",
            "ORD-HARD-150",
            "ORD-HARD-151",
            "ORD-HARD-164",
            "ORD-HARD-165",
        ],
    },
    AcceptanceChecklistAnchor {
        item: 6,
        anchors: &["TUI Debug Quarantine", "ORD-HARD-152", "ORD-HARD-153"],
    },
    AcceptanceChecklistAnchor {
        item: 7,
        anchors: &[
            "Policy And Projection Closures",
            "ORD-HARD-147",
            "ORD-HARD-156",
            "ORD-HARD-148",
            "ORD-HARD-149",
        ],
    },
    AcceptanceChecklistAnchor {
        item: 8,
        anchors: &[
            "Oracle Closures",
            "ORD-HARD-157",
            "ORD-HARD-158",
            "ORD-HARD-159",
            "ORD-HARD-160",
        ],
    },
    AcceptanceChecklistAnchor {
        item: 9,
        anchors: &[
            "0005 Coherence Decisions",
            "ORD-HARD-161",
            "ORD-HARD-162",
            "ORD-HARD-163",
        ],
    },
    AcceptanceChecklistAnchor {
        item: 10,
        anchors: &["Premise-Held Confirmations", "ORD-HARD-140", "ORD-HARD-165"],
    },
    AcceptanceChecklistAnchor {
        item: 11,
        anchors: &[
            "Documentation Diffs",
            "docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md",
        ],
    },
    AcceptanceChecklistAnchor {
        item: 12,
        anchors: &["EMERGE-OBS Derivation", "emerge_obs_v1"],
    },
    AcceptanceChecklistAnchor {
        item: 13,
        anchors: &["Mutation Run Status", "scheduled mutation still pending"],
    },
    AcceptanceChecklistAnchor {
        item: 14,
        anchors: &[
            "Explicit Non-Certification Statement",
            "not full-project certification",
            "not Phase 4 entry",
            "not `FIRST-PROOF-CERT`",
        ],
    },
];

const ACCEPTANCE_0025_CHECKLIST_ANCHORS: &[AcceptanceChecklistAnchor] = &[
    AcceptanceChecklistAnchor {
        item: 1,
        anchors: &[
            "Executable Witness Discipline",
            "ORD-HARD-166",
            "ORD-HARD-175",
        ],
    },
    AcceptanceChecklistAnchor {
        item: 2,
        anchors: &["Provenance-True Kill Set", "ORD-HARD-167"],
    },
    AcceptanceChecklistAnchor {
        item: 3,
        anchors: &[
            "Envelope Fail-Closed Decisions",
            "ORD-HARD-168",
            "ORD-HARD-171",
            "ORD-HARD-184",
        ],
    },
    AcceptanceChecklistAnchor {
        item: 4,
        anchors: &[
            "Manifest Fingerprint Honesty",
            "ORD-HARD-169",
            "ORD-HARD-170",
            "ORD-HARD-183",
        ],
    },
    AcceptanceChecklistAnchor {
        item: 5,
        anchors: &[
            "Embodied Carrier Census And Staleness",
            "ORD-HARD-172",
            "ORD-HARD-173",
        ],
    },
    AcceptanceChecklistAnchor {
        item: 6,
        anchors: &[
            "TUI Gate Depth And Mode Decision",
            "ORD-HARD-174",
            "ORD-HARD-176",
            "ORD-HARD-185",
            "ORD-HARD-186",
        ],
    },
    AcceptanceChecklistAnchor {
        item: 7,
        anchors: &[
            "Census And Oracle Closures",
            "ORD-HARD-177",
            "ORD-HARD-178",
            "ORD-HARD-179",
            "ORD-HARD-180",
            "ORD-HARD-181",
            "ORD-HARD-182",
        ],
    },
    AcceptanceChecklistAnchor {
        item: 8,
        anchors: &[
            "CI And Evidence Honesty Records",
            "ORD-HARD-187",
            "ORD-HARD-188",
            "ORD-HARD-189",
            "ORD-HARD-190",
        ],
    },
    AcceptanceChecklistAnchor {
        item: 9,
        anchors: &["Premise-Held Confirmations", "ORD-HARD-166", "ORD-HARD-190"],
    },
    AcceptanceChecklistAnchor {
        item: 10,
        anchors: &[
            "Documentation Diffs",
            "docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md",
            "docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md",
            "docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md",
        ],
    },
    AcceptanceChecklistAnchor {
        item: 11,
        anchors: &["EMERGE-OBS Derivation", "emerge_obs_v1"],
    },
    AcceptanceChecklistAnchor {
        item: 12,
        anchors: &[
            "Mutation Run Status",
            "scheduled mutation still pending",
            "dated green scheduled mutation run",
        ],
    },
    AcceptanceChecklistAnchor {
        item: 13,
        anchors: &[
            "Explicit Non-Certification Statement",
            "not full-project certification",
            "not Phase 4 entry",
            "not `FIRST-PROOF-CERT`",
        ],
    },
];

fn acceptance_checklist_anchor_errors(
    report: &str,
    anchors: &[AcceptanceChecklistAnchor],
) -> Vec<String> {
    let mut errors = Vec::new();
    for row in anchors {
        for anchor in row.anchors {
            if !report.contains(anchor) {
                errors.push(format!(
                    "acceptance artifact checklist item {} lacks anchor {anchor}",
                    row.item
                ));
            }
        }
    }
    errors
}

fn acceptance_0022_baseline_narrative_errors(report: &str, count: usize) -> Vec<String> {
    let mut errors = Vec::new();
    if report.contains("All 143 remaining normalized baseline entries are ledgered") {
        errors.push("0022 report keeps stale All 143 remaining baseline prose".to_string());
    }
    let expected = format!("normalized-count {count}");
    if !report.contains(&expected) {
        errors.push(format!(
            "0022 report baseline narrative does not name pinned {expected}"
        ));
    }
    errors
}

fn split_top_level_args(args: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut paren_depth = 0_i32;
    let mut bracket_depth = 0_i32;
    let mut brace_depth = 0_i32;
    let mut in_string = false;
    let mut escaped = false;

    for ch in args.chars() {
        if in_string {
            current.push(ch);
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }

        match ch {
            '"' => {
                in_string = true;
                current.push(ch);
            }
            '(' => {
                paren_depth += 1;
                current.push(ch);
            }
            ')' => {
                paren_depth -= 1;
                current.push(ch);
            }
            '[' => {
                bracket_depth += 1;
                current.push(ch);
            }
            ']' => {
                bracket_depth -= 1;
                current.push(ch);
            }
            '{' => {
                brace_depth += 1;
                current.push(ch);
            }
            '}' => {
                brace_depth -= 1;
                current.push(ch);
            }
            ',' if paren_depth == 0 && bracket_depth == 0 && brace_depth == 0 => {
                parts.push(current.trim().to_string());
                current.clear();
            }
            _ => current.push(ch),
        }
    }

    if !current.trim().is_empty() {
        parts.push(current.trim().to_string());
    }
    parts
}

fn call_argument_lists(source: &str, marker: &str) -> Vec<Vec<String>> {
    let mut calls = Vec::new();
    let mut search_from = 0;
    while let Some(relative_start) = source[search_from..].find(marker) {
        let marker_start = search_from + relative_start;
        let Some(open_relative) = source[marker_start..].find('(') else {
            break;
        };
        let open = marker_start + open_relative;
        let mut depth = 0_i32;
        let mut in_string = false;
        let mut escaped = false;
        for (relative_index, ch) in source[open..].char_indices() {
            if in_string {
                if escaped {
                    escaped = false;
                } else if ch == '\\' {
                    escaped = true;
                } else if ch == '"' {
                    in_string = false;
                }
                continue;
            }
            match ch {
                '"' => in_string = true,
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        let close = open + relative_index;
                        calls.push(split_top_level_args(&source[open + 1..close]));
                        search_from = close + 1;
                        break;
                    }
                }
                _ => {}
            }
        }
        if search_from <= marker_start {
            break;
        }
    }
    calls
}

fn string_literal_value(expr: &str) -> Option<String> {
    let trimmed = expr.trim();
    trimmed
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .map(ToString::to_string)
}

fn string_literals_in_stable_id_loop_arrays(source: &str) -> Vec<String> {
    let mut values = Vec::new();
    let mut search_from = 0;
    while let Some(relative_start) = source[search_from..].find("for stable_id in [") {
        let start = search_from + relative_start;
        let array_start = start + "for stable_id in [".len();
        let Some(relative_end) = source[array_start..].find(']') else {
            break;
        };
        let array = &source[array_start..array_start + relative_end];
        for entry in split_top_level_args(array) {
            if let Some(value) = string_literal_value(&entry) {
                values.push(value);
            }
        }
        search_from = array_start + relative_end + 1;
    }
    values
}

fn minted_no_human_surface_fact_ids(source: &str) -> std::collections::BTreeSet<String> {
    let mut stable_ids = std::collections::BTreeSet::new();
    for marker in [
        "ActorKnownFact::observed_now",
        "ActorKnownFact::remembered_belief",
        "ActorKnownFact::routine_assignment",
        "self.push_projection_fact",
    ] {
        for args in call_argument_lists(source, marker) {
            let Some(stable_id_expr) = args.get(1) else {
                continue;
            };
            if let Some(stable_id) = string_literal_value(stable_id_expr) {
                stable_ids.insert(stable_id);
            } else if stable_id_expr == "stable_id" {
                stable_ids.extend(string_literals_in_stable_id_loop_arrays(source));
            }
        }
    }
    stable_ids
}

fn assert_absent_from_sources(sources: &[(String, String)], needle: &str) {
    for (path, source) in sources {
        assert!(
            !source.contains(needle),
            "{path}: forbidden shortcut reintroduced: {needle}"
        );
    }
}

fn truth_accessor_is_allowlisted(path: &str, token: &str) -> bool {
    TRUTH_ACCESSOR_ALLOWLIST.iter().any(|entry| {
        entry.path == path
            && entry.token == token
            && !entry.rationale.is_empty()
            && !entry.responsible_layer.is_empty()
    })
}

fn cognition_input_truth_accessor_violations_from_sources(
    sources: &[(String, String)],
) -> Vec<String> {
    let mut violations = Vec::new();
    for (path, source) in sources {
        for token in COGNITION_TRUTH_ACCESSOR_TOKENS {
            if source.contains(token) && !truth_accessor_is_allowlisted(path, token) {
                violations.push(format!("{path}: unallowlisted truth accessor {token}"));
            }
        }
    }
    violations
}

#[test]
fn cognition_inputs_are_context_backed() {
    let violations = cognition_input_truth_accessor_violations_from_sources(&production_sources());
    assert!(
        violations.is_empty(),
        "truth accessors must be workspace-wide allowlisted outside actor-known context derivation: {violations:#?}"
    );

    let synthetic_sources = vec![(
        "crates/tracewake-core/src/view_models.rs".to_string(),
        "fn leak(state: &PhysicalState) { let _ = state.workplaces.get(&id); }".to_string(),
    )];
    let synthetic = cognition_input_truth_accessor_violations_from_sources(&synthetic_sources);
    assert!(
        synthetic
            .iter()
            .any(|violation| violation.contains("state.workplaces")),
        "synthetic exempt-file truth accessor must fail the workspace-wide guard"
    );
}

#[test]
fn guarded_layer_source_census_matches_module_tree() {
    let actual = guarded_layer_source_paths();
    let expected = WORKSPACE_SOURCE_CLASSIFICATIONS
        .iter()
        .filter_map(|entry| {
            matches!(entry.class, WorkspaceSourceClass::GuardedLayer)
                .then_some(entry.path.to_string())
        })
        .collect::<Vec<_>>();

    assert_eq!(
        actual, expected,
        "new guarded-layer files must be classified in the workspace guard census"
    );
}

#[test]
fn mutation_perimeter_matches_duration_action_rationale_and_ci_filters() {
    let violations = mutation_perimeter_consistency_violations(MUTANTS_TOML, CI_YML);
    assert!(
        violations.is_empty(),
        "mutation perimeter, CI filters, and action-source rationales diverged: {violations:#?}"
    );

    let missing_events_glob =
        MUTANTS_TOML.replace("  \"crates/tracewake-core/src/events/**\",\n", "");
    assert!(
        mutation_perimeter_consistency_violations(&missing_events_glob, CI_YML)
            .iter()
            .any(|violation| violation.contains("events/**")),
        "synthetic missing standing examine glob must fail the perimeter guard"
    );

    let excluded_eat = MUTANTS_TOML.replace(
        "examine_globs = [",
        "exclude_globs = [\n  \"crates/tracewake-core/src/actions/defs/eat.rs\",\n]\n\nexamine_globs = [",
    );
    assert!(
        mutation_perimeter_consistency_violations(&excluded_eat, CI_YML)
            .iter()
            .any(|violation| violation.contains("exclude_globs") && violation.contains("eat.rs")),
        "synthetic specific perimeter-path exclusion must fail the guard"
    );

    let unsupported_key = format!("{MUTANTS_TOML}\nexclude_re = [\".*\"]\n");
    assert!(
        mutation_perimeter_consistency_violations(&unsupported_key, CI_YML)
            .iter()
            .any(|violation| violation.contains("unsupported key exclude_re")),
        "synthetic unsupported cargo-mutants config key must fail the guard"
    );

    let unsupported_glob = MUTANTS_TOML.replace(
        "  \"crates/tracewake-tui/src/transcript.rs\",",
        "  \"**/transcript.rs\",",
    );
    assert!(
        mutation_perimeter_consistency_violations(&unsupported_glob, CI_YML)
            .iter()
            .any(|violation| violation.contains("unsupported glob shape")),
        "synthetic unsupported examine glob must fail closed"
    );

    let in_diff_mutants_invocation = if CI_YML.contains(
        "cargo mutants --in-diff \"$RUNNER_TEMP/guarded.diff\" --no-shuffle --timeout 183",
    ) {
        "cargo mutants --in-diff \"$RUNNER_TEMP/guarded.diff\" --no-shuffle --timeout 183"
    } else {
        "cargo mutants --in-diff \"$RUNNER_TEMP/guarded.diff\" --no-shuffle"
    };
    let swallowed_failure = CI_YML.replace(
        in_diff_mutants_invocation,
        &format!("{in_diff_mutants_invocation} || echo ok"),
    );
    assert!(
        mutation_perimeter_consistency_violations(MUTANTS_TOML, &swallowed_failure)
            .iter()
            .any(|violation| violation.contains("shell suffix")),
        "synthetic swallowed cargo-mutants failure must fail the perimeter guard"
    );

    let multiline_swallowed_failure = CI_YML.replace(
        "          cargo mutants --workspace --no-shuffle",
        "          cargo mutants --workspace --no-shuffle || echo ok",
    );
    assert!(
        mutation_perimeter_consistency_violations(MUTANTS_TOML, &multiline_swallowed_failure)
            .iter()
            .any(|violation| violation.contains("shell suffix")),
        "synthetic_multiline_mutants_swallow_suffix must fail the logical-line perimeter guard"
    );

    let comment_only_capture = CI_YML.replace(
        "          mutants_status=$?\n          set -e\n",
        "          # mutants_status=$?\n          set -e\n",
    );
    assert!(
        mutation_perimeter_consistency_violations(MUTANTS_TOML, &comment_only_capture)
            .iter()
            .any(|violation| violation.contains("same step block")),
        "synthetic comment-only status capture must fail the perimeter guard"
    );

    let missing_timeout_status = CI_YML.replace(" && [ \"$mutants_status\" -ne 3 ]", "");
    assert!(
        mutation_perimeter_consistency_violations(MUTANTS_TOML, &missing_timeout_status)
            .iter()
            .any(|violation| violation.contains("misses/timeouts")),
        "synthetic timeout status removal must fail the perimeter guard"
    );

    let missing_timeout_artifact =
        CI_YML.replace("mutants.out/timeout.txt", "mutants.out/timeout-removed.txt");
    assert!(
        mutation_perimeter_consistency_violations(MUTANTS_TOML, &missing_timeout_artifact)
            .iter()
            .any(|violation| violation.contains("timeout.txt")),
        "synthetic timeout artifact removal must fail the perimeter guard"
    );

    let missing_scheduled_capture =
        CI_YML.replace("          supervisor_status=$?\n          set -e\n", "");
    assert!(
        mutation_perimeter_consistency_violations(MUTANTS_TOML, &missing_scheduled_capture)
            .iter()
            .any(|violation| violation.contains("status")),
        "synthetic scheduled status-capture removal must fail the perimeter guard"
    );

    let missing_event_name_concurrency = CI_YML.replace(
        "ci-${{ github.workflow }}-${{ github.ref }}-${{ github.event_name }}",
        "ci-${{ github.workflow }}-${{ github.ref }}",
    );
    assert!(
        mutation_perimeter_consistency_violations(MUTANTS_TOML, &missing_event_name_concurrency)
            .iter()
            .any(|violation| violation.contains("github.event_name")),
        "synthetic concurrency group event-name removal must fail the perimeter guard"
    );

    let missing_push = CI_YML.replace(
        "github.event_name == 'pull_request' || github.event_name == 'push'",
        "github.event_name == 'pull_request'",
    );
    assert!(
        mutation_perimeter_consistency_violations(MUTANTS_TOML, &missing_push)
            .iter()
            .any(|violation| violation.contains("direct pushes")),
        "synthetic push-gap regression must fail the perimeter guard"
    );

    let divergent_scheduled_filter = CI_YML.replace(
        "          cargo mutants --workspace --no-shuffle",
        "          cargo mutants --workspace -f 'crates/tracewake-core/src/actions/defs/eat.rs' --no-shuffle",
    );
    assert!(
        mutation_perimeter_consistency_violations(MUTANTS_TOML, &divergent_scheduled_filter)
            .iter()
            .any(|violation| violation.contains("divergent file perimeter")),
        "synthetic scheduled filter reintroduction must fail"
    );

    let missing_eat_in_diff = CI_YML.replace(
        "actions/defs/(need_events|eat|sleep|work|wait|continue_routine|movement|checkcontainer)\\.rs",
        "actions/defs/(need_events|sleep|work|wait|continue_routine|movement|checkcontainer)\\.rs",
    );
    assert!(
        mutation_perimeter_consistency_violations(MUTANTS_TOML, &missing_eat_in_diff)
            .iter()
            .any(|violation| violation.contains("actions/defs/eat.rs")),
        "synthetic in-diff regex removal must fail for eat.rs"
    );

    let decoy_in_diff = CI_YML.replace(
        "          if git diff --name-only \"$guarded_range\" | grep -E",
        "          echo \"git diff --name-only decoy\" | grep -E '^(crates/tracewake-core/src/actions/defs/(need_events|eat|sleep|work|wait|continue_routine|movement|checkcontainer)\\.rs)'\n          if git diff --name-only \"$guarded_range\" | grep -E",
    );
    assert!(
        mutation_perimeter_consistency_violations(MUTANTS_TOML, &decoy_in_diff)
            .iter()
            .any(|violation| violation.contains("exactly one git diff --name-only")),
        "synthetic uncommented in-diff filter decoy must fail the perimeter guard"
    );

    let narrowed_agent_in_diff = CI_YML.replace(
        "crates/tracewake-core/src/agent/",
        "crates/tracewake-core/src/agent/planning/",
    );
    assert!(
        mutation_perimeter_consistency_violations(MUTANTS_TOML, &narrowed_agent_in_diff)
            .iter()
            .any(|violation| violation.contains("agent/transaction.rs")),
        "synthetic narrowed in-diff agent regex must fail the canary"
    );

    let false_mutation_rationale_sources = vec![WorkspaceSourceClassification {
        path: "crates/tracewake-core/src/actions/defs/wait.rs",
        class: WorkspaceSourceClass::Exempt {
            rationale: CORE_ACTION_MUTATION_PERIMETER_RATIONALE,
        },
    }];
    assert!(
        mutation_rationale_violations(&false_mutation_rationale_sources)
            .iter()
            .any(|violation| violation.contains("wait.rs")),
        "synthetic non-perimeter mutation rationale must fail"
    );
}

#[test]
fn need_accounting_duration_body_start_is_exclusive_and_terminal_is_inclusive() {
    let actor = ord_life_actor("actor_duration_subject");
    let mut log = EventLog::new();
    let sleep_start = ord_life_duration_event(
        EventKind::SleepStarted,
        &actor,
        "event_duration_subject_sleep_started",
        12,
        vec![EventCause::Proposal(
            ProposalId::new("proposal_duration_subject_sleep").unwrap(),
        )],
    );
    let sleep_done = ord_life_duration_event(
        EventKind::SleepCompleted,
        &actor,
        "event_duration_subject_sleep_completed",
        15,
        vec![EventCause::Event(sleep_start.event_id.clone())],
    );
    log.append(sleep_start).unwrap();
    log.append(sleep_done).unwrap();

    let counts = classify_actor_tick_regimes(&log, &actor, SimTick::new(10), SimTick::new(15));

    assert_eq!(
        counts,
        TickRegimeCounts {
            awake_ticks: 2,
            asleep_ticks: 3,
            working_ticks: 0,
        },
        "ticks 11 and 12 stay awake; only body ticks 13..=15 are asleep"
    );

    let boundary_control =
        classify_actor_tick_regimes(&log, &actor, SimTick::new(12), SimTick::new(15));
    assert_eq!(
        boundary_control,
        TickRegimeCounts {
            awake_ticks: 0,
            asleep_ticks: 3,
            working_ticks: 0,
        },
        "starting the window at the duration start still charges body ticks 13..=15"
    );
}

#[test]
fn need_accounting_current_start_requires_subject_actor_and_absent_log_identity() {
    let subject = ord_life_actor("actor_duration_subject");
    let other = ord_life_actor("actor_duration_other");
    let other_current = ord_life_duration_event(
        EventKind::WorkBlockStarted,
        &other,
        "event_duration_other_work_started",
        10,
        vec![EventCause::Proposal(
            ProposalId::new("proposal_duration_other_work").unwrap(),
        )],
    );
    let empty_log = EventLog::new();

    let subject_counts = classify_actor_tick_regimes_with_start(
        &empty_log,
        &subject,
        SimTick::new(10),
        SimTick::new(12),
        Some(&other_current),
    );
    let other_counts = classify_actor_tick_regimes_with_start(
        &empty_log,
        &other,
        SimTick::new(10),
        SimTick::new(12),
        Some(&other_current),
    );

    assert_eq!(
        subject_counts,
        TickRegimeCounts {
            awake_ticks: 2,
            asleep_ticks: 0,
            working_ticks: 0,
        },
        "another actor's current duration start must not charge the subject"
    );
    assert_eq!(
        other_counts,
        TickRegimeCounts {
            awake_ticks: 0,
            asleep_ticks: 0,
            working_ticks: 2,
        },
        "the same current start remains non-vacuous for its owning actor"
    );

    let mut logged = EventLog::new();
    let logged_start = ord_life_duration_event(
        EventKind::WorkBlockStarted,
        &subject,
        "event_duration_subject_work_started",
        10,
        vec![EventCause::Proposal(
            ProposalId::new("proposal_duration_subject_work").unwrap(),
        )],
    );
    let duplicate_current = logged_start.clone();
    let logged_terminal = ord_life_duration_event(
        EventKind::WorkBlockCompleted,
        &subject,
        "event_duration_subject_work_completed",
        12,
        vec![EventCause::Event(logged_start.event_id.clone())],
    );
    logged.append(logged_start).unwrap();
    logged.append(logged_terminal).unwrap();

    let counts_with_logged_current = classify_actor_tick_regimes_with_start(
        &logged,
        &subject,
        SimTick::new(10),
        SimTick::new(14),
        Some(&duplicate_current),
    );
    assert_eq!(
        counts_with_logged_current,
        TickRegimeCounts {
            awake_ticks: 2,
            asleep_ticks: 0,
            working_ticks: 2,
        },
        "a current start already present in the log must not create a second open owner"
    );
}

#[test]
fn need_accounting_current_start_identity_ignores_unrelated_log_events() {
    let subject = ord_life_actor("actor_duration_subject");
    let mut log = EventLog::new();
    log.append(ord_life_duration_event(
        EventKind::ActorWaited,
        &subject,
        "event_duration_subject_unrelated_wait",
        9,
        vec![EventCause::Proposal(
            ProposalId::new("proposal_duration_subject_wait").unwrap(),
        )],
    ))
    .unwrap();
    let current_start = ord_life_duration_event(
        EventKind::WorkBlockStarted,
        &subject,
        "event_duration_subject_current_work_started",
        10,
        vec![EventCause::Proposal(
            ProposalId::new("proposal_duration_subject_current_work").unwrap(),
        )],
    );

    let counts = classify_actor_tick_regimes_with_start(
        &log,
        &subject,
        SimTick::new(10),
        SimTick::new(12),
        Some(&current_start),
    );

    assert_eq!(
        counts,
        TickRegimeCounts {
            awake_ticks: 0,
            asleep_ticks: 0,
            working_ticks: 2,
        },
        "unrelated log events must not suppress a legitimate absent current start"
    );
}

fn ord_life_actor(id: &str) -> ActorId {
    ActorId::new(id).unwrap()
}

fn ord_life_duration_event(
    kind: EventKind,
    actor: &ActorId,
    id: &str,
    tick: u64,
    causes: Vec<EventCause>,
) -> EventEnvelope {
    let mut event = EventEnvelope::new_caused_v1(
        EventId::new(id).unwrap(),
        kind,
        0,
        0,
        SimTick::new(tick),
        ord_life_ordering_key(tick, actor, kind.stable_id()),
        ContentManifestId::new("phase3a_manifest").unwrap(),
        causes,
    )
    .unwrap();
    event.actor_id = Some(actor.clone());
    event.payload = vec![PayloadField::new("schema_version", EVENT_SCHEMA_V1)];
    event
}

fn ord_life_ordering_key(tick: u64, actor: &ActorId, action_id: &str) -> OrderingKey {
    OrderingKey::new(
        SimTick::new(tick),
        SchedulePhase::NoHumanProcess,
        SchedulerSourceId::Actor(actor.clone()),
        ProposalSequence::new(0),
        ActionId::new(action_id).unwrap(),
        Vec::new(),
        action_id,
    )
}

#[test]
fn mutation_baseline_misses_are_pinned_and_ledgered() {
    let errors =
        mutation_baseline_governance_errors(MUTANTS_BASELINE_MISSES, MUTANTS_BASELINE_LEDGER);
    assert!(
        errors.is_empty(),
        "mutation baseline count/hash/ledger governance failed: {errors:#?}"
    );

    let appended = format!(
        "{MUTANTS_BASELINE_MISSES}\ncrates/tracewake-core/src/actions/defs/eat.rs:1:1: replace build_eat_events -> Vec<EventEnvelope> with vec![]\n"
    );
    let synthetic_errors = mutation_baseline_governance_errors(&appended, MUTANTS_BASELINE_LEDGER);
    assert!(
        synthetic_errors
            .iter()
            .any(|error| error.contains("count changed"))
            && synthetic_errors
                .iter()
                .any(|error| error.contains("hash changed"))
            && synthetic_errors
                .iter()
                .any(|error| error.contains("lacks ledger disposition"))
            && synthetic_errors
                .iter()
                .any(|error| error.contains("non-empty mutation baseline"))
            && synthetic_errors
                .iter()
                .any(|error| error.contains("recorded baseline change log")),
        "synthetic unledgered baseline append must fail count, hash, ledger, and change-log checks"
    );

    let synthetic_unledgered_non_empty_mutation_baseline =
        "synthetic/unledgered.rs:1:1: replace embodied_lock() -> bool with false";
    assert!(
        mutation_baseline_governance_errors(
            synthetic_unledgered_non_empty_mutation_baseline,
            MUTANTS_BASELINE_LEDGER
        )
        .iter()
        .any(|error| error
            .contains("non-empty mutation baseline lacks ledgered per-entry dispositions")),
        "synthetic_unledgered_non_empty_mutation_baseline must fail per-entry baseline governance"
    );

    if !MUTANTS_BASELINE_MISSES.trim().is_empty() {
        let shrink = MUTANTS_BASELINE_MISSES
            .lines()
            .skip(1)
            .collect::<Vec<_>>()
            .join("\n");
        let shrink_errors = mutation_baseline_governance_errors(&shrink, MUTANTS_BASELINE_LEDGER);
        assert!(
            shrink_errors
                .iter()
                .any(|error| error.contains("count changed"))
                && shrink_errors
                    .iter()
                    .any(|error| error.contains("hash changed"))
                && shrink_errors
                    .iter()
                    .any(|error| error.contains("recorded baseline change log")),
            "synthetic unrecorded baseline shrink must fail count, hash, and change-log checks"
        );
    } else {
        assert!(
            MUTANTS_BASELINE_MISSES.trim().is_empty(),
            "zero pinned mutation baseline must have no accepted misses"
        );
    }

    let unrecorded_floor_raise_ledger = MUTANTS_BASELINE_LEDGER.replace(
        "to-count=0 to-fnv1a64=cbf29ce484222325",
        "to-count=1 to-fnv1a64=cbf29ce484222325",
    );
    assert!(
        mutation_baseline_governance_errors(
            MUTANTS_BASELINE_MISSES,
            &unrecorded_floor_raise_ledger
        )
        .iter()
        .any(|error| error.contains("recorded baseline change log")),
        "synthetic unrecorded baseline floor raise must fail change-log governance"
    );

    let missing_predecessor_ledger = MUTANTS_BASELINE_LEDGER.replace(
        "from-count=137 from-fnv1a64=977cce46b241e47b -> to-count=130",
        "from-count=138 from-fnv1a64=977cce46b241e47b -> to-count=130",
    );
    assert!(
        mutation_baseline_governance_errors(MUTANTS_BASELINE_MISSES, &missing_predecessor_ledger)
            .iter()
            .any(|error| error.contains("recorded baseline change log")),
        "synthetic missing-predecessor shrink must fail change-log governance"
    );

    let fabricated_single_link = format!(
        "- synthetic — baseline-delta: from-count={MUTANTS_BASELINE_NORMALIZED_COUNT} from-fnv1a64={MUTANTS_BASELINE_NORMALIZED_FNV1A64:016x} -> to-count={MUTANTS_BASELINE_NORMALIZED_COUNT} to-fnv1a64={MUTANTS_BASELINE_NORMALIZED_FNV1A64:016x}; fabricated head-only ledger."
    );
    assert!(
        mutation_baseline_governance_errors(MUTANTS_BASELINE_MISSES, &fabricated_single_link)
            .iter()
            .any(|error| error.contains("recorded baseline change log")),
        "synthetic fabricated single-link ledger must fail the genesis anchor"
    );

    let shortened_chain = MUTANTS_BASELINE_LEDGER
        .lines()
        .filter(|line| !line.contains("0022PHA3ABASTRI-015"))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        mutation_baseline_governance_errors(MUTANTS_BASELINE_MISSES, &shortened_chain)
            .iter()
            .any(|error| error.contains("recorded baseline change log")),
        "synthetic shortened baseline chain must fail adjacency"
    );

    let bulk_ledger = (0..=MUTATION_LEDGER_MAX_IDENTICAL_RATIONALES)
        .map(|index| {
            format!(
                "- `synthetic/path_{index}.rs: replace x -> y` — justified-baseline: module {index} coverage remains accepted because repeated suffix"
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let synthetic_baseline = (0..=MUTATION_LEDGER_MAX_IDENTICAL_RATIONALES)
        .map(|index| format!("synthetic/path_{index}.rs:1:1: replace x -> y"))
        .collect::<Vec<_>>()
        .join("\n");
    let bulk_errors = mutation_baseline_governance_errors(&synthetic_baseline, &bulk_ledger);
    assert!(
        bulk_errors
            .iter()
            .any(|error| error.contains("repeats one rationale")),
        "synthetic bulk-accepted ledger rationale suffix must fail governance"
    );

    let deferred_ledger = format!(
        "{MUTANTS_BASELINE_LEDGER}\n- `synthetic/path.rs: replace x -> y` — justified-baseline: this warrants a future focused assertion"
    );
    assert!(
        mutation_baseline_governance_errors(
            "synthetic/path.rs:1:1: replace x -> y",
            &deferred_ledger
        )
        .iter()
        .any(|error| error.contains("deferred test debt")),
        "synthetic justified-baseline deferral language must fail governance"
    );

    let bad_tag_ledger = format!(
        "{MUTANTS_BASELINE_LEDGER}\n- `synthetic/path.rs: replace x -> y` — warrants-test:0022PHA3ABASTRI-999: synthetic missing ticket"
    );
    assert!(
        mutation_baseline_governance_errors(
            "synthetic/path.rs:1:1: replace x -> y",
            &bad_tag_ledger
        )
        .iter()
        .any(|error| error.contains("missing ticket")),
        "synthetic warrants-test tag with missing ticket must fail governance"
    );
}

#[test]
fn meta_lock_registry_covers_structural_locks_and_negatives() {
    let errors = meta_lock_registry_errors(META_LOCK_REGISTRY, ANTI_REGRESSION_GUARDS_RS);
    assert!(
        errors.is_empty(),
        "meta-lock registry census failed:\n{}",
        errors.join("\n")
    );

    let mut missing_negative = META_LOCK_REGISTRY.to_vec();
    missing_negative[0].negative_id = "";
    assert!(
        meta_lock_registry_errors(&missing_negative, ANTI_REGRESSION_GUARDS_RS)
            .iter()
            .any(|error| error.contains("lacks a registered negative")),
        "synthetic lock without negative must fail the meta-lock census"
    );

    let mut non_routed_negative = META_LOCK_REGISTRY.to_vec();
    non_routed_negative[0].routing = MetaLockRouting::TicketOwnedDebt {
        ticket: "0022PHA3ABASTRI-999",
    };
    assert!(
        meta_lock_registry_errors(&non_routed_negative, ANTI_REGRESSION_GUARDS_RS)
            .iter()
            .any(|error| error.contains("missing ticket")),
        "synthetic non-routed negative must fail the meta-lock census"
    );

    let scan_shaped_exemption_source = format!(
        "{ANTI_REGRESSION_GUARDS_RS}\n#[test]\nfn synthetic_scan_shaped_exemption() {{ let _ = synthetic_errors(); }}\n"
    );
    let scan_shaped_exemptions = BTreeSet::from(["synthetic_scan_shaped_exemption"]);
    assert!(
        meta_lock_census_exemption_errors(
            "synthetic_scan_shaped_exemption",
            "synthetic rationale without covering lock",
            &scan_shaped_exemptions,
            &scan_shaped_exemption_source,
        )
        .iter()
        .any(|error| error.contains("covering lock_id")),
        "synthetic scan-shaped exemption without covering lock_id must fail"
    );

    let inline_scan_exemption_source = format!(
        "{ANTI_REGRESSION_GUARDS_RS}\n#[test]\nfn synthetic_inline_scan_exemption() {{ let violations = sources.iter().filter(|line| line.contains(\"bad\")).count(); assert_eq!(violations, 0); }}\n"
    );
    assert!(
        meta_lock_census_exemption_errors(
            "synthetic_inline_scan_exemption",
            "synthetic inline-scan rationale without covering lock",
            &BTreeSet::from(["synthetic_inline_scan_exemption"]),
            &inline_scan_exemption_source,
        )
        .iter()
        .any(|error| error.contains("covering lock_id")),
        "synthetic inline-scan exemption without covering lock_id must fail the broadened detector"
    );

    assert!(
        meta_lock_census_exemption_errors(
            "synthetic_inline_scan_exemption",
            "Historical acceptance-artifact anchor audit; not a standing structural lock family.",
            &BTreeSet::from(["synthetic_inline_scan_exemption"]),
            &inline_scan_exemption_source,
        )
        .iter()
        .any(|error| error.contains("covering lock_id")),
        "historical-audit rationale text must not bypass covering-lock validation"
    );

    let anchorless_source = ANTI_REGRESSION_GUARDS_RS.replace(
        "fn apply_event_with_capability",
        "fn stale_apply_event_with_capability",
    );
    assert!(
        meta_lock_registry_errors(META_LOCK_REGISTRY, &anchorless_source)
            .iter()
            .any(|error| error.contains("below minimum")),
        "synthetic anchor-miss scan must fail the live nonzero-witness rule"
    );

    let witness_kind_entry = META_LOCK_REGISTRY
        .iter()
        .find(|entry| {
            entry.lock_id == "guard_018_witness_kind_no_human_fact_stable_ids_have_explicit_arms"
        })
        .unwrap();
    let vacuated_transaction = production(TRANSACTION_RS)
        .replace("fn witness_kind_allowed", "fn stale_witness_kind_allowed");
    assert!(
        witness_kind_arm_inspected_sites(&vacuated_transaction) < witness_kind_entry.witness_min,
        "present-but-vacuous behavior-assertion witness must drop below minimum when its inspected population empties"
    );
    let live_witness_body =
        function_body_if_present(ANTI_REGRESSION_GUARDS_RS, "fn meta_lock_live_witness_count")
            .unwrap();
    let behavior_witness_body = function_body_if_present(
        ANTI_REGRESSION_GUARDS_RS,
        "fn behavior_assertion_inspected_site_count",
    )
    .unwrap();
    for (witness_fn, witness_body) in [
        ("meta_lock_live_witness_count", live_witness_body),
        (
            "behavior_assertion_inspected_site_count",
            behavior_witness_body,
        ),
    ] {
        let ban_errors = witness_shape_ban_errors(witness_body);
        assert!(
            ban_errors.is_empty(),
            "{witness_fn} carries a banned witness shape: {ban_errors:?}"
        );
    }
    assert!(
        !witness_shape_ban_errors("let count = body.matches(\"assert!\").count();").is_empty(),
        "synthetic assertion-token-count witness shape must trip the ban"
    );

    let oblique_entry = META_LOCK_REGISTRY
        .iter()
        .find(|entry| entry.lock_id == "typed_column_closure_oblique_payload_helper_calls")
        .unwrap();
    let oblique_without_inspected_site = EVENTS_APPLY_RS.replace("payload", "typed_payload");
    assert!(
        typed_column_closure_oblique_inspected_site_count(
            &oblique_without_inspected_site,
            TYPED_COLUMN_CLOSURE_EXEMPTIONS,
        ) < oblique_entry.witness_min,
        "synthetic oblique helper site removal must fail the repaired live witness"
    );

    let runner_entry = META_LOCK_REGISTRY
        .iter()
        .find(|entry| entry.lock_id == "guard_011_no_human_day_runner_only_evidence")
        .unwrap();
    let runner_without_inspected_sites = CONTENT_GOLDEN_FIXTURES_RUN_RS
        .replace("has_no_human_event_for_actor", "has_event_for_actor")
        .replace("EventKind::EatFailed", "EventKind::EatStarted")
        .replace("\"actor_mara\"", "\"actor_tomas\"");
    assert!(
        no_human_day_runner_only_inspected_site_count(&runner_without_inspected_sites)
            < runner_entry.witness_min,
        "synthetic runner-only inspected-site removal must fail the repaired live witness"
    );

    let other_emission_entry = META_LOCK_REGISTRY
        .iter()
        .find(|entry| entry.lock_id == "guard_014_perception_visibility_other_emission_paths")
        .unwrap();
    let comment_only_perception = PERCEPTION_RS
        .lines()
        .map(|line| format!("// {line}"))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        perception_prose_scan_inspected_line_count(&comment_only_perception)
            < other_emission_entry.witness_min,
        "synthetic inspected-line removal must fail the repaired perception witnesses while anchors stay textually present"
    );

    let unregistered_guard_source = format!(
        "{ANTI_REGRESSION_GUARDS_RS}\n#[test]\nfn synthetic_unprefixed_structural_lock() {{}}\n"
    );
    assert!(
        meta_lock_registry_errors(META_LOCK_REGISTRY, &unregistered_guard_source)
            .iter()
            .any(|error| error.contains("synthetic_unprefixed_structural_lock")),
        "synthetic unprefixed structural lock must fail the meta-lock census"
    );
}

#[test]
fn generative_lock_source_uses_two_sided_recorded_floors() {
    let errors = generative_floor_ratchet_errors(GENERATIVE_LOCK_RS);
    assert!(
        errors.is_empty(),
        "generative floor ratchet failed: {errors:#?}"
    );

    let raised_floor = GENERATIVE_LOCK_RS.replace(
        "const RECORDED_GENERATIVE_MASK_DIVERSITY: usize = 7;",
        "const RECORDED_GENERATIVE_MASK_DIVERSITY: usize = 8;",
    );
    assert!(
        generative_floor_ratchet_errors(&raised_floor)
            .iter()
            .any(|error| error.contains("RECORDED_GENERATIVE_MASK_DIVERSITY")),
        "synthetic unrecorded generative floor raise must fail"
    );

    let one_sided_floor = GENERATIVE_LOCK_RS.replace("masks.len(),", "masks.len() >= 4,");
    assert!(
        generative_floor_ratchet_errors(&one_sided_floor)
            .iter()
            .any(|error| error.contains("one-sided")),
        "synthetic one-sided generative floor must fail"
    );
}

#[test]
fn acceptance_artifact_0021_maps_spec_section_7_items_to_report_anchors() {
    let errors = acceptance_checklist_anchor_errors(
        ACCEPTANCE_0021_REPORT,
        ACCEPTANCE_0021_CHECKLIST_ANCHORS,
    );
    assert!(
        errors.is_empty(),
        "0021 acceptance artifact checklist anchors are missing: {errors:#?}"
    );

    let mut synthetic = ACCEPTANCE_0021_CHECKLIST_ANCHORS.to_vec();
    synthetic.push(AcceptanceChecklistAnchor {
        item: 99,
        anchors: &["synthetic missing acceptance artifact anchor"],
    });
    let synthetic_errors = acceptance_checklist_anchor_errors(ACCEPTANCE_0021_REPORT, &synthetic);
    assert!(
        synthetic_errors
            .iter()
            .any(|error| error.contains("item 99")),
        "synthetic missing acceptance checklist anchor must fail through the real checker"
    );
}

#[test]
fn acceptance_artifact_0022_maps_spec_section_7_items_to_report_anchors() {
    let mut errors = acceptance_checklist_anchor_errors(
        ACCEPTANCE_0022_REPORT,
        ACCEPTANCE_0022_CHECKLIST_ANCHORS,
    );
    errors.extend(acceptance_0022_baseline_narrative_errors(
        ACCEPTANCE_0022_REPORT,
        MUTANTS_BASELINE_NORMALIZED_COUNT,
    ));
    assert!(
        errors.is_empty(),
        "0022 acceptance artifact checklist anchors are missing: {errors:#?}"
    );

    let mut synthetic = ACCEPTANCE_0022_CHECKLIST_ANCHORS.to_vec();
    synthetic.push(AcceptanceChecklistAnchor {
        item: 99,
        anchors: &["synthetic missing 0022 acceptance artifact anchor"],
    });
    let synthetic_errors = acceptance_checklist_anchor_errors(ACCEPTANCE_0022_REPORT, &synthetic);
    assert!(
        synthetic_errors
            .iter()
            .any(|error| error.contains("item 99")),
        "synthetic missing 0022 acceptance checklist anchor must fail through the real checker"
    );

    let stale_baseline_report = ACCEPTANCE_0022_REPORT.replace(
        "remaining normalized baseline was retired from 143 to 0 through focused follow-up",
        "All 143 remaining normalized baseline entries are ledgered with closed disposition tags. No entry was retired by focused tests in this slice; instead, test debt was filed to the follow-up",
    );
    assert!(
        acceptance_0022_baseline_narrative_errors(
            &stale_baseline_report,
            MUTANTS_BASELINE_NORMALIZED_COUNT
        )
        .iter()
        .any(|error| error.contains("All 143 remaining")),
        "synthetic stale baseline narrative must fail the 0022 acceptance artifact guard"
    );
}

#[test]
fn acceptance_artifact_0023_maps_spec_section_7_items_to_report_anchors() {
    let errors = acceptance_checklist_anchor_errors(
        ACCEPTANCE_0023_REPORT,
        ACCEPTANCE_0023_CHECKLIST_ANCHORS,
    );
    assert!(
        errors.is_empty(),
        "0023 acceptance artifact checklist anchors are missing: {errors:#?}"
    );

    let mut synthetic = ACCEPTANCE_0023_CHECKLIST_ANCHORS.to_vec();
    synthetic.push(AcceptanceChecklistAnchor {
        item: 99,
        anchors: &["synthetic_0023_missing_acceptance_anchor"],
    });
    let synthetic_errors = acceptance_checklist_anchor_errors(ACCEPTANCE_0023_REPORT, &synthetic);
    assert!(
        synthetic_errors
            .iter()
            .any(|error| error.contains("item 99")),
        "synthetic_0023_missing_acceptance_anchor must fail through the real checker"
    );
}

#[test]
fn acceptance_artifact_0024_maps_spec_section_7_items_to_report_anchors() {
    let errors = acceptance_checklist_anchor_errors(
        ACCEPTANCE_0024_REPORT,
        ACCEPTANCE_0024_CHECKLIST_ANCHORS,
    );
    assert!(
        errors.is_empty(),
        "0024 acceptance artifact checklist anchors are missing: {errors:#?}"
    );

    let mut synthetic = ACCEPTANCE_0024_CHECKLIST_ANCHORS.to_vec();
    synthetic.push(AcceptanceChecklistAnchor {
        item: 99,
        anchors: &["synthetic_0024_missing_acceptance_anchor"],
    });
    let synthetic_errors = acceptance_checklist_anchor_errors(ACCEPTANCE_0024_REPORT, &synthetic);
    assert!(
        synthetic_errors
            .iter()
            .any(|error| error.contains("item 99")),
        "synthetic_0024_missing_acceptance_anchor must fail through the real checker"
    );
}

#[test]
fn acceptance_artifact_0025_maps_spec_section_7_items_to_report_anchors() {
    let mut errors = acceptance_checklist_anchor_errors(
        ACCEPTANCE_0025_REPORT,
        ACCEPTANCE_0025_CHECKLIST_ANCHORS,
    );
    errors.extend(mutation_pending_certification_wording_errors(
        ACCEPTANCE_0025_REPORT,
    ));
    assert!(
        errors.is_empty(),
        "0025 acceptance artifact checklist anchors are missing or overstated: {errors:#?}"
    );

    let mut synthetic = ACCEPTANCE_0025_CHECKLIST_ANCHORS.to_vec();
    synthetic.push(AcceptanceChecklistAnchor {
        item: 99,
        anchors: &["synthetic_0025_missing_acceptance_anchor"],
    });
    let synthetic_errors = acceptance_checklist_anchor_errors(ACCEPTANCE_0025_REPORT, &synthetic);
    assert!(
        synthetic_errors
            .iter()
            .any(|error| error.contains("item 99")),
        "synthetic_0025_missing_acceptance_anchor must fail through the real checker"
    );

    let overstated_pending_report =
        ACCEPTANCE_0025_REPORT.replace("Pending is not a pass", "ORD-LIFE-CERT readiness: clear");
    assert!(
        mutation_pending_certification_wording_errors(&overstated_pending_report)
            .iter()
            .any(|error| error.contains("certification wording")),
        "synthetic pending mutation certification claim must fail"
    );
}

#[test]
fn workspace_source_classification_census_matches_production_tree() {
    let actual = production_sources()
        .into_iter()
        .map(|(path, _)| path)
        .collect::<Vec<_>>();
    let expected = WORKSPACE_SOURCE_CLASSIFICATIONS
        .iter()
        .map(|entry| entry.path.to_string())
        .collect::<Vec<_>>();

    assert_eq!(
        actual, expected,
        "every production source file across core, content, and tui must be classified as guarded-layer or exempt-with-rationale"
    );

    for entry in WORKSPACE_SOURCE_CLASSIFICATIONS {
        match entry.class {
            WorkspaceSourceClass::GuardedLayer => {}
            WorkspaceSourceClass::Exempt { rationale } => {
                assert!(
                    !rationale.is_empty(),
                    "{} has an empty exemption rationale",
                    entry.path
                );
            }
        }
    }
}

#[test]
fn workspace_dependency_posture_matches_allowlist() {
    let actual = workspace_dependency_entries();
    let expected = WORKSPACE_DEPENDENCY_ALLOWLIST
        .iter()
        .map(|(manifest_path, section, dependency)| DependencyEntry {
            manifest_path: (*manifest_path).to_string(),
            section: (*section).to_string(),
            dependency: (*dependency).to_string(),
        })
        .collect::<std::collections::BTreeSet<_>>();

    assert!(
        actual
            .iter()
            .filter(
                |entry| entry.manifest_path == "crates/tracewake-core/Cargo.toml"
                    && entry.section == "dependencies"
            )
            .count()
            == 0,
        "tracewake-core must keep [dependencies] empty"
    );
    assert_eq!(
        actual, expected,
        "workspace dependency posture changed; review crate ownership before updating the allowlist"
    );

    let mut synthetic_unlisted_workspace_dependency = actual;
    synthetic_unlisted_workspace_dependency.insert(DependencyEntry {
        manifest_path: "crates/tracewake-core/Cargo.toml".to_string(),
        section: "dependencies".to_string(),
        dependency: "synthetic_unlisted_dependency".to_string(),
    });
    assert_ne!(
        synthetic_unlisted_workspace_dependency, expected,
        "synthetic_unlisted_workspace_dependency must fail the allowlist parity census"
    );
}

#[test]
fn guarded_layer_entries_are_exactly_the_workspace_guarded_classifications() {
    for (path, _) in production_sources() {
        let classified_guarded = WORKSPACE_SOURCE_CLASSIFICATIONS.iter().any(|entry| {
            entry.path == path && matches!(entry.class, WorkspaceSourceClass::GuardedLayer)
        });
        assert_eq!(
            is_guarded_layer_source(&path),
            classified_guarded,
            "{path} guarded-layer predicate drifted from the workspace source classification table"
        );
    }

    assert!(
        WORKSPACE_SOURCE_CLASSIFICATIONS
            .iter()
            .filter(|entry| matches!(entry.class, WorkspaceSourceClass::GuardedLayer))
            .all(|entry| entry.path.starts_with("crates/tracewake-core/src/agent/")
                || entry.path.starts_with("crates/tracewake-core/src/runtime/")
                || entry.path == "crates/tracewake-core/src/scheduler.rs"
                || entry.path == "crates/tracewake-core/src/projections.rs"),
        "guarded layer classification must stay a reviewed explicit set, not a broad crate shortcut"
    );
}

#[allow(
    clippy::disallowed_methods,
    reason = "anti-regression test scans test sources; this helper is not simulation outcome code"
)]
fn test_sources() -> Vec<(String, String)> {
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("tracewake-core has a workspace parent");
    let mut stack = vec![
        repo_root.join("tracewake-core/tests"),
        repo_root.join("tracewake-content/tests"),
        repo_root.join("tracewake-tui/tests"),
    ];
    let mut sources = Vec::new();
    while let Some(path) = stack.pop() {
        for entry in std::fs::read_dir(path).expect("test directory is readable") {
            let entry = entry.expect("test directory entry is readable");
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                let relative = path
                    .strip_prefix(repo_root)
                    .expect("test path is under workspace crates")
                    .display()
                    .to_string();
                let source = std::fs::read_to_string(&path).expect("test file is readable");
                sources.push((relative, source));
            }
        }
    }
    sources
}

fn state_struct_fields_from_source(source: &str, struct_name: &str) -> Vec<String> {
    let marker = format!("pub struct {struct_name} {{");
    let body = source
        .split(&marker)
        .nth(1)
        .unwrap_or_else(|| panic!("{struct_name} declaration is present"))
        .split("}\n")
        .next()
        .unwrap_or_else(|| panic!("{struct_name} body is present"));
    body.lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            let field = trimmed.strip_prefix("pub(crate) ")?;
            Some(field.split(':').next().unwrap().to_string())
        })
        .collect()
}

fn checksum_parity_errors(
    state_source: &str,
    checksum_source: &str,
    physical_coverage: &[(&str, &str)],
    agent_coverage: &[(&str, &str)],
) -> Vec<String> {
    let mut errors = Vec::new();
    errors.extend(state_kind_checksum_parity_errors(
        "PhysicalState",
        state_source,
        checksum_source,
        physical_coverage,
    ));
    errors.extend(state_kind_checksum_parity_errors(
        "AgentState",
        state_source,
        checksum_source,
        agent_coverage,
    ));
    errors
}

fn state_kind_checksum_parity_errors(
    struct_name: &str,
    state_source: &str,
    checksum_source: &str,
    coverage: &[(&str, &str)],
) -> Vec<String> {
    let state_fields = state_struct_fields_from_source(state_source, struct_name);
    let coverage_fields = coverage
        .iter()
        .map(|(field_name, _)| (*field_name).to_string())
        .collect::<Vec<_>>();
    let mut errors = Vec::new();

    if coverage_fields != state_fields {
        errors.push(format!(
            "{struct_name} checksum coverage fields {coverage_fields:?} do not match authoritative fields {state_fields:?}"
        ));
    }

    for (field_name, field_family) in coverage {
        let canonical_prefix = format!("\"{field_family}|");
        if !checksum_source.contains(&canonical_prefix) {
            errors.push(format!(
                "{struct_name}.{field_name} lacks canonical checksum line prefix {field_family}|"
            ));
        }
    }

    errors
}

fn mutation_pending_certification_wording_errors(report: &str) -> Vec<String> {
    let mut errors = Vec::new();
    let pending = report.contains("scheduled mutation still pending")
        || report.contains("pending mutation status")
        || report.contains("still pending");
    if !pending {
        return errors;
    }
    for forbidden in [
        "ORD-LIFE-CERT readiness: clear",
        "ORD-LIFE-CERT passed",
        "Phase 4 entry approved",
        "FIRST-PROOF-CERT passed",
    ] {
        if report.contains(forbidden) {
            errors.push(format!(
                "pending mutation status coexists with certification wording: {forbidden}"
            ));
        }
    }
    for required in [
        "Pending is not a pass",
        "dated green scheduled mutation run",
    ] {
        if !report.contains(required) {
            errors.push(format!(
                "pending mutation status lacks required caveat: {required}"
            ));
        }
    }
    errors
}

fn nondeterminism_api_is_allowlisted(path: &str, token: &str) -> bool {
    NONDETERMINISM_ALLOWLIST
        .iter()
        .any(|entry| entry.path == path && entry.token == token)
}

fn scheduler_marker_allowlist_is_documented(snippet: &str) -> bool {
    SCHEDULER_MARKER_EVENT_ALLOWLIST.iter().any(|entry| {
        entry.snippet == snippet
            && !entry.rationale.is_empty()
            && !entry.responsible_layer.is_empty()
    })
}

fn source_contains_in_order(source: impl AsRef<str>, first: &str, second: &str) -> bool {
    let source = source.as_ref();
    let Some(first_index) = source.find(first) else {
        return false;
    };
    let Some(second_index) = source.find(second) else {
        return false;
    };
    first_index < second_index
}

fn low_pressure_agent_state(
    actor_id: tracewake_core::ids::ActorId,
) -> tracewake_core::state::AgentState {
    use tracewake_core::agent::{NeedChangeCause, NeedKind, NeedState};

    let mut state = AgentSeed::default();
    state.needs_by_actor_mut().insert(
        actor_id,
        [
            (
                NeedKind::Hunger,
                NeedState::initial(NeedKind::Hunger, 100, NeedChangeCause::FixtureInitial),
            ),
            (
                NeedKind::Fatigue,
                NeedState::initial(NeedKind::Fatigue, 100, NeedChangeCause::FixtureInitial),
            ),
            (
                NeedKind::Safety,
                NeedState::initial(NeedKind::Safety, 100, NeedChangeCause::FixtureInitial),
            ),
        ]
        .into_iter()
        .collect(),
    );
    state.build()
}

fn source_bound_human_proposal(
    proposal_id: &str,
    actor_id: &tracewake_core::ids::ActorId,
    action_id: &str,
    semantic_action_id: &str,
    tick: tracewake_core::time::SimTick,
    frontier: u64,
) -> tracewake_core::actions::Proposal {
    use tracewake_core::actions::{
        Proposal, ProposalOrigin, ProposalSource, ProposalSourceContext,
    };
    use tracewake_core::epistemics::KnowledgeContext;
    use tracewake_core::ids::{ActionId, ProposalId, SemanticActionId, ViewModelId};

    let mut proposal = Proposal::new(
        ProposalId::new(proposal_id).unwrap(),
        ProposalOrigin::Human,
        Some(actor_id.clone()),
        ActionId::new(action_id).unwrap(),
        tick,
    );
    let context = KnowledgeContext::embodied_at_frontier(actor_id.clone(), tick, frontier);
    let source_view_model_id =
        ViewModelId::new(format!("view.{}.{}", actor_id.as_str(), tick.value())).unwrap();
    proposal.source_view_model_id = Some(source_view_model_id.clone());
    proposal.source = Some(ProposalSource::TuiSemanticAction(ProposalSourceContext {
        source_view_model_id,
        holder_known_context_id: context.holder_known_context_id().clone(),
        holder_known_context_hash: context.holder_known_context_hash().clone(),
        holder_known_context_frontier: context.event_frontier(),
        context_tick: tick,
        actor_id: actor_id.clone(),
        semantic_action_id: SemanticActionId::new(semantic_action_id).unwrap(),
        provenance_ancestry: vec!["test:current_view".to_string()],
    }));
    proposal
}

fn human_source_report(
    proposal: &tracewake_core::actions::Proposal,
    current_event_frontier: u64,
) -> tracewake_core::actions::ValidationReport {
    use tracewake_core::actions::{
        validate_proposal, ActionDefinition, ActionRegistry, ActionScope, ProposalValidationContext,
    };
    use tracewake_core::controller::ControllerBindings;
    use tracewake_core::events::log::EventLog;
    use tracewake_core::ids::{ActionId, ActorId, ContentManifestId, ControllerId, PlaceId};
    use tracewake_core::scheduler::{
        OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId,
    };
    use tracewake_core::state::{ActorBody, AgentState, ControllerMode};
    use tracewake_core::time::SimTick;

    let actor_id = ActorId::new("actor_tomas").unwrap();
    let controller_id = ControllerId::new("controller_human").unwrap();
    let mut state_seed = PhysicalSeed::default();
    state_seed.actors_mut().insert(
        actor_id.clone(),
        ActorBody::new(actor_id.clone(), PlaceId::new("shop_front").unwrap()),
    );
    let state = state_seed.build();
    let mut registry = ActionRegistry::new();
    registry.register(ActionDefinition::query_only(
        ActionId::new("look").unwrap(),
        ActionScope::Phase1,
    ));
    registry.register(ActionDefinition::query_only(
        ActionId::new("inspect_place").unwrap(),
        ActionScope::Phase1,
    ));
    let content_manifest_id = ContentManifestId::new("phase1_manifest").unwrap();
    let mut bindings = ControllerBindings::new();
    let mut binding_log = EventLog::new();
    bindings.attach(
        controller_id.clone(),
        actor_id,
        ControllerMode::Embodied,
        SimTick::ZERO,
        &mut binding_log,
        content_manifest_id.clone(),
    );
    let ordering_key = OrderingKey::new(
        proposal.requested_tick,
        SchedulePhase::HumanCommand,
        SchedulerSourceId::Controller(controller_id),
        ProposalSequence::new(0),
        proposal.action_id.clone(),
        proposal.target_ids.clone(),
        "source-context-guard",
    );
    validate_proposal(
        ProposalValidationContext {
            registry: &registry,
            state: &state,
            agent_state: &AgentState::default(),
            controller_bindings: Some(&bindings),
            epistemic_projection: None,
            content_manifest_id: &content_manifest_id,
            ordering_key: &ordering_key,
            current_event_frontier,
        },
        proposal,
    )
}

#[test]
fn nondeterminism_api_gate() {
    // Smoke-only substring scan. `clippy.toml` plus the negative fixture runner
    // are the primary banned-API enforcement layer; this catches obvious drift
    // and intentionally remains comment/string sensitive.
    assert!(
        NONDETERMINISM_ALLOWLIST.is_empty(),
        "tracewake-core outcome paths must keep the nondeterminism allowlist empty until a narrow, rationale-bearing exception is reviewed"
    );

    for entry in NONDETERMINISM_ALLOWLIST {
        assert!(
            !entry.path.is_empty()
                && !entry.token.is_empty()
                && !entry.rationale.is_empty()
                && !entry.responsible_layer.is_empty(),
            "nondeterminism allowlist entries require path, token, rationale, and responsible layer"
        );
    }

    let mut violations = Vec::new();
    for (path, source) in production_sources() {
        for banned in BANNED_NONDETERMINISM_TOKENS {
            if source.contains(banned.token)
                && !nondeterminism_api_is_allowlisted(&path, banned.token)
            {
                violations.push(format!(
                    "{} contains {}: {}",
                    path, banned.token, banned.reason
                ));
            }
        }
    }

    assert!(
        violations.is_empty(),
        "nondeterministic API use in tracewake-core outcome path:\n{}",
        violations.join("\n")
    );

    let synthetic_violation_pattern =
        "Adding HashMap to a tracewake-core production source must fail this test unless a narrow allowlist entry explains the exception.";
    assert!(synthetic_violation_pattern.contains("must fail this test"));
}

#[test]
#[allow(
    clippy::disallowed_methods,
    reason = "scanner discovery test creates a temporary source tree outside simulation outcome code"
)]
fn source_guard_discovers_new_nested_production_file() {
    let root = std::env::temp_dir().join(format!("tracewake_source_guard_{}", std::process::id()));
    let nested = root.join("nested/deeper");
    std::fs::create_dir_all(&nested).expect("temporary nested source directory can be created");
    std::fs::write(
        nested.join("prod.rs"),
        "pub fn nested_production_item() {}\n",
    )
    .expect("temporary source file can be written");

    let sources = production_sources_from_roots(vec![root.clone()], &root);
    std::fs::remove_dir_all(&root).expect("temporary source tree can be removed");

    assert!(
        sources
            .iter()
            .any(|(path, source)| path.ends_with("nested/deeper/prod.rs")
                && source.contains("nested_production_item")),
        "source scanner must discover nested production Rust files"
    );
}

#[test]
fn source_guard_does_not_silently_skip_production_after_cfg_test() {
    let source = r#"
pub fn before_cfg_test() {}

#[cfg(test)]
mod tests {
    fn test_only() {
        let _comment_sensitive_smoke_token = "HashMap";
    }
}

pub fn after_cfg_test() {}
"#;

    let production = production(source);
    assert!(production.contains("before_cfg_test"));
    assert!(production.contains("after_cfg_test"));
    assert!(!production.contains("test_only"));
}

#[test]
fn scheduler_never_direct_dispatches_primitive_action() {
    use tracewake_core::actions::pipeline::{run_pipeline, PipelineContext};
    use tracewake_core::actions::proposal::{Proposal, ProposalOrigin};
    use tracewake_core::actions::registry::ActionRegistry;
    use tracewake_core::checksum::{compute_physical_checksum, ChecksumContext};
    use tracewake_core::events::log::EventLog;
    use tracewake_core::events::{EventKind, EventStream};
    use tracewake_core::ids::{
        ActionId, ActorId, ContentManifestId, ContentVersion, FixtureId, PlaceId, ProcessId,
        ProposalId,
    };
    use tracewake_core::scheduler::no_human::{advance_no_human, NoHumanStateMut};
    use tracewake_core::scheduler::{
        OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId,
    };
    use tracewake_core::state::ActorBody;
    use tracewake_core::time::SimTick;

    let scheduler = production(SCHEDULER_RS);
    for forbidden in [
        "actions::defs::accuseprobe",
        "actions::defs::checkcontainer",
        "actions::defs::continue_routine",
        "actions::defs::eat",
        "actions::defs::movement",
        "actions::defs::openclose",
        "actions::defs::takeplace",
        "actions::defs::wait",
        "build_check_container_event",
        "build_continue_routine_event",
        "build_eat_events",
        "build_move_event",
        "build_open_close_event",
        "build_take_place_event",
        "build_wait_events",
        "validate_truthful_accuse_probe",
    ] {
        assert_absent(&scheduler, forbidden);
    }

    for allowed in [
        "build_sleep_completion_events",
        "build_work_completion_events",
        "append_marker",
        "append_and_apply_agent_event",
    ] {
        assert!(
            scheduler_marker_allowlist_is_documented(allowed),
            "scheduler marker allowlist lacks rationale for {allowed}"
        );
        assert!(
            scheduler.contains(allowed),
            "reviewed scheduler marker/event constructor is absent or renamed: {allowed}"
        );
    }

    assert!(
        scheduler.contains("run_pipeline(&mut context, &proposal)"),
        "scheduler ordinary no-human proposals must route through the shared pipeline"
    );
    assert!(
        scheduler.contains("ActorDecisionTransaction::run"),
        "scheduler autonomous proposals must come from the actor decision transaction"
    );
    assert!(
        SLEEP_RS.contains("pub fn build_sleep_completion_events(\n    state: &PhysicalState,\n    agent_state: &AgentState,\n    log: &EventLog,"),
        "sleep completion builder must require current physical state, agent state, and event log for continuity and need-accounting checks"
    );
    assert!(
        WORK_RS.contains("pub fn build_work_completion_events(\n    state: &PhysicalState,\n    agent_state: &AgentState,\n    log: &EventLog,"),
        "work completion builder must require current physical state, agent state, and event log for continuity and need-accounting checks"
    );
    assert!(
        scheduler.contains(
            "build_sleep_completion_events(\n                    state,\n                    &scratch_agent_state,\n                    &scratch_log,"
        ),
        "scheduler must pass current state and preflight log into sleep completion continuity checks"
    );
    assert!(
        scheduler.contains(
            "build_work_completion_events(\n                    state,\n                    &scratch_agent_state,\n                    &scratch_log,"
        ),
        "scheduler must pass current state and preflight log into work completion continuity checks"
    );
    assert!(
        scheduler.contains("if is_duration_terminal(appended.event_type)"),
        "scheduler must classify scheduled duration completions through the shared terminal predicate"
    );
    assert!(
        scheduler.contains("classify_actor_tick_regimes("),
        "scheduler passive need deltas must consume the tick-regime classifier"
    );

    let actor_id = ActorId::new("actor_tomas").unwrap();
    let place_id = PlaceId::new("shop_front").unwrap();
    let mut scheduled_seed = PhysicalSeed::default();
    scheduled_seed
        .actors_mut()
        .insert(actor_id.clone(), ActorBody::new(actor_id.clone(), place_id));
    let mut scheduled_state = scheduled_seed.build();
    let mut scheduled_agent_state = low_pressure_agent_state(actor_id.clone());
    let mut scheduled_log = EventLog::new();
    let mut registry = ActionRegistry::new();
    registry.register_phase1_inspect_wait();
    let content_manifest_id = ContentManifestId::new("phase1_manifest").unwrap();
    let mut proposal = Proposal::new(
        ProposalId::new("proposal_wait").unwrap(),
        ProposalOrigin::Scheduler,
        Some(actor_id.clone()),
        ActionId::new("wait").unwrap(),
        SimTick::ZERO,
    );
    proposal
        .parameters
        .insert("reason".to_string(), "scheduled wait".to_string());
    let context = ChecksumContext {
        fixture_id: FixtureId::new("scheduler_no_direct_dispatch").unwrap(),
        content_version: ContentVersion::new("content_v1").unwrap(),
        sim_tick: SimTick::ZERO,
        world_stream_position_applied: 0,
    };
    let before_checksum = compute_physical_checksum(&scheduled_state, &context).checksum;

    let report = advance_no_human(
        NoHumanStateMut {
            physical: &mut scheduled_state,
            agent: &mut scheduled_agent_state,
        },
        &mut scheduled_log,
        &registry,
        content_manifest_id.clone(),
        SimTick::ZERO,
        1,
        vec![proposal.clone()],
    );

    let after_checksum = compute_physical_checksum(&scheduled_state, &context).checksum;
    assert_eq!(
        after_checksum, before_checksum,
        "wait proposal does not physically mutate; scheduler marker events must not alter physical state"
    );
    assert_eq!(report.marker_event_ids.len(), 2);
    assert_eq!(report.ordinary_pipeline_events, 3);
    assert_eq!(
        scheduled_log
            .events()
            .iter()
            .filter(|event| event.stream == EventStream::Diagnostic)
            .count(),
        2
    );

    let mut direct_seed = PhysicalSeed::default();
    direct_seed.actors_mut().insert(
        actor_id.clone(),
        ActorBody::new(actor_id.clone(), PlaceId::new("shop_front").unwrap()),
    );
    let mut direct_state = direct_seed.build();
    let mut direct_agent_state = low_pressure_agent_state(actor_id.clone());
    let mut direct_log = EventLog::new();
    let mut direct_context = PipelineContext {
        registry: &registry,
        state: &mut direct_state,
        agent_state: &mut direct_agent_state,
        log: &mut direct_log,
        controller_bindings: None,
        epistemic_projection: None,
        content_manifest_id,
        ordering_key: OrderingKey::new(
            SimTick::ZERO,
            SchedulePhase::NoHumanProcess,
            SchedulerSourceId::Process(ProcessId::new("no_human_advance").unwrap()),
            ProposalSequence::new(0),
            ActionId::new("wait").unwrap(),
            Vec::new(),
            "proposal_wait",
        ),
    };
    let direct = run_pipeline(&mut direct_context, &proposal);
    let scheduled_ordinary_kinds = scheduled_log
        .events()
        .iter()
        .filter(|event| event.stream != EventStream::Diagnostic)
        .filter(|event| event.event_type != EventKind::TimeAdvanced)
        .map(|event| event.event_type)
        .collect::<Vec<_>>();
    let direct_ordinary_kinds = direct
        .appended_events
        .iter()
        .map(|event| event.event_type)
        .collect::<Vec<_>>();
    assert_eq!(
        scheduled_ordinary_kinds, direct_ordinary_kinds,
        "scheduler ordinary effects must match the shared pipeline output"
    );
    assert_eq!(
        scheduled_ordinary_kinds,
        vec![
            EventKind::ActorWaited,
            EventKind::NeedDeltaApplied,
            EventKind::NeedDeltaApplied,
        ]
    );
}

#[test]
fn forged_or_stale_source_context_rejected_by_reason_code() {
    use tracewake_core::actions::{
        Proposal, ProposalOrigin, ProposalSource, ReasonCode, ReportStatus,
    };
    use tracewake_core::ids::{ActionId, ActorId, HolderKnownContextId, ProposalId};
    use tracewake_core::time::SimTick;

    let actor_id = ActorId::new("actor_tomas").unwrap();
    let cases = [
        (
            "missing",
            {
                Proposal::new(
                    ProposalId::new("proposal_missing_source").unwrap(),
                    ProposalOrigin::Human,
                    Some(actor_id.clone()),
                    ActionId::new("look").unwrap(),
                    SimTick::ZERO,
                )
            },
            0,
            ReasonCode::ProposalSourceMissing,
        ),
        (
            "stale_frontier",
            source_bound_human_proposal(
                "proposal_stale_frontier",
                &actor_id,
                "look",
                "look",
                SimTick::ZERO,
                0,
            ),
            1,
            ReasonCode::ProposalSourceStale,
        ),
        (
            "forged_semantic_action",
            source_bound_human_proposal(
                "proposal_forged_semantic",
                &actor_id,
                "look",
                "move.to.hidden_room",
                SimTick::ZERO,
                0,
            ),
            0,
            ReasonCode::ProposalSourceForged,
        ),
    ];

    for (case_id, mut proposal, current_frontier, expected_reason) in cases {
        proposal
            .parameters
            .insert("controller_id".to_string(), "controller_human".to_string());
        let report = human_source_report(&proposal, current_frontier);
        assert_eq!(report.status, ReportStatus::Rejected, "{case_id}");
        assert_eq!(report.reason_codes, vec![expected_reason], "{case_id}");
        assert_eq!(
            report
                .reason_codes
                .iter()
                .map(|reason| reason.stable_id())
                .collect::<Vec<_>>(),
            vec![expected_reason.stable_id()],
            "{case_id}"
        );
    }

    let mut raw_action_prefix = source_bound_human_proposal(
        "proposal_raw_action_prefix",
        &actor_id,
        "inspect_place",
        "inspect_place.shop_front",
        SimTick::ZERO,
        0,
    );
    raw_action_prefix
        .parameters
        .insert("controller_id".to_string(), "controller_human".to_string());
    let report = human_source_report(&raw_action_prefix, 0);
    assert_eq!(report.status, ReportStatus::Accepted);
    assert!(
        report.reason_codes.is_empty(),
        "raw action-id semantic prefixes must validate before dotted aliases are considered"
    );

    let mut actor_mismatch = source_bound_human_proposal(
        "proposal_actor_mismatch",
        &actor_id,
        "look",
        "look",
        SimTick::ZERO,
        0,
    );
    if let Some(ProposalSource::TuiSemanticAction(source)) = actor_mismatch.source.as_mut() {
        source.actor_id = ActorId::new("actor_elena").unwrap();
    }
    actor_mismatch
        .parameters
        .insert("controller_id".to_string(), "controller_human".to_string());
    let report = human_source_report(&actor_mismatch, 0);
    assert_eq!(
        report.reason_codes,
        vec![ReasonCode::ProposalSourceActorMismatch]
    );

    let mut stale_tick = source_bound_human_proposal(
        "proposal_stale_tick",
        &actor_id,
        "look",
        "look",
        SimTick::ZERO,
        0,
    );
    if let Some(ProposalSource::TuiSemanticAction(source)) = stale_tick.source.as_mut() {
        source.context_tick = SimTick::new(1);
    }
    stale_tick
        .parameters
        .insert("controller_id".to_string(), "controller_human".to_string());
    let report = human_source_report(&stale_tick, 0);
    assert_eq!(report.reason_codes, vec![ReasonCode::ProposalSourceStale]);

    let mut context_mismatch = source_bound_human_proposal(
        "proposal_context_mismatch",
        &actor_id,
        "look",
        "look",
        SimTick::ZERO,
        0,
    );
    if let Some(ProposalSource::TuiSemanticAction(source)) = context_mismatch.source.as_mut() {
        source.holder_known_context_id = HolderKnownContextId::new("hkc.forged").unwrap();
    }
    context_mismatch
        .parameters
        .insert("controller_id".to_string(), "controller_human".to_string());
    let report = human_source_report(&context_mismatch, 0);
    assert_eq!(
        report.reason_codes,
        vec![ReasonCode::ProposalSourceContextMismatch]
    );

    for report in [
        human_source_report(&actor_mismatch, 0),
        human_source_report(&stale_tick, 0),
        human_source_report(&context_mismatch, 0),
    ] {
        assert!(
            report
                .reason_codes
                .iter()
                .all(|reason| !reason.stable_id().is_empty()),
            "source-context negatives must assert typed reason codes, not actor-facing labels"
        );
    }
}

#[test]
fn diagnostics_never_assert_display_label_as_authority() {
    struct ForbiddenDiagnosticAssertion {
        snippet: &'static str,
        reason: &'static str,
    }

    let forbidden = [
        ForbiddenDiagnosticAssertion {
            snippet: "assert_eq!(report.actor_visible_summary",
            reason: "actor-facing summaries are presentation; assert ReasonCode/stable_id fields",
        },
        ForbiddenDiagnosticAssertion {
            snippet: "assert_eq!(result.report.actor_visible_summary",
            reason: "actor-facing summaries are presentation; assert ReasonCode/stable_id fields",
        },
        ForbiddenDiagnosticAssertion {
            snippet: "assert_eq!(why_not.actor_known_summary",
            reason: "why-not summaries are presentation; assert reason codes and checked facts",
        },
        ForbiddenDiagnosticAssertion {
            snippet: ".actor_visible_summary.contains(\"door_closed",
            reason: "stable reason-code strings belong to ReasonCode::stable_id assertions",
        },
        ForbiddenDiagnosticAssertion {
            snippet: ".actor_visible_summary.contains(\"container_closed",
            reason: "stable reason-code strings belong to ReasonCode::stable_id assertions",
        },
        ForbiddenDiagnosticAssertion {
            snippet: ".actor_visible_summary.contains(\"knowledge_precondition",
            reason: "stable reason-code strings belong to ReasonCode::stable_id assertions",
        },
    ];

    let mut violations = Vec::new();
    for (path, source) in test_sources() {
        let source = source
            .lines()
            .filter(|line| !line.trim_start().starts_with("snippet:"))
            .collect::<Vec<_>>()
            .join("\n");
        for assertion in &forbidden {
            if source.contains(assertion.snippet) {
                violations.push(format!(
                    "{} contains {}: {}",
                    path, assertion.snippet, assertion.reason
                ));
            }
        }
    }

    assert!(
        violations.is_empty(),
        "diagnostic tests must not use display labels as semantic authority:\n{}",
        violations.join("\n")
    );

    let synthetic_bad_assertion = [
        "assert_eq!(report.",
        "actor_visible_summary, \"door closed\")",
    ]
    .concat();
    assert!(
        forbidden
            .iter()
            .any(|assertion| synthetic_bad_assertion.contains(assertion.snippet)),
        "the display-label assertion guard must catch direct summary equality"
    );
}

#[test]
fn validation_report_keeps_typed_provenance_and_actor_debug_split() {
    use tracewake_core::actions::pipeline::PipelineStage;
    use tracewake_core::actions::report::{CheckedFactKey, CheckedFactSource};
    use tracewake_core::actions::{CheckedFact, ReasonCode, ReportStatus, ValidationReport};
    use tracewake_core::ids::{ActionId, ActorId, ProposalId, ValidationReportId};

    for required in [
        "pub failed_stage: Option<crate::actions::pipeline::PipelineStage>",
        "pub reason_codes: Vec<ReasonCode>",
        "pub checked_facts: Vec<CheckedFact>",
        "pub actor_visible_facts: Vec<CheckedFact>",
        "pub debug_only_facts: Vec<CheckedFact>",
        "source: CheckedFactSource",
        "pub actor_visible_summary: String",
        "pub debug_summary: String",
    ] {
        assert!(
            ACTIONS_REPORT_RS.contains(required),
            "diagnostic report typing/separation changed or was removed: {required}"
        );
    }
    assert!(
        source_contains_in_order(
            ACTIONS_REPORT_RS,
            "pub actor_visible_facts: Vec<CheckedFact>",
            "pub debug_only_facts: Vec<CheckedFact>"
        ),
        "actor-visible facts and debug-only facts must remain structurally separate fields"
    );
    assert_absent(ACTIONS_REPORT_RS, "pub facts: Vec<String>");
    assert_absent(ACTIONS_REPORT_RS, "pub reason_codes: Vec<String>");

    let actor_fact = CheckedFact::new("door_id", "door_house_street");
    let debug_fact = CheckedFact::new("holder_known_context_hash", "hkc_hash_hidden");
    let report = ValidationReport {
        validation_report_id: ValidationReportId::new("validation_report_diag_guard").unwrap(),
        proposal_id: ProposalId::new("proposal_diag_guard").unwrap(),
        actor_id: Some(ActorId::new("actor_tomas").unwrap()),
        action_id: ActionId::new("move").unwrap(),
        target_ids: vec!["back_room".to_string()],
        status: ReportStatus::Rejected,
        failed_stage: Some(PipelineStage::PhysicalPreconditionValidation),
        reason_codes: vec![ReasonCode::DoorClosedBlocksMovement],
        checked_facts: vec![actor_fact.clone(), debug_fact.clone()],
        actor_visible_facts: vec![actor_fact.clone()],
        debug_only_facts: vec![debug_fact.clone()],
        actor_visible_summary: "The way is blocked.".to_string(),
        debug_summary: "validator saw closed door and holder-known hash".to_string(),
        would_mutate: false,
        event_ids: Vec::new(),
        checksum_before: None,
        checksum_after: None,
    };

    assert_eq!(
        report.failed_stage,
        Some(PipelineStage::PhysicalPreconditionValidation)
    );
    assert_eq!(
        report.reason_codes,
        vec![ReasonCode::DoorClosedBlocksMovement]
    );
    assert_eq!(
        report
            .reason_codes
            .iter()
            .map(|reason| reason.stable_id())
            .collect::<Vec<_>>(),
        vec!["door_closed_blocks_movement"]
    );
    assert_eq!(report.actor_visible_facts, vec![actor_fact.clone()]);
    assert_eq!(report.debug_only_facts, vec![debug_fact.clone()]);
    assert_eq!(actor_fact.key(), &CheckedFactKey::DoorId);
    assert_eq!(
        debug_fact.key(),
        &CheckedFactKey::Unsupported("holder_known_context_hash".to_string())
    );
    assert_eq!(actor_fact.source(), CheckedFactSource::Validation);
    assert_eq!(debug_fact.source().stable_id(), "validation");
    assert!(
        !report.actor_visible_facts.contains(&debug_fact),
        "debug-only checked facts must not be reused as actor-visible why-not facts"
    );
}

#[test]
fn embodied_projection_never_uses_unfiltered_checked_facts_as_actor_provenance() {
    let projections = production(PROJECTIONS_RS);
    assert!(
        projections.contains("report.actor_visible_facts.iter()"),
        "embodied validator availability must source actor provenance from actor-visible facts"
    );
    assert_absent(&projections, "report.checked_facts.iter()");

    let synthetic_bad_projection = "provenance_refs.extend(report.checked_facts.iter())";
    assert!(
        synthetic_bad_projection.contains("report.checked_facts.iter()"),
        "synthetic violation must exercise the checked-facts source guard"
    );
}

#[test]
fn privileged_tui_proposal_requires_current_view_source_context() {
    let app = production(TUI_APP_RS);
    assert!(
        app.contains("RuntimeCommand::submit_semantic_action"),
        "TUI semantic-action submission must use the runtime command boundary"
    );

    let runtime_session = production(RUNTIME_SESSION_RS);
    assert!(
        runtime_session.contains("proposal_from_current_view_semantic_action"),
        "runtime semantic-action submission must use the current-view source-context constructor"
    );
    assert_absent(runtime_session, "proposal_from_semantic_action_entry");

    let projections = production(PROJECTIONS_RS);
    assert!(
        projections.contains("pub fn proposal_from_current_view_semantic_action"),
        "core must expose a current-view-only semantic-action proposal constructor"
    );
    assert!(
        projections.contains("origin != ProposalOrigin::Human || source_view.is_some()"),
        "optional semantic-action helper must fail closed for human-origin proposals without a source view"
    );
}

#[test]
fn no_direct_apply_event_outside_event_replay_or_pipeline() {
    let sources = core_production_sources();
    let scanned_tokens = apply_mutator_tokens_from(EVENTS_APPLY_RS);
    let errors = apply_mutator_perimeter_errors(EVENTS_APPLY_RS, &sources, &scanned_tokens);
    assert!(
        errors.is_empty(),
        "direct apply mutator call outside allowlisted production seams:\n{}",
        errors.join("\n")
    );
    assert_eq!(
        scanned_tokens,
        BTreeSet::from([
            "apply_agent_event(".to_string(),
            "apply_epistemic_event(".to_string(),
            "apply_event(".to_string(),
            "apply_event_stream(".to_string(),
        ]),
        "apply mutator scan must be derived from every public apply.rs mutator"
    );
    assert!(
        production(include_str!("../src/actions/pipeline.rs")).contains("let mut dry_run = context.state.clone();"),
        "pipeline dry-run validation must apply constructed events to cloned, non-authoritative state"
    );

    let mut injected_sources = sources.clone();
    injected_sources.push((
        "crates/tracewake-core/src/view_models.rs".to_string(),
        "fn leaking_view_model() { apply_agent_event(agent_state, &event).unwrap(); }".to_string(),
    ));
    let injection_errors =
        apply_mutator_perimeter_errors(EVENTS_APPLY_RS, &injected_sources, &scanned_tokens);
    assert!(
        injection_errors
            .iter()
            .any(|error| error.contains("view_models.rs") && error.contains("apply_agent_event(")),
        "synthetic_direct_apply_agent_event_call did not trigger: {injection_errors:?}"
    );

    let divergent_apply_source =
        format!("{EVENTS_APPLY_RS}\npub fn apply_story_event(_state: &mut (), _event: &()) {{}}\n");
    let parity_errors =
        apply_mutator_perimeter_errors(&divergent_apply_source, &sources, &scanned_tokens);
    assert!(
        parity_errors
            .iter()
            .any(|error| error.contains("apply_story_event(")),
        "synthetic_unscanned_public_apply_mutator did not trigger: {parity_errors:?}"
    );

    assert_eq!(
        apply_mutator_live_witness_count(&sources),
        APPLY_MUTATOR_ALLOWLIST.len(),
        "each apply mutator allowlist seam must have a live apply call witness"
    );
    let dropped_scheduler_sources = sources
        .iter()
        .map(|(path, source)| {
            if path == "crates/tracewake-core/src/scheduler.rs" {
                let mut retired_source = source.clone();
                for token in &scanned_tokens {
                    retired_source = retired_source.replace(token, "retired_apply_mutator(");
                }
                (path.clone(), retired_source)
            } else {
                (path.clone(), source.clone())
            }
        })
        .collect::<Vec<_>>();
    assert!(
        apply_mutator_live_witness_count(&dropped_scheduler_sources)
            < APPLY_MUTATOR_ALLOWLIST.len(),
        "synthetic_apply_mutator_witness_drop did not reduce the per-seam witness count"
    );
}

#[test]
fn accepted_action_appends_before_authoritative_apply() {
    use tracewake_core::actions::{
        run_pipeline, validate_proposal, ActionRegistry, PipelineContext, Proposal, ProposalOrigin,
        ProposalValidationContext, ReportStatus,
    };
    use tracewake_core::checksum::{compute_physical_checksum, ChecksumContext};
    use tracewake_core::events::log::EventLog;
    use tracewake_core::ids::{
        ActionId, ActorId, ContainerId, ContentManifestId, ContentVersion, FixtureId, PlaceId,
        ProposalId,
    };
    use tracewake_core::scheduler::{
        OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId,
    };
    use tracewake_core::state::{ActorBody, AgentState, ContainerState, PlaceState};
    use tracewake_core::time::SimTick;

    let pipeline = production(include_str!("../src/actions/pipeline.rs"));
    assert!(
        source_contains_in_order(
            pipeline,
            "context.log.append(event)",
            "apply_event_stream(&mut application_context, &appended)"
        ),
        "run_pipeline must append each accepted event before applying it to authoritative state"
    );

    let actor_id = ActorId::new("actor_tomas").unwrap();
    let place_id = PlaceId::new("shop_front").unwrap();
    let container_id = ContainerId::new("strongbox_tomas").unwrap();
    let mut seed = PhysicalSeed::default();
    seed.places_mut().insert(
        place_id.clone(),
        PlaceState::new(place_id.clone(), "Shop front"),
    );
    seed.actors_mut().insert(
        actor_id.clone(),
        ActorBody::new(actor_id.clone(), place_id.clone()),
    );
    seed.containers_mut().insert(
        container_id.clone(),
        ContainerState::fixed_at_place(container_id.clone(), place_id),
    );
    let mut state = seed.build();
    let mut registry = ActionRegistry::new();
    registry.register_phase1_movement_open_close();
    let mut log = EventLog::new();
    let mut agent_state = AgentState::default();
    let content_manifest_id = ContentManifestId::new("phase1_manifest").unwrap();
    let mut proposal = Proposal::new(
        ProposalId::new("proposal_open_strongbox").unwrap(),
        ProposalOrigin::Test,
        Some(actor_id.clone()),
        ActionId::new("open").unwrap(),
        SimTick::ZERO,
    );
    proposal.target_ids.push(container_id.as_str().to_string());
    let ordering_key = OrderingKey::new(
        SimTick::ZERO,
        SchedulePhase::HumanCommand,
        SchedulerSourceId::Actor(actor_id.clone()),
        ProposalSequence::new(0),
        ActionId::new("open").unwrap(),
        proposal.target_ids.clone(),
        "append-before-apply",
    );
    let checksum_context = ChecksumContext {
        fixture_id: FixtureId::new("append_before_apply").unwrap(),
        content_version: ContentVersion::new("content_v1").unwrap(),
        sim_tick: SimTick::ZERO,
        world_stream_position_applied: 0,
    };
    let before_checksum = compute_physical_checksum(&state, &checksum_context).checksum;
    let before_log_len = log.events().len();

    let result = run_pipeline(
        &mut PipelineContext {
            registry: &registry,
            state: &mut state,
            agent_state: &mut agent_state,
            log: &mut log,
            controller_bindings: None,
            epistemic_projection: None,
            content_manifest_id: content_manifest_id.clone(),
            ordering_key: ordering_key.clone(),
        },
        &proposal,
    );

    assert_eq!(result.report.status, ReportStatus::Accepted);
    assert!(log.events().len() > before_log_len);
    let after_checksum = compute_physical_checksum(&state, &checksum_context).checksum;
    assert_ne!(after_checksum, before_checksum);

    let dry_run_state = state.clone();
    let dry_run_checksum = compute_physical_checksum(&dry_run_state, &checksum_context).checksum;
    let dry_run_log_len = log.events().len();
    let mut rejected = proposal.clone();
    rejected.proposal_id = ProposalId::new("proposal_bad_target").unwrap();
    rejected.target_ids = vec!["missing_container".to_string()];
    let report = validate_proposal(
        ProposalValidationContext {
            registry: &registry,
            state: &dry_run_state,
            agent_state: &AgentState::default(),
            controller_bindings: None,
            epistemic_projection: None,
            content_manifest_id: &content_manifest_id,
            ordering_key: &ordering_key,
            current_event_frontier: log.events().len() as u64,
        },
        &rejected,
    );

    assert_eq!(report.status, ReportStatus::Rejected);
    assert_eq!(log.events().len(), dry_run_log_len);
    assert_eq!(
        compute_physical_checksum(&dry_run_state, &checksum_context).checksum,
        dry_run_checksum
    );

    let synthetic_direct_apply =
        "Adding apply_event(&mut authoritative_state, event) outside actions/pipeline, events, or replay must fail the source scan.";
    assert!(synthetic_direct_apply.contains("must fail"));
}

#[test]
fn movement_door_endpoint_mismatch_rejects_partial_connections() {
    use tracewake_core::actions::defs::movement::build_move_event;
    use tracewake_core::actions::proposal::{Proposal, ProposalOrigin};
    use tracewake_core::actions::ReasonCode;
    use tracewake_core::ids::{DoorId, PlaceId};
    use tracewake_core::state::{ActorBody, DoorState, PhysicalState, PlaceState};

    fn ids() -> (ActorId, PlaceId, PlaceId, PlaceId) {
        (
            ActorId::new("actor_tomas").unwrap(),
            PlaceId::new("shop_front").unwrap(),
            PlaceId::new("back_room").unwrap(),
            PlaceId::new("side_room").unwrap(),
        )
    }

    fn proposal() -> Proposal {
        let (actor_id, _, to_place_id, _) = ids();
        let mut proposal = Proposal::new(
            ProposalId::new("proposal_move_door_endpoint_guard").unwrap(),
            ProposalOrigin::Test,
            Some(actor_id),
            ActionId::new("move").unwrap(),
            SimTick::ZERO,
        );
        proposal.target_ids.push(to_place_id.to_string());
        proposal
    }

    fn ordering_key(target_id: &PlaceId) -> OrderingKey {
        let (actor_id, _, _, _) = ids();
        OrderingKey::new(
            SimTick::ZERO,
            SchedulePhase::HumanCommand,
            SchedulerSourceId::Actor(actor_id),
            ProposalSequence::new(0),
            ActionId::new("move").unwrap(),
            vec![target_id.to_string()],
            "door-endpoint-guard",
        )
    }

    fn state_with_door(endpoint_a: &PlaceId, endpoint_b: &PlaceId) -> PhysicalState {
        let (actor_id, from_place_id, to_place_id, side_place_id) = ids();
        let mut seed = PhysicalSeed::default();
        seed.places_mut().insert(
            from_place_id.clone(),
            PlaceState::new(from_place_id.clone(), "Shop front"),
        );
        seed.places_mut().insert(
            to_place_id.clone(),
            PlaceState::new(to_place_id.clone(), "Back room"),
        );
        seed.places_mut().insert(
            side_place_id.clone(),
            PlaceState::new(side_place_id, "Side room"),
        );
        seed.actors_mut()
            .insert(actor_id.clone(), ActorBody::new(actor_id, from_place_id));
        let mut door = DoorState::new(
            DoorId::new("door_under_test").unwrap(),
            endpoint_a.clone(),
            endpoint_b.clone(),
        );
        door.is_open = true;
        seed.doors_mut().insert(door.door_id.clone(), door);
        seed.build()
    }

    fn rejection_reason_for_door(endpoint_a: &PlaceId, endpoint_b: &PlaceId) -> ReasonCode {
        let (_, _, to_place_id, _) = ids();
        build_move_event(
            &state_with_door(endpoint_a, endpoint_b),
            &proposal(),
            &ordering_key(&to_place_id),
            &ContentManifestId::new("phase1_manifest").unwrap(),
        )
        .unwrap_err()
        .reason_code
    }

    let (_, from_place_id, to_place_id, side_place_id) = ids();

    assert_eq!(
        rejection_reason_for_door(&from_place_id, &side_place_id),
        ReasonCode::NotAdjacent,
        "a door sharing only the origin endpoint must not authorize movement to another destination"
    );
    assert_eq!(
        rejection_reason_for_door(&side_place_id, &to_place_id),
        ReasonCode::NotAdjacent,
        "a door sharing only the destination endpoint must not authorize movement from another origin"
    );
    assert_eq!(
        rejection_reason_for_door(&side_place_id, &from_place_id),
        ReasonCode::NotAdjacent,
        "a reverse-branch door sharing only the origin endpoint must not authorize movement"
    );
    assert_eq!(
        rejection_reason_for_door(&to_place_id, &side_place_id),
        ReasonCode::NotAdjacent,
        "a reverse-branch door sharing only the destination endpoint must not authorize movement"
    );

    for (endpoint_a, endpoint_b) in [
        (&from_place_id, &to_place_id),
        (&to_place_id, &from_place_id),
    ] {
        let event = build_move_event(
            &state_with_door(endpoint_a, endpoint_b),
            &proposal(),
            &ordering_key(&to_place_id),
            &ContentManifestId::new("phase1_manifest").unwrap(),
        )
        .unwrap();

        assert_eq!(event.event_type, EventKind::ActorMoved);
        assert_eq!(event.proposal_id, Some(proposal().proposal_id));
        assert!(event.participants.contains(&from_place_id.to_string()));
        assert!(event.participants.contains(&to_place_id.to_string()));
    }
}

#[test]
fn user_origin_wait_keeps_candidate_goal_reevaluation_false() {
    use tracewake_core::actions::defs::wait::build_wait_events;
    use tracewake_core::actions::proposal::{Proposal, ProposalOrigin};
    use tracewake_core::agent::{NeedChangeCause, NeedKind, NeedState};
    use tracewake_core::ids::PlaceId;
    use tracewake_core::state::ActorBody;

    let actor_id = ActorId::new("actor_tomas").unwrap();
    let place_id = PlaceId::new("shop_front").unwrap();
    let mut physical_seed = PhysicalSeed::default();
    physical_seed
        .actors_mut()
        .insert(actor_id.clone(), ActorBody::new(actor_id.clone(), place_id));
    let physical_state = physical_seed.build();

    let mut agent_seed = AgentSeed::default();
    agent_seed.needs_by_actor_mut().insert(
        actor_id.clone(),
        BTreeMap::from([
            (
                NeedKind::Hunger,
                NeedState::initial(NeedKind::Hunger, 249, NeedChangeCause::TickDelta),
            ),
            (
                NeedKind::Fatigue,
                NeedState::initial(NeedKind::Fatigue, 10, NeedChangeCause::TickDelta),
            ),
        ]),
    );
    let agent_state = agent_seed.build();

    let mut proposal = Proposal::new(
        ProposalId::new("proposal_human_wait_threshold").unwrap(),
        ProposalOrigin::Human,
        Some(actor_id.clone()),
        ActionId::new("wait").unwrap(),
        SimTick::new(4),
    );
    proposal
        .parameters
        .insert("ticks".to_string(), "1".to_string());
    proposal
        .parameters
        .insert("reason".to_string(), "taking a moment".to_string());
    let ordering_key = OrderingKey::new(
        SimTick::new(4),
        SchedulePhase::HumanCommand,
        SchedulerSourceId::Actor(actor_id.clone()),
        ProposalSequence::new(0),
        ActionId::new("wait").unwrap(),
        vec!["1_tick".to_string()],
        "human-wait-origin-guard",
    );

    let events = build_wait_events(
        &physical_state,
        &agent_state,
        &proposal,
        &ordering_key,
        &ContentManifestId::new("phase1_manifest").unwrap(),
    )
    .unwrap();

    let wait_event = events
        .iter()
        .find(|event| event.event_type == EventKind::ActorWaited)
        .expect("wait event emitted");
    assert!(wait_event
        .payload
        .iter()
        .any(|field| field.key == "candidate_goal_reevaluation" && field.value == "false"));
    assert_eq!(wait_event.proposal_id, Some(proposal.proposal_id.clone()));
    assert!(events
        .iter()
        .any(|event| event.event_type == EventKind::NeedThresholdCrossed));
    assert!(events.iter().all(|event| event
        .causes
        .iter()
        .any(|cause| cause == &EventCause::Proposal(proposal.proposal_id.clone()))));

    let mut low_pressure_agent_seed = AgentSeed::default();
    low_pressure_agent_seed.needs_by_actor_mut().insert(
        actor_id.clone(),
        BTreeMap::from([
            (
                NeedKind::Hunger,
                NeedState::initial(NeedKind::Hunger, 10, NeedChangeCause::TickDelta),
            ),
            (
                NeedKind::Fatigue,
                NeedState::initial(NeedKind::Fatigue, 10, NeedChangeCause::TickDelta),
            ),
        ]),
    );
    let mut autonomous_proposal = proposal.clone();
    autonomous_proposal.proposal_id =
        ProposalId::new("proposal_scheduler_wait_no_threshold").unwrap();
    autonomous_proposal.origin = ProposalOrigin::Scheduler;
    let autonomous_events = build_wait_events(
        &physical_state,
        &low_pressure_agent_seed.build(),
        &autonomous_proposal,
        &ordering_key,
        &ContentManifestId::new("phase1_manifest").unwrap(),
    )
    .unwrap();
    let autonomous_wait = autonomous_events
        .iter()
        .find(|event| event.event_type == EventKind::ActorWaited)
        .expect("autonomous wait event emitted");

    assert!(autonomous_wait
        .payload
        .iter()
        .any(|field| field.key == "candidate_goal_reevaluation" && field.value == "false"));
    assert!(!autonomous_events
        .iter()
        .any(|event| event.event_type == EventKind::NeedThresholdCrossed));
    assert_eq!(
        autonomous_events
            .iter()
            .filter(|event| event.event_type == EventKind::NeedDeltaApplied)
            .count(),
        2
    );
}

#[test]
fn event_apply_remains_only_post_seed_mutation_path() {
    // Smoke-only source scan paired with the runtime append-before-apply proof
    // above and the negative fixture capability/seed-mutator checks.
    no_direct_apply_event_outside_event_replay_or_pipeline();
    accepted_action_appends_before_authoritative_apply();
}

#[test]
fn guard_006_scheduler_has_no_direct_routine_or_need_proposal_bypass() {
    let scheduler_sources = guarded_sources_for(GuardedLayer::Scheduler);
    for forbidden in [
        "build_routine_or_need_proposal",
        "eat_proposal",
        "sleep_proposal",
        "work_or_move_proposal",
        "ordinary_proposal",
        "current_hunger",
        "current_fatigue",
    ] {
        assert_absent_from_sources(&scheduler_sources, forbidden);
    }
}

#[test]
fn guard_006_duration_need_deltas_route_through_shared_emitter() {
    let violations = direct_duration_need_delta_construction_violations(&core_production_sources());
    assert!(
        violations.is_empty(),
        "action/scheduler need deltas must be constructed by need_events.rs: {violations:?}"
    );

    let synthetic_sources = vec![
        (
            "crates/tracewake-core/src/actions/defs/work.rs".to_string(),
            r#"
        fn build_synthetic_delta() -> EventEnvelope {
            EventEnvelope::new_caused_v1(
                event_id,
                EventKind::NeedDeltaApplied,
                0,
                0,
                tick,
                ordering_key,
                content_manifest_id,
                causes,
            )
        }
        "#
            .to_string(),
        ),
        (
            "crates/tracewake-core/src/actions/defs/eat.rs".to_string(),
            r#"
        fn build_synthetic_eat_delta() -> EventEnvelope {
            EventEnvelope::new_caused_v1(
                event_id,
                EventKind::NeedDeltaApplied,
                0,
                0,
                tick,
                ordering_key,
                content_manifest_id,
                causes,
            )
        }
        "#
            .to_string(),
        ),
    ];
    let synthetic_violations =
        direct_duration_need_delta_construction_violations(&synthetic_sources);
    assert!(
        synthetic_violations.len() == 2,
        "synthetic direct NeedDeltaApplied construction must fail this guard for every guarded action/scheduler source"
    );
}

fn direct_duration_need_delta_construction_violations(sources: &[(String, String)]) -> Vec<String> {
    sources
        .iter()
        .filter(|(path, source)| need_delta_guard_perimeter(path, source))
        .flat_map(|(path, source)| {
            let normalized = normalized_source(source);
            let mut violations = Vec::new();
            let mut search_start = 0;
            while let Some(relative_index) =
                normalized[search_start..].find("EventKind::NeedDeltaApplied")
            {
                let index = search_start + relative_index;
                let context_start = index.saturating_sub(300);
                let context = &normalized[context_start..index];
                if context.contains("EventEnvelope::new")
                    || context.contains("EventEnvelope :: new")
                {
                    violations.push(format!("{path}: direct NeedDeltaApplied construction"));
                }
                search_start = index + "EventKind::NeedDeltaApplied".len();
            }
            violations
        })
        .collect()
}

fn need_delta_guard_perimeter(path: &str, source: &str) -> bool {
    if path == "crates/tracewake-core/src/actions/defs/need_events.rs" {
        return false;
    }
    let guarded_layer = path == "crates/tracewake-core/src/scheduler.rs"
        || (path.starts_with("crates/tracewake-core/src/actions/defs/") && path.ends_with(".rs"));
    guarded_layer
        && (source.contains("EventKind::NeedDeltaApplied")
            || source.contains("NeedDeltaEventSpec")
            || source.contains("build_need_delta_and_threshold_events"))
}

#[test]
fn guard_006_scheduler_does_not_fabricate_empty_epistemic_projection() {
    let scheduler = guarded_source("src/scheduler.rs");
    assert!(
        scheduler.contains("NoHumanActorKnownSurfaceBuilder::from_projection"),
        "no-human cognition must use the projection-backed sealed actor-known surface builder"
    );
    assert!(
        scheduler.contains("fn epistemic_projection_from_log")
            && scheduler.contains("apply_epistemic_event(&mut projection, event)"),
        "scheduler projection construction must replay epistemic events instead of fabricating an empty projection"
    );
}

#[test]
fn guard_018_actor_known_facts_require_source_event_witness() {
    let actor_known_normalized = normalized_source(ACTOR_KNOWN_RS);
    assert!(
        ACTOR_KNOWN_RS.contains("pub struct SourceEventIds"),
        "actor-known facts must expose a typed source-event witness"
    );
    assert!(
        !ACTOR_KNOWN_RS.contains("pub fn with_source_event_ids"),
        "source ids must be supplied through SourceEventIds, not attached after construction"
    );
    assert!(
        !NO_HUMAN_SURFACE_RS.contains("pub fn with_role_assignment_notice"),
        "raw role-assignment convenience construction must stay deleted"
    );
    assert!(
        !NO_HUMAN_SURFACE_RS.contains("pub fn with_sleep_place_knowledge"),
        "raw sleep-place convenience construction must stay deleted"
    );
    assert!(
        actor_known_normalized.contains("source_event_ids: SourceEventIds"),
        "ActorKnownFact must store the typed source-event witness, not a raw Vec"
    );
    assert!(
        !actor_known_normalized.contains("source_event_ids: Vec < EventId >")
            && !actor_known_normalized.contains("source_event_ids: Vec<EventId>")
            && !actor_known_normalized.contains("source_event_ids: Vec::new")
            && !actor_known_normalized.contains("source_event_ids: vec!"),
        "ActorKnownFact must not store or construct raw/empty source-event id vectors"
    );
    assert!(
        TRANSACTION_RS.contains("fact.source_event_ids().is_empty()")
            && TRANSACTION_RS.contains("BlockerCode::ProvenanceDangling"),
        "transaction boundary must fail closed on empty or dangling actor-known provenance"
    );
}

#[test]
fn guard_018_witness_kind_no_human_fact_stable_ids_have_explicit_arms() {
    let surface = production(NO_HUMAN_SURFACE_RS);
    let transaction = production(TRANSACTION_RS);
    let witness_body = body_after_marker(&transaction, "fn witness_kind_allowed");
    let census = NO_HUMAN_SURFACE_FACT_STABLE_IDS
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();

    for stable_id in NO_HUMAN_SURFACE_FACT_STABLE_IDS {
        assert!(
            surface.contains(&format!("\"{stable_id}\"")),
            "no-human surface fact stable id census contains stale entry {stable_id}"
        );
        assert!(
            witness_body.contains(&format!("\"{stable_id}\"")),
            "fact stable id {stable_id} must have an explicit witness_kind_allowed arm"
        );
    }

    let minted_ids = minted_no_human_surface_fact_ids(&surface);
    let uncensused_ids = minted_ids
        .iter()
        .filter(|stable_id| !census.contains(stable_id.as_str()))
        .collect::<Vec<_>>();
    assert!(
        uncensused_ids.is_empty(),
        "no-human surface minted fact stable ids missing from census: {uncensused_ids:#?}"
    );

    let synthetic_surface = format!(
        "{surface}\nself.facts.push(ActorKnownFact::remembered_belief(self.actor_id.clone(), \"synthetic_uncensused_fact\", \"value\", \"source\", self.decision_tick, source_event_ids));\n"
    );
    let synthetic_minted_ids = minted_no_human_surface_fact_ids(&synthetic_surface);
    assert!(
        synthetic_minted_ids.contains("synthetic_uncensused_fact")
            && synthetic_minted_ids
                .iter()
                .any(|stable_id| !census.contains(stable_id.as_str())),
        "synthetic unregistered no-human fact mint must fail the reverse census direction"
    );

    assert!(
        witness_body.contains("_ => false"),
        "witness_kind_allowed must fail closed for unlisted fact stable ids"
    );
    assert!(
        !witness_body.contains("_ => true"),
        "witness_kind_allowed must not contain a wildcard-true arm"
    );
}

#[test]
fn guard_014_no_human_cognition_surface_does_not_read_raw_assignment_or_sleep_truth() {
    let scheduler = guarded_source("src/scheduler.rs");
    let builder = guarded_source("src/agent/no_human_surface.rs");
    let scheduler_sources = guarded_sources_for(GuardedLayer::Scheduler);
    let run_no_human_actor_decision_transaction =
        body_after_marker(&scheduler, "fn run_no_human_actor_decision_transaction");

    for forbidden in [
        "visible_local_planning_state",
        "state.workplaces",
        "state.food_supplies",
        "state.places",
        "BTreeSet::from([current_place_id",
        "sleep_place_believed_accessible",
        "actor_at_workplace",
        "assigned_workplace_known",
    ] {
        assert_absent(run_no_human_actor_decision_transaction, forbidden);
        assert_absent_from_sources(&scheduler_sources, forbidden);
    }

    assert!(
        builder.contains("pub struct SealedActorKnownSurface"),
        "no-human actor-known surface must be sealed"
    );
    assert!(
        builder.contains("pub struct NoHumanActorKnownSurfaceBuilder"),
        "no-human actor-known surface must be constructed through a named builder"
    );
    assert!(
        builder.contains("fn consume_projection_record"),
        "no-human actor-known surface must consume projection-backed actor-known records"
    );
    assert!(
        builder.contains("ActorKnownProjectionRecord::Workplace"),
        "no-human actor-known workplace facts must come from projection records"
    );
    assert!(
        builder.contains("ActorKnownProjectionRecord::SleepPlace"),
        "no-human actor-known starting beliefs must come from projection records"
    );
    assert_absent(&builder, "PhysicalState");
    assert_absent(&builder, "EventEnvelope");
    assert_absent(&builder, "EventKind::RoleAssignmentNoticeRecorded");
    assert_absent(&builder, "EventKind::StartingBeliefRecorded");
    assert_absent(&builder, "EventKind::ObservationRecorded");
    assert_absent(&builder, "payload_value(");
    assert_absent(&builder, "state.workplaces");
    assert_absent(&builder, "state.food_supplies");
    assert_absent(&builder, "state.sleep_affordances");
    assert_absent(&builder, "BTreeSet::from([current_place_id");
}

#[test]
fn guard_015_ord_hard_008_cognition_channel_stays_evented_and_sealed() {
    let scheduler = guarded_source("src/scheduler.rs");
    let builder = guarded_source("src/agent/no_human_surface.rs");
    let actor_known = guarded_source("src/agent/actor_known.rs");
    let scheduler_sources = guarded_sources_for(GuardedLayer::Scheduler);
    let agent_sources = guarded_sources_for(GuardedLayer::Agent);
    let run_no_human_actor_decision_transaction =
        body_after_marker(&scheduler, "fn run_no_human_actor_decision_transaction");
    let consume_projection_record = body_after_marker(&builder, "fn consume_projection_record");
    let push_projection_fact = body_after_marker(&builder, "fn push_projection_fact");

    for forbidden in [
        "PhysicalState",
        "state.workplaces",
        "state.workplaces()",
        "state.food_supplies",
        "state.food_supplies()",
        "state.sleep_affordances",
        "state.sleep_affordances()",
        "workplaces()",
        "food_supplies()",
        "sleep_affordances()",
    ] {
        assert_absent(&builder, forbidden);
    }

    for forbidden in ["extend_actor_known_facts", "add_actor_known_fact"] {
        assert_absent(run_no_human_actor_decision_transaction, forbidden);
        assert_absent_from_sources(&scheduler_sources, forbidden);
        assert_absent_from_sources(&agent_sources, forbidden);
    }
    assert_absent(
        run_no_human_actor_decision_transaction,
        "food_source_believed_accessible",
    );
    assert_absent_from_sources(&scheduler_sources, "food_source_believed_accessible");

    assert_absent(&actor_known, "pub fn extend_actor_known_facts");
    assert_absent(&actor_known, "pub fn add_actor_known_fact");
    assert!(builder.contains("pub fn from_projection("));
    assert!(builder.contains("fn consume_projection_records"));
    assert!(builder.contains("projection.classified_actor_known_records_for_context"));
    assert!(builder.contains("ActorKnownProjectionFreshness::CurrentlyPerceived"));
    assert_absent(consume_projection_record, "ActorKnownFact::observed_now");
    assert!(push_projection_fact.contains("ActorKnownFact::observed_now"));
    assert!(push_projection_fact.contains("ActorKnownFact::remembered_belief"));
    assert_absent(&builder, "pub fn from_event_log(");
    assert_absent(&builder, "fn consume_role_assignment_notice");
    assert_absent(&builder, "fn consume_starting_belief");
    assert_absent(&builder, "fn consume_observation");
}

#[test]
fn guard_014_embodied_projection_workplaces_are_context_backed() {
    let projection_sources = guarded_sources_for(GuardedLayer::Projections);
    let projection = guarded_source("src/projections.rs");
    let food_helper = body_after_marker(&projection, "fn actor_known_food_sources_for_context");
    let sleep_helper = body_after_marker(&projection, "fn visible_open_sleep_affordance");
    let phase3a_actions = body_after_marker(&projection, "fn phase3a_semantic_actions");
    let view_builder = body_after_marker(&projection, "pub fn build_embodied_view_model");

    assert!(
        projection.contains("fn actor_known_workplaces_for_context(")
            && projection.contains("context: &KnowledgeContext"),
        "embodied workplace affordances must be selected from the sealed holder-known context"
    );
    assert!(
        projection.contains(".actor_known_workplaces()"),
        "embodied projection must read typed context-backed workplace facts"
    );
    assert_absent(
        &projection,
        "workplace.assigned_actor_ids.is_empty()\n                || workplace.assigned_actor_ids.contains",
    );
    assert_absent(&projection, "actor_known_workplaces_for_context(state");
    assert!(
        projection.contains("fn actor_known_food_sources_for_context")
            && projection.contains("context: &KnowledgeContext"),
        "embodied food affordances must be selected from sealed holder-known food facts"
    );
    assert!(
        projection.contains("fn actor_known_sleep_affordances_for_context"),
        "embodied sleep affordances must be selected from sealed holder-known sleep facts"
    );
    assert!(
        projection.contains("fn actor_known_routes_for_context"),
        "embodied exits must be selected from sealed holder-known route facts"
    );
    assert_absent(food_helper, "state.food_supplies");
    assert_absent(food_helper, "food_supplies()");
    assert_absent(sleep_helper, "state.sleep_affordances");
    assert_absent(sleep_helper, "sleep_affordances()");
    assert_absent(phase3a_actions, "state.workplaces");
    assert_absent(phase3a_actions, "workplaces()");
    assert_absent(view_builder, ".adjacent_place_ids");
    assert_absent_from_sources(
        &projection_sources,
        "actor_known_workplaces_for_context(state",
    );
}

#[test]
fn guard_014_embodied_projection_source_has_no_physical_state_field() {
    fn source_shape_errors(source: &str) -> Vec<String> {
        let source_struct = source
            .split("pub struct EmbodiedProjectionSource<'a> {")
            .nth(1)
            .and_then(|tail| tail.split("\n}").next())
            .unwrap_or("");
        let sealed_source_builder = body_after_marker(source, "pub fn from_sealed_context");
        let view_builder = body_after_marker(source, "pub fn build_embodied_view_model");
        let semantic_actions = body_after_marker(source, "fn semantic_actions");
        let phase3a_semantic_actions = body_after_marker(source, "fn phase3a_semantic_actions");
        let mut errors = Vec::new();
        let has_state_field = source_struct
            .lines()
            .any(|line| line.trim_start().starts_with("state:"));
        if source_struct.contains("PhysicalState") || has_state_field {
            errors.push("EmbodiedProjectionSource must not carry PhysicalState".to_string());
        }
        if view_builder.contains("source.state") || view_builder.contains("visible_locality") {
            errors.push(
                "build_embodied_view_model must not read source.state or visible_locality"
                    .to_string(),
            );
        }
        if sealed_source_builder.contains("&PhysicalState")
            || view_builder.contains("&PhysicalState")
            || view_builder.contains("state.items")
            || view_builder.contains("state.places")
            || view_builder.contains("state.actors")
            || semantic_actions.contains("preflight.state.items")
            || semantic_actions.contains("preflight.state.places")
            || semantic_actions.contains("preflight.state.actors")
            || phase3a_semantic_actions.contains("preflight.state.items")
            || phase3a_semantic_actions.contains("preflight.state.places")
            || phase3a_semantic_actions.contains("preflight.state.actors")
        {
            errors.push(
                "embodied projection builders must not read raw PhysicalState truth".to_string(),
            );
        }
        let carrier_snippets = [
            "pub fn from_physical_state(context: &KnowledgeContext, state: &PhysicalState)",
            "pub struct EmbodiedPreflightSource<'a> {\n    state: &'a PhysicalState,",
            "struct SemanticActionPreflightContext<'a> {\n    state: &'a PhysicalState,",
        ];
        for snippet in carrier_snippets {
            if !source.contains(snippet) {
                errors.push(format!("missing enrolled PhysicalState carrier: {snippet}"));
            }
        }
        let carrier_count = source
            .matches(
                "pub fn from_physical_state(context: &KnowledgeContext, state: &PhysicalState)",
            )
            .count()
            + source.matches("{\n    state: &'a PhysicalState,").count();
        if carrier_count != carrier_snippets.len() {
            errors.push(format!(
                "PhysicalState carrier census drifted: expected {} enrolled snippets, found {carrier_count}",
                carrier_snippets.len()
            ));
        }
        errors
    }

    let projection = guarded_source("src/projections.rs");
    let errors = source_shape_errors(&projection);
    assert!(errors.is_empty(), "{errors:?}");

    let synthetic = projection.replace(
        "pub struct EmbodiedProjectionSource<'a> {\n    agent_state: Option<&'a AgentState>,",
        "pub struct EmbodiedProjectionSource<'a> {\n    state: &'a PhysicalState,\n    agent_state: Option<&'a AgentState>,",
    );
    let synthetic_errors = source_shape_errors(&synthetic);
    assert!(
        synthetic_errors
            .iter()
            .any(|error| error.contains("PhysicalState")),
        "synthetic_embodied_projection_source_physical_state_field did not trigger: {synthetic_errors:?}"
    );

    let synthetic_builder_read = projection.replace(
        "let carried_items = source.carried_items.clone();",
        "let _synthetic_items = state.items.values().count();\n    let carried_items = source.carried_items.clone();",
    );
    let synthetic_builder_errors = source_shape_errors(&synthetic_builder_read);
    assert!(
        synthetic_builder_errors
            .iter()
            .any(|error| error.contains("raw PhysicalState truth")),
        "synthetic_embodied_view_builder_state_items_read did not trigger: {synthetic_builder_errors:?}"
    );

    let synthetic_semantic_read = projection.replace(
        "let mut actions = Vec::new();\n    actions.push(with_validator_availability(",
        "let _synthetic_items = preflight.state.items.values().count();\n    let mut actions = Vec::new();\n    actions.push(with_validator_availability(",
    );
    let synthetic_semantic_errors = source_shape_errors(&synthetic_semantic_read);
    assert!(
        synthetic_semantic_errors
            .iter()
            .any(|error| error.contains("raw PhysicalState truth")),
        "synthetic_semantic_actions_preflight_state_items_read did not trigger: {synthetic_semantic_errors:?}"
    );

    let synthetic_carrier = projection.replace(
        "pub struct EmbodiedTruthSnapshot {",
        "struct SyntheticTruthCarrier<'a> {\n    state: &'a PhysicalState,\n}\n\npub struct EmbodiedTruthSnapshot {",
    );
    let synthetic_carrier_errors = source_shape_errors(&synthetic_carrier);
    assert!(
        synthetic_carrier_errors
            .iter()
            .any(|error| error.contains("carrier census drifted")),
        "synthetic_new_physical_state_carrier did not trigger: {synthetic_carrier_errors:?}"
    );
}

#[test]
fn embodied_view_option_and_collection_fields_have_reachable_producers() {
    let projection = production(PROJECTIONS_RS);
    let view_models = production(VIEW_MODELS_RS);
    let tui_app = production(TUI_APP_RS);
    let tui_render = production(TUI_RENDER_RS);
    let producer_sources = [
        ("tracewake-core/src/projections.rs", projection.as_str()),
        ("tracewake-core/src/view_models.rs", view_models.as_str()),
        ("tracewake-tui/src/app.rs", tui_app.as_str()),
    ];
    let consumer_sources = [
        ("tracewake-tui/src/render.rs", tui_render.as_str()),
        ("tracewake-tui/src/app.rs", tui_app.as_str()),
    ];

    let errors =
        embodied_surface_dead_field_errors(&view_models, &producer_sources, &consumer_sources);
    assert!(
        errors.is_empty(),
        "dead embodied surface field sweep found missing producers: {errors:#?}"
    );

    let synthetic_view_models = r#"
        pub struct EmbodiedViewModel {
            pub visible_exits: Vec<VisibleExit>,
            pub synthetic_dead_field: Option<String>,
        }
        pub struct Phase3AEmbodiedStatus { pub salient_interruption: Option<String>, }
        pub struct WhyNotView { pub actor_visible_facts: Vec<String>, }
        pub struct NotebookView { pub typed_leads: Vec<NotebookLeadEntry>, }
        pub struct NotebookBeliefEntry { pub contradiction_ids: Vec<String>, }
        pub struct NotebookObservationEntry { pub observation_id: String, }
        pub struct NotebookContradictionEntry { pub contradiction_id: String, }
        pub struct NotebookLeadEntry { pub lead_id: String, }
        pub struct VisibleExit { pub blocker_summary: Option<String>, }
        pub struct VisibleDoor { pub endpoint_a: PlaceId, pub endpoint_b: PlaceId, }
        pub struct VisibleContainer { pub container_id: ContainerId, }
        pub struct VisibleItem { pub source: VisibleItemSource, }
        pub struct SemanticActionEntry { pub target_ids: Vec<String>, }
        pub enum ActionAvailability { Available, Disabled { debug_only_diagnostics: Vec<String>, } }
    "#;
    let synthetic_projection = r#"
        fn build() -> EmbodiedViewModel {
            EmbodiedViewModel {
                visible_exits: collect_visible_exits(),
                synthetic_dead_field: None,
            }
        }
    "#;
    let synthetic_sources = [("tracewake-core/src/projections.rs", synthetic_projection)];
    let synthetic_consumers = [("tracewake-tui/src/render.rs", "view.visible_exits.len()")];
    let synthetic_errors = embodied_surface_dead_field_errors(
        synthetic_view_models,
        &synthetic_sources,
        &synthetic_consumers,
    );
    assert!(
        synthetic_errors
            .iter()
            .any(|error| error.contains("synthetic_dead_field")),
        "synthetic hardwired default embodied field must fail the sweep"
    );

    let unmatched = std::panic::catch_unwind(|| {
        embodied_surface_fields("pub enum ActionAvailability { Available }")
    });
    assert!(
        unmatched.is_err(),
        "unmatched listed embodied struct names must fail instead of being skipped"
    );

    let enum_view_models = r#"
        pub struct EmbodiedViewModel {
            pub visible_exits: Vec<VisibleExit>,
        }
        pub struct Phase3AEmbodiedStatus { pub salient_interruption: Option<String>, }
        pub struct WhyNotView { pub actor_visible_facts: Vec<String>, }
        pub struct NotebookView { pub typed_leads: Vec<NotebookLeadEntry>, }
        pub struct NotebookBeliefEntry { pub contradiction_ids: Vec<String>, }
        pub struct NotebookObservationEntry { pub observation_id: String, }
        pub struct NotebookContradictionEntry { pub contradiction_id: String, }
        pub struct NotebookLeadEntry { pub lead_id: String, }
        pub struct VisibleExit { pub blocker_summary: Option<String>, }
        pub struct VisibleDoor { pub endpoint_a: PlaceId, pub endpoint_b: PlaceId, }
        pub struct VisibleContainer { pub container_id: ContainerId, }
        pub struct VisibleItem { pub source: VisibleItemSource, }
        pub struct SemanticActionEntry { pub target_ids: Vec<String>, }
        pub enum ActionAvailability {
            Available,
            Disabled { debug_only_diagnostics: Vec<String>, }
        }
    "#;
    let enum_fields = embodied_surface_fields(enum_view_models);
    assert!(
        enum_fields.iter().any(|field| {
            field.struct_name == "ActionAvailability"
                && field.field_name == "debug_only_diagnostics"
        }),
        "enum fields must be enrolled in the embodied surface sweep"
    );

    let cross_struct_view_models = r#"
        pub struct EmbodiedViewModel { pub visible_exits: Vec<VisibleExit>, }
        pub struct Phase3AEmbodiedStatus { pub salient_interruption: Option<String>, }
        pub struct WhyNotView { pub actor_visible_facts: Vec<String>, }
        pub struct NotebookView { pub typed_leads: Vec<NotebookLeadEntry>, }
        pub struct NotebookBeliefEntry { pub contradiction_ids: Vec<String>, }
        pub struct NotebookObservationEntry { pub observation_id: String, }
        pub struct NotebookContradictionEntry { pub contradiction_id: String, }
        pub struct NotebookLeadEntry { pub lead_id: String, }
        pub struct VisibleExit { pub blocker_summary: Option<String>, }
        pub struct VisibleDoor { pub endpoint_a: PlaceId, pub endpoint_b: PlaceId, }
        pub struct VisibleContainer { pub container_id: ContainerId, }
        pub struct VisibleItem { pub source: VisibleItemSource, }
        pub struct SemanticActionEntry { pub target_ids: Vec<String>, }
        pub enum ActionAvailability { Available, Disabled { debug_only_diagnostics: Vec<String>, } }
    "#;
    let cross_struct_sources = [(
        "tracewake-core/src/projections.rs",
        "OtherStruct { typed_leads: vec![lead] }",
    )];
    let cross_struct_consumers = [(
        "tracewake-tui/src/render.rs",
        "view.visible_exits.len(); status.salient_interruption.as_ref(); why.actor_visible_facts.len(); notebook.typed_leads.len(); belief.contradiction_ids.len(); observation.observation_id.len(); contradiction.contradiction_id.len(); lead.lead_id.len(); exit.blocker_summary.as_ref(); door.endpoint_a.as_str(); door.endpoint_b.as_str(); item.source.clone(); action.target_ids.len(); availability.debug_only_diagnostics()",
    )];
    let cross_struct_errors = embodied_surface_dead_field_errors(
        cross_struct_view_models,
        &cross_struct_sources,
        &cross_struct_consumers,
    );
    assert!(
        cross_struct_errors
            .iter()
            .any(|error| error.contains("NotebookView.typed_leads")),
        "producer aliases from the wrong struct must not satisfy the scoped producer sweep"
    );

    let constant_producer_view_models = r#"
        pub struct EmbodiedViewModel {
            pub visible_exits: Vec<VisibleExit>,
            pub debug_available: bool,
        }
        pub struct Phase3AEmbodiedStatus { pub salient_interruption: Option<String>, }
        pub struct WhyNotView { pub actor_visible_facts: Vec<String>, }
        pub struct NotebookView { pub typed_leads: Vec<NotebookLeadEntry>, }
        pub struct NotebookBeliefEntry { pub contradiction_ids: Vec<String>, }
        pub struct NotebookObservationEntry { pub observation_id: String, }
        pub struct NotebookContradictionEntry { pub contradiction_id: String, }
        pub struct NotebookLeadEntry { pub lead_id: String, }
        pub struct VisibleExit { pub blocker_summary: Option<String>, }
        pub struct VisibleDoor { pub endpoint_a: PlaceId, pub endpoint_b: PlaceId, }
        pub struct VisibleContainer { pub container_id: ContainerId, }
        pub struct VisibleItem { pub source: VisibleItemSource, }
        pub struct SemanticActionEntry { pub target_ids: Vec<String>, }
        pub enum ActionAvailability { Available, Disabled { debug_only_diagnostics: Vec<String>, } }
    "#;
    let constant_producer_sources = [(
        "tracewake-core/src/projections.rs",
        "EmbodiedViewModel { visible_exits: collect_visible_exits(), debug_available: true }",
    )];
    let constant_producer_consumers = [(
        "tracewake-tui/src/render.rs",
        "view.visible_exits.len(); view.debug_available; exit.blocker_summary.as_ref(); door.endpoint_a.as_str(); door.endpoint_b.as_str(); item.source.clone(); action.target_ids.len(); availability.debug_only_diagnostics()",
    )];
    let constant_producer_errors = embodied_surface_dead_field_errors(
        constant_producer_view_models,
        &constant_producer_sources,
        &constant_producer_consumers,
    );
    assert!(
        constant_producer_errors
            .iter()
            .any(|error| error.contains("EmbodiedViewModel.debug_available")),
        "constant literal producer must not satisfy the embodied surface producer sweep"
    );

    let synthetic_orphaned_deferral_view_models = r#"
        pub struct EmbodiedViewModel { pub visible_exits: Vec<VisibleExit>, }
        pub struct Phase3AEmbodiedStatus { pub salient_interruption: Option<String>, }
        pub struct WhyNotView { pub actor_visible_facts: Vec<String>, }
        pub struct NotebookView { pub typed_leads: Vec<NotebookLeadEntry>, }
        pub struct NotebookBeliefEntry { pub contradiction_ids: Vec<String>, }
        pub struct NotebookObservationEntry { pub observation_id: String, }
        pub struct NotebookContradictionEntry { pub contradiction_id: String, }
        pub struct NotebookLeadEntry { pub lead_id: String, }
        pub struct VisibleExit { pub blocker_summary: Option<String>, }
        pub struct VisibleDoor { pub endpoint_a: PlaceId, pub endpoint_b: PlaceId, }
        pub struct VisibleContainer { pub container_id: ContainerId, }
        pub struct VisibleItem { pub source: VisibleItemSource, }
        pub struct SemanticActionEntry { pub target_ids: Vec<String>, }
        pub enum ActionAvailability { Available, Disabled { debug_only_diagnostics: Vec<String>, } }
    "#;
    let synthetic_orphaned_deferral_errors = embodied_surface_dead_field_errors(
        synthetic_orphaned_deferral_view_models,
        &[(
            "tracewake-core/src/view_models.rs",
            "pub enum ActionAvailability { Available }",
        )],
        &[(
            "tracewake-tui/src/render.rs",
            "availability.debug_only_diagnostics()",
        )],
    );
    assert!(
        synthetic_orphaned_deferral_errors
            .iter()
            .any(|error| error.contains("ActionAvailability.debug_only_diagnostics")),
        "synthetic_orphaned_deferral_embodied_surface_producer must fail the cite-only producer witness"
    );

    let unconsumed_sources = [(
        "tracewake-core/src/projections.rs",
        "EmbodiedViewModel { visible_exits: collect_visible_exits() }",
    )];
    let unconsumed_errors = embodied_surface_dead_field_errors(
        cross_struct_view_models,
        &unconsumed_sources,
        &[("tracewake-tui/src/render.rs", "")],
    );
    assert!(
        unconsumed_errors
            .iter()
            .any(|error| error.contains("has no embodied render/app consumer")),
        "produced-but-unconsumed fields must fail the two-sided sweep"
    );
}

#[test]
fn guard_014_phase3a_semantic_actions_do_not_use_literal_true_availability() {
    let projection = production(PROJECTIONS_RS);
    let phase3a_actions = body_after_marker(&projection, "fn phase3a_semantic_actions");

    assert_absent(phase3a_actions, "SemanticActionEntry::new(");
    assert!(
        phase3a_actions.contains("SemanticActionEntry::with_availability("),
        "Phase 3A actions must carry explicit availability evidence instead of literal true"
    );
}

#[test]
fn guard_014_no_human_metrics_do_not_scan_display_text() {
    let projection_sources = guarded_sources_for(GuardedLayer::Projections);
    let projection = guarded_source("src/projections.rs");
    let metrics_body = body_after_marker(&projection, "pub fn no_human_day_metrics");

    for forbidden in [
        ".contains(\"planner\")",
        ".contains(\"failed\")",
        ".contains(\"planner_budget_exhausted\")",
    ] {
        assert_absent(metrics_body, forbidden);
        assert_absent_from_sources(&projection_sources, forbidden);
    }
    assert!(
        projection.contains("fn is_typed_planner_failure_event"),
        "no-human metrics must classify planner failures through typed diagnostic fields"
    );
    assert!(
        projection.contains("typed_responsible_layer") && projection.contains("typed_blocker_code"),
        "no-human metrics must read responsible_layer and blocker_code"
    );
}

#[test]
fn guard_014_perception_visibility_uses_typed_place_visibility() {
    let perception = guarded_source("src/agent/perception.rs");
    let violations = perception_visibility_prose_branch_violations(&perception);
    assert!(
        violations.is_empty(),
        "perception visibility must read typed place visibility, not prose or id substrings: {violations:?}"
    );

    let synthetic = r#"
        fn is_visible_exit_target(state: &PhysicalState, place_id: &PlaceId) -> bool {
            let Some(place) = state.places().get(place_id) else {
                return false;
            };
            !place.place_id.as_str().contains("hidden")
                && !place.display_label.to_lowercase().contains("hidden")
        }
    "#;
    let synthetic_violations = perception_visibility_prose_branch_violations(synthetic);
    assert!(
        synthetic_violations
            .iter()
            .any(|violation| violation.contains("place.place_id.as_str().contains")),
        "synthetic id-substring visibility branch must fail this guard"
    );
    assert!(
        synthetic_violations
            .iter()
            .any(|violation| violation.contains("display_label")),
        "synthetic prose/id visibility branch must fail this guard"
    );

    let other_emission_path_synthetic = r#"
        fn current_place_perception_events(
            state: &PhysicalState,
            actor_id: &ActorId,
            decision_tick: SimTick,
            content_manifest_id: &ContentManifestId,
        ) -> Vec<EventEnvelope> {
            let Some(place) = state.places().get(&current_place_id) else {
                return Vec::new();
            };
            let label_payload = PayloadField::string("place_label", place.display_label.clone());
            if place.display_label.to_lowercase().contains("hidden") {
                return Vec::new();
            }
            observation_event(actor_id, decision_tick, 0, content_manifest_id, label_payload)
        }
    "#;
    let other_emission_path_violations =
        perception_visibility_prose_branch_violations(other_emission_path_synthetic);
    assert!(
        other_emission_path_violations
            .iter()
            .any(
                |violation| violation.contains("current_place_perception_events")
                    && violation.contains("display_label")
            ),
        "synthetic prose branch outside is_visible_exit_target must fail this guard"
    );

    let synthetic_laundered_display_label_branch = r#"
        fn is_visible_exit_target(state: &PhysicalState, place_id: &PlaceId) -> bool {
            let Some(place) = state.places().get(place_id) else {
                return false;
            };
            let visible = place.display_label != "Hidden room";
            if visible {
                return true;
            }
            false
        }
    "#;
    let synthetic_laundered_violations =
        perception_visibility_prose_branch_violations(synthetic_laundered_display_label_branch);
    assert!(
        synthetic_laundered_violations
            .iter()
            .any(|violation| violation.contains("let visible = place.display_label")),
        "synthetic_display_label_binding_laundering must fail before the later branch"
    );

    let synthetic_bare_string_id_substring = r#"
        fn is_visible_exit_target(place: &PlaceState) -> bool {
            !place.display_label.starts_with("hid")
                && !place.place_id.ends_with("_hidden")
        }
    "#;
    let synthetic_bare_string_violations =
        perception_visibility_prose_branch_violations(synthetic_bare_string_id_substring);
    assert!(
        synthetic_bare_string_violations
            .iter()
            .any(|violation| violation.contains("display_label.starts_with")),
        "synthetic_bare_display_label_starts_with must fail without an as_str() call"
    );
    assert!(
        synthetic_bare_string_violations
            .iter()
            .any(|violation| violation.contains("place_id.ends_with")),
        "synthetic_bare_id_ends_with must fail without an as_str() call"
    );

    let renamed_parameter_helper_synthetic = r#"
        fn label_blocks_visibility(candidate: &str) -> bool {
            candidate.to_ascii_lowercase().contains("hidden")
        }

        fn is_visible_exit_target(place: &PlaceState) -> bool {
            !label_blocks_visibility(place.display_label.as_str())
        }
    "#;
    let renamed_parameter_helper_violations =
        perception_visibility_prose_branch_violations(renamed_parameter_helper_synthetic);
    assert!(
        renamed_parameter_helper_violations
            .iter()
            .any(
                |violation| violation.contains("label_blocks_visibility(place.display_label")
                    && violation
                        .contains("rule_family=line_calls_sensitive_helper_with_tainted_argument")
            ),
        "synthetic renamed-parameter display-label helper laundering must fail"
    );

    let payload_value_relay_synthetic = r#"
        fn current_place_perception_events(place: &PlaceState) -> Vec<EventEnvelope> {
            let label_payload = PayloadField::string("place_label", place.display_label.clone());
            let label_value = label_payload.value.as_str();
            if label_value.contains("hidden") {
                return Vec::new();
            }
            vec![]
        }
    "#;
    let payload_value_relay_violations =
        perception_visibility_prose_branch_violations(payload_value_relay_synthetic);
    assert!(
        payload_value_relay_violations
            .iter()
            .any(|violation| violation.contains("label_value.contains")
                && violation.contains("rule_family=branches_on_tainted_binding")),
        "synthetic payload-value relay branch must fail after the typed payload sink"
    );

    let provenance_only_helper_synthetic = r#"
        fn gate(candidate: &str) -> bool {
            candidate.starts_with("vault")
        }

        fn is_visible_exit_target(place: &PlaceState) -> bool {
            let label = place.display_label.as_str();
            !gate(label)
        }
    "#;
    let provenance_only_violations =
        perception_visibility_prose_branch_violations(provenance_only_helper_synthetic);
    assert!(
        provenance_only_violations
            .iter()
            .any(|violation| violation.contains("gate(label)")
                && violation
                    .contains("rule_family=line_calls_sensitive_helper_with_tainted_argument")),
        "provenance-only helper laundering must fail through the tainted-argument rule"
    );
    assert!(
        perception_visibility_prose_branch_violations_without_provenance(
            provenance_only_helper_synthetic
        )
        .is_empty(),
        "disabling provenance must make the provenance-only helper synthetic pass"
    );
}

fn perception_visibility_prose_branch_violations(source: &str) -> Vec<String> {
    perception_visibility_prose_branch_violations_with_options(source, true)
}

fn perception_visibility_prose_branch_violations_without_provenance(source: &str) -> Vec<String> {
    perception_visibility_prose_branch_violations_with_options(source, false)
}

fn perception_visibility_prose_branch_violations_with_options(
    source: &str,
    provenance_enabled: bool,
) -> Vec<String> {
    let stripped = source_without_comments(source);
    let sensitive_params = if provenance_enabled {
        perception_sensitive_helper_params(&stripped)
    } else {
        BTreeMap::new()
    };
    let mut violations = Vec::new();
    let mut current_fn = "module";
    let mut tainted_bindings_by_fn = BTreeMap::<String, BTreeSet<String>>::new();
    for line in stripped.lines().map(str::trim) {
        if let Some(function_name) = function_name_from_line(line) {
            current_fn = function_name;
        }
        let current_taints = tainted_bindings_by_fn
            .entry(current_fn.to_string())
            .or_default();
        if let Some(alias) = perception_tainted_let_alias(line, current_taints) {
            current_taints.insert(alias);
        }
        if line.is_empty() {
            continue;
        }
        if perception_line_is_typed_label_payload_write(line) {
            continue;
        }
        if perception_line_is_current_place_label_recording(line) {
            continue;
        }
        let branches_on_display_label =
            line.contains("display_label") && !line_is_plain_perception_identity_alias(line);
        let branches_on_id_substring = branches_on_identity_substring(line);
        let branches_on_hidden_prose = line.contains(".contains(\"hidden\")")
            || line.contains(".to_lowercase()")
            || line.contains(".to_ascii_lowercase()");
        let branches_on_tainted_binding =
            branches_on_hidden_prose && line_mentions_any_binding(line, current_taints);
        let launders_into_sensitive_helper = line_calls_sensitive_helper_with_tainted_argument(
            line,
            current_taints,
            &sensitive_params,
        );
        let rule_family = if launders_into_sensitive_helper {
            Some("line_calls_sensitive_helper_with_tainted_argument")
        } else if branches_on_tainted_binding {
            Some("branches_on_tainted_binding")
        } else if branches_on_display_label {
            Some("branches_on_display_label")
        } else if branches_on_id_substring {
            Some("branches_on_identity_substring")
        } else if branches_on_hidden_prose {
            Some("branches_on_hidden_prose")
        } else {
            None
        };
        if let Some(rule_family) = rule_family {
            violations.push(format!("{current_fn}: {line} [rule_family={rule_family}]"));
        }
    }
    violations
}

fn perception_line_is_typed_label_payload_write(line: &str) -> bool {
    line.contains("PayloadField") && line.contains("display_label")
}

fn perception_line_is_current_place_label_recording(line: &str) -> bool {
    matches!(
        line,
        "display_label,"
            | "\"display_label\","
            | "display_label.clone(),"
            | "place.display_label.clone(),"
    )
}

fn line_is_plain_perception_identity_alias(line: &str) -> bool {
    let Some(let_index) = line.find("let ") else {
        return false;
    };
    let rest = &line[let_index + "let ".len()..];
    let Some((left, right)) = rest.split_once('=') else {
        return false;
    };
    if let_binding_name(left).is_none() {
        return false;
    }
    let rhs = right.trim().trim_end_matches(';').trim();
    rhs_contains_perception_identity_source(rhs)
        && ![
            "==",
            "!=",
            ".contains(",
            ".starts_with(",
            ".ends_with(",
            ".to_lowercase()",
            ".to_ascii_lowercase()",
        ]
        .iter()
        .any(|branch_token| rhs.contains(branch_token))
}

fn branches_on_identity_substring(line: &str) -> bool {
    [".contains(", ".starts_with(", ".ends_with("]
        .iter()
        .any(|method| {
            line.contains(method)
                && (line.contains("display_label")
                    || line.contains("_id")
                    || line.contains(".id")
                    || line.contains("Id"))
        })
}

fn perception_tainted_let_alias(line: &str, tainted_bindings: &BTreeSet<String>) -> Option<String> {
    let let_index = line.find("let ")?;
    let rest = &line[let_index + "let ".len()..];
    let (left, right) = rest.split_once('=')?;
    let alias = let_binding_name(left)?;
    let rhs = right.trim_end_matches(';');
    (rhs_contains_perception_identity_source(rhs)
        || line_mentions_any_binding(rhs, tainted_bindings))
    .then_some(alias)
}

fn rhs_contains_perception_identity_source(rhs: &str) -> bool {
    rhs.contains("display_label")
        || rhs.contains(".place_id")
        || rhs.contains(".actor_id")
        || rhs.contains(".container_id")
        || rhs.contains(".item_id")
        || rhs.contains(".food_supply_id")
        || rhs.contains(".workplace_id")
        || rhs.contains("_id.as_str()")
}

fn line_mentions_any_binding(line: &str, bindings: &BTreeSet<String>) -> bool {
    bindings.iter().any(|binding| token_appears(line, binding))
}

fn token_appears(line: &str, token: &str) -> bool {
    let bytes = line.as_bytes();
    let token_bytes = token.as_bytes();
    if token_bytes.is_empty() {
        return false;
    }
    let mut offset = 0;
    while let Some(found) = line[offset..].find(token) {
        let start = offset + found;
        let end = start + token_bytes.len();
        let before = start
            .checked_sub(1)
            .and_then(|index| bytes.get(index))
            .is_some_and(|byte| byte.is_ascii_alphanumeric() || *byte == b'_');
        let after = bytes
            .get(end)
            .is_some_and(|byte| byte.is_ascii_alphanumeric() || *byte == b'_');
        if !before && !after {
            return true;
        }
        offset = end;
    }
    false
}

fn perception_sensitive_helper_params(source: &str) -> BTreeMap<String, BTreeSet<String>> {
    let mut function_params = BTreeMap::<String, BTreeSet<String>>::new();
    let mut branching_params = BTreeMap::<String, BTreeSet<String>>::new();
    let mut current_fn = "module";
    let mut current_params = BTreeSet::<String>::new();
    for line in source.lines().map(str::trim) {
        if let Some(function_name) = function_name_from_line(line) {
            current_fn = function_name;
            current_params = function_param_names_from_line(line);
            function_params.insert(current_fn.to_string(), current_params.clone());
        }
        if line_has_branching_string_discriminator(line) {
            for param in current_params
                .iter()
                .filter(|param| token_appears(line, param))
            {
                branching_params
                    .entry(current_fn.to_string())
                    .or_default()
                    .insert(param.clone());
            }
        }
    }

    let mut sensitive = BTreeMap::<String, BTreeSet<String>>::new();
    let mut current_fn = "module";
    let mut tainted_bindings_by_fn = BTreeMap::<String, BTreeSet<String>>::new();
    for line in source.lines().map(str::trim) {
        if let Some(function_name) = function_name_from_line(line) {
            current_fn = function_name;
        }
        let current_taints = tainted_bindings_by_fn
            .entry(current_fn.to_string())
            .or_default();
        if let Some(alias) = perception_tainted_let_alias(line, current_taints) {
            current_taints.insert(alias);
        }
        for (helper_name, helper_branching_params) in &branching_params {
            let call_start = format!("{helper_name}(");
            let Some(start) = line.find(&call_start) else {
                continue;
            };
            let args = &line[start + call_start.len()..];
            if rhs_contains_perception_identity_source(args)
                || line_mentions_any_binding(args, current_taints)
            {
                let params = function_params
                    .get(helper_name)
                    .cloned()
                    .unwrap_or_default();
                let selected = if params.is_empty() {
                    helper_branching_params.clone()
                } else {
                    helper_branching_params
                        .intersection(&params)
                        .cloned()
                        .collect()
                };
                sensitive
                    .entry(helper_name.clone())
                    .or_default()
                    .extend(selected);
            }
        }
    }
    sensitive
}

fn line_has_branching_string_discriminator(line: &str) -> bool {
    line.contains(".contains(")
        || line.contains(".starts_with(")
        || line.contains(".ends_with(")
        || line.contains(".to_lowercase()")
        || line.contains(".to_ascii_lowercase()")
}

fn line_calls_sensitive_helper_with_tainted_argument(
    line: &str,
    tainted_bindings: &BTreeSet<String>,
    sensitive_params: &BTreeMap<String, BTreeSet<String>>,
) -> bool {
    sensitive_params.keys().any(|function_name| {
        let call_start = format!("{function_name}(");
        let Some(start) = line.find(&call_start) else {
            return false;
        };
        let args = &line[start + call_start.len()..];
        rhs_contains_perception_identity_source(args)
            || line_mentions_any_binding(args, tainted_bindings)
    })
}

fn function_param_names_from_line(line: &str) -> BTreeSet<String> {
    let Some(open) = line.find('(') else {
        return BTreeSet::new();
    };
    let Some(close) = line[open + 1..].find(')') else {
        return BTreeSet::new();
    };
    line[open + 1..open + 1 + close]
        .split(',')
        .filter_map(|param| param.split_once(':').map(|(name, _)| name.trim()))
        .filter_map(let_binding_name)
        .collect()
}

fn function_name_from_line(line: &str) -> Option<&str> {
    let rest = line
        .strip_prefix("fn ")
        .or_else(|| line.strip_prefix("pub fn "))?;
    let name_end = rest.find('(')?;
    Some(rest[..name_end].trim())
}

#[test]
fn guard_011_no_human_day_runner_only_evidence() {
    let errors = no_human_day_runner_only_evidence_errors(CONTENT_GOLDEN_FIXTURES_RUN_RS);
    assert!(
        errors.is_empty(),
        "canonical no-human day evidence must require runner-only ancestry:\n{}",
        errors.join("\n")
    );

    let synthetic = CONTENT_GOLDEN_FIXTURES_RUN_RS.replace(
        "has_no_human_event(&log, EventKind::WorkBlockCompleted)",
        "has_event(&log, EventKind::WorkBlockCompleted)",
    );
    assert!(
        no_human_day_runner_only_evidence_errors(&synthetic)
            .iter()
            .any(|error| error.contains("WorkBlockCompleted")),
        "synthetic runner-only work completion ancestry removal must fail"
    );
}

fn no_human_day_runner_only_evidence_errors(source: &str) -> Vec<String> {
    let test_body = body_after_marker(
        source,
        "fn no_human_day_fixture_has_roster_activity_and_metrics_envelope",
    );
    let runner_only_section = test_body
        .split("proposal_day_tomas_eat")
        .next()
        .unwrap_or(test_body);
    let mut errors = Vec::new();
    for (kind, snippet) in [
        (
            "WorkBlockCompleted",
            "has_no_human_event(&log, EventKind::WorkBlockCompleted)",
        ),
        (
            "SleepCompleted",
            "has_event(&log, EventKind::SleepCompleted)",
        ),
    ] {
        if !runner_only_section.contains(snippet) {
            errors.push(format!(
                "runner-only section lacks required assertion for {kind}"
            ));
        }
    }
    if !(runner_only_section.contains("has_no_human_event_for_actor")
        && runner_only_section.contains("EventKind::EatFailed")
        && runner_only_section.contains("\"actor_mara\""))
    {
        errors.push(
            "runner-only section lacks explicit Mara fail-only canonical resolution".to_string(),
        );
    }
    errors
}

#[test]
fn guard_014_sleep_validation_requires_modeled_affordance() {
    let sleep = production(SLEEP_RS);
    let projection = production(PROJECTIONS_RS);
    let routine_continuation = production(ROUTINE_CONTINUATION_RS);
    let builder = production(NO_HUMAN_SURFACE_RS);

    assert!(
        sleep.contains("sleep_affordance_id"),
        "sleep start validation must require a concrete modeled rest-surface id"
    );
    assert!(
        sleep.contains("ReasonCode::NoSleepAffordance"),
        "sleep validation must reject absent, forged, closed, or stale rest surfaces with a typed reason"
    );
    assert_absent(&sleep, "sleep_place_id != actor.current_place_id.as_str()");
    assert!(
        projection.contains("visible_open_sleep_affordance"),
        "human semantic sleep actions must be backed by an open modeled affordance"
    );
    assert!(
        routine_continuation.contains("actor_known_sleep_affordance_id"),
        "agent sleep proposals must carry the actor-known affordance id"
    );
    assert!(
        builder.contains("actor_knows_sleep_affordance"),
        "no-human cognition must derive sleep affordance ids as actor-known facts"
    );
}

#[test]
fn guard_0021_actor_known_projection_policy_table_has_production_callers() {
    fn policy_caller_count(source: &str) -> usize {
        production(source).matches(".policy()").count()
    }

    assert!(
        policy_caller_count(EPISTEMIC_PROJECTION_RS) >= 1,
        "ActorKnownProjectionRecord::policy() must drive production classification instead of remaining decorative"
    );
    let synthetic_zero_caller =
        EPISTEMIC_PROJECTION_RS.replace(".policy()", ".decorative_policy()");
    assert_eq!(
        policy_caller_count(&synthetic_zero_caller),
        0,
        "policy caller guard must fail closed when production call sites disappear"
    );
    assert_absent(EPISTEMIC_PROJECTION_RS, "CurrentPlaceLatestOnly");
    assert_absent(
        EPISTEMIC_PROJECTION_RS,
        concat!(
            "actor_known_projection_records",
            "_dispatch_to_declared_policy_table"
        ),
    );
    assert!(
        EPISTEMIC_PROJECTION_RS
            .contains("actor_known_projection_policy_table_drives_record_behavior"),
        "policy-table lock must be behavioral, not a self-echo dispatch assertion"
    );
}

#[test]
fn guard_015_ordinary_life_tuning_comes_from_authored_state() {
    let sleep = production(SLEEP_RS);
    let time = production(TIME_RS);

    for forbidden in [
        "AWAKE_HUNGER_DELTA_PER_TICK",
        "AWAKE_FATIGUE_DELTA_PER_TICK",
        "DEFAULT_SLEEP_DURATION_TICKS",
        "FATIGUE_RECOVERY_PER_SLEEP_TICK",
        "HUNGER_RISE_PER_SLEEP_TICK",
    ] {
        assert_absent(&time, forbidden);
        assert_absent(&sleep, forbidden);
    }
    assert!(
        time.contains("need_model.awake_hunger_delta_per_tick")
            && time.contains("need_model.awake_fatigue_delta_per_tick"),
        "passive awake need deltas must read the authored need model"
    );
    assert!(
        sleep.contains("sleep_affordance.duration_ticks")
            && sleep.contains("sleep_affordance.fatigue_recovery_per_tick")
            && sleep.contains("sleep_affordance.hunger_rise_per_tick"),
        "sleep duration and recovery must read authored sleep affordance state"
    );
}

#[test]
fn scheduler_apply_and_completion_paths_do_not_panic_on_log_derived_data() {
    let sources = [
        (
            "crates/tracewake-core/src/scheduler.rs",
            production(SCHEDULER_RS),
        ),
        (
            "crates/tracewake-core/src/events/apply.rs",
            production(EVENTS_APPLY_RS),
        ),
        (
            "crates/tracewake-core/src/actions/defs/sleep.rs",
            production(SLEEP_RS),
        ),
        (
            "crates/tracewake-core/src/actions/defs/work.rs",
            production(WORK_RS),
        ),
    ];
    let violations = log_derived_panic_violations(&sources);
    assert!(
        violations.is_empty(),
        "unallowlisted panic on apply/completion/scheduler path: {violations:?}"
    );

    let synthetic_sources = [(
        "crates/tracewake-core/src/scheduler.rs",
        r#"
        fn actor_has_open_body_exclusive_duration(log: &EventLog) -> bool {
            open_body_exclusive_starts(log).expect("duplicate duration terminals are rejected before no-human scheduling");
            apply_agent_event(agent_state, &event).expect("routine stuck diagnostic event applies to live agent state");
            event.payload.iter().find(|field| field.key == "actor_id").unwrap();
            true
        }
        "#.to_string(),
    )];
    let synthetic_violations = log_derived_panic_violations(&synthetic_sources);
    assert!(
        synthetic_violations.len() == 3,
        "synthetic_log_derived_unwrap_payload must fail with log-derived expects"
    );
    assert!(
        PANIC_ALLOWLIST
            .iter()
            .all(|entry| !entry.rationale.trim().is_empty()),
        "panic allowlist entries must carry rationales"
    );
}

fn log_derived_panic_violations(sources: &[(&str, String)]) -> Vec<String> {
    let mut violations = Vec::new();
    for (path, source) in sources {
        let lines = source.lines().collect::<Vec<_>>();
        for (index, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if !(trimmed.contains(".expect(")
                || trimmed.contains(".unwrap(")
                || trimmed.contains("assert!("))
            {
                continue;
            }
            let scan_unit = panic_scan_unit(&lines, index);
            if !panic_allowlist_covers(path, &scan_unit) {
                violations.push(format!("{path}: {scan_unit}"));
            }
        }
    }
    violations
}

fn panic_scan_unit(lines: &[&str], index: usize) -> String {
    let start = index.saturating_sub(20);
    lines[start..=index]
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn panic_allowlist_covers(path: &str, line: &str) -> bool {
    PANIC_ALLOWLIST.iter().any(|entry| {
        entry.path == path && line.contains(entry.token) && !entry.rationale.trim().is_empty()
    })
}

#[test]
fn guard_006_scheduler_has_no_routine_family_to_primitive_dispatch() {
    let scheduler = production(SCHEDULER_RS);
    for forbidden in [
        "RoutineFamily::EatMeal => Some(GoalKind::Eat)",
        "RoutineFamily::FindFood => Some(GoalKind::FindFood)",
        "RoutineFamily::SleepNight => Some(GoalKind::SleepOrRest)",
        "RoutineFamily::GoToWork => Some(GoalKind::GoToWork)",
        "RoutineFamily::WorkBlock => Some(GoalKind::PerformWorkBlock)",
        "ActionId::new(\"eat\")",
        "ActionId::new(\"sleep\")",
        "ActionId::new(\"work_block\")",
    ] {
        assert_absent(&scheduler, forbidden);
    }
}

#[test]
fn guard_014_scheduler_cannot_rewrite_transaction_proposals_after_cognition() {
    let scheduler = production(SCHEDULER_RS);
    let transaction = production(TRANSACTION_RS);
    let run_no_human_actor_decision_transaction =
        body_after_marker(&scheduler, "fn run_no_human_actor_decision_transaction");
    let after_transaction_run = body_after_marker(
        run_no_human_actor_decision_transaction,
        "ActorDecisionTransaction::run",
    );

    assert!(
        transaction.contains("pub struct SealedProposal"),
        "actor decision transaction must expose a sealed proposal handoff type"
    );
    assert!(
        transaction.contains("proposal: Proposal"),
        "sealed proposal must retain Proposal behind a private field"
    );
    assert_absent(&transaction, "pub proposal: Proposal");

    for forbidden in [
        "proposal.parameters.insert",
        "proposal.target_ids.push",
        "proposal.action_id =",
        "proposal.actor_id =",
        "let mut proposal = proposed.proposal",
        "proposed.proposal.parameters",
        "proposed.proposal.target_ids",
        "proposed.proposal.action_id",
    ] {
        assert_absent(after_transaction_run, forbidden);
    }
}

#[test]
fn guard_014_transaction_has_no_silent_method_fallback_scan() {
    let transaction = production(TRANSACTION_RS);
    let routine_continuation = production(ROUTINE_CONTINUATION_RS);
    assert_absent(&transaction, "candidate_fallbacks");
    assert_absent(&transaction, ".find_map(|candidate|");
    assert!(
        transaction.contains("pub struct SelectedGoalBundle"),
        "transaction must bind selected candidate, trace, method, local plan, and proposal ancestry"
    );
    assert!(
        routine_continuation.contains("bundle.decision_trace_id.as_str().to_string()"),
        "proposal decision_trace_id must come from the selected goal bundle"
    );
    assert!(
        routine_continuation.contains("bundle.candidate_goal_id.as_str().to_string()"),
        "proposal candidate_goal_id must come from the selected goal bundle"
    );
    assert!(
        transaction.contains("method_selection_rejected"),
        "method fallback rerun must preserve a typed rejection reason for the failed selected candidate"
    );
}

#[test]
fn guard_015_hidden_truth_audit_fails_closed_in_transaction() {
    let transaction = production(TRANSACTION_RS);
    let pipeline = production(PIPELINE_RS);

    assert!(
        transaction.contains("stuck_diagnostic_for_hidden_truth_audit"),
        "transaction must turn failed hidden-truth audit into a typed stuck diagnostic"
    );
    assert!(
        transaction.contains("!selection.trace.hidden_truth_audit_result.actor_known_only"),
        "transaction must gate on the selected trace hidden-truth audit before proposal construction"
    );
    assert!(
        transaction.contains("BlockerCode::HiddenTruthInput"),
        "transaction stuck diagnostic must use hidden_truth_input blocker code"
    );
    assert!(
        transaction.contains("hidden_truth_referenced: true"),
        "transaction stuck diagnostic must preserve hidden-truth typed signal"
    );
    assert!(
        pipeline.contains("ReasonCode::HiddenTruthInput"),
        "pipeline must reject agent-origin proposals carrying a failed hidden-truth audit"
    );
}

#[test]
fn guard_014_decision_hidden_truth_audit_uses_typed_input_refs() {
    let decision = production(DECISION_RS);
    assert_absent(&decision, "actor_known_inputs: Vec<String>");
    assert_absent(&decision, "contains(\"unproven\")");
    assert_absent(&decision, "contains(\"debug_omniscience\")");
    assert_absent(&decision, "contains(\"physical_truth\")");
    assert!(
        decision.contains("struct ActorKnownInputRef"),
        "decision input refs must be typed provenance records"
    );
    assert!(
        decision.contains("source_class.is_forbidden_for_cognition()"),
        "hidden-truth audit must key on typed source class"
    );
}

#[test]
fn guard_003_work_eat_sleep_validators_do_not_read_need_values_from_proposal_parameters() {
    for source in [
        production(EAT_RS),
        production(SLEEP_RS),
        production(WORK_RS),
    ] {
        assert_absent(&source, "parameters.get(\"current_hunger\")");
        assert_absent(&source, "parameters.get(\"current_fatigue\")");
        assert_absent(&source, "parameters[\"current_hunger\"]");
        assert_absent(&source, "parameters[\"current_fatigue\"]");
    }
    assert!(
        production(WORK_RS).contains("need_value(agent_state"),
        "work validator must read authoritative AgentState needs"
    );
}

#[test]
fn agent_world_noop_allowlist_is_explicit_and_excludes_materialized_episode_state() {
    use tracewake_core::checksum::{ChecksumStateKind, PHYSICAL_STATE_CHECKSUM_COVERAGE};
    use tracewake_core::events::apply::AGENT_WORLD_NOOP_ALLOWLIST;
    use tracewake_core::events::EventKind;

    const PAYLOAD_FREE_AGENT_MARKERS: &[EventKind] =
        &[EventKind::NoHumanDayStarted, EventKind::NoHumanDayCompleted];

    let allowlist = AGENT_WORLD_NOOP_ALLOWLIST
        .iter()
        .map(|kind| kind.stable_id())
        .collect::<Vec<_>>();

    assert_eq!(
        allowlist,
        vec![
            "food_consumed",
            "no_human_day_started",
            "no_human_day_completed"
        ]
    );
    assert!(
        PHYSICAL_STATE_CHECKSUM_COVERAGE.iter().any(|entry| {
            entry.state_kind == ChecksumStateKind::Physical
                && entry.field_name == "food_supplies"
                && entry.field_family == "food_supply"
        }) && CHECKSUM_RS.contains("\"food_supply|"),
        "FoodConsumed allowlist entry must be covered by the physical food_supply checksum family"
    );
    for marker in PAYLOAD_FREE_AGENT_MARKERS {
        assert!(
            AGENT_WORLD_NOOP_ALLOWLIST.contains(marker),
            "payload-free marker {} must be explicitly registered",
            marker.stable_id()
        );
    }
    for materialized in [
        EventKind::NeedThresholdCrossed,
        EventKind::CandidateGoalsEvaluated,
        EventKind::SleepStarted,
        EventKind::SleepCompleted,
        EventKind::SleepInterrupted,
        EventKind::FoodServiceUsed,
        EventKind::EatFailed,
        EventKind::WorkBlockStarted,
        EventKind::WorkBlockCompleted,
        EventKind::WorkBlockFailed,
        EventKind::ContinueRoutineProposed,
        EventKind::ContinueRoutineAccepted,
        EventKind::ContinueRoutineRejected,
    ] {
        assert!(!AGENT_WORLD_NOOP_ALLOWLIST.contains(&materialized));
    }
}

#[test]
fn materialized_agent_payload_records_keep_payload_fields() {
    let mut struct_names = Vec::new();
    for line in STATE_RS.lines() {
        if !line.contains("BTreeMap<crate::ids::EventId,") {
            continue;
        }
        let Some((_, tail)) = line.split_once("BTreeMap<crate::ids::EventId,") else {
            continue;
        };
        let Some(struct_name) = tail.split('>').next() else {
            continue;
        };
        struct_names.push(struct_name.trim());
    }
    struct_names.sort_unstable();
    struct_names.dedup();
    assert_eq!(
        struct_names,
        vec![
            "CandidateGoalEvaluationRecord",
            "ContinueRoutineArbitrationRecord",
            "NeedThresholdCrossingRecord",
            "OrdinaryLifeEpisodeRecord",
        ],
        "materialized AgentState event records must be derived from state.rs maps"
    );

    for struct_name in struct_names {
        let marker = format!("pub struct {struct_name} {{");
        let body = STATE_RS
            .split(&marker)
            .nth(1)
            .unwrap_or_else(|| panic!("{struct_name} declaration is present"))
            .split("}\n")
            .next()
            .unwrap_or_else(|| panic!("{struct_name} body is present"));
        assert!(
            body.contains("pub payload_fields: Vec<(String, String)>"),
            "{struct_name} must retain source event payload fields for replay/checksum evidence"
        );
    }
    assert!(
        CHECKSUM_RS.contains("ordinary_life_episode|")
            && CHECKSUM_RS.contains("join_pairs(&episode.payload_fields)"),
        "ordinary-life episode payload fields must enter the canonical agent checksum"
    );
    assert!(
        CHECKSUM_RS.contains("need_threshold_crossing|")
            && CHECKSUM_RS.contains("join_pairs(&crossing.payload_fields)"),
        "need-threshold payload fields must enter the canonical agent checksum"
    );
}

#[test]
fn materialized_agent_apply_arms_require_payload_schema_version() {
    use tracewake_core::checksum::AGENT_STATE_CHECKSUM_COVERAGE;

    let covered_maps = AGENT_STATE_CHECKSUM_COVERAGE
        .iter()
        .map(|entry| entry.field_name)
        .collect::<Vec<_>>();
    let source = production(EVENTS_APPLY_RS);
    let sites = agent_state_map_write_sites(&source, &covered_maps);
    let rebound_errors = agent_state_map_rebound_errors(&source, &covered_maps);
    let errors = materialized_agent_apply_arm_version_errors(
        &source,
        &sites,
        TYPED_COLUMN_CLOSURE_EXEMPTIONS,
    );

    assert_eq!(
        sites
            .iter()
            .map(|site| site.map_name.as_str())
            .collect::<std::collections::BTreeSet<_>>(),
        covered_maps
            .into_iter()
            .collect::<std::collections::BTreeSet<_>>(),
        "derived apply-arm census must cover every checksum-covered AgentState map"
    );
    assert!(
        errors.is_empty(),
        "materialized AgentState map writes without version gate or typed-column exemption:\n{}",
        errors.join("\n")
    );
    assert!(
        rebound_errors.is_empty(),
        "covered AgentState maps must not be rebound through &mut state.<map> aliases:\n{}",
        rebound_errors.join("\n")
    );

    let synthetic_source = r#"
        fn apply_synthetic(state: &mut AgentState) {
            state.needs_by_actor.insert(actor_id, needs);
        }
    "#;
    let synthetic_sites = agent_state_map_write_sites(synthetic_source, &["needs_by_actor"]);
    let synthetic_errors =
        materialized_agent_apply_arm_version_errors(synthetic_source, &synthetic_sites, &[]);
    assert!(
        synthetic_errors
            .iter()
            .any(|error| error.contains("needs_by_actor") && error.contains("apply_synthetic")),
        "synthetic covered-map write without gate or exemption must fail the census"
    );

    let two_arm_inline_source = r#"
        fn apply_agent_event_with_capability(state: &mut AgentState, event: &EventEnvelope) {
            let payload = payload_map(&event.payload);
            match event.event_type {
                EventKind::DecisionTraceRecorded => {
                    require_payload_version(&payload, "trace_schema_version", "1")?;
                    state.decision_traces.insert(trace_id, record);
                }
                EventKind::StuckDiagnosticRecorded => {
                    state.stuck_diagnostics.insert(diagnostic_id, record);
                }
                _ => {}
            }
        }
    "#;
    let two_arm_sites = agent_state_map_write_sites(
        two_arm_inline_source,
        &["decision_traces", "stuck_diagnostics"],
    );
    let two_arm_errors =
        materialized_agent_apply_arm_version_errors(two_arm_inline_source, &two_arm_sites, &[]);
    assert!(
        two_arm_errors
            .iter()
            .any(|error| error.contains("stuck_diagnostics")
                && error.contains("EventKind::StuckDiagnosticRecorded")),
        "synthetic second inline arm without its own gate must fail per-arm anchoring"
    );

    let retain_source = r#"
        fn apply_synthetic(state: &mut AgentState) {
            state.needs_by_actor.retain(|_, _| true);
        }
    "#;
    let retain_sites = agent_state_map_write_sites(retain_source, &["needs_by_actor"]);
    let retain_errors =
        materialized_agent_apply_arm_version_errors(retain_source, &retain_sites, &[]);
    assert!(
        retain_errors
            .iter()
            .any(|error| error.contains("needs_by_actor") && error.contains("retain")),
        "synthetic retain-shaped covered-map write must fail the inverted write scan"
    );

    let rebound_source = r#"
        fn apply_synthetic(state: &mut AgentState) {
            let needs = &mut state.needs_by_actor;
            needs.insert(actor_id, values);
        }
    "#;
    let rebound_errors = agent_state_map_rebound_errors(rebound_source, &["needs_by_actor"]);
    assert!(
        rebound_errors
            .iter()
            .any(|error| error.contains("needs_by_actor")),
        "synthetic &mut state.<map> rebound must fail the source guard"
    );

    let payload_fields_source = r#"
        fn apply_synthetic(state: &mut AgentState, event: &EventEnvelope) {
            state.ordinary_life_episodes.insert(event.event_id.clone(), OrdinaryLifeEpisodeRecord {
                payload_fields: payload_fields(event),
            });
        }
    "#;
    let payload_fields_sites =
        agent_state_map_write_sites(payload_fields_source, &["ordinary_life_episodes"]);
    let payload_fields_errors = materialized_agent_apply_arm_version_errors(
        payload_fields_source,
        &payload_fields_sites,
        &[],
    );
    assert!(
        payload_fields_errors
            .iter()
            .any(|error| error.contains("payload_fields")),
        "synthetic payload_fields retention outside a gated materialized arm must fail"
    );
}

#[test]
fn typed_column_closure_exemptions_are_rationale_bearing_and_live() {
    for exemption in TYPED_COLUMN_CLOSURE_EXEMPTIONS {
        assert!(
            !exemption.anchor.is_empty()
                && !exemption.typed_columns.is_empty()
                && !exemption.rationale.trim().is_empty(),
            "{exemption:?} must name its arm/helper, typed columns, and rationale"
        );
        assert!(
            EVENTS_APPLY_RS.contains(exemption.anchor),
            "{} exemption anchor is present in apply.rs",
            exemption.anchor
        );
        assert!(
            tracewake_core::checksum::AGENT_STATE_CHECKSUM_COVERAGE
                .iter()
                .any(|entry| entry.field_name == exemption.map_name),
            "{} exemption map is checksum-covered",
            exemption.map_name
        );
    }
    let errors =
        typed_column_closure_exemption_errors(EVENTS_APPLY_RS, TYPED_COLUMN_CLOSURE_EXEMPTIONS);
    assert!(
        errors.is_empty(),
        "typed-column exemptions must list every consumed payload key and avoid raw payload retention:\n{}",
        errors.join("\n")
    );

    let synthetic_source = r#"
        fn apply_synthetic(state: &mut AgentState, payload: &BTreeMap<&str, &str>) {
            let value = required(payload, "unlisted_key")?;
            state.intentions.insert(intention_id, value);
        }
    "#;
    let synthetic_exemptions = [TypedColumnClosureExemption {
        map_name: "intentions",
        anchor: "apply_synthetic",
        typed_columns: &["intention_id"],
        rationale: "synthetic exemption with an omitted consumed key",
    }];
    let synthetic_errors =
        typed_column_closure_exemption_errors(synthetic_source, &synthetic_exemptions);
    assert!(
        synthetic_errors
            .iter()
            .any(|error| error.contains("unlisted_key")),
        "synthetic exemption consuming an unlisted payload key must fail"
    );

    let synthetic_oblique_helper_source = r#"
        fn apply_synthetic(state: &mut AgentState, payload: &BTreeMap<&str, &str>) {
            consume_oblique_payload_key(state, &payload);
        }
        fn consume_oblique_payload_key(
            state: &mut AgentState,
            payload: &BTreeMap<&str, &str>,
        ) {
            let value = required(payload, "oblique_unlisted_key")?;
            state.intentions.insert(intention_id, value);
        }
    "#;
    let synthetic_oblique_helper_errors = typed_column_closure_exemption_errors(
        synthetic_oblique_helper_source,
        &synthetic_exemptions,
    );
    assert!(
        synthetic_oblique_helper_errors
            .iter()
            .any(|error| error.contains("oblique_unlisted_key")),
        "synthetic oblique helper call receiving &payload must fail"
    );

    let synthetic_payload_receiver_source = r#"
        fn apply_synthetic(state: &mut AgentState, payload: &BTreeMap<&str, &str>) {
            payload.consume_into(state);
        }
        impl PayloadView {
            fn consume_into(&self, state: &mut AgentState) {
                let payload = self.payload;
                let value = required(payload, "receiver_unlisted_key")?;
                state.intentions.insert(intention_id, value);
            }
        }
    "#;
    let synthetic_payload_receiver_errors = typed_column_closure_exemption_errors(
        synthetic_payload_receiver_source,
        &synthetic_exemptions,
    );
    assert!(
        synthetic_payload_receiver_errors
            .iter()
            .any(|error| error.contains("receiver_unlisted_key")),
        "synthetic_payload_receiver_helper_call must fail"
    );

    let synthetic_payload_alias_source = r#"
        fn apply_synthetic(state: &mut AgentState, payload: &BTreeMap<&str, &str>) {
            let view = &payload;
            consume_alias_payload_key(state, view);
        }
        fn consume_alias_payload_key(
            state: &mut AgentState,
            payload: &BTreeMap<&str, &str>,
        ) {
            let value = required(payload, "alias_unlisted_key")?;
            state.intentions.insert(intention_id, value);
        }
    "#;
    let synthetic_payload_alias_keys =
        consumed_payload_keys_for_anchor(synthetic_payload_alias_source, "apply_synthetic");
    assert!(
        synthetic_payload_alias_keys.contains("alias_unlisted_key"),
        "synthetic payload alias key derivation missed alias_unlisted_key: {synthetic_payload_alias_keys:?}"
    );
    let synthetic_payload_alias_errors = typed_column_closure_exemption_errors(
        synthetic_payload_alias_source,
        &synthetic_exemptions,
    );
    assert!(
        synthetic_payload_alias_errors
            .iter()
            .any(|error| error.contains("alias_unlisted_key")),
        "synthetic_payload_alias_helper_call must fail"
    );

    let synthetic_payload_fields_source = r#"
        fn apply_synthetic(state: &mut AgentState, event: &EventEnvelope) {
            state.intentions.insert(intention_id, intention_with(payload_fields(event)));
        }
    "#;
    let synthetic_payload_fields_errors = typed_column_closure_exemption_errors(
        synthetic_payload_fields_source,
        &synthetic_exemptions,
    );
    assert!(
        synthetic_payload_fields_errors
            .iter()
            .any(|error| error.contains("payload_fields")),
        "synthetic exempted helper retaining payload_fields must fail"
    );
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct AgentStateMapWriteSite {
    map_name: String,
    anchor: String,
    write_token: String,
    index: usize,
}

const AGENT_STATE_MAP_READ_METHODS: &[&str] = &[
    "contains_key",
    "get",
    "is_empty",
    "iter",
    "keys",
    "len",
    "values",
];

fn agent_state_map_write_sites(source: &str, covered_maps: &[&str]) -> Vec<AgentStateMapWriteSite> {
    let scan_source = normalized_source(source).replace(" .", ".");
    let mut sites = Vec::new();
    for map_name in covered_maps {
        let map_token = format!("state.{map_name}.");
        let mut search_start = 0;
        while let Some(relative_index) = scan_source[search_start..].find(&map_token) {
            let index = search_start + relative_index;
            let after = &scan_source[index + map_token.len()..];
            let method_name = after
                .split(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_'))
                .next()
                .unwrap_or("");
            if method_name.is_empty() || AGENT_STATE_MAP_READ_METHODS.contains(&method_name) {
                search_start = index + map_token.len();
                continue;
            }
            sites.push(AgentStateMapWriteSite {
                map_name: (*map_name).to_string(),
                anchor: enclosing_apply_anchor(&scan_source, index),
                write_token: format!("{map_token}{method_name}("),
                index,
            });
            search_start = index + map_token.len();
        }
    }
    sites.sort_by(|left, right| {
        left.map_name
            .cmp(&right.map_name)
            .then(left.anchor.cmp(&right.anchor))
            .then(left.index.cmp(&right.index))
    });
    sites.dedup_by(|left, right| {
        left.map_name == right.map_name
            && left.anchor == right.anchor
            && left.write_token == right.write_token
    });
    sites
}

fn agent_state_map_rebound_errors(source: &str, covered_maps: &[&str]) -> Vec<String> {
    let scan_source = normalized_source(source).replace(" .", ".");
    covered_maps
        .iter()
        .filter_map(|map_name| {
            let token = format!("&mut state.{map_name}");
            scan_source
                .contains(&token)
                .then(|| format!("{map_name} is rebound through {token}"))
        })
        .collect()
}

fn enclosing_apply_anchor(scan_source: &str, index: usize) -> String {
    let prefix = &scan_source[..index];
    let fn_start = prefix
        .rfind(" fn ")
        .or_else(|| prefix.rfind(" pub fn "))
        .or_else(|| prefix.rfind("fn "))
        .unwrap_or(0);
    let fn_name_start = scan_source[fn_start..]
        .find("fn ")
        .map(|offset| fn_start + offset + "fn ".len())
        .unwrap_or(fn_start);
    let fn_name = scan_source[fn_name_start..]
        .split(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_'))
        .next()
        .unwrap_or("unknown");
    if !fn_name.starts_with("apply_agent_event") {
        return fn_name.to_string();
    }
    prefix
        .rfind("EventKind::")
        .and_then(|event_start| {
            prefix[event_start + "EventKind::".len()..]
                .split(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_'))
                .next()
                .map(|event| format!("EventKind::{event}"))
        })
        .unwrap_or_else(|| fn_name.to_string())
}

fn materialized_agent_apply_arm_version_errors(
    source: &str,
    sites: &[AgentStateMapWriteSite],
    exemptions: &[TypedColumnClosureExemption],
) -> Vec<String> {
    let scan_source = normalized_source(source).replace(" .", ".");
    let mut errors = sites
        .iter()
        .filter_map(|site| {
            if apply_write_site_has_version_gate(&scan_source, site) {
                return None;
            }
            if exemptions.iter().any(|exemption| {
                exemption.map_name == site.map_name && exemption.anchor == site.anchor
            }) {
                return None;
            }
            Some(format!(
                "{} at {} via {} lacks version gate or typed-column exemption",
                site.map_name, site.anchor, site.write_token
            ))
        })
        .collect::<Vec<_>>();
    errors.extend(payload_fields_outside_gated_materialized_sites(
        &scan_source,
        sites,
    ));
    errors
}

fn apply_write_site_has_version_gate(scan_source: &str, site: &AgentStateMapWriteSite) -> bool {
    let start = if site.anchor.starts_with("EventKind::") {
        scan_source[..site.index]
            .rfind(&site.anchor)
            .unwrap_or(site.index)
    } else {
        scan_source[..site.index]
            .rfind(&format!("fn {}", site.anchor))
            .unwrap_or(site.index)
    };
    let segment = &scan_source[start..site.index];
    segment.contains(r#"require_payload_version(&payload, "payload_schema_version", "1")"#)
        || segment.contains(r#"require_payload_version(&payload, "trace_schema_version", "1")"#)
        || segment
            .contains(r#"require_payload_version(&payload, "diagnostic_schema_version", "1")"#)
}

fn payload_fields_outside_gated_materialized_sites(
    scan_source: &str,
    sites: &[AgentStateMapWriteSite],
) -> Vec<String> {
    let mut errors = Vec::new();
    let mut search_start = 0;
    while let Some(relative_index) = scan_source[search_start..].find("payload_fields(") {
        let index = search_start + relative_index;
        let anchor = enclosing_apply_anchor(scan_source, index);
        if anchor == "payload_fields" {
            search_start = index + "payload_fields(".len();
            continue;
        }
        let anchored_site = sites
            .iter()
            .filter(|site| site.index <= index && site.anchor == anchor)
            .max_by_key(|site| site.index);
        if !anchored_site.is_some_and(|site| apply_write_site_has_version_gate(scan_source, site)) {
            errors.push(format!(
                "payload_fields at byte {index} is outside a gated materialized write site"
            ));
        }
        search_start = index + "payload_fields(".len();
    }
    errors
}

fn consumed_payload_keys_for_anchor(source: &str, anchor: &str) -> BTreeSet<String> {
    let scan_source = normalized_source(source).replace(" .", ".");
    let mut visited = BTreeSet::new();
    consumed_payload_keys_for_function(&scan_source, anchor, &mut visited)
}

fn consumed_payload_keys_for_function(
    scan_source: &str,
    function_name: &str,
    visited: &mut BTreeSet<String>,
) -> BTreeSet<String> {
    if !visited.insert(function_name.to_string()) {
        return BTreeSet::new();
    }
    let Some(body) = function_body(scan_source, function_name) else {
        return BTreeSet::new();
    };
    let payload_bindings = payload_binding_aliases(body);
    let mut keys = literal_payload_keys(body, &payload_bindings);
    for callee in payload_helper_calls(body, &payload_bindings) {
        if matches!(
            callee.as_str(),
            "required" | "expect_bool" | "parse_i32" | "parse_u8" | "parse_u64_agent"
        ) {
            continue;
        }
        keys.extend(consumed_payload_keys_for_function(
            scan_source,
            &callee,
            visited,
        ));
    }
    keys
}

fn function_body<'a>(scan_source: &'a str, function_name: &str) -> Option<&'a str> {
    let marker = format!("fn {function_name}");
    let after_marker = scan_source.split(&marker).nth(1)?;
    let start = after_marker.find('{')?;
    let mut depth = 0_i32;
    let mut end = None;
    for (offset, byte) in after_marker[start..].bytes().enumerate() {
        match byte {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    end = Some(start + offset + 1);
                    break;
                }
            }
            _ => {}
        }
    }
    end.map(|end| &after_marker[start..end])
}

fn literal_payload_keys(body: &str, payload_bindings: &BTreeSet<String>) -> BTreeSet<String> {
    let mut keys = BTreeSet::new();
    for payload_binding in payload_bindings {
        for marker in [
            format!(r#"required({payload_binding}, ""#),
            format!(r#"required(&{payload_binding}, ""#),
            format!(r#"{payload_binding}.get(""#),
            format!(r#"{payload_binding}, ""#),
        ] {
            let mut search_start = 0;
            while let Some(relative_index) = body[search_start..].find(&marker) {
                let value_start = search_start + relative_index + marker.len();
                if let Some(value_end) = body[value_start..].find('"') {
                    keys.insert(body[value_start..value_start + value_end].to_string());
                }
                search_start = value_start;
            }
        }
    }
    keys
}

fn payload_helper_calls(body: &str, payload_bindings: &BTreeSet<String>) -> BTreeSet<String> {
    let mut calls = BTreeSet::new();
    let mut search_start = 0;
    while let Some(relative_index) = body[search_start..].find('(') {
        let open_index = search_start + relative_index;
        let prefix = body[..open_index].trim_end();
        if let Some(name) = call_name_before_open_paren(prefix) {
            let close_index =
                matching_close_delimiter(body, open_index, b'(', b')').unwrap_or(open_index);
            let args = &body[open_index + 1..close_index];
            if call_arguments_include_payload_binding(args, payload_bindings)
                || call_receiver_includes_payload_binding(prefix, name, payload_bindings)
            {
                calls.insert(name.to_string());
            }
            search_start = close_index + 1;
        } else {
            search_start = open_index + 1;
        }
    }
    calls
}

fn call_name_before_open_paren(prefix: &str) -> Option<&str> {
    let name_start = prefix
        .rfind(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_'))
        .map(|index| index + 1)
        .unwrap_or(0);
    let name = &prefix[name_start..];
    if name.is_empty()
        || matches!(
            name,
            "if" | "while" | "for" | "match" | "return" | "Some" | "Ok" | "Err"
        )
    {
        None
    } else {
        Some(name)
    }
}

fn call_arguments_include_payload_binding(args: &str, payload_bindings: &BTreeSet<String>) -> bool {
    split_top_level_args(args).into_iter().any(|arg| {
        let normalized = normalized_payload_expr(&arg);
        payload_bindings.iter().any(|binding| {
            normalized == *binding
                || normalized
                    .strip_prefix(binding)
                    .is_some_and(|suffix| suffix.starts_with('.'))
        })
    })
}

fn call_receiver_includes_payload_binding(
    prefix: &str,
    method_name: &str,
    payload_bindings: &BTreeSet<String>,
) -> bool {
    let Some(before_method) = prefix.strip_suffix(method_name) else {
        return false;
    };
    let Some(receiver_prefix) = before_method.strip_suffix('.') else {
        return false;
    };
    let receiver = receiver_prefix
        .rsplit(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_' || ch == '&' || ch == '*'))
        .next()
        .unwrap_or("");
    let normalized = normalized_payload_expr(receiver);
    payload_bindings
        .iter()
        .any(|binding| normalized == *binding)
}

fn payload_binding_aliases(body: &str) -> BTreeSet<String> {
    let mut aliases = BTreeSet::from(["payload".to_string()]);
    let mut changed = true;
    while changed {
        changed = false;
        for statement in body.split(';').map(str::trim) {
            let Some(let_index) = statement.find("let ") else {
                continue;
            };
            let rest = &statement[let_index + "let ".len()..];
            let Some((left, right)) = rest.split_once('=') else {
                continue;
            };
            let Some(alias) = let_binding_name(left) else {
                continue;
            };
            let normalized_right = normalized_payload_expr(right.trim_end_matches(';'));
            if aliases.contains(&normalized_right) && aliases.insert(alias) {
                changed = true;
            }
        }
    }
    aliases
}

fn let_binding_name(left: &str) -> Option<String> {
    let left = left.trim().trim_start_matches("mut ").trim();
    let name = left
        .split(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_'))
        .next()
        .unwrap_or("");
    (!name.is_empty()).then(|| name.to_string())
}

fn normalized_payload_expr(expr: &str) -> String {
    expr.trim()
        .trim_start_matches('&')
        .trim_start_matches('*')
        .trim()
        .trim_end_matches(';')
        .trim()
        .to_string()
}

fn matching_close_delimiter(source: &str, open_index: usize, open: u8, close: u8) -> Option<usize> {
    let mut depth = 0_i32;
    let mut in_string = false;
    let bytes = source.as_bytes();
    for index in open_index..bytes.len() {
        match bytes[index] {
            b'"' if index == 0 || bytes[index - 1] != b'\\' => in_string = !in_string,
            byte if !in_string && byte == open => depth += 1,
            byte if !in_string && byte == close => {
                depth -= 1;
                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }
    }
    None
}

fn typed_column_closure_exemption_errors(
    source: &str,
    exemptions: &[TypedColumnClosureExemption],
) -> Vec<String> {
    let mut errors = Vec::new();
    for exemption in exemptions {
        let consumed_keys = consumed_payload_keys_for_anchor(source, exemption.anchor);
        for key in &consumed_keys {
            if !exemption.typed_columns.contains(&key.as_str()) {
                errors.push(format!(
                    "{} exemption for {} consumes unlisted payload key {key}",
                    exemption.anchor, exemption.map_name
                ));
            }
        }
        if function_body(
            &normalized_source(source).replace(" .", "."),
            exemption.anchor,
        )
        .is_some_and(|body| body.contains("payload_fields("))
        {
            errors.push(format!(
                "{} exemption for {} retains raw payload_fields",
                exemption.anchor, exemption.map_name
            ));
        }
    }
    errors
}

#[test]
fn generative_lock_cannot_fabricate_duration_terminals() {
    let sources = [
        ("generative_lock.rs", GENERATIVE_LOCK_RS),
        (
            "support/acceptance_status_manifest.rs",
            SUPPORT_ACCEPTANCE_STATUS_MANIFEST_RS,
        ),
        ("support/generative.rs", SUPPORT_GENERATIVE_RS),
        ("support/mod.rs", SUPPORT_MOD_RS),
    ];
    let errors = generative_duration_terminal_fabricator_errors(&sources);
    assert!(
        errors.is_empty(),
        "generative duration-terminal fabricator ban failed: {errors:#?}"
    );
    let support_sources = support_event_envelope_scan_sources();
    let parity_errors =
        support_directory_parity_errors(&support_sources, &support_directory_files());
    assert!(
        parity_errors.is_empty(),
        "support EventEnvelope scan list must match tests/support directory: {parity_errors:#?}"
    );
    let mut synthetic_support_files = support_directory_files();
    synthetic_support_files.insert("support/synthetic.rs".to_string());
    assert!(
        support_directory_parity_errors(&support_sources, &synthetic_support_files)
            .iter()
            .any(|error| error.contains("support/synthetic.rs")),
        "synthetic third support file must fail support scan parity"
    );

    let support_fabricator = [(
        "support/generative.rs",
        "fn helper() { EventEnvelope::new_v1(id, EventKind::WorkBlockFailed, 0, 0, tick, key, manifest); }",
    )];
    assert!(
        generative_duration_terminal_fabricator_errors(&support_fabricator)
            .iter()
            .any(|error| error.contains("support/generative.rs")
                && error.contains("EventEnvelope")),
        "support-file EventEnvelope construction synthetic must fail"
    );

    let support_default_fabricator = [(
        "support/generative.rs",
        "fn helper() { let _event = EventEnvelope::default(); }",
    )];
    assert!(
        generative_duration_terminal_fabricator_errors(&support_default_fabricator)
            .iter()
            .any(|error| error.contains("support/generative.rs")
                && error.contains("EventEnvelope")),
        "synthetic_support_event_envelope_default must fail the support fabricator ban"
    );

    let support_struct_literal_fabricator = [(
        "support/generative.rs",
        "fn helper() { let _event = EventEnvelope { event_type: EventKind::SleepCompleted, ..base }; }",
    )];
    assert!(
        generative_duration_terminal_fabricator_errors(&support_struct_literal_fabricator)
            .iter()
            .any(|error| error.contains("support/generative.rs")
                && error.contains("EventEnvelope")),
        "synthetic support EventEnvelope struct literal must fail the support fabricator ban"
    );

    let support_alias_fabricator = [
        ("support/mod.rs", "pub type Env = EventEnvelope;"),
        (
            "support/generative.rs",
            "fn helper() { let _event = Env::default(); }",
        ),
    ];
    assert!(
        generative_duration_terminal_fabricator_errors(&support_alias_fabricator)
            .iter()
            .any(|error| error.contains("support/mod.rs") && error.contains("EventEnvelope")),
        "synthetic support/mod.rs EventEnvelope alias must fail the support fabricator ban"
    );

    let direct_envelope = [(
        "generative_lock.rs",
        "fn helper() { EventEnvelope::new_caused_v1(id, EventKind::SleepCompleted, 0, 0, tick, key, manifest, causes); }",
    )];
    assert!(
        generative_duration_terminal_fabricator_errors(&direct_envelope)
            .iter()
            .any(|error| error.contains("directly constructs")),
        "direct terminal envelope construction synthetic must fail"
    );
}

fn support_event_envelope_scan_sources() -> BTreeSet<String> {
    [
        (
            "support/acceptance_status_manifest.rs",
            SUPPORT_ACCEPTANCE_STATUS_MANIFEST_RS,
        ),
        ("support/generative.rs", SUPPORT_GENERATIVE_RS),
        ("support/mod.rs", SUPPORT_MOD_RS),
    ]
    .into_iter()
    .map(|(path, _)| path.to_string())
    .collect()
}

fn support_directory_files() -> BTreeSet<String> {
    let support_dir = format!("{}/tests/support", env!("CARGO_MANIFEST_DIR"));
    fs::read_dir(support_dir)
        .expect("tests/support directory is readable")
        .map(|entry| {
            let entry = entry.expect("tests/support entry is readable");
            let file_name = entry
                .file_name()
                .into_string()
                .expect("support file names are UTF-8");
            format!("support/{file_name}")
        })
        .filter(|path| path.ends_with(".rs"))
        .collect()
}

fn support_directory_parity_errors(
    scanned_support_files: &BTreeSet<String>,
    directory_files: &BTreeSet<String>,
) -> Vec<String> {
    let mut errors = Vec::new();
    for missing in directory_files.difference(scanned_support_files) {
        errors.push(format!(
            "{missing} exists in tests/support but is absent from EventEnvelope scan list"
        ));
    }
    for stale in scanned_support_files.difference(directory_files) {
        errors.push(format!(
            "{stale} is enrolled in EventEnvelope scan list but no longer exists"
        ));
    }
    errors
}

fn generative_duration_terminal_fabricator_errors(sources: &[(&str, &str)]) -> Vec<String> {
    let banned_helpers = [
        "build_sleep_completion_events",
        "build_work_completion_events",
        "append_generated_duration_terminals",
        "generated_duration_completion",
    ];
    let terminal_kinds = [
        "EventKind::SleepCompleted",
        "EventKind::SleepInterrupted",
        "EventKind::WorkBlockCompleted",
        "EventKind::WorkBlockFailed",
    ];
    let mut errors = Vec::new();
    for (path, source) in sources {
        if path.starts_with("support/") && source.contains("EventEnvelope") {
            errors.push(format!(
                "{path} constructs EventEnvelope directly; support generators must use engine-emitted events"
            ));
        }
        for forbidden in banned_helpers {
            if source.contains(forbidden) {
                errors.push(format!(
                    "{path} contains banned fabricator helper {forbidden}"
                ));
            }
        }
        if source.contains("EventEnvelope::new")
            && terminal_kinds.iter().any(|kind| source.contains(kind))
        {
            errors.push(format!(
                "{path} directly constructs a duration-terminal EventEnvelope"
            ));
        }
    }
    errors
}

#[test]
fn guard_002_agent_state_keeps_typed_trace_and_diagnostic_records() {
    assert!(
        STATE_RS.contains("BTreeMap<DecisionTraceId, DecisionTraceRecord>"),
        "decision traces must remain typed records"
    );
    assert!(
        STATE_RS.contains("BTreeMap<StuckDiagnosticId, StuckDiagnosticRecord>"),
        "stuck diagnostics must remain typed records"
    );
    assert_absent(STATE_RS, "BTreeMap<DecisionTraceId, String>");
    assert_absent(STATE_RS, "BTreeMap<StuckDiagnosticId, String>");
}

#[test]
fn guard_001_authoritative_state_fields_are_not_publicly_mutable() {
    use tracewake_core::checksum::{
        AGENT_STATE_CHECKSUM_COVERAGE, PHYSICAL_STATE_CHECKSUM_COVERAGE,
    };

    for field_name in PHYSICAL_STATE_CHECKSUM_COVERAGE
        .iter()
        .chain(AGENT_STATE_CHECKSUM_COVERAGE)
        .map(|entry| entry.field_name)
    {
        let forbidden = format!("pub {field_name}:");
        assert_absent(STATE_RS, &forbidden);

        let required = format!("pub(crate) {field_name}:");
        assert!(
            STATE_RS.contains(&required),
            "authoritative state field visibility changed: {required}"
        );
    }

    assert_absent(STATE_RS, "pub fn set_need_model");
}

#[test]
fn guard_001_mutation_capability_is_private_to_event_application() {
    assert!(
        EVENTS_MOD_RS.contains("mod mutation;"),
        "mutation capability module must stay private to events"
    );
    assert_absent(EVENTS_MOD_RS, "pub mod mutation;");
    assert!(EVENTS_MUTATION_RS.contains("pub struct WorldMutationCapability"));
    assert!(EVENTS_MUTATION_RS.contains("pub struct AgentMutationCapability"));
    assert!(
        EVENTS_MUTATION_RS.contains("_private: ()"),
        "mutation capabilities must keep private fields"
    );
    assert!(
        EVENTS_APPLY_RS.contains("WorldMutationCapability::mint()"),
        "world mutation capability must be minted by event application"
    );
    assert!(
        EVENTS_APPLY_RS.contains("AgentMutationCapability::mint()"),
        "agent mutation capability must be minted by event application"
    );
}

#[test]
fn adding_event_schema_version_requires_migrator_registration() {
    use tracewake_core::events::{
        event_schema_registry, EventSchemaMigration, EventSchemaVersion, EVENT_SCHEMA_V1,
    };

    let registry = event_schema_registry();
    assert_eq!(
        registry.len(),
        EventSchemaVersion::all().len(),
        "every typed event schema version must have one registry entry"
    );
    assert_eq!(registry.len(), 1, "only EVENT_SCHEMA_V1 is live today");

    for version in EventSchemaVersion::all() {
        let entries = registry
            .iter()
            .filter(|entry| entry.version == *version)
            .collect::<Vec<_>>();
        assert_eq!(
            entries.len(),
            1,
            "event schema version {} lacks exactly one registry entry",
            version.as_str()
        );
        assert!(
            matches!(
                entries[0].migration,
                EventSchemaMigration::CurrentNoMigrationRequired
            ),
            "event schema version {} lacks a migration/no-migration proof",
            version.as_str()
        );
    }

    assert_eq!(registry[0].version.as_str(), EVENT_SCHEMA_V1);
}

#[test]
fn event_kind_metadata_is_total() {
    use tracewake_core::events::{EventKind, EventReplayHandling, EventSchemaVersion, EventStream};

    let registry = EventKind::registry();
    let cause_required_body = body_after_marker(EVENTS_ENVELOPE_RS, "const fn cause_required");
    assert!(
        cause_required_body.contains("match self"),
        "cause_required must remain an exhaustive match over EventKind"
    );
    assert!(
        !cause_required_body.contains("matches!(") && !cause_required_body.contains("_ =>"),
        "synthetic_cause_required_match_without_default must fail if cause_required regains a default arm"
    );
    assert_eq!(
        registry.len(),
        EventKind::all().len(),
        "every EventKind variant must have one metadata entry"
    );

    for kind in EventKind::all() {
        let entries = registry
            .iter()
            .filter(|metadata| metadata.kind == *kind)
            .collect::<Vec<_>>();
        assert_eq!(
            entries.len(),
            1,
            "event kind {:?} lacks exactly one metadata entry",
            kind
        );
        let metadata = entries[0];
        assert_eq!(metadata.stream, kind.stream());
        assert_eq!(metadata.schema_version, EventSchemaVersion::V1);
        assert_eq!(metadata.physical_mutating, kind.physical_mutating());
        assert_eq!(metadata.cause_required, kind.requires_cause());
        assert_eq!(
            metadata.replay_handling,
            EventReplayHandling::for_stream(metadata.stream)
        );
        if metadata.physical_mutating {
            assert_eq!(
                metadata.stream,
                EventStream::World,
                "physical-mutating event {:?} must be a world-stream event",
                kind
            );
        }
    }
}

#[test]
fn physical_mutating_event_kinds_have_explicit_world_apply_arms() {
    use tracewake_core::events::EventKind;

    let apply_body = body_after_marker(EVENTS_APPLY_RS, "fn apply_event_with_capability");
    let missing = missing_world_apply_arms(apply_body);
    assert!(
        missing.is_empty(),
        "physical-mutating event kinds lack explicit world apply arms: {missing:?}"
    );
    assert_absent(apply_body, "_ => Ok(ApplyOutcome::NonWorldNoOp)");

    let synthetic_apply = r#"
        fn apply_event_with_capability(event: &EventEnvelope) -> Result<ApplyOutcome, ApplyError> {
            match event.event_type {
                EventKind::ActorMoved => Ok(ApplyOutcome::Applied),
                _ => Ok(ApplyOutcome::NonWorldNoOp),
            }
        }
    "#;
    let synthetic_missing = missing_world_apply_arms(synthetic_apply);
    assert!(
        synthetic_missing.contains(&EventKind::FoodConsumed)
            && synthetic_apply.contains("_ => Ok(ApplyOutcome::NonWorldNoOp)"),
        "synthetic missing-arm catch-all must fail the totality guard"
    );
}

#[test]
fn agent_stream_event_kinds_have_explicit_agent_apply_arms() {
    use tracewake_core::events::EventKind;

    let apply_body = body_after_marker(EVENTS_APPLY_RS, "fn apply_agent_event_with_capability");
    let missing = missing_agent_apply_arms(apply_body);
    assert!(
        missing.is_empty(),
        "agent-stream event kinds lack explicit agent apply arms or no-op allowlist entries: {missing:?}"
    );

    let synthetic_apply = r#"
        fn apply_agent_event_with_capability(event: &EventEnvelope) -> Result<ApplyOutcome, ApplyError> {
            match event.event_type {
                EventKind::NoHumanDayStarted => Ok(ApplyOutcome::WorldNoOp),
                _ => Err(ApplyError::NonAgentEvent),
            }
        }
    "#;
    let synthetic_missing = missing_agent_apply_arms(synthetic_apply);
    assert!(
        synthetic_missing.contains(&EventKind::NeedDeltaApplied),
        "synthetic missing agent-stream apply arm must fail the totality guard"
    );
}

#[test]
fn action_emitted_event_kinds_have_cause_disposition() {
    use tracewake_core::events::EventKind;

    let action_sources = [
        ("movement.rs", MOVEMENT_RS),
        ("wait.rs", WAIT_RS),
        ("openclose.rs", OPENCLOSE_RS),
        ("checkcontainer.rs", CHECKCONTAINER_RS),
        ("takeplace.rs", TAKEPLACE_RS),
        ("perception.rs", PERCEPTION_RS),
        ("eat.rs", EAT_RS),
        ("sleep.rs", SLEEP_RS),
        ("work.rs", WORK_RS),
    ];
    for (_, source) in action_sources {
        assert!(
            source.contains("EventEnvelope::new_caused_v1"),
            "action-emitted source must construct caused envelopes"
        );
    }
    let violations = action_emitted_kind_cause_disposition_violations(&action_sources);
    assert!(
        violations.is_empty(),
        "action-emitted kinds lack required-cause metadata: {violations:?}"
    );

    let synthetic_sources = [(
        "synthetic_action.rs",
        "EventEnvelope::new_caused_v1(event_id, EventKind::ReplayProjectionRebuilt, 0, 0, tick, key, manifest, causes)",
    )];
    let synthetic_violations = action_emitted_kind_cause_disposition_violations(&synthetic_sources);
    assert!(
        synthetic_violations
            .iter()
            .any(|violation| violation
                .contains(&format!("{:?}", EventKind::ReplayProjectionRebuilt))),
        "synthetic action-emitted kind without cause-required metadata must fail"
    );
}

fn missing_world_apply_arms(source: &str) -> Vec<tracewake_core::events::EventKind> {
    tracewake_core::events::EventKind::all()
        .iter()
        .copied()
        .filter(|kind| kind.metadata().physical_mutating)
        .filter(|kind| !source.contains(&format!("EventKind::{kind:?}")))
        .collect()
}

fn missing_agent_apply_arms(source: &str) -> Vec<tracewake_core::events::EventKind> {
    use tracewake_core::events::{apply::AGENT_WORLD_NOOP_ALLOWLIST, EventKind, EventStream};

    EventKind::all()
        .iter()
        .copied()
        .filter(|kind| kind.metadata().stream == EventStream::Agent)
        .filter(|kind| !AGENT_WORLD_NOOP_ALLOWLIST.contains(kind))
        .filter(|kind| !source.contains(&format!("EventKind::{kind:?}")))
        .collect()
}

fn action_emitted_kind_cause_disposition_violations(sources: &[(&str, &str)]) -> Vec<String> {
    tracewake_core::events::EventKind::all()
        .iter()
        .copied()
        .flat_map(|kind| {
            let arm = format!("EventKind::{kind:?}");
            sources
                .iter()
                .filter(move |(_, source)| {
                    source.contains("EventEnvelope::new_caused_v1") && source.contains(&arm)
                })
                .filter(move |_| !kind.metadata().cause_required)
                .map(move |(path, _)| {
                    format!("{path}: action-emitted kind lacks required-cause metadata: {kind:?}")
                })
        })
        .collect()
}

#[test]
fn non_world_stream_cannot_change_physical_checksum() {
    use tracewake_core::checksum::{compute_physical_checksum, ChecksumContext};
    use tracewake_core::events::apply::{apply_event, ApplyOutcome};
    use tracewake_core::events::{EventEnvelope, EventKind, PayloadField};
    use tracewake_core::ids::{
        ActionId, ActorId, ContentManifestId, ContentVersion, EventId, FixtureId,
    };
    use tracewake_core::scheduler::{
        OrderingKey, ProposalSequence, SchedulePhase, SchedulerSourceId,
    };
    use tracewake_core::state::PhysicalState;
    use tracewake_core::time::SimTick;

    let context = ChecksumContext {
        fixture_id: FixtureId::new("anti_regression_fixture").unwrap(),
        content_version: ContentVersion::new("content_v1").unwrap(),
        sim_tick: SimTick::new(7),
        world_stream_position_applied: 3,
    };
    let mut state = PhysicalState::empty(tracewake_core::state::NeedModelState::new(5, 3));
    let before = compute_physical_checksum(&state, &context).checksum;
    let mut event = EventEnvelope::new_v1(
        EventId::new("event_non_world_physical_payload").unwrap(),
        EventKind::ActionRejected,
        0,
        0,
        SimTick::new(7),
        OrderingKey::new(
            SimTick::new(7),
            SchedulePhase::HumanCommand,
            SchedulerSourceId::Actor(ActorId::new("actor_tomas").unwrap()),
            ProposalSequence::new(0),
            ActionId::new("move").unwrap(),
            vec!["back_room".to_string()],
            "tie",
        ),
        ContentManifestId::new("phase1_manifest").unwrap(),
    );
    event.payload = vec![
        PayloadField::new("actor_id", "actor_tomas"),
        PayloadField::new("from_place_id", "shop_front"),
        PayloadField::new("to_place_id", "back_room"),
        PayloadField::new("door_id", "door_shop_back"),
    ];

    assert_eq!(
        apply_event(&mut state, &event),
        Ok(ApplyOutcome::NonWorldNoOp)
    );
    let after = compute_physical_checksum(&state, &context).checksum;
    assert_eq!(after, before);
}

#[test]
fn checksum_coverage_is_total_for_authoritative_state() {
    use tracewake_core::checksum::{
        AGENT_STATE_CHECKSUM_COVERAGE, PHYSICAL_STATE_CHECKSUM_COVERAGE,
    };

    let physical_coverage = PHYSICAL_STATE_CHECKSUM_COVERAGE
        .iter()
        .map(|entry| (entry.field_name, entry.field_family))
        .collect::<Vec<_>>();
    let agent_coverage = AGENT_STATE_CHECKSUM_COVERAGE
        .iter()
        .map(|entry| (entry.field_name, entry.field_family))
        .collect::<Vec<_>>();

    for entry in PHYSICAL_STATE_CHECKSUM_COVERAGE
        .iter()
        .chain(AGENT_STATE_CHECKSUM_COVERAGE)
    {
        assert!(
            !entry.field_family.is_empty(),
            "checksum coverage entry {} lacks a field family",
            entry.field_name
        );
    }

    let errors = checksum_parity_errors(STATE_RS, CHECKSUM_RS, &physical_coverage, &agent_coverage);
    assert!(
        errors.is_empty(),
        "checksum field/registry/canonical parity errors:\n{}",
        errors.join("\n")
    );
}

#[test]
fn replay_physical_state_diff_covers_checksum_families() {
    use tracewake_core::checksum::PHYSICAL_STATE_CHECKSUM_COVERAGE;

    for entry in PHYSICAL_STATE_CHECKSUM_COVERAGE {
        let diff_check = format!(
            "expected.{} != actual.{}",
            entry.field_name, entry.field_name
        );
        assert!(
            REBUILD_RS.contains(&diff_check),
            "diff_physical_state must explain checksum family {}",
            entry.field_name
        );

        let report_prefix = format!("diff.starts_with(\"{} \")", entry.field_name);
        assert!(
            REPORT_RS.contains(&report_prefix),
            "replay report must classify diff family {}",
            entry.field_name
        );
    }
}

#[test]
fn new_authoritative_field_without_checksum_registry_fails() {
    let state_source = r#"
pub struct PhysicalState {
    pub(crate) actors: BTreeMap<ActorId, ActorBody>,
    pub(crate) places: BTreeMap<PlaceId, PlaceState>,
}

pub struct AgentState {
    pub(crate) needs_by_actor: BTreeMap<ActorId, BTreeMap<NeedKind, NeedState>>,
}
"#;
    let checksum_source = r#"
lines.push(format!("actor|id={}", actor_id.as_str()));
lines.push(format!("need|actor={}", actor_id.as_str()));
"#;

    let errors = checksum_parity_errors(
        state_source,
        checksum_source,
        &[("actors", "actor")],
        &[("needs_by_actor", "need")],
    );
    assert!(
        errors
            .iter()
            .any(|error| error.contains("PhysicalState") && error.contains("places")),
        "expected missing PhysicalState.places registry coverage, got {errors:?}"
    );
}

#[test]
fn new_authoritative_field_without_canonical_checksum_line_fails() {
    let state_source = r#"
pub struct PhysicalState {
    pub(crate) actors: BTreeMap<ActorId, ActorBody>,
    pub(crate) places: BTreeMap<PlaceId, PlaceState>,
}

pub struct AgentState {
    pub(crate) needs_by_actor: BTreeMap<ActorId, BTreeMap<NeedKind, NeedState>>,
}
"#;
    let checksum_source = r#"
lines.push(format!("actor|id={}", actor_id.as_str()));
lines.push(format!("need|actor={}", actor_id.as_str()));
"#;

    let errors = checksum_parity_errors(
        state_source,
        checksum_source,
        &[("actors", "actor"), ("places", "place")],
        &[("needs_by_actor", "need")],
    );
    assert!(
        errors
            .iter()
            .any(|error| error.contains("PhysicalState.places") && error.contains("place|")),
        "expected missing PhysicalState.places canonical line, got {errors:?}"
    );
}

#[test]
fn guard_001_no_production_seed_mutation_outside_state_definition() {
    for (path, source) in core_production_sources() {
        if path == "crates/tracewake-core/src/state.rs"
            || path == "crates/tracewake-core/src/content/load.rs"
            || path == "crates/tracewake-core/src/content/validate.rs"
        {
            continue;
        }
        assert!(
            !source.contains("seed_"),
            "{path} uses seed construction mutators in production"
        );
    }
}

#[test]
fn standing_barrier_negative_fixture_runner_keeps_all_feature_boundary_lane() {
    assert_standing_barrier_negative_fixture_runner_registered(NEGATIVE_FIXTURE_RUNNER_RS);

    let synthetic = NEGATIVE_FIXTURE_RUNNER_RS.replace(
        "\"external_crate_cannot_mutate_embodied_temporal_fields\",",
        "",
    );
    assert!(
        std::panic::catch_unwind(|| {
            assert_standing_barrier_negative_fixture_runner_registered(&synthetic)
        })
        .is_err(),
        "standing-barrier topology alarm must fire if an all-feature fixture is removed"
    );
}

fn assert_standing_barrier_negative_fixture_runner_registered(source: &str) {
    for required in [
        "TRACEWAKE_CORE_TEST_SUPPORT_FEATURE",
        "production_boundary_negative_fixtures_fail_with_test_support_feature",
        "ALL_FEATURE_PRODUCTION_BOUNDARY_FIXTURES",
        "external_crate_cannot_mutate_loaded_runtime_fields",
        "external_crate_cannot_mutate_embodied_temporal_fields",
        "external_crate_cannot_construct_pipeline_context_with_runtime_aggregates",
        "external_crate_cannot_assign_scheduler_frontier",
    ] {
        assert!(
            source.contains(required),
            "negative fixture runner must keep all-feature standing-barrier witness {required}"
        );
    }
}

#[test]
fn guard_001_no_direct_state_collection_insert_outside_event_application() {
    use tracewake_core::checksum::{
        AGENT_STATE_CHECKSUM_COVERAGE, PHYSICAL_STATE_CHECKSUM_COVERAGE,
    };

    // Smoke-only source scan: direct mutation is primarily blocked by private
    // fields, private mutation capabilities, and compile-fail fixtures.
    let forbidden_writes = PHYSICAL_STATE_CHECKSUM_COVERAGE
        .iter()
        .chain(AGENT_STATE_CHECKSUM_COVERAGE)
        .map(|entry| format!(".{}.insert", entry.field_name))
        .collect::<Vec<_>>();
    for (path, source) in core_production_sources() {
        if path == "crates/tracewake-core/src/events/apply.rs" {
            continue;
        }
        for forbidden in &forbidden_writes {
            assert_absent(&source, forbidden);
        }
    }
}

#[test]
fn guard_001_actor_known_context_has_no_public_arbitrary_constructor() {
    let actor_known = production(ACTOR_KNOWN_RS);
    assert_absent(&actor_known, "pub fn from_observed_parts");
    assert!(
        actor_known.contains("pub(crate) fn from_observed_parts"),
        "observed-parts constructor must stay crate-private"
    );
}

#[test]
fn guard_0021_actor_known_context_producers_are_projection_backed() {
    let sources = core_production_sources();
    let violations = actor_known_context_constructor_violations(&sources);
    assert!(
        violations.is_empty(),
        "actor-known planning context producers must stay inside the no-human builder/classifier path: {violations:?}"
    );

    let synthetic_sources = vec![(
        "crates/tracewake-core/src/agent/synthetic.rs".to_string(),
        "pub fn build_actor_known_planning_state() -> ActorKnownPlanningContext { ActorKnownPlanningContext::from_observed_parts(todo!(), todo!(), todo!(), todo!(), todo!(), todo!(), todo!(), todo!(), todo!()) }".to_string(),
    )];
    let synthetic_violations = actor_known_context_constructor_violations(&synthetic_sources);
    assert!(
        synthetic_violations.iter().any(|violation| {
            violation.contains("retired context producer")
                || violation.contains("outside the builder/classifier surface")
        }),
        "synthetic context producer must fail the source guard"
    );

    let synthetic_return_sources = vec![(
        "crates/tracewake-core/src/agent/synthetic_return.rs".to_string(),
        "pub fn leak_context() -> ActorKnownPlanningContext { todo!() }".to_string(),
    )];
    assert!(
        actor_known_context_constructor_violations(&synthetic_return_sources)
            .iter()
            .any(|violation| violation.contains("public ActorKnownPlanningContext producer")),
        "synthetic public ActorKnownPlanningContext return producer must fail the source guard"
    );
}

#[test]
fn guard_0021_hidden_truth_gates_use_event_log_provenance() {
    let violations = hidden_truth_harness_provenance_violations(HIDDEN_TRUTH_GATES_RS);
    assert!(
        violations.is_empty(),
        "hidden-truth gates must use real applied epistemic events: {violations:?}"
    );

    let synthetic =
        "fn context(_known_edges: (), _known_food_sources: ()) { let _ = build_actor_known_planning_state(); }";
    let synthetic_violations = hidden_truth_harness_provenance_violations(synthetic);
    assert!(
        synthetic_violations.iter().any(|violation| {
            violation.contains("build_actor_known_planning_state")
                || violation.contains("applying epistemic events")
                || violation.contains("_known_edges")
                || violation.contains("_known_food_sources")
        }),
        "synthetic fabricated hidden-truth harness must fail the source guard"
    );

    let missing_witness =
        HIDDEN_TRUTH_GATES_RS.replace("planner_hidden_truth_fixture_witness_errors", "");
    assert!(
        hidden_truth_harness_provenance_violations(&missing_witness)
            .iter()
            .any(|violation| violation.contains("adversarial fixture witness")),
        "synthetic missing planner witness must fail the source guard"
    );
}

#[test]
fn guard_0021_fabricated_visible_local_event_id_is_retired() {
    let violations = fabricated_planning_event_id_violations(&production_sources());
    assert!(
        violations.is_empty(),
        "fabricated visible-local planning provenance id must not appear in production source: {violations:?}"
    );

    let synthetic_sources = vec![(
        "crates/tracewake-core/src/agent/synthetic.rs".to_string(),
        "let _ = EventId::new(\"event_visible_local_planning_state\");".to_string(),
    )];
    let synthetic_violations = fabricated_planning_event_id_violations(&synthetic_sources);
    assert!(
        synthetic_violations
            .iter()
            .any(|path| path.ends_with("synthetic.rs")),
        "synthetic fabricated event id must fail the source guard"
    );
}

#[test]
fn guard_001_hidden_truth_audit_is_derived_from_provenance_not_tags() {
    let actor_known = production(ACTOR_KNOWN_RS);
    assert!(
        actor_known.contains(".all(ActorKnownFact::is_actor_known)"),
        "hidden-truth audit must derive from fact provenance"
    );
    for source in [
        production(ACTOR_KNOWN_RS),
        production(DECISION_RS),
        production(HTN_RS),
        production(PLANNER_RS),
    ] {
        assert_absent(source, "actor_known_only: true");
    }
}

#[test]
fn guard_006_continue_routine_marker_alone_is_not_behavioral_progress() {
    let scheduler = production(SCHEDULER_RS);
    assert!(
        scheduler.contains("EventKind::ContinueRoutineProposed"),
        "progress guard must explicitly inspect continue-routine marker events"
    );
    assert!(
        scheduler.contains("behavioral_progress"),
        "continue-routine progress must depend on explicit behavioral_progress payload"
    );
    assert!(
        scheduler.contains("EventKind::ActionRejected"),
        "rejected actions must be excluded from behavioral progress"
    );
}

#[test]
fn guard_0057_embodied_continue_non_proposed_outcome_is_typed_stuck() {
    let runtime_session = production(RUNTIME_SESSION_RS);
    assert!(
        runtime_session.contains("ActorDecisionTransactionOutcome::Stuck"),
        "embodied continue_routine must handle stuck actor-decision outcomes explicitly"
    );
    assert!(
        runtime_session.contains("run_embodied_continue_routine_stuck_outcome"),
        "embodied continue_routine must return a typed stuck receipt instead of falling back to the marker receipt"
    );
    assert!(
        runtime_session.contains("build_actor_stuck_diagnostic_event"),
        "embodied continue_routine stuck outcomes must reuse the typed stuck diagnostic event schema"
    );
    assert!(
        runtime_session.contains("append_embodied_routine_stuck_diagnostics"),
        "embodied continue_routine submissions must scan the scheduler-owned stuck diagnostics"
    );
    assert!(
        runtime_session.contains("recursive continue_routine follow-on"),
        "recursive continue_routine follow-ons must be classified as typed stuck, not committed as another marker"
    );
}

#[test]
fn guard_0057_continue_routine_progress_of_record_is_follow_on() {
    let continue_routine = production(CONTINUE_ROUTINE_RS);
    assert!(
        continue_routine.contains("PayloadField::new(\"intention_mutated\", \"false\")"),
        "continue_routine marker must not mutate the active intention"
    );
    assert!(
        continue_routine.contains("PayloadField::new(\"behavioral_progress\", \"false\")"),
        "continue_routine marker must remain explicit non-progress"
    );
    assert_absent(
        continue_routine,
        "PayloadField::new(\"behavioral_progress\", \"true\")",
    );

    let runtime_session = production(RUNTIME_SESSION_RS);
    assert!(
        runtime_session.contains("let mut follow_on_result = run_pipeline"),
        "embodied continue_routine must commit the follow-on through the shared pipeline"
    );
    assert!(
        runtime_session.contains("first_appended_event(&follow_on_result)"),
        "follow-on committed events must be the ordinary-event ancestry source"
    );
    assert!(
        runtime_session.contains("marker_result.appended_events.clone()"),
        "the marker must be reported as marker ancestry, not counted as progress by itself"
    );

    let scheduler = production(SCHEDULER_RS);
    assert!(
        scheduler.contains("event.event_type != EventKind::ContinueRoutineProposed"),
        "scheduler progress accounting must special-case continue_routine markers"
    );
    assert!(
        scheduler.contains("field.key == \"behavioral_progress\" && field.value == \"true\""),
        "a continue_routine marker may count as progress only through an explicit true payload, which the marker action does not emit"
    );
}

#[test]
fn guard_0058_embodied_routine_family_has_no_pre_intention_workplace_selector() {
    let runtime_session = production(RUNTIME_SESSION_RS);
    let errors = embodied_routine_family_authority_errors(&runtime_session);
    assert!(
        errors.is_empty(),
        "embodied routine family must derive from active intention before workplace context: {errors:?}"
    );

    let synthetic = r#"
        fn embodied_routine_window_family(
            agent_state: &AgentState,
            actor_id: &ActorId,
            actor_known_context: &ActorKnownPlanningContext,
        ) -> Option<RoutineFamily> {
            if actor_known_context.known_workplaces().values().any(|place_id| place_id == actor_known_context.current_place_id()) {
                return Some(RoutineFamily::WorkBlock);
            }
            let active_intention_id = agent_state.active_intention_by_actor().get(actor_id)?;
            let _active = agent_state.intentions().get(active_intention_id)?;
            Some(RoutineFamily::EatMeal)
        }
    "#;
    assert!(
        embodied_routine_family_authority_errors(synthetic)
            .iter()
            .any(|error| error.contains("known_workplaces before active_intention")),
        "synthetic_workplace_before_active_intention must fail the guard"
    );
}

#[test]
fn guard_0058_embodied_continue_time_advancing_follow_on_is_gated() {
    let runtime_session = production(RUNTIME_SESSION_RS);
    let errors = embodied_temporal_gateway_errors(&runtime_session);
    assert!(
        errors.is_empty(),
        "embodied continue_routine must gate time-advancing follow-ons before run_pipeline: {errors:?}"
    );

    let synthetic = r#"
        fn run_embodied_continue_routine_follow_on(&mut self) {
            let follow_on_proposal = proposed.proposal.into_proposal();
            let mut context = PipelineContext {};
            let mut follow_on_result = run_pipeline(&mut context, &follow_on_proposal);
            if embodied_follow_on_advances_time(&follow_on_proposal) {
                return self.run_embodied_continue_routine_stuck_outcome();
            }
        }
    "#;
    assert!(
        embodied_temporal_gateway_errors(synthetic)
            .iter()
            .any(|error| error.contains("temporal gateway must precede run_pipeline")),
        "synthetic_direct_wait_follow_on must fail the guard"
    );
}

#[test]
fn guard_0058_embodied_continue_success_path_not_current_stuck() {
    let runtime_session = production(RUNTIME_SESSION_RS);
    let errors = embodied_current_stuck_receipt_errors(&runtime_session);
    assert!(
        errors.is_empty(),
        "embodied continue success receipts must not fabricate a current stuck diagnostic: {errors:?}"
    );

    let synthetic = r#"
        fn run_embodied_continue_routine_follow_on(&mut self) {
            let current_stuck_events = build_actor_stuck_diagnostic_event(actor_id, tick, process, diagnostic, manifest, None);
            let outcome = ActorDecisionTransaction::run(input);
            let proposed = match outcome { ActorDecisionTransactionOutcome::Proposed(proposed) => proposed, _ => todo!() };
            let mut follow_on_result = run_pipeline(&mut context, &follow_on_proposal);
            let mut receipt_prefix = marker_result.appended_events.clone();
            receipt_prefix.extend(current_stuck_events);
            follow_on_result.appended_events.splice(0..0, receipt_prefix);
        }
    "#;
    assert!(
        embodied_current_stuck_receipt_errors(synthetic)
            .iter()
            .any(
                |error| error.contains("current stuck diagnostic before transaction outcome")
                    || error.contains("success receipt prefixes current stuck")
            ),
        "synthetic_success_prefixed_current_stuck must fail the guard"
    );
}

#[test]
fn guard_0058_tui_continue_routine_forwards_only() {
    let app = production(TUI_APP_RS);
    let errors = tui_continue_routine_forwarding_errors(&app);
    assert!(
        errors.is_empty(),
        "TUI app must forward continue_routine through RuntimeCommand without routine authority: {errors:?}"
    );

    let synthetic = r#"
        use tracewake_core::agent::{RoutineFamily, ActorDecisionTransaction};
        fn submit_semantic_action() {
            let routine_window_family = Some(RoutineFamily::WorkBlock);
            let target_ids = vec!["hidden_workshop".to_string()];
            let _ = ActorDecisionTransaction::run(input);
            let _ = RuntimeCommand::submit_semantic_action(controller, actor, entry, view);
        }
    "#;
    assert!(
        tui_continue_routine_forwarding_errors(synthetic)
            .iter()
            .any(|error| error.contains("routine authority token")),
        "synthetic_tui_routine_selection must fail the guard"
    );
}

fn embodied_routine_family_authority_errors(source: &str) -> Vec<String> {
    let Some(body) = function_body_if_present(source, "fn embodied_routine_window_family") else {
        return vec!["missing embodied_routine_window_family".to_string()];
    };
    let active_index = body.find("active_intention_by_actor()");
    let workplace_index = body.find("known_workplaces()");
    let mut errors = Vec::new();
    if active_index.is_none() {
        errors.push("missing active_intention_by_actor authority".to_string());
    }
    if let (Some(workplace), Some(active)) = (workplace_index, active_index) {
        if workplace < active {
            errors.push("known_workplaces before active_intention authority".to_string());
        }
    }
    if body.contains("return Some(RoutineFamily::WorkBlock)")
        && workplace_index.is_some_and(|workplace| {
            active_index.is_none() || workplace < active_index.unwrap_or(usize::MAX)
        })
    {
        errors.push("workplace-derived WorkBlock shortcut before active intention".to_string());
    }
    errors
}

fn embodied_temporal_gateway_errors(source: &str) -> Vec<String> {
    let Some(body) = function_body_if_present(source, "fn run_embodied_continue_routine_follow_on")
    else {
        return vec!["missing run_embodied_continue_routine_follow_on".to_string()];
    };
    let gate = body.find("embodied_follow_on_advances_time(&follow_on_proposal)");
    let pipeline = body.find("run_pipeline(&mut context, &follow_on_proposal)");
    let mut errors = Vec::new();
    if gate.is_none() {
        errors.push("missing embodied temporal gateway".to_string());
    }
    if let (Some(gate), Some(pipeline)) = (gate, pipeline) {
        if pipeline < gate {
            errors.push("temporal gateway must precede run_pipeline".to_string());
        }
    }
    errors
}

fn embodied_current_stuck_receipt_errors(source: &str) -> Vec<String> {
    let Some(body) = function_body_if_present(source, "fn run_embodied_continue_routine_follow_on")
    else {
        return vec!["missing run_embodied_continue_routine_follow_on".to_string()];
    };
    let mut errors = Vec::new();
    let outcome = body.find("let outcome = ActorDecisionTransaction::run");
    let current_stuck = body.find("build_actor_stuck_diagnostic_event");
    if let (Some(current_stuck), Some(outcome)) = (current_stuck, outcome) {
        if current_stuck < outcome {
            errors.push("current stuck diagnostic before transaction outcome".to_string());
        }
    }
    if !body.contains("prior_scheduler_stuck_events") {
        errors.push("missing prior_scheduler_stuck_events naming".to_string());
    }
    for forbidden in [
        "receipt_prefix.extend(current_stuck_events)",
        "receipt_prefix.extend(appended)",
        "receipt_prefix.extend(current_stuck)",
    ] {
        if body.contains(forbidden) {
            errors.push(format!(
                "success receipt prefixes current stuck: {forbidden}"
            ));
        }
    }
    errors
}

fn tui_continue_routine_forwarding_errors(source: &str) -> Vec<String> {
    let mut errors = Vec::new();
    if !source.contains("RuntimeCommand::submit_semantic_action") {
        errors.push("missing RuntimeCommand::submit_semantic_action forwarding".to_string());
    }
    for forbidden in [
        "ActorDecisionTransaction",
        "RoutineFamily",
        "routine_window_family",
        "resolve_routine_step_follow_on",
        "run_embodied_continue_routine_follow_on",
        "run_pipeline(",
    ] {
        if source.contains(forbidden) {
            errors.push(format!(
                "TUI app contains routine authority token: {forbidden}"
            ));
        }
    }
    errors
}

#[test]
fn guard_0059_scheduler_routine_family_authority_cannot_bypass_active_intention() {
    let scheduler = production(SCHEDULER_RS);
    let errors = scheduler_routine_family_authority_errors(&scheduler);
    assert!(
        errors.is_empty(),
        "scheduler routine-window family must be bound to active intention: {errors:?}"
    );

    let synthetic = r#"
        fn routine_window_family(
            agent_state: &AgentState,
            actor_id: &ActorId,
            window: &DayWindow,
            actor_known_state: &ActorKnownPlanningContext,
        ) -> Option<RoutineFamily> {
            if window.start_tick.value() >= 8 && window.start_tick.value() < 17 {
                return Some(RoutineFamily::WorkBlock);
            }
            let active = active_intention_for_actor(agent_state, actor_id)?;
            let _ = actor_known_state.current_place_id();
            Some(RoutineFamily::EatMeal)
        }
    "#;
    assert!(
        scheduler_routine_family_authority_errors(synthetic)
            .iter()
            .any(|error| error.contains("window-keyed family before active intention")),
        "synthetic_0059_window_keyed_routine_family must fail the source guard"
    );
}

#[test]
fn guard_0059_no_clock_keyed_routine_family_selector_without_active_intention_binding() {
    let scheduler = production(SCHEDULER_RS);
    let errors = scheduler_clock_keyed_routine_family_selector_errors(&scheduler);
    assert!(
        errors.is_empty(),
        "scheduler routine-window family must not select family by clock/execution ordering: {errors:?}"
    );

    let synthetic = r#"
        fn routine_window_family(
            agent_state: &AgentState,
            actor_id: &ActorId,
            window: &DayWindow,
            actor_known_state: &ActorKnownPlanningContext,
        ) -> Option<RoutineFamily> {
            let (_id, execution) = agent_state
                .routine_executions
                .iter()
                .filter(|(_, execution)| &execution.actor_id == actor_id)
                .filter(|(_, execution)| execution.start_tick <= window.start_tick)
                .min_by(|(_, left), (_, right)| {
                    left.start_tick
                        .cmp(&right.start_tick)
                        .then_with(|| left.execution_id.cmp(&right.execution_id))
                })?;
            let _active = active_intention_for_actor(agent_state, actor_id)?;
            let _ = actor_known_state.current_place_id();
            Some(execution.family)
        }
    "#;
    let synthetic_errors = scheduler_clock_keyed_routine_family_selector_errors(synthetic);
    assert!(
        synthetic_errors
            .iter()
            .any(|error| error.contains("min_by routine execution selector")),
        "synthetic_0059_eligible_execution_min_by_start must fail the clock-keyed selector guard: {synthetic_errors:?}"
    );
}

#[test]
fn guard_0059_synthetic_negative_census_is_live() {
    let anti_regression_source = production(ANTI_REGRESSION_GUARDS_RS);
    for negative_id in synthetic_0059_negative_ids() {
        assert!(
            anti_regression_source.contains(negative_id),
            "{negative_id} must be registered in the 0059 synthetic negative census"
        );
    }
    let transaction = production(TRANSACTION_RS);
    let generation = production(GENERATION_RS);
    let transaction_errors = routine_window_hint_authority_errors(&transaction);
    let generation_errors = routine_window_candidate_generation_errors(&generation);
    assert!(
        transaction_errors.is_empty(),
        "routine-window hint consumer must fail closed through active intention: {transaction_errors:?}"
    );
    assert!(
        generation_errors.is_empty(),
        "routine-window candidate generation must filter RoutineDuty through active intention: {generation_errors:?}"
    );

    let without_active = r#"
        fn routine_window_goal_from_hint(
            family: Option<RoutineFamily>,
            active_intention: Option<&Intention>,
        ) -> RoutineWindowHint {
            let hint_goal = goal_for_routine_family(family?)?;
            RoutineWindowHint { goal_kind: Some(hint_goal), diagnostic: None }
        }
    "#;
    assert!(
        routine_window_hint_authority_errors(without_active)
            .iter()
            .any(|error| error.contains("hint accepted without active-intention gate")),
        "synthetic_0059_routine_window_family_without_active_intention must fail the consumer guard"
    );

    let conflicting_hint = r#"
        fn routine_window_goal_from_hint(
            family: Option<RoutineFamily>,
            active_intention: Option<&Intention>,
        ) -> RoutineWindowHint {
            let hint_goal = goal_for_routine_family(family?)?;
            let Some(active_intention) = active_intention else {
                return RoutineWindowHint { goal_kind: None, diagnostic: Some("routine_window_family_ignored_without_active_intention".to_string()) };
            };
            let _ = active_intention;
            RoutineWindowHint { goal_kind: Some(hint_goal), diagnostic: None }
        }
    "#;
    assert!(
        routine_window_hint_authority_errors(conflicting_hint)
            .iter()
            .any(|error| error.contains("missing conflict diagnostic")),
        "synthetic_0059_conflicting_routine_window_hint must fail the consumer guard"
    );

    let other_actor = r#"
        fn routine_window_family(
            agent_state: &AgentState,
            actor_id: &ActorId,
            window: &DayWindow,
            actor_known_state: &ActorKnownPlanningContext,
        ) -> Option<RoutineFamily> {
            let active = active_intention_for_actor(agent_state, actor_id)?;
            let execution = agent_state
                .routine_executions
                .values()
                .find(|execution| execution.start_tick <= window.start_tick)?;
            let _ = actor_known_state.current_place_id();
            Some(execution.family)
        }
    "#;
    assert!(
        scheduler_routine_family_authority_errors(other_actor)
            .iter()
            .any(|error| error.contains("routine execution scan must filter actor")),
        "synthetic_0059_other_actor_execution_temptation must fail the producer guard"
    );

    let generation_without_active_filter = r#"
        fn generate_candidate_goals(input: &CandidateGenerationInput) -> CandidateGenerationOutput {
            if let Some(goal_kind) = input.routine_window_goal {
                candidates.push(candidate(
                    input,
                    goal_kind.stable_id(),
                    CandidateGoalSource::RoutineDuty,
                    goal_kind,
                    GoalPriority::RoutineWindowDuty,
                    "routine window is active",
                    Vec::new(),
                    ApplicabilityResult::Applicable,
                    None,
                ));
            }
            CandidateGenerationOutput { candidates, actor_known_inputs_used: Vec::new() }
        }
    "#;
    assert!(
        routine_window_candidate_generation_errors(generation_without_active_filter)
            .iter()
            .any(|error| error.contains("RoutineDuty candidate lacks active-intention compatibility filter")),
        "synthetic_0059_routine_window_family_without_active_intention must fail candidate generation"
    );
}

fn synthetic_0059_negative_ids() -> [&'static str; 5] {
    [
        "synthetic_0059_window_keyed_routine_family",
        "synthetic_0059_eligible_execution_min_by_start",
        "synthetic_0059_routine_window_family_without_active_intention",
        "synthetic_0059_conflicting_routine_window_hint",
        "synthetic_0059_other_actor_execution_temptation",
    ]
}

fn scheduler_routine_family_authority_errors(source: &str) -> Vec<String> {
    let Some(body) = function_body_if_present(source, "fn routine_window_family") else {
        return vec!["missing routine_window_family".to_string()];
    };
    let active_index = body.find("active_intention_for_actor");
    let window_index = body.find("window.start_tick");
    let routine_executions_index = body.find("routine_executions");
    let selected_method_index = body.find("selected_routine_method");
    let actor_filter_index = body.find("&execution.actor_id == actor_id");
    let mut errors = Vec::new();

    if active_index.is_none() {
        errors.push("missing active_intention_for_actor authority".to_string());
    }
    if selected_method_index.is_none() {
        errors.push("missing selected_routine_method authority".to_string());
    }
    if let (Some(window), Some(active)) = (window_index, active_index) {
        if window < active {
            errors.push("window-keyed family before active intention".to_string());
        }
    }
    if let (Some(routine_executions), Some(active)) = (routine_executions_index, active_index) {
        if routine_executions < active {
            errors.push("routine executions inspected before active intention".to_string());
        }
    }
    if routine_executions_index.is_some()
        && actor_filter_index.is_none_or(|actor_filter| {
            routine_executions_index.is_some_and(|scan| actor_filter < scan)
        })
    {
        errors.push("routine execution scan must filter actor".to_string());
    }
    if body.contains("return Some(RoutineFamily::")
        && active_index.is_none_or(|active| {
            body.find("return Some(RoutineFamily::")
                .is_some_and(|return_index| return_index < active)
        })
    {
        errors.push("routine family return before active intention".to_string());
    }
    errors
}

fn scheduler_clock_keyed_routine_family_selector_errors(source: &str) -> Vec<String> {
    let Some(body) = function_body_if_present(source, "fn routine_window_family") else {
        return vec!["missing routine_window_family".to_string()];
    };
    let mut errors = Vec::new();
    for forbidden in [
        ".min_by(",
        "eligible_routine_execution_for_actor(",
        "execution_id.cmp",
    ] {
        if body.contains(forbidden) {
            errors.push(format!(
                "routine_window_family contains clock/execution-keyed selector: {forbidden}"
            ));
        }
    }
    if body.contains(".min_by(|") {
        errors.push("min_by routine execution selector".to_string());
    }
    let active_index = body.find("active_intention_for_actor");
    for clock_token in ["window.start_tick", "start_tick", "deadline_tick"] {
        if let Some(token_index) = body.find(clock_token) {
            if active_index.is_none_or(|active| token_index < active) {
                errors.push(format!(
                    "clock token {clock_token} appears before active-intention binding"
                ));
            }
        }
    }
    errors
}

fn routine_window_hint_authority_errors(source: &str) -> Vec<String> {
    let Some(body) = function_body_if_present(source, "fn routine_window_goal_from_hint") else {
        return vec!["missing routine_window_goal_from_hint".to_string()];
    };
    let mut errors = Vec::new();
    if !body.contains("active_intention") {
        errors.push("missing active_intention parameter".to_string());
    }
    if !body.contains("routine_window_goal_matches_active_intention") {
        errors.push("missing conflict diagnostic path".to_string());
    }
    if !body.contains("routine_window_family_ignored_without_active_intention") {
        errors.push("missing no-active diagnostic".to_string());
    }
    if !body.contains("routine_window_family_ignored_conflicts_with_active_intention") {
        errors.push("missing conflict diagnostic".to_string());
    }
    let accepted_hint = body.find("goal_kind: Some(hint_goal)");
    let active_gate = body.find("let Some(active_intention)");
    if let Some(accepted_hint) = accepted_hint {
        if active_gate.is_none_or(|gate| accepted_hint < gate)
            || !body.contains("routine_window_goal_matches_active_intention")
        {
            errors.push("hint accepted without active-intention gate".to_string());
        }
    }
    errors
}

fn routine_window_candidate_generation_errors(source: &str) -> Vec<String> {
    let Some(body) = function_body_if_present(source, "fn generate_candidate_goals") else {
        return vec!["missing generate_candidate_goals".to_string()];
    };
    let routine_duty = body.find("CandidateGoalSource::RoutineDuty");
    if routine_duty.is_some()
        && !(body.contains("input")
            && body.contains("active_intention")
            && body.contains("routine_window_goal_matches_active_intention"))
    {
        return vec![
            "RoutineDuty candidate lacks active-intention compatibility filter".to_string(),
        ];
    }
    Vec::new()
}

#[test]
fn guard_007_mutation_efficacy_notes_cover_high_risk_shortcuts() {
    let mutation_notes = [
        (
            "routine-family dispatch",
            "Adding `RoutineFamily::EatMeal => Some(GoalKind::Eat)` to scheduler.rs must fail guard_006_scheduler_has_no_routine_family_to_primitive_dispatch.",
        ),
        (
            "proposal-param need read",
            "Adding `proposal.parameters.get(\"current_hunger\")` to a production validator must fail guard_003_work_eat_sleep_validators_do_not_read_need_values_from_proposal_parameters.",
        ),
        (
            "string diagnostic map",
            "Changing AgentState diagnostics to `BTreeMap<StuckDiagnosticId, String>` must fail guard_002_agent_state_keeps_typed_trace_and_diagnostic_records.",
        ),
    ];

    assert_eq!(mutation_notes.len(), 3);
    for (target, note) in mutation_notes {
        assert!(
            note.contains("must fail"),
            "{target} mutation lacks a failure expectation"
        );
    }
}

#[test]
fn guard_008_action_registry_uses_typed_scopes_not_phase1_boolean() {
    use tracewake_core::actions::{ActionRegistry, ActionScope};
    use tracewake_core::ids::ActionId;

    let registry_source = production(ACTIONS_REGISTRY_RS);
    let retired_flag = ["phase1", "_implemented"].concat();
    assert_absent(&registry_source, &retired_flag);
    assert!(
        registry_source.contains("pub enum ActionScope"),
        "action registry must expose a typed action scope"
    );
    assert!(
        registry_source.contains("pub scope: ActionScope"),
        "action definitions must carry typed scope data"
    );
    assert!(
        !registry_source.contains("scope: ActionScope::Phase1"),
        "generic action constructors must not hard-code Phase1 scope"
    );

    let mut registry = ActionRegistry::new();
    registry.register_phase1_movement_open_close();
    registry.register_phase1_take_place();
    registry.register_phase1_inspect_wait();
    registry.register_phase2a_epistemics();
    registry.register_phase3a_sleep();
    registry.register_phase3a_eat();
    registry.register_phase3a_work();
    registry.register_phase3a_continue_routine();

    let expected = [
        ("move", ActionScope::Phase1),
        ("open", ActionScope::Phase1),
        ("close", ActionScope::Phase1),
        ("take", ActionScope::Phase1),
        ("place", ActionScope::Phase1),
        ("look", ActionScope::Phase1),
        ("inspect_place", ActionScope::Phase1),
        ("inspect_entity", ActionScope::Phase1),
        ("wait", ActionScope::Phase1),
        ("check_container", ActionScope::Phase2AHistorical),
        ("truthful_accuse_probe", ActionScope::Phase2AHistorical),
        ("sleep", ActionScope::Phase3AHistorical),
        ("eat", ActionScope::Phase3AHistorical),
        ("work_block", ActionScope::Phase3AHistorical),
        ("continue_routine", ActionScope::Phase3AHistorical),
    ];

    for (action_id, scope) in expected {
        assert_eq!(
            registry
                .get(&ActionId::new(action_id).unwrap())
                .map(|definition| definition.scope),
            Some(scope),
            "{action_id} must carry its explicit action scope"
        );
    }
    assert_eq!(registry.definitions().count(), expected.len());
}

#[test]
fn guard_008_phase1_loader_does_not_register_later_phase_actions() {
    let load_source = production(CONTENT_LOAD_RS);
    let violations = phase1_loader_later_phase_registration_violations(&load_source);
    assert!(
        violations.is_empty(),
        "Phase 1 fixture loading must not directly register later-phase actions; this source guard is secondary to ActionScope/FixtureScope typing. Violations: {violations:?}"
    );

    let registry_source = production(ACTIONS_REGISTRY_RS);
    assert!(
        !registry_source.contains("scope: ActionScope::Phase1"),
        "generic action constructors must not hard-code Phase1 scope for later-phase registrations"
    );
}

#[test]
fn guard_008_phase1_loader_source_guard_has_mutation_self_coverage() {
    let mut mutated = CONTENT_LOAD_RS.to_string();
    mutated = mutated.replace(
        "FixtureScope::Phase1 => {}",
        "FixtureScope::Phase1 => { registry.register_phase2a_epistemics(); }",
    );
    let violations = phase1_loader_later_phase_registration_violations(&mutated);
    assert!(
        violations
            .iter()
            .any(|violation| violation.contains("FixtureScope::Phase1 arm calls")),
        "source guard must fail on a deliberate Phase 1 later-phase registration mutation"
    );
}
