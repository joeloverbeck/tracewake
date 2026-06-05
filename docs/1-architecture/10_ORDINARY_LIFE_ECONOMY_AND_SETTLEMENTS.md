# Ordinary Life, Economy, and Settlements

## Core claim

Adventure is the disruption of ordinary life. Ordinary life must be mechanically deep before adventure matters.

A theft matters because homes, locks, sleep, storage, ownership, household trust, expectation, neighbors, privacy, records, authority, and hiding places exist. A road threat matters because work, food, trade, travelers, families, guards, fear, rumor, and payment exist.

## First priority

The first serious playable slice must prove ordinary life:

- hunger;
- fatigue;
- safety;
- sleep;
- eating;
- work;
- household routines;
- basic social interaction;
- object use;
- ownership and possession;
- storage;
- doors, rooms, homes, workplaces;
- routines as defeasible intentions;
- interruptions caused by modeled conditions;
- simple beliefs and expectations;
- event-sourced traces;
- embodied TUI play;
- possession/debug switching;
- no-player simulation.

Investigation, institutions, notices, expeditions, and threats build on this substrate. They do not replace it.

## Settlement substrate

A settlement supports:

- sleeping;
- meals;
- work;
- storage;
- ownership;
- buying and selling;
- gossip;
- visiting homes, taverns, workplaces, offices, and markets;
- road travel;
- simple production chains;
- office hours;
- notice maintenance;
- household relationships;
- fear;
- routine disruption.

The first village should feel alive before anything dramatic happens.

## Routines as intentions

Bad:

```text
08:00 blacksmith goes to forge.
```

Good:

```text
The blacksmith works because she believes today is a workday,
has orders, needs income, has access to the forge,
has tools and fuel, is not too exhausted,
and has no stronger interruption.
```

This allows work to fail when coal is missing, a child is sick, a guard summons her, the road is blocked, the key is stolen, the shop is unsafe, or she believes a rumor.

## Needs

Start with hunger, fatigue, and safety.

```yaml
Needs:
  hunger:
    ordinary_actions: [eat_at_home, buy_food, eat_at_tavern]
    desperate_actions: [beg, steal_food, eat_stolen_food]
  fatigue:
    ordinary_actions: [sleep_at_home, nap]
    desperate_actions: [sleep_in_public, collapse]
  safety:
    ordinary_actions: [avoid_threat, seek_guard, stay_home, travel_with_companion]
    desperate_actions: [flee_region, arm_self, hide]
```

Needs create pressure. They do not write scripts.

The canonical needs model is `NeedPressure` in `07_AGENT_COMPETENCE_AND_PLANNING_DECISION.md`; this block re-presents the same needs as ordinary-life action menus.

## Homes

Homes are not decorative.

A home provides:

- sleep;
- storage;
- privacy;
- kinship interaction;
- ownership boundaries;
- invitation/trespass logic;
- property expectations;
- hiding places;
- crime scenes;
- line of sight;
- sound propagation;
- doors and locks;
- social meaning.

A door controls access, sound, privacy, legality, and evidence.

## Ownership and possession

Separate:

- legal owner;
- possessor;
- custodian;
- expected location;
- permitted users;
- disputed claimant;
- institutional custody.

This enables theft, borrowing, custody, evidence handling, inheritance, confiscation, ransom, found property, and payment disputes.

## Storage

Containers need:

- location;
- contents;
- access rules;
- lock state;
- expected contents for relevant agents;
- privacy level;
- trace profile;
- affordances;
- damage state;
- ownership/custody.

A strongbox is not an inventory list. It is a causal object.

## Work

Work produces goods, money, records, obligations, services, relationships, expectations, and absence evidence.

Example miller workday:

```text
wake -> eat -> go to mill -> open mill -> process grain
-> receive customers -> record debts/payments -> close mill -> return home
```

Failure cases:

- missing grain;
- broken millstone;
- nonpaying customer;
- illness;
- bandit road closure;
- witness summons;
- stolen tax money;
- fatigue;
- locked workplace;
- fear from rumor.

## Minimal causal economy

The first economy is about consequence, not realism.

Initial goods:

- food;
- coin;
- grain/flour/bread;
- tools;
- paper/ink;
- keys;
- simple weapons as objects, not detailed combat system;
- medicine/bandages;
- stolen valuables.

Initial flows:

```text
farmer -> grain -> miller -> flour -> baker/tavern -> food -> villagers
travelers -> trade -> market -> tax -> authority
authority -> funds -> payment obligations -> guards/adventurers
road risk -> delayed goods -> prices/fear/work disruption
```

## Shops and services

A shop is:

- a location;
- owner/operator;
- inventory;
- hours;
- prices/beliefs;
- records/debts;
- supply dependencies;
- reputation;
- vulnerability to theft, illness, closure, fear, and shortage.

A closed shop should matter.

## Taverns

Taverns move information.

They support:

- eating;
- resting;
- gossip;
- hiring;
- overhearing;
- public disputes;
- hiding in crowds;
- meeting travelers;
- observing absence;
- reading copied or discussed notices.

## Social basics

The first slice needs only enough social action to support ordinary life and epistemics:

- greet;
- ask;
- answer;
- report;
- gossip;
- refuse;
- lie;
- recruit;
- promise payment;
- accuse/suspect with actor-known basis;
- apologize.

These are a scoped subset of the canonical speech-act vocabulary in `08_INFORMATION_ECOLOGY_AND_SPEECH_ACTS.md`.

## Roads as ordinary life

Roads are not just expedition corridors.

Roads move:

- people;
- food;
- goods;
- rumors;
- disease;
- fear;
- institutional orders;
- bandits;
- patrols;
- witnesses.

A dangerous road affects prices, work, meals, sleep, and plans.

## Adventurers and companions

Adventurers are ordinary agents with needs, motives, fear, equipment, relationships, reputation, rates, risk thresholds, and independent goals. They may accept, refuse, betray, flee, demand proof, split loot, insist on rest, or leave for better work.

Do not implement them as player tools.

## No detailed combat first

The first serious vertical slice does not implement detailed combat. Early threats may be represented through:

- route risk;
- injury/death events;
- aftermath traces;
- corpse discovery;
- fear;
- reports;
- route avoidance;
- guard response;
- rumor;
- payment disputes.

Combat is its own system and must not hijack the foundation.

## First settlement

Build:

- 30–60 named agents at varied LOD;
- 8–12 homes;
- tavern;
- market/notice board;
- authority office;
- mill/workshop;
- one road;
- one nearby threat site;
- food/coin/tool flows;
- work/sleep/eat/social routines;
- storage and property;
- rumor spread;
- routine interruption;
- no-player daily simulation;
- embodied TUI.

## Anti-patterns

- “ordinary life” as static backdrop;
- NPCs that only wait for player quests;
- homes with no storage/permission/expectation logic;
- shops that never close;
- needs that are cosmetic;
- schedules that ignore belief and interruption;
- economy realism before consequence;
- adventure systems before sleep/eat/work;
- combat-first prototype.
