# 0020PHA3ACOGSUR-004: Generative-tier fidelity: terminal-targeted tamper, continuity floor, derived seed contributors

**Status**: ✅ COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` test oracle (`tests/generative_lock.rs`, `tests/support/generative.rs`); `scheduler.rs` generative-advance entry only if the per-tick completion flush is chosen for ORD-HARD-060; no kernel behavior change
**Deps**: `archive/specs/0020_PHASE_3A_COGNITION_SURFACE_FRESHNESS_PARITY_DERIVED_CENSUS_CLOSURE_MUTATION_PERIMETER_COMPLETION_AND_GENERATIVE_FIDELITY_HARDENING_SPEC.md` (ORD-HARD-059, ORD-HARD-060, ORD-HARD-061)

## Problem

Three fidelity gaps in the generative lock tier. `ORD-HARD-059`: the per-case tamper
relation (`assert_payload_tamper_poisons_replay`) returns on the FIRST poisoning
field — always discharged by the advance marker's payload, never specifically
perturbing a `SleepCompleted`/`SleepInterrupted`/`WorkBlockCompleted`/
`WorkBlockFailed` payload, the exact surface the tier exists to protect.
`ORD-HARD-060`: `advance_no_human` runs a bare per-tick loop then a single
`append_due_completions` at the final tick, so completion continuity predicates
(`work_completion_failure`, `sleep_interruption_reason`) are evaluated against
final-tick state and the continuity-failure branches are unreachable by the corpus
(every floor is satisfied by need-band branches). `ORD-HARD-061`: the per-family
floors are met with zero margin by hardcoded seed pairs (`{0x…13, 0x…23}` interrupt,
`{0x…11, 0x…21}` wait+work) against `assert_multi_seed_contributors`' bare `>= 2`.

## Assumption Reassessment (2026-06-11)

1. Verified at `main` `96bc387` (operator-verified at the spec's reassessment):
   `assert_payload_tamper_poisons_replay` early-returns on first divergence;
   `advance_no_human` is `for _ in 0..tick_count { advance_one_tick() }` +
   one `append_due_completions`, while production `run_no_human_day` flushes per
   window plus a final flush; `GENERATIVE_SEEDS` (11 seeds) × `mask_for_seed`
   (`seed & 0x0f`: arm 3 → interrupt mask, arm 1 → wait+work) yields exactly two
   contributing seeds per fragile family; `TerminalCounts`/`Reachability` derive
   from `run.log` (advance-emitted only — the 0019 contract, holding).
2. Verified against spec 0020 (reassessed 2026-06-11): findings 059–061; §9 notes
   060's flush-vs-seed decision rule; conformance rows are NOT owed here (the
   0019-corrected generative row stands; no row overstates after these additions).
3. Cross-artifact boundary under audit: the generative oracle contract spanning the
   corpus generator (`support/generative.rs`), the lock assertions
   (`generative_lock.rs`), and the scheduler advance entry — every floor counts only
   advance-emitted events, and every protected payload family is tamper-proven.
4. INV-017/INV-018 restated: randomness seedable/auditable; deterministic replay is
   tested — a replay-integrity tier must demonstrate *detection*, and a floor met by
   construction-coincidence (hardcoded hunger constants sitting in the Severe band)
   is not a lock.
5. Deterministic-replay test-oracle surface touched: all changes strengthen
   detection (terminal-targeted tamper pass, continuity-reason floor,
   predicate-derived contributor counts); no kernel semantics change. If the
   per-tick flush option is chosen for 060, the generative advance entry gains
   production parity (`run_no_human_day`'s per-window flush) — any generated case
   that diverges under the truer path is evidence the lock was hollow, fixed by
   correcting generator seeding, never by weakening the assertion (spec 0019 §9
   precedent, restated by spec 0020).
6. Implementer-recorded choice (spec-assigned, ORD-HARD-060): add a
   displacement/closure seed so continuity branches are reached under current
   single-flush semantics, OR extend the generative advance to flush per tick when
   single-flush makes mid-stream continuity unreachable by construction. Record the
   choice and its rationale in the acceptance artifact (spec §7.5).

## Architecture Check

1. The floors and relations become *derived* rather than coincidental: a
   terminal-targeted tamper pass that fails loudly when the floors claim a terminal
   exists but none is found; a continuity-reason floor counted from advance-emitted
   events; contributor sets derived from a mask predicate with the count asserted
   programmatically (plus band-membership assertions for the hunger constants) — the
   same derivation posture as `-002`'s census, applied to the generative tier.
2. No backwards-compatibility aliasing/shims: if the advance entry changes, it
   changes for the whole generative suite — no second "legacy single-flush" path.

## Verification Layers

1. INV-018 (terminal tamper sensitivity) -> per-case second tamper pass selecting a
   duration-terminal event and asserting each of its payload fields individually
   poisons replay; loud failure when floors claim a terminal and none is found.
2. Continuity-branch reachability -> corpus floor: continuity-reason terminals
   (`actor_displaced`/workplace-unusable class) nonzero across the corpus, counted
   from advance-emitted events.
3. Floor robustness -> predicate-derived contributor counts (interrupt and work-fail
   families) asserted from `mask_for_seed` over `GENERATIVE_SEEDS`, plus explicit
   `NeedBand::for_value` band-membership assertions for the mask hunger constants.
4. Regression safety -> existing 0019 locks stay green: advance-emitted-only
   reachability, fabricator source ban
   (`generative_lock_cannot_fabricate_duration_terminals`), per-family floors.

## What to Change

### 1. Terminal-targeted tamper pass (ORD-HARD-059)

After the existing existential relation, per case: select one duration-terminal
event from the advance-emitted log (fail loudly if floors imply one exists and none
is found) and assert every payload field of it individually poisons replay.

### 2. Continuity-reason floor (ORD-HARD-060)

Per the recorded choice (Assumption item 6): either author a seed/mask whose
sequence changes physical truth between a duration start and its due tick, or give
the generative advance per-tick completion flushing (production parity), then assert
the continuity-reason floor from advance-emitted events.

### 3. Derived contributors (ORD-HARD-061)

Derive the interrupt/work-fail contributing-seed sets from the mask predicate;
assert the derived counts (replacing the bare `>= 2` against hardcoded pairs) and
the hunger constants' Severe-band membership explicitly.

## Files to Touch

- `crates/tracewake-core/tests/generative_lock.rs` (modify)
- `crates/tracewake-core/tests/support/generative.rs` (modify)
- `crates/tracewake-core/src/scheduler.rs` (modify — only if the per-tick flush option is chosen for 060)

## Out of Scope

- Mutation perimeter (`-003`); scheduler window bookkeeping (`-007` — shares
  `scheduler.rs`: coordinate the mechanical merge).
- Any production no-human day-loop change (`run_no_human_day` is the parity
  reference, not a target).
- Conformance-index rows (none owed by this ticket).

## Acceptance Criteria

### Tests That Must Pass

1. Terminal-targeted tamper pass green over the corpus; its loud no-terminal guard
   proven by a synthetic case.
2. Continuity-reason floor nonzero from advance-emitted events; the 060 choice and
   rationale recorded.
3. Predicate-derived contributor counts green; removing a contributing seed fails
   the derived assertion (not silently un-locks).
4. All 0019 generative locks remain green (reachability, fabricator ban, per-family
   floors, multi-seed flags).
5. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`.

