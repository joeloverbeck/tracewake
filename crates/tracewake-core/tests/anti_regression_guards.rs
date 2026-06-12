mod support;

use std::collections::BTreeSet;

use support::{AgentSeed, PhysicalSeed};

const SCHEDULER_RS: &str = include_str!("../src/scheduler.rs");
const ACTOR_KNOWN_RS: &str = include_str!("../src/agent/actor_known.rs");
const NO_HUMAN_SURFACE_RS: &str = include_str!("../src/agent/no_human_surface.rs");
const PERCEPTION_RS: &str = include_str!("../src/agent/perception.rs");
const TRANSACTION_RS: &str = include_str!("../src/agent/transaction.rs");
const DECISION_RS: &str = include_str!("../src/agent/decision.rs");
const PIPELINE_RS: &str = include_str!("../src/actions/pipeline.rs");
const HTN_RS: &str = include_str!("../src/agent/htn.rs");
const PLANNER_RS: &str = include_str!("../src/agent/planner.rs");
const STATE_RS: &str = include_str!("../src/state.rs");
const CHECKSUM_RS: &str = include_str!("../src/checksum.rs");
const EVENTS_MOD_RS: &str = include_str!("../src/events/mod.rs");
const EVENTS_APPLY_RS: &str = include_str!("../src/events/apply.rs");
const EVENTS_MUTATION_RS: &str = include_str!("../src/events/mutation.rs");
const EPISTEMIC_PROJECTION_RS: &str = include_str!("../src/epistemics/projection.rs");
const EAT_RS: &str = include_str!("../src/actions/defs/eat.rs");
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
const VIEW_MODELS_RS: &str = include_str!("../src/view_models.rs");
const REBUILD_RS: &str = include_str!("../src/replay/rebuild.rs");
const REPORT_RS: &str = include_str!("../src/replay/report.rs");
const GENERATIVE_LOCK_RS: &str = include_str!("generative_lock.rs");
const HIDDEN_TRUTH_GATES_RS: &str = include_str!("hidden_truth_gates.rs");
const ANTI_REGRESSION_GUARDS_RS: &str = include_str!("anti_regression_guards.rs");
const SUPPORT_GENERATIVE_RS: &str = include_str!("support/generative.rs");
const CONTENT_LOAD_RS: &str = include_str!("../../tracewake-content/src/load.rs");
const TUI_APP_RS: &str = include_str!("../../tracewake-tui/src/app.rs");
const TUI_RENDER_RS: &str = include_str!("../../tracewake-tui/src/render.rs");
const MUTANTS_TOML: &str = include_str!("../../../.cargo/mutants.toml");
const MUTANTS_BASELINE_MISSES: &str = include_str!("../../../.cargo/mutants-baseline-misses.txt");
const MUTANTS_BASELINE_LEDGER: &str =
    include_str!("../../../reports/0020_mutants_baseline_disposition.md");
const ACCEPTANCE_0021_REPORT: &str =
    include_str!("../../../reports/0021_ord_life_cert_scoped_acceptance.md");
const CI_YML: &str = include_str!("../../../.github/workflows/ci.yml");

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
    witness_count: usize,
    witness_min: usize,
}

const EMBODIED_SURFACE_FIELD_PRODUCERS: &[EmbodiedSurfaceFieldProducer] =
    &[
        EmbodiedSurfaceFieldProducer {
            struct_name: "EmbodiedViewModel",
            field_name: "notebook",
            source_path: "tracewake-tui/src/app.rs",
            producer_snippet: "view.notebook = Some(build_notebook_view",
            cite: "docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md",
            rationale: "The core projection builds the embodied shell and the TUI boundary attaches the actor-known notebook from the same sealed view context.",
        },
        EmbodiedSurfaceFieldProducer {
            struct_name: "EmbodiedViewModel",
            field_name: "debug_available",
            source_path: "tracewake-tui/src/app.rs",
            producer_snippet: "view.debug_available = true;",
            cite: "specs/0021_PHASE_3A_POSSESSION_REBIND_HYGIENE_GUARD_VACUITY_CLOSURE_HARNESS_PROVENANCE_FIDELITY_AND_REJECT_LOUDLY_REPLAY_POSTURE_HARDENING_SPEC.md",
            rationale: "Core leaves debug availability false; the TUI boundary derives it from the presence of debug panel wiring.",
        },
        EmbodiedSurfaceFieldProducer {
            struct_name: "ActionAvailability",
            field_name: "debug_only_diagnostics",
            source_path: "tracewake-core/src/view_models.rs",
            producer_snippet: "debug_only_diagnostics: Vec<String>",
            cite: "specs/0021_PHASE_3A_POSSESSION_REBIND_HYGIENE_GUARD_VACUITY_CLOSURE_HARNESS_PROVENANCE_FIDELITY_AND_REJECT_LOUDLY_REPLAY_POSTURE_HARDENING_SPEC.md",
            rationale: "Current production action availability keeps actor-safe provenance populated and defers debug-only diagnostics until a concrete debug-only disabled site needs them.",
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
            && producer_sources.iter().any(|(path, source)| {
                *path == entry.source_path && source.contains(entry.producer_snippet)
            })
    })
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
            || value.starts_with("false"))
    }) || body
        .lines()
        .any(|line| line.trim() == format!("{field_name},"))
}

