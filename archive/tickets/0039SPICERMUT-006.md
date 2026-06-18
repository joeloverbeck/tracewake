# 0039SPICERMUT-006: Kill `events/apply.rs` SPINE survivors with an event-kind apply/replay matrix

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — adds behavior-witness tests in `tracewake-core` (test-only by default; a production correction in `events/apply.rs` lands only if a survivor reveals a real defect, per spec §4.13).
**Deps**: None

## Problem

Wave B left 44 missed mutants in `crates/tracewake-core/src/events/apply.rs` (spec §5.10), owning SPINE-01 (event application/causality), SPINE-02 (replay), SPINE-06 (transaction consequences), and SPINE-08 (mutation authority). The cluster includes starting-belief application, event allowlist/typed-diagnostic guards, caused-event IDs, parser arms for epistemic and agent payloads, optional routine template IDs, numeric parsers, intention transitions, boolean parsing, item take/place application, and actor/item preconditions. A parser-only assertion can locate a failure, but without observing applied state / provenance or loud rejection, deleted match arms and flipped precondition guards survive.

## Assumption Reassessment (2026-06-18)

1. `apply_event_stream` exists at `crates/tracewake-core/src/events/apply.rs:44`, `apply_event` at `:133`, `apply_event_with_capability` at `:141`; `EventEnvelope` at `crates/tracewake-core/src/events/envelope.rs:717` (verified by grep). The 44 seed-mutant identities are in `reports/0038_spine_cert_mutation_triage_register.md`.
2. Spec §5.10 is the implementation contract; `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md` and `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md` govern event application and replay (verified present).
3. Shared boundary under audit: the `EventEnvelope -> append/log -> apply_event_stream/apply_event -> authoritative state -> projection/replay` path, including epistemic/agent payload parsing and item/location transitions.
4. Motivating invariants: `INV-009 — Meaningful state changes require events`, `INV-010 — Every event needs a cause model`, `INV-011 — Current-state-only simulation is forbidden`, `INV-017` (random draws auditable), `INV-020 — Event schema evolution is mandatory` (malformed/unsupported payloads rejected, not silently repaired).
5. This ticket touches deterministic-replay and event-application enforcement surfaces: caused-event ancestry must be present/ordered/stable after serialization+replay (nonexistent causes fail loudly); malformed payloads cannot be accepted by flipping/deleting a validation predicate; rejected events leave authoritative state/checksum unchanged and emit the responsible typed failure; replay application uses the same authoritative path with no alternate repair path; and tests must not call private mutators directly (mutation-capability closure). The witnesses only strengthen these properties — no leakage or nondeterminism is introduced. There is **no schema shape change** here: this ticket adds tests, not an envelope/event-schema extension (the `events/apply.rs` mutation target and the `event_schema_replay_gates` test home are not schema modifications). This substrate feeds the SPINE-01/02/06/08 re-proof in ticket 021.

## Architecture Check

1. An event-kind/application matrix covering every affected parser arm and transition, asserting applied state / provenance or loud rejection, is stronger than parser-literal tests: deleted arms produce typed apply/replay failures, and precondition mutants change whether authoritative state updates and whether the diagnostic is emitted.
2. No backwards-compatibility aliasing/shims: witnesses travel `append -> apply`; no private mutator is called directly to kill an apply mutant (preserving internal mutation-capability closure).

## Verification Layers

1. INV-009/010 caused application -> replay/golden-fixture check: epistemic belief/observation/contradiction fields and source event IDs survive into sealed actor-known context and replay; caused-event ancestry is present, ordered, and stable after serialization/replay; nonexistent causes fail loudly.
2. Schema/predicate fail-closed (INV-020) -> schema validation: malformed payloads cannot be accepted by flipping/deleting the validation predicate; every affected intention status/source, optional routine-template, and boolean/numeric parser arm has an accepted case plus an unknown-token negative.
3. Item/precondition application -> replay/golden-fixture check: accepted and rejected item take/place and actor/item-precondition transitions — rejected events leave state/checksum unchanged and emit the responsible typed failure; replay uses the same authoritative path with no alternate repair.

## What to Change

### 1. Event-kind apply/replay matrix

