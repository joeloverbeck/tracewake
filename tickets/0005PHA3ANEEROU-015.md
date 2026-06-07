# 0005PHA3ANEEROU-015: Phase 3A content schema additions

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — extends the content schema/loader to parse Phase 3A fixture data: initial needs, homes/sleep/food/work entities, routine templates/assignments, and day windows.
**Deps**: 0005PHA3ANEEROU-004, 0005PHA3ANEEROU-009, 0005PHA3ANEEROU-010

## Problem

Phase 3A ordinary-life setup must not hide in Rust-only test helpers — homes, sleep places, food supplies, workplaces, initial needs, routine assignments, day windows, and Phase 3A affordances must be authored content the loader parses with deterministic canonical ordering and dangling/duplicate rejection (Spec 0005 §16.1, §16.2, §16.3; `INV-061`). This schema is the data contract the validation rules (ticket 016) enforce and the golden fixtures (019–021) are written against.

## Assumption Reassessment (2026-06-07)

1. The content schema/loader live in `crates/tracewake-content/src/schema.rs`, `load.rs`, `manifest.rs`, and `serialization.rs` (all confirmed); the existing schema models actors/places/doors/containers/items/affordances/initial-beliefs (Spec §4.1) with deterministic canonical ordering and duplicate/dangling rejection. This ticket extends those additively.
2. The core entity shapes this schema mirrors are fixed by tickets 001 (needs), 003 (routine template/step/family), 009 (food_supply), 010 (workplace). Spec §16.1 fixes the schema surface (initial needs; homes/sleep/food/work; routine templates/assignments; routine windows/day windows; Phase 3A affordances; and the rejection rules for player-only verbs / hidden outcome chains / schedule teleport — the *rules* are ticket 016, but the *fields* are parsed here). Spec §16.2/§16.3 fix the food and work schema field sets.
3. Shared boundary under audit: this schema is the additive extension of the existing content schema; per `INV-020` it must be additive (new sections/fields) so existing Phase 0–2A fixtures still load unchanged. The field names must match the core entity field names (tickets 001/003/009/010) exactly — a drift makes a fixture parse to the wrong core shape.
4. Invariant motivating this ticket: `INV-061` — "Authored causal machinery is required" (designers author needs, routines, HTN methods, initial state, fixtures as possibility space, not guaranteed arcs). The schema parses authored possibility; it must reject unknown fields by default (existing loader discipline) so behavior-looking fields are caught (the no-scripting *rules* land in ticket 016).
5. Deterministic / fail-closed surface (substrate-only): the loader's canonical ordering and reject-unknown-by-default behavior feed determinism (`INV-018`) and fail-closed validation. This ticket introduces no nondeterminism — parsing preserves canonical ordering and rejects duplicates/dangling refs. The behavior-looking-field and teleport/refill rejection *rules* are ticket 016 (cited); this ticket guarantees the new sections parse deterministically and reject unknown fields.

## Architecture Check

1. Extending the existing content schema additively (rather than a parallel Phase 3A loader) keeps one content pipeline with one canonical ordering and one reject-unknown gate, so Phase 3A data inherits the existing determinism/fail-closed discipline (`INV-020`, `INV-061`). Matching core entity field names exactly avoids a translation layer that could silently drift.
2. No backwards-compatibility shims: additive schema sections with defaults; existing fixtures load unchanged; no alias fields.

## Verification Layers

1. Authored possibility (`INV-061`) -> schema validation: the new sections parse needs/homes/sleep/food/work/routines/windows into the core entity shapes; unknown fields are rejected by the existing loader default.
2. Additive evolution (`INV-020`) -> replay/golden-fixture check: all existing Phase 0–2A fixtures still load byte-identically after the schema extension.
3. Determinism (`INV-018`, substrate-only) -> unit test: loading a Phase 3A fixture yields canonical ordering and rejects duplicate/dangling references. Behavior-field/teleport/refill rejection rules are ticket 016 (cited).

## What to Change

### 1. Schema sections

Extend `crates/tracewake-content/src/schema.rs` with Phase 3A sections: initial need state (per actor, the §8.1 needs); homes/home places; sleep places/beds; food supplies (ID/location/servings/per-serving-reduction, §16.2); workplaces/workstations (ID/place/assigned-actors/window/duration/access/thresholds/output-tag, §16.3); routine templates or routine assignments (§16.4 field set); routine durations/tick-windows/day-windows; Phase 3A action affordances. Field names mirror the core entity shapes (tickets 001/003/009/010).

### 2. Loader + manifest

Extend `crates/tracewake-content/src/load.rs` (and `manifest.rs`/`serialization.rs` as needed) to parse the new sections with canonical ordering and duplicate/dangling-reference rejection, including the expected actor roster and no-human-day manifest entries (§16.1).

## Files to Touch

- `crates/tracewake-content/src/schema.rs` (modify — Phase 3A schema sections)
- `crates/tracewake-content/src/load.rs` (modify — parse new sections, roster/day-window manifest)
- `crates/tracewake-content/src/manifest.rs` (modify — no-human-day manifest entries, if modeled here)
- `crates/tracewake-content/src/serialization.rs` (modify — canonical (de)serialization of new sections, if needed)

## Out of Scope

- The validation *rules* (routine preconditions/failure-modes/interruption/no-teleport, hunger-refill/instant-sleep rejection, forbidden teleport fields) — ticket 016.
- The golden fixtures themselves (tickets 019, 020, 021).
- Core entity definitions (tickets 001, 003, 009, 010).
- TUI/debug surfaces (tickets 022, 023).

## Acceptance Criteria

### Tests That Must Pass

1. A Phase 3A fixture with initial needs, a home, a sleep place, a food supply, a workplace, a routine assignment, and day windows loads into the correct core entity shapes with canonical ordering.
2. Duplicate IDs and dangling actor/place/food/work/routine references are rejected; unknown fields are rejected by the loader default.
3. All existing Phase 0–2A fixtures still load unchanged (additive-evolution proof).

### Invariants

1. The schema parses authored possibility into core entity shapes; field names match the core entities exactly (`INV-061`).
2. The extension is additive; existing fixtures are unaffected; unknown fields fail closed (`INV-020`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/tests/fixtures_load.rs` (modify) — load a Phase 3A sample fixture; canonical ordering; dangling/duplicate rejection.
2. `crates/tracewake-content/src/schema.rs` / `load.rs` (unit tests) — per-section parse and field-name fidelity to core shapes.

### Commands

1. `cargo test -p tracewake-content fixtures_load`
2. `cargo test -p tracewake-content`
3. Content-crate scope is correct; the validation-rule rejections are ticket 016 and the golden fixtures are tickets 019–021.
