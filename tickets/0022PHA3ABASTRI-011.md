# 0022PHA3ABASTRI-011: Canonical no-human-day runner-only evidence and recovery proof

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — content fixtures (`no_human_day_001.rs`), golden-fixture tests (`golden_fixtures_run.rs`)
**Deps**: 0022PHA3ABASTRI-004

## Problem

`ORD-HARD-111`: the canonical no-human-day completion evidence is hand-built —
`no_human_day_fixture_has_roster_activity_and_metrics_envelope` runs
`run_no_human_day`, then injects hand-built proposals (`proposal_day_tomas_work`, the
Elena sleep analog) and calls the completion builders directly before asserting
`WorkBlockCompleted`/`SleepCompleted`/`FoodConsumed` presence — the assertions pass
whether or not the runner produced them autonomously (docs/2-execution/09: a gate's
evidence must be produced by the path under test). `ORD-HARD-120`: 0005 §12's
recommended recovery variant (Mara replans to reachable food and recovers) is proven
only in the capstone's hand-built world; on the canonical day recovery is dynamically
*reachable* (blanket knowledge seeding makes `food_stew_home_tomas` Mara-known) but no
test asserts an autonomous recovery.

## Assumption Reassessment (2026-06-12)

1. Verified at `9ce820f`: the manual injection (`proposal_day_tomas_work`) in
   `crates/tracewake-content/tests/golden_fixtures_run.rs`;
   `crates/tracewake-content/src/fixtures/no_human_day_001.rs` seeds Mara at
   `home_mara` with `food_empty_pantry_mara` (0 servings) and `food_stew_home_tomas`
   (2 servings at `home_tomas`), hunger 900, routine window 4–10, and calls
   `populate_known_food_sources_for_all_actors()` under the legacy `#[expect]`
   allowlist — so the stew is Mara-known and remembered-food planning
   (`ORD-HARD-057`/`075`, verified holding) makes recovery plannable.
2. Verified against `specs/0022_…HARDENING_SPEC.md` §4 `ORD-HARD-111`
   (operator-verified) and `ORD-HARD-120` (operator-verified at reassessment;
   evidence reshaped — the "can only fail" sub-claim was refuted, the missing
   *assertion* confirmed).
3. Shared contract under audit: the canonical-fixture acceptance surface — 0005 §10.4
   ("at least one actor completes or fails a work block"; "actors return/rest/sleep")
   and exec doc 06's no-human proof, evidenced by runner-produced events only.
4. Constitutional invariants restated: INV-004/INV-091 (the world runs coherently
   with no human; no-human tests are mandatory) — evidence injected by the harness is
   not the world running.
5. Deterministic-replay / no-human surface: fixture window/duration retuning (if
   needed) changes the canonical day's event content — golden assertions are
   re-derived from real runs (one batch, coordinated with `0022PHA3ABASTRI-007`'s
   repricing); the runner-only ancestry assertions strengthen the INV-004 gate; no
   epistemic surface is touched (the blanket seeding stays as-is under its allowlist;
   per-actor seeding migration remains future cleanup owned by the `ORD-HARD-093`
   census conventions).
6. Either-or resolutions (recorded at Step-4 approval): **Q2 default** — author the
   canonical fixture's windows/durations so the runner closes at least one work and
   one sleep block unaided, with runner-only assertions requiring
   `SchedulePhase::NoHumanProcess` ancestry *before* any manual injection (preferred
   over splitting the test, since the spec's structural lock names the runner-only
   ancestry assertion). **Q3 (choice-and-record)** — attempt the autonomous-Mara-
   recovery assertion first; if the day's dynamics cannot produce it without
   distorting the canonical fixture's intent, record the fail-only canonical intent
   explicitly instead (both options sanctioned by the spec's lock; the chosen
   resolution is recorded in the acceptance artifact §7 item 9 and in this ticket's
   implementation notes).
7. Change rationale (no silent retcon): the hand-injection pattern predates the
   evidence-provenance discipline (R-27 family); the test's assertions are corrected
   because they rewarded harness-produced evidence (Enforcement reading).

## Architecture Check

