# 0054FOUCONSIX-003: Non-inducible debug-session authority (atomic cutover)

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` runtime-command/session + debug-capability authority surface (debug entry requires an independently-held operator authority; `debug_session_authority_for` no longer mints for a self-bound controller); `tracewake-tui` debug-entry migration; new bypass-shaped external negative fixture
**Deps**: 0054FOUCONSIX-002

## Problem

`DebugSessionAuthority` is unforgeable as a value but **inducible** through the unprivileged public command surface it is meant to guard. `crates/tracewake-core/src/debug_capability.rs` correctly keeps `DebugCapability`/`DebugSessionAuthority` fields private and `mint()` `pub(crate)` (`:36`/`:59`). But `RuntimeCommand::bind_debug_controller(controller_id, actor_id)` (`crates/tracewake-core/src/runtime/command.rs:99`) is `pub fn`, requires no prior authority, and sets `mode: ControllerMode::Debug`; after `submit_command` binds the controller, `LoadedWorldRuntime::debug_session_authority_for` (`crates/tracewake-core/src/runtime/session.rs:234`) returns a runtime-minted `DebugSessionAuthority` whenever `controller_debug_available_for` (`session.rs:221`) sees a Debug-mode binding — i.e. `.then(DebugSessionAuthority::mint)` (`session.rs:240`). `TuiApp::bind_debug_actor` (`crates/tracewake-tui/src/app.rs:102`) and `debug_authority` (`app.rs:159`) delegate to the same path (`app.rs:119`). The 0053 negative fixture `external_crate_cannot_submit_debug_command_without_token` proves only that a caller cannot call `DebugSessionAuthority::mint()` directly; it does **not** prove a caller cannot first submit a public debug bind and then obtain the runtime-minted authority (finding F6-03). This re-opens the F5-03 / F4-02 / F4-07 authority class.

## Assumption Reassessment (2026-06-27)

1. `crates/tracewake-core/src/runtime/command.rs:99` `pub fn bind_debug_controller(...)` requires no prior authority and sets `ControllerMode::Debug`; `run_no_human_day` (`:115`) and `debug_view` (`:135`) already require a `DebugSessionAuthority` argument. `session.rs:234` `debug_session_authority_for` mints via `controller_debug_available_for(...).then(DebugSessionAuthority::mint)`. Confirmed at `7660051`.
2. `crates/tracewake-tui/src/app.rs:102` `bind_debug_actor` submits `RuntimeCommand::bind_debug_controller` (`app.rs:119`); `app.rs:159` `fn debug_authority` calls `debug_session_authority_for` (`app.rs:165`). The sole production caller of `bind_debug_controller` is `app.rs:119` (blast radius confirmed by grep). Confirmed.
3. Shared boundary under audit: the public `RuntimeCommand`/runtime-session command boundary at the `tracewake-core` ↔ external-client seam. The protected claim is the public runtime boundary — a fixture that only asserts the TUI checks debug mode is vacuous.
4. INV-004/INV-005/INV-006 (the authoritative world ignores humans; only normal binding is possession; possession transfers no knowledge), INV-068 (debug mode is visibly non-diegetic), INV-094/INV-107/INV-108 (possession parity tested; debug omniscience quarantined; possession is cognition-neutral). Restated: entering debug mode must require an independently-held operator/debug authority, not a self-bind through an ordinary public command.
5. No-human / authority surface: this ticket touches the debug-capability authority boundary. Confirm that after the fix `debug_session_authority_for` does **not** mint for a controller that entered debug mode through an unprivileged public command, that debug data (`run_no_human_day`, `debug_view`) is reachable only through the real authority path, and that ordinary embodied play cannot reach a debug capability (no leakage of debug-grade truth into the embodied surface).
6. Authority-constructor reseal (additive-vs-breaking = **breaking**, intentional): `bind_debug_controller` / the debug-entry path changes from an unprivileged public command to an operator-gated entrypoint, and `debug_session_authority_for` stops minting on a self-bind. Consumer: `crates/tracewake-tui/src/app.rs:119`/`:159-165` migrates to supply the independently-held operator/debug authority. In-workspace → local compile-atomicity. The implementer-recorded debug-attach model (§10.2 options a/b/c) is recorded in Architecture Check.
7. Removal blast radius of the old public bind path: grep across `crates/`, `tests/`, `.claude/skills/`, `docs/`, `specs/` confirms `bind_debug_controller` is consumed only at `tui/app.rs:119`; the 0053 fixture `external_crate_cannot_submit_debug_command_without_token` (`negative_fixture_runner.rs:196`) stays as a direct-mint guard and a **new** bypass-shaped fixture is added for the induction route. No live doc/sibling-spec deliverable names the public self-bind as committed (only archived 0053).

## Architecture Check

1. Requiring an independently-held operator/debug authority to enter debug mode makes the induction attack unrepresentable: a caller who only holds ordinary public commands cannot reach a `DebugSessionAuthority`, so the token's value-unforgeability is no longer the *only* guard. This closes the "non-forgeable token beside a public bind that induces it" pattern the fourth/fifth passes left open.
2. **Debug-attach model decision (implementer-recorded, §10.2):** implemented the (b)/(c) hybrid: `RuntimeCommand::bind_debug_controller` now requires an already-held `DebugSessionAuthority`, `LoadedWorldRuntime::debug_session_authority_for` validates a supplied authority and never mints from controller binding state, and the local TUI obtains its authority through the explicit `LoadedWorldRuntime::local_operator_debug_authority()` operator entrypoint before debug binding. The ordinary embodied command surface has no compatibility alias for the old two-argument bind.

## Verification Layers

1. Non-inducibility (INV-004/INV-068) → bypass-shaped external negative fixture performing the **actual attack**: build a runtime from a golden fixture, call `RuntimeCommand::bind_debug_controller`, submit it, call `debug_session_authority_for`, then attempt `run_no_human_day(authority)` / `debug_view(authority)` — must fail for lack of operator authority (not because setup is stale).
2. Authority path reachable (INV-068) → positive behavioral test for the approved operator/debug entrypoint proving debug data is reachable only through the real authority path.
3. No embodied leakage (INV-006/INV-107) → assertion that ordinary embodied play obtains no debug capability and possession changes input only.

## What to Change

### 1. Gate debug entry on an independently-held operator authority

Per the recorded §10.2 model, make `DebugSessionAuthority` impossible to obtain through the ordinary public command surface; `debug_session_authority_for` must not mint a token for a controller that entered debug mode through an unprivileged public command.

### 2. Migrate the TUI debug entry

Update `TuiApp::bind_debug_actor` / `debug_authority` (`app.rs:102`/`:119`/`:159`) and `crates/tracewake-tui/src/run.rs`, `input.rs` to obtain debug authority through the approved operator entrypoint (deliberate operator action), not a self-bind.

### 3. Add the bypass-shaped negative fixture

New `tests/negative-fixtures/<name>/` (e.g. `external_crate_cannot_induce_debug_authority_via_public_bind`) registered in `negative_fixture_runner.rs` (FIXTURES + `ALL_FEATURE_PRODUCTION_BOUNDARY_FIXTURES`), performing the full induction attack and failing for lack of operator authority, under default and all supported feature combinations.

## Files to Touch

- `crates/tracewake-core/src/runtime/command.rs` (modify)
- `crates/tracewake-core/src/runtime/session.rs` (modify)
- `crates/tracewake-core/src/debug_capability.rs` (modify)
- `crates/tracewake-tui/src/app.rs` (modify)
- `crates/tracewake-tui/src/run.rs` (modify)
- `crates/tracewake-tui/src/input.rs` (modify)
- `crates/tracewake-core/tests/negative_fixture_runner.rs` (modify)
- `tests/negative-fixtures/external_crate_cannot_induce_debug_authority_via_public_bind/` (new — `Cargo.toml` + `src/lib.rs`)

## Out of Scope

- The wait-receipt seal (ticket 002) — this ticket hardens the authority that gates the debug receipt; the receipt shape lands in 002.
- Bootstrap reseal (ticket 001) and acceptance/governance/mutation-CI changes (tickets 004–006).
- Live-conformance doc-truthing for the debug-authority row (ticket 010).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --test negative_fixture_runner` — the new bypass-shaped induction fixture fails (for lack of operator authority) and the existing `external_crate_cannot_submit_debug_command_without_token` direct-mint guard still fails, both under default and all supported feature combinations.
2. A positive behavioral test: the approved operator/debug entrypoint reaches `debug_view`/`run_no_human_day` data through the real authority path.
3. A possession-parity / no-leak test: ordinary embodied play obtains no debug capability.
4. `cargo build --workspace --all-targets --locked && cargo test --workspace`.

