mod support;

use support::{AgentSeed, PhysicalSeed};

const SCHEDULER_RS: &str = include_str!("../src/scheduler.rs");
const ACTOR_KNOWN_RS: &str = include_str!("../src/agent/actor_known.rs");
const NO_HUMAN_SURFACE_RS: &str = include_str!("../src/agent/no_human_surface.rs");
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
const EAT_RS: &str = include_str!("../src/actions/defs/eat.rs");
const SLEEP_RS: &str = include_str!("../src/actions/defs/sleep.rs");
const TIME_RS: &str = include_str!("../src/time.rs");
const WORK_RS: &str = include_str!("../src/actions/defs/work.rs");
const ACTIONS_REGISTRY_RS: &str = include_str!("../src/actions/registry.rs");
const ACTIONS_REPORT_RS: &str = include_str!("../src/actions/report.rs");
const PROJECTIONS_RS: &str = include_str!("../src/projections.rs");
const GENERATIVE_LOCK_RS: &str = include_str!("generative_lock.rs");
const CONTENT_LOAD_RS: &str = include_str!("../../tracewake-content/src/load.rs");
const TUI_APP_RS: &str = include_str!("../../tracewake-tui/src/app.rs");
const MUTANTS_TOML: &str = include_str!("../../../.cargo/mutants.toml");
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

fn normalized_source(source: &str) -> String {
    source.split_whitespace().collect::<Vec<_>>().join(" ")
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
    "core action validation/registry code is covered by targeted action, pipeline, and duration-definition mutation guards";
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
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/actions/defs/eat.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_ACTION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/actions/defs/inspect.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_ACTION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/actions/defs/mod.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_ACTION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/actions/defs/movement.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_ACTION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/actions/defs/openclose.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_ACTION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/actions/defs/sleep.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_ACTION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/actions/defs/takeplace.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_ACTION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/actions/defs/wait.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_ACTION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/actions/defs/work.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_ACTION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/actions/mod.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_ACTION_RATIONALE } },
    WorkspaceSourceClassification { path: "crates/tracewake-core/src/actions/pipeline.rs", class: WorkspaceSourceClass::Exempt { rationale: CORE_ACTION_RATIONALE } },
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

const MUTATION_PERIMETER_DURATION_DEFS: &[&str] = &[
    "crates/tracewake-core/src/actions/defs/sleep.rs",
    "crates/tracewake-core/src/actions/defs/work.rs",
];

