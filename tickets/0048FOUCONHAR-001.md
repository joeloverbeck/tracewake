# 0048FOUCONHAR-001: Replay-owned temporal-frontier projection and corruption suite

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — adds a replay-owned temporal projection (`crates/tracewake-core/src/replay/temporal.rs`); extends `ProjectionRebuildReport` and `ReplayReport` with a reconstructed temporal frontier and typed temporal violations; splits the replay-derived final tick from caller-supplied checksum-identity inputs. No new event kinds, content, or fixtures.
**Deps**: None

## Problem

Spec 0048 §4.6: replay does not reconstruct or validate the temporal frontier from `TimeAdvanced`. At `cb3102e`, `TimeAdvanced` is `ApplyOutcome::WorldNoOp` in `crates/tracewake-core/src/events/apply.rs` (correct for *physical* state — the marker changes no physical/agent state) but **no projection consumes its `prior_tick`/`resulting_tick` payload**, so `crates/tracewake-core/src/replay/rebuild.rs` references `TimeAdvanced` nowhere. `ProjectionRebuildReport` carries physical/agent/epistemic/checksum/event-count/stream-position outputs but **no reconstructed temporal frontier and no temporal-divergence report**, and `rebuild_projection` accepts the final `sim_tick` as a caller-supplied `ChecksumContext` field. The named witness `empty_world_step_appends_time_advanced_and_rebuilds_frontier` (in `crates/tracewake-core/tests/world_step_coordinator.rs`) calls `rebuild_projection(&…, &checksum_context(SimTick::new(42), 1), …)` — it *feeds in* tick 42 rather than deriving it from the marker, so the test name and the archived `0047` acceptance claim are stronger than the executable assertion. The frontier is an out-of-band caller input to rebuild/checksum, not a replay projection.

This ticket lands the replay half of §8 closure step 1 ("make temporal authority singular") and is additive: it does not touch the runtime scheduler step (closure steps 2–3, tickets 003/004).

## Assumption Reassessment (2026-06-23)

1. `ProjectionRebuildReport` (`crates/tracewake-core/src/replay/rebuild.rs:18`) and `ReplayReport` (`crates/tracewake-core/src/replay/report.rs:32`) both lack any temporal-frontier or temporal-violation field — confirmed by reading the struct definitions. `rebuild_projection` (`rebuild.rs:51`) takes `context: &ChecksumContext` whose `sim_tick` is caller-supplied. `TimeAdvanced` is `ApplyOutcome::WorldNoOp` at `crates/tracewake-core/src/events/apply.rs:194`, and `grep TimeAdvanced crates/tracewake-core/src/replay/rebuild.rs` returns nothing — replay does not consume the marker. The replay module is declared `pub mod replay;` (`crates/tracewake-core/src/lib.rs:15`) with submodules under `crates/tracewake-core/src/replay/mod.rs`, so a new `replay/temporal.rs` registers there.
2. Spec 0048 §4.6 and §8 (closure step 1) own this work; §3.1 records the cause-bearing, world-stream, replay-significant `TimeAdvanced` *envelope* as a preserved holding — this ticket consumes that envelope; it does not redesign marker emission. The `empty_world_step_appends_time_advanced_and_rebuilds_frontier` test in `crates/tracewake-core/tests/world_step_coordinator.rs` (line 361) must be rewritten to withhold the expected final tick from the system under test (§4.6 evidence-honesty check: "the expected value must be withheld … and used only in the assertion").
3. Cross-artifact boundary under audit: the replay temporal contract spanning `events/apply.rs` (marker emission/application), the `replay/temporal.rs` projection (new), and the rebuild/report surfaces. The marker's `prior_tick`/`resulting_tick`/envelope `sim_tick`/cause-model are the shared schema; this ticket reads them and must not change their producer (the scheduler marker builder), which stays as-is until tickets 003/004.
4. Constitutional invariants motivating this ticket, restated: `INV-018` (deterministic replay is foundational), `INV-092` (deterministic replay is tested), and `INV-112` (time may validate, but holder-known time must plan — event/replay time is the authoritative ordering and the replay clock orders/validates). The reconstructed frontier must be derived *from the log*, never accepted as an external input the system under test cannot prove.
5. Enforcement surface (deterministic-replay): this ticket *is* the replay-determinism surface for temporal facts. The temporal projection must be a pure function of the initial frontier plus the ordered `TimeAdvanced` markers — no wall-clock, no hash-map iteration order, no caller-supplied final tick entering the canonical reconstruction. Each malformed marker chain must fail with a *typed temporal divergence*, not merely a differing checksum (§4.6), so a corrupted log is rejected loudly rather than silently producing a wrong-but-self-consistent frontier. Introduces no epistemic-leakage path (the projection reads only world-stream temporal markers, not actor-known state).
6. Schema extension: `ProjectionRebuildReport` and `ReplayReport` each gain a `reconstructed_final_frontier` (typed `SimTick`) and a `temporal_violations: Vec<TemporalDivergence>` field — **additive** for every existing consumer (new fields). The `rebuild_projection` *signature* change (splitting the replay-derived final temporal context out of the static `ChecksumContext` identity inputs) is **breaking-internal**: its consumers are `crates/tracewake-core/src/replay/report.rs` and the rebuild tests in `crates/tracewake-core/tests/` and `crates/tracewake-core/src/replay/rebuild.rs` — all in-workspace, all updated in this diff (local compile-atomicity). No out-of-crate consumer exists.

