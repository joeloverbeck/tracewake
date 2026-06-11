# 0021PHA3APOSREB-002: Demote fabricating planning-context builders and rebuild hidden-truth gates on real event logs

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` (`agent/actor_known`, `agent/mod`, `agent/planner` tests, `tests/hidden_truth_gates`, anti-regression guards); conformance-index row
**Deps**: `specs/0021_PHASE_3A_POSSESSION_REBIND_HYGIENE_GUARD_VACUITY_CLOSURE_HARNESS_PROVENANCE_FIDELITY_AND_REJECT_LOUDLY_REPLAY_POSTURE_HARDENING_SPEC.md` (ORD-HARD-068)

## Problem

`observe_visible_local` and its `pub` callers `build_actor_known_planning_state` /
`build_actor_known_planning_state_with_projection_limitation`
(`crates/tracewake-core/src/agent/actor_known.rs`, re-exported from `agent/mod.rs`)
stamp every fact with the fabricated constant
`EventId::new("event_visible_local_planning_state")` — provenance that passes
`audit_with` but corresponds to no logged event — perform no supersession or freshness
classification, and mint `workplace_assignment_active` from mere workplace visibility
with no notice event. No production caller exists, but `tests/hidden_truth_gates.rs`
builds its planner-gate contexts through these builders, so several hidden-truth gates
assert provenance over the harness's own fabrication — the R-27 pattern inside the
gate suite itself, and an open classifier-bypassing third surface for any future
caller (`ORD-HARD-068`, high; INV-102/026, INV-024 in spirit, the constitution's
Enforcement reading).

## Assumption Reassessment (2026-06-11)

1. Verified at `main` `89059a5`: `pub fn build_actor_known_planning_state` /
   `…_with_projection_limitation` / `pub fn observe_visible_local` in
   `actor_known.rs`; the fabricated `EventId` literal at its minting site; re-exports
   in `agent/mod.rs`; `agent/planner.rs` imports the builders but calls them only
   inside its `#[cfg(test)] mod tests`; `tests/hidden_truth_gates.rs` builds contexts
   through them. The production planner path is single-channel:
   `scheduler.rs` → `NoHumanActorKnownSurfaceBuilder` →
   `ActorDecisionTransaction::run`; replay parity via `replay/rebuild.rs`.
2. Verified against spec 0021 (reassessed 2026-06-11; every finding
   operator-verified): finding ORD-HARD-068 and its §9 risk note — a gate that fails
   after the rebuild is a potential latent product defect, triaged under the
   Enforcement reading before any test adjustment. The 0020 supersession tests
   (`workplace_notices_supersede_before_no_human_facts_are_minted` et al.) already
   demonstrate the real-event-log construction pattern (`apply_epistemic_event`) the
   rebuilt gates will use.
3. Cross-artifact boundary under audit: the `ActorKnownPlanningContext` production
   contract — exactly which constructors may mint actor-known facts, and the
   provenance every minted fact must carry (a real `source_event_id` from the log).
4. INV-102 restated: cognition inputs require provenance sufficient for replay and
   debug review — a fabricated constant id is not provenance. INV-026: important
   beliefs record source and acquisition. INV-024 (spirit): a
   `workplace_assignment_active` fact minted from visibility forges the notice
   channel. Enforcement reading: tests must be corrected when they reward cognition
   derived from raw physical state.
5. Actor-knowledge-filtering / hidden-truth-audit surface touched: the change removes
   a fabrication channel and rebuilds gates on modeled channels — strictly narrowing.
   Deterministic replay untouched in production (the builders have no production
   caller; demotion is visibility-only). The hidden-truth audit (`source_context_check`)
   semantics are unchanged; what changes is that gate inputs become genuinely logged
   events.
6. Public-surface removal blast radius (rename/removal item): demoting the three
   `pub` fns to `#[cfg(test)]`-gated (or `pub(crate)` test-support) affects exactly:
   `agent/mod.rs` re-exports, `agent/planner.rs` test module, and
   `tests/hidden_truth_gates.rs`. Workspace grep confirms no other consumer (content,
   tui, docs, specs clean). Note: an integration test under `tests/` cannot see
   `#[cfg(test)]` items — if the rebuilt gates still need a context-construction
   helper, it lives in the test tree (`tests/support/`), not as a core export.

### Closure Reassessment (2026-06-11)

Implementation discovered one additional non-production consumer missed by the
assumption reassessment: `crates/tracewake-content/tests/golden_fixtures_run.rs`
also used the retired planning-context builders. That integration test surface was
converted to the projection-backed no-human actor-known builder rather than kept on
the deleted helper API.

## Architecture Check

1. Deletion-over-routing: the spec permits routing the builders through
   `classified_actor_known_records_for_context`, but no production caller needs them —
   so the cleaner fix is demotion plus rebuilding the gates on real perception/notice
   event logs (the pattern the 0020 supersession tests proved out). This forecloses
   the third-surface contamination channel entirely instead of sanctifying a second
   sanctioned path. The `workplace_assignment_active` visibility-minting is deleted in
   either case.
2. No backwards-compatibility aliasing/shims: the public builders are removed from the
   production surface, not wrapped; no deprecated re-export remains.

## Verification Layers

1. INV-102 (no fabricated provenance) -> source guard: the fabricated `EventId`
   literal is banned workspace-wide outside its (deleted or test-gated) definition
   site; grep-proof in the guard.
