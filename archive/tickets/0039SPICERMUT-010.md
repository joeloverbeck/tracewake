# 0039SPICERMUT-010: Kill `checksum.rs` SPINE survivors with checksum-identity + replay-divergence witnesses

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — adds behavior-witness tests in `tracewake-core` (test-only by default; a production correction in `checksum.rs` lands only if a survivor reveals a real defect, per spec §4.13).
**Deps**: None

## Problem

Wave B left 6 missed mutants in `crates/tracewake-core/src/checksum.rs` (spec §5.7), owning SPINE-02 (replay determinism) and SPINE-03 (projection/context identity). The cluster mutates `AgentStateChecksum::as_str`, location canonicalization, and routine-step status identity. Asserting `as_str()` in isolation is a getter tautology; the certifying consequence is replay identity and divergence detection.

## Assumption Reassessment (2026-06-18)

1. `AgentStateChecksum::as_str` exists at `crates/tracewake-core/src/checksum.rs:300`; `location_key(...)` is used in container/item canonicalization at `:223`/`:237`; `from_canonical_lines` at `:142`; `RoutineStepStatus` is imported (`:1`) and feeds routine-execution canonical field families (`:91`/`:121`) (verified by grep). The 6 seed-mutant identities are in `reports/0038_spine_cert_mutation_triage_register.md`.
2. Spec §5.7 is the implementation contract; `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` governs deterministic checksums and replay identity (verified present).
3. Shared boundary under audit: the authoritative-checksum seam (location canonicalization + routine-step status identity) consumed as a persisted/replay fingerprint.
4. Motivating invariants: `INV-018 — Deterministic replay is foundational` and `INV-019 — Snapshots and compaction may not erase live ancestry`. The witness makes the checksum observable as a persisted/replay artifact fingerprint, not an `as_str()` assertion.
5. This ticket touches deterministic-replay surfaces: semantically-distinct states differing only in the affected location/routine status must produce different authoritative checksums; semantically-equivalent states with different insertion order must produce equal canonical checksums; live and replay checksums must match exactly; an intentionally corrupted field must cause `ReplayReport` to name a mismatch/first divergence. The witnesses only strengthen checksum determinism — no leakage or nondeterminism is introduced. There is **no schema shape change**: this ticket adds tests, not a schema extension (the `checksum.rs` mutation target and the `event_schema_replay_gates` test home are not schema modifications). This substrate feeds the SPINE-02/03 re-proof in ticket 021.

## Architecture Check

1. Distinct-vs-equivalent state pairs plus serialize/replay equality and an intentional corruption catch the location/routine-status/`as_str` mutants through replay identity and divergence detection — stronger than asserting an implementation fragment.
2. No backwards-compatibility aliasing/shims: the checksum is observed as a persisted/replay fingerprint consequence, not a direct `as_str()` getter.

## Verification Layers

1. INV-018 checksum identity -> replay/golden-fixture check: semantically distinct states (differing only in the affected location/routine status) yield different checksums; semantically equivalent states with different insertion order yield equal canonical checksums.
2. Replay equality + divergence -> replay/golden-fixture check: serialize and replay representative logs and prove live and replay checksums match; intentionally corrupt an affected field and prove `ReplayReport` names a mismatch/first divergence rather than accepting the package.

## What to Change

### 1. Checksum-identity + equivalence pairs

In `event_schema_replay_gates.rs`, build pairs of semantically distinct states differing only in the affected location or routine status and prove their authoritative checksums differ; build semantically equivalent states with different insertion order and prove canonical checksums remain equal.

### 2. Replay equality + corruption witnesses

Serialize and replay representative logs (live == replay checksum); intentionally corrupt an affected field and prove `ReplayReport` names a mismatch/first divergence; make the checksum string observable as a persisted/replay fingerprint (in `golden_scenarios.rs` where a fixture exercises it), not an `as_str()` getter assertion.

### 3. Member matrix

Map all 6 historical mutants (plus any new run survivor in this file) to a concrete replay-identity/divergence consequence.

## Files to Touch

- `crates/tracewake-core/tests/event_schema_replay_gates.rs` (modify)
- `crates/tracewake-core/tests/golden_scenarios.rs` (modify)
- `crates/tracewake-core/src/checksum.rs` (modify — only if a survivor reveals a real defect; default is test-only)

## Out of Scope

- Replay rebuild/report (008/009).
- Holder-known fact identity in `knowledge_context.rs` (ticket 011).
- The full mutation run (ticket 020); SPINE re-proof (ticket 021).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test event_schema_replay_gates` — passes with checksum distinct/equivalent pairs and the corruption first-divergence witness.
2. `cargo test --locked -p tracewake-core --test golden_scenarios` — passes with the persisted/replay-fingerprint observation.
3. `cargo mutants --workspace -f crates/tracewake-core/src/checksum.rs --no-shuffle` — all 6 historical survivors (and any newly enumerated one) are `caught`.

### Invariants

1. Distinct semantic state changes checksum; insertion-order changes do not; the consequence is replay identity, not an `as_str()` literal.
2. A corrupted affected field is named as a mismatch/first divergence; the package is not accepted.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/event_schema_replay_gates.rs` — checksum distinct/equivalent pairs + serialize/replay equality + corruption first-divergence.
2. `crates/tracewake-core/tests/golden_scenarios.rs` — checksum observed as a persisted/replay fingerprint on a golden fixture.

### Commands

1. `cargo test --locked -p tracewake-core --test event_schema_replay_gates --test golden_scenarios`
2. `cargo mutants --workspace -f crates/tracewake-core/src/checksum.rs --no-shuffle`
3. The per-file `-f` run is the correct verification boundary; the full standing campaign is ticket 020.

## Outcome

Completed: 2026-06-18

- Added checksum identity and divergence witnesses in `crates/tracewake-core/tests/event_schema_replay_gates.rs` covering equivalent physical reconstruction, location-only checksum divergence, routine-step-status-only agent checksum divergence, replay artifact agent checksum strings, and first-divergence reporting for a corrupted expected state.
- Strengthened `crates/tracewake-core/tests/golden_scenarios.rs` so the existing no-human replay fingerprint asserts the persisted agent checksum string as a `twa1-` replay artifact and compares expected/final report fingerprints.
- No production change in `crates/tracewake-core/src/checksum.rs` was needed.
- Because ticket 001 converted `.cargo/mutants.toml` into the standing SPINE perimeter, the per-file acceptance run used `cargo mutants --no-config --workspace -C=--locked -f crates/tracewake-core/src/checksum.rs --no-shuffle` to preserve this ticket's exact target. Result: 25 mutants tested, 16 caught, 9 unviable, 0 missed.
- Verification passed:
  - `cargo test --locked -p tracewake-core --test event_schema_replay_gates --test golden_scenarios`
  - `cargo mutants --no-config --workspace -C=--locked -f crates/tracewake-core/src/checksum.rs --no-shuffle`
  - `cargo fmt --all --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo build --workspace --all-targets --locked`
  - `cargo test --workspace --locked`
