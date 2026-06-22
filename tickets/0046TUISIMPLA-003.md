# 0046TUISIMPLA-003: Test-side playable-capability registry

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new `tracewake-tui` integration-test support module
(`crates/tracewake-tui/tests/parity/mod.rs`) and its consuming test target
(`crates/tracewake-tui/tests/playable_capability_parity.rs`); no production-crate change.
**Deps**: None

## Problem

Spec 0046 §4.3 `PAR-007`. There is no structure binding each declared playable capability to its
surface disposition, fixture, typed witness, rendered witness, and anti-leak witness; capability
coverage today is "tests exist somewhere" with no enumerable contract. This ticket creates the
**test-side** registry: a declarative list of capability entries that downstream tickets (the runner
`PAR-008`, the goldens harness `PAR-004`–`006`, the census `PAR-009`) consume. Per Step-4 decision
**Q1 = (a)**, it lives as a TUI integration-test support module — `tracewake-tui` already depends on
both `tracewake-core` and `tracewake-content` and is where `TuiApp`/`render_*` are exercised, so the
registry can reference fixtures and the real render path without inverting any boundary.

## Assumption Reassessment (2026-06-22)

1. Verified against code at baseline `1145e109`: `crates/tracewake-tui/tests/` already holds
   integration tests (`tui_acceptance.rs`, `tui_seam_conformance.rs`, `embodied_flow.rs`,
   `adversarial_gates.rs`, `transcript_snapshot.rs`); the crate depends on `tracewake-core` +
   `tracewake-content`. `ActionRegistry::definitions()` (`crates/tracewake-core/src/actions/registry.rs:99`)
   yields `ActionDefinition { action_id, scope, effect }`; fixtures resolve via
   `tracewake_content::fixtures::by_id` (`crates/tracewake-content/src/fixtures/mod.rs:445`) and
   `::all` (`:174`). No existing capability-registry module — this is greenfield test-side code.
2. Verified against spec 0046 §4.3 (`PAR-007`) and §8 dependency constraint: the registry must not be
   authoritative simulation state, must not be consumed by cognition/scheduling/validation/event
   application, and must not make `tracewake-core` depend on content or TUI. A TUI test-support module
   satisfies all three (it is below the boundary, not in the kernel).
3. Shared boundary under audit: the capability-entry schema is the contract every downstream parity
   ticket binds to. This module is the create-then-modify hub — the runner (004), goldens harness
   (005), and census (006/007) all extend it; each declares `Deps` reaching this ticket.
4. Invariant restated (`PAR-007` motivation): `INV-069` actions stay data-driven pipeline entries (the
   registry is test metadata, never a TUI action-kind switch); `INV-078`/`INV-080` the kernel stays
   genre-agnostic and future packs own flavor — entries carry an ownership scope (base now, or a
   namespaced future pack) so per-pack capability profiles compose without core awareness.
5. Enforcement surface touched (substrate basis): the registry feeds the runner (`PAR-008`, ticket 004)
   that enforces anti-leak / actor-knowledge coverage; this ticket introduces the data model only.
   Confirm the schema carries an explicit `required negative/anti-leak witness` slot for epistemic
   capabilities and a debug-only/headless disposition field, so the later enforcement surface can fail
   closed; the registry itself reads no hidden/debug truth and stores no authoritative state.

## Architecture Check

1. A declarative `Vec<CapabilityEntry>` registration function in a test-support module is cleaner than
   scattering capability metadata across individual test fns: it makes coverage enumerable (the runner
   can iterate it), keeps the schema in one place for the census to fill, and stays entirely test-side
   so no production dependency direction is touched. Stable capability keys + deterministic ordering are
   baked into the schema so the coverage report is reproducible.
2. No backwards-compatibility aliasing/shims: this is new code; no legacy capability-tracking path is
   wrapped or retained.

## Verification Layers

1. `PAR-007`/`INV-069` (data-driven, test-side) → codebase grep-proof: the registry module is under
   `crates/tracewake-tui/tests/`, references no `tracewake_core` mutation/event-application API, and
   `tracewake-core` gains no dependency on content/TUI (Cargo manifest unchanged).
2. `PAR-007` schema completeness → compile + smoke test: a `CapabilityEntry` round-trips all required
   fields (key, ownership scope, capability class, surface disposition + rationale, fixture IDs,
   possessed viewer + setup/advance op, typed witness, rendered witness/golden, anti-leak fixtures,
   replay/no-human flags); the smoke test in `playable_capability_parity.rs` asserts the registration
   vec is non-empty and has unique, deterministically-ordered keys.
3. `INV-078`/`INV-080` (pack composability) → manual review: ownership scope distinguishes base from a
   namespaced future pack; no entry hard-codes genre flavor into the schema.

## What to Change

### 1. Capability-entry schema (`crates/tracewake-tui/tests/parity/mod.rs`)

Define `CapabilityEntry` with: a stable `key`; `ownership_scope` (base / namespaced future pack);
`capability_class` (semantic action / actor-observable state / actor-observable consequence /
notebook-record surface / debug-only infrastructure / headless infrastructure); `surface_disposition`
+ a `disposition_rationale`; `fixture_ids: Vec<&str>`; `viewer_actor` + setup/advance operation;
`typed_witness`; `rendered_witness` (or golden reference); `anti_leak_fixtures` (required for epistemic
classes); and `replay`/`no_human` evidence flags. Encode classes/dispositions as closed enums so
downstream matches stay exhaustive. Provide a `registry() -> Vec<CapabilityEntry>` returning entries in
deterministic (key-sorted) order — initially empty or a single exemplar; the census (006/007) fills it.

### 2. Consuming test target (`crates/tracewake-tui/tests/playable_capability_parity.rs`)

Declare `mod parity;` and add a smoke test asserting the registry compiles, keys are unique and sorted,
and `disposition_rationale` is non-empty for every entry. This gives the module a standalone
compilable + `cargo test`-runnable consumer.

## Files to Touch

- `crates/tracewake-tui/tests/parity/mod.rs` (new)
- `crates/tracewake-tui/tests/playable_capability_parity.rs` (new)

## Out of Scope

- The conformance runner + coverage report (`PAR-008`) — ticket `0046TUISIMPLA-004`.
- The real-pipeline scenario harness + goldens (`PAR-004`–`006`) — ticket `0046TUISIMPLA-005`.
- Populating entries for the 15 actions / 6 families (`PAR-009`) — tickets `0046TUISIMPLA-006`/`007`.
- Any production-crate change; any authoritative-state or cognition consumption of the registry.

## Acceptance Criteria

### Tests That Must Pass

1. The registry module compiles; `CapabilityEntry` carries every `PAR-007` field; capability class and
   surface disposition are closed enums.
2. The `playable_capability_parity.rs` smoke test passes: registration vec has unique, deterministically
   ordered keys and a non-empty rationale per entry.
3. Grep-proof: no `tracewake-core` Cargo manifest dependency on content/TUI was introduced; the module
   references no event-application/cognition API. The four gates pass.

### Invariants

1. The registry is test-side only and never becomes authoritative simulation state or cognition input.
2. `tracewake-core` remains dependency-free of content and TUI (one-way direction preserved).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/playable_capability_parity.rs` — registry smoke test (unique sorted keys,
   non-empty rationale, schema round-trip).

### Commands

1. `cargo test -p tracewake-tui --test playable_capability_parity`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
