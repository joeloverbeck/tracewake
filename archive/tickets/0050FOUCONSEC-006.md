# 0050FOUCONSEC-006: Core owns final perception + interval product; remove TUI reconstruction

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — moves final perception + interval-summary construction into the core scratch transaction; removes TUI event/projection writes
**Deps**: 0050FOUCONSEC-003, 0050FOUCONSEC-005

## Problem

Spec-0050 §4.4 (driver F-04, de-authority half): after `advance_until` returns, the TUI re-calls `record_current_place_perception_and_project` at the scheduler's current tick, rebuilds an "after" context, and calls `actor_known_interval_delta` itself to build the displayed summary; it also appends perception after any accepted command. The perception helper (`crates/tracewake-core/src/agent/perception.rs`) creates perception events, appends them via `log.append()`, and applies them via `project_perception_event` (`perception.rs:553`). The authoritative interval product is split between core and TUI, and the client owns event creation and projection mutation — violating the read-only client boundary.

This ticket makes core return the final typed holder-known interval product from the step, routes accepted-action perception through a core-owned operation, and **removes** all TUI event/projection writes so the TUI only stores/renders the returned product. (The duplicate-`EventId` fail-closed enforcement is `-007`, which depends on this ticket having stopped the TUI from duplicating.)

## Assumption Reassessment (2026-06-24)

1. The TUI re-perception/re-delta path is in `crates/tracewake-tui/src/app.rs` (post-`advance_until` calls to `record_current_place_perception_and_project` and `actor_known_interval_delta`, plus perception append after accepted commands); the perception writer is `record_current_place_perception_and_project` + `project_perception_event` (`crates/tracewake-core/src/agent/perception.rs:553`). The typed interval delta/product lives across `crates/tracewake-core/src/projections.rs` and `crates/tracewake-core/src/view_models.rs`. Verified at `HEAD` (`8d7c119`).
2. Spec-0050 §4.4 and §9.4/§9.7 are authoritative: §9.4 (perception timing within the tick) and §9.7 (world-step vs interval result layering) assign the phase/layering choices to the implementer as recorded choices; the TUI must receive the final typed product and remain read-only either way.
3. Shared boundary under audit: the core↔TUI boundary — the world-step/interval result returned from `scheduler.rs` and consumed in `app.rs`, plus the perception/projection writers in `agent/perception.rs`/`projections.rs`/`view_models.rs`. Adjacent contradiction: `EventId` uniqueness (F-04 second half) is a required *consequence* but is `-007`'s scope (it can only enforce once the TUI stops duplicating, here).
4. `INV-069` (TUI must not implement simulation rules / mutate world state), `INV-067` (embodied mode shows actor-known reality), `INV-101` (actor-known context sealed), `INV-102` (cognition inputs require provenance), `INV-112` (holder-known temporal interpretation) motivate this ticket: event creation, projection mutation, and interval construction are core authority; the TUI consumes a sealed product.
5. Enforcement surface: actor-knowledge / no-leak firewall (`INV-067`/`INV-101`/`INV-102`) and deterministic replay (`INV-018`). The core-returned interval product must be a sealed holder-known delta (no debug/raw context); rendering must be read-only with no log/projection mutation. Replay from the resulting log must reconstruct physical/agent/epistemic/temporal frontiers with no client-side repair.
6. **Schema/contract shape change (additive-vs-breaking)**: the core step/interval result is extended to carry the final typed actor-known interval summary (or closed typed data to render it); consumers are `app.rs` (rewired here) and the holder-known interval projection tests. This is an additive extension to the step result plus a *removal* of the TUI-side reconstruction; both in-workspace, updated in this diff. The interval product is constructed in `projections.rs`/`view_models.rs`.
7. **Removal blast radius**: the TUI calls to `record_current_place_perception_and_project` and `actor_known_interval_delta`, and any post-accepted-command perception append in `app.rs`, are deleted; perception-appending helpers are no longer exported to the TUI crate. Grep blast radius: `app.rs` (rewired), the perception helper's visibility (narrowed to core), and the holder-known interval tests (assert read-only consumption). No `docs/`/`specs/` consumer references these calls.

## Architecture Check

1. A single core-owned holder-known interval product consumed read-only by the TUI restores the doctrine boundary: one authoritative embodied product, positively constructed, with the client unable to mutate world/projection state. This is cleaner than the split-authority status quo and removes an entire class of client-side drift/repair.
2. No backwards-compatibility shims: the TUI reconstruction path and perception-append exports are deleted, not wrapped. The TUI keeps only store/render.

## Verification Layers

