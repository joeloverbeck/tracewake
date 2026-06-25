# 0052FOUCONFOU-004: F4-07 — complete TUI de-authority over no-human / replay / perception / view / checksum / debug (atomic cutover)

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — removes public raw aggregate getters; folds no-human/rebuild/perception/view/checksum/debug into typed commands and sealed receipts behind the existing debug capability
**Deps**: 0052FOUCONFOU-001, 0052FOUCONFOU-003

## Problem

Spec 0052 F4-07 (§1.1.7, §4.8): the runtime owns aggregate fields, but exports enough references and commands for the TUI to remain an authoritative orchestrator. `TuiApp::current_view` (`app.rs:154`) reads raw `physical_state`/`agent_state`/registry/projection/tick/manifest/log-length and constructs `KnowledgeContext`/`EmbodiedTruthSnapshot`/snapshots itself; `run_no_human_day` in the runtime (`session.rs:254`) accepts a caller `Vec<ActorId>`, and `TuiApp::run_no_human_day` (`app.rs:307`) passes an empty vector then separately computes a checksum context, calls `rebuild_from_owned_log` (`session.rs:269`), and refreshes perception (`session.rs:293`); the six raw aggregate getters (`session.rs:113–133`) and the two mutators are public. Other clients could compose a different transaction order or read authoritative aggregates to build their own surfaces.

This ticket exposes one opaque session handle with typed commands and sealed view/debug receipts and removes the public raw aggregate getters from the client-facing API: no-human advancement becomes a single runtime command that derives its actor census internally and performs advancement/rebuild/projection/perception atomically, returning a typed debug/observer receipt; rebuild becomes an internal recovery operation; possession binding/perception update becomes one core transaction with a receipt; debug queries route through the existing `debug_capability` boundary. Closes standing survivors #20 (`rebuild_from_owned_log -> Ok(())`, jointly with 005) and #21 (delete `refresh_actor_current_place_perception`).

## Assumption Reassessment (2026-06-25)

1. The six public getters `registry`/`physical_state`/`agent_state`/`event_log`/`epistemic_projection`/`controller_bindings` (`session.rs:113–133`) and the public mutators `run_no_human_day(Vec<ActorId>)` (254), `rebuild_from_owned_log` (269), `refresh_actor_current_place_perception` (293) currently exist. `TuiApp::current_view` (app.rs:154) and `current_view_context` (app.rs:200) assemble the view over the raw getters; `TuiApp::run_no_human_day` (app.rs:307) authors the no-human transaction order. The existing non-forgeable debug boundary is `crates/tracewake-core/src/debug_capability.rs` (with `debug_reports.rs`, `checksum.rs`).
2. Spec home: `specs/0052_…_HARDENING_SPEC.md` §4.8; closure-order step 3 ("internalize all client orchestration"). The opaque session/sealed receipts and closed command dispatch this rides on are from 0052FOUCONFOU-001 and -003.
3. Cross-artifact boundary under audit: the `tracewake-core` public client surface consumed by `tracewake-tui` — specifically the view/debug/no-human/rebuild/perception/checksum surfaces. After this ticket the TUI receives sealed receipts and a debug receipt under capability, never general-purpose authoritative references. Core-internal modules may still use references; the restriction is the public production client surface.
4. Motivating invariants: INV-069 (TUI is a client, must not implement simulation rules), INV-108 (human possession is cognition-neutral), INV-009/INV-018 (eventful, replay-safe mutation), INV-101 (core-sealed cognition products).
5. Fail-closed / replay / actor-knowledge surface: the no-human command derives its actor census from the runtime-owned loaded actor set (no caller list), and performs advancement/rebuild/projection/perception as one atomic cutover (scratch-state, all-or-nothing) — preserving deterministic replay (INV-018) and committing perception/epistemic deltas through events (INV-009). Possession binding/perception becomes one core transaction with explicit event/projection effects; the debug receipt may expose exact state safely only through `debug_capability` (INV-068/INV-107 quarantine), so removing the raw getters introduces no leakage path and closes one — the TUI can no longer read raw projection/state to build an unfiltered surface (INV-024, INV-093).
6. Removal blast radius (old menu item 7): the six getters and three mutators are removed from the client API; consumers are `runtime/session.rs` (defns), `tracewake-tui/src/app.rs` (`current_view`, `current_view_context`, `run_no_human_day`, debug/checksum panels), and the `run_no_human_day_refuses_intrinsically_without_debug_availability` test (app.rs:683, re-expressed against the typed debug command). All in-workspace; updated in this diff. Core-internal reference use is retained.

## Architecture Check

