# 0003PHA1SPIANT-010: Content schema field-registration, no-prose-born-fact fixture, and typed diagnostics

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-content` (`validate.rs` field-registration discipline, `serialization.rs` canonical rule coverage, new forbidden-content fixture, `forbidden_content.rs` tests)
**Deps**: None

## Problem

Content validation is strong today — typed phases/reports (`validate.rs:21-58`, `:141-160`), raw forbidden-key/unknown-field rejection (`:164-228`, `:1670-1727`), no-player/no-script/outcome-chain/determinism validations (`:263-432`, `:1202-1324`, `:1532-1607`), and forbidden-content tests (`forbidden_content.rs:23-198`). But a future schema field can bypass no-script/provenance/canonicalization review without a field-registration gate, and there is no regression fixture proving a prose-born "fact" (notes implying a culprit/outcome/hidden truth) is rejected by typed phase. Spec `0003` §5.7 / SPINE-AC-010 require schema-field-registration discipline plus an adversarial fixture per forbidden-content family.

## Assumption Reassessment (2026-06-08)

1. `crates/tracewake-content/src/validate.rs` carries typed phases/reports (`:21-58`, `:141-160`), raw-field rejection (`:164-228`), reserved player/story/quest/culprit/director rejection (`:263-432`), script-marker / authored-outcome-chain / shortcut-truth / determinism / serialization-drift checks (`:1202-1324`, `:1426-1607`, `:1670-1727`). `crates/tracewake-content/src/serialization.rs` holds canonical serialization. `crates/tracewake-content/tests/forbidden_content.rs:23-198` covers quest/reward/player/culprit/scripted-outcome cases.
2. Spec `specs/0003_PHASE_1_SPINE_ANTI_CONTAMINATION_AND_STRUCTURAL_HARDENING_SPEC.md` §7 SPINE-AC-010 mandates: every new authored schema field registered with a typed domain field/parser, a validation phase, a canonical serialization rule, a provenance/no-script decision, and a diagnostic reason code / validation error kind; raw prose fields describe labels/flavor only after typed validation proves they create no simulation fact; a new forbidden-content fixture per new no-script boundary; validation reports remain typed by phase/error, not display text.
3. Boundary under audit: the content schema (`schema.rs`) → validation phases (`validate.rs`) → canonical serialization (`serialization.rs`) → forbidden-content fixtures contract. This is fixture/content authority (Spec 0001 ontology + `docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md` / `09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md`).
4. INV motivating this ticket: `INV-058`–`INV-063` (no-scripting / content-is-possibility / no authored outcomes / no prose-born facts), `INV-020`/`INV-022` (schema/provenance discipline), `INV-097` (validation integrity). Restated: content is typed possibility; no simulation fact may be born from prose, and a new field cannot enter without a typed validation home.
5. Fail-closed-validation surface touched: `validate.rs` is the content firewall. The field-registration gate must keep validation deterministic, fail-closed, and blocking; unknown fields stay rejected by default; behavior-looking fields stay refused. The new prose-born-fact fixture must be *rejected* by a typed phase (a negative fixture). No determinism weakening; canonical serialization stays stable.
6. Schema extension (content authoring schema): adds a field-registration requirement over the existing content schema. Consumers = `validate.rs` phases, `serialization.rs` canonical rules, the fixture loader (`load.rs`, `fixtures_load.rs`), and golden fixtures. The discipline is **additive** — existing fields keep their validation/serialization; the gate ensures future fields carry all five registration facets. No existing fixture changes meaning.

## Architecture Check

1. A field-registration conformance test ("a schema field without a validation phase + canonical rule + provenance decision + reason code fails the build") closes the drift vector where a new field slips past no-script/provenance review, and a negative prose-born-fact fixture proves the firewall catches the subtle case (flavor text smuggling a simulation fact). This is stronger than relying on reviewers to remember the no-script doctrine per field.
2. No backwards-compatibility shim: unknown/unregistered fields are rejected, not defaulted; raw prose is admitted only after typed validation proves it creates no fact.

## Verification Layers

1. `INV-058`–`INV-063` (no-scripting / no prose-born facts) -> schema validation: a new prose-born-fact fixture is rejected by a typed validation phase.
2. `INV-020`/`INV-022` (schema/provenance) -> conformance test: a schema field lacking validation phase / canonical serialization / provenance decision fails the field-registration test.
3. `INV-097`/`DIAG` (typed validation reports) -> codebase grep-proof: validation reports/tests are typed by phase/error, not display text.

## What to Change

### 1. Schema field-registration discipline

Add a `content_new_field_requires_validator_and_canonical_serialization` conformance test: the set of authored schema fields must each map to a validation phase, a canonical serialization rule, a provenance/no-script decision, and a reason-code/error kind.

### 2. No-prose-born-fact regression fixture

Add an adversarial fixture whose prose/notes imply a culprit/outcome/hidden truth and assert it is rejected by a typed validation phase (`content_prose_born_fact_rejected`).

### 3. Per-family adversarial coverage

Ensure at least one adversarial fixture exists per forbidden-content family; keep validation reports typed by phase/error.

## Files to Touch

- `crates/tracewake-content/src/validate.rs` (modify — field-registration check)
- `crates/tracewake-content/src/serialization.rs` (modify — canonical rule coverage if a field lacks one)
- `crates/tracewake-content/src/fixtures/prose_born_fact_rejected_001.rs` (new — adversarial negative fixture) and `crates/tracewake-content/src/fixtures/mod.rs` (modify — register it)
- `crates/tracewake-content/tests/forbidden_content.rs` (modify — `content_prose_born_fact_rejected`, `content_new_field_requires_validator_and_canonical_serialization`)

## Out of Scope

- Action/pipeline diagnostic typing (0003PHA1SPIANT-011) — this ticket covers content-validation diagnostics.
- Any new authored mechanic or content field — this ticket builds the *gate* and a negative fixture, not new content surface.
- State-mutation seal (0003PHA1SPIANT-001).

## Acceptance Criteria

### Tests That Must Pass

1. `content_prose_born_fact_rejected` — a fixture with prose implying culprit/outcome/hidden truth is rejected by a typed phase.
2. `content_new_field_requires_validator_and_canonical_serialization` — fails if a schema field lacks validation/canonical-serialization/provenance/reason-code.
3. `cargo test --workspace` passes with existing golden fixtures loading deterministically.

### Invariants

1. No simulation fact is born from prose; every authored field has a typed validation home (`INV-058`–`INV-063`, `INV-020`).
2. Validation reports are typed by phase/error, not display text (`INV-097`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/tests/forbidden_content.rs` — `content_prose_born_fact_rejected`, `content_new_field_requires_validator_and_canonical_serialization`.
2. `crates/tracewake-content/src/fixtures/prose_born_fact_rejected_001.rs` — adversarial negative fixture.

### Commands

1. `cargo test -p tracewake-content --test forbidden_content`
2. `cargo test --workspace`
