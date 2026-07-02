# 0065TUIFULCRO-002: Fullscreen event loop, key→intent mapping, and buffer redraw

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — new shell event loop and `KeyEvent`→`UiIntent` mapping in `tracewake-tui`
**Deps**: 0065TUIFULCRO-001

## Problem

With the terminal lifecycle guard in place (001), the shell needs the interactive loop itself: poll `crossterm` key/resize events, translate each key into a `UiIntent`, feed it to the existing Spec 0062 `reduce` seam, track presentation + terminal-size state, and redraw the Spec 0064 `ratatui` buffer from the current actor-filtered `EmbodiedViewModel` (§1.1.2, §1.1.3, §4.1). The live key→intent mapping must produce the **identical** intent stream the Spec 0062 key scripts do, so interactive and headless play cannot fork semantics (§4.3).

## Assumption Reassessment (2026-07-02)

1. The reducer seam is `reduce(app: &mut TuiApp, state: &mut PresentationState, intent: UiIntent) -> Result<ReduceOutcome, ReduceError>` (`crates/tracewake-tui/src/intent/reducer.rs:32`; `ReduceOutcome::Quit` is the exit signal). `PresentationState` (`intent/state.rs:6`) already carries `focused_pane`, `selection_indices`, `scroll_offsets`, `help_open` — the shell reuses it; only "last terminal size" (§1.1.3) is unrepresented and is tracked shell-locally. The redraw seam is `build_embodied_screen_model(&EmbodiedViewModel, RenderOptions)` (`screen/model.rs`, re-exported at `screen/mod.rs:12`) → `render_embodied_to_buffer(&EmbodiedScreenModel, Rect)` (`screen/buffer_render.rs:15`); `RenderOptions` (model.rs:29) and `TerminalSize` (model.rs:46) already exist and carry terminal size into layout. The view-model source is `TuiApp::current_view() -> Result<EmbodiedViewModel, AppError>` (`app.rs:159`).
2. Spec 0065 §1.1.2, §1.1.3, §4.1, §4.3, §4.5 item 2 (event loop) merged with item 3 (shell state). `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` — no direct dispatch: the shell submits world change only via `reduce`'s authorized methods.
3. Shared boundary under audit: the Spec 0062 intent contract. The shell's `KeyEvent`→`UiIntent` mapping and the script token mapping `parse_key_token` (`intent/key_script.rs:15`, driven by `parse_key_script`:8) must agree on the same `UiIntent` for equivalent keys (§4.3). This is a **new** mapping — `parse_key_token` maps *script string tokens*, not live crossterm `KeyEvent`s, so it is not directly reusable; the two converge on the same values, verified by test and preferably by sharing the token→intent core so equivalence holds by construction.
4. INV-108 (possession is cognition-neutral): interactive input must map to the same intents and authority as scripts — restated so the live path gains no capability the scripted path lacks. INV-069 (the TUI implements no simulation rules): the loop reads no hidden truth and mutates no world state except through `reduce`.
5. Fail-closed / actor-knowledge / replay surface: the redraw draws only from `TuiApp::current_view()` — the actor-filtered `EmbodiedViewModel` — and submits world change only through the `reduce` seam (authorized methods). This ticket adds no new truth-read and no new mutation path; it must not draw from any non-actor-filtered source, preserving INV-024 (no telepathy) / INV-006. World state stays event-sourced (INV-018): the shell holds only presentation + terminal-size state, none of it replayable. No schema is extended — the shell-local terminal size is a new field on shell state, and `PresentationState` / `TerminalSize` / `RenderOptions` are reused unchanged (no additive/breaking schema-shape change).

## Architecture Check

1. Feeding the existing `reduce` seam (rather than a parallel dispatch path) means interactive play reuses the exact authorized-dispatch + presentation-mutation logic the scripts use — one reducer, no forked semantics. Deriving the redraw from `current_view()` → `build_embodied_screen_model` → `render_embodied_to_buffer` reuses the Spec 0064 buffer path unchanged.
2. No backwards-compatibility shims: the loop is new; it does not wrap or alias `run_command_loop`.

## Verification Layers

1. INV-108 (cognition-neutral input) -> test that the shell `KeyEvent`→`UiIntent` mapping yields the same `UiIntent` sequence as `parse_key_script` for equivalent keys.
2. INV-069 / INV-024 (no rule / no leak) -> grep-proof + review that the loop's only world-mutation path is `reduce` and its only embodied draw source is `current_view()` (actor-filtered) — no debug/truth accessor in the draw path.
3. INV-018 (deterministic replay unaffected) -> review that shell state is presentation/terminal-size only and never enters the event log.

## What to Change

### 1. `KeyEvent`→`UiIntent` mapping

