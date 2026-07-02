# 0065TUIFULCRO-001: crossterm dependency and terminal lifecycle restore guard

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adds the `crossterm` dependency to `tracewake-tui`; introduces a new `shell` module (terminal lifecycle + RAII restore guard)
**Deps**: None

## Problem

The interactive TUI has no real terminal event loop — `main.rs` runs `run_command_loop`, a line REPL (`crates/tracewake-tui/src/run.rs:13`). Before a fullscreen event loop can exist (Spec 0065 §1.1.2), the TUI must be able to enter raw mode and the alternate screen and, critically, **guarantee restore on normal exit AND on panic**, so a crash never leaves the user's terminal corrupted (§1.1.1, §4.2). No panic hook / restore convention exists today (grep: no `set_hook` / `panic::` under `crates/tracewake-tui/src/`).

## Assumption Reassessment (2026-07-02)

1. `crates/tracewake-tui/src/lib.rs` declares modules `app, debug_panels, input, intent, launch, render, run, screen, transcript` — there is no `shell` module yet; this ticket adds `pub mod shell;`. `crates/tracewake-tui/Cargo.toml` has `ratatui = { version = "0.29.0", default-features = false }` and no `crossterm` entry.
2. Spec 0065 §1.1.1 (terminal lifecycle: enter/exit alternate screen and raw mode, guaranteed restore, handle resize) and §4.2 (guaranteed restore on exit/panic); §8 states `crossterm` is the only new dependency here. `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` governs the client boundary — the shell is a presentation-only adapter holding no world authority.
3. Shared boundary under audit: the new `shell` module boundary inside `tracewake-tui` (presentation/IO only, no core/world state). The `crossterm` version must match the one ratatui 0.29 already resolves — `Cargo.lock` pins `crossterm 0.29.0` transitively — so add `crossterm = "0.29"` (whose default features include `event`) to avoid a duplicate crossterm in the tree; the `ratatui::crossterm` re-export is the version-safe alternative.
4. INV-065 (the TUI is a primary product interface — must remain playable and architecturally protected) motivates the guaranteed-restore requirement: a corrupted terminal on crash makes the primary interface unusable.

## Architecture Check

1. An RAII guard (a struct whose `Drop` disables raw mode and leaves the alternate screen) is cleaner than scattered enter/exit calls: `Drop` runs on normal return AND during panic unwind, so restore is guaranteed *by construction* rather than by remembering cleanup on every exit path. A panic hook additionally resets the terminal before the default panic message prints, so the message is legible.
2. No backwards-compatibility shims: this is a new module; `run_command_loop` is untouched by this ticket.

## Verification Layers

1. INV-065 (robust primary interface) -> controlled test (ratatui `TestBackend`, no TTY) that the guard applies and then restores the terminal state sequence on drop.
2. Restore-on-panic robustness -> test that a panic inside the guarded scope still restores state (`catch_unwind`).
3. Single-layer note: this ticket touches no validation / actor-knowledge / deterministic-replay surface (pure terminal I/O), so no epistemic/determinism layer mapping applies.

## What to Change

### 1. Add the crossterm dependency

`crates/tracewake-tui/Cargo.toml`: add `crossterm = "0.29"` (matches the transitively-locked `0.29.0`; default features include the `event` module needed by 002).

### 2. New `shell` module with a terminal restore guard

- `crates/tracewake-tui/src/lib.rs`: add `pub mod shell;`.
- `crates/tracewake-tui/src/shell/mod.rs`: module root; re-export the guard.
- `crates/tracewake-tui/src/shell/terminal.rs`: a `TerminalGuard` that on construction enables raw mode + `EnterAlternateScreen`, and on `Drop` restores (`disable_raw_mode` + `LeaveAlternateScreen`); install a panic hook that restores the terminal before delegating to the previous hook. Write the guard over a generic/​injectable backend so the restore sequencing is testable without a real TTY.

## Files to Touch

- `crates/tracewake-tui/Cargo.toml` (modify)
- `crates/tracewake-tui/src/lib.rs` (modify)
- `crates/tracewake-tui/src/shell/mod.rs` (new)
- `crates/tracewake-tui/src/shell/terminal.rs` (new)

## Out of Scope

- The event loop / key handling / redraw (0065TUIFULCRO-002).
- Entrypoint / launch wiring (0065TUIFULCRO-003).
- Deleting or reframing `run_command_loop` (0065TUIFULCRO-003).

## Acceptance Criteria

### Tests That Must Pass

1. A controlled test (ratatui `TestBackend`, no TTY) constructs the guard, asserts the enter sequence, drops it, and asserts the restore sequence — proving restore on normal exit.
2. A test asserts restore still occurs when the guarded scope panics (`std::panic::catch_unwind`).
3. `cargo build -p tracewake-tui --all-targets --locked` compiles with the new dependency and `cargo test -p tracewake-tui shell::terminal` passes.

### Invariants

1. The guard restores terminal state on every exit path (normal return + panic unwind).
2. The `shell` module holds no world/simulation state — presentation/IO only.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/src/shell/terminal.rs` (unit tests) — enter/restore sequencing via `TestBackend`; panic-restore via `catch_unwind`. A real raw-mode enable/disable smoke test, if added, is `#[ignore]`d (requires a TTY; not CI-authoritative per §4.4).

### Commands

1. `cargo test -p tracewake-tui shell::terminal`
2. `cargo build -p tracewake-tui --all-targets --locked`

## Outcome

Completed: 2026-07-02

Implemented the ticket by adding `crossterm = "0.29"` to `tracewake-tui`,
updating `Cargo.lock`, exporting a new `shell` module, and adding
`shell::terminal` with `TerminalGuard`, `CrosstermTerminal`, and an injectable
`TerminalLifecycle` test seam. The real terminal lifecycle enables raw mode,
enters the alternate screen, and restores raw mode plus alternate screen on
drop. The crossterm constructor also installs a panic hook that attempts a real
terminal restore before delegating to the previous hook.

Deviation from plan: the reassessment note said `Cargo.lock` already pinned
`crossterm 0.29.0` transitively through `ratatui`; live inspection showed it did
not, because `ratatui` is built with `default-features = false`. Cargo resolved
and locked `crossterm 0.29.0` when the dependency was added. The no-TTY tests
use an injected lifecycle recorder rather than `ratatui::TestBackend`, because
raw-mode and alternate-screen lifecycle operations are `crossterm` I/O
operations, not buffer-rendering behavior.

Verification:

1. `cargo test -p tracewake-tui shell::terminal` — passed; 2 terminal lifecycle
   tests passed, including normal drop and panic unwind restore.
2. `cargo build -p tracewake-tui --all-targets --locked` — passed.
