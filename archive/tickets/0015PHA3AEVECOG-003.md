# 0015PHA3AEVECOG-003: Atomic cutover — actor-known surface consumes the epistemic substrate; seal closed

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` actor-known surface builder rebuilt to consume the epistemic projection + event log; `extend_actor_known_facts` removed from the scheduler path and the post-seal extension API closed
**Deps**: 0015PHA3AEVECOG-001, 0015PHA3AEVECOG-002

## Problem

This is the ORD-HARD-008 channel cutover (spec required-correction items 3–4). With seed knowledge (`-001`) and current-place perception (`-002`) now flowing as events, the no-human actor-known surface must stop reverse-engineering knowledge from raw `PhysicalState` tables and instead **consume the event-sourced epistemic substrate**, with every `ActorKnownFact` carrying `source_event_ids` that exist in the log. The scheduler's `extend_actor_known_facts(transaction_facts)` call (`scheduler.rs:599`) that re-opens the sealed context and mints a `food_source_believed_accessible` `observed_now` fact must be removed — the scheduler is manufacturing cognition input (INV-103) and the seal is decorative at that seam (INV-101).

Per the spec's anti-contamination thesis ("the channel fix, not another call-site fix"; "Close the seal … no post-seal extension API at all"), this is an **atomic cutover**, not an incremental one: a split that consumed events for some facts while leaving the raw-table reads or the scheduler re-open live would leave a contamination path open and violate the spec's integrity. The new path was built unwired in 001/002; this ticket flips to it and deletes the bypasses in one reviewable diff. The regression lock (source guards, replay gate, negative fixtures) lands immediately after in `0015PHA3AEVECOG-004`.

## Assumption Reassessment (2026-06-09)

1. Current code (all verified): `no_human_surface.rs` reads `state.workplaces()` / `state.sleep_affordances()` / `state.food_supplies()` in its `observe_*` methods and seals via `context.extend_actor_known_facts(self.facts)` at `no_human_surface.rs:127`; `into_context` (`no_human_surface.rs:26`) returns `ActorKnownPlanningContext`. The scheduler re-opens the sealed context at `scheduler.rs:599` (`actor_known_state.extend_actor_known_facts(transaction_facts)`) and mints `food_source_believed_accessible` (`scheduler.rs:593`). `extend_actor_known_facts` is defined at `agent/actor_known.rs:315`. The epistemic substrate (`KnowledgeContext`, `ForbiddenKnowledgeSource`) lives in `crates/tracewake-core/src/epistemics/knowledge_context.rs:237`/`:48`.
2. Specs/docs: spec 0015 §ORD-HARD-008 "Required correction" items 3–4 and "Structural lock" (source-guard + sealed-types bullets); `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md` ("raw route/workplace/food/sleep tables used directly for cognition" is a forbidden source); `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md` (the decision-transaction contract).
3. Information-path refactor under audit (per `tickets/README.md` pre-check 8): workplace/sleep/food knowledge currently has two lawful transport paths into cognition — the raw-table read and (after 001/002) the event channel. The canonical end-state path is the event channel only; this ticket removes the raw-table path so a single lawful path remains. The proof surface that must stay strong: every `ActorKnownFact`'s `source_event_ids` must resolve in the log (verified by 004's rebuild gate).
4. INV-101 — action proposal generation must consume a *sealed* actor-known context that cannot contain validator-only truth; the post-`into_context` extension API makes the seal decorative and must be removed. INV-103 — the scheduler must not construct proposals/facts from raw state, true item/workplace locations, or true route availability. INV-099 — truth may validate, not plan.
5. Fail-closed / actor-knowledge / deterministic-replay surface (this is the enforcement cutover): names the enforcement surface = the actor-known surface builder + the scheduler seam. Confirm the change does not weaken epistemic-leakage prevention — it strengthens it (raw-table cognition reads removed; context immutable at seal). Confirm it does not break deterministic replay — every consumed fact is event-derived, so the context is reconstructable from the log (the property 004 asserts). No `PhysicalState` table accessor remains reachable from the builder.
6. Schema/shape change: `ActorKnownFact` gains `source_event_ids` (additive field referencing log events); the surface builder's constructor changes to accept the epistemic projection + event log instead of `PhysicalState` (make the wrong input uncompilable, mirroring `ActorKnownPlanningContext`). Consumers: the decision transaction and trace (which already read the context), the scheduler (loses its re-open path). Additive on the fact record; **breaking** on the builder constructor signature — all in-workspace consumers change in this same diff (local compile-atomicity).
7. Removal blast radius: `extend_actor_known_facts` (3 sites — def `actor_known.rs:315`, builder use `no_human_surface.rs:127`, scheduler use `scheduler.rs:599`). The scheduler use is deleted; the builder's own seal use is replaced by a sealing constructor / `into_context` that consumes facts at seal time; the public method is removed so no post-seal extension API survives. Raw accessors `state.workplaces()`/`state.sleep_affordances()`/`state.food_supplies()` reads are removed from `no_human_surface.rs` (the accessors themselves remain on `state` for other lawful callers).

## Architecture Check

1. A sealed builder that consumes only the event-sourced epistemic projection makes hidden-truth cognition structurally impossible at this seam, where the current label-on-raw-read approach only *names* a channel. Making the wrong constructor input uncompilable is stronger than a runtime guard. Atomic flip (vs. incremental) is mandatory here: any retained raw-table or re-open path would be a live contamination bypass the spec forbids — this overrides the default split-when-large rule (atomic-cutover exception).
2. No backwards-compatibility shims: the raw-table reads and the scheduler re-open are deleted, not wrapped behind a fallback or feature flag; `extend_actor_known_facts` is removed, not deprecated.

## Verification Layers

1. INV-101 → codebase grep-proof: no post-`into_context` fact-insertion API exists; `extend_actor_known_facts` is gone (the 004 guard enforces this durably).
2. INV-103 → codebase grep-proof: `scheduler.rs` no longer calls `extend_actor_known_facts` and no longer mints `food_source_believed_accessible`.
3. INV-099/100 → manual review + replay: the builder's inputs are the epistemic projection and event log only; no `PhysicalState` table accessor is reachable from it.
4. INV-018 → replay/golden-fixture check: existing no-human fixtures still pass with knowledge now sourced from events; every `ActorKnownFact` carries `source_event_ids` present in the log (the byte-match is locked in 004).

## What to Change

### 1. Rebuild the surface builder on the epistemic substrate

Change the builder constructor/`into_context` to accept the epistemic projection (`KnowledgeContext`-derived) + event log + `AgentState`, not `PhysicalState`. Replace each `observe_*` raw-table method with consumption of the seed/observation events from 001/002. Every emitted `ActorKnownFact` carries `source_event_ids`.

### 2. Close the seal

Remove the public `extend_actor_known_facts` from `actor_known.rs`; the context becomes immutable at seal time (facts supplied at construction / `into_context`).

### 3. Remove the scheduler re-open

Delete the `extend_actor_known_facts(transaction_facts)` call and the `food_source_believed_accessible` minting in `scheduler.rs`. Framing facts the transaction genuinely needs (wait reason, reevaluation window, active-intention summary) are built inside the sealed builder with typed provenance classes, or passed as explicitly non-cognitive transaction parameters — never as minted `observed_now` facts.

## Files to Touch

- `crates/tracewake-core/src/agent/no_human_surface.rs` (modify — remove raw reads; rebuild on substrate)
- `crates/tracewake-core/src/agent/actor_known.rs` (modify — remove `extend_actor_known_facts`; add `source_event_ids`; sealing constructor)
- `crates/tracewake-core/src/scheduler.rs` (modify — delete re-open + minting; route framing facts non-cognitively)
- `crates/tracewake-core/src/epistemics/knowledge_context.rs` (modify — projection access the builder consumes, if needed)
- Golden fixtures under `crates/tracewake-content/src/fixtures/` (modify — checksum updates, explained)

## Out of Scope

- Source guards, context-hash rebuild gate, and the three negative fixtures (`0015PHA3AEVECOG-004` — lands immediately after).
- Embodied food/sleep/route projection (`0015PHA3AEVECOG-006`).
- The hidden-truth audit fail-closed gate (`0015PHA3AEVECOG-005`).
- **Recorded minimum-cut (spec §9):** if full unification with Phase 2A belief structures proves too large, the acceptable cut is evented seed + evented perception + builder-consumes-only-those-events, deferring deeper belief-structure unification to a recorded follow-up — never a silent partial. Invoking the cut requires an explicit note here and a follow-up ticket, not a quiet scope trim.

## Acceptance Criteria

### Tests That Must Pass

1. No-human fixtures advance with knowledge sourced from seed/observation events; every committed decision's `ActorKnownFact`s carry `source_event_ids` present in the log.
2. `scheduler.rs` contains no `extend_actor_known_facts` call and mints no `food_source_believed_accessible` fact.
3. `extend_actor_known_facts` no longer exists as a public extension API.
4. `cargo test --workspace` green; golden-fixture checksum diffs explained.

### Invariants

1. The actor-known surface builder takes no `PhysicalState` table input; the wrong input does not compile.
2. The sealed context is immutable after `into_context`; no post-seal fact insertion exists anywhere.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/src/fixtures/` — update existing no-human fixtures to the evented path; assert `source_event_ids` resolve.
2. `crates/tracewake-core/src/agent/no_human_surface.rs` — unit test that the builder consumes events and rejects (uncompilable) a `PhysicalState` input.

