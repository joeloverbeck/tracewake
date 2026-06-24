# 0050FOUCONSEC-016: Mutation witnesses for actor-known-record novelty predicate

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — test-only (adds witnesses in `crates/tracewake-core/src/epistemics/projection.rs` test module).
**Deps**: 0050FOUCONSEC-011 (recorded the standing survivors and routed real survivors to separate remediation tickets)

## Problem

The CI gate `mutants-in-diff` ("mutation in-diff (lock layer)", `.github/workflows/ci.yml`) fails on PR #62 / branch `implemented-0050`. Three of the nine normalized survivor classes (≈26 underlying mutants, the largest cluster) are in `actor_known_record_is_novel_to_context` (`crates/tracewake-core/src/epistemics/projection.rs:904`):

- `replace && with || in actor_known_record_is_novel_to_context`
- `replace == with != in actor_known_record_is_novel_to_context`
- `delete ! in actor_known_record_is_novel_to_context`

The function is a `match` over `ActorKnownProjectionRecord` variants; each arm returns `!context.actor_known_X().iter().any(|fact| fact.f1() == a && fact.f2() == b && …)` — i.e. a record is **novel** iff no existing actor-known fact matches it on **all** discriminating fields. The variants flagged on this PR's diff (e.g. `CurrentPlace`, `LocalDoor`, `LocalContainer`, `Workplace`, `SleepPlace`, `FoodSource`, `Route`, `CarriedItem`, `LocalItem`, `LocalActor`) lack per-field witnesses, so flipping a conjunct (`&& → ||`), an equality (`== → !=`), or the outer negation (`delete !`) survives. This is missing actor-known novelty coverage — the truth firewall requires it tested.

## Assumption Reassessment (2026-06-24)

1. `actor_known_record_is_novel_to_context(record, context) -> bool` (`crates/tracewake-core/src/epistemics/projection.rs:904`) matches each `ActorKnownProjectionRecord` variant to `!context.actor_known_*().iter().any(|fact| <field eqs joined by &&>)`. Each mutant is killable with a per-variant unit test: build a `KnowledgeContext` whose existing facts match the candidate record on **all but one** field → the record is novel; flipping a `==` to `!=` or an `&&` to `||` changes that verdict, and `delete !` inverts a known-existing record's verdict. `KnowledgeContext` and the `ActorKnown*Fact` types are constructible directly in the test module (`#[cfg(test)] mod tests` at `:1881`).
2. `.cargo/mutants-baseline-misses.txt` is empty (verified); the gate accepts zero misses and `docs/2-execution/10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` requires a typed behavior witness (general anti-vacuity rule). The fix is witnesses, not baseline padding — especially because the gate normalizes survivors by stripping `:line:col:`, so a baseline entry would whitelist *all* `&&→||` (or `==→!=`) mutants across every variant of this function.
3. Shared boundary under audit: the actor-known novelty predicate in `epistemics/projection.rs` (listed in the `.cargo/mutants.toml` perimeter). Test code only; no perimeter or production change.
4. `INV-101` (actor-known context is sealed) and `INV-093` (actor-knowledge leakage is a high-severity defect) are the governing invariants: `actor_known_record_is_novel_to_context` decides whether a projected record is new to the actor's sealed context — a wrong verdict either re-emits known facts or suppresses genuinely new ones, corrupting what the actor is modeled to know. `INV-102` (cognition inputs require provenance) is adjacent (novelty gates what enters the context). `INV-098`/`INV-092` require regression evidence measured.
5. Enforcement surface: the novelty predicate consumed when projecting actor-known records into the sealed context. Witnesses assert the per-field novelty verdict; they add no production code and no leak path, so they cannot weaken epistemic-leakage prevention or replay — they prove the existing filter is correct.

## Architecture Check

1. Per-variant witnesses are cleaner and far stronger than baseline-padding: they pin the exact field-by-field novelty contract per record variant, which is the actual truth-firewall obligation, and keep the empty baseline intact so the gate stays sharp on future diffs.
2. No backwards-compatibility aliasing/shims introduced — test-only addition.

## Verification Layers

