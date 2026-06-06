# 0003PHA1AEXETUI-002: Executable stdin/stdout command loop runner and thin main

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new `crates/tracewake-tui/src/run.rs` (runner + in-file test module), rewrites `crates/tracewake-tui/src/main.rs`, registers the module in `crates/tracewake-tui/src/lib.rs`.
**Deps**: 0003PHA1AEXETUI-001

## Problem

`main.rs` (`crates/tracewake-tui/src/main.rs:1-10`) is a one-shot path: load the default fixture, bind `actor_tomas`, print the readiness line, render one embodied view, and exit. It never reads stdin or loops. Spec §Goals and §"Startup" require `cargo run -p tracewake-tui` to enter a genuine stdin/stdout command loop, and §"Required implementation shape" requires a testable runner the binary and tests share.

## Assumption Reassessment (2026-06-06)

1. `TuiApp` (`crates/tracewake-tui/src/app.rs`, read this session) already exposes everything the loop needs: `load_default` (72), `bind_actor` (103), `current_view` / `render_current_view` (119 / 133), `submit_semantic_action` (137), and `render_debug_*` panels (228-281). `startup_message()` returns `"tracewake-tui ready"` (`lib.rs:9-11`). The loop adds no new world logic.
2. The parser variants this loop dispatches (`Help`, `View`, `SelectByMenuIndex`, `Debug(..)`) are produced by 0003PHA1AEXETUI-001 (Deps). Numeric resolution uses the existing `semantic_id_for_selection(&view, n)` (`input.rs:45-56`), which maps a 1-based menu index to a stable `SemanticActionId`.
3. Shared boundary under audit: this ticket consumes the `UiCommand`/`DebugCommand` contract from 001 and the `TuiApp` facade. The loop is a presentation/dispatch layer only — it submits typed attempts and renders results.
4. INV-069 (the TUI must not implement simulation rules) and INV-065 (the TUI is a primary product interface) motivate this ticket — restated: the loop "consumes actor-filtered view models and submits typed action attempts; it must not query hidden truth in embodied mode, maintain quest state, bypass validators, or mutate world state." Every mutation routes `loop → TuiApp::submit_semantic_action → Proposal → run_pipeline` (spec §"Mutation boundary"). INV-069 at `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md:299`; INV-065 at :283.
5. Deterministic-replay / no-leak enforcement surface: the loop must keep debug output non-diegetic (INV-068) and never let debug truth or hidden state enter the embodied render or change the physical checksum (INV-024 / INV-093; spec §"Debug boundary"). The loop introduces no RNG and no wall-clock; ordering stays the scheduler's, so it adds no nondeterminism path. The negative regressions that prove this are owned by sibling 0003PHA1AEXETUI-003.

## Architecture Check

