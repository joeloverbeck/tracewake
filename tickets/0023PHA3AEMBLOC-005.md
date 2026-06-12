# 0023PHA3AEMBLOC-005: Embodied truth-access compile-time quarantine and INV-093 locks

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `EmbodiedProjectionSource` shape (`projections.rs`), consumers (`controller.rs`, TUI `app.rs`, `hidden_truth_gates.rs`), INV-093 regression tests
**Deps**: archive/tickets/0023PHA3AEMBLOC-004.md, `specs/0023_PHASE_3A_EMBODIED_LOCALITY_TRUTH_FIREWALL_META_LOCK_WITNESS_INTEGRITY_AND_GUARD_EVASION_CLOSURE_HARDENING_SPEC.md`

## Problem

After -004 migrates the locality surface onto the epistemic projection, the embodied
builder can still *name* raw truth: `EmbodiedProjectionSource` carries a
`state: &PhysicalState` field. Spec 0023 `ORD-HARD-121`'s structural lock requires
compile-time impossibility — the lineage's strongest tier — so that no future surface
can be born reading truth in the embodied builder, plus the behavioral negatives: an
INV-093 absence test (an item/actor physically present with no projected observation
for the viewer must not render) and a staleness positive (a projected-then-moved item
renders from the stale observation, not live truth).

## Assumption Reassessment (2026-06-12)

1. Verified at `db4b53a`: `EmbodiedProjectionSource` consumers are `projections.rs`,
   `controller.rs`, `crates/tracewake-tui/src/app.rs`, and
   `tests/hidden_truth_gates.rs` (grep, this decomposition); after -004, no embodied
   derivation reads `source.state` — this ticket removes the capability itself.
2. Verified against `specs/0023_…HARDENING_SPEC.md` §4 `ORD-HARD-121` structural
   lock and §8's staging note (this is the second stage: the compile-time lock lands
   against an already-correct builder, no intermediate broken state). The spec's
   drop-vs-newtype option resolved at decomposition to **drop**: `visible_locality`'s
   lawful validation consumer (`inspect.rs`) takes `PhysicalState` maps directly and
   does not flow through `EmbodiedProjectionSource`, so no embodied-side consumer
   needs a quarantined handle — the field can go entirely.
3. Shared contract under audit: the `EmbodiedProjectionSource` construction seam —
   producers in `controller.rs`/`app.rs` and the sealed-context path
   (`EmbodiedProjectionSource::from_sealed_context`, exercised by
   `hidden_truth_gates.rs`) — every constructor site changes shape in this diff
   (local compile-atomicity: the field removal and all in-workspace consumer updates
   must land together or the tree does not compile).
4. Constitutional motivation restated: INV-093 (actor-knowledge leakage is a
   high-severity defect; embodied view models must be tested against hidden-truth
   leakage) and the Enforcement reading (embodied views generated from truth are a
   failure even when tests pass) — the compile-time removal makes the failure shape
   unwritable, not merely untested.
5. This ticket touches the actor-knowledge filtering surface directly: removing
   truth access from the embodied builder strengthens the firewall; the INV-093
   negatives prove the discrimination (present-in-truth, absent-from-projection ⇒
   absent-from-view) and the staleness positive proves the view follows belief, not
   truth. No replay surface changes (view models are projections, not log content).
6. Schema change: `EmbodiedProjectionSource` loses its `state` field — breaking for
   in-workspace constructors only (no external consumers exist; all sites enumerated
   in Assumption 3 and updated in this diff). The view-model schema is unchanged.
7. Removal blast radius (grep, this decomposition): `EmbodiedProjectionSource` sites
   in `projections.rs`, `controller.rs`, `app.rs`, `hidden_truth_gates.rs` — all join
   Files to Touch; no doc names the field.
8. Change rationale (no silent retcon): the field is removed because the spec's
   structural-lock clause requires the embodied builder to be *unable* to read truth;
   mandated by `ORD-HARD-121`.

## Architecture Check

1. Dropping the field outright is cleaner than a validation-only newtype quarantine:
   a newtype leaves a readable path that future code can launder through, while
   absence is unforgeable — and the one lawful truth consumer (`inspect.rs`
   validation) never flowed through this struct, so nothing legitimate is lost. This
   mirrors the lineage's `from_observed_parts` demotion (0021) and compile-fail
   posture: prefer making the violation unwritable over policing it.
2. No backwards-compatibility aliasing/shims: no deprecated field, no accessor
   returning truth, no constructor variant keeping the old shape.

## Verification Layers

1. Compile-time impossibility -> codebase grep-proof: `EmbodiedProjectionSource` has
   no `PhysicalState` field; `build_embodied_view_model` body contains no
   `source.state` / `visible_locality` reference (the latter survives only in
   validation modules).
2. INV-093 absence -> codebase test-proof: an item and an actor present in
   `PhysicalState` with no projected observation for the viewer do not appear in the
   embodied view (negative routed through the real builder).
3. Belief-over-truth staleness (INV-067/028) -> codebase test-proof: an observed item
   subsequently moved in truth still renders from the stale observation until a new
   observation supersedes it.
4. Sealed-context parity -> replay/test check: `hidden_truth_gates.rs` embodied gates
   stay green against the reshaped source.

## What to Change

### 1. Remove the truth field

Drop `state: &PhysicalState` (and any truth-bearing accessor) from
`EmbodiedProjectionSource`; update every constructor/consumer site
(`projections.rs`, `controller.rs`, `app.rs`, `hidden_truth_gates.rs`) in the same
diff.

### 2. INV-093 negatives and staleness positive

Add the absence negative (present-in-truth, unobserved ⇒ unrendered, for an item and
a co-located actor) and the staleness positive (projected-then-moved renders stale)
against the real `build_embodied_view_model` path.

### 3. Source-shape guard

Add a structural guard asserting `EmbodiedProjectionSource` declares no
`PhysicalState`-typed field (so the field cannot quietly return), enrolled under the
-001 registry mechanics with its synthetic negative.

## Files to Touch

- `crates/tracewake-core/src/projections.rs` (modify)
- `crates/tracewake-core/src/controller.rs` (modify)
- `crates/tracewake-tui/src/app.rs` (modify)
- `crates/tracewake-core/tests/hidden_truth_gates.rs` (modify)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)

## Out of Scope

- The migration itself (landed in -004).
- Identity-uncertainty mechanics (Phase 3B+).
- Debug overlay wiring (-006) — `debug_available_for` derivation is untouched.

## Acceptance Criteria

### Tests That Must Pass

1. The workspace compiles with `EmbodiedProjectionSource` carrying no
   `PhysicalState` field; grep-proof: zero `source.state` references in
   `build_embodied_view_model`.
2. INV-093 absence negatives fire (unobserved item and unobserved co-located actor
   absent from embodied render); staleness positive passes.
3. The source-shape guard's synthetic negative fires; `hidden_truth_gates.rs` green.
4. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace` green.

### Invariants

1. The embodied builder cannot reference authoritative `PhysicalState` — the
   capability is absent at the type level, not policed by review.
2. Embodied rendering follows projected belief, including when belief is stale
   relative to truth.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/projections.rs` (inline tests) — INV-093 absence
   negatives + staleness positive.
2. `crates/tracewake-core/tests/anti_regression_guards.rs` — source-shape guard +
   synthetic negative, registry-enrolled.
3. `crates/tracewake-core/tests/hidden_truth_gates.rs` — constructor-site updates;
   embodied gates re-anchored to the reshaped source.

### Commands

1. `cargo test -p tracewake-core`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
