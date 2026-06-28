# 0054FOUCONSIX-008: Publicly-forced `food_source` actor-known witness

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` projection witness + public actor-known/TUI behavior tests killing the const-true/false and ordering mutants; content fixture for competing food-source facts
**Deps**: None

## Problem

The `food_source_fact_supersedes` family is materially addressed but lacks a public actor-known forcing witness. `crates/tracewake-core/src/projections.rs` `food_source_fact_supersedes` (`:260`) and `crates/tracewake-core/tests/food_source_projection.rs` now exercise source-bearing / source-less replacement and deterministic source-key ordering through `build_embodied_view_model` (`projections.rs:599`); `.cargo/mutants.toml` includes the projection files in the standing perimeter (`:17`/`:38`-`:40`). But closure is via a **private/projection helper** (`food_source_fact_supersedes` is `fn`, not `pub fn`); no public actor-known/TUI witness with event/provenance ancestry forces the routed-forward pattern, and no current zero-survivor mutation run may be certified by static survey (finding F6-07).

## Assumption Reassessment (2026-06-27)

1. `crates/tracewake-core/src/projections.rs:260` defines `fn food_source_fact_supersedes(...)` (private), called at `:242` inside the supersession loop; `build_embodied_view_model` (`:599`) is the public projection entry. `crates/tracewake-core/tests/food_source_projection.rs` exists and exercises the direct semantics. Confirmed at `7660051`.
2. `crates/tracewake-content/src/fixtures/seeded_food_source_unknown_to_all_actors_001.rs` exists as a food-source fixture exemplar; `crates/tracewake-core/src/epistemics/projection.rs` exists (the actor-known projection surface). `.cargo/mutants.toml` already includes `projections.rs` (`:17`) and `epistemics/**` (`:38`-`:40`). Confirmed.
3. Shared boundary under audit: the actor-known projection surface (`projections.rs` / `epistemics/projection.rs`) and its public/TUI consumption. The witness must drive two competing food-source facts through modeled observation/record paths and prove the source-bearing later/stronger fact survives into the actor-visible view **without reading raw truth**.
4. INV-024 (no telepathy), INV-067 (embodied mode shows actor-known reality), INV-002/INV-023/INV-030 (belief before truth; records separate; evidence is not truth), INV-059 (leads/tasks are source-bound projections). Restated: the actor-known belief and rendered/decision consequence must arise from modeled channels, not ground truth.
5. No-leak / actor-knowledge surface: confirm the witness asserts the resulting actor-known belief and rendered/decision consequence **through a public boundary** without reading raw truth — the projection must not expose the losing/source-less fact's hidden provenance to the embodied viewer. No determinism regression: source-key ordering stays deterministic.
6. Schema/projection touch (additive-vs-breaking = **no shape change** for `projections.rs`; **additive-only** for the new content fixture): the supersession rule is already repaired — this ticket adds a public actor-known/TUI witness and a new competing-fact content fixture, not a projection schema change. The new fixture uses the existing content-pack fixture schema (additive content, new optional fixture file with provenance); existing consumers of the projection are unchanged. Keyed distinct from item 5.

## Architecture Check

1. A public actor-known/TUI witness forces the routed-forward supersession pattern at the production boundary the private helper cannot prove on its own, and killing the const-true/false + ordering mutants from a public path proves the rule is consequential to the actor-visible output (not merely internally consistent). This closes the "materially repaired but no public forcing witness" residual.
2. No backwards-compatibility aliasing/shims: the existing projection tests are kept and extended, not replaced; the witness adds public coverage rather than re-deriving the rule.

## Verification Layers

1. Actor-known supersession (INV-024/INV-067) → public projection/actor-known test: two food-source facts with controlled source presence and source keys introduced through modeled observation/record paths; assert the actor-known belief + rendered/decision consequence through a public boundary.
2. No raw-truth read (INV-002/INV-030) → manual-review + assertion that the losing/source-less fact's hidden provenance does not reach the embodied view.
3. Mutation sensitivity → the witness kills the constant true/false and ordering mutants on `food_source_fact_supersedes` (focused `cargo mutants -f projections.rs`; the standing run is ticket 009).

## What to Change

### 1. Public actor-known / TUI witness

Add a public projection/actor-known/TUI fixture where event/provenance ancestry creates two competing food-source facts and proves the source-bearing later/stronger fact survives into the actor-visible view without reading raw truth. Keep the existing `food_source_projection.rs` tests.

### 2. Competing-fact content fixture

Add a content fixture (modeled observation/record paths) producing the two competing food-source facts with controlled source presence and source keys.

### 3. Mutation perimeter confirmation

Confirm `.cargo/mutants.toml` / `.cargo/mutants-baseline-misses.txt` cover the projection seam for the witness (the files are already in the perimeter; adjust only if the witness adds a new file). The clean-baseline run is ticket 009 — do not claim current zero survivors here; accept no "equivalent mutant" claim without a semantic proof tied to the actor-known output.

## Files to Touch

- `crates/tracewake-core/src/projections.rs` (modify — only if a public witness accessor is required; no supersession-rule shape change)
- `crates/tracewake-core/src/epistemics/projection.rs` (modify — only if the actor-known witness needs a public projection seam)
- `crates/tracewake-core/tests/food_source_projection.rs` (modify)
- `crates/tracewake-content/src/fixtures/` (new — competing-food-source-facts fixture, registered in the fixtures module)
- a TUI seam/conformance test under `crates/tracewake-tui/tests/` or `crates/tracewake-tui/src/` (new — actor-visible rendered/decision consequence)
- `.cargo/mutants.toml` (modify — only if the witness adds a projection file not already in the perimeter)
- `.cargo/mutants-baseline-misses.txt` (modify — only if baseline-miss rows change)

## Out of Scope

- Reclassifying the `food_source_fact_supersedes` family to a lower tier or asserting equivalence without semantic proof (spec §1.2).
- Running the standing mutation campaign / certifying zero survivors (ticket 009).
- Any acceptance/governance/bootstrap/receipt/debug change (tickets 001–006).

## Acceptance Criteria

### Tests That Must Pass

1. A public projection/actor-known test: the source-bearing later/stronger food-source fact survives into the actor-visible view; the source-less/losing fact's hidden provenance does not leak to the embodied viewer.
2. A TUI seam/conformance test: the rendered/decision consequence reflects the surviving fact through a public boundary.
3. `cargo build --workspace --all-targets --locked && cargo test --workspace`.

### Invariants

1. The actor-known belief and its rendered/decision consequence arise only from modeled channels (INV-024/INV-067) — no raw-truth read.
2. Source-key ordering is deterministic; the const-true/false and ordering mutants on `food_source_fact_supersedes` are killed from a public path.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/food_source_projection.rs` — extended with the public actor-known competing-fact witness.
2. `crates/tracewake-content/src/fixtures/<competing-food-source-facts>.rs` — the modeled-observation/record fixture.
3. a TUI seam/conformance test — the actor-visible rendered/decision consequence.

### Commands

1. `cargo test -p tracewake-core --test food_source_projection && cargo test -p tracewake-tui`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
3. `cargo mutants -f crates/tracewake-core/src/projections.rs` — focused campaign confirming the witness kills the const/ordering mutants (the standing campaign is ticket 009).

## Outcome

Completed: 2026-06-27

Added the `competing_food_source_facts_001` content fixture and registered it in the fixture catalog. The fixture seeds an authored source-less known-food edge for `actor_tomas -> food_empty_bowl_tomas` while the public embodied view observes the same food source with zero servings, forcing the source-bearing serving fact to win through modeled actor-known channels.

Extended `crates/tracewake-core/tests/food_source_projection.rs` so the source-bearing/source-less ordering cases assert the public disabled consequence (`You know that food source is empty.`) and confirm no debug-only diagnostics leak into the action availability. Added a TUI acceptance test proving the fixture-backed public menu renders exactly one disabled `eat.food.food_empty_bowl_tomas` action with the public reason and without source-key provenance leakage. Added the new fixture fingerprint and source-classification entries required by existing guards.

Verification run:

- `cargo test -p tracewake-core --test food_source_projection` — passed.
- `cargo test -p tracewake-tui` — passed.
- `cargo mutants --no-config -f crates/tracewake-core/src/projections.rs --list` — selected 168 projections-file mutants.
- `cargo mutants --no-config -f crates/tracewake-core/src/projections.rs -F 'food_source_fact_supersedes|actor_known_food_sources_for_context'` — focused owned seam selected 13 mutants; 12 caught, 1 unviable, 0 missed.
- `cargo test -p tracewake-content --test golden_fixtures_run` — passed after adding the new frozen fixture fingerprint.
- `cargo fmt --all --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo test --workspace` — passed.

The exact unfiltered `cargo mutants -f crates/tracewake-core/src/projections.rs` standing/full campaign remains owned by ticket 009; this ticket records focused mutation proof for the food-source supersession seam only.

Unrelated pre-existing dirty paths left untouched: `.claude/skills/spec-to-tickets/SKILL.md`, `.claude/skills/spec-to-tickets/references/decomposition-patterns.md`, `CLAUDE.md`, and `tools/clean-build-scratch.sh`.
