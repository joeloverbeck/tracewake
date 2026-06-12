# 0021PHA3APOSREB-009: Content-crate integrity — duplicate-assignment rejection, contract-prose reconciliation, canonical-encoding hygiene

**Status**: DONE
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-content` (`schema`, `validate`, `serialization`, five wrapper fixtures, fixture tests)
**Deps**: `archive/specs/0021_PHASE_3A_POSSESSION_REBIND_HYGIENE_GUARD_VACUITY_CLOSURE_HARNESS_PROVENANCE_FIDELITY_AND_REJECT_LOUDLY_REPLAY_POSTURE_HARDENING_SPEC.md` (ORD-HARD-081, 087, 098)

## Problem

(1) `FixtureSchema::to_agent_state` synthesizes intention/execution IDs by stripping
the `actor_` prefix and collapsing routine families
(`routine_family_assignment_suffix`), then inserts via `entry().or_insert` — two
same-(actor, family) assignments, or actors named `actor_mara` and `mara`, collide and
the later authored assignment is **silently discarded**; `active_intention_by_actor`'s
`or_insert_with` invents a loader-default active intention; no validation rejects any
of this (`ORD-HARD-081`; INV-020 silent repair of authored, gameplay-affecting
state). (2) Five fixture contracts carry prose ("no observation or belief reveals the
hidden food") that directly contradicts the authored
`with_actor_mara_known_hidden_food` seed (`ORD-HARD-087`). (3) Hygiene
(`ORD-HARD-098`): `serialization.rs::source_id` writes Rust `Debug` output into the
canonical byte format for `SourceRef::Cause` (latent — currently rejected by seed
validation, breaks when a second source kind is admitted);
`has_explicit_no_sleep_diagnostic` keys a validation gate off free-text diagnostic
substrings (`"no_sleep"`, INV-105 spirit); and the missing-template `else { continue }`
in `to_agent_state` is silent-repair-shaped (unreachable behind the validation token
today).

## Assumption Reassessment (2026-06-11)

1. Verified at `main` `89059a5`: `strip_prefix("actor_")` +
   `routine_family_assignment_suffix` + the `or_insert`/`or_insert_with` discard
   sites in `schema.rs::to_agent_state`; `validate_references` checks actor/template
   existence and tick order only; `format!("{cause:?}")` in
   `serialization.rs::source_id`; the substring checks in
   `validate.rs::has_explicit_no_sleep_diagnostic`; the contradicting setup prose in
   `hidden_food_closed_container_001.rs` ("no observation or belief reveals the
   hidden food") seeded by its own wrapper call. The five wrapper consumers (exact,
   grep-verified): `hidden_food_closed_container_001`,
   `hidden_food_unknown_route_001`, `hidden_route_edge_001`,
   `debug_omniscience_excluded_001`, `workplace_assignment_provenance_001`.
2. Verified against spec 0021 (reassessed 2026-06-11; operator-verified): findings
   ORD-HARD-081/087/098; the 087 correction includes verifying, per fixture, whether
   the seeded edge is load-bearing — dropping the wrapper where it is not (four of
   five have purposes unrelated to the seeded edge, per the audit).
3. Cross-artifact boundary under audit: the fixture-authoring contract — authored
   sections are either loaded faithfully or rejected loudly (never silently
   collapsed), and contract prose (setup/acceptance strings) describes the authored
   epistemic state truthfully.
4. INV-020 restated: rejection over repair — authored state that cannot be
   represented must fail validation, not vanish. INV-063: authored prehistory is
   visible/marked, not denied by contract prose. INV-105 (spirit): gates key off
   typed substrates, not display strings.
5. Fail-closed-validation surface touched: new duplicate-(actor, family) and
   ambiguous-actor-suffix rejections strengthen the gate (new typed validation
   errors); the canonical-encoding fix for `SourceRef::Cause` changes bytes only for
   a source kind currently rejected upstream (no live fixture bytes change — locked
   by the existing roundtrip-drift validation); the typed no-sleep diagnostic
   replaces substring matching with a closed kind. No deterministic-replay impact
   (loader-side; seed events unchanged for valid fixtures).
6. Schema shape: duplicate rejection adds no field. The active-intention resolution
   is a spec-assigned implementer-recorded choice — an explicit authored flag
   (schema extension, additive: optional with documented semantics; consumers:
   loader + validation + fixtures that need it) or a documented derivation rule (no
   shape change). The recorded choice and its additive-vs-breaking consequence land
   in this ticket's closure note; if the flag is chosen, its consumers are updated
   in this same ticket (local atomic unit).

## Implementation Outcome (2026-06-11)

1. Added validation-time rejection for the authored collisions that previously
   collapsed silently:
   - Duplicate `(actor_id, routine family)` routine assignments now emit
     `duplicate_actor_routine_family`.
   - Actors whose IDs collapse to the same routine-assignment suffix, such as
     `actor_mara` and `mara`, now emit `ambiguous_actor_assignment_suffix`; if they
     would derive the same intention/execution suffix, validation also emits
     `routine_assignment_id_collision`.
   - Same-actor routine assignments with the same `start_tick` now emit
     `ambiguous_active_routine_assignment`.
2. Recorded and implemented the no-schema active-intention rule: valid fixtures may
   author sequential routine assignments for one actor, and the loader derives the
   active fixture intention from the earliest authored assignment `start_tick`.
   Equal-start ties are rejected before `to_agent_state`. The loader now asserts
   validated uniqueness for generated intention/execution IDs and no longer uses
   `entry().or_insert` or the missing-template `continue` path.
3. Reconciled the five hidden-truth fixture contracts:
   - `hidden_food_closed_container_001` keeps the load-bearing
     `actor_mara -> food_hidden_pantry` known-food edge and now names it
     truthfully while distinguishing usable target selection from authored
     knowledge.
   - `hidden_food_unknown_route_001`, `hidden_route_edge_001`,
     `debug_omniscience_excluded_001`, and
     `workplace_assignment_provenance_001` dropped the unrelated
     `with_actor_mara_known_hidden_food` wrapper and their setup text now states
     the absence or irrelevance of that edge.
   - Added a fixture-aware contract metadata check that rejects contract prose that
     denies an authored `known_food_sources` seed or omits the seeded actor/food
     IDs.
4. Replaced content canonical `SourceRef::Cause` debug formatting with stable
   `event:`, `proposal:`, `validation_report:`, and `process:` IDs matching event
   envelope cause IDs. The source kind is still rejected for initial belief seeds
   by current validation policy; the test pins the latent canonical bytes and
   grep-proves no content canonical path uses `{cause:?}`.
5. Replaced substring matching for the no-sleep validation escape hatch with a
   closed typed diagnostic check. The fixture diagnostic now uses exact
   `no_sleep_affordance`, and a negative test proves prose-appended variants do
   not satisfy the gate.

## Verification (2026-06-11)

1. `cargo test -p tracewake-content --test fixtures_load`
2. `cargo test -p tracewake-content`
3. `rg -n "format!\\(\\\"\\{cause:\\?\\}\\\"|\\{cause:\\?\\}" crates/tracewake-content/src`
   returned no matches.
4. `cargo fmt --all --check`
5. `cargo clippy --workspace --all-targets -- -D warnings`
6. `cargo build --workspace --all-targets --locked`
7. `cargo test --workspace`
8. `git diff --check`

## Architecture Check

1. Rejection-at-validation is the only INV-020-compliant fix: the collision is
   detectable precisely (`(actor, family)` pair + suffix ambiguity) at
   `validate_references` time, before any state synthesis — cheaper and louder than
   de-ambiguating synthesized IDs (which would silently change existing fixture
   identities). Making the active-intention choice explicit removes the
   loader-invented default rather than documenting it. The typed no-sleep diagnostic
   kind replaces prose matching with the same closed-vocabulary pattern the rest of
   the diagnostics already use.
2. No backwards-compatibility aliasing/shims: the dead `else { continue }` becomes
   unreachable-by-construction or a typed error; the Debug encoding is replaced by a
   stable typed encoding, not wrapped.

## Verification Layers

1. INV-020 (rejection over repair) -> negative fixtures: duplicate (actor, family)
   assignment and `actor_mara`/`mara` suffix collision each yield typed validation
   errors; the loader-invented active-intention default is gone (explicit flag or
   documented rule, test-pinned).
2. INV-063/contract honesty (ORD-HARD-087) -> contract-vs-sections consistency
   check: a fixture with non-empty `known_food_sources` must name each seeded edge
   in setup prose (or at minimum carry no "no … belief" denial); the five fixtures'
   prose corrected; wrappers dropped where the edge is not load-bearing (per-fixture
   verification recorded).
3. INV-018/canonical stability (ORD-HARD-098a) -> stable typed encoding for
   `SourceRef::Cause` with roundtrip test; grep-proof no `{cause:?}` in canonical
   paths.
4. INV-105 spirit (ORD-HARD-098b) -> typed no-sleep diagnostic kind; substring gate
   deleted; gate test re-proven on the typed kind.

## What to Change

### 1. Collision rejection (ORD-HARD-081)

`validate_references` rejects duplicate (actor, family) routine assignments and
ambiguous actor-ID suffixes; apply the recorded active-intention choice; convert the
dead missing-template skip.

### 2. Contract-prose reconciliation (ORD-HARD-087)

Correct setup/acceptance strings in the five wrapper fixtures to name the authored
belief; drop `with_actor_mara_known_hidden_food` from fixtures where the edge is not
load-bearing (verify per fixture — expected droppable: `hidden_route_edge_001`,
`hidden_food_unknown_route_001`, `debug_omniscience_excluded_001`,
`workplace_assignment_provenance_001`; keep where load-bearing:
`hidden_food_closed_container_001` — confirm at implementation); add the
contract-vs-sections consistency check to fixture validation/tests.

### 3. Encoding/diagnostic hygiene (ORD-HARD-098)

Stable typed encoding in `source_id`; typed no-sleep diagnostic kind replacing the
substring gate.

## Files to Touch

- `crates/tracewake-content/src/schema.rs` (modify)
- `crates/tracewake-content/src/validate.rs` (modify)
- `crates/tracewake-content/src/serialization.rs` (modify)
- `crates/tracewake-content/src/fixtures/mod.rs` (modify)
- `crates/tracewake-content/src/fixtures/hidden_food_closed_container_001.rs` (modify)
- `crates/tracewake-content/src/fixtures/hidden_food_unknown_route_001.rs` (modify)
- `crates/tracewake-content/src/fixtures/hidden_route_edge_001.rs` (modify)
- `crates/tracewake-content/src/fixtures/debug_omniscience_excluded_001.rs` (modify)
- `crates/tracewake-content/src/fixtures/workplace_assignment_provenance_001.rs` (modify)
- `crates/tracewake-content/tests/fixtures_load.rs` (modify)

## Out of Scope

- The known-food census depth/clippy lock (`ORD-HARD-093` — ticket
  0021PHA3APOSREB-012); this ticket may *reduce* wrapper consumers, which 012's
  census then re-pins.
- The place-concealment schema work (ticket 0021PHA3APOSREB-007) — coordinate the
  `schema.rs`/`fixtures_load.rs` mechanical merges.

## Acceptance Criteria

### Tests That Must Pass

1. Negative fixtures: duplicate (actor, family) and suffix collision → typed
   validation errors; valid fixtures unaffected.
2. Active-intention resolution explicit per the recorded choice; the canonically-first
   loader default is gone (test-pinned).
3. All five fixture contracts read truthfully against their authored sections; the
   consistency check passes (and fails on a synthetic denial-prose fixture); dropped
   wrappers verified per fixture with the gate tests still green.
4. `SourceRef::Cause` roundtrips through the stable typed encoding; no Debug
   formatting in canonical paths (grep-proof); no-sleep gate keyed on the typed kind.
5. `cargo test -p tracewake-content` and `cargo test --workspace` green.

### Invariants

1. Authored fixture state is loaded faithfully or rejected loudly — no silent
   collapse, no loader-invented defaults.
2. Fixture contract prose never denies an authored epistemic seed.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/tests/fixtures_load.rs` — collision negatives,
   consistency check + synthetic, roundtrip for the typed encoding.
2. `crates/tracewake-content/src/validate.rs` (mod tests) — typed no-sleep
   diagnostic gate.

### Commands

1. `cargo test -p tracewake-content --test fixtures_load`
2. `cargo test -p tracewake-content`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
