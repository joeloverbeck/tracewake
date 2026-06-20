# 0043ORDLIFCER-001: Converge the in-diff mutation trigger on the standing perimeter and make the guard drift-proof

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — CI workflow (`.github/workflows/ci.yml`) and the core CI guard test (`crates/tracewake-core/tests/ci_workflow_guards.rs`); no production/simulation logic change.
**Deps**: None

## Problem

The standing mutation perimeter (`.cargo/mutants.toml`) was widened by spec 0042 to cover six ORD-LIFE paths — `crates/tracewake-core/src/need_accounting.rs`, `actions/registry.rs`, `actions/defs/need_events.rs`, `actions/defs/wait.rs`, `actions/defs/continue_routine.rs`, `actions/defs/movement.rs` — but the `mutants-in-diff` guarded-path trigger in `.github/workflows/ci.yml` was never updated to match. Edits to any of those six paths therefore do not fire the fast in-diff detector. This is the ORD-LIFE analogue of the 0041 in-diff trigger gap (spec §3.4, §4.6).

The root cause is structural: the CI guard test `crates/tracewake-core/tests/ci_workflow_guards.rs` carries two independent hand-maintained duplicates of one seam — `STANDING_MUTATION_PERIMETER` (checked against `.cargo/mutants.toml`) and `STANDING_MUTATION_TRIGGER_FRAGMENTS` (checked against `ci.yml`). 0042 widened the perimeter list without the trigger list (or the trigger itself) following, and the guard's containment-only check did not catch the divergence. The reassessment of spec 0043 surfaced this as findings M1 (the trigger fragment constant is a mandatory co-edit) and A1 (the guard must assert perimeter↔trigger consistency to prevent recurrence).

This ticket makes the minimal correction to the in-diff trigger AND closes the drift surface that let it open, so the durability gap cannot recur on the next perimeter change.

(spec §3.4, §4.6, §4.1 item 6; reassessment findings M1/A1)

## Assumption Reassessment (2026-06-20)

1. The in-diff guarded-path regex lives in `.github/workflows/ci.yml` at the `mutants-in-diff` job's "Check guarded-layer diff" step (`ci.yml:115`). Verified this session: the regex matches `agent/`, `scheduler.rs`, `projections.rs`, `actions/pipeline.rs`, `actions/defs/(eat|sleep|work)\.rs`, `events/`, `replay/`, `checksum.rs`, `state.rs`, `actions/(proposal|report)\.rs`, `view_models.rs`, `debug_capability.rs`, `controller.rs`, `debug_reports.rs`, `epistemics/`, `content/src/(manifest|load|schema|serialization|validate)\.rs`, `tui/src/(app|debug_panels|render|transcript)\.rs` — and omits all six 0042-added paths. The scheduled `mutants-lock-layer` job (`ci.yml:177`) already runs `cargo mutants --workspace --no-shuffle` (full perimeter) and needs no trigger change.
2. The guard test `crates/tracewake-core/tests/ci_workflow_guards.rs` defines `STANDING_MUTATION_PERIMETER` (`:16`) checked against `mutants.toml` and `STANDING_MUTATION_TRIGGER_FRAGMENTS` (`:47`) checked against `ci.yml`; `mutation_perimeter_errors` (`:307`) asserts each listed fragment is *contained* in the workflow (`:326-332`) — a one-directional containment check that does not enforce perimeter↔trigger agreement. Verified this session. `STANDING_MUTATION_TRIGGER_FRAGMENTS` currently lacks the six 0042 paths.
3. Cross-artifact boundary under audit: the three-way contract between `.cargo/mutants.toml` (the certifying denominator), the `ci.yml` `mutants-in-diff` trigger (the fast non-certifying detector), and `ci_workflow_guards.rs` (the regression guard that must keep the first two in agreement). The spec forbids shrinking the perimeter, laundering survivors into `.cargo/mutants-baseline-misses.txt`, and using `--no-config`/`--in-diff`/`-f` as the certifying denominator (spec §4.3, §4.6).
4. Invariant/rule motivation: INV-098 (feature acceptance is harsh — a runnable feature is done only when regression-tested) and the execution evidence/mutation-completion doctrine (`docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`). The in-diff lane is an explicitly non-certifying fast detector; it must cover the full standing seam so a regression on a configured path is caught early, and the guard must make that coverage self-enforcing. cargo-mutants warns `--in-diff` can miss coverage regressions outside changed production lines (spec §3.4) — so this lane never substitutes for the scheduled full run.
5. Enforcement surface: the `ci_workflow_guards.rs` regression guard is itself the fail-closed gate here. The change adds coverage to a CI detector and tightens its guard; it touches no simulation-world fail-closed validation, actor-knowledge filtering, or deterministic-replay surface, and introduces no epistemic-leakage or replay-nondeterminism path. The mutation denominator (`.cargo/mutants.toml`, 60 files / 2877 mutants at the authoring baseline) is verified unchanged — the perimeter is not shrunk.
6. Mismatch + correction (reassessment M1/A1): without the `STANDING_MUTATION_TRIGGER_FRAGMENTS` co-edit, an explicit six-fragment addition to `ci.yml` would pass the guard while leaving the corrected trigger unenforced (silent recurrence), and a preferred directory-level rewrite would instead fail the literal-fragment containment check until the constant is updated. Either way the constant is a mandatory co-edit, and the consistency assertion (A1) is required to prevent the next perimeter widening from re-opening the gap.