fn source_has_field_consumer(source: &str, field_name: &str) -> bool {
    source.contains(&format!(".{field_name}"))
        || source.contains(&format!("{field_name}()"))
        || source.contains(&format!("{field_name} = Some("))
        || source.contains(&format!("{field_name} = true"))
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
        "crates/tracewake-tui/Cargo.toml",
        "dependencies",
        "tracewake-content",
    ),
    (
        "crates/tracewake-tui/Cargo.toml",
        "dependencies",
        "tracewake-core",
    ),
];

const WORKSPACE_SOURCE_CLASSIFICATIONS: &[WorkspaceSourceClassification] = &[
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/aged_food_record_surfaces_as_remembered_belief_not_observation_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/container_item_move_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/debug_attach_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/debug_omniscience_excluded_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-content/src/fixtures/door_access_001.rs", class: WorkspaceSourceClass::Exempt { rationale: CONTENT_RATIONALE } },
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
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/agent/trace.rs", class: WorkspaceSourceClass::GuardedLayer },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/agent/transaction.rs", class: WorkspaceSourceClass::GuardedLayer },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/checksum.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_FOUNDATION_RATIONALE } },
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

const MUTATION_PERIMETER_REQUIRED_FILTERS: &[&str] = &[
    "-f 'crates/tracewake-core/src/agent/**'",
    "-f 'crates/tracewake-core/src/scheduler*'",
    "-f 'crates/tracewake-core/src/projections*'",
    "-f 'crates/tracewake-core/src/actions/pipeline.rs'",
    "-f 'crates/tracewake-core/src/actions/defs/eat.rs'",
    "-f 'crates/tracewake-core/src/actions/defs/sleep.rs'",
    "-f 'crates/tracewake-core/src/actions/defs/work.rs'",
];

const MUTATION_PERIMETER_CANARY_PATHS: &[&str] = &[
    "crates/tracewake-core/src/agent/transaction.rs",
    "crates/tracewake-core/src/scheduler.rs",
    "crates/tracewake-core/src/projections.rs",
    "crates/tracewake-core/src/actions/pipeline.rs",
    "crates/tracewake-core/src/actions/defs/eat.rs",
    "crates/tracewake-core/src/actions/defs/sleep.rs",
    "crates/tracewake-core/src/actions/defs/work.rs",
];

const MUTANTS_BASELINE_NORMALIZED_COUNT: usize = 143;
const MUTANTS_BASELINE_NORMALIZED_FNV1A64: u64 = 0xbd18_55a5_ee82_b428;
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
const META_LOCK_REGISTRY_MIN_ENTRIES: usize = 35;

