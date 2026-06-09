# 0004PHA1THIHAR-003: Compile-time no-direct-mutation enforcement + seal-bypass fixtures

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — seal-bypass negative fixtures (reuse the ticket-002 runner); `anti_regression_guards.rs` direct-apply guard reclassified to documented smoke
**Deps**: 0004PHA1THIHAR-001, 0004PHA1THIHAR-002

## Problem

Sealing the seed mutators (ticket 001) closes the public bypass, but the no-direct-mutation guarantee must be *proven* compile-time/capability-enforced, not merely asserted by a substring scanner. The current direct-apply guard `no_direct_apply_event_outside_event_replay_or_pipeline` (`anti_regression_guards.rs:889`) and `guard_001_no_direct_state_collection_insert_outside_event_application` (`:1364`) match by substring and are evadable by alias/re-export/macro (spec §6 F-010, §8 THIRD-AC-002). This ticket adds adversarial negative fixtures proving seed-mutator access, forged capability construction, and direct `apply_event` from a forbidden module all fail, and demotes the substring guards to documented smoke.

## Assumption Reassessment (2026-06-08)

<!-- Items 1-3 always required; 4-5 selected for an invariant-motivated, capability-enforcement ticket. -->

1. `WorldMutationCapability` / `AgentMutationCapability` have private fields and `pub(in crate::events)` minting (`crates/tracewake-core/src/events/mutation.rs:2`,`:7`); `apply_event` mints them internally (`events/apply.rs:111`, `:220`). The substring guards are `no_direct_apply_event_outside_event_replay_or_pipeline` (`anti_regression_guards.rs:889`) and `guard_001_no_direct_state_collection_insert_outside_event_application` (`:1364`).
2. The remediation is spec §8 `THIRD-AC-002` + the §9.1 direct-mutation/capability fixtures (reassessed this session).
3. Shared boundary under audit: this ticket *consumes* two upstream products — the seal from ticket 001 (the `external_crate_cannot_call_seed_mutators_after_load` fixture only compile-fails once the public mutators are gone) and the negative-fixture runner harness from ticket 002. Both are declared in `Deps`.
4. Motivating invariants (restated): `INV-009` — meaningful state changes require events; `INV-101` — actor-known context is sealed; `INV-104` — routines/needs do not dispatch directly; `INV-106` — validation failure feeds replanning without leakage. The mutation capability is the structural embodiment of all four.
5. Fail-closed / replay surface: the enforcement surface is the `events`-module-private mutation capability + the `apply.rs` mint sites. The fixtures confirm the boundary cannot be forged or bypassed; the runtime test confirms accepted actions append-before-apply. No weakening of leakage prevention or replay determinism — the change only adds proofs and a smoke-only annotation.

## Architecture Check

1. Capability-enforced + adversarially-fixtured proof is strictly stronger than substring scanning, which an alias, re-export, or macro can slip past. Demoting the scanner to documented smoke (rather than deleting it) keeps a cheap early-warning layer while making the negative fixtures the primary guarantee — matching the spec's "scanner is smoke-only" directive.
2. No backwards-compatibility shims: no alias path or wrapper is added around the mutation capability; the scanner is annotated, not replaced by a parallel mechanism.

## Verification Layers

1. `INV-009` / `INV-101` (no direct post-seed mutation) -> negative-fixture check (via ticket-002 runner): `external_crate_cannot_call_seed_mutators_after_load`, `external_crate_cannot_mutate_agent_state_seed_maps`, and `external_crate_cannot_forge_world_mutation_capability` fail to compile.
2. `INV-104` / no-direct-dispatch -> runtime/conformance check: `event_apply_remains_only_post_seed_mutation_path` proves accepted actions append before authoritative apply and no forbidden module invokes apply directly.
3. Scanner-smoke demotion -> codebase grep-proof: the demoted guards carry an in-code comment marking substring matching as smoke-only.

