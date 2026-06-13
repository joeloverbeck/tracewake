# Golden Fixtures, Scenarios, and Replay Acceptance

## Status

Live execution doctrine for golden fixture acceptance. This document splits fixture acceptance from authoring/schema validation.

## Authority boundary

This document owns golden fixture families, replay acceptance, and success/forbidden scenario coverage. It does not define implementation code and does not certify existing fixtures.

## Depends on

- `docs/0-foundation/03_CAUSAL_EVENT_TRACE_AND_REPLAY_CONTRACT.md`
- `docs/0-foundation/12_FIRST_PLAYABLE_SCOPE_AND_ACCEPTANCE_GATES.md`
- `docs/1-architecture/02_EVENT_LOG_REPLAY_PROJECTIONS_SAVE_AND_RANDOMNESS.md`
- `docs/1-architecture/13_VALIDATION_OBSERVABILITY_ACCEPTANCE_AND_REVIEW_ARTIFACTS.md`
- `docs/2-execution/08_DATA_AUTHORING_SCHEMA_PROVENANCE_AND_VALIDATION.md`

## Core rule

Golden fixtures prove both what Tracewake can do and what it refuses to do.

A friendly fixture that proves only the happy path is not enough. Every acceptance family needs a paired forbidden or adversarial case.

## Fixture taxonomy

| Fixture class | Purpose |
|---|---|
| Baseline | Establish physical/action/event/TUI/replay spine. |
| Epistemic | Prove observations, beliefs, contradictions, notebooks, and actor-known filtering. |
| Possession | Prove possession parity and no human special case. |
| Ordinary-life | Prove needs, routines, intentions, no-human advancement, and blockers. |
| Anti-contamination | Prove hidden truth, debug truth, and direct-dispatch shortcuts are excluded. |
| Content validation | Prove forbidden authoring forms are rejected. |
| Replay/projection | Prove deterministic rebuild and divergence diagnostics. |
| Phase-entry | Prove entry gates before institutions, records, wrong suspicion, or second-proof expansion. |

## Required first-proof fixture families

The exact filenames may change only with an execution-index update. The fixture families may not be removed without replacing their proof burden.

| Family | Must prove |
|---|---|
| Strongbox/physical baseline | Stable IDs, places, doors, containers, items, action proposals, validation, events, TUI, debug, replay. |
| Expected absence | Actor had source-backed expectation; search/check produces absence and contradiction without culprit knowledge. |
| Possession parity | Human binding and autonomous operation share action rules; possession does not reset actor state. |
| View filtering | Embodied view excludes hidden truth; debug view may show truth non-diegetically. |
| Knowledge blocker | Accusation/report/procedure is blocked by holder-known insufficiency, not by true culprit absence. |
| Sound/uncertain observation | Low-confidence sensory evidence stays uncertain and source-bound. |
| No-human epistemic action | An actor can observe/check without privileged human presence. |
| No-human ordinary day | Multiple actors progress through ordinary transactions without human input. |
| Food unavailable replan | Actor cannot use hidden food truth; fallback/wait/stuck is typed. |
| Routine blocked diagnostic | Blocker produces typed diagnostic and replayable artifact. |
| Routine no-teleport | Routine cannot begin remote work/sleep without movement or duration ancestry. |
| Planner trace | Candidate/method/local-plan decisions and rejections are inspectable. |
| No hidden truth planning | Hidden target exists but is excluded from candidate generation and method selection. |
| Debug omniscience excluded | Debug truth cannot feed actor affordances, cognition, or tests claiming actor knowledge. |
| Forbidden content | Quest/player/culprit/outcome fields are rejected. |

## Replay acceptance

A golden fixture passes replay only when:

- initial fixture plus ordered events rebuilds physical state;
- epistemic projections rebuild holder-known beliefs, observations, contradictions, and notebook views;
- agent projections rebuild needs, intentions, routine state, candidate traces, and stuck diagnostics;
- debug projections rebuild non-diegetic reports without becoming actor inputs;
- random draws, if any, are scoped and replayable;
- divergence diagnostics name first mismatching event, projection, checksum, or schema boundary.

A replay test that checks only final item location is insufficient.

## Adversarial fixture rules

Each adversarial fixture must include one tempting shortcut and prove it is unusable. Examples:

- true food exists but the actor does not know it;
- hidden route exists but actor-known route does not;
- closed container contains the missing item but embodied view cannot know that;
- debug report contains culprit-like truth but actor notebook does not;
- routine target exists in fixture data but lacks actor-known assignment provenance;
- validator knows rejection reason but actor-visible why-not excludes hidden detail;
- content field uses a renamed forbidden concept;
- scheduler has an opportunity to shortcut but produces transaction trace or diagnostic instead.

## Fixture acceptance artifact

Each fixture contract must record:

- doctrine dependencies;
- fixture purpose;
- expected actor-known inputs;
- hidden truth present for negative checks;
- expected proposals/actions/events;
- expected rejections/failures/stuck diagnostics;
- actor-visible outputs;
- debug outputs;
- replay expectations;
- forbidden shortcuts being tested;
- gate names satisfied.

Where a fixture family exercises first-proof living-world acceptance or the
canonical no-human corpus, the fixture contract may supply semantic support for
the observer-only `EMERGE-OBS` artifact by naming the events, projections, and
phenomenon families available for retrospective extraction. That support is not
a fixture pass threshold and does not convert `EMERGE-OBS` into a gate.

## Golden certification gate `FIXTURE-CERT`

`FIXTURE-CERT` is a phase-certification artifact label from the execution
sequence. It consumes canonical gate evidence; it is not a new canonical gate
code beyond `00_EXECUTION_INDEX_AND_AUTHORITY.md`.

`FIXTURE-CERT` passes only when:

1. Every live gate has at least one positive fixture and one negative fixture.
2. Fixtures are deterministic under repeated load and replay.
3. Failure fixtures fail at the expected layer.
4. Fixture manifests do not rely on archived spec authority.
5. Debug truth is tested as quarantined truth, not embodied knowledge.
6. No fixture proves a behavior by parsing display strings.
7. CI-equivalent outputs include artifact names and responsible layers.
8. Fixture coverage is updated before any new phase-entry gate is claimed.

## Phase 4 and second-proof fixtures

Institutions, records, wrong suspicion, notices, travel, regional scale, and story-sifting fixtures may be drafted as locked contracts. They may not become live acceptance fixtures for implementation until documents `11` and `12` entry gates pass.