## Architecture Check

1. A dedicated replay-owned temporal projection (a pure fold over ordered `TimeAdvanced` markers, starting from an explicit initial frontier) keeps temporal authority on the replay side symmetric with the runtime authority tickets 003/004 establish: rebuild *derives* the frontier and *validates* the chain rather than trusting a caller's tick. Leaving the frontier as a `ChecksumContext` input would keep an out-of-band authority the determinism contract forbids. Splitting static checksum-identity inputs from the replay-derived final temporal context (then computing final checksums from the derived value) removes the "pass the answer in" defect at its root.
2. No backwards-compatibility aliasing/shims: `rebuild_projection`'s old "final tick as checksum fact" parameter is removed, not retained behind a defaulted overload; every in-workspace caller is updated in the same diff.

## Verification Layers

1. `INV-018` deterministic replay -> replay/golden-fixture check: a single-tick and a multi-tick log, supplied only an initial frontier, reconstruct the same final frontier the live run produced (the live value withheld from the projection, asserted only at the end).
2. `INV-112` temporal authority -> codebase grep-proof + unit: `TimeAdvanced` is consumed by `replay/temporal.rs` (grep), and each marker validates `prior_tick == current frontier`, `resulting_tick == prior_tick + 1`, envelope `sim_tick == resulting_tick`, required cause ancestry, and canonical ordering relative to step effects.
3. `INV-092` replay tested -> replay/golden-fixture check (corruption suite): missing / duplicate / prior-result-mismatch / `+2` jump / backward / envelope-payload disagreement / missing-or-wrong cause / ordinary-future-timestamp-without-marker each fail with a distinct typed `TemporalDivergence`, not a bare checksum mismatch.
4. Single-layer note N/A — three distinct invariants map to three distinct proof surfaces above.

## What to Change

### 1. Add the replay-owned temporal projection

New `crates/tracewake-core/src/replay/temporal.rs`: a `TemporalProjection` that starts from an explicit initial `SimTick` frontier and folds the ordered `TimeAdvanced` markers, plus a typed `TemporalDivergence` enum (one variant per failure class in Verification Layer 3). For each marker validate: supported schema/kind; required cause model and ordering ancestry; payload `prior_tick == current frontier`; payload `resulting_tick == prior_tick + 1`; envelope `sim_tick == resulting_tick`; no duplicate/gap/backward/multi-tick jump; marker ordering relative to the step's effects follows the canonical transaction contract. Register the module in `crates/tracewake-core/src/replay/mod.rs`.

### 2. Run the projection during rebuild and surface its outputs

