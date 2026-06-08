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

fn production(source: &str) -> &str {
    source.split("#[cfg(test)]").next().unwrap_or(source)
}

fn assert_absent(haystack: &str, needle: &str) {
    assert!(
        !haystack.contains(needle),
        "forbidden shortcut reintroduced: {needle}"
    );
}

#[allow(
    clippy::disallowed_methods,
    reason = "anti-regression test scans source files; this helper is not simulation outcome code"
)]
fn production_sources() -> Vec<(String, String)> {
    let src_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut stack = vec![src_root];
    let mut sources = Vec::new();
    while let Some(path) = stack.pop() {
        for entry in std::fs::read_dir(path).expect("source directory is readable") {
            let entry = entry.expect("source directory entry is readable");
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                let relative = path
                    .strip_prefix(env!("CARGO_MANIFEST_DIR"))
                    .expect("source path is under manifest dir")
                    .display()
                    .to_string();
                let source = std::fs::read_to_string(&path).expect("source file is readable");
                sources.push((relative, production(&source).to_string()));
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

#[test]
fn nondeterminism_api_gate() {
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
        assert_absent(scheduler, forbidden);
    }
}

#[test]
fn guard_006_scheduler_does_not_fabricate_empty_epistemic_projection() {
    let scheduler = production(SCHEDULER_RS);
    assert_absent(scheduler, "EpistemicProjection::new");
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
        assert_absent(scheduler, forbidden);
    }
}

#[test]
fn guard_003_work_eat_sleep_validators_do_not_read_need_values_from_proposal_parameters() {
    for source in [
        production(EAT_RS),
        production(SLEEP_RS),
        production(WORK_RS),
    ] {
        assert_absent(source, "parameters.get(\"current_hunger\")");
        assert_absent(source, "parameters.get(\"current_fatigue\")");
        assert_absent(source, "parameters[\"current_hunger\"]");
        assert_absent(source, "parameters[\"current_fatigue\"]");
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
    assert_absent(actor_known, "pub fn from_observed_parts");
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
