# 0007PHA3ASECHAR-002: Typed routine-family dispatch on RoutineExecution

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` `RoutineExecution` schema (`agent/routine.rs`), scheduler dispatch (`scheduler.rs`), replay rebuild (`replay/rebuild.rs`, `events/apply.rs`)
**Deps**: <none>

## Problem

The no-human runner picks the ordinary action family by substring-matching the routine template id (Spec 0007 D-03, D-01; §Forbidden shortcuts "HTN applicability decided by substring or prefix over raw strings"). `build_routine_or_need_proposal` (`crates/tracewake-core/src/scheduler.rs:547-556`) dispatches via `template_id.contains("sleep")`, `template_id.contains("food") || template_id.contains("eat")`, `template_id.contains("work")`. This exists because `RoutineExecution` (`crates/tracewake-core/src/agent/routine.rs:351`) carries only `template_id: RoutineTemplateId` with no typed family — so the semantic family is "discoverable only by substring", exactly what §Content-and-validation forbids. A template renamed without the magic substring silently stops dispatching.

## Assumption Reassessment (2026-06-07)

1. The substring dispatch is confirmed at `crates/tracewake-core/src/scheduler.rs:547-556`. `RoutineExecution` (`crates/tracewake-core/src/agent/routine.rs:351-365`) has `template_id` but no `family` field. The typed `RoutineFamily` enum already exists at `crates/tracewake-core/src/agent/routine.rs:9`, and `RoutineTemplate` (`routine.rs:268`) is the authoring surface that names a family.
2. Spec 0007 D-03 requires "No routine execution may dispatch by substring or template-name convention" (Binding constraint 4); §Content-and-validation requires routine templates use typed families and forbids template names whose semantic meaning is only discoverable by substring.
3. Shared boundary under audit: the `RoutineExecution` record between the agent state (producer, via routine assignment instantiation) and the scheduler dispatch + replay rebuild (consumers). The typed family must be derived from the assigned `RoutineTemplate`'s declared family at instantiation, then read directly — never re-parsed from the id string.
4. Motivating invariants (restated): INV-035 "Routines are defeasible intentions" and INV-036 "HTN methods are procedures, not story scripts" — a routine's family is modeled typed state, not an emergent property of its name.
5. Deterministic-replay surface touched: `RoutineExecution` is rebuilt during replay (`crates/tracewake-core/src/replay/rebuild.rs`, `events/apply.rs`). The typed family must be reconstructed deterministically from the same routine-assignment/template event ancestry, so the live and replay-rebuilt `RoutineExecution` are byte-identical for checksum purposes. No actor-knowledge leakage: the family is the actor's own routine assignment, not hidden world truth.
6. Schema change: `RoutineExecution` gains `family: RoutineFamily`. Consumers: scheduler dispatch (`scheduler.rs`), replay rebuild (`replay/rebuild.rs`), and any debug/view-model surface rendering routine state (read in 0007PHA3ASECHAR-009). The field is populated at instantiation from the typed template; it is additive to the struct but **replaces** the substring dispatch as the authority — there is no string-dispatch fallback.

## Architecture Check

1. Reading a typed `family` set at instantiation replaces a fragile name-convention lookup with an explicit modeled field — robust to template renames, inspectable in the trace, and validatable at content-load time (0007PHA3ASECHAR-011). It also removes the only place the scheduler parses a routine id string as semantic authority.
2. No backwards-compatibility aliasing/shims: the `template_id.contains(...)` dispatch is deleted; no substring fallback remains.

## Verification Layers

1. INV-035 / INV-036 (typed routine family) -> codebase grep-proof: no `template_id.contains(` remains in `scheduler.rs`; dispatch reads `execution.family`.
2. Deterministic replay -> replay/golden-fixture check: a no-human routine run replays with byte-identical `RoutineExecution.family` (agent-state checksum equality over a routine fixture).
3. Content typing -> schema validation (enforced in 0007PHA3ASECHAR-011): a routine template must declare a typed family; this ticket provides the field that validation keys on.

## What to Change

### 1. Add typed family to `RoutineExecution`

Add `family: RoutineFamily` to `RoutineExecution` (`agent/routine.rs`), populated at instantiation from the assigned `RoutineTemplate`'s declared family. Ensure canonical serialization (`serialize_canonical*`) and the replay rebuild path set and reproduce it deterministically.

### 2. Dispatch on the typed family

Rewrite `build_routine_or_need_proposal` (`scheduler.rs:547-556`) to `match execution.family { RoutineFamily::Sleep => sleep_proposal(..), RoutineFamily::Food/Eat => eat_proposal(..), RoutineFamily::Work => work_or_move_proposal(..), .. }`, removing all `template_id.contains(...)` checks.

## Files to Touch

- `crates/tracewake-core/src/agent/routine.rs` (modify)
- `crates/tracewake-core/src/scheduler.rs` (modify)
- `crates/tracewake-core/src/events/apply.rs` (modify)
- `crates/tracewake-core/src/replay/rebuild.rs` (modify)

## Out of Scope

- Removing the PhysicalState oracle inside `sleep_proposal`/`eat_proposal`/`work_or_move_proposal` (0007PHA3ASECHAR-004).
- Content-side validation that a template declares a typed family (0007PHA3ASECHAR-011).
- Routine step ancestry / interruption behavior (0007PHA3ASECHAR-007).

## Acceptance Criteria

### Tests That Must Pass

1. Grep-proof: `grep -n 'template_id.contains' crates/tracewake-core/src/scheduler.rs` returns nothing.
2. A unit test: a `RoutineExecution` whose `template_id` lacks any magic substring still dispatches to the correct proposal builder via its typed `family`.
3. Replay test: agent-state checksum equality for a routine fixture confirms `RoutineExecution.family` survives replay.
4. `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check` pass.

### Invariants

1. No substring/prefix dispatch over routine ids remains in `scheduler.rs`.
2. `RoutineExecution.family` is set from the typed template and reproduced identically in replay.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/agent/routine.rs` — unit tests: family populated from template; canonical serialization round-trip includes family.
2. `crates/tracewake-core/src/scheduler.rs` — unit test: dispatch by typed family with a non-magic template id.
3. `crates/tracewake-core/src/replay/rebuild.rs` — replay reconstructs `family` deterministically.

### Commands

1. `cargo test -p tracewake-core agent::routine`
2. `cargo test --workspace`
3. `cargo clippy --workspace --all-targets -- -D warnings`

## Outcome

Completed: 2026-06-07

What changed:

- Added `family: RoutineFamily` to `RoutineExecution`.
- Populated `RoutineExecution.family` from the selected or assigned `RoutineTemplate.family` in HTN selection and content routine-assignment instantiation.
- Replaced no-human routine dispatch by `template_id.contains(...)` with a `match` on `execution.family`.
- Added `family` to routine debug rows and agent-state checksum canonical input so replay/checksum comparisons observe the field.
- Added regression coverage for a routine execution whose template id has no magic substring but dispatches to `eat` through `RoutineFamily::EatMeal`.

Deviations from original plan:

- `events/apply.rs` only needed constructor test-state updates; routine step events mutate existing executions and do not reconstruct the family.
- `replay/rebuild.rs` did not need a code change because replay starts from canonical agent state and applies events to existing routine executions; adding `family` to the checksum canonical input makes replay equality sensitive to the field.

Verification results:

- `rg -n "template_id\\.contains" crates/tracewake-core/src/scheduler.rs` returned no matches.
- `cargo test -p tracewake-core agent::routine` passed.
- `cargo test -p tracewake-core scheduler::no_human::tests::routine_dispatch_uses_family_when_template_id_has_no_magic_substring` passed.
- `cargo test --workspace` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo fmt --all --check` passed.
