# 0039SPICERMUT-004: Kill `content/serialization.rs` SPINE survivors with typed round-trip + replay witnesses

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adds behavior-witness tests in `tracewake-content` (test-only by default; a production correction in `serialization.rs` lands only if a survivor reveals a real defect, per spec §4.13).
**Deps**: None

## Problem

Wave B left 17 missed mutants in `crates/tracewake-content/src/serialization.rs` (spec §5.3), owning SPINE-01 (event serialization), SPINE-02 (replay inputs), and SPINE-05 (save/schema integrity). The cluster includes `serialize_event_log -> vec![]`, parser-arm deletion for channels, stances, routine families, and institution privacy scope, `parse_optional_tick -> Ok(None)`, and `split_usize -> vec![0]`. Current tests do not round-trip each affected enum variant through serialize → deserialize → replay with a semantic-state comparison, so an empty-log serializer or a deleted parser arm survives.

## Assumption Reassessment (2026-06-18)

1. `serialize_event_log(log: &EventLog) -> Vec<u8>` exists at `crates/tracewake-content/src/serialization.rs:521`; `parse_optional_tick(value: &str) -> Result<Option<SimTick>, SerializationError>` at `:597`; `split_usize(value: &str) -> Result<Vec<usize>, SerializationError>` at `:768` (verified by grep). The 17 seed-mutant identities are in `reports/0038_spine_cert_mutation_triage_register.md`.
2. Spec §5.3 is the implementation contract; `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` and `docs/2-execution/09_GOLDEN_FIXTURES_SCENARIOS_AND_REPLAY_ACCEPTANCE.md` govern canonical serialization and replay acceptance (verified present).
3. Shared boundary under audit: the content serialization seam where typed event-log / fixture values become canonical bytes and back, consumed by save/replay packages.
4. Motivating invariants: `INV-018 — Deterministic replay is foundational`, `INV-019 — Snapshots and compaction may not erase live ancestry`, and `INV-020 — Event schema evolution is mandatory`. A canonical save/log round trip whose replay reconstructs identical state is the strongest witness; direct parser unit tests may supplement but not replace it.
5. This ticket touches a deterministic-replay and serialization surface: an empty event-log serialization must not pass save/replay validation as a valid nonempty fixture log, optional ticks (present and absent, including nonzero ordering-relevant values) must round-trip, and unknown-token/truncated-field/malformed-list inputs must fail loudly with serialization diagnostics. The witnesses only strengthen byte-and-semantic round-trip determinism; they add no schema shape change (test additions, not a schema extension). This substrate feeds the SPINE-01/02/05 re-proof in ticket 021.

## Architecture Check

1. Exhaustive typed round-trip families (serialize a real typed value → deserialize → replay/consume → compare semantic state) catch a deleted parser arm or an always-`None`/`vec![0]` mutation that a bytes-only comparison or a single-variant test would miss; unaffected sibling variants prevent an always-error parser from passing.
2. No backwards-compatibility aliasing/shims: witnesses use the production round trip; direct parser tests are supplementary only.

## Verification Layers

1. INV-018 round-trip determinism -> replay/golden-fixture check: canonical save/log round trip reconstructs the same state, holder-known projection, and manifest-scoped fingerprint.
2. Enum-variant exhaustiveness -> schema validation: each affected channel / stance / routine-family / privacy-scope variant (plus unaffected siblings) round-trips to the correct typed value.
3. Fail-loud negatives -> schema validation: unknown-token, truncated-field, and malformed-list inputs fail with serialization diagnostics; an empty-log serialization cannot pass as a valid nonempty fixture log.

## What to Change

### 1. Typed round-trip families per affected variant

In `golden_fixtures_run.rs`, build round-trip families exercising `touch_or_search` / `absence_marker` / `reading_placeholder_schema_only`; `believes_true` / `believes_false` / `doubts` / `unknown_or_unresolved`; the affected routine-family arms `morning_wake` / `return_home` / `leave_unsafe_place` / `continue_current_intention` / `wait` / `idle_with_reason` plus unaffected siblings; institution privacy scope; present/absent optional ticks including nonzero ordering-relevant values; and empty/singleton/multi-value interruption-point vectors with nonzero members and canonical ordering.

### 2. Negative + empty-log guards

In `forbidden_content.rs`, prove an empty event-log serialization cannot pass save/replay package validation as a valid nonempty fixture log, and add unknown-token / truncated-field / malformed-list negatives that fail loudly with serialization diagnostics.

### 3. Member matrix

Map each of the 17 seed mutants (and any new run survivor in this file) to a concrete round-trip or negative case.

## Files to Touch

- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify)
- `crates/tracewake-content/tests/forbidden_content.rs` (modify)
- `crates/tracewake-content/src/serialization.rs` (modify — only if a survivor reveals a real defect; default is test-only)

## Out of Scope

- Fixture-load scope arms (ticket 002) and schema-conversion bound selection (ticket 003).
- Broad content validation matrix in `validate.rs` (ticket 005).
- The full mutation run (ticket 020); SPINE re-proof (ticket 021).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-content --test golden_fixtures_run` — passes with the per-variant round-trip families and the canonical save/log replay-identity witness.
2. `cargo test --locked -p tracewake-content --test forbidden_content` — passes with the empty-log guard and the fail-loud serialization negatives.
3. `cargo mutants --workspace -f crates/tracewake-content/src/serialization.rs --no-shuffle` — every historical `serialization.rs` survivor (and any newly enumerated one) is `caught`.

### Invariants

1. Each affected enum variant is round-tripped to its correct typed value and an empty-log serializer cannot pass as a valid nonempty fixture log.
2. The certifying witness is a production save/log round trip whose replay reconstructs identical state, not a bytes-only or parser-literal assertion.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/tests/golden_fixtures_run.rs` — typed round-trip families + canonical save/log replay-identity comparison.
2. `crates/tracewake-content/tests/forbidden_content.rs` — empty-log rejection + unknown-token/truncated/malformed-list negatives.

### Commands

1. `cargo test --locked -p tracewake-content --test golden_fixtures_run --test forbidden_content`
2. `cargo mutants --workspace -f crates/tracewake-content/src/serialization.rs --no-shuffle`
3. The per-file `-f` run is the correct verification boundary; the full standing campaign is ticket 020.
