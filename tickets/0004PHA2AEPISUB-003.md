# 0004PHA2AEPISUB-003: KnowledgeContext and epistemic projection state

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adds `KnowledgeContext` and the epistemic projection store to the `tracewake-core` `epistemics` module.
**Deps**: 0004PHA2AEPISUB-002

## Problem

Phase 2A must filter what each actor knows and store epistemic state separately from physical state. Spec 0004 §8.7 requires a `KnowledgeContext` (viewer, mode, allowed/forbidden sources, scopes) used by every embodied view and knowledge-dependent precondition, and §8.8 requires an epistemic projection (observations/beliefs/contradictions/notebook-entries indexed by id and by holder, with projection version and content-manifest id) stored alongside — never on — physical entities. This is the firewall substrate the no-leak invariants depend on.

## Assumption Reassessment (2026-06-06)

1. Neither type exists: `grep -nE "KnowledgeContext|Epistemic|notebook" crates/tracewake-core/src/projections.rs crates/tracewake-core/src/state.rs` returns nothing — `projections.rs` builds embodied views from physical locality and hides closed-container contents physically, but has no explicit knowledge context or actor-known belief filter.
2. Record types (`Observation`/`Belief`/`Contradiction`) and IDs come from tickets 002/001; the projection indexes them. Spec §8.7/§8.8 fix the field sets; `docs/1-architecture/05_CLAIMS_BELIEFS_TRACES_MEMORY_AND_INFORMATION_FLOW.md` is the governing actor-knowledge-filtering contract.
3. Shared boundary under audit: `KnowledgeContext`'s allowed/forbidden source lists are the firewall every reader crosses — embodied view (ticket 009), why-not probe (ticket 008), and any future planner. The allowed/forbidden source enumerations are fixed here per §8.7.
4. Invariant motivating this ticket: `INV-024` (no telepathy — information reaches actors only through modeled channels) and `INV-006` (possession transfers no world knowledge); the forbidden-source list explicitly bars unobserved event-log truth, other actors' private beliefs, hidden item location, human/debug notes, and previous-possessed-actor knowledge.
5. Actor-knowledge / no-leak surface: this *is* the enforcement-substrate ticket. The `KnowledgeContext` allowed/forbidden split and the holder-keyed projection indexes guarantee a viewer can only reach its own observations/beliefs. This ticket adds no leakage path and introduces no nondeterminism — all projection collections are ordered/sorted by stable id, never `HashMap` iteration (`INV-018`). Consumers (009, 008) enforce the filter at read time; ticket 005 rebuilds the projection deterministically.

## Architecture Check

1. A first-class `KnowledgeContext` carrying explicit allowed/forbidden source sets makes the firewall inspectable and testable, where filtering inline at each view site would scatter the policy and invite leaks. An epistemic projection *beside* physical state (not belief-flags on entities) preserves single-source physical location while letting beliefs diverge per holder.
2. No backwards-compatibility shims: greenfield; physical `projections.rs`/`state.rs` are not given epistemic fields.

## Verification Layers

1. No telepathy (`INV-024`) -> unit test + manual review: a `KnowledgeContext` for actor A cannot enumerate actor B's private beliefs or unobserved truth; forbidden sources are explicit and tested.
2. Possession transfers no knowledge (`INV-006`) -> unit test: the projection is holder-keyed, so a context built for a newly-possessed actor exposes only that actor's records (full proof is ticket 016's property test).
3. Determinism (`INV-018`) -> grep-proof: projection indexes are ordered maps/sorted vecs keyed by stable id; `projection_version` and `content_manifest_id` are recorded for rebuild.

## What to Change

### 1. KnowledgeContext

Add `crates/tracewake-core/src/epistemics/knowledge_context.rs` with a `KnowledgeContext` struct (§8.7 fields: `viewer_actor_id`, `mode`, `current_tick`, `allowed_sources`, `forbidden_sources`, perception/belief/observation scopes, `projection_schema_version`) and a `ViewMode` enum (`embodied`, `debug`). Provide a constructor that builds an embodied context with the §8.7 allowed set and the forbidden set baked in, and a debug context flagged non-diegetic.

### 2. Epistemic projection store

Add `crates/tracewake-core/src/epistemics/projection.rs` with an `EpistemicProjection` struct holding the §8.8 indexes (`observations_by_id`/`_by_actor`, `beliefs_by_id`/`_by_holder`, `contradictions_by_id`/`_by_holder`, `notebook_entries_by_actor`, `projection_version`, event range/checkpoint, `content_manifest_id`) using ordered collections. Provide read accessors that take a `KnowledgeContext` and return only allowed records.

### 3. Module registration

Extend `crates/tracewake-core/src/epistemics/mod.rs` with `pub mod knowledge_context; pub mod projection;`.

## Files to Touch

- `crates/tracewake-core/src/epistemics/knowledge_context.rs` (new)
- `crates/tracewake-core/src/epistemics/projection.rs` (new)
- `crates/tracewake-core/src/epistemics/mod.rs` (modify — file created by 0004PHA2AEPISUB-001)

## Out of Scope

- Event application that *fills* the projection (ticket 005).
- Notebook entry rendering and embodied view model (ticket 009).
- Debug epistemics view (ticket 010).
- Why-not knowledge-basis query (ticket 008).

## Acceptance Criteria

### Tests That Must Pass

1. An embodied `KnowledgeContext` for actor A, queried against a projection containing B's private belief, returns no B-only record.
2. The forbidden-source set explicitly contains unobserved event-log truth, hidden item location, other actors' private beliefs, human/debug notes, and previous-possessed-actor knowledge.
3. Projection indexes iterate in deterministic stable-id order across repeated builds.

### Invariants

1. Epistemic state lives in `EpistemicProjection`, never as flags on physical entities; physical location remains single-source.
2. A reader reaches records only through a `KnowledgeContext`; the allowed/forbidden lists are the single policy site.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/epistemics/knowledge_context.rs` (unit tests) — allowed/forbidden enforcement, embodied vs debug mode.
2. `crates/tracewake-core/src/epistemics/projection.rs` (unit tests) — holder-keyed retrieval, deterministic ordering.

### Commands

1. `cargo test -p tracewake-core epistemics::knowledge_context epistemics::projection`
2. `cargo build --workspace`
3. Core-crate unit scope is correct: filtering has no cross-crate consumer until the view models (009/010) and events (005) land.