1. Authoring the canonical day so the runner itself closes duration blocks — then
   asserting `NoHumanProcess` ancestry on those events — makes the canonical fixture
   the actual no-human proof, with the hand-driven payload-shape coverage retained
   afterward as a separately-labeled section. This beats a test split (two fixtures
   to keep canonical) and beats deleting the manual section (losing payload-shape
   coverage).
2. No backwards-compatibility aliasing/shims; fixture changes are authored honestly,
   not tuned to make assertions pass without the behavior (floors re-derived from
   real runs).

## Verification Layers

1. Evidence provenance (exec 09) -> replay/golden check: runner-only assertions —
   before any manual injection, the canonical day's log contains ≥1
   `WorkBlockCompleted` and ≥1 `SleepCompleted` with `NoHumanProcess` ancestry.
2. Recovery variant (0005 §12) -> behavior assertion: an autonomous Mara
   `FoodConsumed` (recovery) on the canonical day, OR the recorded fail-only intent
   (Q3 resolution) — whichever lands is asserted/recorded explicitly.
3. INV-004/091 -> no-human gate: the canonical-day test's runner-only section runs
   with zero harness-built proposals; manual-injection section clearly labeled as
   payload-shape coverage.
4. Census registration (§5.1) -> the runner-only assertions register with the `-004`
   bijection census (negative: stripping the ancestry requirement must fail).

## What to Change

### 1. Author the canonical day for autonomous closure

Adjust `no_human_day_001` windows/durations (e.g. Tomas's work block and Elena's
sleep block scheduled to complete within the day) so `run_no_human_day` closes at
least one work and one sleep block unaided. Keep the roster, places, and tension
seeds intact — the fixture stays canonical.

### 2. Runner-only ancestry assertions

In `no_human_day_fixture_has_roster_activity_and_metrics_envelope` (or a sibling
test): assert the runner-produced log carries the completions with
`SchedulePhase::NoHumanProcess` ancestry before any manual proposal injection;
relabel the manual section as payload-shape coverage.

### 3. Recovery assertion (Q3 resolution)

Attempt: assert an autonomous Mara `FoodConsumed` after her food-unavailable
interruption (the stew at `home_tomas` is known and reachable). If dynamics cannot
produce it without distorting the fixture, record the fail-only canonical intent in
the fixture contract prose and this ticket's notes, and assert the failure variant
explicitly instead.

## Files to Touch

- `crates/tracewake-content/src/fixtures/no_human_day_001.rs` (modify)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — census
  registration)

## Out of Scope

- The capstone synthetic-world recovery proof (`no_human_capstone.rs` — already
  proves the mechanism; untouched).
- Migrating `no_human_day_001` off the blanket knowledge seeding (legacy-allowlisted;
  future cleanup under the `ORD-HARD-093` conventions).
- The eat-crossing repricing (`0022PHA3ABASTRI-007`) — coordinate golden re-derivation
  ordering at implementation.

## Acceptance Criteria

### Tests That Must Pass

1. Runner-only section: canonical-day log contains ≥1 `WorkBlockCompleted` and ≥1
   `SleepCompleted` with `NoHumanProcess` ancestry, asserted before any manual
   injection; stripping the ancestry requirement fails the census negative.
2. The Q3 resolution is asserted: autonomous Mara recovery (`FoodConsumed` with
   `NoHumanProcess` ancestry) OR the explicitly-recorded fail-only intent with its
   failure variant asserted.
3. `cargo test -p tracewake-content` and
   `cargo test -p tracewake-core --test anti_regression_guards` green.
4. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. Canonical-day acceptance evidence is produced by `run_no_human_day`, never by
   harness-built proposals (exec 09; INV-004/091).
2. The fixture remains canonical: roster, tension seeds, and authored possibility
   space unchanged except window/duration authoring.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/tests/golden_fixtures_run.rs` — runner-only ancestry
   assertions + relabeled manual section + recovery (or fail-intent) assertion.
2. `crates/tracewake-core/tests/anti_regression_guards.rs` — census entry + negative.

### Commands

1. `cargo test -p tracewake-content --test golden_fixtures_run`
2. `cargo test -p tracewake-core --test anti_regression_guards`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
