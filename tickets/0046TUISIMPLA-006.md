# 0046TUISIMPLA-006: Baseline census — 15 registered action capabilities

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — new census module (`crates/tracewake-tui/tests/parity/census_actions.rs`), registry-module registration, and focused per-action goldens + any gap-filling fixtures in `tracewake-content`. No `tracewake-core` change.
**Deps**: 0046TUISIMPLA-003, 0046TUISIMPLA-004, 0046TUISIMPLA-005

## Problem

Spec 0046 §4.3 `PAR-009` (action half). Every registered action must map to a registry entry with
coverage; the baseline matrix must show zero uncovered entries at acceptance. This ticket enumerates
the **15** action IDs the registry can hold and binds each to a capability entry, fixture, typed
witness, rendered witness, and (for epistemic actions) an anti-leak witness, so the runner (004) passes
with every `ActionRegistry::definitions()` entry dispositioned.

## Assumption Reassessment (2026-06-22)

1. Verified against code at baseline `1145e109`: the base action set registered across
   `crates/tracewake-core/src/actions/registry.rs:107`–`200` is **15** IDs —
   `move`, `open`, `close` (`register_phase1_movement_open_close`); `take`, `place`
   (`register_phase1_take_place`); `look`, `inspect_place`, `inspect_entity` (query-only), `wait`
   (`register_phase1_inspect_wait`); `check_container`, `truthful_accuse_probe` (query-only)
   (`register_phase2a_epistemics`); `sleep`, `eat`, `work_block`, `continue_routine` (the Phase-3A
   registrars). Re-derive the count from `ActionRegistry::definitions()` at implementation, do not
   hardcode 15 — a registrar added later must surface as an uncovered entry, not be silently missed.
2. Verified against spec 0046 §4.3 (`PAR-009`) and §6: reuse the existing 58-fixture `GoldenFixture`
   substrate (`fixtures::by_id`/`all`) and add focused fixtures only where the census shows a gap;
   query-only actions (`look`/`inspect_place`/`inspect_entity`/`truthful_accuse_probe`) classify as
   actor-observable state/consequence, not necessarily `semantic_actions` submissions — disposition
   each by its actual capability class, not by name.
3. Shared boundary under audit: `ActionRegistry::definitions()` ↔ the capability registry. The runner
   (004) fails if any definition lacks a disposition + fixture, so this ticket's completeness is the
   gate. This ticket `(modify)`s `parity/mod.rs` (created by 003) and reuses the 005 scenario harness.
4. Invariant restated (`PAR-009` motivation): `INV-094` possession parity is tested (each entry binds a
   possessed viewer and proves binding changes input only); `INV-091` no-human tests are mandatory
   (replay/no-human flags carry where doctrine applies). Actions stay data-driven (`INV-069`).
5. Enforcement surface touched (actor-knowledge filtering): the epistemic actions
   (`check_container`, `truthful_accuse_probe`) require paired anti-leak witnesses — confirm an
   unknown/stale/unobserved case stays absent or disabled with an actor-safe why-not and surfaces no
   hidden/validator-only truth (`INV-093`/`INV-099`–`101`). Non-epistemic actions assert observable
   consequence only.
6. Schema extension (additive): adding gap-filling fixtures extends the content fixture set — name the
   consumers (`tracewake_content::fixtures::all`/`by_id`) and keep it **additive-only** (new fixture
   files registered in `crates/tracewake-content/src/fixtures/mod.rs`, no edit to existing fixtures'
   shape or IDs). Registry capability entries are additive test-side data, not a production schema.

## Architecture Check

1. Deriving the action set structurally from `ActionRegistry::definitions()` (rather than hand-listing)
   is cleaner and self-maintaining: a future registrar's action shows up as an uncovered entry until
   dispositioned, exactly the drift the contract closes. Reusing existing fixtures where they already
   exercise an action keeps the diff focused on witnesses + entries.
2. No backwards-compatibility aliasing/shims: no fixture is duplicated or aliased; gap fixtures are new
   and additive.

## Verification Layers

1. `PAR-009`/`INV-094` (per-action coverage) → runner pass: every `ActionRegistry::definitions()` entry
   has a registry disposition + fixture; the runner reports zero uncovered action entries.
2. `INV-093` (epistemic anti-leak) → paired negatives for `check_container` / `truthful_accuse_probe`:
   unknown/stale case stays absent/disabled with an actor-safe why-not.
3. `INV-018` (determinism) → golden check: each action's rendered witness matches its checked-in
   byte-stable golden; coverage report stays deterministic.

## What to Change

### 1. Action capability entries (`crates/tracewake-tui/tests/parity/census_actions.rs`)

Add a registry entry per action ID (derived from `definitions()`), each with capability class, surface
disposition + rationale, fixture ID(s), possessed viewer + setup op, typed witness, rendered
witness/golden, and anti-leak fixtures for the epistemic actions. Register the module in `parity/mod.rs`
so `registry()` includes these entries.

### 2. Focused goldens + gap fixtures

Add a checked-in golden per action through the 005 harness. Where no existing fixture exercises an
action, add a focused `GoldenFixture` in `tracewake-content` and register it in `fixtures/mod.rs`
(`all`/`by_id`).

## Files to Touch

- `crates/tracewake-tui/tests/parity/census_actions.rs` (new)
- `crates/tracewake-tui/tests/parity/mod.rs` (modify — file created by 0046TUISIMPLA-003)
- `crates/tracewake-tui/tests/goldens/` (modify — per-action golden files; dir created by 0046TUISIMPLA-005)
- `crates/tracewake-content/src/fixtures/` (new — gap-filling fixtures, only where the census shows a gap)
- `crates/tracewake-content/src/fixtures/mod.rs` (modify — register any new fixtures in `all`/`by_id`)

## Out of Scope

- Non-action capability families (epistemic filtering, no-human autonomy, needs/routines,
  notebook/leads, rejection/why-not, debug quarantine) — ticket 007 (shares `parity/mod.rs` +
  `fixtures/mod.rs`).
- Runner/harness logic (003/004/005). CI lane (008). Acceptance assembly (012).

## Acceptance Criteria

### Tests That Must Pass

1. Member matrix — every action ID from `ActionRegistry::definitions()` (15 at decomposition; re-count
   at implementation) has a registry entry with fixture + typed + rendered witness; epistemic actions
   (`check_container`, `truthful_accuse_probe`) carry an anti-leak witness. Runner reports zero
   uncovered action entries. No deferred/excluded action without a recorded justification.
2. Each action's rendered witness matches its checked-in golden; paired epistemic negatives pass.
3. `cargo test -p tracewake-tui --test playable_capability_parity` and the four gates pass; any new
   fixture validates under the existing content schema/fixture checks.

### Invariants

1. No registered action lacks a capability disposition + fixture coverage (runner-enforced).
2. Gap fixtures are additive-only; no existing fixture ID/shape is altered.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/playable_capability_parity.rs` — action entries flow through the runner +
   harness; per-action goldens + epistemic anti-leak negatives.
2. `crates/tracewake-content` fixture tests — any new gap fixture is covered by the existing fixture
   validation suite.

### Commands

1. `cargo test -p tracewake-tui --test playable_capability_parity`
2. `cargo test -p tracewake-content`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
