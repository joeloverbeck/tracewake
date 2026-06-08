# 0002TUIPROOSUR-009: Quarantine no-human execution as non-diegetic operator/proof tooling

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-tui` (operator-command classification, no-human dispatch/render)
**Deps**: 0002TUIPROOSUR-002, 0002TUIPROOSUR-004

## Problem

`run no-human-day` is exposed as a top-level command alongside embodied commands: the parser yields `RunNoHumanDay` (`crates/tracewake-tui/src/input.rs:4-16,66-70`) and the loop calls `app.run_no_human_day()` then renders a debug panel plus the embodied view (`crates/tracewake-tui/src/run.rs:64-77`; `crates/tracewake-tui/src/app.rs:249-264`). It is valuable for proof but must be classified and tested as non-diegetic operator/proof tooling, not embodied player agency. Spec 0002 §4 TUI-AC-006 requires it to be typed as an operator/proof command, rendered only via debug/proof-report types carrying the sealed debug capability, with the subsequent embodied view rebuilt from the bound actor's sealed context — not from debug report rows.

## Assumption Reassessment (2026-06-08)

1. The parser exposes `RunNoHumanDay` in the `UiCommand` enum (`crates/tracewake-tui/src/input.rs:13`, parsed `:66-70`); `run.rs:64-77` dispatches `app.run_no_human_day()` then renders a debug panel + embodied view; `TuiApp::run_no_human_day` advances the scheduler and returns a report (`crates/tracewake-tui/src/app.rs:249-264`). The no-human report type is `NoHumanDayDebugReport` (`crates/tracewake-core/src/debug_reports.rs`).
2. Spec 0002 §4 TUI-AC-006: type `run no-human-day` as an operator/proof command (e.g. an `OperatorProofCommand` classification distinct from embodied commands); render its result only through debug/proof-report types carrying the sealed debug capability; rebuild the next embodied view from the bound actor's sealed context.
3. Cross-artifact boundary under audit: the TUI command classification (`input.rs`/`run.rs`) ↔ the no-human report (`debug_reports.rs`, capability-gated by ticket 004) ↔ the embodied rebuild path (`current_view`, sealed context, ticket 002).
4. Invariants restated: **INV-068** — debug/operator output is visibly non-diegetic and never confused with embodied knowledge; **INV-091** — no-human simulation tests are mandatory (the command must keep working headless); **INV-107** — debug omniscience is quarantined and may not create actor knowledge; **INV-004** — the world runs without a human controller.
5. Fail-closed / quarantine surface: confirm no-human debug metrics never enter actor knowledge — after `run no-human-day`, the next embodied view is rebuilt from the bound actor's sealed context (ticket 002), not from debug report rows, and the report renders only behind the debug capability (ticket 004). The operator command is classified separately from embodied commands so it cannot be mistaken for an actor action.

## Architecture Check

1. Classifying `run no-human-day` as a distinct operator/proof command (not an embodied `UiCommand` variant routed through affordances) makes its non-diegetic status structural: it cannot flow through the embodied proposal path, and its output is reachable only via the capability-gated debug report — so it cannot masquerade as actor agency or seed actor knowledge (INV-068/INV-107).
2. No backwards-compatibility aliasing/shims: the command is reclassified, not duplicated; the embodied rebuild after it uses the sealed-context path, with no debug-row carryover retained.

## Verification Layers

1. INV-107 (quarantine) -> codebase grep-proof: the no-human report renders only through the capability-gated debug path; the post-run embodied view is built via the sealed-context `current_view`.
2. INV-068 (non-diegetic) -> transcript/type test: no actor-visible notebook/action/why-not output cites no-human debug metrics as actor knowledge.
3. INV-091/INV-004 (no-human run) -> replay/golden-fixture check: `no_human_day_001` advances headlessly and remains deterministic.

## What to Change

### 1. Operator-command classification

Reclassify `run no-human-day` as an `OperatorProofCommand` (or equivalent) distinct from embodied `UiCommand` variants in `input.rs`/`run.rs`.

### 2. Capability-gated render + sealed rebuild

Render the no-human result only via the debug/proof report behind the debug capability (ticket 004); rebuild the subsequent embodied view from the bound actor's sealed context (ticket 002), never from report rows.

## Files to Touch

- `crates/tracewake-tui/src/input.rs` (modify)
- `crates/tracewake-tui/src/run.rs` (modify)
- `crates/tracewake-tui/src/app.rs` (modify)

## Out of Scope

- The debug capability type itself (ticket 004).
- The sealed-context embodied builder (ticket 002).
- No-human simulation *mechanics* in core (`run_no_human_day` advance logic is unchanged; only its TUI classification/render is hardened).

## Acceptance Criteria

### Tests That Must Pass

1. A transcript/type-level test proves no actor-visible notebook, action, or why-not output cites no-human debug metrics as actor knowledge unless a later ordinary event/record made it actor-known.
2. After `run no-human-day`, the next embodied view is shown to be built from the bound actor's sealed context (not debug rows), and the report renders only behind the debug capability.
3. `cargo test -p tracewake-tui` passes; `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` pass.

### Invariants

1. No-human output is non-diegetic and creates no actor knowledge (INV-068/INV-107).
2. The no-human path stays headless and deterministic (INV-004/INV-091).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/tui_acceptance.rs` — no-human operator command is non-diegetic; post-run embodied view from sealed context.
2. `crates/tracewake-core/tests/no_human_capstone.rs` — confirm no-human advance stays deterministic under the reclassification (no behavior regression).

### Commands

1. `cargo test -p tracewake-tui tui_acceptance`
2. `cargo test -p tracewake-core no_human_capstone`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`

## Outcome

Completed: 2026-06-08

Changed:
- Reclassified `run no-human-day` as `UiCommand::OperatorProof(OperatorProofCommand::RunNoHumanDay)`, separate from embodied command variants.
- Removed the raw `Ran no-human day: ... ordinary_events=...` command-loop summary so the run report is rendered only through `render_debug_no_human_day_panel`.
- Preserved the subsequent actor view render through `render_current_view()`, which rebuilds from the bound actor's sealed holder-known context.
- Added acceptance coverage that the post-run embodied view's holder-known context frontier matches the event log frontier, contains no debug metric rows, and does not leak hidden pantry food.
- Updated command-loop transcript coverage to require the no-human output to be `DEBUG NON-DIEGETIC: No Human Day` only, with no raw operator summary.

Deviations:
- `crates/tracewake-tui/src/app.rs` did not require code changes; `run_no_human_day`, `render_debug_no_human_day_panel`, and `current_view()` already provided the needed headless run, debug-only report, and sealed-context rebuild boundaries.
- The requested `cargo test -p tracewake-tui tui_acceptance` filter matches no test names in the current suite, so the meaningful no-human acceptance proof was run with `cargo test -p tracewake-tui no_human_day` plus the full TUI suite.

Verification:
- `cargo test -p tracewake-tui tui_acceptance` (0 matching tests)
- `cargo test -p tracewake-tui no_human_day`
- `cargo test -p tracewake-core no_human_capstone`
- `cargo test -p tracewake-tui`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`
