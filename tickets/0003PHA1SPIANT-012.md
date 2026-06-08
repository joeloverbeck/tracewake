# 0003PHA1SPIANT-012: TUI proof-seam regression gate and adversarial checks

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-tui` (adversarial TUI tests; expanded direct-applier source scan)
**Deps**: None

## Problem

The TUI proof seam (current-view command path, debug quarantine, typed why-not, deterministic transcript) holds at this commit and is covered by existing tests (`embodied_flow.rs:7-90`, `transcript_snapshot.rs:9-40`, etc.). But the spine hardening must not weaken it, and the direct-applier source scan, stale-current-view rejection, and debug-not-a-command checks are not yet a named, expanded regression gate. Spec `0003` §5.8 / SPINE-AC-012 require keeping the TUI-proof tests as a required gate, expanding direct-applier scans, and adding adversarial stale-context / debug-no-affordance tests.

## Assumption Reassessment (2026-06-08)

1. The TUI current-view/submit/pipeline path is at `crates/tracewake-tui/src/app.rs:151-240`; debug panels at `app.rs:285-367` and `crates/tracewake-tui/src/debug_panels.rs:10-104`; typed rendering at `crates/tracewake-tui/src/render.rs:4-27`,`:210-225`; command parsing at `crates/tracewake-tui/src/input.rs:93-159`; deterministic transcript at `crates/tracewake-tui/src/transcript.rs:19-121`. TUI uses `run_pipeline` (`crates/tracewake-core/src/actions/pipeline.rs:158`), not direct event application. Existing tests: `crates/tracewake-tui/tests/{embodied_flow,transcript_snapshot,adversarial_gates,tui_acceptance}.rs`.
2. Spec `specs/0003_PHASE_1_SPINE_ANTI_CONTAMINATION_AND_STRUCTURAL_HARDENING_SPEC.md` §7 SPINE-AC-012 mandates: keep TUI-proof tests in the workspace gate; expand direct-applier source scans so the TUI cannot import/call event application directly except through `run_pipeline` or explicit replay/debug display paths; add adversarial tests for stale current-view context, debug command strings not accepted as embodied commands, and debug-panel rendering not altering semantic action lists; `cargo test --workspace` runs the re-verification suite; snapshot tests stay deterministic with debug non-diegetic markers.
3. Cross-artifact boundary under audit: the TUI presentation/possession seam vs. the kernel pipeline — the TUI may only submit through `run_pipeline` and may only render actor-filtered/debug-marked views. The re-verification table in spec §5.8 (TUI-AC-001…012) is the baseline this ticket locks as a regression gate.
4. INV motivating this ticket: `INV-008` (kernel authority; the TUI decides no legality), `INV-065`–`INV-070` (possession parity; typed why-not; no player privilege), `INV-093`–`INV-095`/`INV-107`/`INV-108` (debug non-diegetic; no bypass). Restated: the TUI is presentation + possession only — no direct event application, no debug truth as affordance, no stale/forged selection enacted.
5. Actor-knowledge / no-leak surface touched: the TUI seam is where hidden/debug truth could leak into embodied affordances. The adversarial tests assert debug panels do not alter the semantic action list or source context (no leak), and that a selection captured from a prior view is rejected after the view hash/frontier changes (no stale enactment). Snapshot determinism is preserved (`INV-018`); debug markers remain present.

## Architecture Check

1. Promoting the TUI-proof re-verification into a named, required regression suite with an expanded direct-applier scan ensures the spine hardening (especially the 001 reseal and 008 pipeline gate) cannot silently regress the TUI boundary, and the adversarial stale-context / debug-no-affordance tests close the specific bypass routes. This is a verification-strengthening ticket, not a gameplay change.
2. No backwards-compatibility shim: the TUI retains exactly one mutation route (`run_pipeline`); no direct event-application convenience path is introduced.

## Verification Layers

1. `INV-008`/`INV-104` (kernel authority; single mutation path) -> codebase grep-proof: an expanded source scan fails if the TUI imports/calls `apply_event`/`apply_event_stream` except via `run_pipeline` or explicit replay/debug display paths.
2. `INV-065`/`INV-069` (current-view authority; no stale enactment) -> adversarial TUI test: a numeric/semantic action captured from a previous view is rejected after the view hash/frontier changes.
3. `INV-107`/`INV-024` (debug non-diegetic; no leak) -> adversarial TUI test: opening/rendering a debug panel leaves the semantic action list and source context unchanged; debug command strings are not accepted as embodied commands.
4. `INV-018` (deterministic transcript) -> snapshot check: transcript snapshot stays byte-stable with debug non-diegetic markers.

## What to Change

### 1. Lock the TUI-proof suite as a required gate

Keep `embodied_flow.rs`, `transcript_snapshot.rs`, `adversarial_gates.rs`, `tui_acceptance.rs` in `cargo test --workspace`; mark the re-verification mapping (TUI-AC-001…012) as named tests.

### 2. Expand the direct-applier source scan

Add/extend a TUI source scan asserting no direct `apply_event`/`apply_event_stream` import or call outside `run_pipeline` / explicit replay/debug display paths.

### 3. Adversarial TUI tests

Add `tui_current_view_submission_rejects_stale_selection` (stale view hash/frontier), a debug-string-not-a-command test, and a debug-panel-no-affordance-change test.

## Files to Touch

- `crates/tracewake-tui/tests/adversarial_gates.rs` (modify — stale-selection, debug-not-command, debug-no-affordance)
- `crates/tracewake-tui/tests/tui_acceptance.rs` (modify — expanded direct-applier source scan; named re-verification mapping)
- `crates/tracewake-tui/tests/transcript_snapshot.rs` (modify — keep byte-stable snapshot with debug markers)

## Out of Scope

- Core-level debug/actor-known unforgeability (0003PHA1SPIANT-009) — this ticket is the TUI-level seam.
- Source-context core sealing (0003PHA1SPIANT-007).
- Any change to TUI gameplay, rendering content, or command vocabulary beyond the regression locks.

## Acceptance Criteria

### Tests That Must Pass

1. `tui_current_view_submission_rejects_stale_selection` — a selection from a previous view rejects after a view hash/frontier change.
2. Debug-panel rendering leaves the semantic action list and source context unchanged; debug strings are not accepted as embodied commands.
3. The expanded direct-applier source scan passes (TUI mutates only via `run_pipeline`); `cargo test --workspace` runs the full TUI re-verification suite.

### Invariants

1. The TUI is presentation + possession only; it applies no event directly and enacts no stale/forged selection (`INV-008`, `INV-104`).
2. Debug truth never becomes an embodied affordance; transcript snapshots stay byte-stable with debug markers (`INV-107`, `INV-018`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/adversarial_gates.rs` — `tui_current_view_submission_rejects_stale_selection`, debug-not-command, debug-no-affordance.
2. `crates/tracewake-tui/tests/tui_acceptance.rs` — expanded direct-applier scan.
3. `crates/tracewake-tui/tests/transcript_snapshot.rs` — byte-stable snapshot retained.

### Commands

1. `cargo test -p tracewake-tui`
2. `cargo test --workspace`
