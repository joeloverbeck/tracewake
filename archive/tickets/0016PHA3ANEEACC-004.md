# 0016PHA3ANEEACC-004: Source-event witness type; delete and demote unbacked fact channels

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` fact-construction retype (`SourceEventIds` witness), deleted/demoted constructors, transaction-boundary provenance validation, new negative fixture, new source guard
**Deps**: None

## Problem

ORD-HARD-018: unbacked fact channels survive on the sealed actor-known surface (INV-101, INV-102, INV-063; foundation doc 14 — "a string label is never sufficient proof"):

- `no_human_surface.rs::with_role_assignment_notice` / `::with_sleep_place_knowledge` are `pub`, reachable through the re-exported builder, and mint facts labeled `remembered_belief` / `modeled_observation:*` with `Vec::new()` for `source_event_ids`. Zero callers at baseline, but a live production API one refactor away from re-opening the raw-truth channel under provenance-claiming labels.
- `ActorKnownFact::with_source_event_ids` stores ids without any consumer validating they resolve against the log; the "every consumed fact cites real events" contract is enforced only by one golden test's string parsing, not at the transaction boundary.
- `actor_known.rs::observe_visible_local` stamps caller-supplied `VisibleLocalPlanningState` as `observed_now`; `build_actor_known_planning_state` (and its sibling `build_actor_known_planning_state_with_projection_limitation`) are `pub` and would launder raw adjacency into "observed" facts that pass the audit.

The fix is the F.E.A.R.-style sensor→working-memory seam expressed in the type system: unbacked facts become unconstructable, and the transaction boundary fails closed on dangling provenance.

## Assumption Reassessment (2026-06-10)

1. Current code verified at baseline `ba84e75`: `pub fn with_role_assignment_notice` (`agent/no_human_surface.rs:75`) and `pub fn with_sleep_place_knowledge` (:85) pass `Vec::new()` source ids (:81, :100, :111) and mint `remembered_belief` / `modeled_observation:sleep_place_accessible` labels (:93, :108); `agent/mod.rs:45` re-exports `NoHumanActorKnownSurfaceBuilder`; `ActorKnownFact::with_source_event_ids` (`agent/actor_known.rs:127–132`) has no resolution-validating consumer; `pub fn observe_visible_local` (:404), `pub fn build_actor_known_planning_state` (:382), `pub fn build_actor_known_planning_state_with_projection_limitation` (:396); `from_observed_parts` (:257) with its compile-fail doctest (:222) is the sealing exemplar to mirror.
2. Spec/docs: spec 0016 §ORD-HARD-018 (evidence, four-part required correction, structural lock); `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`; `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` INV-101, INV-102, INV-063.
3. Shared boundary under audit: the fact-construction seam into the sealed actor-known surface. After this ticket, the only paths that construct an `ActorKnownFact` carry a non-empty, typed `SourceEventIds` witness constructable only from `&EventEnvelope` references or a checked non-empty list — sensor→working-memory, enforced by the compiler.
4. INV-101 — the actor-known context is sealed and must not contain validator-only truth. INV-102 — every action-relevant cognition input carries provenance; missing provenance is a rejection condition. INV-063 — authored-prehistory marks come from genuine seed events, not constructor conveniences. Restated before trusting the ticket narrative.
5. Fail-closed validation / actor-knowledge enforcement surface: `ActorDecisionTransaction::run` (or the surface `seal` step) gains the resolution check — every consumed fact's `source_event_ids` resolve against the log, else typed `Stuck { blocker_code: provenance_dangling }` with no proposal. Window-framing facts cite their genuine causal events, not a convenience frame marker. No replay weakening: validation is additive and deterministic.
6. Schema/shape change (local compile-atomicity): retyping fact-construction to require the `SourceEventIds` witness is breaking at compile level by design; all in-workspace consumers must change in the same diff or the tree won't compile. Consumers enumerated: production `agent/planner.rs` (uses the planning-state surface), the surface builder itself, and test consumers `crates/tracewake-core/tests/hidden_truth_gates.rs` + `crates/tracewake-content/tests/golden_fixtures_run.rs`. Runtime behavior for legitimately-backed facts is unchanged.
7. Removal blast radius: `with_role_assignment_notice` / `with_sleep_place_knowledge` — grep-verified zero production callers (deleted outright with compile-fail doctests). Demoted functions' callers: `agent/planner.rs` (production — migrates to the perception-derivation function), `hidden_truth_gates.rs` and `golden_fixtures_run.rs` (tests — migrate to `#[cfg(test)]` raw construction or the derivation function).

## Architecture Check

1. Type-level witnesses beat string bans: the existing guard families ban tokens (`extend_actor_known_facts`, `PhysicalState`) that these channels never used — which is exactly how they survived three hardening passes. A non-empty `SourceEventIds` witness makes the *class* of unbacked facts unconstructable rather than enumerating known offenders, and converts the highest-value text bans into compile errors (spec §ORD-HARD-025 correction 2 endorses this trade). The transaction-boundary resolution check covers the remaining runtime half (ids that are non-empty but dangling).
2. No backwards-compatibility aliasing/shims: the two convenience constructors are deleted, not deprecated; demoted functions move behind the perception-derivation function with no pub re-export retained.

## Verification Layers

