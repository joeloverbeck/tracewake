# 0002PHA1KERTUI-014: Controller binding and possession parity

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adds the `controller` module to `tracewake-core`; emits `ControllerAttached`/`ControllerDetached` on the `controller` stream.
**Deps**: 0002PHA1KERTUI-005, 0002PHA1KERTUI-008

## Problem

The authoritative world must not contain a sacred player entity; controller binding is runtime/control metadata, not a physical/social/mental/epistemic fact (Spec 0002 §6.1, §16). Binding must be stored outside authoritative world state; changing it must not alter any actor component, create knowledge, reveal debug truth, change preconditions, or add player-only verbs (§16.2). Human commands and non-human proposals share the validation/commit path after a single command-authorization check (§16.3). This ticket implements the `ControllerBinding` lifecycle, the `controller`-stream events, and the binding-check hook the pipeline calls.

## Assumption Reassessment (2026-06-06)

1. The `ControllerBinding` record exists in `state` (ticket 003) but is stored as non-world runtime metadata; the `controller` stream and `ControllerAttached`/`ControllerDetached` kinds exist from ticket 005; the pipeline's binding-check stage hook exists from ticket 008. This ticket adds `controller.rs` and wires the binding-check.
2. The binding model is `specs/0002_…_SPEC.md` §16.1 (`controller_id`, `bound_actor_id?`, `mode`, `binding_sequence`, `created_at_tick` for correlation only); the no-knowledge-transfer rules are §16.2; the shared-path rule is §16.3.
3. Shared boundary under audit: the binding-check hook consumed by the pipeline (008, human-origin authorization only) and the `controller`-stream events that must never be applied to physical state (the ticket-006 applier treats them as physical no-ops).
4. Invariant motivating this ticket: INV-004 (the authoritative world ignores human existence), INV-005 (the only normal controller binding is ordinary possession — input binding only), INV-006 (possession transfers no world knowledge), and INV-094 (possession parity is tested).
5. No-leak + replay surface: `ControllerAttached`/`Detached` live on the non-authoritative `controller` stream and must not enter physical reconstruction or actor state (INV-006). The binding-check is pure command authorization ("is this controller bound to this actor?") and changes no precondition. This ticket guarantees binding changes produce no physical checksum change and no actor-state mutation; the possession-parity regression is ticket 022.

## Architecture Check

1. Storing binding outside world state and emitting it only on the `controller` stream makes "possession transfers no knowledge" structurally true: the physical reconstruction path never sees binding, so it cannot be influenced by it. A binding field on the actor record (the alternative) would make the world know which actor is human-controlled (violates INV-004).
2. No backwards-compatibility shims: greenfield; no player-entity concept is introduced to alias.

## Verification Layers

1. Binding is non-world metadata (INV-004/006) -> unit test: attach/switch/detach changes no actor component and no physical checksum.
2. Shared path after authorization (INV-007; §16.3) -> unit test: after the binding check, a human-origin proposal runs the identical validation as a non-human proposal for the same (actor, action, target, state).
3. Controller events non-authoritative (INV-006) -> unit test: applying a `ControllerAttached` event to physical state is a no-op (reuses ticket-006 guarantee).

## What to Change

### 1. Controller module

Add `crates/tracewake-core/src/controller.rs`: the `ControllerBinding` lifecycle (attach/switch/detach) stored outside world state, emitting `ControllerAttached`/`ControllerDetached` on the `controller` stream with deterministic `binding_sequence`.

### 2. Binding-check hook

Wire the pipeline's binding-check stage (ticket 008) to confirm, for human-origin commands only, that a controller is bound to the submitted actor — command authorization, not world validation.

### 3. Registration

Add `pub mod controller;` to `crates/tracewake-core/src/lib.rs`.

## Files to Touch

- `crates/tracewake-core/src/controller.rs` (new)
- `crates/tracewake-core/src/actions/pipeline.rs` (modify — wire binding-check hook; file created by ticket 008)
- `crates/tracewake-core/src/lib.rs` (modify — add `pub mod controller;`; file created by ticket 001)

## Out of Scope

- The TUI surface for binding/switching (ticket 020) and the binding debug panel (ticket 021).
- The controller-binding debug report (ticket 016).
- Belief/knowledge parity (Phase 2); Phase 1 proves action-path + metadata parity only (§16.4).

## Acceptance Criteria

### Tests That Must Pass

1. Attaching, switching, and detaching a controller changes no actor component and leaves the physical checksum unchanged.
2. After the binding check, identical human-origin and non-human proposals produce identical validation reports and events.
3. A human-bound actor exposes no extra ordinary semantic action versus a non-human proposal enumeration for the same actor/state.

### Invariants

1. Controller binding lives outside authoritative world state.
2. Binding changes create no world event on the `world` stream and no knowledge transfer.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/controller.rs` (unit tests) — binding lifecycle, checksum invariance, parity after authorization.

### Commands

1. `cargo test -p tracewake-core controller`
2. `cargo build --workspace`
3. Unit scope is correct; the full `debug_attach_001` possession-parity scenario runs in ticket 022.
