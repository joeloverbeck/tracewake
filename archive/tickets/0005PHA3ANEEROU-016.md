# 0005PHA3ANEEROU-016: Phase 3A content validation rules and forbidden teleport/refill fields

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adds Phase 3A fail-closed validation (routine preconditions/failure-modes/interruption/no-teleport) and extends the forbidden-content list with schedule-teleport and refill shortcut fields.
**Deps**: 0005PHA3ANEEROU-015

## Problem

Phase 3A content validation is mandatory and fail-closed: routine templates must have failure modes and bounded durations, no step may teleport an actor/item or set needs without ancestry, and the validator must reject schedule-teleport fields (`appear_at`, `force_location_at_tick`, `scripted_absence`, `story_beat`) and shortcut effects (hunger refill without food, instant sleep refill, work-always-succeeds) — extending, not replacing, the current forbidden-content tests (Spec 0005 §16.4, §16.5; `INV-060`, `INV-061`). This is the no-scripting gate guarding every Phase 3A fixture.

## Assumption Reassessment (2026-06-07)

1. Content validation lives in `crates/tracewake-content/src/validate.rs`; the forbidden-field gate is `is_forbidden_key`/`is_player_key`/`is_script_key` (`validate.rs:1026-1035`) and the existing forbidden set already rejects `culprit`/`true_culprit`/`stolen_flag`/`npc_knows_truth`/`quest_state`/`player_memory`/`force_event`/`quest`/`reward`/`player`/`director` (confirmed in `validate.rs` + `tests/forbidden_content.rs:37-44`). Spec §16.5 says the new forbidden list "must extend the current forbidden-content tests rather than replace them" — this ticket adds to the existing set.
2. The Phase 3A schema sections (ticket 015) provide the parsed routine/food/work/needs data this validates. Spec §16.4 fixes routine validation (actor/routine refs exist; every step has a valid kind; every concrete action maps to a known registry/declared Phase 3A action; ≥1 failure mode; positive bounded durations; interruption points declared; fallback declared or explicit "fail with diagnostic"; no step teleports actor/item; no step sets needs without action/time ancestry; no hidden true item location; no player/story/culprit/quest fields). Spec §16.5 fixes the invalid-content examples and the new forbidden teleport fields.
3. Shared boundary under audit: this extends the repo-wide forbidden-content contract; the new keys must be added to `validate.rs`'s forbidden set AND asserted in `tests/forbidden_content.rs` (the test enumerates each forbidden key). The registry-action existence check cross-references the action IDs registered by tickets 008–011.
4. Invariants motivating this ticket: `INV-060` — "No authored outcome chains" (forbidden: scenario scripts, drama directors, guaranteed targets/rewards, player-conditioned injection) — and `INV-061` — "Authored causal machinery is required" (designers author possibility, not guaranteed arcs). A schedule-teleport field or a hunger-refill-without-food fixture is exactly the authored outcome chain `INV-060` forbids. Validation must be deterministic, fail-closed, and blocking (`docs/1-architecture/13_*`).
5. Fail-closed / no-scripting surface: this IS the Phase 3A no-scripting enforcement surface. It blocks behavior-looking fields and authored shortcuts; failures are blocking errors, not warnings (matching the existing `forbidden_form`/`reserved_player_or_story_id` blocking phases at `validate.rs:179-189`, `342`). It adds no leakage and no nondeterminism — validation is a deterministic pass over parsed content.

## Architecture Check

1. Extending the existing forbidden-key gate and routine-structural validation (rather than a parallel Phase 3A validator) keeps one fail-closed content gate with one blocking-vs-warning contract, so Phase 3A shortcuts are rejected by the same machinery that already rejects quest/culprit fields (`INV-060`). Cross-checking routine step actions against the registered action IDs makes "routine names an unknown action" a load-time error, not a runtime surprise.
2. No backwards-compatibility shims: additive forbidden keys and additive routine-validation rules; the existing forbidden set and its tests are extended, not rewritten.

## Verification Layers

