# 0005PHA1DOCCOD-006: CI lock-layer wiring + ALIGN-001 acceptance capstone

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — `.github/workflows/ci.yml` lock-layer job; no new production logic (acceptance capstone for the spec's §9 checklist).
**Deps**: 0005PHA1DOCCOD-002, 0005PHA1DOCCOD-003, 0005PHA1DOCCOD-004, 0005PHA1DOCCOD-005

## Problem

The new Phase 1 action-scope gates added by `0005PHA1DOCCOD-002`…`-005` must run in CI's named lock-layer suites so the `ALIGN-001` boundary cannot silently regress, and the spec's §9 acceptance checklist must be exercised end-to-end on the remediation commit. This ticket extends the existing lock-layer job (`.github/workflows/ci.yml:64-89`, which already runs named strengthened gates) to cover the new/extended gates if they are not already covered by `cargo test --workspace --locked`, and serves as the acceptance capstone enumerating the spec's §9 gates. Per `ALIGN-REQ-005`, the lock-layer line is only required if the maintainers do not intentionally rely on `cargo test --workspace --locked` (in which case the rationale is documented).

## Assumption Reassessment (2026-06-09)

1. `.github/workflows/ci.yml` has a `lock-layer-gates` job (`:64`) running named suites including `negative_fixture_runner`, `doc_invariant_references`, `spine_conformance`, `anti_regression_guards`, `forbidden_content`, plus TUI suites (`:80-89`), and a primary job running `cargo test --workspace --locked` (`:62`). The pinned toolchain is in `rust-toolchain.toml`.
2. Spec `ALIGN-REQ-005` requires the new content/core action-scope gates to run in named lock-layer suites, OR a documented reliance on `cargo test --workspace --locked`. Spec §9 enumerates the acceptance checklist (workspace gates + new/extended lock-layer gates + per-requirement evidence + scoped certification wording).
3. Shared boundary under audit: the CI gate set vs the tests produced by `0005PHA1DOCCOD-002`…`-005`. This ticket adds no production logic; it wires gates and enumerates acceptance evidence. It doubles as the capstone (deliverable-doubles-as-capstone shape) since the phase's verification harness is itself this deliverable.
4. Invariant motivating this ticket: `INV-098` (feature acceptance is harsh — the remediation is "done" only when all named gates pass on the fix commit, with adversarial coverage wired into CI).
5. Deterministic/fail-closed surface: CI runs the gates with `--locked` on the pinned toolchain; confirm the new gate lines reference real test targets (the suites extended by `-002`…`-005`) and that the job fails closed when any gate fails. No epistemic-leakage or replay-semantics change — this is CI wiring + acceptance enumeration.

## Architecture Check

1. Wiring the new gates into the existing named lock-layer job (rather than a new bespoke job) keeps the CI surface consistent with the prior hardening specs' lock-layer convention and makes the `ALIGN-001` gates first-class, auditable, named runs. Folding the acceptance capstone here avoids a synthetic verification-only ticket: the phase's harness IS this deliverable.
2. No backwards-compatibility shim: existing gate lines are extended/added, none aliased; no fallback that skips a gate.

## Verification Layers

1. `INV-098` (harsh acceptance wired) -> CI config grep-proof: `.github/workflows/ci.yml` lock-layer job names (or `cargo test --workspace --locked` provably covers) the `forbidden_content` Phase-1-rejection cases, the content schema-conformance scope check, the core `spine_conformance`/`acceptance_gates` pipeline-boundary coverage, and the `anti_regression_guards`/`negative_fixture_runner` loader guard.
2. Acceptance §9 -> replay/golden-fixture + workspace check: the four workspace gates (`fmt`, `clippy -D warnings`, `build --locked`, `test --workspace`) plus the per-requirement evidence rows all pass on the remediation commit.
3. Scoped wording -> manual review: the acceptance artifact uses the spec §9 allowed scoped-remediation wording and none of the forbidden over-claims.

## What to Change

### 1. Extend the CI lock-layer job

In `.github/workflows/ci.yml`, add (or confirm coverage of) the new/extended gates in the `lock-layer-gates` job: the `forbidden_content` Phase-1 rejection cases (`-003`), the content schema-conformance scope representation (`-002`), the core pipeline-boundary coverage (`-004`), and the loader-registration guard (`-005`). If relying solely on `cargo test --workspace --locked`, add a comment documenting why no new named line is needed.

### 2. Record the acceptance evidence (capstone)

Enumerate the spec §9 acceptance checklist as the ticket's acceptance criteria sub-cases (workspace gates + per-requirement evidence ALIGN-REQ-001…006), each mapped to a re-runnable command. Use the spec §9 allowed scoped-remediation certification wording; avoid the forbidden over-claims.

## Files to Touch

- `.github/workflows/ci.yml` (modify)

## Out of Scope

- The implementation of any gate (`0005PHA1DOCCOD-001`…`-005`) — this ticket wires and exercises them, it does not modify their source.
- Spec promotion to `archive/specs/` and any `docs/4-specs/SPEC_LEDGER.md` annotation — those are spec-promotion steps outside this ticket batch (see Step 6 cross-spec follow-ups).
- Latest-main, later-phase, or full-project certification (spec §10 boundary).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo fmt --all --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo build --workspace --all-targets --locked`
4. `cargo test --workspace` — and each per-requirement gate from `0005PHA1DOCCOD-001`…`-005` passes (ALIGN-REQ-001…006 evidence).

### Invariants

1. Every new `ALIGN-001` gate runs in CI as a named lock-layer suite or is provably covered by `cargo test --workspace --locked` with a documented rationale (`INV-098`).
2. The acceptance artifact uses only the spec §9 allowed scoped-remediation wording — no latest-main / later-phase / full-project over-claim.

## Test Plan

### New/Modified Tests

1. `None — CI-wiring + acceptance capstone; verification is command-based and the underlying gates ship in `0005PHA1DOCCOD-001`…`-005`.`

### Commands

1. `grep -nE "forbidden_content|schema_conformance|spine_conformance|anti_regression_guards|negative_fixture_runner" .github/workflows/ci.yml` — confirm the gate lines (or the documented `--workspace --locked` reliance).
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
