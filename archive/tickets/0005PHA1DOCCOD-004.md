# 0005PHA1DOCCOD-004: Make the pipeline phase-boundary guard reachable and typed-covered

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` pipeline phase-boundary stage and its conformance/acceptance coverage (and `report.rs` only if a new reason code is needed).
**Deps**: 0005PHA1DOCCOD-001

## Problem

The pipeline already has a `PipelineStage::PhaseBoundaryValidation` stage (`crates/tracewake-core/src/actions/pipeline.rs:46`) and a typed `ReasonCode::PhaseUnsupportedAction` (`crates/tracewake-core/src/actions/report.rs:12`, display `"phase_unsupported_action"` at `:52`), but the guard at `pipeline.rs:496` was unreachable while every action was `phase1_implemented: true`. After `0005PHA1DOCCOD-001` retypes the flag to `ActionScope`, the guard reads scope against an active-scope set but (per that ticket) still admits all registered scopes, so the rejection path remains unexercised. `ALIGN-REQ-004` requires this defense-in-depth guard to be reachable and covered: either an adversarial test drives a registered-but-out-of-active-phase action through `PhaseBoundaryValidation` and asserts the typed stage + reason code, or — if the Phase 1 API makes that unconstructible — a compile-fail test proves the impossibility plus a narrow unit test covers the typed rejection path.

## Assumption Reassessment (2026-06-09)

1. `PipelineStage::PhaseBoundaryValidation` is declared at `crates/tracewake-core/src/actions/pipeline.rs:46` and sits in the stage order before mutation/append (`:71` in the canonical stage list). `ReasonCode::PhaseUnsupportedAction` exists at `report.rs:12` with stable string `"phase_unsupported_action"` (`:52`). The guard is at `pipeline.rs:496` (`if !definition.phase1_implemented` pre-`001`; `definition.scope` vs active-scope post-`001`).
2. Spec `ALIGN-REQ-004` requires the guard reachable in ≥1 adversarial test, or a compile-fail proof of impossibility across the Phase 1 API plus a narrow internal unit test for the typed rejection.
3. Shared boundary under audit: the `ActionScope`/active-scope contract from `0005PHA1DOCCOD-001` and the pipeline's `PhaseBoundaryValidation` stage. This ticket exercises the guard; it does not change the content loader.
4. Invariants motivating this ticket: `INV-098` (a guard that cannot fire is not proven), `INV-105` (the rejection is observable as a typed `PipelineStage` + `ReasonCode`, not a display string).
5. Deterministic-replay surface: `PhaseBoundaryValidation` participates in pipeline acceptance, so a rejected proposal must produce a deterministic, typed `ValidationReport` and append no event (rejection is eventful only as a failed-attempt report where modeled, not as a world mutation). Confirm the test drives the guard in a non-Phase1/internal context so it does not change any currently-passing fixture's accepted-action set, and that determinism of accepted runs is unaffected.

## Architecture Check

1. Proving the guard fires (or proving it cannot be reached from the Phase 1 API and unit-testing the typed path) closes the gap where a defense-in-depth stage silently never executes. Driving it from an internal/non-Phase1 context keeps the production Phase 1 path unchanged while still exercising the rejection — the guard remains a real second line of defense behind the content-validation boundary, not dead code.
2. No backwards-compatibility shim: the existing `PhaseBoundaryValidation` stage and `ReasonCode::PhaseUnsupportedAction` are reused, not duplicated; no new always-true flag is introduced.

## Verification Layers

1. `INV-098` (guard reachable/proven) -> replay/golden-fixture check: an adversarial test submits a registered action whose `ActionScope` is outside the active scope and asserts the proposal is rejected at `PipelineStage::PhaseBoundaryValidation`.
2. `INV-105` (typed rejection) -> codebase grep-proof + test assertion: the rejection carries `ReasonCode::PhaseUnsupportedAction`; the test inspects the typed stage and reason code, not a message substring.
3. Compile-fail alternative (if applicable) -> manual review: if the Phase 1 API cannot construct an out-of-scope proposal, a `trybuild`/compile-fail test (or documented equivalent) proves it, and a narrow unit test still covers the typed rejection path so the reason code is not dead.

## What to Change

### 1. Exercise the phase-boundary guard

In `crates/tracewake-core/src/actions/pipeline.rs` (and/or its test module), ensure the guard at `:496` rejects an action whose `ActionScope` is outside the active-scope set, returning a `ValidationReport` whose failed stage is `PhaseBoundaryValidation` and whose reason includes `ReasonCode::PhaseUnsupportedAction`. Add a new reason code in `report.rs` only if the existing one is insufficient.

### 2. Add the adversarial (or compile-fail) coverage

In `crates/tracewake-core/tests/spine_conformance.rs` or `crates/tracewake-core/tests/acceptance_gates.rs`, add a test that constructs/obtains a registered out-of-active-scope action in an internal/non-Phase1 context and asserts the typed stage + reason code. If type-state makes that unconstructible across the Phase 1 API, add the compile-fail proof plus a narrow unit test for the typed rejection.

## Files to Touch

- `crates/tracewake-core/src/actions/pipeline.rs` (modify)
- `crates/tracewake-core/src/actions/report.rs` (modify — only if a new reason code is required)
- `crates/tracewake-core/tests/spine_conformance.rs` (modify) or `crates/tracewake-core/tests/acceptance_gates.rs` (modify)

## Out of Scope

- The `ActionScope` retype (`0005PHA1DOCCOD-001`, dependency).
- Content-side rejection before runtime (`0005PHA1DOCCOD-002`/`-003`) — this ticket covers the pipeline defense-in-depth layer, not content validation.
- The source-level loader-registration guard (`0005PHA1DOCCOD-005`).
- CI wiring (`0005PHA1DOCCOD-006`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --locked -p tracewake-core --test spine_conformance` (or `--test acceptance_gates`) — the phase-boundary rejection test passes with the typed stage + reason code.
2. `cargo test --workspace` — full suite green.

