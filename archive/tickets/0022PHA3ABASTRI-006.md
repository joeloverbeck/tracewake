# 0022PHA3ABASTRI-006: Table-driven policy behavioral locks and supersede-subject pairing

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — actor-known policy layer (`epistemics/projection.rs`), behavioral lock tests, census registration
**Deps**: 0022PHA3ABASTRI-004

## Problem

The 0021 `ORD-HARD-074` structural lock required "a behavioral test per record kind
driven from the table"; what landed is a self-echo (`ORD-HARD-106`):
`actor_known_projection_records_dispatch_to_declared_policy_table` asserts
`record.policy().classification() == policies[kind].classification()`, but `policy()`
is implemented as `actor_known_projection_kind_policy(kind)` reading the identical
map — the assertion cannot fail for any table contents. Concretely unfireable
negative: workplace's `ActorKnownProjectionEmbodiedScope::CurrentPlaceOnly` gate —
every embodied workplace test co-locates actor and workplace, so flipping or removing
the scope passes the suite. Companion latent trap (`ORD-HARD-115`):
`includes_classified_record`'s `SupersedeNewestBySubject` arm hard-returns `false` for
non-Workplace records, so a future kind declared supersede-by-subject is silently
dropped rather than superseded.

## Assumption Reassessment (2026-06-12)

1. Verified at `9ce820f`: `ActorKnownProjectionRecord::policy()` delegates per-kind to
   `actor_known_projection_kind_policy`, which reads
   `actor_known_projection_policy_kinds()` — the same map the dispatch test compares
   against (`crates/tracewake-core/src/epistemics/projection.rs`); the
   `SupersedeNewestBySubject` arm carries
   `let ActorKnownProjectionRecord::Workplace { workplace_id, .. } = record else { return false; }`.
   The post-audit `-014` test `food_source_accessible_fact_requires_from_any_place_scope`
   behaviorally locks the food accessibility-scope gate but does not drive from the
   table and does not cover the workplace embodied-scope negative.
2. Verified against `specs/0022_…HARDENING_SPEC.md` §4 `ORD-HARD-106`/`115`
   (operator-verified) and the verified-holding record that `policy()` has real
   production callers (`classified_actor_known_records_for_context`,
   `current_place_knowledge_context`, `consume_projection_record`) — the product
   behavior is correct; the *lock* is vacuous.
3. Shared contract under audit: the per-kind policy table
   (`actor_known_projection_policy_kinds()`) as the single declaration of
   classification / embodied-scope / accessibility-scope per record kind, consumed by
   both the no-human and embodied surfaces.
4. Constitutional motivation restated: INV-028 (staleness is not automatically
   corrected — the freshness/supersession behavior the table declares) and the R-29
   lock-durability rule (a lock must fail when behavior diverges from declaration).
5. This ticket touches the actor-knowledge-filtering surface's acceptance tests: the
   new behavioral tests assert existing correct behavior on both surfaces and add the
   missing negatives; no production epistemics change except the `ORD-HARD-115`
   supersede-subject pairing, which is behavior-preserving for all current kinds
   (only Workplace carries the policy today) and fail-loud for future kinds.
6. Schema note (additive-vs-breaking): pairing `SupersedeNewestBySubject` with a
   subject extractor (or a compile-time assertion that only Workplace may carry it) is
   an internal non-breaking change — no serialized schema, event payload, or
   view-model shape changes; consumers are the two projection surfaces, verified
   unchanged in behavior by the new parametric test.
7. Change rationale (no silent retcon): the dispatch test is replaced because the 0022
   audit proved it tautological; the replacement asserts the behavior the 0021
   correction promised.

## Architecture Check

1. A parametric test iterating `actor_known_projection_policy_kinds()` and asserting
   *observed* supersession/scope behavior per kind on both surfaces binds behavior to
   declaration at the only place drift can start — adding a kind or editing an arm
   without matching behavior fails one loop iteration. Pairing the supersede policy
   with a subject extractor removes the silent-drop arm entirely.
2. No backwards-compatibility aliasing/shims: the self-echo test is deleted, not kept
   alongside the behavioral one.

## Verification Layers

1. Behavior-bound-to-declaration (R-29) -> replay/projection check: for each kind in
   the table, build a projection exercising that kind from real events and assert the
   no-human and embodied surfaces reflect the declared
   classification/embodied_scope/accessibility_scope.
2. Workplace embodied-scope negative -> projection check: actor at place B, workplace
   record at place A — embodied context drops the workplace, no-human retains it
   (the `CurrentPlaceOnly` discrimination the suite currently cannot fire).
3. Supersede-subject totality (`ORD-HARD-115`) -> codebase grep-proof + test: every
   kind declared `SupersedeNewestBySubject` has a subject extractor (or the
   only-Workplace assertion); a synthetic non-Workplace supersede kind fails loudly
   rather than dropping records.
