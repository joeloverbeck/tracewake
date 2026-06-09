# 0015PHA3AEVECOG-004: ORD-HARD-008 regression lock — source guards, context-hash rebuild gate, negative fixtures

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new guards in `anti_regression_guards.rs`; a context-hash rebuild-from-log replay test; three negative fixtures in `tracewake-content`
**Deps**: 0015PHA3AEVECOG-003

## Problem

The ORD-HARD-008 cutover (`-003`) removed the raw-table cognition reads and the scheduler re-open, but anti-contamination doctrine requires a **structural lock** so the channel cannot be re-forged by a later edit (the spec's "Why 0014's locks did not catch this" lesson: a guard that allowlisted the offending functions let the contamination through). This ticket lands the lock immediately after the flip: source guards that fail the build if raw `PhysicalState` tables or the removed extension API reappear in the cognition path, a replay gate proving every decision's actor-known context is reconstructable from the event log, and the three negative fixtures the spec names.

## Assumption Reassessment (2026-06-09)

1. Current code: `crates/tracewake-core/tests/anti_regression_guards.rs` holds per-file `include_str!` constants (`NO_HUMAN_SURFACE_RS:7`, `SCHEDULER_RS:5`, …) and a `guard_014_*` family (`guard_014_no_human_cognition_surface_does_not_read_raw_assignment_or_sleep_truth:1303`, etc.) plus `production_sources()` (:229). After `-003` the raw reads and `extend_actor_known_facts` are gone; this ticket asserts they stay gone. The fixture registry is `crates/tracewake-content/src/fixtures/mod.rs` (`mod <name>;` per fixture).
2. Specs/docs: spec 0015 §ORD-HARD-008 "Structural lock" (source-guard, replay-gate, negative-fixtures bullets) and §5 (anti-contamination lock layer, `guard_015_*`); INV-018 (deterministic replay), INV-101 (sealed context).
3. Shared boundary under audit: the guard suite (`anti_regression_guards.rs`) and the fixture runner. This is a **shared-file merge hub** — siblings 005/006/007/008 also append guards to this file, and 009 rewrites it; this ticket appends the ORD-HARD-008 guards. The fixtures register structurally via `fixtures/mod.rs`.
4. INV-101 — the sealed context must contain no validator-only truth; the guard asserts no raw-table accessor and no post-seal extension API in the cognition path. INV-018 — replay must reconstruct the decision context from the log; the rebuild gate asserts byte-match.
5. Fail-closed / deterministic-replay surface: the rebuild gate is itself a determinism enforcement surface. Confirm it recomputes each decision's actor-known context purely from replayed events and byte-matches the recorded hash — no `PhysicalState` read in the rebuild path. The source guards are fail-closed (a banned token fails `cargo test`). No epistemic-leakage weakening: the guards only tighten.
6. Schema extension: none — this ticket adds tests/guards/fixtures, not a state or event schema. (Item omitted from the menu as not applicable; guards consume the `-001` event-kind schema and the `-003` `source_event_ids` field read-only.)

## Architecture Check

1. A glob/recursive source guard plus an event-log rebuild gate locks the channel structurally rather than trusting reviewers; the rebuild gate proves the *property* (context derivable from log), not just symbol absence, closing the gap 0014's allowlist guard left. Guards are added here (not folded into 003) so the atomic cutover stays a reviewable diff and the lock is independently reviewable.
2. No shims: guards ban re-introduction outright; no allowlist carve-out for the removed raw-table functions (that allowlist was the 0014 hole).

## Verification Layers

1. INV-101 → codebase grep-proof (guard): `no_human_surface.rs` contains no `state.workplaces()`/`state.sleep_affordances()`/`state.food_supplies()`/`PhysicalState` table accessor; `scheduler.rs` contains no `extend_actor_known_facts`.
2. INV-018 → replay/golden-fixture check: the rebuild test recomputes every decision's actor-known context from replayed events and byte-matches the recorded hash.
3. INV-102 → replay/golden-fixture check (fixture `no_human_observation_facts_cite_log_events_001`): every `ActorKnownFact` consumed by a committed decision cites `source_event_ids` present in the log.
4. INV-063/024 → replay/golden-fixture check (fixtures `no_human_workplace_knowledge_requires_notice_event_001`, `no_human_sleep_knowledge_requires_observation_or_record_001`): with the seed/perception channel disabled, no workplace/sleep fact and no resulting proposal; the stuck diagnostic names `holder_known_context`.

## What to Change

### 1. Source guards (`guard_015_*`)

Add guards to `anti_regression_guards.rs`: ban `state.workplaces()`, `state.sleep_affordances()`, `state.food_supplies()`, and any `PhysicalState` table accessor inside `no_human_surface.rs`; ban `extend_actor_known_facts` (and any post-`into_context` fact insertion) in `scheduler.rs`. Prefer the recursive `production_sources()`-style scan over the single-file `include_str!` constant where the rule is layer-scoped (009 generalizes this; here at least cover the named files).

### 2. Context-hash rebuild-from-log replay gate

Add a test that, for a canonical no-human fixture, recomputes every decision's actor-known context purely from replayed events and asserts a byte-match with the recorded context hash.

### 3. Three negative fixtures

`no_human_workplace_knowledge_requires_notice_event_001`, `no_human_sleep_knowledge_requires_observation_or_record_001`, `no_human_observation_facts_cite_log_events_001` — registered in `fixtures/mod.rs`.

## Files to Touch

- `crates/tracewake-core/tests/anti_regression_guards.rs` (modify — **shared merge hub**: also touched by 005/006/007/008, rewritten by 009)
- `crates/tracewake-core/tests/` (modify/new — context-hash rebuild test, if a separate replay test file is the right home)
- `crates/tracewake-content/src/fixtures/no_human_workplace_knowledge_requires_notice_event_001.rs` (new)
- `crates/tracewake-content/src/fixtures/no_human_sleep_knowledge_requires_observation_or_record_001.rs` (new)
- `crates/tracewake-content/src/fixtures/no_human_observation_facts_cite_log_events_001.rs` (new)
- `crates/tracewake-content/src/fixtures/mod.rs` (modify — register the three fixtures)

## Out of Scope

- The cutover itself (`0015PHA3AEVECOG-003`).
- Glob-based guard discovery + census guard (`0015PHA3AEVECOG-009` generalizes the guard mechanism).
- The audit fail-closed gate and its fixture (`0015PHA3AEVECOG-005`).

## Acceptance Criteria

### Tests That Must Pass

1. The three negative fixtures pass: disabled channel → no fact, no proposal, `holder_known_context` stuck diagnostic; and every committed `ActorKnownFact` cites log-present `source_event_ids`.
2. The context-hash rebuild-from-log test byte-matches for the canonical fixture.
3. Re-introducing a banned raw-table read or `extend_actor_known_facts` fails `cargo test` (guard demonstrated).
4. `cargo test --workspace` green.

### Invariants

1. No raw `PhysicalState` table accessor and no post-seal extension API exists in the cognition path (guard-enforced).
2. Every decision's actor-known context is reconstructable from the event log alone.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — `guard_015_*` source guards.
2. `crates/tracewake-core/tests/` — context-hash rebuild-from-log byte-match test.
3. `crates/tracewake-content/src/fixtures/no_human_{workplace_knowledge_requires_notice_event,sleep_knowledge_requires_observation_or_record,observation_facts_cite_log_events}_001.rs` — the three negatives.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards && cargo test -p tracewake-content`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
