# 0057EMBROUCON-003: No silent wedge — typed block / modeled wait / embodied stuck eligibility

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` typed-outcome surfacing for a blocked embodied follow-on (`runtime/session.rs`) and cross-tick stuck eligibility for repeated no-progress continuations (`scheduler.rs`)
**Deps**: 0057EMBROUCON-002

## Problem

Spec 0057 §4.3 and §1.1(3). Once the embodied follow-on is wired (0057EMBROUCON-002), a `Continue routine` whose resolved follow-on cannot be committed (precondition unmet, route blocked, workplace closed by belief, need-blocked, intention terminal) must surface the existing typed rejection / why-not, or a modeled wait / typed stuck record — **never** a silent Accepted no-op. And repeated no-progress continuations within one routine window must be eligible for the same cross-tick stuck diagnostics the no-human path uses (`past_expected_progress_window`, `repeated_idle_wait`), so an embodied player is not silently wedged the way 38 consecutive marker-only submissions wedged the actor in the §3 reproduction.

## Assumption Reassessment (2026-06-30)

1. Stuck-diagnostic surface verified: `crates/tracewake-core/src/scheduler.rs:3535` `append_routine_stuck_diagnostics(...)`; `StuckDiagnosticKind::PastExpectedProgressWindow` / `RepeatedIdleWait` (`scheduler.rs:4526-27`) with stable ids `"past_expected_progress_window"` / `"repeated_idle_wait"` (`scheduler.rs:4534-35`). The no-human path already emits these during world-step advance; the embodied submission path (`runtime/session.rs::run_semantic_proposal`, wired in 0057EMBROUCON-002) must feed the same routine-progress tracking so repeated no-progress continuations become eligible on the same terms.
2. Spec assumption: `archive/specs/0057_…_SPEC.md` §4.3 governs; the reassessment corrected the diagnostic token to `repeated_idle_wait` (the scheduler stable id), which this ticket uses verbatim. The typed blocker vocabulary (precondition / route / workplace / need / terminal) is the resolver's typed result from 0057EMBROUCON-001 — this ticket surfaces it, it does not invent new blocker kinds.
3. **Cross-artifact boundary under audit**: the seam between the embodied submission (`runtime/session.rs`) and the scheduler's cross-tick stuck detection (`scheduler.rs`) — the routine-progress / expected-progress-window state both must read and update consistently, so an embodied window and a no-human window are diagnosed "on the same terms."
4. INV-070 (why-not explanations are mandatory — a blocked action explains missing preconditions in actor-known terms), INV-015 (failure is eventful when consequential — a blocked/abandoned continuation produces events), INV-035 (routines are defeasible — failure/interruption/alternatives must be available, not a silent wedge), INV-106 (validation failure feeds replanning without leakage — the actor learns only modeled rejection reasons, not validator-only truth).
5. **Fail-closed / replay / actor-knowledge enforcement surface**: the typed-rejection/why-not path and the stuck-diagnostic emitter. Confirm: (a) a blocked embodied follow-on returns a typed why-not or a modeled-wait/typed-stuck record — never a silent Accepted no-op (fail-closed); (b) the why-not exposes only actor-known rejection reasons, no validator-only hidden truth (INV-106/024); (c) stuck diagnostics remain typed/structurally-inspectable records emitted deterministically and replay-reconstructably (INV-018/105), identical in form to the no-human path.
6. **Schema-shape (runtime-authority surface touched)**: `runtime/session.rs` is modified — **no schema shape change**. The typed-outcome surfacing reuses the existing rejection/why-not report and the existing `StuckDiagnosticKind` records; no public enum/struct reseal, no new serialized field. The embodied stuck path emits the *same* existing diagnostic kinds. Additive behavior only.

## Architecture Check

1. Reusing the existing `StuckDiagnosticKind` emitter and the existing typed-rejection path — rather than a new embodied-only diagnostic — makes "an embodied window is diagnosed on the same terms as a no-human window" structural: there is one diagnostic vocabulary and one progress-window model. A separate embodied stuck path would let the two surfaces drift (the same divergence hazard 0057EMBROUCON-001 closes for resolution).
2. No backwards-compatibility aliasing or shims: the embodied path feeds the existing progress-tracking state; no parallel stuck tracker, no wrapper.

## Verification Layers

1. INV-070 / INV-015 (why-not; eventful failure) -> behavioral test: a belief-closed-workplace / unknown-route continuation returns a typed why-not (actor-known terms) or a modeled wait, and emits an event — never a silent Accepted no-op.
2. INV-106 / INV-024 (no leakage in failure) -> actor-knowledge negative test: the returned why-not contains no validator-only hidden-truth detail (only modeled rejection reasons available to the actor).
3. INV-018 / INV-105 (deterministic, typed diagnostics) -> replay/golden: repeated no-progress embodied continuations within one window emit `past_expected_progress_window` / `repeated_idle_wait` as typed records, identical in form and replay-reconstructable to the no-human path.

## What to Change

### 1. Surface the typed blocker / modeled wait

In `run_semantic_proposal` (`runtime/session.rs`), when 0057EMBROUCON-001's resolver returns a typed blocker or a modeled-wait directive for the follow-on, return the existing typed rejection / why-not (actor-known) or commit the modeled wait / typed stuck record as the receipt outcome — never a silent Accepted no-op for the marker alone.

### 2. Make repeated embodied continuations stuck-eligible

Feed the embodied submission's routine-progress state into the same expected-progress-window / repeated-idle tracking the scheduler's `append_routine_stuck_diagnostics` reads, so repeated no-progress continuations within one routine window emit `past_expected_progress_window` / `repeated_idle_wait` on the same terms as the no-human path.

## Files to Touch

- `crates/tracewake-core/src/runtime/session.rs` (modify — surface typed blocker / modeled wait from the follow-on; feed routine-progress state)
- `crates/tracewake-core/src/scheduler.rs` (modify — embodied continuations eligible for the existing cross-tick stuck diagnostics)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — blocked-continuation + repeated-no-progress stuck guard; shares this file with 0057EMBROUCON-004, append only)

## Out of Scope

- Committing the successful follow-on (0057EMBROUCON-002, depended on).
- New blocker kinds or a new stuck-diagnostic vocabulary — this ticket reuses the resolver's typed result and the existing `StuckDiagnosticKind`.
- The post-work `stuck_actors` accounting observation (spec §1.2 — recorded for a separate determination).
- Parity / golden fixtures (0057EMBROUCON-005).

## Acceptance Criteria

### Tests That Must Pass

1. Blocked-continuation test: a routine step whose follow-on is precondition-blocked (belief-closed workplace or unknown route) returns a typed why-not / modeled wait, not a silent Accepted no-op.
2. Embodied-stuck-eligibility test: repeated no-progress `Continue routine` submissions within one routine window emit `past_expected_progress_window` then `repeated_idle_wait`, matching the no-human path's diagnostics.
3. No-leak test: the returned why-not exposes only actor-known rejection reasons; no validator-only hidden truth appears (INV-106/024).
4. `cargo test --locked -p tracewake-core` — typed-outcome + stuck-eligibility green; existing no-human stuck diagnostics unchanged.

### Invariants

1. An embodied continuation that cannot progress always surfaces a typed why-not, a modeled wait, or a typed stuck record — never a silent Accepted no-op.
2. Embodied and no-human routine windows are diagnosed by one shared stuck-diagnostic vocabulary and progress-window model.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — blocked-continuation typed why-not + repeated-no-progress `repeated_idle_wait`/`past_expected_progress_window` eligibility, with a hidden-truth no-leak assertion on the why-not.
2. `crates/tracewake-core/src/scheduler.rs` (in-module tests) — embodied-window eligibility matches the existing no-human-window diagnostics.

### Commands

1. `cargo test --locked -p tracewake-core scheduler` — stuck-diagnostic eligibility (embodied + no-human parity of form).
2. `cargo test --locked -p tracewake-core` — typed-outcome surfacing, no-leak, full core suite.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` — four-gate suite.

