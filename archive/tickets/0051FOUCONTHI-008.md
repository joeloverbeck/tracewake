# 0051FOUCONTHI-008: F-07 sealed split temporal products

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — split the world-advance product into a sealed internal/debug receipt and a sealed actor-facing embodied summary; reseal all view-model fields to crate-private; remove the public struct-literal / `From` forging vectors.
**Deps**: 0051FOUCONTHI-007

## Problem

The default embodied renderer prints `Actor: <id> | Tick: <exact SimTick>` (`render.rs`), exact interval start/stop ticks, the internal stop-reason `stable_id`, and each notice's raw source `EventId`; a renderer test locks actor-facing `controller_safety_bound` output. Every field of `EmbodiedViewModel` and `TypedActorKnownIntervalSummary` is public (including `sim_tick`, frontiers, and `stop_reason`), and `IntervalStopReason`'s variants are public (`projections.rs:757`), so external code can forge an apparently embodied summary by a direct struct literal — empty `notices` with arbitrary ticks/frontiers and `stop_reason: ControllerSafetyBound` — without the verified holder-known constructor (F-07, high; hardening gap). The public `From<ActorKnownIntervalDelta>` is a secondary concern: its input has private fields + a crate-private `from_verified` constructor, so it is not itself an external forging vector — its defect is performing the internal→embodied conversion on the TUI side (`app.rs:370`) rather than in core. The fix splits the product into two sealed types and reseals the fields.

## Assumption Reassessment (2026-06-24)

1. Codebase: `render_embodied_view` prints `Actor: {} | Tick: {}` via `sim_tick.value()` (`crates/tracewake-tui/src/render.rs:13`+) and locks `controller_safety_bound` (`render.rs:397`); `EmbodiedViewModel` all-`pub` fields incl. `sim_tick`/`holder_known_context_frontier` (`crates/tracewake-core/src/view_models.rs`); `TypedActorKnownIntervalSummary` all-`pub` fields + `impl From<ActorKnownIntervalDelta>` (`view_models.rs:46`,`56`); `ActorKnownIntervalDelta` has private fields + `pub(crate) from_verified` (`crates/tracewake-core/src/projections.rs:827`,`840`); `IntervalStopReason` public variants (`projections.rs:757`); the TUI conversion at `crates/tracewake-tui/src/app.rs:370`. Existing fixtures `external_crate_cannot_forge_interval_notice`, `external_crate_cannot_convert_debug_report_to_interval_summary`, `external_crate_cannot_assign_scheduler_frontier` pin related seals (extend, do not duplicate).
2. Specs/docs: spec `0051` §4.7 (F-07), §9.6 (receipt ergonomics), §9.7 (actor-facing temporal language — both implementer-recorded choices). Architecture home `docs/1-architecture/10_*`; execution `docs/2-execution/07_*`.
3. Shared boundary under audit: the world-advance product crossing core→TUI — it must arrive already separated into a sealed debug receipt and a sealed embodied summary, neither externally constructible nor field-mutable.
4. INV-067 (embodied mode shows actor-known reality), INV-068 (debug mode is visibly non-diegetic), INV-093 (actor-knowledge leakage is a high-severity defect), INV-112 (temporal authority — exact ticks only when an actor-known source supports them): restated — exact replay ticks/frontiers, controller stop reason, and raw event IDs are operator/debug data, not embodied content.
5. Fail-closed / no-leak surface: this is the embodied-leak enforcement surface. Default embodied transcripts must omit exact replay tick/frontier, raw `EventId`, `controller_safety_bound`, and pause logistics; an information-flow differential must prove embodied output is unchanged when hidden ticks/debug receipts differ but the actor-known temporal source does not. `UserPausedBeforeNextTick` is client UI state; `ControllerSafetyBound` belongs to the operator/debug receipt.
6. Schema extension / reseal (additive-vs-breaking): make `EmbodiedViewModel` / `TypedActorKnownIntervalSummary` / the new `WorldAdvanceReceipt` / `EmbodiedIntervalSummary` fields private and constructors crate-private; expose read-only getters per surface; implement no debug→embodied conversion. **Breaking-internal**: public fields → crate-private is a visibility/shape change within the workspace; consumers are `render.rs` and `app.rs:370` (updated in the same diff — local compile-atomicity). No external workspace consumer exists. (Keyed distinctly from item 5 and item 7.)
7. Removal blast radius: remove the public all-fields construction surface and the public `From<ActorKnownIntervalDelta>` (relocated into core). Repo-wide grep: consumers are `render.rs` (field reads → getters) and `app.rs:370` (`From` → receives the already-separated product). No external alias retained.

## Architecture Check

1. Two sealed types — an internal/debug `WorldAdvanceReceipt` (exact ticks/frontiers, controller stop reason, appended event IDs, diagnostics) and an actor-facing `EmbodiedIntervalSummary` (only positively-constructed actor-legible notices and source-supported temporal language) — with private fields and crate-private constructors is the only design that makes a forged embodied summary unrepresentable and routes the conversion into core. Exact ticks appear in embodied mode only when a modeled actor-known clock/calendar/record supports them (§9.7 implementer-recorded choice).
2. No backwards-compatibility alias: the all-public product and the TUI-side `From` are removed, not retained; the TUI receives the already-separated product from core.

## Verification Layers

