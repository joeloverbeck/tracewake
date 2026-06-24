# 0050FOUCONSEC-014: Mutation witnesses for actor-known door/container source-key dedup

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: None — test-only (adds witnesses in `crates/tracewake-core/src/projections.rs` test module).
**Deps**: 0050FOUCONSEC-011 (recorded the standing survivors and routed real survivors to separate remediation tickets)

## Problem

The CI gate `mutants-in-diff` ("mutation in-diff (lock layer)", `.github/workflows/ci.yml` job `mutants-in-diff`) fails on PR #62 / branch `implemented-0050`. It runs `cargo mutants --in-diff` over guarded-layer lines changed in the diff and fails when a surviving mutant is not in the accepted baseline `.cargo/mutants-baseline-misses.txt` (which is empty by design — zero accepted misses on the lock layer).

Two of the nine new survivor classes are in `projections.rs`:

- `crates/tracewake-core/src/projections.rs: replace > with >= in actor_known_doors_for_context` (L319)
- `crates/tracewake-core/src/projections.rs: replace > with >= in actor_known_containers_for_context` (L353)

Both functions dedup actor-known facts by `door_id` / `container_id`, keeping the fact with the greater `source_key()` (`fact.source_key() > existing.source_key()`). No test distinguishes `>` from `>=`, so the "keep-latest-source" tiebreak is unwitnessed: a mutant that flips the tiebreak on equal source keys survives. This is a missing behavior witness on the actor-known projection surface, not intended-deferred behavior.

## Assumption Reassessment (2026-06-24)

1. `actor_known_doors_for_context` (`crates/tracewake-core/src/projections.rs:311`) and `actor_known_containers_for_context` (`:343`) both select the fact with the greater `source_key()` per id via `if fact.source_key() > existing.source_key() { *existing = fact; }`. `source_key()` returns `&str` (`:821`). Two facts sharing an id but differing in `source_key` (and/or surface payload) are constructible directly in a unit test via `KnowledgeContext`, so the `> → >=` tiebreak is observable: with `>`, on equal `source_key` the first-seen fact wins; with `>=`, the later-seen fact wins.
2. `.cargo/mutants-baseline-misses.txt` is empty (verified, 0 bytes); the gate accepts zero misses (`docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` L205–208, zero-floor ratchet). `docs/2-execution/10` requires a typed behavior witness, not baseline padding (general anti-vacuity / behavior-witness rule). So the fix is a witness test, not a baseline entry.
3. Shared boundary under audit: the actor-known projection dedup surface in `projections.rs` and the standing mutation perimeter `.cargo/mutants.toml` (which lists `projections.rs`). This ticket changes test code only; it touches no `.cargo/mutants.toml` perimeter and no production behavior.
4. `INV-101` (actor-known context is sealed) and `INV-093` (actor-knowledge leakage is a high-severity defect) motivate this ticket: the dedup tiebreak decides which source's surface the actor is shown; an unwitnessed tiebreak is exactly the kind of actor-known projection logic the truth firewall requires tested. `INV-098` (feature acceptance is harsh) and `INV-092` (deterministic replay is tested) require the regression evidence to be measured, not declared.
5. Enforcement surface: `actor_known_doors_for_context` / `actor_known_containers_for_context` produce the actor-known door/container surfaces consumed downstream. The witness asserts the deterministic tiebreak on equal `source_key`; it adds no production code and no replay/leak path, so it cannot weaken epistemic-leakage prevention or deterministic replay — it strengthens the evidence that the existing filter is correct.

## Architecture Check

1. Adding a targeted witness in the existing `projections.rs` test module (`#[cfg(test)] mod tests` at `:1607`) is cleaner than baseline-padding: the gate normalizes survivors by stripping `:line:col:`, so a baseline entry `replace > with >= in actor_known_doors_for_context` would whitelist *every* `>`→`>=` mutant in that function on future diffs, blinding the gate. A witness keeps the baseline empty and the gate sharp.
2. No backwards-compatibility aliasing/shims introduced — test-only addition.

## Verification Layers

