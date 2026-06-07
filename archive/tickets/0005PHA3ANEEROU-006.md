# 0005PHA3ANEEROU-006: Agent-state replay rebuild, checksum, and projection comparison

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — extends replay rebuild, the checksum/agent-state projection, and the replay report to reconstruct and verify Phase 3A cognition state.
**Deps**: 0005PHA3ANEEROU-005

## Problem

Replay must reconstruct Phase 3A need values, intention/routine state, salient traces, and diagnostics byte-identically, and a replay that silently drops a Phase 3A need/routine/diagnostic event is a test failure (Spec 0005 §15.1, §15.3, §15.4; `INV-018`, `INV-019`). The existing checksum focuses on physical state, so Phase 3A must extend checksum/replay reporting (or add a complementary agent-state projection check) so need/routine/intention state cannot diverge silently. This is the determinism enforcement surface tickets 001–005 deferred to.

## Assumption Reassessment (2026-06-07)

1. Replay rebuild lives in `crates/tracewake-core/src/replay/rebuild.rs`, the report in `crates/tracewake-core/src/replay/report.rs`, the module in `replay/mod.rs`; the physical checksum is in `crates/tracewake-core/src/checksum.rs` and projections in `crates/tracewake-core/src/projections.rs` (all confirmed present). Phase 2A added an epistemic projection comparison alongside the physical checksum (Spec §15.4 notes physical checksum may stay separate if there is an explicit agent-state check); this ticket follows that precedent for agent state.
2. The Phase 3A event kinds and the agent-state slice on `state.rs` land in ticket 005; this ticket consumes them. Spec §15.1 fixes what replay must reconstruct (need values/effects, threshold crossings that affected decisions, candidate-goal evaluations where eventful, intention lifecycle, routine step lifecycle, sleep/eat/work events, autonomous proposals, salient planner traces, no-human day markers, stuck diagnostics). Spec §15.3 fixes replay-failure reporting fields (event position, kind, schema/version issue, unsupported kind, missing projection/state support, checksum mismatch, actor/routine/trace ID).
3. Shared boundary under audit: this ticket adds a replay/checksum arm for every Phase 3A `EventKind` variant from ticket 005 — an unhandled variant must fail loudly with event position (Spec §15.3), not silently drop. The agent-state projection version is `AgentProjectionVersion` from ticket 001.
4. Invariants motivating this ticket: `INV-018` — "Deterministic replay is foundational" (identical inputs + versions reproduce state byte-identically) — and `INV-019` — "Snapshots and compaction may not erase live ancestry". Windowed/batched passive need deltas (Spec §15.2) must still rebuild identical values and traces.
5. Deterministic-replay / no-leak surface: this IS the determinism enforcement surface. It verifies agent-state rebuild is byte-identical and that unsupported versions reject loudly; it adds no production decision logic and no leakage (the agent-state projection is a deterministic function of the log, carrying no out-of-band truth). Because it changes replay/checksum *coverage* but not replay/hash *semantics* (the envelope/ordering/version scheme is unchanged, additive per ticket 005), no foundational-doc amendment is required — confirm this holds during implementation and escalate to `/reassess-spec` if the agent-state checksum would alter the canonical physical-state hash.

## Architecture Check

1. A separate agent-state projection/checksum alongside the physical and epistemic ones (rather than folding cognition into the physical hash) keeps each projection's ancestry independently debuggable and avoids perturbing the existing physical checksum's golden values — Spec §15.4 explicitly permits this. Failing loudly on an unsupported Phase 3A event kind with its event position turns a silent drop into a typed replay error (`INV-018`).
2. No backwards-compatibility shims: additive projection/checksum and report fields; the physical checksum and its goldens are untouched.

## Verification Layers

1. Deterministic replay (`INV-018`) -> replay/golden-fixture check: replaying a Phase 3A event log rebuilds need/intention/routine/trace state byte-identically; an agent-state projection comparison catches divergence.
2. Loud failure (`INV-018`; Spec §15.3) -> unit test: an unsupported/dropped Phase 3A event kind or version produces a replay-failure report carrying event position, kind, and actor/routine/trace ID — never a silent skip.
3. Ancestry preserved (`INV-019`) -> unit test: windowed passive need-delta batching rebuilds identical values; a batched-vs-unbatched run yields the same final need state and traces.

