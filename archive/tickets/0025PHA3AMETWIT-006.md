# 0025PHA3AMETWIT-006: TUI gate depth — intrinsic method gate, arm-complete dispatch census, local lock registry, ControllerMode decision

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-tui` (`app.rs`, `run.rs`) and, per the recorded `ControllerMode` decision, `tracewake-core` (`state.rs`).
**Deps**: None

## Problem

Spec 0025 findings `ORD-HARD-176` (medium), `ORD-HARD-174` (medium),
`ORD-HARD-185` (low), `ORD-HARD-186` (low). The 0024 quarantine
(`ORD-HARD-152`/`153`) gated the *command surface* but not the *operation*:
`pub fn run_no_human_day(&mut self) -> NoHumanDayReport` on `TuiApp` is
world-advancing and ungated — the only gate is the `DebugCommand::RunNoHumanDay`
dispatch path in `render_debug`; tests already call the method unbound. The gating
guard itself is positional: `debug_command_gating_errors` asserts only that the
substring `if !app.debug_available()` precedes `match debug_command` — no variant
enumeration, no early-return detection; a second match or a `return` above the
gate passes. The TUI guards are enrolled in no lock registry (core's
`META_LOCK_REGISTRY` cannot reach TUI tests), so deleting them is itself uncaught.
And `ControllerMode::Debug` is non-discriminating: `debug_available_for` requires
only a bound, non-`Detached` binding, so `Embodied` grants debug identically and
the `Debug` variant gates nothing — a dead-variant smell that will mislead a
future author.

## Assumption Reassessment (2026-06-12)

1. Verified against code at baseline `9e33d7a`: `pub fn run_no_human_day` at
   `crates/tracewake-tui/src/app.rs`; `debug_command_gating_errors`'s two-substring
   positional check; zero lock-registry references anywhere in `tracewake-tui`;
   `debug_available_for`'s `bound_actor_id == actor && mode != Detached` predicate;
   `enum ControllerMode` in `crates/tracewake-core/src/state.rs`. All
   operator-verified per spec 0025 §8.
2. Verified against spec 0025 §4 and doctrine: INV-068/107
   (`docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`),
   `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`
   (the 0024-recorded operator-proof classification and gating policy this ticket
   deepens); the apply-mutator census (`apply_mutator_tokens_from`,
   `apply_mutator_perimeter_errors`) is the proven parity-derivation model to
   mirror.
3. Shared boundary under audit: the derived `debug_available` gate ↔ the set of
   world-advancing and debug-revealing operations — the gate must be a property of
   the operations, not of one dispatch path.
4. Invariant restated before trusting the narrative (INV-068 — debug mode is
   visibly non-diegetic; INV-107 — debug omniscience is quarantined): debug
   surfaces are operator-only and must not be reachable as normal-play
   world-affecting shortcuts; the gate's depth is part of the quarantine.
5. Enforcement surface touched: debug quarantine gating. All changes are added
   refusals and guard derivations — read-only with respect to cognition and
   replay; no actor-knowledge filtering is relaxed and no event semantics change
   (a refused operation emits nothing).
6. Removal blast radius (conditional on the recorded `ControllerMode` decision):
   if the decision is **delete the `Debug` variant**, grep-verified consumers are
   `state.rs` (the enum), `controller.rs` unit tests (the only constructors of
   `Debug` mode), and `debug_available_for`'s matches — all updated in this diff;
   no doc-governed contract names the variant. If the decision is **require
   `mode == Debug`**, the TUI's `bind_actor` call sites gain an explicit mode and
   the refusal tests pin `Embodied`-binding ⇒ no debug. Either way the decision is
   recorded per spec 0025 §6 (the TUI conformance-row home /
   `docs/2-execution/07`).

## Architecture Check

1. Making the gate intrinsic to the operation (typed
   `Result<NoHumanDayReport, DebugUnavailable>` refusal inside the method) is
   cleaner than guarding call paths: every future caller inherits the refusal, and
   the guard burden shifts from "enumerate all paths" to "assert the method
   consults the gate" — the same intrinsic-over-perimeter reasoning the spec's
   lock-depth findings apply throughout. Deriving the dispatch census from
   `enum DebugCommand` mirrors the proven `apply_mutator_tokens_from` parity
   model.
2. No backwards-compatibility aliasing/shims: the ungated method signature is
   replaced, not wrapped; the positional guard is replaced by the derived census,
   not kept alongside; a deleted `Debug` variant (if so decided) leaves no alias.

## Verification Layers

1. Intrinsic gate (`ORD-HARD-176`) → refusal test invoking the *method* (not the
   command) on an unbound app: typed refusal, `event_count` unchanged; plus a
   guard enumerating world-advancing `pub fn` on `TuiApp` (those mutating
   `self.log`/`self.state`) asserting each consults `debug_available` internally
   or carries a rationale-bearing exemption.
2. Arm-complete census (`ORD-HARD-174`) → arm set derived from
   `enum DebugCommand` in `input.rs` source: each variant appears exactly once
   inside the single `match debug_command`, strictly after the gate, with no
   `return` between function entry and the gate; parity synthetic (unrouted
   variant) and early-return synthetic both fire.
3. TUI-local registry (`ORD-HARD-185`) → a TUI mini-registry (required-member
   name census over the `#[cfg(test)]` guard fns) fails when an enrolled guard is
   removed; removing a member synthetically fires it.
