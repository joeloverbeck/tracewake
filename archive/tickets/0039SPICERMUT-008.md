# 0039SPICERMUT-008: Kill `replay/rebuild.rs` SPINE survivors with deterministic-rebuild + metamorphic witnesses

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — adds behavior-witness tests in `tracewake-core` (test-only by default; a production correction in `replay/rebuild.rs` lands only if a survivor reveals a real defect, per spec §4.13).
**Deps**: None

## Problem

Wave B left 7 missed mutants in `crates/tracewake-core/src/replay/rebuild.rs` (spec §5.13), owning SPINE-02 (deterministic rebuild) and SPINE-03 (reconstructed context/projection). The cluster replaces decision-context hash output, removes event-cause handling, and alters latest-perception / latest-need selection predicates. A helper returning `Ok(())` survives unless the rebuilt package itself lacks the required semantic evidence; asserting a private helper value directly is insufficient.

## Assumption Reassessment (2026-06-18)

1. `rebuild_decision_context_hashes` exists at `crates/tracewake-core/src/replay/rebuild.rs:207`, `rebuild_decision_context_hash` at `:223`, `latest_current_place_perception_event_id` at `:392`, `latest_need_event_id` at `:410`; cause handling matches on `EventCause` (`:240`); `decision_context_hash_failures` is reported in the replay output (`:36`/`:173`) (verified by grep). The 7 seed-mutant identities are in `reports/0038_spine_cert_mutation_triage_register.md`.
2. Spec §5.13 is the implementation contract; `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` and `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` govern deterministic rebuild (verified present).
3. Shared boundary under audit: the replay-rebuild seam reconstructing authoritative state, projections, and decision-context hashes/frontiers from persisted event history.
4. Motivating invariants: `INV-018 — Deterministic replay is foundational` and `INV-092 — Deterministic replay is tested`. The consequence is replay identity and first-divergence detection, not an asserted private helper value.
5. This ticket touches deterministic-replay surfaces: rebuilding the same package twice must yield identical state/projections/context-hashes/frontiers/report fields; corrupting or removing a cause must fail with first-divergence/causal diagnostics rather than continuing; an unrelated event must not change latest-selection results while a later relevant event does. The witnesses only strengthen replay determinism — no leakage or nondeterminism is introduced. There is **no schema shape change**: this ticket adds tests, not a schema extension (the `replay/rebuild.rs` mutation target and the `event_schema_replay_gates` test home are not schema modifications). This substrate feeds the SPINE-02/03 re-proof in ticket 021.

## Architecture Check

1. Double-rebuild equality, cause-corruption first-divergence, and metamorphic latest-selection relations make each of the 7 mutants observable through the rebuilt package's semantic evidence — stronger than asserting a private helper's return value.
2. No backwards-compatibility aliasing/shims: witnesses observe the rebuilt package output, not the private rebuild helper directly.

## Verification Layers

1. INV-018/092 deterministic rebuild -> replay/golden-fixture check: rebuild the same package twice and compare authoritative state, projections, context hashes/frontiers, and replay-report fields.
2. Causal fail-loud -> replay/golden-fixture check: corrupt or remove a cause and prove rebuild fails with first-divergence/causal diagnostics rather than continuing; empty/constant context-hash mutants cause a real-fixture mismatch.
3. Latest-selection correctness -> replay/golden-fixture check (metamorphic): competing perception/need events across actors/kinds/channels/ticks select the intended latest event; an unrelated added event does not change the result while a later relevant event does.

## What to Change

### 1. Deterministic-rebuild + cause-corruption witnesses

In `event_schema_replay_gates.rs`, rebuild the same event package twice and compare authoritative state, projections, context hashes/frontiers, and replay-report fields; corrupt/remove a cause and prove rebuild fails with first-divergence/causal diagnostics; ensure empty/constant context-hash mutants produce a real-fixture mismatch.

### 2. Latest-selection metamorphic rows

Construct competing perception and need events across actors/kinds/channels/ticks proving the intended latest event is selected; add an unrelated event metamorphically and prove it does not change latest-selection while a later relevant event does.

### 3. Member matrix

Map all 7 historical mutants (plus any new run survivor in this file) to a concrete rebuilt-package consequence.

## Files to Touch

- `crates/tracewake-core/tests/event_schema_replay_gates.rs` (modify)
- `crates/tracewake-core/tests/golden_scenarios.rs` (modify)
- `crates/tracewake-core/src/replay/rebuild.rs` (modify — only if a survivor reveals a real defect; default is test-only)

## Out of Scope

- Divergence-report equality condition in `replay/report.rs` (ticket 009).
- Checksum identity/canonicalization (ticket 010).
- The full mutation run (ticket 020); SPINE re-proof (ticket 021).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test event_schema_replay_gates` — passes with double-rebuild equality, cause-corruption first-divergence, and metamorphic latest-selection rows.
2. `cargo test --locked -p tracewake-core --test golden_scenarios` — passes with the rebuilt-package state/projection consequences.
3. `cargo mutants --workspace -f crates/tracewake-core/src/replay/rebuild.rs --no-shuffle` — all 7 historical survivors (and any newly enumerated one) are `caught`.

### Invariants

1. Each mutant is caught because the rebuilt package lacks the required semantic evidence, not because a private helper value is asserted directly.
2. A removed/corrupted cause produces a first-divergence/causal diagnostic; rebuild never silently continues.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/event_schema_replay_gates.rs` — double-rebuild equality + cause-corruption first-divergence + metamorphic latest-perception/need selection.
2. `crates/tracewake-core/tests/golden_scenarios.rs` — rebuilt-package authoritative-state/projection consequence on a golden fixture.

### Commands

1. `cargo test --locked -p tracewake-core --test event_schema_replay_gates --test golden_scenarios`
2. `cargo mutants --workspace -f crates/tracewake-core/src/replay/rebuild.rs --no-shuffle`
3. The per-file `-f` run is the correct verification boundary; the full standing campaign is ticket 020.

## Outcome

Completed: 2026-06-18

- Added a test-only deterministic rebuild witness in `crates/tracewake-core/tests/event_schema_replay_gates.rs` covering double-rebuild equality, `ordinary_event_id` fallback through `EventCause::Event`, fail-loud missing-cause diagnostics, and latest perception/need metamorphic selection.
- The new witness catches the historical `replay/rebuild.rs` survivors without asserting the private helper directly. No production change in `crates/tracewake-core/src/replay/rebuild.rs` was needed.
- Because ticket 001 converted `.cargo/mutants.toml` into the standing SPINE perimeter, the per-file acceptance run used `cargo mutants --no-config --workspace -C=--locked -f crates/tracewake-core/src/replay/rebuild.rs --no-shuffle` to preserve this ticket's exact target. Result: 58 mutants tested, 50 caught, 8 unviable, 0 missed.
- Verification passed:
  - `cargo test --locked -p tracewake-core --test event_schema_replay_gates --test golden_scenarios`
  - `cargo mutants --no-config --workspace -C=--locked -f crates/tracewake-core/src/replay/rebuild.rs --no-shuffle`
  - `cargo fmt --all --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo build --workspace --all-targets --locked`
  - `cargo test --workspace --locked`
