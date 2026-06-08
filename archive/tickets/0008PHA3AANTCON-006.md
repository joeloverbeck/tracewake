# 0008PHA3AANTCON-006: Atomic flip — scheduler delegates to the transaction; remove all bypasses; chronological completions

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` scheduler: no-human run delegates to the actor-decision transaction; all direct-dispatch bypasses deleted; due completions processed in event-time order; `continue_routine` marker barred from progress
**Deps**: 0008PHA3AANTCON-003, 0008PHA3AANTCON-004

## Problem

This is the single atomic diff that removes the contamination. At the target commit the no-human path (Findings 1/3/4/7/8; D-0008-01/03/04/07/08) still: produces proposals by routine family (`scheduler.rs:670-677`: `SleepNight→sleep_proposal`, `EatMeal|FindFood→eat_proposal`, `GoToWork|WorkBlock→work_or_move_proposal`) and by need threshold (`:701-706`); constructs an empty `EpistemicProjection::new(...)` (`:534`) as actor-known proof; builds `VisibleLocalPlanningState` from raw `PhysicalState`; hard-codes `actor_known_only` (`:1027`); appends completion events after window processing rather than in chronological order; and can treat a `continue_routine` marker as progress.

Per the spec's anti-contamination thesis (§14.1-14.3) and the approved capstone-atomic structure, these are removed **together in one reviewable diff** so there is never an intermediate state that claims hardening while a bypass survives behind a wrapper.

## Assumption Reassessment (2026-06-08)

1. Bypasses confirmed in `crates/tracewake-core/src/scheduler.rs`: `build_agent_proposal` (`:525`) calls `build_routine_or_need_proposal` (`:654`) first (`:569`) returning early; routine-family arms (`:670-677`); need-threshold eat/sleep (`:701-706`); empty `EpistemicProjection::new` (`:534`, and test helper `:2202`); `actor_known_only` hard-coded (`:1027`); `run_no_human_day` (`:289`) with `NoHumanDayConfig`/`NoHumanDayReport` (`:256/:262`); completions appended around `NoHumanDayCompleted` (`:502`). `continue_routine` records `behavioral_progress=false` (`actions/defs/continue_routine.rs:84`).
2. Spec §9.1 lists exactly the forbidden constructs to remove and the required delegation; §8.1 stage 1 mandates "apply due completions before this decision observes state"; §9.4 + §10.5 mandate `ContinueRoutineProposed` count nothing as progress without a follow-on ordinary action or typed blockage; §14.1-14.3 forbid partial convenience layers, wrapper-hidden dispatch, and shape-matching test additions.
3. Cross-artifact boundary under audit: the **scheduler ↔ transaction** seam — after this ticket the scheduler may select actor/time/window and trigger the transaction (-004) only; it owns no ordinary-life decision policy. Depends on `PipelineReadContext.agent_state` (-003) so the transaction's proposals validate against authoritative state.
4. INV-008 (UI/assistance is not authority — here the scheduler must not be a special-purpose ordinary-life brain), INV-035 (schedules are not teleports/puppet strings), INV-091 (no-human tests mandatory), INV-004 (the authoritative world ignores human existence): the no-human runner must drive ordinary life through the same cognition transaction a possessed actor would, not a privileged path.
5. Enforcement surface: deterministic replay + fail-closed validation + actor-knowledge firewall — all three at once. Confirm: due completions are applied as eventful state transitions in chronological order so live and replay agree at each transaction boundary (not only at final report); the empty-projection actor-known proof is removed (the transaction receives a real epistemic/knowledge interface or records a typed limitation); no-human metrics are computed from transaction outcomes, not event-label counts; `continue_routine` alone never increments ordinary progress. No nondeterminism introduced — completion ordering is by event time, deterministic.
6. Restructures `NoHumanDayReport` metrics from label counts to transaction-outcome aggregates (additive/limited-breaking to the report struct). Consumers: `run_no_human_day` callers in `scheduler.rs` tests (~`:2272`), `crates/tracewake-core/tests/acceptance_gates.rs`, `crates/tracewake-core/tests/golden_scenarios.rs` — updated here or exercised by the capstone (-011).
7. Removes `build_routine_or_need_proposal`, `eat_proposal`/`sleep_proposal`/`work_or_move_proposal`, the empty-`EpistemicProjection` no-human path, and the hard-coded `actor_known_only`. Blast radius grep at implementation: `grep -rn "build_routine_or_need_proposal\|work_or_move_proposal\|actor_known_only" crates/` must return zero outside deleted code and the -007 anti-regression guards that assert their absence; `grep -rn "EpistemicProjection::new" crates/tracewake-core/src/scheduler.rs` must return zero in the cognition path. Removal of `VisibleLocalPlanningState`/`ActorKnownPlanningState` public construction (started in -001) completes here (no surviving adapter).

## Architecture Check

1. Making the scheduler a pure trigger (actor/time/window) and the transaction the sole cognition owner is the only structure that satisfies §14.3 — there is no wrapper left to hide dispatch behind. Atomic removal (vs. incremental) is required because any intermediate merge that keeps a bypass passes tests while the architecture is wrong (§15).
2. No backwards-compatibility aliasing: deleted functions are not re-exported or wrapped; the empty-projection path and `actor_known_only` boolean are removed outright. Any transitional adapter from -001 is deleted in this ticket.

## Verification Layers

1. INV-035 no-teleport/no-puppet → codebase grep-proof: scheduler contains no `RoutineFamily`→proposal arm and no need-threshold→proposal path (`grep -n "RoutineFamily\|sleep_proposal\|eat_proposal\|work_or_move_proposal" scheduler.rs` shows none in cognition code).
2. INV-008 scheduler-not-authority → manual review: `run_no_human_day` calls the transaction per actor/window and maps `SleepNight`/`EatMeal`/`WorkBlock` to nothing — the transaction owns the mapping.
3. INV-018 replay parity at boundaries → replay/golden-fixture check: due completions applied in event-time order; live and replay state equal after each transaction boundary, not only at final report.
4. Continue-not-progress → unit test (§10.5 gates 1–3): a run of only `ContinueRoutineProposed` fails the ordinary-life gate; a marker plus a follow-on ordinary action with matching ancestry counts only the ordinary action.

## What to Change

### 1. Delegate no-human cognition to the transaction

`run_no_human_day` selects actor/time/window and triggers the transaction (-004) per window; commits the transaction's validated proposal and typed records through the shared pipeline/event log. Pass a real epistemic/knowledge interface (or record a typed limitation) — no empty `EpistemicProjection::new`.

### 2. Delete every bypass

Remove `build_routine_or_need_proposal`, `eat_proposal`/`sleep_proposal`/`work_or_move_proposal`, routine-family arms, need-threshold paths, raw-`PhysicalState` `VisibleLocalPlanningState` construction in scheduler, and the hard-coded `actor_known_only`.

### 3. Chronological completions

Process due sleep/work completions as eventful transitions in chronological order relative to decision transactions (§8.1 stage 1), so a later decision in the same run observes completed effects.

### 4. Outcome-based metrics + continue-routine non-progress

Compute `NoHumanDayReport` metrics from transaction outcomes, not event-label counts. `ContinueRoutineProposed` (`continue_routine.rs`) is non-progress in all metrics; a continuation must pair with a follow-on ordinary action or a typed blocked/replan/stuck record.

## Files to Touch

- `crates/tracewake-core/src/scheduler.rs` (modify — delegate; delete bypasses; chronological completions; outcome metrics)
- `crates/tracewake-core/src/actions/defs/continue_routine.rs` (modify — ensure marker is structurally non-progress; ancestry hook for follow-on)

## Out of Scope

- Anti-regression/mutation guards asserting the bypasses stay gone (0008PHA3AANTCON-007).
- Hidden-truth adversarial fixtures (0008PHA3AANTCON-008).
- Debug/TUI projection of the now-typed records (0008PHA3AANTCON-010).
- The end-to-end capstone ancestry gate and ledger flip (0008PHA3AANTCON-011).

## Acceptance Criteria

### Tests That Must Pass

1. `run_no_human_day` produces ordinary actions only through the transaction; no proposal originates from routine-family or need-threshold dispatch.
2. Chronological-completion replay test: live and replay state equal after each transaction boundary (§10.6 / INV-018).
3. Continue-routine gates: a run of only markers fails the ordinary-life gate; marker+follow-on counts only the ordinary action (§10.5 gates 1–3).
4. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` green.

