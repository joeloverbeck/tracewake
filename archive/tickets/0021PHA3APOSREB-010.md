# 0021PHA3APOSREB-010: Two-sided embodied dead-surface sweep and surfacing work

**Status**: DONE
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` (`projections`, `view_models`, sweep guard), `tracewake-tui` (`render`); conformance-index row
**Deps**: `archive/specs/0021_PHASE_3A_POSSESSION_REBIND_HYGIENE_GUARD_VACUITY_CLOSURE_HARNESS_PROVENANCE_FIDELITY_AND_REJECT_LOUDLY_REPLAY_POSTURE_HARDENING_SPEC.md` (ORD-HARD-083)

## Problem

The embodied dead-surface family outlives the 0020 sweep because the sweep's universe
excludes consumers, scalars, and enums (`ORD-HARD-083`; INV-070/071 — the fourth-plus
dead embodied surface in three passes). Enumerated members: (a)
`NotebookView.typed_leads` is produced (`build_notebook_view` populates the
doctrine-mandated lead anatomy) but `render_notebook` prints only bare
`possible_leads` — no TUI file reads `typed_leads`; (b)
`EmbodiedViewModel.debug_available` is hardwired `true` at its sole producer, has no
production consumer, and is semantically wrong (availability should derive from debug
capability/binding); (c) `ActionAvailability` is an enum — listed in the sweep yet
silently skipped (`continue` on unmatched struct marker) — and its
`debug_only_diagnostics` is always empty in production; (d)
`VisibleDoor.endpoint_a/endpoint_b` and `VisibleItem.source` have no TUI consumer;
(e) the sweep's "reachable producer" predicate is syntactic and unscoped by struct —
re-hardwiring `visible_exit_blocker_summary` to `None` would still pass. Plus the
`blocker_summary` perception gate's co-location assumption needs pinning.

## Assumption Reassessment (2026-06-11)

1. Verified at `main` `89059a5`: `typed_leads` zero TUI reads (`render_notebook`,
   `render.rs`); `debug_available: true` at its sole producer (`projections.rs`) with
   only a test reader; `embodied_surface_fields` (anti_regression_guards.rs) scans a
   hand-enumerated 9-struct list, silently skips unmatched names (enums), matches
   producers by field name only across three sources; `EMBODIED_SURFACE_FIELD_PRODUCERS`
   carries the one deferral (notebook, cited); `NotebookLeadEntry` (`view_models.rs`)
   carries staleness/how-wrong/next-actions/source/confidence fields;
   `visible_exit_blocker_summary` reads `state.doors` gated only by geometry-implied
   co-location.
2. Verified against spec 0021 (reassessed 2026-06-11; operator-verified): finding
   ORD-HARD-083 with its five enumerated dead surfaces and five sweep-universe
   extensions; populate-vs-defer-with-cite is the spec-assigned implementer-recorded
   choice per field.
3. Cross-artifact boundary under audit: the embodied view-model contract between
   core producers (`projections.rs`/`view_models.rs`) and TUI consumers
   (`render.rs`/`app.rs`) — every embodied field has a reachable producer AND a
   consumer (or a cited deferral), in both directions.
4. INV-070 restated: why-not explanations are mandatory in actor-known terms.
   INV-071: mechanics hidden from play are unfinished — a produced-but-unrendered
   lead anatomy is hidden from play exactly as a dead field is. INV-068 (for the
   `debug_available` derivation): debug availability must derive from the debug
   capability/binding, never leak debug data itself.
5. Actor-knowledge surface touched: surfacing `typed_leads` renders already
   actor-known notebook content (no new information path); `debug_available`
   derivation reads binding/capability presence only (a boolean about the *mode*,
   not hidden truth). The `blocker_summary` co-location pin prevents a future
   visibility refinement from silently reverting that field to truth leakage. No
   replay surface touched (view-model layer only).
6. View-model shape: surfacing changes population and rendering, not shape. If the
   recorded choice for any member is *delete* (e.g. `debug_available`,
   `endpoint_a/b`), that is a field removal — blast radius per field is its producer
   plus tests (verified small for all five members); the removal is recorded with
   its rationale, and the sweep then proves the field absent rather than dead.

## Implementation Outcome (2026-06-11)

1. Surfaced the current dead embodied members:
   - `NotebookView.typed_leads` now renders in the notebook with contradiction,
     belief, observation, source kind, source summary, confidence, detected tick,
     staleness, how-wrong text, next actions, and summary. Legacy
     `possible_leads` remains a fallback only when no typed leads exist.
   - `EmbodiedViewModel.debug_available` is no longer hardwired true in core.
     Core projections default it to false; the TUI boundary sets it true when it
     attaches the notebook/debug-capable app surface, and render output displays
     the availability boolean.
   - `ActionAvailability::Disabled.debug_only_diagnostics` is deferred with a
     cited rationale for production population, but the TUI renderer now consumes
     it when present and the guard enrolls the enum payload.
   - `VisibleDoor.endpoint_a`/`endpoint_b` now render as door endpoints.
   - `VisibleItem.source` now renders for visible items and inventory items.
2. Extended the embodied surface guard:
   - The census now includes the previously skipped surface structs plus selected
     scalar fields and the `ActionAvailability` enum payload.
   - Missing/mismatched census names fail instead of silently continuing.
   - Producer matching is scoped to the owning struct, with explicit cited entries
     for TUI notebook attachment, TUI debug availability, and deferred debug-only
     diagnostics.
   - The guard now checks both producers and TUI render/app consumers, with
     synthetics for hardwired defaults, unmatched census entries, enum payloads,
     cross-struct producer aliases, and produced-but-unconsumed fields.
3. Pinned the `visible_exit_blocker_summary` co-location assumption: blocker
   summaries only consider doors admitted by the connected-door set from visible
   locality.
4. Updated the architecture conformance row from the old producer-only
   Option/collection sweep to the two-sided embodied-field sweep.

## Verification (2026-06-11)

1. `cargo test -p tracewake-tui render`
2. `cargo test -p tracewake-core --test anti_regression_guards embodied -- --nocapture`
3. `cargo test -p tracewake-core projections::tests::visible_exit_blocker_summary`
4. `cargo fmt --all --check`
5. `cargo clippy --workspace --all-targets -- -D warnings`
6. `cargo build --workspace --all-targets --locked`
7. `cargo test --workspace`
8. `git diff --check`

## Architecture Check

1. The sweep extension closes the *universe* gaps rather than adding instance
   checks: (i) derive the struct list from the view-model source; (ii) fail on
   unmatched listed names and handle enum variants; (iii) scope producer matches by
   struct; (iv) add the consumer-side sweep (every embodied census field appears in
   a render/app arm or carries a cited deferral); (v) include constant-scalar
   producers. That makes the fifth dead-surface instance the last one detectable
   only by audit — subsequent instances fail the guard. Surfacing work then clears
   the current census honestly instead of deferring everything.
2. No backwards-compatibility aliasing/shims: dead fields are surfaced, deferred
   with cites, or deleted — never kept as unrendered decoration.

## Verification Layers

1. INV-070/071 (typed leads reach play) -> render test asserting the typed lead
   anatomy (staleness label, how-this-may-be-wrong, next actions) renders in the
   notebook view.
2. INV-068/071 (`debug_available`) -> per the recorded choice: derivation test
   (false without binding/capability, true with) or deletion grep-proof.
3. Sweep universe (family lock) -> synthetic negatives per new dimension: an
   unlisted new embodied struct, an unmatched listed name, a cross-struct aliased
   producer, a constant-scalar producer, a produced-but-unconsumed field — each must
   fail.
4. Blocker-summary assumption pin -> guard or viewer-keyed routing test pinning the
   co-location assumption (per spec ORD-HARD-083(f)).

## What to Change

### 1. Surfacing pass (recorded choice per member)

`typed_leads` rendered in `render_notebook`; `debug_available` derived from debug
capability/binding or deleted; `debug_only_diagnostics` populated at the disabled
sites or deferred with cite; `VisibleDoor.endpoint_a/endpoint_b` and
`VisibleItem.source` consumed or deleted/deferred with cite. Each member's choice +
rationale recorded in the ticket closure and acceptance artifact.

### 2. Sweep extension (five universe dimensions)

Derived struct list; fail-on-unmatched + enum handling; struct-scoped producer
matching; consumer-side sweep; constant-scalar inclusion. Synthetic negative per
dimension.

### 3. Blocker-summary pin + documentation

Pin the co-location assumption (guard or viewer-keyed perceived-door routing);
update the `0020 dead embodied-field sweep` conformance row to the two-sided sweep
(`docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md`).

## Files to Touch

- `crates/tracewake-core/src/projections.rs` (modify)
- `crates/tracewake-core/src/view_models.rs` (modify)
- `crates/tracewake-tui/src/render.rs` (modify)
- `crates/tracewake-tui/src/app.rs` (modify — only if consumer wiring lands there; as surfaced)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)
- `docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` (modify)

## Out of Scope

- Rejection/provenance hygiene on the same files (ticket 0021PHA3APOSREB-001) —
  coordinate the `projections.rs`/`render.rs` mechanical merges.
- New embodied mechanics — this ticket surfaces or dispositions existing fields
  only.

## Acceptance Criteria

### Tests That Must Pass

1. Typed-lead render test green; notebook output carries the lead anatomy.
2. Each of the five enumerated members carries its recorded disposition
   (surfaced-with-test / deferred-with-cite / deleted-with-grep-proof) — one
   acceptance check per member.
3. Extended sweep green; all five synthetic universe negatives individually fail.
4. Blocker-summary pin test/guard green.
5. `cargo test --workspace` green.

### Invariants

1. Every embodied view/status field (struct or enum, Option/collection/scalar) has a
   non-default reachable producer AND a consumer in play, or a cited deferral, or is
   absent.
2. The sweep derives its universe from source — adding an embodied struct or field
   automatically enrolls it.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/src/render.rs` (mod tests) — typed-lead + consumer tests.
2. `crates/tracewake-core/src/projections.rs` (mod tests) — `debug_available`
   disposition test.
3. `crates/tracewake-core/tests/anti_regression_guards.rs` — two-sided sweep + five
   synthetics.

### Commands

1. `cargo test -p tracewake-tui render`
2. `cargo test -p tracewake-core --test anti_regression_guards embodied -- --nocapture`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
