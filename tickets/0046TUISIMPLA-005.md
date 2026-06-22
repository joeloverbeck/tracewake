# 0046TUISIMPLA-005: Real-pipeline capability-golden harness — typed→rendered witnesses, anti-leak pairing, checked-in renderings

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new `tracewake-tui` scenario harness (`crates/tracewake-tui/tests/parity/scenario.rs`) + checked-in golden directory (`crates/tracewake-tui/tests/goldens/`); extends the registry test target; preserves `transcript_snapshot.rs`. No production-crate change.
**Deps**: 0046TUISIMPLA-003, 0046TUISIMPLA-004

## Problem

Spec 0046 §4.2 `PAR-004`/`PAR-005`/`PAR-006`. The registry (003) and runner (004) prove an entry is
*declared* complete; they do not prove the declared witnesses actually arise from the real pipeline.
This ticket establishes the per-capability scenario harness that runs the genuine path — load a
manifest-backed `GoldenFixture` through the ordinary content path, possess/bind the viewer through
`TuiApp`, advance/replay through the normal scheduler/event path, build `TuiApp::current_view`, call
`render_current_view`, and (for actionable capabilities) select + submit a real `SemanticActionEntry`
through `TuiApp::submit_semantic_action` — and asserts witnesses in the spec's required order, with
anti-leak pairing for epistemic features and checked-in accepted renderings (Step-4 decision **Q2 =
(a)**: the repo's byte-stable checked-in style, not `insta`). The determinism test
`transcript_snapshot.rs` is preserved and **not** reinterpreted as content acceptance.

## Assumption Reassessment (2026-06-22)

1. Verified against code at baseline `1145e109`: the real seam is `TuiApp` (`crates/tracewake-tui/src/app.rs:74`)
   with `current_view` (`:171`), `render_current_view` (`:219`), `submit_semantic_action` (`:232`);
   `build_embodied_view_model` (`crates/tracewake-core/src/projections.rs:539`) gathers actor-known
   routes/doors/containers/items/local-actors/semantic-actions; `EmbodiedViewModel::semantic_actions`
   is `Vec<SemanticActionEntry>` (`crates/tracewake-core/src/view_models.rs:32`). Fixtures load via
   `fixtures::by_id` (`crates/tracewake-content/src/fixtures/mod.rs:445`). `transcript_snapshot.rs`
   asserts `assert_eq!(first_sections, second_sections)` (`:16`) — a run-A-equals-run-B determinism
   check, not an approval golden.
2. Verified against spec 0046 §4.2 (`PAR-004`–`006`): the ticket consumes the registry/runner from
   003/004 (in-batch create-then-modify on `parity/mod.rs` + the test target). The golden style is the
   repo's explicit byte-stable form per Q2(a); `insta` is explicitly not added (§8 no-new-deps, §1.2
   no-uplift).
3. Shared boundary under audit: the projection→render seam (`build_embodied_view_model` →
   `render_current_view`). The harness asserts the **typed** causal witness before the **rendered**
   witness, so a passing render can never substitute for a missing typed cause.
4. Invariant restated (`PAR-005` motivation): `INV-067` embodied mode shows actor-known reality;
   `INV-093` actor-knowledge leakage is a high-severity defect; `INV-099`/`INV-100`/`INV-101` truth may
   validate but not plan, hidden-truth cognition is forbidden, the actor-known context is sealed;
   `INV-107` debug omniscience is quarantined.
5. Enforcement surface touched (actor-knowledge filtering + deterministic replay): the harness asserts
   actor-knowledge witnesses through the sealed actor-known context and forbids satisfying an embodied
   assertion from hidden/validator-only truth. Witness order is fixed: (1) typed causal witness,
   (2) actor-knowledge witness (with source/freshness where relevant), (3) rendered witness (checked-in
   text), (4) negative/anti-leak witness for epistemic/hidden-state features (unknown/stale/
   contradictory/unobserved stays absent or disabled with an actor-safe why-not), (5) debug witness if
   required (diagnoses the mechanism without satisfying the embodied assertion). For autonomous-agent
   features the embodied witness proves only legitimately observable consequence — never private
   intentions or hidden truth. Where doctrine requires, rebuild/replay and compare the actor-filtered
   result; `transcript_snapshot.rs` is preserved unchanged as the determinism check.

## Architecture Check

