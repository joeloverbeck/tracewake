# 0017PHA3ATICLED-007: Content numeric field-policy registry and union marker scan

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-content` (`validate`); no core code changes (reads `NEED_MAX` only)
**Deps**: `specs/0017_PHASE_3A_TICK_LEDGER_EPISTEMIC_STALENESS_REPLAY_PAYLOAD_EVIDENCE_AND_GENERATIVE_LOCK_HARDENING_SPEC.md` (ORD-HARD-030); no ticket dependencies

## Problem

Content validation is partial in two dimensions. **Numeric**: `validate.rs::validate_nonnegative_tuning` rejects `< 0` and `> 10_000` — but `10_000` is a magic cap ten times `NEED_MAX` (1000), zero is permitted for relief-direction fields (a food source with `hunger_reduction_per_serving: 0` is a silently-dead resource), and `WorkplaceSchema.work_duration_ticks` / `max_fatigue_to_start` / `max_hunger_to_start` are entirely unvalidated (zero-length work blocks; vacuous or dead need gates). `InitialNeedSchema.value` is silently clamped at seed time (`schema.rs::to_agent_state` → `NeedState::initial` → `clamp_need_value`) instead of rejected — the authored seed and the materialized seed disagree with no error. **Free-text**: scan routing is asymmetric — `output_tag` goes only through `reject_shortcut_text` (`PHASE3A_SHORTCUT_MARKERS`), `display_label` only through `reject_script_marker_text`; the union is applied to no field, while the `SCANNED_STRING_FIELDS` census asserts mere membership, conflating the two policies (INV-022, INV-061/062, INV-080).

## Assumption Reassessment (2026-06-10)

1. Verified against current code at `main` `114e2af`: `validate_nonnegative_tuning` body (reject `< 0`, reject `> 10_000`, zero passes); `validate_phase3a_no_shortcuts` routes `workplaces[i].output_tag` through `reject_shortcut_text` only; `display_label` is scanned by `reject_script_marker_text` only; the `initial_needs` validation loops check duplicate `(actor, kind)` and actor references, never `value`; `work_duration_ticks`/`max_fatigue_to_start`/`max_hunger_to_start` appear in validation code only as test-fixture constructor values. `NEED_MAX: u16 = 1000` is exported from `crates/tracewake-core/src/agent/need.rs`.
2. Spec 0017 §ORD-HARD-030: a numeric field-policy registry parallel to the string one (pressure-direction `>= 0`, relief-direction `> 0`, duration `>= 1`, need-band `0..=NEED_MAX`, caps derived from `NEED_MAX`); reject out-of-band initial needs; union marker scan with per-field policy recorded.
3. Cross-crate boundary under audit: the content-schema → core-seed contract — content validation bounds must be expressed in terms of the core need band (`NEED_MAX`), not parallel constants that can drift.
4. INV-022/INV-061 restated: deterministic bounded seed state; designers author causal machinery, not outcomes — a numeric field that encodes a refill, a dead resource, or a vacuous gate is an outcome channel wearing a tuning name.
5. Fail-closed validation surface touched: all new rules are deterministic, blocking errors (existing `ContentValidationError` vocabulary), and must keep every existing golden fixture loading — current fixtures author values within the new bounds (verified for the canonical fixtures during the 0017 audit; any newly-surfaced out-of-band authored value is a fixture bug to fix, not a validation relaxation). No replay or leakage surface is involved.
6. Adjacent precedent classified: core's `need_state_v2` *parse* path already rejects `> NEED_MAX` (`need.rs`) — this ticket brings the content *seed* path to the same posture, replacing the clamp-at-load asymmetry; the clamp itself (`clamp_need_value`) stays for runtime delta arithmetic, where clamping is the modeled semantics. Per the 0016 precedent recorded in spec §ORD-HARD-034, content rejection coverage is delivered as named inline `validate.rs` unit tests (the repo-root `tests/negative-fixtures/` census is reserved for clippy/compile-fail fixtures).

## Architecture Check

1. A declarative per-field policy registry (field → direction class + bound expression) enumerated against the schema is cleaner than scattering ad-hoc `if` checks: a new numeric field fails the census until classified (the same mechanism that already works for string fields), and bounds derive from `NEED_MAX` so the band cannot drift in one place only. The union scanner with per-field policy makes the registry's assurance honest instead of conflated.
2. No backwards-compatibility aliasing/shims: the magic `10_000` cap is replaced, not kept as a fallback ceiling; the clamp-at-seed path is removed in favor of rejection.

## Verification Layers

1. INV-022 bounded seeds -> inline rejection tests: `fixture_workplace_zero_duration_rejected_001`, `fixture_initial_need_out_of_band_rejected_001`, plus relief-zero and band-cap cases — each asserting the exact `ContentValidationError` code.
2. INV-061/062 no outcome channels -> inline rejection test `fixture_output_tag_script_marker_rejected_001` (an authored-outcome marker in `output_tag` now rejected by the union scan).
3. Registry completeness -> census tests: every numeric tuning/seed field in `FixtureSchema` appears in the numeric policy registry; every free-text field's scan policy is the union or carries a recorded narrower-policy rationale — a new unclassified field fails the census.
4. Golden compatibility -> `cargo test --workspace`: all existing fixtures still load and replay unchanged (validation-only ticket; no event or checksum changes).

## What to Change

### 1. Numeric field-policy registry in `validate.rs`

A registry mapping each numeric schema field to a policy: pressure-direction (`>= 0`), relief-direction (`> 0`), duration (`>= 1`), need-band (`0..=NEED_MAX`), with per-tick/per-serving caps derived from `NEED_MAX`. `validate_state` consumes it; `validate_nonnegative_tuning`'s magic `10_000` is retired. New coverage: `work_duration_ticks >= 1`, `max_fatigue_to_start`/`max_hunger_to_start` in `0..=NEED_MAX`, `InitialNeedSchema.value <= NEED_MAX` (rejection, not clamp), relief fields `> 0`, `food.servings` policy recorded explicitly (zero stays legal as a deliberately-empty supply — distinguishable from a dead reducer).

### 2. Union marker scan with per-field policy

Free-text fields route through the union of `reject_shortcut_text` + `reject_script_marker_text` (+ authored-outcome markers); the `SCANNED_STRING_FIELDS` registry records which policy applies per field, and its census asserts policy, not membership.

### 3. Census + rejection tests

Numeric-registry census (enumerated against `FixtureSchema`); the named inline rejection tests in Verification Layers.

## Files to Touch

- `crates/tracewake-content/src/validate.rs` (modify)
- `crates/tracewake-content/tests/schema_conformance.rs` (modify — census extension, as surfaced by where the existing field-registry census lives)

## Out of Scope

- Core tuning-struct constructors and `clamp_need_value` runtime semantics (unchanged; the clamp stays correct for delta arithmetic).
- Schema field additions or shape changes (validation-only).
- Lock-layer guard mechanics and clippy bans (ticket `-008`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-content fixture_initial_need_out_of_band_rejected` — out-of-band authored need value is a blocking validation error, not a clamp.
2. `cargo test -p tracewake-content fixture_output_tag_script_marker_rejected` — union scan rejects authored-outcome markers in `output_tag`.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. Every numeric tuning/seed field in `FixtureSchema` carries a registered direction-and-bound policy; bounds derive from `NEED_MAX`, never a parallel constant.
2. No fixture free-text field escapes the union marker scan without a recorded narrower-policy rationale; authored seed values equal materialized seed values (no load-time coercion).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/src/validate.rs` (inline unit tests) — `fixture_workplace_zero_duration_rejected_001`, `fixture_initial_need_out_of_band_rejected_001`, `fixture_output_tag_script_marker_rejected_001`, relief-zero and band-cap rejections.
2. `crates/tracewake-content/tests/schema_conformance.rs` — numeric-registry census; per-field scan-policy census.

### Commands

1. `cargo test -p tracewake-content validate`
2. `cargo test --workspace`
