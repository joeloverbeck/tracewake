# 0047TUIAUTWOR-010: Refactor `run_no_human_day`/`advance_no_human` onto the coordinator

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `crates/tracewake-core` (`scheduler.rs` no-human runner refactor; remove duplicate pending-duration authority); regenerated golden traces/checksums across ~8 no-human test consumers
**Deps**: 0047TUIAUTWOR-005, 0047TUIAUTWOR-006, 0047TUIAUTWOR-007, 0047TUIAUTWOR-008

## Problem

Spec 0047 §1.9 requires `run_no_human_day` and `advance_no_human` to loop the shared coordinator while preserving their day windows, progress metrics, markers, reports, and existing acceptance behavior — so they no longer own a separate definition of time progression or a second pending-duration authority. Today they walk ticks and drain batch-local `pending_sleep_starts`/`pending_work_starts` vectors via `append_due_completions` (`scheduler.rs:911`), a parallel time-progression path that cannot see durations started in other transactions. Once the coordinator (0047TUIAUTWOR-005…008) is complete, this ticket refactors both runners onto it and removes the duplicate pending authority — establishing the single semantic definition of a loaded-world tick (§8).

## Assumption Reassessment (2026-06-22)

1. `run_no_human_day` (`scheduler.rs:344`) and `advance_no_human` (`scheduler.rs:2147`) live in the `scheduler::no_human` module; their public types are `NoHumanDayConfig` (`:283`), `NoHumanDayReport` (`:289`), `DayWindow` (`:268`), `NoHumanStateMut` (`:262`), `NoHumanAdvanceReport` (`:254`). `advance_no_human` already contains a true tick loop (`for expected_tick { scheduler.advance_one_tick(); append_due_completions(...) }`, `:2192-2255`). The refactor must preserve these public signatures and report shapes — they are the standing contract.
2. ~8 test files consume these runners and assert on their reports / event logs: `crates/tracewake-core/tests/{acceptance_gates,golden_scenarios,no_human_capstone,event_schema_replay_gates,generative_lock,emergence_ledger,anti_regression_guards}.rs` and `crates/tracewake-tui/tests/parity/scenario.rs` (via `app.run_no_human_day()`). The refactor must keep their behavioral assertions passing; their golden/checksum baselines change (item 5).
3. Cross-artifact boundary under audit: this is the information-path consolidation spec §8 mandates — "the no-human runner loops [the coordinator]; it does not own a parallel definition." The canonical end-state path is the coordinator's discovery (0047TUIAUTWOR-006) + lifecycle (0047TUIAUTWOR-007) + accounting (0047TUIAUTWOR-008); the batch-local pending vectors and the runner-local `append_due_completions` drain are the duplicate transport to remove. The proof surface (no-human capstone, golden scenarios, replay gates) must stay strong enough to debug the canonical path after the change.
4. Constitutional invariant motivating the ticket: `INV-091` (no-human tests are mandatory; the world continues without a human) and `INV-004` (the authoritative world ignores human existence). The runners retain their acceptance role while delegating time progression to the coordinator.
5. Enforcement surface (deterministic replay + no-human acceptance): the no-human path now emits one `TimeAdvanced` marker per tick (via the coordinator), so no-human golden traces and replay checksums **change** and must be regenerated under review as an intentional update (reassessment M2) — the marker is never suppressed to keep a golden green (§8). The change removes the batch-local pending authority entirely (no second source of truth for open durations, §8); replay must reconstruct identical final physical/agent state, frontier, duration state, need ledger, recovery, and output through the single path. The no-human capstone + replay-gate suites are the regression lock.

## Architecture Check

1. Looping the coordinator from both runners — rather than keeping `append_due_completions`'s batch-local drain — gives one semantic definition of a tick shared by human and no-human paths, which is the precondition for the human/no-human differential (0047TUIAUTWOR-016) to be meaningful (held-equal inputs must traverse identical progression). Two definitions would make the differential prove nothing.
2. No backwards-compatibility aliasing/shims: the batch-local pending vectors and the runner-local completion drain are deleted, not wrapped behind a fallback; `advance_one_tick`'s remaining role is the coordinator's private increment (0047TUIAUTWOR-005).

## Verification Layers

1. `INV-091` no-human -> replay/golden-fixture check: the no-human capstone and golden no-human scenarios pass (with regenerated baselines) and a duration started before the runner invocation completes after rebuild.
2. `INV-018` deterministic replay -> replay/golden-fixture check: regenerated no-human golden traces + checksums are stable across repeated runs; the empty-tick `TimeAdvanced` ancestry rebuilds the frontier.
3. Single authority -> codebase grep-proof: `pending_sleep_starts`/`pending_work_starts` and the runner-local `append_due_completions` drain are removed; no second open-duration source remains.

