# 0016PHA3ANEEACC-008: Embodied staleness from the epistemic projection + recorded unification deferral

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` embodied knowledge-context staleness sourced from the Phase 2A epistemic projection; new multi-tick fixture; authored deferral ticket
**Deps**: 0016PHA3ANEEACC-007

## Problem

ORD-HARD-021: Phase 3A cognition and embodied perception fork the epistemic substrate, and 0015's promised deferral was never recorded (foundation doc 14 and architecture doc 03 — the holder-known projection is the cognition source; 0015 §9's own condition: "deferring deeper unification with Phase 2A belief structures to a recorded follow-up, not silently"):

- `NoHumanActorKnownSurfaceBuilder` re-derives knowledge by parsing raw event payloads into its own tables, bypassing `EpistemicProjection`/`KnowledgeContext` — event-backed (INV-102 holds) but structurally forked.
- `agent/perception.rs::current_place_knowledge_context` recomputes embodied food/sleep/route facts from live state at view time: a food supply removed from truth this tick vanishes from the menu without the actor learning it; one added appears with no perception latency. Multi-tick staleness is untested and wrong.
- The promised recorded follow-up does not exist: `tickets/` holds only the template and README; `reports/0015_ord_life_cert_scoped_acceptance.md` does not mention the cut; no conformance-index row records the fork.

**Scope decision (decomposition Q1, approved)**: this ticket implements the spec's *minimum acceptable cut* — (a) embodied facts gain multi-tick durability/staleness from the projection, and (b) the remaining unification is recorded as a real ticket plus a conformance-index row (the recording is an acceptance criterion, not advice). Full substrate unification is the recorded follow-up — spec §9 anticipates it as spec 0017's core.

## Assumption Reassessment (2026-06-10)

1. Current code verified at baseline `ba84e75`: `consume_observation` / `consume_starting_belief` / `consume_role_assignment_notice` parse raw payloads into builder-local tables (`agent/no_human_surface.rs:132–147`); `EpistemicProjection` is defined at `crates/tracewake-core/src/epistemics/projection.rs:32` and `KnowledgeContext` at `crates/tracewake-core/src/epistemics/knowledge_context.rs:351`; `current_place_knowledge_context` lives in `agent/perception.rs:99–170` (consumed by `actions/pipeline.rs` and `actions/defs/continue_routine.rs`) and recomputes from live state at view time. `tickets/` contains only `_TEMPLATE.md` and `README.md` (plus this batch); `reports/0015_ord_life_cert_scoped_acceptance.md` contains no deferral mention (grep-verified); the conformance index (`docs/1-architecture/00_ARCHITECTURE_INDEX_AND_CONFORMANCE.md` Phase 3A table) has no cognition-substrate-source row.
2. Spec/docs: spec 0016 §ORD-HARD-021 (evidence, required correction with the minimum-cut definition, structural lock) and §9 (full unification may become spec 0017's core); `docs/0-foundation/14_ACTOR_KNOWN_COGNITION_TRANSACTION_AND_TRUTH_FIREWALL.md`; `docs/1-architecture/03_HOLDER_KNOWN_CONTEXTS_TRUTH_FIREWALL_AND_PROVENANCE.md`.
3. Shared boundary under audit: the cognition-substrate seam — Phase 3A's surface builder and embodied context builder versus the Phase 2A epistemic projection. The minimum cut moves the embodied context's *knowledge inputs* onto projection-derived observations/beliefs (with staleness); the surface builder's full re-derivation fork is the recorded follow-up.
4. INV-024 (information reaches actors only through modeled channels — perception latency is part of the channel) and foundation 14's holder-known-projection-as-cognition-source doctrine. Restated before trusting the ticket narrative.
5. Actor-knowledge / staleness enforcement surface: after this ticket, embodied food/sleep/route facts have multi-tick durability — a truth change without a perception event must not alter the embodied menu (the new fixture locks both directions: no vanishing, no premature appearing). Behavior feeding decisions through `actions/pipeline.rs` / `continue_routine.rs` may change in stale-truth scenarios, so golden checksums may reprice — each diff explained; ticket 003's re-derivation gate stays green (production-builder based).
6. Adjacent contradictions: the unrecorded-deferral debt is resolved by this ticket *authoring* the follow-up ticket (`tickets/0016PHA3ANEEACC-015.md`, full unification of the surface builder onto the epistemic projection — ID confirmed against `tickets/` at implementation time) and by ticket 0016PHA3ANEEACC-013 adding the conformance-index row naming the current substrate source and its recorded deferral. Required consequences, split by surface ownership.

## Architecture Check

1. Sourcing embodied knowledge from the durable epistemic projection is the doctrine-mandated direction (the holder-known projection is *the* cognition source); the minimum cut lands the user-visible correctness half (staleness) now without forcing the surface-builder rework into the same diff, and converts the silent fork into a recorded, indexed deferral — the precise failure 0015 was cited for. The alternative (full unification now) is the recorded follow-up because it reshapes every consume-path in the builder and belongs in its own reviewable spec-sized change.
2. No backwards-compatibility aliasing/shims: the live-state recomputation in `current_place_knowledge_context` is replaced by projection-derived facts — not kept as a fallback when the projection lacks an entry (absence of knowledge is the correct outcome).

## Verification Layers

1. Staleness (INV-024 channel latency) → multi-tick fixture `embodied_menu_lags_truth_change_without_perception_001`: truth changes; no perception event; embodied output must not change until perception fires.
2. Recorded deferral (0015 §9 condition; spec acceptance criterion) → ticket-existence check: `tickets/0016PHA3ANEEACC-015.md` exists with the unification scope; verified again by ticket 014's acceptance artifact.
3. Substrate-source documentation → conformance-index row (owned by 0016PHA3ANEEACC-013; this ticket's content feeds it).
4. INV-018 (replay) → replay/golden-fixture check: any repriced goldens replay byte-identically with explained diffs.

## What to Change

### 1. Projection-backed embodied knowledge context

`current_place_knowledge_context` consumes the Phase 2A epistemic projection (`EpistemicProjection`/`KnowledgeContext` observations/beliefs with staleness) instead of recomputing food/sleep/route facts from live state at view time. Embodied facts gain multi-tick durability.

### 2. Multi-tick fixture

`embodied_menu_lags_truth_change_without_perception_001`: a food supply is removed (and another added) in truth with no perception event; the embodied menu is unchanged until a perception event fires, then updates.

### 3. Authored deferral ticket

Write `tickets/0016PHA3ANEEACC-015.md` (ID confirmed free at implementation) scoping the remaining unification: `NoHumanActorKnownSurfaceBuilder` consuming the epistemic projection instead of its parallel payload-parsing tables. This file's existence is an acceptance criterion of this ticket.

## Files to Touch

- `crates/tracewake-core/src/agent/perception.rs` (modify)
- `crates/tracewake-core/src/agent/no_human_surface.rs` (modify — only where the embodied-context cut touches shared derivations; the builder fork itself is deferred)
- `crates/tracewake-core/src/epistemics/projection.rs` (modify — accessor surface for staleness-bearing reads, as surfaced)
- `crates/tracewake-content/src/fixtures/embodied_menu_lags_truth_change_without_perception_001.rs` (new)
- `crates/tracewake-content/src/fixtures/mod.rs` (modify)
- `crates/tracewake-content/tests/golden_fixtures_run.rs` (modify — run the new fixture)
- `tickets/0016PHA3ANEEACC-015.md` (new — the recorded unification deferral, authored from `tickets/_TEMPLATE.md`)

## Out of Scope

- Full unification of `NoHumanActorKnownSurfaceBuilder` onto the epistemic projection (the recorded follow-up ticket this ticket authors; candidate core of spec 0017).
- The conformance-index row itself (0016PHA3ANEEACC-013 — cites this ticket's landed state).
- Workplace availability/believed-access (0016PHA3ANEEACC-007 — upstream of this ticket).

## Acceptance Criteria

### Tests That Must Pass

1. `embodied_menu_lags_truth_change_without_perception_001`: embodied output is unchanged across the truth change until a perception event fires; replay byte-match.
2. `tickets/0016PHA3ANEEACC-015.md` exists, authored from the template, scoping the surface-builder unification (the recording acceptance criterion).
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`.

### Invariants

1. Embodied knowledge has multi-tick durability sourced from the durable epistemic projection — no view-time recomputation from live truth.
2. The remaining substrate fork is recorded (ticket + conformance row), never silent.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/src/fixtures/embodied_menu_lags_truth_change_without_perception_001.rs` — the ORD-HARD-021 structural-lock fixture.
2. `crates/tracewake-core/src/agent/perception.rs` — unit tests for projection-backed staleness (entry persists without perception; updates on perception).

### Commands

1. `cargo test -p tracewake-core perception && cargo test -p tracewake-content golden`
2. `test -f tickets/0016PHA3ANEEACC-015.md`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
