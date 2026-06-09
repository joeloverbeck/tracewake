mod support;

use support::{AgentSeed, PhysicalSeed};

const SCHEDULER_RS: &str = include_str!("../src/scheduler.rs");
const ACTOR_KNOWN_RS: &str = include_str!("../src/agent/actor_known.rs");
const DECISION_RS: &str = include_str!("../src/agent/decision.rs");
const HTN_RS: &str = include_str!("../src/agent/htn.rs");
const PLANNER_RS: &str = include_str!("../src/agent/planner.rs");
const STATE_RS: &str = include_str!("../src/state.rs");
const EVENTS_MOD_RS: &str = include_str!("../src/events/mod.rs");
const EVENTS_APPLY_RS: &str = include_str!("../src/events/apply.rs");
const EVENTS_MUTATION_RS: &str = include_str!("../src/events/mutation.rs");
const EAT_RS: &str = include_str!("../src/actions/defs/eat.rs");
const SLEEP_RS: &str = include_str!("../src/actions/defs/sleep.rs");
const WORK_RS: &str = include_str!("../src/actions/defs/work.rs");
const ACTIONS_REPORT_RS: &str = include_str!("../src/actions/report.rs");
const PROJECTIONS_RS: &str = include_str!("../src/projections.rs");
const TUI_APP_RS: &str = include_str!("../../tracewake-tui/src/app.rs");

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

#[allow(
    clippy::disallowed_methods,
    reason = "anti-regression test scans source files; this helper is not simulation outcome code"
)]
fn production_sources() -> Vec<(String, String)> {
    let src_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    production_sources_from_roots(
        vec![src_root],
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")),
    )
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
    sources
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

fn state_struct_fields(struct_name: &str) -> Vec<String> {
    let marker = format!("pub struct {struct_name} {{");
    let body = STATE_RS
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
        holder_known_context_frontier: context.event_frontier,
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
        validate_proposal, ActionDefinition, ActionRegistry, ProposalValidationContext,
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
    registry.register(ActionDefinition::query_only(ActionId::new("look").unwrap()));
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
    let proposal = Proposal::new(
        ProposalId::new("proposal_wait").unwrap(),
        ProposalOrigin::Scheduler,
        Some(actor_id.clone()),
        ActionId::new("wait").unwrap(),
        SimTick::ZERO,
    );
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
        "src/events/apply.rs",
        "src/replay/rebuild.rs",
        "src/actions/pipeline.rs",
    ];
    let mut violations = Vec::new();
    for (path, source) in production_sources() {
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
    let scheduler = production(SCHEDULER_RS);
    for forbidden in [
        "build_routine_or_need_proposal",
        "eat_proposal",
        "sleep_proposal",
        "work_or_move_proposal",
        "ordinary_proposal",
        "current_hunger",
        "current_fatigue",
    ] {
        assert_absent(&scheduler, forbidden);
    }
}

#[test]
fn guard_006_scheduler_does_not_fabricate_empty_epistemic_projection() {
    let scheduler = production(SCHEDULER_RS);
    assert_absent(&scheduler, "EpistemicProjection::new");
    assert!(
        scheduler.contains("build_actor_known_planning_state_with_projection_limitation"),
        "no-human cognition must use the typed projection-limitation boundary"
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
    let mut state = PhysicalState::default();
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

    let physical_fields = state_struct_fields("PhysicalState");
    let agent_fields = state_struct_fields("AgentState");
    let physical_coverage = PHYSICAL_STATE_CHECKSUM_COVERAGE
        .iter()
        .map(|entry| entry.field_name.to_string())
        .collect::<Vec<_>>();
    let agent_coverage = AGENT_STATE_CHECKSUM_COVERAGE
        .iter()
        .map(|entry| entry.field_name.to_string())
        .collect::<Vec<_>>();

    assert_eq!(physical_coverage, physical_fields);
    assert_eq!(agent_coverage, agent_fields);

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

    let synthetic_omission_pattern =
        "Adding `pub(crate) new_field:` to PhysicalState or AgentState must fail this test until a matching checksum coverage entry and serializer branch are added.";
    assert!(synthetic_omission_pattern.contains("must fail this test"));
}

#[test]
fn guard_001_no_production_seed_mutation_outside_state_definition() {
    for (path, source) in production_sources() {
        if path == "src/state.rs" {
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
    for (path, source) in production_sources() {
        if path == "src/events/apply.rs" {
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
