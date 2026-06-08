# 0008PHA3AANTCON-009: Content-validation anti-script hardening

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-content`: extend `validate_no_script` to reject authored outcome chains / provenance-less preloaded planner facts; adversarial content fixtures
**Deps**: 0008PHA3AANTCON-001

## Problem

Spec 0008 Finding 11 (D-0008-11) §9.7 + §10.8: content validation must make outcome-chain authoring impossible enough — fixtures must not smuggle intended outcomes into planner-visible inputs, routine labels, debug strings, or synthetic action chains, nor preload route/food/work access facts without visible/belief provenance. Validators must reject: routine templates encoding guaranteed outcome chains; fixture fields preloading hidden food/workplace/route as actor-known without provenance; debug labels intended to satisfy acceptance tests; player/protagonist-conditioned ordinary-life content; routine step names implying success without shared pipeline events.

The content crate already has anti-script infrastructure — `validate_no_script` (`validate.rs:155`), `is_forbidden_key` (`:182`), `forbidden_form` errors (`:191`) — so this is an **extension**, not greenfield.

## Assumption Reassessment (2026-06-08)

1. `crates/tracewake-content/src/validate.rs` has `validate_fixture` (`:120`), `validate_fixture_errors` (`:141`) calling `validate_no_script` (`:155`), `validate_raw_lines` (`:164`) with `is_forbidden_key` (`:182`)/`forbidden_form` (`:191`); `schema.rs` defines the fixture schema; `crates/tracewake-content/tests/forbidden_content.rs` is the existing negative-test home.
2. Spec §9.7 + §10.8 enumerate the five rejection classes. §13 acceptance: "Content validation rejects authored outcome chains." The sealed-context provenance vocabulary (-001) defines what "without provenance" means for preloaded actor-known facts.
3. Cross-artifact boundary under audit: the **fixture schema ↔ content validator ↔ actor-known provenance contract** — a fixture's initial belief/workplace/sleep/food/route facts intended for autonomous planning must carry actor-known provenance or be rejected/flagged.
4. INV-060 (No authored outcome chains) and INV-097 (No-script compliance is tested): forbidden are quest beats, guaranteed targets/rewards, player-conditioned injection, storyteller causation; the validator is the fail-closed enforcement of this for fixtures.
5. Enforcement surface: fail-closed content validation. Confirm the new rules are deterministic and blocking, reject unknown/behavior-looking fields by default, distinguish blockers from warnings, and name what failing means — consistent with the existing `validate_no_script` contract. Provenance-less preloaded planner facts are a blocker, not a warning.
6. Extends the content validation/schema contract: new forbidden forms + a provenance requirement on planner-intended initial facts (additive validation rules; schema may gain an optional provenance field on initial-belief/access facts). Consumers: `validate_fixture` callers, all existing fixtures (must still pass — they are possibility-space, not scripts), `forbidden_content.rs` (gains the new negative cases). Additive to passing fixtures; breaking only to genuinely script-like content (intended).

## Architecture Check

1. Extending the existing `validate_no_script` keeps one fail-closed validation path rather than a parallel checker — the doctrine-correct place for no-scripting enforcement. Requiring provenance on planner-intended facts ties content validation to the same provenance vocabulary the runtime planner enforces (-001), closing the "authored as actor-known without a channel" gap.
2. No backwards-compatibility aliasing: new rules extend the existing validator; no second validation entry point.

## Verification Layers

1. INV-060 no-outcome-chains → schema validation: a fixture encoding a guaranteed outcome chain (routine template with success-implying step names / synthetic action chain) is rejected by `validate_fixture`.
2. INV-097 no-script-tested → unit test (`forbidden_content.rs`): each of the five §10.8 rejection classes has a failing-fixture negative test.
3. Provenance requirement → unit test: a fixture preloading food/workplace/route as actor-known without provenance is rejected; the same fact with explicit provenance passes.
4. Non-regression → schema validation: every existing fixture still validates (possibility-space content is not falsely rejected).

## What to Change

### 1. Extend anti-script validation

Extend `validate_no_script`/`is_forbidden_key` (`validate.rs`) to reject the five §10.8 classes: guaranteed-outcome routine templates, provenance-less preloaded actor-known facts, acceptance-gaming debug labels, player/protagonist-conditioned content, success-implying routine step names.

### 2. Provenance requirement on planner-intended initial facts

Where a fixture supplies initial belief/workplace/sleep/food/route facts for autonomous planning, require actor-known provenance (schema field + validation), aligned with -001's provenance variants.

### 3. Negative + positive fixtures

Add adversarial failing fixtures (one per rejection class) and confirm existing fixtures still pass.

## Files to Touch

- `crates/tracewake-content/src/validate.rs` (modify — extend `validate_no_script`/`is_forbidden_key`/forbidden forms)
- `crates/tracewake-content/src/schema.rs` (modify — optional provenance field on planner-intended initial facts)
- `crates/tracewake-content/tests/forbidden_content.rs` (modify — five negative cases)

## Out of Scope

- Runtime planner provenance enforcement (0008PHA3AANTCON-001).
- Adversarial runtime hidden-truth gates (0008PHA3AANTCON-008).
- The capstone gate (0008PHA3AANTCON-011).

## Acceptance Criteria

### Tests That Must Pass

1. Each of the five §10.8 rejection classes is rejected by `validate_fixture` (negative tests in `forbidden_content.rs`).
2. A provenance-less preloaded actor-known fact is rejected; the same fact with provenance passes.
3. All existing fixtures still validate; `cargo test -p tracewake-content` green.

### Invariants

1. No-scripting enforcement stays in one fail-closed validation path.
2. Planner-intended initial facts without provenance are blockers.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/tests/forbidden_content.rs` — five §10.8 negative cases + provenance pass/fail.
2. New adversarial fixtures under `crates/tracewake-content/src/fixtures/` (names as surfaced) for the negative cases.

### Commands

1. `cargo test -p tracewake-content forbidden_content`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