1. A `run_command_loop(app, reader, writer)` over `impl BufRead` / `impl Write` lets the binary and the scripted tests exercise the identical loop (spec §"Required implementation shape"), with `main` reduced to a thin stdin/stdout wrapper. This is cleaner than embedding the loop directly in `main`, which would be unreachable from test code.
2. No backwards-compatibility shims: `main.rs` is rewritten (its one-shot render becomes the loop's initial render), not aliased alongside a legacy path.

## Verification Layers

1. INV-069 / INV-008 (no rule logic; index ≠ identity) -> codebase grep-proof: `run.rs` calls only `TuiApp` facade methods plus `semantic_id_for_selection`; it contains no `apply_event` and no direct `PhysicalState` field write (the guard is extended to `run.rs` in 0003PHA1AEXETUI-003).
2. INV-070 (why-not) -> in-file runner test + manual review: a rejected command prints the actor-visible why-not summary and re-renders the view.
3. INV-068 / INV-024 (debug non-diegetic / no leak) -> binary regression in 0003PHA1AEXETUI-003 (named, not collapsed here): debug panels carry the non-diegetic marker and never alter the next embodied render or the checksum.

## What to Change

### 1. New `run.rs` runner

`run_command_loop(app: &mut TuiApp, reader: impl BufRead, writer: impl Write) -> std::io::Result<()>`: render the initial embodied view and a prompt; for each input line call `parse_command` and dispatch — `Help` prints command help (no mutation); `View` re-renders (no mutation); `BindActor` calls `bind_actor` then renders; `SelectSemanticAction` and `SelectByMenuIndex` (resolved via `semantic_id_for_selection` against the current view) call `submit_semantic_action`, then print an accepted/why-not line and re-render; `WaitOneTick` submits the wait action through the same path; `Debug(..)` calls the matching `render_debug_*`; `Quit` and EOF return cleanly. Invalid input prints an actor-safe error line and mutates nothing.

### 2. Thin `main.rs`

`load_default`, default-bind `actor_tomas`, print `startup_message()`, then call `run_command_loop(&mut app, stdin().lock(), stdout().lock())`.

### 3. Register the module

Add `pub mod run;` to `crates/tracewake-tui/src/lib.rs`.

## Files to Touch

- `crates/tracewake-tui/src/run.rs` (new)
- `crates/tracewake-tui/src/main.rs` (modify)
- `crates/tracewake-tui/src/lib.rs` (modify)

## Out of Scope

- Numeric / debug command *parsing* (0003PHA1AEXETUI-001).
- Actual-binary scripted-process tests and the no-`apply_event` guard extension (0003PHA1AEXETUI-003).
- README / ledger reconciliation (0003PHA1AEXETUI-004).
- A `ratatui`/`crossterm` runtime backend, new domain mechanics, Phase 2 (spec §Non-goals).

## Acceptance Criteria

### Tests That Must Pass

1. The in-file runner test drives `run_command_loop` over an in-memory scripted session and asserts readiness/render/accepted/why-not/`DEBUG NON-DIEGETIC`/clean-exit output.
2. `cargo build --workspace --all-targets --locked`.
3. `cargo clippy --workspace --all-targets -- -D warnings`.

### Invariants

1. Every world mutation flows `run_command_loop → TuiApp::submit_semantic_action → run_pipeline`; `run.rs` and `main.rs` contain no `apply_event` and no direct `PhysicalState` write.
2. Debug commands render panels only; the next `view` renders actor-filtered embodied content unchanged.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/src/run.rs` (in-file `#[cfg(test)]` module) — drives `run_command_loop` with an in-memory `BufReader` over a scripted session (`view`, a numeric selection, `wait`, `debug log`, an invalid line, `quit`) and a `Vec<u8>` writer; asserts the expected substrings and a clean return. (Rationale: the runner's dispatch/output is fully testable in-memory; the *actual-binary* process layer is owned by 0003PHA1AEXETUI-003.)

### Commands

1. `cargo test -p tracewake-tui run`
2. `printf 'view\nquit\n' | cargo run -q -p tracewake-tui` (manual loop smoke)
3. `cargo build --workspace --all-targets --locked && cargo clippy --workspace --all-targets -- -D warnings`

## Outcome

Completion date: 2026-06-06

What changed:

- Added `tracewake-tui::run::run_command_loop`, a shared stdin/stdout command runner over `BufRead` and `Write`.
- Reworked `main.rs` into a thin wrapper that loads the default fixture, binds `actor_tomas`, prints `tracewake-tui ready`, and enters the shared command loop.
- Added in-file runner tests for scripted command dispatch, numeric selection, accepted and rejected action output, debug panel rendering, invalid input, prompt output, and help vocabulary.

Deviations from original plan:

- The runner prints the initial embodied view and prompt; `main.rs` remains responsible for the readiness line, matching the binary startup contract without duplicating that line in library tests.
- The prompt is emitted as a deterministic `tracewake>` line so scripted stdin/stdout output remains easy to assert.

Verification results:

- `cargo test -p tracewake-tui run` — passed.
- `cargo fmt --all --check` — passed.
- `printf 'view\nquit\n' | cargo run -q -p tracewake-tui` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `rg -n "apply_event|PhysicalState" crates/tracewake-tui/src/run.rs crates/tracewake-tui/src/main.rs` — no matches.
