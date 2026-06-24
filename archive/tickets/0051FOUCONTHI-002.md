# 0051FOUCONTHI-002: F-01 production loaded-world discovery and seed-method de-authority

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — derive the initial actor census and declared-process registry from loaded content in the core runtime bootstrap; restrict `schedule_loaded_actor_decision` / `register_cadenced_world_process` from external reach.
**Deps**: 0051FOUCONTHI-001

## Problem

`DeterministicScheduler::new` initializes `loaded_actor_next_decision_tick` and `declared_world_processes` as empty maps (`scheduler.rs:499`–`500`); the only writers are the public `schedule_loaded_actor_decision` (`583`) and `register_cadenced_world_process` (`592`). The production TUI bootstrap (`TuiApp::from_golden` → `DeterministicScheduler::new(SimTick::ZERO)`, `app.rs:128`) never derives or registers actor/process eligibility from loaded world state, while the "production reachability" witness in `world_step_coordinator.rs` calls the seed methods itself before invoking the coordinator — so waiting advances a manufactured registry, not the loaded world (F-01, vacuity gap). The fix derives the census and process registry from authoritative loaded content during the same bootstrap the TUI uses, and removes the external seeding capability so no caller can reseed eligibility.

## Assumption Reassessment (2026-06-24)

1. Codebase: empty-map init at `crates/tracewake-core/src/scheduler.rs:499`–`500`; public seeders at `583`/`592`; their only consumers are `crates/tracewake-core/tests/world_step_coordinator.rs` (integration test, separate compilation unit — sees only `pub` items). Content loading exposes `LoadedFixture` (`crates/tracewake-content/src/load.rs:45`) via `load_fixture_package` (`54`). The `-001` runtime owns the census/registry fields this ticket populates.
2. Specs/docs: spec `0051` §4.2 (production loaded-world discovery), §9.1. INV homes `docs/1-architecture/04_*` (scheduler/world-step) and `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md`.
3. Shared boundary under audit: the loaded-content → runtime eligibility derivation, and the external seeding capability being removed — after this ticket, only the core bootstrap (or a core-crate test-only path) may populate eligibility.
4. INV-004 (the authoritative world ignores human existence), INV-087 (human focus is not player privilege), INV-088 (regional/world processes are declared causal processes), INV-103 (the scheduler must not construct proposals from raw state): restated — waiting must run the *loaded* world with no human and no caller-seeded eligibility; the registry is derived from declared content, not injected.
5. Fail-closed / actor-knowledge substrate: discovery reads authoritative *loaded content* (declared actors/processes), not hidden runtime truth, so it introduces no telepathy or nondeterminism path; the per-tick consumption that must stay leak-free is the census (`-005`) and the replay reconstruction is `-003`, both cited as the downstream enforcement surfaces.
6. Removal blast radius: restricting `schedule_loaded_actor_decision` / `register_cadenced_world_process` from external reach — repo-wide grep shows consumers only in `crates/tracewake-core/tests/world_step_coordinator.rs` (no `tui`/`content` production callers). Those tests migrate to construction through the production bootstrap (a real loaded package) or a core-crate test-only builder — the implementer-recorded choice per §4.2; record which in implementation notes. A new negative fixture pins the external-seal diagnostic.

## Architecture Check

1. Deriving eligibility from declared content at the one bootstrap the TUI uses removes the vacuity (an empty registry that only a test could fill) and makes the registry a function of loaded state, not caller choreography — the only design that satisfies INV-004/088 without a seam a caller can reopen. Removing the public seeders (rather than documenting "don't call them") is what makes the seal type-enforced.
2. No backwards-compatibility alias: the public seed methods are removed/sealed, not deprecated-but-retained; test setup moves to a real loaded package or a crate-private builder, not a kept-alive public seeder.

## Verification Layers

1. INV-004 / INV-088 (no-human loaded-world progression) -> replay/golden-fixture check: a loaded package with ≥1 actor and ≥1 declared process yields a non-empty derived census/registry with no test registration call.
2. INV-103 (scheduler not constructing from raw state) -> codebase grep-proof: derivation reads `LoadedFixture` content, not authoritative runtime truth.
3. Seal -> negative-fixture compile-fail: an external crate naming `schedule_loaded_actor_decision` / `register_cadenced_world_process` fails with a privacy/not-found diagnostic pinned in `negative_fixture_runner.rs`.

## What to Change

### 1. Derive census and process registry at bootstrap

In the `-001` runtime constructor used by `TuiApp::from_golden`, derive the initial actor census (`loaded_actor_next_decision_tick`-equivalent) and the declared-process registry from the loaded content (`LoadedFixture`). Maintain eligibility as part of the core runtime; TUI construction receives the already-initialized owner (the TUI rewiring itself lands in `-007`).

### 2. Remove external seeding capability

