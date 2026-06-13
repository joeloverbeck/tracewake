# First Proof Scope, Current Baseline, and Acceptance Contract

## Status

Live execution doctrine for the first playable proof. Replaces the old first-proof, phase-charter, and missing-property execution documents.

## Authority boundary

This document owns first-proof scope and acceptance shape. It does not certify current code and does not authorize Phase 4 entry.

## Depends on

- `docs/0-foundation/01_PROJECT_CHARTER.md`
- `docs/0-foundation/06_ACTIONS_AFFORDANCES_ORDINARY_LIFE_AND_SURVIVAL.md`
- `docs/0-foundation/08_TUI_POSSESSION_VIEW_MODELS_AND_DEBUG.md`
- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`
- `docs/1-architecture/09_ORDINARY_LIFE_SETTLEMENT_SPATIAL_ECONOMY_AND_PROPERTY.md`
- `docs/1-architecture/10_POSSESSION_TUI_VIEW_MODELS_DEBUG_AND_CLIENT_BOUNDARIES.md`
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`

## First proof identity

The first proof is a small village simulation in which an expected property is missing and the world continues without knowing or caring whether a human is present.

The proof is not a mystery script. It is not a player-centered quest. It is not a Phase 4 institution showcase. It is the executable demonstration that ordinary life, subjective belief, possession parity, event-sourced causality, and replayable diagnostics can support a missing-property situation without hidden-truth or protagonist gravity.

## Current baseline status

The repository contains historical implementation for kernel/TUI/replay, epistemics, possession parity, needs/routines, no-human day, and anti-contamination hardening work. That history is useful evidence. It is not certification.

The first-proof baseline remains open until `P0-CERT` proves that current implementation satisfies this replacement execution contract.

## The acceptance contract

A first-proof candidate must satisfy all of these gates in one coherent artifact set.
The local `Gate` headings below are first-proof acceptance-contract labels.
They do not add canonical gate codes beyond `00_EXECUTION_INDEX_AND_AUTHORITY.md`.

### Gate `EVENT`

Every meaningful world change is represented by an event with stable identity, causal ancestry, schema/version information, deterministic ordering, and replay effect.

Current state is not authority. It is projection from initial fixture plus events.

### Gate `TRUTH-FIREWALL`

Truth may validate facts such as reachability, item presence, resource availability, access rights, and invariant preservation. Truth must not choose actor goals, routine methods, hidden food targets, suspects, witnesses, clues, or view-model affordances.

### Gate `ACTOR-KNOWN`

Actor decisions use actor-known context with provenance. Beliefs, memories, observations, routine assignments, records, and known affordances must state their source. Unknowns and stale knowledge are explicit.

### Gate `POSSESSION-PARITY`

A human-controlled actor and an autonomous actor operate under the same world rules. Possession binds input to an ordinary actor. It does not grant a privileged body, privileged memory, privileged actions, or hidden truth.

### Gate `NO-HUMAN-ORDINARY-LIFE`

The simulation can advance through a boring ordinary-life interval without human input. Progress means ordinary actions, modeled waits, or typed stuck/failure records through the normal transaction and action pipeline. It does not mean scheduler-authored state deltas.

### Gate `MISSING-PROPERTY`

A missing expected property is discovered through actor-relative expectation and observation, not through global theft truth. The holder may believe an item should be somewhere, observe absence, form a contradiction, search, ask, report, misremember, suspect wrongly, or do nothing. The world does not supply a culprit flag.

### Gate `VIEW-DEBUG-SPLIT`

Embodied view models expose only the possessed actor's holder-known context and actor-legible affordances. Debug surfaces may expose truth only with non-diegetic markers and must not feed embodied affordances, actor cognition, institution procedure, or acceptance tests that claim actor knowledge.

### Gate `REPLAY`

Replay from the fixture plus ordered events rebuilds authoritative projections and diagnostic artifacts. The replay report must identify divergence location, not only assert mismatch.

### Gate `FIXTURE-NEGATIVE`

Golden fixtures include success and forbidden paths. At minimum, they must prove no hidden-truth planning, no direct dispatch, no human special case, no debug leakage, no quest/culprit data, no marker-only ordinary life, and no replay drift.

## Observer-only emergence evidence artifact

Any first-proof acceptance packet that claims living-world acceptance must also
include the observer-only emergence-evidence artifact produced under
`10_TESTING_OBSERVABILITY_DIAGNOSTICS_AND_REVIEW_ARTIFACTS.md` (`EMERGE-OBS`).
This artifact is required beside the blocking gate list above, not inside it.
It is retrospective, observer-only, event-log-ancestry-backed, and never a
pass/fail threshold or certification gate.

## First-proof scenario families

The first proof may use several fixtures, but acceptance is for the combined contract, not for individual fixtures in isolation.

| Scenario family | Required proof |
|---|---|
| Physical custody baseline | Items, places, containers, doors, access, take/place/open/close/inspect/search, event log, replay, and embodied TUI. |
| Expectation contradiction | An actor with source-backed expectation discovers absence and records actor-known contradiction without culprit knowledge. |
| Possession parity | Binding a controller changes input routing only and does not reset or privilege the actor. |
| Epistemic filtering | Actor notebook/view shows holder-known facts; debug surfaces remain separate. |
| No-hidden-truth planning | Needs/routines/planner cannot select targets from hidden truth or fixture-only facts. |
| No-human ordinary day | Actors pursue sleep/eat/work/wait/stuck paths through transaction and action pipeline without human input. |
| Routine blocking | Blockers produce typed diagnostics and modeled fallback/wait, not loops or teleportation. |
| Replay rebuild | Event log rebuilds projections and diagnostics deterministically. |
| Content rejection | Forbidden authoring fields and outcome chains are rejected before runtime. |

## Definition of first-proof done

The first proof is done only when:

- `P0-CERT` passes;
- all first-proof gates above pass in CI-equivalent review artifacts;
- every failure artifact names the responsible layer;
- no accepted behavior requires knowing which actor, if any, is human-controlled;
- no accepted behavior depends on hidden truth planning, debug leakage, direct dispatch, or archived-spec authority;
- the code audit report says which implementation seams were inspected and which gates they satisfy.

## Explicit non-goals before first-proof completion

The first proof does not include LLM dialogue, full institutional investigation, regional travel, story sifting, quest boards, adventure routes, long-history generation, or graphical client work.

Those areas remain locked behind named gates in documents `11` and `12`.
