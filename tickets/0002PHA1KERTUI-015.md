# 0002PHA1KERTUI-015: No-human advance

**Status**: PENDING
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — adds the no-human advance command to `tracewake-core`'s scheduler; emits `NoHumanAdvanceStarted`/`NoHumanAdvanceCompleted`.
**Deps**: 0002PHA1KERTUI-004, 0002PHA1KERTUI-008

## Problem

The simulation must advance with no sacred player state and no controller bound (Spec 0002 §4.1 area 18, §18.5). No-human advance loads the fixture, runs the scheduler for a fixed tick count with no controller binding, produces deterministic events/reports, and replays — proving the scheduler/event/projection code does not require a controller and contains no player-only branch. Where an ordinary scheduled action occurs, it must use the same action pipeline (ticket 008).

## Assumption Reassessment (2026-06-06)

1. The deterministic scheduler/tick advance exists from ticket 004; the shared pipeline exists from ticket 008; the `NoHumanAdvanceStarted`/`NoHumanAdvanceCompleted` event kinds exist from ticket 005. This ticket adds the no-human advance command, modifying `scheduler.rs` (created by 004).
2. The no-human gate is `specs/0002_…_SPEC.md` §18.5 and `docs/2-execution/05_…REPLAY.md` no-human gate: minimum is a no-op/time-advance proof that there is no controller/player dependency and the event/replay machinery is real; any ordinary scheduled action must use the shared pipeline.
3. Shared boundary under audit: the no-human advance entry point shares the ticket-008 pipeline and ticket-004 scheduler — it must not introduce a parallel advance path or a player-conditioned branch.
4. Invariant motivating this ticket: INV-004 (the authoritative world ignores human existence — the simulation must run with no human controller) and INV-091 (no-human tests are mandatory).
5. Deterministic-replay surface: no-human advance emits `NoHumanAdvance*` markers (on `diagnostic`/process stream, not implying player absence as a world fact) and any scheduled-action world events through the shared pipeline; replay reproduces the run (§18.5). It introduces no controller dependency and no nondeterminism. Enforcement (a no-human scenario that replays) is ticket 022's `no_human_advance_001`.

## Architecture Check

1. Implementing no-human advance as "run the existing scheduler with no binding" (rather than a dedicated headless mode) guarantees the same code runs with and without a human — the strongest possible proof of INV-004. A separate headless loop could drift from the human path.
2. The `NoHumanAdvance*` markers are non-world process markers, so they do not assert "no player exists" as a world fact (§11.3). No backwards-compatibility shims: greenfield.

## Verification Layers

1. No-controller execution (INV-004/091) -> replay/golden check: the advance runs to completion with no controller bound and emits no player-referencing event.
2. Shared pipeline reuse (INV-007) -> codebase grep-proof: no-human advance proposes ordinary actions through the ticket-008 pipeline, not a parallel applier.
3. Deterministic markers -> unit test: `NoHumanAdvanceStarted`/`Completed` are on the process/diagnostic stream and do not mutate physical state.

## What to Change

### 1. No-human advance command

Add the no-human advance entry point in `crates/tracewake-core/src/scheduler.rs` (or a small `scheduler/no_human.rs`): load-fixture-independent advance for N ticks with no controller, emitting `NoHumanAdvanceStarted` then `NoHumanAdvanceCompleted`, routing any ordinary scheduled action through the shared pipeline.

## Files to Touch

- `crates/tracewake-core/src/scheduler.rs` (modify — add no-human advance command; file created by ticket 004)

## Out of Scope

- Routines / autonomous agent cognition (Phase 3) — Phase 1 no-human may be a no-op/time-advance proof (§18.5).
- The fixture that drives `no_human_advance_001` (ticket 019) and its scenario test (ticket 022).

## Acceptance Criteria

### Tests That Must Pass

1. No-human advance runs for a fixed tick count with no controller bound and commits no player-referencing event.
2. Time advances only through the deterministic tick/event machinery; replay reproduces the result.
3. Any ordinary scheduled action in the advance is validated through the shared pipeline.

### Invariants

1. The scheduler/event/projection path requires no controller binding.
2. No `PlayerCharacter`, player objective, or player-only branch exists in the advance.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/scheduler.rs` (unit tests) — no-controller advance, deterministic markers, pipeline reuse.

### Commands

1. `cargo test -p tracewake-core scheduler::no_human`
2. `cargo build --workspace`
3. Unit scope is correct; the `no_human_advance_001` fixture replay runs end-to-end in ticket 022.
