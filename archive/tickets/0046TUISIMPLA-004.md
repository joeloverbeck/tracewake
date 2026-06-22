# 0046TUISIMPLA-004: Conformance runner + deterministic coverage report

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new `tracewake-tui` test-support runner (`crates/tracewake-tui/tests/parity/runner.rs`), extends the registry module and its test target; no production-crate change.
**Deps**: 0046TUISIMPLA-003

## Problem

Spec 0046 §4.3 `PAR-008`. The capability registry (ticket 003) is inert data until something iterates
it and fails closed on gaps. This ticket adds the generic conformance runner that validates the
registry and emits a deterministic coverage report suitable for the acceptance artifact. Without it,
a registry entry can omit a fixture, a witness, or an anti-leak case and nothing complains; a
registered action can have no capability disposition; a "playable" capability can render empty at its
declared observation point — all silently.

## Assumption Reassessment (2026-06-22)

1. Verified against code at baseline `1145e109`: the registry module
   `crates/tracewake-tui/tests/parity/mod.rs` and its consuming target
   `crates/tracewake-tui/tests/playable_capability_parity.rs` are created by ticket 003 (in-batch
   create-then-modify dependency — this ticket `(modify)`s both). `ActionRegistry::definitions()`
   (`crates/tracewake-core/src/actions/registry.rs:99`) and `fixtures::by_id`
   (`crates/tracewake-content/src/fixtures/mod.rs:445`) are the cross-references the runner checks
   entries against.
2. Verified against spec 0046 §4.3 (`PAR-008`): the runner's required failure conditions are
   enumerated in the spec and map to registry fields defined in ticket 003. The report contents
   (capability count, class, fixture IDs, typed witness, rendered witness, negative witness,
   replay/no-human status, pass/fail) feed the acceptance artifact per `PAR-DOC-006` (ticket 011).
3. Shared boundary under audit: the registry-schema ↔ runner contract. The runner consumes the closed
   `capability_class`/`surface_disposition` enums from ticket 003 with exhaustive matches (no `_`),
   mirroring the Hop-2 discipline so a new class forces a runner decision.
4. Invariant restated (`PAR-008` motivation): `INV-066` every runnable phase has a TUI/view-model
   gate; `INV-095` TUI/view-model tests are acceptance tests. The runner makes the gate complete
   *per capability* rather than per remembered test — a missing entry/witness is an acceptance failure.
5. Enforcement surface touched (fail-closed validation): the runner is the fail-closed gate over the
   parity contract. Name and confirm each failing condition is blocking and deterministic — duplicate/
   empty/non-deterministically-ordered keys; missing fixture/typed/rendered witness; a fixture ID absent
   from `fixtures::by_id`; an epistemic capability lacking an anti-leak witness; an actionable capability
   whose declared action is absent from the actor's `semantic_actions` under its positive scenario; a
   registered `ActionRegistry::definitions` entry with no capability disposition + fixture coverage; an
   embodied-marked capability rendering empty/unchanged at its observation point; a debug-only capability
   classified ordinary-playable; a future active pack with no profile or uncovered entries. The report is
   emitted in deterministic (key-sorted) order so it is byte-stable across runs (no wall-clock,
   `INV-017`/`INV-018` posture for the evidence artifact).

## Architecture Check

1. A single generic runner iterating the registry is cleaner than per-capability assertion fns: it
   centralizes the fail-closed conditions, keeps the report format in one place, and turns "add a
   capability" into "add a registry entry" (the runner enforces the rest). Cross-checking against
   `ActionRegistry::definitions()` catches a registered action with no disposition — the gap a
   per-test approach cannot see.
2. No backwards-compatibility aliasing/shims: new code; no legacy coverage-checking path retained.

## Verification Layers

1. `PAR-008`/`INV-095` (fail-closed coverage) → executed negatives: synthetic registry entries that
   each trip one failure condition (missing fixture, missing anti-leak, unmapped action, empty render)
   must fail the runner; a complete entry passes.
2. `PAR-008` (deterministic report) → replay/determinism check: the coverage report is byte-identical
   across two runs (assert in `playable_capability_parity.rs`).
