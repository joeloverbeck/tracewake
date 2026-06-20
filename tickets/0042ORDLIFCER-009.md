# 0042ORDLIFCER-009: ORD-LIFE-08 — ordinary action affordances, causal movement, durations, terminals, and no-teleport behavior

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — evidence-only; runs existing tests/fixtures and records witnesses. May add test-only instrumentation per spec §2 (evidence instrumentation, not production remediation).
**Deps**: 0042ORDLIFCER-001

## Problem

Spec §5 audit point ORD-LIFE-08 must be proven: eat, sleep, work, wait, movement, and routine-continuation actions must be proposed through the ordinary transaction and accepted only after current validation of target, location, affordance, access, reservation, and source-context parameters; remote work/sleep/eat cannot begin without causal movement ancestry or explicitly valid duration semantics; and every open duration has one authoritative start and exactly one legal terminal. This ticket records the proposal/validation reports, movement/duration ancestry, no-teleport negatives, and live/replay equality into the acceptance artifact.

## Assumption Reassessment (2026-06-20)

1. The audited surfaces exist at `98dc042`: `crates/tracewake-core/src/actions/defs/{eat,sleep,work,wait,continue_routine,movement}.rs` (`require_sleep_affordance`, `build_sleep_start_event`, `SleepAffordanceState`), `actions/pipeline.rs`, `actions/proposal.rs` (`Proposal`, `ProposalSource`, `ProposalSourceContext`), `need_accounting.rs`, `state.rs`, `location.rs`, `events/envelope.rs` (confirmed in the 0042 reassessment census).
2. Spec §5 ORD-LIFE-08 (doctrine, positive/adversarial fixtures, exact commands, evidence mechanics) and §6.1 pass conditions 1/4/5/9 govern this ticket; §4.4 "event before derived state" binds it — a correct final location without movement ancestry is a failure.
3. Cross-artifact shared boundary under audit: the ORD-LIFE-08 section of `reports/0042_…_acceptance.md` (created `(new)` by `0042ORDLIFCER-001`); this ticket appends only that section.
4. Motivating invariants (spec §5 ORD-LIFE-08): `INV-044` (causal movement / no teleport), `INV-043`, `INV-001`/`INV-009` (eventful actions with cause), `INV-015`/`INV-045` (one start, one legal terminal; single charge), `INV-048`, `INV-104`/`INV-106`. Restate before trusting the narrative: every ordinary action passes current validation; remote durations require movement ancestry; one start ↔ exactly one legal terminal.
5. Evidence-consumer surface (audit-reads, does not modify): the ordinary-action validation pipeline and its append-before-apply replay. This ticket runs affordance/duration/displacement fixtures and records proposal/validation reports; it adds no direct-write action path, introduces no nondeterminism, and any instrumentation stays observer-only.

## Architecture Check

1. Requiring full proposal/validation reports with context-parity tuples and movement/duration ancestry — and treating a correct final location without movement ancestry as a failure — is the decisive no-teleport proof, stronger than checking the end state.
2. No backwards-compatibility aliasing/shims introduced; evidence-only ticket. No shim may preserve a teleport/affordance bypass.

## Verification Layers

1. `INV-044` causal movement / no teleport -> replay/golden-fixture check (`routine_no_teleport_001`, `work_completion_fails_when_actor_displaced_001`, hidden-route fixtures: remote work/sleep/eat requires movement ancestry or legal duration semantics).
2. `INV-043` affordance/access validation -> codebase grep-proof + manual review (`sleep_rejects_current_place_without_sleep_affordance_001`, `no_human_current_place_without_sleep_affordance_does_not_sleep_001`: legal sleep only at an actor-known sleep affordance; proposal source context ID/hash/frontier + actor/action/target params unchanged from sealed transaction to validator).
3. `INV-015`/`INV-045` one start/one terminal + single charge -> replay/golden-fixture check (duplicate completion/interruption/failure, terminal-without-start, start-while-exclusive-duration-open, completion-after-displacement each identified at the exact action-validation/accounting failure; clean replay reproduces per-tick ledger and final state).

