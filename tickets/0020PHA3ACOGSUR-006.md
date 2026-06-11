# 0020PHA3ACOGSUR-006: Exit-blocker surfacing and the dead-embodied-field sweep

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` (`projections.rs` exit derivation, `view_models.rs` as surfaced), `tracewake-tui` (`render.rs` arm reachability, `app.rs` as surfaced), dead-field sweep guard; `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` sweep row
**Deps**: `specs/0020_PHASE_3A_COGNITION_SURFACE_FRESHNESS_PARITY_DERIVED_CENSUS_CLOSURE_MUTATION_PERIMETER_COMPLETION_AND_GENERATIVE_FIDELITY_HARDENING_SPEC.md` (ORD-HARD-063)

## Problem

`projections.rs::build_embodied_view_model` constructs every
`VisibleExit { …, blocker_summary: None }` — the field's only producer — so the
`render.rs` arm formatting `" blocked: {summary}"` is unreachable and door/access
blockers on actor-known routes (a live mechanic) have no embodied surfacing
(`ORD-HARD-063`, INV-070/071). This is the exact `ORD-HARD-052` defect shape one
field over — the third dead-embodied-surface finding in two passes — so the spec
mandates the *family* lock alongside the instance fix: a sweep guard over every
`Option`/collection field on embodied status/view structs.

## Assumption Reassessment (2026-06-11)

1. Verified at `main` `96bc387`: `build_embodied_view_model` is defined in
   `projections.rs` (its callers live in `controller.rs` / TUI `app.rs`); the
   `VisibleExit` construction maps `actor_known_routes` destinations with
   `blocker_summary: None` unconditionally; `render.rs` carries the live-but-
   unreachable `exit.blocker_summary` arm. Current sweep inventory (the quantifier,
   enumerated): 16 `Option`/`Vec` fields across the embodied structs in
   `view_models.rs` — `visible_exits`, `visible_doors`, `visible_containers`,
   `visible_items`, `carried_items`, `local_actors`, `semantic_actions`,
   `phase3a_status`, `last_rejection_summary`, `last_rejection_why_not`,
   `notebook`, `need_summaries`, `intention_summary`, `routine_summary`,
   `salient_interruption` (live since 0019), `reason_codes` — plus
   `VisibleExit.blocker_summary` (the dead one).
2. Verified against spec 0020 (reassessed 2026-06-11): ORD-HARD-063 + the §5 tier-3
   dead-field sweep mandate; the sweep conformance row lands with this ticket per
   the approved distribution. Conditional carried from decomposition: the audit
   found NO phase-ladder deferral cite for exit-blocker surfacing — re-check
   `docs/2-execution/03` and `12` first; if a cite IS found, delete the field and
   render arm instead and record the deferral in the conformance index (the
   ORD-HARD-052/0019 deferral-re-check pattern).
3. Cross-artifact boundary under audit: the embodied surfacing contract spanning
   `projections.rs` (producer), `view_models.rs` (shape), and the TUI render path
   (consumer) — every embodied field has a reachable producer and a proven render
   path, or a registered deferral.
4. INV-070 restated: why-not explanations are mandatory in actor-known terms.
   INV-071 restated: mechanics hidden from TUI/view-model reachability are
   unfinished. A struct field whose only producer hardwires its default is a hidden
   mechanic wearing a shipped field's clothes.
5. Actor-knowledge surface touched: `blocker_summary` derives viewer-keyed from the
   actor's OWN perceived door state on known routes (the `ORD-HARD-052` derivation
   pattern — materialized actor-known evidence only); no path reads unperceived
   truth or other actors' state, so embodied mode gains no hidden-truth channel
   (INV-024/067 preserved). Deterministic replay untouched (projection-only).
6. Schema shape: no shape change — `blocker_summary: Option<String>` already exists
   on `VisibleExit`; this ticket populates it. Additive-vs-breaking is N/A.

## Architecture Check

1. The sweep guard is the family lock the instance fixes lacked: it asserts every
   embodied `Option`/collection field has at least one non-default producer or a
   registered deferral cite, so the fourth dead surface becomes a CI failure at
   introduction time instead of an audit finding two passes later. Derivation over
   `view_models.rs` source (the census pattern) rather than a hand list.
2. No backwards-compatibility aliasing/shims: if the deferral-cite re-check fires,
   the field and arm are deleted, not left dormant behind the sweep's registry.

## Verification Layers

1. INV-070/071 (instance) -> fixture with a known route through a perceived
   closed/locked door asserting `blocker_summary.is_some()` and the render emits the
   `blocked:` line.
2. INV-024/067 (no leak) -> negative case: an unperceived door state on a known
   route yields `None` (the viewer cannot know it); viewer-keying asserted.
3. Family lock -> dead-field sweep guard over the embodied structs: every
   `Option`/`Vec` field has a non-default producer in `projections.rs` or a
   registered deferral entry with a cite; synthetic dead-field case fails.
4. Render reachability -> TUI render test proving the arm fires (the
   `renderer_prints_phase3a_salient_interruption` pattern).

## What to Change

### 1. Deferral re-check, then derive `blocker_summary`

Re-check `docs/2-execution/` for an exit-blocker deferral cite (none found at
audit). Absent a cite: derive `blocker_summary` in `build_embodied_view_model` from
the viewer's actor-known door state along each known route (perceived closed/locked
only), viewer-keyed exactly as `phase3a_salient_interruption`. If a cite is found:
delete the field + render arm and record the deferral in the conformance index.

### 2. Dead-embodied-field sweep guard

Add the derived guard: enumerate `Option`/collection fields on embodied status/view
structs from `view_models.rs` source; assert each has a non-default construction
site in `projections.rs` or a registered deferral entry (with rationale + cite);
include a synthetic dead-field regression case.

### 3. Conformance row

Add the dead-field sweep row to
`docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` describing exactly
what the sweep derives and the deferral-registry semantics.

## Files to Touch

- `crates/tracewake-core/src/projections.rs` (modify)
- `crates/tracewake-core/src/view_models.rs` (modify — only if the delete branch fires or sweep anchors are needed; no shape change otherwise)
- `crates/tracewake-tui/src/render.rs` (modify — render test; arm itself already exists)
- `crates/tracewake-tui/src/app.rs` (modify — as surfaced, if the render path threads through it)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — sweep guard)
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (modify)

## Out of Scope

- New blocker *semantics* (what counts as blocked is existing actor-known door
  state; no new world rules — INV-069).
- `salient_interruption` (landed in 0019; the sweep covers it as a live field).
- Embodied fields owned by other phases' mechanics beyond sweep registration.

## Acceptance Criteria

### Tests That Must Pass

1. Known-route-through-perceived-closed-door fixture: `blocker_summary` is `Some`,
   render emits the `blocked:` line.
2. Unperceived-door negative case: `None`; viewer-keying proven.
3. Sweep guard green over the enumerated 16-field inventory + `blocker_summary`,
   with the synthetic dead-field case failing as designed.
4. Deferral re-check outcome recorded (implement vs delete+record).
5. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`.

### Invariants

1. Every embodied view field has a reachable producer or a registered, cited
   deferral — dead surfaces fail CI at introduction (INV-071; lock durability).
2. Embodied blocker text derives only from the viewer's actor-known evidence
   (INV-024/067/070).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/projections.rs` (test block) — derivation + negative
   viewer-keying cases.
2. `crates/tracewake-tui/src/render.rs` / `tests/tui_acceptance.rs` — render-line
   reachability test.
3. `crates/tracewake-core/tests/anti_regression_guards.rs` — dead-field sweep guard
   + synthetic regression case.

### Commands

1. `cargo test -p tracewake-core projections`
2. `cargo test -p tracewake-core --test anti_regression_guards`
3. `cargo test --workspace` (full pipeline incl. TUI acceptance)
