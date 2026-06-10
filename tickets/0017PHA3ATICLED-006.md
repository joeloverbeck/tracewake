# 0017PHA3ATICLED-006: Believed-access workplace facts and actor-known embodied availability

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` (`epistemics/knowledge_context`, `epistemics/projection`, `projections`); `tracewake-content` (one reworked + one new fixture); `tracewake-tui` (embodied tests); one new source guard
**Deps**: `archive/tickets/0017PHA3ATICLED-003.md` (workplace facts flow through the projection freshness rework); `specs/0017_PHASE_3A_TICK_LEDGER_EPISTEMIC_STALENESS_REPLAY_PAYLOAD_EVIDENCE_AND_GENERATIVE_LOCK_HARDENING_SPEC.md` (ORD-HARD-029)

## Problem

Embodied workplace believed-access was never modeled, so the 0016 fix landed hollow. `epistemics/knowledge_context.rs::ActorKnownWorkplaceFact` carries no access dimension — belief-vs-truth divergence is structurally unrepresentable; `projections.rs::phase3a_semantic_actions` computes workplace `enabled = at_workplace` alone and pushes the sleep/eat/continue entries with hard-coded `SemanticActionEntry::new(..., true, None)` (the Phase-1/2 entries route through `with_validator_availability`; the Phase-3A entries assert availability by fiat). The 0016-mandated adversarial fixture `embodied_workplace_availability_reflects_belief_not_truth_001` seeds `initial_beliefs: Vec::new()` and re-tests the already-covered absence case — the "truth open, belief closed ⇒ embodied shows closed" case is untested, and the 0016 acceptance report overstates it as proven (INV-067, INV-070; architecture doc 03's embodied-affordance formula). The current error direction is leak-safe (`guard_014`'s truth-read ban holds); what's missing is the belief side.

## Assumption Reassessment (2026-06-10)

1. Verified against current code at `main` `114e2af`: `ActorKnownWorkplaceFact { workplace_id, place_id, source_key }` — no access dimension (the spec's prose said "workplace_id and place_id only"; the third field `source_key` exists but changes nothing — mechanical drift, corrected here); `phase3a_semantic_actions` pushes `SemanticActionEntry::new(` directly at three sites while the general `semantic_actions` builder wraps entries in `with_validator_availability`; the fixture's `initial_beliefs: Vec::new()` and its TUI test assert absence only.
2. Spec 0017 §ORD-HARD-029: carry believed-access on the workplace fact (sourced from notice/observation events through the projection); compute embodied availability from actor-known facts; deliver the real divergence fixture pair; belief decides the menu, the validator decides the commit (INV-099).
3. Cross-artifact boundary under audit: the knowledge-context workplace fact ↔ embodied projection contract — embodied availability must be a function of context facts only, never `PhysicalState`.
4. INV-067/INV-070 restated: embodied mode shows actor-known reality, and blocked actions explain missing preconditions in actor-known terms — availability computed from belief, with why-not from believed conditions; truth surfaces only at commit time through modeled consequences (INV-106).
5. Actor-knowledge filtering surface touched: this ticket moves availability *toward* belief (no truth read is introduced; `guard_014`'s `state.workplaces` ban remains). The inverse divergence fixture (belief open, truth closed) exercises the INV-106 learning loop — embodied shows available, commit fails with the modeled `access` consequence. Deterministic replay: fixtures replay byte-stable; embodied projections are derived views, not checksum inputs, beyond the golden trace expectations updated as surfaced.
6. Schema extension keyed separately from item 5: `ActorKnownWorkplaceFact` (view-feeding knowledge-context schema) gains a believed-access field; where the spec's lock requires the zero-known-servings eat case, the food fact gains a believed-servings dimension the same way. Consumers: `epistemics/projection.rs` record producers, `agent/no_human_surface.rs` structured derivation (post `-004`), `projections.rs` availability computation. Crate-internal struct — all consumers updated in this diff (local compile-atomicity), additive in intent.
7. Mismatch + correction: the spec's field enumeration omitted `source_key` (see item 1) — no scope impact; the ticket proceeds against the actual three-field struct.

## Architecture Check

1. Computing Phase-3A embodied availability from actor-known facts (an actor-known preflight) is the architecture doc 03 formula applied uniformly — strictly better than routing through `with_validator_availability`, which reads validator truth and would *introduce* the leak direction this surface has so far avoided. Hard-coded `true` flags are replaced by derived availability with actor-known why-not, eliminating the projection-local fiat that ORD-HARD-029 flagged.
2. No backwards-compatibility aliasing/shims: the literal-`true` entry construction is deleted; the placeholder fixture is given real divergence content rather than kept alongside a second "real" fixture.

## Verification Layers

1. INV-067 belief-over-truth -> reworked fixture `embodied_workplace_availability_reflects_belief_not_truth_001`: context workplace fact with believed-access closed, truth open ⇒ embodied shows unavailable-with-reason; debug shows the divergence non-diegetically; replay byte-match.
2. INV-106 learning loop -> new inverse fixture `embodied_workplace_believed_open_truth_closed_commit_fails_001`: belief open, truth closed ⇒ embodied shows available; commit fails with the modeled `access` consequence.
3. No fiat availability -> source guard: no `SemanticActionEntry::new` call with a literal `true` enabled flag in `phase3a_semantic_actions`; runtime backstop: zero believed servings ⇒ eat entry disabled with a typed actor-known reason.
4. No truth read -> existing `guard_014_embodied_projection_*` family still passes (grep-proof that `state.workplaces` stays banned in the builder).

## What to Change

### 1. Believed-access on the workplace fact

`ActorKnownWorkplaceFact` gains a believed-access dimension, populated from role-assignment notices and workplace observations through `EpistemicProjection` (under `-003`'s freshness rule); the food fact gains believed servings analogously for the eat-availability backstop.

### 2. Actor-known availability in `phase3a_semantic_actions`

Workplace availability = at-place (context) ∧ believed-access; sleep/eat/continue entries derive enabled/why-not from context facts (known sleep affordance, believed servings > 0, active-intention fact) instead of literal `true`. Why-not strings stay actor-known-termed.

### 3. Fixture pair + TUI tests

Rework the placeholder fixture into the real divergence case; add the inverse fixture; register in `fixtures/mod.rs`; extend `embodied_flow.rs` with both renders (closed-shown / available-then-commit-fails) asserting band-style actor-known phrasing.

### 4. Guard

`anti_regression_guards.rs`: ban the literal-`true` `SemanticActionEntry::new` form in the Phase-3A builder.

## Files to Touch

- `crates/tracewake-core/src/epistemics/knowledge_context.rs` (modify)
- `crates/tracewake-core/src/epistemics/projection.rs` (modify)
- `crates/tracewake-core/src/projections.rs` (modify)
- `crates/tracewake-core/src/agent/no_human_surface.rs` (modify — structured derivation of the extended facts, as surfaced post `-004`)
- `crates/tracewake-content/src/fixtures/embodied_workplace_availability_reflects_belief_not_truth_001.rs` (modify)
- `crates/tracewake-content/src/fixtures/embodied_workplace_believed_open_truth_closed_commit_fails_001.rs` (new)
- `crates/tracewake-content/src/fixtures/mod.rs` (modify)
- `crates/tracewake-tui/tests/embodied_flow.rs` (modify)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify)

## Out of Scope

- Validator-side access semantics (`work.rs` commit-time checks stay as they are — truth validates, belief plans).
- Need display (band-only rendering already holds); debug projections beyond the divergence comparison.
- Freshness/provenance mechanics (ticket `-003`/`-004` surfaces).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p tracewake-tui embodied_workplace_availability_reflects_belief_not_truth` — belief-closed/truth-open shows closed.
2. `cargo test -p tracewake-tui embodied_workplace_believed_open_truth_closed` — belief-open/truth-closed shows available, commit fails with modeled `access` consequence.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

### Invariants

1. Phase-3A embodied availability is a function of actor-known context facts only — no `PhysicalState` read, no literal `true` fiat.
2. Belief decides the menu; the validator decides the commit; validation failure teaches only modeled consequences (INV-099/INV-106).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/src/fixtures/embodied_workplace_availability_reflects_belief_not_truth_001.rs` — real divergence content (belief closed, truth open).
2. `crates/tracewake-content/src/fixtures/embodied_workplace_believed_open_truth_closed_commit_fails_001.rs` — inverse case + commit consequence.
3. `crates/tracewake-tui/tests/embodied_flow.rs` — both renders.
4. `crates/tracewake-core/tests/anti_regression_guards.rs` — literal-`true` ban; `guard_014` family unchanged-passing.
5. `crates/tracewake-core/src/projections.rs` (unit test) — zero believed servings disables eat with typed reason.

### Commands

1. `cargo test -p tracewake-tui embodied_workplace`
2. `cargo test --workspace`
