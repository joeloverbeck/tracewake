# 0002TUIPROOSUR-008: Command loop submits only current-view semantic actions

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-tui` (command loop: `wait`/`do`/numeric resolution)
**Deps**: 0002TUIPROOSUR-002, 0002TUIPROOSUR-006

## Problem

Numeric selection resolves through the current view's `semantic_actions` (`crates/tracewake-tui/src/run.rs:54-58`), but the `wait` shortcut hard-codes `const WAIT_ACTION_ID: &str = "wait.1_tick"` and constructs that `SemanticActionId` directly (`crates/tracewake-tui/src/run.rs:10-12,60-63`) instead of resolving it from the current view. This weakens the rule that the TUI submits only currently-surfaced affordances. Spec 0002 §4 TUI-AC-005 requires every embodied world-affecting command — including `wait`/`w` — to resolve through the current view's typed `semantic_actions` (or a typed actor-safe alias mapping to a visible current-view action token), under the same sealed-context/proposal path as numeric and `do <id>`.

## Assumption Reassessment (2026-06-08)

1. `WAIT_ACTION_ID = "wait.1_tick"` is at `crates/tracewake-tui/src/run.rs:10-12`; `wait` builds the id directly (`:60-63`). Numeric selection resolves through the current view (`:54-58`). `do <id>` goes through `TuiApp::submit_semantic_action`, which looks the id up in `current_view().semantic_actions` (`crates/tracewake-tui/src/app.rs:183-195`). The command parser is `crates/tracewake-tui/src/input.rs`.
2. Spec 0002 §4 TUI-AC-005: remove the hard-coded `WAIT_ACTION_ID` (or make it a lookup key that must find exactly one current-view entry); `do <id>` legal only if the id exists in the current view and matches the context hash; numeric selection is presentation only.
3. Cross-artifact boundary under audit: the TUI command loop (`run.rs`/`input.rs`/`app.rs`) ↔ the sealed-context `current_view().semantic_actions` (ticket 002) ↔ the source-context proposal path (ticket 006). After this ticket all three command forms share one resolution path.
4. Invariants restated: **INV-069** — the TUI submits typed action attempts from surfaced affordances and must not implement simulation rules or bypass validators; **INV-008** — UI assistance is not authority; **INV-070** — actor-known why-not when a command resolves to no current affordance.
5. Fail-closed / leak surface: removing the bypass ensures every embodied command flows through the same sealed-context/proposal path (006), so the freshness/membership/forgery checks apply uniformly. Confirm no command path constructs a `SemanticActionId` outside the current view; a `wait` with no surfaced wait action must fail (resolve to nothing), not synthesize one.
6. Rename/removal blast radius: `WAIT_ACTION_ID` is removed. Pre-implementation grep `WAIT_ACTION_ID` and `"wait.1_tick"` across `crates/`, `docs/`, `specs/`: known site is `run.rs:10-12,60-63`; any match joins Files to Touch (the wait action definition `crates/tracewake-core/src/actions/defs/wait.rs` keeps its own id — only the TUI's hard-coded shortcut is removed).

## Architecture Check

1. Routing `wait`/`do`/numeric through one current-view resolution path makes "the TUI submits only surfaced affordances" a structural property: a command that does not map to a visible current-view token cannot execute, so a stale/hidden/privileged shortcut is impossible by construction (INV-069). It also removes a divergent code path that the source-context checks (006) would otherwise not cover.
2. No backwards-compatibility aliasing/shims: the hard-coded shortcut is deleted, not kept as a fallback; `wait` becomes a typed alias that resolves through the same lookup as `do`.

## Verification Layers

1. INV-069 (only surfaced affordances) -> codebase grep-proof: `WAIT_ACTION_ID`/`"wait.1_tick"` no longer constructed in `tracewake-tui`; `wait` resolves via `current_view().semantic_actions`.
2. INV-008 (UI not authority) -> behavior test: with no wait action surfaced, `wait` cannot execute; numeric/`do`/`wait` all submit via the same proposal/context path.
3. INV-070 (why-not) -> manual review: an unresolved command yields an actor-safe "no such current action" response, not a synthesized id.

## What to Change

### 1. Remove the hard-coded wait shortcut

Delete `WAIT_ACTION_ID`; resolve `wait`/`w` as a typed actor-safe alias that looks up exactly one matching action token in `current_view().semantic_actions`.

### 2. Uniform resolution

Ensure `do <id>` checks current-view membership + context hash, numeric stays presentation-only, and all three forms reach `submit_entry` via the same sealed-context/proposal path (ticket 006).

## Files to Touch

- `crates/tracewake-tui/src/run.rs` (modify)
- `crates/tracewake-tui/src/input.rs` (modify)
- `crates/tracewake-tui/src/app.rs` (modify)

## Out of Scope

- No-human operator-command classification (ticket 009).
- The action definition `actions/defs/wait.rs` semantics (unchanged — only the TUI shortcut is removed).
- Proposal source-context validation internals (ticket 006).

## Acceptance Criteria

### Tests That Must Pass

1. Remove/disable `wait` from a fixture's current view and prove `wait` cannot execute; restore a view-surfaced wait action and prove `wait` executes through the same proposal/context path as numeric and `do`.
2. A grep proves no `SemanticActionId` for wait is constructed from a hard-coded constant in `tracewake-tui`.
3. `cargo test -p tracewake-tui` passes; `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` pass.

### Invariants

1. Every embodied command resolves to a surfaced current-view action token (INV-069).
2. No TUI path synthesizes an action id outside the current view (INV-008).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/command_loop_session.rs` — `wait`/`do`/numeric parity through one resolution path; unsurfaced-`wait` fails.
2. `crates/tracewake-tui/tests/tui_acceptance.rs` — update any assertion tied to the removed shortcut.

### Commands

1. `cargo test -p tracewake-tui command_loop_session`
2. `grep -rn "WAIT_ACTION_ID\|\"wait.1_tick\"" crates/tracewake-tui/` (must return zero after implementation)
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`

## Outcome

Completed: 2026-06-08

Changed:
- Removed the TUI command-loop hard-coded wait semantic action constant.
- Routed `wait`/`w` through a current-view alias lookup that requires exactly one surfaced `action_id == "wait"` semantic action.
- Kept numeric selection presentation-only and `do <id>` membership-checked through `TuiApp::submit_semantic_action`.
- Made missing current-view semantic actions fail closed in the command loop with `Error: no such current action: ...` instead of synthesizing an ID or bubbling out as an I/O failure.
- Updated transcript and TUI acceptance paths to submit the current view's wait token instead of constructing the wait semantic ID directly.
- Added command-loop coverage proving `wait` executes the currently surfaced wait action and unit coverage proving the wait alias returns `None` when the current view lacks wait.

Deviations:
- `crates/tracewake-tui/src/app.rs` did not need changes; it already looked up `do <id>` submissions in `current_view().semantic_actions` before building a proposal.
- The unsurfaced-wait proof is a direct current-view resolver unit test rather than a fixture mutation in the binary loop, because fixture affordances are generated from registered actions.

Verification:
- `rg -n "WAIT_ACTION_ID|\"wait\\.1_tick\"" crates/tracewake-tui` returned no matches.
- `cargo test -p tracewake-tui wait`
- `cargo test -p tracewake-tui command_loop_session`
- `cargo test -p tracewake-tui`
- `cargo fmt --all --check`
- `cargo build --workspace --all-targets --locked`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
