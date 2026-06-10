# 0017PHA3ATICLED-002: Single-charge tick ledger covering action-emitted deltas

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` (`scheduler`, `need_accounting`); `tracewake-content` (golden gate rewrite, one new adversarial fixture); golden checksums reprice
**Deps**: `0017PHA3ATICLED-001` (consumes the shared open-duration authority); `specs/0017_PHASE_3A_TICK_LEDGER_EPISTEMIC_STALENESS_REPLAY_PAYLOAD_EVIDENCE_AND_GENERATIVE_LOCK_HARDENING_SPEC.md` (ORD-HARD-026)

## Problem

Wait-advanced ticks are charged twice as awake need pressure. `actions/defs/wait.rs::build_wait_events` resolves `ticks` with `.unwrap_or(1)` (autonomous waits supply no param) and emits a `NeedDeltaApplied` with `cause_kind="tick_delta"` for the advanced span; `scheduler.rs::run_no_human_day` then sets `last_decision_tick_by_actor` to `window.start_tick` unconditionally, so the next window's `append_passive_need_events_before_decision` re-charges the same ticks as passive awake. Both deltas materialize into `AgentState`. The 0016 lock that should catch this — `golden_fixtures_run.rs::assert_no_duplicate_need_regime_charges` — dedupes by regime *label* (`BTreeSet<&'static str>` insert of `"awake"`), so two awake charges on one tick collapse to a set of size one: the gate asserts the negation of its name. The canonical day contains `ActorWaited`, so causally false arithmetic is encoded in today's golden checksums (INV-039/043/044/045; eventful-but-false per INV-009 — same class as ORD-HARD-014).

## Assumption Reassessment (2026-06-10)

