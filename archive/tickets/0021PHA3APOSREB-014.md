# 0021PHA3APOSREB-014: Kill the five new in-diff mutants on guarded-layer lines (mutation gate closure)

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: None — test-only (new unit tests in existing inline `#[cfg(test)]` modules); no production logic, schema, fixture, or doc changes
**Deps**: `archive/specs/0021_PHASE_3A_POSSESSION_REBIND_HYGIENE_GUARD_VACUITY_CLOSURE_HARNESS_PROVENANCE_FIDELITY_AND_REJECT_LOUDLY_REPLAY_POSTURE_HARDENING_SPEC.md`; the guarded-layer surfaces this PR changed (`crates/tracewake-core/src/scheduler.rs`, `.../agent/no_human_surface.rs`, `.../actions/defs/wait.rs`). Last merge blocker for PR #29.

## Problem

The PR-only CI job `mutation in-diff (lock layer)` (`.github/workflows/ci.yml:96`) runs
`cargo mutants --in-diff` over the guarded-layer lines this PR changed and ratchets the
result against `.cargo/mutants-baseline-misses.txt` (matching on file+mutation+function,
line:col stripped). Run `27379385281` reported **9 missed mutants; 4 match accepted
baseline entries, 5 are new** → `exit 1`. The five new misses sit on freshly-changed
guarded code and each encodes a real behavioral distinction no test currently asserts;
none is an equivalent mutant. Per `ORD-HARD-025` (archived spec 0016 §5.5),
*"missed mutants on guard-relevant code become explicit accepted-risk entries or new
tests"* — and §4.4, *"a guard that kills no mutants is theater."* Because these are
brand-new guarded lines, baselining them would accept risk on exactly the code the lock
layer exists to protect; the doctrine-aligned resolution is **new tests that kill each
mutant**, with no baseline expansion.

The five new misses (line:col from the failing run; the gate matches on function so the
tests, not the line numbers, are what must hold):

- O1 — `scheduler.rs:1502:53` and `:1503:17`, `no_human::build_window_passive_need_events`:
  `&&` → `||` on `field.key == "to_band" && field.value == "severe"` and on
  `(…any…) && has_active_intention`.
- O2 — `scheduler.rs:2056:42`, `no_human::advance_no_human`: `<` → `==` and `<` → `>` on
  the tick catch-up `while scheduler.current_tick < proposal.requested_tick`.
- O3 — `no_human_surface.rs:364:39`,
  `NoHumanActorKnownSurfaceBuilder::add_food_source_knowledge`: `==` → `!=` on the
  `policy.accessibility_scope == ActorKnownProjectionAccessibilityScope::FromAnyPlace`
  gate that decides whether the `food_source_believed_accessible` belief fact is pushed.
- O4 — `wait.rs:132:39`, `build_wait_events`: `==` → `!=` on
  `event.event_type == EventKind::NeedThresholdCrossed` (the `reevaluate` decision).

## Assumption Reassessment (2026-06-12)

1. Verified against the failing run (`gh run view 27379385281 --log-failed`) and the live
   sources at HEAD `ad39160`: all four sites exist as cited and each survived mutant is a
   genuine behavioral distinction (catch-up loop bound, belief-fact gate, threshold-band
   match, threshold-crossing predicate), not an equivalent mutant. The two `eat.rs`
   (`:209`, `:230`) and one `trace.rs` (`:397`) misses from the same run already match
   accepted baseline entries (`.cargo/mutants-baseline-misses.txt:149,153,135`, different
   line numbers — the intended line-shift tolerance) and are correctly NOT re-flagged;
   they are out of scope.
2. Verified against `.github/workflows/ci.yml:131-162` and `.cargo/mutants.toml`: the gate
   logic is correct (distinguishes tool failure from misses, requires `mutants.out`
   artifacts, applies the normalized `comm -23` ratchet). The failure is a true coverage
   gap, not a tooling defect — no `ci.yml` or `mutants.toml` change is in scope.
3. Shared contract under audit: the lock layer's effectiveness measure (`ORD-HARD-025`).
   The fix must strengthen it (kill the mutants via tests), never weaken it (no entries
   added to `.cargo/mutants-baseline-misses.txt`).
4. `ORD-HARD-025` doctrine restated: a guard that kills no mutants is theater; a new
   missed mutant on guarded code is resolved by a new test or an explicit accepted-risk
   baseline entry. This ticket takes the test path for all four findings.
5. Deterministic-replay / epistemic surfaces: the new tests are read-only assertions over
   existing deterministic construction paths (event payloads, projection facts); they add
   no production behavior and cannot alter replay or actor-knowledge filtering. O3 in
   particular pins an actor-belief fact (`food_source_believed_accessible`) to its
   accessibility-scope gate — asserting, not relaxing, the epistemic boundary.