4. Census registration (§5.1) -> guard check: the parametric test and its
   mutate-one-arm synthetic negative register with the `-004` bijection census.

## What to Change

### 1. Replace the self-echo dispatch test with a table-driven behavioral test

In `epistemics/projection.rs` tests (or the integration-test home the existing
both-surface tests use — implementation-discovered within
`crates/tracewake-core/tests/`): iterate `actor_known_projection_policy_kinds()`; per
kind, build projections from real event logs exercising fresh/stale/other-place
variants; assert both surfaces' observed inclusion/freshness/accessibility against the
declared policy. Delete `actor_known_projection_records_dispatch_to_declared_policy_table`
and the hardcoded-literal half of
`actor_known_projection_policy_table_declares_every_record_kind` (keep its
kind-coverage assertion, derived).

### 2. Add the workplace other-place negative

The embodied/no-human split test from verification layer 2.

### 3. Pair supersede policy with subject extraction

Replace the Workplace let-else in `includes_classified_record` (and the sibling
freshness arm) with a per-kind subject extractor paired to the policy declaration, or
a compile/test assertion that only Workplace may declare `SupersedeNewestBySubject`;
synthetic negative proving a non-Workplace supersede kind cannot silently drop.

### 4. Census registration

Register the parametric test + synthetic negative with the `-004` bijection census.

## Files to Touch

- `crates/tracewake-core/src/epistemics/projection.rs` (modify)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — census
  registration)
- Behavioral-test home (modify — implementation-discovered within
  `crates/tracewake-core/tests/`; parent directory verified)

## Out of Scope

- Any change to the declared policies themselves (the table's contents are doctrine
  the 0021 pass settled; this ticket locks behavior to them).
- The hidden-truth gate restoration (`0022PHA3ABASTRI-005`).
- `no_human_surface.rs` / `perception.rs` production logic (their consumption of
  `policy()` is verified correct; tests drive them as-is).

## Acceptance Criteria

### Tests That Must Pass

1. The parametric per-kind test passes on both surfaces for every kind in
   `actor_known_projection_policy_kinds()`, and a temporarily-mutated arm (one kind's
   behavior diverged from declaration) fails it — kill-proof applied and reverted
   surgically during implementation.
2. The workplace other-place negative passes (embodied drops, no-human retains).
3. `grep -c "actor_known_projection_records_dispatch_to_declared_policy_table" crates/`
   returns 0 (self-echo deleted).
4. `cargo test -p tracewake-core` and the four-gate suite:
   `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. Every record kind's observed projection behavior on both surfaces matches its
   declared policy; divergence fails the suite.
2. No policy variant can silently drop records: supersede-by-subject either extracts
   a subject or refuses the declaration at test time.

## Test Plan

### New/Modified Tests

1. Table-driven parametric behavioral test (both surfaces, per kind) — replaces the
   self-echo dispatch test.
2. Workplace `CurrentPlaceOnly` other-place negative.
3. Non-Workplace supersede-kind synthetic negative.
4. `crates/tracewake-core/tests/anti_regression_guards.rs` — census entries.

### Commands

1. `cargo test -p tracewake-core epistemics`
2. `cargo test -p tracewake-core --test anti_regression_guards`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-12

- Replaced the self-echo policy dispatch test with
  `actor_known_projection_policy_table_drives_record_behavior`.
- Added `workplace_current_place_scope_drops_other_place_from_embodied_context` so
  the workplace `CurrentPlaceOnly` embodied-scope negative can fire.
- Added `supersede_newest_by_subject_requires_subject_extractor`; a synthetic
  non-workplace supersede policy now panics instead of silently dropping records.
- Added `ActorKnownProjectionRecord::kind()` and a narrow supersede subject extractor
  used by the `SupersedeNewestBySubject` branch.
- Registered the new policy behavioral locks with the meta-lock census and extended
  the anti-regression guard to keep the retired self-echo test name absent.

Verification run:

1. `cargo test -p tracewake-core epistemics`
2. `cargo test -p tracewake-core --test anti_regression_guards`
3. `grep -R -c "actor_known_projection_records_dispatch_to_declared_policy_table" crates/tracewake-core/src/epistemics/projection.rs crates/tracewake-core/tests/anti_regression_guards.rs` (printed zero counts; grep exits `1` on zero matches)
4. `cargo fmt --all`
5. `cargo test -p tracewake-core epistemics`
6. `cargo test -p tracewake-core --test anti_regression_guards`
7. `cargo fmt --all --check`
8. `cargo clippy --workspace --all-targets -- -D warnings`
9. `cargo build --workspace --all-targets --locked`
10. `cargo test --workspace`
