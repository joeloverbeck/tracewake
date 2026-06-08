# 0008PHA3AANTCON-007: Static / anti-regression + mutation guards for forbidden shortcuts

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tracewake-core` test crate: static/anti-regression guard tests + mutation-style proofs that the suite fails when a bypass is reintroduced
**Deps**: 0008PHA3AANTCON-006

## Problem

Spec 0008 Finding 10 (D-0008-10) §10.7: after the atomic flip (-006), nothing prevents a future contributor from quietly reintroducing a bypass (the §15 contamination-spread risk). The spec enumerates the exact forbidden shortcuts and requires tests/lints that fail on each, plus mutation tests that deliberately reintroduce a shortcut and confirm the suite fails. This ticket lands the lock that keeps the flip from eroding.

## Assumption Reassessment (2026-06-08)

1. Post-flip the forbidden constructs are gone (verified by -006's deletion proofs). This ticket asserts their continued absence: `build_routine_or_need_proposal`, `eat_proposal`/`sleep_proposal`/`work_or_move_proposal`, scheduler `EpistemicProjection::new` in the cognition path, `RoutineFamily`→primitive dispatch, `work`/`eat`/`sleep` validators reading need values from `Proposal.parameters`, `AgentState` storing `decision_traces`/`stuck_diagnostics` as `String`, hard-coded `actor_known_only=true`, public arbitrary construction of the actor-known context, hidden-truth audit trusting tags, `ContinueRoutineProposed` contributing to progress alone.
2. Spec §10.7 lists exactly these nine guard targets and mandates mutation tests. §14.2 forbids "more tests around the current bypassing shape" — these guards assert the shape is absent, the opposite of shape-matching.
3. Cross-artifact boundary under audit: the **anti-regression guard ↔ {scheduler (-006), validators (-003), state (-002), actor-known context (-001)}** seam — each guard names one prior ticket's invariant and proves it cannot regress.
4. INV-097 (No-script compliance is tested): systems must be inspected for hidden outcome chains / shortcut causation; these guards are the automated form of that inspection for the no-human cognition path.

## Architecture Check

1. Grep-proof + mutation tests are the correct lock: a structural absence assertion (no symbol, no dispatch arm, no string map) cannot be satisfied by adding more behavior tests around a bypass. Mutation tests prove the guards have teeth (reintroducing a shortcut turns the suite red).
2. No backwards-compatibility aliasing: guards assert removal, introducing no production code and no shim.

## Verification Layers

1. INV-097 no-script compliance → codebase grep-proof: a test asserts each forbidden symbol/pattern returns zero matches in the cognition path (source-level guard).
2. Guard efficacy → mutation test: a deliberately reintroduced bypass (e.g. a routine-family dispatch arm) makes the guard suite fail (documented mutation, reverted).
3. Distinct-invariant mapping → manual review: each of the nine §10.7 targets has its own named guard, not one collapsed "no shortcuts" test.

## What to Change

### 1. Static absence guards

New test module asserting each §10.7 forbidden construct is absent (source-scan or symbol-resolution guards), one named test per target, each citing the ticket whose invariant it protects.

### 2. Mutation-style efficacy proofs

For the highest-risk targets (routine-family dispatch, proposal-param need reads, string diagnostic maps), document a mutation that reintroduces the shortcut and assert the guard/behavior suite fails — kept as `#[ignore]` reproduction notes or a scripted mutation check per repo convention.

## Files to Touch

- `crates/tracewake-core/tests/anti_regression_guards.rs` (new)

## Out of Scope

- The behavioral gates themselves (owned by -003/-005/-006).
- Content anti-script validation (0008PHA3AANTCON-009).
- Hidden-truth fixtures (0008PHA3AANTCON-008).

## Acceptance Criteria

### Tests That Must Pass

1. Each of the nine §10.7 forbidden constructs has a passing absence guard.
2. A documented reintroduction of any guarded shortcut fails the suite (mutation efficacy).
3. `cargo test -p tracewake-core --test anti_regression_guards` green.

### Invariants

1. Every forbidden shortcut from §10.7 is covered by exactly one named guard.
2. Guards assert absence/structure, never re-test the bypassing shape.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/tests/anti_regression_guards.rs` — nine absence guards + mutation efficacy notes.

### Commands

1. `cargo test -p tracewake-core --test anti_regression_guards`
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace --all-targets --locked && cargo test --workspace`