## Outcome

Completed: 2026-06-30

Implemented typed non-silent handling for blocked embodied routine continuations:

- `runtime/session.rs` now turns non-proposed actor-decision outcomes into a rejected `continue_routine` receipt with `ReasonCode::RoutineStepBlocked`, a typed `StuckDiagnosticRecorded` event, and the original `ContinueRoutineProposed` marker in the receipt event list.
- Recursive `continue_routine` follow-ons are classified as typed stuck diagnostics instead of committing another marker-only continuation.
- `scheduler.rs` exposes a narrow embodied stuck-scan helper that reuses the existing scheduler routine diagnostic vocabulary and emits `routine_expected_next_progress_stuck_detection` / `routine_repeated_idle_wait_stuck_detection` records from the same `RoutineExecution` progress-window state used by no-human windows.
- The TUI submit path permits `continue_routine` receipts to return typed diagnostic targets while preserving the same-action target assertion for ordinary submissions.
- Added an embodied-flow regression for blocked follow-ons and a scheduler in-module regression proving the embodied stuck scan emits the no-human diagnostic source tokens and de-duplicates through applied agent diagnostics.
- Added a meta-lock covered anti-regression guard for the runtime path so future changes cannot fall back to marker-only accepted no-ops for non-proposed embodied continuation outcomes.

Verification run:

- `cargo fmt --all --check`
- `cargo test --locked -p tracewake-core scheduler`
- `cargo test --locked -p tracewake-core`
- `cargo test --locked -p tracewake-tui`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`