In `crates/tracewake-core/src/replay/rebuild.rs`, run the temporal projection over the same ordered event stream the rebuild loop iterates, and add `reconstructed_final_frontier: SimTick` and `temporal_violations: Vec<TemporalDivergence>` to `ProjectionRebuildReport`. Split the static checksum-identity inputs from the final temporal context: stop accepting the expected final tick as a `ChecksumContext` fact; derive it from the temporal projection, then compute final checksums from the derived value. Surface the same two outputs on `ReplayReport` (`crates/tracewake-core/src/replay/report.rs`).

### 3. Rewrite the empty-tick witness and add the corruption suite

In `crates/tracewake-core/tests/world_step_coordinator.rs`, rewrite `empty_world_step_appends_time_advanced_and_rebuilds_frontier` to withhold tick 42 from the system under test — derive the frontier from the marker and assert it equals 42, rather than feeding 42 into the rebuild context. Add new `crates/tracewake-core/tests/replay_temporal_frontier.rs` covering: single otherwise-empty tick (initial frontier only); multiple chained ticks and advance-until; mid-duration save/rebuild/resume (the §3.1 log-derived-duration holding under restore); and the malformed-chain corruption cases, each asserting a distinct typed `TemporalDivergence`.

## Files to Touch

- `crates/tracewake-core/src/replay/temporal.rs` (new)
- `crates/tracewake-core/src/replay/mod.rs` (modify — declare the temporal module)
- `crates/tracewake-core/src/replay/rebuild.rs` (modify — run projection; add fields; split checksum-identity from derived final tick)
- `crates/tracewake-core/src/replay/report.rs` (modify — surface frontier + temporal violations on `ReplayReport`)
- `crates/tracewake-core/tests/world_step_coordinator.rs` (modify — withhold-the-final-tick rewrite of the empty-tick witness; shared with tickets 003/004/006 — append/edit distinct items)
- `crates/tracewake-core/tests/replay_temporal_frontier.rs` (new — chained-tick, save/resume, and corruption suite)

## Out of Scope

- Any runtime scheduler change — privatizing the frontier, the canonical one-tick step, and caller refactor are tickets 002–004 (§8 closure steps 1-runtime/2/3).
- Changing `TimeAdvanced` emission, envelope, or cause model (§3.1 preserved holding — this ticket consumes it).
- The `ApplyOutcome::WorldNoOp` handling of `TimeAdvanced` for *physical* state stays correct and unchanged (the marker mutates no physical/agent state; temporal authority is a separate projection).
- Any save-package format change beyond surfacing the restoration-frontier needed for resume (the typed restoration constructor is ticket 004, §4.3).

## Acceptance Criteria

### Tests That Must Pass

1. `empty_world_step_appends_time_advanced_and_rebuilds_frontier` reconstructs tick 42 from the marker without 42 being supplied to `rebuild_projection`; asserting the derived frontier equals 42.
2. Each corruption case in `replay_temporal_frontier.rs` (missing / duplicate / prior-result mismatch / `+2` jump / backward / envelope-payload disagreement / missing-or-wrong cause / ordinary-future-timestamp-without-marker) yields its distinct `TemporalDivergence` variant.
3. `cargo test -p tracewake-core` passes with the new fields and rebuilt callers.

### Invariants

1. The reconstructed final frontier is a pure function of the initial frontier plus the ordered `TimeAdvanced` markers — no caller-supplied final tick, no wall-clock, no incidental iteration order enters it (`INV-018`).
2. A malformed temporal chain fails with a typed `TemporalDivergence`, never a silently-different-but-self-consistent frontier (`INV-092`/`INV-112`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/replay_temporal_frontier.rs` (new) — chained-tick reconstruction, mid-duration save/rebuild/resume, and the malformed-chain corruption matrix.
2. `crates/tracewake-core/tests/world_step_coordinator.rs` (modify) — withhold-the-final-tick rewrite of the empty-tick witness.

### Commands

1. `cargo test -p tracewake-core --test replay_temporal_frontier`
2. `cargo test -p tracewake-core --test world_step_coordinator`
3. `cargo test -p tracewake-core` (full-crate, confirms the `rebuild_projection` signature split compiles across every in-workspace caller).
