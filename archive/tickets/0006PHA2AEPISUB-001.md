# 0006PHA2AEPISUB-001: Seal holder-known KnowledgeContext and capability-gate debug context

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` epistemics: `KnowledgeContext` field visibility sealed, debug-context construction gated behind `DebugCapability`; same-crate consumers migrated to accessors.
**Deps**: None

## Problem

`KnowledgeContext` is documented as a sealed, decision-facing holder-known context, but its authority-bearing fields are `pub` and therefore mutable after the hash/audit is computed (`crates/tracewake-core/src/epistemics/knowledge_context.rs:194-206`: `pub viewer_actor_id`, `pub bound_actor_id`, `pub mode`, `pub allowed_sources`, `pub forbidden_sources`, `pub perception_scope`, `pub belief_scope`, `pub observation_scope`, `pub projection_schema_version`, `pub debug_non_diegetic`). `permits_scope` trusts those mutable fields (`:327-335`), and `KnowledgeContext::debug` is a public constructor that mints a truth-revealing `ViewMode::Debug` context with `ScopeFilter::DebugAll` and requires no `DebugCapability` (`:310-325`). Any caller can rewrite `mode`/`viewer_actor_id` after sealing to make `beliefs_for_context` return another holder's private beliefs, or forge a debug context through ordinary public API. This makes cross-holder leakage and forged debug omniscience reachable through safe Rust, defeating the truth firewall the type claims to enforce (spec EPI-HARD-001).

## Assumption Reassessment (2026-06-09)

1. `KnowledgeContext` fields are `pub` and mutable after `seal()` computes the hash/audit: confirmed at `crates/tracewake-core/src/epistemics/knowledge_context.rs:194-206`; the integrity fields (`holder_known_context_id`, `holder_known_context_hash`, `provenance_entries`, `forbidden_truth_audit`, `status`) are already private (`:207-211`). `KnowledgeContext::debug` is `pub` with no capability arg (`:310-325`); `permits_scope` branches on `self.mode`/`self.viewer_actor_id` (`:327-335`).
2. Spec authority confirmed: `specs/0006_PHASE_2A_EPISTEMIC_SUBSTRATE_ALIGNMENT_AND_ANTI_CONTAMINATION_HARDENING_SPEC.md` §6 EPI-HARD-001; architecture requires a decision-facing holder-known context to be built and then sealed with provenance (`docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md:21-24`, `:31-61`, `:93-97`).
3. Shared boundary under audit: the `KnowledgeContext` public API consumed by the epistemic projection's filtered read APIs (`crates/tracewake-core/src/epistemics/projection.rs:124-180`, which read `context.viewer_actor_id`/`context.mode`), the embodied view-model projection builder (`crates/tracewake-core/src/projections.rs:47,75,288,436,501`), the accuse-probe knowledge precondition (`crates/tracewake-core/src/actions/defs/accuseprobe.rs:35`), and the knowledge-basis query (`crates/tracewake-core/src/epistemics/knowledge_basis.rs:11`). All live in `tracewake-core`, so accessor migration is intra-crate.
4. Constitutional invariants motivating this ticket: `INV-024` (no telepathy — information reaches actors only through modeled channels), `INV-002` (belief before truth), `INV-006` (possession transfers no world knowledge), `INV-068` (debug mode is visibly non-diegetic and must never be confused with embodied knowledge), and the truth-firewall set `INV-101` (actor-known context is sealed) and `INV-107` (debug omniscience is quarantined). A mutable-after-seal context with a free public debug constructor violates all of these.
5. Enforcement surface: this ticket hardens the actor-knowledge-filtering surface (`permits_scope` + the `_for_context` filters that depend on the context's `mode`/`viewer_actor_id`). The change strengthens epistemic-leakage prevention — sealing makes post-seal mutation unrepresentable — and does not touch deterministic replay: the canonical hash inputs (`canonical_hash_inputs`, `:269-287`) already cover every authority field, so privatizing the fields changes no serialized/replayed bytes.
6. Visibility/shape change (not a schema extension): the `KnowledgeContext` field set is unchanged; only field visibility moves from `pub` to private + read-only accessors, and `debug()` gains a `DebugCapability` parameter. Consumers are the four same-crate read sites in item 3; the change is breaking for any external `pub` field read/write and for `KnowledgeContext::debug()` callers, so each consumer migrates in this diff. No external crate currently reads these fields (verified: TUI/content read view-model and binding fields named similarly, not `KnowledgeContext` fields).
7. Removal blast radius of the old public surface (grep `crates`): genuine `KnowledgeContext`-field reads are `epistemics/projection.rs` (`context.viewer_actor_id`/`context.mode` at `:126,135,143,153,159,168,176`), `projections.rs` (`:47,75,288,436,501`), `accuseprobe.rs` (`:35` builds the context), `knowledge_basis.rs` (`:11`), and the in-module test at `projection.rs:373,380` (`KnowledgeContext::debug(...)`, `context.debug_non_diegetic`). Look-alike matches on `bound_actor_id`/`viewer_actor_id` in `controller.rs`, `app.rs`, `render.rs`, `view_models.rs`, `debug_reports.rs` are other structs (controller bindings, view models) and are out of scope.

## Architecture Check

1. Rust item-visibility is the correct structural lever: removing `pub` from proof-bearing fields makes illegal post-seal mutation a compile error rather than a convention guarded by comments (spec §4 "Parse, don't validate" / Rust-visibility rationale). Read-only accessors preserve every legitimate read while denying writes. Gating `debug()` on a `DebugCapability` (already a crate-private mint, `crates/tracewake-core/src/debug_capability.rs:27`) mirrors the existing debug-report quarantine pattern rather than inventing a parallel mechanism.
2. No backwards-compatibility aliasing or shims: the old `pub` fields are removed, not re-exported behind a deprecated alias; the public `debug()` constructor is replaced by a capability-gated form, not wrapped.

## Verification Layers

1. `INV-024` / `INV-101` (sealed, no post-seal mutation) -> codebase grep-proof: no `pub` authority-bearing field remains on `KnowledgeContext` (asserted by the source guard extended in 0006PHA2AEPISUB-005) + runtime test that a sealed embodied context's `permits_scope` cannot be flipped to debug behavior.
2. `INV-068` / `INV-107` (debug non-diegetic, capability-gated) -> codebase grep-proof: `KnowledgeContext::debug` requires a `DebugCapability` argument; manual review that the only minting path stays crate-private.
3. `INV-018` (deterministic replay unaffected) -> replay/golden-fixture check: `cargo test -p tracewake-core --test event_schema_replay_gates` and the projection checksum tests stay green (canonical hash inputs unchanged).

## What to Change

### 1. Privatize `KnowledgeContext` authority fields and add accessors

In `crates/tracewake-core/src/epistemics/knowledge_context.rs`, remove `pub` from `viewer_actor_id`, `bound_actor_id`, `mode`, `current_tick`, `event_frontier`, `allowed_sources`, `forbidden_sources`, `perception_scope`, `belief_scope`, `observation_scope`, `projection_schema_version`, `debug_non_diegetic`. Add read-only accessor methods (`viewer_actor_id()`, `mode()`, `allowed_sources()`, etc.) alongside the existing `holder_known_context_id()`/`holder_known_context_hash()` accessors. `permits_scope` already reads through `self`; it stays correct once fields are private because no external write path remains.

### 2. Capability-gate debug-context construction

Change `KnowledgeContext::debug` to take a `&DebugCapability` (or move it behind a crate-private factory used only by the debug-view builder). External crates must not be able to mint a `ViewMode::Debug` context via public API. Update the in-module test at `:373` to obtain a capability through the crate-private mint.

### 3. Migrate same-crate consumers to accessors

Update `epistemics/projection.rs` (`_for_context` filters), `projections.rs` (embodied view-model builder), `accuseprobe.rs`, and `knowledge_basis.rs` to call the new accessors instead of reading fields directly.

## Files to Touch

- `crates/tracewake-core/src/epistemics/knowledge_context.rs` (modify)
- `crates/tracewake-core/src/epistemics/projection.rs` (modify)
- `crates/tracewake-core/src/projections.rs` (modify)
- `crates/tracewake-core/src/actions/defs/accuseprobe.rs` (modify)
- `crates/tracewake-core/src/epistemics/knowledge_basis.rs` (modify)
- `crates/tracewake-core/src/debug_capability.rs` (modify — only if the debug factory needs a new crate-private entry point)

## Out of Scope

- `EpistemicProjection` storage sealing and raw-map removal (0006PHA2AEPISUB-002).
- Epistemic record (`Belief`/`Observation`/`Contradiction`) sealing (0006PHA2AEPISUB-003).
- Compile-fail negative fixtures proving the seal (0006PHA2AEPISUB-004) and source-guard extension (0006PHA2AEPISUB-005).
- Any change to the canonical hash/serialization semantics (must remain byte-identical).

## Acceptance Criteria

### Tests That Must Pass

1. A new runtime regression test in `knowledge_context.rs`: a sealed `KnowledgeContext::embodied(actor_a, tick)` returns `false` from `permits_scope` for `actor_b`'s `ActorPrivate` scope, and there is no public API to change its `mode` to `Debug` afterward.
2. `cargo test -p tracewake-core` (epistemics + projections + actions suites) passes with consumers migrated to accessors.
3. `cargo build --workspace --all-targets --locked` and `cargo clippy --workspace --all-targets -- -D warnings` pass.

### Invariants

1. No `pub` authority-bearing field remains on `KnowledgeContext`; all reads go through accessors.
2. `KnowledgeContext::debug` cannot be invoked without a `DebugCapability`; the canonical hash inputs (and therefore replay/projection checksums) are byte-identical to pre-change.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/epistemics/knowledge_context.rs` (in-module tests) — add the seal-integrity regression test; update the existing debug-context test to mint a capability.
2. `crates/tracewake-core/src/epistemics/projection.rs` / `src/projections.rs` — existing tests adjusted only where they read now-private fields.