## What to Change

### 1. Record positive affordance/movement/duration witnesses

Run the §5 positives: legal sleep at an actor-known sleep affordance, legal eating of an actor-known accessible item, movement to a known workplace followed by work, and modeled wait with a typed reason — all through proposal, validation, event append, application, and replay. For each duration action record start, covered ticks, interruption/completion/failure terminal, need accounting, reservation release, and final actor/place state. Run `severe_safety_with_known_exit_produces_move_001`, `sleep_eat_work_001`, `ordinary_workday_001`, and causal container/item movement fixtures as positive ancestry witnesses. Record proposal source context ID/hash/frontier and actor/action/target parameters unchanged from sealed transaction to validator.

### 2. Record adversarial no-teleport and validation negatives

Record the §5 adversarial cases: `routine_no_teleport_001`, `sleep_rejects_current_place_without_sleep_affordance_001`, `no_human_current_place_without_sleep_affordance_does_not_sleep_001`, `work_completion_fails_when_actor_displaced_001`, hidden-route fixtures; forged/stale target / actor place / item location / sleep affordance / workplace assignment / context hash-frontier / reservation / duration start / terminal cause (validation rejects with no accepted event); duplicate completion/interruption/failure, terminal-without-start, start-while-exclusive-open, completion-after-displacement (each identifies the exact action-validation/accounting failure); and proof that a routine label / need band / scheduler window / validator-discovered true target cannot directly create an action or movement event.

## Files to Touch

- `reports/0042_ord_life_cert_needs_routines_intentions_no_human_life_planner_traces_and_stuck_diagnostics_certification_acceptance.md` (modify — ORD-LIFE-08 section; file created by 0042ORDLIFCER-001)

## Out of Scope

- Production remediation of any affordance/movement/terminal defect (record `fail` + responsible layer; route to a later separately-numbered remediation spec).
- Single-charge accounting certification (`-002`), scheduler no-direct-dispatch (`-012`), and replay-derivation lock (`-013`) — exercised here only at the ordinary-action seam.
- The aggregate verdict and per-condition/fixture-family tables (`-016`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test acceptance_gates` and `cargo test --locked -p tracewake-core --test anti_regression_guards` pass; `routine_no_teleport_001` and `work_completion_fails_when_actor_displaced_001` show remote/displaced actions requiring movement ancestry or failing, with no teleport normalized away.
2. `cargo test --locked -p tracewake-core --test golden_scenarios` and `cargo test --locked -p tracewake-content --test golden_fixtures_run` pass; `sleep_rejects_current_place_without_sleep_affordance_001` rejects sleep without an actor-known affordance and the proposal context-parity tuple is unchanged transaction→validator.
3. `cargo test --locked -p tracewake-core --test event_schema_replay_gates` passes; duplicate/orphan terminal and start-while-open cases hit the exact action-validation/accounting failure, and clean replay reproduces the per-tick ledger and final actor/place state.

### Invariants

1. No teleport: a correct final location without causal movement ancestry is a failure; remote durations require movement ancestry or explicitly valid duration semantics.
2. One start, one legal terminal: every open duration has exactly one authoritative start and one legal terminal; duplicate/orphan terminals are rejected at the responsible layer.

## Test Plan

### New/Modified Tests

1. `None — evidence-only ticket; the gate's existing suites below are the verification surface. Any test-only instrumentation added under spec §2 stays observer-only.`

### Commands

1. `cargo test --locked -p tracewake-core --test acceptance_gates`
2. `cargo test --locked -p tracewake-core --test anti_regression_guards`
3. `cargo test --locked -p tracewake-core --test golden_scenarios`
4. `cargo test --locked -p tracewake-core --test event_schema_replay_gates`
5. `cargo test --locked -p tracewake-content --test golden_fixtures_run`
