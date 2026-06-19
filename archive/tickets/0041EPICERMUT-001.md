# 0041EPICERMUT-001: Standing perimeter confirmation, CI in-diff trigger convergence, and config comment correction

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — `.cargo/mutants.toml` (descriptive comment), `.github/workflows/ci.yml` (in-diff mutation trigger), `crates/tracewake-core/tests/ci_workflow_guards.rs` (standing-trigger fragment guard)
**Deps**: None

## Problem

Spec 0041 §3.3–§3.4 names a documentation/CI durability gap that must be closed before the certifying mutation campaign (ticket 0041EPICERMUT-009) can claim its denominator is honestly guarded.

At the authoring baseline `7a17447d…`, `.cargo/mutants.toml` already carries the standing `crates/tracewake-core/src/epistemics/**` examine glob, which covers all four EPI survivor-bearing files (`proposition.rs`, `belief.rs`, `contradiction.rs`, `observation.rs`); `test_workspace = true` and `additional_cargo_args = ["--locked"]` are present; and `.cargo/mutants-baseline-misses.txt` is empty. The scheduled lock-layer mutation job consumes that full configured perimeter.

But the in-diff trigger regex in `.github/workflows/ci.yml` (job `mutants-in-diff`) names only `crates/tracewake-core/src/epistemics/(knowledge_context|projection)\.rs`. A pull request that touches only `belief.rs`, `contradiction.rs`, `observation.rs`, or `proposition.rs` therefore does **not** launch the change-scoped mutation job unless another guarded path also changes — the configured denominator is present for scheduled runs but evadable in-diff. The minimal correction (§3.4) is to align the trigger with the standing `epistemics/**` perimeter by matching the `crates/tracewake-core/src/epistemics/` directory rather than two named files. This must not narrow, duplicate, or replace the checked-in config — the config stays the single denominator of record.

Separately, the `.cargo/mutants.toml` leading comment still calls the union a "Standing SPINE-CERT mutation perimeter." Since the file now also carries the EPI expansion, that descriptive comment must be corrected to a neutral standing certification/lock-layer perimeter. This is documentation status, not a new gate.

## Assumption Reassessment (2026-06-19)

