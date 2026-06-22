# 0046TUISIMPLA-007: Baseline census — 6 non-action capability families

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — new census module (`crates/tracewake-tui/tests/parity/census_families.rs`), registry-module registration, and focused goldens + any gap-filling fixtures in `tracewake-content`. No `tracewake-core` change.
**Deps**: 0046TUISIMPLA-003, 0046TUISIMPLA-004, 0046TUISIMPLA-005

## Problem

Spec 0046 §4.3 `PAR-009` (non-action half). Beyond the registered actions, the base build has playable
capability families with no single action ID — they must also map to registry entries so the baseline
matrix shows zero uncovered entries. This ticket enumerates the **6** likely-minimum families the spec
names and binds each to a capability entry, fixture, typed witness, rendered witness, and (for
epistemic families) anti-leak witnesses, with explicit justified debug-only/headless dispositions.

## Assumption Reassessment (2026-06-22)

1. Verified against code at baseline `1145e109`: the family surfaces exist in the projection/render
   path — `build_embodied_view_model` (`crates/tracewake-core/src/projections.rs:539`) and
   `build_notebook_view` (`:674`) gather actor-known state, leads, and why-not; `render_notebook`
   (`crates/tracewake-tui/src/render.rs:196`) and `render_debug_overlay` (`:153`) present notebook and
   debug surfaces. The 58-fixture substrate includes epistemic/no-human fixtures (e.g.
   `embodied_menu_lags_truth_change_without_perception_001`,
   `seeded_food_source_unknown_to_all_actors_001`).
2. Verified against spec 0046 §4.3 (`PAR-009`, "likely minimum non-action families"): the six families
   are **epistemic filtering, no-human autonomy, needs/routines, notebook/leads, rejection/why-not, and
   debug quarantine**. This is the granularity floor (spec §9.5 leaves which non-action capabilities
   deserve separate entries an implementation-first decision); split a family into finer entries only
   if one exceeds a reviewable witness set, and record any such split. "Debug-only"/"headless" are
   explicit dispositions with justification, not escape hatches.
3. Shared boundary under audit: the projection→render→notebook/debug seams ↔ the capability registry.
   This ticket `(modify)`s `parity/mod.rs` (created by 003) alongside ticket 006 (both register census
   modules) and may `(modify)` `fixtures/mod.rs` alongside 006 — coordinate the mechanical merge.
4. Invariant restated (`PAR-009` motivation): `INV-067` embodied shows actor-known reality; `INV-068`/
   `INV-107` debug is non-diegetic and quarantined (the debug-quarantine family proves a debug witness
   diagnoses but never satisfies an embodied assertion); `INV-070` why-not is mandatory (rejection
   family); `INV-091` no-human autonomy is tested.
5. Enforcement surface touched (actor-knowledge filtering): epistemic-filtering, notebook/leads, and
   rejection/why-not families require paired anti-leak witnesses — unknown/stale/contradictory/
   unobserved states stay absent or disabled with an actor-safe why-not and surface no hidden/
   validator-only truth (`INV-093`/`INV-099`–`101`). The no-human-autonomy family proves only
   legitimately observable consequence (presence/movement/work/environment/communication/records), never
   private intention or hidden truth. The debug-quarantine family proves debug truth never enters the
   embodied assertion (`INV-107`).
6. Schema extension (additive): any gap-filling fixture is additive-only in `tracewake-content`
   (`fixtures::all`/`by_id` consumers; new fixture files registered in `fixtures/mod.rs`, no edit to
   existing fixtures). Registry entries are additive test-side data.

## Architecture Check

1. Grouping non-action capabilities by actor-observable contract (epistemic filtering, autonomy,
   needs/routines, notebook/leads, why-not, debug quarantine) rather than by module name is cleaner and
   matches the spec's principled-grouping guidance (§9.1): each family maps to a distinct observable
   surface the runner can check, and reuses existing fixtures that already exercise these seams.
2. No backwards-compatibility aliasing/shims: no fixture duplicated; gap fixtures additive.

## Verification Layers