### Invariants

1. The `PhaseBoundaryValidation` guard is demonstrably reachable (or provably unconstructible from the Phase 1 API, with the typed path unit-covered) (`INV-098`).
2. The rejection is observable as a typed `PipelineStage` + `ReasonCode`, never only a display string (`INV-105`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/spine_conformance.rs` (or `acceptance_gates.rs`) — adversarial out-of-active-scope action → `PhaseBoundaryValidation` + `ReasonCode::PhaseUnsupportedAction`; or the compile-fail proof + narrow unit test.

### Commands

1. `cargo test --locked -p tracewake-core --test spine_conformance`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`

## Outcome

Completed on 2026-06-09.

Changes:

1. Added `ActionRegistry::new_with_active_scopes(...)` so tests and internal callers can construct a registry whose active phase scopes are narrower than the registered action definitions.
2. Added `out_of_active_scope_action_rejects_at_phase_boundary` in `crates/tracewake-core/tests/acceptance_gates.rs`, registering `check_container` while activating only `ActionScope::Phase1`.
3. Asserted the rejection is typed as `PipelineStage::PhaseBoundaryValidation` with `ReasonCode::PhaseUnsupportedAction`, and that the rejected action appends only an `ActionRejected` event.

Deviations:

1. The guard was covered in `acceptance_gates.rs` rather than `spine_conformance.rs` because the former already owns end-to-end pipeline acceptance behavior.
2. No new reason code was required; the existing `ReasonCode::PhaseUnsupportedAction` matched the contract.

Verification:

1. `cargo fmt --all --check`
2. `cargo test --locked -p tracewake-core --test acceptance_gates`
3. `cargo clippy --workspace --all-targets -- -D warnings`
4. `cargo build --workspace --all-targets --locked`
5. `cargo test --workspace`
