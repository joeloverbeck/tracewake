# 0006PHA3ANEEROU-010: Capstone — full no-human-day behavior and replay-equivalence acceptance

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — acceptance harness only (`no_human_day_001` fixture run, `acceptance_gates.rs`, `golden_scenarios.rs`); `docs/4-specs/SPEC_LEDGER.md` Status flip on landing. No new production logic.
**Deps**: 0006PHA3ANEEROU-007, 0006PHA3ANEEROU-008, 0006PHA3ANEEROU-009

## Problem

Spec §8 acceptance is earned only when the no-human ordinary-life substrate is proven end-to-end against an actual run, not synthetic logs (audit **D-09 / F-09**). The current core no-human acceptance accepts `ActorWaited`; the no-human metrics golden test manually constructs a log; replay tests append synthetic agent events. This capstone exercises the pipeline composed by tickets 001–009: it runs a full deterministic no-human day (§7.1) and proves replay equivalence (§7.8) for physical + agent + epistemic projections from the **real** run, then flips the Spec 0006 ledger Status. It introduces no new production logic.

## Assumption Reassessment (2026-06-07)

1. `no_human_day_001` fixture exists (`crates/tracewake-content/src/fixtures/no_human_day_001.rs`) with a contract expecting `FoodConsumed`/`EatFailed`, `SleepCompleted`, `WorkBlock*`, movement before workplace access, missing-food failure, closed-workplace failure, and no teleportation. Core acceptance (`crates/tracewake-core/tests/acceptance_gates.rs`) and `golden_scenarios.rs` currently accept wait/synthetic behavior. The autonomous loop (0006PHA3ANEEROU-007), debug/TUI surfaces (008), and content validation (009) must be landed for this fixture to run for real.
2. Spec §7.1 (full no-human day behavior) requires `NoHumanDayStarted`/`NoHumanDayCompleted`, multiple actors in stable order, ≥1 live need tick/delta, ≥1 sleep + fatigue change, ≥1 eat/eat-failure + hunger consequence, ≥1 movement before remote work, ≥1 work start/complete/fail, ≥1 decision trace with candidate+method reasons, and no silent stuck actor. §7.8 (replay equivalence) requires a real (not synthetic) no-human-day log to replay to the same physical checksum, agent-state checksum, epistemic projection checksum, no-human metrics summary, and decision-trace/stuck-diagnostic projection, with loud replay failure identifying event position/kind/version. §8 enumerates the 11 acceptance criteria; criterion 11 (status docs) is already landed (see §6 below).
3. Shared boundary under audit: this capstone is the end-to-end acceptance surface over every prior ticket; it adds verification + the ledger Status flip, not production logic. Cross-artifact boundary: the `no_human_day_001` fixture contract ↔ the integrated engine run ↔ replay rebuild ↔ `docs/4-specs/SPEC_LEDGER.md`.
4. Motivating invariants (restated): INV-091 "No-human tests are mandatory", INV-092 "Deterministic replay is tested", INV-098 "Feature acceptance is harsh" (done only when caused, agent-possible, eventful, trace-aware, epistemically bounded, TUI-playable, debug-inspectable, no-human-runnable, replay-safe, LLM-independent, non-scripted, regression-tested).
5. Deterministic-replay surface touched: the capstone proves byte-identical replay of the real no-human-day log across physical+agent+epistemic projections + metrics + trace projection, and asserts loud failure on mismatch. Expected counts are re-enumerated from the fixture at test start, not hardcoded (hardcoded counts go stale). No nondeterministic input enters the run.

## Architecture Check

1. A single acceptance capstone over a real no-human-day run is the correct shape: it cannot pass while the no-human day is wait-only or while live `AgentState` is stale (the exact failures the audit caught), because it asserts ordinary-action ancestry and live-vs-replay checksum equality from the actual run. Re-enumerating counts from the fixture keeps the test robust to fixture growth.
2. No backwards-compatibility aliasing/shims: the synthetic-log acceptance and `ActorWaited`-accepting smoke are removed, not kept beside the real-run assertions. This ticket adds no production logic (capstone constraint).

## Verification Layers

1. INV-091 / §7.1 (no-human day) -> replay/golden-fixture check: `no_human_day_001` run end-to-end emits the full §7.1 ancestry with no silent stuck actor.
2. INV-092 / §7.8 (deterministic replay) -> replay/golden-fixture check: the real no-human-day log replays to identical physical + agent + epistemic checksums + metrics + trace projection; a forced mismatch fails loudly with position/kind/version.
3. INV-098 (harsh acceptance) -> invariants alignment check: the §8 criteria map to the Phase 3 acceptance gates (`docs/2-execution/02_PHASE_LADDER_AND_ACCEPTANCE_GATES.md`, `docs/2-execution/07_PHASE_3_NEEDS_ROUTINES_AND_NO_HUMAN_LIFE.md` minimum gates), with missing-property setup + minimal speech explicitly deferred per spec §11.

## What to Change

### 1. Full no-human-day acceptance over a real run

Replace synthetic/wait-accepting no-human acceptance in `acceptance_gates.rs`/`golden_scenarios.rs` with assertions over an actual `no_human_day_001` run, enumerating expected events/counts from the fixture at test start.

### 2. Replay-equivalence over the real log

Add a replay-equivalence test: capture the live no-human-day log, rebuild, and assert physical + agent + epistemic checksum + metrics + trace-projection equality; assert loud failure on a forced corruption.

### 3. Ledger Status flip (closeout artifact)

On landing, flip the Spec 0006 ledger entry Status in `docs/4-specs/SPEC_LEDGER.md` from "Proposed corrective hardening spec; blocking, not yet landed" to landed, recording the acceptance evidence. (Criterion 11 status-docs wording was already corrected in-session; this is the completion flip.)

## Files to Touch

- `crates/tracewake-content/src/fixtures/no_human_day_001.rs` (modify)
- `crates/tracewake-core/tests/acceptance_gates.rs` (modify)
- `crates/tracewake-core/tests/golden_scenarios.rs` (modify)
- `docs/4-specs/SPEC_LEDGER.md` (modify)

## Out of Scope

- All production logic — implemented by 0006PHA3ANEEROU-001..009; this capstone only exercises and verifies it (does not modify their files).
- Missing-property story setup and minimal social speech — deferred to Phase 3B per spec §3/§11.

## Acceptance Criteria

### Tests That Must Pass

1. `no_human_day_001` run produces the full §7.1 ancestry (start/complete markers, multi-actor stable order, live need delta, sleep+fatigue, eat/eat-failure+hunger, movement before remote work, work start/complete/fail, decision trace with candidate+method reasons, no silent stuck actor).
2. The real no-human-day log replays to identical physical + agent + epistemic checksums + metrics + trace projection; a corrupted log fails loudly with event position/kind/version.
3. The full §8 acceptance set passes: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace --all-targets --locked`, `cargo test --workspace`.

### Invariants

1. Acceptance is proven from an actual integrated run, never a synthetic/hand-constructed log; expected counts are re-enumerated from the fixture.
2. The Spec 0006 ledger Status reflects landed acceptance only after the above pass.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/acceptance_gates.rs` — full no-human-day ancestry assertions over a real run; replay-equivalence (tri-projection checksum + metrics + trace).
2. `crates/tracewake-core/tests/golden_scenarios.rs` — replace synthetic no-human metrics log with the real-run golden scenario.

### Commands

1. `cargo test -p tracewake-core --test acceptance_gates`
2. `cargo test --workspace`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` — full §8 acceptance gate.
