# 0050FOUCONSEC-015: Bound the perception event-id collision loop and witness its rename path

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — production change in `crates/tracewake-core/src/agent/perception.rs` (bound an unbounded `loop` in `record_current_place_perception`); no events/schemas/fixtures/skills/docs change.
**Deps**: 0050FOUCONSEC-011 (recorded the standing survivors and routed real survivors to separate remediation tickets)

## Problem

The CI gate `mutants-in-diff` ("mutation in-diff (lock layer)", `.github/workflows/ci.yml`) fails on PR #62 / branch `implemented-0050`. Four of the nine new survivor classes are in `crates/tracewake-core/src/agent/perception.rs`, all in the event-id collision-rename path:

- `delete ! in record_current_place_perception` (L41) — **TIMEOUT**
- `replace += with -= in record_current_place_perception` (L45) — MISSED
- `replace += with *= in record_current_place_perception` (L45) — MISSED
- `delete match arm "source_event_id" in rename_perception_event` (L59) — MISSED

`record_current_place_perception` (`:20`) renames duplicate current-place perception events to a deterministic log-frontier id before `EventLog`'s fail-closed duplicate check. The collision branch is an **unbounded** `loop` (`:38`) that increments `occurrence` until a free `EventId` is found:

```rust
let mut occurrence = log.events().len();
loop {
    let candidate = EventId::new(format!("{base}.occurrence.{occurrence}")).unwrap();
    if !log.contains_event_id(&candidate) {        // L41
        rename_perception_event(&mut event, candidate);
        break;
    }
    occurrence += 1;                                // L45
}
```

The `delete !` mutant at L41 makes the loop never break → infinite loop → it is **always** a TIMEOUT, which no test can convert to CAUGHT. The user-approved remediation is to **bound the loop** (fail-closed cap) so the negation-deleted mutant exhausts the cap and panics (killable + hardens against a real infinite loop), then add witnesses that exercise the collision/rename path.

## Assumption Reassessment (2026-06-24)

1. `record_current_place_perception` (`crates/tracewake-core/src/agent/perception.rs:20`) enters the collision branch only when `log.contains_event_id(&event.event_id)` is true (`:35`). The inner `loop` (`:38`) breaks on the first free candidate (`:41–43`); `occurrence += 1` (`:45`) executes only when the **first** candidate (`occurrence = log.events().len()`) also collides — which is why L45 mutants are MISSED (existing duplicate-path coverage triggers only a single collision, so L45 never runs) while the L41 `delete !` is a TIMEOUT (the branch does run, and the mutated loop hangs).
2. Termination is guaranteed for correct input: candidate ids are distinct, and with at most `log.events().len()` existing events, a free id exists within `log.events().len() + 1` distinct candidates (pigeonhole). So a fail-closed cap of `log.events().len() + 1` attempts changes no observable behavior on correct input while making the `delete !` mutant terminate-and-fail.
3. `.cargo/mutants-baseline-misses.txt` is empty (verified); the gate accepts zero misses. The user chose bound-the-loop over baseline-accepting the timeout, so no baseline entry is added.
4. `INV-092` (deterministic replay is tested) is the primary invariant: this path assigns deterministic frontier ids that replay depends on; the bound preserves the exact id sequence on correct input (`base.occurrence.{len}`, `…{len+1}`, …) while the cap only fires on impossible input. `INV-098` (feature acceptance is harsh) requires the regression evidence measured via mutation. `INV-101`/`INV-093` apply because perception events feed actor-known context.
5. Enforcement surface: the collision-rename path in `perception.rs`. The bound is a fail-closed guard (panic/`expect` with a clear message on impossible exhaustion), consistent with `EventLog`'s fail-closed duplicate posture; it does not weaken epistemic-leakage prevention and preserves the deterministic id sequence, so replay is unaffected. `rename_perception_event` (`:54`) must rewrite both `observation_id` and `source_event_id` payload fields and the envelope `event_id`; the `delete match arm "source_event_id"` survivor shows the `source_event_id` rewrite is unwitnessed.

## Architecture Check

1. Bounding the loop is cleaner than baseline-accepting the timeout: it removes a real (if currently unreachable) infinite-loop hazard, converts an unkillable forced-timeout into a CAUGHT mutant, and keeps `.cargo/mutants-baseline-misses.txt` empty so the gate stays sharp. Baseline-accepting would add the first miss to a deliberately-empty perimeter and leave the hazard in production.
2. No backwards-compatibility aliasing/shims introduced. The cap is a fail-closed guard, not a compatibility branch; the happy-path id sequence is byte-identical.

## Verification Layers

