# 0004PHA1THIHAR-009: Add debug-quarantine negative tests

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` debug/view-model negative tests + a compile-fail fixture (via the ticket-002 runner); `tracewake-tui` adversarial debug tests
**Deps**: 0004PHA1THIHAR-002

## Problem

The debug-capability boundary is structurally strong — `DebugCapability` has a private marker and `pub(crate)` minting, debug reports store a `DebugCapability`, and `debug_only()` exposes classification without exposing minting — but no negative tests prove debug truth *cannot* construct embodied view facts, actor-known context, affordances, or TUI actions, nor that a debug report cannot be forged outside the crate (spec §6 F-007, §8 THIRD-AC-010). Debug omniscience leaking into embodied cognition would violate the truth firewall.

## Assumption Reassessment (2026-06-08)

<!-- Items 1-3 always required; 4-5 selected for an invariant-motivated, no-leak-firewall ticket. -->

1. `crates/tracewake-core/src/debug_capability.rs` defines `DebugCapability` (private marker struct at `:22`) with `const fn debug_only()` at `:33`; `crates/tracewake-core/src/debug_reports.rs` stores the capability across report types, exposing `debug_only()` (`:101`–`:137`) without exposing minting. The TUI embodied path uses `build_embodied_view_model` / `EmbodiedProjectionSource` and `KnowledgeContext::embodied` (`crates/tracewake-tui/tests/adversarial_gates.rs:16`,`:209`); `crates/tracewake-core/tests/hidden_truth_gates.rs` is the core hidden-truth gate suite.
2. The remediation is spec §8 `THIRD-AC-010` + the §9.6 TUI/debug fixtures (`debug_report_construction_without_capability_compile_fails`, `debug_panel_does_not_change_embodied_affordances`, `debug_truth_never_enters_holder_known_context_hash`, `debug_command_strings_are_not_embodied_commands`, `tui_current_view_submission_rejects_stale_selection`, `tui_transcript_snapshot_remains_byte_stable`, `tui_transcript_marks_debug_sections_non_diegetic`), reassessed this session.
3. Shared boundary under audit: the firewall between core debug reports and the TUI embodied view. The compile-fail fixture reuses the negative-fixture runner harness from ticket 002 (declared in `Deps`). `hidden_truth_gates.rs` is also edited by ticket 001 (seed migration) — coordinate the mechanical merge.
4. Motivating invariants (restated): `INV-008` (UI is not authority), `INV-024` (no telepathy), `INV-068`/`INV-107` (debug omniscience is quarantined / non-diegetic), `INV-100` (hidden-truth cognition forbidden), `INV-108` (possession is cognition-neutral).
5. No-leak firewall surface: the enforcement surfaces are the `DebugCapability` boundary and the actor-knowledge filter feeding `build_embodied_view_model`. The fixtures prove debug truth never reaches the holder-known context hash or embodied affordances, and a debug report cannot be constructed without the capability. The ticket adds proofs only; it weakens no filter.

## Architecture Check

1. A compile-fail fixture (debug report unforgeable without `DebugCapability`) plus runtime negatives (opening a debug panel leaves embodied affordances and the holder-known context hash unchanged) prove the quarantine structurally, vs trusting the capability boundary by inspection. The compile-fail fixture reuses the ticket-002 runner, adding no new harness.
2. No backwards-compatibility shims: the tests add no production path; the debug boundary is unchanged.

## Verification Layers

1. `INV-107` (debug quarantine) -> negative-fixture (compile-fail via ticket-002 runner): `debug_report_construction_without_capability_compile_fails`.
2. `INV-024` / `INV-100` (no telepathy / hidden-truth cognition) -> runtime negative: `debug_panel_does_not_change_embodied_affordances`, `debug_truth_never_enters_holder_known_context_hash`.
3. `INV-068` / `INV-108` (debug non-diegetic) -> transcript test: `tui_transcript_marks_debug_sections_non_diegetic`, `debug_command_strings_are_not_embodied_commands`, `tui_current_view_submission_rejects_stale_selection`, `tui_transcript_snapshot_remains_byte_stable`.

## What to Change

### 1. Compile-fail: debug report unforgeable

Add a fixture crate (run by the ticket-002 harness) attempting to construct a debug report without `DebugCapability`; expected compile failure.

### 2. Core runtime negatives

In `hidden_truth_gates.rs`, prove that opening/computing a debug report does not alter embodied affordances and that debug truth never enters the holder-known context hash.

### 3. TUI adversarial debug tests

In `adversarial_gates.rs`, assert debug sections are marked non-diegetic, debug command strings are not embodied commands, stale current-view selections are rejected, and the transcript snapshot remains byte-stable.

## Files to Touch

- `tests/negative-fixtures/` (new — debug-construction compile-fail fixture; harness created by ticket 002)
- `crates/tracewake-core/tests/hidden_truth_gates.rs` (modify — shared w/ ticket 001: debug-truth/holder-known-hash runtime negatives)
- `crates/tracewake-tui/tests/adversarial_gates.rs` (modify — TUI debug non-diegetic / stale-selection / transcript-stability negatives)

## Out of Scope

- The negative-fixture runner harness itself (ticket 002).
- The conformance evidence-kind matrix (ticket 010), which records this ticket's negatives as debug-quarantine evidence.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --workspace` — the runtime and transcript negatives pass; the compile-fail fixture fails-as-expected under the ticket-002 runner.
2. `cargo test -p tracewake-tui` — the TUI adversarial debug negatives pass.
3. `cargo build --workspace --all-targets --locked` — the workspace (excluding the isolated fixture crate) compiles.

