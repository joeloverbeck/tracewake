# 0039SPICERMUT-007: Kill `events/envelope.rs` SPINE survivors with envelope-identity + random-draw round-trips

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — adds behavior-witness tests in `tracewake-core` (test-only by default; a production correction in `events/envelope.rs` lands only if a survivor reveals a real defect, per spec §4.13).
**Deps**: None

## Problem

Wave B left 9 missed mutants in `crates/tracewake-core/src/events/envelope.rs` (spec §5.11), owning SPINE-01 (envelope identity), SPINE-02 (replay input), SPINE-04 (random-stream records), and SPINE-05 (persistence). The cluster deletes or corrupts stable mappings for controller/replay-debug streams, deferred/replay schedule phases, controller source, validation-report cause, and random-draw serialization/deserialization. Direct stable-ID assertions are supporting only; without observing envelope identity as consumed by log ordering, replay, validation ancestry, or random-stream audit, a deleted arm can silently map to a default.

## Assumption Reassessment (2026-06-18)

1. `EventStream` enum exists at `crates/tracewake-core/src/events/envelope.rs:68` with `EventKind::stable_id` at `:294`, `from_stable_id` at `:355`, and `EventReplayHandling::for_stream` at `:510`; `SchedulePhase` is imported from `crate::scheduler` (`:6`); `EventEnvelope` at `:717` (verified by grep). The 9 seed-mutant identities (including the random-draw serialize/deserialize survivors) are in `reports/0038_spine_cert_mutation_triage_register.md`.
2. Spec §5.11 is the implementation contract; `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` governs envelope identity, replay inputs, and random streams (verified present).
3. Shared boundary under audit: canonical `EventEnvelope` serialization — the stream/source/schedule-phase/cause stable mappings and the random-draw record — as consumed by event-log fingerprints, stream ordering, replay application, and random-stream audit.
4. Motivating invariants: `INV-017 — Randomness must be seedable and auditable`, `INV-018 — Deterministic replay is foundational`, `INV-020 — Event schema evolution is mandatory`. The required witness is envelope identity as consumed downstream, not a stable-ID getter literal.
5. This ticket touches deterministic-replay and random-stream-discipline surfaces: each affected stream/source/phase/cause variant round-trips through canonical serialization and is observable in log fingerprints / stream ordering / replay / cause provenance; random-draw records use nonzero seed/stream/draw values and survive duplicate replay; malformed/unknown/out-of-range random-draw inputs fail with typed diagnostics; one corrupted envelope's first divergence is recorded by replay. The witnesses only strengthen these properties — no leakage or nondeterminism is introduced. There is **no schema shape change**: this ticket adds tests, not an envelope-schema extension (the `events/envelope.rs` reference is the mutation target, not a schema modification). The random-draw envelope parser/serializer survivors must be killed under either SPINE-04 branch (§6.4), because envelope schema integrity is a SPINE property even when current ordinary fixtures produce no state-affecting draw. This substrate feeds the SPINE-01/02/04/05 re-proof in ticket 021.

## Architecture Check

1. Exhaustively round-tripping every affected stream/phase/source/cause variant and making each field observable in fingerprints/ordering/replay/cause reports proves a deleted arm cannot silently map to a default stream/source/phase/cause — stronger than a stable-ID getter assertion.
2. No backwards-compatibility aliasing/shims: witnesses observe envelope identity through canonical serialization + replay consumption, not a private mapping helper.

## Verification Layers

1. INV-018 envelope-identity determinism -> replay/golden-fixture check: each affected stream/phase/source/cause variant round-trips and is observable in event-log fingerprints, stream ordering, replay, or cause/provenance reports; a corrupted envelope's first divergence is recorded.
2. INV-017 random-stream audit -> replay/golden-fixture check: random-draw records with nonzero seed/stream/draw values preserve identity and outcomes across duplicate replay.
3. Schema fail-loud (INV-020) -> schema validation: malformed/unknown/out-of-range random-draw inputs fail with typed envelope/serialization diagnostics.