### Commands

1. `cargo test -p tracewake-core --lib epistemics::knowledge_context`
2. `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test -p tracewake-core --test event_schema_replay_gates` — narrow proof that sealing did not perturb deterministic replay/hash inputs.

## Outcome

Completed: 2026-06-09

What changed:
- `KnowledgeContext` authority-bearing fields are now private and exposed only through read-only accessors.
- `KnowledgeContext::debug` now requires a `DebugCapability`, so ordinary public API callers cannot mint debug contexts without the core capability token.
- Core and TUI consumers were migrated from direct field reads to accessors.
- Added a runtime seal regression proving an embodied context remains embodied and cannot permit another actor's private scope after construction.

Deviations from original plan:
- The TUI fixture builders also needed accessor migration because they construct proposal/view-model test data from a sealed `KnowledgeContext`; this stayed within the ticket's API-sealing scope.
- Compile-fail fixtures and source-guard expansion remain deferred to tickets `0006PHA2AEPISUB-004` and `0006PHA2AEPISUB-005` as planned.

Verification results:
- `cargo test -p tracewake-core --lib epistemics::knowledge_context` — passed.
- `cargo test -p tracewake-core` — passed.
- `cargo test -p tracewake-core --test event_schema_replay_gates` — passed.
- `cargo fmt --all --check` — passed.
- `cargo build --workspace --all-targets --locked` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo test --workspace` — passed.
