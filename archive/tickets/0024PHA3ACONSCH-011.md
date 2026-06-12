# 0024PHA3ACONSCH-011: Closed-world context discrimination and perception-scan binding provenance

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` tests (`hidden_truth_gates.rs`, `anti_regression_guards.rs`); no production crate code.
**Deps**: 0024PHA3ACONSCH-001

## Problem

Spec 0024 findings `ORD-HARD-148` (medium) and `ORD-HARD-149` (medium):

- `148`: the in-`context()` discrimination witness
  (`assert_context_excludes_unseeded_hidden_counterparts`) runs unconditionally but
  its assertions are gated on hardcoded counterpart literals
  (`food_visible_table`, the known-market edge) — a caller with differently-named
  fixtures gets zero hidden-exclusion checks, and a leak under a new name passes.
  The standalone `synthetic_context_leak` witness is a string asserted to contain
  itself — it can never fail (the R-29 decorative shape).
- `149`: `perception_visibility_prose_branch_violations` is per-line lexical
  (display_label / `_id`-family / hidden-prose tokens) with a same-line
  `PayloadField` exemption and no binding provenance — routing a label through an
  exempted typed-payload write, then branching on the payload value in a helper
  whose `&str` parameter is renamed, escapes every token.

## Assumption Reassessment (2026-06-12)

1. Verified against code at baseline `4d62f61`: the
   `if requested_food_sources.contains("food_visible_table")` gates and the
   `synthetic_context_leak.contains("pub viewer_actor_id")` self-assert in
   `hidden_truth_gates.rs`; the scan body (per-line, `perception_line_is_typed_label_payload_write`
   same-line exemption, `branches_on_identity_substring` token set) in
   `anti_regression_guards.rs` — both re-verified at spec reassessment.
2. Verified against spec 0024 §4 (`ORD-HARD-148`/`149`), 0023 §4 `ORD-HARD-131`'s
   "asserted unconditionally" contract and `ORD-HARD-127`'s laundering family, and
   R-29.
3. Cross-artifact boundary: the `context()` test-harness contract (every truth fact
   not seeded as an epistemic event must be absent from the returned context) and
   the perception channel's typed-payload-only discipline (INV-022).
4. INV-024/INV-022 restated: information reaches actors only through modeled
   channels — the discrimination witness must prove the *closed world* (known set ⊆
   seeded set), not the absence of two named ids; prose/identity tokens may not
   branch perception visibility regardless of how many bindings launder them.
5. Enforcement surface (epistemics gates / test oracle): both corrections are
   test-side strengthenings; production perception emission paths verified clean at
   audit. The closed-world subset assertion makes future `context()` callers
   inherit discrimination automatically; the provenance-tracked scan adopts the
   alias-set machinery already proven for the consumed-key derivation
   (`payload_binding_aliases`, `ORD-HARD-130`).

## Architecture Check

1. Closed-world subset assertion (derive the seeded set from the epistemic events
   actually applied; assert known food-sources/edges/workplaces ⊆ seeded) is
   name-independent and caller-independent — strictly stronger than per-literal
   checks and immune to fixture renames. Porting the existing
   `payload_binding_aliases` provenance approach to the perception scan reuses a
   proven mechanism rather than inventing a second tracker; the kill-set synthetics
   (two semantically-equivalent rewrites of the known-bad branch) follow the
   metamorphic-rule-testing pattern recorded in spec §2.
2. No backwards-compatibility aliasing/shims: the self-contains vacuity assert is
   deleted, not kept beside the real witness; the lexical-only scan path is
   replaced.

## Verification Layers

1. INV-024 closed-world (`148`) → leak synthetic: seeding the projection with one
   extra unseeded hidden food/route makes `context()` fail; the vacuity assert is
   gone (codebase grep-proof: `synthetic_context_leak` absent) (tests).
2. INV-022 provenance scan (`149`) → kill set: the renamed-parameter helper rewrite
   and the payload-value relay rewrite both fail the scan (two synthetics).
3. Existing-shape regression → the 0023-era laundering synthetics
   (`synthetic_display_label_binding_laundering`,
   `synthetic_bare_display_label_starts_with`) still fire.
4. Whole-pipeline → full workspace gates.

## What to Change

### 1. Closed-world discrimination (`148`)

`assert_context_excludes_unseeded_hidden_counterparts` derives the seeded set from
the applied epistemic events and asserts the returned context's known sets are
subsets — independent of literal names; delete `synthetic_context_leak`; add the
unseeded-fact leak synthetic that must fail inside `context()`.

### 2. Perception-scan provenance (`149`)

Track bindings derived from `display_label`/`*_id` projections across `let` and
parameter boundaries within the file (port `payload_binding_aliases`); the typed
`PayloadField` write is the only sink; flag branches on any derived binding.

### 3. Kill set

Two synthetics: the renamed-`&str`-helper laundering shape and the
payload-value-relay shape — both must fail the repaired scan.

## Files to Touch

- `crates/tracewake-core/tests/hidden_truth_gates.rs` (modify)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)

## Out of Scope

- Production perception emitters (`current_place_perception_events` etc. — verified
  clean; untouched).
- The consumed-key derivation itself (`ORD-HARD-130`, landed; only its alias
  machinery is reused).
- Witness-rule mechanics (ticket -001, dependency — the repaired scan enrolls under
  it).

## Acceptance Criteria

### Tests That Must Pass

1. The unseeded-hidden-fact synthetic fails inside `context()` regardless of
   fixture names; `grep -c "synthetic_context_leak"` over the test tree returns 0.
2. Both kill-set synthetics fail the repaired perception scan; the two 0023-era
   laundering synthetics still fail; production sources pass (zero violations).
3. The repaired scan's live witness counts inspected sites (enrollment per
   ticket -001).
4. The four workspace gates pass.

### Invariants

1. `context()` structurally proves the closed world: nothing unseeded is knowable,
   for every caller, under any fixture naming.
2. Perception-visibility scanning tracks data provenance, not line tokens; the
   typed-payload write remains the only lawful sink.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/hidden_truth_gates.rs` — closed-world subset
   assertion; unseeded-fact leak synthetic; vacuity-assert deletion.
2. `crates/tracewake-core/tests/anti_regression_guards.rs` — provenance-tracked
   scan; two kill-set synthetics.

### Commands

1. `cargo test -p tracewake-core --test hidden_truth_gates --test anti_regression_guards`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-12

- Replaced literal-specific hidden counterpart assertions with closed-world subset
  checks over every actor-known food source, route, and workplace returned by
  `context()`.
- Removed the vacuous `synthetic_context_leak` self-assert and added projection
  leak synthetics that fail when unseeded food or route knowledge reaches the
  context.
- Strengthened the perception visibility source guard to track display-label/id
  derived bindings through local aliases, typed payload relays, and helper calls;
  added kill-set synthetics for renamed-parameter helper laundering and payload
  value relay branching.

Proof:

1. `cargo test -p tracewake-core --test hidden_truth_gates --test anti_regression_guards`
2. `rg -n "synthetic_context_leak" crates/tracewake-core/tests` returned no matches
3. `cargo fmt --all --check`
4. `cargo clippy --workspace --all-targets -- -D warnings`
5. `cargo build --workspace --all-targets --locked`
6. `cargo test --workspace`