Remove or restrict `schedule_loaded_actor_decision` and `register_cadenced_world_process` so external crates/clients cannot seed actor eligibility or process declarations directly. Provide a core-crate test-only builder (or use a real loaded package) for deterministic test setup.

### 3. Migrate the existing seed-method consumers

Rewrite `world_step_coordinator.rs` setup that calls the public seeders to construct through the production bootstrap or the test-only builder, asserting the same scenarios without external registration.

### 4. Negative fixture

Add `external_crate_cannot_seed_loaded_actor_or_process_eligibility` pinned to a privacy/constructor diagnostic, and register it in `negative_fixture_runner.rs`.

## Files to Touch

- `crates/tracewake-core/src/runtime/session.rs` (modify — file created by 0051FOUCONTHI-001)
- `crates/tracewake-core/src/scheduler.rs` (modify) — restrict seed methods; expose crate-internal derivation hook; merge-hub contributor
- `crates/tracewake-content/src/load.rs` (modify — as surfaced; only if derivation needs a new `LoadedFixture` accessor)
- `crates/tracewake-core/tests/world_step_coordinator.rs` (modify) — migrate off public seeders
- `crates/tracewake-core/tests/negative_fixture_runner.rs` (modify) — register the new fixture; merge-hub contributor
- `tests/negative-fixtures/external_crate_cannot_seed_loaded_actor_or_process_eligibility/` (new)

## Out of Scope

- Reconstructing eligibility on replay/restore (F-02 → `-003`).
- The two-actor + one-process production-bootstrap differential that also asserts the process effect (needs F-03/F-04 → `-009`).
- Migrating the TUI itself onto the runtime (F-06 → `-007`).

## Acceptance Criteria

### Tests That Must Pass

1. A core test loading a ≥1-actor + ≥1-declared-process package through the production bootstrap asserts a non-empty derived census and process registry with **no** call to any scheduler registration method.
2. `cargo test -p tracewake-core --test negative_fixture_runner` passes, including the new eligibility-seal fixture.
3. `cargo test -p tracewake-core --test world_step_coordinator` is green after the seed-method migration.

### Invariants

1. `schedule_loaded_actor_decision` / `register_cadenced_world_process` are not callable from outside `tracewake-core`.
2. Derived eligibility is a function of loaded content only — no hidden runtime truth feeds it.

## Test Plan

### New/Modified Tests

1. `tests/negative-fixtures/external_crate_cannot_seed_loaded_actor_or_process_eligibility/` — compile-fail seal proof.
2. `crates/tracewake-core/tests/world_step_coordinator.rs` — production-bootstrap-derived setup replacing manual seeding.

### Commands

1. `cargo test -p tracewake-core --test negative_fixture_runner`
2. `cargo test -p tracewake-core --test world_step_coordinator`
3. `cargo build --workspace --all-targets --locked && cargo clippy --workspace --all-targets -- -D warnings`

## Outcome

Completed: 2026-06-24

Implemented loaded-world due-work derivation through
`DeterministicScheduler::from_loaded_world`, which derives actor eligibility
from actors present in `PhysicalState` that also have `AgentState` needs, and
derives a deterministic manifest-scoped cadenced process
`process_loaded_world_tick` from loaded content identity. The arbitrary public
`schedule_loaded_actor_decision` and `register_cadenced_world_process` seeders
were removed rather than retained as aliases or shims.

Added the content-side production handoff
`LoadedFixture::into_runtime_initial_state`, allowing loaded fixture packages to
produce `RuntimeInitialState` without introducing a core -> content dependency.
The handoff uses `DeterministicScheduler::from_loaded_world` and the
core-owned runtime established by `0051FOUCONTHI-001`.

Migrated `crates/tracewake-core/tests/world_step_coordinator.rs` off caller
registration and onto the derived loaded-world constructor. The non-vacuous
loaded-world witness now observes derived actor work and the derived process
without any scheduler registration call. Because per-tick controlled/autonomous
actor exclusion is explicitly owned by a later ticket, actor-count assertions in
that witness are lower-bound/non-vacuity checks rather than a one-opportunity
claim.

Added
`tests/negative-fixtures/external_crate_cannot_seed_loaded_actor_or_process_eligibility/`
and registered it in `negative_fixture_runner` to prove an external crate cannot
call the removed seed methods.

Deviations:

- Current fixture schema has no explicit declared-process section, so the
  process registry is derived as one deterministic loaded-world cadence process
  from content manifest identity. A future process schema can replace that
  derivation without restoring arbitrary external seeders.

Verification:

- `cargo test -p tracewake-core --test negative_fixture_runner` — passed.
- `cargo test -p tracewake-core --test world_step_coordinator` — passed.
- `cargo test -p tracewake-content loaded_fixture_hands_off_derived_runtime_due_work` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo fmt --all --check` — passed.
