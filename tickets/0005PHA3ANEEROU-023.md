# 0005PHA3ANEEROU-023: Debug surface — needs, routines, planner traces, candidate goals, stuck, no-human day

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — extends debug reports and TUI debug panels to expose full Phase 3A cognition state and the no-human day summary, non-authoritatively.
**Deps**: 0005PHA3ANEEROU-017, 0005PHA3ANEEROU-018

## Problem

Debug must expose the full Phase 3A state — all actors' needs, intentions, routines, routine step state, candidate goals, selected/rejected methods with reasons, planner/decision traces, stuck diagnostics, the no-human day summary, replay/projection comparison, and the hidden-truth audit — while never mutating actor knowledge, adding observations, repairing failures, or becoming authority (Spec 0005 §14.2, §3.7; `INV-008`, `INV-041`). This is how the harsh "explain Mara's missing food / name every stuck actor" acceptance bar is met.

## Assumption Reassessment (2026-06-07)

1. Debug reports are generated in `crates/tracewake-core/src/debug_reports.rs` and rendered in the TUI `crates/tracewake-tui/src/debug_panels.rs` (with `app.rs`/`input.rs` for commands); Phase 2A already exposes debug epistemics/notebooks/replay panels (Spec §4.1). This ticket extends that surface, reusing the non-diegetic debug discipline.
2. The cognition data/trace/diagnostic models (001–004), the decision/planner emitters (012–014), the runner (017), and the metrics artifact (018) provide the state. Spec §14.2 fixes the required debug surface and the acceptable commands (`debug needs`, `debug routines`, `debug planner actor_mara`, `debug stuck`, `debug no-human-day`, `debug actor actor_mara`). Spec §3.7 fixes that debug may reveal full state but must not mutate/repair/authorize.
3. Shared boundary under audit: debug projections are read-only derivations over the log/agent-state projection (ticket 006); rendering must not feed back into the actor-knowledge path (`view_models.rs`, ticket 022). The debug commands extend the TUI input dispatch alongside the existing `debug epistemics`/`debug replay` commands.
4. Invariants motivating this ticket: `INV-008` — "UI assistance is not authority" (debug reveals but decides nothing) — and `INV-041` — "Agent decisions need debug traces" (debug must show beliefs used, needs/duties considered, active intention, selected method/plan, alternatives rejected, blocked preconditions, interruptions, resulting events). Spec §25.4 warns against "debug panels that reveal but cannot explain" — the planner-trace view must answer *why*.
5. Actor-knowledge / debug-authority surface: debug is the truth-revealing surface that must stay non-diegetic. It renders full cognition state (including other actors and the hidden-truth audit result) for the operator, but it must not create observations, mutate `NeedState`/beliefs, or repair failures (Spec §3.7). This ticket adds no authority and no leakage *into the actor model* — the leak that matters (`INV-024`) is debug truth reaching the embodied view, which ticket 022 prevents and the capstone proves.

## Architecture Check

1. Extending the existing debug-report projection + TUI debug panels (rather than a new authoritative inspector) keeps debug as a read-only projection over the log, structurally unable to mutate or repair state (`INV-008`, `INV-041`). A dedicated planner-trace view that renders candidate goals/selected method/rejected reasons/hidden-truth audit answers the "why" the spec demands, not just "what."
2. No backwards-compatibility shims: additive debug reports/panels/commands; reuses the Phase 2A debug command dispatch and non-diegetic discipline.

## Verification Layers

1. Debug-not-authority (`INV-008`; Spec §3.7) -> codebase grep-proof + unit test: debug report generation and panel rendering take read-only references; no path mutates needs/beliefs/world state or creates observations.
2. Explains why (`INV-041`) -> view-model/debug test: `debug planner actor_mara` renders the candidate goals, selected method, rejected reasons, blocked preconditions, and hidden-truth audit for Mara's food failure; `debug stuck` shows blocker category + outcome.
3. Deterministic projection (`INV-018`) -> replay/golden check: the no-human day summary and debug projections render deterministically and match replay.

## What to Change

### 1. Debug reports

Extend `crates/tracewake-core/src/debug_reports.rs` with read-only Phase 3A projections: all actors' need state, current routine/intention, routine step state, candidate goals considered, selected/rejected methods with reasons, planner/decision traces, stuck diagnostics, the no-human day summary + metrics (ticket 018), replay/projection comparison, and the hidden-truth audit result.

### 2. TUI debug panels + commands

Extend `crates/tracewake-tui/src/debug_panels.rs` (and `app.rs`/`input.rs`) to render these and dispatch the §14.2 commands (`debug needs`, `debug routines`, `debug planner <actor>`, `debug stuck`, `debug no-human-day`, `debug actor <actor>`). Ordinary-life actions remain semantic-menu affordances (ticket 022), not raw debug commands (Spec §14.2).

## Files to Touch

- `crates/tracewake-core/src/debug_reports.rs` (modify — Phase 3A read-only debug projections)
- `crates/tracewake-tui/src/debug_panels.rs` (modify — render Phase 3A debug panels)
- `crates/tracewake-tui/src/app.rs` (modify — debug command dispatch)
- `crates/tracewake-tui/src/input.rs` (modify — parse `debug needs`/`routines`/`planner`/`stuck`/`no-human-day`/`actor`)

## Out of Scope

- The embodied (actor-known) surface (ticket 022) — debug is full-state for the operator.
- Core cognition/runner/metrics logic (tickets 001–018).
- The full acceptance/no-leak gate (ticket 025).

## Acceptance Criteria

### Tests That Must Pass

1. `debug planner actor_mara` renders Mara's candidate goals, selected method, rejected reasons, blocked preconditions, and hidden-truth audit; `debug stuck` shows each stuck actor's blocker category and outcome.
2. `debug needs`/`debug routines`/`debug no-human-day`/`debug actor <actor>` render deterministically and match replay.
3. A grep-proof confirms debug report generation and panel rendering mutate no need/belief/world state and create no observations.

### Invariants

1. Debug reveals full state but mutates nothing and authorizes nothing (`INV-008`, Spec §3.7).
2. Debug answers *why* (needs, beliefs, candidates, selected/rejected method, blockers, interruptions) (`INV-041`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/tui_acceptance.rs` (modify) — debug command rendering for needs/routines/planner/stuck/no-human-day (extended by capstone).
2. `crates/tracewake-core/src/debug_reports.rs` (unit tests) — read-only projection content + determinism.

### Commands

1. `cargo test -p tracewake-tui --test tui_acceptance`
2. `cargo test -p tracewake-core debug_reports`
3. TUI + core-projection scope is the correct debug boundary; the full no-human-day explainability gate is the capstone (025).
