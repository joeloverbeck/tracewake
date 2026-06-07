# 0007TUIFIXSEL-001: Select the TUI fixture and actor from the command line

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — no simulation-kernel change; adds `tracewake-content` fixture lookup (`fixtures::by_id`) and a `tracewake-tui` `launch` module + `main.rs` argv handling; updates `README.md`. No new events, schemas, or fixtures.
**Deps**: None (builds on landed Phase 1/2A/3A fixture and possession machinery).

## Problem

`cargo run -p tracewake-tui` is hardcoded to load `strongbox_001` and bind `actor_tomas`
(`crates/tracewake-tui/src/main.rs`), so the newest, most complex fixtures landed by Phase
3A — `no_human_day_001`, `ordinary_workday_001`, `sleep_eat_work_001`,
`food_unavailable_replan_001`, `routine_blocked_diagnostic_001`, `planner_trace_001`, etc.
— cannot be exercised through the running binary without editing source and rebuilding. The
kernel already supports loading any of them (`TuiApp::from_golden`), and possession binding
already works at runtime; only launch-time *selection* is missing. This leaves Phase 3A
mechanics reachable only through headless tests, in tension with the TUI-reachability
doctrine. The fix is to let the operator pass a fixture id (and optional actor id) on the
command line, with `--list` to enumerate the catalog.

## Assumption Reassessment (2026-06-07)

1. `crates/tracewake-tui/src/main.rs` hardcodes `TuiApp::load_default()` (→
   `fixtures::strongbox_001()` per `crates/tracewake-tui/src/app.rs:85-87`) then
   `bind_actor("actor_tomas")`. It reads neither `std::env::args()` nor any env var.
   Confirmed.
2. `TuiApp::from_golden(GoldenFixture)` (`crates/tracewake-tui/src/app.rs:89-126`) already
   loads any golden and registers all Phase 1/2A/3A action families unconditionally
   (`register_phase1_*`, `register_phase2a_epistemics`, `register_phase3a_*`), so every
   fixture is runnable once selected. `bind_actor` (`app.rs:128-142`) validates the actor
   against `state.actors` and emits the binding through
   `ControllerBindings::attach` into the event log. Confirmed.
3. `tracewake_content::fixtures::all()` (`crates/tracewake-content/src/fixtures/mod.rs:93-118`)
   returns all 22 goldens; each `GoldenFixture.fixture.fixture_id` is a `FixtureId` and
   `GoldenFixture.fixture.actors` is an ordered `Vec<ActorSchema>` (`crates/tracewake-content/src/schema.rs:19-22,40-42`).
   There is **no** by-id lookup today; `--list` and id resolution will be built on
   `all()`. Confirmed.
4. INV under audit: **INV-065 / INV-066 / INV-071** (TUI is a primary interface; every
   runnable phase has a TUI/view-model gate; mechanics hidden from TUI reachability are
   unfinished). This ticket *advances* these — it makes the landed Phase 3A fixtures
   reachable through the running binary. It also operates within **INV-005 / INV-006 /
   INV-094** (possession changes input binding only, transfers no world knowledge, parity
   tested) by reusing the existing event-sourced `bind_actor`, and within **INV-087**
   (human focus is not player privilege — launch-time actor choice creates no events and
   guarantees no outcomes) and **INV-091** (no-human runnable — the headless no-human tests
   and `debug no-human-day` path are untouched). **INV-069** (the TUI must not implement
   simulation rules / mutate world state) holds: the new `launch` module only selects a
   golden and an actor id; it adds no validator bypass or state-mutation path.
5. No fail-closed validation, actor-knowledge filtering, or replay surface is touched. The
   embodied/debug split, notebook projection, and replay/projection rebuild paths are
   unchanged; fixture selection happens strictly before any pipeline runs.
6. No schema is extended. `fixtures::by_id` is a read-only lookup over existing data;
   `ActorSchema`/`FixtureSchema` are consumed, not modified.
7. Adjacent observation (classify as **future cleanup, not this ticket**): `main.rs` uses
   `.expect()` for user-facing failure. This ticket replaces `.expect()` only on the
   newly-added user-input paths (unknown fixture/actor) with friendly messages + nonzero
   exit; it does not refactor unrelated panics.
8. Mismatch + correction: none. The brainstorm preview suggested an unbound startup line
   for no-actor launches; the chosen behavior is **auto-bind the first authored actor**
   (`fixture.actors.first()`), which preserves today's "renders something on startup" UX
   and means startup always has a bound actor (no graceful-unbound code path needed, since
   every golden has ≥1 authored actor).