1. A shared scenario harness (load → possess → advance → build → render → optionally submit → re-render)
   parameterized by a registry entry is cleaner than bespoke per-capability tests: it guarantees every
   capability runs the *real* path (not a hand-built view model), enforces the typed-before-rendered
   ordering uniformly, and makes the census (006/007) a matter of adding entries + goldens. Checked-in
   byte-stable renderings keep diffs reviewable without a snapshot-library dependency.
2. No backwards-compatibility aliasing/shims: the harness drives the production `TuiApp` seam directly;
   no parallel "fake render" path is introduced, and `transcript_snapshot.rs` is not repurposed.

## Verification Layers

1. `PAR-005`/`INV-099`–`101` (typed-before-rendered, sealed context) → real-pipeline scenario: an
   exemplar capability asserts its typed causal witness, then its actor-knowledge witness, then its
   rendered witness, all from the real `TuiApp` path.
2. `INV-093`/`INV-070` (anti-leak + why-not) → paired negative: an epistemic exemplar's
   unknown/stale/unobserved case stays absent/disabled with an actor-safe why-not; no hidden truth
   appears in the embodied render.
3. `INV-018` (determinism) → replay/golden check: the checked-in rendering is byte-stable; the
   preserved `transcript_snapshot.rs` run-A-equals-run-B check still passes.

## What to Change

### 1. Scenario harness (`crates/tracewake-tui/tests/parity/scenario.rs`)

Add `run_scenario(entry: &CapabilityEntry) -> ScenarioWitnesses` that loads the entry's fixture via the
ordinary content path, binds the viewer through `TuiApp`, advances/replays per the entry's setup op,
builds `current_view`, calls `render_current_view`, and for actionable entries selects + submits the
declared `SemanticActionEntry` via `submit_semantic_action`, then re-renders. Return the typed,
actor-knowledge, rendered, and (where applicable) anti-leak/debug witnesses for ordered assertion.

### 2. Checked-in golden convention (`crates/tracewake-tui/tests/goldens/`)

Establish the byte-stable golden storage (short, reviewable, generated through the real TUI path) and a
helper that compares a scenario's rendered witness to its checked-in golden. Add at least one positive
exemplar and one paired epistemic anti-leak exemplar to prove the pattern (the census fills the rest).

### 3. Preserve the determinism test

Leave `crates/tracewake-tui/tests/transcript_snapshot.rs` as the determinism check; do not convert it
to a content-acceptance golden. A broad integrated transcript MAY supplement but must not be the only
completeness evidence.

## Files to Touch

- `crates/tracewake-tui/tests/parity/scenario.rs` (new)
- `crates/tracewake-tui/tests/goldens/` (new — exemplar golden files)
- `crates/tracewake-tui/tests/parity/mod.rs` (modify — file created by 0046TUISIMPLA-003)
- `crates/tracewake-tui/tests/playable_capability_parity.rs` (modify — file created by 0046TUISIMPLA-003)

## Out of Scope

- Enumerating all 15 actions / 6 families (`PAR-009`) — tickets 006/007 (this ticket adds exemplars
  only, to prove the harness).
- Bind-time perception behavior (spec §9.6) — owned by the architecture owner, not added as a
  parity-test convenience.
- Any production-crate or projection change; any repurposing of `transcript_snapshot.rs`.

## Acceptance Criteria

### Tests That Must Pass

1. The positive exemplar asserts typed→actor-knowledge→rendered witnesses in order, all from the real
   `TuiApp` pipeline; the rendered witness matches a checked-in byte-stable golden.
2. The paired epistemic exemplar's anti-leak case stays absent/disabled with an actor-safe why-not; no
   hidden/validator-only truth appears in the embodied render (`INV-093` negative).
3. `transcript_snapshot.rs` remains green as the determinism check; `cargo test -p tracewake-tui
   --test playable_capability_parity` and the four gates pass.

### Invariants

1. Every capability witness is produced by the real pipeline; a rendered witness never substitutes for
   a missing typed causal witness (ordering enforced).
2. No embodied assertion is satisfiable from hidden or debug truth; epistemic absence yields an
   actor-safe why-not.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-tui/tests/playable_capability_parity.rs` — exemplar positive scenario + paired
   epistemic anti-leak scenario through the harness, against checked-in goldens.

### Commands

1. `cargo test -p tracewake-tui --test playable_capability_parity`
2. `cargo test -p tracewake-tui --test transcript_snapshot`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
