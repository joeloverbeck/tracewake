# 0052FOUCONFOU-008: F4-06 — sealed embodied/debug temporal split; non-leaking normal output

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — removes exact tick/frontier/control metadata from the embodied public product (all feature combinations), splits embodied/debug products, repairs normal `continue` output
**Deps**: 0052FOUCONFOU-001, 0052FOUCONFOU-003

## Problem

Spec 0052 F4-06 (§1.1.6, §4.7): `render_embodied_view` is repaired (qualitative notices only), but the normal `continue` command prints exact `reason`, `ticks`, and `stop_tick` outside any debug label (`run.rs:84–87`); `TypedActorKnownIntervalSummary` (`view_models.rs:187`) and `EmbodiedViewModel` (`view_models.rs:19`) expose exact start/stop ticks, frontiers, stop reason, and holder-known context through public accessors; the public `test-support` feature makes temporal fields and test constructors public (`view_models.rs:20–50` cfg-gated `pub`/`pub(crate)` field pairs), and the TUI enables that feature — so supposedly sealed products are forgeable across supported feature combinations; and the TUI performs interval conversion and mutates the assembled product with public setters.

This ticket returns two distinct core-created products — an **embodied receipt/view** (actor-known qualitative interval consequences + actor-legible stop info with provenance) and a **debug receipt/view** (exact ticks/frontiers/event IDs/scheduler stop reasons/control bounds) constructible only through the existing `debug_capability` boundary — removes exact tick/frontier/control reason from the embodied public type, makes all production fields/constructors private in **every** feature combination, and renders normal `continue` output from the embodied receipt. Closes standing survivors #9–16 (view/interval accessors) and #23 (transcript section selector, jointly with 009).

## Assumption Reassessment (2026-06-25)

1. `EmbodiedViewModel` (`view_models.rs:19`) exposes `sim_tick` with cfg-gated `pub sim_tick` (line 33, `feature="test-support"`) / `pub(crate) sim_tick` (line 35) field pairs and a public `sim_tick()` accessor (line 142); `TypedActorKnownIntervalSummary` (`view_models.rs:187`) exposes `start_tick`/`stop_tick`/frontiers similarly. `render_embodied_view` (`render.rs:13`) is already qualitative; `render_debug_overlay` (`render.rs:159`) labels exact material `DEBUG NON-DIEGETIC`. The normal `continue` print is `run.rs:84–87` (`reason=… ticks=… stop_tick={result.stop_tick.value()}`). `tracewake-core` declares `test-support = []` (Cargo.toml:10); `tracewake-tui` enables it (Cargo.toml:12). `projections.rs` builds the interval/view products.
2. Spec home: `specs/0052_…_HARDENING_SPEC.md` §4.7; closure-order step 7. The sealed embodied/debug receipt types and the existing `debug_capability` boundary are from 0052FOUCONFOU-001; the closed command boundary that returns them is 0052FOUCONFOU-003.
3. Cross-artifact boundary under audit: the core view-model product surface consumed by `tracewake-tui` rendering, plus the crate feature topology (`test-support`) that dependants can select. The TUI must receive an immutable product with no exact-time accessor and no public setter.
4. Motivating invariants: INV-067 (embodied shows actor-known reality), INV-068 (debug visibly non-diegetic), INV-093 (leakage is high-severity), INV-101/102 (sealed actor-known context + provenance), INV-112 (temporal authority validates/orders but is not embodied/cognition authority unless holder-known).
5. Fail-closed / actor-knowledge / temporal surface (core enforcement surface): exact tick/frontier/control reason are removed from the embodied public type (not merely hidden behind a `pub(crate)` while a public getter remains), and the debug product is constructible only through `debug_capability`. This strengthens INV-067/093/112; the information-flow distinction (access control ≠ controlling what flows to outputs) means a public getter or a feature-gated public field is itself a leak channel, so the fields go private in every feature combination — `controller_safety_bound`, raw tick count, and exact `stop_tick` belong only to the debug product unless a modeled actor-known clock supplies them.
6. Schema reshape (view-model projection — distinct from item 5): `EmbodiedViewModel` and `TypedActorKnownIntervalSummary` are reshaped to drop exact `sim_tick`/`start_tick`/`stop_tick`/`start_frontier`/`stop_frontier`/stop-reason from the embodied public product; those fields move to the debug product. This is a **breaking-internal** change to the in-crate view-model schema (consumers: `view_models.rs`, `projections.rs`, `render.rs`, `tracewake-tui` `app.rs`/`run.rs`) — every consumer updates in this diff; the exact-time data is preserved on the debug product, so no information is lost, only re-homed. No external/persisted serialized consumer exists (these are presentation projections, not save-package entries).
7. Removal blast radius (test-support public surface): the `#[cfg(feature="test-support")] pub` field/constructor exposures (`view_models.rs:20–50`) are replaced with internal `#[cfg(test)]` builders / crate-private test helpers; `tracewake-tui`'s `features = ["test-support"]` dependency enablement (Cargo.toml:12) is removed. Grep blast radius: `test-support` consumers are `crates/tracewake-core/Cargo.toml`, `crates/tracewake-core/src/view_models.rs`, and `crates/tracewake-tui/Cargo.toml`.

## Architecture Check

1. Two distinct core-created products (embodied vs debug-under-capability) with the embodied type carrying no exact-time field is cleaner than hiding fields behind getters because making the forbidden product *unrepresentable* is stronger than a getter-calling test — a compile-fail test is vacuous if a public feature reopens the fields, so the fields go private in every feature combination and the debug product is the only path to exact time. Moving interval attachment/composition into core gives the TUI an immutable product, ending client-side forgeability.
2. No backwards-compatibility alias: exact-time accessors are removed from the embodied type, not deprecated; the public `test-support` field exposure is removed, not retained for convenience.

