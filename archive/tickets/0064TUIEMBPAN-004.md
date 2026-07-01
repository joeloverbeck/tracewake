# 0064TUIEMBPAN-004: Responsive collapse, explicit truncation, and minimum-size floor

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — modifies the `tracewake-tui` pane layout and binding renderers; new responsive integration test
**Deps**: 0064TUIEMBPAN-001, 0064TUIEMBPAN-002

## Problem

Spec 0064 §1.1.4 / §4.4 require responsive behavior that never silently loses safety-critical facts: collapse to a single-column stack on narrow terminals with headings and focus controls; never silently omit why-not, actions, or needs; explicit truncation markers with detail expansion; no color-only semantics. §6 requires an overflow fixture (actors/actions exceeding the pane show truncation markers, not silent loss), a `60x20` narrow-collapse that retains needs/why-not/actions, and a non-vacuity check (a layout that clips all actors/actions fails a snapshot/accessibility check). §9.1 open question #7 requires a terminal-size floor: hard-fail below the floor with an actor-safe message. #8 (theming) allows TUI-only theming but requires all semantics present in text.

## Assumption Reassessment (2026-07-02)

1. This ticket modifies the layout and binding modules created earlier in this batch — `crates/tracewake-tui/src/screen/pane_layout.rs` (0064TUIEMBPAN-001) and `crates/tracewake-tui/src/screen/pane_bindings.rs` (0064TUIEMBPAN-002) — adding narrow-collapse region reflow, truncation-marker emission, and the minimum-size floor. It depends on both (declared in Deps); the files do not exist until 001/002 land.
2. The fixed sizes and golden convention are set by spec §6 and the existing harness (`crates/tracewake-tui/tests/embodied_screen_dump.rs`, goldens at `crates/tracewake-tui/tests/goldens/`). `60x20` is the narrow-collapse case; the floor is below the smallest supported size. Truncation is presentation-only — it shortens *display*, never the underlying actor-known content set, and marks every elision.
3. Shared boundary under audit: the region model (001) and binding renderers (002). Reflow reorders/omits *rendering* of peripheral regions under size pressure but must keep the priority regions (needs, why-not, actions) present; the priority ordering established in 001 is the contract that decides what survives collapse.
4. Constitutional invariants under audit: **INV-070 — Why-not explanations are mandatory** (why-not must never be truncated to nothing); **INV-069 — The TUI must not implement simulation rules** (collapse/truncation is presentation-only, no rule); **INV-024 — No telepathy** (truncation reveals nothing new; detail expansion still draws only from actor-filtered fields); **INV-008 — UI assistance is not authority** (a floor hard-fail message is actor-safe UI text, granting no knowledge).
5. Actor-knowledge / safety-integrity surface: the failure mode this ticket guards is *silent loss* of safety-critical actor-known facts (why-not, actions, needs) under size pressure — a legibility/leak-adjacent integrity property. Truncation must be explicit (marker) and reversible (detail expansion), never a silent drop; the non-vacuity check enforces that a layout clipping all actors/actions fails loudly rather than presenting an empty-but-plausible screen. Deterministic behavior at each size (spec §8) is preserved so 0064TUIEMBPAN-005's narrow/overflow witnesses are byte-stable.

## Architecture Check

1. Encoding collapse as a priority-driven region reflow over the existing region model (rather than per-backend ad-hoc trimming) keeps the "what survives narrowing" decision in one inspectable place and lets both the buffer and text paths inherit it. Explicit truncation markers + a non-vacuity guard make overflow a testable property, not an emergent accident (§4.4).
2. No backwards-compatibility aliasing or shims: this extends the layout/binding modules in place; no parallel narrow-mode renderer is forked.

## Verification Layers

