# 0022PHA3ABASTRI-009: Embodied debug-render split and sweep producer semantics

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — TUI render boundary (`render.rs`, `app.rs`), view-model production (`projections.rs`), dead-surface sweep (`anti_regression_guards.rs`)
**Deps**: 0022PHA3ABASTRI-004

## Problem

`ORD-HARD-103`: `render_embodied_view` unconditionally appends `Debug: available={}`,
the `debug_only_diagnostics()` join, and the
`Knowledge context: DEBUG NON-DIEGETIC … frontier=…` line (a global event-log scalar)
into the same string an embodied actor reads — a label is not a quarantine
(INV-067/068), and the proving test asserts the marked line is *present*, locking the
co-mingling in. `ORD-HARD-104`: `app.rs` hardcodes `view.debug_available = true;` in
production while the census rationale claims it is "derived from the presence of
debug panel wiring"; the sweep's producer predicate accepts `= true` as a reachable
producer, and the `debug_only_diagnostics` deferral is registered with a
declaration-aliasing `producer_snippet`.

## Assumption Reassessment (2026-06-12)

1. Verified at `9ce820f`: `render_embodied_view` (`crates/tracewake-tui/src/render.rs`,
   fn at file top) pushes `Debug: available=` unconditionally and formats the
   `Knowledge context: DEBUG NON-DIEGETIC` line; `view.debug_available = true;` in
   production `crates/tracewake-tui/src/app.rs`; `projections.rs` initializes the
   field `false`; `EMBODIED_SURFACE_FIELD_PRODUCERS` and the sweep predicates live in
   `crates/tracewake-core/tests/anti_regression_guards.rs`;
   `renderer_marks_knowledge_context_frontier_non_diegetic` (render.rs tests) asserts
   marker presence only.
2. Verified against `specs/0022_…HARDENING_SPEC.md` §4 `ORD-HARD-103`/`104`
   (operator-verified; 103's severity medium per the `ORD-HARD-082`/`096` precedent —
   structural channel, no live leak content today: diagnostics are empty in
   production, the flag is constant, the frontier line is marked).
3. Shared contract under audit: the embodied/debug render boundary
   (`EmbodiedViewModel` → `render_embodied_view`) and the two-sided dead-surface
   sweep's producer semantics.
4. Constitutional invariants restated: INV-067 (embodied mode shows actor-known
   reality), INV-068 (debug visibly non-diegetic, never confused with embodied
   knowledge), INV-008 (UI assistance is not authority).
5. Actor-knowledge surface: splitting the debug overlay out of the embodied render
   strengthens the quarantine; the embodied string loses only non-diegetic engine
   metadata, no actor-known content — possession/embodied gates must stay green, and
   the new absence test must not weaken the existing debug-mode presence coverage
   (both directions asserted).
6. Either-or resolution (recorded at Step-4 approval, Q1 default): `debug_available`
   is **derived** from actual debug capability/binding (not deleted) — deletion would
   remove the TUI's debug-mode signal; derivation matches INV-068's
   capability-scoped debug posture. View-model shape unchanged (the field stays;
   its production becomes derived) — no schema extension; N/A for additive-vs-breaking
   beyond this no-shape-change statement.
7. Change rationale (no silent retcon): the 0021 `ORD-HARD-096` correction chose the
   marker option and satisfied its letter; the 0022 audit found the marker insufficient
   as structure (R-28 family closure on the render boundary), and the
   `ORD-HARD-083` "derived or deleted" requirement was satisfied by neither.

## Architecture Check

1. A structural split (embodied render carries zero debug tokens; a debug overlay
   renders them only under debug capability) is enforceable by a one-line absence
   assertion forever — strictly stronger than label discipline, and it makes the
   `debug_available` derivation the single gate for the overlay. Excluding
   constant-literal initializers from the sweep's producer predicate and giving
   deferrals a cite-only path (no declaration-aliasing snippets) closes the
   constant-producer hole generically.
2. No backwards-compatibility aliasing/shims: the co-mingled lines are moved, not
   duplicated behind a flag default.

## Verification Layers

1. INV-067/068 quarantine -> render negative test: a pure-embodied render contains no
   `Debug:` / `debug_diagnostics` / `Knowledge context` tokens (`assert_absent`),
   alongside the existing debug-mode presence test — both directions locked.
2. Derived `debug_available` -> behavior test: the flag is true iff debug
   capability/binding is active (both states asserted); `grep -n "debug_available = true" crates/tracewake-tui/src/app.rs` returns 0.
3. Sweep producer semantics (R-29) -> guard check: a synthetic constant-literal
   producer fails the sweep; deferral entries carry cites only (no
   `producer_snippet` aliasing a declaration); nonzero-witness applied to the sweep's
   per-entry matches (census-registered, `-004`).
4. Census rationale honesty -> grep-proof: the `EMBODIED_SURFACE_FIELD_PRODUCERS`
   entry for `debug_available` describes the real derivation.

## What to Change

### 1. Split the debug overlay out of `render_embodied_view`

Move the `Debug: available=`, `debug_only_diagnostics`, and `Knowledge context`
(frontier/hash) lines into a debug-only render path (a `render_debug_overlay` fn or
an explicit debug flag consumed only by the debug view), gated on the derived
`debug_available`. The embodied string carries none of them.

### 2. Derive `debug_available`

Replace the `app.rs` hardcoded `true` with derivation from the actual debug
capability/binding state the TUI holds; correct the census rationale text; assert
both states in a unit test.

### 3. Harden the sweep's producer semantics

Exclude constant-literal initializers (`= true`/`= false`/literal numerics) from
`source_has_non_default_struct_field_producer`'s accepted-producer forms; convert the
`debug_only_diagnostics` deferral (and any other deferral entries) to a cite-only
exemption shape; synthetic constant-producer negative; register with the `-004`
census including the nonzero-witness minimums.

## Files to Touch

- `crates/tracewake-tui/src/render.rs` (modify)
- `crates/tracewake-tui/src/app.rs` (modify)
- `crates/tracewake-core/src/projections.rs` (modify — only if the derivation needs a
  view-model production change; field shape unchanged)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)

## Out of Scope

- Populating `debug_only_diagnostics` (its cited deferral stands; this ticket fixes
  the deferral's registration shape).
- The possession/rejection hygiene surfaces (0021 `ORD-HARD-066`/`082` — verified
  holding, untouched).
- Any new debug content; the overlay renders exactly what the embodied path
  previously carried.

## Acceptance Criteria

### Tests That Must Pass

1. The embodied-render absence test: no `Debug:`, `debug_diagnostics`, or
   `Knowledge context` token in a pure-embodied render; existing debug-mode presence
   test still green.
2. `debug_available` derivation test (true under debug binding, false otherwise);
   `grep -n "debug_available = true" crates/tracewake-tui/src/app.rs` → 0 matches.
3. `cargo test -p tracewake-core --test anti_regression_guards` — constant-producer
   synthetic fires; deferrals are cite-only; nonzero-witness green.
4. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. The embodied render path emits actor-known content only; every debug token lives
   behind the derived debug gate (INV-067/068 structural, not label-based).
2. The dead-surface sweep accepts no constant literal as a producer and no
   declaration alias as a deferral.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/src/render.rs` (tests) — embodied absence + debug presence
   pair.
2. `crates/tracewake-tui/src/app.rs` (tests) — `debug_available` derivation.
3. `crates/tracewake-core/tests/anti_regression_guards.rs` — sweep predicate
   hardening + synthetic.

### Commands

1. `cargo test -p tracewake-tui`
2. `cargo test -p tracewake-core --test anti_regression_guards`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
