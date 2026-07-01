# 0063CORACTKNO-004: Content fixtures for observed co-present activity

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — adds golden fixtures to `tracewake-content` exercising the observed-activity cases; registers them in `fixtures/mod.rs`
**Deps**: 0063CORACTKNO-002

## Problem

The observed-activity firewall is only credible if it is exercised by typed fixtures covering the
full case matrix (Spec 0063 §1.6, §6): a co-present actor visibly sleeping/eating/working, a
present-but-activity-unknown actor, a hidden/occluded actor (no row), and a stale-remembered
activity. These fixtures are the shared scenario substrate the anti-leak / occlusion / stale /
possession-parity assertions (ticket 005) run against, and the golden-replay corpus that proves
the pipeline is deterministic. Fixtures are **possibility, not script** — they seed world truth
and let the perception/projection pipeline (ticket 002) produce (or withhold) the activity; they
never author an outcome (§9 risk 2; INV-060/061/062).

## Assumption Reassessment (2026-07-01)

1. Content fixtures are Rust functions returning `GoldenFixture`, one per file under
   `crates/tracewake-content/src/fixtures/*.rs`, registered by `mod <name>;` in
   `crates/tracewake-content/src/fixtures/mod.rs` (exemplar:
   `embodied_view_omits_unobserved_food_at_open_place_001.rs` — a `FixtureSchema` with `actors`,
   `places`, `initial_needs`, `routine_templates`/`routine_assignments`, `day_windows`, closed with
   `.canonicalize()`). A co-present-activity scenario seeds ≥2 actors in one place (an observer +
   an observed actor whose routine/need state puts it in the activity) so the observed actor's
   activity is real world truth that perception may or may not capture.
2. Spec 0063 §1.6 enumerates the required cases: visible sleeping/eating/working, present-but-unknown,
   hidden/occluded (absent), stale-remembered. §6 additionally uses one fixture under two possessions
   for possession parity — that is a ticket-005 assertion reusing these fixtures, not a new fixture.
   §9.1 #3: no identity-uncertain "someone nearby" fixture (deferred).
3. **Shared boundary under audit:** the golden-fixture contract (`GoldenFixture` / `FixtureSchema`
   via `crate::fixtures::*`) consumed by ticket 005's core anti-leak/parity tests and the replay
   acceptance corpus. No `FixtureSchema` shape change is needed — observed activity is expressed
   through existing fields (routines, needs, place co-location, doors for occlusion), so these are
   schema *instances*, not a schema extension.
4. **Motivating invariants.** INV-060/061/062 (no authored outcome chains; authored causal machinery
   is possibility, not script): the fixtures seed initial conditions only; the activity row is
   produced by the modeled pipeline, never authored. INV-093 (actor-knowledge leakage is a
   high-severity defect, tested): the occlusion and present-but-unknown fixtures are the negative
   substrate proving the firewall.
5. **Enforcement surface (substrate basis).** These fixtures feed a deferred enforcement surface —
   ticket 005's anti-leak/possession-parity assertions and the deterministic-replay corpus. Confirm
   each fixture introduces no leakage or nondeterminism path the later surface would have to undo:
   the occluded/unknown fixtures seed *no* perceptible activity cue (so a correct pipeline yields no
   activity row), and all fixtures `.canonicalize()` for byte-stable replay. No fixture pre-seeds a
   belief that would fake an observation (contrast the "no scripting" discipline of
   `embodied_view_omits_*` siblings).

## Architecture Check

1. Authoring one fixture per case (mirroring the existing `embodied_view_omits_*` naming) keeps each
   scenario a small, independently-reviewable golden and reuses the established `FixtureSchema` +
   `.canonicalize()` plumbing, rather than one mega-fixture whose failures are hard to localize. The
   occlusion case reuses the existing door/closed-place machinery so "no modeled cue → no row" is a
   real physical fact, not an authored suppression.
2. No backwards-compatibility aliasing/shims: new fixtures are additive files + `mod` lines; no
   existing fixture or the `FixtureSchema` contract changes.

## Verification Layers

1. INV-060/061/062 (possibility, not script) -> schema validation: each fixture passes content
   validation (`cargo test -p tracewake-content`) with no behavior-looking field; the activity is
   absent from the seed and only appears via the pipeline.
2. INV-093 (leakage tested) -> replay/golden-fixture check (asserted in ticket 005): the
   occluded/present-but-unknown fixtures produce no activity row; the visible fixtures produce a
   typed row with source + freshness.
