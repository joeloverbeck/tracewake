# 0002PHA1KERTUI-020: TUI shell — app loop, render, semantic input

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — populates `tracewake-tui` with `app`, `render`, `input` modules and the binary entry.
**Deps**: 0002PHA1KERTUI-012, 0002PHA1KERTUI-008, 0002PHA1KERTUI-017

## Problem

A passing Phase 1 must expose the slice through a TUI; a test-only or debug-command-only kernel is insufficient (Spec 0002 §6.5, §15). The TUI consumes kernel view models and submits stable semantic commands; it may be ugly but not fake, and must not mutate world state, apply events, infer rules, or use a terminal menu index as action identity (§15.2). This ticket builds the app loop, the renderer, and the input mapping (typed keys/selections → stable semantic action IDs) for embodied play: bind/attach, render local view, list/submit semantic actions, why-not, wait/advance.

## Assumption Reassessment (2026-06-06)

1. `tracewake-tui` exists as an empty crate from ticket 001 (depending on core + content); this populates it. It consumes the embodied view model + semantic-action entries (ticket 012), submits proposals through the shared pipeline (ticket 008), and loads a fixture via the content crate (ticket 017).
2. TUI responsibilities/non-responsibilities are `specs/0002_…_SPEC.md` §15.1/§15.2; the embodied view-model fields it renders are §15.3; semantic-action selection by stable ID (not menu index) is §15.4. The terminal-UI library choice is an implementation detail deferred from ticket 001 and selected here (kept out of `tracewake-core`).
3. Shared boundary under audit: the TUI→pipeline submission path (TUI submits `Proposal`s, never mutates state) and the view-model→render path (TUI renders kernel-produced models, never infers availability). Debug panels are a separate surface (ticket 021).
4. Invariant motivating this ticket: INV-065 (the TUI is a primary product interface), INV-069 (the TUI must not implement simulation rules), and INV-008 (UI assistance is not authority).
5. No-leak surface: the TUI renders only the embodied view model (ticket 012), which is already actor-knowledge-filtered (§12.4); it has no access to hidden truth in embodied mode. It submits proposals and renders results; it never applies events or mutates state. This ticket adds no leakage path and no rule logic; the TUI-never-mutates regression is in ticket 022.

## Architecture Check

1. A TUI that maps input to stable semantic action IDs from the view model and submits proposals to the kernel (rather than computing affordances itself) keeps all rules in the kernel — the TUI is a thin client. Using the view model's `semantic_action_id` (not the terminal row index) as identity makes selection stable and testable (§15.4). The terminal library stays a `tracewake-tui` dependency only, preserving the §7.2 dependency direction.
2. No backwards-compatibility shims: greenfield.

## Verification Layers

1. TUI owns no rules (INV-069) -> codebase grep-proof: `tracewake-tui` never takes `&mut` core state and never calls the event applier; it submits proposals only.
2. Stable semantic selection (§15.4) -> unit test: input mapping resolves a selection to the view model's `semantic_action_id`, not the menu index.
3. View-model-only rendering (INV-008/067) -> manual review: the renderer reads only the embodied view model, with no access to hidden state or the full event log in embodied mode.

## What to Change

### 1. App loop

Add `crates/tracewake-tui/src/app.rs`: the runtime loop binding a controller to an actor, requesting an embodied view model each tick, and dispatching submitted semantic actions to the shared pipeline.

### 2. Render + input

Add `crates/tracewake-tui/src/render.rs` (terminal display of the embodied view model — local place, exits, doors, containers, visible items, actors, semantic actions, why-not) and `crates/tracewake-tui/src/input.rs` (map keys/selections to stable semantic action IDs or control commands).

### 3. Binary entry

Wire `crates/tracewake-tui/src/main.rs` to load a Phase 1 fixture and start the app.

## Files to Touch

- `crates/tracewake-tui/src/app.rs` (new)
- `crates/tracewake-tui/src/render.rs` (new)
- `crates/tracewake-tui/src/input.rs` (new)
- `crates/tracewake-tui/src/main.rs` (modify — load fixture + start app; file created by ticket 001)
- `crates/tracewake-tui/src/lib.rs` (modify — declare `app`/`render`/`input`; file created by ticket 001)
- `crates/tracewake-tui/Cargo.toml` (modify — add the chosen terminal-UI dependency; file created by ticket 001)

## Out of Scope

- Debug panels and the transcript/snapshot harness (ticket 021).
- Any rule logic, event application, or hidden-state query (forbidden — kernel owns rules).

## Acceptance Criteria

### Tests That Must Pass

1. The TUI binds to an actor, renders the local embodied view, lists stable semantic actions, submits one, and renders the updated view.
2. A rejected action renders a structured why-not derived from the validation report.
3. Input selection resolves to a `semantic_action_id`, never a terminal menu index; `tracewake-tui` never mutates core state directly.

### Invariants

1. The TUI submits proposals and renders view models only; it never mutates world state or applies events.
2. Action identity is the stable semantic ID, not a menu index.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/src/input.rs` (unit tests) — selection→semantic-ID mapping.
2. `crates/tracewake-tui/tests/embodied_flow.rs` — bind → render → submit → re-render → why-not over a fixture.

### Commands

1. `cargo test -p tracewake-tui`
2. `cargo build --workspace`
3. TUI-crate scope is correct; the full `view_model_local_actions_001` transcript determinism runs in ticket 021/022.
