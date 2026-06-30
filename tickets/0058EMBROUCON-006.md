# 0058EMBROUCON-006: Meta-lock guards and focused mutation

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — adds source-scan guards to `anti_regression_guards.rs`; runs focused mutation (evidence recorded in -007)
**Deps**: 0058EMBROUCON-003, 0058EMBROUCON-004, 0058EMBROUCON-005

## Problem

Spec §6.3 / §6.4 — add source-scan guards (only after the behavioral tests exist) that lock the remediations against quiet reintroduction, and run focused mutation over the touched seam files. Each guard needs a lock id, a `synthetic_*` negative id (per the in-session `/reassess-spec` finding M1), and a non-vacuous witness minimum.

## Assumption Reassessment (2026-06-30)

1. The existing guard `guard_0057_continue_routine_progress_of_record_is_follow_on` lives at `crates/tracewake-core/tests/anti_regression_guards.rs:10312` (registered at 2160 / 3018). The house `negative_id` convention is snake_case `synthetic_<desc>` (e.g. `synthetic_meta_lock_without_negative`) — **not** a dotted `negNNNN.` form (reassess M1; spec §6.3 updated to match). `crates/tracewake-core/tests/ci_workflow_guards.rs` exists; no new touched files fall outside existing trigger coverage (all are existing seam files), so it is untouched.
2. Spec §6.3 requires four guards with negative ids and witness minima; §6.4 requires focused mutation over the touched seam files using the **existing** perimeter (§9.3: the seam files are inside the existing ~3.5K-mutant perimeter), via `--in-diff` / `--file` / `--iterate`, with no full standing campaign by default. `.cargo/mutants.toml` is **not** modified (the perimeter already covers the seams).
3. Cross-artifact boundary under audit: the source-scan guard registry (`anti_regression_guards.rs`) ↔ the remediated seam files (`session.rs`, `app.rs`). Guards are added only after the behavioral tests (-001…-005) exist.
4. Invariants under audit: INV-098 (feature acceptance is harsh; standing guards prevent quiet reintroduction). The guards lock INV-035/104 (-001), INV-112 (-002), INV-015/105 (-003), and INV-069/108 (-005).
5. Enforcement surface: the four source-scan guards. They lock the actor-knowledge / temporal / diagnostic / forwarding surfaces by source scan; confirm each guard is non-vacuous (a synthetic source minimum that actually fails) and introduces no determinism/leak path (guards are static source scans with no runtime state).

## Architecture Check

1. Source-scan guards added *after* the behavioral tests are the house meta-lock pattern: behavioral tests prove the fix, guards prevent silent reintroduction (INV-098). Focused mutation over the existing perimeter — not repeated full standing campaigns — is the spec's §6.4 / §9.3 evidence discipline.
2. No backwards-compatibility aliasing/shims: guards reject synthetic regressions outright; negative ids follow the existing `synthetic_*` registry, not a new dotted form.

## Verification Layers

1. INV-098 (harsh acceptance) → the four guards run under `cargo test --workspace`, each with a non-vacuous synthetic witness.
2. (mutation evidence) → focused `cargo mutants` over the seam files; misses dispositioned honestly (recorded in -007).
3. INV-035/104/112/015/105/069/108 → each guard maps to its remediation surface (source grep-proof).

## What to Change

### 1. Four source-scan guards

Add to `anti_regression_guards.rs`, after the behavioral tests exist:

- `guard_0058_embodied_routine_family_has_no_pre_intention_workplace_selector` — negative id `synthetic_workplace_before_active_intention`; fails on a synthetic source returning `RoutineFamily::WorkBlock` from `known_workplaces()` before `active_intention_by_actor()`.
- `guard_0058_embodied_continue_time_advancing_follow_on_is_gated` — negative id `synthetic_direct_wait_follow_on`; fails on a synthetic source running `run_pipeline` for a `wait` follow-on without checking the temporal gateway.
- `guard_0058_embodied_continue_success_path_not_current_stuck` — negative id `synthetic_success_prefixed_current_stuck`; fails on a synthetic source building a current `StuckDiagnosticRecorded` before matching the transaction outcome and prefixing it into success.
- `guard_0058_tui_continue_routine_forwards_only` — negative id `synthetic_tui_routine_selection`; fails if `tracewake-tui/src/app.rs` imports or calls routine/planner/session internals beyond the runtime command surface, or constructs follow-on targets.

Each guard registers a lock id, the `synthetic_*` negative id, and a witness minimum consistent with the house meta-lock registry.

### 2. Focused mutation

Run focused mutation over the touched seam files (`session.rs`, `routine_continuation.rs`, `continue_routine.rs`, `wait.rs`, `need_events.rs`, `scheduler.rs`, `pipeline.rs`, `app.rs`, `census_actions.rs`, `runner.rs`, `anti_regression_guards.rs`) via `--in-diff` and/or per-file `--file`, converging with `--iterate`. Do **not** modify `.cargo/mutants.toml` (the existing perimeter covers the seams). Record commands, tool version, scope, `mutants.out` summary, and missed/unviable/timeout disposition for -007.

## Files to Touch

- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)

## Out of Scope

- Modifying `.cargo/mutants.toml` (the perimeter already covers the seams — §9.3).
- `ci_workflow_guards.rs` (no new touched files outside existing trigger coverage).
- A full standing mutation campaign (reserved for the very end, only if focused results expose cross-seam erosion — §6.4).

## Acceptance Criteria

### Tests That Must Pass

1. The four `guard_0058_*` tests pass and each fails on its synthetic negative source (non-vacuous): `synthetic_workplace_before_active_intention`, `synthetic_direct_wait_follow_on`, `synthetic_success_prefixed_current_stuck`, `synthetic_tui_routine_selection`.
2. `cargo test --workspace` passes.
3. Focused mutation over the seam files catches the meaningful mutants named by the spec, or documents an equivalent/unviable disposition.

### Invariants

1. Each remediation surface has a standing source-scan guard with a non-vacuous synthetic witness (INV-098).
2. Negative ids follow the house `synthetic_*` convention, not a dotted `negNNNN.` form.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` (modify) — four guard fns plus their negative-id registrations.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards guard_0058`
2. `cargo test --workspace`
3. `cargo mutants --in-diff <diff>` (or per-file `--file` over the seam files) `--iterate` — focused mutation, deliberately narrower than a full standing campaign per §6.4 / §9.3 (the correct verification boundary for a scoped seam-hardening ticket).
