# 0042ORDLIFCER-005: ORD-LIFE-04 — defeasible routine templates, HTN method families, interruptors, failure modes, and fallback

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only; runs existing tests/fixtures and records witnesses. May add test-only instrumentation per spec §2 (evidence instrumentation, not production remediation).
**Deps**: 0042ORDLIFCER-001

## Problem

Spec §5 audit point ORD-LIFE-04 must be proven: routines are defeasible HTN-like causal machinery, not scripts or guaranteed arcs; every live routine template must state holder-known applicability, steps or method families, required known affordances/records, interruptors, explicit failure modes, fallback/wait/stuck semantics, and trace/diagnostic expectations, and no routine may dispatch a primitive action merely because its label or schedule says so. This ticket records the routine-template census, per-template positive/negative reachability, method/interruptor/fallback traces, and live/replay equality into the acceptance artifact.

## Assumption Reassessment (2026-06-20)

1. The audited surfaces exist at `98dc042`: `crates/tracewake-core/src/agent/routine.rs` (`RoutineFamily`, `RoutineStep`, `RoutineCondition`, `RoutineTemplate`, `RoutineExecution`, `RoutineStepStatus`, `RoutineDiagnosticKind`), `agent/methods.rs` / `agent/htn.rs` (`phase3a_routine_templates`, `family_for_goal`, `select_phase3a_method`, `resolve_template_conditions`, `MethodSelectionFailure`), `agent/planner.rs`, `agent/decision.rs`, `agent/intention.rs`, `actions/defs/continue_routine.rs`, `actions/defs/wait.rs` (confirmed in the 0042 reassessment census).
2. Spec §5 ORD-LIFE-04 (doctrine, positive/adversarial fixtures, exact commands, evidence mechanics) and §6.1 pass conditions 1/3/9 govern this ticket; §4.4 "routine machinery, not script" binds it — a happy-path sequence alone is not a routine proof.
3. Cross-artifact shared boundary under audit: the ORD-LIFE-04 section of `reports/0042_…_acceptance.md` (created `(new)` by `0042ORDLIFCER-001`); this ticket appends only that section. The routine-template family set returned by `phase3a_routine_templates` is a quantifier to enumerate (spec §5 "enumerate every family"): each family's applicability, steps/methods, affordance/record requirements, interruptors, non-empty failure modes, fallback rules, and diagnostic expectations must be individually reachable through committed tests.
4. Motivating invariants (spec §5 ORD-LIFE-04): `INV-106` (routine validity / defeasible failure modes), `INV-035`/`INV-036`/`INV-037` (durable intentions, routines, planning), `INV-040`/`INV-041`, `INV-104`/`INV-105`. Restate before trusting the narrative: every template exposes applicability + alternatives/methods + interruptors + non-empty failure modes + fallback/wait/stuck + trace expectations.
5. Evidence-consumer surface (audit-reads, does not modify): the routine/method selection path and its replay. This ticket runs template-reachability fixtures and records traces; it adds no script-like dispatch path, introduces no nondeterminism, and any instrumentation stays observer-only.

## Architecture Check

1. Enumerating every `phase3a_routine_templates` family and proving per-template positive AND negative reachability (failure mode reached, fallback edge taken, interruptor triggered) is the only check that distinguishes machinery from a recorded happy-path action list — spec §5 states source shape without live failure reachability is non-certifying.
2. No backwards-compatibility aliasing/shims introduced; evidence-only ticket.

## Verification Layers

1. `INV-106` routine validity -> codebase grep-proof + manual review (template census: every family has applicability, steps/methods, affordance/record requirements, interruptors, non-empty failure modes, fallback, diagnostics — each reachable through a committed test).
2. `INV-037`/`INV-040` no-script dispatch -> replay/golden-fixture check (`routine_no_teleport_001` and blocked-affordance fixtures: a remote work/sleep step does not skip movement ancestry or affordance validation).
3. `INV-018` routine replay -> replay/golden-fixture check (selected method, interruptor, failure-mode/fallback edge, resulting routine/intention status reproduced on clean replay; deterministic selection under template/hash-map insertion-order change).

## What to Change

### 1. Record the per-template census and positive reachability

Enumerate every `phase3a_routine_templates` family into an explicit member list. For each (morning wake, eat meal, find food, go to work, work block, return home, sleep night, leave unsafe place, continue intention, wait — as enumerated against code), record applicability conditions, steps/methods, affordance/record requirements, interruptors, non-empty failure modes, fallback rules, and diagnostic expectations, exercised through at least one accepted method and one rejected alternative where applicable. Run `routine_blocked_diagnostic_001`, `method_fallback_requires_new_trace_or_stuck_001`, `work_block_failed_then_sleep_succeeds_001`, `severe_safety_with_known_exit_produces_move_001`. Prove routine progress survives across ticks/proposals while method failure yields coherent fallback/wait/stuck/failure/new-traced-method, never silent restart.

