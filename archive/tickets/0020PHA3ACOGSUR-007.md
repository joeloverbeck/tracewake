# 0020PHA3ACOGSUR-007: Scheduler boundary semantics: completion-window progress and the routine-window bound

**Status**: COMPLETED
**Priority**: LOW
**Effort**: Small
**Engine Changes**: Yes — `tracewake-core` (`scheduler.rs` window bookkeeping and `routine_window_family` bound); boundary fixture; no new events or schemas
**Deps**: `archive/specs/0020_PHASE_3A_COGNITION_SURFACE_FRESHNESS_PARITY_DERIVED_CENSUS_CLOSURE_MUTATION_PERIMETER_COMPLETION_AND_GENERATIVE_FIDELITY_HARDENING_SPEC.md` (ORD-HARD-064, ORD-HARD-065)

## Problem

Two `scheduler.rs` boundary-semantics findings. `ORD-HARD-064`: in
`run_no_human_day`'s final sweep, `WindowNoProgress` is suppressed only by
`duration_skip_by_window_actor` (open body-exclusive duration at `window.start_tick`)
or `progress_by_window_actor` (written only at the proposal-results site) — a window
whose only legitimate activity was a duration completion appended by
`append_due_completions` feeds neither, so a spurious stuck diagnostic may fire
against an actor that genuinely progressed (mechanism operator-verified; whether the
diagnostic actually fires is decided by the boundary fixture this ticket writes
first). `ORD-HARD-065`: `routine_window_family` filters executions by
`execution.start_tick <= window.end_tick`, admitting a not-yet-started mid-window
execution as the active window duty at `window.start_tick`, while the sibling
selection site in the same file uses `execution.start_tick <= window.start_tick` —
two bounds encoding different semantics for the same question (INV-035/038).

## Assumption Reassessment (2026-06-11)

