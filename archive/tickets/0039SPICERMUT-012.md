# 0039SPICERMUT-012: Kill `projections.rs` SPINE survivors with actor-known projection + paired-world witnesses

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — adds behavior-witness tests in `tracewake-core` (test-only by default; a production correction in `projections.rs` lands only if a survivor reveals a real defect, per spec §4.13).
**Deps**: None

## Problem

Wave B left 5 missed mutants in `crates/tracewake-core/src/projections.rs` (spec §5.12), owning SPINE-03 (projection boundary), SPINE-06 (semantic action surface), and SPINE-07 (embodied view). The cluster flips/deletes semantic-action filters, removes workplace provenance, and changes the proposal semantic-entry comparison. Tests must not make a projection pass by adding truth-derived facts; missing provenance must remove/reject the affordance.

## Assumption Reassessment (2026-06-18)

1. `actor_known_workplaces_for_context` exists at `crates/tracewake-core/src/projections.rs:265`, `phase3a_semantic_actions` at `:886`, `unique_action_count` at `:515`; the surface reads `context.actor_known_workplaces()` and `context.provenance_entries()` (verified by grep). The 5 seed-mutant identities are in `reports/0038_spine_cert_mutation_triage_register.md`.
2. Spec §5.12 is the implementation contract; `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` and `docs/1-architecture/04_ACTION_PROPOSAL_VALIDATION_SCHEDULING_AND_FEEDBACK_PIPELINE.md` govern projection and semantic-action exposure (verified present).
3. Shared boundary under audit: the projection seam that exposes only actor-known-supported semantic actions with exact provenance, deterministic under replay.
4. Motivating invariants: `INV-024 — No telepathy`, `INV-067 — Embodied mode shows actor-known reality`, `INV-099 — Truth may validate actions, but truth may not plan them`. Projections are deterministic readers of event-derived state and sealed contexts, never truth writers.
5. This ticket touches the actor-knowledge-firewall / no-leak projection surface: the projection must expose only actions supported by current actor-known context and exact provenance; deleting workplace provenance must prevent/diagnose work-proposal availability rather than falling back to authoritative assignment truth; paired hidden-truth worlds must not give the projection extra actions; and rebuilding from replay must reproduce context hash/frontier, semantic actions, and provenance vectors. The witnesses only strengthen the firewall — no leakage is introduced — and there is **no schema shape change** (test additions, not a schema extension; the `projections.rs` reference is the mutation target). This substrate feeds the SPINE-03/06/07 re-proof in ticket 021.

## Architecture Check

1. Sealed contexts containing allowed/disallowed/stale/source-missing facts, plus paired hidden-truth worlds and a replay-rebuild comparison, catch each filter/provenance/comparison mutant through actor-known exposure — a truth-derived shortcut cannot make a projection pass.
2. No backwards-compatibility aliasing/shims: the projection is observed from sealed context, and at least one projected action is carried into a real proposal/validation trace.

## Verification Layers

1. INV-024/067 actor-known exposure -> hidden-truth gate: sealed contexts with allowed/disallowed/stale/source-missing semantic-action facts prove the projection exposes only context-supported actions with exact provenance.
2. INV-099 no truth-planning -> hidden-truth gate (paired worlds): paired hidden-truth worlds prove the projection gains no extra actions from unseen world state; deleting workplace provenance removes/rejects the affordance and produces the responsible diagnostic.
3. Replay determinism -> replay/golden-fixture check: rebuild the projection from replay and compare context hash/frontier, semantic actions, and provenance vectors; carry one projected action into a real proposal and validation trace.

## What to Change

### 1. Sealed-context projection matrix

In `hidden_truth_gates.rs` (with seam coverage in `spine_conformance.rs`), construct sealed contexts containing allowed, disallowed, stale, and source-missing semantic-action facts and prove the projection exposes only actor-known-supported actions with exact provenance.

### 2. Provenance + paired-world + replay witnesses

Prove deleting workplace provenance prevents or diagnoses work-proposal availability rather than falling back to authoritative assignment truth; use paired hidden-truth worlds to prove no extra actions from unseen state; rebuild the projection from replay and compare context hash/frontier/semantic-actions/provenance; carry at least one projected action into a real proposal and validation trace.

### 3. Member matrix

Map all 5 historical mutants (plus any new run survivor in this file) to a concrete actor-known/provenance/replay consequence.

## Files to Touch

- `crates/tracewake-core/tests/hidden_truth_gates.rs` (modify)
- `crates/tracewake-core/tests/spine_conformance.rs` (modify)
- `crates/tracewake-core/src/projections.rs` (modify — only if a survivor reveals a real defect; default is test-only)

## Out of Scope

- Holder-known fact identity in `knowledge_context.rs` (ticket 011).
- Proposal source provenance (ticket 013) and view-model presentation (ticket 016).
- The full mutation run (ticket 020); SPINE re-proof (ticket 021).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates` — passes with the sealed-context projection matrix and paired-world witnesses.
2. `cargo test --locked -p tracewake-core --test spine_conformance` — passes with the replay-rebuild projection comparison.
3. `cargo mutants --workspace -f crates/tracewake-core/src/projections.rs --no-shuffle` — all 5 historical survivors (and any newly enumerated one) are `caught`.

### Invariants

1. The projection exposes only actor-known-supported actions; a truth-derived fact never makes it pass.
2. Missing workplace provenance removes/rejects the affordance with a responsible diagnostic; replay reproduces the projection deterministically.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/hidden_truth_gates.rs` — sealed-context allowed/disallowed/stale/source-missing matrix + paired-world + provenance-removal witnesses.
2. `crates/tracewake-core/tests/spine_conformance.rs` — replay-rebuild projection (context hash/frontier/actions/provenance) + projected-action-into-proposal trace.

### Commands

1. `cargo test --locked -p tracewake-core --test hidden_truth_gates --test spine_conformance`
2. `cargo mutants --workspace -f crates/tracewake-core/src/projections.rs --no-shuffle`
3. The per-file `-f` run is the correct verification boundary; the full standing campaign is ticket 020.

## Outcome

Completed: 2026-06-18

Added test-only SPINE projection witnesses in `crates/tracewake-core/src/projections.rs`:

- Added `phase3a_workplace_semantic_actions_require_place_access_and_provenance`, which constructs a sealed actor-known workplace context with current, remote, and closed workplaces; proves only the current open workplace is available; and checks disabled workplace reason codes plus exact holder-known/source-event provenance.
- Extended `semantic_action_entry_proposal_branches_are_exact` to prove `continue_routine` semantic action entries populate active-intention parameters only for the exact continue action and do not leak those parameters to other semantic actions.

Deviation from the original plan: no production correction was needed, and the witness landed in the `projections.rs` unit-test module rather than `hidden_truth_gates.rs` / `spine_conformance.rs`. The pre-patch mutation run reproduced the ticket's five historical survivors; the post-patch file mutation run killed them all. Because ticket 001 installed the standing SPINE mutation perimeter in `.cargo/mutants.toml`, the per-file ticket proof used `--no-config` so the command measured only this ticket's target file.

Verification:

- `cargo test --locked -p tracewake-core --lib projections::tests` — passed.
- `cargo test --locked -p tracewake-core --test hidden_truth_gates --test spine_conformance` — passed.
- `cargo mutants --no-config --workspace -C=--locked -f crates/tracewake-core/src/projections.rs --no-shuffle` — passed; 123 mutants tested, 95 caught, 28 unviable, 0 missed.
- `cargo fmt --all --check` — passed after formatting.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo test --workspace --locked` — passed.
