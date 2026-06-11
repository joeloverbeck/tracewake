# 0021PHA3APOSREB-005: Behavioral per-kind policy dispatch and sleep-accessibility parity

**Status**: ✅ COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` (`epistemics/projection`, `agent/no_human_surface`, `agent/perception`); conformance-index row; possible embodied-context/golden repricing in `tracewake-content` as surfaced
**Deps**: `specs/0021_PHASE_3A_POSSESSION_REBIND_HYGIENE_GUARD_VACUITY_CLOSURE_HARNESS_PROVENANCE_FIDELITY_AND_REJECT_LOUDLY_REPLAY_POSTURE_HARDENING_SPEC.md` (ORD-HARD-074, ORD-HARD-075)

## Problem

The per-kind supersession policy table that 0020's correction promised as the
structural lock is decorative: `ActorKnownProjectionRecord::policy()` has zero
callers, `ActorKnownProjectionPolicy::CurrentPlaceLatestOnly` is a dead variant, and
`classified_actor_known_records_for_context` hardcodes
`match record { Workplace => supersede, _ => pass }` — while the embodied path
(`current_place_knowledge_context`) silently implements exactly the undeclared
`CurrentPlaceLatestOnly` policy for non-workplace kinds. Behavior can drift while the
table-contents test stays green — guard vacuity (`ORD-HARD-074`). On the same surface,
the `ORD-HARD-057` asymmetry survives one fact over: `sleep_place_believed_accessible`
is minted only at the actor's current place while food's accessibility fact is minted
from anywhere, so a remembered sleep place is planner-unreachable from elsewhere
(`ORD-HARD-075`; INV-028 in spirit; R-28 incomplete correction closure). Also folded
in per the spec: `actor_knows_sleep_place` is stamped `remembered_belief` even when
currently perceived — an unexplained per-fact divergence the policy table must
declare.

## Assumption Reassessment (2026-06-11)

1. Verified at `main` `89059a5`: `policy()` has zero callers
   (`epistemics/projection.rs`); `CurrentPlaceLatestOnly` appears only at its
   definition; the classifier hardcodes the Workplace-only supersession;
   `current_place_knowledge_context` drops non-workplace records failing
   `is_latest_current_place_record()`; `add_sleep_place_knowledge`
   (`no_human_surface.rs`) gates `sleep_place_believed_accessible` on
   `place_id == self.current_place_id` while `add_food_source_knowledge` does not;
   `routine_sleep_night` requires `SleepPlaceBelievedAccessible` and its first step is
   `move_toward_place`. Record kinds enumerated: exactly four — `Route`, `FoodSource`,
   `SleepPlace`, `Workplace`.
2. Verified against spec 0021 (reassessed 2026-06-11; operator-verified): findings
   ORD-HARD-074/075; §6 conformance row for behavioral policy dispatch; §9 notes any
   embodied-context hash churn gets the per-actor ledger treatment.
3. Cross-artifact boundary under audit: the projection-records → actor-known-facts
   contract spanning the shared classifier and its two consuming surfaces — one
   declared policy per record kind (supersession, freshness stamping, accessibility
   gating), dispatched from the table, identical on both surfaces or declared
   different.
4. INV-028 restated: staleness reclassifies knowledge, it does not delete or suppress
   it — and the accessibility fact is the actionable form of the knowledge, so
   place-gating it suppresses remembered knowledge for planning. INV-026: provenance
   over incidental structure. INV-108: embodied and autonomous views of one
   projection must not diverge in epistemic semantics except by declared policy.
5. Actor-knowledge-filtering surface touched: dispatching from the table and
   un-gating sleep accessibility change which facts enter actor-known contexts —
   embodied-context hashes and golden expectations may reprice (one batched
   repricing with per-actor ledger diffs). Direction is strictly
   widening-toward-doctrine (remembered knowledge becomes plannable; no hidden-truth
   path is added — all minted facts remain projection-derived). Replay-deterministic
   by construction (no new ordering or randomness).
6. Schema shape: no serialized-schema extension. `ActorKnownProjectionPolicy` is an
   internal enum — implementing or deleting `CurrentPlaceLatestOnly` (and adding an
   accessibility-gating axis to the table) changes no event payload, fixture format,
   or checksum family. Additive-vs-breaking N/A (internal dispatch only).

## Architecture Check

1. Making the classifier and both surfaces dispatch on `record.policy()` turns the
   table from documentation into the single behavioral authority — a per-kind change
   then happens in exactly one place, and a third consuming surface inherits correct
   policy by construction (the family-closure direction 0020 established). The
   embodied path's current-place affordance semantics, if intentional, become that
   kind's *declared* embodied policy rather than silent divergence. The
   sleep-vs-food accessibility decision is a spec-assigned implementer-recorded
   choice (mirror food's from-anywhere treatment, or record an explicit per-kind
   deferral with rationale in the table + conformance row); the recorded choice and
   grounds land in this ticket's closure note and the acceptance artifact.
2. No backwards-compatibility aliasing/shims: the hardcoded match is replaced by
   table dispatch, not kept as a fallback; the dead variant is implemented or
   deleted, not retained as decoration.

## Verification Layers

1. Guard de-vacuity (ORD-HARD-074) -> behavioral per-kind tests driven *from* the
   table: iterate `actor_known_projection_policy_kinds()` and assert observed
   supersession/reclassification/gating per kind on BOTH surfaces; source guard that
   `policy()` has ≥1 production caller.
2. INV-028 (sleep parity, ORD-HARD-075) -> the sleep mirror of
   `food_record_from_other_place_surfaces_as_remembered_find_food_input`: a
   remembered sleep place at place A is planner-reachable for `routine_sleep_night`
   after moving to B (or the registered deferral, per the recorded choice).
3. INV-108 (two-surfaces-one-policy) -> parity assertion comparing per-kind fact
   sets across `NoHumanActorKnownSurfaceBuilder` and
   `current_place_knowledge_context` for the same actor/tick/projection, modulo
   declared embodied-policy differences.
4. Repricing honesty -> per-actor ledger diff for any golden/context-hash churn,
   recorded in the acceptance artifact (spec §9).

## What to Change

### 1. Behavioral table dispatch (ORD-HARD-074)

`classified_actor_known_records_for_context` (and the embodied context's per-kind
gating) dispatch on `record.policy()`; implement `CurrentPlaceLatestOnly` as the
declared embodied policy for the kinds that genuinely carry it, or delete it; extend
the table to declare accessibility gating and freshness stamping per kind (resolving
the `actor_knows_sleep_place` hardcoded `remembered_belief` divergence in the table).

### 2. Sleep-accessibility parity (ORD-HARD-075)

Apply the recorded choice: mirror food's from-anywhere accessibility minting for
sleep places, or register the explicit per-kind deferral. Either way the table
declares it and the test pins it.

### 3. Locks and documentation

Behavioral per-kind tests (both surfaces), the `policy()`-has-callers source guard,
the parity assertion; conformance row for behavioral policy dispatch in
`docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`.

## Files to Touch

- `crates/tracewake-core/src/epistemics/projection.rs` (modify)
- `crates/tracewake-core/src/agent/no_human_surface.rs` (modify)
- `crates/tracewake-core/src/agent/perception.rs` (modify)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — `policy()` caller guard)
- `crates/tracewake-content/tests/` golden expectations (modify — as surfaced by repricing; parent dir exists)
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (modify)

## Out of Scope

- The fabricating planning-context builders (`archive/tickets/0021PHA3APOSREB-002.md`).
- The typed place-concealment attribute in perception (ticket 0021PHA3APOSREB-007) —
  coordinate the `perception.rs` mechanical merge.
- Embodied dead-surface sweep (ticket 0021PHA3APOSREB-010).

## Acceptance Criteria

### Tests That Must Pass

1. Per-kind behavioral tests, driven from the table, pass on both surfaces for all
   four kinds (`Route`, `FoodSource`, `SleepPlace`, `Workplace`).
2. The sleep-accessibility mirror test (or its registered-deferral assertion) passes
   per the recorded choice; `routine_sleep_night` planner-reachability from
   elsewhere is proven (or explicitly deferred with cite).
3. `policy()` caller guard passes (synthetic zero-caller case fails); the
   `CurrentPlaceLatestOnly` variant is either dispatched or absent.
4. Existing supersession/parity/tie-break tests from 0020 stay green; any golden
   repricing carries the per-actor ledger diff.
5. `cargo test --workspace` green.

### Invariants

1. Every record kind's supersession, freshness, and accessibility treatment is
   declared in the policy table and behaviorally proven on both consuming surfaces.
2. No record kind's accessibility fact is place-gated except by declared,
   test-pinned policy.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/epistemics/projection.rs` (mod tests) — table-driven
   per-kind behavioral tests.