4. `ControllerMode` decision (`ORD-HARD-186`) → a three-arm test pinning
   `debug_available_for` against every `ControllerMode` variant per the recorded
   decision; the decision text recorded in the §6-named home (grep-proof).
5. Whole-suite regression → `cargo test -p tracewake-tui`, `cargo test -p
   tracewake-core`, and the four-gate workspace run.

## What to Change

### 1. Intrinsic method gate (`ORD-HARD-176`)

`run_no_human_day` returns a typed refusal when `!self.debug_available()`; the
`render_debug` arm handles the `Result`; tests bind first (or use an explicitly
recorded test-only path). Add the world-advancing-`pub fn` guard.

### 2. Arm-complete dispatch census (`ORD-HARD-174`)

Replace `debug_command_gating_errors`'s positional check with the derived
variant census + early-return detection; add the two firing synthetics.

### 3. TUI-local lock registry (`ORD-HARD-185`)

Add the mini-registry enumerating the TUI structural guards (the gating census,
the render-reachability guard, the new method-gate guard, the refusal tests) with
a firing removed-member synthetic.

### 4. `ControllerMode` decision (`ORD-HARD-186`) — implementer-recorded choice

Decide and record: require `mode == Debug` for debug availability (TUI binds
modes explicitly), or delete/document the variant as reserved. Implement with the
three-arm pin either way (Assumption item 6 carries the per-branch blast radius).

## Files to Touch

- `crates/tracewake-tui/src/app.rs` (modify)
- `crates/tracewake-tui/src/run.rs` (modify)
- `crates/tracewake-core/src/state.rs` (modify — only as the recorded `ControllerMode` decision requires)
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md` (modify — record the two decisions per spec §6)

## Out of Scope

- The `ORD-HARD-095` bind-time perception deferral (standing owner decision).
- Embodied snapshot/carrier work (`ORD-HARD-172`/`173`) — ticket
  `0025PHA3AMETWIT-005` (note: it also modifies `app.rs`; coordinate the
  mechanical merge).
- Building player-facing time acceleration (Phase 3B+ staging holds).

## Acceptance Criteria

### Tests That Must Pass

1. The unbound *method* call yields the typed refusal with `event_count`
   unchanged; the world-advancing-`pub fn` guard passes and fails on a synthetic
   second ungated world-advancer.
2. The derived dispatch census passes; the unrouted-variant and early-return
   synthetics fire; the TUI mini-registry fails on a removed member; the
   three-arm `ControllerMode` pin matches the recorded decision, and the decision
   text is present in `docs/2-execution/07` (grep-proof).
3. `cargo test -p tracewake-tui`, `cargo test -p tracewake-core`, and the four
   gates (`cargo fmt --all --check`,
   `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo build --workspace --all-targets --locked`, `cargo test --workspace`) pass.

### Invariants

1. World-advancing operations on `TuiApp` refuse intrinsically without derived
   debug availability — gate depth is a property of the operation, not a dispatch
   path.
2. Every `DebugCommand` variant dispatches exactly once, behind the single gate;
   the TUI guard set is registry-locked against silent deletion.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/src/app.rs` (`#[cfg(test)]`) — method-refusal test,
   derived dispatch census + synthetics, world-advancing-`pub fn` guard, TUI
   mini-registry + removed-member synthetic, three-arm `ControllerMode` pin.

### Commands

1. `cargo test -p tracewake-tui`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-13

### Files Changed

- `crates/tracewake-tui/src/app.rs`
- `crates/tracewake-tui/src/input.rs`
- `crates/tracewake-tui/src/main.rs`
- `crates/tracewake-tui/src/run.rs`
- `crates/tracewake-tui/tests/adversarial_gates.rs`
- `crates/tracewake-tui/tests/readme_sample_session.rs`
- `crates/tracewake-tui/tests/tui_acceptance.rs`
- `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`
- `README.md`

### Decisions

- Chose and recorded the strict `ControllerMode` policy: debug availability
  requires `ControllerMode::Debug`; ordinary `ControllerMode::Embodied` binding
  does not grant debug.
- Added explicit TUI debug binding via `TuiApp::bind_debug_actor` and
  `bind-debug <actor_id>` so operator debug remains reachable without making
  normal embodied play debug-capable.
- Made `TuiApp::run_no_human_day` intrinsically gated and changed it to return
  `Result<NoHumanDayReport, AppError>`, with `AppError::DebugUnavailable` when
  debug availability is absent.
- Replaced the positional debug-dispatch guard with an enum-derived arm census
  plus early-return and unrouted-variant synthetics.
- Added a TUI-local guard registry and a world-advancing mutable-method census
  with a rationale-bearing exemption for ordinary embodied submission.

### Acceptance Disposition

- Unbound method refusal: completed; no-human-day refuses with
  `DebugUnavailable` and leaves `event_count` unchanged.
- World-advancing public-method guard: completed; synthetic ungated method fails
  the census.
- Derived debug dispatch census: completed; unrouted variant and early-return
  synthetics fire.
- TUI local guard registry: completed; synthetic removed-member check fires.
- `ControllerMode` three-arm pin and recorded decision: completed in TUI tests
  and `docs/2-execution/07_EPISTEMIC_VIEW_MODELS_POSSESSION_AND_DEBUG_PROOF.md`.
- Deferred or dropped work: none.

### Verification

- `cargo test -p tracewake-tui`
- `cargo test -p tracewake-core`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`