1. No authored outcome chains (`INV-060`) -> schema validation + unit test: a fixture with `appear_at`/`force_location_at_tick`/`scripted_absence`/`story_beat`, hunger-refill-without-food, instant-sleep-refill, or work-always-succeeds is rejected with a blocking error.
2. Authored possibility (`INV-061`) -> schema validation: a routine with no failure modes, a non-positive duration, an unknown step action, or a dangling actor/routine reference is rejected.
3. Fail-closed/blocking (`docs/1-architecture/13_*`) -> unit test: each new rejection is a blocking error (not a warning) and names what failed; valid Phase 3A fixtures still pass.

## What to Change

### 1. Forbidden teleport/shortcut fields

Extend the forbidden-key set in `crates/tracewake-content/src/validate.rs` (`is_script_key`/`is_forbidden_key` path) with the §16.5 schedule-teleport fields (`appear_at`, `force_location_at_tick`, `scripted_absence`, `story_beat`) and any §16.5 shortcut markers, as blocking errors. Add the corresponding assertions to `crates/tracewake-content/tests/forbidden_content.rs` (extend the enumerated forbidden-key list).

### 2. Routine + effect validation rules

Add §16.4 routine validation in `validate.rs`: actor/routine reference existence, valid step kinds, every concrete action maps to a known registry/declared Phase 3A action, ≥1 failure mode, positive bounded durations, declared interruption points, declared fallback (or explicit fail-with-diagnostic), no direct teleport/item-move/need-set step, no hidden true item location. Add the §16.5 effect rejections: hunger refill without food/service/action, instant sleep refill, work-always-succeeds.

## Files to Touch

- `crates/tracewake-content/src/validate.rs` (modify — forbidden teleport/shortcut keys + routine/effect validation rules)
- `crates/tracewake-content/tests/forbidden_content.rs` (modify — assert the new forbidden keys)

## Out of Scope

- Schema parsing of the new sections (ticket 015).
- The golden fixtures (tickets 019, 020, 021) — this gate guards them.
- Core action/registry definitions (tickets 008–011) — this only cross-references their IDs.

## Acceptance Criteria

### Tests That Must Pass

1. Fixtures containing `appear_at`/`force_location_at_tick`/`scripted_absence`/`story_beat` are rejected with blocking errors asserted in `forbidden_content.rs`.
2. A routine with no failure modes / non-positive duration / unknown step action / dangling reference is rejected; a hunger-refill-without-food, instant-sleep-refill, or work-always-succeeds fixture is rejected.
3. Valid Phase 3A fixtures still validate; existing Phase 0–2A forbidden-content assertions still pass (extension, not replacement).

### Invariants

1. Schedule-teleport and refill shortcuts are blocking errors; no authored outcome chain passes (`INV-060`).
2. Routine content is validated as defeasible authored possibility; validation is deterministic and fail-closed (`INV-061`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/tests/forbidden_content.rs` (modify) — new teleport/shortcut forbidden-key assertions.
2. `crates/tracewake-content/src/validate.rs` (unit tests) — routine structural rejections and effect-shortcut rejections.

### Commands

1. `cargo test -p tracewake-content forbidden_content`
2. `cargo test -p tracewake-content validate`
3. Content-crate scope is correct; full fixture validation under the no-human day is exercised by tickets 021/025.

## Outcome

Completed 2026-06-07.

Extended the content validator's forbidden script-key gate with Phase 3A schedule-teleport and shortcut-effect markers, including `appear_at`, `force_location_at_tick`, `scripted_absence`, `teleport_actor`, `move_item_to`, `set_need`, `hunger_refill_without_food`, `instant_sleep_refill`, `work_always_succeeds`, and `hidden_true_item_location`.

Added parsed Phase 3A routine validation: concrete routine step semantic action IDs must resolve to a registered base action, routines must declare fallback rules or an explicit diagnostic failure, and authored string fields in sleep places, workplaces, and routine templates reject shortcut markers as blocking `NoScript` errors. Existing routine shape validation from ticket 015 still catches missing failure modes, non-positive duration, invalid interruption points, and dangling routine references.

Verification:

1. `cargo fmt --all --check`
2. `cargo test -p tracewake-content forbidden_content`
3. `cargo test -p tracewake-content validate`
4. `cargo test -p tracewake-content`
5. `git diff --check`

Deviations: none. The fixture-level unknown-field test now uses a genuinely unknown key because `teleport_actor` is intentionally a known forbidden script key after this ticket.
