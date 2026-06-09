# 0014PHA3AORDLIF-007: Sleep-affordance content schema, fixtures, and content validation

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-content` (`schema.rs` sleep-surface schema, `validate.rs` content validation, sleep-routine fixtures), 1 adversarial golden fixture
**Deps**: 0014PHA3AORDLIF-006

## Problem

The modeled sleep/rest affordance added to `PhysicalState` (ticket 0014PHA3AORDLIF-006) must be authorable as typed content and enforced by content validation, and the Phase 3A sleep-routine fixtures must provide a sleep surface (or an explicit no-sleep diagnostic expectation) so the firewall is exercised through normal `cargo test`. Without this, the kernel validation has no authored surface to validate against and the no-human/ordinary-life fixtures cannot prove the corrected sleep path. This is ORD-HARD-005 (content half).

## Assumption Reassessment (2026-06-09)

1. Content schema/validation live in `crates/tracewake-content/src/schema.rs`, `crates/tracewake-content/src/validate.rs`, with loading in `load.rs` and the manifest in `manifest.rs`. Sleep-touching fixtures present today include `crates/tracewake-content/src/fixtures/sleep_eat_work_001.rs` and `ordinary_workday_001.rs` (returning `GoldenFixture`, registered in `fixtures/mod.rs`). The authoritative sleep/rest affordance component on `PhysicalState` is introduced by ticket -006 (state.rs).
2. Spec §ORD-HARD-005 requires content validation requiring Phase 3A sleep-routine fixtures to provide a sleep surface or an explicit no-sleep diagnostic expectation, and the negative fixture `sleep_rejects_current_place_without_sleep_affordance_001`.
3. Shared boundary under audit: the content sleep-surface schema ↔ the kernel affordance component (from -006) ↔ content validation. The contract: authored sleep surfaces are typed possibility (a rest surface with id/place/open-state), never a behavior selector or scripted outcome.
4. Invariants motivating this ticket: **INV-045** (ordinary survival is causal), **INV-061** (designers author affordances/initial state as possibility space, not guaranteed arcs), and **INV-060** (no authored outcome chains). The sleep surface is authored causal machinery, not a script.
5. Fail-closed content-validation + no-scripting enforcement surface: content validation must reject (block, not warn) a sleep-routine fixture that neither provides a sleep surface nor declares an explicit no-sleep diagnostic expectation, and must reject unknown / behavior-looking fields on the sleep-surface schema (no selectors/branches/triggers — no-scripting doctrine, foundation 09). Validation stays deterministic. This does not weaken actor-knowledge filtering: the authored surface is world truth; the actor still needs actor-known provenance (ticket -001) to use it.
6. Schema extension — additive-vs-breaking: adds a sleep/rest-surface entry to the content schema. Consumers: `validate.rs` (content validation), `load.rs` (loading into `PhysicalState`), and the fixtures. Additive; existing non-sleep fixtures are unaffected, but Phase 3A sleep-routine fixtures (`sleep_eat_work_001`, `ordinary_workday_001`) must gain a sleep surface to keep passing once -006's validation requires one.

## Architecture Check

1. Authoring the sleep surface as typed content (validated, unknown/behavior-looking fields rejected) keeps content as possibility, not script (foundation 09), and lets the kernel validate against authored truth rather than a hard-coded assumption — extensible to future rest-surface kinds without touching the kernel validator.
2. No backwards-compatibility shim: sleep-routine fixtures are updated to carry real sleep surfaces rather than relying on the removed current-place default; no parallel "implicit sleep place" content path is kept.

## Verification Layers

1. INV-045/061 (authored causal possibility) -> schema validation: the sleep-surface schema validates; unknown / behavior-looking fields are rejected.
2. INV-060 (no authored outcome chains) -> manual review + schema validation: the sleep surface carries no selector/trigger/branch field.
3. INV-043/044 (validation against modeled affordance) -> replay/golden-fixture check: `sleep_rejects_current_place_without_sleep_affordance_001` proves a sleep attempt at a place with no authored sleep surface is rejected; updated sleep-routine fixtures advance through the corrected path.

## What to Change

### 1. Sleep-surface content schema

In `crates/tracewake-content/src/schema.rs`, add a typed sleep/rest-surface schema entry (id, place, open-state) mapping to the kernel component from ticket -006; reject unknown and behavior-looking fields.

### 2. Content validation

In `crates/tracewake-content/src/validate.rs`, require Phase 3A sleep-routine fixtures to provide a sleep surface or carry an explicit no-sleep diagnostic expectation; block otherwise.

### 3. Provision existing fixtures + add negative fixture

Update `sleep_eat_work_001.rs` and `ordinary_workday_001.rs` to author a sleep surface. Add `sleep_rejects_current_place_without_sleep_affordance_001.rs` (no authored sleep surface → typed `NoSleepAffordance` rejection), registered in `fixtures/mod.rs`.

## Files to Touch

- `crates/tracewake-content/src/schema.rs` (modify — sleep-surface schema)
- `crates/tracewake-content/src/validate.rs` (modify — sleep-routine content validation)
- `crates/tracewake-content/src/fixtures/sleep_eat_work_001.rs` (modify — author sleep surface)
- `crates/tracewake-content/src/fixtures/ordinary_workday_001.rs` (modify — author sleep surface)
- `crates/tracewake-content/src/fixtures/sleep_rejects_current_place_without_sleep_affordance_001.rs` (new)
- `crates/tracewake-content/src/fixtures/mod.rs` (modify — register fixture; **shared hub**)

## Out of Scope

- The kernel affordance state + sleep validation (ticket 0014PHA3AORDLIF-006 — this ticket depends on it).
- Eat/work content-schema changes.

## Acceptance Criteria

### Tests That Must Pass

1. `sleep_rejects_current_place_without_sleep_affordance_001` — a sleep attempt with no authored sleep surface is rejected with the typed `NoSleepAffordance` reason.
2. `cargo test -p tracewake-content` — content validation blocks a sleep-routine fixture lacking a sleep surface and an explicit no-sleep expectation; rejects unknown/behavior-looking sleep-surface fields; updated `sleep_eat_work_001` / `ordinary_workday_001` pass.
3. `cargo test --workspace` — no-human/ordinary-life fixtures advance through the corrected sleep path; replay holds.

### Invariants

1. Sleep surfaces are typed authored possibility with no behavior-looking fields (INV-060/061, foundation 09).
2. Content validation is fail-closed on a missing sleep surface / unknown field (INV-045; arch 13 / exec 08).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/src/fixtures/sleep_rejects_current_place_without_sleep_affordance_001.rs` — adversarial no-sleep-surface rejection.
2. `crates/tracewake-content/src/fixtures/sleep_eat_work_001.rs` / `ordinary_workday_001.rs` — provisioned with authored sleep surfaces.

### Commands

1. `cargo test -p tracewake-content`
2. `cargo test --workspace`
3. `cargo test -p tracewake-core --test no_human_capstone` — confirms the corrected sleep path advances and replays.
