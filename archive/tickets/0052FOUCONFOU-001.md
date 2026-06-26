# 0052FOUCONFOU-001: Structural keystone — opaque core-owned session, closed command family, sealed receipts (additive)

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` runtime/session command + receipt surface (new `runtime/command.rs`, `runtime/receipt.rs`), opaque loader→runtime production constructor seam
**Deps**: None

## Problem

Spec 0052 §4.1 establishes the cycle-breaking keystone: 0051 delivered a core-owned `LoadedWorldRuntime` with private aggregate fields but left a correct constructor *beside* an injectable one, a closed command token (`OneTickWait`) *beside* open public methods, and raw aggregate getters that let the TUI keep authoring transaction choreography (driver §6.1). Findings F4-01, F4-02, F4-05, F4-06, and F4-07 all ride on one structural change: reduce the runtime's public surface to (a) an opaque loader-to-runtime production constructor consumable only by the production bootstrap, (b) a closed set of typed commands, and (c) immutable sealed typed receipts.

This ticket builds that new path **additively, kept unwired** — the closed `RuntimeCommand` family (expanded beyond `OneTickWait`), the sealed embodied/debug/rejection receipt types, and the opaque core-owned bootstrap/seed product — alongside the existing injectable `from_initial_state` / raw-getter API. The bypass deletions and caller flips are the *subsequent* closure-order tickets (002 F4-01, 003 F4-02, 004 F4-07); this ticket must leave the old path intact and compiling so those flips are reviewable as atomic cutovers (driver closure order step 1; spec §9 "writing tests around the present public ownership shape before replacing it would harden the wrong API").

## Assumption Reassessment (2026-06-25)

1. `crates/tracewake-core/src/runtime/session.rs` currently defines `pub struct RuntimeCommand` (line 46), `enum RuntimeCommandKind { OneTickWait { origin: WorldAdvanceOrigin } }` (line 51, single variant), a `pub(crate) fn one_tick_wait` constructor (line 354), and the public injectable `pub struct RuntimeInitialState { … pub scheduler: DeterministicScheduler … }` (lines 20–29) consumed by `LoadedWorldRuntime::from_initial_state`. The runtime module is registered via `crates/tracewake-core/src/runtime/mod.rs` (`mod session;`); new sibling modules register here.
2. Spec authority: this is archived spec `archive/specs/0052_FOUNDATIONAL_CONFORMANCE_FOURTH_HARDENING_…_HARDENING_SPEC.md` §4.1 (keystone) and §9 (closure order). Sibling precedent `archive/tickets/0051FOUCONTHI-001.md` ("Core-owned loaded-world runtime/session keystone") established the analogous additive foundation for the third pass; this is the fourth-pass successor that must finish closing the authority *class*, not just instances.
3. Cross-artifact boundary under audit: the `tracewake-core` ⇄ `tracewake-content` integration seam (the loader→runtime handoff) and the `tracewake-core` public client surface consumed by `tracewake-tui`. Core must not depend on content or tui (`docs/1-architecture/01_AUTHORITY_BOUNDARIES_RUST_WORKSPACE_AND_DEPENDENCY_RULES.md`); the opaque bootstrap product is owned by core and produced from content-loaded state, preserving the one-way direction.
4. Motivating invariant: INV-069 ("The TUI must not implement simulation rules"). The keystone's purpose is to make client-side simulation authority *unrepresentable* in the type system — `#[non_exhaustive]` alone is not an authority boundary (§4.1); private fields, crate-private constructors, opaque loader exports, unexported authority tokens, closed command constructors, and sealed receipts are the controlling mechanism.
5. This ticket builds the *inputs* to the deterministic-replay (INV-018) and no-leak / actor-knowledge (INV-024, INV-067, INV-093) enforcement surfaces that sibling tickets implement: the sealed embodied receipt (enforced non-leaking by 004 F4-07 and 008 F4-06), the debug receipt behind the existing `debug_capability` boundary (enforced by 008), and the opaque replay seed/snapshot product (consumed by 005 F4-03). As substrate it introduces no leakage or nondeterminism path: the receipt types carry no exact-time field on the embodied variant, the debug variant is constructible only through `debug_capability`, and the seed product is a deterministic function of accepted initial configuration. The named enforcement surfaces are the sibling tickets, not this one.