### 2. Record adversarial routine negatives

Record the §5 adversarial cases: a routine with no failure modes / empty non-diagnostic steps / hidden-truth precondition / no severe-safety interruptor / repeat-failed-method fallback (validation/test oracles fail it); `routine_no_teleport_001` + blocked-affordance fixtures (no movement/affordance skip); every method precondition failing, planner-budget exhaustion, validation rejection (each identifies method/routine identity, blocker, responsible layer, resulting status); and deterministic selected method under template-order/hash-map insertion change with equal semantic inputs.

## Files to Touch

- `reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md` (modify — ORD-LIFE-04 section; file created by 0042ORDLIFCER-001)

## Out of Scope

- Production remediation of any routine defect (record `fail` + responsible layer; route to a later separately-numbered remediation spec).
- Temporal-premise certification (`-006`), bounded local planning / budget (`-007`), and stuck-diagnostic taxonomy (`-011`) — exercised here only where routine fallback produces them.
- The aggregate verdict and per-condition/fixture-family tables (`-016`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test generative_lock` and `cargo test --locked -p tracewake-core --test golden_scenarios` pass; the recorded template census enumerates every `phase3a_routine_templates` family with non-empty failure modes and a reachable fallback/interruptor edge per family.
2. `cargo test --locked -p tracewake-content --test golden_fixtures_run` passes; `routine_no_teleport_001` and blocked-affordance fixtures prove no remote step skips movement ancestry or affordance validation.
3. `cargo test --locked -p tracewake-core --test no_human_capstone` and `cargo test --locked -p tracewake-core --test event_schema_replay_gates` pass; clean replay reproduces selected method, interruptor, failure-mode/fallback edge, and resulting routine/intention status.

### Invariants

1. Defeasible, non-script routines: every live template exposes applicability + methods + interruptors + non-empty failure modes + fallback/wait/stuck + trace expectations; a happy-path sequence alone does not certify.
2. No teleport / no label-dispatch: a routine label, need band, or schedule window cannot skip movement ancestry or affordance validation, and template/hash-map order does not change the deterministic selected method.

## Test Plan

### New/Modified Tests

1. Added `phase3a_template_census_has_defeasible_machinery_for_each_family` in `crates/tracewake-core/src/agent/methods.rs` as test-only instrumentation to enumerate every current `phase3a_routine_templates()` family and prove each has proposal steps, interruptor checkpoints, explicit failure modes, fallback rules, and family trace/debug labels. This is not production remediation and does not alter routine behavior.

### Commands

1. `cargo test --locked -p tracewake-core --test generative_lock`
2. `cargo test --locked -p tracewake-core --test golden_scenarios`
3. `cargo test --locked -p tracewake-core --test no_human_capstone`
4. `cargo test --locked -p tracewake-content --test golden_fixtures_run`
5. `cargo test --locked -p tracewake-core --test event_schema_replay_gates`

## Outcome

Completed: 2026-06-20

Populated the ORD-LIFE-04 section of `reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md` with command transcript fingerprints, an exhaustive `phase3a_routine_templates()` family census, routine behavior witnesses, replay/negative evidence, and a local `pass` result.

Added the test-only `phase3a_template_census_has_defeasible_machinery_for_each_family` unit test in `crates/tracewake-core/src/agent/methods.rs` to satisfy the ticket's enumerated-family criterion. The current census is: `MorningWake`, `EatMeal`, `GoToWork`, `WorkBlock`, `ReturnHome`, `SleepNight`, `FindFood`, `LeaveUnsafePlace`, `ContinueCurrentIntention`, and `Wait`. The report also records live routine evidence from `routine_blocked_fixture_records_access_failure_without_silent_loop`, `routine_no_teleport_fixture_fails_remote_work_without_movement_ancestry`, `planner_trace_fixture_exposes_selection_rejections_and_hidden_truth_audit`, `severe_safety_with_known_exit_produces_move_and_replays`, `work_block_failed_then_sleep_succeeds_fixture_closes_reservation`, no-human routine ancestry, event-schema replay gates, generated locks, and content validation negatives.

Verification:

- `cargo test --locked -p tracewake-core phase3a_template_census_has_defeasible_machinery_for_each_family` — passed.
- `cargo fmt --all --check` — passed after the test-only edit.
- `cargo test --locked -p tracewake-core --test generative_lock` — passed after the test-only edit.
- `cargo test --locked -p tracewake-core --test golden_scenarios` — passed after the test-only edit.
- `cargo test --locked -p tracewake-core --test no_human_capstone` — passed after the test-only edit.
- `cargo test --locked -p tracewake-content --test golden_fixtures_run` — passed after the test-only edit.
- `cargo test --locked -p tracewake-core --test event_schema_replay_gates` — passed after the test-only edit.

No production behavior changed. No remediation was needed or performed.
