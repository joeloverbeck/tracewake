# 0053FOUCONFIF-005: Core-owned immutable embodied interval/receipt products (atomic cutover)

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` receipt/view-model authority surface (sealed embodied receipt, removal of public exact getters + mutators, core-owned interval construction); `tracewake-tui` consumer migration; extended external negative fixture
**Deps**: 0053FOUCONFIF-004

## Problem

Spec 0053 §4.2 (F5-02): `RuntimeReceiptKind::Continued(AdvanceUntilResult)` publicly carries the raw advance result; `TuiApp::advance_until` constructs `TypedActorKnownIntervalSummary::from_actor_known_delta` client-side; and `EmbodiedViewModel` / `TypedActorKnownIntervalSummary` expose public exact getters (start/stop tick, start/stop frontier, stop reason) and public mutators (`set_actor_known_interval_summary`, `set_debug_available`, `set_notebook`). The leak path is the public `Continued` receipt → public `AdvanceUntilResult.actor_known_interval_delta` field → public delta getters + public `from_actor_known_delta` (the standalone `ActorKnownIntervalDelta::from_verified` is correctly `pub(crate)`, so the leak is through the receipt, not standalone construction; §0 F5-02 as refined at reassessment). The normal renderer hides exact values, but hiding-on-render is not sealing-at-the-boundary.

This ticket seals the product boundary so external clients receive opaque, immutable, actor-known interval data and cannot read exact temporal internals or assemble the product themselves — an atomic cutover, since removing the public getters/mutators and resealing the receipt requires migrating the TUI consumer in the same diff (§4.2, §9 step 4). Exact tick/frontier/stop/replay detail reaches only debug/operator callers through a separately token-gated debug receipt — that token gating is ticket 006 (F5-03); this ticket establishes the sealed embodied receipt and the debug-receipt split point.

## Assumption Reassessment (2026-06-26)

1. `crates/tracewake-core/src/runtime/receipt.rs` defines `pub enum RuntimeReceiptKind` (line 16) with `OneTickAdvanced(WorldAdvanceResult)` (17) and `Continued(AdvanceUntilResult)` (19). `crates/tracewake-core/src/view_models.rs` exposes `TypedActorKnownIntervalSummary::from_actor_known_delta` (pub, line 189), exact getters `start_tick`/`stop_tick`/`start_frontier`/`stop_frontier`/`stop_reason` (all pub), and public mutators `set_notebook` (140) / `set_debug_available` (144) / `set_actor_known_interval_summary` (148). `crates/tracewake-tui/src/app.rs:220` `advance_until` matches `Continued` (231) and calls `from_actor_known_delta` (237). **Blast radius (grepped this session)**: the setters / `from_actor_known_delta` are consumed in `view_models.rs`, `runtime/session.rs`, `tui/src/app.rs`, and `tests/anti_regression_guards.rs`. `ActorKnownIntervalDelta::from_verified` is `pub(crate)` (projections.rs:840).
2. Spec authority: `specs/0053_…_HARDENING_SPEC.md` §4.2 (four required changes + anti-regression guard), §10.3 (which temporal fields, if any, are legitimately actor-known via a modeled source — **implementer-recorded** within this ticket; default: none actor-known unless architecture models a specific value via a source, everything else debug/operator-only), §9 step 4. Reassessment improvement M1: **extend** the existing `tests/negative-fixtures/external_crate_cannot_construct_actor_interval_summary` fixture rather than add a duplicate.
3. Cross-artifact boundary under audit: the `tracewake-core` view-model/receipt product surface consumed by `tracewake-tui` (`app.rs`, `render.rs`). Core owns the sealed product; the TUI receives and renders it and never assembles it. Core must not depend on tui.
4. Motivating invariants: INV-008 (UI assistance is not authority), INV-030 (evidence is not truth), INV-031 (human/debug notes are non-diegetic), INV-067 (embodied mode shows actor-known reality), INV-068 (debug mode is visibly non-diegetic) — the TUI renders but must not assemble or read hidden temporal truth.
5. This ticket touches the no-leak / actor-knowledge enforcement surface (INV-024 no telepathy, INV-093 leakage is high-severity): exact tick/frontier/stop/replay detail must not reach the embodied (non-debug) client. The reseal strengthens the firewall — it removes the public getters and the public `from_actor_known_delta`, moves construction into core via crate-private builders, and confines exact fields to the debug receipt (token-gated in 006). No replay nondeterminism is introduced: the sealed product is a deterministic projection of the actor-known delta; serialization/hash semantics are unchanged.
6. Schema/visibility change (additive-vs-breaking): the embodied product's public read surface (exact getters) and write surface (the three setters) are removed; `RuntimeReceiptKind::Continued` no longer publicly exposes the raw `AdvanceUntilResult`. Consumers are the TUI (`app.rs`, `render.rs`) and the in-core/test setter call sites (`session.rs`, `anti_regression_guards.rs`). This is **breaking-internal** (deliberate reseal; no external stability owed; no alias).
7. Removal blast radius (grep-proof, item-7): deleting public `set_actor_known_interval_summary` / `set_debug_available` / `set_notebook`, the public exact getters, and public `from_actor_known_delta` access — consumers in item 1 migrate to core-private builders (production) or `test-support`-gated read accessors (tests). No public mutator or exact getter survives on the embodied product.

## Architecture Check

1. Sealing at the boundary (opaque immutable product + core-private builders) is cleaner than the current hide-on-render: render-time hiding leaves the exact fields publicly readable, so any non-TUI client (or a future TUI bug) can read hidden temporal truth. Moving construction into core and removing the public getters makes the leak *unrepresentable*, mirroring the bootstrap reseal (004). The atomic cutover is required because a split leaving the getters live behind a deprecated path would re-create the leak.
2. No backwards-compatibility aliasing or durable shim. Test read-access uses `#[cfg(feature = "test-support")]` accessors, not public production getters.

