# Genre-Agnostic Domain Model

## Core claim

Tracewake's core is genre-agnostic. Domain packs provide flavor.

Fantasy, post-apocalyptic, Lovecraftian, historical, science-fiction, and other settings must run on the same causal substrate:

```text
actors, bodies, places, events, actions, beliefs, traces, speech acts,
needs, roles, norms, institutions, records, ownership, travel, LOD,
view models, replay, debugging
```

## Core ownership

The core owns:

- event sourcing;
- action pipeline;
- entity/component framework;
- belief model;
- perception model;
- trace model;
- speech-act model;
- institutions/norm engine;
- ordinary-life primitives;
- ownership/custody;
- travel abstractions;
- LOD;
- UI view models;
- replay/debugging;
- deterministic scheduling;
- data validation;
- source/provenance mechanics.

The core must not know it is fantasy.

## Domain pack ownership

Domain packs own:

- species/body types;
- technology level;
- magic/metaphysics;
- monsters/threats;
- domain institutions;
- laws/norms;
- occupations;
- items;
- object affordances;
- special traces;
- diseases/conditions;
- speech/culture tags;
- terrain/place types;
- procedural constraints;
- scenario seeds;
- clothing/equipment;
- food types;
- local calendars;
- domain-specific UI labels.

## No fantasy leakage

Forbidden kernel assumptions:

- monsters as native category;
- magic as privileged truth access;
- adventurer as core role;
- medieval law hard-coded;
- coins as only money;
- tavern as core institution;
- swords/combat as core interaction;
- humans as only bodies;
- feudal titles in kernel rules;
- divine truth as default channel;
- “quest board” as engine object.

Allowed core categories are neutral:

```text
threat process, information channel, institution, role, norm, item,
body, condition, route, record, notice, contract, speech act
```

## First domain

The first domain should be restrained neutral medieval-ish low fantasy.

Use it because it supports:

- homes;
- roads;
- taverns;
- notice boards;
- imperfect evidence;
- fallible local authority;
- simple technology;
- visible labor;
- slow information;
- public/private spaces;
- ownership disputes;
- ordinary-life routines.

Do not overbuild:

- magic;
- species;
- religion;
- combat;
- procedural terrain;
- large kingdoms;
- graphical presentation.

## Magic and special senses

Magic is modeled causality.

A divination spell is an information channel/action with:

- caster;
- target;
- cost;
- required belief/skill/material;
- range;
- reliability;
- distortion;
- failure modes;
- traces;
- countermeasures;
- institutional meaning;
- belief provenance.

It does not bypass epistemic doctrine.

## Monsters and threats

A monster is not a quest target. It is an actor, group, process, or hazard with:

- needs or drives;
- territory;
- perception;
- behavior;
- traces;
- vulnerabilities;
- movement;
- origin if relevant;
- ecological consequences;
- institutional consequences;
- public beliefs that may be wrong.

A beast killing livestock should create missing animals, tracks, fear, rumors, reports, patrol decisions, economic effects, and route avoidance.

## Technology differences

The same core pattern supports different technology.

Communication examples:

```text
Medieval-ish: messenger, town crier, notice board, rumor
Post-apocalyptic: radio, courier, signal flare, graffiti
Lovecraftian: letter, telegram, dream, occult sign
Science fiction: network packet, drone, sensor, encrypted log
```

All are information channels with delay, reliability, access, distortion, and provenance.

## Norm differences

Different domains define different norms:

- necromancy may be illegal, sacred, licensed, or common;
- theft may lead to jail, exile, debt bondage, feud, or insurance claim;
- monster hunting may require license or be tolerated vigilantism;
- property may belong to households, corporations, temples, lords, or no one;
- testimony may be weighted by class, species, caste, credential, or reputation.

The core norm engine stays the same.

## Injury and healing

The core eventually supports injury, pain, bleeding, mobility impairment, unconsciousness, death, recovery, infection, and disease hooks. Domain packs define herbal medicine, surgery, antibiotics, magic, ritual purification, alien technology, or cybernetics.

Detailed combat is not required for the first vertical slice.

## Domain pack test

A domain pack is valid only if:

- its actions go through the core action pipeline;
- its information channels produce sourced beliefs;
- its institutions use explicit norms/procedures;
- its special powers leave causal traces or costs;
- its threats are processes/actors/hazards, not quest targets;
- it can run no-player simulation;
- the TUI can play it through view models;
- it does not require LLM authority.

## Anti-patterns

- hard-coding fantasy into kernel;
- magic as omniscience;
- monsters as quest targets;
- sanity as random nonsense;
- post-apocalyptic scarcity as mere low item counts;
- domain prose creating facts;
- UI labels driving rules;
- domain pack bypassing belief provenance.
