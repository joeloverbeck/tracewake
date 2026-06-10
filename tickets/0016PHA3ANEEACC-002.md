# 0016PHA3ANEEACC-002: Single-regime tick classifier for all need accounting

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — new `tracewake-core` need-accounting tick classifier; passive/completion delta rewiring; golden checksum repricing; new adversarial fixture; need-ledger invariant test
**Deps**: `archive/tickets/0016PHA3ANEEACC-001.md`

## Problem

ORD-HARD-014 (blocker for scoped `ORD-LIFE-CERT` evidence): passive awake need deltas double-charge ticks spanned by accepted durations. `scheduler.rs::no_human::run_no_human_day` charges every window × actor `passive_awake_need_deltas` over `elapsed_ticks = window.start_tick − last_decision_tick` with no check for an open `SleepStarted`/`WorkBlockStarted` spanning those ticks; `append_due_completions` then independently applies per-tick sleep recovery / hunger-rise and work costs for the same span. A slept tick is charged awake hunger *and* sleep hunger-rise; awake fatigue *and* sleep recovery. `clamp_need_value(0, 1000)` masks the arithmetic at the bounds while mid-range states encode the error into golden checksums. The doubled deltas are eventful but causally false (INV-009, INV-039, INV-043/044/045). This overturns 0015 §3's "do not double-count sleep ticks" verified-holding claim.

The fix: one per-actor elapsed-tick accounting authority — every charge interval `(from, to]` classifies each tick as awake / asleep / working from the open-duration events in the log, and **all** need deltas (passive awake, sleep recovery, sleep hunger-rise, work cost) route through it so each tick is charged by exactly one regime.

## Assumption Reassessment (2026-06-10)

1. Current code verified at baseline `ba84e75`: `append_passive_need_events_before_decision` (`crates/tracewake-core/src/scheduler.rs` ~:1053–1114) applies authored awake rates unconditionally; elapsed-tick computation at ~:352–355; `append_due_completions` runs immediately after (~:370–380); `build_sleep_completion_events` (`actions/defs/sleep.rs`) and `build_work_completion_events` (`actions/defs/work.rs`) independently apply per-tick recovery/cost for the same span; `clamp_need_value` (`agent/need.rs`) clamps to (0, 1000). `passive_awake_need_deltas_are_deterministic_and_non_reducing` (`time.rs`) asserts sign only. The golden no-human run *does* drive `append_due_completions` through window boundaries spanned by open durations (`no_human_day_001` holds a sleep spanning ticks 24–32 and a work block spanning 10–18) but asserts only byte-stability — the double-charge is already encoded in today's golden checksums.
2. Spec/docs: spec 0016 §ORD-HARD-014 (evidence, required correction, structural lock), §7 item 1 (per-actor need ledgers explain every golden diff), §9 ("ORD-HARD-014 reprices the whole day"); `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` INV-009, INV-039, INV-043/044/045.
3. Shared boundary under audit: the tick-regime classification consumed by four delta paths — passive awake, sleep recovery/hunger-rise, work cost, and completion/interruption proration (`sleep_interruption_reason` / `work_completion_failure` prorated paths from ORD-HARD-011). The classifier derives regimes from open-duration events using `is_duration_terminal` from `archive/tickets/0016PHA3ANEEACC-001.md` (spec §5 item 1: the predicate is the single open/closed authority) — no second classification scheme.
4. INV-039 — needs are pressures, not puppet strings: the arithmetic feeding pressure must be causally honest. INV-043/044/045 — need changes must reflect real regimes, no fake accounting. INV-009 — event ancestry must not assert "awake time" for ticks the actor verifiably spent asleep. Restated before trusting the ticket narrative.
5. Deterministic-replay surface: every golden no-human checksum changes. Determinism is preserved — the classifier reads only ordered log events and authored rates (no wall-clock, no map-iteration order); replay stays byte-identical against the new goldens. Each fixture checksum diff must be explained with a per-actor need ledger, recorded for the acceptance artifact (ticket 0016PHA3ANEEACC-014). The need-ledger invariant test is the runtime enforcement surface: no tick charged by two regimes.
6. Adjacent contradictions: 0015's overturned "do not double-count" verified-holding claim must be recorded in the conformance index — that recording is ticket 0016PHA3ANEEACC-013's surface (spec §6), a required consequence routed to its own ticket, not silent history rewriting.

## Architecture Check

1. A single accounting authority is the only structure that makes the four delta paths provably consistent: per-path conditional checks (e.g. "skip passive if sleeping") would re-create the drift this spec series keeps finding. Routing proration through the same classifier keeps ORD-HARD-011's interrupted/displaced paths consistent by construction. Golden repricing is unavoidable — the current checksums encode wrong values; the per-actor ledger makes the diff reviewable instead of opaque.
2. No backwards-compatibility aliasing/shims: the unconditional passive-delta path is replaced by the classifier-routed path; no fallback branch retains the old arithmetic.

