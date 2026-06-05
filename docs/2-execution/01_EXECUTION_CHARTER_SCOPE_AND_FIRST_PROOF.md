# Execution Charter, Scope, and First Proof

## Status

This document is part of the corrected execution-level replacement set for **Tracewake** at target commit `5f01f72e0d3f42243becd95160a98cf7565fdb1c`. See `00_EXECUTION_INDEX_AND_AUTHORITY.md` for the evidence ledger and exact-commit source policy.

Execution documents define what gets built first, in what order, with which gates, fixtures, acceptance criteria, validation rules, and deferrals. They do not replace foundation doctrine or architecture contracts. They do not contain implementation tickets or Rust source code.

## Authority relationship

Execution obeys this authority order:

```text
foundation doctrine
 -> architecture contracts
 -> execution phase gates and fixtures
 -> implementation specs/code
 -> tests and validation reports
```

When execution conflicts with foundation or architecture, execution is wrong. When a later implementation is more convenient than the execution gates, the implementation is wrong.

Execution may choose phase order, proof scope, fixture names, test minimums, and data-authoring priorities. Execution may not weaken the constitutional rules:

- causality before drama;
- belief before truth;
- ordinary life before adventure;
- no sacred player entity;
- every world-affecting player action must be possible for an ordinary agent under equivalent conditions;
- institutions are fallible social machines;
- quests are projections, not ontology;
- authored causal machinery is allowed;
- authored outcome chains are forbidden;
- symbolic, inspectable agents before generative agents;
- TUI-first, playable always;
- genre-agnostic kernel;
- story is observed, not directed;
- LLMs may render or parse language only behind validation;
- LLMs are not authoritative simulation brains;
- Rust-first implementation;
- event sourcing and forensic causality are foundational;
- no-human simulation is mandatory in every runnable phase;
- UI is not a rules engine;
- persistence is not the simulation model;
- domain packs cannot bypass the action/event pipeline;
- debug tools cannot leak into embodied play.

## Replacement decisions

The earlier execution set was directionally strong but too broad. It mixed the first village proof with a second proof: notices, route threat, stale leads, companion recruitment, expedition travel, proof, and payment. Those ideas are preserved, but not in the first proof.

This replacement set makes these corrections:

```text
old gravity:
  village + road threat + notice board + companion + stale-lead travel + proof/payment

new gravity:
  missing property + expectation contradiction + partial observation + report + record
  + wrong suspicion + possession parity + no-human ordinary life + forensic replay
```

The first proof is **The Missing Property Village**. The second proof is **Notices, Travel, and Regional Expansion**.

## Product priority

Tracewake execution obeys this priority order:

1. playable TUI simulation first;
2. research-grade emergent simulation engine second;
3. future graphical presentation later.

The first serious deliverable is not a graphical demo, not a chatbot, not a quest board, not a bounty loop, and not a combat-first RPG. It is a small, inspectable, TUI-playable ordinary-life village whose agents can live, work, expect, misperceive, report, record, suspect, and be wrong without a human present.

## First proof

The first proof is **The Missing Property Village**.

It must prove that:

1. an actor stores or expects property somewhere;
2. another actor, driven by modeled needs, beliefs, opportunity, risk, or desperation, takes or moves it;
3. the victim later discovers absence through expectation contradiction or search;
4. a witness may have uncertain, partial, stale, or misinterpreted observations;
5. testimony, gossip, or report can propagate;
6. a clerk or local authority may open a record from partial information;
7. wrong suspicion can arise for legible reasons;
8. debug possession can move between ordinary actors without knowledge transfer;
9. debug inspection can explain what happened, why it was possible, what traces exist, who knows what, who is wrong, and what later events became possible;
10. the whole chain can occur with no human input.

This is the first miracle. Nothing else outranks it.

## What was narrowed

The replacement execution set explicitly narrows:

- population from 30–60 varied-LOD agents to about 10–20 high-detail named agents initially, expanding toward 10–30 only after stability;
- map from village plus road/quarry/threat site to village rooms, homes, workplaces, office, containers, and local ordinary movement;
- social scope from recruitment/travel/expedition to ask, answer, tell, report, gossip, refuse, lie, accuse/suspect, and minimal testimony;
- institution scope from notice/bounty/proof/payment to one local authority, report intake, incident ledger, minimal questioning, and wrong suspicion;
- story scope from “The Village That Notices” to missing property, expectation, absence, partial knowledge, records, and suspicion;
- LLM scope from dialogue surface ambition to deterministic templates/mocks only;
- data scope from broad authoring guide to first-proof fixtures and schema validation;
- testing scope from broad slice validation to hard phase gates and named golden scenarios.

## First proof vs second proof

First proof owns:

- homes, rooms, doors, beds, containers, food, sleep, work;
- ownership, possession, custody, access, expected location;
- local movement and basic storage/search;
- hunger, fatigue, safety as ordinary pressures;
- no-human day simulation;
- direct perception, sound observation, absence-as-evidence;
- belief store, confidence, source, acquisition time, contradiction;
- actor-known view models and debug view models;
- debug possession parity;
- speech acts sufficient for report, testimony, gossip, refusal, lying, suspicion;
- one local authority office;
- clerk, reeve, possibly one or two guards;
- incident ledger and record provenance;
- wrong suspicion from partial information.

