# 0016PHA3ANEEACC-009: Directional content validation, required tuning, scanned-field registry

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-content` numeric/prose validation rules; `tracewake-core` tuning-constructor hardening (`Default` deletion); two new negative fixtures; schema String-field census
**Deps**: None

## Problem

ORD-HARD-022: content validation permits refill-direction tuning, latent kernel defaults, and unscanned prose channels (INV-061, INV-080; archived 0005 §16.5's rejection class — "hunger refills without food/service/action" is reachable today through legal *numeric* fields; foundation doc 09 — no simulation fact born from prose):

- `validate.rs::validate_state` checks `duration_ticks == 0` but imposes no sign or bound on `awake_hunger_delta_per_tick`, `awake_fatigue_delta_per_tick`, `fatigue_recovery_per_tick`, `hunger_rise_per_tick`, workplace `fatigue_delta_per_tick`/`hunger_delta_per_tick`, or `hunger_reduction_per_serving`. `awake_hunger_delta_per_tick: -50` is a passive hunger refill with no food ancestry and no banned word — the forbidden-content scans are name blacklists and never see it.
- Latent kernel tuning: `NeedModelState::default()` returns `{5, 3}`; `SleepAffordanceState::new` and `WorkplaceState::new` hardcode tuning; `PhysicalState::from_seed_parts` seeds `NeedModelState::default()` — the ORD-HARD-012 shortcut surviving as fallbacks reachable by any non-content construction site.
- `PlaceSchema.display_label` flows into `PlaceState` unvisited by `validate_no_script`, `contains_prose_born_fact_marker`, or `reject_script_marker_text` — a free-text channel into authoritative state.

## Assumption Reassessment (2026-06-10)

1. Current code verified at baseline `ba84e75`: `validate_state` duration check at `crates/tracewake-content/src/validate.rs:858–889` with no sign/bound rules on the listed fields; `PHASE3A_SHORTCUT_MARKERS` / `is_script_key` string scans at :1413–1432; `NeedModelState::default()` `{5, 3}` at `crates/tracewake-core/src/state.rs:398–405`; `from_seed_parts` seeds the default at :173; `SleepAffordanceState::new` hardcodes `{4, 20, 2}`; `PlaceSchema.display_label` (`schema.rs:282`) → `PlaceState` (`state.rs:117`), unscanned. Authored fixture values are all positive today, so the new directional rules break no existing fixture. `Default`-deletion blast radius (grep-enumerated): `state.rs:173` (`from_seed_parts`), test callers `time.rs:60/:66/:67` and `scheduler.rs:3239`.
2. Spec/docs: spec 0016 §ORD-HARD-022 (evidence, required correction, structural lock); `archive/specs/0005_*` §16.5 (:1317); `docs/0-foundation/09_NO_SCRIPTING_AUTHORING_SEEDS_AND_PREHISTORY.md`; `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` INV-061, INV-080.
3. Shared boundary under audit: the content→core tuning seam — all need/sleep/work tuning is authored content; the kernel carries no reachable fallback values, and every authored free-text channel passes the prose scans.
4. INV-061 — authored causal machinery creates possibility space; a sign-inverted rate is a script wearing a number. INV-080 — domain content may not bypass foundation doctrine. Foundation 09 — no simulation fact born from prose; "do not hide scripts behind prettier names." Restated before trusting the ticket narrative.
5. Fail-closed validation enforcement surface: `validate_state` gains deterministic, blocking, directional/bounded numeric rules (pressure-direction deltas `>= 0`, relief-direction fields `>= 0` in their modeled direction, magnitude caps) and the must-be-scanned field registry. Both reject — no warnings-as-passes; the String-field census makes an unclassified new free-text field a test failure (fail-closed at the schema level). No replay surface is touched (validation precedes simulation).
6. Schema/validation extension: `validate_state` rules extend the content-validation contract; consumers are the fixture set (all currently-valid fixtures remain valid — verified positive values) and the negative-fixture runner. Additive for valid content; rejecting-by-design for newly-invalid content.
7. Removal blast radius: `Default for NeedModelState` and zero-arg tuning constructors are deleted; `from_seed_parts` takes an explicit `NeedModelState`. Enumerated call sites to update: `state.rs:173`, `time.rs:60/:66/:67` (tests construct explicit tuning), `scheduler.rs:3239` (test), plus any `from_seed_parts` callers surfaced at compile time (compile-driven enumeration — the deletion makes missed sites unbuildable, which is the lock working).

## Architecture Check

1. Directional validation closes the class, not the instance: name blacklists catch words, never arithmetic — a sign constraint on every pressure/relief field makes the §16.5 rejection class structural. Deleting `Default`/zero-arg constructors converts "content overrides the default" into "no default exists," so non-content construction sites cannot silently resurrect kernel tuning (compile-level, the spec's preferred tier). The scanned-field registry inverts the prose-scan posture from enumerate-known-fields to classify-every-String-field.
2. No backwards-compatibility aliasing/shims: `Default for NeedModelState` is deleted, not deprecated; no `new_with_defaults` alias appears; tuning-bearing constructors take all tuning as required parameters.

## Verification Layers

1. INV-061/0005 §16.5 (no numeric refill scripts) → negative fixture `fixture_negative_awake_delta_rejected_001`: a negative awake hunger delta is rejected by `validate_state`.
2. Foundation 09 (no prose-born facts) → negative fixture `fixture_display_label_script_marker_rejected_001`: a script marker in `display_label` is rejected.
3. Registry completeness → schema census test: every `String` field in `FixtureSchema` is enumerated against the scanned-field registry; a new free-text field fails until classified.
4. No latent kernel tuning → compile-level proof: no `Default`/zero-arg constructor exists for tuning-bearing state structs (grep-proof + the workspace fails to compile if a caller still expects one).

## What to Change

### 1. Directional and bounded numeric validation

In `validate_state`: pressure-direction deltas (`awake_hunger_delta_per_tick`, `awake_fatigue_delta_per_tick`, `hunger_rise_per_tick`, workplace `fatigue_delta_per_tick`/`hunger_delta_per_tick`) `>= 0`; relief-direction fields (`fatigue_recovery_per_tick`, `hunger_reduction_per_serving`) `>= 0` in their modeled direction; magnitude caps for all of them. Deterministic, blocking rejections.

### 2. Delete latent kernel tuning

Remove `Default for NeedModelState`; make `SleepAffordanceState::new` / `WorkplaceState::new` take all tuning as required parameters; `PhysicalState::from_seed_parts` takes an explicit `NeedModelState`. Update the enumerated callers (production + tests).

### 3. Must-be-scanned field registry

Route `display_label` (and every `FixtureSchema` `String` field) through the script/prose marker scans via a registry; add the census test enumerating every `String` field against it.

### 4. Negative fixtures

`fixture_negative_awake_delta_rejected_001` and `fixture_display_label_script_marker_rejected_001`, registered in `negative_fixture_runner.rs::FIXTURES`.

## Files to Touch

- `crates/tracewake-content/src/validate.rs` (modify)
- `crates/tracewake-content/src/schema.rs` (modify — registry plumbing)
- `crates/tracewake-core/src/state.rs` (modify)
- `crates/tracewake-core/src/time.rs` (modify — test callers)
- `crates/tracewake-core/src/scheduler.rs` (modify — test caller)
- `crates/tracewake-core/tests/negative_fixture_runner.rs` (modify — FIXTURES registration)
- `tests/negative-fixtures/fixture_negative_awake_delta_rejected_001/` (new — negative fixture directory)
- `tests/negative-fixtures/fixture_display_label_script_marker_rejected_001/` (new — negative fixture directory)

## Out of Scope

- Need-delta routing/accounting semantics (`archive/tickets/0016PHA3ANEEACC-002.md` — this ticket constrains authored values, not how they are charged).
- The fixture-directory census reconciling `FIXTURES` against disk (0016PHA3ANEEACC-012).
- New tuning vocabulary or content fields (validation of the existing surface only).

## Acceptance Criteria

### Tests That Must Pass

1. Both negative fixtures rejected with deterministic, typed validation errors; all existing positive fixtures still validate.
2. Schema census test: every `FixtureSchema` `String` field classified in the scanned-field registry (adding an unclassified field fails).
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`.

