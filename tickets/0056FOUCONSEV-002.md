# 0056FOUCONSEV-002: Operator-gated, non-embodied debug authority

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` debug-authority API reseal, `tracewake-tui` launch-mode split and command-loop gating, the debug-authority negative fixture, and ordinary-mode noninducibility tests
**Deps**: None

## Problem

Finding F7-02. The sixth-pass repair made `DebugCapability::mint()` / `DebugSessionAuthority::mint()` crate-private and token-gated `RuntimeCommand::bind_debug_controller` / `run_no_human_day` / `debug_view`. But debug-session authority is still inducible from ordinary embodied input. At `2720167`:

- `crates/tracewake-core/src/runtime/session.rs:244` `pub fn LoadedWorldRuntime::local_operator_debug_authority(&self) -> DebugSessionAuthority` mints a token with no externally supplied operator proof.
- `crates/tracewake-tui/src/app.rs:104-105` `TuiApp::bind_debug_actor(...)` calls it.
- `crates/tracewake-tui/src/input.rs:95-97` parses `bind-debug <actor_id>` into `UiCommand::BindDebugActor`, dispatched by the same command-loop parser as `wait`, `look`, and semantic actions.

Debug authority is therefore inducible from the ordinary embodied command surface rather than gated behind an explicit non-embodied operator entrypoint. This is the residual operator/TUI path, not a resurrection of the direct-mint bug.

## Assumption Reassessment (2026-06-28)

1. Live API verified at `2720167`: `debug_capability.rs:36/59` `DebugCapability::mint()` / `DebugSessionAuthority::mint()` are `pub(crate)` (preserve); `runtime/command.rs:104/124/144` `bind_debug_controller` / `run_no_human_day` / `debug_view` require a `DebugSessionAuthority` (preserve); `session.rs:244` `local_operator_debug_authority` is `pub` and mints without operator proof (the live hole); `app.rs:105` calls it from `bind_debug_actor`; `input.rs:95` parses `bind-debug ` → `BindDebugActor`. TUI launch surface: `launch.rs:29` `resolve(...) -> Launch` (a `debug_attach_001` golden already exists); `run.rs:13` `run_command_loop(...)`.
2. Spec `specs/0056_FOUNDATIONAL_CONFORMANCE_SEVENTH_HARDENING_*.md` §4.2 + §10.2 + §9 govern this ticket; the operator/debug launch UX is a spec-assigned implementer-recorded choice (item 3). Driver F7-02 in `reports/0047-foundational-hardening-research-report-seventh-pass.md`.
3. **Embodied/operator boundary under audit**: the split between the ordinary embodied TUI and the explicit operator/debug launch. The decision is the implementer's, recorded here (§10.2): local-TUI debug attach may remain convenient, but it must live outside ordinary embodied command input, carry an operator capability created at launch/session setup, and be visibly non-diegetic. The ordinary command parser must be incapable of producing debug authority.
4. INV-031 (human/debug notes are non-diegetic), INV-067 (embodied mode shows actor-known reality), INV-068 (debug mode is visibly non-diegetic), INV-093 (actor-knowledge leakage is high-severity), INV-107 (debug omniscience is quarantined); plus INV-004/005/006/094/108 (no-human authority, ordinary-possession-only binding, possession transfers no knowledge, possession parity, cognition-neutral possession) and INV-008/065/069 (UI is not authority; TUI is a primary product interface; TUI must not implement simulation rules) — debug authority self-bound through ordinary input breaches the embodied/debug quarantine and lets a possessed/ordinary surface mint omniscient authority.
5. **Actor-knowledge / debug-quarantine enforcement surface**: `runtime/session.rs` (authority mint), `runtime/command.rs` (token-gated debug commands — preserved), and the TUI command loop (`tui/src/{app,input,run,debug_panels}.rs`). Removing the ordinary-input mint path strengthens the no-leak firewall: debug truth becomes reachable only through the explicit operator launch, so no embodied command can surface debug views. Deterministic replay is unaffected (debug authority is a viewer capability, not a world-state input). Confirm no debug panel renders in embodied mode after the split.
6. **Visibility/shape change (reseal — breaking, intra-workspace)**: `local_operator_debug_authority` is removed from the public ordinary runtime API (or gated behind an unforgeable operator-mode capability the parser cannot produce). Consumers: `session.rs:1026` (test) and `app.rs:105` (`bind_debug_actor`) — both migrate to the operator-launch path in this diff. The change is **breaking** within the workspace; `bind_debug_actor` and the `BindDebugActor`/`bind-debug` ordinary-parser arm are removed or rerouted.
7. **Removal blast radius** (grepped repo-wide): `local_operator_debug_authority` → 2 call sites (`session.rs:1026` test, `app.rs:105`), no `docs/`/`specs/`/`.claude/skills/` consumers. `bind-debug` / `BindDebugActor` → `input.rs` (parser + its unit test at `:249`) and `app.rs`. No sibling-spec or doc consumer. Removing the ordinary `bind-debug` command is a TUI surface change only.

## Architecture Check

1. Splitting launch modes — an ordinary embodied TUI with no `bind-debug`, no debug mint, no debug panels; and an explicit operator/debug launch carrying an operator capability created at launch — makes debug authority *unobtainable* from the embodied command surface by construction, rather than relying on a runtime check the parser could still route around. This is the compile-time/launch-time boundary the layered barrier requires, stronger than a guarded runtime method beside a public mint.
2. No backwards-compatibility aliasing or shims: `local_operator_debug_authority` and the ordinary `bind-debug` path are removed, not wrapped behind a deprecated alias. `bind_debug_controller` stays token-gated (preserved property). Core retains no dependency on tui.

## Verification Layers

1. INV-068/INV-107 (debug quarantine) -> public TUI command-loop behavioral test: an ordinary-mode script containing `bind-debug actor_tomas` then a `debug ...` command proves debug is **not** enabled and no debug view is produced (submitted through the public loop, not a source-text guard).
2. INV-031/INV-093 (non-diegetic, no leakage) -> positive operator-launch test: debug data is reachable only through the explicit operator entrypoint and is visibly non-diegetic.
3. INV-008/INV-069 (UI is not authority) -> external-crate negative fixture: any public runtime/TUI API capable of minting `DebugSessionAuthority` without an operator proof fails with a privacy/authority diagnostic (distinct proof surface from the behavioral tests).
4. Mutation sensitivity -> coverage over `debug_capability.rs`, `runtime/session.rs`, `runtime/command.rs`, and the TUI app/input/run/debug-panels/launch files (perimeter owned by 0056FOUCONSEV-005; survivors recorded by -006).

## What to Change

### 1. Remove the ordinary-input debug authority mint

Remove `LoadedWorldRuntime::local_operator_debug_authority()` from the public ordinary runtime API, or make it require an unforgeable operator-mode capability the command parser cannot produce. Keep `bind_debug_controller` token-gated.

### 2. Split TUI launch modes (record the chosen UX, §10.2)

Introduce an explicit operator/debug launch that carries an operator capability created at launch/session setup, visibly non-diegetic; the ordinary embodied launch has no `bind-debug` command, no debug authority mint, and no debug panels. Record the chosen launch UX and the rejected alternatives here.

### 3. Make the ordinary parser incapable of producing debug authority

Remove/reroute the `bind-debug` arm and `UiCommand::BindDebugActor` so typing a debug command in ordinary mode fails actor-safely and does not mutate controller bindings.

### 4. Repair the debug-authority negative fixture (F7-07 part)

Retarget `tests/negative-fixtures/external_crate_cannot_induce_debug_authority_via_public_bind/src/lib.rs` to attack the live `local_operator_debug_authority` / ordinary-mode debug path (currently it attacks `bind_debug_controller` directly without a token — a narrow, drifted route). Re-register in `negative_fixture_runner.rs`.

## Files to Touch

- `crates/tracewake-core/src/runtime/session.rs` (modify — coordinates with 0056FOUCONSEV-001)
- `crates/tracewake-core/src/runtime/command.rs` (modify — preserve token gating; adjust only as the operator capability requires)
- `crates/tracewake-core/src/debug_capability.rs` (modify — operator-capability type, if the chosen topology adds one)
- `crates/tracewake-tui/src/app.rs` (modify — remove `bind_debug_actor` ordinary path; coordinates with 0056FOUCONSEV-001)
- `crates/tracewake-tui/src/input.rs` (modify — remove `bind-debug`/`BindDebugActor`)
- `crates/tracewake-tui/src/run.rs` (modify — launch-mode routing)
- `crates/tracewake-tui/src/launch.rs` (modify — explicit operator/debug launch)
- `crates/tracewake-tui/src/debug_panels.rs` (modify — gate panels behind operator mode)
- `tests/negative-fixtures/external_crate_cannot_induce_debug_authority_via_public_bind/src/lib.rs` (modify)
- `crates/tracewake-core/tests/negative_fixture_runner.rs` (modify — re-register; coordinates with 0056FOUCONSEV-001)

## Out of Scope

- F7-01 bootstrap sealing (0056FOUCONSEV-001) — even though it shares `session.rs`/`app.rs`/`negative_fixture_runner.rs`; this ticket touches the debug-authority functions only.
- Re-sealing `DebugCapability::mint()` / `DebugSessionAuthority::mint()` (already `pub(crate)` — preserved) or un-gating `bind_debug_controller`.
- Acceptance taxonomy, doctrine synchronization, conformance truthing, and the mutation run.

## Acceptance Criteria

### Tests That Must Pass

1. A public TUI command-loop test: `bind-debug <actor>` + `debug ...` in ordinary mode enables nothing and produces no debug view; no controller binding mutates.
2. A positive operator-launch test: debug data is reachable only through the explicit operator entrypoint and is non-diegetic.
3. `cargo test --locked -p tracewake-core --test negative_fixture_runner` — the repaired debug fixture compile-fails on a privacy/authority diagnostic; census passes.

### Invariants

1. No ordinary embodied command-loop input can mint or obtain `DebugSessionAuthority`; debug entry requires an independently-held operator capability created at launch.
2. `bind_debug_controller` / `run_no_human_day` / `debug_view` remain token-gated; `DebugCapability`/`DebugSessionAuthority` mints remain crate-private.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/` (or the existing TUI command-loop test module) — ordinary-mode noninducibility script + positive operator-launch reachability test.
2. `tests/negative-fixtures/external_crate_cannot_induce_debug_authority_via_public_bind/src/lib.rs` — retargeted to the live no-operator-proof mint path.

### Commands

1. `cargo test --locked -p tracewake-tui` — command-loop noninducibility + operator-launch tests.
2. `cargo test --locked -p tracewake-core --test negative_fixture_runner`
3. `cargo build --workspace --all-targets --locked && cargo test --workspace`
