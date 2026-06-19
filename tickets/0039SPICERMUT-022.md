# 0039SPICERMUT-022: Controller binding mutation survivors

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — strengthens controller binding tests around possession/debug authorization. No schema changes.
**Deps**: 0039SPICERMUT-020

## Problem

The full standing mutation campaign in ticket 020 found five surviving mutants in
`crates/tracewake-core/src/controller.rs`. They weaken controller possession
binding attach/detach accounting, debug binding visibility, and authorization.
They are not accepted as equivalent/non-critical and block a passing SPINE-CERT
mutation row.

## Assumption Reassessment (2026-06-18)

1. The survivors are recorded in `reports/0039_spine_cert_mutation_triage_register.md` and `reports/0039_spine_cert_mutation_missed.txt`.
2. The governing contract is spec 0039 §4.8–§4.12: missed tool outcomes remain separate from certification disposition and must be killed or reviewer-approved equivalent/non-critical.
3. Shared boundary under audit: controller binding authorization and debug exposure as part of possession parity and debug/non-diegetic quarantine.
4. Motivating invariants: possession parity and no debug-only path may silently bypass runtime authority.
5. This ticket touches controller authorization tests only; it must not add aliases, shims, or runtime-only branches to suppress cargo-mutants.

## Architecture Check

1. Add behavioral witnesses that fail when attach/detach accounting, debug binding enumeration, or authorization is removed or inverted.
2. No backwards-compatibility aliasing/shims introduced.

## Verification Layers

1. Controller authorization -> targeted unit/integration tests around `ControllerBindings::authorize`.
2. Controller binding accounting -> tests that observe attach/detach count deltas and binding visibility.
3. Mutation proof -> `cargo mutants --no-config --workspace -C=--locked -f crates/tracewake-core/src/controller.rs --no-shuffle`.

## What to Change

### 1. Kill controller survivors

Add focused tests for these identities:

```text
crates/tracewake-core/src/controller.rs:115:9: replace ControllerBindings::authorize -> Result<(), ControllerError> with Ok(())
crates/tracewake-core/src/controller.rs:83:28: replace += with *= in ControllerBindings::detach
crates/tracewake-core/src/controller.rs:50:28: replace += with *= in ControllerBindings::attach
crates/tracewake-core/src/controller.rs:107:9: replace ControllerBindings::debug_bindings -> Vec<&RuntimeControllerBinding> with vec![]
crates/tracewake-core/src/controller.rs:83:28: replace += with -= in ControllerBindings::detach
```

## Files to Touch

- `crates/tracewake-core/src/controller.rs` (modify tests)

## Out of Scope

- State and epistemic projection survivors from ticket 020.
- Accepting baseline misses or equivalent/non-critical dispositions.

## Acceptance Criteria

### Tests That Must Pass

1. Targeted controller tests added by this ticket.
2. `cargo mutants --no-config --workspace -C=--locked -f crates/tracewake-core/src/controller.rs --no-shuffle` kills the listed controller survivors or records reviewed unviable/equivalent evidence.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace --locked`.

### Invariants

1. Controller authorization remains fail-closed.
2. Debug binding visibility remains non-diegetic and cannot authorize production possession changes.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/controller.rs` — controller binding authorization/accounting tests.

### Commands

1. `cargo mutants --no-config --workspace -C=--locked -f crates/tracewake-core/src/controller.rs --no-shuffle`
2. `cargo test --workspace --locked`
3. Per-file mutation is the correct first proof boundary because ticket 020 already established the full standing denominator.
