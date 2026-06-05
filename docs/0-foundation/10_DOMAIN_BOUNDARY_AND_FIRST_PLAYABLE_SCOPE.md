# Domain Boundary and First Playable Scope

## Core claim

Tracewake's kernel is genre-agnostic.

The first domain is a neutral medieval-ish ordinary-life village without magic because it is legible: homes, roads, rooms, doors, containers, workshops, storage, taverns, offices, notices, local authority, travel, imperfect evidence, public records, and household obligation.

The kernel must not become medieval, fantasy, feudal, religious, combat-centric, monster-aware, or magic-aware.

## Kernel boundary

The kernel may know about general simulation concepts:

- actors;
- bodies;
- places;
- objects;
- containers;
- doors and access;
- needs;
- intentions;
- events;
- traces;
- beliefs;
- perception;
- memory;
- speech acts;
- records;
- norms;
- institutions;
- households;
- custody and ownership claims;
- travel;
- LOD;
- regional processes;
- action validation;
- view-model filtering;
- replay/debug.

The kernel must not assume:

- magic;
- monsters;
- species taxonomy;
- divine authority;
- adventurer classes;
- feudal law;
- medieval technology as universal truth;
- combat categories;
- loot tiers;
- quest givers;
- objective markers;
- graphical presentation.

Domain packs own flavor. The kernel owns causal contracts.

## First domain

The first domain should be neutral medieval-ish ordinary life without magic.

Allowed first-domain elements:

- village houses;
- rooms;
- doors;
- roads;
- storage;
- food;
- money;
- work routines;
- workshops;
- tavern or food-service equivalent;
- reeve/clerk/town-office equivalent;
- notice board;
- household obligations;
- simple local norms;
- basic travel;
- basic theft/missing-property incident;
- speech acts, testimony, gossip, lying;
- public records;
- stale information;
- wrong suspicion.

Deferred from v1 foundation:

- strict fantasy elements;
- magic;
- monsters;
- non-human species;
- divine perception or authority;
- granular combat;
- detailed injury simulation;
- procedural dungeons;
- hero/adventurer classes;
- full feudal hierarchy;
- full religion simulation;
- large regional world generation;
- graphical client.

A domain may later add these through causal channels, not through kernel assumptions.

## First serious vertical slice

The first serious implementation target is a small, inspectable, TUI-playable ordinary-life village.

Approximate scope:

```text
10-30 high-detail agents
or at most a few dozen
```

Required emphasis:

- deep causality over scale;
- ordinary routines over adventure;
- actor knowledge over omniscience;
- playability through TUI;
- no-human simulation;
- debug possession and causal inspection;
- deterministic replay of meaningful outcomes.

The village should run for days without human input.

## Ordinary substrate before adventure

The first village must establish enough ordinary life to make disruption meaningful.

Prioritize:

- homes;
- rooms;
- doors;
- containers;
- storage;
- food;
- sleep;
- hunger;
- fatigue;
- work;
- wages or simple payment;
- money and custody;
- ownership/access claims;
- households;
- basic social interaction;
- belief and memory;
- perception;
- traces;
- search;
- records;
- reports;
- TUI embodied play.

Do not prioritize:

- combat;
- monsters;
- magic;
- large world maps;
- authored mysteries;
- procedural quest boards;
- LLM freeform NPCs;
- graphics;
- expedition content;
- macroeconomics.

Adventure may emerge later from ordinary systems. It must not be used to avoid building them.

## Canonical first miracle

The canonical early proof is a theft or missing-property incident emerging from ordinary life.

The scenario should demonstrate:

1. an actor stores or expects property somewhere;
2. another actor, driven by modeled needs, beliefs, opportunity, risk, motive, access, and routine, takes or moves it;
3. the victim later discovers absence through expectation contradiction or search;
4. a witness may have uncertain, partial, stale, or misinterpreted observations;
5. testimony, gossip, or report can propagate;
6. a clerk/reeve/authority may open a record from partial information;
7. wrong suspicion can arise for legible reasons;
8. notices or public artifacts can become stale;
9. the human can possess different ordinary actors in debug mode and verify no knowledge leaks;
10. debug inspection can explain what happened, why it was possible, what traces exist, who knows what, who is wrong, and which later events became possible.