1. INV-070 (why-not mandatory) -> integration test: at `60x20` the why-not region is present and non-empty when the model carries a rejection.
2. Overflow explicitness (spec §4.4/§6) -> integration test: a fixture whose actors/actions exceed the region shows a truncation marker and no silent loss; detail expansion recovers the elided entries.
3. Non-vacuity (spec §6) -> accessibility/snapshot check: a layout that clips all actors/actions fails the check rather than passing as empty.
4. Minimum-size floor (§9.1 #7) -> unit test: below the floor, rendering yields the actor-safe hard-fail message, not a garbled partial layout.

## What to Change

### 1. Narrow-collapse reflow

In `pane_layout.rs`: add single-column stacking below a width threshold, preserving priority regions (needs, why-not, actions) and keeping region headings + focus indicators.

### 2. Explicit truncation + non-vacuity

In `pane_bindings.rs`: when a region's content exceeds its allotted lines, emit a truncation marker and an expandable detail affordance rather than dropping content; never truncate why-not/actions/needs to empty. Add a non-vacuity guard that fails when all actors/actions are clipped. Ensure no semantic is conveyed by color alone (§9.1 #8) — every state has a text token.

### 3. Minimum-size floor

Add a size floor: below it, render a single actor-safe message (no partial layout).

## Files to Touch

- `crates/tracewake-tui/src/screen/pane_layout.rs` (modify — file created by 0064TUIEMBPAN-001)
- `crates/tracewake-tui/src/screen/pane_bindings.rs` (modify — file created by 0064TUIEMBPAN-002)
- `crates/tracewake-tui/tests/embodied_pane_responsive.rs` (new)

## Out of Scope

- The `ratatui` buffer path and standard-size goldens (0064TUIEMBPAN-003).
- Live resize handling / event loop (Spec 0065) — this ticket renders deterministically at a given size.
- The consolidated §7 acceptance artifact (0064TUIEMBPAN-005).
- Any core view-model change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-tui --test embodied_pane_responsive` — `60x20` narrow-collapse retains needs, why-not, and actions (spec §6).
2. Overflow fixture: exceeding actors/actions produce truncation markers, not silent loss; detail expansion recovers elided entries.
3. Non-vacuity: a clip-all layout fails the snapshot/accessibility check.
4. Below-floor rendering yields the actor-safe hard-fail message (§9.1 #7).
5. `cargo test --workspace` passes.

### Invariants

1. Safety-critical actor-known facts (why-not, actions, needs) are never silently dropped (INV-070; spec §4.4).
2. Collapse/truncation is presentation-only and deterministic per size (INV-069 / spec §8); no semantic is color-only (§9.1 #8).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/embodied_pane_responsive.rs` — narrow-collapse retention, overflow truncation markers + expansion, non-vacuity guard, minimum-size floor message.

### Commands

1. `cargo test -p tracewake-tui --test embodied_pane_responsive`
2. `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --all --check`

## Outcome

Completed: 2026-07-02

Implemented responsive pane behavior in the TUI presentation layer. The buffer
renderer now has an actor-safe minimum-size floor, a narrow single-column
priority allocation that keeps why-not, actions, and needs visible at `60x20`,
and explicit truncation markers instead of silent clipping. The binding layer
now exposes truncation, expansion, and non-vacuity helper checks so overflow is
testable without backend-specific logic.

Added `crates/tracewake-tui/tests/embodied_pane_responsive.rs` covering:

- `60x20` retention of why-not, actions, and needs;
- overflow truncation markers;
- expansion recovery of elided action lines;
- non-vacuity failure when actions are clipped to zero visible lines;
- below-floor actor-safe rendering.

Updated the existing `insta` buffer snapshots from 0064TUIEMBPAN-003 so the
fixed-size goldens record the new explicit truncation markers.

Verification run:

- `cargo test -p tracewake-tui --test embodied_pane_responsive`
- `cargo test -p tracewake-tui --test embodied_pane_buffer` (failed once before
  snapshot refresh because truncation markers changed the expected buffers)
- `INSTA_UPDATE=always cargo test -p tracewake-tui --test embodied_pane_buffer`
- `cargo test -p tracewake-tui --test embodied_pane_buffer`
- `cargo test -p tracewake-tui pane_bindings`
- `cargo fmt --all --check`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`

No simulation rule, core/content view-model field, hidden-truth read, color-only
semantic, crossterm/raw-mode event loop, backwards-compatibility shim, or alias
path was added.