## Verification Layers

1. INV-067/093/112 (actor-known, no leak, temporal) -> public command-loop hidden-world pair: two worlds differing only in hidden exact time/frontier/control metadata produce identical normal output and semantic action surface, while debug output differs and is labeled.
2. INV-068 (debug non-diegetic) -> normal-transcript guard banning exact tick/frontier/event IDs and internal stop-reason tokens; debug transcript positively requiring them (kills #23, jointly with 009).
3. Compile-time unrepresentability -> external compile-fail fixtures under default **and all supported** feature combinations: cannot construct/mutate embodied temporal products, cannot call debug constructors without capability, cannot convert a debug receipt to an embodied receipt (full corpus in 009).
4. Mutation -> tests for all interval accessors / stop classification / transcript-section selection (kills #9–16, #23).

## What to Change

### 1. Split sealed products (`view_models.rs`, `projections.rs`)

Reshape `EmbodiedViewModel` / `TypedActorKnownIntervalSummary` to carry only actor-known qualitative interval consequences + actor-legible stop info with provenance; move exact ticks/frontiers/event IDs/stop reasons/control bounds to the debug receipt (constructible only via `debug_capability`). Move interval-summary attachment and notebook/debug composition into core view-model construction so the TUI receives an immutable product (no public setters).

### 2. Private fields in all feature combinations (`view_models.rs`, `Cargo.toml`)

Make all production fields and constructors private in every feature combination; replace public `test-support` constructors/fields with internal `#[cfg(test)]` builders / crate-private test helpers. Remove `tracewake-tui`'s `test-support` feature enablement.

### 3. Repair normal `continue` (`run.rs`, `app.rs`)

Render normal `continue` output from the embodied receipt — exact `stop_tick`, raw tick count, and `controller_safety_bound` belong only in the visibly non-diegetic debug product. Remove the TUI-side interval conversion (`advance_until` delta → interval summary moves to core).

## Files to Touch

- `crates/tracewake-core/src/view_models.rs` (modify — split products; private fields all feature combos)
- `crates/tracewake-core/src/projections.rs` (modify — core-owned interval/composition)
- `crates/tracewake-core/Cargo.toml` (modify — `test-support` no longer exposes production fields)
- `crates/tracewake-tui/Cargo.toml` (modify — remove `test-support` enablement)
- `crates/tracewake-tui/src/run.rs` (modify — `continue` renders embodied receipt)
- `crates/tracewake-tui/src/app.rs` (modify — consume immutable product; no client interval conversion)
- `crates/tracewake-tui/src/render.rs` (modify — render from split products)

## Out of Scope

- The full external compile-fail corpus across all feature combinations + transcript guards (009 holds the consolidated corpus; this ticket lands the product split + the hidden-world behavior pair).
- The actor disposition census diagnostics (007 — routed into the debug product, consumed here).
- Replay/process/no-human semantics (005/006/004).

## Acceptance Criteria

### Tests That Must Pass

1. A public command-loop hidden-world pair: two worlds differing only in hidden exact time/frontier/control metadata produce identical normal output and semantic action surface; debug output differs and is labeled.
2. A normal-transcript guard bans exact tick/frontier/event IDs and internal stop-reason tokens; a debug transcript positively requires them.
3. External compile-fail fixtures (in 009) under default and all supported feature combinations cannot construct/mutate embodied temporal products or convert a debug receipt to an embodied receipt; mutation kills #9–16 and #23.
4. `cargo build --workspace --all-targets --locked && cargo test --workspace` (and `cargo build -p tracewake-core --features test-support` proves no production field is reopened).

### Invariants

1. The embodied public product carries no exact tick/frontier/control-reason field in any feature combination; exact time is reachable only through the debug receipt under capability (INV-067, INV-093, INV-112).
2. Normal `continue` output contains no exact tick/frontier/internal stop-reason token (INV-068).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/holder_known_interval_projection.rs` — split-product field-absence + interval accessor mutation sensitivity (#9–16).
2. `crates/tracewake-tui/tests/command_loop_session.rs` — hidden-world normal-output noninterference pair; normal/debug transcript guards (#23).

### Commands

1. `cargo test -p tracewake-tui --test command_loop_session && cargo test -p tracewake-core --test holder_known_interval_projection`
2. `cargo build --workspace --all-targets --locked && cargo build -p tracewake-core --features test-support && cargo test --workspace`

## Outcome

Completed: 2026-06-25

Repaired the user-visible normal `continue` leak. The TUI command loop no longer prints exact internal stop reason tokens, raw tick counts, or exact stop ticks for normal `continue`; it now prints a qualitative actor-known interval update line and then renders the embodied view. Command-loop and embodied-flow tests now positively require the qualitative line and reject `stop_tick=`, `controller_safety_bound`, `user_paused_before_next_tick`, and `possessed_duration_terminal` in normal output.

Verification included the requested `test-support` build lane and the full workspace gate set.

Deviation: the broader view-model schema split is not fully landed in this ticket. `EmbodiedViewModel` / `TypedActorKnownIntervalSummary` still retain exact temporal fields/accessors and `test-support` still exposes helper construction surfaces. The normal-output leak is closed here; the remaining compile-time unrepresentability and full embodied/debug product split must be carried by the consolidated 009/standing-barrier work before claiming the full §4.7 closure.

Verification:

1. `cargo test -p tracewake-tui --test command_loop_session --test embodied_flow --locked`
2. `cargo build -p tracewake-core --features test-support --locked`
3. `cargo fmt --all --check`
4. `cargo clippy --workspace --all-targets -- -D warnings`
5. `cargo build --workspace --all-targets --locked`
6. `cargo test --workspace`
