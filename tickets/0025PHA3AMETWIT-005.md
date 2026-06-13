# 0025PHA3AMETWIT-005: Embodied observation-time capture and truth-carrier census

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` projections and perception
(`projections.rs`, `agent/perception.rs`), `tracewake-tui` view construction
(`app.rs`), and core test guards.
**Deps**: 0025PHA3AMETWIT-001

## Problem

Spec 0025 findings `ORD-HARD-173` (medium) and `ORD-HARD-172` (medium). The 0024
embodied refactor made truth access *explicit* but not *observation-timed*:
`TuiApp::current_view` calls
`EmbodiedTruthSnapshot::from_physical_state(&context, &self.state)` on every
render, re-reading the current place's `display_label` and each carried item's
attributes from live `PhysicalState` — a truth change after the actor's last
observation renders fresh, against INV-067's actor-known-reality contract. The
delivered staleness test (`embodied_place_label_is_captured_before_truth_preflight`)
mutates a separate preflight world, so it discriminates snapshot-vs-preflight, not
observation-vs-render — it cannot catch the defect. Meanwhile `guard_014`'s lock
region is narrower than the relocated truth surface: it scans only the
`EmbodiedProjectionSource` struct body, `from_sealed_context`, and
`build_embodied_view_model`, while the new carriers (`EmbodiedTruthSnapshot`,
`EmbodiedPreflightSource`, `SemanticActionPreflightContext`) and the module-sibling
builders (`semantic_actions`, `phase3a_semantic_actions`) are unscanned — a future
`preflight.state.items` read inside `semantic_actions` would feed the rendered
candidate set and trip nothing.

## Assumption Reassessment (2026-06-12)

1. Verified against code at baseline `9e33d7a`: `from_physical_state(&context,
   &self.state)` in `crates/tracewake-tui/src/app.rs::current_view`; zero
   references to `EmbodiedTruthSnapshot`/`EmbodiedPreflightSource` in
   `anti_regression_guards.rs`; `semantic_actions`/`phase3a_semantic_actions` are
   module-siblings of the private `state` field in
   `crates/tracewake-core/src/projections.rs`. No truth read exists in the sibling
   builders today (verified — the candidate set derives from sealed
   `actor_known_*` collections). All operator-verified per spec 0025 §8.
2. Verified against spec 0025 §4 and the archived predecessor: 0024
   `ORD-HARD-154`'s required correction ("derive the label and item attributes at
   observation/construction time into the sealed source") did not land as
   specified; `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`
   governs the sealed-context boundary.
3. Shared boundary under audit: the embodied sealed-source contract — which
   symbols may carry `PhysicalState`, and the rule that rendered values derive
   from recorded observations, with `record_current_place_perception` as the sole
   lawful truth→event boundary.
4. Invariants restated before trusting the narrative (INV-067 — embodied mode
   shows actor-known reality; INV-093 — actor-knowledge leakage is a
   high-severity defect): normal play shows the actor's perceived/believed/
   remembered world; hidden-truth freshness on the embodied surface is leakage
   even when the value is mundane.
5. Actor-knowledge and replay surfaces touched: the capture moves truth reads to
   the lawful observation boundary (`record_current_place_perception`), recording
   them as events — strengthening the firewall; the new recorded facts are evented
   and therefore replay-deterministic. Carried-item *membership* remains lawful
   body-state. No leakage path is added; the validator preflight keeps lawful
   truth access under INV-099 (truth may validate).
6. Schema extension: the actor-known projection record family
   (`ActorKnownProjectionRecord` and the perception event payloads it derives
   from) gains place-label and carried-item attribute facts. Consumers: the
   epistemic projection, the embodied builder, and the TUI view models. The
   extension is additive (new recorded fact fields/kinds with absent-until-observed
   semantics); the spec's §8 ordering note applies — land this capture *before*
   widening the census (change areas are ordered accordingly), and reprice any
   affected goldens once, honestly, in this ticket.

## Architecture Check

1. Recording the place label and carried-item attributes at observation time and
   deriving the snapshot from those records is cleaner than freezing a render-time
   copy: it makes staleness a *modeled epistemic property* (the actor knows what
   they last observed) instead of a cache artifact, matching how `visible_items`
   already derive from `ActorKnownItemFact` — one derivation discipline for the
   whole embodied surface. The carrier census then makes the remaining truth reads
   an enumerated, parity-checked set rather than a convention.
2. No backwards-compatibility aliasing/shims: `from_physical_state`'s
   label/attribute reads are removed, not kept as a fallback; the old
   snapshot-vs-preflight test is replaced by true staleness positives.

## Verification Layers

1. INV-067 observation-time capture (`ORD-HARD-173`) → true staleness positives:
   mutate `self.state`'s `display_label` and a carried item's `portable` *after*
   the recorded observation; the render must show the recorded values.
2. Carrier census (`ORD-HARD-172`) → parity assert that
   `EmbodiedTruthSnapshot`/`EmbodiedPreflightSource`/`SemanticActionPreflightContext`
   are the only `projections.rs` symbols naming `PhysicalState`; a new carrier
   fails enrollment.
3. Widened lock region → firing synthetic injecting `preflight.state.items` into
   `semantic_actions`; `guard_014` extended to the full builder cluster
   (`semantic_actions`, `phase3a_semantic_actions`, `with_validator_availability`).
4. INV-093 regression → the existing absence negative and item staleness positive
   (`embodied_projection_omits_unobserved_present_item_and_actor`,
   `embodied_projection_renders_stale_projected_item_not_live_truth`) stay green.
5. Whole-suite regression → four-gate workspace run; meta-enrollment under the
   0025PHA3AMETWIT-001 repaired routing.

## What to Change

### 1. Observation-time capture (`ORD-HARD-173` — lands first per spec §8 ordering note)

Record the current place's `display_label` and each carried item's
`portable`/`source` into actor-known facts at the
`record_current_place_perception` boundary (`agent/perception.rs`); derive
`EmbodiedTruthSnapshot` from those records; shrink `from_physical_state` to lawful
body-state membership (carried-item IDs); update `TuiApp::current_view`
construction accordingly.

### 2. True staleness positives

Replace the mis-aimed `embodied_place_label_is_captured_before_truth_preflight`
discriminator with positives that mutate the *authoritative* world post-observation
and assert the render shows recorded values (place label; carried-item
`portable`).

### 3. Carrier census + widened guard region (`ORD-HARD-172` — lands second)

Extend `guard_014`'s scanned region to the full embodied builder cluster; add the
enumerated-carrier parity census; add the `semantic_actions` truth-injection
synthetic; enroll under the repaired witness routing.

## Files to Touch

- `crates/tracewake-core/src/agent/perception.rs` (modify)
- `crates/tracewake-core/src/projections.rs` (modify)
- `crates/tracewake-tui/src/app.rs` (modify)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)

## Out of Scope

- Removing validator-preflight truth access (lawful under INV-099; the preflight
  seam stays as 0024 left it).
- The `ORD-HARD-095` bind-time perception deferral (standing owner decision,
  untouched per spec 0025 §1).
- TUI gating changes (`ORD-HARD-174`/`176`) — ticket `0025PHA3AMETWIT-006`.

## Acceptance Criteria

### Tests That Must Pass

1. Both true staleness positives pass (place label; carried-item attribute):
   truth mutated post-observation renders the recorded value.
2. The carrier parity census passes and fails on a synthetic new carrier; the
   `semantic_actions` truth-injection synthetic fires; existing INV-093 negatives
   stay green.
3. `cargo test -p tracewake-core` (including `--test anti_regression_guards`),
   `cargo test -p tracewake-tui`, and the four gates (`cargo fmt --all --check`,
   `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo build --workspace --all-targets --locked`, `cargo test --workspace`) pass.

### Invariants

1. Every embodied-rendered value derives from recorded actor-known state or lawful
   body-state membership; no render-time read of mutable world truth.
2. The set of `PhysicalState` carriers in `projections.rs` is enumerated and
   parity-locked; growth requires an explicit census edit.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/projections.rs` / its test home — true staleness
   positives (place label, carried-item `portable`).
2. `crates/tracewake-core/tests/anti_regression_guards.rs` — carrier parity
   census, widened `guard_014`, truth-injection synthetic, enrollment rows.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