1. `INV-092` (deterministic replay tested) -> replay/golden-fixture check + manual review: a witness drives a double-collision so the loop iterates past L45 and asserts the **exact** deterministic candidate id sequence is unchanged by the bound.
2. `INV-098` (regression evidence measured) -> mutation campaign: `cargo mutants` reports `delete !` (L41), `+= → -=` and `+= → *=` (L45), and `delete match arm "source_event_id"` (L59) as CAUGHT.
3. `INV-101`/`INV-093` (perception feeds actor-known context) -> manual review: the rename preserves correct `observation_id` / `source_event_id` provenance so downstream actor-known context is not corrupted.

## What to Change

### 1. Bound the candidate-id search loop (production)

In `record_current_place_perception`, replace the unbounded `loop` with a bounded search of at most `log.events().len() + 1` attempts. On exhaustion (impossible for correct input), fail closed with a clear `panic!`/`expect` message (e.g. "no free perception-event frontier id within bound") rather than spinning. The happy-path id sequence (`base.occurrence.{len}`, `{len+1}`, …) must be unchanged.

### 2. Witness the collision and rename path (test)

In the `perception.rs` test module (`#[cfg(test)] mod tests` at `:745`), add tests that:
- pre-seed an `EventLog` so the base perception id **and** `base.occurrence.{len}` already exist, forcing the loop to iterate past L45; assert the renamed event's `event_id` equals the exact expected `base.occurrence.{len+1}` (kills `+= → -=` and `+= → *=`, and — via the bound — `delete !`);
- assert the renamed event's `source_event_id` payload field equals the new id (kills `delete match arm "source_event_id"`), alongside `observation_id`.

## Files to Touch

- `crates/tracewake-core/src/agent/perception.rs` (modify — bounded loop in production + witnesses in test module)

## Out of Scope

- Any `.cargo/mutants.toml` perimeter change (spec-0050 §6.3: none).
- Any `.cargo/mutants-baseline-misses.txt` entry — the timeout is remediated by bounding, not accepted.
- The `projections.rs` dedup (`-014`) and epistemics novelty (`-016`) survivors — separate tickets/diffs.

## Acceptance Criteria

### Tests That Must Pass

1. A double-collision witness asserts the exact deterministic renamed id (`base.occurrence.{len+1}`) and that both `observation_id` and `source_event_id` payload fields are rewritten.
2. `cargo mutants --in-diff` over the changed lines reports all four `perception.rs` survivors (`delete !` L41, `+= → -=` / `+= → *=` L45, `delete match arm "source_event_id"` L59) as CAUGHT.
3. The four gates pass (`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace --all-targets --locked`, `cargo test --workspace`).

### Invariants

1. The collision-rename path terminates (fail-closed bound) and assigns the same deterministic frontier ids on correct input (`INV-092`).
2. The rename preserves `observation_id` and `source_event_id` provenance for downstream actor-known context (`INV-101`, `INV-093`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/agent/perception.rs` (`mod tests`) — double-collision witness asserting exact renamed id + payload-field rewrites; single-collision case retained.

### Commands

1. `cargo test -p tracewake-core perception`
2. `cargo mutants --in-diff <(git diff origin/main...HEAD -- crates/tracewake-core/src/agent/perception.rs) --no-shuffle --jobs 2 --timeout 183` — reproduces the in-diff gate's input selection for the changed `perception.rs` lines.
3. The per-file mutation command is the correct boundary because the CI gate scopes mutation to changed guarded-layer lines; reproduce the gate's `--timeout 183` so the bounded loop is confirmed to no longer time out.

## Outcome

Completed: 2026-06-24

Bounded the perception event-id collision search in `record_current_place_perception` with a fail-closed attempt cap while preserving the existing deterministic candidate sequence. Added a double-collision witness in `crates/tracewake-core/src/agent/perception.rs` that pre-seeds the base perception id and the first frontier candidate, then asserts the next frontier id plus rewritten `observation_id` and `source_event_id` payload fields.

Verification run:

- `cargo test -p tracewake-core perception` — pass.
- `cargo mutants --in-diff <(git diff origin/main...HEAD -- crates/tracewake-core/src/agent/perception.rs) --no-shuffle --jobs 2 --timeout 183` — not accepted as evidence; failed before mutation because the committed diff was stale against the edited working tree (`loop` in diff, bounded `for` in source).
- `cargo mutants --in-diff <(git diff origin/main -- crates/tracewake-core/src/agent/perception.rs) --no-shuffle --jobs 2 --timeout 183` — pass; 8 mutants tested, 7 caught, 1 unviable, 0 missed.

No mutation perimeter or baseline files changed. The command selector was adjusted from `origin/main...HEAD` to `origin/main` so `cargo mutants --in-diff` consumed a diff matching the current source tree after this ticket's edit.
