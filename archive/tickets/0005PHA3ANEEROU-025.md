# 0005PHA3ANEEROU-025: Capstone — Phase 3A acceptance gates, test matrix, and spec ledger

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — adds the Phase 3A acceptance/property/golden/transcript tests across crates and records the Spec 0005 entry in `docs/4-specs/SPEC_LEDGER.md`. Introduces no new production logic.
**Deps**: 0005PHA3ANEEROU-006, 0005PHA3ANEEROU-019, 0005PHA3ANEEROU-020, 0005PHA3ANEEROU-021, 0005PHA3ANEEROU-022, 0005PHA3ANEEROU-023, 0005PHA3ANEEROU-024

## Problem

Phase 3A is accepted only when all exit criteria pass (Spec 0005 §23, §18): the no-human day runs without a controller and exercises wake/eat/move/work/return/sleep/interruption; no player-conditioned facts; the food-unavailable replan chain; ≥1 physical/access blocker; possession does not reset intention/routine/needs; no-hidden-truth planning; content validation rejects shortcut fields; replay rebuilds Phase 3A state/traces/metrics deterministically; TUI embodied + debug surfaces expose the mechanics; metrics derive from events. This capstone exercises every prior ticket end-to-end, adds the cross-cutting test matrix, and records the ledger entry. It introduces no new production logic.

## Assumption Reassessment (2026-06-07)

1. The acceptance/test harnesses exist: `crates/tracewake-core/tests/acceptance_gates.rs`, `crates/tracewake-core/tests/golden_scenarios.rs`, `crates/tracewake-content/tests/golden_fixtures_run.rs`, `crates/tracewake-tui/tests/tui_acceptance.rs`, `crates/tracewake-tui/tests/transcript_snapshot.rs` are the surfaces this capstone extends (all confirmed). `docs/4-specs/SPEC_LEDGER.md` has no Spec 0005 entry yet (confirmed; its "Next allowed spec" note authorizes opening a later phase if Phase 2A boundaries are preserved and deferred scope is not represented as complete).
2. The gate set is fixed by Spec §23 (20 exit criteria) and the §18 testing matrix (unit/integration/golden/TUI/replay/validation/regression). The canonical fixture comes from ticket 021, the guard fixtures from ticket 020, the action fixtures from ticket 019, the runner/metrics from 017/018, replay from 006, embodied/debug from 022/023, docs from 024. Counts are re-enumerated from fixtures at test start, not hardcoded.
3. Shared boundary under audit: this is the deliverable-doubles-as-capstone case — it ships the §18 property/golden/transcript test matrix and the ledger entry, exercising tickets 001–024 without modifying their production files. `Deps` are the leaf set (006 replay branch; 019/020/021 fixtures; 022 embodied; 023 debug; 024 docs) whose transitive `Deps` cover the full DAG.
4. Invariants motivating this ticket: `INV-098` — "Feature acceptance is harsh" (caused, agent-possible, eventful, trace-aware, epistemically bounded, TUI-playable, debug-inspectable, no-human runnable, replay-safe, LLM-independent, non-scripted, regression-tested) — plus `INV-091` (no-human tests mandatory), `INV-092` (deterministic replay tested), `INV-093` (actor-knowledge leakage is high-severity), `INV-094` (possession parity tested), `INV-095` (TUI/view-model acceptance tests). The capstone is the test bed enforcing all of these for Phase 3A.
5. Actor-knowledge / deterministic-replay surface: the capstone is the enforcing test bed for the no-leak, possession-parity, no-hidden-truth, and determinism invariants. It asserts no embodied view/notebook leaks hidden truth (`INV-093`/`INV-024`), possession does not reset intention/needs (`INV-094`/`INV-006`), the planner used no ground truth (`INV-024`), the no-human day runs controller-free (`INV-091`/`INV-004`), and replay rebuilds need/routine/intention state and metrics byte-identically (`INV-092`/`INV-018`). It adds no production logic and no nondeterminism; it pins expected behavior. The Spec §23.1 note (Phase 3A is a subset of the full Phase 3 gate; `speak minimally` is deferred to 3B) governs the ledger entry's scope marking.

## Architecture Check

1. A single capstone exercising the composed pipeline (plus the distributed §18 property/transcript/golden tests) verifies the phase end-to-end without duplicating logic, and is the correct home for the cross-cutting no-human/possession/no-leak/determinism proofs no single implementation ticket fully exercises. The ledger entry lives here because it is gated on this exit evidence passing.
2. No backwards-compatibility shims: tests and a ledger entry only; no production code changes.

## Verification Layers

1. No-human day (`INV-091`/`INV-004`) -> replay/golden check: `no_human_day_001` runs with no controller, exercises wake/eat/move/work/return/sleep + the food-unavailable interruption + ≥1 physical blocker, and emits no player-conditioned facts.
2. Replay determinism (`INV-092`/`INV-018`) -> replay/hash check: replay rebuilds need/routine/intention state, scheduled completions, routine starts/completions/failures, salient traces, and metrics byte-identically; unsupported Phase 3A kinds reject loudly.
3. No leakage (`INV-093`/`INV-024`) -> leak/transcript test: no embodied view/notebook reveals hidden true food location, true routine-failure cause, or other-actor cognition.
4. Possession parity (`INV-094`/`INV-006`) -> property test: possession during an active routine preserves intention/routine/needs; `continue_routine` resumes them.
5. TUI acceptance (`INV-095`) -> transcript test: a possess→needs→wait→continue→`debug planner`/`debug stuck` session is reachable and deterministic.