### Invariants

1. A debug report cannot be constructed outside the crate without `DebugCapability`.
2. Debug truth never alters embodied affordances or the holder-known context hash, and debug sections are non-diegetic.

## Test Plan

### New/Modified Tests

1. `tests/negative-fixtures/*` — `debug_report_construction_without_capability_compile_fails`; rationale: prove the capability is unforgeable.
2. `crates/tracewake-core/tests/hidden_truth_gates.rs` — `debug_panel_does_not_change_embodied_affordances`, `debug_truth_never_enters_holder_known_context_hash`; rationale: prove debug truth does not feed embodied cognition.
3. `crates/tracewake-tui/tests/adversarial_gates.rs` — `tui_transcript_marks_debug_sections_non_diegetic`, `debug_command_strings_are_not_embodied_commands`, `tui_current_view_submission_rejects_stale_selection`, `tui_transcript_snapshot_remains_byte_stable`; rationale: prove the TUI debug seam stays non-diegetic and stable.

### Commands

1. `cargo test -p tracewake-core --test hidden_truth_gates && cargo test -p tracewake-tui --test adversarial_gates`
2. `cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-09

What changed:

- Added the negative fixture `tests/negative-fixtures/external_crate_cannot_construct_debug_report`, which attempts to construct a `ControllerBindingDebugReport` and mint `DebugCapability` from outside `tracewake-core`.
- Registered that fixture in `crates/tracewake-core/tests/negative_fixture_runner.rs` and added the named test `debug_report_construction_without_capability_compile_fails`.
- Added `debug_truth_never_enters_holder_known_context_hash` to `crates/tracewake-core/tests/hidden_truth_gates.rs`.
- Added the named TUI transcript proof `tui_transcript_marks_debug_sections_non_diegetic` to `crates/tracewake-tui/tests/adversarial_gates.rs`.
- Retained and verified the already-present TUI negatives for debug affordance stability, debug command strings, stale current-view submissions, and transcript byte stability.

Deviations from original plan:

- The compile-fail fixture is executed through the existing negative-fixture runner with `cargo clippy`, matching the ticket-002 harness pattern already present in the repo.

Verification:

- `cargo test -p tracewake-core --test hidden_truth_gates` — passed, 8 tests.
- `cargo test -p tracewake-core --test negative_fixture_runner` — passed, 2 tests.
- `cargo test -p tracewake-tui --test adversarial_gates` — passed, 14 tests.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo fmt --all --check` — passed.
- `cargo test --workspace` — passed.