## Architecture Check

1. Cleaner than alternatives: an in-TUI `load` command (runtime fixture hot-swap) would
   require a full `TuiApp` state reset and re-bind path, more invasive than the ask; an env
   var is undiscoverable and awkward for passing an actor. CLI args with `--list` are the
   smallest discoverable change, reuse `from_golden`/`bind_actor` verbatim, and keep
   selection logic in a testable `launch` module rather than the untestable `main`. The
   `fixtures::by_id` helper lives in `tracewake-content` (owner of the fixture catalog),
   preserving the one-way dependency direction `tui → content → core`.
2. No backwards-compatibility aliasing/shims: no-arg launch keeps the exact existing
   behavior (`strongbox_001` + `actor_tomas`) without an alias path; `load_default()` is
   retained as the no-arg default, not duplicated.

## Verification Layers

1. INV-066 / INV-071 (Phase 3A mechanics reachable through the running TUI) -> manual
   review (launch each newest fixture and inspect via embodied view + debug panels) plus
   replay/golden-fixture check (existing `from_golden`/golden tests already prove each
   fixture loads and runs).
2. INV-005 / INV-006 / INV-094 (possession binding only, no knowledge transfer, parity) ->
   codebase grep-proof (`launch` resolves to an `ActorId` and calls the existing
   `bind_actor`/`ControllerBindings::attach`; no new binding or state path) plus existing
   `possession_parity_001` / `possession_does_not_reset_intention_001` golden coverage,
   unchanged.
3. INV-069 (TUI implements no simulation rules; no state mutation outside the pipeline) ->
   codebase grep-proof (the `launch` module contains only argv parsing, `fixtures::by_id`
   lookup, and actor-id selection; it touches no validator and mutates no `PhysicalState`).
4. INV-091 (no-human runnable) -> replay/golden-fixture check (no-human tests and the
   `debug no-human-day` path are untouched; auto-bind is possession, not a human-required
   gate).

## What to Change

### 1. `tracewake-content`: fixture lookup by id

Add `pub fn by_id(fixture_id: &str) -> Option<GoldenFixture>` to
`crates/tracewake-content/src/fixtures/mod.rs`, implemented as a scan over `all()` matching
`golden.fixture.fixture_id.as_str()`. This is the single source of truth for resolving a
command-line fixture id and for enumerating the catalog (`all()` already exposes ids and
`fixture.actors`).

### 2. `tracewake-tui`: a testable `launch` module

Add `crates/tracewake-tui/src/launch.rs` (declared `pub mod launch;` in
`crates/tracewake-tui/src/lib.rs`) exposing a pure, testable resolver over an arg slice,
e.g.:

- `pub enum Launch { Run { golden: GoldenFixture, actor_id: ActorId }, List, Help }`
- `pub enum LaunchError { UnknownFixture(String), UnknownActor { fixture_id: String, actor_id: String, available: Vec<String> }, TooManyArgs, BadActorId(String) }`
- `pub fn resolve(args: &[String]) -> Result<Launch, LaunchError>`

Resolution rules:
- no args → `Run { strongbox_001, actor_tomas }` (today's default, sourced via
  `fixtures::by_id("strongbox_001")` and that fixture's first authored actor for
  consistency, which is `actor_tomas`).
- `--list` / `-l` → `List`; `--help` / `-h` → `Help`.
- `<fixture_id>` → resolve via `fixtures::by_id`; unknown → `UnknownFixture`. Actor =
  `golden.fixture.actors.first()` (deterministic authored order).
- `<fixture_id> <actor_id>` → as above but bind the given actor; if the parsed actor id is
  not present in `golden.fixture.actors` → `UnknownActor` carrying the fixture's authored
  actor ids; malformed id → `BadActorId`.
- more than two positional args → `TooManyArgs`.

Provide `pub fn render_catalog() -> String` (one line per fixture: id + authored actor ids,
in `all()` order) and `pub fn usage() -> String` for `--list`/`--help`/error output.

### 3. `tracewake-tui`: thin `main.rs`

Rewrite `crates/tracewake-tui/src/main.rs` to collect argv (skipping arg 0), call
`launch::resolve`, and:
- `Help` / `List` → print usage / catalog, exit 0.
- `Err(_)` → print the friendly message (including `available` actors for `UnknownActor`
  and a "try `--list`" hint for `UnknownFixture`) to stderr, exit nonzero.