1. `INV-101` / `INV-093` (sealed actor-known novelty) -> manual review + codebase grep-proof: per-variant tests assert "matches on all fields ⇒ not novel" and "differs on exactly one field ⇒ novel", covering the variants present on the diff.
2. `INV-098` / `INV-092` (regression evidence measured) -> mutation campaign: `cargo mutants` reports the `&& → ||`, `== → !=`, and `delete !` mutants in `actor_known_record_is_novel_to_context` as CAUGHT.
3. Single-surface note: one function, many variants; each variant maps to the same witness pattern, so the per-variant cases are the proof surface — no additional layer mapping applies.

## What to Change

### 1. Per-variant novelty witnesses

In the `epistemics/projection.rs` test module, add tests for each `ActorKnownProjectionRecord` variant flagged by the diff. For each variant:
- a "not novel" case: the context already holds a fact matching the candidate record on **every** discriminating field → assert `actor_known_record_is_novel_to_context` returns `false` (kills `delete !`);
- a "novel" case per discriminating field: the context holds a fact matching on all fields **except one** → assert the function returns `true` (kills the `&& → ||` for that conjunct and the `== → !=` for that equality).

Prioritize the variants whose lines changed in this PR's diff so every in-diff survivor is covered; reuse a small helper to build single-fact contexts to keep the cases compact.

## Files to Touch

- `crates/tracewake-core/src/epistemics/projection.rs` (modify — test module only)

## Out of Scope

- Any `.cargo/mutants.toml` perimeter change (spec-0050 §6.3: none).
- Any `.cargo/mutants-baseline-misses.txt` entry — survivors are killable; padding is rejected (`docs/2-execution/10`).
- The `projections.rs` dedup (`-014`) and perception (`-015`) survivors — separate tickets/diffs.

## Acceptance Criteria

### Tests That Must Pass

1. Per-variant novelty witnesses assert the all-fields-match ⇒ not-novel and single-field-differs ⇒ novel contract for each variant flagged on the diff.
2. `cargo mutants --in-diff` over the changed lines reports the `&& → ||`, `== → !=`, and `delete !` survivors in `actor_known_record_is_novel_to_context` as CAUGHT (no new survivor).
3. The four gates pass (`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace --all-targets --locked`, `cargo test --workspace`).

### Invariants

1. The actor-known novelty verdict is correct per record variant and regression-witnessed (`INV-101`, `INV-093`, `INV-098`).
2. No production behavior changes; deterministic replay is unaffected (`INV-092`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/epistemics/projection.rs` (`mod tests`) — per-variant novelty witnesses (not-novel + single-field-differs cases) for the variants present on the diff.

### Commands

1. `cargo test -p tracewake-core epistemics::projection`
2. `cargo mutants --in-diff <(git diff origin/main...HEAD -- crates/tracewake-core/src/epistemics/projection.rs) --no-shuffle --jobs 2 --timeout 183` — reproduces the in-diff gate's input selection for the changed `projection.rs` lines.
3. The per-file mutation command is the correct boundary because the CI gate scopes mutation to changed guarded-layer lines; the full standing perimeter is the scheduled `mutants-lock-layer` matrix, not this PR gate.

## Outcome

Completed: 2026-06-24

Implemented test-only field-sensitivity witnesses in `crates/tracewake-core/src/epistemics/projection.rs` for `actor_known_record_is_novel_to_context`. The new test covers all context-backed `ActorKnownProjectionRecord` variants in the predicate: `CurrentPlace`, `CarriedItem`, `Route`, `FoodSource`, `SleepPlace`, `Workplace`, `LocalDoor`, `LocalContainer`, `LocalItem`, and `LocalActor`. For each variant, it asserts the all-discriminating-fields-match case is not novel and every discriminating single-field difference is novel.

Verification run:

- `cargo test -p tracewake-core epistemics::projection` — pass.
- `cargo mutants --in-diff <(git diff origin/main -- crates/tracewake-core/src/epistemics/projection.rs) --no-shuffle --jobs 2 --timeout 183` — pass; 57 mutants tested, 56 caught, 1 unviable, 0 missed.

No production behavior, mutation perimeter, or baseline files changed. As with the sibling survivor tickets, the mutation selector used `origin/main` so the in-diff input matched the current edited source tree rather than the pre-ticket committed diff.
