# 0039SPICERMUT-016: Kill `view_models.rs` SPINE survivors with channel-quarantine + provenance witnesses

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adds behavior-witness tests in `tracewake-core` (test-only by default; a production correction in `view_models.rs` lands only if a survivor reveals a real defect, per spec §4.13).
**Deps**: None

## Problem

Wave B left 24 missed mutants in `crates/tracewake-core/src/view_models.rs` (spec §5.16), owning SPINE-03 (actor-filtered projection) and SPINE-07 (embodied/debug presentation). The cluster includes why-not/provenance stable IDs, empty/`xyzzy` debug diagnostics, `debug_only` flags across view types, and truth-belief mismatch non-diegetic markers. A `debug_only() == true` getter test is expressly insufficient; the certifying observation is channel quarantine and noninterference with embodied behavior.

## Assumption Reassessment (2026-06-18)

1. `view_models.rs` exposes `debug_only()` on multiple debug view types (`:492`, `:505`, `:519`, `:532`, `:545`, `:558`, `:583`, each delegating to `self.debug_capability.debug_only()`) and `debug_only_diagnostics()` at `:318` (verified by grep). The 24 seed-mutant identities are in `reports/0038_spine_cert_mutation_triage_register.md`.
2. Spec §5.16 is the implementation contract; `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md` and `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` govern embodied/debug view models (verified present).
3. Shared boundary under audit: the view-model seam where typed why-not/provenance values and `debug_only` flags route embodied vs debug content from the same event frontier.
4. Motivating invariants: `INV-006 — Possession transfers no world knowledge`, `INV-068 — Debug mode is visibly non-diegetic`, `INV-107 — Debug omniscience is quarantined`.
5. This ticket touches the no-leak / debug-quarantine enforcement surface: typed why-not and provenance values from a real rejected/blocked action must reach view-model and TUI/transcript consumption; stable IDs must be observable through typed serialization/snapshot or consumer routing (not literal getters); all debug view types built from the same frontier must remain in the debug channel; truth-belief mismatch views must carry a non-diegetic marker and never appear as actor-known facts or semantic actions; and possession must change viewpoint/input binding without transferring another actor's knowledge or debug facts. The witnesses only strengthen quarantine — no leakage is introduced — and there is **no schema shape change** (test additions, not a schema extension; the `view_models.rs` reference is the mutation target). This substrate feeds the SPINE-03/07 re-proof in ticket 021.

## Architecture Check

1. Paired embodied/debug view models from the same event frontier, with negative channel-routing per grouped `debug_only` mutant, catch the flag/string/provenance mutants through channel quarantine and noninterference — a `debug_only() == true` getter test does not.
2. No backwards-compatibility aliasing/shims: typed why-not/provenance values are carried from a real rejected/blocked action into view-model and TUI/transcript consumption, not asserted as getter literals.

## Verification Layers

1. INV-068/107 channel quarantine -> hidden-truth gate: paired embodied/debug view models from the same frontier prove all debug view types remain in the debug channel; a negative channel-routing case per grouped `debug_only` mutant shows the member-specific mutant causes leakage or rejection of the expected debug view.
2. Provenance/why-not observability -> spine-conformance seam check: typed why-not and provenance values from a real rejected/blocked action reach view-model and TUI/transcript consumption; stable IDs are observed through typed serialization/snapshot or consumer routing.
3. INV-006 possession neutrality -> hidden-truth gate: possession changes viewpoint/input binding without transferring another actor's knowledge or debug facts; truth-belief mismatch views carry a non-diegetic marker and never appear as actor-known facts/semantic actions.

## What to Change

### 1. Channel-quarantine + provenance matrix

In `hidden_truth_gates.rs` (with seam coverage in `spine_conformance.rs`), build paired embodied/debug view models from the same event frontier and prove all debug view types remain in the debug channel; carry typed why-not and provenance values from a real rejected/blocked action into view-model and TUI/transcript consumption, observing stable IDs via typed serialization/snapshot or consumer routing.

### 2. Non-diegetic + possession + negative-routing rows

Prove truth-belief mismatch views carry a non-diegetic marker and never appear as actor-known facts/semantic actions; exercise nonempty diagnostic rows so empty/`xyzzy` mutants alter an inspectable debug artifact; prove possession changes viewpoint/input binding without transferring knowledge/debug facts; add a negative channel-routing case for every grouped `debug_only` mutant.

### 3. Member matrix

Map all 24 historical mutants (plus any new run survivor in this file) to a concrete channel-quarantine, provenance, or non-diegetic-marker consequence.

