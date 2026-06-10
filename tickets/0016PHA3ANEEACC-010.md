# 0016PHA3ANEEACC-010: Cross-tick stuck detection and wait discipline

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` cross-window stuck detectors, wait-reason rejection, authoritative threshold recomputation, open-duration decision skip, typed payload errors; two new fixtures; capstone extension; golden repricing
**Deps**: `archive/tickets/0016PHA3ANEEACC-001.md`, 0016PHA3ANEEACC-002

## Problem

ORD-HARD-023: stuck-actor detection lacks its cross-tick categories and wait discipline has gaps (INV-040/041; execution doc 06's stuck-diagnostic clauses; archived 0005 §8.8 mandated diagnostics for "no progress past expected progress window" and "repeats idle/wait without typed reason"; §9.10 made autonomous wait reasons mandatory):

- All current stuck diagnostics are single-window fail-closed cases. `RoutineExecution` carries `last_progress_tick`, `expected_next_progress_tick`, `fallback_attempts`, but no code consumes them — the no-progress and repeated-idle detectors do not exist.
- `wait.rs::build_wait_events` fills a missing reason with `unwrap_or("unspecified_wait")` — the action layer fabricates the reason doctrine requires the actor to supply.
- `build_threshold_events` reads `current_hunger`/`current_fatigue` from proposal parameters; absent or malformed params silently skip the crossing event, and the parameters are forgeable in principle.
- `run_no_human_day` generates a decision proposal every window even while a body-exclusive duration is open; the rejection churn pollutes progress accounting.
- `payload_i32` panics on absent/malformed payload fields in completion builders — a latent crash where a typed application failure is required.

## Assumption Reassessment (2026-06-10)

1. Current code verified at baseline `ba84e75`: `RoutineExecution` fields at `agent/routine.rs:351–367`, consumed only by `checksum.rs`, `debug_reports.rs`, and tests; `build_wait_events` fabricates the reason (`actions/defs/wait.rs:78–82`, `unwrap_or_else(|| "unspecified_wait")`); `build_threshold_events` silently skips on missing params (`wait.rs:188–219`, `proposal.parameters.get(parameter)?` in a `filter_map`); `run_no_human_day` proposes every actor × window with no open-duration skip (`scheduler.rs` ~:388); `payload_i32` panics in `sleep.rs:239–244` (`panic!`) and `work.rs:509–513` (`expect`). Work validation already recomputes needs from authoritative `AgentState` (`work.rs:64/:84`) — the discipline this ticket generalizes. Stuck-diagnostic records live across `state.rs` (`AgentState`, :140), `events/envelope.rs`, `checksum.rs`, `debug_reports.rs`.
2. Spec/docs: spec 0016 §ORD-HARD-023 (evidence, required correction, structural lock); `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` stuck clauses; `archive/specs/0005_*` §8.8 (:496) and §9.10 (:689); `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` INV-040, INV-041.
3. Shared boundary under audit: the typed stuck-diagnostic vocabulary spanning scheduler, routine lifecycle, and the wait/duration action layer. The open-duration window-skip consumes `is_duration_terminal` (`archive/tickets/0016PHA3ANEEACC-001.md`) — no third open/closed classification (spec §5 item 1).
4. INV-040 — agents are bounded but competent: an actor silently spinning on waits is neither. INV-041 — decisions need debug traces; stuck states need typed, inspectable diagnostics. Restated before trusting the ticket narrative.
5. Deterministic-replay / fail-closed surfaces: (a) the window-skip removes per-window decision churn for actors inside open durations — golden checksums reprice (diffs ledger-explained, building on 0016PHA3ANEEACC-002's accounting); (b) threshold crossings recompute from authoritative `AgentState`, closing the forgeable-parameter channel (proposal-supplied need params are ignored, as work validation already does); (c) `payload_i32` panics become typed event-application errors, so adversarial/corrupt logs produce loud typed failures instead of crashes (feeding 0016PHA3ANEEACC-011's poisoned-rebuild contract).
6. Schema extension: the stuck-diagnostic vocabulary gains the two canonical cross-tick categories (no-progress-past-expected-window, repeated-idle) as typed variants. Consumers: diagnostic records in `AgentState`/checksum, debug reports, the capstone's metrics. Additive-only — no existing diagnostic changes shape.
7. Adjacent contradictions: execution doc 06 does not yet list the cross-tick categories explicitly — the execution-tier clarification is ticket 0016PHA3ANEEACC-013's surface (spec §6), a required consequence routed to the docs ticket.

## Architecture Check

1. Consuming the already-persisted `RoutineExecution` progress fields is the minimal honest design: the state was built for exactly this and is already checksummed; the detectors read it across windows instead of introducing a parallel progress store. Rejecting reasonless waits at the boundary (instead of fabricating `unspecified_wait`) keeps INV-105's typed-diagnostics doctrine — a reason the actor never supplied is a display string posing as cognition. Authoritative recomputation for thresholds mirrors the work-validation precedent already in the codebase.
2. No backwards-compatibility aliasing/shims: `unspecified_wait` fabrication is removed, not defaulted differently; `payload_i32`'s panic path is replaced by typed errors, not wrapped in a catch.

## Verification Layers

1. INV-040/exec-06 (no-progress detection) → fixtures `repeated_wait_without_progress_emits_stuck_001` and `stalled_intention_past_expected_progress_emits_stuck_001`: canonical stuck diagnostics emitted, replay byte-match.
2. §9.10 (mandatory wait reasons) → rejection test: a reasonless Scheduler-origin wait proposal is rejected with a typed reason code; capstone extension asserts the canonical day contains zero `unspecified_wait` reasons.
3. Forgeable-params closure → forged-need-param test: threshold crossings derive from `AgentState`, not proposal parameters.
4. INV-018 (replay) → replay/golden-fixture check: repriced goldens replay byte-identically; typed payload errors surface as loud application failures under corrupted-log tests (verified jointly with ticket 011's gates).

## What to Change

### 1. Cross-window detectors

A no-progress detector consuming `last_progress_tick`/`expected_next_progress_tick` plus a repeated-idle counter (`fallback_attempts`-backed), emitting the canonical stuck-diagnostic categories as typed records.

### 2. Wait-reason discipline

`build_wait_events` rejects reasonless autonomous waits with a typed reason-missing code; no fabricated default.

### 3. Authoritative threshold recomputation

`build_threshold_events` recomputes hunger/fatigue from `AgentState` inside the wait/duration paths; proposal-supplied need params are ignored.

### 4. Open-duration window skip

`run_no_human_day` skips decision generation for actors with an open body-exclusive duration spanning the window, classified via `is_duration_terminal`.

### 5. Typed payload errors

`payload_i32` (sleep.rs, work.rs) returns typed event-application errors on absent/malformed fields.

### 6. Locks

The two fixtures; the reasonless-wait rejection test; the forged-param test; the capstone zero-`unspecified_wait` assertion.

## Files to Touch

- `crates/tracewake-core/src/scheduler.rs` (modify)
- `crates/tracewake-core/src/agent/routine.rs` (modify)
- `crates/tracewake-core/src/actions/defs/wait.rs` (modify)
- `crates/tracewake-core/src/actions/defs/sleep.rs` (modify)
- `crates/tracewake-core/src/actions/defs/work.rs` (modify)
- `crates/tracewake-core/src/state.rs` (modify — typed diagnostic variants, as surfaced)
- `crates/tracewake-core/src/events/envelope.rs` (modify — diagnostic/event vocabulary, as surfaced)
- `crates/tracewake-core/tests/no_human_capstone.rs` (modify — zero-`unspecified_wait` assertion)
- `crates/tracewake-content/src/fixtures/repeated_wait_without_progress_emits_stuck_001.rs` (new)
- `crates/tracewake-content/src/fixtures/stalled_intention_past_expected_progress_emits_stuck_001.rs` (new)
- `crates/tracewake-content/src/fixtures/mod.rs` (modify)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify — run fixtures)
- Golden fixtures under `crates/tracewake-content/src/fixtures/` (modify — checksum repricing as surfaced, ledger-explained)

## Out of Scope

- The tick-regime accounting itself (0016PHA3ANEEACC-002) and the terminal predicate (`archive/tickets/0016PHA3ANEEACC-001.md` — both consumed here).
- Replay ordering/poisoning gates (0016PHA3ANEEACC-011 — consumes this ticket's typed payload errors).
- The exec-doc-06 clause-list clarification (0016PHA3ANEEACC-013).

## Acceptance Criteria

### Tests That Must Pass

1. Both stuck fixtures emit their canonical typed diagnostics and replay byte-identically.
2. Reasonless Scheduler-origin wait is rejected; forged need params are ignored (authoritative recomputation proven); capstone day contains zero `unspecified_wait`.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`.

### Invariants

1. Every autonomous wait carries an actor-supplied typed reason; every cross-window stall is detected and typed — no silent idling (INV-040/041).
2. Need-threshold crossings derive only from authoritative `AgentState`; completion builders never panic on payloads — typed failures only.

## Test Plan

### New/Modified Tests

1. The two stuck fixtures — ORD-HARD-023 structural locks.
2. `crates/tracewake-core/src/actions/defs/wait.rs` — reasonless-wait rejection + forged-need-param tests.
3. `crates/tracewake-core/tests/no_human_capstone.rs` — zero-`unspecified_wait` extension.

### Commands

1. `cargo test -p tracewake-core wait && cargo test -p tracewake-core --test no_human_capstone`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
