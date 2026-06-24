# 0051FOUCONTHI-007: F-06 TUI de-authority atomic cutover

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — move all authoritative mutation behind the core runtime; remove public `&mut`-aggregate paths, the public perception-append helper, and the scheduler-forwarding writer from external reach; the TUI becomes a read-only client.
**Deps**: 0051FOUCONTHI-006

## Problem

`TuiApp` owns the authoritative action registry, physical state, agent state, `EventLog`, `ControllerBindings`, `DeterministicScheduler`, and `EpistemicProjection`, and passes mutable references directly into core mutation functions (bind-time perception append, direct `run_pipeline` for non-wait actions, post-action perception append, `advance_until` aggregate hand-off, and a separate `run_no_human_day` rebuild/replace path). The advertised negative fixture calls a no-argument `tracewake_tui::record_current_place_perception_and_project()` symbol that does not exist (its `expected_stderr` is the vacuous `"cannot find function"`), while the real writer is the public core `record_current_place_perception_and_project` (`agent/perception.rs:71`) plus the public scheduler forwarding writer `record_actor_current_place_perception` (`scheduler.rs:614`) (F-06, critical; vacuity gap). The fix makes the `-001` runtime own all authoritative aggregates and routes every world-affecting action through typed session commands, removing the public write paths.

## Assumption Reassessment (2026-06-24)

1. Codebase: `TuiApp` aggregate ownership in `crates/tracewake-tui/src/app.rs` (`from_golden:98`, scheduler/state/projection fields, `run_no_human_day:375`); the real perception writer `record_current_place_perception_and_project` (`crates/tracewake-core/src/agent/perception.rs:71`, also called in `scheduler.rs:623`/`893`/`1052` and in `tracewake-content/tests/`); the public forwarding writer `record_actor_current_place_perception` (`scheduler.rs:614`). The vacuous fixture source is `tests/negative-fixtures/external_crate_cannot_call_tui_perception_append_helper/` (no-arg call; its replacement is owned by `-009`). The `-001` runtime + commands are the authoritative owner the TUI migrates onto.
2. Specs/docs: spec `0051` §4.7 (F-06). Architecture homes `docs/1-architecture/01_*` (authority boundaries), `10_*` (TUI/client boundaries); execution `docs/2-execution/05_*` (no direct dispatch).
3. Shared boundary under audit: the core↔TUI public surface — no `&mut PhysicalState`/`AgentState`/`EventLog`/`EpistemicProjection` may cross it; every world-affecting semantic action goes through typed session commands.
4. INV-065 (the TUI is a primary product interface), INV-069 (the TUI must not implement simulation rules), INV-008 (UI assistance is not authority): restated — the TUI consumes actor-filtered view models and submits typed action attempts; it does not mutate world state or bypass validators.
5. Fail-closed / actor-knowledge / authority surface: this is the de-authority enforcement surface. Confirm the TUI cannot construct a `PipelineContext` over authoritative aggregates, append perception, replace replay state, or reach mutable runtime fields; bind-time perception, if retained, occurs inside core as a modeled channel (no leakage). The no-human path becomes a command on the same runtime, not a TUI rebuild/replace choreography.
6. Removal blast radius: remove the public event-appending perception helper and the public scheduler forwarding writer (`record_actor_current_place_perception`) from external reach, and remove the TUI's direct `run_pipeline`/`advance_until`/`run_no_human_day` rebuild paths. Repo-wide grep: external consumers of `record_current_place_perception_and_project` are `crates/tracewake-content/tests/fixtures_load.rs` and `golden_fixtures_run.rs` (migrate to the modeled channel or a core path); core call sites (`scheduler.rs`) stay in-core. No public alias is retained.

## Architecture Check

1. This is the §4.1 keystone payoff and the cycle-breaking atomic flip: making client write-authority type-unrepresentable (private fields + crate-private constructors + unexported tokens) is the only design that prevents the seam reopening a fourth time. A split that landed the runtime while leaving any public mutable-aggregate path live would violate the spec's integrity, so this is a deliberate single atomic-cutover diff (override the Split default) — build-additively-then-flip, with the old paths deleted in the same diff.
2. No backwards-compatibility alias: the public perception/forwarding writers and the TUI rebuild path are removed, not wrapped; `#[non_exhaustive]` is insufficient, so the controlling mechanism is privacy + crate-private constructors.

## Verification Layers

1. INV-069 (TUI must not implement simulation rules) -> negative fixtures: external crates cannot construct `PipelineContext` over authoritative aggregates, append perception, replace replay state, or reach mutable runtime fields.
2. INV-008 (UI assistance is not authority) -> codebase grep-proof: no `pub fn` exposes `&mut PhysicalState`/`AgentState`/`EventLog`/`EpistemicProjection` across the TUI/core boundary.
3. INV-065 (TUI remains playable) -> positive in-crate test: the single core owner still performs each operation (bind, submit semantic action, wait, duration, no-human) via typed commands.