1. INV-093 (no actor-knowledge leakage) -> default-transcript assertions: absence of exact replay tick/frontier, raw `EventId`, `controller_safety_bound`, and pause logistics.
2. INV-068 (debug non-diegetic) -> debug-overlay test: those fields present only under explicit debug capability.
3. INV-067 / INV-112 (actor-known temporal source) -> information-flow differential: identical holder-known contexts with different hidden ticks/debug receipts yield unchanged embodied output unless an actor-known temporal source differs.
4. Seal -> negative fixtures: external crates cannot construct embodied summaries, mutate source/frontier fields, or convert debug receipts to embodied products.

## What to Change

### 1. Split into two sealed types

Split the current product into at least an internal/debug `WorldAdvanceReceipt` and an actor-facing `EmbodiedIntervalSummary` (names per §9.6). `UserPausedBeforeNextTick` is client UI state; `ControllerSafetyBound` belongs to the operator/debug receipt; raw event IDs remain internal provenance handles.

### 2. Reseal fields + getters

Make fields private and constructors crate-private; expose read-only getters per surface; implement no debug→embodied conversion; the TUI receives the already-separated product from core (conversion moves out of `app.rs:370`).

### 3. Renderer

The embodied renderer stops emitting exact `Tick:`/frontier/`controller_safety_bound`; the debug overlay carries them.

## Files to Touch

- `crates/tracewake-core/src/view_models.rs` (modify) — split + reseal `EmbodiedViewModel`/`TypedActorKnownIntervalSummary`; new sealed types
- `crates/tracewake-core/src/projections.rs` (modify) — `IntervalStopReason` routing; relocate the delta→summary conversion into core
- `crates/tracewake-tui/src/render.rs` (modify) — embodied vs debug-overlay field routing; merge-hub contributor
- `crates/tracewake-tui/src/app.rs` (modify) — receive the already-separated product; remove the `From` use at `370`; merge-hub contributor
- `crates/tracewake-core/tests/holder_known_interval_projection.rs` (modify) — leak-absence + debug-presence + information-flow differential
- `crates/tracewake-core/tests/negative_fixture_runner.rs` (modify) — extend the forge/mutate/convert seals; merge-hub contributor

## Out of Scope

- The `#15–17` / `#47–48` mutation witnesses + focused kills at this production boundary (owned by F-08 → `-010`; this ticket creates the sealed boundary they exercise).
- TUI write-path removal (F-06 → `-007`).

## Acceptance Criteria

### Tests That Must Pass

1. A default embodied transcript contains no exact replay tick/frontier, raw `EventId`, `controller_safety_bound`, or pause logistics; the debug overlay shows them under explicit debug capability.
2. An information-flow differential proves embodied output is unchanged when hidden ticks/debug receipts differ but the actor-known temporal source does not.
3. Negative fixtures prove external crates cannot construct embodied summaries, mutate source/frontier fields, or convert debug receipts to embodied products.
4. `cargo test --workspace` is green.

### Invariants

1. Embodied and debug products are distinct sealed types with crate-private constructors.
2. No debug→embodied conversion exists; exact ticks appear in embodied mode only with an actor-known source.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/holder_known_interval_projection.rs` — leak-absence, debug-presence, information-flow differential.
2. `crates/tracewake-core/tests/negative_fixture_runner.rs` — extended forge/mutate/convert seals.

### Commands

1. `cargo test -p tracewake-core --test holder_known_interval_projection`
2. `cargo test -p tracewake-core --test negative_fixture_runner`
3. `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings`

## Outcome

Completed: 2026-06-24

Implemented the F-07 temporal-product seal and embodied no-leak cutover. `EmbodiedViewModel` now seals replay/control metadata fields (`view_model_id`, `mode`, `viewer_actor_id`, `sim_tick`, holder-known context id/hash/frontier/source summary, `actor_known_interval_summary`, and `debug_available`) behind read-only accessors and controlled setters. `TypedActorKnownIntervalSummary` now seals exact ticks/frontiers, stop reason, notices, and the no-new-information flag behind accessors. The former public `From<ActorKnownIntervalDelta>` conversion was removed; the TUI now receives the actor-facing interval summary through `TypedActorKnownIntervalSummary::from_actor_known_delta`, whose input remains core-verified and unforgeable.

The existing scheduler advance result remains the debug/control receipt carrying exact stop ticks and stop reasons for command/debug surfaces. The actor-facing interval summary is now rendered without exact replay ticks, frontier values, raw `EventId`s, `controller_safety_bound`, or pause logistics. The debug overlay continues to show exact tick/frontier details only when debug capability is active.

Added `tracewake-core/test-support` for synthetic TUI unit fixtures only; production and negative-fixture builds do not enable it, so external crates still cannot construct or mutate sealed temporal products. Negative fixtures now cover direct actor interval summary construction and mutation of embodied temporal/debug fields, and the existing debug-report conversion fixture is pinned to the removed conversion path.

Verification run:

1. `cargo test -p tracewake-core --test holder_known_interval_projection --quiet` — passed.
2. `cargo test -p tracewake-core --test negative_fixture_runner --quiet` — passed.
3. `cargo test -p tracewake-tui --quiet` — passed.
4. `cargo test --workspace --quiet` — passed.
5. `cargo build --workspace --all-targets --locked` — passed.
6. `cargo clippy --workspace --all-targets -- -D warnings` — passed.
7. `cargo fmt --all --check` — passed.