1. One opaque session handle + typed commands + sealed receipts is cleaner than nine public methods/getters because authority is judged by what the client can *cause and observe*, not by which struct stores the fields (driver §6.1 mechanism 3) — the TUI cannot compose a different transaction order, omit rebuild/perception, supply a different actor list, or read authoritative aggregates. The no-human postconditions become a single core-owned atomic transaction rather than a client-authored sequence.
2. No backwards-compatibility alias: the getters and mutators are removed, not retained behind a read-only wrapper. The debug surface is the existing `debug_capability` receipt, not a new general-purpose accessor.

## Verification Layers

1. INV-069/INV-108 (client boundary, possession-neutral) -> external negative fixtures (corpus in 009): the production session yields no `&PhysicalState`/`&AgentState`/`&EventLog`/`&EpistemicProjection`/`&ControllerBindings`/`&ActionRegistry` and cannot call rebuild, perception refresh, or no-human with an actor list.
2. INV-018/INV-009 (replay-safe, eventful) -> fault-injection fixture: an internal post-step failure proves no partial state/log/projection cutover; mutation kills for `rebuild_from_owned_log -> Ok(())` (#20) and perception-refresh deletion (#21).
3. INV-024/INV-093 (no-leak) -> manual epistemic-leakage review: the sealed view receipt carries only actor-known content; exact state reaches only the debug receipt under capability.
4. INV-009 (perception eventful) -> behavior test: a binding/post-rebuild command causes a provenance-bearing perception/epistemic delta observable in the sealed view and replay.

## What to Change

### 1. Opaque session + sealed view/debug receipts (`session.rs`, new core view/debug facade)

Remove the six public aggregate getters from the client-facing API. Move view assembly into core (a view/debug facade producing sealed receipts), so the TUI receives an immutable embodied receipt and a debug receipt under `debug_capability` — not raw aggregates. (The embodied product's exact-metadata reseal and `view_models.rs` reshape are 008; this ticket relocates *assembly ownership*, not the embodied field set.)

### 2. No-human as one atomic command (`session.rs`)

Replace `run_no_human_day(Vec<ActorId>)` with a single typed debug-authorized command that derives the actor census from the runtime-owned loaded actor set and performs advancement/rebuild/projection/perception atomically, returning a typed debug/observer receipt.

### 3. Internal rebuild + possession/perception transaction (`session.rs`)

Make `rebuild_from_owned_log` an internal recovery operation (the runtime owns its seed/snapshot + checksum context), not a client command. Make possession binding/perception update one core transaction with explicit event/projection effects and a receipt; remove `refresh_actor_current_place_perception` as an independent public method.

### 4. TUI consumes receipts (`app.rs`)

Rewrite `current_view` / `run_no_human_day` to submit typed commands and consume sealed receipts; delete raw-aggregate assembly and the client-authored no-human sequence.

## Files to Touch

- `crates/tracewake-core/src/runtime/session.rs` (modify — remove getters, no-human single command, internal rebuild, possession/perception transaction)
- `crates/tracewake-core/src/runtime/view_facade.rs` (new — core-owned sealed view/debug receipt assembly)
- `crates/tracewake-core/src/runtime/mod.rs` (modify — register `view_facade`)
- `crates/tracewake-tui/src/app.rs` (modify — consume sealed receipts; typed no-human command)

## Out of Scope

- Reshaping `EmbodiedViewModel` / `TypedActorKnownIntervalSummary` to remove exact tick/frontier metadata and repairing normal `continue` output (008).
- Replay-authority reconstruction semantics / fail-closed restore (005 — this ticket only makes rebuild internal-atomic).
- The actor disposition census (007).
- The external negative-fixture corpus, fault-injection corpus, and conformance lane (009).

## Acceptance Criteria

### Tests That Must Pass

1. Public-boundary TUI tests bootstrap production, invoke no-human through a typed debug command, and assert atomic committed receipt + replay equivalence — with no client-side actor list, rebuild call, or perception refresh.
2. A fault-injection/duplicate-event fixture forces an internal post-step failure and proves no partial state/log/projection cutover.
3. A binding/post-rebuild command causes a provenance-bearing perception/epistemic delta observable in the sealed view and replay.
4. `cargo build --workspace --all-targets --locked && cargo test --workspace`.

### Invariants

1. The production session exposes no `&PhysicalState`/`&AgentState`/`&EventLog`/`&EpistemicProjection`/`&ControllerBindings`/`&ActionRegistry`, no client rebuild, no perception refresh, and no no-human actor-list parameter (INV-069, INV-108).
2. No-human advancement is one atomic transaction; partial failure commits nothing (INV-018, INV-009).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/command_loop_session.rs` — typed no-human debug command; atomic receipt + replay equivalence; re-express `run_no_human_day_refuses_intrinsically_without_debug_availability` against the typed command.
2. `crates/tracewake-core/tests/world_step_coordinator.rs` — fault-injection no-partial-cutover; binding/perception provenance delta.

### Commands

1. `cargo test -p tracewake-tui --test command_loop_session`
2. `cargo build --workspace --all-targets --locked && cargo test --workspace`