### Invariants

1. Every reachability/diversity floor counts only advance-emitted events (the 0019
   contract, extended — INV-017/018).
2. The tier demonstrates detection on the duration-terminal payload family
   specifically, not just existentially.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/generative_lock.rs` — terminal-targeted tamper pass,
   continuity-reason floor, derived contributor assertions, band-membership checks.
2. `crates/tracewake-core/tests/support/generative.rs` — displacement seed/mask or
   per-tick-flush support, per the recorded choice.

### Commands

1. `cargo test -p tracewake-core --test generative_lock`
2. `cargo test -p tracewake-core --test anti_regression_guards generative` (fabricator ban stays green)
3. `cargo test --workspace` (full pipeline)

## Outcome

Completed: 2026-06-11

What changed:

- Added a terminal-targeted tamper pass in `generative_lock.rs`: for every
  generated run that emits a duration terminal, the test selects a terminal event
  and verifies each payload field individually poisons replay.
- Added a synthetic no-terminal guard proving the terminal-targeted tamper helper
  fails loudly when no duration terminal exists.
- Added a continuity-reason floor counted from advance-emitted terminal events:
  `actor_displaced` / access-closure class reasons are distinguished from
  severe-need interruption.
- Added predicate-derived contributor assertions for sleep interruption, work
  failure, and continuity failure based on generated masks over `GENERATIVE_SEEDS`.
- Added explicit Severe-band assertions for the hunger constants that drive the
  sleep-interrupt and work-fail masks.
- Extended the generator with a narrow `wait_work_displace` mask and one generated
  move step through the shared movement pipeline so work completion can fail for
  `actor_displaced`.

ORD-HARD-060 choice:

- Chosen option: author a displacement seed/mask rather than change
  `advance_no_human` to per-tick completion flushing.
- Rationale: current generated scheduled proposals already run through the shared
  pipeline in requested-tick order before due completions are flushed; a generated
  move after a work start changes physical truth before the work completion is
  evaluated, reaching the continuity branch without changing scheduler semantics.

Deviations from original plan:

- No `scheduler.rs` change was needed. The continuity branch is reached by generator
  support only, using the existing production movement action and event application.
- No conformance row was added; the ticket and spec both identify none as owed here.

Verification results:

- `cargo test -p tracewake-core --test generative_lock -- --nocapture` — passed.
- `cargo test -p tracewake-core --test anti_regression_guards generative` — passed.
- `cargo fmt --all --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo test --workspace` — passed.
