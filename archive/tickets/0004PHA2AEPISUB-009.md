# 0004PHA2AEPISUB-009: Actor-known notebook projection and embodied view model

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adds the actor-known notebook projection and extends the embodied view model in `tracewake-core` (`view_models.rs`, `projections.rs`).
**Deps**: 0004PHA2AEPISUB-007

## Problem

Embodied mode must show only actor-known reality (`INV-067`), and actor-knowledge leakage is a high-severity defect (`INV-093`). Spec 0004 §8.8/§12.1/§12.2 require an actor-known notebook projection (source-bound beliefs, recent observations, confidence, acquisition tick, contradiction links, expectation-contradiction summary, and actor-known leads only if directly derivable and not quest-like) and an embodied view-model extension that consumes the actor's `KnowledgeContext` — never ground truth. The notebook must exclude hidden truth, debug notes, previous-body knowledge, omniscient culprit labels, and other actors' private beliefs.

## Assumption Reassessment (2026-06-06)

1. `crates/tracewake-core/src/view_models.rs` has an embodied physical view model plus debug view models, but no notebook/belief/observation view; `crates/tracewake-core/src/projections.rs` builds embodied views from physical locality and hides closed-container contents physically, with no actor-known belief filtering. This ticket adds both.
2. Notebook contents/exclusions and the recommended wording are fixed by Spec 0004 §12.1/§12.2; the actor-known filter reads through the `KnowledgeContext` (ticket 003) over the projection populated by tickets 005/007.
3. Shared boundary under audit: `view_models.rs` and `projections.rs` are modified here and by ticket 010 (debug epistemics) as **parallel independent siblings** — both branch off ticket 007 with no dep between them, so they will mechanically conflict on `view_models.rs`; implementers must coordinate the merge. This ticket owns the embodied/notebook surface; ticket 010 owns the debug surface.
4. Invariant motivating this ticket: `INV-067` (embodied mode shows actor-known reality), `INV-024` (no telepathy), and `INV-059` (leads/tasks are source-bound projections over claims/beliefs, not ground-truth objectives — the notebook's "possible leads" are projections, never quest markers).
5. Actor-knowledge / no-leak surface: this is the primary embodied no-leak enforcement ticket. The notebook is built strictly from records the viewer's `KnowledgeContext` admits; it must never read `PhysicalState` ground truth, possession history, or other holders' beliefs. No nondeterminism: notebook entries are ordered by stable id/acquisition tick. Candidate-goal *planning* is deferred to Phase 3 (spec §3.2); only derivable, non-quest-like leads surface here.

## Architecture Check

1. Building the notebook as a projection over the `KnowledgeContext`-filtered records (not a separate writable store) guarantees there is no second path that could admit hidden truth, and keeps the notebook a pure read model rebuilt from beliefs/observations/contradictions. Dumping the full belief store into embodied mode (the §12.1 anti-pattern) is explicitly avoided.
2. No backwards-compatibility shims: the embodied view model is extended additively; the physical view model is unchanged.

## Verification Layers

1. Embodied actor-known only (`INV-067`/`INV-093`) -> manual review + leak test: the notebook for Tomas after checking shows the missing-property belief with source/confidence/contradiction summary but contains no culprit/ground-truth/previous-possession string (the negative-assertion test seeded here, fully exercised in ticket 016).
2. No telepathy (`INV-024`) -> unit test: a notebook built for actor A contains no record sourced from actor B's private beliefs or from unobserved truth.
3. Leads are projections (`INV-059`) -> manual review: any "possible leads" line is derived from the actor's own beliefs and carries no objective/quest marker.

## What to Change

### 1. Actor-known notebook projection

Add a `notebook` projection builder (in `projections.rs`, backed by a `NotebookView` type in `view_models.rs`) that takes a `KnowledgeContext` + `EpistemicProjection` and returns source-bound beliefs, recent observations, confidence, acquisition/last-verified tick, contradiction links, and an expectation-contradiction summary, excluding all §12.2 forbidden content.

### 2. Embodied view-model extension

Extend the embodied view model with the §12.1 actor-known companion fields (`knowledge_context_id`/viewer marker, `notebook_summary_available`, salient actor-known beliefs, recent actor-known observations, known contradictions, source/confidence labels), sourced from the `KnowledgeContext`, not ground truth.

### 3. Actor-known filtering in projections

Add the actor-known belief/observation filtering path to `projections.rs` so embodied projection composes physical locality with `KnowledgeContext`-admitted epistemic records.

## Files to Touch

- `crates/tracewake-core/src/view_models.rs` (modify — add `NotebookView` + embodied companion fields; shared with ticket 010)
- `crates/tracewake-core/src/projections.rs` (modify — notebook builder + actor-known filtering)

## Out of Scope

- Debug epistemics view models (ticket 010).
- TUI `notebook` command, parser, and renderer (ticket 011).
- The beliefs/contradictions themselves (tickets 002/007).

## Acceptance Criteria

### Tests That Must Pass

1. Tomas's notebook after checking shows a missing-property belief with source, confidence, acquisition tick, and a "contradicts your earlier expectation" summary.
2. Tomas's notebook contains no substring naming Mara as culprit, no debug note, and no previous-possession knowledge.
3. A notebook built for Tomas excludes Elena's private beliefs.

### Invariants

1. The notebook is a pure projection over `KnowledgeContext`-admitted records; it reads no ground truth.
2. Possible-leads entries are source-bound projections (`INV-059`), never quest markers.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/projections.rs` (unit tests) — actor-known filtering, notebook contents/exclusions.
2. `crates/tracewake-core/src/view_models.rs` (unit tests) — embodied companion fields sourced from context.

### Commands

1. `cargo test -p tracewake-core projections:: view_models::`
2. `cargo test -p tracewake-core`
3. `cargo build --workspace --all-targets --locked`

## Outcome

Completion date: 2026-06-07

What changed:
- Added actor-scoped `NotebookView` and notebook entry view models for source-bound beliefs, recent observations, contradictions, and source-bound possible leads.
- Extended `EmbodiedViewModel` with optional `knowledge_context_id` and `notebook` companion fields while preserving existing physical view behavior.
- Added `build_notebook_view` and `build_embodied_view_model_with_notebook`, both driven by `KnowledgeContext`-filtered epistemic projection records.
- Added tests proving Tomas's notebook shows missing-property belief/source/confidence/contradiction summary, excludes Mara/culprit/debug/previous strings, and excludes Elena's private belief.

Deviations from original plan:
- `tracewake-tui` test literals were updated with `None` for the new optional embodied fields.
- The ticket's combined cargo test filter is not valid cargo syntax, so `projections::` and `view_models::` were run as separate filters, followed by full core tests.

Verification results:
- `cargo test -p tracewake-core projections::`
- `cargo test -p tracewake-core view_models::`
- `cargo test -p tracewake-core`
- `cargo build --workspace --all-targets --locked`
- `cargo fmt --all --check`
