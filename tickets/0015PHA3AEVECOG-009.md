# 0015PHA3AEVECOG-009: Guard durability — module-tree globs and a census guard

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `anti_regression_guards.rs` targeted guards take module-tree globs; new census guard
**Deps**: 0015PHA3AEVECOG-004, 0015PHA3AEVECOG-005, 0015PHA3AEVECOG-006, 0015PHA3AEVECOG-007, 0015PHA3AEVECOG-008

## Problem

ORD-HARD-013: the targeted source guards scan hardcoded per-file constants, so a new submodule escapes the targeted truth-firewall rules. `anti_regression_guards.rs` defines per-file `include_str!` constants (`SCHEDULER_RS:5`, `TRANSACTION_RS:8`, `NO_HUMAN_SURFACE_RS:7`, …); only the banned-nondeterminism scan walks `production_sources()` (:229) recursively. A future `scheduler/completions.rs` or `agent/` submodule would be covered by the determinism scan but by **none** of the targeted truth-firewall rules — the lock layer itself drifts. (No doctrine is breached directly; this is lock-layer durability: the anti-contamination measure must resist drift. `clippy.toml`/CI were verified strong and need no change.)

This ticket generalizes the targeted guards to take module-tree globs where the rule is about a *layer* rather than one file, and adds a census guard so adding a file to a guarded layer without classifying it fails the suite. It depends on the guard-adding tickets (004/005/006/007/008) so it generalizes the final guard set.

## Assumption Reassessment (2026-06-09)

1. Current code (verified): `anti_regression_guards.rs` per-file constants (`SCHEDULER_RS:5` … `TUI_APP_RS:24`); `production_sources()` (:229) and `production_sources_from_roots()` (:241) already do recursive discovery for the nondeterminism scan; the `guard_014_*`/`guard_006_*`/`guard_003_*`/etc. families use the single-file constants. After 004/005/006/007/008 the `guard_015_*` and extended `guard_014_*` rules exist; this ticket re-expresses the layer-scoped ones over globs.
2. Specs/docs: spec 0015 §ORD-HARD-013 (required correction) and §5.1 (glob-based file discovery + census guard); no INV directly (lock-layer durability), but the guards enforce INV-099–103/018/024 etc. for their layers.
3. Shared boundary under audit: the guard suite's file-discovery mechanism. The census guard asserts the guard file-list equals the actual module tree for the guarded layers (`src/agent/**`, `src/scheduler*`, `src/projections*`), so an unclassified new file fails the suite. This ticket **rewrites** shared-hub `anti_regression_guards.rs` after the per-finding guards land — coordinate the merge (Step 6 hub).
4. INV (indirect) — the guards exist to enforce the truth-firewall set; generalizing their discovery keeps that enforcement from silently lapsing when the module tree grows. The census guard is the meta-lock.
5. Fail-closed / deterministic-replay surface (the guards themselves): confirm the glob-based guards remain deterministic (sorted file iteration, not incidental directory order) and fail-closed (an unclassified file or a banned token fails `cargo test`). No behavior change to production code; this is test-infrastructure only.
6. Schema extension: none — test infrastructure only. (Menu item not applicable.)

## Architecture Check

1. Layer-scoped globs plus a census guard make the lock layer self-maintaining: a new `agent/`/`scheduler*`/`projections*` file is automatically in-scope or fails the census, closing the drift gap a per-file constant leaves. Reusing the existing `production_sources_from_roots()` recursion avoids a second discovery mechanism.
2. No shims: the per-file constants for layer-scoped rules are replaced by globs, not kept alongside as a fallback; single-file constants remain only where a rule is genuinely about one specific file.

## Verification Layers

1. Lock durability → codebase grep-proof: the layer-scoped guards iterate a module-tree glob (`src/agent/**`, `src/scheduler*`, `src/projections*`), not a single `include_str!` constant.
2. Census → schema validation (test): the census guard asserts the guarded file-list equals the actual module tree; adding an unclassified file to a guarded layer fails the suite.
3. Determinism → manual review: glob iteration is sorted/stable so the guard is order-independent.
4. Single-layer ticket — additional layer mapping not applicable; this ticket's deliverable *is* the verification infrastructure, verified by demonstrating a planted unclassified file fails the census.

## What to Change

### 1. Glob-based targeted guards

Re-express the layer-scoped targeted guards (truth-firewall rules over `agent/`, `scheduler*`, `projections*`) to discover files via a module-tree glob using `production_sources_from_roots()`-style recursion, instead of single-file `include_str!` constants.

### 2. Census guard

Add a guard asserting the guard file-list equals the actual module tree for the guarded layers, so adding a file without classifying it fails the suite.

## Files to Touch

- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — **shared merge hub**: rewrites the layer-scoped guards added by 004/005/006/007/008)

## Out of Scope

- The per-finding guards themselves (added by 004/005/006/007/008; this ticket generalizes their discovery).
- `clippy.toml` / CI (verified strong; no change).
- Any production-code change.

## Acceptance Criteria

### Tests That Must Pass

1. The layer-scoped truth-firewall guards cover a newly-added file under a guarded layer without editing the guard file (demonstrated by a temporary planted file in a test, or by the census guard failing on an unclassified file).
2. The census guard fails when a file is added to a guarded layer without classification.
3. `cargo test -p tracewake-core --test anti_regression_guards` green.

### Invariants

1. Layer-scoped guards are discovery-driven (module-tree glob), not per-file-constant, so new submodules are in-scope automatically.
2. The guard file-list provably equals the guarded module tree (census).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — glob-based layer guards + census guard.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
