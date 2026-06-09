# 0014PHA3AORDLIF-008: Embodied projection — context-backed workplace facts (no raw table)

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` (`projections.rs` embodied workplace path, sealed-context packet), new source guard, 1 adversarial view-model fixture
**Deps**: 0014PHA3AORDLIF-001

## Problem

`EmbodiedProjectionSource::from_sealed_context` (`crates/tracewake-core/src/projections.rs:42`) calls `actor_known_workplaces_for_context(state, viewer_actor_id)` (`projections.rs:50`). That helper reads `state.workplaces` and returns every workplace whose `assigned_actor_ids` is empty or contains the viewer (`projections.rs:100-116`) — leaking all unassigned workplaces to any viewer plus the viewer's own assignments, with no actor-known-provenance check. Embodied views are holder-known surfaces; they must not infer affordances from raw workplace assignment truth (INV-067/024). A human-bound actor should not see or select a work affordance solely because the state table assigns it. This is ORD-HARD-006.

## Assumption Reassessment (2026-06-09)

1. `EmbodiedProjectionSource::from_sealed_context` is at `crates/tracewake-core/src/projections.rs:42`, taking `&KnowledgeContext` + `&PhysicalState`; the raw call is at `projections.rs:50`; `actor_known_workplaces_for_context` (reads `state.workplaces`, filters `assigned_actor_ids.is_empty() || contains(viewer)`) is at `projections.rs:100-116`. `KnowledgeContext` is at `crates/tracewake-core/src/epistemics/knowledge_context.rs`. Debug panels are at `crates/tracewake-tui/src/debug_panels.rs`.
2. Spec §ORD-HARD-006 and §5.1 (guard 5) require: move embodied workplace affordances into the sealed context packet; projection renders only context-backed workplace facts + actor-visible action definitions; debug projection may compare context-vs-truth but stays structurally separate and non-diegetic; a source guard bans `state.workplaces` in embodied actor-known projection helpers.
3. Shared boundary under audit: the embodied projection ↔ the sealed `KnowledgeContext` packet ↔ the (separate) debug projection. The contract: embodied output is a pure function of the sealed context + action defs; raw `state.workplaces` is reachable only by the non-diegetic debug comparison path. The sealed-surface type and the "context-backed affordance builder" pattern are shared with ticket -001's `SealedActorKnownSurface`.
4. Invariants motivating this ticket: **INV-067** (embodied mode shows actor-known reality, hides truth), **INV-024** (no telepathy), **INV-006** (possession transfers no world knowledge), **INV-068** (debug is visibly non-diegetic), and the truth-firewall set **INV-100/101/103**.
5. No-leak / actor-knowledge-firewall enforcement surface: the embodied projection must not read raw `state.workplaces`; it renders only workplace facts present in the sealed `KnowledgeContext`. This closes a hidden-information path to a viewer the actor-knowledge filter forbids. Determinism is preserved (projection is a pure function of context + sorted action defs). Debug omniscience stays quarantined in a separate, non-diegetic comparison helper (INV-068/107).
6. Schema extension — additive-vs-breaking: workplace affordance facts must be carried in the sealed context packet rather than recomputed from raw state. Consumers of the context-backed workplace surface: `projections.rs` (embodied source), the debug comparison path, and the TUI view models / `debug_panels.rs`. Additive to the context packet; the embodied source's raw-read path is removed.
7. Removal blast radius: `actor_known_workplaces_for_context`'s raw `state.workplaces` read is removed from the embodied path; grep confirms `projections.rs` is its only caller. The guard bans `state.workplaces` in embodied actor-known projection helpers (debug comparison helper excepted by allowlist).

## Architecture Check

1. Rendering embodied workplace affordances purely from the sealed context packet makes leakage structurally impossible in the embodied path — the projection cannot reach raw assignment truth — while preserving a separate, clearly non-diegetic debug comparison. This matches the holder-known projection doctrine rather than patching the filter after the fact.
2. No backwards-compatibility shim: the raw `actor_known_workplaces_for_context` embodied read is removed, not gated; debug comparison is a distinct helper, not a reused embodied path.

## Verification Layers

1. INV-067/024 (embodied shows only actor-known) -> codebase grep-proof: source guard bans `state.workplaces` inside embodied actor-known projection helpers (debug comparison allowlisted).
2. INV-006/100 (no leak via possession/raw truth) -> replay/golden-fixture check: `embodied_view_omits_raw_assignment_without_context_001` — raw assignment exists but no actor-known workplace fact is in the context; embodied output omits the work affordance while debug output may show the discrepancy.
3. INV-068 (debug non-diegetic) -> manual review: the context-vs-truth comparison lives in a separate debug helper that cannot feed the embodied view model.

## What to Change

### 1. Context-backed embodied workplace facts

In `crates/tracewake-core/src/projections.rs`, replace the `actor_known_workplaces_for_context(state, …)` raw read in `EmbodiedProjectionSource::from_sealed_context` with workplace facts drawn from the sealed `KnowledgeContext` packet; render only context-backed facts + actor-visible action definitions.

### 2. Quarantined debug comparison

Keep any context-vs-truth comparison in a separate, non-diegetic debug helper that cannot be consumed by the embodied projection.

### 3. Source guard + fixture

Add the `state.workplaces`-in-embodied ban guard and the adversarial view-model fixture.

## Files to Touch

- `crates/tracewake-core/src/projections.rs` (modify — embodied workplace path; **also touched by 0014PHA3AORDLIF-009**)
- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — guard #5; **N-way shared hub**)
- `crates/tracewake-content/src/fixtures/embodied_view_omits_raw_assignment_without_context_001.rs` (new)
- `crates/tracewake-content/src/fixtures/mod.rs` (modify — register fixture; **shared hub**)

## Out of Scope

- The no-human cognition surface builder (ticket 0014PHA3AORDLIF-001 — this ticket reuses the shared sealed-surface pattern).
- No-human metrics (ticket 0014PHA3AORDLIF-009).
- Eat/food and sleep embodied affordances — workplace only this round.

## Acceptance Criteria

### Tests That Must Pass

1. `embodied_view_omits_raw_assignment_without_context_001` — raw assignment present, no context-backed workplace fact; embodied output omits the work affordance; debug output may show the discrepancy non-diegetically.
2. `cargo test -p tracewake-core --test anti_regression_guards` — `state.workplaces`-in-embodied ban guard passes.
3. `cargo test --workspace` — possession-parity and view-filtering tests still pass; no embodied leak.

### Invariants

1. The embodied projection renders only context-backed workplace facts; it reads no raw `state.workplaces` (INV-067/024/006).
2. Debug comparison is structurally separate and non-diegetic (INV-068/107).

## Test Plan

### New/Modified Tests

1. `crates/tracewake-content/src/fixtures/embodied_view_omits_raw_assignment_without_context_001.rs` — adversarial embodied-vs-debug workplace fixture.
2. `crates/tracewake-core/tests/anti_regression_guards.rs` — guard banning `state.workplaces` in embodied actor-known projection helpers.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo test -p tracewake-content`
3. `cargo test --workspace`
