# 0039SPICERMUT-023: State accessor and door connectivity mutation survivors

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — strengthens state accessor and connectivity tests. No schema changes.
**Deps**: 0039SPICERMUT-020

## Problem

The full standing mutation campaign in ticket 020 found four surviving mutants in
`crates/tracewake-core/src/state.rs`. They remove observable threshold/candidate
goal records or invert/force door connectivity. They are not accepted as
equivalent/non-critical and block a passing SPINE-CERT mutation row.

## Assumption Reassessment (2026-06-18)

1. The survivors are recorded in `reports/0039_spine_cert_mutation_triage_register.md` and `reports/0039_spine_cert_mutation_missed.txt`.
2. The governing contract is spec 0039 §4.8–§4.12: missed tool outcomes remain separate from certification disposition and must be killed or reviewer-approved equivalent/non-critical.
3. Shared boundary under audit: authoritative state accessors and place connectivity used by action validation, projection, and replay evidence.
4. Motivating invariants: event-sourced causality and replayable traces require state records and connectivity checks to remain observable and truthful.
5. This ticket touches state tests only; it must not add aliases, shims, or runtime-only branches to suppress cargo-mutants.

## Architecture Check

1. Add behavioral witnesses for populated accessor maps and positive/negative door connectivity cases.
2. No backwards-compatibility aliasing/shims introduced.

## Verification Layers

1. State accessor records -> tests that populate and read threshold/candidate-goal maps.
2. Door connectivity -> tests for both connected and unconnected place IDs.
3. Mutation proof -> `cargo mutants --no-config --workspace -C=--locked -f crates/tracewake-core/src/state.rs --no-shuffle`.

## What to Change

### 1. Kill state survivors

Add focused tests for these identities:

```text
crates/tracewake-core/src/state.rs:335:9: replace AgentState::need_threshold_crossings -> &BTreeMap<crate::ids::EventId, NeedThresholdCrossingRecord> with Box::leak(Box::new(BTreeMap::new()))
crates/tracewake-core/src/state.rs:347:9: replace AgentState::candidate_goal_evaluations -> &BTreeMap<crate::ids::EventId, CandidateGoalEvaluationRecord> with Box::leak(Box::new(BTreeMap::new()))
crates/tracewake-core/src/state.rs:422:9: replace DoorState::connects_place -> bool with true
crates/tracewake-core/src/state.rs:422:58: replace == with != in DoorState::connects_place
```

## Files to Touch

- `crates/tracewake-core/src/state.rs` (modify tests)

## Out of Scope

- Controller and epistemic projection survivors from ticket 020.
- Accepting baseline misses or equivalent/non-critical dispositions.

## Acceptance Criteria

### Tests That Must Pass

1. Targeted state tests added by this ticket.
2. `cargo mutants --no-config --workspace -C=--locked -f crates/tracewake-core/src/state.rs --no-shuffle` kills the listed state survivors or records reviewed unviable/equivalent evidence.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace --locked`.

### Invariants

1. State accessors remain truthful projections of recorded state.
2. Door connectivity cannot pass for unrelated places.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/state.rs` — accessor and door connectivity tests.

### Commands

1. `cargo mutants --no-config --workspace -C=--locked -f crates/tracewake-core/src/state.rs --no-shuffle`
2. `cargo test --workspace --locked`
3. Per-file mutation is the correct first proof boundary because ticket 020 already established the full standing denominator.
