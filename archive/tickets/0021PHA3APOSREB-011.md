# 0021PHA3APOSREB-011: Generative tier — two-file fabricator ban, production-parity flushing, tamper-coverage locks

**Status**: DONE
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` test oracle (`tests/generative_lock.rs`, `tests/support/generative.rs`, fabricator-ban guard) and `scheduler` generative-advance entry
**Deps**: `archive/specs/0021_PHASE_3A_POSSESSION_REBIND_HYGIENE_GUARD_VACUITY_CLOSURE_HARNESS_PROVENANCE_FIDELITY_AND_REJECT_LOUDLY_REPLAY_POSTURE_HARDENING_SPEC.md` (ORD-HARD-084, 086, 092)

## Problem

(1) The fabricator ban (`generative_lock_cannot_fabricate_duration_terminals`) scans
only `generative_lock.rs` and bans builder names only — a helper in
`support/generative.rs` (where 0020 added new surface) synthesizing
`EventEnvelope::new_v1(..., EventKind::WorkBlockFailed, ...)`, or direct envelope
construction in the lock file, evades every banned token (`ORD-HARD-084`; R-27
family). (2) The continuity floor's evidence is reachable only through the advance
path's flush divergence: `advance_no_human` runs a bare per-tick loop with a single
`append_due_completions` at the final tick while production `run_no_human_day`
flushes per window — and the displacing Move is sequenced at/after the work block's
scheduled completion, so under production flushing the displacement regime never
occurs; `sleep_affordance_closed`/`workplace_unusable` are accepted continuity
reasons no mask can produce (`ORD-HARD-086`; residual root of `ORD-HARD-060`). (3)
Minor locks (`ORD-HARD-092`): only the first duration terminal per case is tampered
with no per-kind coverage floor; the synthetic-removal `assert_ne!(missing_one,
expected)` is tautological given the preceding equality assert; `as u16` casts in the
Severe-band assertions would wrap a negative constant.

## Assumption Reassessment (2026-06-11)

1. Verified at `main` `89059a5`: the ban's include list covers `generative_lock.rs`
   only; `advance_no_human` (`scheduler.rs`) loops `advance_one_tick` then calls
   `append_due_completions` once at the final tick; the displacement mask sequences
   Work second-to-last and Move last with inter-step gap `rng.range(2,4)` ≥ the
   workplace's `work_duration_ticks` (2), so the Move never strictly precedes the
   scheduled completion; the accepted-reason alternations include
   `sleep_affordance_closed`/`workplace_unusable` (generative_lock.rs) with no
   closing mask in `support/generative.rs`; `duration_terminal_event_index` selects
   via `.position()` (first only); the tautological `assert_ne!`; the `as u16` casts
   at the Severe assertions.
2. Verified against spec 0021 (reassessed 2026-06-11; operator-verified including
   the flush-divergence arithmetic): findings ORD-HARD-084/086/092; the spec
   *prefers* per-window flushing in `advance_no_human` for production parity — this
   ticket bakes the preferred option; §9 requires every floor and contributor
   assertion re-derived honestly after the flush change, never adjusted to pass.
3. Cross-artifact boundary under audit: the generative evidence contract — every
   counted terminal is emitted by the scheduler path under production-equivalent
   flushing semantics, and no test-tree channel can fabricate one.
4. Doctrine restated (docs/2-execution/09 + R-27): a gate's evidence must be
   produced by the path under test; lock durability requires the ban to cover the
   channel (envelope construction), not just today's symbols; INV-017/018 anchor the
   tier's replay/tamper relations.
5. Test-oracle / replay-evidence surface touched: per-window flushing changes the
   generative corpus's event sequences — floors, contributor sets, and the
   in-block displacement case are re-derived from the new emitted logs (honest
   re-derivation, recorded in the acceptance artifact); no production scheduler
   semantics change (`run_no_human_day` untouched; `advance_no_human` is the
   test-tier entry — confirm at implementation it has no production caller, else
   the parity change is itself production-affecting and escalates per spec §9).

## Implementation Outcome (2026-06-11)

1. Changed `advance_no_human` to flush scheduled duration completions as scheduler
   time advances, before each later proposal and then tick-by-tick to the final
   tick. `run_no_human_day` was not changed; current callers of `advance_no_human`
   are tests/generative gates and scheduler tests.
2. Rescheduled the displacement generator so the Move starts strictly inside the
   Work block (`work.start_tick + 1`), and added an oracle assertion that the move
   event tick precedes the scheduled work completion tick for displacement seeds.
3. Re-derived continuity to the corpus-produced reason set. The accepted
   continuity reason is now `actor_displaced`; the unproduced
   `sleep_affordance_closed` and `workplace_unusable` alternatives were removed
   rather than adding a new closure mask.
4. Widened the fabricator ban to scan both `tests/generative_lock.rs` and
   `tests/support/generative.rs`, reject old helper names, and reject direct
   `EventEnvelope::new*` construction paired with duration-terminal event kinds.
   Synthetic support-file and direct-construction violations are pinned.
5. Replaced first-terminal tamper coverage with per-terminal tampering across the
   corpus and asserted coverage for all four terminal kinds:
   `sleep_completed`, `sleep_interrupted`, `work_block_completed`, and
   `work_block_failed`.
6. Replaced the tautological contributor synthetic assertion with an honest
   non-empty predicate plus exact one-contributor removal check. Severe-band
   constants are range-asserted before `u16` conversion.
7. `emergence_ledger` remained green under the flush change; no ledger rows needed
   repricing.

## Verification (2026-06-11)

1. `cargo test -p tracewake-core --test generative_lock -- --nocapture`
2. `cargo test -p tracewake-core --test anti_regression_guards generative -- --nocapture`
3. `cargo test -p tracewake-core --test emergence_ledger`
4. `cargo fmt --all --check`
5. `cargo clippy --workspace --all-targets -- -D warnings`
6. `cargo build --workspace --all-targets --locked`
7. `cargo test --workspace`
8. `git diff --check`

## Architecture Check

1. Giving `advance_no_human` per-window flushing makes the generative tier's
   physics match production — the displacement case then proves a regime production
   can actually reach, with the in-block move (`requested_tick <
   work.requested_tick + work_duration_ticks`) and the ordering assertion making
   the evidence shape explicit. Widening the ban to both test files AND the
   construction channel (`EventEnvelope::new` + terminal `EventKind::` tokens)
   bans the *capability*, not the current symbol list. Per-kind tamper coverage
   (accumulate tampered `stable_id`s, assert all four terminal kinds) locks the
   tier's purpose rather than its first sample.
2. No backwards-compatibility aliasing/shims: the deferred-flush behavior is
   replaced, not kept behind a flag; the tautological assert is replaced by the
   honest non-emptiness form.

## Verification Layers

1. R-27/evidence reachability (ORD-HARD-086) -> in-block displacement ordering
   assertion (move `sim_tick` < scheduled completion tick) on the displacement
   seed; continuity floor re-derived from per-window-flush logs; accepted-reason
   set trimmed to corpus-producible reasons or a closing mask added (recorded
   choice).
2. Lock durability (ORD-HARD-084) -> two-file ban including
   `EventEnvelope::new` + terminal-kind constructor tokens; synthetic violating
   helper in support must fail.
3. Tamper coverage (ORD-HARD-092a) -> per-kind tampered-set assertion covering all
   four terminal kinds across the corpus.
4. Assertion honesty (ORD-HARD-092b/c) -> tautological assert replaced; Severe-band
   constants range-asserted before banding (or banded via `NeedState::band()`).

## What to Change

### 1. Production-parity flushing + in-block displacement (ORD-HARD-086)

Per-window (or per-tick) completion flushing in `advance_no_human`; reschedule the
displacing Move strictly in-block; add the ordering assertion; re-derive all floors
and contributor sets from the new logs; resolve the dead access-closure reasons
(trim, or add a closing mask via the shared open/close action path — recorded
choice).

### 2. Widened fabricator ban (ORD-HARD-084)

Include `support/generative.rs`; ban `EventEnvelope::new` and terminal
`EventKind::` constructor tokens in both test sources; synthetic violation case.

### 3. Minor locks (ORD-HARD-092)

Per-kind tamper coverage set; honest non-emptiness assert; range-asserted Severe
constants.

## Files to Touch

- `crates/tracewake-core/tests/generative_lock.rs` (modify)
- `crates/tracewake-core/tests/support/generative.rs` (modify)
- `crates/tracewake-core/src/scheduler.rs` (modify — `advance_no_human` flush semantics)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — widened ban)
- `crates/tracewake-core/tests/emergence_ledger.rs` (modify — only if the flush change reprices the EMERGE-OBS derivation rows; as surfaced)

## Out of Scope

- Production `run_no_human_day` window semantics (`ORD-HARD-088` — ticket
  0021PHA3APOSREB-012) — coordinate the `scheduler.rs` mechanical merge.
- Mutation perimeter/baseline (`archive/tickets/0021PHA3APOSREB-004.md`).
- New generative masks beyond the access-closure resolution.

## Acceptance Criteria

### Tests That Must Pass

1. Generative suite green under per-window flushing with honestly re-derived
   floors/contributors (re-derivation diff recorded in the acceptance artifact);
   the displacement case asserts in-block ordering.
2. Accepted continuity reasons are exactly the corpus-producible set (or the new
   closing mask produces the closure reasons, floor-asserted) — recorded choice.
3. Widened ban green; synthetic support-file fabricator and direct
   envelope-construction cases each fail.
4. Per-kind tampered-set assertion covers `SleepCompleted`, `SleepInterrupted`,
   `WorkBlockCompleted`, `WorkBlockFailed`; Severe constants range-asserted.
5. `cargo test --workspace` green.

### Invariants

1. Every generative floor counts only events emitted by the scheduler path under
   production-equivalent flushing semantics.
2. No test-tree channel can construct a duration-terminal envelope outside the
   scheduler path.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/generative_lock.rs` — ordering assertion, per-kind
   tamper set, honest asserts, re-derived floors.
2. `crates/tracewake-core/tests/support/generative.rs` — in-block displacement
   scheduling; closing mask if chosen.
3. `crates/tracewake-core/tests/anti_regression_guards.rs` — two-file ban +
   synthetics.

### Commands

1. `cargo test -p tracewake-core --test generative_lock -- --nocapture`
2. `cargo test -p tracewake-core --test anti_regression_guards generative -- --nocapture`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
