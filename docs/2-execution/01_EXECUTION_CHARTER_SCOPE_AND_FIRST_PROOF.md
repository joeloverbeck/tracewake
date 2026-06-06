# Execution Charter, Scope, and First Proof

## Status

This document defines Tracewake's execution-level first proof and scope limits.

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

Execution may choose phase order, proof scope, fixture names, test minimums, and data-authoring priorities.

Execution may not weaken the constitutional rules:

- causality before drama;
- belief before truth;
- ordinary life before adventure;
- no sacred player entity;
- every world-affecting player action must be possible for an ordinary agent under equivalent actor conditions;
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

## Scope decision

The first execution scope is deliberately narrow.

Older or broader design ambitions such as notice boards, route threats, stale leads, companion recruitment, expedition travel, proof delivery, and payment remain valuable, but they are not the first proof. They belong behind the second-proof boundary.

The first execution gravity is:

```text
missing property
+ expectation contradiction
+ partial observation
+ search and questioning
+ report
+ record
+ wrong suspicion
+ possession parity
+ no-human ordinary life
+ forensic replay
```

The first proof is **The Missing Property Village**. The deferred second proof is **Notices, Travel, and Regional Expansion**.

## Product priority

Tracewake execution obeys this priority order:

1. playable TUI simulation first;
2. research-grade emergent simulation engine second;
3. future graphical presentation later.

The first serious deliverable is not a graphical demo, not a chatbot, not a quest board, not a bounty loop, and not a combat-first RPG.

It is a small, inspectable, TUI-playable ordinary-life village whose agents can live, work, expect, misperceive, report, record, suspect, and be wrong without a human present.

## First proof

The first proof is **The Missing Property Village**.

It must prove that:

1. an actor stores or expects property somewhere;
2. another actor, driven by modeled needs, beliefs, opportunity, risk, desperation, obligation, or routine pressure, takes or moves it;
3. the victim later discovers absence through expectation contradiction or search;
4. a witness may have uncertain, partial, stale, or misinterpreted observations;
5. testimony, gossip, or report can propagate;
6. a clerk or local authority may open a record from partial information;
7. wrong suspicion can arise for legible reasons;
8. debug possession can move between ordinary actors without knowledge transfer;
9. debug inspection can explain what happened, why it was possible, what traces exist, who knows what, who is wrong, and what later events became possible;
10. the whole chain can occur with no human input.

This is the first miracle. Nothing else outranks it.

## What is narrowed

The first execution scope narrows:

- population from broad mixed-detail settlement ambitions to about 10-20 high-detail named agents initially, expanding toward 10-30 only after stability;
- map from village plus road/quarry/threat site to village rooms, homes, workplaces, office, containers, and local ordinary movement;
- social scope from recruitment/travel/expedition to ask, answer, tell, report, gossip, refuse, lie, accuse/suspect, and minimal testimony;
- institution scope from notice/bounty/proof/payment to one local authority, report intake, incident ledger, minimal questioning, and wrong suspicion;
- story scope from overt dramatic lead presentation to missing property, expectation, absence, partial knowledge, records, and suspicion;
- LLM scope from dialogue surface ambition to deterministic templates/mocks only;
- data scope from broad authoring guide to first-proof fixtures and schema validation;
- testing scope from broad slice validation to hard phase gates and named golden scenarios.

## First-proof fixture posture

The first proof needs enough authored starting conditions to expose the machinery without scripting the outcome.

Allowed initial conditions include:

- prior relationships;
- debts;
- grudges;
- trust or distrust;
- access permissions;
- work routines;
- container ownership;
- physical coin or value-token placement;
- expectations about property location;
- partial records;
- a reason someone might need money;
- a prior visitor or rumor;
- institutional role assignments;
- local norms and office hours.

Forbidden fixture shortcuts include:

- a quest flag saying property is stolen;
- a hidden director forcing discovery;
- an omniscient culprit pointer;
- guaranteed accusation;
- guaranteed resolution;
- reward completion state;
- authored dramatic pacing;
- a UI-only fact that has no event, observation, record, or belief source;
- abstract money balances where the proof requires physical custody.

## Required human experience

A human should be able to enter the TUI and meaningfully play the proof from actor-grounded knowledge.

During embodied play, the human sees only what the possessed actor can see, remember, infer, afford, or access through records and speech. Debug inspection may reveal more, but it must be explicitly separated from embodied play.

Possessing a different actor changes the view to that actor's beliefs and affordances. The simulation does not pretend to erase the player's real memory, but it must not grant the newly possessed actor the previous actor's beliefs.

## Required no-human experience

The same kind of chain must be possible with no human input.

A no-human run does not need to produce the same chain every seed, but accepted fixtures must include deterministic seeds where the chain occurs through modeled causes. The run must be replayable and explainable from the event log and causal graph.

## Deferred second proof

The following remain deferred until the first proof is accepted:

- roads;
- quarry or wilderness sites;
- route threat;
- beasts;
- caravan attack;
- adventurer recruitment;
- companion travel;
- notice/bounty/payment loops;
- proof-of-kill or proof-of-service artifacts;
- regional history beyond grounded local context;
- large LOD simulation.

The second proof may use the same machinery later. It may not backfill missing first-proof foundations.

## Execution readiness bar

Execution is ready for implementation only when a coding agent can begin the next phase without inventing product policy on:

- scripting vs allowed authored seeds;
- belief vs truth;
- actor-grounded possession;
- structured speech and belief transmission;
- LLM non-authority;
- physical property and custody;
- story sifting as projection only;
- no-human operation;
- deterministic replay and provenance.

If those answers are missing or contradictory, implementation must not begin.
