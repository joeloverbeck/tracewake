# 0004PHA1THIHAR-008: Make content field coverage typed, not source-string-only

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-content` (`schema.rs` typed field registry; `validate.rs` and `serialization.rs` consume it); `forbidden_content.rs` tests
**Deps**: None

## Problem

Content validation correctly rejects no-script constructs, but proves *new schema fields are covered* by source-string inspection (`content_new_field_requires_validator_and_canonical_serialization` at `forbidden_content.rs:323`). A new content field could slip through as a prose-born fact, shortcut-truth field, authored outcome, or nondeterministic serialization gap unless covered (spec §6 F-009, §8 THIRD-AC-009). The coverage guard should move to a typed schema-field registry that drives validation and canonical serialization.

## Assumption Reassessment (2026-06-08)

<!-- Items 1-3 always required; 4-6 selected for an invariant-motivated, fail-closed-validation, schema-extension ticket. -->

1. `crates/tracewake-content/src/validate.rs` rejects reserved IDs / display-name-as-ID via `reject_reserved_or_display(...)` (`:388`–`:436`) and rejects prose implying culprit/outcome/hidden-truth facts (`:341`). `crates/tracewake-content/tests/forbidden_content.rs` has `content_prose_born_fact_rejected` (`:311`) and `content_new_field_requires_validator_and_canonical_serialization` (`:323`). `crates/tracewake-content/src/serialization.rs` centralizes canonical serialization.
2. The remediation is spec §8 `THIRD-AC-009` + the §9.5 content fixtures (`content_quest_objective_reward_ids_rejected`, `content_player_protagonist_culprit_director_ids_rejected`, `content_prose_born_fact_rejected`, `content_shortcut_truth_fields_rejected`, `content_new_field_requires_typed_validation_and_canonical_serialization_metadata`, `content_serialization_is_canonical_independent_of_authoring_order`), reassessed this session.
3. Shared boundary under audit: the content schema ↔ validation ↔ serialization contract. A typed field registry becomes the single source of truth; its consumers are `validate.rs` (validation phases) and `serialization.rs` (canonical keys).
4. Motivating invariants (restated): `INV-022` / `INV-060` / `INV-061` (no-scripting; content is possibility, not script), `INV-097` (no prose-born/shortcut truth), `INV-102` (cognition inputs require provenance), `INV-105` (authoritative diagnostic data).
5. Fail-closed validation surface: content validation is the deterministic, blocking gate that rejects unknown fields by default and refuses behavior-looking fields. The typed registry keeps it fail-closed — a field without `validation_phase` / `forbidden_construct_policy` metadata fails — and introduces no leakage path; the source-string scanner is retained only as smoke.
6. Schema extension (additive-vs-breaking): introducing the typed field registry reshapes the content schema's internal representation. Consumers are `validate.rs` and `serialization.rs`; the change is **breaking** to the internal contract (both must consume the registry) but additive to authored content (no fixture field changes). Adding a field without registry metadata becomes a hard failure.

## Architecture Check

1. A typed schema-field registry — each field carrying `validation_phase`, `canonical_serialization_key`, and `forbidden_construct_policy` — that drives both validation and serialization is stronger than source-string smoke: a new field with no metadata fails to compile/validate rather than slipping past a substring check. The source scanner is kept only as a smoke layer.
2. No backwards-compatibility shims: validation and serialization are repointed at the single registry; no parallel hand-maintained coverage list is left aliased alongside it.

## Verification Layers

1. `INV-060` / `INV-061` / `INV-097` (no-script / no prose-born truth) -> schema validation: `content_quest_objective_reward_ids_rejected`, `content_player_protagonist_culprit_director_ids_rejected`, `content_prose_born_fact_rejected`, `content_shortcut_truth_fields_rejected`.
2. `INV-022` / `INV-102` (typed possibility + provenance) -> test: `content_new_field_requires_typed_validation_and_canonical_serialization_metadata`.
3. Determinism -> test: `content_serialization_is_canonical_independent_of_authoring_order`.

## What to Change

### 1. Typed schema-field registry

Add a typed registry in `schema.rs` enumerating every content schema field with `validation_phase`, `canonical_serialization_key`, and `forbidden_construct_policy` metadata.

### 2. Repoint validation + serialization at the registry

Have `validate.rs` and `serialization.rs` consume (or be generated from) the registry so a field without metadata cannot validate or serialize. Retain the existing source-string field check only as a smoke guard.

## Files to Touch

- `crates/tracewake-content/src/schema.rs` (modify — typed field registry)
- `crates/tracewake-content/src/validate.rs` (modify — consume registry; keep `reject_reserved_or_display` semantics)
- `crates/tracewake-content/src/serialization.rs` (modify — consume registry for canonical keys)
- `crates/tracewake-content/tests/forbidden_content.rs` (modify — typed-coverage + canonical-order fixtures)

## Out of Scope

- Event-schema replay fixtures (ticket 007) and core checksum parity (ticket 006).
- Any new authored content fields — this ticket changes the *coverage mechanism*, not the content surface.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-content` — all §9.5 fixtures pass, including the typed-metadata-required and canonical-order fixtures.
2. `cargo test --workspace` — full pipeline green.
3. `cargo build --workspace --all-targets --locked` — registry-driven validation/serialization compiles.

### Invariants

1. Every content schema field appears in the typed registry with validation + canonical-serialization metadata.
2. Canonical serialization is independent of authoring order; behavior-looking / reserved-ID / prose-born fields stay rejected.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/tests/forbidden_content.rs` — `content_new_field_requires_typed_validation_and_canonical_serialization_metadata`, `content_shortcut_truth_fields_rejected`, `content_serialization_is_canonical_independent_of_authoring_order` (and retained reserved-ID / prose-born fixtures); rationale: typed coverage replaces source-string smoke as the primary guard.

### Commands

1. `cargo test -p tracewake-content`
2. `cargo build --workspace --all-targets --locked && cargo test --workspace`