## What to Change

### 1. Runtime owns aggregates; TUI is a client

The `-001` runtime owns all authoritative mutable aggregates; the TUI owns only the client/session handle, presentation state, command-parsing state, and immutable typed results.

### 2. All world-affecting actions become commands

Every world-affecting semantic action — including non-wait actions — goes through typed session commands (bind/unbind, submit semantic action, one-tick wait, duration continuation/advance-until, debug-authorized no-human execution). Bind-time perception, if retained, occurs inside core as a modeled channel. The no-human path becomes a command on the same runtime, not a TUI rebuild/replace.

### 3. Remove public write paths

Remove the public event-appending perception helper and the public scheduler forwarding writer from external reach; migrate the `tracewake-content/tests/` consumers. Expose no `&mut PhysicalState`/`AgentState`/`EventLog`/`EpistemicProjection` across the TUI/core public boundary.

## Files to Touch

- `crates/tracewake-core/src/runtime/session.rs` (modify — file created by 0051FOUCONTHI-001)
- `crates/tracewake-tui/src/app.rs` (modify) — TUI becomes a read-only client; merge-hub contributor
- `crates/tracewake-tui/src/run.rs` (modify) — command loop uses session commands
- `crates/tracewake-core/src/agent/perception.rs` (modify) — bind-time perception as an in-core modeled channel; remove public-helper external reach
- `crates/tracewake-core/src/scheduler.rs` (modify) — remove the public forwarding writer from external reach; merge-hub contributor
- `crates/tracewake-content/tests/fixtures_load.rs` (modify) — migrate perception-helper consumer
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify) — migrate perception-helper consumer
- `crates/tracewake-core/tests/negative_fixture_runner.rs` (modify) — add `PipelineContext`/perception/replay-state/runtime-field seals + the positive in-crate test; merge-hub contributor

## Out of Scope

- Splitting the temporal products into sealed debug/embodied types (F-07 → `-008`).
- Replacing the vacuous perception fixture text + the production-path harness (→ `-009`).

## Acceptance Criteria

### Tests That Must Pass

1. Negative fixtures prove the TUI cannot construct `PipelineContext` over authoritative aggregates, append perception, replace replay state, or reach mutable runtime fields (privacy/constructor diagnostics).
2. A positive in-crate test proves the single core owner still performs bind / submit / wait / duration / no-human via typed commands.
3. `cargo test --workspace` is green after the cutover (TUI parity preserved).

### Invariants

1. No `pub fn` exposes `&mut` over the four authoritative aggregates across the TUI/core boundary.
2. The TUI mutates no world state; all mutation is behind typed session commands.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/negative_fixture_runner.rs` + new fixtures — TUI write-path seals.
2. `crates/tracewake-tui/tests/` parity — bind/submit/wait/duration/no-human via commands (existing parity suite updated).

### Commands

1. `cargo test -p tracewake-core --test negative_fixture_runner`
2. `cargo test --workspace`
3. `cargo build --workspace --all-targets --locked && cargo clippy --workspace --all-targets -- -D warnings`

## Outcome

Completed: 2026-06-24

Implemented the F-06 TUI de-authority cutover. `LoadedWorldRuntime` now owns the authoritative registry, physical state, agent state, event log, controller bindings, scheduler, epistemic projection, and manifest id for loaded sessions. The TUI stores the runtime plus presentation/session state and routes bind, submit, advance-until, no-human, rebuild, and perception-refresh mutation through runtime commands instead of holding mutable authoritative aggregates.

Sealed the direct perception append surfaces from external reach: the core perception append helpers are crate-private, the scheduler forwarding writer is crate-private, and content/core integration tests that previously used the helper now build perception events and apply them explicitly for fixture setup. Added compile-fail fixtures for the real core perception append helper, the scheduler forwarding writer, runtime field mutation, and `PipelineContext::new` construction. The older vacuous TUI fixture remains for the `-009` replacement ticket, as scoped in this ticket's Out of Scope.

Updated TUI seam/parity tests to assert the runtime command boundary (`submit_controlled_proposal`) instead of direct `run_pipeline` calls in TUI source, and boxed the parity harness `AppError` variant to keep `clippy::result_large_err` green after runtime errors became richer.

Verification run:

1. `cargo test -p tracewake-core --test negative_fixture_runner --quiet` — passed.
2. `cargo fmt --all --check` — passed.
3. `cargo build --workspace --all-targets --locked` — passed.
4. `cargo clippy --workspace --all-targets -- -D warnings` — passed.
5. `cargo test --workspace --quiet` — passed.
