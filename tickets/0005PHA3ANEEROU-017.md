# 0005PHA3ANEEROU-017: No-human day runner, decision ordering, day windows, and stuck detection

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — adds a real no-human ordinary-day runner on the existing no-human scheduler skeleton, with deterministic actor decision ordering, day windows, and aggressive stuck-actor detection.
**Deps**: 0005PHA3ANEEROU-011, 0005PHA3ANEEROU-014

## Problem

Phase 3A needs a real no-human ordinary-day runner: starting with no bound controller, advancing windows/ticks deterministically, evaluating needs/routine windows, generating candidate goals, adopting/continuing intentions, submitting ordinary proposals through the shared pipeline, scheduling duration completions, detecting salient interruptions, emitting day start/summary markers, and recording stuck diagnostics — without pre-scheduling hard-coded actor locations (Spec 0005 §10.1, §10.2, §10.3, §8.8; `INV-004`, `INV-018`). This orchestrates tickets 007–014 into a replayable boring day and is the hard gate of the phase.

## Assumption Reassessment (2026-06-07)

1. The existing no-human skeleton is `crates/tracewake-core/src/scheduler.rs` `mod no_human` with `advance_no_human` emitting `NoHumanAdvanceStarted`/`NoHumanAdvanceCompleted`, sorting scheduled proposals through the pipeline, and advancing ticks (confirmed `scheduler.rs:116-206`). Spec §4.1 calls this "an excellent skeleton, but not yet an autonomous ordinary-life day." This ticket builds the day runner on top of it, not a parallel scheduler.
2. The decision engine (012), routine selector (013), planner (014), ordinary actions + continue/reservation (007–011), and event/replay (005/006) are the machinery the runner invokes. Spec §10.1 fixes the deterministic actor decision ordering keys (tick → schedule phase → source kind → actor ID → routine/intention sequence → proposal sequence). Spec §10.2 fixes the day-window model (abstract `SimTick`; `no_human_day_001` default windows pre_dawn/morning/work_window/evening/night). Spec §10.3 fixes the runner behavior list. Spec §8.8 fixes when a stuck diagnostic must be emitted.
3. Shared boundary under audit: the runner must produce a day via action events, not a pre-scheduled list of actor locations (Spec §10.3 closing line, §21 forbidden shortcut). It may schedule decision opportunities and duration completions; concrete physical changes come from action events. Stuck detection wires into the runner's decision/advance loop and into `continue_routine` (ticket 011).
4. Invariants motivating this ticket: `INV-004` — "The authoritative world ignores human existence" (the simulation runs coherently with no human controller; no sacred player entity) — and `INV-018` — "Deterministic replay is foundational". No-human events must reference no player/controller identity (Spec §3.2, §21); decision ordering must be stable; no randomness is needed (Spec §10.1 — any randomness would have to be seeded/logged/replay-safe).
5. No-human / deterministic-replay / no-leak surface: this is the no-human gate enforcement. The runner binds no controller, emits no player-conditioned facts, orders decisions by stable keys (no RNG), and records stuck diagnostics for any unexplained idle/active end state. It adds no leakage (autonomous actors read only actor-known facts via tickets 012/014) and no nondeterminism. Replay determinism is verified by ticket 006/025; this ticket guarantees the run is controller-free and deterministically ordered.

## Architecture Check

1. Building the day runner on the existing `no_human` skeleton (scheduling decision opportunities + duration completions, with all physical change via action events) keeps Phase 3A inside the one deterministic scheduler and the one action pipeline, so the day is replayable and contains no teleport/pre-scheduled-location shortcut (`INV-004`, `INV-018`, Spec §10.3). Stable decision-ordering keys give determinism without RNG.
2. No backwards-compatibility shims: the runner extends the existing no-human process; no parallel scheduler, no hard-coded location schedule.

## Verification Layers

1. No-human world (`INV-004`) -> replay/golden check + grep-proof: the runner advances a full day with no bound controller and emits no player/controller-identity facts; a grep-proof over emitted events confirms no `player`/controller world fact.
2. Determinism (`INV-018`) -> replay/golden check: multiple actors deciding at the same tick are ordered by the stable §10.1 keys; the run replays byte-identically (verified with ticket 006's agent-state projection).
3. Aggressive stuck detection (`INV-041`; Spec §8.8) -> unit test: an actor with no progress past its expected window, a repeated reasonless idle, a failed step with no fallback, or an end-of-run unexplained idle/active state emits a typed `StuckDiagnostic`.

## What to Change

### 1. No-human day runner

Extend `crates/tracewake-core/src/scheduler.rs` (`mod no_human`) with an ordinary-day runner: emit a no-human day-start marker, advance windows/ticks deterministically, and per decision opportunity invoke generation (012) → method selection (013) → planning (014) → proposal submission through the shared pipeline, schedule/handle duration completions (sleep/work, tickets 008/010), detect salient interruptions, and emit a no-human day-summary marker. Physical change comes only from action events.

### 2. Deterministic decision ordering + day windows

Implement the §10.1 stable ordering keys for multi-actor decisions and the §10.2 day-window model (abstract ticks; default windows for the canonical fixture). No randomness.

### 3. Stuck detection

Implement §8.8 detection at decision/advance points and at end-of-run: emit a typed `StuckDiagnostic` (ticket 004) for any of the §8.8 conditions, including an end-of-run actor left in an unexplained active/idle state.

## Files to Touch

- `crates/tracewake-core/src/scheduler.rs` (modify — no-human day runner, decision ordering, day windows, stuck detection)

## Out of Scope

- The metrics artifact (ticket 018) — the runner emits markers; metrics derive from them.
- The canonical `no_human_day_001` fixture content (ticket 021) — this is the engine that runs it.
- TUI/debug rendering and the no-human-day debug report (ticket 023).
- The full acceptance gate over the canonical fixture (ticket 025).

## Acceptance Criteria

### Tests That Must Pass

1. The runner advances a full set of day windows with no bound controller, driving a small actor set through generation→selection→planning→proposals; all physical change is via action events (no pre-scheduled locations).
2. Multiple actors deciding at one tick are ordered by the stable §10.1 keys; emitted events contain no player/controller identity (grep-proof).
3. An actor with no progress past its window, a reasonless repeated idle, or an end-of-run unexplained state emits a typed `StuckDiagnostic`.

### Invariants

1. The day runs coherently with no human controller and no player-conditioned event facts (`INV-004`, Spec §3.2).
2. Decision ordering is deterministic by stable keys; the run is replayable; no randomness is introduced (`INV-018`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/scheduler.rs` (unit tests) — day advancement, stable ordering, action-event-only physical change, stuck detection cases.
2. `crates/tracewake-core/tests/acceptance_gates.rs` (modify) — a minimal no-human day-advance smoke (extended to the full gate by ticket 025).

### Commands

1. `cargo test -p tracewake-core scheduler`
2. `cargo test -p tracewake-core --test acceptance_gates`
3. Core-crate scope is correct; the full `no_human_day_001` acceptance/replay gate is ticket 025.
