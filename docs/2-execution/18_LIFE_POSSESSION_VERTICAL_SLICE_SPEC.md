# Life-Possession Vertical Slice Specification

## Goal

Build a small playable TUI simulation proving the thesis:

> Ordinary agents live, need, work, believe, expect, notice, misinterpret, speak, remember, record, and act through the same causal machinery as the human-controlled body.

This slice is about causal credibility and ordinary life. It is not about graphics, combat depth, procedural terrain, open-ended LLM chat, or authored quests.

## Slice identity

Working name: **The Village That Notices**.

The experience should feel like a life-possession sandbox first and an investigation/adventure seed second.

## Non-negotiable exclusions

The first serious vertical slice does not include:

- detailed combat;
- procedural quest generation;
- drama director;
- large world generation;
- graphical client;
- open-ended LLM NPCs;
- magic system;
- species complexity;
- authored quest beats;
- player-exclusive verbs.

Threats may exist through route risk, injury/death events, aftermath traces, fear, reports, and institutional response.

## Map

A single village and nearby road:

- 8–12 homes;
- mill or workshop;
- tavern;
- market square;
- authority office;
- notice board;
- optional holding cell;
- north road;
- old quarry / nearby threat site;
- farm or delivery point.

Rooms, doors, containers, and storage must matter.

## Population

30–60 agents at varied LOD.

Required named actors:

- Tomas, miller and property owner;
- Elena, uncertain witness;
- Mara, thief/desperate worker;
- Anna, clerk;
- Elias, guard;
- Oren, traveler/adventurer/possible companion;
- Reeve, local authority;
- one bandit group, beast, or route threat represented without detailed combat.

Each major actor needs at least one project or durable concern.

## Required ordinary-life systems

### Needs and routines

- hunger;
- fatigue;
- safety;
- sleep;
- eating;
- work;
- household routine;
- tavern/social routine;
- office hours;
- interruptible intentions;
- no-player daily simulation.

### Objects and spaces

- rooms;
- doors;
- homes;
- workplace;
- containers;
- ownership;
- possession;
- storage;
- keys/locks;
- basic shop/tavern affordances.

### Social basics

- ask;
- answer;
- tell;
- report;
- gossip;
- refuse;
- lie;
- recruit;
- accuse/suspect with actor-known basis.

### Epistemics

- direct observation;
- sound observation;
- reading;
- speech report;
- belief store;
- confidence/source;
- expectation contradiction;
- missing-item trace;
- disturbed-container trace;
- stale information;
- rumor chain;
- wrong suspicion.

### Kernel and replay

- event log;
- action pipeline;
- preconditions/effects;
- traces;
- deterministic replay;
- causal inspector;
- snapshots if needed.

### TUI

- embodied play;
- local view;
- current actor status;
- needs/intention panel;
- actor-known beliefs;
- action menu;
- object/storage interface;
- conversation/report interface;
- notice board;
- lead notebook;
- possession switch in debug mode;
- event log/causal graph in debug mode;
- why-not explanations.

## Scenario A — Ordinary day without player

The village runs for at least several simulated days with no human input.

Pass conditions:

- agents sleep, eat, work, visit, talk, and rest;
- needs affect routines;
- shops/offices open and close;
- work interruptions occur only for modeled reasons;
- no event references player identity;
- TUI can attach to any major actor and inspect their current perceived state.

## Scenario B — Missing coins

Mara steals coins from Tomas. Elena may hear uncertain noise. Tomas later checks the strongbox, experiences expectation contradiction, and chooses among search, ask, report, accuse, or conceal depending on belief/motive.

Pass conditions:

- nobody knows Mara stole unless information reaches them;
- Tomas discovers by expectation violation;
- Mara continues autonomously after possession switch;
- event log explains the chain;
- traces are ambiguous;
- Tomas cannot accuse with proof he does not have.

## Scenario C — Body-switch parity

The player steals as Mara, switches to Tomas or Elias.

Pass conditions:

- no player-specific suspicion logic;
- possession does not clear culpability;
- player knowledge does not become actor knowledge;
- current body uses same actions as NPCs;
- previous body retains needs, intentions, possession, risk, and relationships.

## Scenario D — Interrupted routine

A workday is disrupted by a missing tool, theft report, witness summons, hunger/fatigue, locked workplace, office closure, fear, or road threat.

Pass conditions:

- routine changes for modeled reasons;
- closed or interrupted work affects others;
- causal chain is inspectable;
- no hidden script moved the actor.

## Scenario E — Report and record

Tomas or another actor reports a missing item or suspicious event to Anna.

Pass conditions:

- office must be reachable/open or emergency exception modeled;
- report is a speech act;
- clerk receives it;
- incident record is created with claims and provenance;
- institution does not learn ground truth;
- actor may be refused, delayed, or deprioritized for modeled reasons.

## Scenario F — Wrong suspicion

Uncertain testimony, rumor, absence, bias, or poor inference causes Elias or another actor to question/watch the wrong person.

Pass conditions:

- wrong suspicion has visible causes;
- correct culprit is not required;
- suspicion threshold is actor/institution-specific;
- player can hide, reveal, forge, redirect, or exploit evidence only through valid actions.

## Scenario G — Notice lifecycle

A route threat harms someone. A survivor or witness reports. Reeve/Anna create a notice if procedure and resources allow.

Notice states:

```text
drafted -> carried -> posted -> read -> acted_on -> updated/stale/removed/destroyed
```

Pass conditions:

- notice has author, issuer, structured claims, source record, posting event, and last verification;
- notice can be removed, forged, destroyed, ignored, or stale;
- reading it creates actor-known belief/lead;
- TUI shows claims as claims, not truth.

## Scenario H — Stale lead travel

A player-controlled actor reads a notice, talks to locals, recruits Oren or another agent, travels to the alleged site, and finds the situation changed or uncertain.

Pass conditions:

- travel is an event chain;
- companion has independent needs/fear/payment expectations;
- threat truth and public belief can diverge;
- arrival does not force an encounter;
- route risk may manifest as injury/death/aftermath traces without detailed combat;
- proof/payment requires institution verification.

## Scenario I — LLM-safe dialogue mock

The player asks Elena about the night noise. Elena's belief packet permits an uncertain answer. A deterministic mock or template renders surface text. Speech act transfers an uncertain claim to the current actor.

Pass conditions:

- unsupported facts are not transmitted;
- uncertainty is preserved;
- prose does not create truth;
- structured speech act is visible in debug overlay.

## Required metrics

This is the slice-scoped subset of the canonical metrics catalog in `20_TESTING_VALIDATION_AND_DEBUGGING.md`; that document owns the full list and definitions.

Track:

- significant events/day;
- routine events/day;
- causal coverage;
- belief confidence by source;
- stale beliefs;
- contradictions;
- routine interruptions;
- meals/sleep/work completions;
- notices read;
- reports filed;
- cases opened/closed/stale;
- wrong suspicions;
- recruitment attempts;
- travel events;
- replay failures;
- planner time;
- TUI action coverage.

## Definition of done

The slice is done when a player can:

1. enter the village in TUI embodied mode;
2. possess major actors in debug mode;
3. sleep, eat, work, talk, use doors, use containers, and handle possessions;
4. steal or observe theft;
5. see discovery through expectation contradiction;
6. watch a report become a record;
7. watch a route threat become a notice;
8. read a notice as an actor-known source;
9. recruit someone and travel on a stale lead;
10. see wrong suspicion arise without scripting;
11. inspect causal chains in debug mode;
12. replay major consequences;
13. run the village without human input.

If ordinary life is shallow, the slice fails even if the notice expedition works.
