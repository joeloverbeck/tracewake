# Vertical Slice Specification: The Village That Notices

## Goal

Build a small simulation that proves the core thesis:

> People live ordinary lives, act from beliefs, produce institutional artifacts, and respond to disruptions without knowing who the player is.

This slice is not about combat feel, art, map scale, or prose quality. It is about causal credibility.

## Map

A single village and nearby road.

Required places:

- 8–12 homes;
- mill or workshop;
- tavern;
- market square;
- authority office;
- notice board;
- small jail or holding room optional;
- north road;
- bandit camp or hideout;
- one farm or delivery point.

## Population

30–80 agents.

Named required actors:

### Tomas

- miller;
- owns a strongbox with coins;
- spouse or close household member Elena;
- moderate trust in authority;
- high property concern.

### Elena

- Tomas’s spouse or household member;
- hears a faint noise during theft;
- uncertain witness.

### Mara

- thief or desperate worker;
- motive to steal coins;
- can lie;
- has routine that gives opportunity.

### Anna

- clerk at reeve’s office;
- writes reports and notices;
- has office hours and fatigue.

### Elias

- guard;
- investigates reports;
- fallible and time-limited.

### Oren

- adventurer/traveler;
- reads notices;
- may pursue stale bounty.

### Reeve

- authority decision-maker;
- allocates bounty funds;
- biased toward preserving trade.

### Bandit group

- 3–6 agents;
- attacks road travelers;
- can be killed, flee, or move;
- does not exist only for the player.

## Required systems

### World kernel

- entities/components;
- rooms/paths;
- event log;
- action preconditions/effects;
- trace generation;
- deterministic replay target.

### Agent cognition

- needs: hunger, fatigue, safety;
- beliefs with source/confidence;
- current intention;
- simple relationships;
- routines as interruptible intentions;
- local planning for movement, objects, and speech.

### Information

- direct observation;
- overheard noise;
- speech report;
- rumor chain;
- written notice;
- stale information;
- belief contradiction.

### Institutions

- report intake;
- incident ledger;
- bounty creation;
- notice writing/posting;
- guard assignment;
- basic investigation;
- proof/payment handling.

### Player

- possession switching;
- actor/player knowledge distinction;
- lead notebook;
- causal inspector;
- belief inspector.

## Scenario A — Missing coins

### Setup

- Tomas stores coins in a strongbox.
- Tomas believes coins are in the strongbox.
- Mara believes Tomas has coins and believes access risk is acceptable.

### Expected chain

1. Mara enters house under plausible conditions.
2. Mara opens/forces/accesses strongbox.
3. Mara removes coins.
4. Event creates traces.
5. Elena may hear a noise with low confidence.
6. Tomas later checks strongbox.
7. Tomas observes absence and contradicts belief.
8. Tomas forms belief: coins missing.
9. Tomas chooses one of: search, ask Elena, report, accuse, conceal.
10. Later consequences proceed from his choice.

### Pass criteria

- No one knows Mara stole unless information reaches them.
- Tomas discovers theft through expectation violation.
- The event log explains the chain.
- The player can switch to Mara after theft and Mara continues normally or anxiously.

## Scenario B — Stale bounty

### Setup

- Bandits attack a traveler.
- Survivor reaches village and reports.
- Reeve decides to post bounty.

### Expected chain

1. Report is recorded.
2. Reeve assesses threat and reserves funds.
3. Anna writes notice.
4. Notice is posted physically.
5. Oren reads notice and forms intention to investigate.
6. Bandits are killed or leave before Oren arrives.
7. Notice remains until update reaches office.

### Pass criteria

- Notice has author, issuer, posting event, and structured claims.
- Threat truth and authority belief can diverge.
- Oren can act on stale information.
- Stale bounty is legible, not silently corrected.

## Scenario C — Wrong suspect

### Setup

- Elena hears noise.
- Mara is seen near street earlier.
- Oren has poor reputation or was nearby.

### Expected chain

1. Witness statements contain uncertainty.
2. Rumor mutates at tavern.
3. Guard weights testimony through trust/status/bias.
4. Wrong suspect may be questioned or watched.
5. True culprit remains free unless evidence emerges.

### Pass criteria

- Wrong suspicion has visible causes.
- The system does not require correct culprit identification.
- Player can intervene by hiding, revealing, forging, or redirecting evidence.

## Scenario D — Player body switch

### Setup

- Player controls Mara and steals coins.
- Player switches to Tomas or guard.

### Expected chain

- Mara continues as an autonomous agent.
- Tomas/guard do not inherit player knowledge.
- Player can see consequences from another perspective.
- If the player later controls Mara again, Mara’s beliefs and risks have evolved.

### Pass criteria

- No `PlayerCharacter`-specific suspicion logic.
- Possession does not clear culpability.
- Actor knowledge and player knowledge remain distinguishable.

## Scenario E — Interrupted routine

### Setup

- Blacksmith or miller has normal workday.

### Interruptions

- missing tool;
- reported theft;
- guard summons witness;
- bandit threat closes road;
- hunger/fatigue;
- locked workplace.

### Pass criteria

- Routine changes for modeled reasons.
- Closed shop affects other agents.
- Consequences propagate.

## Scenario F — Notice lifecycle

### Required notice states

```text
drafted -> carried -> posted -> read -> acted_on -> updated/stale/removed/destroyed
```

### Pass criteria

- Notice can be torn down or forged.
- Destroying notice affects who learns about bounty.
- Authority may repost if it notices removal.
- Notice text is generated from structured claims.

## Metrics

Track:

- number of significant events per simulated day;
- percentage of events with causal parents;
- average belief confidence by source type;
- number of stale beliefs;
- number of contradiction detections;
- number of routine interruptions;
- notice reads per day;
- reports filed;
- cases opened/closed/stale;
- wrong accusations;
- replay determinism failures;
- planner time per actor.

## Debug tools required

- event timeline;
- causal graph viewer;
- actor belief viewer;
- institution ledger viewer;
- trace viewer by location;
- possession history;
- stale knowledge report;
- “why did actor choose this?” view;
- “why is action unavailable?” view.

## Out of scope

- world generation beyond one hand-authored village;
- complex combat;
- deep economy;
- marriage/inheritance;
- multi-town politics;
- full natural-language dialogue;
- magic;
- procedural terrain;
- hundreds of item types;
- rich animation.

## Definition of done

The vertical slice is done when a player can:

1. watch village life run without input;
2. possess any major actor;
3. steal or observe theft;
4. see discovery through belief contradiction;
5. watch a report become a record;
6. watch a threat become a bounty notice;
7. pursue a stale notice;
8. inspect the causal chain;
9. produce at least one wrong suspicion without scripting;
10. replay the simulation and explain every major consequence.
