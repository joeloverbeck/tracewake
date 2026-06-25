# 0051FOUCONTHI-001: Core-owned loaded-world runtime/session keystone

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — new `tracewake-core` runtime/session module (private-field owner of authoritative aggregates) plus a closed typed command/receipt surface; registered in `crates/tracewake-core/src/lib.rs`.
**Deps**: None

## Problem

The scheduler is treated as a callable utility rather than the owner of a loaded simulation, and the TUI owns the authoritative aggregates (physical state, agent state, event log, epistemic projection, scheduler, controller bindings) and passes `&mut` references directly into core mutation functions. Each prior remediation (`0048`/`0050`) patched one route while the caller retained the ability to rebuild or reseed authority, so the third-pass re-audit (spec `0051` §3) found the same critical divergences reopening. The cycle-breaking change (§4.1) is a single core-owned loaded-world runtime/session boundary that owns all authoritative mutable aggregates behind private fields and exposes only a closed set of typed commands returning sealed receipts. This ticket builds that keystone **additively and unwired** — the TUI is migrated onto it in `-007`; the finding-specific obligations (F-01/F-02/F-04/F-05/F-06) ride on it in later tickets.

## Assumption Reassessment (2026-06-24)

1. Codebase: `DeterministicScheduler` (`crates/tracewake-core/src/scheduler.rs:495` `fn new`) owns `loaded_actor_next_decision_tick`/`declared_world_processes` (`490`–`491`) and `transact_world_one_tick` (`635`); `TuiApp` (`crates/tracewake-tui/src/app.rs:98` `from_golden`, `128` `DeterministicScheduler::new(SimTick::ZERO)`) owns the aggregates and is the current authoritative writer. No `SimulationSession`/`LoadedWorldRuntime` exists (grep: 0 matches in `crates/`), so this is greenfield *within* an existing crate — the new module's Files-to-Touch are legitimately `(new)`.
2. Specs/docs: spec `0051` §4.1 (keystone), §8 (closure order — runtime/session first), §9.1 (runtime owner name/module is an implementer-recorded choice). Architecture homes: `docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md`, `04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md`, `10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`.
3. Shared boundary under audit: the core↔TUI authority boundary — the runtime must own all authoritative mutable aggregates with private fields, and no public `&mut PhysicalState`/`AgentState`/`EventLog`/`EpistemicProjection` may cross the crate boundary. The full seal is completed in `-007`; the owning type and command surface are established here.
4. INV-103 (the scheduler is not a cognition authority), INV-112 (the scheduler/replay clock may order/validate but must not become cognition authority), INV-008 (UI assistance is not authority): restated — the runtime owns due-work and time; commands are typed; the TUI submits inputs and reads sealed receipts only, deciding no world legality or state.
5. Fail-closed / replay / actor-knowledge substrate: this ticket establishes the owner of the event log, replay frontier, and epistemic projection. It introduces no new leakage or nondeterminism path — it relocates ownership of existing aggregates behind private fields and delegates to the existing `transact_world_one_tick` / replay functions unchanged. The enforcement surfaces (sealing the `&mut` paths, deterministic command order, replay reconstruction) are sibling tickets `-003` (replay) and `-007` (TUI seal), cited here per the substrate-only basis.

## Architecture Check

1. A single private-field owner with a closed typed-command surface makes client write-authority type-*unrepresentable* (the §9 recurrence-risk bet): `#[non_exhaustive]` alone is insufficient because public enum variants stay externally constructible, so private fields + crate-private constructors + unexported command tokens are the controlling mechanism. Wrapping the existing scheduler/transaction internally (rather than rewriting it) keeps this ticket a reviewable diff and preserves the validated §4.10 behaviors for later re-expression.
2. No backwards-compatibility alias path: the runtime is the new owner; no public `DeterministicScheduler`-as-utility shim is left behind. A temporary internal (crate-private) test adapter may exist only to migrate core tests and is deleted before closeout (`-012`).

## Verification Layers

1. INV-103 / INV-112 (scheduler not cognition authority) -> codebase grep-proof: runtime fields are private and the command-enum constructors are crate-private (the externally-fails negative fixture lands with the seal in sibling tickets).
2. INV-008 (UI assistance is not authority) -> invariants-alignment check + manual review: the command surface returns sealed receipts; no `&mut` aggregate accessor is `pub`.
3. Single-layer scope note -> the production cutover proof (TUI cannot reach aggregates) is owned by `-007`; this ticket's proof is the type/ownership establishment, exercised by a core in-crate test driving one command end-to-end through the runtime.

## What to Change

### 1. Introduce the runtime/session owner