2. `crates/tracewake-core/src/agent/no_human_surface.rs` (mod tests) — sleep
   remembered-accessibility mirror + builder-side per-kind assertions.
3. `crates/tracewake-core/src/agent/perception.rs` (mod tests) — embodied-side
   declared-policy assertions + parity test.
4. `crates/tracewake-core/tests/anti_regression_guards.rs` — `policy()` caller
   source guard.

### Commands

1. `cargo test -p tracewake-core projection`
2. `cargo test -p tracewake-core no_human`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-11

What changed:

- Replaced the decorative actor-known projection policy enum use with a per-kind policy table that declares classification, embodied inclusion scope, and accessibility-fact scope for all four record kinds (`Route`, `FoodSource`, `SleepPlace`, `Workplace`).
- Routed `classified_actor_known_records_for_context`, `NoHumanActorKnownSurfaceBuilder`, and `current_place_knowledge_context` through `ActorKnownProjectionRecord::policy()`.
- Removed the unused `CurrentPlaceLatestOnly` variant; the embodied latest-current-place behavior is now represented by the declared `ActorKnownProjectionEmbodiedScope::LatestCurrentPlaceOnly` axis.
- Applied the recorded sleep-accessibility choice: sleep places now mirror food-source accessibility from any remembered place on the no-human actor-known surface, and `actor_knows_sleep_place` uses projection freshness instead of being hardcoded as remembered.
- Added table-driven policy tests, no-human sleep reachability coverage, embodied/no-human declared-scope coverage, a source guard proving production `policy()` callers, and a conformance row in `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`.
- Updated the content replay tamper test to mutate decision-trace source evidence directly, keeping the test aligned with the now-broader remembered sleep-place facts.

Deviations from original plan:

- No content golden/context-hash baseline repricing was needed. The only content test change was a tamper-helper correction after the widened sleep fact surface made the old sleep-place-value mutation no longer prove the intended source-evidence mismatch.

Verification:

- `cargo test -p tracewake-core projection` — passed.
- `cargo test -p tracewake-core no_human` — passed.
- `cargo test -p tracewake-core current_place_knowledge_context` — passed.
- `cargo test -p tracewake-content --test golden_fixtures_run no_human_decision_context_hash_gate_fails_when_source_evidence_tampered` — passed.
- `cargo fmt --all --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo test --workspace` — passed.