## What to Change

### 1. Replay rebuild

Extend `crates/tracewake-core/src/replay/rebuild.rs` to apply every Phase 3A `EventKind` variant into the agent-state slice during rebuild, reconstructing need values, intention lifecycle, routine step lifecycle, and salient traces/diagnostics. An unrecognized/unsupported Phase 3A kind or version halts with a typed error rather than skipping.

### 2. Agent-state checksum/projection

Extend `crates/tracewake-core/src/checksum.rs` and/or `crates/tracewake-core/src/projections.rs` with an agent-state projection (versioned by `AgentProjectionVersion`) and a comparison usable in acceptance tests, so need/routine/intention divergence between original and replayed runs is detected.

### 3. Replay report

Extend `crates/tracewake-core/src/replay/report.rs` with the §15.3 failure fields (event position, kind, schema/version, unsupported-kind, missing-projection/state support, checksum mismatch, actor/routine/trace ID).

## Files to Touch

- `crates/tracewake-core/src/replay/rebuild.rs` (modify — rebuild agent state from Phase 3A events)
- `crates/tracewake-core/src/replay/report.rs` (modify — Phase 3A replay-failure fields)
- `crates/tracewake-core/src/replay/mod.rs` (modify — re-export if needed)
- `crates/tracewake-core/src/checksum.rs` (modify — agent-state checksum/projection comparison)
- `crates/tracewake-core/src/projections.rs` (modify — agent-state projection)

## Out of Scope

- Emitting the events (tickets 005, 007–017) — this ticket only rebuilds/verifies them.
- The no-human day runner and metrics (tickets 017, 018).
- TUI/debug rendering of agent state (tickets 022, 023).
- The full Phase 3A acceptance/replay gate over `no_human_day_001` (ticket 025).

## Acceptance Criteria

### Tests That Must Pass

1. A synthetic Phase 3A event log replays to a byte-identical agent-state projection; mutating one need-delta event changes the projection (catch-divergence proof).
2. An unsupported Phase 3A event kind/version produces a replay-failure report with event position, kind, and actor/routine/trace ID — not a silent skip.
3. A run using windowed passive need-delta batching rebuilds the same final need values and traces as the unbatched equivalent.

### Invariants

1. Identical inputs + versions reproduce agent state byte-identically; no Phase 3A event is silently dropped (`INV-018`).
2. Batching/windowing preserves ancestry and final values (`INV-019`); the physical checksum and its goldens are unchanged.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/replay/rebuild.rs` (unit tests) — agent-state rebuild and unsupported-kind loud failure.
2. `crates/tracewake-core/tests/golden_scenarios.rs` (modify) — an agent-state replay-determinism scenario over a synthetic Phase 3A log (extended further by the capstone).

### Commands

1. `cargo test -p tracewake-core replay`
2. `cargo test -p tracewake-core --test golden_scenarios`
3. Core-crate + golden-scenario scope is the correct determinism boundary; the end-to-end `no_human_day_001` replay gate is the capstone (025).

## Outcome

Completed: 2026-06-07

What changed:
- Added deterministic agent-state checksum reporting for `AgentState` while leaving the physical checksum unchanged.
- Extended replay rebuild to apply `Agent` stream events into an agent-state slice and expose final agent state/checksum.
- Added Phase 3A replay failure records carrying event position, kind, schema version, actor ID, routine ID, and trace ID where available.
- Extended replay reports with agent checksum/projection version and Agent-stream failure fields.
- Added unit and golden coverage for deterministic agent replay, checksum divergence on changed need deltas, unsupported Agent-event schema reporting, and windowed need-delta batching equivalence.

Deviations from original plan:
- The agent-state projection is implemented as a checksum report over canonical agent-state lines rather than a separate view-model style projection module; this follows the spec allowance for a complementary agent-state checksum.
- Behavior-specific Agent events that do not yet mutate the agent-state slice are processed as supported no-op events until their emitters/state effects land in later tickets.

Verification results:
- `cargo fmt --all --check` passed.
- `cargo test -p tracewake-core replay` passed.
- `cargo test -p tracewake-core --test golden_scenarios` passed.