- `Run { golden, actor_id }` → `TuiApp::from_golden(golden)`, `bind_actor(actor_id)`, print
  `startup_message()`, run the command loop (unchanged).

### 4. `README.md`: Running the TUI

Update the "Running the TUI" section to document the argument forms
(`cargo run -p tracewake-tui -- <fixture_id> [actor_id]`), the `--list` catalog, and that
no-arg launch still loads `strongbox_001` + `actor_tomas`. Mention that no-human fixtures
such as `no_human_day_001` are inspected via the existing `debug no-human-day` /
`debug planner <actor_id>` panels after launch.

## Files to Touch

- `crates/tracewake-content/src/fixtures/mod.rs` (modify — add `by_id`)
- `crates/tracewake-tui/src/launch.rs` (new)
- `crates/tracewake-tui/src/lib.rs` (modify — declare `pub mod launch;`)
- `crates/tracewake-tui/src/main.rs` (modify — argv → resolve → run)
- `README.md` (modify — Running the TUI)

## Out of Scope

- Runtime fixture hot-swap inside the command loop (no in-TUI `load` command).
- Environment-variable selection.
- Any change to `TuiApp::from_golden`, `bind_actor`, the action registry, the pipeline, or
  any fixture content.
- Refactoring unrelated `.expect()` panics in `main.rs`.
- A graceful-unbound startup path (auto-bind-first-actor makes it unnecessary).

## Acceptance Criteria

### Tests That Must Pass

1. `fixtures::by_id` returns the matching golden for a known id and `None` for an unknown
   id.
2. `launch::resolve` covers: no-arg default (`strongbox_001` + `actor_tomas`); valid
   `<fixture_id>` binds the first authored actor; valid `<fixture_id> <actor_id>` binds the
   named actor; unknown fixture → `UnknownFixture`; unknown actor → `UnknownActor` with the
   fixture's authored actor ids; `--list`/`--help` → `List`/`Help`; >2 args → `TooManyArgs`.
3. `cargo test --workspace` passes; existing `app`/`run` tests (which call `from_golden`
   directly) remain green and unmodified.

### Invariants

1. The new code introduces no path that mutates `PhysicalState` or bypasses the action
   pipeline; world changes still flow only through `run_pipeline` (INV-069).
2. Launch-time selection reuses the event-sourced `bind_actor`; possession remains
   input-binding-only with no knowledge transfer (INV-005/INV-006/INV-094).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/src/fixtures/mod.rs` (`#[cfg(test)]`) — `by_id` hit/miss.
2. `crates/tracewake-tui/src/launch.rs` (`#[cfg(test)]`) — `resolve` cases enumerated in
   Acceptance Criteria #2, plus a `render_catalog` smoke test asserting a known fixture id
   and its actor appear.

### Commands

1. `cargo test -p tracewake-content -p tracewake-tui`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
3. Manual reachability: `cargo run -p tracewake-tui -- --list`, then
   `cargo run -p tracewake-tui -- no_human_day_001` followed by `debug no-human-day` and
   `debug planner actor_mara` — narrower than a scripted test because it verifies
   interactive operator reachability of the newest fixtures (INV-066/INV-071).

## Outcome

Completed: 2026-06-07

What changed:
- Added `tracewake_content::fixtures::by_id` as a catalog lookup over existing golden
  fixtures, with hit/miss tests.
- Added `tracewake_tui::launch`, covering CLI resolution, catalog rendering, usage text,
  friendly launch errors, and resolver tests for defaults, fixture selection, actor
  selection, unknown fixture/actor, malformed actor id, list/help flags, and extra args.
- Updated `tracewake-tui` main to route argv through the launch resolver, print list/help
  output without entering the command loop, and run selected fixtures through
  `TuiApp::from_golden` plus the existing event-sourced `bind_actor`.
- Updated `README.md` with fixture/actor CLI forms, `--list`, the preserved no-arg default,
  and no-human debug-panel reachability.

Deviations from original plan:
- The live `strongbox_001` fixture authors actors as `actor_elena, actor_tomas`, so no-arg
  launch binds the explicit historic default `actor_tomas` instead of using first-authored
  order. Explicit `<fixture_id>` launches still bind the first authored actor.
- `Launch::Run` boxes `GoldenFixture` to satisfy clippy's `large_enum_variant` lint.

Verification:
- `cargo test -p tracewake-content -p tracewake-tui`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`
- `cargo run -p tracewake-tui -- --list`
- `printf 'debug no-human-day\ndebug planner actor_mara\nq\n' | cargo run -p tracewake-tui -- no_human_day_001`
