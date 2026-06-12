# 0023PHA3AEMBLOC-007: Policy-table surface-driven behavioral lock

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — epistemics test suite (`epistemics/projection.rs` tests, `anti_regression_guards.rs` enrollment)
**Deps**: archive/tickets/0023PHA3AEMBLOC-001.md, `specs/0023_PHASE_3A_EMBODIED_LOCALITY_TRUTH_FIREWALL_META_LOCK_WITNESS_INTEGRITY_AND_GUARD_EVASION_CLOSURE_HARDENING_SPEC.md`

## Problem

The 0022 `ORD-HARD-106`/`115` behavioral lock landed partially self-echoing (spec
0023 `ORD-HARD-126`, medium): in
`actor_known_projection_policy_table_drives_record_behavior`, the
`classification()`/`embodied_scope()`/`accessibility_scope()` equality asserts compare
`record.policy()` against `policies[kind]` — but `record.policy()` reads the same
table, so three of four axes cannot fail for any table contents. The one non-echo
assertion (`includes_in_embodied_context`) never instantiates
`NoHumanActorKnownSurfaceBuilder` or `current_place_knowledge_context`, and
`accessibility_scope` has zero surface-level behavioral coverage (only the food-only
unit test exercises it; sleep and workplace `FromAnyPlace` effects are untested at
surface level).

## Assumption Reassessment (2026-06-12)

1. Verified at `db4b53a`: the test exists in `epistemics/projection.rs` tests;
   `record.policy()` delegates to `actor_known_projection_kind_policy` reading the
   same `actor_known_projection_policy_kinds()` map (the 0022 audit's
   operator-verified delegation chain); the real per-kind behavioral coverage is
   hand-written scenarios; `supersede_newest_by_subject_requires_subject_extractor`
   is a genuine behavioral negative (verified holding — not re-landed here).
2. Verified against `specs/0023_…HARDENING_SPEC.md` §4 `ORD-HARD-126`
   (operator-verified at the test symbol; per-assert self-echo detail agent-traced
   during the audit and consistent with the delegation chain).
3. Shared contract under audit: the actor-known projection policy table —
   declaration (`actor_known_projection_policy_kinds()`) vs behavior
   (`NoHumanActorKnownSurfaceBuilder` no-human surface;
   `current_place_knowledge_context` embodied surface) — consumed by all cognition
   context producers. Note: -004 adds new locality record kinds to this table; this
   ticket's table-driven test must iterate *whatever kinds the table declares*
   (derived iteration), so it covers -004's additions automatically whichever lands
   first — no Deps edge needed beyond -001.
4. Constitutional motivation restated: INV-099–106 (the truth-firewall set: the
   policy table governs what enters actor-known context per record kind) and R-29
   (a policy table without a behavioral caller is the named decorative-lock symptom;
   0021's `ORD-HARD-074` lock clause required behavior driven *from* the table).
5. This ticket touches the actor-knowledge filtering enforcement surface: the
   table-driven test proves declared classification/embodied_scope/accessibility_scope
   manifest in emitted facts/affordances on both surfaces — strengthening leakage
   prevention; no production behavior changes, no replay surface touched.
6. Change rationale (no silent retcon): the self-echo asserts are replaced because
   the 0022 evidence proves they cannot fail; mandated by `ORD-HARD-126`'s required
   correction (the original `ORD-HARD-106` contract, completed).

## Architecture Check

1. A table-driven test that, per declared kind, builds the real no-human surface and
   the real embodied context from event-sourced records and asserts the *emitted
   facts/affordances* match the declared policy is cleaner than more hand-written
   scenarios: it derives its case list from the table (new kinds are covered at
   birth — including -004's locality kinds), and it kills the self-echo by putting
   production surfaces on the observed side of every assert.
2. No backwards-compatibility aliasing/shims: the self-echo asserts are replaced by
   surface-driven asserts, not kept as a parallel "fast check".

## Verification Layers

1. Policy→behavior binding (INV-099–106, R-29) -> codebase test-proof: per kind in
   `actor_known_projection_policy_kinds()`, the no-human and embodied surfaces
   reflect declared classification/embodied_scope/accessibility_scope, including the
   `*_believed_accessible` facts for sleep and workplace.
2. Lock non-vacuity -> codebase test-proof: a row-mutation negative (flip one kind's
   `accessibility_scope`/`embodied_scope`) changes surface output and fails the test
   — routed through the production builders, enrolled under -001's registry with a
   live witness.
3. Workplace scope discrimination -> codebase test-proof: the other-place case
   (actor at B, workplace at A) drops from embodied and is retained no-human-side —
   the concretely-unfireable negative 0022 named, now fireable.

## What to Change

### 1. Surface-driven table test

Replace the self-echo asserts in
`actor_known_projection_policy_table_drives_record_behavior` with per-kind
construction of real surfaces from event-sourced records
(`NoHumanActorKnownSurfaceBuilder`, `current_place_knowledge_context`), asserting
emitted facts/affordances per declared axis — including sleep/workplace
`FromAnyPlace` accessibility effects and the workplace other-place negative.

### 2. Row-mutation negative and enrollment

Add the mutation negative (one row's declared scope flipped ⇒ surface output changes
⇒ test fails) routed through the production builders; enroll lock + negative in the
-001-repaired registry.

## Files to Touch

- `crates/tracewake-core/src/epistemics/projection.rs` (modify — test module)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — enrollment)

## Out of Scope

- New policy rows themselves (-004 adds the locality kinds; this test derives its
  iteration from the table either way).
- `SupersedeNewestBySubject` extractor pairing (landed in 0022; verified holding).
- Production policy/dispatch code changes — this is a test-oracle completion.

## Acceptance Criteria

### Tests That Must Pass

1. Per-kind surface-driven asserts pass across every declared kind on both surfaces;
   `accessibility_scope` proven at surface level for food, sleep, and workplace
   (`cargo test -p tracewake-core`).
2. The row-mutation negative fails when any single declared axis is flipped; the
   workplace other-place negative fires on gate removal.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` green.

### Invariants

1. Every declared policy axis is bound to observed surface behavior — no axis is
   asserted only against the table that declares it.
2. The test's case list derives from the live table, so a kind added without
   behavioral conformance fails at birth.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/epistemics/projection.rs` (tests) — surface-driven
   table iteration + row-mutation negative + workplace other-place case.
2. `crates/tracewake-core/tests/anti_regression_guards.rs` — registry enrollment of
   the lock and negative.

### Commands

1. `cargo test -p tracewake-core`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-12
