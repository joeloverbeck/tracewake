# 0024PHA3ACONSCH-007: Embodied truth-access compile-time completion

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` (`projections.rs`, `controller.rs`), `tracewake-tui` (`app.rs`), core tests (`hidden_truth_gates.rs`, `anti_regression_guards.rs`).
**Deps**: 0024PHA3ACONSCH-001

## Problem

Spec 0024 findings `ORD-HARD-143` (medium) and `ORD-HARD-154` (low): 0023's promised
compile-time truth-access lock did not land. `build_embodied_view_model` still takes
`state: &PhysicalState` (flowing into `SemanticActionPreflightContext`), and
`EmbodiedProjectionSource::from_sealed_context` reads `state.items` /
`state.places` / `state.actors` live for carried-item fields (`portable`, source)
and `place_label`. The only enforcement is `guard_014`'s two-token denylist
(`source.state`, `visible_locality`), which a bare `state.items` read inside the
builder evades — leaving the INV-067 boundary one unreviewed edit away from a
truth-derived surface again. The display surface itself is verified migrated; this
ticket completes the lock tier.

## Assumption Reassessment (2026-06-12)

1. Verified against code at baseline `4d62f61`: the `state: &PhysicalState`
   parameter on `build_embodied_view_model` and `from_sealed_context`
   (`projections.rs`); the live `state.items.get(item_id)` / `item.portable` /
   place `display_label` reads; `guard_014_embodied_projection_source_has_no_physical_state_field`'s
   token set. Caller blast radius grepped at decomposition: `projections.rs`,
   `controller.rs`, `crates/tracewake-tui/src/app.rs`,
   `crates/tracewake-core/tests/hidden_truth_gates.rs`,
   `crates/tracewake-core/tests/anti_regression_guards.rs`.
2. Verified against spec 0024 §4 (`ORD-HARD-143`/`154`), 0023 §4 `ORD-HARD-121`'s
   structural-lock clause (the unredeemed compile-time promise), and
   `docs/1-architecture/10_*`'s generation rule.
3. Cross-artifact boundary: the embodied view-model generation contract — the
   builder consumes only the sealed `EmbodiedProjectionSource`; truth access for
   action preflight stays lawful (INV-099: truth may validate) but moves behind a
   boundary the builder cannot iterate.
4. INV-024/INV-067/INV-093 restated: embodied views are generated from actor-known
   context; truth may not be readable from the generation path; leakage surfaces
   are high-severity defects requiring structural (here compile-time) prevention.
5. Enforcement surface (actor-knowledge filtering): this ticket strengthens the
   firewall — the display surface already consumes projected facts (verified); the
   change removes the *capability* to regress. Preflight validation semantics are
   relocated, not weakened: the validator still checks authoritative truth at
   proposal time (INV-099), and the existing INV-093 absence negative and staleness
   positive must pass unchanged.
6. View-model projection shape change (breaking-internal): `EmbodiedProjectionSource`
   gains construction-time-derived fields (place label, carried-item attributes)
   and the builder loses its `&PhysicalState` parameter. Consumers — the four
   caller files in item 1 — are all in-workspace and updated in this same diff
   (local compile-atomicity: splitting would land an uncompilable intermediate
   state; deliberate Large size flagged at decomposition).
7. Removal blast radius (`&PhysicalState` parameter + any retired helper): grepped
   per item 1 — four caller files plus the guard; no doc-governed contract names the
   Rust signature, so docs are unaffected until the capstone's conformance rows.
8. Adjacent contradiction (spec §9 risk, implementer judgment recorded): if the
   preflight refactor resists cleanly removing truth from the builder, the recorded
   fallback is the `ValidatorOnlyState` newtype quarantine (no map accessors) plus
   the widened `guard_014` — an owner-recorded narrowing per spec §9, not a silent
   re-scope.

## Architecture Check

1. Deriving carried-item attributes and the place label at sealed-source
   construction (observation/body-state time) and passing the validator a
   preflight-result or capability object — rather than handing the builder raw
   truth — makes the truth-free builder a *type-level* fact, the lineage's strongest
   tier, and removes the need for ever-wider token denylists. The fallback newtype
   preserves the same property with one more seam if the preflight resists.
2. No backwards-compatibility aliasing/shims: the `&PhysicalState` parameter is
   removed (or sealed), not defaulted; no truth-reading helper survives behind a
   wrapper.

## Verification Layers

1. INV-067/024 compile-time lock → grep-proof: `PhysicalState` is not nameable in
   `build_embodied_view_model` / `from_sealed_context` signatures or bodies; the
   widened `guard_014` bans the type token in the builder region with a firing
   bare-`state.items`-read synthetic (codebase grep-proof + test).
2. INV-093 non-regression → the existing absence negative
   (`embodied_projection_omits_unobserved_present_item_and_actor`) and staleness
   positive (`embodied_projection_renders_stale_projected_item_not_live_truth`)
   pass unchanged; a new place-label staleness positive renders from the recorded
   label, not live truth (tests).
3. Preflight legality preserved (INV-099/043) → existing semantic-action
   validation tests pass: rejected proposals still reject, accepted still accept
   (replay/golden-fixture check).
4. Whole-pipeline → full workspace gates; golden re-derivation only if repricing
   is honest and recorded (spec §8 ordering note).

## What to Change

### 1. Sealed-source derivation (`ORD-HARD-154`)

`EmbodiedProjectionSource` carries construction-time place label and carried-item
attributes (membership stays body-state per 0023's sanction; per-item `portable`/
source fields and the label are derived where the source is constructed, not read
live in the builder).

### 2. Parameter removal (`ORD-HARD-143`)

Remove `state: &PhysicalState` from `build_embodied_view_model` and
`from_sealed_context`; preflight receives a sealed result/capability. Update all
four caller files in the same diff. Fallback per Assumption item 8 if needed.

### 3. Lock widening

`guard_014` additionally bans the `PhysicalState` token across the embodied-builder
region as the backstop, with a synthetic re-introducing a bare `state.items` read
that must fail; enroll under the live-witness rule (ticket -001).

### 4. Staleness positive

Place-label staleness test: truth's label changes post-observation; the embodied
view renders the recorded label.

## Files to Touch

- `crates/tracewake-core/src/projections.rs` (modify)
- `crates/tracewake-core/src/controller.rs` (modify)
- `crates/tracewake-tui/src/app.rs` (modify)
- `crates/tracewake-core/tests/hidden_truth_gates.rs` (modify)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)

## Out of Scope

- INV-029 identity-uncertainty mechanics (the existing seam is preserved, not
  extended — spec Out of scope).
- Re-pointing the perception emitters (landed in 0023; untouched).
- The render/debug-gating surfaces (ticket -009).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -n "PhysicalState" `-proof over `build_embodied_view_model` and
   `from_sealed_context`: the type is not nameable there (or, under the recorded
   fallback, only as the sealed `ValidatorOnlyState` newtype with no map accessors).
