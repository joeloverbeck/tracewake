# 0007PHA3ASECHAR-005: Live needs integration with ancestry

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` needs (`agent/need.rs`), no-human loop passive deltas (`scheduler.rs`), event application (`events/apply.rs`), action need-delta emission (`actions/defs/eat.rs`, `actions/defs/sleep.rs`, `actions/defs/work.rs`)
**Deps**: 0007PHA3ASECHAR-004

## Problem

Needs must drive live decisions through replayable event ancestry rather than being read as static thresholds at decision time (Spec 0007 D-02, §Needs). The no-human loop reads live need values (`agent_state.needs_by_actor`, `hunger >= 500`, `fatigue >= 500` at `crates/tracewake-core/src/scheduler.rs:559-575`) but does not consistently (a) emit passive need deltas with tick/window ancestry *before* each decision, (b) emit `NeedDeltaApplied` carrying action-event ancestry from action effects, (c) re-evaluate candidates on threshold crossings, or (d) let a severe need interrupt a lower-priority routine intention through typed events. §Needs also forbids encoding need state as proposal parameters where the pipeline can read live `AgentState` from `PipelineContext`.

## Assumption Reassessment (2026-06-07)

1. Confirmed: the no-human loop reads live needs at `crates/tracewake-core/src/scheduler.rs:559-575`. The need machinery exists — `NeedState::apply_delta` (`crates/tracewake-core/src/agent/need.rs:166`), `NeedState::threshold_crossing` (`need.rs:177`), `NeedChangeCause` (`need.rs:85`), `NeedState::last_change_source_label` (`need.rs:162`), `NeedPressure`/`derive_pressure` (`need.rs:197`). The event kinds `NeedDeltaApplied` and `NeedThresholdCrossed` already exist (`crates/tracewake-core/src/events/envelope.rs:46`).
2. Spec 0007 §Needs requires passive deltas emitted as agent events with tick/window ancestry, action effects emitting `NeedDeltaApplied` with action-event ancestry, threshold crossings triggering candidate re-evaluation, severe needs interrupting lower-priority routine intentions via typed events, and replay reproducing the same values + last-cause summaries. D-02 requires needs advance "through the same event ancestry it emits".
3. Shared boundary under audit: the `NeedState` records in `AgentState` between the no-human scheduler (which must emit passive + action-derived deltas) and the replay rebuild + debug/view-model surfaces (consumers that reproduce value, band, last cause, last threshold crossing). No new event kind is needed; the gap is emission with correct ancestry from the integrated path.
4. Motivating invariants (restated): INV-039 "Needs are pressures, not puppet strings" — needs influence selection without erasing belief/cost/access; INV-045 "Ordinary survival is causal" — food/sleep/fatigue must be real enough to cause behavior, no fake meter refills; INV-009/INV-010 — each meaningful need change is eventful with a cause model.
5. Deterministic-replay surface touched: `NeedDeltaApplied` / `NeedThresholdCrossed` events are the replay source for need state. Passive deltas must be emitted with deterministic tick/window ancestry and applied in `events/apply.rs` so the live and replay-rebuilt `NeedState` (value, band, last cause, last threshold crossing) are byte-identical (agent-state checksum). No actor-knowledge leakage: a need is the actor's own interoceptive state, not hidden world truth.

## Architecture Check

1. Emitting passive and action-derived need deltas as events (rather than mutating need state silently and reading it at decision time) makes need evolution causal, replayable, and debuggable — satisfying current-state-is-not-enough (INV-011) — and lets the severe-need interruption be a typed event other surfaces can inspect, instead of an implicit branch.
2. No backwards-compatibility aliasing/shims: need state is not copied into proposal parameters where the pipeline can read live `AgentState` from `PipelineContext`; the event-sourced path is the single source.

## Verification Layers

1. INV-045 / INV-009 (causal needs, eventful) -> replay/golden-fixture check: a no-human run emits passive `NeedDeltaApplied` before decisions and action-derived `NeedDeltaApplied` after eat/sleep, each with ancestry.
2. INV-018 (replay equality) -> replay/golden-fixture check: agent-state checksum equality reproduces need value, band, last cause, last threshold crossing.
3. INV-039 (threshold-driven re-eval / interruption) -> unit test: a threshold crossing triggers candidate re-evaluation; a severe need interrupts a lower-priority routine intention via a typed event.

## What to Change

### 1. Passive need deltas before decisions

In the no-human loop, before each actor decision, emit passive `NeedDeltaApplied` (and `NeedThresholdCrossed` when crossed) derived from elapsed time / window, with tick/window ancestry, applied through `events/apply.rs`.

### 2. Action-derived need deltas with ancestry

Have eat/sleep/work action effects emit `NeedDeltaApplied` carrying the action event as ancestry, using `NeedChangeCause` and `NeedState::apply_delta`.

### 3. Threshold re-evaluation and severe-need interruption

On a threshold crossing, trigger candidate re-evaluation; when a severe need outranks an active routine intention, emit a typed intention/routine interruption (the lifecycle event is owned by 0007PHA3ASECHAR-006; this ticket raises the severe-need cause).

## Files to Touch

- `crates/tracewake-core/src/agent/need.rs` (modify)
- `crates/tracewake-core/src/scheduler.rs` (modify)
- `crates/tracewake-core/src/events/apply.rs` (modify)
- `crates/tracewake-core/src/actions/defs/eat.rs` (modify)
- `crates/tracewake-core/src/actions/defs/sleep.rs` (modify)
- `crates/tracewake-core/src/actions/defs/work.rs` (modify)

## Out of Scope

- The intention lifecycle event transitions themselves (0007PHA3ASECHAR-006) — this ticket supplies the severe-need interruption cause, not the intention state machine.
- Routine step progress ancestry (0007PHA3ASECHAR-007).
- Debug/TUI rendering of need value/band/last-cause (0007PHA3ASECHAR-009).
- Live-vs-replay checksum plumbing for agent state (0007PHA3ASECHAR-010) — this ticket emits the events that checksum consumes.

## Acceptance Criteria

### Tests That Must Pass

1. A no-human run emits passive `NeedDeltaApplied` before decisions and action-derived `NeedDeltaApplied` (with action ancestry) after eat/sleep, asserted by event scan.
2. Replay reproduces need value, band, last cause, and last threshold crossing (agent-state checksum equality).
3. A unit test: a fatigue threshold crossing re-evaluates candidates; a severe hunger interrupts a lower-priority routine intention via a typed event.
4. `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check` pass.

### Invariants

1. Every meaningful need change is an event with tick/window or action ancestry; no silent need mutation read at decision time.
2. Need state is not transported as proposal parameters where live `AgentState` is readable from `PipelineContext`.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/agent/need.rs` — unit tests: apply_delta cause + last-cause label; threshold crossing detection.
2. `crates/tracewake-core/src/scheduler.rs` — unit test: passive delta emitted before decision; severe-need interruption raised.
3. `crates/tracewake-content/src/fixtures/sleep_eat_work_001.rs` — assert action-derived `NeedDeltaApplied` ancestry (via existing fixture run).

### Commands

1. `cargo test -p tracewake-core agent::need`
2. `cargo test --workspace`
3. `cargo clippy --workspace --all-targets -- -D warnings`
