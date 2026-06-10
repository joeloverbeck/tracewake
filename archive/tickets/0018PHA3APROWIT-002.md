# 0018PHA3APROWIT-002: Duration-delta elapsed_ticks and runtime single-charge coverage

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` (`actions/defs/sleep`, `actions/defs/work`, `events/apply`); reconciliation-gate rewrite in `tracewake-content` tests; golden checksum updates
**Deps**: `archive/tickets/0018PHA3APROWIT-001.md` (rides the same golden re-baseline window; spec §8 ordering); `specs/0018_PHASE_3A_PROVENANCE_WITNESS_EPISODE_REPLAY_EVIDENCE_GENERATIVE_REACHABILITY_AND_SEED_EPISTEMICS_HARDENING_SPEC.md` (ORD-HARD-038)

## Problem

The release-build single-charge invariant (`apply.rs::assert_single_tick_delta_charge`, 0017 §5 tier 2) is dead for duration charges: it early-returns when the payload lacks `elapsed_ticks`, and the sleep/work `NeedDeltaApplied` payloads (built by `sleep.rs::need_delta_event` and `work.rs::need_delta_event`) carry `actor_id/need_kind/delta/cause_kind="action_effect"/cause_action_id` but no `elapsed_ticks` — the field exists only on the lifecycle terminal events. The assert's `"action_effect"` match arm is unreachable, so asleep/working ticks never enter `state.need_tick_charges` and a forged same-tick duplicate duration charge cannot trip the runtime assert. Separately, the test-oracle gate `golden_fixtures_run.rs::assert_no_duplicate_need_regime_charges` derives `action_effect` intervals by locating a sibling terminal with matching keys and falls back to `.unwrap_or(0)` — a duration delta with a missing or key-divergent terminal silently contributes an empty interval (INV-045, INV-039, INV-009).

## Assumption Reassessment (2026-06-10)

1. Verified against current code at `main` `a9c62e0`: `assert_single_tick_delta_charge` is a release-build `assert!` writing checksummed `AgentState::need_tick_charges`, with the early-return on missing `elapsed_ticks`; `sleep.rs::need_delta_event` and `work.rs::need_delta_event` payloads omit `elapsed_ticks` while the builders already hold the values (`sleep_ticks`, `elapsed_ticks` params on the lifecycle event builders); the reconciliation gate's `action_effect` branch resolves intervals via sibling-terminal lookup with `unwrap_or(0)`.
2. Spec 0018 ORD-HARD-038 (required correction + structural lock); 0017 §5 tier 2 mandated the runtime invariant cover *all* charge classes — this completes it.
3. Cross-artifact boundary under audit: the tick-charge attribution contract spanning the event payloads (producers), the apply-path runtime assert (kernel enforcement), and the golden reconciliation gate (test oracle) — one interval source, the delta event's own payload.
4. INV-045/INV-039 restated: need accounting must be causally honest — fake or double charges disconnected from world state are forbidden; needs are pressures whose deltas attribute to real elapsed time exactly once.
5. Deterministic-replay surface touched: adding `elapsed_ticks` to duration delta payloads changes event bytes, so golden no-human checksums reprice once (batched behind ticket 001's re-baseline per spec §8); the runtime assert is release-build (`assert!`, Tiger-style) and the change strengthens fail-closed accounting without altering replay semantics. Per-actor need ledgers for every changed fixture go to the acceptance artifact (spec §7 item 3).
6. Schema extension: the `NeedDeltaApplied` payload (sleep/work `action_effect` variants) gains `elapsed_ticks` — additive (a new payload field; no existing field changes meaning). Consumers: `assert_single_tick_delta_charge` (begins recording duration ticks), the reconciliation gate (switches to this field as its single interval source), and golden checksum expectations (reprice). The window-passive `tick_delta` payloads already carry the field; this closes the asymmetry.

## Architecture Check

1. Deriving the charged interval from the delta event's own payload makes the event self-describing — the gate's fragile sibling-terminal join (and its silent `unwrap_or(0)` fallback) is deleted rather than patched, so a missing or key-divergent terminal can no longer mask a double charge. The runtime assert and the test oracle then consume the same single source, eliminating the divergence class.
2. No backwards-compatibility aliasing/shims: no optional-field fallback retained in the gate; the sibling-terminal lookup path is removed, not kept as a fallback.

## Verification Layers

1. INV-045 single charge per tick (runtime) -> new unit test in `apply.rs`: two `action_effect` deltas covering an overlapping tick panic the release-build assert.
2. INV-009 gate detects duration double-attribution -> rewritten `assert_no_duplicate_need_regime_charges`: `action_effect` intervals read from the delta payload; a zero/absent interval on a duration-regime charge fails the gate instead of skipping.
3. INV-018 deterministic replay -> golden fixtures replay byte-identically at the repriced checksums; `sleep_spanning_window_boundary_charges_each_tick_once` and `wait_then_window_passive_charges_each_tick_once` remain green.
4. Payload completeness -> grep-proof: both `need_delta_event` builders emit `elapsed_ticks`; the assert's `action_effect` arm is reachable (unit test exercises it).

## What to Change

### 1. Emit `elapsed_ticks` on duration delta payloads

`sleep.rs::need_delta_event` and `work.rs::need_delta_event` add `PayloadField::new("elapsed_ticks", …)` from the values the call sites already compute.

### 2. Runtime assert covers duration charges

With the field present, `apply.rs::assert_single_tick_delta_charge`'s `action_effect` arm records each covered tick into `need_tick_charges`; add the overlapping-duplicate unit test asserting the panic.

### 3. Single-source reconciliation gate

`golden_fixtures_run.rs::assert_no_duplicate_need_regime_charges`: derive `action_effect` intervals from the delta event's `elapsed_ticks`, drop the sibling-terminal lookup and its `unwrap_or(0)`, and assert a positive interval for every `action_effect` charge resolving to a duration regime.

### 4. Golden updates

Reprice golden checksum expectations changed by the payload addition; record per-actor need ledgers for the acceptance artifact.

## Files to Touch

- `crates/tracewake-core/src/actions/defs/sleep.rs` (modify)
- `crates/tracewake-core/src/actions/defs/work.rs` (modify)
- `crates/tracewake-core/src/events/apply.rs` (modify)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify)

## Out of Scope

- Episode payload materialization into `OrdinaryLifeEpisodeRecord` (ticket `0018PHA3APROWIT-003`).
- Payload schema-version gates (ticket `-004`).
- Any change to the tick-regime classifier (`need_accounting.rs::classify_actor_tick_regimes`) — it remains the attribution authority; only the evidence and enforcement around it change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core single_tick_delta_charge` — overlapping duplicate `action_effect` charge panics; single charges pass.
2. `cargo test -p tracewake-content --test golden_fixtures_run` — rewritten gate green on all fixtures; positive-interval assertion active.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. Every `NeedDeltaApplied` with `cause_kind` in `{tick_delta, action_effect}` carries `elapsed_ticks`, and every covered tick enters `need_tick_charges` exactly once.
2. The reconciliation gate has exactly one interval source — the delta event's own payload — with no sibling-event fallback.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/events/apply.rs` (unit test) — overlapping `action_effect` double-charge panic.
2. `crates/tracewake-content/tests/golden_fixtures_run.rs` — rewritten reconciliation gate + repriced expectations.

### Commands

1. `cargo test -p tracewake-core single_tick_delta_charge`
2. `cargo test --workspace`

## Outcome

Completed: 2026-06-11

What changed:

- Sleep and work `NeedDeltaApplied` events with `cause_kind=action_effect` now carry
  `elapsed_ticks` from the duration interval they charge.
- The release-build single-charge assertion now records duration action-effect ticks
  because those payloads are self-describing.
- Added a runtime assertion test proving overlapping action-effect duration charges panic
  with `duplicate need tick charge`.
- Rewrote the golden reconciliation gate to read action-effect intervals from the
  `NeedDeltaApplied` event's own `elapsed_ticks` payload and to require positive
  intervals for both `tick_delta` and `action_effect` charges.

Deviations:

- No golden checksum or context-hash rebaseline was needed; the golden fixture suite
  stayed green after the payload addition.

Verification:

- `cargo fmt --all --check` — passed.
- `cargo test -p tracewake-core single_tick_delta_charge` — passed.
- `cargo test -p tracewake-content --test golden_fixtures_run` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo test --workspace` — passed.