## Architecture Check

1. Building the closed command/receipt/opaque-constructor path additively first — then flipping callers in dedicated atomic-cutover tickets — is cleaner than a single mega-diff because each subsequent flip (002/003/004) stays a reviewable atomic diff while the tree compiles throughout; it also lets the keystone land and be reviewed for type-level soundness before any production behavior moves. The alternative (mutating the public API in place) would either break the tree mid-series or require exactly the temporary public alias the spec forbids.
2. No backwards-compatibility aliasing or shims are introduced as durable surface. A temporary internal adapter may exist only long enough to migrate core tests and MUST be removed before closeout (§4.1); it is `pub(crate)`/test-only, never a public alias. The injectable `from_initial_state` path is *retained unchanged* in this ticket (it is deleted in 002), not aliased.

## Verification Layers

1. INV-069 (TUI not authority) -> codebase grep-proof: the new `RuntimeCommand` constructors and sealed receipt types expose no public field granting aggregate/scheduler/sequence authority; `RuntimeInitialState` is still present but no *new* injectable path is added.
2. INV-018 (deterministic replay) -> manual review + unit test: the opaque seed/snapshot product reconstructs identical initial aggregates from accepted configuration (additive; no replay semantics changed yet).
3. INV-067/INV-093 (actor-known / leakage) -> codebase grep-proof: the embodied receipt type carries no exact tick/frontier/stop-reason field; the debug receipt is constructible only via `debug_capability` (invariants alignment check against §4.1/§4.7).
4. Cross-artifact: the content→core opaque bootstrap export compiles with core not depending on content (dependency-direction grep-proof against `Cargo.toml` members).

## What to Change

### 1. Closed command family (`runtime/command.rs`, new)

Introduce a closed `RuntimeCommand` family with variants/opaque constructors covering the semantic player intents the seam needs: semantic action proposal (actor-filtered target selection only), one-tick wait, continue/advance-until, controller bind/unbind, debug-authorized no-human execution, replay/recovery, and view/debug queries. Constructors take semantic intent only — never a scheduler, a proposal sequence, or a world-advance boolean. Keep the existing `OneTickWait`/`one_tick_wait` behavior reachable through the new family. Do not yet route the TUI through it (that is 002/003).

### 2. Sealed receipts (`runtime/receipt.rs`, new)

Introduce immutable sealed typed receipts: an embodied receipt/view (actor-known qualitative consequences + actor-legible stop info with provenance, **no exact tick/frontier/stop-reason**), a debug receipt/view (exact ticks/frontiers/event IDs/stop reasons) constructible only through the existing `debug_capability` boundary, and a typed rejection. Fields private; constructors crate-private. These are the product classes commands will return in 003/004/008.

### 3. Opaque production bootstrap + replay seed (`runtime/session.rs`, `tracewake-content/src/load.rs`)

Add a core-owned opaque bootstrap/export type and a production constructor consuming it; add the content-side opaque export produced from `LoadedFixture` (built additively beside `into_runtime_initial_state`, which stays until 002). Add an opaque core-owned replay seed/snapshot product so a future TUI need not retain mutable initial aggregates. All additive and unwired this ticket.

## Files to Touch

- `crates/tracewake-core/src/runtime/command.rs` (new)
- `crates/tracewake-core/src/runtime/receipt.rs` (new)
- `crates/tracewake-core/src/runtime/mod.rs` (modify — register `command`, `receipt`)
- `crates/tracewake-core/src/runtime/session.rs` (modify — opaque production constructor, replay seed product, command/receipt wiring; injectable path retained)
- `crates/tracewake-content/src/load.rs` (modify — additive opaque bootstrap export beside `into_runtime_initial_state`)

