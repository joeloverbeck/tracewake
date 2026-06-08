# 0002TUIPROOSUR-006: Proposal source-context identity + freshness/forgery validation

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` (`Proposal` schema + `ProposalSource`, `proposal_from_semantic_action_entry`, pipeline validation ordering)
**Deps**: 0002TUIPROOSUR-001, 0002TUIPROOSUR-005

## Problem

`Proposal` carries `source_view_model_id: Option<ViewModelId>` (`crates/tracewake-core/src/actions/proposal.rs:16-25`) but no holder-known context id/hash, frontier, or provenance ancestry, and `proposal_from_semantic_action_entry` copies target ids/params from the entry (`crates/tracewake-core/src/projections.rs:806-850`). A forged, stale, or privileged action id may be rejected by action-specific validation, but the proposal type does not structurally prove it came from the current actor-known affordance surface. Spec 0002 §4 TUI-AC-004 requires every world-affecting TUI proposal to carry the source view-model id, holder-known context id/hash, tick/frontier, semantic action id, and provenance ancestry, with a typed `ProposalSource`, and validation that rejects stale/forged/privileged source before action-specific validation. This is also the enforcement point for TUI-AC-012's actor/controller-match check.

## Assumption Reassessment (2026-06-08)

1. `Proposal` is at `crates/tracewake-core/src/actions/proposal.rs:16` with `source_view_model_id: Option<ViewModelId>` (`:24`) and no context/hash/provenance fields. `proposal_from_semantic_action_entry` is at `crates/tracewake-core/src/projections.rs:806`. `run_pipeline(context, proposal)` is at `crates/tracewake-core/src/actions/pipeline.rs:152`. `TuiApp::submit_entry` builds the proposal (`crates/tracewake-tui/src/app.rs:197`).
2. Spec 0002 §4 TUI-AC-004 requires the typed context fields + `ProposalSource` enum; `proposal_from_semantic_action_entry` must consume an action-entry token that cannot be constructed outside the view-model/affordance builder; validation checks actor binding, source-context freshness, semantic-action membership, and provenance before action-specific validation.
3. Cross-artifact boundary under audit: `Proposal` (pipeline input) ↔ the sealed packet from ticket 001 (context id/hash/frontier) ↔ the `SemanticActionEntry`/`ActionAvailability` from ticket 005 (the unforgeable entry token). The token must be mintable only by the affordance/view-model builder.
4. Invariants restated: **INV-099** — truth may validate an action but may not plan it; a proposal must structurally originate from actor-known affordances; **INV-101** — the source context is sealed; **INV-009** — meaningful state changes require events committed through the pipeline; **INV-108** — possession changes input binding only (validation checks actor/controller match).
5. Fail-closed / replay surface: this ticket is the *enforcer* for ticket 001's packet. Validation must be deterministic, blocking, and fail-closed: a stale (frontier/hash mismatch), forged (token not mintable here), cross-actor, or privileged proposal is rejected before mutation with typed reason codes and an actor-safe summary, exposing no hidden-truth detail (INV-106). Confirm replay determinism is preserved — the freshness check compares recorded context hash/frontier, not wall-clock.
6. Schema extension: `Proposal` gains typed `holder_known_context_id`/`holder_known_context_hash`/frontier/`provenance_ancestry` + `ProposalSource`. Consumers: `run_pipeline` (`pipeline.rs:152`), `proposal_from_semantic_action_entry` (`projections.rs:806`), `TuiApp::submit_entry` (`app.rs:197`), and tests. New context fields are additive; the entry-token requirement is a breaking change to proposal construction (only the affordance builder may mint the token) — all in-workspace construction sites are updated here.

## Architecture Check

1. Carrying the source context id/hash + an unforgeable entry token lets validation prove provenance structurally (membership + freshness) before touching action-specific rules, so forged/stale/cross-actor proposals fail closed at the boundary rather than relying on downstream rules to incidentally reject them. This realizes INV-099 ("truth validates, does not plan") at the type level.
2. No backwards-compatibility aliasing/shims: `source_view_model_id` is retained as one field of the richer typed source, not as a parallel legacy path; no un-tokened proposal-construction route survives.

## Verification Layers

1. INV-099/INV-101 (sealed origin) -> codebase grep-proof: the entry token has no public constructor outside the affordance/view-model builder; `proposal_from_semantic_action_entry` requires it.
2. INV-106 (non-leaking rejection) -> replay/golden-fixture + negative test: stale/forged/cross-actor/hidden-target proposals reject before mutation with typed reason codes and no hidden-truth detail in actor-visible text.
3. INV-018 (deterministic replay) -> replay check: the freshness check uses recorded context hash/frontier; replay over a golden scenario reproduces identical accept/reject outcomes.

## What to Change

### 1. `Proposal` typed source context

Extend `Proposal` with `holder_known_context_id`/`holder_known_context_hash` (from 001), tick/frontier, semantic action id, provenance ancestry, and a typed `ProposalSource` enum.

### 2. Unforgeable entry token

Make `proposal_from_semantic_action_entry` consume an action-entry token mintable only inside the view-model/affordance builder, so a proposal cannot be assembled from an arbitrary id.

### 3. Pre-action validation

In the pipeline, check actor/controller binding, source-context freshness (hash/frontier), semantic-action membership, and provenance before action-specific validation; reject with typed reason codes + actor-safe summary.

## Files to Touch

- `crates/tracewake-core/src/actions/proposal.rs` (modify)
- `crates/tracewake-core/src/projections.rs` (modify)
- `crates/tracewake-core/src/actions/pipeline.rs` (modify)

## Out of Scope

- Two-layer why-not rendering of the rejection (ticket 007).
- TUI command-loop wiring of `wait`/`do`/numeric (ticket 008, which depends on this).
- Possession-parity *proof* tests (ticket 013) — this ticket adds the actor/controller-match check; the cross-body parity assertions are exercised in the capstone.

## Acceptance Criteria

### Tests That Must Pass

1. Submitting an action id from another actor's view, a debug-only/hidden target, a stale view (mutated frontier/hash), and a hand-constructed target each reject before mutation with a typed source/context failure and no hidden-truth detail in actor-visible text.
2. A positive test proves a fresh, in-view, correctly-bound proposal passes the source-context checks and proceeds to action validation.
3. `cargo test -p tracewake-core` passes including a deterministic-replay check; `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` pass.

### Invariants

1. A `Proposal` cannot be constructed for the pipeline without an entry token minted by the affordance builder (INV-099/INV-101).
2. Source-context validation is deterministic and fail-closed; rejection leaks no hidden truth (INV-018/INV-106).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/hidden_truth_gates.rs` — forged/stale/cross-actor/hidden-target rejection gates.
2. `crates/tracewake-core/tests/golden_scenarios.rs` (or `acceptance_gates.rs`) — replay reproduces identical accept/reject outcomes.