Second proof owns:

- notice lifecycle as a primary public-artifact flow;
- public work artifacts;
- road threat;
- stale lead travel;
- companion recruitment;
- route risk;
- proof/payment;
- simple regional expansion;
- observer story sifting beyond debug recap.

A second-proof feature cannot be started to compensate for a failed first-proof gate.

## Strict phase ladder policy

The implementation order is a strict ladder:

```text
Phase 0  Paper ontology and fixture design
Phase 1  Runnable kernel, TUI harness, and event log
Phase 2  Epistemics, view models, and possession parity
Phase 3  Needs, routines, work, and no-human daily simulation
Phase 4  Institutions, records, and wrong suspicion
Second proof only after Phase 4 exits
```

Later phases do not backfill earlier failure. A notice board cannot hide a shallow home model. An expedition cannot hide weak routines. A clever dialogue surface cannot hide missing belief provenance. A debug command cannot stand in for TUI playability.

## TUI-first policy

Every runnable phase must be playable or view-model-acceptance-testable through the TUI.

Required pattern:

```text
minimal kernel + minimal TUI harness
 -> new mechanic
 -> actor-filtered view model
 -> TUI reachability
 -> why-not explanations
 -> debug inspection
 -> deterministic replay
 -> automated acceptance test
```

A headless feature is not accepted after Phase 1 unless it has a TUI/view-model gate.

The TUI may present. It may not rule. The TUI consumes view models and submits commands. It does not read hidden truth in embodied mode, mutate state directly, invent quest progress, or grant a player-only verb.

## No-human from Phase 1

Every runnable phase must run with no human controller bound.

In Phase 1 this may only prove that the scheduler advances, simple actions resolve, and no event references player identity. From Phase 3 onward it must prove ordinary daily life: sleep, eat, work, wait, speak minimally, store, search, report when warranted, and respond to modeled interruptions.

No-human simulation is not a special mode. It is normal simulation without a human boundary.

## No player privilege

The only acceptable controller model is:

```text
HumanController -> ActorId
```

Possession changes input binding only. It does not move beliefs, intentions, obligations, memory, suspicion, property, guilt, records, or reputation between actors.

A world-affecting action available to the human must be available to an ordinary NPC under equivalent conditions. Debug-only commands must be visibly non-diegetic and excluded from embodied acceptance paths.

## LLM execution policy

LLMs are deferred beyond the first proof.

For the first proof, language uses deterministic templates or mocks for:

- structured speech-act rendering;
- record summaries;
- debug explanations;
- test fixtures.

LLMs may not decide actions, parse authoritative freeform play, create facts, create records, create clues, determine guilt, mutate state, create quests, or make tests depend on live output. The core must run with LLMs disabled.

## Story sifting policy

Story sifting is not part of the first proof unless it is minimal, observer-only, and debug/no-human oriented.

Allowed early:

- group event chains after they occur;
- summarize no-human simulation;
- surface contradictions;
- help debug interesting wrong beliefs;
- produce non-authoritative debug recaps.

Forbidden:

- spawn events;
- repair pacing;
- select culprits;
- create clues;
- make NPCs wait;
- reveal hidden truth in embodied mode;
- become a director.

## Data format posture

Do not freeze the final content file syntax early.

Execution requires:

- schema-validated human-editable data where practical;
- stable IDs and deterministic loading order;
- versioned fixtures;
- source-backed initial beliefs;
- source-backed reports, records, traces, notices, and speech acts;
- domain data that creates possibility space, not outcome chains.

RON, TOML, YAML, JSON, or a custom format remain candidates. Execution defines logical contracts and validation gates first.

## Rust-first posture

Tracewake is Rust-first. Execution may name candidate crates, but not lock the project into them without later justification.

Candidates that may be evaluated:

- Ratatui or equivalent for TUI;
- crossterm or equivalent terminal backend;
- proptest for property tests;
- insta for snapshot/golden output tests;
- cargo-nextest for test execution;
- criterion for benchmarks;
- serde plus schema tools for data;
- tracing/logging tools for debug.

These are candidates, not doctrine. The doctrine is deterministic core, event sourcing, replay, schema validation, actor-knowledge filtering, action parity, no-human simulation, TUI playability, and clear boundaries.

## Severe anti-scope-creep rules

Do not start the second proof until Phase 4 passes.

Do not implement:

- road-threat bounty as first proof;
- companion recruitment as first proof;
- stale-lead expedition as first proof;
- detailed combat;
- magic;
- adventurer classes;
- LLM NPC brains;
- graphical client;
- region generator;
- large map;
- objective quest tracker;
- global wanted meter;
- player-only verbs;
- hidden drama director.

Do not preserve exciting scope merely because it is exciting. Delete premature ambition. The village must be able to explain one missing coin before it tries to explain a road.
