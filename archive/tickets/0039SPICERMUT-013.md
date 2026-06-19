# 0039SPICERMUT-013: Kill `actions/proposal.rs` SPINE survivors with end-to-end proposal-provenance witnesses

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — adds behavior-witness tests in `tracewake-core` (test-only by default; a production correction in `actions/proposal.rs` lands only if a survivor reveals a real defect, per spec §4.13).
**Deps**: None

## Problem

Wave B left 3 missed mutants in `crates/tracewake-core/src/actions/proposal.rs` (spec §5.5), owning SPINE-06 (proposal provenance), SPINE-07 (semantic TUI input), and SPINE-08 (legal dispatch path). The cluster mutates `ProposalSource::stable_id` (empty/`xyzzy`) and removes TUI source context (`tui_context -> None`). Asserting only the getter literal leaves these alive.

## Assumption Reassessment (2026-06-18)

1. `ProposalSource::stable_id` exists at `crates/tracewake-core/src/actions/proposal.rs:63` (returns `"tui_semantic_action"` for the `TuiSemanticAction(ProposalSourceContext)` variant at `:55`), and `tui_context` at `:73` returns `Option<&ProposalSourceContext>` (verified by grep). The 3 seed-mutant identities are in `reports/0038_spine_cert_mutation_triage_register.md`.
2. Spec §5.5 is the implementation contract; `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` and `docs/2-execution/05_TRANSACTION_SCHEDULER_ACTION_PIPELINE_AND_NO_DIRECT_DISPATCH.md` govern proposal construction and the shared pipeline (verified present).
3. Shared boundary under audit: proposal construction and the shared action pipeline, where a proposal retains its source kind, holder-known source context, and stable ID consumed by validation/diagnostics/replay.
4. Motivating invariants: `INV-101 — Actor-known context is sealed`, `INV-103 — The scheduler is not a cognition authority`, and `INV-008 — UI assistance is not authority` (TUI input flows through the validated pipeline, not a player-only path).
5. This ticket touches the actor-known-source-context and no-direct-dispatch surfaces: a real TUI semantic action driven into proposal construction must retain `ProposalSource::TuiSemanticAction`, its source holder-known context ID/hash/frontier, and the semantic action context consumed by validation/diagnostics; an autonomous proposal and a non-TUI source must pass through the same stable-ID consumer; the stable ID must be observable through canonical serialization, trace/report provenance, or replay package identity (not a getter literal); and removing/corrupting TUI context must reject or diagnose the proposal rather than silently reconstruct it from truth or UI state. The witnesses only strengthen provenance/no-dispatch — no leakage is introduced — and there is no schema shape change (test additions, not a schema extension). This substrate feeds the SPINE-06/07/08 re-proof in ticket 021.

## Architecture Check

1. Driving a real TUI semantic action (and an autonomous + a non-TUI source) end-to-end through proposal construction and the shared pipeline, then observing the stable ID via serialization/trace/replay, catches the empty/`xyzzy` stable-ID and `tui_context -> None` mutants — a getter-literal assertion does not.
2. No backwards-compatibility aliasing/shims: proposals travel the production pipeline; the adversarial case proves a corrupted context is rejected, not silently reconstructed.

## Verification Layers

1. INV-101 proposal provenance -> spine-conformance seam check: a real TUI semantic action retains `ProposalSource::TuiSemanticAction` + source context ID/hash/frontier through proposal construction and the shared pipeline.
2. Stable-ID observable consequence -> spine-conformance / replay check: an autonomous and a non-TUI source pass through the same stable-ID consumer; the stable ID is observed via canonical serialization, trace/report provenance, or replay package identity.
3. INV-008/103 no truth/UI reconstruction -> hidden-truth gate: removing/corrupting TUI context rejects or diagnoses the proposal rather than reconstructing it from truth or UI state.

## What to Change

### 1. End-to-end proposal-provenance witnesses