6. The three target functions each already have an inline `#[cfg(test)] mod tests`
   (confirmed in `scheduler.rs`, `agent/no_human_surface.rs`, `actions/defs/wait.rs`);
   the helpers under test (`build_window_passive_need_events`, `advance_no_human`,
   `build_wait_events`) are module-private, so the killing tests belong inline, not in an
   integration test crate.

## Architecture Check

1. Function-local unit tests in the existing inline `#[cfg(test)]` modules are the
   tightest reviewable proof: each asserts the exact value the mutated operator controls,
   so the operator flip is observable and the mutant dies. A single ticket (one test diff)
   fits the four tightly-coupled sites better than four near-identical tickets.
2. No backwards-compatibility aliasing/shims; no production code touched; no baseline
   file edited.

## Verification Layers

1. `ORD-HARD-025` (lock-layer durability) -> `cargo mutants --in-diff` over the PR diff
   produces no new misses beyond `.cargo/mutants-baseline-misses.txt`; equivalently the
   CI `mutation in-diff (lock layer)` job goes green.
2. O1 behavioral distinction -> a `build_window_passive_need_events` test asserts the
   `severe_need_interrupts_active_intention` / `interruption_cause` payload is `true` /
   `severe_need_pressure` only for a `to_band == "severe"` crossing with an active
   intention, and `false` / `none` when either conjunct is absent (kills both `&&`→`||`).
3. O2 behavioral distinction -> an `advance_no_human` test with a proposal requested
   strictly more than one tick ahead asserts the scheduler advances tick-by-tick to the
   requested tick (intervening due-completions fire), which `<`→`==` and `<`→`>` both
   break.
4. O3 behavioral distinction -> an `add_food_source_knowledge` test asserts
   `food_source_believed_accessible` is present iff `accessibility_scope == FromAnyPlace`
   (kills `==`→`!=`).
5. O4 behavioral distinction -> a `build_wait_events` test asserts `reevaluate` (its
   observable payload/effect) is set iff a need-threshold crossing occurred among the
   hunger/fatigue events (kills `==`→`!=`).

## What to Change

### 1. O1 — `scheduler.rs` `no_human::build_window_passive_need_events`

Add an inline test that builds a passive-need window producing a `NeedThresholdCrossed`
event with `to_band == "severe"` and (a) an active intention, then (b) no active
intention and (c) a non-severe band, asserting the
`severe_need_interrupts_active_intention` and `interruption_cause` payload fields take the
expected values in each case. Both conjuncts (`key=="to_band" && value=="severe"`, and
`… && has_active_intention`) must be independently exercised so neither `&&`→`||` survives.

### 2. O2 — `scheduler.rs` `no_human::advance_no_human`

Add an inline test driving a no-human advance where a proposal's `requested_tick` is at
least two ticks beyond the scheduler's `current_tick`, asserting the catch-up loop
advances through every intervening tick (e.g. observed `current_tick` reaches
`requested_tick`, and tick-keyed due-completions land). This distinguishes `<` from both
`==` and `>`, which would skip the catch-up.

### 3. O3 — `agent/no_human_surface.rs` `NoHumanActorKnownSurfaceBuilder::add_food_source_knowledge`

Add an inline test that calls the builder with `accessibility_scope == FromAnyPlace` and
asserts the `food_source_believed_accessible` projection fact is present, and with a
non-`FromAnyPlace` scope asserts it is absent (while `actor_knows_food_source` is present
in both). Kills `==`→`!=`.

### 4. O4 — `actions/defs/wait.rs` `build_wait_events`

Add an inline test covering one wait whose hunger/fatigue deltas cross a need threshold
(emitting `NeedThresholdCrossed`) and one that does not, asserting the `reevaluate`-driven
observable (the resulting event payload/flag) differs accordingly. Kills `==`→`!=`.

Doctrine note: `wait` is a **non-perimeter** action def (ORD-HARD-118, staged spec 0022 §4
— `wait` is listed with `takeplace`/`continue_routine`/`movement` outside the reviewed
mutation perimeter, which is the duration defs `eat`/`sleep`/`work` plus
`scheduler*`/`agent/**`/`projections*`/`pipeline.rs`). The in-diff gate mutated `wait.rs`
only incidentally — `mutants.toml` `exclude_globs` does not yet list the non-perimeter
defs (that leak *is* ORD-HARD-118). This O4 test is legitimate hardening of real `wait`
threshold-crossing behavior and unblocks PR #29 now; once ORD-HARD-118 lands and excludes
`wait.rs` from mutation, the test simply stops being mutation-relevant (it still passes as
an ordinary unit test). This ticket deliberately does **not** touch `exclude_globs` — the
perimeter/exclusion correction is owned by spec 0022, not pulled forward here.

## Files to Touch

