# Foundational Invariants

These are hard rules. They are not preferences, moods, or phase-specific aspirations. If Tracewake violates them routinely, it stops being Tracewake.

## TUI and playability

### INV-001 — The TUI is a primary product interface

The TUI is not a throwaway debug shell. It must remain playable, legible, and architecturally reusable.

### INV-002 — Every runnable phase has a playable TUI gate

A phase is not complete unless the new mechanic can be reached, used, inspected, and regression-tested through the TUI.

### INV-003 — Mechanics hidden from play are not progress

A system that exists only in headless tests is unfinished unless the phase is explicitly pre-runnable ontology work. Once the kernel can run, playability and inspectability advance together.

### INV-004 — Embodied mode preserves actor knowledge

Normal play shows the current actor's perceived world, not ground truth. Debug mode may reveal truth, event graphs, hidden beliefs, and planner state, but it must be visibly separate.

## Player ontology

### INV-005 — No sacred player entity

The authoritative simulation must not contain a metaphysically special player character.

Allowed:

```text
HumanController -> ActorId
```

Forbidden:

```text
World.GenerateQuestFor(PlayerCharacter)
Guard.KnowsCrimesOf(PlayerCharacter)
NPCs.IgnoreNeedsNearPlayer()
Threat.SpawnBecausePlayerIsBored()
Reward.SpawnForPlayerCompletion()
```

### INV-006 — Possession changes input, not reality

Switching bodies changes only the controller binding. The previous body remains an ordinary agent with beliefs, needs, intentions, injuries, relationships, possessions, obligations, suspicions, memories, and plans.

### INV-007 — Body-switching is foundational and debug-first

Possession is currently a testing/debugging tool and an architectural proof of actor parity. It may later become diegetic in a domain pack, but the kernel must not depend on possession metaphysics.

### INV-008 — Player knowledge is not actor knowledge

The human may remember what happened while controlling another actor. The current actor may not assert, accuse, testify, claim payment, persuade, report, or institutionally act from facts that have not reached that actor through modeled channels.

### INV-009 — Every world-affecting player action must be NPC-possible

UI affordances may help the human understand options, but every committed world action must correspond to an action a non-player agent could attempt under equivalent physical, social, epistemic, resource, and institutional conditions.

### INV-010 — UI assistance is not world privilege

Action lists, why-not explanations, sorting, notebooks, maps, filters, and debug overlays may clarify. They must not bypass preconditions, create facts, grant unearned knowledge, or alter institutional recognition.

## Causality and events

### INV-011 — Meaningful state changes require events

If a change can affect gameplay, history, belief, traces, reputation, law, economy, memory, UI leads, replay, or later reasoning, it must be represented by one or more events.

### INV-012 — Every event needs a cause model

Events must be explainable through preceding state, actor intention, need pressure, belief, routine, environmental process, institutional procedure, regional process, authored initial condition, or explicit exogenous boundary input.

### INV-013 — Events leave traces

Every meaningful event leaves physical, mental, social, institutional, economic, environmental, spatial, procedural, or erasure traces. A clean crime includes successful trace-removal events.

### INV-014 — Failure is an event

Failed attempts matter. Refused reports, interrupted routines, missed meetings, failed searches, botched accusations, failed payments, failed travel, rejected speech acts, and invalid plans produce events when they can affect future reasoning.

### INV-015 — Absence can be evidence

A missing item, absent guard, closed shop, unposted notice, quiet road, uncollected debt, empty bed, unlit hearth, or missing witness can be evidence if an agent expected otherwise.

### INV-016 — Simulation first, narration second

No prose, dialogue, notice text, UI summary, story sift, or LLM output defines authoritative reality. Text renders or proposes structured state; it does not create truth.

### INV-017 — Replay is a design requirement

The engine must support reconstruction, audit, causal debugging, and temporal query for significant events. Determinism is not optional theater; it is a foundation for trust.

## Epistemics

### INV-018 — Ground truth, subjective belief, and public record are separate

The engine must distinguish what happened, what each agent believes, and what an institution, group, ledger, rumor network, or public artifact treats as known.

### INV-019 — No telepathy

Information reaches agents only through modeled channels: perception, testimony, rumor, reading, records, inference, instruction, memory, domain-defined special senses, or institutional notification.

### INV-020 — Wrong beliefs are first-class state

Mistakes, stale information, false rumors, lies, confabulations, misidentifications, biased inferences, contradictory hypotheses, and partial memories are required mechanics.

### INV-021 — Important beliefs need provenance

Important beliefs record holder, proposition, confidence, source, channel, acquisition time, believed event time when available, and contradiction links.

### INV-022 — Discovery requires expectation or search

An agent cannot notice coins are missing unless the agent expected coins to be present, had reason to check, or searched the relevant place.

### INV-023 — Staleness is not automatically corrected

Notices, maps, reports, rumors, memories, and institutional records remain stale until updated through modeled channels.

## Agents

### INV-024 — Agents are symbolic and inspectable in v1

V1 agents use explicit beliefs, values, needs, goals, projects, intentions, HTN methods, bounded local planning, budgets, and failure events. Their decisions must be explainable.

### INV-025 — Agents need durable intentions

Agents need commitments that persist across small fluctuations and can be interrupted by stronger modeled causes. Avoid utility jitter.