## What to Change

### 1. Seal-bypass negative fixtures (§9.1)

Add fixture crates (run by the ticket-002 harness): attempt `state.seed_items_mut().insert(...)`, `AgentState::seed_intentions_mut(...)` outside seed-builder scope, and construction of `WorldMutationCapability` / `AgentMutationCapability` from outside `crate::events`. Each expected to fail compilation.

### 2. Append-before-apply runtime proof

Add `event_apply_remains_only_post_seed_mutation_path` asserting accepted actions append to the log before authoritative apply, and that no non-`events`/`replay`/`pipeline` module calls `apply_event` directly.

### 3. Demote substring guards to smoke

Annotate `no_direct_apply_event_outside_event_replay_or_pipeline` and `guard_001_no_direct_state_collection_insert_outside_event_application` in `anti_regression_guards.rs` as smoke-only (substring/token matching), pointing to the negative fixtures as the primary enforcement.

## Files to Touch

- `tests/negative-fixtures/` (new — seal-bypass fixture crates; harness created by ticket 002)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — shared hub w/ 001, 004, 006: add append-before-apply runtime test; annotate the two direct-mutation guards as smoke-only)

## Out of Scope

- The seal of `seed_*_mut` itself (ticket 001) and the runner harness (ticket 002).
- Token-scanner walk/match hardening and `source_guard_*` fixtures (ticket 004).
- Banned-API clippy fixtures (ticket 002).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core` — the seal-bypass fixtures fail-as-expected and `event_apply_remains_only_post_seed_mutation_path` passes.
2. `cargo build --workspace --all-targets --locked && cargo test --workspace` — full pipeline green.
3. `grep -q 'smoke' crates/tracewake-core/tests/anti_regression_guards.rs` near the demoted guards — smoke annotation present.

### Invariants

1. No external module can construct a mutation capability or mutate authoritative state outside event application (compile-enforced).
2. The append-before-apply ordering holds for every accepted action.

## Test Plan

### New/Modified Tests

1. `tests/negative-fixtures/*` — seal-bypass + forged-capability compile-fail fixtures; rationale: prove the capability boundary is unforgeable.
2. `crates/tracewake-core/tests/anti_regression_guards.rs` — `event_apply_remains_only_post_seed_mutation_path` runtime proof + smoke annotations; rationale: replace assumed substring coverage with a runtime + compile proof.

### Commands

1. `cargo test -p tracewake-core`
2. `cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-09

What changed:
- Extended the negative fixture runner so fixtures can assert either Clippy lint categories or Rust compile-error fragments.
- Added seal-bypass fixtures proving external crates cannot call removed `PhysicalState` / `AgentState` seed mutators and cannot reach or forge mutation capabilities.
- Added `event_apply_remains_only_post_seed_mutation_path`, which ties the no-direct-apply source scan to the append-before-apply runtime proof.
- Marked the direct-apply and direct-state-write source scans as smoke-only, with capability privacy and compile-fail fixtures documented as the primary enforcement layer.

Deviations from original plan:
- The forged-capability fixture fails because `tracewake_core::events::mutation` itself is private, before reaching private-field construction. This is a stronger live boundary than the ticket's original field-private expectation.
- A direct external `apply_event` compile-fail fixture was not added because `apply_event` is intentionally public in the current core API and is used by replay/pipeline proof surfaces. The direct-apply boundary remains enforced by the smoke source scan plus append-before-apply runtime proof.

Verification results:
- `cargo test -p tracewake-core --test negative_fixture_runner` — passed.
- `cargo test -p tracewake-core --test anti_regression_guards event_apply_remains_only_post_seed_mutation_path` — passed.
- `cargo test -p tracewake-core` — passed.
- `cargo fmt --all --check` — passed.
- `rg -n "Smoke-only source scan|smoke-only" crates/tracewake-core/tests/anti_regression_guards.rs` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo test --workspace` — passed.