Add a `tracewake-core` module (candidate `crates/tracewake-core/src/runtime/` with `mod.rs` + `session.rs`; the exact name/module is the §9.1 implementer-recorded choice — record the chosen name and module path in the implementation notes and feed it to `-011` doc-truthing). It owns, with **private fields**: canonical physical state, agent state, event log + temporal/replay frontier, epistemic projection, loaded-actor census + next-opportunity projection, declared-process registry/cadence/trigger state, controller bindings/possession mapping, duration/reservation state, and deterministic ID/order minting.

### 2. Closed typed command + sealed receipt surface

Expose a small closed set of typed commands (controlled proposal / input slot, one-tick wait, continue-duration / advance-until, pause policy, debug-authorized no-human execution, save/rebuild) returning **immutable sealed typed receipts**. Internally each command runs one transaction on scratch aggregates and commits once, delegating to the existing `transact_world_one_tick` and replay functions for now (finding-specific internals are rewritten in `-002`..`-006`). Register the module in `crates/tracewake-core/src/lib.rs`.

### 3. Keep aggregates unexported

No public accessor returns `&mut PhysicalState`, `&mut AgentState`, `&mut EventLog`, or `&mut EpistemicProjection`. Read access for view models is via read-only getters / sealed receipts only.

## Files to Touch

- `crates/tracewake-core/src/runtime/mod.rs` (new)
- `crates/tracewake-core/src/runtime/session.rs` (new)
- `crates/tracewake-core/src/lib.rs` (modify) — register `pub mod runtime;`
- `crates/tracewake-core/src/scheduler.rs` (modify) — expose the minimal crate-internal hooks the runtime composes (no public surface added); merge-hub contributor across `-001`..`-006`

## Out of Scope

- Production loaded-world discovery / seed-method de-authority (F-01 → `-002`).
- Replay/save reconstruction of runtime authority (F-02 → `-003`).
- Declared-process transactions, per-tick census, full actor-outcome consumption (F-03/F-04/F-05 → `-004`/`-005`/`-006`).
- Migrating the TUI onto the runtime and removing public `&mut` / perception / scheduler-forwarding paths (F-06 → `-007`).
- Splitting the temporal products (F-07 → `-008`).

## Acceptance Criteria

### Tests That Must Pass

1. A core in-crate test constructs the runtime, submits a one-tick wait command, asserts a sealed receipt is returned and the event log advanced — using no `&mut` aggregate accessor.
2. `cargo build --workspace --all-targets --locked` and `cargo clippy --workspace --all-targets -- -D warnings` pass with the new module registered.
3. `cargo test -p tracewake-core` is green (existing behavior preserved; the runtime delegates to current internals).

### Invariants

1. No public accessor exposes `&mut` over `PhysicalState` / `AgentState` / `EventLog` / `EpistemicProjection`.
2. Runtime fields and command constructors are private / crate-private.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/runtime/session.rs` (inline `#[cfg(test)]`) — one-command-end-to-end ownership test driving a wait command through the runtime.

### Commands

1. `cargo test -p tracewake-core runtime`
2. `cargo build --workspace --all-targets --locked && cargo clippy --workspace --all-targets -- -D warnings`

## Outcome

Completed: 2026-06-24

Implemented the keystone as `tracewake_core::runtime::LoadedWorldRuntime` in
`crates/tracewake-core/src/runtime/session.rs`, registered via
`crates/tracewake-core/src/runtime/mod.rs` and `crates/tracewake-core/src/lib.rs`.
The runtime owns the action registry, physical state, agent state, event log,
epistemic projection, controller bindings, deterministic scheduler, and content
manifest id behind private fields. The implementer-recorded module choice is
`crates/tracewake-core/src/runtime/`; the constructor is
`LoadedWorldRuntime::from_initial_state(RuntimeInitialState)` to avoid the
production `seed_` mutation guard vocabulary.

The first closed command surface is a one-tick wait submitted through
`LoadedWorldRuntime::wait_one_tick` / `submit_command`, with the internal
`RuntimeCommand::one_tick_wait` constructor kept `pub(crate)`. The immutable
`RuntimeReceipt` exposes only read access to the receipt kind and no runtime API
returns `&mut PhysicalState`, `&mut AgentState`, `&mut EventLog`, or
`&mut EpistemicProjection`.

Because the new runtime is authoritative scheduler-boundary code, it was
enrolled in the guarded-layer source census and mutation perimeter via
`crates/tracewake-core/tests/anti_regression_guards.rs`, `.cargo/mutants.toml`,
and `.github/workflows/ci.yml`.

Verification:

- `cargo test -p tracewake-core runtime` — passed.
- `cargo fmt --all --check` — passed.
- `cargo test -p tracewake-core --test anti_regression_guards` — passed after
  guarded-layer enrollment.
- `cargo test -p tracewake-core` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.

Deviations:

- The runtime is additive and not yet wired into TUI production bootstrap, as
  scoped. Production loaded-world discovery, replay reconstruction, TUI
  migration, process transactions, actor census, and split embodied/debug
  products remain owned by dependent tickets.