2. Enforcement reading (gates assert modeled channels) -> source guard:
   `hidden_truth_gates.rs` contains no call to the fabricating builders; gate
   contexts are built via `apply_epistemic_event` over real logs.
3. INV-101 (sealed context producers) -> source guard: no `pub` producer of
   `ActorKnownPlanningContext` exists outside the
   `NoHumanActorKnownSurfaceBuilder`/classifier pair.
4. Latent-defect triage (spec §9) -> manual review step in this ticket's closure:
   every gate that fails after the rebuild is dispositioned (product defect vs.
   harness artifact) in the ticket's closure note before any assertion is changed.

## What to Change

### 1. Demote the builders

Remove `observe_visible_local`, `build_actor_known_planning_state`, and
`build_actor_known_planning_state_with_projection_limitation` from the public surface
(delete, or move into test support under `tests/support/` if the rebuilt gates need a
shared constructor); delete the `workplace_assignment_active` visibility-minting and
the fabricated `EventId` constant; drop the `agent/mod.rs` re-exports.

### 2. Rebuild the hidden-truth gates

Reconstruct every `hidden_truth_gates.rs` context from real event logs
(`apply_epistemic_event` over seeded perception/notice events), so each asserted fact
carries a genuine `source_event_id`. Rebuild `planner.rs`'s test-module usages the
same way. Triage newly-failing gates per spec §9 before adjusting them.

### 3. Source guards

Add to the anti-regression suite: (a) no `pub` `ActorKnownPlanningContext` producer
outside the builder/classifier pair; (b) `hidden_truth_gates.rs` contains no
fabricating-builder call; (c) the fabricated `EventId` literal does not appear in
production source.

### 4. Documentation

Add the harness-provenance fidelity conformance row to
`docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`.

## Files to Touch

- `crates/tracewake-core/src/agent/actor_known.rs` (modify)
- `crates/tracewake-core/src/agent/mod.rs` (modify)
- `crates/tracewake-core/src/agent/planner.rs` (modify — test module)
- `crates/tracewake-core/tests/hidden_truth_gates.rs` (modify)
- `crates/tracewake-core/tests/support/` (new file only if a shared test constructor is needed — implementation-discovered; parent dir exists)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (modify)

## Out of Scope

- The shared classifier's per-kind policy dispatch (`ORD-HARD-074` — ticket
  0021PHA3APOSREB-005); this ticket removes the bypass, it does not extend the
  classifier.
- Possession/embodied surfaces (ticket 0021PHA3APOSREB-001).
- Any production planner behavior change — the production path is already
  single-channel and stays untouched.

## Acceptance Criteria

### Tests That Must Pass

1. All rebuilt `hidden_truth_gates.rs` gates pass with contexts built from real event
   logs; any gate whose assertion changed carries a dispositioned rationale (product
   defect vs. harness artifact) recorded in the ticket closure.
2. The three source guards pass, each with a synthetic violation case proving it
   fires.
3. `cargo test -p tracewake-core` green; `cargo test --workspace` green.

### Invariants

1. Every actor-known fact asserted by a hidden-truth gate carries a
   `source_event_id` present in the test's event log.
2. No production-visible constructor can mint `ActorKnownPlanningContext` facts
   outside the builder/classifier pair.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/hidden_truth_gates.rs` — rebuilt contexts (real
   logs), every planner gate re-proven.
2. `crates/tracewake-core/tests/anti_regression_guards.rs` — three source guards +
   synthetic violations.
3. `crates/tracewake-core/src/agent/planner.rs` — test module rebuilt on real logs.

### Commands

1. `cargo test -p tracewake-core --test hidden_truth_gates`
2. `cargo test -p tracewake-core --test anti_regression_guards`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

- Removed `VisibleLocalPlanningState`, `observe_visible_local`,
  `build_actor_known_planning_state`, and
  `build_actor_known_planning_state_with_projection_limitation` from the public
  actor-known/planner surface.
- Rebuilt `hidden_truth_gates.rs` contexts from a real `EventLog` plus
  `apply_epistemic_event`; every asserted actor-known source event is checked
  against the test log.
- Converted content integration tests that used the retired helper API to
  `NoHumanActorKnownSurfaceBuilder::from_projection`.
- Added source guards for the retired fabricated event id, the hidden-truth gate
  construction path, and production `ActorKnownPlanningContext` producer sites,
  each with a synthetic failure case.
- Added the 0021 harness-provenance fidelity row to the architecture conformance
  table.
- Removed the now-dead `EpistemicProjection::belief_count_for_actor` helper.

Gate disposition: no hidden-truth gate assertion needed a product-behavior
change after the rebuild. The failing intermediate state was a harness payload
schema issue (`direct_perception` was not a valid channel id), fixed by using the
production `direct_sight` channel id.

## Verification

- `cargo test -p tracewake-core --test hidden_truth_gates`
- `cargo test -p tracewake-content --test golden_fixtures_run planner_trace_fixture_exposes_selection_rejections_and_hidden_truth_audit`
- `cargo test -p tracewake-content --test golden_fixtures_run severe_safety_without_known_exit_is_local_knowledge_blocker`
- `cargo test -p tracewake-content --test golden_fixtures_run no_hidden_truth_fixture_keeps_hidden_food_out_of_planner_inputs`
- `cargo test -p tracewake-core --test anti_regression_guards guard_0021`
- `cargo test -p tracewake-core --test anti_regression_guards`
- `cargo test -p tracewake-core`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`
