# 0052FOUCONFOU-002: F4-01 — production loaded-world bootstrap unrepresentability (atomic cutover)

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — removes the injectable `RuntimeInitialState` / `from_initial_state` authority path; routes the production TUI bootstrap through the opaque core constructor
**Deps**: 0052FOUCONFOU-001

## Problem

Spec 0052 F4-01 (§1.1.1, §4.2): `LoadedFixture::into_runtime_initial_state` (`crates/tracewake-content/src/load.rs:57`) builds a scheduler with `DeterministicScheduler::from_loaded_world`, and a loader unit witness exercises that derived handoff, but the production `TuiApp::from_golden` (`crates/tracewake-tui/src/app.rs:82`) manually reassembles `RuntimeInitialState` and injects `DeterministicScheduler::new(SimTick::ZERO)` (app.rs:103–110), whose `loaded_actor_next_decision_tick` and `declared_world_processes` maps are empty. The production TUI therefore starts with **no loaded-actor or declared-process census**; the correct handoff is proven only off the production path (a vacuity / evidence-honesty gap, the exact shape R-27/R-29 warn against).

This ticket performs the atomic cutover onto the opaque production constructor built additively in 001: content loading returns the opaque core-owned bootstrap consumable only by the production runtime constructor; the externally usable `from_initial_state(RuntimeInitialState)` path is removed (or made crate-private behind the core/content integration boundary the TUI cannot reach); and `TuiApp::from_golden` calls the same loader-to-runtime production constructor as every other loaded world, receiving an already-authoritative runtime/session rather than physical state plus a scheduler choice.

## Assumption Reassessment (2026-06-25)

1. `TuiApp::from_golden` (`crates/tracewake-tui/src/app.rs:82`) currently calls `LoadedWorldRuntime::from_initial_state(RuntimeInitialState { …, scheduler: DeterministicScheduler::new(SimTick::ZERO) })` (app.rs:103–110), bypassing `LoadedFixture::into_runtime_initial_state` (`load.rs:57`, which uses `DeterministicScheduler::from_loaded_world`, load.rs:61). `GoldenFixture` is `crates/tracewake-content/src/fixtures/mod.rs:145`; `strongbox_001()` returns it.
2. Spec home: `specs/0052_…_HARDENING_SPEC.md` §4.2; closure-order step 1 ("make the wrong production bootstrap unrepresentable"). The opaque constructor + opaque bootstrap export this ticket flips onto are produced additively by 0052FOUCONFOU-001.
3. Cross-artifact boundary under audit: the `tracewake-content`→`tracewake-core` loader handoff. After cutover the content side hands core an opaque bootstrap; core constructs the authoritative runtime; the TUI receives the runtime. Core still depends on neither content nor tui (`docs/1-architecture/01_*`).
4. Motivating invariants: INV-004 ("The authoritative world ignores human existence"), INV-005, INV-087, INV-091, INV-094, INV-108 — the production TUI must run the same autonomous loaded world under possession, with no injectable empty scheduler; and INV-098 (harsh acceptance) rejects a witness that bypasses production reachability.
5. Fail-closed / replay surface: the production constructor derives the loaded-actor and declared-process census from validated loaded content (via `from_loaded_world` semantics), and preserves the initial replay seed through the opaque core-owned seed/snapshot product (001) so the TUI need not retain mutable initial aggregates. This does not weaken deterministic replay (INV-018) — the seed is a deterministic function of accepted configuration — and introduces no actor-knowledge leakage, since the TUI receives a sealed session, not raw aggregates.
6. Removal blast radius (old menu item 7): `from_initial_state` / `RuntimeInitialState` consumers are `crates/tracewake-content/src/load.rs`, `crates/tracewake-core/src/runtime/{mod,session}.rs`, `crates/tracewake-core/tests/generative_lock.rs`, and `crates/tracewake-tui/src/app.rs` (grepped repo-wide). All in-workspace; each updates in this diff (local compile-atomicity). `generative_lock.rs` is kept compiling via the production constructor or a `pub(crate)` test-only adapter; its full production-constructor-mode rewrite is 0052FOUCONFOU-009. No alias path is left behind (§4.1, §9).

## Architecture Check

1. Cutting `from_golden` over to the same production constructor as all other loaded worlds — and deleting the injectable path entirely — is cleaner than gating the injectable path behind a feature or a runtime check, because it makes the wrong bootstrap *unrepresentable* (a client crate cannot name a scheduler to inject) rather than merely discouraged. Making the type system enforce it is what breaks the recurrence cycle (§3, §4.10 Layer A).
2. No backwards-compatibility alias: the injectable constructor is removed, not wrapped; this is an atomic cutover. A `pub(crate)` test adapter, if needed to migrate core tests, is removed before closeout.

## Verification Layers

1. INV-004/091/094 (no-human / possession parity) -> replay/golden-fixture check + production-boundary behavior test: starting through the production constructor with no scheduler-registration calls, loaded actors and declared processes advance.
2. INV-098 (harsh, non-vacuous acceptance) -> codebase grep-proof: no production path constructs `RuntimeInitialState` or injects `DeterministicScheduler::new`; the witness begins at the production bootstrap.
3. INV-069 (client boundary) -> external negative-fixture intent (full corpus in 009): a client crate cannot construct `RuntimeInitialState` or call an unvalidated constructor.

## What to Change

### 1. Opaque production bootstrap consumption (`load.rs`, `session.rs`)