This is not an authored mystery. It is a proof of causal epistemic simulation.

## First playable actions

A first serious village should support staged versions of:

- look/perceive surroundings;
- move between adjacent places;
- open/close doors where possible;
- inspect containers;
- take/place/store/hide items;
- eat from available food/service;
- sleep;
- wait;
- work routine;
- receive/pay money;
- buy simple food or service;
- search a place/container;
- speak through structured speech acts;
- ask/tell/report/deny/lie/gossip;
- read notices/records;
- report to clerk/reeve;
- continue current intention;
- debug possess another high-detail ordinary actor;
- debug inspect causality and belief state.

Each committed world-affecting action must be NPC-possible under equivalent conditions.

## First institution

The first public institution should be a small clerk/reeve/town-office structure.

It should prove:

- office hours;
- reports;
- records;
- limited authority;
- limited funds;
- procedure;
- delay;
- refusal;
- bias or social position effects;
- jurisdiction;
- notice posting;
- stale records;
- wrong suspicion;
- non-omniscience.

It should not be a quest hub.

## First households

Households must exist early.

They provide:

- homes;
- membership;
- sleeping locations;
- food access;
- shared/private storage;
- ownership ambiguity;
- access permissions;
- privacy expectations;
- routine coordination;
- care obligations;
- domestic conflict;
- expectation contradiction;
- inheritance hooks for later systems.

The first theft proof is much weaker without households.

## First economy

The first economy should be small but causal.

Required:

- food objects or service abstractions under custody;
- hunger pressure;
- sleep pressure;
- basic work routines;
- basic wages/payment;
- money custody and ownership claim;
- shops/tavern/households/workplaces as causal stock/service surfaces;
- buying, stealing, storing, hiding, paying, promising payment, and refusing payment as actions;
- no fake meter refill disconnected from storage, travel, work, or access.

Full production chains, pricing models, taxation, debt law, inheritance economics, and trade networks are future architecture/domain work.

## First memory and belief scope

Belief fallibility is foundational. Implementation may stage it.

Early versions should prioritize:

- actor-held propositions;
- confidence;
- source/channel;
- acquisition time;
- believed event time when known;
- staleness;
- contradiction links;
- direct observation vs hearsay;
- lying as structured speech act;
- wrong suspicion;
- expectation contradiction;
- simple confidence decay or memory staleness hooks.

Full forgetting, misremembering, distortion, identity confusion, and memory reconstruction may deepen in later architecture, but the foundation must not block them.

## Domain pack doctrine

Domain packs may add:

- species/body types;
- technology;
- magic/metaphysics;
- special senses;
- threats;
- domain institutions;
- occupations;
- items;
- affordances;
- unique traces;
- diseases;
- speech/culture tags;
- terrain/place types;
- scenario seeds;
- domain laws.

Special powers are causal channels or actions with costs, reliability, distortion, provenance, traces, counters, and failure modes.

A domain pack may not bypass foundational invariants.

## Deferred work

Intentionally deferred from the foundation set:

- concrete Rust module layout;
- crate selection;
- storage engine selection;
- TUI library choice;
- networking model;
- detailed ECS/database architecture;
- exact planner implementation;
- exact schema definitions;
- detailed combat/injury;
- graphics;
- large world generation;
- LLM conversation product design;
- per-task implementation templates.

These belong in architecture, execution, or domain-pack documents after the foundation is stable.

## Vertical-slice acceptance

The first serious village slice is not done unless it proves:

```text
no-human simulation for days
TUI embodied play
actor-knowledge filtering
ordinary life substrate
debug possession parity
property/custody/access distinctions
theft or missing-property miracle
belief provenance
witness uncertainty or wrong belief
report/record/authority fallibility
stale notice or public artifact
deterministic replay
causal debug explanation
LLM-disabled operation
```