2. The widened `guard_014` synthetic (bare `state.items` read in the builder) fails;
   the existing INV-093 absence negative and staleness positive pass unchanged; the
   new place-label staleness positive passes.
3. Semantic-action preflight behavior is unchanged (existing acceptance/rejection
   tests green); any golden repricing is re-derived once, honestly, in this ticket.
4. The four workspace gates pass.

### Invariants

1. The embodied builder cannot iterate authoritative physical truth — enforced at
   the type level (or sealed-newtype level under the recorded fallback), with the
   token guard as backstop.
2. Truth continues to validate proposals (INV-099) — relocated, never weakened.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — widened `guard_014` +
   bare-read synthetic, live-witness enrollment.
2. `crates/tracewake-core/tests/hidden_truth_gates.rs` — place-label staleness
   positive; updated call sites.

### Commands

1. `cargo test -p tracewake-core --test hidden_truth_gates --test anti_regression_guards`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome

Completed: 2026-06-12

Implemented the embodied truth-access compile-time lock by moving physical-state
reads for place label and carried-item attributes into an `EmbodiedTruthSnapshot`
constructed before view generation. `EmbodiedProjectionSource::from_sealed_context`
now consumes that snapshot, while `build_embodied_view_model` no longer accepts
`&PhysicalState`, `ActionRegistry`, or `ContentManifestId` directly. Truth-backed
semantic-action validation remains available only through `EmbodiedPreflightSource`.

Updated core, controller, TUI, and test call sites to use the snapshot/preflight
boundary. Widened `guard_014` to reject `PhysicalState`/bare `state.items` style
builder regressions and added a live synthetic proving that the new guard fires.
Added `embodied_place_label_is_captured_before_truth_preflight` to prove a
post-snapshot truth label change does not alter the embodied view label.

Verification:

1. `cargo test -p tracewake-core --test hidden_truth_gates --test anti_regression_guards` passed.
2. `cargo test -p tracewake-core` passed.
3. `cargo fmt --all --check` passed.
4. `cargo clippy --workspace --all-targets -- -D warnings` passed.
5. `cargo build --workspace --all-targets --locked` passed.
6. `cargo test --workspace` passed.
