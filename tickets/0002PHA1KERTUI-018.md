# 0002PHA1KERTUI-018: Content validation — phases, required failures, forbidden forms

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — adds the `validate` module to `tracewake-content`.
**Deps**: 0002PHA1KERTUI-017

## Problem

Invalid content must fail before play (Spec 0002 §13). This ticket adds the deterministic content validator: the 14 validation phases (§13.1 — parse/schema, canonicalization, ID, referential, location, physical topology, state, action-registry parity, semantic view, no-player, no-script, determinism-hazard, fixture-contract, serialization), the required validation failures it must catch and test (§13.2), and the forbidden content forms it rejects (§13.3 — quest/objective/reward/player/protagonist/culprit/story-beat/director keys; `on_enter`/`on_open`/`on_tick`/`force_event`/etc. triggers; content-driven state mutation; TUI-specific availability rules; LLM prompts as authoritative content). This is the no-scripting gate that keeps content possibility, not script.

## Assumption Reassessment (2026-06-06)

1. The content schema/load exist from ticket 017; this adds `crates/tracewake-content/src/validate.rs`, depending on the core runtime invariant checks (`tracewake-core::validation`) and the action registry (008) for action-registry-parity validation.
2. The validation phases are `specs/0002_…_SPEC.md` §13.1; required failures are §13.2; forbidden forms are §13.3; the validator must reject unknown/behavior-looking fields. Authorities: `docs/1-architecture/13_VALIDATION_OBSERVABILITY_METRICS_AND_ACCEPTANCE_ARCHITECTURE.md`, `docs/2-execution/09_DATA_AUTHORING_VALIDATION_AND_GOLDEN_FIXTURES.md`, foundation 09 (no-scripting).
3. Shared boundary under audit: the validator gate that every fixture (019) and the TUI loader (020) pass through before producing an accepted `InitialWorld`. It consumes the action registry (008) to check affordance/target-kind parity.
4. Invariant motivating this ticket: INV-060 (no authored outcome chains) and INV-061 (authored causal machinery creates possibility space, not guaranteed arcs); INV-058 (quest is not a primary data type). The validator is the enforcement surface for the no-scripting doctrine.
5. Fail-closed validation surface: the validator must be deterministic, blocking, and fail-closed — reject unknown fields by default, refuse behavior-looking fields (selectors/branches/triggers/authored outcome chains), distinguish blockers from any warnings, sort validation errors deterministically (§10.2), and name what failing means (block). It is the direct enforcement of the no-scripting boundary; this ticket does not weaken it. Determinism-hazard validation (§13.1 phase 12) blocks outcome-affecting unordered collections.

## Architecture Check

1. A single deterministic validator running ordered phases that fail closed (rejection over silent acceptance, §20.1) centralizes the no-scripting/no-player/determinism gates in one place every accepted fixture passes — there is no path that accepts unvalidated content. Allowing inert display prose containing words like "reward" only as strings (§13.3) while strictly rejecting mechanical keys keeps authoring expressive without opening a scripting hole.
2. No backwards-compatibility shims: greenfield; no permissive legacy mode.

## Verification Layers

1. No-scripting (INV-060/061) -> schema validation: content containing `on_open`/`force_event`/`complete_objective`/authored outcome chains is rejected with a deterministic error.
2. No-player (INV-058; §13.1 phase 10) -> schema validation: player-only verbs/entities/tags and `PlayerCharacter`/`Quest`/`Reward` constructs are rejected.
3. Fail-closed + determinism (arch 13; INV-018) -> unit test: unknown fields are rejected by default; validation errors are deterministically sorted; an outcome-affecting unordered collection without canonical ordering is flagged.
4. Action-registry parity (§13.1 phase 8) -> codebase grep-proof + unit test: every fixture-declared affordance references a known action definition and a supported target kind.

## What to Change

### 1. Validator

Add `crates/tracewake-content/src/validate.rs` running the 14 §13.1 phases in order, each fail-closed, producing a deterministically-ordered validation report and an accepted `InitialWorld` only when all phases pass.

### 2. Forbidden-form rejection

Implement the §13.3 forbidden-key/trigger rejection and the §13.2 required-failure catalog (missing/duplicate ID, bad reference, wrong target kind, item double-location, container/item mismatch, door endpoint missing, player-only verb, quest/objective/reward, authored outcome chain, fixture missing golden assertions).

### 3. Registration

Declare `validate` in `crates/tracewake-content/src/lib.rs`.

## Files to Touch

- `crates/tracewake-content/src/validate.rs` (new)
- `crates/tracewake-content/src/lib.rs` (modify — declare `validate`; file created by ticket 017)

## Out of Scope

- The seven golden fixtures the validator accepts (ticket 019) and their negative-validation fixtures (ticket 022 / 019 as test data).
- Runtime action validation (ticket 008 — distinct from content validation).

## Acceptance Criteria

### Tests That Must Pass

1. Content with a missing ID, duplicate ID, bad reference, wrong target kind, or item double-location is rejected with a deterministic, source-naming error.
2. Content containing quest/objective/reward/player/protagonist constructs or `on_*`/`force_event`/`complete_objective` triggers is rejected.
3. An outcome-affecting unordered collection without canonical ordering is flagged; validation errors are emitted in deterministic order.
4. Every fixture-declared affordance resolves to a known action definition and supported target kind.

### Invariants

1. Validation is deterministic, blocking, and fail-closed; unknown and behavior-looking fields are rejected by default.
2. No content path produces an accepted `InitialWorld` without passing all phases.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/src/validate.rs` (unit tests) — per-phase rejection cases and the §13.2 required-failure catalog.
2. `crates/tracewake-content/tests/forbidden_content.rs` — no-script/no-player rejection over representative bad fixtures.

### Commands

1. `cargo test -p tracewake-content validate forbidden_content`
2. `cargo build --workspace`
3. Content-crate scope is correct: validation is a pure gate over content+registry; full golden-fixture acceptance runs in ticket 022.