## What to Change

### 1. Acceptance + property gate tests

Extend `crates/tracewake-core/tests/acceptance_gates.rs` and `crates/tracewake-core/tests/golden_scenarios.rs` with the §18.3/§18.7 assertions: the §23 no-human-day exit criteria, the no-hidden-truth-planning proof, the no-teleport proof, the possession-does-not-reset proof, repeated-idle/stuck-actor failure, bounded needs, body-exclusive non-overlap, duration-only sleep/work completion, autonomous proposals use the shared pipeline.

### 2. Content validation + golden gate

Extend `crates/tracewake-content/tests/golden_fixtures_run.rs` with the §18.6 validation gate (Phase 3A shortcut/teleport/refill fields rejected; valid fixtures load) and the §18.3 golden assertions over `no_human_day_001` and the guard/action fixtures, re-enumerating expected counts from the fixtures.

### 3. TUI transcript + replay gate

Extend `crates/tracewake-tui/tests/tui_acceptance.rs` and `transcript_snapshot.rs` with the §18.4 possess/wait/continue/debug transcript and the no-leak assertions, plus the no-human-run + replay-determinism assertions over `no_human_day_001`.

### 4. SPEC_LEDGER entry

Add the Spec 0005 entry to `docs/4-specs/SPEC_LEDGER.md` marked **Phase 3A only** (not full Phase 3 — cite Spec §23.1: `speak minimally` deferred to Phase 3B), stating deferred scope (Phase 3B/4), gated on this capstone's exit evidence.

## Files to Touch

- `crates/tracewake-core/tests/acceptance_gates.rs` (modify — Phase 3A exit-criteria + property assertions)
- `crates/tracewake-core/tests/golden_scenarios.rs` (modify — agent-state replay-determinism scenarios)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify — validation gate + golden assertions)
- `crates/tracewake-tui/tests/tui_acceptance.rs` (modify — possess/continue/debug transcript + no-leak)
- `crates/tracewake-tui/tests/transcript_snapshot.rs` (modify — deterministic Phase 3A transcript)
- `docs/4-specs/SPEC_LEDGER.md` (modify — Spec 0005 Phase 3A entry)

## Out of Scope

- Any new production logic (this is acceptance/verification only).
- README/fixture-contract prose (ticket 024).
- Stretch fixtures and Phase 3B/4 scope (Spec §17.2, §20, §24 — explicit non-goals).

## Acceptance Criteria

### Tests That Must Pass

1. `no_human_day_001` runs without a controller, exercises wake/eat/move/work/return/sleep + the food-unavailable replan chain + ≥1 physical/access blocker, emits no player-conditioned facts, and replay rebuilds its state/projections/metrics byte-identically.
2. Possession does not reset intention/routine/needs; no-hidden-truth planning holds; no embodied surface leaks hidden truth; repeated idle/stuck actors produce typed diagnostics and fail the run unless a fixture expects a typed stable wait.
3. The CI gate passes: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace --all-targets --locked`, `cargo test --workspace`; existing Phase 0–2A fixtures and commands still pass.

### Invariants

1. All §23 Phase 3A exit criteria hold; no embodied surface leaks hidden truth; the day is replay-deterministic (`INV-098`, `INV-091`–`INV-095`).
2. The ledger entry marks Phase 3A only and names deferred scope (Phase 3B `speak minimally`, Phase 3B/4 systems), per Spec §23.1.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/acceptance_gates.rs` — Phase 3A exit-criteria + property assertions.
2. `crates/tracewake-content/tests/golden_fixtures_run.rs` — validation gate + `no_human_day_001`/guard/action golden assertions.
3. `crates/tracewake-tui/tests/tui_acceptance.rs` — possess/continue/debug transcript + no-leak.

### Commands

1. `cargo test --workspace`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked`
3. `cargo test -p tracewake-content --test golden_fixtures_run` (the end-to-end Phase 3A no-human-day acceptance boundary)

## Outcome

Completed: 2026-06-07

Added the Phase 3A capstone evidence across core, content, and TUI tests: Phase 3A event-kind coverage, no-human metrics replay byte identity, Phase 3A fixture roster validation, deterministic Phase 3A debug snapshots, and a possess/wait/debug TUI transcript. Recorded Spec 0005 in `docs/4-specs/SPEC_LEDGER.md` as Phase 3A only with full Phase 3 speech and later systems explicitly deferred.

Deviation: the full clippy gate exposed pre-existing Phase 3A helper lints. This ticket includes lint-only production hygiene (`#[allow]` on intentional helper signatures/large error returns and one mechanical `?` rewrite) so the required CI gate passes; no production behavior was added for the capstone.

Verification:

1. `cargo fmt --all --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo build --workspace --all-targets --locked`
4. `cargo test --workspace`
5. `cargo test -p tracewake-content --test golden_fixtures_run`