## Architecture Check

1. Correcting the trigger AND making the guard assert perimeter↔trigger consistency is cleaner than a one-time fragment patch: a patch fixes today's six paths but leaves the same hand-maintained-duplicate drift surface that produced the 0042 gap. Deriving the expected trigger coverage from the standing perimeter (or cross-checking the two lists) means a future perimeter widening that forgets the trigger fails the guard immediately, at the cheapest possible layer.
2. No backwards-compatibility aliasing/shims. The in-diff lane remains explicitly non-certifying; it is not made a substitute for the scheduled full configured run, and `.cargo/mutants.toml` is not shrunk to make the trigger expression simpler.

## Verification Layers

1. In-diff trigger covers the standing perimeter → codebase grep-proof: every `.cargo/mutants.toml` examined path (or its covering directory prefix) appears in the `ci.yml` `mutants-in-diff` guarded-path expression, including the six 0042-added paths.
2. Guard drift-proofness (INV-098 regression-tested) → core test: `ci_workflow_guards.rs` fails when the in-diff trigger omits any standing-perimeter path (add/extend a negative assertion that a perimeter path missing from the trigger is an error), proven by a masked-workflow assertion in the same style as the existing `masked_gate` check.
3. Perimeter not shrunk; baseline not laundered → codebase grep-proof: `.cargo/mutants.toml` still expands to the standing 60-file seam and `.cargo/mutants-baseline-misses.txt` remains empty (0 bytes).
4. Single-layer note: no replay/golden-fixture or actor-knowledge surface is touched, so no epistemic/replay layer mapping applies (item 5 above records why).

## What to Change

### 1. Extend the `ci.yml` in-diff guarded-path trigger

In `.github/workflows/ci.yml`, the `mutants-in-diff` job's "Check guarded-layer diff" step (`ci.yml:115`): extend the `git diff --name-only … | grep -E '…'` alternation so it covers all six omitted 0042 paths. Prefer a directory-level or consolidated expression where it exactly represents the standing seam and avoids future drift (e.g. broaden `actions/defs/(eat|sleep|work)\.rs` to cover `need_events|wait|continue_routine|movement` as well, and add `need_accounting\.rs` and `actions/registry\.rs`), provided the expression matches the standing perimeter exactly and adds no path outside it. Retain in-diff mutation artifacts on failure (the existing `if: always()`-style retention).

### 2. Co-edit the guard's trigger expectation and add a consistency assertion

In `crates/tracewake-core/tests/ci_workflow_guards.rs`: extend `STANDING_MUTATION_TRIGGER_FRAGMENTS` (`:47`) so it represents the corrected trigger exactly, then add an assertion that the in-diff trigger covers the standing perimeter — derive the expected trigger coverage from `STANDING_MUTATION_PERIMETER` (or cross-check the two lists) so a perimeter path with no covering trigger fragment is a guard failure. The existing containment check (`:326-332`) stays; the new direction (perimeter ⇒ trigger) is what prevents recurrence.

