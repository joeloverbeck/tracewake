# 0016PHA3ANEEACC-007: Belief-gated embodied workplace availability; band-only need display

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` workplace-fact attributes + embodied projection rewiring; embodied need-display field removal; TUI render update; new fixture + guard extension
**Deps**: 0016PHA3ANEEACC-005

## Problem

ORD-HARD-019 (completes the unfinished half of ORD-HARD-010): two embodied leaks (INV-067, INV-069; architecture doc 03's embodied-affordance formula; foundation doc 08's banded need display):

- `projections.rs::phase3a_semantic_actions`'s workplace branch iterates `source.actor_known_workplaces` (context-backed identity — the 0014/0015 fix) but then calls `state.workplaces.get(workplace_id)` and computes `enabled = at_workplace && workplace.access_open` — ground-truth location and open/closed truth decide the embodied affordance. An actor with stale knowledge ("workplace closed") sees truth, not belief.
- `phase3a_status` builds `NeedStatusEntry { value: need.value(), … }` and the embodied render prints `value={} band={}` — the exact internal scalar on the embodied surface, where doctrine requires bands (exact numbers debug-only).

## Assumption Reassessment (2026-06-10)

1. Current code verified at baseline `ba84e75`: workplace branch at `crates/tracewake-core/src/projections.rs:688–725` (context-backed identity, truth-gated availability); `phase3a_status` at :622–625 with exact `value`; TUI render prints `"- {}: value={} band={} cause={}"` at `crates/tracewake-tui/src/render.rs:48`; `NeedStatusEntry` consumers are `projections.rs`, `view_models.rs`, and the TUI render (grep-verified). `guard_014_embodied_projection_workplaces_are_context_backed` exists (`anti_regression_guards.rs:1508`) but covers identity, not availability; existing tests (`embodied_view_omits_raw_workplace_assignment_without_context`, tui `embodied_flow.rs`) assert absence-without-context, never belief/truth divergence; render tests assert the band label is present, never that the raw scalar is absent.
2. Spec/docs: spec 0016 §ORD-HARD-019 (evidence, required correction, structural lock); `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` (embodied-affordance formula); `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md` (banded need display embodied; exact numbers debug-only); `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md` INV-067, INV-069.
3. Shared boundary under audit: the projection/view-model seam — what the embodied surface may read. After this ticket, embodied workplace availability derives from context workplace facts (place + believed access) and embodied need display carries band + dominant cause only; `state.workplaces` is banned from the embodied semantic-action builder.
4. INV-067 — embodied mode shows actor-known reality, hiding hidden truth. INV-069 — the TUI must not query hidden truth in embodied mode; view models are presentation-only. Restated before trusting the ticket narrative.
5. Actor-knowledge / deterministic-replay surface: the enforcement surfaces are the embodied projection builder (truth read removed; guard extension bans `state.workplaces` there) and the render contract (no `value=` token embodied). Enriching workplace facts with place/believed-access changes the serialized actor-known input shape, so recorded decision context hashes may reprice — diffs must be explained, and ticket 003's from-log re-derivation gate must stay green against the new shape (it re-derives via the production builder, so it verifies rather than fights this change).
6. Schema extension: workplace facts on the actor-known surface gain place + believed-access attributes (consumers: the planning-context accessors from 0016PHA3ANEEACC-005, the embodied projection, the surface builder). Additive on the fact side. The same change is also the item-5 surface above — keyed separately per the template contract.
7. Removal blast radius: `value` leaves the embodied `NeedStatusEntry`. Grep-enumerated consumers: `projections.rs` (producer), `view_models.rs` (embodied view shape), `crates/tracewake-tui/src/render.rs:48` (display), TUI render tests (`embodied_flow.rs`). The debug needs report retains exact values (non-diegetic surface, INV-068) — debug consumers unaffected.

## Architecture Check

1. Carrying believed access on the workplace facts is the same fix shape that already worked for food/sleep/route (ORD-HARD-010): the embodied surface consumes facts, never `state.*`. Computing availability in the view from truth was the last truth read in the embodied builder; removing the field from the embodied view model (rather than hiding it at render time) makes the leak structurally impossible rather than cosmetically suppressed.
2. No backwards-compatibility aliasing/shims: the embodied `NeedStatusEntry` loses `value` outright (no deprecated field, no debug passthrough on the embodied path); the truth-gated availability branch is removed, not feature-flagged.

## Verification Layers

1. INV-067 (belief decides availability) → adversarial fixture `embodied_workplace_availability_reflects_belief_not_truth_001`: truth open, belief closed ⇒ embodied shows closed.
2. INV-069 (no truth reads in embodied builder) → codebase grep-proof: `guard_014_embodied_projection_*` family extended to ban `state.workplaces` in the embodied semantic-action builder.
3. Banded display (foundation 08) → render test: embodied output contains the band label and does not contain the `value=` token.
4. INV-018 (replay) → replay/golden-fixture check: repriced context-input shapes replay byte-identically; ticket 003's re-derivation gate green against the new fact shape.

## What to Change

### 1. Workplace facts carry place and believed access

The surface builder's workplace facts gain place + believed-access attributes (sourced from the same evented knowledge channels that back workplace identity).

### 2. Embodied availability from belief

`phase3a_semantic_actions` computes workplace `enabled` from the context's workplace facts (believed place reachability + believed access) — no `state.workplaces` read. The debug projection may show the belief-vs-truth comparison non-diegetically.

### 3. Band-only embodied need display

Drop `value` from the embodied `NeedStatusEntry` (band + dominant cause remain); update `view_models.rs` and `render.rs`; exact values remain in the debug needs report.

### 4. Locks

Guard-family extension (ban `state.workplaces` in the embodied builder); the adversarial fixture; the render negative test.

## Files to Touch

- `crates/tracewake-core/src/projections.rs` (modify)
- `crates/tracewake-core/src/view_models.rs` (modify)
- `crates/tracewake-core/src/agent/no_human_surface.rs` (modify — workplace-fact attributes)
- `crates/tracewake-core/src/agent/actor_known.rs` (modify — fact attribute surface, atop ticket 005's accessors)
- `crates/tracewake-tui/src/render.rs` (modify)
- `crates/tracewake-tui/tests/embodied_flow.rs` (modify — render negative test)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — guard extension)
- `crates/tracewake-content/src/fixtures/embodied_workplace_availability_reflects_belief_not_truth_001.rs` (new)
- `crates/tracewake-content/src/fixtures/mod.rs` (modify)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify — run the new fixture; context-hash diffs explained)

## Out of Scope

- Multi-tick staleness of embodied knowledge from the epistemic projection (0016PHA3ANEEACC-008 — Deps on this ticket).
- The computed-accessor restructure itself (0016PHA3ANEEACC-005).
- Debug-view redesign (the debug needs report keeps its current exact-value shape).

## Acceptance Criteria

### Tests That Must Pass

1. `embodied_workplace_availability_reflects_belief_not_truth_001`: truth open + belief closed ⇒ embodied shows closed (and the inverse divergence honors belief).
2. Render test: embodied need output contains the band label and not the `value=` token; debug needs report still carries exact values.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`.

### Invariants

1. The embodied semantic-action builder reads no `state.workplaces` (guard-enforced); embodied availability derives only from context facts.
2. The embodied view model carries no exact need scalar; exact values exist only on debug surfaces (INV-068 quarantine).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/src/fixtures/embodied_workplace_availability_reflects_belief_not_truth_001.rs` — the ORD-HARD-019 structural-lock fixture.
2. `crates/tracewake-tui/tests/embodied_flow.rs` — render assertion: band present, `value=` absent.
3. `crates/tracewake-core/tests/anti_regression_guards.rs` — `state.workplaces` ban in the embodied builder.

### Commands

1. `cargo test -p tracewake-core projections && cargo test -p tracewake-tui`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