- `crates/tracewake-core/src/scheduler.rs` (modify — inline `#[cfg(test)] mod tests`: O1, O2)
- `crates/tracewake-core/src/agent/no_human_surface.rs` (modify — inline tests: O3)
- `crates/tracewake-core/src/actions/defs/wait.rs` (modify — inline tests: O4)

## Out of Scope

- Any edit to `.cargo/mutants-baseline-misses.txt` — the resolution is tests, not
  baseline expansion (would weaken the lock layer on new code; `ORD-HARD-025`).
- Any production-logic change to the four sites — the operators are correct; the gap is
  test coverage only.
- Any `ci.yml` / `mutants.toml` change. The `wait.rs` perimeter inconsistency (it is
  mutated by the in-diff gate but is a non-perimeter def absent from `exclude_globs` and
  the scheduled baseline `-f` set) is already captured as **ORD-HARD-118** in staged spec
  `specs/0022_…HARDENING_SPEC.md` §4 and is owned there — not re-ticketed and not
  pre-empted by this fix (see the O4 doctrine note above).
- The four already-baselined `eat.rs`/`trace.rs` misses — correctly not re-flagged.

## Acceptance Criteria

### Tests That Must Pass

1. New inline tests for O1–O4 pass and each fails if its targeted operator is flipped
   (mutation-killing — confirm by a local `cargo mutants --in-diff` over the PR diff
   showing the five signatures move from MISSED to caught, with zero new misses vs.
   `.cargo/mutants-baseline-misses.txt`).
2. `cargo test --locked -p tracewake-core` passes.
3. The four finished-tree gates pass:
   `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo build --workspace --all-targets --locked`, `cargo test --workspace`.
4. CI `mutation in-diff (lock layer)` job is green on the PR (the merge blocker clears).

### Invariants

1. `.cargo/mutants-baseline-misses.txt` is unchanged (no accepted-risk entries added).
2. No production behavior changes — replay determinism and actor-knowledge filtering are
   untouched; the new tests only assert existing construction paths.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/scheduler.rs` (inline `mod tests`) — kills the O1 `&&`→`||`
   pair and the O2 `<`→`==`/`<`→`>` pair.
2. `crates/tracewake-core/src/agent/no_human_surface.rs` (inline `mod tests`) — kills the
   O3 `==`→`!=`.
3. `crates/tracewake-core/src/actions/defs/wait.rs` (inline `mod tests`) — kills the O4
   `==`→`!=`.

### Commands

1. `cargo test --locked -p tracewake-core`
2. `git diff origin/main...HEAD > /tmp/guarded.diff && cargo mutants --in-diff /tmp/guarded.diff --no-shuffle` (expect the five signatures caught; no new misses vs. baseline)
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-12

What changed:

- Added inline scheduler tests for O1 and O2:
  `severe_passive_need_interrupt_payload_requires_severe_crossing_and_active_intention`
  asserts the severe-threshold/active-intention conjunction and payload cause fields;
  `advance_catches_up_intervening_ticks_before_later_proposal` asserts due duration
  completions are appended before later requested-tick proposals.
- Added inline no-human surface test
  `food_source_accessible_fact_requires_from_any_place_scope`, proving
  `food_source_believed_accessible` is emitted only for
  `ActorKnownProjectionAccessibilityScope::FromAnyPlace` while
  `actor_knows_food_source` remains present for both scopes.
- Added inline wait test
  `wait_without_threshold_crossing_keeps_reevaluation_flag_false`, proving the
  `candidate_goal_reevaluation` flag remains false without a
  `NeedThresholdCrossed` event.
- No production logic, schema, fixture, CI, mutation config, or mutation baseline file was
  changed.

Deviations from original plan:

- The literal mutation command
  `git diff origin/main...HEAD > /tmp/guarded.diff && cargo mutants --in-diff /tmp/guarded.diff --no-shuffle`
  failed before mutation because the three-dot diff line numbers did not match the edited
  working tree. The source-matching equivalent
  `git diff origin/main > /tmp/guarded.diff && cargo mutants --in-diff /tmp/guarded.diff --no-shuffle`
  was run instead.

Verification results:

- `cargo test --locked -p tracewake-core` — passed.
- `git diff origin/main > /tmp/guarded.diff && cargo mutants --in-diff /tmp/guarded.diff --no-shuffle`
  — completed with 116 mutants tested: 73 caught, 39 unviable, 4 missed. The O1, O2, O3,
  and O4 target signatures were present in `mutants.out/caught.txt`; the four remaining
  misses were existing accepted baseline entries after line/column normalization, and
  `comm -23` against `.cargo/mutants-baseline-misses.txt` printed no new misses.
- `cargo fmt --all --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo test --workspace` — passed.

Dropped/deferred enumerated members:

- None.
