# 0004PHA2AEPISUB-007: Expectation contradiction and absence-as-evidence

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — adds contradiction detection and missing-property belief derivation to the `tracewake-core` epistemic path.
**Deps**: 0004PHA2AEPISUB-006

## Problem

The central Phase 2A proof is that a checked-but-absent expected item becomes a source-backed missing-property belief through contradiction (Spec 0004 §8.6, §10.4; `INV-016`). When an actor with a relevant expectation checks a container and the expected item is absent, the system must create a contents observation, an `ExpectationContradicted` link, and a `BeliefUpdated` for `ItemMissingFromExpectedLocation` — and must do none of this for an actor with no expectation, or whose expectation names a different location (§10.5).

## Assumption Reassessment (2026-06-06)

1. No contradiction logic exists: the `Contradiction` record (ticket 002) and `ExpectationContradicted`/`BeliefUpdated` events (ticket 004) are defined but nothing detects or emits them; `check_container` (ticket 006) records only the observation.
2. Detection rules and the negative cases are fixed by Spec 0004 §8.6/§10.4/§10.5; the required Tomas expectation and contradiction-result shapes are §8.5. The expectation belief is supplied as content seed (ticket 012/013), not hard-coded in tests (§18 forbidden shortcut).
3. Shared boundary under audit: contradiction detection consumes the holder's expectation beliefs from the projection (ticket 003) and the fresh observation from ticket 006, and emits events applied by ticket 005 — the detection sits in the epistemic action path, not in physical apply.
4. Invariant motivating this ticket: `INV-016` (absence becomes evidence only through expectation + intentional search), `INV-002` (belief before truth — the missing-property belief is Tomas's, derived from his own check, not from ground truth), and `INV-025` (wrong beliefs are first-class — the belief is "missing from expected location", not "stolen by X").
5. Actor-knowledge / no-leak surface: the contradiction links the holder's own prior expectation to the holder's own observation, and never links debug/ground truth to the belief (§9.6). The derived belief's `source_kind` is `observation` with the holder's `source_id`; confidence derives from the check confidence. This adds no leakage path and no nondeterminism — detection is a pure function of (expectation beliefs, observation) producing deterministically-ordered events.

## Architecture Check

1. Deriving the missing-property belief from a contradiction *link* (rather than writing the belief directly on absence) preserves the provenance chain expectation→observation→contradiction→belief, which the notebook (ticket 009) and debug mismatch (ticket 010) render. Writing a belief without the link would lose the "contradicts your earlier expectation" audit trail (§12.2).
2. No backwards-compatibility shims: detection is new logic in the epistemic path; it does not alter physical apply.

## Verification Layers

1. Absence requires expectation (`INV-016`) -> unit + integration test: an actor with no expectation checking an empty container gets an observation but no contradiction/belief; an expectation for a *different* location does not contradict this check.
2. Belief is holder-specific and source-backed (`INV-002`/`INV-026`) -> unit test: the derived `ItemMissingFromExpectedLocation` belief has holder = the checking actor, `source_kind: observation`, and a confidence bounded by the check.
3. No culprit leak (`INV-024`/`INV-025`) -> manual review + test: the contradiction/belief names only the missing item and expected location, never an actor as culprit; the link references the holder's observation id, never ground truth.

## What to Change

### 1. Contradiction detection

Add contradiction detection (in `epistemics/contradiction.rs` or a `epistemics/detect.rs` helper) that, given the holder's expectation beliefs and a fresh container-contents observation, determines whether an expected item is absent from the checked container. Wire it into the `check_container` outcome path (ticket 006 step 3).

### 2. Contradiction and belief emission

On a detected contradiction, emit `ExpectationContradicted` (linking expectation belief id + observation id, §9.6) and `BeliefUpdated` for `ItemMissingFromExpectedLocation(item, InContainer(container))` with `source_kind: observation`, the observation `source_id`, channel, and derived confidence (§8.5 required result). Apply through the shared event/projection path (ticket 005).

### 3. Negative-case guards

Ensure no contradiction/belief is emitted when the actor has no relevant expectation, the expectation names a different location, or the item is present.

## Files to Touch

- `crates/tracewake-core/src/epistemics/contradiction.rs` (modify — add detection; file created by 0004PHA2AEPISUB-002)
- `crates/tracewake-core/src/actions/defs/checkcontainer.rs` (modify — invoke detection in outcome path; file created by 0004PHA2AEPISUB-006)

## Out of Scope

- Notebook rendering of the contradiction (ticket 009).
- Debug truth/belief-mismatch view (ticket 010).
- Candidate-goal/lead planning (deferred to Phase 3 per spec §3.2/§10.4); only the notebook-projection lead surface is in scope, in ticket 009.
- Sound-derived beliefs (ticket 014).

## Acceptance Criteria

### Tests That Must Pass

1. Tomas (with the seeded expectation) checking an empty `strongbox_tomas` produces `ExpectationContradicted` + a `BeliefUpdated` missing-property belief held by Tomas.
2. An actor with no expectation checking the same empty container produces only the observation — no contradiction, no missing-item belief.
3. An expectation naming a different location does not contradict the current container check.

### Invariants

1. The contradiction links the holder's own expectation and the holder's own observation; it never links ground truth to the belief.
2. The derived belief names the missing item and expected location only — never a culprit.

## Test Plan

### New/Modified Tests

1. `crates/tracewake-core/src/epistemics/contradiction.rs` (unit tests) — detection true/false across §10.5 negative cases.
2. `crates/tracewake-core/tests/golden_scenarios.rs` (extend) — Tomas check → contradiction → missing-property belief end-to-end.

### Commands

1. `cargo test -p tracewake-core epistemics::contradiction`
2. `cargo test -p tracewake-core --test golden_scenarios`
3. `cargo build --workspace --all-targets --locked`

## Outcome

Completion date: 2026-06-07

What changed:
- Added pure expected-absence detection that compares a holder's expectation beliefs with observed container contents and derives a contradiction plus missing-property belief only when the expected item is absent from the checked container.
- Wired `check_container` pipeline outcomes to optionally consume an `EpistemicProjection`, apply the observation, emit/apply `ExpectationContradicted` and `BeliefUpdated` events, and leave existing callers unchanged when no projection is supplied.
- Added unit tests for positive detection, no-expectation, different-location expectation, and present-item negative cases.
- Added a golden scenario proving Tomas's seeded expectation plus an empty strongbox check yields `ContainerChecked`, `ObservationRecorded`, `ExpectationContradicted`, and `BeliefUpdated` with no culprit language.

Deviations from original plan:
- `PipelineContext` gained an optional `epistemic_projection` field so detection can consume seeded expectations without adding a parallel mutation path. Existing callers pass `None`.

Verification results:
- `cargo test -p tracewake-core epistemics::contradiction`
- `cargo test -p tracewake-core --test golden_scenarios`
- `cargo build --workspace --all-targets --locked`
- `cargo fmt --all --check`