## Files to Touch

- `crates/tracewake-core/tests/hidden_truth_gates.rs` (modify)
- `crates/tracewake-core/tests/spine_conformance.rs` (modify)
- `crates/tracewake-core/src/view_models.rs` (modify — only if a survivor reveals a real defect; default is test-only)

## Implementation Disposition (2026-06-18)

Current-code reassessment found the surviving `debug_only() -> true` mutants were not meaningfully killable while `DebugCapability::debug_only()` itself returned unconditional `true`. The implemented seam therefore makes `DebugCapability::debug_only()` compare the private marker to `DEBUG_NON_DIEGETIC_MARKER` and adds a `#[cfg(test)]` forged non-debug capability constructor so negative channel-routing witnesses can reject non-minted debug views.

The behavior witnesses landed in `crates/tracewake-core/src/view_models.rs` unit tests instead of modifying `hidden_truth_gates.rs` and `spine_conformance.rs`: this keeps the tests next to the private debug view fields required to forge non-debug instances without expanding production constructors or adding compatibility shims. The named integration suites still passed unchanged and remain the broader hidden-truth/conformance proof surface.

## Out of Scope

- Debug-report construction internals (ticket 017) and TUI rendering (ticket 018).
- The full mutation run (ticket 020); SPINE re-proof (ticket 021).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates` — passes with paired embodied/debug quarantine, non-diegetic-marker, possession-neutrality, and per-`debug_only`-mutant negative-routing rows.
2. `cargo test --locked -p tracewake-core --test spine_conformance` — passes with the why-not/provenance observability seam assertions.
3. `cargo mutants --workspace -f crates/tracewake-core/src/view_models.rs --no-shuffle` — all 24 historical survivors (and any newly enumerated one) are `caught`.

### Invariants

1. The certifying observation is channel quarantine + embodied noninterference; a `debug_only() == true` getter test never suffices.
2. Truth-belief mismatch details and debug-only tokens never reach embodied output; possession transfers no knowledge.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/hidden_truth_gates.rs` — paired embodied/debug channel quarantine + non-diegetic marker + possession neutrality + per-`debug_only`-mutant negative routing.
2. `crates/tracewake-core/tests/spine_conformance.rs` — why-not/provenance stable-ID observability via serialization/snapshot/routing.

### Commands

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates --test spine_conformance`
2. `cargo mutants --workspace -f crates/tracewake-core/src/view_models.rs --no-shuffle`
3. The per-file `-f` run is the correct verification boundary; the full standing campaign is ticket 020.

## Outcome

Completed: 2026-06-18

Changed `crates/tracewake-core/src/debug_capability.rs` so `DebugCapability::debug_only()` is true only for the private non-diegetic marker, and added a test-only forged non-debug constructor for negative quarantine witnesses.

Added `crates/tracewake-core/src/view_models.rs` behavior-witness tests that route every minted debug view through a typed debug channel, reject forged non-debug variants for each grouped `debug_only` survivor, preserve the truth-belief mismatch non-diegetic marker in routing, and snapshot typed why-not/provenance/diagnostic values through consumer-style routing rather than literal getter assertions.

Deviations from the original plan:

- The new witnesses live in `view_models.rs` unit tests, with existing `hidden_truth_gates.rs` and `spine_conformance.rs` retained as integration proof suites. This was necessary to forge private debug view fields without widening production APIs.
- `debug_capability.rs` received a narrow production correction because an unconditional `debug_only() == true` made the `debug_only -> true` mutants semantically live.
- The mutation command used `cargo mutants --no-config --workspace -C=--locked -f crates/tracewake-core/src/view_models.rs --no-shuffle` instead of the ticket's bare command, because ticket 001 installed the standing `.cargo/mutants.toml`; `--no-config` preserves this ticket's per-file Wave B proof boundary.

Verification:

- `cargo test --locked -p tracewake-core --lib view_models::tests` — passed, 11 tests.
- `cargo test --locked -p tracewake-core --lib debug_capability::tests` — passed, 1 test.
- `cargo test --locked -p tracewake-core --test hidden_truth_gates --test spine_conformance` — passed, 16 hidden-truth tests and 6 spine-conformance tests.
- `cargo mutants --no-config --workspace -C=--locked -f crates/tracewake-core/src/view_models.rs --no-shuffle` — passed with 50 mutants tested, 42 caught, 8 unviable, 0 missed.
- `cargo fmt --all --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo test --workspace --locked` — passed.
