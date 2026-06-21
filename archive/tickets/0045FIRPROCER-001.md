# 0045FIRPROCER-001: Converge the in-diff mutation trigger + CI guard on the standing FIRST-PROOF perimeter (drift-proof)

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — CI workflow (`.github/workflows/ci.yml`) and the core CI guard test (`crates/tracewake-core/tests/ci_workflow_guards.rs`); no production/simulation logic change.
**Deps**: None

## Problem

The standing mutation perimeter (`.cargo/mutants.toml`) already covers the two 0044 FIRST-PROOF additions — `crates/tracewake-core/src/time.rs` and `crates/tracewake-core/src/actions/defs/checkcontainer.rs` (both present in `examine_globs`) — but the fast `mutants-in-diff` guarded-path trigger in `.github/workflows/ci.yml` was never updated to match, and neither were the guard constants in `crates/tracewake-core/tests/ci_workflow_guards.rs`. Edits to either path therefore do not fire the fast in-diff detector, and the guard is internally consistent with stale data rather than derived from the configured perimeter — so it cannot catch the divergence. This is the FIRST-PROOF analogue of the in-diff trigger gaps closed for ORD-LIFE (0043-001) and EPI (spec §3.4, §9.6).

This ticket makes the minimal correction to the in-diff trigger AND closes the drift surface that let it open (the hand-maintained `STANDING_MUTATION_PERIMETER` / `STANDING_MUTATION_TRIGGER_FRAGMENTS` duplicate pair), so the durability gap cannot recur on the next perimeter change. It is explicitly *not* a change to the standing denominator: `.cargo/mutants.toml` already contains both carriers and must not be shrunk.

(spec §3.4, §4.8 items 1–4, §9.6; reassessment confirmed the trigger/guard omission of both paths)

## Assumption Reassessment (2026-06-21)

1. The in-diff guarded-path regex lives in `.github/workflows/ci.yml` at the `mutants-in-diff` job's guarded-diff step (`ci.yml:115`). Verified this session: the alternation matches `agent/`, `need_accounting\.rs`, `scheduler\.rs`, `projections\.rs`, `actions/pipeline\.rs`, `actions/registry\.rs`, `actions/defs/(need_events|eat|sleep|work|wait|continue_routine|movement)\.rs`, `events/`, `replay/`, `checksum\.rs`, `state\.rs`, `actions/(proposal|report)\.rs`, `view_models\.rs`, `debug_capability\.rs`, `controller\.rs`, `debug_reports\.rs`, `epistemics/`, `content/src/(manifest|load|schema|serialization|validate)\.rs`, `tui/src/(app|debug_panels|render|transcript)\.rs` — and **omits both `crates/tracewake-core/src/time.rs` and `crates/tracewake-core/src/actions/defs/checkcontainer.rs`** (the `actions/defs` group lacks `checkcontainer`; there is no `time\.rs` term). The scheduled `mutants-lock-layer` job (`ci.yml:177`, single `cargo mutants --workspace --no-shuffle` at `:198`) already runs the full perimeter and needs no trigger change here (its sharded restructuring is -004).
2. The guard test `crates/tracewake-core/tests/ci_workflow_guards.rs` defines `STANDING_MUTATION_PERIMETER` (`:16`) checked against `mutants.toml` and `STANDING_MUTATION_TRIGGER_FRAGMENTS` (`:54`) checked against `ci.yml`; the perimeter loop (`:339`) and trigger loop (`:350`) enforce containment, and a per-path trigger-fragment mapping (`map_standing_path_to_trigger`, ending `:453` with a `panic!` on unmapped paths) translates each perimeter path to its expected trigger fragment. Verified this session: a `grep -nE "time\.rs|checkcontainer"` over the file returns **no matches** — both constants omit the two paths, so the mapping has no arm for them and the guard is self-consistent with stale data. Synthetic negatives already exist (`:154` "divergent scheduled mutation perimeter", `:181` "in-diff trigger missing a standing path"); a new removal-negative per added path extends that pattern.
3. Cross-artifact boundary under audit: the three-way contract between `.cargo/mutants.toml` (the certifying denominator — unchanged here), the `ci.yml` `mutants-in-diff` trigger (the fast, explicitly non-certifying detector), and `ci_workflow_guards.rs` (the regression guard that must keep trigger and perimeter in agreement). The spec forbids shrinking the perimeter, laundering survivors into `.cargo/mutants-baseline-misses.txt`, and treating `--in-diff`/`--no-config`/`-f` as the certifying denominator (spec §1.5, §4.3, §4.8).
4. Invariant/rule motivation: the execution evidence/mutation-completion doctrine (`docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`) and the anti-Goodhart / perimeter-honesty controls (`docs/3-reference/01_DESIGN_RISK_REGISTER.md`; spec §10.3). The in-diff lane is an explicitly non-certifying fast detector that must cover the full standing seam so a regression on a configured path is caught early, and the guard must make that coverage self-enforcing. cargo-mutants warns `--in-diff` is inherently a changed-code subset and never substitutes for the scheduled full run (spec §3.4, §4.8 item 7).
5. Enforcement surface: the `ci_workflow_guards.rs` regression guard is itself the fail-closed gate here. The change adds coverage to a CI detector and tightens its guard; it touches no simulation-world fail-closed validation, actor-knowledge filtering, or deterministic-replay surface, and introduces no epistemic-leakage or replay-nondeterminism path. The mutation denominator (`.cargo/mutants.toml`, 62 selected files / 2,901 generated mutants at the authoring baseline `fd5ae94`) is verified unchanged — the perimeter is not shrunk.
6. Mismatch + correction: a bare two-fragment addition to `ci.yml` would pass the current containment guard while leaving the corrected trigger unenforced (silent recurrence), and a preferred consolidated rewrite would fail the literal-fragment containment check until the constant is updated. Either way the `STANDING_MUTATION_TRIGGER_FRAGMENTS` constant and the `map_standing_path_to_trigger` arm are mandatory co-edits, and a per-path removal-negative is required to prevent the next perimeter widening from re-opening the gap.