`crates/tracewake-tui/src/shell/key_map.rs` (new): map crossterm `KeyEvent`s to `UiIntent` values, converging with `parse_key_token`'s token→intent semantics (share the core mapping where practical so §4.3 equivalence is by construction).

### 2. Fullscreen event loop

`crates/tracewake-tui/src/shell/event_loop.rs` (new): poll crossterm key/resize events; on a key → map → `reduce(app, &mut state, intent)`, handle `ReduceOutcome` (including `Quit`); on resize → update the tracked `TerminalSize`; redraw by `build_embodied_screen_model(&app.current_view()?, RenderOptions { size, .. })` → `render_embodied_to_buffer` → draw the buffer through the 001 terminal guard. Hold a `PresentationState` (existing) plus the last `TerminalSize`.

### 3. Register in the shell module

`crates/tracewake-tui/src/shell/mod.rs` (modify — created by 0065TUIFULCRO-001): declare and re-export `key_map` and `event_loop`.

## Files to Touch

- `crates/tracewake-tui/src/shell/key_map.rs` (new)
- `crates/tracewake-tui/src/shell/event_loop.rs` (new)
- `crates/tracewake-tui/src/shell/mod.rs` (modify — file created by 0065TUIFULCRO-001)

## Out of Scope

- Entrypoint / launch wiring (0065TUIFULCRO-003).
- Any change to the reducer, the `PresentationState` schema, or the Spec 0064 buffer path.
- Deleting or reframing `run_command_loop` (0065TUIFULCRO-003).

## Acceptance Criteria

### Tests That Must Pass

1. Key→intent equivalence: for a representative key set, the shell mapping produces the same `Vec<UiIntent>` as `parse_key_script` on the equivalent script tokens (no TTY).
2. Redraw parity: given a fixture view model and a fixed `TerminalSize`, the loop's redraw produces the same buffer as `build_embodied_screen_model` + `render_embodied_to_buffer` applied directly (no drift).
3. `cargo test -p tracewake-tui shell::` passes and `cargo build -p tracewake-tui --all-targets --locked` compiles.

### Invariants

1. The only world-mutation path in the loop is the `reduce` seam (authorized methods).
2. The only embodied draw source is `TuiApp::current_view()` (actor-filtered); no hidden-truth accessor appears in the draw path.
3. Shell state is presentation + terminal-size only; nothing it holds enters the event log.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/src/shell/key_map.rs` (unit tests) — equivalence with `parse_key_script` for equivalent keys.
2. `crates/tracewake-tui/src/shell/event_loop.rs` (unit tests) — redraw parity via ratatui `TestBackend` (no TTY); `Quit` outcome terminates the loop.

### Commands

1. `cargo test -p tracewake-tui shell::`
2. `cargo test -p tracewake-tui --test intent_conformance` (guard: the intent-stream contract is unchanged)
3. `cargo build -p tracewake-tui --all-targets --locked`

## Outcome

Completed: 2026-07-02

Implemented the fullscreen shell event-loop seam by adding:

1. `shell::key_map`, which maps live `crossterm::event::KeyEvent` values to
   existing `UiIntent` values through the same token parser used by Spec 0062
   key scripts.
2. `shell::event_loop::ShellState`, carrying only presentation state plus the
   last terminal size.
3. `run_fullscreen_shell`, `apply_key_event`, `redraw_buffer`, `redraw`, and
   `draw_buffer`, routing key effects through `intent::reducer::reduce` and
   deriving redraws from `TuiApp::current_view()` into the Spec 0064
   `render_embodied_to_buffer` surface.

Deviation from plan: the live crate keeps `ratatui` on
`default-features = false`, so the shell does not enable or depend on
`ratatui::backend::CrosstermBackend`. Instead, it draws the already-produced
Spec 0064 `ratatui::buffer::Buffer` through direct `crossterm` clear/move/print
commands. This keeps `crossterm` as the only new dependency while preserving
the required buffer redraw seam.

Verification:

1. `cargo test -p tracewake-tui shell::` — passed; 8 shell tests passed,
   including normal/panic terminal restore, key-script equivalence,
   redraw-buffer parity, no-TTY buffer output, and quit-step termination.
2. `cargo test -p tracewake-tui --test intent_conformance` — passed; 5
   conformance tests passed, including the no-direct-world-mutation reducer
   source guard and byte-identical script/dump witnesses.
3. `cargo build -p tracewake-tui --all-targets --locked` — passed.

Source audit: `rg` over `crates/tracewake-tui/src/shell` found the shell's only
world-effect path at `reduce(app, state.presentation_mut(), intent)` and its
only embodied redraw source at `app.current_view()`. No runtime/debug truth
accessor or direct dispatch path was introduced in the shell module.
