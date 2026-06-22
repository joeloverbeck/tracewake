# 0046TUISIMPLA-001: Hop-2 exhaustive `EmbodiedViewModel` destructure at the renderer boundary + source-conformance guard

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-tui` renderer (`crates/tracewake-tui/src/render.rs`) and seam-conformance guard (`crates/tracewake-tui/tests/tui_seam_conformance.rs`); no production-crate (`tracewake-core`/`tracewake-content`) change.
**Deps**: None

## Problem

Spec 0046 §4.1 `PAR-001`/`PAR-002`. `render_embodied_view(view: &EmbodiedViewModel) -> String`
(`crates/tracewake-tui/src/render.rs:12`) reads fields à la carte (`view.viewer_actor_id`,
`view.place_label`, `view.last_rejection_why_not`, …) with **no** leading exhaustive destructure.
`EmbodiedViewModel` (`crates/tracewake-core/src/view_models.rs:19`) is a closed workspace struct with
**21 public fields** and no `#[non_exhaustive]`. Because the renderer never names the field set,
adding a 22nd field to the actor-filtered view model compiles silently with that field rendered
nowhere — the projection→render "Hop 2" drift the spec closes. The intended tripwire is a deliberate
compile break at the in-workspace core→TUI boundary whenever the contract grows: every field gets a
conscious render-or-skip disposition. A second failure mode is a later "cleanup" that deletes the
destructure while preserving identical output; nothing today would catch that, so the destructure
needs a source-conformance guard in the existing `tui_seam_conformance.rs` style.

## Assumption Reassessment (2026-06-22)

1. Verified against code at baseline `1145e109`: `render_embodied_view` at
   `crates/tracewake-tui/src/render.rs:12` opens `let mut lines = Vec::new();` and reads fields
   directly (`view.viewer_actor_id.as_str()`, `view.place_label`, `view.last_rejection_why_not`) with
   no destructure and no local lint attribute. `EmbodiedViewModel` has 21 public fields and no
   `#[non_exhaustive]` (`crates/tracewake-core/src/view_models.rs:19`). The sibling
   `visible_item_source_label` (`render.rs:186`) already matches its enum exhaustively with no
   wildcard — the disposition discipline this ticket generalizes to the struct.
2. Verified against spec 0046 §4.1 (`PAR-001`/`PAR-002`) and §3.1 verified holdings; the spec's §1.2
   non-goal forbids marking `EmbodiedViewModel` `#[non_exhaustive]` — the destructure-plus-`deny`
   tripwire is the chosen mechanism precisely because it forces deliberate breakage rather than
   granting downstream tolerance (§3.2).
3. Shared boundary under audit: the closed core→TUI presentation contract
   (`EmbodiedViewModel` ↔ `render_embodied_view`). Every field receives exactly one disposition —
   rendered/consumed, or an underscore-prefixed named binding with an adjacent rationale naming the
   correct alternate owner or why it is internal metadata. A bare `_`, `field: _`, or `..` rest
   pattern for omission is forbidden.
4. Invariant restated before trusting the narrative (`PAR-001` motivation): `INV-066` — every runnable
   phase's mechanics must be reachable/inspectable through the TUI or the same actor-filtered view
   models; `INV-069` — the TUI consumes actor-filtered view models and must not implement simulation
   rules. The destructure is presentation-completeness enforcement, not a new rule: it renders from
   bound names, adding no world logic.
5. Enforcement surface touched: the renderer boundary is the Hop-2 leg of the no-leak firewall —
   `render_embodied_view` may present only actor-known fields of `EmbodiedViewModel`
   (`INV-067`/`INV-093`). This ticket strengthens completeness and weakens nothing: it adds no field
   access beyond what the struct already exposes to the renderer, introduces no hidden-truth/debug
   read, and changes no projection. The `unused_variables` `deny` is scoped local to the renderer (or
   its module), independent of workspace CI, so the tripwire fires even outside `-D warnings`.

## Architecture Check

1. An exhaustive `let EmbodiedViewModel { field_a, field_b, .. forbidden } = view;` destructure plus a
   renderer-local `#![deny(unused_variables)]` (or `#[deny(unused_variables)]` on the function/module)
   is cleaner than `#[non_exhaustive]` (which grants tolerance, the opposite of the wanted breakage —
   §3.2) and cleaner than a hand-maintained field-count assertion (which a new field with a default
   render path satisfies vacuously). The compiler enforces the disposition; the source guard enforces
   the destructure's continued existence.