## Architecture Check

1. Correcting the trigger AND making the guard assert perimeter↔trigger consistency per path is cleaner than a one-time fragment patch: a patch fixes today's two paths but leaves the same hand-maintained-duplicate drift surface that produced the gap. Keeping the per-path `map_standing_path_to_trigger` mapping exhaustive (its trailing `panic!` already forces an arm for every perimeter path) means a future perimeter widening that forgets the trigger fails the guard immediately, at the cheapest possible layer.
2. No backwards-compatibility aliasing/shims. The in-diff lane remains explicitly non-certifying; it is not made a substitute for the scheduled full configured run, and `.cargo/mutants.toml` is not shrunk to make the trigger expression simpler.

## Verification Layers

1. In-diff trigger covers the standing perimeter → codebase grep-proof: every `.cargo/mutants.toml` examined path (or its covering directory prefix) appears in the `ci.yml` `mutants-in-diff` guarded-path expression, including `time.rs` and `actions/defs/checkcontainer.rs`.
2. Guard drift-proofness (evidence/mutation-completion doctrine) → core test: `ci_workflow_guards.rs` fails when the in-diff trigger omits any standing-perimeter path, proven by a synthetic removal assertion per added path in the style of the existing `:181` negative.
3. Perimeter not shrunk; baseline not laundered → codebase grep-proof: `.cargo/mutants.toml` still expands to the standing 62-file seam (both carriers present) and `.cargo/mutants-baseline-misses.txt` remains empty (0 bytes).
4. Single-layer note: no replay/golden-fixture or actor-knowledge surface is touched, so no epistemic/replay layer mapping applies (item 5 above records why).

## What to Change

### 1. Extend the `ci.yml` in-diff guarded-path trigger

In `.github/workflows/ci.yml`, the `mutants-in-diff` job's guarded-diff step (`ci.yml:115`): extend the `git diff --name-only … | grep -E '…'` alternation so it covers `crates/tracewake-core/src/time.rs` and `crates/tracewake-core/src/actions/defs/checkcontainer.rs`. Prefer a consolidated expression where it represents the standing seam exactly — broaden the `actions/defs/(…)\.rs` group to add `checkcontainer`, and add a `time\.rs` term — provided the expression matches the standing perimeter exactly and adds no path outside it. Retain the existing in-diff artifact retention behaviour.

### 2. Co-edit the guard's perimeter/trigger expectations and the per-path mapping

In `crates/tracewake-core/tests/ci_workflow_guards.rs`: add both paths to `STANDING_MUTATION_PERIMETER` (`:16`); extend `STANDING_MUTATION_TRIGGER_FRAGMENTS` (`:54`) to represent the corrected trigger exactly; and add the `map_standing_path_to_trigger` arm (before the trailing `panic!` at `:453`) for each new path so the perimeter⇒trigger mapping stays total. The existing containment loops stay; the per-path mapping is what enforces agreement.

### 3. Add synthetic removal negatives for both added carriers

