# 0039SPICERMUT-009: Kill `replay/report.rs` SPINE survivor with a mandatory match/mismatch witness pair

**Status**: PENDING
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — adds behavior-witness tests in `tracewake-core` (test-only by default; a production correction in `replay/report.rs` lands only if a survivor reveals a real defect, per spec §4.13).
**Deps**: None

## Problem

Wave B left 1 missed mutant in `crates/tracewake-core/src/replay/report.rs` (spec §5.14), owning SPINE-02 (divergence reporting). The historical mutant reverses the equality/mismatch condition in `run_replay`. A test containing only the mismatch case can be satisfied by an implementation that always reports failure, so the two-arm match/mismatch pair is mandatory.

## Assumption Reassessment (2026-06-18)

1. `run_replay` exists at `crates/tracewake-core/src/replay/report.rs:58` and produces a `ReplayReport` (`:32`) whose `matches_expected` field is set from the checksum/equality comparison (`:53`/`:97`/`:138`); existing in-file tests `replay_clean_log_matches_expected` (`:285`) and the corrupt-log negative (`:320`) already touch this surface (verified by grep). The 1 seed-mutant identity is in `reports/0038_spine_cert_mutation_triage_register.md`.
2. Spec §5.14 is the implementation contract; `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` governs replay divergence reporting (verified present).
3. Shared boundary under audit: the `run_replay` divergence-reporting seam that sets `matches_expected`, first divergence, and expected/actual fingerprints.
4. Motivating invariant: `INV-018 — Deterministic replay is foundational` (a corrupt package must fail; there is no repaired-success path). The witness asserts semantic report fields, not rendered wording.
5. This ticket touches a deterministic-replay surface: a known-good package must report `matches_expected` plus checksum/report equality, and a corrupted package must report non-match, first divergence, expected/actual fingerprints, and responsible layer without being repaired. The witnesses only strengthen divergence reporting — no leakage or nondeterminism is introduced. There is **no schema shape change**: this ticket adds tests, not a schema extension (the `replay/report.rs` mutation target and the `event_schema_replay_gates` test home are not schema modifications). This substrate feeds the SPINE-02 re-proof in ticket 021.

## Architecture Check

1. A mandatory two-arm pair (known-good matches; corrupted does not, with first divergence and responsible layer) is the only structure that kills the reversed-condition mutant: a single-arm test passes under an always-fail or always-pass implementation.
2. No backwards-compatibility aliasing/shims: the witness asserts the semantic `ReplayReport` fields produced by `run_replay`, not rendered wording.

## Verification Layers

1. INV-018 divergence reporting -> replay/golden-fixture check: a known-good replay package proves `matches_expected` plus checksum/report equality; a corrupted package proves non-match, first divergence, expected/actual fingerprints, and responsible layer.
2. No repaired-success path -> replay/golden-fixture check: the corrupt package is not repaired or treated as a passing replay.

## What to Change

### 1. Mandatory match/mismatch pair

In `event_schema_replay_gates.rs`, add (or strengthen) a paired witness: one known-good replay package asserting `matches_expected` + checksum/report equality, and one intentionally corrupted package asserting non-match, first divergence, expected/actual fingerprints, and responsible layer — asserting the semantic report fields, not rendered wording, and proving the corrupt package is not treated as a passing replay.

### 2. Member matrix

Map the 1 historical mutant (and any new run survivor in this file) to the mismatch-arm failure under the reversed condition.

## Files to Touch

- `crates/tracewake-core/tests/event_schema_replay_gates.rs` (modify)
- `crates/tracewake-core/src/replay/report.rs` (modify — only if a survivor reveals a real defect; default is test-only)

## Out of Scope

- Replay rebuild and latest-selection predicates (ticket 008).
- Checksum identity/canonicalization (ticket 010).
- The full mutation run (ticket 020); SPINE re-proof (ticket 021).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test event_schema_replay_gates` — passes with the mandatory match/mismatch pair asserting semantic report fields.
2. `cargo mutants --workspace -f crates/tracewake-core/src/replay/report.rs --no-shuffle` — the historical survivor (and any newly enumerated one) is `caught`.

### Invariants

1. Both arms are present; a single-arm test that an always-fail implementation satisfies is rejected.
2. The corrupt package is never repaired or reported as a passing replay.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/event_schema_replay_gates.rs` — paired known-good / corrupted `run_replay` witness asserting `matches_expected`, first divergence, and responsible layer.

### Commands

1. `cargo test --locked -p tracewake-core --test event_schema_replay_gates`
2. `cargo mutants --workspace -f crates/tracewake-core/src/replay/report.rs --no-shuffle`
3. The per-file `-f` run is the correct verification boundary; the full standing campaign is ticket 020.
