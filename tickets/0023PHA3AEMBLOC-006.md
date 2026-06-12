# 0023PHA3AEMBLOC-006: Debug overlay production wiring and derived debug-token negative

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — TUI debug command/panel (`app.rs`, `render.rs`), derived debug-token constant, render guards
**Deps**: 0023PHA3AEMBLOC-004, `specs/0023_PHASE_3A_EMBODIED_LOCALITY_TRUTH_FIREWALL_META_LOCK_WITNESS_INTEGRITY_AND_GUARD_EVASION_CLOSURE_HARDENING_SPEC.md`

## Problem

Two TUI debug-quarantine residues from 0022 (spec 0023 `ORD-HARD-125` medium,
`ORD-HARD-135` low):

- `render_debug_overlay` (`crates/tracewake-tui/src/render.rs`) has no production
  caller — only its own test module invokes it; `TuiApp::render_current_view` renders
  only the embodied view. The 0022 `ORD-HARD-103` split quarantined the debug overlay
  into unreachability: the derived `debug_available` gates a surface no production
  path renders, leaving the debug mechanic hidden from play (INV-071 direction) and
  the `ORD-HARD-103` contract ("consumed only by the debug path") half-landed —
  no debug path consumes it.
- The embodied debug-token negative
  (`renderer_keeps_debug_tokens_out_of_embodied_view`) asserts absence of four
  hand-listed literals (`Knowledge context`, `DEBUG NON-DIEGETIC`, `Debug:`,
  `debug_diagnostics`); a future debug field hand-pushed into `render_embodied_view`
  with new tokens would leak past it.

## Assumption Reassessment (2026-06-12)

1. Verified at `db4b53a`: `render_debug_overlay` callers exist only in `render.rs`
   tests; `debug_available_for` genuinely derives from live `controller_bindings`
   (0022 fix holding); the negative's four-literal list confirmed at
   `renderer_keeps_debug_tokens_out_of_embodied_view`.
2. Verified against `specs/0023_…HARDENING_SPEC.md` §4 `ORD-HARD-125`/`135`
   (operator-verified evidence). The spec's wire-vs-defer option resolved at
   decomposition to **wire**: `docs/1-architecture/10` explicitly sanctions "opening
   debug panels" as a TUI command, and INV-041/068 want debug reachable to be
   "visibly non-diegetic"; a cite-only deferral would just re-defer the surface 0022
   already built.
3. Shared contract under audit: the embodied/debug render boundary —
   `render_embodied_view` (actor-known only) vs `render_debug_overlay`
   (`debug_available`-gated truth) — consumed by `TuiApp` view rendering and the
   render-guard family. Ordered after -004 because the migration churns the embodied
   render inputs this boundary separates.
4. Constitutional motivation restated: INV-068 (debug mode is visibly non-diegetic
   and must never be confused with embodied knowledge), INV-071 (mechanics hidden
   from play are unfinished), INV-067/008 (the embodied surface stays actor-known;
   debug never feeds it).
5. This ticket touches the actor-knowledge/debug quarantine surface: wiring the
   overlay must keep it strictly additive on a debug-gated path (command available
   only when `debug_available`; output rendered as a separate non-diegetic surface,
   never concatenated into the embodied string); the derived-token negative
   strengthens leakage prevention. No replay surface is touched.
6. Schema note: no view-model shape change — the overlay consumes existing
   debug-classified fields; the new `DEBUG_TOKENS` constant is a test/render-shared
   const, not a schema. (No additive-vs-breaking analysis owed beyond this N/A.)
7. Change rationale (no silent retcon): the overlay gains a production caller because
   the 0022 correction's own contract requires a debug path to consume it; the token
   list becomes derived because the hand-list provably cannot cover future debug
   fields. Mandated by `ORD-HARD-125`/`135`.

## Architecture Check

1. A debug-panel command consuming `render_debug_overlay` behind `debug_available`
   completes the 0022 split as designed — cleaner than deleting the overlay (which
   would orphan the derived `debug_available` and regress INV-041's debug-inspection
   mandate) and cleaner than a cite-only deferral (which leaves a built mechanic
   unreachable, the INV-071 smell). Deriving the forbidden-token set from one
   `DEBUG_TOKENS` const consumed by both the overlay renderer and the negative makes
   the negative auto-extend with every new debug surface.
2. No backwards-compatibility aliasing/shims: one debug entry point; no duplicate
   debug strings in the embodied path; the hand-listed tokens are replaced by the
   derived set, not kept alongside.

## Verification Layers

1. INV-071 reachability -> codebase test-proof: a TUI guard asserts every
   `pub fn render_*` in `render.rs` has a non-test caller in the app layer (with
   rationale-bearing allowlist), and the debug command renders the overlay when
   `debug_available`.
2. INV-068 quarantine -> codebase test-proof: the embodied render contains none of
   the derived `DEBUG_TOKENS`; the debug overlay carries the non-diegetic marker —
   both sides of the boundary asserted.
3. Derived-token closure (R-28) -> codebase test-proof: a synthetic new debug token
   added to `DEBUG_TOKENS` is automatically asserted-absent from the embodied render
   (negative extends without edit).
4. Possession/viewer scoping (INV-108) -> codebase test-proof: the debug command is
   unavailable when the viewer's binding is detached (`debug_available_for` gating
   holds at the command layer).

## What to Change

### 1. Production debug command/panel (`ORD-HARD-125`)

Add a TUI debug command (per `docs/1-architecture/10`'s sanctioned command set) in
`app.rs` that, when `debug_available`, renders `render_debug_overlay` as a separate
non-diegetic surface. Add the render-reachability guard (every `pub fn render_*` has
a non-test app-layer caller, allowlisted with rationale otherwise).

### 2. Derived debug-token negative (`ORD-HARD-135`)

Introduce a single `DEBUG_TOKENS` const consumed by `render_debug_overlay` (it emits
only token-prefixed surfaces) and by the embodied negative (assert each token absent
from `render_embodied_view` output). Add the synthetic new-token case proving the
negative auto-extends.

## Files to Touch

- `crates/tracewake-tui/src/app.rs` (modify)
- `crates/tracewake-tui/src/render.rs` (modify)

## Out of Scope

- `debug_available` derivation (landed in 0022; verified holding).
- The embodied sweep's deferral mechanics (-003) — if -003 recorded a
  `debug_only_diagnostics` deferral, this ticket graduates it to a real
  producer/consumer entry; coordinate the entry shape.
- Core-side INV-093 locks (-005).

## Acceptance Criteria

### Tests That Must Pass

1. With a bound (non-detached) controller, the debug command renders the overlay
   (marker line present); with `debug_available == false` the command is
   unavailable/no-op (`cargo test -p tracewake-tui`).
2. Embodied render asserts absence of every `DEBUG_TOKENS` entry; the synthetic
   new-token case fails without any test edit.
3. The render-reachability guard fires on a synthetic uncalled `pub fn render_*`.
4. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` green.

### Invariants

1. Every debug-rendering surface is reachable from a production debug path gated on
   `debug_available` — no built-but-unreachable debug mechanics.
2. The embodied render's forbidden-token set is derived from the same constant the
   overlay renders from — a new debug surface cannot leak past a stale hand-list.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/src/render.rs` (tests) — derived-token negative + synthetic
   new-token case; overlay presence test re-anchored to the command path.
2. `crates/tracewake-tui/src/app.rs` (tests) — debug-command availability/gating;
   render-reachability guard.

### Commands

1. `cargo test -p tracewake-tui`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
