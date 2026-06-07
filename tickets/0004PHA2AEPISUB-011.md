# 0004PHA2AEPISUB-011: TUI parser, app, and renderers for notebook and debug epistemics

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — extends the `tracewake-tui` parser, app loop, and renderers (`input.rs`, `app.rs`, `render.rs`, `debug_panels.rs`).
**Deps**: 0004PHA2AEPISUB-008, 0004PHA2AEPISUB-009, 0004PHA2AEPISUB-010

## Problem

Mechanics hidden from the TUI are unfinished (`INV-071`); the TUI is a primary product interface (`INV-065`) and must consume view models without implementing rules (`INV-069`). Spec 0004 §12.4/§12.5 require the TUI to parse and render `notebook`, `debug epistemics`, `debug beliefs <actor_id>`, and `debug observations <actor_id>`, plus the actor-known why-not summary for the accusation probe — all by consuming the view models from tickets 008/009/010, preserving existing commands, and rejecting bad actor ids / debug shapes with typed errors.

## Assumption Reassessment (2026-06-06)

1. `crates/tracewake-tui/src/input.rs` parses `help`, `view`, `bind`, `do`, numeric selection, `wait`, and existing debug commands, with no `notebook`/`debug epistemics`/`debug beliefs`/`debug observations`. `crates/tracewake-tui/src/render.rs` and `debug_panels.rs` render existing panels; `app.rs` is the loop. This ticket extends all four.
2. Command names/shapes are fixed by Spec 0004 §12.4; the renderers consume `NotebookView` (ticket 009) and the debug epistemics views (ticket 010) and the why-not summary (ticket 008). The TUI must not re-derive epistemic state.
3. Shared boundary under audit: the TUI command enums (`UiCommand`/`DebugCommand`) and the renderer dispatch are extended here; existing command tests (`crates/tracewake-tui/tests/command_loop_session.rs`, `tui_acceptance.rs`, `transcript_snapshot.rs`) must continue passing.
4. Invariant motivating this ticket: `INV-065` (TUI is a primary product interface), `INV-069` (TUI must not implement simulation/epistemic rules — it consumes view models and submits typed attempts), and `INV-070` (why-not explanations available, actor-filtered).
5. Actor-knowledge / no-leak surface: the embodied `notebook` renderer must display only the actor-known `NotebookView` (ticket 009) and never query ground truth; the debug renderers display the non-diegetic-marked debug views (ticket 010) only under debug mode. The TUI adds no epistemic rule and no leakage path — it is a pure presentation consumer. Determinism: rendering must be transcript/snapshot-stable (no hash-ordered tables).

## Architecture Check

1. Extending the existing `UiCommand`/`DebugCommand` parser and the renderer dispatch (rather than a parallel epistemic input path) preserves the single command-loop contract and keeps the TUI presentation-only. The renderers consume pre-built view models, so no epistemic logic leaks into the TUI layer.
2. No backwards-compatibility shims: new commands are additive; existing command parsing and transcripts are preserved.

## Verification Layers

1. TUI reachability (`INV-071`/`INV-066`) -> transcript test: `notebook`, `debug epistemics`, `debug beliefs <actor>`, `debug observations <actor>` are reachable and produce deterministic output.
2. TUI implements no rules (`INV-069`) -> manual review + grep-proof: the renderers call view-model accessors only; no belief/observation/contradiction is computed in `tracewake-tui`.
3. Embodied why-not, debug separated (`INV-070`/`INV-067`) -> transcript test: the embodied notebook/why-not shows no ground-truth string; debug panels carry the non-diegetic marker.

## What to Change

### 1. Command parsing

Extend `input.rs` `UiCommand` with `notebook` and `DebugCommand` with `epistemics`, `beliefs <actor_id>`, `observations <actor_id>`; reject bad actor ids and malformed debug shapes with typed errors. Preserve existing commands and numeric selection resolving to stable semantic action ids.

### 2. App loop wiring

Wire the new commands in `app.rs` to fetch the corresponding view models (notebook, debug epistemics/beliefs/observations) and the probe why-not summary, and dispatch to renderers.

### 3. Renderers and panels

Add renderers in `render.rs` for the actor-known notebook and the embodied why-not summary, and in `debug_panels.rs` for the debug epistemics/beliefs/observations panels (preserving the non-diegetic marker). Rendering must be deterministic and snapshot-testable.

## Files to Touch

- `crates/tracewake-tui/src/input.rs` (modify — `UiCommand`/`DebugCommand` extensions)
- `crates/tracewake-tui/src/app.rs` (modify — command wiring)
- `crates/tracewake-tui/src/render.rs` (modify — notebook + why-not renderers)
- `crates/tracewake-tui/src/debug_panels.rs` (modify — debug epistemics panels)

## Out of Scope

- The view models themselves (tickets 008/009/010).
- README/doc updates for the new commands (ticket 015).
- The end-to-end transcript acceptance session (ticket 016).

## Acceptance Criteria

### Tests That Must Pass

1. `notebook` renders Tomas's actor-known notebook; `debug epistemics`/`debug beliefs actor_tomas`/`debug observations actor_tomas` render their panels with the non-diegetic marker.
2. A malformed `debug beliefs <bad-id>` produces a typed error, not a panic or silent ignore.
3. Existing command-loop/transcript tests still pass; numeric selection still resolves to stable semantic action ids.

### Invariants

1. The TUI computes no epistemic state; it renders view models only.
2. Embodied output leaks no ground truth; debug output is visibly non-diegetic.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/command_loop_session.rs` (extend) — new commands parse and dispatch.
2. `crates/tracewake-tui/tests/transcript_snapshot.rs` (extend) — deterministic notebook/debug-epistemics rendering.

### Commands

1. `cargo test -p tracewake-tui`
2. `cargo test -p tracewake-tui --test transcript_snapshot`
3. `cargo build --workspace --all-targets --locked`