3. `INV-069` (data-driven) → grep-proof: the runner contains no action-kind switch that duplicates
   simulation rules; it only inspects registry data + `semantic_actions`/`definitions`.

## What to Change

### 1. Runner (`crates/tracewake-tui/tests/parity/runner.rs`)

Add `run_conformance(entries: &[CapabilityEntry]) -> CoverageReport` implementing every `PAR-008`
failure condition (Assumption 5) as a hard error, with exhaustive matches over the registry's closed
enums. Cross-check every `ActionRegistry::definitions()` entry has a disposition + fixture coverage,
and every referenced fixture ID resolves via `fixtures::by_id`.

### 2. Deterministic coverage report

Define `CoverageReport` (per-capability rows: key, class, fixture IDs, typed/rendered/negative witness
presence, replay/no-human status, pass/fail) with a deterministic key-sorted serialization for the
acceptance artifact.

### 3. Wire into the test target

In `crates/tracewake-tui/tests/playable_capability_parity.rs` (`mod parity;` adds `pub mod runner;`),
call the runner over the registry and assert zero failures + byte-stable report across two runs.

## Files to Touch

- `crates/tracewake-tui/tests/parity/runner.rs` (new)
- `crates/tracewake-tui/tests/parity/mod.rs` (modify — file created by 0046TUISIMPLA-003)
- `crates/tracewake-tui/tests/playable_capability_parity.rs` (modify — file created by 0046TUISIMPLA-003)

## Out of Scope

- Populating the registry with the 15 actions / 6 families (`PAR-009`) — tickets 006/007.
- The real-pipeline scenario harness + goldens (`PAR-004`–`006`) — ticket 005.
- CI lane wiring (`PAR-011`) — ticket 008. Acceptance-artifact assembly — ticket 012.

## Acceptance Criteria

### Tests That Must Pass

1. Each `PAR-008` failure condition is exercised by a synthetic entry that fails the runner; a complete
   entry passes. The runner matches the registry's closed enums exhaustively (no `_`).
2. The coverage report is byte-identical across two consecutive runs (deterministic ordering).
3. `cargo test -p tracewake-tui --test playable_capability_parity` and the four gates pass.

### Invariants

1. The conformance gate fails closed: any missing entry/witness/fixture/disposition is a hard failure,
   never a warning.
2. The runner introduces no simulation rule; it only reads registry + `definitions()`/`semantic_actions`.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/playable_capability_parity.rs` — runner pass over the live registry +
   per-condition synthetic-failure negatives + report determinism assertion.

### Commands

1. `cargo test -p tracewake-tui --test playable_capability_parity`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-22

Added `crates/tracewake-tui/tests/parity/runner.rs` with
`run_conformance`, deterministic `CoverageReport` serialization, per-row
pass/fail status, hard failures for malformed registry data, fixture resolution
checks, live semantic-action presence checks through `TuiApp`, rendered-surface
witness checks, anti-leak requirements for epistemic/notebook-shaped entries,
and an embodied-render probe that uses the real TUI render path by default.
`crates/tracewake-tui/tests/parity/mod.rs` now exposes the runner module and
the seeded wait exemplar was corrected from the action id `wait` to the live
semantic action id `wait.1_tick`.

Extended `crates/tracewake-tui/tests/playable_capability_parity.rs` so the live
registry passes conformance and emits byte-identical reports across consecutive
runs. The test also exercises synthetic hard failures for duplicate keys,
missing fixtures, unknown fixtures, missing rendered witnesses, missing
anti-leak coverage, absent semantic actions, empty embodied render output, and
uncovered registered actions.

Verification:

- `cargo test -p tracewake-tui --test playable_capability_parity` — passed
- `cargo fmt --all --check` — passed
- `cargo clippy --workspace --all-targets -- -D warnings` — passed
- `cargo build --workspace --all-targets --locked` — passed
- `cargo test --workspace` — passed

Deviations:

- Full live registered-action zero-uncovered enforcement is exposed through
  `registered_action_coverage_failures` and proven with a synthetic missing
  action, but it is not yet run against every current `ActionRegistry`
  definition because tickets `0046TUISIMPLA-006` and `0046TUISIMPLA-007` own
  the full action/family census population.
