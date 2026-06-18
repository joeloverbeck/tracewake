# 0039SPICERMUT-019: Kill `tui/transcript.rs` SPINE survivor with representative-section + quarantine witnesses

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — adds behavior-witness tests in `tracewake-tui` (test-only by default; a production correction in `tui/transcript.rs` lands only if a survivor reveals a real defect, per spec §4.13).
**Deps**: None

## Problem

Wave B left 1 missed mutant in `crates/tracewake-tui/src/transcript.rs` (spec §5.18), owning SPINE-07 (transcript evidence). The historical mutant reverses the representative-section comparison. The selection must be observable through the archived transcript evidence package, not only an internal filter result.

## Assumption Reassessment (2026-06-18)

1. `crates/tracewake-tui/src/transcript.rs` exists (verified present); §5.18 describes the surviving representative-section comparison rather than naming a specific function symbol — the 1 exact mutant identity is the seed work-list entry in `reports/0038_spine_cert_mutation_triage_register.md`, mapped at implementation time.
2. Spec §5.18 is the implementation contract; `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` and `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` govern the transcript proof surface (verified present).
3. Shared boundary under audit: the transcript seam that selects representative sections in canonical order for the archived transcript evidence package.
4. Motivating invariant: `INV-067 — Embodied mode shows actor-known reality` (debug-only sections do not enter the ordinary transcript).
5. This ticket touches the no-leak / transcript-quarantine surface: the intended representative sections must be selected in canonical order with unrelated sections excluded; the same state run twice must produce the same semantic transcript section sequence; and paired embodied/debug transcripts must prove debug-only sections do not enter the ordinary transcript. The witnesses only strengthen quarantine + determinism — no leakage or nondeterminism is introduced — and there is no schema shape change (test additions, not a schema extension). This substrate feeds the SPINE-07 re-proof in ticket 021.

## Architecture Check

1. A deterministic transcript with required and non-required sections, run twice for sequence stability and paired embodied/debug, catches the reversed representative-section comparison through the archived evidence package — not an internal filter result alone.
2. No backwards-compatibility aliasing/shims: selection is observed through the archived transcript evidence package.

## Verification Layers

1. Representative-section selection -> transcript snapshot check: intended representative sections are selected in canonical order and unrelated sections are excluded; the same state run twice yields the same semantic transcript section sequence.
2. INV-067 quarantine -> TUI seam check: paired embodied and debug transcripts prove debug-only sections do not enter the ordinary transcript; selection is observable through the archived transcript evidence package.

## What to Change

### 1. Representative-section + determinism witnesses

In `transcript_snapshot.rs`, build a deterministic transcript with multiple required and non-required sections; prove the intended representative sections are selected in canonical order and unrelated sections are excluded; run the same state twice and compare the semantic transcript section sequence.

### 2. Paired embodied/debug quarantine

In `tui_seam_conformance.rs`, pair embodied and debug transcripts to prove debug-only sections do not enter the ordinary transcript; make the selection observable through the archived transcript evidence package, not only an internal filter result.

### 3. Member matrix

Map the 1 historical mutant (and any new run survivor in this file) to the reversed-comparison failure observed in the archived transcript.

## Files to Touch

- `crates/tracewake-tui/tests/transcript_snapshot.rs` (modify)
- `crates/tracewake-tui/tests/tui_seam_conformance.rs` (modify)
- `crates/tracewake-tui/src/transcript.rs` (modify — only if a survivor reveals a real defect; default is test-only)

## Implementation Disposition (2026-06-18)

Current-code reassessment confirmed the historical survivor remained in `capture_representative_transcript_sections`: the wait-action selector could be reversed and still satisfy the prior transcript checks.

The implemented witness is a test-only assertion in `crates/tracewake-tui/src/transcript.rs` because the per-file mutation test slice needed the exact package unit test to observe the selected action. It asserts the canonical representative section order and that `view.after_wait` advances to `Tick: 1`, so a reversed selector cannot pass by submitting a different current action.

## Out of Scope

- Render-section predicates (ticket 018).
- The full mutation run (ticket 020); SPINE re-proof (ticket 021).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-tui --test transcript_snapshot` — passes with canonical representative-section selection and twice-run sequence stability.
2. `cargo test --locked -p tracewake-tui --test tui_seam_conformance` — passes with the paired embodied/debug transcript quarantine.
3. `cargo mutants --workspace -f crates/tracewake-tui/src/transcript.rs --no-shuffle` — the historical survivor (and any newly enumerated one) is `caught`.

### Invariants

1. The selection is observed through the archived transcript evidence package, not an internal filter result alone.
2. Debug-only sections never enter the ordinary transcript; the section sequence is deterministic across identical runs.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/transcript_snapshot.rs` — deterministic representative-section selection + twice-run sequence-stability comparison.
2. `crates/tracewake-tui/tests/tui_seam_conformance.rs` — paired embodied/debug transcript quarantine through the archived evidence package.

### Commands

1. `cargo test --locked -p tracewake-tui --test transcript_snapshot --test tui_seam_conformance`
2. `cargo mutants --workspace -f crates/tracewake-tui/src/transcript.rs --no-shuffle`
3. The per-file `-f` run is the correct verification boundary; the full standing campaign is ticket 020.

## Outcome

Completed: 2026-06-18

Added a transcript behavior witness that compares the exact representative section sequence across two runs and verifies the post-wait section reflects the intended wait action by rendering `Tick: 1`. Existing snapshot and seam-conformance tests continue to prove byte stability and debug-section quarantine.

Deviations from the original plan:

- `transcript_snapshot.rs` and `tui_seam_conformance.rs` remained unchanged; their existing target suites passed. The new branch-killing assertion landed in `transcript.rs` unit tests because cargo-mutants exercised this file through the package test slice.
- The mutation command used `cargo mutants --no-config --workspace -C=--locked -f crates/tracewake-tui/src/transcript.rs --no-shuffle` instead of the ticket's bare command, because ticket 001 installed the standing `.cargo/mutants.toml`; `--no-config` preserves this ticket's per-file Wave B proof boundary.

Verification:

- `cargo test --locked -p tracewake-tui --lib transcript::tests` — passed, 1 transcript unit test.
- `cargo test --locked -p tracewake-tui --test transcript_snapshot --test tui_seam_conformance` — passed, 3 transcript snapshot tests and 2 seam-conformance tests.
- `cargo mutants --no-config --workspace -C=--locked -f crates/tracewake-tui/src/transcript.rs --no-shuffle` — passed with 8 mutants tested, 6 caught, 2 unviable, 0 missed.
- `cargo fmt --all --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo test --workspace --locked` — passed.