## Verification Layers

1. INV-043/044/045 (single-regime charging) → need-ledger invariant test: reconstruct per-actor per-tick charge attribution from the event log for every no-human golden run; assert no tick is charged by two regimes (`no_human_day_001` + the window-spanning fixture).
2. INV-009 (causally honest events) → runtime test: a mid-sleep window's passive event payload `elapsed_ticks` excludes slept ticks.
3. INV-018 (byte-identical replay) → replay/golden-fixture check: repriced goldens replay byte-identically; every checksum diff carries a ledger explanation.
4. Source-guard positive presence → codebase grep-proof (`anti_regression_guards.rs`): the passive-delta path references the tick classifier, with the runtime test as backstop (spec §ORD-HARD-014 structural lock).

## What to Change

### 1. Need-accounting tick classifier (new module)

`crates/tracewake-core/src/need_accounting.rs`: given an actor, a charge interval `(from, to]`, and the event log, classify every tick as awake / asleep / working from open-duration events (`SleepStarted`/`WorkBlockStarted` paired with terminals via `is_duration_terminal`). Deterministic, log-derived, no raw-state shortcuts. Register in `lib.rs`.

### 2. Route all delta paths through the classifier

- `append_passive_need_events_before_decision`: passive awake deltas apply only to awake-classified ticks; the emitted payload's `elapsed_ticks` reflects the awake count.
- `build_sleep_completion_events` / `build_work_completion_events`: per-tick recovery/cost spans come from the classifier's asleep/working classifications.
- Completion/interruption proration (`sleep_interruption_reason`, `work_completion_failure` paths) consumes the same classifier.

### 3. Need-ledger invariant test

For every no-human golden run, reconstruct per-actor per-tick charge attribution from the event log and assert single-regime charging. Runs over `no_human_day_001` and the new fixture.

### 4. Adversarial fixture `sleep_spanning_window_boundary_charges_each_tick_once_001`

A sleep spanning a window boundary; asserts each spanned tick is charged exactly once (sleep regime), with byte-identical replay.

### 5. Source guard + golden repricing

Extend `anti_regression_guards.rs` (guard_016 family) with the positive-presence check that the passive path references the classifier. Update all golden checksums; produce the per-actor before/after ledger for each diff (input to ticket 014's acceptance artifact).

## Files to Touch

- `crates/tracewake-core/src/need_accounting.rs` (new)
- `crates/tracewake-core/src/lib.rs` (modify — module registration)
- `crates/tracewake-core/src/scheduler.rs` (modify)
- `crates/tracewake-core/src/time.rs` (modify)
- `crates/tracewake-core/src/actions/defs/sleep.rs` (modify)
- `crates/tracewake-core/src/actions/defs/work.rs` (modify)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — source guard)
- `crates/tracewake-content/src/fixtures/sleep_spanning_window_boundary_charges_each_tick_once_001.rs` (new)
- `crates/tracewake-content/src/fixtures/mod.rs` (modify)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify — ledger invariant test)
- Golden fixtures under `crates/tracewake-content/src/fixtures/` (modify — checksum repricing as surfaced, each diff ledger-explained)

## Out of Scope

- The duration-terminal predicate itself (`archive/tickets/0016PHA3ANEEACC-001.md` — consumed here).
- Skipping decision generation during open durations (0016PHA3ANEEACC-010 — changes goldens again, sequenced after this ticket).
- The context-hash replay gate (0016PHA3ANEEACC-003).
- The acceptance-artifact report assembly (0016PHA3ANEEACC-014 — this ticket produces the ledger data).

## Acceptance Criteria

### Tests That Must Pass

1. Need-ledger invariant test: zero ticks charged by two regimes across `no_human_day_001` and `sleep_spanning_window_boundary_charges_each_tick_once_001`.
2. Mid-sleep-window runtime test: passive event payload `elapsed_ticks` excludes slept ticks.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` — with all repriced goldens green and ledger-explained.

### Invariants

1. Exactly one tick-regime classification authority exists; all four delta paths (passive, sleep, work, proration) consume it; the classifier consumes `is_duration_terminal` (no second open/closed scheme).
2. Need accounting derives only from ordered log events and authored rates — deterministic and replay-derivable.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/tests/golden_fixtures_run.rs` — the need-ledger single-charge invariant test over all no-human golden runs.
2. `crates/tracewake-content/src/fixtures/sleep_spanning_window_boundary_charges_each_tick_once_001.rs` — the ORD-HARD-014 structural-lock fixture.
3. `crates/tracewake-core/src/need_accounting.rs` — unit tests: regime classification across window boundaries, interval edges, and interleaved durations.
4. `crates/tracewake-core/tests/anti_regression_guards.rs` — source guard: passive-delta path references the classifier.

### Commands

1. `cargo test -p tracewake-core need_accounting && cargo test -p tracewake-content golden`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
