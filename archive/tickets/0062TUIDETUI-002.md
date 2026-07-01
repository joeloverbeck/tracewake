# 0062TUIDETUI-002: Intent reducer (presentation-state mutation and authorized dispatch)

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new `tracewake-tui` reducer (`intent/reducer.rs`); registers in `intent/mod.rs`; may expose the existing debug-authority dispatch (`run.rs`) for reuse. No core types, no event/schema/view-model changes.
**Deps**: 0062TUIDETUI-001.

## Problem

Spec 0062 §1.1 item 2 requires a single dispatcher mapping each `UiIntent` either to a mutation of the presentation-state model or to an already-authorized `TuiApp`/core-runtime call — including the existing debug-authority path for `SubmitDebugCommand` (§4.5). The reducer is the seam that guarantees keyboard, key-script, and semantic-script producers cannot fork semantics (§4.1) and that no intent mutates world state directly (§4.2). It creates no new world-mutation path and bypasses no validation.

## Assumption Reassessment (2026-07-01)

1. **Authorized dispatch targets (code).** The reducer routes world-advancing intents to existing methods: `TuiApp::submit_semantic_action` (`crates/tracewake-tui/src/app.rs`), the time-control path `TuiApp::advance_until` (`app.rs`, dispatches `RuntimeCommand::continue_until`) and the one-tick wait path (currently reached via `UiCommand::WaitOneTick` in `run.rs`). These are the settled kernel-owned surfaces; the reducer calls them, it does not reimplement them.
2. **Debug-authority path (code).** Debug commands are currently executed by `render_debug(app, debug_command, writer)` in `crates/tracewake-tui/src/run.rs` (reached via `UiCommand::Debug(DebugCommand)`). Routing `UiIntent::SubmitDebugCommand(DebugCommand)` through the reducer reuses this authority-gated path; if the call site is not reusable as-is, expose it via a small `pub(crate)` seam in `run.rs` or a `TuiApp` method rather than duplicating debug logic. `DebugCommand` is defined at `crates/tracewake-tui/src/input.rs:19`.
3. **Cross-artifact boundary under audit.** The reducer mutates the presentation-state model from 0062TUIDETUI-001 and calls `TuiApp`; it is the shared authority seam that Spec 0065's shell will feed with real key events. Confirm the reducer signature takes `&mut TuiApp` + `&mut PresentationState` so both the headless runner (005) and the 0065 shell drive it identically (INV-108: same authority as a human).
4. **Motivating invariant (INV-104).** `INV-104` ("Routines and needs do not dispatch primitive actions directly … None of them may bypass candidate generation, actor-known context, local planning, proposal construction, shared validation, and event commitment") — its no-direct-dispatch principle governs here: the reducer must reach world changes only through the existing proposal/validation pipeline (`submit_semantic_action`, the scheduler-owned time-control path), never by constructing or applying events itself. `INV-069` (TUI implements no rules) reinforces this.
5. **Fail-closed / no-bypass enforcement surface.** The reducer is the no-direct-dispatch and possession-parity enforcement surface. It must not weaken validation or leak hidden truth: world-advancing arms call only already-authorized methods (which run full validation), the debug arm stays authority-gated and non-diegetic, and presentation-mutation arms touch only the presentation-state model. No new event-application, RNG, or replay path is introduced; determinism and actor-knowledge filtering are unchanged because the reducer adds no authoritative surface. Change rationale (no silent retcon): this replaces the `run.rs` per-`UiCommand` dispatch with an equivalent reducer over `UiIntent`, preserving every existing authorized call and adding no new world-mutation path.

## Architecture Check

1. A single reducer is the one seam through which all three producers (keyboard via 0065, key script, semantic script) reach world/presentation changes, structurally preventing semantic forks (§4.1) — cleaner than per-producer dispatch, which could drift. Reusing `submit_semantic_action`, the time-control path, and the existing debug path (rather than reimplementing any) keeps world authority in the kernel and the debug path authority-gated.
2. No backwards-compatibility aliasing or shims; the legacy line grammar is not preserved as an undocumented alias (spec §1.2).

## Verification Layers

1. INV-104 / INV-069 (no direct dispatch) -> codebase grep-proof: the reducer constructs/applies no event and calls no world-mutation path other than `submit_semantic_action` and the kernel-owned time-control methods.
2. INV-108 (possession cognition-neutral) -> manual review + test: the reducer's world-advancing arms are identical to the human path; scripts gain no player-only action.
3. Debug non-diegetic / authority-gated -> manual review + test: `SubmitDebugCommand` routes through the existing authority-gated debug path; no debug output enters embodied surfaces.