### Commands

1. `cargo build -p tracewake-core --locked && cargo test -p tracewake-core agent:: scheduler::`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`

## Outcome (2026-06-09)

Implemented the atomic cutover. `NoHumanActorKnownSurfaceBuilder` now builds the no-human cognition surface from the event log plus agent state, consuming role-assignment notices, starting beliefs, and current-place observation events instead of `PhysicalState` table reads. `ActorKnownFact` now carries `source_event_ids`, decision trace input refs serialize those event ids, and `ActorKnownPlanningContext` no longer exposes the public post-seal `add_actor_known_fact` / `extend_actor_known_facts` mutation API.

The scheduler now passes the live event log into the sealed surface builder and no longer appends transaction facts after `into_context`. The scheduler path no longer mints `food_source_believed_accessible`; food accessibility facts now come from event-derived actor-known observations inside the surface. Loader-backed no-human fixture runs and the TUI now start from `loaded.seed_event_log`, so authored prehistory seed events participate in no-human cognition. Hand-built capstone tests seed equivalent role-notice events explicitly.

Acceptance greps:

- `rg -n "extend_actor_known_facts|add_actor_known_fact|from_modeled_observations" crates` returned no matches.
- `rg -n "food_source_believed_accessible|extend_actor_known_facts" crates/tracewake-core/src/scheduler.rs` returned no matches.
- `rg -n "PhysicalState|state\\.workplaces|state\\.food_supplies|state\\.sleep_affordances|food_supplies\\(\\)|workplaces\\(\\)|sleep_affordances\\(\\)" crates/tracewake-core/src/agent/no_human_surface.rs` returned only test assertions against `known_workplaces`, not raw `PhysicalState` inputs or raw table reads.

Verification:

- `cargo test -p tracewake-core agent::`
- `cargo test -p tracewake-core scheduler::`
- `cargo test -p tracewake-core --test acceptance_gates integrated_no_human_day_capstone_emerges_from_one_autonomous_run`
- `cargo test -p tracewake-core --test no_human_capstone`
- `cargo test -p tracewake-tui --test command_loop_session`
- `cargo test -p tracewake-tui --test embodied_flow`
- `cargo test -p tracewake-tui --test tui_acceptance tui_runs_no_human_day_and_inspects_real_post_run_panels`
- `cargo test --workspace`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace --all-targets --locked`

Deviations / follow-up: this uses the spec's recorded minimum cut: evented seed knowledge plus evented perception plus a builder that consumes only those events. Deeper Phase 2A belief-structure unification remains deferred to the follow-up tickets called out by the spec.
