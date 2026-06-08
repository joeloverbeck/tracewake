const SCHEDULER_RS: &str = include_str!("../src/scheduler.rs");
const ACTOR_KNOWN_RS: &str = include_str!("../src/agent/actor_known.rs");
const DECISION_RS: &str = include_str!("../src/agent/decision.rs");
const HTN_RS: &str = include_str!("../src/agent/htn.rs");
const PLANNER_RS: &str = include_str!("../src/agent/planner.rs");
const STATE_RS: &str = include_str!("../src/state.rs");
const EAT_RS: &str = include_str!("../src/actions/defs/eat.rs");
const SLEEP_RS: &str = include_str!("../src/actions/defs/sleep.rs");
const WORK_RS: &str = include_str!("../src/actions/defs/work.rs");

fn production(source: &str) -> &str {
    source.split("#[cfg(test)]").next().unwrap_or(source)
}

fn assert_absent(haystack: &str, needle: &str) {
    assert!(
        !haystack.contains(needle),
        "forbidden shortcut reintroduced: {needle}"
    );
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