## What to Change

### 1. Add the reducer

Create `crates/tracewake-tui/src/intent/reducer.rs` with a function (e.g. `reduce(app: &mut TuiApp, state: &mut PresentationState, intent: UiIntent) -> ReduceOutcome`) that dispatches each `UiIntent`:
- Presentation intents (`FocusNext`, `FocusPane`, `MoveSelection`, `ActivateSelection`, `ToggleHelp`, `FocusNotebook`) → mutate `PresentationState` only.
- `SubmitSemanticAction(id)` → `TuiApp::submit_semantic_action`.
- `WaitOneTick` / `ContinueUntil { max_ticks }` → the settled kernel-owned time-control methods.
- `SubmitDebugCommand(cmd)` → the existing authority-gated debug-execution path (reuse `run.rs::render_debug`; expose a `pub(crate)` seam if needed).
- `Quit` → a typed quit outcome the runner/shell handles.
Return a typed outcome (rendered output / receipt / quit) so callers render uniformly. Construct or apply no events.

### 2. Register and (if needed) expose the debug seam

Add `pub mod reducer;` to `crates/tracewake-tui/src/intent/mod.rs` (created by 0062TUIDETUI-001). If `render_debug` is not reusable as a function, add a minimal `pub(crate)` wrapper in `crates/tracewake-tui/src/run.rs` (no behavior change) rather than duplicating debug logic.

## Files to Touch

- `crates/tracewake-tui/src/intent/reducer.rs` (new)
- `crates/tracewake-tui/src/intent/mod.rs` (modify — file created by 0062TUIDETUI-001)
- `crates/tracewake-tui/src/run.rs` (modify — as surfaced, only to expose the existing debug-authority dispatch for reuse)

## Out of Scope

- The `UiIntent` enum and presentation-state struct definitions (0062TUIDETUI-001).
- Parsing scripts into intents (003 / 004).
- The headless runner, transcript emission, and screen dumps (005).
- Any new world-mutation path, new core type, or change to semantic-action / 0047 time-control semantics (spec §1.2).
- Retiring the legacy `run_command_loop` line grammar (a later spec; §1.2 Stage E).

## Acceptance Criteria

### Tests That Must Pass

1. A test drives representative intents through `reduce` against a fixture `TuiApp`: a presentation intent mutates `PresentationState` and makes no world change; `SubmitSemanticAction` produces the same receipt the direct `submit_semantic_action` path does; a time-control intent advances the world through the kernel-owned method.
2. A test asserts `SubmitDebugCommand` routes through the authority-gated debug path and produces no embodied-surface output.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test -p tracewake-tui`.

### Invariants

1. The reducer constructs/applies no events and reaches world changes only through existing authorized methods (INV-104, INV-069).
2. Scripted and human dispatch produce identical world effects; no player-only action is introduced (INV-108).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/src/intent/reducer.rs` (unit tests) or `crates/tracewake-tui/tests/` — reducer dispatch coverage for presentation, submit, time-control, and debug arms; rationale: proves the seam routes correctly and mutates world state only through authorized paths.

### Commands

1. `cargo test -p tracewake-tui reducer`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test -p tracewake-tui`

## Outcome

Completed: 2026-07-01

Implemented `intent::reducer::reduce(app, state, intent)` with typed outcomes
for presentation changes, authorized semantic-action receipts, authorized
time-control receipts, debug-render output, and quit. Presentation intents mutate
only `PresentationState`; world-affecting intents route through
`TuiApp::submit_semantic_action`, the current-view wait action followed by
`TuiApp::submit_semantic_action`, or `TuiApp::advance_until`. Debug intents reuse
the existing authority-gated `run.rs` debug rendering path via a narrow
`pub(crate)` seam, so no parallel debug authority path was introduced.

The reducer constructs or applies no events, adds no core type, and introduces
no validator-bypass or hidden-truth surface. The new production reducer source
was classified in the workspace source census with the existing TUI
presentation-boundary rationale.

Verification:

- `cargo test -p tracewake-tui reducer`
- `cargo test -p tracewake-core --test anti_regression_guards workspace_source_classification_census_matches_production_tree`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test -p tracewake-tui`
