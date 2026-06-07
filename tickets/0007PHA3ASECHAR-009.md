# 0007PHA3ASECHAR-009: TUI / view-model / debug Phase 3A behavioral proof

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` view models + debug reports (`view_models.rs`, `debug_reports.rs`); `tracewake-tui` panels and command loop (`debug_panels.rs`, `render.rs`, `run.rs`, `app.rs`)
**Deps**: 0007PHA3ASECHAR-004, 0007PHA3ASECHAR-005, 0007PHA3ASECHAR-006, 0007PHA3ASECHAR-007

## Problem

Phase 3A behavior must be reachable, inspectable, and regression-tested through the TUI / actor-filtered view models, with proof of actual decisions/blockers/needs/intentions/routine progress — not just labels (Spec 0007 D-08, D-07, §TUI/view-models/debug panels). Embodied views must show Phase 3A needs and the active intention for the possessed actor without revealing hidden food/route/workplace; debug panels must expose no-human metrics, planner trace, needs, intentions, routine execution state, stuck diagnostics, and replay comparison, derived from event ancestry and typed records, read-only, and asserted by behavior-specific rows rather than labels like `candidate_goals` or `no_human_day_metrics_v1`.

## Assumption Reassessment (2026-06-07)

1. Confirmed surfaces exist: `crates/tracewake-core/src/view_models.rs`, `crates/tracewake-core/src/debug_reports.rs`, `crates/tracewake-tui/src/debug_panels.rs`, `crates/tracewake-tui/src/render.rs`, `crates/tracewake-tui/src/run.rs`, and `App::run_no_human_day` (`crates/tracewake-tui/src/app.rs:247`). Existing TUI acceptance tests already assert hidden food is not rendered (`crates/tracewake-tui/tests/tui_acceptance.rs:126,154,173`).
2. Spec 0007 D-08 requires tests proving the TUI exposes actual no-human decisions, blockers, needs, intentions, routine progress, replay metrics, and actor-filtered views without hidden truth; D-07 requires debug output derived from event ancestry and typed records, not seeded labels/static strings; §TUI requires debug panels be read-only and covered by checksum/event-count tests, with behavior-specific row assertions.
3. Cross-artifact boundary under audit: the actor-filtered view-model contract between `tracewake-core` (view models / debug reports, producers) and `tracewake-tui` (panels / renderer / command loop, consumers). The embodied view must project only actor-known state; the debug panels must be non-mutating projections of typed records.
4. Motivating invariants (restated): INV-067 "Embodied mode shows actor-known reality" — hides hidden truth/other minds; INV-068 "Debug mode is visibly non-diegetic"; INV-069 "The TUI must not implement simulation rules" — consumes view models, mutates nothing; INV-070 "Why-not explanations are mandatory"; INV-065/INV-066 TUI is a primary, gated interface; INV-093 "Actor-knowledge leakage is a high-severity defect".
5. Actor-knowledge-filtering surface touched (central): embodied view models must not project hidden food, hidden route, hidden workplace access, or omniscient planner facts; debug panels are non-diegetic and read-only (no world/agent/epistemic mutation). The TUI submits typed action attempts only; it does not read hidden truth in embodied mode. No determinism change — panels render replay-derived metrics.

## Architecture Check

1. Deriving panels from typed decision/planner/routine records and the no-human log (rather than seeded labels) makes the TUI a genuine inspection surface whose tests assert behavior, not cosmetics, and keeps the embodied/debug split honest (actor-filtered vs non-diegetic). Keeping all projection logic in `tracewake-core` view models preserves the TUI-consumes-view-models boundary (INV-069).
2. No backwards-compatibility aliasing/shims: panels do not read hidden truth in embodied mode and do not mutate state; no label-only rows substitute for behavioral rows.

## Verification Layers

1. INV-067 / INV-093 (actor-known embodied view) -> manual review + test: embodied view shows the possessed actor's Phase 3A needs and active intention but not hidden food/route/workplace (negative assertion).
2. INV-068 / INV-069 (debug non-diegetic, read-only) -> replay/golden-fixture check: debug no-human / planner / stuck panels render replay-derived metrics and are covered by checksum/event-count tests; a before/after state checksum proves no mutation.
3. INV-070 (why-not) -> test: the debug planner panel exposes the selected goal, rejected goals, method-condition proofs, hidden-truth proof sources, and blocker categories for a real no-human decision.
4. INV-066 (TUI gate) -> test: `run no-human-day` through the command loop produces the integrated capstone ancestry (asserted by behavior-specific rows).

## What to Change

### 1. Embodied Phase 3A view models

Extend `view_models.rs` so the embodied view for the possessed actor surfaces Phase 3A needs (value/band/last cause) and the active intention, actor-filtered — no hidden food/route/workplace.

### 2. Debug panels derived from typed records

Make `debug_reports.rs` / `debug_panels.rs` expose no-human metrics, planner trace (selected/rejected goals, condition proofs, hidden-truth proof sources, blocker categories), active/suspended intentions, routine execution state, stuck diagnostics, and replay comparison — derived from event ancestry and typed records, read-only.

### 3. Command-loop reachability

Ensure `run`/`app` expose running/advancing no-human segments and inspecting the resulting metrics through the command loop.

## Files to Touch

- `crates/tracewake-core/src/view_models.rs` (modify)
- `crates/tracewake-core/src/debug_reports.rs` (modify)
- `crates/tracewake-tui/src/debug_panels.rs` (modify)
- `crates/tracewake-tui/src/render.rs` (modify)
- `crates/tracewake-tui/src/run.rs` (modify)
- `crates/tracewake-tui/src/app.rs` (modify)

## Out of Scope

- The underlying decision/need/intention/routine emission (0007PHA3ASECHAR-004/005/006/007, dependencies) — this ticket renders them, it does not produce them.
- Live-vs-replay checksum plumbing internals (0007PHA3ASECHAR-010) — consumed here for the replay-comparison panel.
- The capstone end-to-end assertion (0007PHA3ASECHAR-012).

## Acceptance Criteria

### Tests That Must Pass

1. A TUI test: the embodied view shows the possessed actor's Phase 3A needs and active intention; a negative assertion proves hidden food/route/workplace are not rendered.
2. A TUI test: the debug planner panel shows selected goal, rejected goals, method-condition proofs, hidden-truth proof sources, and blocker categories for a real no-human decision.
3. A TUI test: the debug no-human panel is log-derived and read-only (before/after state checksum equality); the stuck panel shows typed no-progress diagnostics from the integrated run.
4. A TUI test: `run no-human-day` through the command loop produces the integrated capstone ancestry (behavior-specific rows, not labels).
5. `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check` pass.

### Invariants

1. Embodied views are actor-filtered; no hidden truth leaks into any rendered row.
2. Debug panels are read-only projections of typed records; they mutate no world/agent/epistemic state.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/tui_acceptance.rs` — embodied needs/intention rows; hidden-truth negative assertions; debug planner/no-human/stuck panel behavior rows; read-only checksum.
2. `crates/tracewake-tui/tests/command_loop_session.rs` — `run no-human-day` integrated ancestry through the loop.
3. `crates/tracewake-core/src/view_models.rs` — unit tests: actor-filtered Phase 3A projection.

### Commands

1. `cargo test -p tracewake-tui`
2. `cargo test --workspace`
3. `cargo clippy --workspace --all-targets -- -D warnings`
