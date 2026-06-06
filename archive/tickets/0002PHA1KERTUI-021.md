# 0002PHA1KERTUI-021: TUI debug panels and deterministic transcript harness

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adds `debug_panels` and `transcript` modules to `tracewake-tui`.
**Deps**: 0002PHA1KERTUI-020, 0002PHA1KERTUI-016

## Problem

The TUI must expose debug panels for item location, action rejection, event log, controller binding, projection rebuild, and replay report, and preserve debug/embodied separation visually and structurally (Spec 0002 §15.5). It must also support deterministic transcript or snapshot tests (§15.6). This ticket adds the debug panels (rendering the ticket-016 reports, marked non-diegetic) and the transcript/snapshot harness that makes TUI play regression-testable.

## Assumption Reassessment (2026-06-06)

1. The TUI app/render/input exist from ticket 020; the structured debug reports exist from ticket 016. This adds `debug_panels.rs` and `transcript.rs` and declares them, modifying `tracewake-tui/src/lib.rs` (created by 001, populated by 020).
2. Required debug panels are `specs/0002_…_SPEC.md` §15.5 (controller binding, event log, item location, action rejection, projection rebuild, replay report); debug panels must be visually marked as debug/non-diegetic; transcript/snapshot tests are §15.6. Tests must prove opening debug does not change embodied view facts.
3. Shared boundary under audit: the debug panels render the ticket-016 `debug_only` reports through a surface kept separate from the embodied renderer (ticket 020); the transcript harness captures a deterministic play sequence.
4. Invariant motivating this ticket: INV-068 (debug mode is visibly non-diegetic) and INV-095 (TUI/view-model tests are acceptance tests; debug can explain truth separately) — the debug panels reveal truth but in a clearly-separated, non-diegetic surface.
5. No-leak + deterministic surface: debug panels render only the ticket-016 `debug_only` reports and never feed back into the embodied view model (ticket 012); opening a debug panel must not change embodied facts or the physical checksum. The transcript harness is deterministic (stable view-model ordering from ticket 012). This ticket adds no leakage path; the debug-non-leakage regression is asserted in ticket 022.

## Architecture Check

1. Rendering debug reports through a distinct `debug_panels` surface (not the embodied renderer) keeps the embodied/debug separation structural — the embodied render path never receives a `debug_only` report. A deterministic transcript harness (stable view-model + action ordering) makes the TUI itself regression-testable, satisfying the §15.6 / INV-095 acceptance requirement.
2. No backwards-compatibility shims: greenfield.

## Verification Layers

1. Non-diegetic debug (INV-068) -> manual review + unit test: debug panels are marked debug and render only `debug_only` reports; the embodied renderer cannot receive them.
2. Debug does not change embodied facts (INV-093) -> unit/transcript test: opening a debug panel leaves the embodied view model and physical checksum unchanged.
3. Deterministic transcript (§15.6; INV-095) -> snapshot test: a representative play transcript (view, action, rejection, wait, debug) is byte-identical across runs.

## What to Change

### 1. Debug panels

Add `crates/tracewake-tui/src/debug_panels.rs`: panels for controller binding, event log, item-location report, action-rejection report, projection-rebuild report, and replay report — each rendering the ticket-016 `debug_only` structures and visually marked non-diegetic.

### 2. Transcript harness

Add `crates/tracewake-tui/src/transcript.rs`: a deterministic transcript/snapshot capture of a play sequence for regression tests.

### 3. Registration

Declare `debug_panels` and `transcript` in `crates/tracewake-tui/src/lib.rs`.

## Files to Touch

- `crates/tracewake-tui/src/debug_panels.rs` (new)
- `crates/tracewake-tui/src/transcript.rs` (new)
- `crates/tracewake-tui/src/lib.rs` (modify — declare modules; file created by ticket 001, populated by ticket 020)

## Out of Scope

- The debug report computation (ticket 016 — this ticket renders it).
- The seven-fixture acceptance run (ticket 022 — this ticket provides the transcript harness it uses).

## Acceptance Criteria

### Tests That Must Pass

1. Debug panels render the item-location, rejection, event-log, binding, projection-rebuild, and replay reports, each marked non-diegetic.
2. Opening any debug panel leaves the embodied view model and physical checksum unchanged.
3. A representative play transcript (view → action → rejection → wait → debug) is deterministic across runs.

### Invariants

1. Debug panels render only `debug_only` reports and never feed the embodied view model.
2. The transcript harness is deterministic.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/transcript_snapshot.rs` — deterministic transcript over a representative session.
2. `crates/tracewake-tui/src/debug_panels.rs` (unit tests) — embodied facts/checksum unchanged on debug open.

### Commands

1. `cargo test -p tracewake-tui transcript_snapshot debug_panels`
2. `cargo build --workspace`
3. TUI-crate scope is correct; the cross-cutting debug-leakage regression over all panels runs in ticket 022.

## Outcome (2026-06-06)

Implemented `tracewake-tui::debug_panels` for item location, action rejection, event log, controller binding, projection rebuild, and replay report panels. Each panel asserts/render-checks `debug_only` input and is visibly marked `DEBUG NON-DIEGETIC`.

Extended `TuiApp` with read-only debug render accessors and checksum/context helpers. Opening debug panels renders core debug reports without feeding data back into embodied view models or mutating physical state. Added `tracewake-tui::transcript` with a deterministic representative session covering view, rejected semantic action, why-not, wait, and debug panels.

Verification:

1. `cargo test -p tracewake-tui transcript_snapshot debug_panels` failed because Cargo accepts only one test filter.
2. `cargo test -p tracewake-tui transcript_snapshot` passed.
3. `cargo test -p tracewake-tui debug_panels` passed.
4. `cargo test -p tracewake-tui` passed.
5. `cargo build --workspace` passed.