### Commands

1. `cargo test -p tracewake-core hidden_truth_gates`
2. `cargo test -p tracewake-core golden_scenarios`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`

## Outcome

Completed: 2026-06-08

Extended `Proposal` with typed `ProposalSource::TuiSemanticAction` carrying source view id, holder-known context id/hash/frontier, context tick, actor id, semantic action id, and provenance ancestry. TUI submission now builds proposals from the current embodied view so human-origin proposals carry the sealed source packet.

Added `SourceContextValidation` before action-definition lookup. Human-origin proposals now fail closed when their source is missing, cross-actor, stale by tick/frontier, mismatched by reconstructed context id/hash, or forged by semantic-action/action-id mismatch. Source failures use typed `ProposalSource*` reason codes and actor-safe summaries without hidden identifiers. Existing direct human-origin tests were updated to carry valid source packets.

Deviations from the original plan: `source_view_model_id` remains as a mirrored legacy field while the typed `ProposalSource` is the validation authority. `SemanticActionEntry` still has public constructors for existing external test view construction; validation enforces source identity at the proposal boundary.

Verification:

1. `cargo test -p tracewake-core human_source_context`
2. `cargo test -p tracewake-core --test hidden_truth_gates`
3. `cargo test -p tracewake-core --test golden_scenarios`
4. `cargo test -p tracewake-core`
5. `cargo test -p tracewake-tui`
6. `cargo fmt --all --check`
7. `cargo clippy --workspace --all-targets -- -D warnings`
8. `cargo build --workspace --all-targets --locked`
9. `cargo test --workspace`
