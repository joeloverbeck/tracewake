# 0004PHA2AEPISUB-012: Content schema, serialization, and validation for epistemic seeds

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — extends `tracewake-content` (`schema.rs`, `serialization.rs`, `validate.rs`) with an epistemic seed section and its validation.
**Deps**: 0004PHA2AEPISUB-002

## Problem

Initial expectations must be content data with structured provenance, not hard-coded test state (Spec 0004 §11; `INV-063` authored prehistory must be marked). The content layer must serialize epistemic seeds deterministically and the validator must reject belief/source/proposition failures and forbidden shortcut-truth fields (`culprit`, `stolen_flag`, `npc_knows_truth`, …) while continuing to reject player/quest/script shortcuts. This is fail-closed validation (`INV-097` no-script compliance is tested; foundation 09 no-scripting).

## Assumption Reassessment (2026-06-06)

1. `crates/tracewake-content/src/schema.rs` `FixtureSchema` (`schema.rs:10`) serializes `actors/places/doors/containers/items/affordances` with no belief/expectation/proposition section. `crates/tracewake-content/src/validate.rs` runs `validate_no_player`, `validate_no_script`, `validate_determinism`, `validate_fixture_contract`, `validate_serialization_roundtrip`, plus id/reference/topology/state/registry-parity/semantic checks; its raw-line parser (`validate_raw_lines`, `validate.rs:156`) accepts only physical Phase 1 tags. `crates/tracewake-content/src/serialization.rs` carries the canonical content serialization discipline.
2. The seed section name (`initial_beliefs`/`expectations`), field set, forbidden-field list, and validation failures are fixed by Spec 0004 §11.1/§11.5/§11.6. The `Proposition`/`Belief` types come from tickets 001/002 (content depends on core).
3. Shared boundary under audit: `FixtureSchema` is the contract consumed by the fixture constructors (ticket 013), the loader (`load.rs`), and the content round-trip test; this is an additive new field (`INV-020` event/schema evolution — additive with a default empty section).
4. Invariant motivating this ticket: `INV-021`/`INV-022` (typed propositions, no prose-as-state), `INV-060` (no authored outcome chains), and `INV-097` (no-script compliance is tested — reject objective markers, guaranteed targets, culprit reveals).
5. Fail-closed validation / no-leak surface: the validator is the enforcement surface. It must reject (block, not warn) an important belief missing holder/source, a proposition referencing an unknown id, duplicate belief/observation ids, unstable post-canonicalization ordering, unknown proposition variants, unsupported epistemic schema versions, raw-prose belief content, and any shortcut-truth field; and must keep existing no-player/no-script rejection. It introduces no leakage (seeds carry holder/source, no cross-actor grant) and no nondeterminism (canonical field/collection order, stable-id sort, no hash-map iteration). Unknown fields are rejected by default.

## Architecture Check

1. Extending `FixtureSchema` with a typed `initial_beliefs`/`expectations` section (parsed into `Proposition`/`Belief` values) keeps content as typed possibility, not script — the validator can structurally reject behavior-looking and shortcut-truth fields, which a free-form prose blob could not. Adding the seed validation as new phases alongside the existing `validate_*` functions matches the established validator structure.
2. No backwards-compatibility shims: the seed section is additive with an empty default, so existing Phase 1 fixtures validate unchanged; no alias fields.

## Verification Layers

1. Typed/no-prose seeds (`INV-021`/`INV-022`) -> schema validation: a seed with raw-prose belief content and no typed proposition is rejected.
2. No shortcut truth / no-script (`INV-060`/`INV-097`) -> schema validation test: fixtures containing `culprit`, `true_culprit`, `stolen_flag`, `npc_knows_truth`, `quest_state`, `player_memory`, `knows_mara_did_it`, or equivalents are rejected; existing no-player/no-script rejection still fires.
3. Deterministic round-trip (`INV-018`) -> schema validation + round-trip test: epistemic seed data round-trips canonically; reordering source fields yields identical canonical form; an unsupported epistemic schema version rejects loudly.

## What to Change

### 1. Schema extension

Add an `initial_beliefs`/`expectations` section to `FixtureSchema` (`schema.rs`) with the §11.1 per-seed fields (`belief_id`, `holder_actor_id`, `proposition`, `stance`, `confidence`, `source_kind`, `source_id`, `channel`, `acquired_tick`, optional `last_verified_tick`, privacy/scope, `schema_version`), using the core `Proposition`/`Belief` types. The final section name is fixed consistently across schema, serialization, validation, fixtures, and tests.

### 2. Canonical serialization

Extend `serialization.rs` so epistemic seed data round-trips through the same canonical discipline (deterministic field/collection order, stable-id sort, no hash-map iteration), per §11.6.

### 3. Validation phases

Extend `validate.rs` with epistemic-seed validation (the §11.5 rejection set), extend the raw-line allowlist (`validate_raw_lines`) to admit epistemic tags only after the typed validation exists, and add the forbidden shortcut-truth field rejection. Do not weaken existing no-player/no-script validation.

## Files to Touch

- `crates/tracewake-content/src/schema.rs` (modify — add seed section to `FixtureSchema`)
- `crates/tracewake-content/src/serialization.rs` (modify — canonical seed round-trip)
- `crates/tracewake-content/src/validate.rs` (modify — seed validation + forbidden-field rejection + raw-line allowlist)

## Out of Scope

- The actual fixture data (Tomas expectation, Mara, golden scenarios) — ticket 013.
- Projection application of seeds (ticket 005 consumes them).
- Sound observation seed shape specifics beyond schema (ticket 014).

## Acceptance Criteria

### Tests That Must Pass

1. A fixture with a valid Tomas expectation seed validates and produces canonical deterministic output.
2. Seeds missing holder or source, referencing unknown ids, with duplicate belief ids, or with an unsupported epistemic schema version are rejected (blocked, not warned).
3. A fixture containing any shortcut-truth field (`culprit`, `stolen_flag`, `npc_knows_truth`, …) is rejected; existing Phase 1 fixtures still validate.

### Invariants

1. Epistemic seeds are typed propositions with holder + source; no prose-as-state, no global truth flag.
2. Validation is fail-closed and blocking; unknown and behavior-looking fields are rejected by default.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/tests/forbidden_content.rs` (extend) — shortcut-truth field rejection, belief/source/proposition failures.
2. `crates/tracewake-content/tests/fixtures_load.rs` (extend) — valid epistemic seed loads and round-trips canonically.

### Commands

1. `cargo test -p tracewake-content`
2. `cargo test -p tracewake-content --test forbidden_content`
3. `cargo build --workspace --all-targets --locked`
