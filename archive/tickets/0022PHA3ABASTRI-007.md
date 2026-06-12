# 0022PHA3ABASTRI-007: Eat crossing emitter routing and derived guard perimeter

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — actions kernel (`actions/defs/eat.rs`), shared emitter consumption, test-oracle guard perimeter, golden-log repricing
**Deps**: 0022PHA3ABASTRI-004

## Problem

`ORD-HARD-107` — the spec's single product-behavior doctrine gap: `build_eat_events`
constructs `EventKind::NeedDeltaApplied` via its private `hunger_delta_event` instead
of `need_events::build_need_delta_and_threshold_events`, so a serving reduction
crossing a band boundary (e.g. 510→390, Urgent→Rising) is a real band change with no
`NeedThresholdCrossed` event (INV-009/012/013 at the letter). The shared-emitter
source guard's `GUARDED_PATHS` lists wait/sleep/work/scheduler only — eat.rs is
structurally exempt from the very lock (`ORD-HARD-076`) whose contract is "no builder
can apply a band-changing delta without the paired crossing event". The eat unit test
asserts exactly two events, baking the omission in.

## Assumption Reassessment (2026-06-12)

1. Verified at `9ce820f`: `hunger_delta_event` defined and called in
   `crates/tracewake-core/src/actions/defs/eat.rs` (no
   `build_need_delta_and_threshold_events` call — callers are sleep.rs/work.rs/wait.rs
   /scheduler.rs); `GUARDED_PATHS` in
   `crates/tracewake-core/tests/anti_regression_guards.rs` lists exactly
   `defs/wait.rs`, `defs/sleep.rs`, `defs/work.rs`, `scheduler.rs`; the shared emitter
   `build_need_delta_and_threshold_events` lives in
   `crates/tracewake-core/src/actions/defs/need_events.rs` and is direction-agnostic
   (verified holding, `ORD-HARD-076`).
2. Verified against `specs/0022_…HARDENING_SPEC.md` §4 `ORD-HARD-107`
   (operator-verified; severity medium per the `ORD-HARD-076` precedent) and §9's
   repricing risk: crossing events added to eating reprice golden logs — batch once,
   re-derive honestly (never adjust assertions to pass).
3. Shared contract under audit: the single-shared-emitter contract — every
   band-changing need delta pairs with its `NeedThresholdCrossed` — across all action
   defs and the scheduler.
4. Constitutional invariants restated: INV-009 (meaningful state changes require
   events), INV-012/013 (mental/threshold events are real and leave traces when they
   affect later reasoning) — a hunger band change from eating affects the next
   decision's band reads.
5. Deterministic-replay surface: new crossing events change future event-log *content*
   (and golden fixtures), not replay semantics; live and replay paths share the
   emitter, so determinism is preserved; repriced goldens are re-derived from real
   runs. No actor-knowledge surface is touched (crossings are the actor's own need
   events).
6. Schema note (additive-vs-breaking): no event-schema shape change —
   `NeedThresholdCrossed` and its payload already exist with apply arms and version
   gates (`ORD-HARD-069`/`070` census, verified holding); this ticket adds *emissions*
   on the eat path. N/A for consumer migration beyond golden repricing.
7. Change rationale (no silent retcon): the eat path's two-event shape was an
   `ORD-HARD-076` closure gap (R-28), not a design decision; the eat unit test is
   corrected because it rewarded the omission (Enforcement reading).

## Architecture Check

1. Routing eat's hunger delta through the shared emitter (which already handles both
   crossing directions and the paired-event contract) is strictly simpler than
   emitting a crossing locally — one emitter, one contract, no second implementation
   to drift. Deriving `GUARDED_PATHS` from "the set of action defs that construct
   `NeedDeltaApplied`" makes the perimeter self-extending: a future def cannot be born
   outside the lock.
2. No backwards-compatibility aliasing/shims: `hunger_delta_event` is deleted, not
   kept as a wrapper.

## Verification Layers

1. INV-009/012/013 -> behavior test: an eat that crosses a band boundary emits
   `NeedDeltaApplied` + `NeedThresholdCrossed` (both directions covered: a
   pressure-decreasing crossing from eating, and the upward case via an authored
   negative-effect food if representable — else the downward case plus the emitter's
   existing direction tests).
2. Lock perimeter (R-28) -> guard check: `GUARDED_PATHS` derived from the
   `NeedDeltaApplied`-constructing def set; synthetic negative — a def constructing
   `NeedDeltaApplied` outside the derived perimeter fails
   `guard_006_duration_need_deltas_route_through_shared_emitter`; registered with the
   `-004` census.
3. Replay determinism (INV-018) -> replay/golden-fixture check: repriced canonical
   days replay byte-identically; checksum totals re-derived from real runs.