In `event_schema_replay_gates.rs`, build a matrix covering every affected parser arm and transition through `EventEnvelope -> append/log -> apply_event_stream/apply_event -> authoritative state -> projection/replay`, using nontrivial numeric values whose effect is visible in state/ordering/duration/trace (zero-only tests are insufficient).

### 2. Epistemic + causal + precondition witnesses

Prove epistemic event fields and source event IDs survive into sealed actor-known context and replay (cross-checking with `hidden_truth_gates.rs` where the firewall is observed); prove caused-event ancestry survives serialization/replay; include accepted and rejected item take/place and actor/item-precondition transitions; prove replay application gains no alternate repair path.

### 3. Member matrix

Map all 44 historical mutants (plus any new run survivor in this file) to a concrete applied-state/provenance or loud-rejection case.

## Files to Touch

- `crates/tracewake-core/tests/event_schema_replay_gates.rs` (modify)
- `crates/tracewake-core/tests/hidden_truth_gates.rs` (modify)
- `crates/tracewake-core/src/events/apply.rs` (modify — only if a survivor reveals a real defect; default is test-only)

## Out of Scope

- Envelope identity/stream/random-draw round-trips (ticket 007).
- Replay rebuild/report (008/009) and checksum (010).
- The full mutation run (ticket 020); SPINE re-proof (ticket 021).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test event_schema_replay_gates` — passes with the event-kind apply/replay matrix and accepted/rejected precondition transitions.
2. `cargo test --locked -p tracewake-core --test hidden_truth_gates` — passes with the epistemic-field survival witnesses.
3. `cargo mutants --workspace -f crates/tracewake-core/src/events/apply.rs --no-shuffle` — all 44 historical survivors (and any newly enumerated one) are `caught`.

### Invariants

1. Every affected parser arm and precondition is observed via applied state/provenance or loud rejection; no parser-literal assertion alone certifies it.
2. No test calls a private mutator directly to kill an apply mutant; replay uses the same authoritative path with no alternate repair.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/event_schema_replay_gates.rs` — event-kind apply/replay matrix with caused-ancestry, malformed-payload, and accepted/rejected precondition rows.
2. `crates/tracewake-core/tests/hidden_truth_gates.rs` — epistemic belief/observation/contradiction field + source-event-ID survival into sealed context and replay.

### Commands

1. `cargo test --locked -p tracewake-core --test event_schema_replay_gates --test hidden_truth_gates`
2. `cargo mutants --workspace -f crates/tracewake-core/src/events/apply.rs --no-shuffle`
3. The per-file `-f` run is the correct verification boundary; the full standing campaign is ticket 020.

## Outcome

Completed: 2026-06-18

Implemented the `events/apply.rs` SPINE survivor witnesses as test-only coverage:

- Added event-kind apply/replay matrix rows in `crates/tracewake-core/tests/event_schema_replay_gates.rs` for epistemic stance/channel parser arms, starting-belief/observation/contradiction source survival, safety/routine need deltas, intention source and status transitions, routine step progress/fallback counters, caused-event IDs, typed diagnostic hidden-truth boolean validation, narrow agent no-op allowlist behavior, item take/place application, actor/item precondition rejection, and door/container boolean precondition rejection.
- Added an epistemic survival witness in `crates/tracewake-core/tests/hidden_truth_gates.rs` proving belief, observation, contradiction, channel, stance, and source-event-ID fields survive application and canonical log replay into sealed-context-visible projection records.
- No production correction in `crates/tracewake-core/src/events/apply.rs` was required; the survivors closed through behavior witnesses. The only source-shape assertion added is paired with behavior for the currently unreachable agent no-op fallback guard, whose non-allowlisted branch has no live agent event kind to exercise directly.

Verification:

- `cargo test --locked -p tracewake-core --test event_schema_replay_gates --test hidden_truth_gates` — passed.
- `cargo mutants --no-config --workspace -C=--locked -f crates/tracewake-core/src/events/apply.rs --no-shuffle` — passed with 198 mutants tested, 158 caught, 40 unviable, 0 missed. The `--no-config` deviation was required because ticket 001 made `.cargo/mutants.toml` the standing SPINE perimeter; using the literal ticket command with config would enumerate the full perimeter instead of only `events/apply.rs`.
- `cargo fmt --all --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo test --workspace --locked` — passed.
