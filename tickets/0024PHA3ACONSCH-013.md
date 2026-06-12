# 0024PHA3ACONSCH-013: 0005 coherence — candidate-priority decision and pin, disjoint day windows, capstone EatFailed

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` (`agent/candidate.rs`, `agent/decision.rs` tests, `scheduler.rs`), core tests (`no_human_capstone.rs`); possibly a conformance-row note (recording destination per the decision).
**Deps**: None

## Problem

Spec 0024 findings `ORD-HARD-161`, `162`, `163` (all low):

- `161`: `GoalPriority::selection_rank` ranks `UrgentHungerOrFatigue` (3) and
  `RoutineWindowDuty` (4) above `ActiveIntentionContinuation` (5), diverging from
  0005 §8.3's recommended order (active intention above urgent need and routine
  duty). The §8.3 hard floor (active intention beats *mild* pressure) holds and the
  foundation sanctions need interruptions, so behavior is constitution-defensible —
  but no test pins active-vs-urgent ordering and the divergence is recorded nowhere.
- `162`: `default_day_windows` emits inclusive-inclusive ranges sharing boundary
  ticks (4/10/18/24 each belong to two windows); today's consumers are idempotent so
  no double-charge occurs, but the overlap is a latent footgun for any future
  per-window counting consumer.
- `163`: `assert_required_acceptance_events` (the no-human capstone) lists
  `WorkBlockFailed`/`SleepCompleted`/`FoodConsumed`/`RoutineStepFailed` but not
  `EatFailed` — the single most-cited Phase 3A failure proof is not witnessed by the
  gate that bills itself as end-to-end, even though 0005 §10.4 places the
  food-unavailable chain inside `no_human_day_001`.

## Assumption Reassessment (2026-06-12)

1. Verified against code at baseline `4d62f61`: the `selection_rank` table
   (`agent/candidate.rs`); `default_day_windows`'s `(0,4),(4,10),(10,18),(18,24),(24,32)`
   offsets and `contains_tick`'s `start <= tick && tick <= end` (`scheduler.rs`);
   `EatFailed` absent from `assert_required_acceptance_events`
   (`no_human_capstone.rs`); the existing intention-durability tests cover only
   active-vs-mild.
2. Verified against archived 0005 §8.3 (recommended order + the hard floor
   sentence), `docs/0-foundation/05_*` (hunger/fatigue as legitimate interruptors),
   and spec 0024 §4 (`ORD-HARD-161`/`162`/`163`).
3. Cross-artifact boundary: the 0005 feature contract ↔ implementation coherence —
   recorded order, window semantics, and capstone evidence must agree with the spec
   lineage's record after this ticket.
4. Implementer-recorded choice (spec-assigned — `ORD-HARD-161`'s correction is "an
   owner decision: restore §8.3's order or record the chosen order"): the decision
   is made in this ticket and recorded. Options: (a) restore — re-rank
   `ActiveIntentionContinuation` above `UrgentHungerOrFatigue`/`RoutineWindowDuty`,
   repricing autonomous-day behavior (goldens/capstone metrics shift); (b) record —
   keep the current foundation-defensible order and record it as the decided order
   (conformance-index agent-cognition row or a 0005 outcome note), with the §8.3
   divergence named. Whichever pole is chosen, the recording destination is the
   conformance index row this batch's capstone (-014) lands — coordinate the row
   text; and the order is pinned by tests either way. The repricing cost of (a) and
   the foundation's sanction of need interruptions make (b) the default unless the
   implementer finds (a)'s behavior strictly better in the canonical day.
5. Enforcement surface (deterministic replay / canonical-day evidence): (b) changes
   no behavior; (a) reprices goldens once, honestly, within this ticket. The window
   disjointness fix must preserve current behavior — boundary ticks 4/10/18/24
   currently resolve via idempotent first-window crediting; making windows half-open
   (`start <= tick < end`-equivalent via end-offset-1) keeps every tick in exactly
   one window — verify capstone metrics and golden fixtures are unchanged, or
   record the honest repricing if a boundary tick's window attribution shifts.
6. Adjacent-contradiction classification: 0005 §8.3 is an archived spec's
   *recommendation* — this ticket does not edit the archive; the decision record
   lives in the live conformance surface (per item 4), which is the lineage's
   sanctioned mechanism for recording a divergence from an archived recommendation.

## Architecture Check

1. Pinning the priority table with a snapshot test plus an explicit
   active-vs-urgent behavioral test makes any future re-rank a visible,
   intentional edit regardless of which pole the decision takes — the cheapest
   durable fix for an unrecorded ordering. Half-open windows remove the dual
   membership at the type of the range rather than relying on every consumer
   staying idempotent. Adding `EatFailed` to the capstone's required list aligns
   the end-to-end gate with 0005 §10.4's stated obligation at one line of cost.
2. No backwards-compatibility aliasing/shims: one ordering, one window semantics;
   no transitional flags.

## Verification Layers

1. Order pin (`161`) → `selection_rank` snapshot test +
   `urgent_need_vs_active_intention_follows_documented_order` behavioral test in
   `decision.rs` (tests); decision recorded at the named destination (grep-proof,
   coordinated with -014's conformance row).
2. Window disjointness (`162`) → `default_day_windows_are_disjoint_and_cover` unit
   test; capstone metrics unchanged (or honest repricing recorded) (tests).
3. Capstone coverage (`163`) → `EatFailed` in `assert_required_acceptance_events`;
   the canonical no-human day passes with it required (replay/golden-fixture
   check).
4. Whole-pipeline → full workspace gates.

## What to Change

### 1. Priority decision + pin (`161`)

Make and record the order decision (Assumption item 4); add the snapshot and
behavioral tests pinning whichever order is decided.

### 2. Disjoint windows (`162`)

`default_day_windows` becomes disjoint (half-open or end-offset-1); add the
disjointness/coverage unit test; verify boundary-tick behavior per Assumption
item 5.

### 3. Capstone EatFailed (`163`)

Add `EventKind::EatFailed` to the capstone's required acceptance events.

## Files to Touch

- `crates/tracewake-core/src/agent/candidate.rs` (modify)
- `crates/tracewake-core/src/agent/decision.rs` (modify — tests)
- `crates/tracewake-core/src/scheduler.rs` (modify)
- `crates/tracewake-core/tests/no_human_capstone.rs` (modify)

## Out of Scope

- Editing archived 0005 (the divergence record lives in the live conformance
  surface).
- Changing need bands, thresholds, or interruption semantics beyond the ordering
  decision.
- The conformance-index row text itself (lands with -014; this ticket supplies the
  decided content).

## Acceptance Criteria

### Tests That Must Pass

1. The `selection_rank` snapshot test and the active-vs-urgent behavioral test pin
   the decided order; the decision and its rationale are recorded at the named
   destination.
2. `default_day_windows_are_disjoint_and_cover` passes; every tick 0..32 belongs to
   exactly one window; capstone metrics are unchanged (or the repricing is recorded
   honestly with the boundary-tick attribution named).
3. The capstone requires and finds `EatFailed`; the full canonical-day suite passes.
4. The four workspace gates pass.

### Invariants

1. The candidate-priority order is a recorded, test-pinned decision — never silent
   drift from the 0005 recommendation.
2. Day windows partition the day: no tick belongs to two windows.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/agent/candidate.rs` (inline) — `selection_rank`
   snapshot test.
2. `crates/tracewake-core/src/agent/decision.rs` (inline) —
   `urgent_need_vs_active_intention_follows_documented_order`.
3. `crates/tracewake-core/src/scheduler.rs` (inline) —
   `default_day_windows_are_disjoint_and_cover`.
4. `crates/tracewake-core/tests/no_human_capstone.rs` — `EatFailed` required-event
   addition.

### Commands

1. `cargo test -p tracewake-core agent:: scheduler:: --test no_human_capstone`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