4. Honest repricing -> manual review + two-sided floors (`-004`): any golden count
   change is recorded as a delta, not silently re-pinned.

## What to Change

### 1. Route eat through the shared emitter

In `build_eat_events`, replace `hunger_delta_event` with
`need_events::build_need_delta_and_threshold_events` (the builder has the actor's
current hunger available); delete `hunger_delta_event`.

### 2. Derive the guard perimeter

Replace the hardcoded `GUARDED_PATHS` array with a derivation: scan
`crates/tracewake-core/src/actions/defs/*.rs` + `scheduler.rs` for
`NeedDeltaApplied` construction and require shared-emitter routing for every hit;
synthetic out-of-perimeter negative; census registration.

### 3. Correct the eat tests and reprice goldens

Fix `eat_consumes_one_serving_and_emits_hunger_delta` (and siblings) to assert the
paired crossing when a boundary is crossed and its absence when not; add the
across-boundary eat test both directions where representable. Re-derive any golden
fixture/event-count assertions the new crossings reprice (content-crate golden runs,
EMERGE-OBS inputs are measured later by `-014`), in one batch, from real runs.

## Files to Touch

- `crates/tracewake-core/src/actions/defs/eat.rs` (modify)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)
- Golden fixtures/tests repriced by the new crossings (modify — an
  implementation-discovered set surfaced by running the suite; expected within
  `crates/tracewake-content/tests/` and `crates/tracewake-core/tests/`; parent
  directories verified)

## Out of Scope

- The reject-loudly scheduler conversions (`0022PHA3ABASTRI-008`).
- Emitter-internal changes (`build_need_delta_and_threshold_events` is verified
  correct; this ticket adds a caller).
- The canonical-day runner-evidence work (`0022PHA3ABASTRI-011`) — even though both
  touch content-crate goldens; coordinate ordering at implementation.

## Acceptance Criteria

### Tests That Must Pass

1. Across-boundary eat test: serving reduction crossing a band emits the paired
   `NeedThresholdCrossed` with correct from/to bands; non-crossing eat emits no
   crossing.
2. `cargo test -p tracewake-core --test anti_regression_guards` — derived perimeter
   green; the out-of-perimeter synthetic fires; `grep -c "hunger_delta_event" crates/`
   returns 0.
3. Full workspace suite green after one-batch golden repricing, with each repriced
   value re-derived from a real run (recorded in the implementation notes).
4. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. No action def or scheduler path constructs `NeedDeltaApplied` outside the shared
   emitter; the perimeter is derived, not hand-listed.
2. Every band-changing need delta pairs with a crossing event in both live and replay
   logs.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/actions/defs/eat.rs` (inline tests) — across-boundary
   and non-crossing cases.
2. `crates/tracewake-core/tests/anti_regression_guards.rs` — derived perimeter +
   synthetic negative.

### Commands

1. `cargo test -p tracewake-core eat`
2. `cargo test -p tracewake-core --test anti_regression_guards && cargo test -p tracewake-content`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-12

Eat hunger deltas now route through `need_events::build_need_delta_and_threshold_events`
with the current actor hunger from `AgentState`, so eating emits the existing
`NeedThresholdCrossed` event when the serving's hunger reduction crosses a need band.
The private direct constructor was deleted. `build_eat_events` now receives
`AgentState` from the shared action pipeline, while failure paths keep their previous
single-event behavior.

The shared-emitter anti-regression guard now derives its perimeter from action-def and
scheduler sources that reference need-delta emission, excluding only
`need_events.rs`; synthetic direct constructions in both `work.rs` and `eat.rs` prove
the derived guard fails for newly participating action defs. Eat tests now cover a
non-crossing reduction (`450 -> 330`, no crossing) and a crossing reduction
(`510 -> 390`, urgent -> rising).

Golden repricing was required in one TUI assertion: the real
`no_human_day_001` debug panel now reports `need_crossings=5` instead of `4`, derived
from `run no-human-day` / `debug no-human-day` against the actual fixture. No fixture
files or canonical logs were rewritten.

Verification:

1. `cargo test -p tracewake-core eat`
2. `cargo test -p tracewake-core --test anti_regression_guards`
3. `cargo test -p tracewake-content`
4. `grep -R -c "hunger_delta_event" crates` returned zero matches and exit 1,
   the expected grep behavior for no matches.
5. `cargo test -p tracewake-tui --test tui_acceptance tui_runs_no_human_day_and_inspects_real_post_run_panels`
6. `cargo fmt --all --check`
7. `cargo clippy --workspace --all-targets -- -D warnings`
8. `cargo build --workspace --all-targets --locked`
9. `cargo test --workspace`