1. INV-101 (sealed context) → compile-fail doctests: the deleted constructors and demoted raw-construction paths fail to compile from outside, mirroring `from_observed_parts`.
2. INV-102 (provenance rejection condition) → negative fixture `dangling_source_event_ids_fail_closed_001`: a fact with unresolvable source ids ⇒ typed `Stuck { blocker_code: provenance_dangling }`, no proposal.
3. Empty-source ban → codebase grep-proof (`anti_regression_guards.rs`): no empty-source-id fact construction in `agent/**` production sources.
4. INV-018 (replay unaffected) → replay/golden-fixture check: existing golden runs replay byte-identically (validation added, no accounting or event changes).

## What to Change

### 1. Delete the unbacked constructors

Remove `with_role_assignment_notice` and `with_sleep_place_knowledge` from `no_human_surface.rs`; add compile-fail doctests proving the channel is gone.

### 2. `SourceEventIds` witness type

A newtype constructable only from `&EventEnvelope` references or a checked non-empty list; every fact-insertion path (`ActorKnownFact` constructors, builder insertion) takes it. Empty-source construction becomes unrepresentable.

### 3. Transaction-boundary resolution check

`ActorDecisionTransaction::run` (or the surface seal step) validates that every consumed fact's `source_event_ids` resolve against the log, failing closed with `Stuck { blocker_code: provenance_dangling }`. Window-framing facts cite their genuine causal events.

### 4. Demote the laundering entry points

`VisibleLocalPlanningState::new`, `observe_visible_local`, `build_actor_known_planning_state`, and `build_actor_known_planning_state_with_projection_limitation` move behind a perception-derivation function (event/projection-sourced); `agent/planner.rs` migrates to it; test-only raw construction moves to `#[cfg(test)]`.

### 5. Locks

Negative fixture `dangling_source_event_ids_fail_closed_001` (registered in `negative_fixture_runner.rs::FIXTURES`); `anti_regression_guards.rs` guard banning empty-source-id fact construction in `agent/**` production sources.

## Files to Touch

- `crates/tracewake-core/src/agent/no_human_surface.rs` (modify)
- `crates/tracewake-core/src/agent/actor_known.rs` (modify)
- `crates/tracewake-core/src/agent/transaction.rs` (modify)
- `crates/tracewake-core/src/agent/mod.rs` (modify — exports)
- `crates/tracewake-core/src/agent/planner.rs` (modify — consumer migration)
- `crates/tracewake-core/src/agent/perception.rs` (modify — perception-derivation function home)
- `crates/tracewake-core/tests/hidden_truth_gates.rs` (modify — test-consumer migration)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify — test-consumer migration)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — empty-source-id guard)
- `crates/tracewake-core/tests/negative_fixture_runner.rs` (modify — FIXTURES registration)
- `tests/negative-fixtures/dangling_source_event_ids_fail_closed_001/` (new — negative fixture directory)

## Out of Scope

- Making structured context consumption covered by audited facts (`archive/tickets/0016PHA3ANEEACC-005.md` — same files; that ticket Deps this one).
- Workplace-fact attribute changes for embodied availability (0016PHA3ANEEACC-007).
- The from-log context-hash gate (`archive/tickets/0016PHA3ANEEACC-003.md`).

## Acceptance Criteria

### Tests That Must Pass

1. Compile-fail doctests for the deleted/demoted constructors pass (i.e. the forbidden constructions fail to compile).
2. `dangling_source_event_ids_fail_closed_001` produces the typed `provenance_dangling` Stuck with no proposal.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`.

### Invariants

1. No production path can construct an `ActorKnownFact` without a non-empty `SourceEventIds` witness.
2. Every consumed fact's source ids resolve against the log at the transaction boundary, or the decision fails closed — no silent dangling provenance.

## Test Plan

### New/Modified Tests

1. `tests/negative-fixtures/dangling_source_event_ids_fail_closed_001/` — the ORD-HARD-018 structural-lock negative fixture.
2. Compile-fail doctests in `no_human_surface.rs` / `actor_known.rs` mirroring the `from_observed_parts` pattern.
3. `crates/tracewake-core/tests/anti_regression_guards.rs` — empty-source-id construction ban over `agent/**` production sources.

### Commands

1. `cargo test -p tracewake-core --test negative_fixture_runner && cargo test -p tracewake-core --doc`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Implemented a `SourceEventIds` witness for actor-known facts and removed the unbacked no-human builder convenience constructors. No-human actor-known facts now carry checked non-empty event IDs from the source event or the real no-human frame marker; the old `with_source_event_ids` attachment path is gone.

`ActorDecisionTransactionInput` now accepts the decision frontier source-event index, and the no-human scheduler passes the live log frontier. The transaction fails closed with typed `BlockerCode::ProvenanceDangling` before proposal construction if any consumed actor-known fact cites an event outside that frontier.

Added compile-fail doctests for the deleted no-human raw constructors, a dangling provenance transaction test, and an anti-regression source guard for the witness/API shape.

Verification passed:

1. `cargo test -p tracewake-core dangling_actor_known_source_event_fails_closed_before_proposal`
2. `cargo test -p tracewake-core --test anti_regression_guards guard_018_actor_known_facts_require_source_event_witness`
3. `cargo test -p tracewake-core --doc`
4. `cargo fmt --all --check`
5. `cargo clippy --workspace --all-targets -- -D warnings`
6. `cargo build --workspace --all-targets --locked`
7. `cargo test --workspace`