1. Codebase check: `.cargo/mutants.toml` contains `"crates/tracewake-core/src/epistemics/**"` in `examine_globs` (covers all four survivor files), plus redundant explicit `knowledge_context.rs`/`projection.rs` entries; `test_workspace = true` and `additional_cargo_args = ["--locked"]` are present; the leading comment reads `# Standing SPINE-CERT mutation perimeter.`. `.github/workflows/ci.yml` job `mutants-in-diff` (step `Check guarded-layer diff`) greps a regex whose EPI fragment is `crates/tracewake-core/src/epistemics/(knowledge_context|projection)\.rs`. `crates/tracewake-core/tests/ci_workflow_guards.rs` pins the trigger via `STANDING_MUTATION_TRIGGER_FRAGMENTS` (the `crates/tracewake-core/src/epistemics/(knowledge_context|projection)\\.rs` fragment) and asserts it through `mutation_perimeter_errors`; `.cargo/mutants-baseline-misses.txt` is empty (0 lines). All confirmed at this baseline.
2. Specs/docs check: spec 0041 §3.4 mandates the minimal trigger correction "preferably by matching `crates/tracewake-core/src/epistemics/`" and the neutral-comment correction; §4.2 forbids the certifying run from using `--no-config`/`-f`/`--exclude`/`--in-diff`; §4.3 requires scheduled mutation to keep consuming the checked-in config and the baseline-miss file to stay un-laundered. `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` line ~330 describes `mutants-in-diff` as "standing SPINE-perimeter mutation checks" — an optional descriptive neutralization (the guard test's `doc_workflow_parity_errors` compares only job IDs, which are unchanged, so doc parity does not block this edit).
3. Cross-artifact shared boundary under audit: the four-way contract between (a) `.cargo/mutants.toml` `examine_globs` (denominator of record), (b) the `ci.yml` in-diff trigger regex (change detector), (c) `ci_workflow_guards.rs` `STANDING_MUTATION_TRIGGER_FRAGMENTS` (drift guard), and (d) doc 10's CI prose. The trigger and its guard fragment must move together or `mutation_perimeter_errors` fails; the config glob is not narrowed.
4. Motivating invariants (INV restate): `INV-091`–`INV-095` (mutation lock layer / harsh acceptance) require no-human, replay, leakage, possession, and view-model behavior to remain regression-tested and that metric improvement cannot replace behavior evidence. This ticket strengthens the lock layer's in-diff durability without changing the denominator; it adds no score-based gate (§2.5: `EMERGE-OBS` and mutation scores remain non-thresholds).
5. Enforcement-surface confirmation (substrate basis): this ticket touches the deterministic mutation lock-layer that feeds the certifying campaign in 0041EPICERMUT-009 — the deferred enforcement surface. Broadening the in-diff trigger from two named files to the `epistemics/` directory strictly enlarges the set of changes that trigger mutation; it removes an evasion path and introduces no path that lets a survivor-bearing file leave the certified denominator. The config glob (the actual denominator) is unchanged, so deterministic replay/projection behavior and actor-knowledge filtering are untouched; no epistemic leakage path is opened.
6. Blast radius of the trigger change (grep proof): the EPI trigger fragment appears in exactly two enforced surfaces — `.github/workflows/ci.yml` (the live regex) and `crates/tracewake-core/tests/ci_workflow_guards.rs` (`STANDING_MUTATION_TRIGGER_FRAGMENTS`, asserted by `mutation_perimeter_errors`). Both change in this diff. doc 10's parity check (`doc_workflow_parity_errors`) compares job IDs only, so it does not require the regex; the only doc-10 touch is the optional descriptive-prose neutralization. No other consumer references the two-file EPI fragment.

## Architecture Check

1. Matching the `crates/tracewake-core/src/epistemics/` directory at the trigger layer (while leaving the `.cargo/mutants.toml` glob as the single denominator) is cleaner than enumerating the four survivor files in the trigger: it tracks any future EPI source file automatically, mirrors how the scheduled job already consumes the whole configured perimeter, and keeps exactly one source of truth for the denominator. Enumerating files would re-introduce the same drift class this ticket closes.
2. No backwards-compatibility aliasing/shims: the old two-file fragment is replaced, not wrapped or kept alongside a fallback. The config is corrected in place; no alternate trigger path is added.

## Verification Layers

1. INV-091–095 (mutation lock-layer durability) -> codebase grep-proof: the broadened directory fragment is present in `ci.yml` and in `ci_workflow_guards.rs`'s `STANDING_MUTATION_TRIGGER_FRAGMENTS`, and the two-file fragment is absent.
2. CI trigger ↔ config ↔ guard parity -> `cargo test -p tracewake-core --test ci_workflow_guards` passes (the `ci_workflow_guards_cover_workflow_integrity` test exercises `mutation_perimeter_errors`).
3. Denominator-of-record integrity -> manual review: `.cargo/mutants.toml` `examine_globs` is unchanged except the descriptive comment; the `epistemics/**` glob, `test_workspace = true`, and `additional_cargo_args = ["--locked"]` remain.

## What to Change

### 1. Broaden the in-diff mutation trigger to the `epistemics/` directory

In `.github/workflows/ci.yml`, job `mutants-in-diff`, step `Check guarded-layer diff`: replace the regex alternation fragment `crates/tracewake-core/src/epistemics/(knowledge_context|projection)\.rs` with a directory-level fragment matching `crates/tracewake-core/src/epistemics/` (so any `epistemics/**` source change launches the job). Do not otherwise narrow, duplicate, or restructure the regex.

### 2. Update the standing-trigger drift guard

In `crates/tracewake-core/tests/ci_workflow_guards.rs`, update the `STANDING_MUTATION_TRIGGER_FRAGMENTS` entry for the EPI surface from `crates/tracewake-core/src/epistemics/(knowledge_context|projection)\\.rs` to the broadened directory fragment, so `mutation_perimeter_errors` asserts the corrected trigger and would fail if it regresses to the two-file form.

### 3. Neutralize the `.cargo/mutants.toml` descriptive comment

Change the leading comment `# Standing SPINE-CERT mutation perimeter.` to a neutral standing certification/lock-layer perimeter description (the union now spans SPINE and EPI expansions). Do not change any `examine_globs` entry, `test_workspace`, or `additional_cargo_args`.

### 4. (Conditional) Neutralize doc 10's descriptive CI prose

If retained for status honesty, update the `mutants-in-diff` row prose in `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` from "standing SPINE-perimeter mutation checks" to a neutral standing-perimeter description. This is descriptive only; the guard's `doc_workflow_parity_errors` checks job IDs, not this prose.

## Files to Touch

- `.github/workflows/ci.yml` (modify)
- `crates/tracewake-core/tests/ci_workflow_guards.rs` (modify)
- `.cargo/mutants.toml` (modify)
- `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` (modify — conditional descriptive-prose neutralization only)

## Out of Scope

- Narrowing, duplicating, or replacing the `.cargo/mutants.toml` `examine_globs` denominator (§4.2: config stays the single denominator of record).
- Adding any survivor to `.cargo/mutants-baseline-misses.txt` (§4.3 — that file stays empty here; laundering is forbidden).
- A 0039-style wholesale perimeter re-derivation (§11.4 — the glob already covers the four files; this is the minimal §3.4 correction only).
- Running the certifying mutation campaign or capturing `--list-files`/`--list` census (owned by 0041EPICERMUT-009).
- Killing any survivor or adding behavior-witness tests (owned by the per-file kill tickets 002–008).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --test ci_workflow_guards` — passes with the broadened EPI directory fragment asserted by `mutation_perimeter_errors`.
2. `grep -E 'epistemics/' .github/workflows/ci.yml` shows the directory-level fragment and `grep -F 'epistemics/(knowledge_context|projection)' .github/workflows/ci.yml` returns nothing (the two-file fragment is gone).
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` — clean baseline preserved.

### Invariants

1. The `.cargo/mutants.toml` `examine_globs` denominator is unchanged except the descriptive comment; `epistemics/**`, `test_workspace = true`, and `additional_cargo_args = ["--locked"]` all remain.
2. The in-diff trigger and its `ci_workflow_guards.rs` drift guard reference the identical broadened fragment (no trigger/guard skew).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/ci_workflow_guards.rs` — update `STANDING_MUTATION_TRIGGER_FRAGMENTS` to the broadened EPI directory fragment so `mutation_perimeter_errors` certifies the corrected trigger and locks against regression to the two-file form.

### Commands

1. `cargo test -p tracewake-core --test ci_workflow_guards`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
3. The targeted `ci_workflow_guards` run is the correct verification boundary: it is the suite that pins the CI trigger ↔ config ↔ doc contract this ticket edits; the full pipeline confirms no collateral breakage.

## Outcome

Completed: 2026-06-19

Implemented the standing EPI in-diff trigger correction by replacing the two-file `knowledge_context|projection` regex fragment in `.github/workflows/ci.yml` with the directory-level `crates/tracewake-core/src/epistemics/` fragment. Updated `crates/tracewake-core/tests/ci_workflow_guards.rs` to pin the same broadened fragment.

Corrected the stale SPINE-only perimeter wording in `.cargo/mutants.toml` and in `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md`. The checked-in `.cargo/mutants.toml` denominator remained unchanged apart from the descriptive comment; `epistemics/**`, `test_workspace = true`, and `additional_cargo_args = ["--locked"]` remain present.

During full-gate verification, `cargo test --workspace` exposed an additional enforced consumer: `crates/tracewake-core/tests/anti_regression_guards.rs` still parsed only the old grouped epistemics regex and reported the broadened trigger as omitting `knowledge_context.rs` and `projection.rs`. Updated that parser narrowly so either the new directory fragment or the historical grouped fragment satisfies those existing required paths; reran the failing target successfully before rerunning the full gate set.

Verification:

- `cargo test -p tracewake-core --test ci_workflow_guards` passed.
- `grep -E 'epistemics/' .github/workflows/ci.yml` showed the directory-level trigger fragment.
- `grep -F 'epistemics/(knowledge_context|projection)' .github/workflows/ci.yml` returned no matches.
- `cargo test -p tracewake-core --test anti_regression_guards mutation_perimeter_matches_duration_action_rationale_and_ci_filters` passed after the parser update.
- `cargo fmt --all --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo build --workspace --all-targets --locked` passed.
- `cargo test --workspace` passed.

Deviations: the implementation touched `crates/tracewake-core/tests/anti_regression_guards.rs` in addition to the originally listed files because the full workspace test proved that guard is part of the same CI-filter contract. No survivor was added to `.cargo/mutants-baseline-misses.txt`; the certifying mutation campaign remains owned by `0041EPICERMUT-009`.