fn mutation_perimeter_consistency_violations(mutants_toml: &str, ci_yml: &str) -> Vec<String> {
    let mut violations = Vec::new();

    if mutants_toml.contains("crates/tracewake-core/src/actions/defs/**") {
        violations.push("mutants.toml excludes all action definitions".to_string());
    }

    let in_diff_filter_line = ci_yml
        .lines()
        .find(|line| line.contains("grep -E") && line.contains("actions/defs/"))
        .unwrap_or_default();
    let in_diff_defs_group = in_diff_filter_line
        .split("actions/defs/(")
        .nth(1)
        .and_then(|tail| tail.split(")\\.rs").next())
        .unwrap_or_default();

    for required_path in MUTATION_PERIMETER_DURATION_DEFS {
        let scheduled_filter = format!("-f '{required_path}'");
        if !ci_yml.contains(&scheduled_filter) {
            violations.push(format!(
                "scheduled mutation baseline omits required filter {required_path}"
            ));
        }

        let stem = required_path
            .rsplit('/')
            .next()
            .and_then(|file_name| file_name.strip_suffix(".rs"))
            .unwrap_or(required_path);
        if !in_diff_defs_group.split('|').any(|entry| entry == stem) {
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

    for line in ci_yml
        .lines()
        .filter(|line| line.contains("cargo mutants --in-diff"))
    {
        if line.contains("|| true") {
            violations.push("in-diff cargo-mutants invocation swallows failures".to_string());
        }
    }
    if !ci_yml.contains("mutants_status=$?") {
        violations.push("in-diff cargo-mutants status is not captured".to_string());
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
    if !ci_yml.contains("github.event_name == 'pull_request' || github.event_name == 'push'") {
        violations.push("mutation in-diff job does not run on guarded direct pushes".to_string());
    }
    if !ci_yml.contains("HEAD^..HEAD") {
        violations.push("push mutation diff does not compare against HEAD^".to_string());
    }

    violations
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

    let swallowed_failure = CI_YML.replace(
        "cargo mutants --in-diff \"$RUNNER_TEMP/guarded.diff\" --no-shuffle",
        "cargo mutants --in-diff \"$RUNNER_TEMP/guarded.diff\" --no-shuffle || true",
    );
    assert!(
        mutation_perimeter_consistency_violations(MUTANTS_TOML, &swallowed_failure)
            .iter()
            .any(|violation| violation.contains("swallows failures")),
        "synthetic swallowed cargo-mutants failure must fail the perimeter guard"
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
    let required_call = r#"require_payload_version(&payload, "payload_schema_version", "1")"#;
    let materialized_maps = [
        "need_threshold_crossings",
        "ordinary_life_episodes",
        "candidate_goal_evaluations",
        "continue_routine_arbitrations",
    ];
    for map_name in materialized_maps {
        let insert_token = format!("state.{map_name}.insert(");
        let insert_index = EVENTS_APPLY_RS
            .find(&insert_token)
            .unwrap_or_else(|| panic!("{map_name} insert is present in apply.rs"));
        let before_insert = &EVENTS_APPLY_RS[..insert_index];
        let arm_start = before_insert
            .rfind("EventKind::")
            .unwrap_or_else(|| panic!("{map_name} insert has an EventKind arm"));
        let arm = &EVENTS_APPLY_RS[arm_start..insert_index];
        assert!(
            arm.contains(required_call),
            "{map_name} materialization arm must require payload_schema_version"
        );
    }
}

#[test]
fn generative_lock_cannot_fabricate_duration_terminals() {
    for forbidden in [
        "build_sleep_completion_events",
        "build_work_completion_events",
        "append_generated_duration_terminals",
        "generated_duration_completion",
    ] {
        assert_absent(GENERATIVE_LOCK_RS, forbidden);
    }
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
    for forbidden in [
        "pub actors:",
        "pub places:",
        "pub doors:",
        "pub containers:",
        "pub items:",
        "pub food_supplies:",
        "pub workplaces:",
        "pub needs_by_actor:",
        "pub intentions:",
        "pub active_intention_by_actor:",
        "pub routine_executions:",
        "pub decision_traces:",
        "pub stuck_diagnostics:",
    ] {
        assert_absent(STATE_RS, forbidden);
    }

    for required in [
        "pub(crate) actors:",
        "pub(crate) places:",
        "pub(crate) doors:",
        "pub(crate) containers:",
        "pub(crate) items:",
        "pub(crate) food_supplies:",
        "pub(crate) workplaces:",
        "pub(crate) needs_by_actor:",
        "pub(crate) intentions:",
        "pub(crate) active_intention_by_actor:",
        "pub(crate) routine_executions:",
        "pub(crate) decision_traces:",
        "pub(crate) stuck_diagnostics:",
    ] {
        assert!(
            STATE_RS.contains(required),
            "authoritative state field visibility changed: {required}"
        );
    }
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
    // Smoke-only source scan: direct mutation is primarily blocked by private
    // fields, private mutation capabilities, and compile-fail fixtures.
    let forbidden_writes = [
        ".actors.insert",
        ".places.insert",
        ".doors.insert",
        ".containers.insert",
        ".items.insert",
        ".food_supplies.insert",
        ".workplaces.insert",
        ".needs_by_actor.insert",
        ".intentions.insert",
        ".active_intention_by_actor.insert",
        ".routine_executions.insert",
        ".decision_traces.insert",
        ".stuck_diagnostics.insert",
    ];
    for (path, source) in core_production_sources() {
        if path == "crates/tracewake-core/src/events/apply.rs" {
            continue;
        }
        for forbidden in forbidden_writes {
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