Add (or extend) a synthetic-workflow negative in the style of the existing `:181` "in-diff trigger missing a standing path" assertion proving that removing the `time.rs` trigger coverage, and separately the `checkcontainer.rs` coverage, fails the guard.

### 4. Verify the perimeter and baseline-miss posture are unchanged

Confirm `.cargo/mutants.toml` still expands to the standing 62-file seam (no shrinkage, no `exclude_globs`, no `#[mutants::skip]` laundering) and `.cargo/mutants-baseline-misses.txt` is still empty. Record the verification; no edit expected to either file.

## Files to Touch

- `.github/workflows/ci.yml` (modify)
- `crates/tracewake-core/tests/ci_workflow_guards.rs` (modify)
- `.cargo/mutants.toml` (modify — verify-only; no change expected, both carriers already present)
- `.cargo/mutants-baseline-misses.txt` (modify — verify-only; must remain empty)

## Out of Scope

- The sharded scheduled/manual completion lane, supervisor retention, and the set-union merger (→ -002, -003, -004).
- The full configured campaign run, census, completion proof, and triage register (→ -005).
- Live FIRST-PROOF re-proof and the replacement acceptance artifact (→ -006).
- Any shrinkage of `.cargo/mutants.toml` or addition of survivors to `.cargo/mutants-baseline-misses.txt` (forbidden — spec §1.5, §4.8, §12).
- Restructuring the scheduled `mutants-lock-layer` job into a matrix (→ -004); this ticket leaves that job's invocation unchanged.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test ci_workflow_guards` passes with the corrected trigger, the extended `STANDING_MUTATION_PERIMETER` / `STANDING_MUTATION_TRIGGER_FRAGMENTS`, and the total per-path mapping.
2. A synthetic negative fails the guard when the in-diff trigger omits `time.rs`, and a second when it omits `actions/defs/checkcontainer.rs` (in the style of the existing `:181` assertion).
3. `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace --locked` all pass.

### Invariants

1. The `ci.yml` in-diff trigger covers every standing-perimeter path — `time.rs` and `actions/defs/checkcontainer.rs` included — and the guard fails if it ever does not.
2. `.cargo/mutants.toml` is not shrunk and `.cargo/mutants-baseline-misses.txt` remains empty — the certifying denominator is preserved.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/ci_workflow_guards.rs` — add both carriers to `STANDING_MUTATION_PERIMETER` and `STANDING_MUTATION_TRIGGER_FRAGMENTS`, add their `map_standing_path_to_trigger` arms, and add a per-carrier synthetic removal negative proving an omitted path is caught.

### Commands

1. `cargo test --locked -p tracewake-core --test ci_workflow_guards`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --locked`
3. `grep -nE 'time\.rs|checkcontainer' .github/workflows/ci.yml` — confirm both carriers now appear in the in-diff trigger expression; `wc -c .cargo/mutants-baseline-misses.txt` — confirm `0` bytes (no laundering).

## Outcome

Completed: 2026-06-21

The `mutants-in-diff` guarded-path regex now covers both FIRST-PROOF standing
perimeter additions: `crates/tracewake-core/src/time.rs` and
`crates/tracewake-core/src/actions/defs/checkcontainer.rs`. The core CI workflow
guard now includes both paths in `STANDING_MUTATION_PERIMETER`, includes the
matching trigger fragments, maps each perimeter path to its trigger coverage,
and has synthetic removal negatives for both new carriers.

During verification, the existing anti-regression mutation-perimeter guard also
proved to model the grouped `actions/defs` regex. Its synthetic regression
literals were updated to the widened group so that guard continues to test the
live CI expression instead of a stale pre-0045 shape.

No mutation denominator shrinkage or survivor laundering was performed:
`.cargo/mutants.toml` still contains both carriers, and
`.cargo/mutants-baseline-misses.txt` remains `0` bytes.

Verification run:

- `cargo test --locked -p tracewake-core --test ci_workflow_guards` — passed.
- `cargo test --locked -p tracewake-core --test anti_regression_guards mutation_perimeter_matches_duration_action_rationale_and_ci_filters` — passed after the stale synthetic literals were corrected.
- `cargo fmt --all --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo test --workspace --locked` — passed.
- `grep -nE 'time\.rs|checkcontainer' .github/workflows/ci.yml crates/tracewake-core/tests/ci_workflow_guards.rs .cargo/mutants.toml` — confirmed the CI trigger, guard, and standing config all name the carriers.
- `wc -c .cargo/mutants-baseline-misses.txt` — reported `0`.