1. `INV-101` / `INV-093` (actor-known projection correctness) -> manual review + codebase grep-proof: the new tests construct a `KnowledgeContext` with two same-id facts of differing `source_key` and assert the greater-`source_key` surface wins and the equal-`source_key` tiebreak keeps first-seen.
2. `INV-098` / `INV-092` (regression evidence measured) -> replay/golden-fixture check is N/A (pure projection unit); the mutation campaign is the proof surface — `cargo mutants` over the two functions reports the `> → >=` mutants as CAUGHT.
3. Single-surface note: one surface (actor-known dedup projection) with two functions; both map to the same witness style, so no additional layer mapping applies.

## What to Change

### 1. Witness for `actor_known_doors_for_context` tiebreak

In `crates/tracewake-core/src/projections.rs` test module, add a test that builds a `KnowledgeContext` containing two `ActorKnownDoorFact`s with the same `door_id` but different `source_key` and different surface (e.g. `is_open`). Assert the resulting `ActorKnownDoorSurface` reflects the **greater-`source_key`** fact. Add a second case with **equal** `source_key` and assert the **first-seen** fact wins (the `>` behavior), which is what distinguishes `>` from `>=` and kills the mutant.

### 2. Witness for `actor_known_containers_for_context` tiebreak

Mirror the above for `ActorKnownContainerFact` / `ActorKnownContainerSurface` (same `container_id`, differing `source_key` and `is_open`/`is_locked`).

## Files to Touch

- `crates/tracewake-core/src/projections.rs` (modify — test module only)

## Out of Scope

- Any `.cargo/mutants.toml` perimeter change (spec-0050 §6.3: none).
- Any `.cargo/mutants-baseline-misses.txt` entry — these survivors are killable; baseline padding is rejected (`docs/2-execution/10`).
- The perception (`-015`) and epistemics novelty (`-016`) survivors — separate tickets/diffs.

## Acceptance Criteria

### Tests That Must Pass

1. New tests assert the greater-`source_key` surface wins and the equal-`source_key` tiebreak keeps first-seen, for both doors and containers.
2. `cargo mutants --in-diff` over the changed lines reports the two `replace > with >=` mutants in `actor_known_doors_for_context` / `actor_known_containers_for_context` as CAUGHT (no new survivor).
3. `cargo test --workspace` and the four gates (`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace --all-targets --locked`, `cargo test --workspace`) pass.

### Invariants

1. The actor-known door/container dedup keeps the greater-`source_key` fact deterministically; the tiebreak is regression-witnessed (`INV-101`, `INV-093`, `INV-098`).
2. No production behavior changes; deterministic replay is unaffected (`INV-092`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/projections.rs` (`mod tests`) — two witnesses (`actor_known_doors_for_context`, `actor_known_containers_for_context`) covering greater-source-key selection and the equal-source-key first-seen tiebreak.

### Commands

1. `cargo test -p tracewake-core projections`
2. `cargo mutants --in-diff <(git diff origin/main...HEAD -- crates/tracewake-core/src/projections.rs) --no-shuffle --jobs 2 --timeout 183` — reproduces the in-diff gate's input selection for the changed `projections.rs` lines.
3. The narrower per-file mutation command is the correct boundary because the CI gate scopes mutation to changed guarded-layer lines; the full standing perimeter is `mutants-lock-layer` (scheduled), not this PR gate.

## Outcome

Completed: 2026-06-24

Implemented test-only witnesses in `crates/tracewake-core/src/projections.rs` for the actor-known door and container dedup surfaces. Each witness covers the greater-`source_key` selection and the equal-`source_key` tie-break that distinguishes the production `>` comparison from a `>=` mutant. During implementation, the equal-key assertion was corrected to match the real sealed-context contract: `KnowledgeContext::seal` sorts facts before projection, so the tie-break keeps the first fact in sealed canonical order, not caller insertion order.

Verification run:

- `cargo test -p tracewake-core projections` — pass.
- `cargo mutants --in-diff <(git diff origin/main...HEAD -- crates/tracewake-core/src/projections.rs) --no-shuffle --jobs 2 --timeout 183` — pass; 16 mutants tested, 12 caught, 4 unviable, 0 missed.

No production behavior, mutation perimeter, or baseline files changed. The full workspace gates remain for series closeout after the remaining `0050FOUCONSEC-015` and `0050FOUCONSEC-016` tickets.