## What to Change

### 1. Refactor `run_no_human_day` onto the coordinator (`scheduler.rs`)

Replace the runner's per-window tick walk + `append_due_completions` drain with a loop over the coordinator's canonical step, preserving day windows, progress metrics, markers, and the `NoHumanDayReport` shape.

### 2. Refactor `advance_no_human` onto the coordinator (`scheduler.rs`)

Replace its `for expected_tick { advance_one_tick(); append_due_completions(...) }` loop with the coordinator step loop, preserving `NoHumanAdvanceReport`.

### 3. Remove the duplicate pending authority (`scheduler.rs`)

Delete `pending_sleep_starts`/`pending_work_starts` and the runner-local `append_due_completions` drain; open-duration discovery is the coordinator's log-derived authority (0047TUIAUTWOR-006).

### 4. Regenerate affected golden traces/checksums

Update the no-human golden baselines and replay checksums in the ~8 consumer tests to reflect the per-tick `TimeAdvanced` marker — an intentional, reviewed regeneration (M2), not a suppression.

## Files to Touch

- `crates/tracewake-core/src/scheduler.rs` (modify)
- `crates/tracewake-core/tests/acceptance_gates.rs` (modify — golden/checksum regeneration)
- `crates/tracewake-core/tests/golden_scenarios.rs` (modify — golden/checksum regeneration)
- `crates/tracewake-core/tests/no_human_capstone.rs` (modify — golden/checksum regeneration)
- `crates/tracewake-core/tests/event_schema_replay_gates.rs` (modify — golden/checksum regeneration)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — regeneration as surfaced)
- `crates/tracewake-tui/tests/goldens/` (modify — as surfaced; no-human parity goldens)

## Out of Scope

- TUI human one-tick wiring (0047TUIAUTWOR-011) and the advance-until controller (0047TUIAUTWOR-012).
- The human/no-human differential suite (0047TUIAUTWOR-016) — this ticket makes the no-human path loop the coordinator; the differential is proven separately.
- Changing any `NoHuman*` public type shape (preserved; only internals refactor).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core` — the full no-human suite (acceptance_gates, golden_scenarios, no_human_capstone, event_schema_replay_gates, generative_lock, emergence_ledger, anti_regression_guards) passes with regenerated golden/checksum baselines.
2. A duration started before the runner invocation (independent transaction) completes correctly after the refactor and after rebuild — proving the batch-local authority is gone.
3. `grep -n "pending_sleep_starts\|pending_work_starts" crates/tracewake-core/src/scheduler.rs` returns nothing (duplicate authority removed); `cargo test --workspace` passes (TUI parity scenario still drives `app.run_no_human_day()`).

### Invariants

1. There is one semantic definition of a loaded-world tick; the no-human runners loop the coordinator and own no parallel time-progression authority (`INV-091`/§8).
2. No second source of truth for open durations remains; replay reconstructs identical state through the single path (`INV-018`/`INV-019`).

## Test Plan

### New/Modified Tests

1. Modified golden/checksum baselines across the ~8 no-human consumer tests (regeneration, reviewed) — rationale: the per-tick `TimeAdvanced` marker changes event-log content (M2).
2. `crates/tracewake-core/tests/no_human_capstone.rs` (modify) — add a "duration from prior independent transaction completes under the refactored runner" assertion.

### Commands

1. `cargo test -p tracewake-core`
2. `cargo test --workspace`
3. `grep -rn "pending_sleep_starts\|pending_work_starts" crates/tracewake-core/src/` — must return zero matches (removal proof). Full-workspace test is the correct boundary because the TUI parity scenario consumes the no-human runner.

## Outcome

Completed: 2026-06-22

Refactored both no-human runners onto the shared world-step coordinator and removed the batch-local pending-duration authority. Added a regression proving an independently started sleep duration completes through `advance_no_human`, updated no-human replay/golden/TUI expectations for coordinator-driven tick accounting, and kept runner reports/metrics intact.

Verification:

- `cargo fmt --all --check`
- `cargo clippy -p tracewake-core --all-targets -- -D warnings`
- `cargo test -p tracewake-core`
- `rg -n "pending_sleep_starts|pending_work_starts" crates/tracewake-core/src/` returned no matches.
- `cargo test --workspace`
