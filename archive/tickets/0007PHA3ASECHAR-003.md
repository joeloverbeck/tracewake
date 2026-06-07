# 0007PHA3ASECHAR-003: Typed candidate generation without string prefixes

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` candidate generation (`agent/generation.rs`, `agent/candidate.rs`)
**Deps**: 0007PHA3ASECHAR-001

## Problem

Candidate goal generation decides food availability by string-prefix scanning the raw actor-known input list (Spec 0007 D-03; §Forbidden shortcuts "HTN applicability decided by substring or prefix over raw strings"). `generate_candidate_goals_from_agent_state` scans `inputs.starts_with("known_food:")` (`crates/tracewake-core/src/agent/generation.rs:179`) to decide whether a hunger-driven eat candidate is applicable. This re-encodes actor-known food as an untyped string prefix, so any caller that pushes a `"known_food:..."` token — regardless of modeled provenance — makes eat applicable.

## Assumption Reassessment (2026-06-07)

1. The prefix scan is confirmed at `crates/tracewake-core/src/agent/generation.rs:179` (`.any(|input| input.starts_with("known_food:"))`). Candidate generation entry is `generate_candidate_goals_from_agent_state(&LiveCandidateGenerationInput)` (`generation.rs:26` defines `LiveCandidateGenerationInput`). `GoalKind` is at `crates/tracewake-core/src/agent/candidate.rs:7`.
2. Spec 0007 D-03 requires replacing `starts_with("known_food:")` candidate generation with typed actor-known facts; Binding constraint 5 forbids a candidate becoming applicable "through raw caller self-attestation such as `reason_available`, `route_planner_available`, or unchecked string facts".
3. Shared boundary under audit: the candidate-generation input contract between the scheduler/decision driver (producer of `LiveCandidateGenerationInput`) and `generate_candidate_goals_from_agent_state` (consumer). Food/work/sleep applicability must read the typed actor-known facts introduced in 0007PHA3ASECHAR-001, not parse string prefixes.
4. Motivating invariants (restated): INV-024 "No telepathy" — an eat candidate is applicable only when food is actor-known through a modeled channel; INV-039 "Needs are pressures, not puppet strings" — hunger raises the candidate's salience but applicability still requires actor-known means.
5. Actor-knowledge-filtering surface touched: candidate applicability is where a need pressure becomes an actionable goal. Reading typed actor-known food facts (vs a string prefix any caller can forge) is the leakage guard — an unbacked food fact must not yield an applicable eat candidate. No determinism change: generation already iterates canonical inputs; the typed facts are read in the same deterministic order.

## Architecture Check

1. Consuming the typed actor-known facts from 0007PHA3ASECHAR-001 makes candidate applicability provable from modeled state rather than from a forgeable string prefix, and aligns candidate generation with the same actor-known contract the planner and HTN resolver use — one typed source of truth instead of two parallel string conventions.
2. No backwards-compatibility aliasing/shims: the `starts_with("known_food:")` scan is removed; no string-prefix fallback remains.

## Verification Layers

1. INV-024 (food applicability is actor-known) -> codebase grep-proof: no `starts_with("known_food:")` (or sibling prefix scan) remains in `agent/generation.rs`.
2. No-leak -> unit/negative test: a hunger pressure with no typed actor-known food fact produces no applicable eat candidate (it must instead route to a food-search/replan candidate).
3. Determinism -> manual review: candidate ordering is unchanged and stable.

## What to Change

### 1. Read typed actor-known food facts

Replace the `starts_with("known_food:")` scan in `generate_candidate_goals_from_agent_state` with a check over the typed actor-known facts from 0007PHA3ASECHAR-001. An eat candidate is applicable only when a typed actor-known food fact is present; otherwise generate a food-search / replan / wait-with-typed-reason candidate.

### 2. Extend the same typing to other prefix-coded candidate inputs

Audit `generation.rs` / `candidate.rs` for any sibling prefix-coded applicability (work, sleep, route) and route each through the typed facts rather than string parsing.

## Files to Touch

- `crates/tracewake-core/src/agent/generation.rs` (modify)
- `crates/tracewake-core/src/agent/candidate.rs` (modify)

## Out of Scope

- Constructing the actor-known facts themselves (0007PHA3ASECHAR-001, dependency) and the actor-known planning context in the proposal builders (0007PHA3ASECHAR-004).
- Live need-delta emission and threshold re-evaluation (0007PHA3ASECHAR-005).

## Acceptance Criteria

### Tests That Must Pass

1. Grep-proof: `grep -n 'starts_with("known_food' crates/tracewake-core/src/agent/generation.rs` returns nothing.
2. A negative unit test: hunger above threshold with no typed actor-known food fact yields no eat candidate but does yield a food-search/replan candidate.
3. A positive unit test: hunger plus a typed actor-known food fact yields an applicable eat candidate.
4. `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check` pass.

### Invariants

1. No string-prefix applicability scan remains in candidate generation.
2. Eat/work/sleep candidate applicability is decided from typed actor-known facts.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/agent/generation.rs` — unit tests: eat applicability with/without a typed actor-known food fact; food-search fallback path.

### Commands

1. `cargo test -p tracewake-core agent::generation`
2. `cargo test --workspace`
3. `cargo clippy --workspace --all-targets -- -D warnings`

## Outcome

Completed: 2026-06-07

What changed:

- Changed `CandidateGenerationInput` and `LiveCandidateGenerationInput` to consume `ActorKnownFact` values instead of raw actor-known input strings.
- Replaced the `starts_with("known_food:")` hunger candidate shortcut with a modeled `actor_knows_food_source` fact check.
- Kept candidate/decision trace inputs as rendered proof notes derived from typed facts.
- Updated scheduler, decision, and golden fixture call sites to pass typed facts into candidate generation.
- Added positive and negative generation tests proving modeled food facts yield `Eat`, while absent or unproven food facts yield `FindFood` and no `Eat`.

Deviations from original plan:

- `candidate.rs` did not require a structural edit; the existing `CandidateGoal.belief_inputs` remains a rendered trace/debug surface populated from typed facts.

Verification results:

- `rg -n "starts_with\\(\\\"known_food" crates/tracewake-core/src/agent/generation.rs` returned no matches.
- `cargo test -p tracewake-core agent::generation` passed.
- `cargo test --workspace` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo fmt --all --check` passed.