### INV-026 — Routines are defeasible intentions

A blacksmith works because she believes it is a workday, has obligations, needs income, can reach the forge, has tools, and has no stronger interruption. Schedules are not teleports.

### INV-027 — Needs are pressures, not puppet strings

Hunger, fatigue, safety, warmth, shame, greed, loyalty, affection, duty, ambition, fear, and curiosity influence choice without flattening characters into meter machines.

### INV-028 — Competence must be bounded

Agents should be resourceful enough to be interesting but limited by beliefs, skill, cost, time, risk, social position, resources, and planner budget.

### INV-029 — No LLM brains

LLMs may not be authoritative agent planners in v1. They may later render language, paraphrase structured records, or parse proposed speech acts behind validation.

## Institutions and norms

### INV-030 — Institutions are not omniscient

Authorities, guilds, households, markets, temples, gangs, and offices act from reports, records, testimony, observations, procedures, jurisdiction, manpower, budgets, delay, fear, bias, corruption, and relationships.

### INV-031 — Law is a social machine

A norm violation becomes actionable through observation, trace, report, evidence, record, investigation, institutional belief, or procedure. Violation and detection are separate.

### INV-032 — Norms are explicit

Obligations, permissions, prohibitions, constitutive norms, roles, sanctions, and institutional facts must be represented explicitly enough to inspect and test.

### INV-033 — Records are world artifacts

Bounties, warrants, ledgers, contracts, tax records, case files, orders, and notices have authors, issuers, custody, timestamps, claims, and provenance.

### INV-034 — Procedure can fail

The law may ignore the poor, protect the powerful, lose a report, run out of funds, arrest the wrong person, fail to pay, or never learn the relevant fact.

## Questlessness and story

### INV-035 — No quest as primary data type

The engine stores incidents, needs, requests, contracts, promises, obligations, notices, rumors, records, threats, opportunities, investigations, sanctions, and leads.

### INV-036 — Tasks are projections

A player-facing objective or notebook card is a view over source-bound world state. It is not the process itself.

### INV-037 — Rewards require custody and verification

Payment depends on who promised it, whether funds exist, what proof is accepted, who verifies it, whether the payer remains able and willing, and whether anyone disputes the claim.

### INV-038 — No authored outcome chains

Forbidden: quest beats, scenario scripts, drama directors, boredom detectors, “when player does X, make Y happen,” NPC behavior that exists only to serve the player, content that waits indefinitely, and objective markers to ground truth.

### INV-039 — Story sifting may curate only

Story recognition, salience, recaps, notebooks, and event summaries may highlight what happened. They may not cause events or repair pacing.

## Ordinary life and scope

### INV-040 — Ordinary life comes before adventure

Sleep, eating, work, homes, storage, ownership, doors, rooms, routines, needs, basic social interaction, belief, traces, and TUI embodied play are earlier than expeditions, investigation depth, combat, magic, worldgen, or graphics.

### INV-041 — No detailed combat in the first serious vertical slice

Early threats may appear through travel risk, injury/death events, aftermath traces, fear, reports, route avoidance, and institutional response. Detailed combat is deferred.

### INV-042 — Start small enough to inspect

A tiny village with deep causality is better than a large opaque region.

### INV-043 — The world continues without the human

No-player simulations must produce ordinary life, reports, wrong beliefs, stale records, trade, routine interruption, social propagation, and institutional action or failure.

## Domain boundary

### INV-044 — No fantasy leakage into the kernel

The core must not assume monsters, magic, species, feudal law, medieval technology, divine authority, adventurer classes, or combat categories.

### INV-045 — Domain packs own flavor

Domain packs own species/body types, technology, magic/metaphysics, threats, domain institutions, occupations, items, affordances, unique traces, diseases, speech/culture tags, terrain/place types, scenario seeds, and domain-specific laws.

### INV-046 — Special powers are causal channels

Magic, sensors, dreams, psychic impressions, radios, rituals, or alien technology are modeled as channels or actions with costs, reliability, distortion, provenance, traces, counters, and failure modes.

## LOD and scale

### INV-047 — Scale through honest abstraction

Low-fidelity processes produce inspectable summary events and preserve causal ancestry. They do not hide a director.

### INV-048 — Low-LOD agents are still people

Lower detail is dormant or summarized detail, not permission to turn citizens into props.

### INV-049 — Promotion is relevance-based, not pacing-based

Promote simulation detail when an event becomes causally, spatially, institutional, social, or debug-relevant. Never promote because the player is bored.

## LLM boundary

### INV-050 — LLMs may not mutate authoritative state directly

Any state change implied by language must be represented as a typed, validated action or speech act.

### INV-051 — LLMs may not invent hidden evidence or ground truth

No LLM output may determine guilt, create a clue, correct stale information, bypass preconditions, create a quest, or grant NPCs unearned knowledge.

### INV-052 — Core mechanics must work with LLMs disabled

Tests use deterministic mocks. LLM calls are optional surfaces, not dependencies.

## Review rule

Any proposed feature must answer:

```text
What caused it?
Who knows it?
How can they be wrong?
What traces exist?
What institution or norm cares?
Can an NPC do the same kind of thing?
Can it be played and inspected through the TUI?
Does it leak player privilege, ground truth, genre assumptions, scripting, or LLM authority?
```
