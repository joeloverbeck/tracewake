# 0002PHA1KERTUI-016: Structured debug provenance reports

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adds the `debug_reports` module to `tracewake-core`.
**Deps**: 0002PHA1KERTUI-008, 0002PHA1KERTUI-013, 0002PHA1KERTUI-014

## Problem

Debug must answer "where is this item and what event put it there?" and "why was this action rejected?" without leaking that truth into embodied actor state (Spec 0002 §6.7, §17). This ticket adds the structured item-location report (§17.1), action-rejection report (§17.2), projection-rebuild report (§17.3), replay report surfacing (§17.4 → reuses ticket 013), and the controller-binding report — each explicitly marked non-diegetic. These are consumed by the TUI debug panels (021) and the capstone debug-leakage regression (022).

## Assumption Reassessment (2026-06-06)

1. No `debug_reports` module exists; registers `pub mod debug_reports;` in `crates/tracewake-core/src/lib.rs` (001), reading validation reports (008), the replay/rebuild report (013), and controller binding (014).
2. The report field sets are `specs/0002_…_SPEC.md` §17.1 (item-location: holder chain, last-location event, event chain, physical checksum, `debug_only`), §17.2 (rejection: failed stage, reason codes, checked facts, precondition trace, `mutation_attempted=false`, `checksum_before==checksum_after`), §17.3 (projection rebuild), §17.4 (replay). Every report carries an explicit `debug_only` non-diegetic marker.
3. Shared boundary under audit: the report types consumed by the TUI debug panels (021) and the leakage regression (022). They read truth but are a separate surface from the embodied view model (012).
4. Invariant motivating this ticket: INV-068 (debug mode is visibly non-diegetic) and INV-093 (actor-knowledge leakage is a high-severity defect) — debug reports may reveal truth but must never enter embodied actor state.
5. No-leak surface: debug reports are the *sanctioned* truth-revealing surface, kept strictly separate from the embodied view model (ticket 012). They are read-only over state/events and write nothing back to actor state; each is tagged `debug_only`. This ticket guarantees the reports cannot mutate world/actor state; the negative regression (opening debug does not change embodied facts/checksum) is ticket 022.

## Architecture Check

1. A dedicated read-only report module that returns `debug_only`-tagged structures (rather than threading debug truth through the view-model layer) keeps the leakage boundary explicit: debug reports are produced by functions the embodied projection never calls. Reusing the ticket-013 replay report for §17.4 avoids a second replay surface.
2. No backwards-compatibility shims: greenfield.

## Verification Layers

1. Non-diegetic marking (INV-068) -> unit test: every report carries `debug_only = true`.
2. No mutation from inspection (INV-093) -> unit test: producing any report leaves physical checksum and actor state unchanged (`checksum_before == checksum_after`).
3. Item-location provenance (§17.1) -> unit test: the item-location report names the last location-changing world event (or the fixture-origin marker before any movement).
4. Rejection provenance (§17.2; INV-070) -> unit test: the rejection report names the decisive failed stage and the checked facts, with `mutation_attempted = false`.

## What to Change

### 1. Debug reports

Add `crates/tracewake-core/src/debug_reports.rs` with: item-location report (§17.1), action-rejection report (§17.2), projection-rebuild report (§17.3), a replay-report accessor (§17.4 reusing ticket 013), and a controller-binding report. All read-only; all `debug_only`-tagged.

### 2. Registration

Add `pub mod debug_reports;` to `crates/tracewake-core/src/lib.rs`.

## Files to Touch

- `crates/tracewake-core/src/debug_reports.rs` (new)
- `crates/tracewake-core/src/lib.rs` (modify — add `pub mod debug_reports;`; file created by ticket 001)

## Out of Scope

- Rendering the reports in the TUI (ticket 021).
- The replay computation itself (ticket 013 — this ticket surfaces it).

## Acceptance Criteria

### Tests That Must Pass

1. The item-location report returns the holder chain and names the last location-changing event (or fixture-origin marker before movement).
2. The action-rejection report names the failed stage and checked facts with `mutation_attempted = false` and equal before/after checksum.
3. Producing any report leaves the physical checksum and actor state unchanged, and every report is `debug_only`.

### Invariants

1. Debug reports never mutate world or actor state.
2. Debug truth is carried only on `debug_only`-tagged structures, never on the embodied view model.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/debug_reports.rs` (unit tests) — provenance correctness, non-mutation, `debug_only` tagging.

### Commands

1. `cargo test -p tracewake-core debug_reports`
2. `cargo build --workspace`
3. Unit scope is correct; the embodied↔debug non-leakage round-trip regression runs in ticket 022.
