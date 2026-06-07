# 0006PHA3ANEEROU-008: Debug/metrics/TUI visibility and run-no-human-day command

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` debug reports/metrics (`debug_reports.rs`); `tracewake-tui` panels/command surface (`app.rs`, `input.rs`, `run.rs`, `render.rs`, `debug_panels.rs`); README runbook
**Deps**: 0006PHA3ANEEROU-007

## Problem

Phase 3A behavior is not TUI-reachable or debug-inspectable enough (audit **D-08 / F-08**, and **D-07 / F-07** read side). Debug panels render synthetic/seeded data; the command parser exposes `debug no-human-day` (a metrics summary of the current log), not a command to run/advance a no-human day; TUI action submission does not refresh live `AgentState` from agent events (fixed structurally in 0006PHA3ANEEROU-001 but the read surfaces must consume it). Spec §5.10 requires debug/report surfaces (needs with last cause, intention/routine/candidate/method/proposal/fallback/stuck/hidden-truth audit, no-human metrics from the real event log, tri-projection replay comparison) and TUI/view-model reachability: embodied actor-filtered status without hidden truth, debug view of Phase 3A internals as non-diegetic, a command/harness to run/advance a no-human day, and post-run panels showing real traces.

## Assumption Reassessment (2026-06-07)

1. TUI surfaces confirmed: `crates/tracewake-tui/src/{app,input,run,render,debug_panels}.rs`; `debug_reports.rs` in core. The command parser exposes `debug no-human-day` (metrics view) per audit F-08; no run/advance command exists. After 0006PHA3ANEEROU-007 the no-human loop and structured traces exist; this ticket renders them.
2. Spec §5.10 enumerates the required debug surfaces and the TUI/view-model reachability list (embodied actor-filtered status without hidden truth; debug view non-diegetic; run/advance command or stable view-model harness; post-run panels show real traces; possession attach/detach/switch testable without resetting AI state). §7.10 requires a TUI/view-model test that runs/inspects an actual no-human day/segment.
3. Shared boundary under audit: the read/projection edge from structured traces/metrics (0006PHA3ANEEROU-007) and live `AgentState` (0006PHA3ANEEROU-001) to (a) the embodied actor-filtered view model and (b) the non-diegetic debug view. The TUI consumes view models and submits typed actions; it must not query hidden truth in embodied mode.
4. Motivating invariants (restated): INV-065 "The TUI is a primary product interface", INV-066 "Every runnable phase has a TUI/view-model gate", INV-070 "Why-not explanations are mandatory", INV-067 "Embodied mode shows actor-known reality", INV-068 "Debug mode is visibly non-diegetic".
5. No-leak surface touched: the embodied view model must show needs/intention/routine status filtered to actor-known reality (no hidden truth, other minds, or debug causal graphs); the debug view may reveal truth but is visibly non-diegetic and read-only with respect to actor cognition (inspection does not mutate AI state). Determinism: metrics are derived from the actual event log, not seeded; the replay-comparison surface reports physical+epistemic+agent projection equivalence.
6. Schema change: the Phase 3A view-model projection is extended (needs-with-cause, intention/routine/candidate/method panels, no-human run command output). Consumers: TUI render/panels and the §7.10 test. Additive to the view-model projection (new read-only fields/panels); no embodied field exposes hidden truth.

## Architecture Check

1. A run/advance command (or stable view-model harness) that drives the 0006PHA3ANEEROU-007 loop and renders the resulting real traces is what turns Phase 3A from "panels over seeded data" into a playable, inspectable substrate (INV-066/071). Keeping embodied and debug projections as distinct view models enforces the no-leak boundary structurally — the embodied panel cannot render a field the actor-filter omits.
2. No backwards-compatibility aliasing/shims: seeded/synthetic panel data paths are removed; panels render real projections only. The TUI keeps submitting typed action attempts and consuming view models — no simulation rule moves into the TUI (INV-069).

## Verification Layers

1. INV-066 / INV-071 (TUI/view-model gate) -> replay/golden-fixture check: a TUI or view-model test runs/advances an actual no-human day/segment and inspects resulting actor-filtered + debug projections (§7.10).
2. INV-067 / INV-024 (embodied actor-known only) -> manual review + test: the embodied Phase 3A view shows needs/intention/routine status with no hidden truth; a negative test asserts a hidden fact is absent from the embodied view but present in debug.
3. INV-068 (debug non-diegetic, read-only) -> manual review + test: debug inspection and possession attach/detach do not mutate AI state; metrics derive from the real event log.

## What to Change

### 1. Real debug reports/metrics

`debug_reports.rs` renders structured traces/diagnostics (0006PHA3ANEEROU-007) and computes no-human metrics from the actual event log; add the tri-projection replay-comparison surface (physical + epistemic + agent).

### 2. Run/advance no-human-day command

Add a TUI command (or stable view-model harness entry) to run/advance a no-human day/segment, distinct from the existing `debug no-human-day` metrics view; refresh live `AgentState` from emitted events after each step.

### 3. Actor-filtered embodied vs non-diegetic debug panels

Embodied Phase 3A status panel shows actor-known needs/intention/routine without hidden truth; debug panels show Phase 3A internals as non-diegetic; post-run panels show real generated traces. Update the README runbook for the new command.

## Files to Touch

- `crates/tracewake-core/src/debug_reports.rs` (modify)
- `crates/tracewake-tui/src/app.rs` (modify)
- `crates/tracewake-tui/src/input.rs` (modify)
- `crates/tracewake-tui/src/run.rs` (modify)
- `crates/tracewake-tui/src/render.rs` (modify)
- `crates/tracewake-tui/src/debug_panels.rs` (modify)
- `crates/tracewake-tui/tests/tui_acceptance.rs` (modify)
- `README.md` (modify)

## Out of Scope

- The decision loop and structured-trace generation (0006PHA3ANEEROU-007, dependency) — this ticket renders/inspects them.
- The full no-human-day capstone + replay-equivalence acceptance (0006PHA3ANEEROU-010).
- Any simulation-rule logic in the TUI (forbidden by INV-069).

## Acceptance Criteria

### Tests That Must Pass

1. A TUI/view-model test runs or advances an actual no-human day/segment and asserts the embodied view shows needs/intention/routine status without hidden truth, and debug panels show real generated traces/diagnostics.
2. A negative test: a hidden fact is absent from the embodied view but visible in debug; possession/debug inspection does not mutate AI state; no-human metrics reflect actual events.
3. `cargo test --workspace` and `cargo clippy --workspace --all-targets -- -D warnings` pass.

### Invariants

1. Embodied view models never expose hidden truth; debug view is non-diegetic and read-only with respect to actor cognition.
2. The no-human substrate is reachable/inspectable through the TUI or the same actor-filtered view-model boundary.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/tui_acceptance.rs` — run/advance a no-human segment; assert embodied vs debug projection content and read-only inspection.

### Commands

1. `cargo test -p tracewake-tui --test tui_acceptance`
2. `cargo test --workspace`
3. `cargo clippy --workspace --all-targets -- -D warnings`
