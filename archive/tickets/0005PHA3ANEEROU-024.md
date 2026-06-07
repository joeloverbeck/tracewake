# 0005PHA3ANEEROU-024: Cross-cutting docs — README Phase 3A surface and fixture-contract notes

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — documentation only: updates `README.md` to describe the Phase 3A ordinary-life surface (needs, ordinary actions, routines, no-human day, fixtures, debug commands).
**Deps**: 0005PHA3ANEEROU-019, 0005PHA3ANEEROU-021, 0005PHA3ANEEROU-023

## Problem

Once Phase 3A lands, the repo's player-facing/runbook documentation must describe the new ordinary-life surface coherently — the needs model, the sleep/eat/work/continue/wait actions, routines and the no-human day, the new golden fixtures, and the debug commands — so a reader can run and inspect a boring day. This must land atomically after the implementation tickets so the documented symbols/commands/fixtures all exist (Spec 0005 §14.2, §17, §23).

## Assumption Reassessment (2026-06-07)

1. `README.md` at the repo root is the runbook/sample-session surface (confirmed present; the TUI `readme_sample_session.rs` test pins README sample fidelity). Phase 1A/2A documented their TUI commands and fixtures here. This ticket updates the same surface; it does not touch foundation/architecture/execution doctrine (Phase 3A introduces no doctrine change and needs no foundational-doc amendment — Spec carries no amendment).
2. The documented surfaces must exist first: the new actions (tickets 008–011), debug commands (ticket 023), and golden fixtures (tickets 019/021) are the citations. Spec §14.2 fixes the debug command names; §17 fixes the fixture names; §23 fixes the exit posture.
3. Shared boundary under audit: this is a cross-cutting docs ticket — it references symbols/commands/fixtures that several implementation tickets introduce, so it depends on those leaves and must land after them to avoid documenting a non-existent command. The `readme_sample_session.rs` test means any README sample session must stay runnable.
4. No constitutional invariant motivates a behavior change here; the relevant contract is `tickets/README.md`'s no-silent-retcon rule — this ticket adds documentation for new surfaces, changing no existing behavior. (Conditional menu item 4 is therefore not invoked; this is a docs-only ticket.)
5. This ticket touches no fail-closed-validation, actor-knowledge-filtering, or deterministic-replay surface — it is documentation. Verification is grep-proofs against the post-implementation tree plus the existing `readme_sample_session` test; no leakage or determinism surface is affected.

## Architecture Check

1. A single trailing docs ticket (rather than per-ticket README edits) lets the Phase 3A surface be described coherently once all commands/fixtures exist, avoiding a staleness window where the README cites a half-landed command. The README stays the single runbook surface.
2. No backwards-compatibility shims: additive documentation; no behavior change; existing README sample sessions stay runnable.

## Verification Layers

1. Surface fidelity (`tickets/README.md` no-retcon) -> codebase grep-proof: every action/command/fixture the README names resolves to a landed symbol (registry action ID, `debug` command, fixture in `fixtures/mod.rs::all()`).
2. Runnable samples -> replay/golden check: the `readme_sample_session.rs` test still passes against any README sample session.
3. Docs-only boundary -> manual review: Files to Touch are markdown only; no production code is modified.

## What to Change

### 1. README Phase 3A surface

Update `README.md` with a Phase 3A section: the bounded needs (hunger/fatigue/safety, banded), the ordinary actions (`sleep`/`eat`/`work_block`/`continue_routine`/extended `wait`), routines and the no-human day, the new golden fixtures (names from §17.1), and the debug commands (`debug needs`/`routines`/`planner <actor>`/`stuck`/`no-human-day`/`actor <actor>`). Keep any embedded sample session runnable.

## Files to Touch

- `README.md` (modify — Phase 3A ordinary-life surface, actions, fixtures, debug commands)

## Out of Scope

- The `docs/4-specs/SPEC_LEDGER.md` Spec 0005 entry — that is gated on exit evidence and lands in the capstone (ticket 025).
- Any foundation/architecture/execution doc change (Phase 3A introduces no doctrine change).
- Implementation surfaces (tickets 001–023) — this ticket documents them, it does not modify them.

## Acceptance Criteria

### Tests That Must Pass

1. Every action/command/fixture named in the README resolves to a landed symbol (grep-proof against the code tree).
2. `cargo test -p tracewake-tui --test readme_sample_session` passes (README samples runnable).
3. Files to Touch are markdown only (no production code modified).

### Invariants

1. The README documents only landed Phase 3A surfaces; no command/fixture is documented before it exists (no-retcon).
2. Documentation changes no behavior; sample sessions stay runnable.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline coverage is named in Assumption Reassessment.`

### Commands

1. `cargo test -p tracewake-tui --test readme_sample_session`
2. `grep -nE "sleep|eat|work_block|continue_routine|debug needs|no_human_day_001" README.md` (confirm documented surfaces present and resolvable)
3. A docs-only grep + the existing README sample-session test is the correct verification boundary because this ticket adds no production code.

## Outcome

Completed: 2026-06-07

Updated `README.md` with the landed Phase 3A ordinary-life surface: bounded needs, ordinary action families, routine/no-human day behavior, debug commands, and Phase 3A fixture contracts.

Deviation: none.

Verification:

1. `cargo test -p tracewake-tui --test readme_sample_session`
2. `grep -nE "sleep|eat|work_block|continue_routine|debug needs|no_human_day_001" README.md`
3. `rg -n "register_phase3a_sleep|register_phase3a_eat|register_phase3a_work|register_phase3a_continue_routine|\"needs\" => Ok\\(DebugCommand::Needs\\)|\"no-human-day\" => Ok\\(DebugCommand::NoHumanDay\\)|pub use no_human_day_001::no_human_day_001|no_human_day_001\\(\\)" crates README.md`
