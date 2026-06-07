# 0005PHA3ANEEROU-007: Need effects and extended wait action

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — wires passive/active need deltas into time advancement and extends the existing `wait` action so it interacts with needs, reasons, and threshold-crossing reevaluation.
**Deps**: 0005PHA3ANEEROU-005

## Problem

Phase 3A needs must change causally with time and action — hunger/fatigue rise with awake time and work, fall only through eventful food/sleep — and the existing trivial `wait` must become a need-aware time control that applies passive deltas, carries a reason, and triggers threshold-crossing reevaluation without masking idle loops (Spec 0005 §8.1 required effects, §9.10; `INV-039`, `INV-045`). This is the first behavior wired onto the event spine (ticket 005) and the foundation every later action and the runner build on.

## Assumption Reassessment (2026-06-07)

1. The existing `wait` action is defined in `crates/tracewake-core/src/actions/defs/wait.rs` and flows through the shared pipeline (`actions/pipeline.rs`) and registry (`actions/registry.rs`); time advancement is in `crates/tracewake-core/src/scheduler.rs` (`advance_one_tick`) and `time.rs`. This ticket extends `wait` and the need-delta application rather than adding a parallel path.
2. The need model (ticket 001) and need-delta/threshold event kinds + apply arms (ticket 005) provide the mutation surface; this ticket emits those events from time advancement and from `wait`. Spec §8.1 fixes the effect directions: hunger/fatigue rise with awake time and work; hunger falls only via food consumption/service; fatigue falls via scheduled sleep/rest over time; safety rises in actor-known unsafe/blocked situations and falls on reaching safe/known/allowed places.
3. Shared boundary under audit: passive need-delta application is the deterministic windowed-delta path replay (ticket 006) reconstructs (Spec §15.2 batching). The delta schedule (per-tick/per-window rates) is fixed here and must be a pure function of elapsed ticks and awake/work state — no wall-clock, no RNG.
4. Invariants motivating this ticket: `INV-039` — "Needs are pressures, not puppet strings" (a high need increases pressure to act, never forces telepathic knowledge or bypasses access) — and `INV-045` — "Ordinary survival is causal" (no fake meter refills; hunger falls only through food/service). The extended `wait` must apply rises only and never refill; reductions come from sleep/eat tickets (008, 009).
5. Deterministic-replay surface: passive deltas and threshold crossings feed replay and the agent-state checksum (ticket 006). This change adds no nondeterminism — deltas are a fixed function of elapsed ticks and state, emitted as reconstructible windowed-delta events; threshold crossings emit explicit events. Enforcement is ticket 006 (cited).

## Architecture Check

1. Driving need rises from time advancement and `wait` (emitting reconstructible windowed-delta + threshold events) keeps need change inside the event/replay story rather than a hidden tick mutation (`INV-011`), and reusing the existing `wait` action keeps autonomous and human time control on one pipeline (Spec §3.4). A reason-bearing wait with bounded duration and threshold-triggered reevaluation structurally prevents the "idle loop disguised as autonomy" failure (Spec §25.4) — an unconditional wait would mask it.
2. No backwards-compatibility shims: `wait` is extended in place; passive-delta emission reuses ticket 005's event kinds, no parallel meter path.

## Verification Layers

1. Causal needs / no fake refill (`INV-045`) -> unit test + codebase grep-proof: `wait`/time advancement emit need-*rise* deltas only; no reduction path exists here (reductions are events from sleep/eat tickets).
2. Pressures not puppets (`INV-039`) -> unit test: a need rise produces a `NeedPressure`/threshold event that triggers candidate-goal reevaluation (consumed by ticket 012) but does not itself mutate location/knowledge or force an action.
3. Determinism (`INV-018`, substrate-only) -> unit test: passive deltas are a pure function of elapsed ticks + awake/work state; a replay of a wait sequence rebuilds identical need values. Full replay enforcement is ticket 006 (cited).

## What to Change

### 1. Passive need-delta application

Add a deterministic need-delta schedule (a pure function of elapsed ticks and awake/work state) and apply it during time advancement (`crates/tracewake-core/src/scheduler.rs` / `time.rs`) by emitting ticket 005's windowed need-delta and threshold-crossing events. No reduction is applied here.

### 2. Extended wait action

Extend `crates/tracewake-core/src/actions/defs/wait.rs` so a wait: carries an optional reason (mandatory when autonomous — enforced when ticket 012 submits it), applies/reconstructs passive need deltas over its duration, can trigger threshold crossings and flag candidate-goal reevaluation, stops for actor-perceived salient interruptions, does not skip scheduled completions, and surfaces its reason for debug. Pipeline integration stays in `actions/pipeline.rs`.

### 3. Reservation/exclusivity hook (minimal)

Ensure `wait` does not run concurrently with a body-exclusive action (the full reservation check lands in ticket 011); this ticket only marks `wait` as non-exclusive/interruptible.

## Files to Touch

- `crates/tracewake-core/src/actions/defs/wait.rs` (modify — need-aware, reason-bearing, interruptible wait)
- `crates/tracewake-core/src/scheduler.rs` (modify — emit passive need-delta + threshold events during advancement)
- `crates/tracewake-core/src/time.rs` (modify — deterministic elapsed-tick delta helper, if placed here)
- `crates/tracewake-core/src/actions/pipeline.rs` (modify — wait outcome carries reason/threshold reevaluation flag)

## Out of Scope

- Need *reductions* via sleep/eat (tickets 008, 009) and work need impact (ticket 010).
- Body-exclusive reservation enforcement (ticket 011).
- Candidate-goal generation that consumes the reevaluation flag (ticket 012).
- The no-human runner that drives autonomous waits with mandatory reasons (ticket 017).

## Acceptance Criteria

### Tests That Must Pass

1. Advancing N ticks raises hunger and fatigue by the deterministic schedule amount and emits reconstructible delta events; replaying the sequence yields identical need values.
2. A wait crossing a need threshold emits a threshold event and sets the candidate-goal reevaluation flag; the wait carries its reason for debug.
3. No path in this ticket reduces a need value (grep-proof + test); a wait does not skip a scheduled completion at the same tick.

### Invariants

1. Needs rise only by deterministic time/work deltas here; no fake refill (`INV-045`).
2. A need rise creates pressure/reevaluation, never a forced action or knowledge grant (`INV-039`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/actions/defs/wait.rs` (unit tests) — reason, passive delta, threshold reevaluation, no-skip.
2. `crates/tracewake-core/src/scheduler.rs` (unit tests) — deterministic passive-delta emission over advancement.

### Commands

1. `cargo test -p tracewake-core actions::defs::wait`
2. `cargo test -p tracewake-core scheduler`
3. Core-crate scope is correct; cross-pipeline integration (autonomous mandatory-reason wait) is exercised by tickets 012/017 and the capstone.
