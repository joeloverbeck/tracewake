# 0053FOUCONFIF-007: Close the `food_source_fact_supersedes` survivor family

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` projection behavior (food-source supersession rule) + public behavior tests killing the seven survivors
**Deps**: 0053FOUCONFIF-001

## Problem

Spec 0053 §4.5 / §5 (F5-05): seven `food_source_fact_supersedes` mutants survive — constant `true`; constant `false`; deletion of the `Some` arm; deletion of the `None` arm; `<`→`==`; `<`→`>`; `<`→`<=`. They are cross-cutting and were honestly not laundered as equivalent, but a route-forward with no forcing function keeps the canonical standing perimeter non-green indefinitely. Per the approved disposition (Q3, 2026-06-26) this line **closes** the family (the spec's stated preference, §5) rather than routing it forward: decide the intended ordering/replacement rule (§10.4) and encode it in public behavior tests that kill the constant and ordering mutants. Closing it lets the manifest (ticket 001) record the survivors as `closed` and unblocks a green canonical-perimeter claim.

## Assumption Reassessment (2026-06-26)

1. `crates/tracewake-core/src/projections.rs:256` defines `fn food_source_fact_supersedes(...)` — a **private** predicate (no `pub`). The seven survivors are recorded narratively in `archive/reports/0052_foundational_conformance_fourth_hardening_acceptance.md` (lines 145–158), not in `.cargo/mutants-baseline-misses.txt` (which is currently **empty**, 0 bytes — verified this session). Closing the family therefore adds public behavior tests and, if any mutant proves genuinely equivalent, a documented skip; it does not assume a pre-existing survivor registry.
2. Spec authority: `specs/0053_…_HARDENING_SPEC.md` §4.5 (resolve or force; public tests, not private-function-only), §5 (the seven survivors + the close-vs-route choice — **close chosen**), §10.4 (food-source semantics — the intended ordering/replacement rule for source-bearing vs. source-less facts and source-key tie-breaking; an **implementer-recorded** decision encoded in public tests). §9 step 6.
3. Cross-artifact boundary under audit: the projection surface (`projections.rs`) and the actor-known/projection paths that consume `food_source_fact_supersedes`; the standing mutation perimeter (`.cargo/mutants.toml`) that surfaced the survivors. The kill must run through public observation/record paths, not private-function-only tests.
4. Motivating invariants: INV-092 (deterministic replay is tested — the mutation campaign is the regression that proves sensitivity), with INV-028 (staleness is not automatically corrected) and INV-002/INV-025 (belief before truth; wrong beliefs are first-class) framing the supersession semantics — food-source belief replacement is a belief-state rule, decided deterministically.
5. This ticket touches the projection / actor-known surface: the §10.4 rule decides which food-source fact supersedes which, on actor-known/projection paths. It introduces no hidden-truth leakage (the rule operates on actor-known belief/record inputs, not validator-only truth) and no replay nondeterminism (the supersession order is a deterministic function of source presence + source-key tie-break; projection output retains the same shape — **no schema shape change**, item 6 N/A — so replay/hash semantics are unchanged). The decision is recorded in What to Change per §10.4 (implementer-recorded choice).

## Architecture Check

1. Closing the family within this line (Q3) is cleaner than routing it forward: a route-forward keeps the canonical perimeter non-green and defers the §10.4 semantics decision indefinitely, re-creating the "honest non-green with no forcing function" state the fifth pass flagged. Encoding the supersession rule in public behavior tests (not private-function tests) kills the mutants through the same paths real consumers use, so the tests stay meaningful after refactors.
2. No backwards-compatibility aliasing or shims. Any genuinely equivalent mutant receives a narrow `#[mutants::skip]` / registry entry **with a semantic proof** — "the suite did not kill it" is not equivalence evidence (§4.5).

## Verification Layers

