# 0003PHA1SPIANT-005: Nondeterminism banned-API gate (clippy + source scan)

**Status**: PENDING
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — new workspace `clippy.toml`; new no-dependency source-scan conformance test in `tracewake-core`
**Deps**: None

## Problem

The audited spine uses ordered `BTreeMap`/`BTreeSet` and discrete `SimTick` time, with no outcome-path use of `HashMap`, `HashSet`, `SystemTime`, `Instant`, unseeded RNG, threads, network, or filesystem reads. But that absence is a fact at this commit, not a structural rule — a future contributor could introduce a randomly-seeded `HashMap` iteration, a wall-clock read, or an unseeded RNG into an outcome path and break replay determinism, and CI would not catch it. Spec `0003` §5.4 / SPINE-AC-005 require a pinned-toolchain-compatible nondeterminism gate (clippy `disallowed-types`/`disallowed-methods` plus a no-dependency source-scan test) with a narrow, rationale-bearing allowlist.

## Assumption Reassessment (2026-06-08)

1. Repo-wide grep at this commit confirms **zero** `HashMap`/`HashSet`/`SystemTime`/`Instant`/`rand`/thread-spawn uses in `crates/tracewake-core/src`, `crates/tracewake-content/src`, and `crates/tracewake-tui/src` (recorded in spec §7 SPINE-AC-005 Current evidence after the in-session reassessment). So the gate can land with an **empty allowlist** and pass clean on first run.
2. The workspace pins Rust `1.93.0` with clippy in `rust-toolchain.toml:1-7`; CI runs fmt/clippy/build/test (`.github/workflows/ci.yml:20-62`); no `clippy.toml` or `deny.toml` exists yet (verified absent). Clippy at this toolchain supports `disallowed-types` / `disallowed-methods`. Spec `specs/0003_PHASE_1_SPINE_ANTI_CONTAMINATION_AND_STRUCTURAL_HARDENING_SPEC.md` §7 SPINE-AC-005 mandates a `clippy.toml` banning `std::collections::HashMap`/`HashSet`, `std::time::SystemTime`/`Instant`, and known RNG/process/thread APIs in outcome-affecting code, plus a no-dep source-scan test where clippy cannot express module-scoped exceptions, with a narrow allowlist (empty for `tracewake-core` outcome paths).
3. Boundary under audit: the workspace lint configuration (`clippy.toml`) and CI gate (`cargo clippy`/`cargo test`) vs. the outcome-affecting modules of `tracewake-core` (events, replay, checksum, scheduler, actions, agent, state). No code module is renamed; this adds enforcement.
4. INV motivating this ticket: `INV-017` (seedable, auditable randomness only — never wall-clock seeding) and `INV-018` (byte-identical deterministic replay). Restated: nondeterministic API use in outcome paths is a replay/desync hazard and must be banned mechanically, not by audit memory.
5. Deterministic-replay surface touched: this *is* the determinism-enforcement surface. The gate must fail closed (CI fails if a banned API appears outside an allowlisted file), and the allowlist must not become a blanket escape hatch — each exception carries a rationale and responsible layer. Clippy's project-wide `disallowed-types` cannot express "core outcome paths only," so the no-dep source-scan test enforces the module scoping that clippy cannot.

## Architecture Check

1. A two-layer gate (clippy `disallowed-types` for broad bans + a no-dep source-scan test for module-scoped precision) expresses the rule inside the pinned toolchain without adding a non-core tool dependency (`cargo-deny` is deferred to the §11 suggestions appendix, out of scope here). This converts a present-tense fact into a build-time invariant.
2. No backwards-compatibility shim: no blanket allowlist; the allowlist starts empty and any future entry is narrow and justified.

## Verification Layers

1. `INV-017` (no wall-clock/unseeded randomness) -> lint gate: `cargo clippy --workspace --all-targets -- -D warnings` fails on a banned type/method in outcome code.
2. `INV-018` (deterministic replay) -> source-scan conformance test: a no-dep `cargo test` scans outcome-affecting modules for banned API tokens and fails outside the allowlist.

## What to Change

### 1. Add workspace `clippy.toml`

Create `clippy.toml` with `disallowed-types` for `std::collections::HashMap`, `std::collections::HashSet`, `std::time::SystemTime`, `std::time::Instant`, and `disallowed-methods` for known RNG/process/thread entry points, each with a clear message pointing to the deterministic alternative.

### 2. No-dep source-scan conformance test

Add a `tracewake-core` test that scans the outcome-affecting module sources for banned tokens (`HashMap`, `HashSet`, `SystemTime`, `Instant`, `rand::`, `thread::spawn`, network/filesystem entry points) and fails on any match outside a small allowlist constant (initially empty), with rationale required per allowlist entry.

### 3. Document the allowlist

Keep the allowlist inline in the test with a per-entry rationale + responsible layer; it must be empty for `tracewake-core` outcome mutation/replay/scheduler/action paths.

## Files to Touch

- `clippy.toml` (new)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — `nondeterminism_api_gate` source-scan test)

## Out of Scope

- `cargo-deny` / `deny.toml` (§11 suggestions appendix; not a mandatory gate in this spec).
- Banning these APIs in non-outcome presentation-only code where a deterministic wrapper is unnecessary — scope is outcome-affecting paths.
- Refactoring any existing code (none uses the banned APIs).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo clippy --workspace --all-targets -- -D warnings` passes at this commit (allowlist empty, no banned API present) and would fail if a banned type were introduced into outcome code.
2. `nondeterminism_api_gate` source-scan test passes and fails when a banned token is introduced outside the allowlist.
3. `cargo test --workspace` passes.

### Invariants

1. Outcome-affecting code contains no nondeterministic API outside a narrow, rationale-bearing allowlist (`INV-017`).
2. The allowlist is empty for `tracewake-core` outcome paths (`INV-018`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — `nondeterminism_api_gate` (no-dep source scan).

### Commands

1. `cargo clippy --workspace --all-targets -- -D warnings`
2. `cargo test -p tracewake-core --test anti_regression_guards`
3. `cargo test --workspace`