1. Verified against current code at `main` `114e2af`: `wait.rs` `tick_count` is `.unwrap_or(1)` with `0` rejected; the planner's `PlannerGoal::WaitWithReason` plan supplies no `ticks` parameter; `scheduler.rs` inserts `window.start_tick` into `last_decision_tick_by_actor` unconditionally after the passive charge; the gate keys `BTreeMap<(String, String, u64), BTreeSet<&'static str>>` and fails only on `len() > 1`; `no_human_capstone.rs` asserts `EventKind::ActorWaited` is present in the canonical day.
2. Spec 0017 §ORD-HARD-026 requires routing all `tick_delta`-class charges through the single tick-regime accounting authority and rewriting the gate to count occurrences; `docs/2-execution/06_ORDINARY_LIFE_NEEDS_ROUTINES_AND_NO_HUMAN_PROOF.md` gains the corresponding clause via ticket `-010`.
3. Cross-artifact boundary under audit: the `NeedDeltaApplied` payload contract (`cause_kind`, `elapsed_ticks`, `sim_tick`) between core emission sites (wait action, passive appender, completion builders) and the content-side golden reconciliation gate — both sides must agree on per-tick charge attribution.
4. INV-039/INV-045 restated: needs are pressures whose arithmetic must be causally honest; a tick charged twice as "awake time" asserts elapsed time that did not elapse twice. INV-009: the duplicate charge is eventful but causally false.
5. Deterministic-replay surface touched: golden checksums for every no-human run change (the doubled values are currently encoded). The change must keep accounting fully deterministic; each fixture's checksum diff is explained with a per-actor need ledger in the acceptance artifact (ticket `-011`). The decision context-hash gate is unaffected (need values are not context-hash inputs); replay byte-stability is re-established at the corrected values. No epistemic-leakage surface is involved.
6. Adjacent contradictions classified: (a) `clamp_need_value` masking at the 0/1000 bounds is a required consequence to keep in mind when choosing ledger fixtures (mid-range values expose the error; bounds hide it) — covered by the adversarial fixture design here, no separate ticket; (b) the gate's `continue` on non-Sleep/Work `action_effect` causes (eat deltas never reconciled) is corrected by the occurrence-count rewrite in this ticket.

## Architecture Check

1. Correction option chosen (spec offers two): the scheduler consumes committed action-emitted `tick_delta` coverage when computing each actor's passive span — `last_decision_tick_by_actor` advances past ticks already charged by an `ActorWaited`-anchored `NeedDeltaApplied`. This keeps `need_accounting.rs::classify_actor_tick_regimes` the single attribution authority (it gains an awake-by-action interval source built on `-001`'s shared pairing helpers) and avoids subtractive special-casing inside the appender. No event payload shape changes.
2. No backwards-compatibility aliasing/shims: the old unconditional `window.start_tick` insertion is replaced, not kept behind a flag; the label-set gate body is rewritten, not paralleled by a second gate.

## Verification Layers

1. INV-039/045 single-charge arithmetic -> replay/golden-fixture check: rewritten gate counts charge *occurrences* per `(actor, need, tick)` and reconciles the event ledger against `classify_actor_tick_regimes` output for every golden no-human run.
2. INV-009 causal honesty -> new adversarial fixture `wait_then_window_passive_charges_each_tick_once_001` (autonomous wait mid-window crossing a window boundary; mid-range need values so clamping cannot mask).
3. INV-018 deterministic replay -> all golden runs replay byte-identically at the corrected checksums; `cargo test --workspace`.
4. Runtime guard -> release-build `assert!` in the need-accounting apply path: a second charge for an already-attributed `(actor, need, tick)` halts rather than runs wrong (spec §5 tier 2, Tiger-Style).

## What to Change

### 1. Awake-by-action interval source in `need_accounting.rs`

Extend the tick-regime classification with intervals derived from action-emitted `tick_delta` `NeedDeltaApplied` events (anchored to `ActorWaited`), using `-001`'s shared pairing helpers, so every tick's charging regime (asleep / working / awake-passive / awake-by-action) is attributed exactly once.

### 2. Scheduler passive-span computation

`run_no_human_day` computes each actor's passive span from the classifier's uncharged-awake set instead of raw `window.start_tick − last_decision_tick`; `last_decision_tick_by_actor` advances past action-charged ticks.

### 3. Gate rewrite in `golden_fixtures_run.rs`

`assert_no_duplicate_need_regime_charges` accumulates a count per `(actor, need, tick)` (no label dedupe, no `action_effect` skip for eat/wait) and asserts ≤1 charge per tick per need, reconciled against the classifier — a true reconciliation, not a label-set.

### 4. Adversarial fixture + golden reprice

New content fixture `wait_then_window_passive_charges_each_tick_once_001` (registered in `fixtures/mod.rs`); update every golden checksum constant the reprice touches (surfaced by the test run); produce the before/after per-actor need ledger data for ticket `-011`.

### 5. Release-build single-charge assert

In the need-delta application path, `assert!` (not `debug_assert!`) that the charged tick set for an actor/need does not double-attribute.

## Files to Touch

- `crates/tracewake-core/src/need_accounting.rs` (modify)
- `crates/tracewake-core/src/scheduler.rs` (modify)
- `crates/tracewake-core/src/agent/need.rs` (modify — apply-path assert, as surfaced by where deltas materialize)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify)
- `crates/tracewake-content/src/fixtures/wait_then_window_passive_charges_each_tick_once_001.rs` (new)
- `crates/tracewake-content/src/fixtures/mod.rs` (modify)
- Golden checksum constants in `crates/tracewake-core/tests/no_human_capstone.rs`, `crates/tracewake-core/tests/golden_scenarios.rs`, `crates/tracewake-core/tests/acceptance_gates.rs` (modify — implementation-discovered set, as surfaced by the reprice)

## Out of Scope

- The wait action's own emission shape (`wait.rs` payload stays as-is; attribution is fixed scheduler/classifier-side).
- Provenance classes, replay payload materialization, embodied availability, content validation (sibling tickets).
- The acceptance-artifact ledger document itself (ticket `-011` consumes this ticket's data).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-content no_human_need_ledger` — rewritten occurrence-count gate passes on every golden no-human run.
2. `cargo test -p tracewake-content wait_then_window_passive_charges_each_tick_once` — adversarial fixture proves single charge across the wait/window boundary at mid-range values.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. Every elapsed tick for an actor is charged by exactly one regime (asleep / working / awake-passive / awake-by-action); the classifier is the single attribution authority.
2. Golden checksum diffs introduced by this ticket are each explainable by the per-actor ledger delta (doubled awake charges removed), and replay is byte-stable at the corrected values.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/tests/golden_fixtures_run.rs` — occurrence-counting reconciliation gate (rewritten); ledger reconciliation against the classifier.
2. `crates/tracewake-content/src/fixtures/wait_then_window_passive_charges_each_tick_once_001.rs` — adversarial wait/window-boundary fixture.
3. `crates/tracewake-core/src/need_accounting.rs` (unit tests) — awake-by-action interval classification; double-attribution assert fires under a forced duplicate.

### Commands

1. `cargo test -p tracewake-content golden_fixtures_run`
2. `cargo test --workspace`