3. Determinism (INV-018) -> replay/golden-fixture check: `.canonicalize()` + `cargo test
   -p tracewake-content` fixtures-load census keeps the corpus byte-stable.

## What to Change

### 1. Add the observed-activity fixtures

Under `crates/tracewake-content/src/fixtures/`, add (names mirror the sibling convention;
`_001` suffix):

- `co_present_actor_visible_working_001.rs` (and/or sleeping/eating) — observer + observed actor
  co-located; observed actor in a working/sleeping/eating routine state; expect a typed activity row.
- `co_present_actor_activity_not_apparent_001.rs` — co-located but no perceptible activity cue;
  expect a present-but-`ActivityNotApparent` row.
- `co_present_actor_occluded_no_row_001.rs` — observed actor behind a closed door / separate place
  with no modeled cue; expect no row (omission).
- `co_present_actor_stale_remembered_activity_001.rs` — an activity observed earlier with no fresh
  observation; expect a stale-labeled row.

Each seeds actors/places/needs/routines, `.canonicalize()`s, and returns a `GoldenFixture`.

### 2. Register the fixtures

Add a `mod <name>;` line for each new fixture to `crates/tracewake-content/src/fixtures/mod.rs`
(alphabetically, matching the existing ordering), and expose it through whatever public
fixture-listing the module already provides for the sibling fixtures.

## Files to Touch

- `crates/tracewake-content/src/fixtures/co_present_actor_visible_working_001.rs` (new)
- `crates/tracewake-content/src/fixtures/co_present_actor_activity_not_apparent_001.rs` (new)
- `crates/tracewake-content/src/fixtures/co_present_actor_occluded_no_row_001.rs` (new)
- `crates/tracewake-content/src/fixtures/co_present_actor_stale_remembered_activity_001.rs` (new)
- `crates/tracewake-content/src/fixtures/mod.rs` (modify)

## Out of Scope

- The assertions over these fixtures (anti-leak, occlusion, stale, possession-parity, two-hop) —
  ticket 005.
- Any `FixtureSchema` contract extension (none needed).
- Identity-uncertain "someone nearby" fixtures (deferred, §9.1 #3).
- The acceptance artifact (ticket 006).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-content` — all new fixtures load, canonicalize, and pass content
   validation and the fixtures-load census (proves possibility-only, byte-stable).
2. Each of the four §1.6 cases is present as a distinct fixture (member checklist): visible-activity,
   present-but-`ActivityNotApparent`, occluded-no-row, stale-remembered — verified by the mod.rs
   registration and a fixture-listing test.
3. `cargo build -p tracewake-content --all-targets --locked` — the crate builds with the new
   fixtures registered.

### Invariants

1. No fixture authors an activity outcome or pre-seeds a belief that fakes an observation; the
   activity is produced only by the pipeline (INV-060/061/062).
2. Every fixture `.canonicalize()`s and is deterministic under the fixtures-load census (INV-018).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/src/fixtures/mod.rs` (or the crate's fixture-listing test) — the four
   new fixtures are registered and enumerable.
2. Existing `cargo test -p tracewake-content` validation/census suite — extended coverage via the
   new fixtures; no new bespoke test harness (the census + validation already gate content).

### Commands

1. `cargo test -p tracewake-content`
2. `cargo build -p tracewake-content --all-targets --locked`
3. The content-crate boundary is correct: these are content artifacts; their behavioral assertions
   run in `tracewake-core`/`tracewake-tui` (ticket 005) and are consolidated by the capstone (006).

## Outcome

Completed: 2026-07-01

Added and registered four Phase3A historical golden fixtures covering the ticket matrix:
`co_present_actor_visible_working_001`, `co_present_actor_activity_not_apparent_001`,
`co_present_actor_occluded_no_row_001`, and `co_present_actor_stale_remembered_activity_001`.
Each fixture seeds actors, places, needs, and only the modeled substrate needed for later
actor-known observation/projection assertions, then canonicalizes. The visible and stale cases use
ordinary work routine/workplace substrate; the apparent/occluded negative cases author no activity
cue or pre-seeded belief.

Updated the frozen fixture fingerprint table for the four new fixtures. Implementation note:
`FixtureSchema` and `ActorBody` do not currently contain an authoritative activity field, so these
fixtures intentionally remain possibility-only content substrates. Ticket 005 is responsible for
the behavior assertions that produce, withhold, or age actor-known activity rows from modeled
perception/projection evidence.

Verification:

1. `cargo fmt --all --check`
2. `cargo build -p tracewake-content --all-targets --locked`
3. `cargo test -p tracewake-content`
