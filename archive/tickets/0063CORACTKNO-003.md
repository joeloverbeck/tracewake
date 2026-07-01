# 0063CORACTKNO-003: `VisibleActor` presence/activity view and `build_embodied_view_model` transfer

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — extends `VisibleActor` (core view model) with a presence/activity view; transfers sealed actor-known activity through `build_embodied_view_model`; updates `EmbodiedProjectionSource`
**Deps**: 0063CORACTKNO-002, 0063CORACTKNO-001

## Problem

The embodied view model must surface the sealed actor-known activity so the TUI can render it
without deriving anything itself (Spec 0063 §1.1, §1.4; INV-008, INV-069). Today `VisibleActor`
is identity-only (`crates/tracewake-core/src/view_models.rs:390` — `{ pub actor_id: ActorId }`),
`EmbodiedProjectionSource.actor_known_local_actors` is a bare `Vec<ActorId>`
(`crates/tracewake-core/src/projections.rs:90`), and `build_embodied_view_model` maps it to
`VisibleActor { actor_id }` (`projections.rs:693`), dropping everything ticket 002 sealed into
`ActorKnownLocalActorFact`. This ticket extends `VisibleActor` with a `display_label`, a presence
descriptor (source summary, observed tick, staleness label, optional uncertainty), and an optional
observed-activity view (closed activity kind, actor-safe summary, source kind, source summary,
observed tick, staleness, uncertainty), then transfers the sealed activity from the holder-known
fact into `VisibleActor` — preserving deterministic ordering (§1.4, §8). This is Hop 1's landing
point (core event/perception → `VisibleActor.observed_activity`, §6 two-hop).

## Assumption Reassessment (2026-07-01)

1. `VisibleActor` (`crates/tracewake-core/src/view_models.rs:390`) is identity-only. The sole
   non-test construction site is `build_embodied_view_model`
   (`crates/tracewake-core/src/projections.rs:693`, `.map(|actor_id| VisibleActor { actor_id })`),
   fed by `EmbodiedProjectionSource.actor_known_local_actors: Vec<ActorId>` (`projections.rs:90`,
   built by `actor_known_local_actors_for_context`, `projections.rs:412`, which reads
   `context.actor_known_local_actors() -> &[ActorKnownLocalActorFact]`). After ticket 002 the fact
   carries observed activity + source + freshness, so the transfer can carry it end-to-end.
2. Spec 0063 §1.1 fixes the required properties (not the field names, which are authoring choices):
   closed enums (ticket 001), actor-safe summaries, source/freshness labels, and **no UI-framework
   type** in the view model (§1.2, §8 — zero-UI-dependency posture preserved). §4.4: each activity
   kind/source needs an explicit presentation disposition — enforced downstream by the Spec 0069
   conformance guard; this ticket only lands the closed-typed fields that force it.
3. **Shared boundary under audit:** the transfer seam `ActorKnownLocalActorFact (sealed, ticket 002)
   → EmbodiedProjectionSource → VisibleActor`. Additive required fields on `VisibleActor` are
   compile-forced onto every constructor in the workspace: besides `projections.rs:693`, three
   `#[cfg(test)]` constructors in `tracewake-tui` build `VisibleActor { … }` —
   `screen/struct_dump.rs:178`, `screen/text_dump.rs:420`, `screen/model.rs:333` — and must be
   updated in this same diff (cross-crate local-compile-atomicity). The dump *read* paths
   (`screen/text_dump.rs:102`, `screen/model.rs:205`) read only `.actor_id` and are additive-safe;
   the real dump/render **disposition** of the new fields is ticket 005 (sequential-edit after this
   ticket, not a parallel merge).
4. **Motivating invariants.** INV-008 / INV-069 (UI assistance is not authority; the TUI implements
   no rules): the view model carries fully-resolved, closed-typed activity so no TUI derivation is
   possible. INV-024 / INV-067 (no telepathy; embodied mode shows actor-known reality): `VisibleActor`
   is populated only from the sealed holder-known fact, never from truth.