2. No backwards-compatibility aliasing/shims: the à-la-carte `view.field` reads are replaced by the
   bound names, not kept as a fallback path; no second render entry point is introduced.

## Verification Layers

1. `INV-066`/`PAR-001` (field completeness) → compiler proof: a temporary 22nd field on
   `EmbodiedViewModel` fails to compile in `render.rs` until dispositioned (exercised as the controlled
   compile-break in `0046TUISIMPLA-012`; asserted here by the destructure naming all 21 fields).
2. `PAR-002` (tripwire durability) → codebase grep-proof / source-conformance test in
   `tui_seam_conformance.rs`: the renderer source contains the exhaustive destructure, and the pattern
   contains no `..` and no bare-wildcard field disposition.
3. `INV-067`/`INV-093` (no-leak at render) → manual review + existing `tui_acceptance.rs` /
   `adversarial_gates.rs` regression: the destructure adds no new field read and no hidden/debug-truth
   path; embodied output is unchanged byte-for-byte.

## What to Change

### 1. Exhaustive destructure at the start of `render_embodied_view`

Open the function body with `let EmbodiedViewModel { /* all 21 fields named */ } = view;` (binding by
reference). Render from the bound names. For each field not rendered on the embodied surface, bind it
as `_field_name` with an adjacent line-comment rationale identifying the correct alternate owner (e.g.
notebook view, debug overlay) or why it is internal metadata. No `..`, no `field: _`, no bare `_`.

### 2. Renderer-local `unused_variables` deny

Add `#![deny(unused_variables)]` at the renderer module scope (or `#[deny(unused_variables)]` on
`render_embodied_view`), so an undispositioned bound field fails the build locally, independent of the
workspace `-D warnings` posture.

### 3. Source-conformance guard for the destructure (`PAR-002`)

Add a `TuiSeamEvidence`-style entry / test in `crates/tracewake-tui/tests/tui_seam_conformance.rs`
that `include_str!`s `../src/render.rs` and asserts: (a) `render_embodied_view`'s body contains the
`let EmbodiedViewModel {` destructure; (b) that destructure contains no `..`; (c) no bare-wildcard
field disposition. Phrase the `acceptance_condition` so a "cleanup" deleting the destructure fails the
test while preserving output.

## Files to Touch

- `crates/tracewake-tui/src/render.rs` (modify)
- `crates/tracewake-tui/tests/tui_seam_conformance.rs` (modify)

## Out of Scope

- Closed-enum exhaustive presentation matches (`PAR-003`) — ticket `0046TUISIMPLA-002` (shares
  `render.rs` and `tui_seam_conformance.rs`; coordinate the mechanical merge).
- Real-pipeline goldens, registry, runner, census — tickets `0046TUISIMPLA-003`…`007`.
- Marking `EmbodiedViewModel` `#[non_exhaustive]` (spec §1.2 non-goal).
- Any change to `EmbodiedViewModel`'s field set or to `build_embodied_view_model` projection.

## Acceptance Criteria

### Tests That Must Pass

1. Grep-proof: `render_embodied_view` body contains `let EmbodiedViewModel {` naming all 21 fields
   (re-count from `crates/tracewake-core/src/view_models.rs` at implementation, do not hardcode), with
   no `..` and no bare `_`/`field: _` omission; a renderer-local `deny(unused_variables)` is present.
2. The `tui_seam_conformance.rs` source guard passes and fails when the destructure is removed
   (demonstrated via a scratch deletion during implementation).
3. `cargo test -p tracewake-tui --test tui_seam_conformance` and the four gates
   (`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo build --workspace --all-targets --locked`, `cargo test --workspace`) pass; embodied render
   output is unchanged (existing `tui_acceptance.rs` / `transcript_snapshot.rs` remain green).

### Invariants

1. No field of `EmbodiedViewModel` can be added without a conscious render-or-skip disposition in
   `render_embodied_view` (compiler-enforced via the destructure + local deny).
2. The destructure cannot be removed while keeping the suite green (source-conformance guard).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/tui_seam_conformance.rs` — source-conformance entry asserting the
   exhaustive destructure exists with no `..`/wildcard disposition.
2. `crates/tracewake-tui/src/render.rs` — the destructure + local deny is itself the compile-time guard;
   existing render unit tests confirm unchanged output.

### Commands

1. `cargo test -p tracewake-tui --test tui_seam_conformance`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