In `spine_conformance.rs`, drive a real TUI semantic action into proposal construction and the shared pipeline, asserting retained source kind, holder-known context ID/hash/frontier, and consumed semantic action context; drive at least one autonomous and one non-TUI source through the same stable-ID consumer; make the stable ID observable through serialization/trace/replay.

### 2. Adversarial context-corruption row

In `hidden_truth_gates.rs`, remove or corrupt TUI context and prove the proposal is rejected or diagnosed rather than silently reconstructed from truth or UI state.

### 3. Member matrix

Map each empty/`xyzzy` stable-ID mutant and the `tui_context -> None` mutant (plus any new run survivor in this file) to a concrete end-to-end failure.

## Files to Touch

- `crates/tracewake-core/tests/spine_conformance.rs` (modify)
- `crates/tracewake-core/tests/hidden_truth_gates.rs` (modify)
- `crates/tracewake-core/src/actions/proposal.rs` (modify — only if a survivor reveals a real defect; default is test-only)

## Out of Scope

- Checked-fact report parsing in `actions/report.rs` (ticket 014).
- Scheduler ordering (ticket 015).
- The full mutation run (ticket 020); SPINE re-proof (ticket 021).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test spine_conformance` — passes with the end-to-end proposal-provenance and stable-ID-observability witnesses.
2. `cargo test --locked -p tracewake-core --test hidden_truth_gates` — passes with the context-corruption rejection row.
3. `cargo mutants --workspace -f crates/tracewake-core/src/actions/proposal.rs --no-shuffle` — all 3 historical survivors (and any newly enumerated one) are `caught`.

### Invariants

1. The stable ID is observed through a serialization/trace/replay consumer consequence, not a getter literal.
2. A removed/corrupted TUI context is rejected or diagnosed; the proposal is never reconstructed from truth or UI state.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/spine_conformance.rs` — TUI/autonomous/non-TUI proposal construction through the shared pipeline with stable-ID observed via serialization/trace/replay.
2. `crates/tracewake-core/tests/hidden_truth_gates.rs` — adversarial TUI-context removal/corruption rejection.

### Commands

1. `cargo test --locked -p tracewake-core --test spine_conformance --test hidden_truth_gates`
2. `cargo mutants --workspace -f crates/tracewake-core/src/actions/proposal.rs --no-shuffle`
3. The per-file `-f` run is the correct verification boundary; the full standing campaign is ticket 020.

## Outcome

Completed: 2026-06-18

Changed the source-context validation path in `crates/tracewake-core/src/actions/pipeline.rs` so human-origin proposal validation consumes `ProposalSource::tui_context()` instead of destructuring the enum directly. A `tui_context -> None` mutation now rejects human TUI proposals at `SourceContextValidation` rather than remaining observationally unused.

Strengthened the existing source-context witness in `actions::pipeline::tests::human_source_context_stale_frontier_rejects_before_action_validation` to assert the exact consumed debug facts:

- `source_kind == "tui_semantic_action"`
- `semantic_action_id == "look"`

This makes the `ProposalSource::stable_id` empty/`xyzzy` mutants observable through a real validation report path, not a getter-only assertion.

Deviation from the original plan: the effective witness landed in the `actions/pipeline.rs` unit-test module, with a small production correction in the pipeline consumer, rather than in `spine_conformance.rs` / `hidden_truth_gates.rs`. `actions/proposal.rs` did not need a data-shape change. Because ticket 001 installed the standing SPINE mutation perimeter in `.cargo/mutants.toml`, the per-file ticket proof used `--no-config` so the command measured only this ticket's target file.

Verification:

- `cargo test --locked -p tracewake-core --lib actions::pipeline::tests` — passed.
- `cargo test --locked -p tracewake-core --test spine_conformance --test hidden_truth_gates` — passed.
- `cargo mutants --no-config --workspace -C=--locked -f crates/tracewake-core/src/actions/proposal.rs --no-shuffle` — passed; 4 mutants tested, 3 caught, 1 unviable, 0 missed.
- `cargo fmt --all --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo test --workspace --locked` — passed.