### Invariants

1. No tuning-bearing state struct has a `Default` or zero-arg constructor; all tuning is authored content (INV-061).
2. Every authored free-text field passes the prose/script scans; pressure/relief numeric fields are direction- and bound-constrained — fail-closed, blocking, no override.

## Test Plan

### New/Modified Tests

1. `tests/negative-fixtures/fixture_negative_awake_delta_rejected_001/` — numeric-direction rejection lock.
2. `tests/negative-fixtures/fixture_display_label_script_marker_rejected_001/` — prose-channel rejection lock.
3. `crates/tracewake-content/src/validate.rs` — schema String-field census test against the scanned-field registry.

### Commands

1. `cargo test -p tracewake-content validate && cargo test -p tracewake-core --test negative_fixture_runner`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Implemented ORD-HARD-022 content/tuning hardening:

- `validate_state` now rejects negative or excessive need/sleep/work/food tuning values with typed validation errors.
- `places[*].display_label`, affordance targets, and routine failure-mode strings now pass through script/prose marker scanning.
- Added a schema string-field census test so new `FixtureSchema` string fields fail until classified in the scan registry.
- Removed `Default` for `NeedModelState`; `PhysicalState` no longer derives `Default`.
- Added explicit `PhysicalState::empty(NeedModelState)`, required `NeedModelState` in `from_seed_parts`, and made sleep/workplace constructors require tuning values.
- Updated test construction sites to pass explicit authored tuning.

The two requested negative content cases are covered by focused content validation tests (`negative_need_tuning_direction_is_rejected` and `display_label_script_marker_is_rejected`). The existing `negative_fixture_runner` remains compile-fail-crate oriented and was verified unchanged.

Verification run:

- `cargo test -p tracewake-content validate`
- `cargo test -p tracewake-core --test negative_fixture_runner`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`