## What to Change

### 1. Envelope-identity round-trip matrix

In `event_schema_replay_gates.rs`, round-trip every affected stream, schedule phase, source, and cause variant through canonical envelope serialization, making each field observable in event-log fingerprints, stream ordering, replay application, or cause/provenance reports; prove a deleted arm cannot silently map to a default.

### 2. Random-draw record witnesses + corruption

Exercise random-draw records with nonzero seed/stream/draw values (duplicate replay preserves identity/outcomes); add malformed/unknown/out-of-range random-draw negatives failing with typed diagnostics; include one corrupted envelope whose first divergence is recorded by replay.

### 3. Member matrix

Map all 9 historical mutants (plus any new run survivor in this file) to a concrete downstream-identity or fail-loud case.

## Files to Touch

- `crates/tracewake-core/tests/event_schema_replay_gates.rs` (modify)
- `crates/tracewake-core/src/events/envelope.rs` (modify — only if a survivor reveals a real defect; default is test-only)

## Out of Scope

- Event application/precondition matrix (ticket 006).
- Replay rebuild/report (008/009) and checksum (010).
- The full mutation run (ticket 020); SPINE re-proof (ticket 021).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test event_schema_replay_gates` — passes with the envelope-identity round-trip matrix and random-draw + corruption witnesses.
2. `cargo mutants --workspace -f crates/tracewake-core/src/events/envelope.rs --no-shuffle` — all 9 historical survivors (and any newly enumerated one) are `caught`.

### Invariants

1. A deleted stream/source/phase/cause arm cannot silently map to a default; each is observed through downstream identity consumption.
2. Random-draw envelope records round-trip with nonzero values and fail loudly on malformed/out-of-range input, independent of whether current fixtures produce a state-affecting draw.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/event_schema_replay_gates.rs` — envelope-identity round-trip matrix + random-draw nonzero-value + malformed-input + corrupted-envelope first-divergence rows.

### Commands

1. `cargo test --locked -p tracewake-core --test event_schema_replay_gates`
2. `cargo mutants --workspace -f crates/tracewake-core/src/events/envelope.rs --no-shuffle`
3. The per-file `-f` run is the correct verification boundary; the full standing campaign is ticket 020.

## Outcome

Completed: 2026-06-18

Implemented the `events/envelope.rs` SPINE survivor witnesses as test-only coverage in `crates/tracewake-core/tests/event_schema_replay_gates.rs`:

- Added an envelope identity round-trip matrix that serializes through `EventLog`, deserializes twice, and asserts downstream-consumed identity for controller and replay-debug streams, deferred and replay schedule phases, controller scheduler source IDs, validation-report causes, validation-report IDs, stream positions, and nonzero random draw records.
- Added random-draw corruption rows that replace the canonical `random_draws` field with malformed tuple and bad-inner-hex encodings, proving typed `EventEnvelopeParseError::InvalidTuple` and `EventEnvelopeParseError::BadHex` failures.
- No production correction in `crates/tracewake-core/src/events/envelope.rs` was required. The parser has string-valued random draw refs, so there is no numeric out-of-range branch to exercise without changing schema; the fail-loud coverage matches the current parser's concrete malformed-input diagnostics.

Verification:

- `cargo test --locked -p tracewake-core --test event_schema_replay_gates` — passed.
- `cargo mutants --no-config --workspace -C=--locked -f crates/tracewake-core/src/events/envelope.rs --no-shuffle` — passed with 112 mutants tested, 80 caught, 32 unviable, 0 missed. The `--no-config` deviation was required because ticket 001 made `.cargo/mutants.toml` the standing SPINE perimeter; using the literal ticket command with config would enumerate the full perimeter instead of only `events/envelope.rs`.
- `cargo fmt --all --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo test --workspace --locked` — passed.