1. `INV-069`/`INV-067` (read-only client; actor-known embodied output) → replay/golden-fixture check: record log length + projection checksum before the TUI consumes the result; rendering/store must not change either.
2. `INV-101`/`INV-102` (sealed, provenance-bearing) → manual review + actor-knowledge negative: paired hidden-world worlds with identical possessed-actor holder-known inputs produce identical typed interval products; a hidden event for another actor changes nothing.
3. `INV-018`/`INV-112` (deterministic replay; temporal interpretation) → replay/golden-fixture check: replay from the resulting log reconstructs physical/agent/epistemic/temporal frontiers with no client-side repair.

## What to Change

### 1. Return the final typed interval product from the core step

In `scheduler.rs` (+ `projections.rs`/`view_models.rs`), make the one-tick step / `advance_until` result carry the final typed holder-known interval summary (or closed typed data needed to render it without another projection query or event append). Generate final perception, interval-stop, and resume-boundary evidence inside the scratch transaction. Layering is the implementer's recorded choice per §9.7.

### 2. Route accepted-action perception through a core-owned operation

Move the post-accepted-action perception into a modeled core transition / explicit core-owned perception operation; narrow the perception-appending helper visibility so it is not exported to the TUI crate.

### 3. Remove TUI event/projection writes

In `crates/tracewake-tui/src/app.rs`, delete the post-`advance_until` `record_current_place_perception_and_project` and `actor_known_interval_delta` calls and the post-accepted-command perception append. The TUI stores/renders the returned product only.

### 4. Witnesses

In `crates/tracewake-core/tests/holder_known_interval_projection.rs` (and a TUI seam test as needed): the read-only consumption witness (log-length + projection-checksum unchanged by render/store) and the paired hidden-world noninterference witness.

## Files to Touch

- `crates/tracewake-core/src/scheduler.rs` (modify)
- `crates/tracewake-core/src/agent/perception.rs` (modify)
- `crates/tracewake-core/src/projections.rs` (modify)
- `crates/tracewake-core/src/view_models.rs` (modify)
- `crates/tracewake-tui/src/app.rs` (modify)
- `crates/tracewake-core/tests/holder_known_interval_projection.rs` (modify)

## Out of Scope

- Fail-closed duplicate-`EventId` enforcement — `0050FOUCONSEC-007` (depends on this ticket stopping TUI duplication first).
- Salience policy for the stop decision — `0050FOUCONSEC-009`.
- Compile-fail fixture forbidding TUI perception export / debug→embodied conversion — folded into `0050FOUCONSEC-010`'s compile-fail consolidation.

## Acceptance Criteria

### Tests That Must Pass

1. Read-only consumption witness: log length and projection checksum are unchanged by the TUI consuming/rendering the step result.
2. Paired hidden-world noninterference: identical possessed-actor holder-known inputs with different hidden events produce identical typed interval products.
3. Replay from the resulting log reconstructs all frontiers with no client repair; `cargo test -p tracewake-core --test holder_known_interval_projection`, the relevant `tracewake-tui` test, and `cargo build --workspace --all-targets --locked` green.

### Invariants

1. The TUI performs no event creation or projection mutation; the embodied interval product is a single sealed core-owned holder-known delta (`INV-069`/`INV-067`/`INV-101`).
2. Hidden world state never alters the actor-known interval product or rendered output (`INV-102`); replay needs no client repair (`INV-018`/`INV-112`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/holder_known_interval_projection.rs` — read-only consumption witness + paired hidden-world noninterference witness.

### Commands

1. `cargo test -p tracewake-core --test holder_known_interval_projection`
2. `cargo test -p tracewake-tui --test tui_seam_conformance`
3. `cargo build --workspace --all-targets --locked && cargo clippy --workspace --all-targets -- -D warnings`

## Outcome

Completed: 2026-06-24

`AdvanceUntilResult` now carries the core-built aggregate
`actor_known_interval_delta`. `DeterministicScheduler::advance_until` captures
the initial holder-known context before stepping and constructs the final delta
inside core for every stop path, using the final core projection/log frontier.
The TUI stores/renders that returned product instead of rebuilding an "after"
context or calling `actor_known_interval_delta` itself.

Removed direct TUI calls to `record_current_place_perception_and_project` and
`actor_known_interval_delta`. Needed current-view refreshes now route through
`DeterministicScheduler::record_actor_current_place_perception`, a core-owned
operation that performs perception event creation/projection mutation without
the TUI owning the helper. Added a TUI read-only consumption witness proving
that rendering after storing the core interval product does not change log
length or epistemic projection checksum.

The existing holder-known interval tests continue to cover hidden-world
noninterference and fail-closed source validation; the full TUI suite covers
the production render/command seams after removing direct TUI perception and
interval writes.

Verification run:

- `cargo test -p tracewake-core --test holder_known_interval_projection`
- `cargo test -p tracewake-tui --test tui_seam_conformance`
- `cargo test -p tracewake-tui`
- `rg -n "record_current_place_perception_and_project|actor_known_interval_delta\\(" crates/tracewake-tui/src` (no matches)
- `cargo build --workspace --all-targets --locked`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo fmt --all --check`
- `git diff --check`
