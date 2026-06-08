# 0004PHA1THIHAR-012: Lock CI to the strengthened gates + strengthened-gate acceptance capstone

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `.github/workflows/ci.yml` (run the strengthened gates on the pinned toolchain); §10 acceptance-evidence run (manual runbook)
**Deps**: 0004PHA1THIHAR-005, 0004PHA1THIHAR-010, 0004PHA1THIHAR-011

## Problem

The strengthened gates (negative-fixture runner, conformance evidence matrix, invariant-coverage linter, checksum parity, content typed coverage, debug quarantine, schema/replay rejections) must run in CI on the pinned toolchain, and the §10 acceptance checklist must pass end-to-end — otherwise the hardened guards exist locally but do not fail pull-request integration (spec §8 THIRD-AC-011, §10). This is the closeout capstone: it locks CI and runs the full §10 acceptance evidence.

## Assumption Reassessment (2026-06-08)

<!-- Items 1-3 always required; 4-5 selected for an invariant-motivated, deterministic-replay acceptance ticket. -->

1. `.github/workflows/ci.yml` runs `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace --all-targets --locked`, `cargo test --workspace --locked`; `rust-toolchain.toml` pins `1.93.0`. Most new gates (tickets 002–010) are `cargo test` targets already swept by `cargo test --workspace`; this ticket confirms each runs under the pinned toolchain and adds any dedicated job needed.
2. The remediation is spec §8 `THIRD-AC-011` + the §10 acceptance checklist (§10.1 mandatory workspace gates, §10.2 new lock-layer gates, §10.3 per-requirement acceptance conditions, §10.4 scoped certification wording).
3. Shared boundary under audit (capstone): the `Deps` leaf set {005, 010, 011} transitively covers the whole batch — ticket 010 reaches 001/002/003/004/006/007/008/009 through its dependencies; 005 (invariant linter) and 011 (acceptance wording) are independent leaves. This ticket exercises all of them end-to-end and introduces no new production logic (deliverable-doubles-as-capstone: it ships CI config + the acceptance runbook).
4. Motivating invariants (restated): `INV-018` (deterministic replay), `INV-091`/`INV-092` (acceptance gates / replay tested), `INV-105` (authoritative diagnostics). The acceptance gates exercised include the no-human run and deterministic replay.
5. Deterministic-replay / acceptance surface: the capstone runs the deterministic-replay and no-human acceptance gates. Confirm no gate requires an unpinned external service, network access during outcome resolution, or a tool unavailable under `rust-toolchain.toml` — every gate is a `cargo` invocation on the pinned toolchain.

## Architecture Check

1. Locking the strengthened gates into CI so none escapes the pinned toolchain, plus one capstone that runs the §10 checklist end-to-end, converts "guards exist" into "guards block integration" — the correct closeout for an anti-contamination campaign. It ships CI config and an acceptance runbook, not new production logic.
2. No backwards-compatibility shims; no gate introduces a network or unpinned-tool dependency.

## Verification Layers

1. §10.1 mandatory gates -> CI gate: the four workspace commands pass on the pinned toolchain.
2. §10.2 new lock-layer gates -> CI gate: negative-fixture runner, conformance matrix, invariant linter, checksum parity, content typed coverage, debug quarantine, and schema/replay rejections all run under `cargo test --workspace` (or a dedicated job) on the pinned toolchain.
3. No-human run / deterministic replay (`INV-018`/`INV-091`) -> replay/golden-fixture check: the acceptance run reproduces state byte-identically and advances with no human controller.

## What to Change

### 1. Lock CI

Extend `.github/workflows/ci.yml` so any dedicated negative-fixture-runner job runs on the pinned toolchain, and confirm every new gate is reached by `cargo test --workspace`. No gate may require network access or a tool absent from `rust-toolchain.toml`.

### 2. §10 acceptance-evidence runbook (manual + automated)

Author the acceptance-evidence runbook: run §10.1 (the four workspace commands) and §10.2 (the lock-layer gates) capturing output summaries; record the §10.3 per-requirement acceptance conditions; emit the §10.4 scoped certification wording using ticket 011's forbidden-wording check; name the exact commit and residual convention-only items. CI-runnable assertions are the gate commands; the artifact authoring + wording check is the implementer runbook step.

## Files to Touch

- `.github/workflows/ci.yml` (modify — the only ticket touching CI; run the strengthened gates on the pinned toolchain)

## Out of Scope

- The individual gate implementations (tickets 001–011).
- The `docs/4-specs/SPEC_LEDGER.md` entry and the final-home move to `archive/specs/` — deferred to spec promotion (a cross-spec follow-up), not part of this batch.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` — all §10.1 mandatory gates pass on the pinned toolchain.
2. `cargo test --workspace` reaches every §10.2 lock-layer gate (negative-fixture runner, conformance matrix, invariant linter, checksum parity, content typed coverage, debug quarantine, schema/replay) — confirmed green.
3. The §10 acceptance artifact is produced with scoped wording, exact commit, and gates-run summaries, and passes the ticket-011 forbidden-wording check.

### Invariants

1. No strengthened gate exists only locally — each runs in CI on the pinned toolchain.
2. The acceptance run satisfies the deterministic-replay and no-human acceptance gates and uses scoped, non-overclaiming wording.

## Test Plan

### New/Modified Tests

1. `None — capstone/CI ticket; verification is the §10 gate commands plus the acceptance runbook. New test infrastructure (the strengthened gates) is owned by tickets 002–011; this ticket exercises and locks them.`

### Commands

1. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings`
2. `cargo build --workspace --all-targets --locked && cargo test --workspace`
