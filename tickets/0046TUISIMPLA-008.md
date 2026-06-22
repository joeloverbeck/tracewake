# 0046TUISIMPLA-008: Conformance suite + goldens in the ordinary CI evidence lane

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — CI workflow (`.github/workflows/ci.yml`); no crate code. Confirms the parity suite is an ordinary workspace test target, not a periodic audit.
**Deps**: 0046TUISIMPLA-004, 0046TUISIMPLA-005, 0046TUISIMPLA-006, 0046TUISIMPLA-007

## Problem

Spec 0046 §4.4 `PAR-011`. The parity contract is only standing if the conformance suite + focused
goldens run on every change under the warnings-denied posture, rather than as an occasional audit.
`ci.yml` already runs `cargo test --workspace --locked` with `RUSTFLAGS: "-D warnings"`, so an ordinary
integration-test target under `crates/tracewake-tui/tests/` is picked up automatically — `PAR-011` is
largely satisfied-by-construction. The residual work is to confirm the suite is **not** `#[ignore]`d or
feature-gated out of the default run, and to make the parity evidence nameable in CI (an explicitly
named step) so the acceptance artifact can cite it.

## Assumption Reassessment (2026-06-22)

1. Verified against config at baseline `1145e109`: `.github/workflows/ci.yml` sets
   `RUSTFLAGS: "-D warnings"` (`:21`), runs `cargo clippy --workspace --all-targets -- -D warnings`
   (`:49`), `cargo build --workspace --all-targets --locked` (`:66`), and `cargo test --workspace
   --locked` (`:68`). The parity target `crates/tracewake-tui/tests/playable_capability_parity.rs`
   (created by tickets 003–007) is an ordinary integration test, so the existing `test` job already
   executes it.
2. Verified against spec 0046 §4.4 (`PAR-011`) and §7: the spec wants the suite in the "ordinary
   workspace/CI evidence lane, under the existing warnings-denied posture" and names "no new canonical
   gate code invented" — so this is lane membership + a named step, not a new gate. The acceptance
   artifact (ticket 012) cites the named CI output.
3. Shared boundary under audit: the CI `test` job ↔ the parity suite. This ticket touches `ci.yml`
   only; it adds no crate code and reclassifies no test.
4. Invariant restated (`PAR-011` motivation): `INV-098` feature acceptance is harsh — TUI-playable +
   regression-tested is part of "done"; `INV-066` every runnable phase has a TUI/view-model gate. The
   parity suite must run on every change for the gate to be standing.
5. Enforcement surface touched (evidence-lane basis): the conformance suite is the standing fail-closed
   gate over the parity contract (ticket 004) and the no-leak/determinism witnesses (ticket 005);
   running it in the ordinary `cargo test --workspace` lane introduces no leakage or nondeterminism (it
   is deterministic test execution, no wall-clock). Confirm the suite is not `#[ignore]`/feature-gated
   so it cannot silently drop out of the default run.

## Architecture Check

1. Relying on the existing `cargo test --workspace` lane (plus an explicitly named step for evidence)
   is cleaner than minting a separate parity CI job or gate code: it avoids audit lag (the suite runs
   on every change with the rest of the workspace tests) and honors the spec's "no new canonical gate
   code" constraint while keeping the evidence nameable.
2. No backwards-compatibility aliasing/shims: no parallel or legacy CI lane is introduced; the suite
   is an ordinary target.

## Verification Layers

1. `PAR-011`/`INV-098` (standing CI) → grep-proof on `ci.yml`: a named parity/conformance step (or the
   documented `cargo test --workspace` coverage) is present; the parity target is not excluded.
2. `PAR-011` (not silently droppable) → grep-proof on the suite: no `#[ignore]` / feature gate hides
   `playable_capability_parity` from the default `cargo test --workspace` run.
3. `INV-066` (gate runs) → local run: `cargo test -p tracewake-tui --test playable_capability_parity`
   passes within the standard test invocation.

## What to Change

### 1. Name the parity evidence in CI (`.github/workflows/ci.yml`)

Either add an explicitly named step (e.g. "TUI playable-capability parity") invoking
`cargo test -p tracewake-tui --test playable_capability_parity --locked`, or document that
`cargo test --workspace --locked` already covers it and add a named coverage note — whichever the
existing job structure favors — keeping the warnings-denied posture.

### 2. Confirm non-ignored / non-gated

Verify (and, if needed, correct) that the parity target and its goldens run under the default
`cargo test --workspace` invocation — no `#[ignore]`, no feature gate.

## Files to Touch

- `.github/workflows/ci.yml` (modify)

## Out of Scope

- The conformance suite/runner/goldens themselves (003–007).
- Minting any new canonical gate code (spec §1.2 / §4.4 forbid it).
- Acceptance-artifact assembly (012) and doctrine amendments (009–011).

## Acceptance Criteria

### Tests That Must Pass

1. `ci.yml` runs the parity suite under the warnings-denied posture on every change (named step or
   documented `--workspace` coverage); the parity target is not `#[ignore]`d or feature-gated.
2. `cargo test -p tracewake-tui --test playable_capability_parity --locked` passes; the four gates pass.

### Invariants

1. The parity conformance suite runs in the ordinary CI lane on every change, not as a periodic audit.
2. No new canonical certification gate code is introduced.

## Test Plan

### New/Modified Tests

1. `None — CI-config ticket; verification is grep-based landing on `ci.yml` plus running the existing
   parity target. Pipeline coverage is named in Assumption Reassessment.`

### Commands

1. `grep -nE "playable_capability_parity|parity|cargo test --workspace" .github/workflows/ci.yml`
2. `cargo test -p tracewake-tui --test playable_capability_parity --locked`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