### 3. Verify the perimeter and baseline-miss posture are unchanged

Confirm `.cargo/mutants.toml` still expands to the standing 60-file seam (no shrinkage, no `exclude_globs`, no `#[mutants::skip]` laundering) and `.cargo/mutants-baseline-misses.txt` is still empty. Record the verification; no edit expected to either file.

## Files to Touch

- `.github/workflows/ci.yml` (modify)
- `crates/tracewake-core/tests/ci_workflow_guards.rs` (modify)
- `.cargo/mutants.toml` (modify — verify-only; no change expected unless the perimeter has drifted)
- `.cargo/mutants-baseline-misses.txt` (modify — verify-only; must remain empty)

## Out of Scope

- The full configured campaign run, completion proof, and triage register (→ -004).
- Diagnosing the PTY/wrapper non-completion or building the supervised launch (→ -003).
- Killing the three `need_accounting.rs` survivors (→ -002).
- Live ORD-LIFE re-proof and the replacement acceptance artifact (→ -005).
- Any shrinkage of `.cargo/mutants.toml` or addition of survivors to `.cargo/mutants-baseline-misses.txt` (forbidden — spec §4.6, §12).
- Changing the scheduled `mutants-lock-layer` job, which already runs the full `--workspace --no-shuffle` posture.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test ci_workflow_guards` passes with the corrected trigger and the extended `STANDING_MUTATION_TRIGGER_FRAGMENTS`.
2. New/extended guard assertion fails on a mutated workflow whose in-diff trigger omits a standing-perimeter path (masked-workflow assertion, in the style of the existing `masked_gate` check).
3. `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace --locked` all pass.

### Invariants

1. The `ci.yml` in-diff trigger covers every standing-perimeter path (the six 0042 paths included); the guard fails if it ever does not.
2. `.cargo/mutants.toml` is not shrunk and `.cargo/mutants-baseline-misses.txt` remains empty — the certifying denominator is preserved.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/ci_workflow_guards.rs` — extend `STANDING_MUTATION_TRIGGER_FRAGMENTS` to the corrected trigger; add the perimeter⇒trigger consistency assertion and a masked-workflow negative case proving an omitted path is caught.

### Commands

1. `cargo test --locked -p tracewake-core --test ci_workflow_guards`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --locked`
3. `grep -nE 'need_accounting|actions/registry|need_events|continue_routine|movement|defs/[^ ]*wait' .github/workflows/ci.yml` — confirm the six 0042 paths now appear in the in-diff trigger expression.

## Outcome

Completed: 2026-06-20

Implemented the in-diff trigger convergence for the standing ORD-LIFE mutation
perimeter. The `mutants-in-diff` guarded-path regex now covers
`need_accounting.rs`, `actions/registry.rs`, and the `need_events`, `wait`,
`continue_routine`, and `movement` action definitions alongside the existing
ordinary-life action definitions.

Tightened the CI workflow guard so `STANDING_MUTATION_PERIMETER` matches the
checked-in `.cargo/mutants.toml` standing seam and every perimeter path must map
to a covering in-diff trigger fragment. Added a synthetic negative case proving
that removing a standing path from the workflow trigger fails the guard. Updated
the existing anti-regression synthetic in-diff cases to use the widened
action-definition trigger expression.

Verification:

- `cargo test --locked -p tracewake-core --test ci_workflow_guards` — passed.
- `cargo fmt --all --check` — passed after applying rustfmt to the new test.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo test --workspace --locked` — passed.
- `grep -nE 'need_accounting|actions/registry|need_events|continue_routine|movement|defs/[^ ]*wait' .github/workflows/ci.yml` — confirmed the six 0042 paths in the in-diff trigger expression.
- `rg -n 'need_accounting|actions/registry|need_events|continue_routine|movement|wait' .cargo/mutants.toml` — confirmed the standing perimeter entries remain present.
- `wc -c .cargo/mutants-baseline-misses.txt` — confirmed `0` bytes; no survivor laundering.
