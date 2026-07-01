# 0063CORACTKNO-001: Closed observed-activity taxonomy enums

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — adds two closed enums to `tracewake-core` (`view_models`): `ObservedActorActivityKind`, `ActorKnownActivitySourceKind`
**Deps**: None

## Problem

Co-present actor activity must be presented as a bounded, inspectable taxonomy, never a
free-form string derived from prose or physical truth. Spec 0063 §1.2 and §4.4 require two
**closed** enums as the vocabulary for the whole feature: `ObservedActorActivityKind` (what an
actor is observed doing) and `ActorKnownActivitySourceKind` (through which modeled information
channel it was learned, mapped to the foundation-04 information sources). These are the
foundation types every downstream ticket (002 projection/sealing, 003 view-model transfer,
005 render disposition) consumes. Landing them first, standalone and closed, forces every later
consumer to dispose of each variant explicitly and prevents a wildcard/`Default` laundering path
(§4.4, §4.5).

## Assumption Reassessment (2026-07-01)

<!-- Items 1-3 always required. Items 4+ selected by scope and renumbered from 4. -->

1. Neither `ObservedActorActivityKind` nor `ActorKnownActivitySourceKind` exists yet: `grep -rn
   "ObservedActorActivityKind\|ActorKnownActivitySourceKind" crates/` returns zero definitions, so
   this is net-new, no collision. `VisibleActor` today is identity-only
   (`crates/tracewake-core/src/view_models.rs:390` — `pub struct VisibleActor { pub actor_id: ActorId }`).
2. Spec 0063 §1.2 fixes the initial variant sets: activity = `Sleeping, Eating, Working, Moving,
   Speaking, Waiting, ContinuingRoutine, ApparentIdle, ActivityNotApparent`; source =
   `DirectPerception, IndirectPerception, Memory, Testimony, Record, Inference`. §9 open question
   §9.1 #1 records the "start coarse" decision — additional domain subtypes are deferred, flavor
   is carried by an actor-safe summary string on `VisibleActor` (ticket 003), not by widening this
   enum.
3. **Shared boundary under audit:** this ticket defines the closed *taxonomy contract* consumed by
   the projection record + `ActorKnownLocalActorFact` sealing (002), the `VisibleActor` view (003),
   and the render disposition (005). Placement is `crates/tracewake-core/src/view_models.rs` because
   `crates/tracewake-core/src/epistemics/projection.rs:18` already does `use crate::view_models::{…}`
   while `view_models.rs` imports nothing from `epistemics` — so both the epistemic record layer and
   the view-model layer can reference these types with no module cycle. This matches the spec's §4.4
   seam ("closed types force explicit presentation — seam: `view_models.rs`").
4. **Motivating invariant — INV-022 (raw prose is not authoritative state).** Activity must be a
   typed, closed value, not a display/debug string; the enums are the mechanism that keeps observed
   activity from becoming a prose-born fact (§9 risk 2). The closed shape also serves INV-024/067 by
   giving downstream tickets no wildcard escape that could smuggle unfiltered state into presentation.

## Architecture Check

1. Defining the vocabulary once, as standalone closed enums with no `Default` and no catch-all,
   is cleaner than inlining string constants or per-consumer enums: it gives one authoritative
   variant set, makes exhaustiveness a compiler-enforced obligation at every future `match`, and
   keeps the "start coarse, extend deliberately" decision (§9.1 #1) in a single place.
2. No backwards-compatibility aliasing or shims: these are new types with no prior form to alias.
   Derives (`Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash`) mirror the sibling
   `ActorKnownProjectionSource` enum so the types slot into deterministically-ordered collections
   (needed for the deterministic `VisibleActor` ordering in ticket 003) without bespoke ordering.

## Verification Layers

1. INV-022 (typed, not prose) -> codebase grep-proof: both enums are defined, `pub`, carry no
   `Default` impl and no free-form `String` payload; a closedness unit test exhaustively matches
   every variant so adding a variant later fails to compile until handled.
2. Single-layer rationale -> this ticket adds only two leaf types with no behavior, no schema
   extension, and no actor-knowledge/replay surface touched, so no additional invariant-to-surface
   mapping applies; item 5/6/7 of the reassessment menu are not engaged (nothing sealed, projected,
   validated, renamed, or removed here). Those surfaces are exercised by tickets 002/003/005.

## What to Change

### 1. Add `ObservedActorActivityKind`

In `crates/tracewake-core/src/view_models.rs`, add a closed enum with the §1.2 variants
`Sleeping, Eating, Working, Moving, Speaking, Waiting, ContinuingRoutine, ApparentIdle,
ActivityNotApparent`, deriving `Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash`. No
`Default`, no catch-all, no string payload.

### 2. Add `ActorKnownActivitySourceKind`

In the same file, add a closed enum with the §1.2 variants `DirectPerception, IndirectPerception,
Memory, Testimony, Record, Inference`, same derives. Add a short doc-comment on each type noting
the foundation-04 information-source mapping and that the set is intentionally coarse (§9.1 #1),
extended only by a deliberate spec revision.

## Files to Touch

- `crates/tracewake-core/src/view_models.rs` (modify)

## Out of Scope

- Any field on `VisibleActor` carrying these enums (ticket 003).
- The projection record / `ActorKnownLocalActorFact` fields that seal activity (ticket 002).
- Render-side disposition of the variants (ticket 005).
- Any additional/domain-specific activity subtypes — deferred per §9.1 #1.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --lib view_models` — a new closedness unit test that exhaustively
   matches all nine `ObservedActorActivityKind` and all six `ActorKnownActivitySourceKind` variants
   compiles and passes (proves the sets are exactly §1.2's and are closed).
2. `cargo build -p tracewake-core --all-targets --locked` — the crate builds with the new types.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings` — no format
   or lint regressions.

### Invariants

1. Both enums are closed: no `Default` impl, no catch-all variant, no free-form `String` payload
   (INV-022 — activity is typed, not prose).
2. Both enums derive a total order and `Hash`, matching `ActorKnownProjectionSource`, so downstream
   deterministic ordering (ticket 003) needs no bespoke comparator.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/view_models.rs` (inline `#[cfg(test)]`) — a closedness/exhaustiveness
   test per enum, asserting the exact variant set and that an exhaustive match compiles.

### Commands

1. `cargo test -p tracewake-core --lib view_models`
2. `cargo build -p tracewake-core --all-targets --locked`
3. A crate-scoped test filter is the correct boundary because this ticket adds only leaf types to
   `tracewake-core` with no cross-crate consumer yet; workspace-wide tests are exercised by the
   consuming tickets (002/003/005) and the capstone (006).

## Outcome

Completed: 2026-07-01

Implemented the closed `ObservedActorActivityKind` and `ActorKnownActivitySourceKind` enums in
`crates/tracewake-core/src/view_models.rs`, with coarse Spec 0063 variant sets, no `Default` or
catch-all variant, and derive coverage for deterministic downstream ordering and hashing. Added an
exhaustive `view_models` unit test that enumerates every activity and source variant through
explicit matches.

Verification:

- `cargo test -p tracewake-core --lib view_models` — passed.
- `cargo build -p tracewake-core --all-targets --locked` — passed.
- `cargo fmt --all --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.

No deviations from the ticket scope. The projection, sealing, `VisibleActor` transfer, fixtures, and
render/disposition work remain for dependent tickets.
