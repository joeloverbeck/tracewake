# Vertical Slice Specification: The Village That Notices

## Goal

Build a small playable TUI simulation proving the thesis:

> People live ordinary lives, act from beliefs, produce institutional artifacts, respond to disruptions, and can be possessed by the player without the world knowing who the player is.

This slice is about causal credibility, not graphics, combat depth, terrain, or open-ended LLM chat.

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
- old quarry / threat site / bandit camp;
- farm or delivery point.

## Population

30–60 agents.

Required named actors: Tomas the miller, Elena uncertain witness, Mara thief/desperate worker, Anna clerk, Elias guard, Oren adventurer/traveler, Reeve authority, and one bandit group or creature threat.

## Required systems

World kernel: entities/components, rooms/places/routes, event log, preconditions/effects, traces, deterministic replay.

Agent architecture: hunger/fatigue/safety, beliefs with source/confidence, current intention, one project for major actors, simple relationships, interruptible routines, bounded planning.

Information: observation, overheard sound, speech report, rumor chain, written notice, stale information, belief contradiction, lie speech act.

Institutions: report intake, incident ledger, bounty creation, notice writing/posting, guard assignment, investigation, proof/payment.

Player/TUI: possession, actor/player knowledge distinction, embodied view, action menu, conversation/report interface, notice board, lead notebook, debug causal inspector.

## Scenario A — Missing coins

Mara steals coins from Tomas. Elena may hear uncertain noise. Tomas later checks, contradicts his belief, and searches, asks, reports, accuses, or conceals.

Pass: nobody knows Mara stole unless information reaches them; Tomas discovers by expectation violation; event log explains chain; Mara continues autonomously after possession switch.

## Scenario B — Stale notice expedition

A threat harms someone. Survivor reports. Reeve posts notice. Player reads it, talks to locals, recruits Oren or another adventurer, travels, and finds the situation changed.

Pass: notice has author/issuer/posting/structured claims; threat truth and public belief diverge; companion has independent needs/fear/payment expectations; stale notice is legible.

## Scenario C — Wrong suspect

Uncertain testimony and rumor cause Elias to question or watch the wrong person.

Pass: wrong suspicion has visible causes; correct culprit is not required; player can hide, reveal, forge, redirect, or exploit evidence.

## Scenario D — Player body switch

Player steals as Mara, switches to Tomas or Elias.

Pass: no player-specific suspicion logic; possession does not clear culpability; actor knowledge and player knowledge remain distinct.

## Scenario E — Interrupted routine

A workday is disrupted by missing tool, theft report, witness summons, road threat, hunger/fatigue, locked workplace, or recruitment.

Pass: routine changes for modeled reasons; closed shop affects others; consequences propagate.

## Scenario F — Notice lifecycle

States:

```text
drafted -> carried -> posted -> read -> acted_on -> updated/stale/removed/destroyed
```

Pass: notice can be removed or forged; destruction affects who learns; authority may repost if it notices; text derives from structured claims.

## Scenario G — LLM-safe dialogue mock

Player asks Elena about night noise. Elena’s belief packet permits uncertain answer. Speech act transfers uncertain claim to current actor. TUI records actor-known hearsay.

Pass: unsupported facts are not transmitted; uncertainty preserved; prose does not create truth.

## Metrics

Significant events/day, causal coverage, belief confidence by source, stale beliefs, contradictions, routine interruptions, notices read, reports filed, cases opened/closed/stale, wrong accusations, recruitment attempts, travel events, replay failures, planner time.

## Definition of done

The slice is done when a player can enter village in TUI embodied mode, possess major actors, steal or observe theft, see discovery through contradiction, watch report become record, watch threat become notice, recruit someone, pursue a stale lead, inspect causal chain in debug mode, produce wrong suspicion without scripting, and replay major consequences.