## Verification Layers

1. INV-024/INV-067/INV-093 (no telepathy / actor-known / leakage) -> external negative fixtures (compile-fail): an external crate cannot call `from_actor_known_delta`, cannot call the exact tick/frontier/stop getters, cannot call the embodied setters, and cannot pattern-match a public `RuntimeReceiptKind` to extract raw `AdvanceUntilResult` exact fields.
2. INV-008/INV-031/INV-068 (UI not authority / debug non-diegetic) -> TUI behavior test: the normal `advance_until` / command-loop `continue` path receives an actor-known update and cannot render exact internals; render tests retained but not sole certification.
3. Mutation sensitivity -> mutation check: every surviving "return default tick/frontier/stop reason" mutant on a public product accessor is killed through a public behavior witness or removed from the public embodied API.
4. Cross-artifact: the TUI receives an already-sealed product and never calls `from_actor_known_delta` (grep-proof: no `from_actor_known_delta` call in `tracewake-tui`).

## What to Change

### 1. Sealed receipt surface (`runtime/receipt.rs`)

Replace public `RuntimeReceiptKind::Continued(AdvanceUntilResult)` exposure with a sealed public receipt: normal callers receive an embodied receipt carrying only actor-known, source-bearing qualitative interval data; exact tick/frontier/stop/replay detail reaches only debug/operator callers through a separately token-gated debug receipt (the token gating is 006; this ticket defines the split and the embodied side).

### 2. Core-owned interval construction (`view_models.rs`, `projections.rs`)

Move `TypedActorKnownIntervalSummary` construction fully into core (crate-private builder); the TUI receives and stores an already-sealed product (or opaque renderable DTO) and never calls `from_actor_known_delta`. Remove public `from_actor_known_delta`.

### 3. Remove/restrict public exact getters + mutators (`view_models.rs`)

Remove the public exact getters (tick/frontier/stop reason) from the embodied product unless architecture explicitly models a specific value as actor-known via a source (§10.3 recorded decision; default none). Remove the public mutators `set_actor_known_interval_summary`, `set_debug_available`, `set_notebook` — core uses crate-private builders and public clients receive immutable products. Provide `#[cfg(feature = "test-support")]` read accessors for tests only.

### 4. TUI consumer migration (`tui/src/app.rs`, `tui/src/render.rs`) + setter call-site migration (`runtime/session.rs`)

`TuiApp::advance_until` receives the sealed product instead of constructing the summary; `render.rs` renders the opaque product. Migrate the in-core setter call sites in `session.rs` to crate-private builders. Update `tests/anti_regression_guards.rs` assertions to the sealed surface.

### 5. Extend the existing negative fixture (`tests/negative-fixtures/external_crate_cannot_construct_actor_interval_summary`) + runner

Extend it (M1) to cover the live path — public `Continued` receipt / `AdvanceUntilResult.actor_known_interval_delta` field, the delta getters, and `from_actor_known_delta` — not just struct-literal construction. Update `negative_fixture_runner.rs` registration as needed.

## Files to Touch

- `crates/tracewake-core/src/runtime/receipt.rs` (modify)
- `crates/tracewake-core/src/view_models.rs` (modify)
- `crates/tracewake-core/src/projections.rs` (modify)
- `crates/tracewake-core/src/runtime/session.rs` (modify — setter call-site migration; created/sealed in 004, `Deps: 004`)
- `crates/tracewake-tui/src/app.rs` (modify)
- `crates/tracewake-tui/src/render.rs` (modify)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)
- `crates/tracewake-core/tests/negative_fixture_runner.rs` (modify)
- `tests/negative-fixtures/external_crate_cannot_construct_actor_interval_summary/src/lib.rs` (modify — extend per M1)

## Out of Scope

- Token-gating the debug receipt / debug-view constructors and no-human command authority (006) — this ticket defines the embodied/debug split point; 006 installs the token.
- The bootstrap reseal (004), `food_source` survivors (007), mutation run (009), doc-truthing (008).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --test negative_fixture_runner` — the extended `external_crate_cannot_construct_actor_interval_summary` fixture fails to compile when calling `from_actor_known_delta`, the exact getters, the setters, or extracting raw `AdvanceUntilResult` exact fields from a public `Continued` receipt.
2. `grep -rnE "from_actor_known_delta" crates/tracewake-tui/src` returns nothing (TUI no longer assembles the product).
3. `grep -nE "pub fn set_(actor_known_interval_summary|debug_available|notebook)|pub fn from_actor_known_delta" crates/tracewake-core/src/view_models.rs` returns nothing (public mutators + public constructor removed).
4. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` — clean.

### Invariants

1. No public path lets an embodied (non-debug) client read exact tick/frontier/stop detail or assemble the interval product (INV-024/067/093).
2. The TUI renders but never constructs the actor-known interval product (INV-008/069).

## Test Plan

### New/Modified Tests

1. `tests/negative-fixtures/external_crate_cannot_construct_actor_interval_summary/src/lib.rs` (modify) — extended compile-fail coverage of the receipt/getter/`from_actor_known_delta` path.
2. `crates/tracewake-tui/` behavior test — `advance_until` / `continue` receives an actor-known update and cannot render exact internals.
3. `crates/tracewake-core/tests/anti_regression_guards.rs` (modify) — assert the sealed surface; default-transcript exact-leak-absence.

### Commands

1. `cargo test -p tracewake-core --test negative_fixture_runner`
2. `cargo test -p tracewake-tui`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
