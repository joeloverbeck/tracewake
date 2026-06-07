# 0006PHA3ANEEROU-003: Typed HTN method conditions with actor-known resolver

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tracewake-core` HTN applicability (`agent/htn.rs`, `agent/methods.rs`); routine-template condition representation (content schema consumed by `tracewake-content`)
**Deps**: 0006PHA3ANEEROU-002

## Problem

HTN method applicability auto-passes string prefixes/substrings (audit **D-03 / F-03**). `template_applicable` (`crates/tracewake-core/src/agent/htn.rs:138`) accepts a condition if it `starts_with("actor_has_")`, equals `active_intention`/`reason_available` (`htn.rs:140-142`), or appears as a substring of any supplied actor-known string. The method library uses bare strings like `actor_has_food_search_knowledge` (`agent/methods.rs:78`) and `reason_available` (`methods.rs:101`). A content author or scaffold can type `actor_has_food_search_knowledge` and applicability becomes true with no belief/memory/observation/routine-assignment/role/record backing it. Spec §5.6 requires typed condition values resolved by a resolver that returns `satisfied`/`rejected`/`unknown`, each satisfied condition carrying a proof source, with rejected conditions recorded in the trace and deterministic selection with stable tie-breaks.

## Assumption Reassessment (2026-06-07)

1. `template_applicable` (`crates/tracewake-core/src/agent/htn.rs:138`) and its `condition.starts_with(...)` checks (`htn.rs:140-142`) are confirmed; the `fail:`-prefix precondition handling (`htn.rs:65`) and the string condition library in `agent/methods.rs:78/101` are confirmed. Selection entry is `htn.rs:55` (`.find(|template| template_applicable(...))`).
2. Spec §5.6 requires typed conditions + a resolver with `satisfied`/`rejected`/`unknown` and proof source; §9 forbids auto-passing `actor_has_*`, `reason_available`, `active_intention` strings. Spec §7.9 (planner trace) requires at least one rejected method condition recorded.
3. Shared boundary under audit: the routine-template condition contract between content (routine templates authored in `tracewake-content`) and the engine resolver. Conditions change representation string → typed value; content fixtures and content validation (0006PHA3ANEEROU-009) are the consumers.
4. Motivating invariants (restated): INV-032 "V1 agents are symbolic and inspectable", INV-036 "HTN methods are procedures, not story scripts" — method applicability must be explicit, state-conditioned, and inspectable, never a convenient string that always passes.
5. Actor-knowledge-filtering surface touched: each condition resolves against the actor-known planning state from 0006PHA3ANEEROU-002. The resolver must return `unknown/not-actor-known` (not silently `satisfied`) when no modeled source backs a condition, so no hidden fact leaks in as a passing string. Determinism: method selection stays deterministic with stable tie-breaks (no hash-map iteration order dependence).
6. Schema change: routine-template method conditions move from `String` to a typed condition value (enum/struct). Consumers: content routine templates / fixtures (`crates/tracewake-content/src/fixtures/*`), content validation (0006PHA3ANEEROU-009), and the trace projection. This is a **breaking** representation change (no string conditions remain) — intentional per §9; there is no back-compat string path.

## Architecture Check

1. Typed conditions resolved by a dedicated resolver replace "string that matches a prefix" with "predicate that must be proven from actor-known state," which is both safer (no accidental auto-pass) and inspectable (rejected conditions and their reasons appear in the trace). A typed enum also lets content validation (0006PHA3ANEEROU-009) reject unknown/behavior-looking conditions at load time.
2. No backwards-compatibility aliasing/shims: the `starts_with`/substring path is removed entirely; no string-condition fallback is retained.

## Verification Layers

1. INV-036 / INV-032 (explicit state-conditioned methods) -> codebase grep-proof: no `starts_with("actor_has_")` / substring-match applicability remains in `agent/htn.rs`; conditions are typed values resolved by the resolver.
2. No-leak resolution -> replay/golden-fixture check: a spoofed `actor_has_*` / `reason_available` condition with no modeled source resolves to `rejected`/`unknown` and is recorded in the trace (negative test).
3. Determinism -> manual review + test: method selection over equal-scoring candidates is stable across runs (stable tie-break).

## What to Change

### 1. Typed condition values

Replace string method conditions with a typed condition representation in `agent/methods.rs` / routine templates. Each variant names what must be proven (actor has belief X, routine assignment Y, visible affordance Z, current intention, etc.).

### 2. Actor-known condition resolver

Add a resolver that, given the actor-known planning state (0006PHA3ANEEROU-002), returns `satisfied{proof_source}` / `rejected{reason}` / `unknown` per condition. Proof sources: live `AgentState`, actor-known belief, observation/memory, routine assignment, current intention, visible local affordance, or an explicitly validation-allowed fixture seed.

### 3. Rewrite `template_applicable` + record rejections

`template_applicable` resolves every condition through the resolver; a template is applicable only if all conditions are `satisfied`. Rejected/unknown conditions and their reasons are recorded for the decision trace. Selection is deterministic with stable tie-breaks.

## Files to Touch

- `crates/tracewake-core/src/agent/htn.rs` (modify)
- `crates/tracewake-core/src/agent/methods.rs` (modify)
- `crates/tracewake-core/src/agent/trace.rs` (modify)
- `crates/tracewake-content/src/fixtures/planner_trace_001.rs` (modify)

## Out of Scope

- Content-side validation of typed conditions (0006PHA3ANEEROU-009) — this ticket defines the typed representation; the validator enforcing it lands there.
- Building the actor-known planning state (0006PHA3ANEEROU-002, dependency).
- The decision loop that selects methods per window (0006PHA3ANEEROU-007).

## Acceptance Criteria

### Tests That Must Pass

1. A spoof test: a method whose condition is a bare `actor_has_food_search_knowledge` string-equivalent with no modeled source is rejected (not applicable), and the rejection reason is recorded.
2. `planner_trace_001` includes at least one rejected method condition with reason and a selected method with proof sources.
3. `cargo test --workspace` and `cargo clippy --workspace --all-targets -- -D warnings` pass.

### Invariants

1. No string-prefix/substring applicability shortcut remains in `agent/htn.rs`.
2. Every satisfied condition carries a proof source; every rejected condition carries a reason in the trace.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/agent/htn.rs` — unit tests: typed condition resolution (satisfied/rejected/unknown), spoof rejection, stable tie-break.
2. `crates/tracewake-content/src/fixtures/planner_trace_001.rs` — assert rejected-condition + proof-source content from real resolution.

### Commands

1. `cargo test -p tracewake-core agent::htn`
2. `cargo test --workspace`
3. `cargo clippy --workspace --all-targets -- -D warnings`

## Outcome

Completed: 2026-06-07

Implemented typed `RoutineCondition` values across core routine templates and
content fixture schema. HTN method selection now resolves applicability and
preconditions through an actor-known condition resolver that returns satisfied,
rejected, or unknown outcomes; selected conditions carry proof sources and
rejected/unknown conditions are recorded in `DecisionTrace.rejected_methods`.

Removed string-prefix/substr applicability shortcuts from `agent/htn.rs`,
updated scheduler method selection to pass the actor-known planning state, and
added tests for spoof rejection, rejected-condition trace recording, stable
tie-breaks, fixture trace proof sources, and typed content serialization.

Verified with:

1. `cargo test -p tracewake-core agent::htn`
2. `cargo test --workspace`
3. `cargo clippy --workspace --all-targets -- -D warnings`
4. `cargo fmt --all --check`