const META_LOCK_REGISTRY: &[MetaLockRegistryEntry] = &[
    MetaLockRegistryEntry {
        lock_id: "meta_lock_registry_census",
        negative_id: "synthetic_meta_lock_without_negative",
        routing: MetaLockRouting::SharedScan,
        witness_count: META_LOCK_REGISTRY_MIN_ENTRIES,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "mutation_baseline_misses_are_pinned_and_ledgered",
        negative_id: "synthetic_unrecorded_mutation_baseline_shrink",
        routing: MetaLockRouting::SharedScan,
        witness_count: MUTANTS_BASELINE_NORMALIZED_COUNT,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "generative_lock_two_sided_floor_ratchets",
        negative_id: "synthetic_unrecorded_generative_floor_raise",
        routing: MetaLockRouting::SharedScan,
        witness_count: 3,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "physical_mutating_event_kinds_have_explicit_world_apply_arms",
        negative_id: "synthetic_missing_arm_catch_all",
        routing: MetaLockRouting::TicketOwnedDebt {
            ticket: "0022PHA3ABASTRI-008",
        },
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "embodied_projection_never_uses_unfiltered_checked_facts_as_actor_provenance",
        negative_id: "synthetic_unfiltered_checked_facts_iter",
        routing: MetaLockRouting::TicketOwnedDebt {
            ticket: "0022PHA3ABASTRI-009",
        },
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_006_scheduler_has_no_direct_routine_or_need_proposal_bypass",
        negative_id: "synthetic_direct_scheduler_proposal_bypass",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_006_duration_need_deltas_route_through_shared_emitter",
        negative_id: "synthetic_direct_duration_need_delta_construction",
        routing: MetaLockRouting::SharedScan,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_006_scheduler_does_not_fabricate_empty_epistemic_projection",
        negative_id: "synthetic_empty_epistemic_projection",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_018_actor_known_facts_require_source_event_witness",
        negative_id: "synthetic_actor_known_fact_without_witness",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_018_witness_kind_no_human_fact_stable_ids_have_explicit_arms",
        negative_id: "synthetic_missing_witness_kind_stable_id_arm",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_014_no_human_cognition_surface_does_not_read_raw_assignment_or_sleep_truth",
        negative_id: "synthetic_raw_assignment_or_sleep_truth_read",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_015_ord_hard_008_cognition_channel_stays_evented_and_sealed",
        negative_id: "synthetic_unsealed_cognition_channel",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_014_embodied_projection_workplaces_are_context_backed",
        negative_id: "synthetic_contextless_workplace_projection",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_014_phase3a_semantic_actions_do_not_use_literal_true_availability",
        negative_id: "synthetic_literal_true_action_availability",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_014_no_human_metrics_do_not_scan_display_text",
        negative_id: "synthetic_display_text_metric_scan",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_014_perception_visibility_uses_typed_place_visibility",
        negative_id: "synthetic_prose_visibility_branch",
        routing: MetaLockRouting::SharedScan,
        witness_count: 3,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_014_sleep_validation_requires_modeled_affordance",
        negative_id: "synthetic_sleep_validation_without_affordance",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_0021_actor_known_projection_policy_table_has_production_callers",
        negative_id: "synthetic_projection_policy_without_caller",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_015_ordinary_life_tuning_comes_from_authored_state",
        negative_id: "synthetic_ordinary_life_tuning_literal",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_006_scheduler_has_no_routine_family_to_primitive_dispatch",
        negative_id: "synthetic_routine_family_to_primitive_dispatch",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_014_scheduler_cannot_rewrite_transaction_proposals_after_cognition",
        negative_id: "synthetic_scheduler_rewrites_transaction_proposal",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_014_transaction_has_no_silent_method_fallback_scan",
        negative_id: "synthetic_silent_method_fallback_scan",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_015_hidden_truth_audit_fails_closed_in_transaction",
        negative_id: "synthetic_hidden_truth_audit_open_transaction",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_014_decision_hidden_truth_audit_uses_typed_input_refs",
        negative_id: "synthetic_hidden_truth_audit_string_tag",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id:
            "guard_003_work_eat_sleep_validators_do_not_read_need_values_from_proposal_parameters",
        negative_id: "synthetic_need_value_parameter_read",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_002_agent_state_keeps_typed_trace_and_diagnostic_records",
        negative_id: "synthetic_untyped_agent_trace_record",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_001_authoritative_state_fields_are_not_publicly_mutable",
        negative_id: "synthetic_public_authoritative_state_field",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_001_mutation_capability_is_private_to_event_application",
        negative_id: "synthetic_public_mutation_capability",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_001_no_production_seed_mutation_outside_state_definition",
        negative_id: "synthetic_seed_mutation_outside_state",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_001_no_direct_state_collection_insert_outside_event_application",
        negative_id: "synthetic_direct_state_collection_insert",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_001_actor_known_context_has_no_public_arbitrary_constructor",
        negative_id: "synthetic_public_actor_known_context_constructor",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_0021_actor_known_context_producers_are_projection_backed",
        negative_id: "synthetic_unbacked_actor_known_context_producer",
        routing: MetaLockRouting::SharedScan,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_0021_hidden_truth_gates_use_event_log_provenance",
        negative_id: "synthetic_hidden_truth_gate_without_event_log",
        routing: MetaLockRouting::SharedScan,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "hidden_food_closed_container_is_not_actor_known_food_source",
        negative_id: "synthetic_empty_hidden_food_adversarial_fixture",
        routing: MetaLockRouting::SharedScan,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "hidden_route_edge_absent_from_actor_context_blocks_route_plan",
        negative_id: "synthetic_empty_hidden_route_adversarial_fixture",
        routing: MetaLockRouting::SharedScan,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "planner_hidden_truth_fixture_witness_fails_on_empty_adversarial_context",
        negative_id: "synthetic_empty_planner_hidden_truth_fixture",
        routing: MetaLockRouting::SharedScan,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "actor_known_projection_policy_table_drives_record_behavior",
        negative_id: "synthetic_policy_table_behavior_drift",
        routing: MetaLockRouting::SharedScan,
        witness_count: 4,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "workplace_current_place_scope_drops_other_place_from_embodied_context",
        negative_id: "synthetic_workplace_embodied_scope_removed",
        routing: MetaLockRouting::SharedScan,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "supersede_newest_by_subject_requires_subject_extractor",
        negative_id: "synthetic_non_workplace_supersede_subject",
        routing: MetaLockRouting::SharedScan,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_0021_fabricated_visible_local_event_id_is_retired",
        negative_id: "synthetic_fabricated_visible_local_event_id",
        routing: MetaLockRouting::SharedScan,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_001_hidden_truth_audit_is_derived_from_provenance_not_tags",
        negative_id: "synthetic_hidden_truth_audit_tag_match",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_006_continue_routine_marker_alone_is_not_behavioral_progress",
        negative_id: "synthetic_continue_marker_as_progress",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_007_mutation_efficacy_notes_cover_high_risk_shortcuts",
        negative_id: "synthetic_missing_mutation_efficacy_note",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_008_action_registry_uses_typed_scopes_not_phase1_boolean",
        negative_id: "synthetic_action_registry_phase1_boolean",
        routing: MetaLockRouting::BehaviorAssertion,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_008_phase1_loader_does_not_register_later_phase_actions",
        negative_id: "synthetic_phase1_loader_registers_later_phase",
        routing: MetaLockRouting::SharedScan,
        witness_count: 1,
        witness_min: 1,
    },
    MetaLockRegistryEntry {
        lock_id: "guard_008_phase1_loader_source_guard_has_mutation_self_coverage",
        negative_id: "synthetic_phase1_loader_guard_mutation",
        routing: MetaLockRouting::SharedScan,
        witness_count: 1,
        witness_min: 1,
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

fn recognized_mutants_exclude_glob(pattern: &str) -> bool {
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

fn parse_exclude_globs(mutants_toml: &str) -> Vec<String> {
    let Some(after_marker) = mutants_toml.split("exclude_globs = [").nth(1) else {
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
    } else if required_path == "crates/tracewake-core/src/scheduler.rs" {
        filter_line.contains("crates/tracewake-core/src/scheduler\\.rs")
    } else if required_path == "crates/tracewake-core/src/projections.rs" {
        filter_line.contains("crates/tracewake-core/src/projections\\.rs")
    } else if required_path == "crates/tracewake-core/src/actions/pipeline.rs" {
        filter_line.contains("crates/tracewake-core/src/actions/pipeline\\.rs")
    } else {
        let stem = required_path
            .rsplit('/')
            .next()
            .and_then(|file_name| file_name.strip_suffix(".rs"))
            .unwrap_or(required_path);
        filter_line
            .split("actions/defs/(")
            .nth(1)
            .and_then(|tail| tail.split(")\\.rs").next())
            .is_some_and(|group| group.split('|').any(|entry| entry == stem))
    }
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

fn anti_regression_guard_test_names(source: &str) -> BTreeSet<String> {
    source
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim_start();
            trimmed
                .strip_prefix("fn guard_")
                .and_then(|tail| tail.split_once('('))
                .map(|(name, _)| format!("guard_{name}"))
        })
        .collect()
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
        if entry.witness_min == 0 {
            errors.push(format!(
                "meta-lock {} has a zero witness minimum",
                entry.lock_id
            ));
        }
        if entry.witness_count < entry.witness_min {
            errors.push(format!(
                "meta-lock {} witness count {} is below minimum {}",
                entry.lock_id, entry.witness_count, entry.witness_min
            ));
        }
    }

    if !lock_ids.contains("meta_lock_registry_census") {
        errors.push("meta-lock registry does not list its own census lock".to_string());
    }
    if !negative_ids.contains("synthetic_meta_lock_without_negative") {
        errors.push("meta-lock registry does not list its reflexive negative".to_string());
    }

    for guard_name in anti_regression_guard_test_names(anti_regression_source) {
        if !lock_ids.contains(guard_name.as_str()) {
            errors.push(format!(
                "anti-regression structural guard {guard_name} is missing from meta-lock registry"
            ));
        }
    }

    errors
}

fn mutation_perimeter_consistency_violations(mutants_toml: &str, ci_yml: &str) -> Vec<String> {
    let mut violations = Vec::new();

    for key in parse_mutants_toml_top_level_keys(mutants_toml) {
        if !matches!(key.as_str(), "additional_cargo_args" | "exclude_globs") {
            violations.push(format!("mutants.toml uses unsupported key {key}"));
        }
    }

    if mutants_toml.contains("crates/tracewake-core/src/actions/defs/**") {
        violations.push("mutants.toml excludes all action definitions".to_string());
    }

    let scheduled_block =
        yaml_step_run_block(ci_yml, "Run guarded-layer mutation baseline").unwrap_or_default();
    let scheduled_effective_lines = non_comment_lines(scheduled_block).join("\n");
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

    for scheduled_filter in MUTATION_PERIMETER_REQUIRED_FILTERS {
        if !scheduled_effective_lines.contains(scheduled_filter) {
            violations.push(format!(
                "scheduled mutation baseline omits required filter {scheduled_filter}"
            ));
        }
    }

    for excluded in parse_exclude_globs(mutants_toml) {
        if !recognized_mutants_exclude_glob(&excluded) {
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

    let exclude_globs = parse_exclude_globs(mutants_toml);
    for entry in WORKSPACE_SOURCE_CLASSIFICATIONS.iter().filter(|entry| {
        entry
            .path
            .starts_with("crates/tracewake-core/src/actions/defs/")
            && entry.path.ends_with(".rs")
    }) {
        let excluded = exclude_globs
            .iter()
            .any(|pattern| simple_glob_matches(pattern, entry.path));
        let in_mutation_perimeter = MUTATION_PERIMETER_CANARY_PATHS.contains(&entry.path);
        if in_mutation_perimeter && excluded {
            violations.push(format!(
                "mutants.toml excludes guarded action definition {}",
                entry.path
            ));
        }
        if !in_mutation_perimeter && !excluded {
            violations.push(format!(
                "mutants.toml does not exclude non-perimeter action definition {}",
                entry.path
            ));
        }
    }

    for required_path in MUTATION_PERIMETER_CANARY_PATHS {
        if !in_diff_filter_matches_path(in_diff_filter_line, required_path) {
            violations.push(format!(
                "in-diff mutation guarded-path filter omits {required_path}"
            ));
        }

        let Some(classification) = WORKSPACE_SOURCE_CLASSIFICATIONS
            .iter()
            .find(|entry| entry.path == *required_path)
        else {
            violations.push(format!(
                "{required_path} missing from source classification table"
            ));
            continue;
        };
        match classification.class {
            WorkspaceSourceClass::GuardedLayer => {}
            WorkspaceSourceClass::Exempt { rationale } => {
                if !rationale.contains("mutation") {
                    violations.push(format!(
                        "{required_path} classification rationale must name mutation coverage"
                    ));
                }
            }
        }
    }

    violations.extend(mutation_rationale_violations(
        WORKSPACE_SOURCE_CLASSIFICATIONS,
    ));

    for line in non_comment_lines(ci_yml)
        .into_iter()
        .filter(|line| line.contains("cargo mutants"))
    {
        if line.contains("||") || line.contains("&&") {
            violations.push("cargo-mutants invocation has shell suffix".to_string());
        }
    }
    for block in yaml_step_blocks_with_cargo_mutants(ci_yml) {
        let effective_lines = non_comment_lines(block);
        let mutants_invocations = effective_lines
            .iter()
            .filter(|line| line.contains("cargo mutants"))
            .count();
        let status_captures = effective_lines
            .iter()
            .filter(|line| line.contains("mutants_status=$?"))
            .count();
        if status_captures < mutants_invocations {
            violations.push(
                "cargo-mutants step does not capture status in the same step block".to_string(),
            );
        }
    }
    if !ci_yml.contains("\"$mutants_status\" -ne 0")
        || !ci_yml.contains("\"$mutants_status\" -ne 2")
    {
        violations.push(
            "in-diff cargo-mutants status handling does not separate tool failure from misses"
                .to_string(),
        );
    }
    if !ci_yml.contains("[ ! -d mutants.out ]") {
        violations.push("in-diff mutation job does not require output artifacts".to_string());
    }
    if !scheduled_block.contains("echo \"mutants_status=$mutants_status\" >> \"$GITHUB_OUTPUT\"") {
        violations
            .push("scheduled cargo-mutants status is not exposed to baseline check".to_string());
    }
    if !ci_yml.contains("if: always() && steps.scheduled_mutants.outcome != 'skipped'") {
        violations.push(
            "scheduled mutation baseline check does not run after accepted misses".to_string(),
        );
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

fn mutation_baseline_change_log_records(ledger: &str, count: usize, hash: u64) -> bool {
    let count_marker = format!("normalized-count={count}");
    let hash_marker = format!("fnv1a64={hash:016x}");
    ledger.lines().any(|line| {
        line.contains("baseline-delta:")
            && line.contains(&count_marker)
            && line.contains(&hash_marker)
    })
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

    let excluded_defs = MUTANTS_TOML.replace(
        "  \"crates/tracewake-core/src/actions/mod.rs\",",
        "  \"crates/tracewake-core/src/actions/defs/**\",\n  \"crates/tracewake-core/src/actions/mod.rs\",",
    );
    assert!(
        mutation_perimeter_consistency_violations(&excluded_defs, CI_YML)
            .iter()
            .any(|violation| violation.contains("excludes all action definitions")),
        "synthetic broad action-def exclusion must fail the perimeter guard"
    );

    let excluded_eat = MUTANTS_TOML.replace(
        "  \"crates/tracewake-core/src/actions/mod.rs\",",
        "  \"crates/tracewake-core/src/actions/defs/eat.rs\",\n  \"crates/tracewake-core/src/actions/mod.rs\",",
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
        "  \"crates/tracewake-core/src/actions/defs/wait.rs\",",
        "  \"**/wait.rs\",",
    );
    assert!(
        mutation_perimeter_consistency_violations(&unsupported_glob, CI_YML)
            .iter()
            .any(|violation| violation.contains("unsupported glob shape")),
        "synthetic unsupported exclude glob must fail closed"
    );

    let missing_wait_exclusion = MUTANTS_TOML.replace(
        "  \"crates/tracewake-core/src/actions/defs/wait.rs\",\n",
        "",
    );
    assert!(
        mutation_perimeter_consistency_violations(&missing_wait_exclusion, CI_YML)
            .iter()
            .any(|violation| violation.contains("wait.rs")),
        "synthetic non-perimeter action def missing from exclude_globs must fail parity"
    );

    let swallowed_failure = CI_YML.replace(
        "cargo mutants --in-diff \"$RUNNER_TEMP/guarded.diff\" --no-shuffle",
        "cargo mutants --in-diff \"$RUNNER_TEMP/guarded.diff\" --no-shuffle || echo ok",
    );
    assert!(
        mutation_perimeter_consistency_violations(MUTANTS_TOML, &swallowed_failure)
            .iter()
            .any(|violation| violation.contains("shell suffix")),
        "synthetic swallowed cargo-mutants failure must fail the perimeter guard"
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

    let missing_scheduled_capture = CI_YML.replace(
        "          mutants_status=$?\n          set -e\n          echo \"mutants_status=$mutants_status\" >> \"$GITHUB_OUTPUT\"\n",
        "",
    );
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

    let missing_eat_scheduled = CI_YML.replace(
        "            -f 'crates/tracewake-core/src/actions/defs/eat.rs' \\\n",
        "",
    );
    assert!(
        mutation_perimeter_consistency_violations(MUTANTS_TOML, &missing_eat_scheduled)
            .iter()
            .any(|violation| violation.contains("actions/defs/eat.rs")),
        "synthetic scheduled-filter removal must fail for eat.rs"
    );

    let decoy_scheduled_filter = CI_YML.replace(
        "            -f 'crates/tracewake-core/src/actions/defs/eat.rs' \\\n",
        "          # -f 'crates/tracewake-core/src/actions/defs/eat.rs' \\\n",
    );
    assert!(
        mutation_perimeter_consistency_violations(MUTANTS_TOML, &decoy_scheduled_filter)
            .iter()
            .any(|violation| violation.contains("actions/defs/eat.rs")),
        "synthetic commented scheduled-filter decoy must fail for eat.rs"
    );

    let missing_eat_in_diff = CI_YML.replace(
        "actions/defs/(eat|sleep|work)\\.rs",
        "actions/defs/(sleep|work)\\.rs",
    );
    assert!(
        mutation_perimeter_consistency_violations(MUTANTS_TOML, &missing_eat_in_diff)
            .iter()
            .any(|violation| violation.contains("actions/defs/eat.rs")),
        "synthetic in-diff regex removal must fail for eat.rs"
    );

    let decoy_in_diff = CI_YML.replace(
        "          if git diff --name-only \"$guarded_range\" | grep -E",
        "          echo \"git diff --name-only decoy\" | grep -E '^(crates/tracewake-core/src/actions/defs/(eat|sleep|work)\\.rs)'\n          if git diff --name-only \"$guarded_range\" | grep -E",
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
                .any(|error| error.contains("recorded baseline change log")),
        "synthetic unledgered baseline append must fail count, hash, ledger, and change-log checks"
    );

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

    let unrecorded_floor_raise_ledger = MUTANTS_BASELINE_LEDGER.replace(
        "baseline-delta: normalized-count=143 fnv1a64=bd1855a5ee82b428",
        "baseline-delta: normalized-count=144 fnv1a64=bd1855a5ee82b428",
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

    let deferred_ledger = MUTANTS_BASELINE_LEDGER.replacen(
        "warrants-test:0022PHA3ABASTRI-015:",
        "justified-baseline: this warrants a future focused assertion",
        1,
    );
    assert!(
        mutation_baseline_governance_errors(MUTANTS_BASELINE_MISSES, &deferred_ledger)
            .iter()
            .any(|error| error.contains("deferred test debt")),
        "synthetic justified-baseline deferral language must fail governance"
    );

    let bad_tag_ledger = MUTANTS_BASELINE_LEDGER.replacen(
        "warrants-test:0022PHA3ABASTRI-015:",
        "warrants-test:0022PHA3ABASTRI-999:",
        1,
    );
    assert!(
        mutation_baseline_governance_errors(MUTANTS_BASELINE_MISSES, &bad_tag_ledger)
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

    let mut zero_match_scan = META_LOCK_REGISTRY.to_vec();
    zero_match_scan[0].witness_count = 0;
    assert!(
        meta_lock_registry_errors(&zero_match_scan, ANTI_REGRESSION_GUARDS_RS)
            .iter()
            .any(|error| error.contains("below minimum")),
        "synthetic zero-match scan must fail the nonzero-witness rule"
    );

    let unregistered_guard_source = format!(
        "{ANTI_REGRESSION_GUARDS_RS}\n#[test]\nfn guard_synthetic_unregistered_lock() {{}}\n"
    );
    assert!(
        meta_lock_registry_errors(META_LOCK_REGISTRY, &unregistered_guard_source)
            .iter()
            .any(|error| error.contains("guard_synthetic_unregistered_lock")),
        "synthetic unregistered structural guard must fail the meta-lock census"
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
        scheduler.contains("build_sleep_completion_events(\n                    state,\n                    agent_state,\n                    log,"),
        "scheduler must pass current state and log into sleep completion continuity checks"
    );
    assert!(
        scheduler.contains("build_work_completion_events(\n                    state,\n                    agent_state,\n                    log,"),
        "scheduler must pass current state and log into work completion continuity checks"
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
        app.contains("proposal_from_current_view_semantic_action"),
        "TUI semantic-action submission must use the current-view source-context constructor"
    );
    assert_absent(app, "proposal_from_semantic_action_entry");

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
    // Smoke-only source scan: compile-fail fixtures and capability privacy are
    // the primary enforcement layer; this catches obvious new direct calls.
    let allowed_paths = [
        "crates/tracewake-core/src/events/apply.rs",
        "crates/tracewake-core/src/replay/rebuild.rs",
        "crates/tracewake-core/src/actions/pipeline.rs",
    ];
    let mut violations = Vec::new();
    for (path, source) in core_production_sources() {
        let contains_direct_apply =
            source.contains("apply_event(") || source.contains("apply_event_stream(");
        if contains_direct_apply && !allowed_paths.iter().any(|allowed| *allowed == path) {
            violations.push(path);
        }
    }

    assert!(
        violations.is_empty(),
        "direct apply_event/apply_event_stream call outside event/replay/pipeline production code:\n{}",
        violations.join("\n")
    );
    assert!(
        production(include_str!("../src/actions/pipeline.rs")).contains("let mut dry_run = context.state.clone();"),
        "pipeline dry-run validation must apply constructed events to cloned, non-authoritative state"
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
        "wait/sleep/work/window need deltas must be constructed by need_events.rs: {violations:?}"
    );

    let synthetic_sources = vec![(
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
    )];
    let synthetic_violations =
        direct_duration_need_delta_construction_violations(&synthetic_sources);
    assert!(
        !synthetic_violations.is_empty(),
        "synthetic direct NeedDeltaApplied construction must fail this guard"
    );
}

fn direct_duration_need_delta_construction_violations(sources: &[(String, String)]) -> Vec<String> {
    const GUARDED_PATHS: &[&str] = &[
        "crates/tracewake-core/src/actions/defs/wait.rs",
        "crates/tracewake-core/src/actions/defs/sleep.rs",
        "crates/tracewake-core/src/actions/defs/work.rs",
        "crates/tracewake-core/src/scheduler.rs",
    ];
    sources
        .iter()
        .filter(|(path, _)| GUARDED_PATHS.contains(&path.as_str()))
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
    let build_agent_proposal = body_after_marker(&scheduler, "fn build_agent_proposal");

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
        assert_absent(build_agent_proposal, forbidden);
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
    let build_agent_proposal = body_after_marker(&scheduler, "fn build_agent_proposal");
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
        assert_absent(build_agent_proposal, forbidden);
        assert_absent_from_sources(&scheduler_sources, forbidden);
        assert_absent_from_sources(&agent_sources, forbidden);
    }
    assert_absent(build_agent_proposal, "food_source_believed_accessible");
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
        synthetic_violations.len() >= 3,
        "synthetic prose/id visibility branch must fail this guard"
    );
}

fn perception_visibility_prose_branch_violations(source: &str) -> Vec<&'static str> {
    let stripped = source_without_comments(source);
    let visibility_body = body_after_marker(&stripped, "fn is_visible_exit_target");
    [
        "display_label",
        ".to_lowercase()",
        ".contains(\"hidden\")",
        "place.place_id.as_str()",
        "place_id.as_str().contains",
    ]
    .into_iter()
    .filter(|needle| visibility_body.contains(needle))
    .collect()
}

#[test]
fn guard_014_sleep_validation_requires_modeled_affordance() {
    let sleep = production(SLEEP_RS);
    let projection = production(PROJECTIONS_RS);
    let transaction = production(TRANSACTION_RS);
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
        transaction.contains("actor_known_sleep_affordance_id"),
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
    let build_agent_proposal = body_after_marker(&scheduler, "fn build_agent_proposal");
    let after_transaction_run =
        body_after_marker(build_agent_proposal, "ActorDecisionTransaction::run");

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
    assert_absent(&transaction, "candidate_fallbacks");
    assert_absent(&transaction, ".find_map(|candidate|");
    assert!(
        transaction.contains("pub struct SelectedGoalBundle"),
        "transaction must bind selected candidate, trace, method, local plan, and proposal ancestry"
    );
    assert!(
        transaction.contains("bundle.decision_trace_id.as_str().to_string()"),
        "proposal decision_trace_id must come from the selected goal bundle"
    );
    assert!(
        transaction.contains("bundle.candidate_goal_id.as_str().to_string()"),
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
    let mut keys = literal_payload_keys(body);
    for callee in payload_helper_calls(body) {
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

fn literal_payload_keys(body: &str) -> BTreeSet<String> {
    let mut keys = BTreeSet::new();
    for marker in [
        r#"required(payload, ""#,
        r#"required(&payload, ""#,
        r#"payload.get(""#,
        r#"payload, ""#,
    ] {
        let mut search_start = 0;
        while let Some(relative_index) = body[search_start..].find(marker) {
            let value_start = search_start + relative_index + marker.len();
            if let Some(value_end) = body[value_start..].find('"') {
                keys.insert(body[value_start..value_start + value_end].to_string());
            }
            search_start = value_start;
        }
    }
    keys
}

fn payload_helper_calls(body: &str) -> BTreeSet<String> {
    let mut calls = BTreeSet::new();
    let mut search_start = 0;
    while let Some(relative_index) = body[search_start..].find("(payload") {
        let open_index = search_start + relative_index;
        let prefix = body[..open_index].trim_end();
        let name_start = prefix
            .rfind(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_'))
            .map(|index| index + 1)
            .unwrap_or(0);
        let name = &prefix[name_start..];
        if !name.is_empty() {
            calls.insert(name.to_string());
        }
        search_start = open_index + "(payload".len();
    }
    calls
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
        ("support/generative.rs", SUPPORT_GENERATIVE_RS),
    ];
    let errors = generative_duration_terminal_fabricator_errors(&sources);
    assert!(
        errors.is_empty(),
        "generative duration-terminal fabricator ban failed: {errors:#?}"
    );

    let support_fabricator = [(
        "support/generative.rs",
        "fn helper() { EventEnvelope::new_v1(id, EventKind::WorkBlockFailed, 0, 0, tick, key, manifest); }",
    )];
    assert!(
        generative_duration_terminal_fabricator_errors(&support_fabricator)
            .iter()
            .any(|error| error.contains("support/generative.rs")),
        "support-file terminal fabricator synthetic must fail"
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
    for kind in EventKind::all()
        .iter()
        .copied()
        .filter(|kind| kind.metadata().physical_mutating)
    {
        let arm = format!("EventKind::{kind:?}");
        assert!(
            apply_body.contains(&arm),
            "physical-mutating event kind lacks explicit world apply arm: {kind:?}"
        );
    }
    assert_absent(apply_body, "_ => Ok(ApplyOutcome::NonWorldNoOp)");

    let synthetic_apply = r#"
        fn apply_event_with_capability(event: &EventEnvelope) -> Result<ApplyOutcome, ApplyError> {
            match event.event_type {
                EventKind::ActorMoved => Ok(ApplyOutcome::Applied),
                _ => Ok(ApplyOutcome::NonWorldNoOp),
            }
        }
    "#;
    assert!(
        !synthetic_apply.contains("EventKind::FoodConsumed")
            && synthetic_apply.contains("_ => Ok(ApplyOutcome::NonWorldNoOp)"),
        "synthetic missing-arm catch-all must fail the totality guard"
    );
}

#[test]
fn action_emitted_event_kinds_have_cause_disposition() {
    use tracewake_core::events::EventKind;

    let required = [
        EventKind::ActorMoved,
        EventKind::ActorWaited,
        EventKind::BeliefUpdated,
        EventKind::ContainerChecked,
        EventKind::ContainerClosed,
        EventKind::ContainerOpened,
        EventKind::DoorClosed,
        EventKind::DoorOpened,
        EventKind::ItemPlacedInContainer,
        EventKind::ItemPlacedInPlace,
        EventKind::ItemRemovedFromContainer,
        EventKind::ItemTakenFromPlace,
        EventKind::ObservationRecorded,
    ];

    for kind in required {
        assert!(
            kind.requires_cause(),
            "action-emitted kind lacks required-cause disposition: {kind:?}"
        );
    }

    for source in [
        MOVEMENT_RS,
        WAIT_RS,
        OPENCLOSE_RS,
        CHECKCONTAINER_RS,
        TAKEPLACE_RS,
        PERCEPTION_RS,
    ] {
        assert!(
            source.contains("EventEnvelope::new_caused_v1"),
            "action-emitted source must construct caused envelopes"
        );
    }
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
        if path == "crates/tracewake-core/src/state.rs" {
            continue;
        }
        assert!(
            !source.contains("seed_"),
            "{path} uses seed construction mutators in production"
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