1. Verified at `main` `96bc387` (operator-verified at the spec's reassessment):
   `progress_by_window_actor` written only at the proposal-results site;
   `duration_skip_by_window_actor` populated pre-decision via
   `actor_has_open_body_exclusive_duration`; the final sweep checks only those two
   before emitting `StuckDiagnosticKind::WindowNoProgress`; `routine_window_family`
   uses `<= window.end_tick` (with a deadline lower bound and `min_by(start_tick)`)
   while the sibling site uses `<= window.start_tick`. Canonical fixtures pair
   completions with same-window decisions, so neither boundary case is exercised.
2. Verified against spec 0020 (reassessed 2026-06-11): ORD-HARD-064's
   fix-vs-validated-no-action is decided by the boundary fixture (spec-assigned,
   recorded outcome either way — a no-fire result is recorded in the acceptance
   artifact, never silently dropped); ORD-HARD-065 allows aligning the bound or
   documenting window-scoped eligibility — but both sites must encode one rule.
3. Cross-artifact boundary under audit: the no-human window bookkeeping contract
   spanning `append_due_completions`, the progress/skip maps, and the stuck-sweep —
   a typed diagnostic must be evidence of genuine non-progress (INV-105 spirit).
4. INV-038 restated: goal/duty selection is deterministic and justified — a duty
   whose window has not opened at the decision tick is not yet a duty. INV-105
   restated: diagnostics are authoritative diagnostic data; a false
   `WindowNoProgress` erodes the stuck-detector's evidentiary value.
5. Deterministic-replay surface touched (scheduler bookkeeping): the fix (if it
   fires) records `(window, actor)` progress for completion-bearing windows from the
   completions' own deterministic `(tick, event_id)` ordering — no new
   nondeterminism; no event payloads change; replay/golden gates must stay green
   (no repricing expected — diagnostics only become *more* accurate).

## Architecture Check

1. Fixture-first for 064: the deciding boundary test (sleep completing at a window
   boundary with no further proposal that window) lands before any fix, so the
   correction is driven by reproduced behavior, not by the mechanism's plausibility
   — the spec's own evidence discipline applied to its own finding. If it fires,
   record completion-bearing windows as progress (a `completed_by_window_actor` set
   fed where `append_due_completions` lands a terminal inside a window); if it does
   not, the finding downgrades to validated-no-action with the fixture retained as
   the regression guard.
2. For 065, aligning to `<= window.start_tick` (the sibling site's bound) is
   preferred over documenting divergent semantics: one rule, two sites, plus the
   two-execution test — no shims, no per-site exceptions.

## Verification Layers

1. INV-105 (064) -> boundary fixture: duration completes at a window boundary with
   no same-window proposal; asserts either no `WindowNoProgress` (post-fix) or
   documents the no-fire outcome (validated-no-action) — retained as a regression
   guard in both branches.
2. INV-038/035 (065) -> two-execution test: one execution active at
   `window.start_tick`, one authored to start mid-window; asserts the already-open
   execution is selected.
3. Consistency -> grep-proof: both selection sites carry the same bound expression
   after the change.
4. Replay safety -> `cargo test --workspace` green; no golden repricing.

## What to Change

### 1. ORD-HARD-064 — boundary fixture, then the recorded outcome

Write the completion-only-window fixture first. If the spurious diagnostic fires:
record completion-bearing windows as progress in the final sweep (deterministic,
keyed off the completion events `append_due_completions` emits). If it does not
fire: keep the fixture as a guard and record validated-no-action in the acceptance
artifact (spec §7.8).

### 2. ORD-HARD-065 — one bound, both sites

Align `routine_window_family` to `execution.start_tick <= window.start_tick`
(matching the sibling site), or — if window-scoped eligibility turns out to be
load-bearing for an existing golden — document the intended rule and make both
sites encode it identically; add the two-execution test either way.

## Files to Touch

- `crates/tracewake-core/src/scheduler.rs` (modify — shared with `-004` if its per-tick-flush option fires: coordinate the mechanical merge)

## Out of Scope

- The generative advance entry (`-004`).
- Stuck-diagnostic vocabulary or any new diagnostic kinds.
- Need-charging, completion-event semantics, or ordering keys (verified holding).

## Acceptance Criteria

### Tests That Must Pass

1. Completion-only-window boundary fixture green, with the 064 outcome (fix or
   validated-no-action) explicitly recorded.
2. Two-execution routine-window test green; mid-window execution not selected at
   `window.start_tick` (or the documented single rule proven at both sites).
3. Grep-proof: one bound expression at both selection sites.
4. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`.

### Invariants

1. A `WindowNoProgress` diagnostic is evidence of genuine non-progress (INV-105).
2. Routine-window duty eligibility follows one deterministic rule at every selection
   site (INV-035/038).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/scheduler.rs` (test block) — completion-only-window
   boundary fixture; two-execution routine-window selection test.

### Commands

1. `cargo test -p tracewake-core scheduler`
2. `cargo test --workspace` (full pipeline)

## Outcome

Completed 2026-06-11. The ORD-HARD-064 boundary fixture reproduced the
completion-only-window shape: a SleepNight routine starts in the first window,
then the next window's only legitimate progress is the scheduled `SleepCompleted`
terminal. The fix records duration completions returned by `append_due_completions`
as deterministic `(actor, tick)` progress for the containing window, so the final
stuck sweep no longer emits a spurious `WindowNoProgress` for that window.

For ORD-HARD-065, `routine_window_family` now uses
`execution.start_tick <= window.start_tick`, matching the sibling routine-step
completion selection bound. The two-execution test also covers the stricter case:
a mid-window-only routine is not eligible at `window.start_tick`; when an
already-open routine is present, it is selected.

Verification:

1. `cargo test -p tracewake-core scheduler -- --nocapture`
2. Grep proof: `rg -n "execution\\.start_tick <= window\\.(start_tick|end_tick)|routine_window_family\\(" crates/tracewake-core/src/scheduler.rs` shows both selection sites use `window.start_tick` and no `window.end_tick` eligibility bound remains.
3. `cargo test -p tracewake-tui --test command_loop_session no_human_day_command_loop_renders_phase3a_behavior_rows -- --nocapture`
4. `cargo test -p tracewake-tui --test tui_acceptance no_human -- --nocapture`
5. `cargo fmt --all --check`
6. `cargo clippy --workspace --all-targets -- -D warnings`
7. `cargo build --workspace --all-targets --locked`
8. `cargo test --workspace`

TUI no-human debug metric assertions were updated from the old early-eligibility
surface (`work_failed=2`, `need_crossings=1`) to the corrected run
(`work_failed=1`, `need_crossings=2`). The rendered routine panel now shows
`routine_exec_tomas_work` as `Completed`, matching the fixed duty eligibility.
