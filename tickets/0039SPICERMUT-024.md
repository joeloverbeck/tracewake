# 0039SPICERMUT-024: Epistemic projection mutation survivors

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — strengthens epistemic projection checksum, actor-known projection, location, and source-summary tests. No schema changes.
**Deps**: 0039SPICERMUT-020

## Problem

The full standing mutation campaign in ticket 020 found 29 surviving mutants in
`crates/tracewake-core/src/epistemics/projection.rs`; the timeout retry
reclassified one additional checksum timeout as a missed mutant. These 30
projection survivors are not accepted as equivalent/non-critical and block a
passing SPINE-CERT mutation row.

## Assumption Reassessment (2026-06-18)

1. The survivors are recorded in `reports/0039_spine_cert_mutation_triage_register.md`, `reports/0039_spine_cert_mutation_missed.txt`, and `reports/0039_spine_cert_mutation_timeout_retry_missed.txt`.
2. The governing contract is spec 0039 §4.8–§4.12: missed tool outcomes remain separate from certification disposition and must be killed or reviewer-approved equivalent/non-critical.
3. Shared boundary under audit: holder-known epistemic projection checksums, actor-known record serialization, freshness classification, observation location/holder keys, and source summaries.
4. Motivating invariants: subjective epistemics and no simulation fact born from prose require projection checksums and actor-known serialization to be behaviorally witnessed.
5. This ticket touches projection tests only; it must not add aliases, shims, or runtime-only branches to suppress cargo-mutants.

## Architecture Check

1. Add behavioral witnesses around projection checksum sensitivity, non-empty/has-belief semantics, context notebook filtering, stable source IDs, canonical serialization, freshness ordering, and observation key/source-summary output.
2. No backwards-compatibility aliasing/shims introduced.

## Verification Layers

1. Projection checksum -> tests that assert canonical-line sensitivity and `as_str` value preservation.
2. Actor-known projection -> tests for source stable IDs, serialized records, location/holder/source summaries, and observation classification.
3. Mutation proof -> `cargo mutants --no-config --workspace -C=--locked -f crates/tracewake-core/src/epistemics/projection.rs --no-shuffle`.

## What to Change

### 1. Kill projection survivors

Add focused tests for the projection identities listed in
`reports/0039_spine_cert_mutation_triage_register.md`.

## Files to Touch

- `crates/tracewake-core/src/epistemics/projection.rs` (modify tests)

## Out of Scope

- Controller and state survivors from ticket 020.
- Accepting baseline misses or equivalent/non-critical dispositions.

## Acceptance Criteria

### Tests That Must Pass

1. Targeted projection tests added by this ticket.
2. `cargo mutants --no-config --workspace -C=--locked -f crates/tracewake-core/src/epistemics/projection.rs --no-shuffle` kills the listed projection survivors or records reviewed unviable/equivalent evidence.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace --locked`.

### Invariants

1. Projection checksums remain sensitive to canonical record changes.
2. Actor-known projections remain holder-known, source-ancestry preserving, and replayable.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/epistemics/projection.rs` — checksum and actor-known projection tests.

### Commands

1. `cargo mutants --no-config --workspace -C=--locked -f crates/tracewake-core/src/epistemics/projection.rs --no-shuffle`
2. `cargo test --workspace --locked`
3. Per-file mutation is the correct first proof boundary because ticket 020 already established the full standing denominator.
