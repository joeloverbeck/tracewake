# 0004PHA1THIHAR-006: Add checksum field-to-registry parity guard

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` (`checksum.rs` single-source registry or parity check; `state.rs` field enumeration if a macro/derive is introduced)
**Deps**: None

## Problem

Checksum coverage is manually mirrored: `PHYSICAL_STATE_CHECKSUM_COVERAGE` / `AGENT_STATE_CHECKSUM_COVERAGE` (`checksum.rs`) list fields by hand against the `state.rs` field set, and the existing guard `checksum_coverage_is_total_for_authoritative_state` (`anti_regression_guards.rs:1315`) asserts current strings rather than comparing the actual authoritative fields to the registry and canonical emission. A new authoritative `PhysicalState`/`AgentState` field could be added without checksum coverage and silently break replay-equality totality (spec §6 F-004, §8 THIRD-AC-007).

## Assumption Reassessment (2026-06-08)

<!-- Items 1-3 always required; 4-5 selected for an invariant-motivated, deterministic-replay ticket. -->

1. In `crates/tracewake-core/src/checksum.rs`: `StateChecksumCoverage` (`:14`), `PHYSICAL_STATE_CHECKSUM_COVERAGE` (`:20`), `AGENT_STATE_CHECKSUM_COVERAGE` (`:58`), each emitting canonical checksum lines from ordered collections. Authoritative state fields are `pub(crate)` in `crates/tracewake-core/src/state.rs:123`–`:134` (and the agent block). The current guard is `checksum_coverage_is_total_for_authoritative_state` at `anti_regression_guards.rs:1315`.
2. The remediation is spec §8 `THIRD-AC-007` + the §9.4 checksum fixtures (`new_authoritative_field_without_checksum_registry_fails`, `new_authoritative_field_without_canonical_checksum_line_fails`), reassessed this session.
3. Shared boundary under audit: `state.rs` is also modified by ticket 001 — but ticket 001 reseals the *mutation API*, leaving the field *set* unchanged, so there is no logical conflict, only a mechanical merge on `state.rs`. `anti_regression_guards.rs` is the N-way hub (tickets 001, 003, 004); coordinate the merge.
4. Motivating invariants (restated): `INV-018` (deterministic replay is foundational), `INV-092` (deterministic replay is tested), `INV-105` (actor decision traces / diagnostics are authoritative). Checksum totality is what makes replay-equality meaningful — an uncovered field makes two divergent states hash equal.
5. Deterministic-replay surface: the enforcement surface is the canonical checksum emission over ordered collections in `checksum.rs`. The parity guard enumerates field names and compares them to the coverage registries and canonical-line labels — a deterministic structural comparison introducing no nondeterminism.

## Architecture Check

1. A single-source registry/macro that defines the authoritative fields and their checksum coverage together (or, if macros are rejected, a test that enumerates the field names and compares them to `PHYSICAL_STATE_CHECKSUM_COVERAGE` / `AGENT_STATE_CHECKSUM_COVERAGE` and the canonical emission labels) makes "add an authoritative field without coverage" a hard failure — strictly stronger than asserting today's string list, which goes stale the moment a field is added.
2. No backwards-compatibility shims: the manual coverage const is either replaced by the single source or kept and *checked* against the field set; no parallel aliased coverage table is introduced.

## Verification Layers

1. `INV-018` / `INV-092` (checksum totality) -> new test: `new_authoritative_field_without_checksum_registry_fails` (structural — a field absent from coverage fails).
2. Canonical-emission parity -> new test: `new_authoritative_field_without_canonical_checksum_line_fails`.
3. Replay equality preserved -> replay/golden-fixture check: `replay_rebuild_checksum_matches_original_after_no_human_day` (shared with ticket 007's replay surface) still holds.

## What to Change

### 1. Field-to-registry parity

Introduce a single-source registry/macro defining authoritative `PhysicalState`/`AgentState` fields together with their checksum coverage, OR a parity test that enumerates the field names and compares them to `PHYSICAL_STATE_CHECKSUM_COVERAGE`, `AGENT_STATE_CHECKSUM_COVERAGE`, and the canonical-line emission labels.

### 2. Upgrade the totality guard

Replace the string assertion in `checksum_coverage_is_total_for_authoritative_state` with the structural field-vs-registry comparison, so adding an authoritative field without checksum coverage fails the build.

## Files to Touch

- `crates/tracewake-core/src/checksum.rs` (modify — single-source registry/macro, if chosen)
- `crates/tracewake-core/src/state.rs` (modify — only if a field-enumeration macro/derive is introduced here; shared w/ ticket 001)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — shared hub w/ 001, 003, 004: upgrade `checksum_coverage_is_total_for_authoritative_state` to a structural parity check)

## Out of Scope

- Unsupported schema-version / stream-mismatch replay fixtures (ticket 007).
- The seed-mutation seal (ticket 001), which shares `state.rs` but changes the mutation API, not the field set.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core` — the parity guard fails when an authoritative field lacks checksum registry coverage or a canonical emission line.
2. `cargo test --workspace` — existing checksum/replay equality tests pass after the upgrade.
3. `cargo build --workspace --all-targets --locked` — single-source registry/macro (if introduced) compiles cleanly.

### Invariants

1. Every authoritative `PhysicalState`/`AgentState` field is represented in checksum coverage and canonical line emission.
2. Replay-equality totality cannot be silently broken by an uncovered new field.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — structural `checksum_coverage_is_total_for_authoritative_state` + `new_authoritative_field_without_*` fixtures; rationale: convert a string assertion into a field-set parity proof.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed on 2026-06-09.

Upgraded `checksum_coverage_is_total_for_authoritative_state` from a field-list assertion into a structural parity guard. The guard now:

- parses authoritative `PhysicalState` and `AgentState` fields from `state.rs`;
- compares those fields to `PHYSICAL_STATE_CHECKSUM_COVERAGE` and `AGENT_STATE_CHECKSUM_COVERAGE`;
- verifies every coverage `field_family` has a canonical checksum line prefix in `checksum.rs`;
- keeps checksum field-family metadata non-empty.

Added synthetic regression fixtures:

- `new_authoritative_field_without_checksum_registry_fails`;
- `new_authoritative_field_without_canonical_checksum_line_fails`.

Implementation note: no state macro or registry rewrite was needed; the parity test keeps the existing checksum registry shape and makes drift a hard test failure.

Verified with:

1. `cargo fmt --all --check`
2. `cargo test -p tracewake-core --test anti_regression_guards`
3. `cargo build --workspace --all-targets --locked`
4. `cargo clippy --workspace --all-targets -- -D warnings`
5. `cargo test --workspace`
