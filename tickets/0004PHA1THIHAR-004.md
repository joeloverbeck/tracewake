# 0004PHA1THIHAR-004: Harden token-scanner walk/match and document it smoke-only

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` tests (`anti_regression_guards.rs` source walk, match behavior, smoke-only documentation, discovery fixtures)
**Deps**: None

## Problem

The banned-token scanner in `anti_regression_guards.rs` is brittle: `production_sources()` walks only `crates/tracewake-core/src` by a hand-maintained root, `production()` truncates a source at the first `#[cfg(test)]`, and `nondeterminism_api_gate()` matches by `source.contains(token)` — so a new production file in a nested module, a `#[cfg(test)]` block with production items after it, or a token inside a comment/string can silently evade or falsely trip the scan (spec §6 F-010, §8 THIRD-AC-003). The scanner must be explicitly documented as smoke-only and its walk/match hardened, with the clippy policy (ticket 002) as the primary layer.

## Assumption Reassessment (2026-06-08)

<!-- Items 1-3 always required; 4-5 selected for an invariant-motivated, determinism-smoke ticket. -->

1. In `crates/tracewake-core/tests/anti_regression_guards.rs`: `BANNED_NONDETERMINISM_TOKENS` (`:35`, includes the `Command::new` token at `:85`), `production()` splits on `#[cfg(test)]` (`:115`), `production_sources()` walks `crates/tracewake-core/src` (`:130`), `nondeterminism_api_gate()` does `source.contains(token)` (`:357`).
2. The remediation is spec §8 `THIRD-AC-003` + the §9.2 `source_guard_*` fixtures (`source_guard_discovers_new_nested_production_file`, `source_guard_does_not_silently_skip_production_after_cfg_test`), reassessed this session.
3. Shared boundary under audit: `anti_regression_guards.rs` is an N-way hub also modified by tickets 001, 003, and 006 — coordinate the mechanical merge; this ticket owns the scanner walk/match and its smoke documentation.
4. Motivating invariants (restated): `INV-017` (seedable/auditable randomness), `INV-018` (deterministic replay), `INV-092` (deterministic replay is tested). The scanner guards nondeterminism entry but is the weak layer.
5. Determinism-smoke surface: the scanner is a cheap early-warning smoke feeding the determinism guarantee; the primary enforcement is `clippy.toml` + the ticket-002 negative-fixture runner. This ticket only widens the walk and labels match sensitivity — it weakens nothing and removes no guard.

## Architecture Check

1. Widening the walk (recurse all production roots, or enumerate intentionally-excluded roots with rationale; do not truncate production at the first `#[cfg(test)]` when non-test items follow; ignore comments/strings or label that sensitivity) plus an explicit smoke-only annotation is cleaner than silently trusting a brittle substring scan: it keeps a cheap smoke layer honest while clippy + the ticket-002 runner carry the real guarantee.
2. No backwards-compatibility shims: the scanner is hardened and annotated in place, not replaced by a parallel mechanism or aliased.

## Verification Layers

1. `INV-092` (determinism tested) -> new test: `source_guard_discovers_new_nested_production_file` proves the walk finds a newly added nested-module source.
2. `cfg(test)` robustness -> new test: `source_guard_does_not_silently_skip_production_after_cfg_test`.
3. Smoke documentation -> codebase grep-proof: the scanner's doc comment / conformance metadata declares substring matching smoke-only and names clippy + the ticket-002 runner as primary.

## What to Change

### 1. Harden the source walk

Recurse every intended production root (`core/src`, and — if in scope — `content/src`, `tui/src`), or enumerate intentionally-excluded roots with a rationale. Stop truncating production at the first `#[cfg(test)]` when production items can follow.

### 2. Harden the match

Ignore comments/strings during token matching, or explicitly label comment/string sensitivity as smoke-only in code and conformance metadata.

### 3. Document smoke-only + add discovery fixtures

Annotate the scanner as smoke-only (pointing to clippy + ticket-002 runner as primary). Add `source_guard_discovers_new_nested_production_file` and `source_guard_does_not_silently_skip_production_after_cfg_test`.

## Files to Touch

- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — shared hub w/ 001, 003, 006: walk/match hardening, smoke annotation, `source_guard_*` fixtures using an in-test temp module tree)

## Out of Scope

- The clippy-negative banned-API fixtures and the runner harness (ticket 002).
- Checksum field-to-registry parity (ticket 006) and the direct-mutation guards (ticket 003), even though they share this file.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core` — `source_guard_discovers_new_nested_production_file` and `source_guard_does_not_silently_skip_production_after_cfg_test` pass.
2. `cargo test --workspace` — existing `nondeterminism_api_gate` still passes after the walk/match changes.
3. `grep -qiE 'smoke' crates/tracewake-core/tests/anti_regression_guards.rs` near the scanner — smoke-only annotation present.

### Invariants

1. A newly added nested production source is discovered by the walk (no silent skip).
2. Production items after a `#[cfg(test)]` block are not truncated out of the scan.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — `source_guard_*` discovery fixtures + hardened walk/match; rationale: prove the smoke layer cannot silently miss a new file or `cfg(test)` rearrangement.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo build --workspace --all-targets --locked && cargo test --workspace`
