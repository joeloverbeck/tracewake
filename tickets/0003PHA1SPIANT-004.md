# 0003PHA1SPIANT-004: Total replay/checksum coverage over authoritative state

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` (`checksum.rs` coverage declaration; `state.rs` coverage registration; replay-report typed divergence detail; conformance test)
**Deps**: 0003PHA1SPIANT-001

## Problem

Physical and agent checksums are hand-written over state fields (`checksum.rs:47-152` physical, `:189-292` agent) and proven stable / insertion-order-independent (`checksum.rs:443-474`). But coverage is not total: a new field added to `PhysicalState`/`AgentState` can be omitted from the checksum with no compiler error, silently breaking replay equality while every existing test still passes (no fixture exercises the new field). Spec `0003` §5.3 / SPINE-AC-004 require a coverage declaration tied to the state field list and a conformance test that fails when an authoritative field lacks checksum coverage.

## Assumption Reassessment (2026-06-08)

1. Replay rebuild computes physical/epistemic/agent checksums (`crates/tracewake-core/src/replay/rebuild.rs:45-150`); checksum code covers existing physical fields (`crates/tracewake-core/src/checksum.rs:47-152`) and agent fields (`:189-292`); stability/order-independence tests exist (`:443-474`). The authoritative state field lists live in `crates/tracewake-core/src/state.rs` (`PhysicalState`, `AgentState`).
2. Spec `specs/0003_PHASE_1_SPINE_ANTI_CONTAMINATION_AND_STRUCTURAL_HARDENING_SPEC.md` §7 SPINE-AC-004 mandates: a checksum-coverage declaration next to state or checksum code; a conformance test failing if a `PhysicalState`/`AgentState` field lacks a coverage entry; new outcome-affecting structs either implement canonical checksum serialization or are explicitly marked non-authoritative with a reason; and replay reports carrying enough typed detail to identify the first divergent event and field family.
3. Boundary under audit: the `state.rs` field list ↔ `checksum.rs` coverage ↔ `replay/report.rs` divergence reporting contract. **Depends on 0003PHA1SPIANT-001**: that ticket reseals `PhysicalState`/`AgentState` (private fields + accessors), so this ticket's coverage declaration and field-enumeration must sit against the post-seal shape and use the read accessors. Shared files `state.rs`/`checksum.rs` with 001.
4. INV motivating this ticket: `INV-017`/`INV-018` (seedable/auditable randomness; byte-identical deterministic replay) and `INV-092` (checksum/state integrity). Restated: every authoritative field must enter the canonical checksum, or replay equality is not actually guaranteed — coverage must be mechanically total, not hand-maintained.
5. Deterministic-replay surface touched directly: the physical/epistemic/agent checksum functions and the replay report. The coverage test must fail closed (a missing field is a hard failure), and the field-family divergence detail must be deterministic. This ticket adds no nondeterministic input; it adds a coverage gate and richer typed divergence reporting.
6. Schema extension (coverage manifest over an existing schema): adds a coverage declaration enumerating `PhysicalState`/`AgentState` fields, consumed by the conformance test. Extension is **additive** to checksum/report code; it changes no checksum *value* for current state (existing fields keep their canonical serialization) — it only guarantees future fields cannot escape.

## Architecture Check

1. A declarative coverage manifest + a source/field-list conformance test converts "did the author add the new field to the checksum?" from an invisible omission into a build failure, closing the highest-risk replay drift vector. Typed first-divergence/field-family reporting makes replay failures debuggable rather than opaque. A field-level mutation test per major state family proves the checksum actually responds to each family.
2. No backwards-compatibility shim: no default "uncovered fields are ignored" path — an uncovered authoritative field fails the build; a deliberately-excluded field must be explicitly marked non-authoritative with a recorded reason.

## Verification Layers

1. `INV-018` (deterministic replay) -> conformance test: a coverage test fails if any `PhysicalState`/`AgentState` field lacks a checksum-coverage entry.
2. `INV-092` (state/checksum integrity) -> replay/golden-fixture check: a field-level mutation in each major state family (actor location; door/container; item/food; workplace assignment; need; routine/intention; decision trace; stuck diagnostic) changes the checksum.
3. `INV-018` (debuggable replay) -> manual review + test: replay report identifies the first divergent event and field family with typed detail.

## What to Change

### 1. Checksum coverage declaration

Add a coverage declaration (a registered field list / macro / table) next to `state.rs` definitions or `checksum.rs`, enumerating every authoritative `PhysicalState`/`AgentState` field that must be checksummed, using the post-seal read accessors from 0003PHA1SPIANT-001.

### 2. Coverage conformance test

Add a test that fails if a state field exists without a coverage entry (field-list parse or exhaustive-match pattern), and documents the synthetic new-field-omission pattern.

### 3. Typed replay divergence detail

Extend `replay/report.rs` so a checksum divergence reports the first divergent event and the field family, typed (not display-string).

### 4. Per-family checksum-response tests

Add tests mutating one field in each major state family and asserting the checksum changes.

## Files to Touch

- `crates/tracewake-core/src/checksum.rs` (modify)
- `crates/tracewake-core/src/state.rs` (modify — coverage registration; file also resealed by 0003PHA1SPIANT-001)
- `crates/tracewake-core/src/replay/report.rs` (modify — typed divergence detail)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — `checksum_coverage_is_total_for_authoritative_state` + per-family response tests)

## Out of Scope

- The state reseal itself (0003PHA1SPIANT-001).
- Event-schema version handling (0003PHA1SPIANT-002) and event-kind metadata (0003PHA1SPIANT-003).
- Any change to existing checksum *values* for current fields.

## Acceptance Criteria

### Tests That Must Pass

1. `checksum_coverage_is_total_for_authoritative_state` — fails if a `PhysicalState`/`AgentState` field lacks a coverage entry.
2. Per-family checksum-response tests — a field mutation in each major state family changes the checksum.
3. `cargo test --workspace` passes with existing checksum stability/order-independence tests unchanged.

### Invariants

1. Every authoritative state field is covered by the canonical checksum (`INV-018`, `INV-092`).
2. Replay divergence is reported with typed first-divergent-event + field-family detail (`INV-018`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — coverage-totality + per-family checksum-response tests.
2. `crates/tracewake-core/src/replay/report.rs` (`#[cfg(test)]`) — typed divergence-detail unit test.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo test --workspace`