### Invariants

1. No sequence of ordinary public commands yields a `DebugSessionAuthority`; debug entry requires an independently-held operator authority.
2. Possession/embodied play remains cognition-neutral — no debug-grade truth leaks into the embodied surface (INV-006/INV-107).

## Test Plan

### New/Modified Tests

1. `tests/negative-fixtures/external_crate_cannot_induce_debug_authority_via_public_bind/src/lib.rs` — bypass-shaped induction compile/authority-fail.
2. `crates/tracewake-core/tests/negative_fixture_runner.rs` — register the new fixture + `expected_stderr`.
3. A positive operator-entry behavioral test (extend the runtime/TUI debug test surface) — proves debug data reachable only through the real authority.

### Commands

1. `cargo test -p tracewake-core --test negative_fixture_runner`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
3. `cargo mutants -f crates/tracewake-core/src/runtime/command.rs -f crates/tracewake-core/src/runtime/session.rs -f crates/tracewake-core/src/debug_capability.rs` — focused campaign over the debug-authority path (standing campaign is ticket 009).

## Outcome

Completed: 2026-06-27

Implemented the atomic debug-authority cutover. `RuntimeCommand::bind_debug_controller` now requires a pre-existing `DebugSessionAuthority`, the runtime session no longer mints a debug-session authority from a controller's debug binding state, and the TUI keeps debug entry behind a local operator entrypoint instead of inducing authority through an ordinary public command. A failed ordinary rebind no longer clears an existing debug session before actor validation, preserving the command-loop unknown-actor behavior without weakening the debug availability gate.

