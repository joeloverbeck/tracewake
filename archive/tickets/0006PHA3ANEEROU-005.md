# 0006PHA3ANEEROU-005: Needs lifecycle and candidate reevaluation

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` needs + candidate generation (`agent/need.rs`, `agent/generation.rs`); eat/sleep/work need-cost modeling
**Deps**: 0006PHA3ANEEROU-001

## Problem

Needs exist as types but do not reliably drive live behavior (audit **D-05 / F-05**, needs aspect). `NeedState`, `NeedBand`, `NeedChangeCause`, and `NeedPressure` exist (`crates/tracewake-core/src/agent/need.rs`), and candidate generation can derive goals from needs when supplied them — but the no-human runner passes `needs: Vec::new()`, so threshold crossings never trigger candidate reevaluation. With 0006PHA3ANEEROU-001 making need-delta events apply to live `AgentState`, this ticket makes those live needs meaningful to candidate generation: hunger/fatigue/safety bands drive candidate goals, threshold crossings trigger reevaluation, and eat/sleep/work apply modeled need costs. Implements spec §5.3.

## Assumption Reassessment (2026-06-07)

1. `agent/need.rs` defines `NeedState`/`NeedBand`/`NeedChangeCause`/`NeedPressure` (confirmed per audit §5). Candidate generation lives in `crates/tracewake-core/src/agent/generation.rs`. Eat/sleep/work emit `NeedDeltaApplied`/`FoodConsumed` (event kinds confirmed present in `events/`).
2. Spec §5.3 requires hunger/fatigue/safety with current values + bands meaningful to candidate generation, logged+applied tick/time effects, eating reducing hunger via `FoodConsumed`/`NeedDeltaApplied` ancestry, sleep reducing fatigue, work increasing fatigue/hunger, threshold crossings triggering candidate reevaluation, and debug/replay explaining the event that last changed each need. §9 forbids direct hidden need mutation and fixture-only need changes used as behavioral proof.
3. Shared boundary under audit: live `AgentState` needs (applied by 0006PHA3ANEEROU-001) → candidate generation input. After this ticket, candidate generation reads live needs/bands, not empty vectors.
4. Motivating invariants (restated): INV-039 "Needs are pressures, not puppet strings" (needs influence choice without erasing belief/cost/risk) and INV-045 "Ordinary survival is causal" (no fake meter refills disconnected from world state).
5. Deterministic-replay surface touched: tick/time need effects and threshold crossings must be eventful and replay identically. This ticket adds no nondeterministic input; need deltas remain event-derived (via 0006PHA3ANEEROU-001) and band/threshold computation is pure. Debug/replay can name the last need-change event per need.

## Architecture Check

1. Deriving candidate goals from live need bands (rather than from caller-supplied vectors) is the only way needs become a behavioral pressure; it keeps utility scoring a bounded heuristic over explicit motives (INV-038) rather than a meter readout. Threshold-crossing reevaluation is modeled as need-event consequence, not a polling hack.
2. No backwards-compatibility aliasing/shims: the empty-needs construction in the scheduler is removed by the decision loop (0006PHA3ANEEROU-007); this ticket provides the live-needs → candidate path it consumes, with no parallel fake-meter path.

## Verification Layers

1. INV-045 (causal survival) -> replay/golden-fixture check: eating reduces hunger only through `FoodConsumed`/`NeedDeltaApplied` ancestry; no fixture-only need jump counts.
2. INV-039 (needs as pressure) -> manual review + test: a hunger threshold crossing produces a hunger candidate goal without overriding access/cost/belief constraints.
3. Debug explainability -> codebase grep-proof + test: the last need-change event per need is recoverable for debug/replay.

## What to Change

### 1. Live needs → candidate generation

`agent/generation.rs` consumes live `AgentState` needs/bands to generate candidate goals; bands map to candidate priorities.

### 2. Threshold-crossing reevaluation

A need crossing a band threshold (via applied need events) triggers candidate reevaluation for that actor.

### 3. Eat/sleep/work need costs

Confirm eat reduces hunger, sleep reduces fatigue, work increases fatigue/hunger through emitted need events (the application is 0006PHA3ANEEROU-001; this ticket ensures the deltas and bands are modeled coherently).

## Files to Touch

- `crates/tracewake-core/src/agent/need.rs` (modify)
- `crates/tracewake-core/src/agent/generation.rs` (modify)
- `crates/tracewake-core/src/agent/candidate.rs` (modify)

## Out of Scope

- The decision loop that calls candidate generation per window (0006PHA3ANEEROU-007).
- Intention/routine lifecycle (0006PHA3ANEEROU-006).
- The mechanics of applying need events to live state (0006PHA3ANEEROU-001, dependency).

## Acceptance Criteria

### Tests That Must Pass

1. A test where a hunger need crosses threshold produces a hunger candidate goal from live `AgentState` (not from a supplied need vector).
2. A test that eating reduces hunger only via `FoodConsumed`/`NeedDeltaApplied` ancestry; a hidden/direct need set is rejected or impossible.
3. `cargo test --workspace` and `cargo clippy --workspace --all-targets -- -D warnings` pass.

### Invariants

1. Candidate generation reads live needs; no empty-needs or fixture-only need path is treated as behavioral proof.
2. Every need change is event-derived and band/threshold computation is deterministic.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/agent/generation.rs` — unit tests: band → candidate goal, threshold-crossing reevaluation.
2. `crates/tracewake-core/src/agent/need.rs` — unit tests: deterministic band/threshold computation, last-change-event recovery.

### Commands

1. `cargo test -p tracewake-core agent::generation`
2. `cargo test --workspace`
3. `cargo clippy --workspace --all-targets -- -D warnings`

## Outcome

Completed: 2026-06-07

Added `generate_candidate_goals_from_agent_state`, a live `AgentState` entry
point that derives candidate needs from the actor's current event-applied need
state instead of requiring callers to pass an ad hoc need vector. The no-human
scheduler bridge now calls that live-needs generator; when the current wait-only
bridge cannot resolve a selected need goal into a proposal yet, it falls back to
the existing idle wait candidate rather than dropping the actor from the
scheduled decision order.

Added deterministic threshold-crossing reevaluation logic and tests proving an
increasing hunger band crossing requests candidate reevaluation. Added
need-source provenance via `last_change_source_label`, with round-trip coverage
so debug/replay can name the last modeled cause of a need value.

Verified with:

1. `cargo test -p tracewake-core agent::generation`
2. `cargo test --workspace`
3. `cargo clippy --workspace --all-targets -- -D warnings`
4. `cargo build --workspace --all-targets --locked`
5. `cargo fmt --all --check`