## Out of Scope

- Removing `from_initial_state` / `RuntimeInitialState` / raw aggregate getters / the `advance_world_after_acceptance` boolean / `assign_proposal_sequence` (002, 003, 004 — atomic cutovers).
- Routing `TuiApp` through the closed command family (002/003).
- Replay authority reconstruction semantics (005), real declared process (006), census (007), embodied/debug normal-output repair (008).
- Any negative-fixture / conformance-lane / mutation work (009/010/011).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo build -p tracewake-core --all-targets` — the additive command/receipt/bootstrap surface compiles with the existing injectable path retained.
2. A new `tracewake-core` unit test constructs a `RuntimeCommand` for one-tick wait through the closed family and asserts the same committed effect as the existing `one_tick_wait` path (behavior preserved, not yet TUI-wired).
3. A new unit test asserts the opaque replay seed/snapshot reconstructs byte-identical initial aggregates from accepted configuration.
4. `cargo test --workspace` — no regression in the retained path.

### Invariants

1. No public constructor, field, or method introduced this ticket grants a client scheduler injection, aggregate handle, proposal-sequence allocation, or world-advance boolean (INV-069).
2. The embodied receipt type has no exact tick/frontier/stop-reason field; the debug receipt is constructible only via `debug_capability` (INV-067/INV-093).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/runtime/session.rs` (or `runtime/command.rs` test module) — closed-family one-tick-wait equivalence to the existing path.
2. `crates/tracewake-core/src/runtime/receipt.rs` test module — embodied-receipt field-absence and debug-receipt capability-gating compile/behavior assertions.

### Commands

1. `cargo test -p tracewake-core runtime`
2. `cargo build -p tracewake-core --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-25

Implemented the additive keystone surface without deleting the old injectable
runtime path reserved for later tickets:

- Added `runtime/command.rs` with a closed `RuntimeCommand` family covering
  proposal submission, one-tick wait, continue, controller bind/detach,
  debug-authorized no-human execution, replay-seed rebuild, perception refresh,
  embodied view, and debug view requests. Constructors take semantic inputs and
  no scheduler, proposal sequence, or world-advance boolean.
- Added `runtime/receipt.rs` with immutable typed receipts, including an
  embodied receipt that exposes only actor-visible summary/provenance and a
  debug receipt gated by `DebugCapability`.
- Added `LoadedWorldBootstrap` and `RuntimeReplaySeed`, plus
  `LoadedFixture::into_runtime_bootstrap`, so loaded content can hand core a
  scheduler-free bootstrap product while `LoadedWorldRuntime::from_bootstrap`
  derives scheduler authority inside core.
- Added focused unit/loader tests for closed one-tick command behavior,
  replay-seed reconstruction, scheduler-free bootstrap derivation, and
  content-loader bootstrap handoff.
- Updated existing receipt matches and the workspace source-classification guard
  for the new runtime modules.

Deviations from the ticket plan:

- `RuntimeInitialState`, `from_initial_state`, raw getters,
  `assign_proposal_sequence`, and `advance_world_after_acceptance` remain
  intentionally present because this ticket is additive only; removal and caller
  cutover are owned by later 0052 tickets.

Verification:

- `cargo fmt --all --check` — passed after formatting.
- `cargo test -p tracewake-core runtime` — passed.
- `cargo test -p tracewake-content loaded_fixture_exports_scheduler_free_runtime_bootstrap` — passed.
- `cargo build -p tracewake-core --all-targets --locked` — passed.
- `cargo test -p tracewake-core --test anti_regression_guards workspace_source_classification_census_matches_production_tree` — passed after adding the new runtime modules to the source-classification census.
- `cargo test --workspace` — passed on the final tree.