Added `external_crate_cannot_induce_debug_authority_via_public_bind` as a bypass-shaped negative fixture. The fixture uses the old public two-argument debug bind shape and fails to compile with `this function takes 3 arguments`, making the former induction attack unrepresentable for external crates. The existing direct-mint guard remains covered by `negative_fixture_runner`.

Verification run:

- `cargo test -p tracewake-core --test negative_fixture_runner` — passed.
- `cargo test -p tracewake-core debug_session_authority_requires_supplied_operator_authority` — passed.
- `cargo test -p tracewake-tui app_runs_no_human_day_into_real_log_metrics` — passed.
- `cargo test -p tracewake-tui run_no_human_day_refuses_intrinsically_without_debug_availability` — passed.
- `cargo test -p tracewake-tui controller_mode_debug_availability_decision_is_explicit` — passed.
- `cargo test -p tracewake-tui unknown_actor_arguments_are_reported_without_crashing` — passed after the failed-bind state-preservation fix.
- `cargo fmt --all --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo test --workspace` — passed.
- `git diff --check` — passed.

Mutation evidence:

- `cargo mutants --list -f crates/tracewake-core/src/runtime/command.rs -f crates/tracewake-core/src/runtime/session.rs -f crates/tracewake-core/src/debug_capability.rs --no-config` listed 82 focused mutants in the intended files.
- The required command `cargo mutants -f crates/tracewake-core/src/runtime/command.rs -f crates/tracewake-core/src/runtime/session.rs -f crates/tracewake-core/src/debug_capability.rs` selected 3445 mutants under repository config and was interrupted after the selection line. Result recorded as incomplete; no mutation pass is claimed here. Standing mutation completion remains ticket 009 scope.

Unrelated pre-existing dirty paths left untouched: `.claude/skills/spec-to-tickets/SKILL.md` and `.claude/skills/spec-to-tickets/references/decomposition-patterns.md`.