5. **Enforcement surface — actor-knowledge filtering + deterministic ordering.** The transfer in
   `build_embodied_view_model` must read only `context.actor_known_local_actors()` (the sealed fact),
   never a truth handle, preserving the existing firewall. Determinism (INV-018, §8): `local_actors`
   is currently `sort()`ed after construction (`projections.rs:695`); the new fields must keep a
   total order (ticket 001's enums derive `Ord`) so both the actor rows and their activity rows are
   deterministically ordered and the screen dump stays byte-stable.
6. **Schema extension (additive).** Extends the `VisibleActor` view-model projection. Consumers:
   `build_embodied_view_model` (producer), `crates/tracewake-tui/src/render.rs:129` (embodied read),
   and the 0061 screen-dump surfaces (`screen/model.rs` `EmbodiedScreenModel.local_actors`,
   `screen/text_dump.rs`, `screen/struct_dump.rs`). Extension is **additive** — new fields with an
   "identity-only, no activity observed" default shape (`observed_activity: None`, an
   `ActivityNotApparent`-equivalent presence) — so the existing dumps keep rendering identity
   unchanged until ticket 005 disposes the new fields. No back-compat shim (§8).

## Architecture Check

1. Resolving activity into closed-typed `VisibleActor` fields inside the kernel projection — rather
   than passing raw facts to the TUI or letting the TUI join `actor_id` against other state — keeps
   all epistemic resolution on the authoritative side of the boundary and leaves the TUI a pure
   consumer (INV-069). Threading through the existing `EmbodiedProjectionSource` carrier reuses the
   established sort/dedup determinism rather than inventing a parallel ordering.
2. No backwards-compatibility aliasing/shims: `VisibleActor` gains fields additively; the identity
   -only case is the default field shape, not a legacy path. The tui test-constructor updates are
   mechanical field additions, not compatibility wrappers.

## Verification Layers

1. INV-008 / INV-069 (no TUI authority) -> codebase grep-proof: `VisibleActor` carries closed-typed
   activity + labels; `render.rs` and the dumps read fields only, join nothing.
2. INV-024 / INV-067 (actor-known only) -> codebase grep-proof + manual review: the transfer reads
   only `context.actor_known_local_actors()`; no truth handle appears in `build_embodied_view_model`.
3. INV-018 (deterministic ordering, §8) -> replay/golden-fixture check: `cargo test -p tracewake-core`
   view-model tests and `cargo test -p tracewake-tui` screen-dump tests prove actor + activity rows
   are deterministically ordered and dumps stay byte-stable.

## What to Change

### 1. Extend `VisibleActor`

In `crates/tracewake-core/src/view_models.rs`, add to `VisibleActor`: `display_label: String`; a
presence view (source summary, observed tick, staleness label, `Option` uncertainty label); and
`observed_activity: Option<ObservedActivityView>` where `ObservedActivityView` bundles the closed
`ObservedActorActivityKind`, an actor-safe summary, the `ActorKnownActivitySourceKind`, a source
summary, observed tick, staleness, and uncertainty (all from ticket 001; names are authoring
choices per §1.1). No `ratatui`/`crossterm` or any UI type (§1.2, §8). Keep total-order/`Hash`
derives for deterministic ordering.

### 2. Carry activity through `EmbodiedProjectionSource` and the transfer

In `crates/tracewake-core/src/projections.rs`, change `EmbodiedProjectionSource.actor_known_local_actors`
from `Vec<ActorId>` to carry the sealed activity (e.g. `Vec<ActorKnownLocalActorFact>` or a typed
projection struct); update `actor_known_local_actors_for_context` (line 412) to return it and
`build_embodied_view_model` (line 689) to construct each `VisibleActor` from the fact, resolving the
presence/activity view, then `sort()` as today (line 695).

### 3. Update compile-forced tui test constructors

In `crates/tracewake-tui/src/screen/{struct_dump,text_dump,model}.rs`, update the `#[cfg(test)]`
`VisibleActor { … }` constructors (struct_dump.rs:178, text_dump.rs:420, model.rs:333) to supply the
new fields with the identity-only/no-activity default shape. Do **not** add dump rendering of the new
fields here — that is ticket 005.

## Files to Touch

- `crates/tracewake-core/src/view_models.rs` (modify)
- `crates/tracewake-core/src/projections.rs` (modify)
- `crates/tracewake-tui/src/screen/struct_dump.rs` (modify — compile-forced test constructor)
- `crates/tracewake-tui/src/screen/text_dump.rs` (modify — compile-forced test constructor)
- `crates/tracewake-tui/src/screen/model.rs` (modify — compile-forced test constructor)

## Out of Scope

- Rendering/disposing the new fields in `render.rs` or the screen dumps (ticket 005).
- Content fixtures (ticket 004) and anti-leak/possession-parity/two-hop assertions (ticket 005).
- Producing/sealing activity into the fact (ticket 002) — this ticket consumes it.
- Identity-uncertain presence rows (deferred, §9.1 #3).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-core --lib view_models projections` — `VisibleActor` carries the
   presence/activity view; `build_embodied_view_model` transfers a sealed fact's activity into
   `VisibleActor.observed_activity` (Hop 1), and a fact with no activity yields the identity-only /
   `ActivityNotApparent` shape.
2. `cargo test -p tracewake-tui` — the screen-dump suite compiles and passes with the additive
   fields (byte-stable identity dumps unchanged).
3. `cargo build --workspace --all-targets --locked` — the whole workspace compiles (proves the
   cross-crate additive field change is complete).

### Invariants

1. `VisibleActor` contains only closed-typed / plain-string presentation data — no UI-framework
   type, no truth handle (INV-008, INV-069, INV-024).
2. Actor rows and their activity views are deterministically ordered; identity-only dumps are
   byte-identical to pre-change output (INV-018).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/projections.rs` (`#[cfg(test)]`) — transfer test: sealed fact with
   activity → `VisibleActor.observed_activity`; sealed fact without → identity-only view (Hop 1).
2. `crates/tracewake-tui/src/screen/model.rs` (`#[cfg(test)]`) — updated constructor; identity dump
   equality (`screen.actors.local_actors == view.local_actors`) still holds.

### Commands

1. `cargo test -p tracewake-core --lib view_models projections`
2. `cargo build --workspace --all-targets --locked && cargo test -p tracewake-tui`
3. The two-crate boundary (core transfer + tui compile) is correct here because the field addition
   is compile-forced across both; the end-to-end Hop 2 render/dump disposition is proven by ticket
   005 and the capstone (006).

## Outcome

Completed: 2026-07-01

Extended `VisibleActor` with actor-known presence metadata and an optional `ObservedActivityView`
without adding UI-framework types or TUI-side derivation. `EmbodiedProjectionSource` now carries the
sealed `ActorKnownLocalActorFact` values instead of bare actor IDs, and `build_embodied_view_model`
constructs each `VisibleActor` from those sealed facts. Activity-not-apparent facts remain
presence-only (`observed_activity: None`), while a sealed activity fact transfers into
`VisibleActor.observed_activity` with source, tick, staleness, and uncertainty fields. TUI test
constructors were updated with the identity-only helper; render/dump disposition is still reserved
for ticket 005.

Verification:

- `cargo test -p tracewake-core --lib view_models` — passed.
- `cargo test -p tracewake-core --lib projections` — passed.
- `cargo test -p tracewake-tui` — passed.
- `cargo fmt --all --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo build --workspace --all-targets --locked` — passed.

Deviation recorded: Cargo accepts one test filter per invocation, so the ticket's combined
`view_models projections` filter was run as two separate filters with the same package/lib boundary.