### Invariants

1. The scheduler owns no ordinary-life decision policy; the transaction is the only cognition path.
2. No empty-`EpistemicProjection` actor-known proof; no hard-coded `actor_known_only`.
3. `continue_routine` alone is never counted as behavioral progress.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/scheduler.rs` tests — delegation, deletion proofs, chronological completion ordering, continue-routine non-progress.
2. `crates/tracewake-core/tests/acceptance_gates.rs` — update no-human assertions to transaction-outcome metrics.

### Commands

1. `cargo test -p tracewake-core scheduler`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-08

What changed:

1. The no-human scheduler now delegates ordinary proposal construction to `ActorDecisionTransaction`; the removed bypass helpers no longer construct eat/sleep/work proposals directly from routine family or need thresholds.
2. The scheduler passes only the active routine family to the transaction. The family-to-goal mapping lives in the transaction, and fallback tries later generated candidates before idle so an inapplicable urgent need does not suppress an applicable routine.
3. The no-human actor-known path no longer fabricates an empty `EpistemicProjection::new(...)`; it records a typed no-human projection limitation in actor-known proof sources.
4. Transaction records now feed `DecisionTraceRecorded` events through the canonical `DecisionTraceRecord`, including derived hidden-truth audit fields instead of a hard-coded `actor_known_only` payload.
5. Sleep/work completions are drained in deterministic due order before later decisions observe state, and duration completions now complete matching in-progress routine steps through agent events.
6. `ContinueRoutineProposed` and `ActionRejected` are not counted as no-human behavioral progress.
7. TUI no-human assertions were updated to the new deterministic metrics produced by the transaction path and chronological duration completions.

Deviations:

1. `RoutineFamily` still appears in scheduler completion ancestry and test fixture setup, but no scheduler code maps routine families directly to ordinary proposals.
2. The no-human visible-local boundary remains the scheduler's observation input to the actor-known builder; the empty epistemic projection was removed and replaced by an explicit typed limitation.

Verification:

1. `cargo test -p tracewake-core scheduler` passed.
2. `cargo test -p tracewake-core agent::` passed.
3. `cargo test -p tracewake-content --test golden_fixtures_run no_human_day_real_run_replays_metrics_and_trace_projection` passed.
4. Grep proof passed for removed bypasses in `crates/tracewake-core/src/scheduler.rs`: `build_routine_or_need_proposal`, `work_or_move_proposal`, `eat_proposal`, `sleep_proposal`, `current_fatigue`, `current_hunger`, empty `EpistemicProjection::new`, and hard-coded `PayloadField::new("hidden_truth_audit", "actor_known_only")`.
5. `cargo fmt --all --check` passed.
6. `cargo clippy --workspace --all-targets -- -D warnings` passed.
7. `cargo build --workspace --all-targets --locked` passed.
8. `cargo test --workspace` passed.
