# 0051FOUCONTHI-005: F-04 closed exhaustive per-tick actor disposition census

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — build one closed, exhaustive per-tick actor census assigning exactly one disposition to each loaded actor before executing any proposal.
**Deps**: 0051FOUCONTHI-004

## Problem

`transact_world_one_tick` processes controlled proposals first (`scheduler.rs:786`), then runs `ActorDecisionTransaction` for every due actor with `controller_bindings: None` (`888`–`963`), with no exclusion for an actor already represented in `controlled_proposals`, no controller-binding check in the `due_actor_ids` census (`868`), and no closed per-tick actor disposition table. Today's collision is masked only because F-01 left the registry empty; closing F-01 (`-002`) makes the double-opportunity reachable (F-04, structural gap). The fix builds one closed, exhaustive census that gives each loaded actor exactly one disposition, where a valid controlled input consumes that tick's ordinary actor opportunity.

## Assumption Reassessment (2026-06-24)

1. Codebase: controlled-then-due ordering in `crates/tracewake-core/src/scheduler.rs` (controlled at `786`, `due_actor_ids` derived at `868`, the due loop running `ActorDecisionTransaction::run` with `controller_bindings: None` at `963`) with no exclusion of actors already in `controlled_proposals`; `reservation_body_exclusive_census.rs` already exercises body-exclusive reservation. No `ActorDisposition`/`ActorOpportunity` enum exists (grep: 0 matches) — to be created, private to core.
2. Specs/docs: spec `0051` §4.4, §9.4 (actor next-decision rule), §9.5 (disposition data shape — closed enum, indexed census table, or typestate builder: implementer-recorded choice). Architecture home `docs/1-architecture/04_*`; execution `docs/2-execution/05_*` (no direct dispatch).
3. Shared boundary under audit: the per-tick disposition assignment — the census enum and constructors stay private to core; external callers submit commands, not dispositions.
4. INV-004 (no-human authority), INV-005 (the only normal controller binding is ordinary possession), INV-043 (action validation is ordinary-agent validation — no player-privilege branch), INV-094 (possession parity is tested), INV-108 (human possession is cognition-neutral): restated — a controlled input consumes the ordinary opportunity, the possessed actor is not special in rules, and possession changes input only.
5. Fail-closed / possession-parity surface: the census assigns exactly one disposition per loaded actor (controlled attempt, autonomous decision, body-exclusive duration continuation/reservation, deferred/not-due, stuck/blocked); duration reservation takes precedence consistently for human and autonomous actors; no controller-specific validation branch is introduced. This is the possession-parity / no-double-opportunity enforcement surface; exhaustive handling is required before commit. No schema is extended (the census enum is private, in-memory only — heuristic N/A).

## Architecture Check

1. A closed `ActorDisposition` enum requiring exhaustive handling before commit makes "exactly one opportunity per actor" a type-level property rather than an ordering accident — the only design that survives F-01's now-non-empty registry. Building the census *before* executing any proposal, with the controlled input consuming the ordinary slot, removes the double-opportunity at its root.
2. No backwards-compatibility alias: the controlled-then-due-without-exclusion ordering is replaced by the census, not wrapped; no public disposition constructor is exposed.

## Verification Layers

1. INV-005 / INV-108 (possession changes input only) -> behavior test: a single tick with a controlled possessed actor + a second unpossessed due actor + a third under body-exclusive duration asserts exactly one disposition per actor and ≤1 ordinary proposal per actor.
2. INV-043 (ordinary-agent validation) -> codebase grep-proof: no controller-specific validation branch in the census/decision path.
3. INV-094 (possession parity) -> permutation/differential test: reordering proves canonical output and possession parity.

## What to Change

### 1. Closed per-tick census

Before executing any proposal, build one closed, exhaustive per-tick actor census assigning exactly one disposition to each loaded actor (controlled attempt for the bound actor, autonomous decision opportunity, body-exclusive duration continuation/reservation, deferred/not-due, stuck/blocked). The census enum and constructors are private to core.

### 2. Controlled input consumes the ordinary opportunity

A valid controlled input consumes that tick's ordinary actor opportunity; the possessed actor is not special in rules. Duration reservation takes precedence consistently for human and autonomous actors.

## Files to Touch

- `crates/tracewake-core/src/runtime/session.rs` (modify — file created by 0051FOUCONTHI-001)
- `crates/tracewake-core/src/scheduler.rs` (modify) — closed census before proposals; controlled-consumes-opportunity; merge-hub contributor
- `crates/tracewake-core/tests/world_step_coordinator.rs` (modify) — one-disposition-per-actor + permutation/differential; merge-hub contributor
- `crates/tracewake-core/tests/reservation_body_exclusive_census.rs` (modify) — duration-precedence integration with the census
- `crates/tracewake-core/tests/negative_fixture_runner.rs` (modify) — extend the disposition-injection seal; merge-hub contributor

## Out of Scope

- Full actor decision-transaction consumption (F-05 → `-006`).
- Declared-process dispositions (processes are handled by F-03 → `-004`; this census is over actors).

## Acceptance Criteria

### Tests That Must Pass

1. A single tick with a controlled possessed actor + an unpossessed due actor + a body-exclusive-duration actor yields exactly one disposition per actor and ≤1 ordinary proposal per actor.
2. Permutation/differential reordering produces canonical output and possession parity.
3. `cargo test -p tracewake-core --test world_step_coordinator --test reservation_body_exclusive_census` (run each target) is green.

### Invariants

1. Exactly one disposition per loaded actor per tick; controlled and autonomous are mutually exclusive.
2. No controller-specific validation branch exists.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/world_step_coordinator.rs` — census cardinality + permutation cases.
2. `crates/tracewake-core/tests/reservation_body_exclusive_census.rs` — body-exclusive precedence under the census.

### Commands

1. `cargo test -p tracewake-core --test world_step_coordinator`
2. `cargo test -p tracewake-core --test reservation_body_exclusive_census`
3. `cargo build --workspace --all-targets --locked && cargo clippy --workspace --all-targets -- -D warnings`

## Outcome

Completed: 2026-06-24

Implemented the per-tick actor opportunity exclusion by building a private
`controlled_actor_ids` census from submitted controlled proposals before
autonomous actor transactions run. The autonomous due-actor list is filtered
against that census, so a controlled input consumes that actor's ordinary
opportunity for the tick and cannot also receive a no-human actor decision in
the same world step.

Updated the loaded-world differential witness to assert exactly one autonomous
actor transaction when the possessed actor submits a controlled wait and a
second actor is due. The existing body-exclusive reservation census remains
green and continues to prove reservation precedence against ordinary actions.

Deviations:

- The implemented private census is a `BTreeSet<ActorId>` of controlled actors
  plus the existing due-actor vector, rather than a named public or persisted
  `ActorDisposition` enum. The shape is intentionally private and in-memory;
  no external disposition constructor was added.

Verification:

- `cargo test -p tracewake-core --test world_step_coordinator` — passed.
- `cargo test -p tracewake-core --test reservation_body_exclusive_census` — passed.
- `cargo fmt --all --check` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
