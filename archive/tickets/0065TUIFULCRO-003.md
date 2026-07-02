# 0065TUIFULCRO-003: Interactive entrypoint wiring and line-loop reframing

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — main entrypoint and launch-option changes in `tracewake-tui`
**Deps**: 0065TUIFULCRO-002

## Problem

The fullscreen shell (001 + 002) exists but nothing launches it. Per §1.1.4 it must become an available interactive entrypoint; per §1.2 the unconditional default-launch flip is deferred to Spec 0069. Approved resolution (Step-4 Q1 option (a)): wire the shell as an available interactive mode through the `launch` module, keep `run_command_loop` as a test/automation utility, and do **not** flip the default launch or migrate the `UiCommand` test corpus.

## Assumption Reassessment (2026-07-02)

1. `crates/tracewake-tui/src/main.rs` resolves `launch::resolve(&args) -> Launch` and then calls `run::run_command_loop(&mut app, stdin, stdout)`. The `launch` module (`src/launch.rs`) exposes `Launch::{Run, List, Help}`, `LaunchMode::{Embodied, OperatorDebug}`, `resolve` (`:36`), `render_catalog` (`:121`), `usage` (`:135`), `render_error` (`:147`). `run_command_loop` (`src/run.rs:13`) has five consumers — `main.rs:44` plus four test files (`tests/adversarial_gates.rs`, `tests/embodied_flow.rs`, `tests/tui_acceptance.rs`, and `run.rs`'s own unit tests) — so it is retained as a test/automation utility.
2. Spec 0065 §1.1.4 (retire/reframe the line loop), §1.2 (default-launch flip deferred to Spec 0069), §9 ("the shell is additive"). Approved Q1(a).
3. Shared boundary under audit: the `launch` CLI contract — adding a fullscreen interactive option (a `LaunchMode` variant or a flag on `Launch::Run`) without removing existing modes or changing the default.
4. INV-065 / INV-071 (the TUI is a primary product interface; mechanics must be reachable through the TUI): wiring the fullscreen shell makes the interactive client genuinely reachable rather than dead code.
5. Mismatch + correction: §1.1.4 read literally ("becomes the interactive entrypoint") would flip the default and delete the line loop; reconciled against §1.2 (flip deferred to 0069) and the five-consumer blast radius of `run_command_loop`, this ticket makes the shell *available* and defers the unconditional default flip to Spec 0069 (Q1(a)).

## Architecture Check

1. Adding the fullscreen path as a launch option (additive) keeps the change small and reversible and honors §1.2's deferral of the default flip — versus deleting the line loop, which would break four test files and contradict §9 ("additive"). `run_command_loop` remains the automation/test grammar until Spec 0069 ratifies the flip.
2. No backwards-compatibility shim: the fullscreen shell is a real new path, not a wrapper over `run_command_loop`; `run_command_loop` is retained deliberately and documented as a test utility, not aliased.

## Verification Layers

1. INV-065 / INV-071 (reachable interactive client) -> grep-proof that `main.rs` dispatches to `shell::…` for the fullscreen option; best-effort interactive smoke that the option starts the shell.
2. No-semantic-fork (§4.1 / §4.3) -> regression test that driving `wait` / `continue` / an action through the shell path yields the same view-model outcomes as the line/script path.

## What to Change

### 1. Launch option

`crates/tracewake-tui/src/launch.rs`: add a fullscreen interactive option (e.g. a `LaunchMode::Fullscreen`, or a `--fullscreen` flag on `Launch::Run`), reflected in `render_catalog` and `usage`.

### 2. main.rs dispatch

`crates/tracewake-tui/src/main.rs`: route the fullscreen option to the shell event loop (001 + 002) inside a `TerminalGuard`; keep the existing default path calling `run_command_loop`.

### 3. Reframe the line loop

`crates/tracewake-tui/src/run.rs`: doc-comment `run_command_loop` as the non-interactive test/automation line driver (no longer the interactive entrypoint). No deletion.

## Files to Touch

- `crates/tracewake-tui/src/launch.rs` (modify)
- `crates/tracewake-tui/src/main.rs` (modify)
- `crates/tracewake-tui/src/run.rs` (modify — doc/reframe only)

## Out of Scope

- The unconditional default-launch flip (Spec 0069).
- Deleting `run_command_loop` or migrating the `UiCommand` test corpus to Spec 0062 intent scripts.
- New panes or view-model changes.

## Acceptance Criteria

### Tests That Must Pass

1. `launch::resolve` accepts the new fullscreen option and yields the expected `Launch` / `LaunchMode`; existing modes and the default resolution are unchanged.
2. No-fork regression: `wait` / `continue` / action driven via the shell path yields the same view-model outcomes as the line/script path.
3. `cargo test -p tracewake-tui` is green — every existing `run_command_loop`-based test still passes, proving it remains a working utility.

### Invariants

1. `run_command_loop` remains callable (test/automation utility); no consumer breaks.
2. The default launch is unchanged (the flip is deferred to Spec 0069).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/src/launch.rs` (unit tests) — the new fullscreen option resolves; existing modes/default unchanged.
2. `crates/tracewake-tui/tests/` (new no-fork regression, or an extension of `embodied_flow.rs`) — shell path vs line/script path view-model parity.

### Commands

1. `cargo test -p tracewake-tui launch`
2. `cargo test -p tracewake-tui`

## Outcome

Completed: 2026-07-02

Implemented the fullscreen launch option by adding a `fullscreen: bool` flag to
`Launch::Run`, accepting `--fullscreen [fixture_id] [actor_id]`, documenting it
in `usage()`, and routing the fullscreen path in `main.rs` through
`TerminalGuard::enter_crossterm()` and `shell::event_loop::run_fullscreen_shell`.
The no-argument default remains unchanged and still runs the stdin/stdout line
driver. `run_command_loop` remains callable and is now documented as the
non-interactive test/automation line driver.

Additional implementation needed for the ticket's no-fork action witness:
`UiIntent::ActivateSelection` now submits the selected Actions-pane semantic
action through the existing authorized `TuiApp::submit_semantic_action` path.
Non-Actions panes and out-of-range action selections remain presentation-only.

Deviation from plan: no live TTY smoke was run; the acceptance lane is the
headless/no-TTY evidence required by the spec. The launch source grep confirmed
that the fullscreen option reaches the shell entrypoint, and shell parity tests
prove the key-step behavior without requiring an interactive terminal.

Verification:

1. `cargo test -p tracewake-tui launch` — passed; 12 launch tests passed,
   including fullscreen resolution and unchanged default launch.
2. `cargo test -p tracewake-tui shell::` — passed; 11 shell tests passed,
   including wait, continue, and selected-action parity against direct
   authorized operations.
3. `cargo test -p tracewake-tui intent::reducer` — passed; 7 reducer tests
   passed, including selected-action activation through the authorized path.
4. `cargo test -p tracewake-tui` — passed; 107 unit tests and all package
   integration/doc tests passed.
5. Source grep over `main.rs`, `launch.rs`, and `run.rs` confirmed fullscreen
   dispatch to `TerminalGuard`/`run_fullscreen_shell` and continued default
   dispatch to `run_command_loop`.