1. `PAR-009`/`INV-091` (family coverage) → runner pass: each of the six families has a registry entry
   with fixture + typed + rendered witness; the baseline matrix reports zero uncovered entries (combined
   with ticket 006's action entries).
2. `INV-093`/`INV-070` (anti-leak + why-not) → paired negatives for the epistemic/why-not families.
3. `INV-107` (debug quarantine) → the debug-quarantine entry's debug witness diagnoses the mechanism
   without satisfying the embodied assertion (separate evidence).

## What to Change

### 1. Family capability entries (`crates/tracewake-tui/tests/parity/census_families.rs`)

Add a registry entry per family with class, surface disposition + rationale (explicit justified
debug-only/headless where applicable), fixture ID(s), possessed viewer + setup/advance op, typed
witness, rendered witness/golden, and required anti-leak fixtures for epistemic families. Register the
module in `parity/mod.rs`.

### 2. Focused goldens + gap fixtures

Add a checked-in golden per family through the 005 harness; add focused `GoldenFixture`s in
`tracewake-content` only where the census shows a gap, registered in `fixtures/mod.rs`.

## Files to Touch

- `crates/tracewake-tui/tests/parity/census_families.rs` (new)
- `crates/tracewake-tui/tests/parity/mod.rs` (modify — file created by 0046TUISIMPLA-003; also modified by 006)
- `crates/tracewake-tui/tests/goldens/` (modify — per-family golden files; dir created by 0046TUISIMPLA-005)
- `crates/tracewake-content/src/fixtures/` (new — gap-filling fixtures, only where the census shows a gap)
- `crates/tracewake-content/src/fixtures/mod.rs` (modify — register any new fixtures; also modified by 006)

## Out of Scope

- The 15 action capabilities (`PAR-009` action half) — ticket 006.
- Runner/harness logic (003/004/005). CI lane (008). Acceptance assembly (012).
- Bind-time perception behavior (spec §9.6) — architecture owner's decision, not a parity-test
  convenience.

## Acceptance Criteria

### Tests That Must Pass

1. Member matrix — each of the six families (epistemic filtering, no-human autonomy, needs/routines,
   notebook/leads, rejection/why-not, debug quarantine) has a registry entry with fixture + typed +
   rendered witness; epistemic/why-not families carry anti-leak witnesses; debug quarantine carries a
   debug witness that does not satisfy the embodied assertion. Any family split into finer entries is
   recorded with rationale; no family dropped without recorded justification.
2. Combined with ticket 006, the runner reports **zero uncovered entries** across the full baseline
   matrix (actions + families).
3. `cargo test -p tracewake-tui --test playable_capability_parity` and the four gates pass; new
   fixtures validate under the content schema/fixture checks.

### Invariants

1. Every base playable capability family has an actor-filtered surface contract or an explicit,
   justified non-playable (debug-only/headless) disposition.
2. No debug or hidden truth satisfies any embodied family witness (`INV-107`).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/playable_capability_parity.rs` — family entries through runner + harness;
   per-family goldens + epistemic/why-not anti-leak negatives + debug-quarantine separation.
2. `crates/tracewake-content` fixture tests — any new gap fixture covered by existing fixture validation.

### Commands

1. `cargo test -p tracewake-tui --test playable_capability_parity`
2. `cargo test -p tracewake-content`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-22

- Added `parity::census_families` with baseline entries for all six non-action families named by
  spec 0046: epistemic filtering, no-human autonomy, needs/routines, notebook/leads,
  rejection/why-not, and debug quarantine.
- Extended the test-side scenario harness with notebook rendering, debug-overlay rendering, and
  debug-gated no-human-day execution so family witnesses come from the real `TuiApp` path rather than
  hand-built view models.
- Added checked-in family goldens for the six entries. The debug quarantine entry is explicitly
  `DebugOnly`; notebook/leads renders through `render_notebook`; no-human autonomy renders
  post-scheduler observable embodied consequences; anti-leak families assert hidden/debug/validator
  truth does not satisfy embodied or notebook witnesses.
- Added a parity test that fails if any of the six baseline non-action family entries is missing from
  the combined registry. Together with 0046TUISIMPLA-006, the registry now covers the action and
  non-action baseline matrix.

Verification:

- `cargo test -p tracewake-tui --test playable_capability_parity`
- `cargo test -p tracewake-content`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace`