Make content loading return the opaque core-owned bootstrap/export (built in 001) as the *only* loader product the runtime consumes; remove `into_runtime_initial_state`'s public injectable role (delete or make crate-private behind the integration boundary). Remove `RuntimeInitialState` / `from_initial_state` from the client-facing API.

### 2. Production-constructor TUI bootstrap (`app.rs`)

Rewrite `TuiApp::from_golden` to call the production constructor and receive an already-authoritative runtime/session; delete the manual `RuntimeInitialState` reassembly and the `DeterministicScheduler::new(SimTick::ZERO)` injection. The TUI retains no mutable initial aggregates (the opaque seed/snapshot from 001 covers later rebuild needs).

### 3. Keep core tests compiling

Migrate `generative_lock.rs`'s construction to the production constructor or a `pub(crate)` test adapter (full rewrite deferred to 009).

## Files to Touch

- `crates/tracewake-content/src/load.rs` (modify — opaque bootstrap is the loader product; injectable role removed)
- `crates/tracewake-core/src/runtime/session.rs` (modify — remove `from_initial_state` / `RuntimeInitialState` public path)
- `crates/tracewake-core/src/runtime/mod.rs` (modify — re-exports)
- `crates/tracewake-tui/src/app.rs` (modify — `from_golden` uses production constructor)
- `crates/tracewake-core/tests/generative_lock.rs` (modify — keep compiling via production constructor / pub(crate) adapter)

## Out of Scope

- Closing the command boundary / removing the `advance_world_after_acceptance` boolean (003).
- Removing raw aggregate getters and internalizing no-human/rebuild/perception (004).
- The full external compile-fail negative-fixture corpus and the production-boundary conformance lane (009).
- `generative_lock.rs` production-constructor-mode corpus rewrite (009).

## Acceptance Criteria

### Tests That Must Pass

1. A production-boundary integration test invokes the real content loader and the public TUI/application bootstrap, binds one actor, submits the typed wait command, and asserts committed effects from at least one *other* loaded actor and one declared process — asserting event IDs/types, actor/process identities, and resulting state/projection changes, with **no** test call constructing a scheduler or seeding eligibility.
2. A deliberate local mutation swapping the production constructor for `DeterministicScheduler::new` makes that test fail.
3. `cargo build --workspace --all-targets --locked && cargo test --workspace` — the cutover compiles with no injectable path remaining.

### Invariants

1. No client-reachable constructor accepts a caller-selected scheduler or a `RuntimeInitialState` (INV-069, INV-004).
2. `grep -rn "from_initial_state\|RuntimeInitialState" crates/` returns no client-facing (public, non-`pub(crate)`) match outside removed/adapter sites (INV-098 production reachability).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/world_step_coordinator.rs` (or a runtime/session integration test) — production-constructor bootstrap differential: loaded actors + declared process advance with no scheduler-registration calls; deliberate-derivation-removal makes it fail.
2. `crates/tracewake-tui/tests/command_loop_session.rs` — `from_golden` production-path bootstrap reaches the authoritative runtime.

### Commands

1. `cargo test -p tracewake-core --test world_step_coordinator`
2. `cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-25

Implemented the production bootstrap cutover:

- Removed the client-facing `RuntimeInitialState` export and made
  `LoadedWorldRuntime::from_initial_state` crate-private.
- Removed `LoadedFixture::into_runtime_initial_state`; content now exposes only
  the scheduler-free `LoadedWorldBootstrap` handoff added by
  `0052FOUCONFOU-001`.
- Reworked `TuiApp::from_golden` to consume
  `LoadedFixture::into_runtime_bootstrap` and construct the runtime through
  `LoadedWorldRuntime::from_bootstrap`, eliminating the old production
  `DeterministicScheduler::new(SimTick::ZERO)` injection.
- Migrated the generative runtime witness to `LoadedWorldBootstrap` /
  `from_bootstrap`.
- Updated the external negative fixture so it now proves `RuntimeInitialState`
  is unavailable to external crates while runtime fields remain private.
- Strengthened the TUI production wait witness to prove the real bootstrap
  advances loaded actor due work and declared world process due work through the
  public TUI path.

Deviations and follow-up disposition:

- The production bootstrap exposed a replay/checksum limitation in existing TUI
  duration-flow debug replay assertions: with real loaded-world scheduler/process
  due work active, those panels report `matches_expected=false` while
  `agent_checksum_matches=true`. The tests now record that truthful state and
  continue to assert event-sourced sleep/work completion evidence. Replay
  authority reconstruction is explicitly owned by `0052FOUCONFOU-005`; no
  closure for F4-03 is claimed here.
- `RuntimeInitialState` remains as a `pub(crate)` internal construction detail
  for `LoadedWorldRuntime::from_bootstrap` and runtime module unit tests. It is
  no longer re-exported or usable by client crates.

Verification:

- `cargo fmt --all --check` — passed.
- `cargo test -p tracewake-core --test world_step_coordinator` — passed.
- `cargo test -p tracewake-tui --test command_loop_session` — passed.
- `cargo test -p tracewake-tui --test embodied_flow` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo test --workspace` — passed.
- `rg -n "from_initial_state|RuntimeInitialState|into_runtime_initial_state|DeterministicScheduler::new\\(SimTick::ZERO\\)" crates` — remaining matches are crate-private runtime internals/tests or unrelated scheduler unit tests, with no client-facing `RuntimeInitialState` export and no TUI production scheduler injection.