1. INV-092 (deterministic replay / mutation sensitivity) -> mutation check: `cargo mutants -f crates/tracewake-core/src/projections.rs` kills the seven enumerated survivors (or each surviving one carries a documented semantic-equivalence skip).
2. Supersession correctness -> public projection/actor-known behavior test: two food-source facts with controlled source presence and source keys, introduced through modeled observation/record paths, yield the decided actor-known belief and rendered/decision consequences.
3. Determinism -> manual review: the §10.4 rule is a deterministic function of source presence + source-key tie-break; projection output shape unchanged (no replay/hash impact).
4. Single-surface ticket: the kill is behavioral on one private predicate; the mapping above covers the mutation, correctness, and determinism layers distinctly.

## What to Change

### 1. Decide + encode the food-source supersession rule (`projections.rs`)

Decide (and record here, per §10.4) the intended ordering/replacement rule for source-bearing vs. source-less food-source facts and source-key tie-breaking, and adjust `food_source_fact_supersedes` so the rule is unambiguous and every enumerated mutant changes observable behavior. Record the chosen rule in this ticket's implementation notes (implementer-recorded choice).

### 2. Public kill tests (`tests/food_source_projection.rs`, new)

Add public projection/actor-known tests that introduce two food-source facts with controlled source presence and source keys through modeled observation/record paths, then assert the resulting actor-known belief and rendered/decision consequences — killing constant `true`/`false`, the `Some`/`None` arm deletions, and the `<`→`==`/`>`/`<=` ordering mutants.

### 3. Survivor disposition (`.cargo/mutants.toml`, `.cargo/mutants-baseline-misses.txt`) — conditional

If a mutant proves genuinely equivalent, add a narrow `#[mutants::skip]` (or registry entry) with a written semantic proof; otherwise leave the (empty) baseline-misses registry empty — the family is killed, not routed.

## Files to Touch

- `crates/tracewake-core/src/projections.rs` (modify)
- `crates/tracewake-core/tests/food_source_projection.rs` (new)
- `.cargo/mutants.toml` (modify — only if a genuinely-equivalent survivor needs a documented skip)
- `.cargo/mutants-baseline-misses.txt` (modify — only if a survivor is registered as a justified, semantically-proven non-blocking residual)

## Out of Scope

- Routing the family forward under a forcing function (Q3 selected close-within-0053).
- The bootstrap/interval/debug seals (004/005/006), the full standing mutation run (009 — this ticket's boundary is `-f projections.rs`), and doc-truthing (008).
- Reclassifying the family into a lower tier or asserting equivalence without a per-mutant semantic proof (§1.2).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo mutants -f crates/tracewake-core/src/projections.rs` — the seven enumerated `food_source_fact_supersedes` survivors are caught (constant `true`; constant `false`; `Some`-arm deletion; `None`-arm deletion; `<`→`==`; `<`→`>`; `<`→`<=`), or each surviving one carries a documented semantic-equivalence skip.
2. `cargo test -p tracewake-core --test food_source_projection` — the public supersession behavior tests pass and exercise source-bearing vs. source-less + tie-break cases.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` — clean.

### Invariants

1. Every one of the seven enumerated survivors is killed by a public behavior test, or carries a written per-mutant semantic-equivalence proof (INV-092; no "suite did not kill it" equivalence).
2. The supersession rule is deterministic and changes no projection serialization/hash shape (replay-safe).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/food_source_projection.rs` (new) — public observation/record-driven food-source replacement tests killing the seven survivors.
2. `crates/tracewake-core/src/projections.rs` (modify) — the supersession rule itself; any `#[mutants::skip]` carries a semantic proof.

### Commands

1. `cargo test -p tracewake-core --test food_source_projection`
2. `cargo mutants -f crates/tracewake-core/src/projections.rs` — the per-ticket mutation verification boundary; this is expensive and is scoped to the one file rather than the full standing campaign (009 runs the full perimeter), so the narrow `-f` form is the correct boundary here.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
