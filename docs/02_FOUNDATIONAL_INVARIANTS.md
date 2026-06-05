# Foundational Invariants

These are not preferences. They are guardrails. If the project violates them often, it will become a conventional RPG, colony sim, or procedural quest system.

## Player ontology

### INV-001 — No sacred player entity

The simulation must not contain an ontologically special `PlayerCharacter` whose identity changes world laws.

Allowed:

```text
HumanController -> ActorId
```

Forbidden:

```text
World.GenerateQuestFor(PlayerCharacter)
Guard.KnowsCrimesOf(PlayerCharacter)
NPCs.IgnoreNeedsNearPlayer()
```

### INV-002 — Possession changes input, not reality

When the human switches to another actor, only the controller binding changes. The previous body remains an agent with beliefs, needs, intentions, injuries, relationships, possessions, obligations, and plans.

### INV-003 — Every player action must be NPC-possible

The user interface may be clearer for the human, but every player action must correspond to an action available to some non-player agent under equivalent conditions.

If the player can steal, accuse, read, lie, burn a notice, forge a record, or search a room, some NPC must be able to do it too.

### INV-004 — Player knowledge is not actor knowledge

The human may know things that the possessed actor does not. The actor’s speech, accusations, institutional credibility, and planning should depend on actor knowledge unless the design explicitly allows “metagame possession leakage” as a special mode.

Default: no leakage.

## Causality

### INV-005 — All meaningful state changes require events

If state changes matter to gameplay, history, belief, evidence, reputation, law, economy, or later reasoning, the change must be recorded as an event.

### INV-006 — Every event needs a cause model

An event must be explainable in terms of preceding state, actor intention, environmental process, institutional procedure, or exogenous world process.

Forbidden: “because the player entered the region” unless the player’s entry is itself a modeled cause observed by relevant systems.

### INV-007 — Events leave traces

Every meaningful event leaves at least one of:

- physical trace;
- mental trace;
- social trace;
- institutional trace;
- economic trace;
- environmental trace;
- deliberately erased trace.

A clean crime is not traceless; it includes successful trace removal events.

### INV-008 — Absence can be evidence

A missing item, missing person, absent guard, broken routine, unposted notice, uncollected tax, or quiet road can be evidence if an agent expected otherwise.

### INV-009 — Failure is an event

Failed actions must often be recorded: failed lockpicking, failed search, refused report, botched accusation, unanswered knock, interrupted routine, rejected plan.

Failure events are crucial because they alter beliefs, relationships, and future choices.

### INV-010 — Simulation first, narration second

No prose, dialogue, quest text, UI card, or LLM output may be the source of authoritative reality. Text describes structured world state; it does not define it.

## Belief and information

### INV-011 — Ground truth and belief are separate

World truth is not agent truth. Agent decisions must query belief state, not authoritative state.

### INV-012 — No telepathy

An agent may only know information that reached them through a modeled channel: perception, testimony, rumor, record, inference, instruction, magic, or memory.

### INV-013 — Wrong beliefs are valid simulation state

Mistakes, stale reports, false rumors, confabulations, lies, bad inference, and misidentification are not bugs. They are required features.

### INV-014 — Beliefs need provenance

Important beliefs should record source, acquisition time, believed event time, confidence, and whether the belief came from perception, testimony, rumor, record, inference, or instruction.

### INV-015 — Belief contradiction must be possible

Agents can hold contradictory beliefs, or unresolved competing hypotheses, until something forces revision or one belief decays.

### INV-016 — Agents need expectations

Discovery depends on violated expectations. A character cannot “notice coins are missing” unless they previously expected coins to be present or had reason to check.

## Agents

### INV-017 — Agents need durable intentions

Agents should not merely choose a maximum-utility action every tick. They must form commitments that persist across small fluctuations, while remaining interruptible by stronger causes.

### INV-018 — Needs do not equal personality

Hunger, fatigue, fear, greed, loyalty, ambition, shame, duty, and curiosity should feed decisions, but not flatten characters into need meters.

### INV-019 — Routines are defeasible

A blacksmith does not go to work because the clock says so. She goes because she believes it is a workday, has obligations, needs money, has materials, is able to travel, and lacks stronger interruptions.

### INV-020 — Agents must be allowed to ignore the player

Most citizens should continue living lives unaffected by the player most of the time.

## Institutions

### INV-021 — Institutions are not omniscient

Authorities act from reports, records, testimony, procedures, jurisdiction, manpower, politics, and bias.

### INV-022 — Law is a social machine

Crime is not detected by morality fields. A violation becomes actionable when observed, reported, recorded, investigated, believed, or institutionally inferred.

### INV-023 — Records are world artifacts

Bounties, warrants, ledgers, contracts, tax records, court notes, guild orders, and public notices must be simulated artifacts or entries with provenance.

### INV-024 — Institutions have delay

Reports wait. Clerks sleep. Guards are busy. Budgets run out. Messengers get lost. A stale notice is a success case, not an error.

### INV-025 — Procedure can fail

The law may ignore the poor, protect the powerful, arrest the wrong person, lose a report, underfund a patrol, or post an outdated bounty.

## Questlessness

### INV-026 — No quest as primary data type

The engine must not store “quest” as the foundational object. Use incidents, needs, requests, obligations, contracts, rumors, threats, opportunities, and records.

### INV-027 — A task is a projection

A player-facing objective is a view over a world process, not the process itself.

### INV-028 — Quest completion is not authoritative

The world decides whether a problem is solved. The UI can summarize likely satisfaction conditions, but resolution belongs to institutions, victims, records, and consequences.

### INV-029 — Rewards require custody and verification

Payment should depend on who promised it, where the funds are, what proof is accepted, whether the payer still believes the conditions were met, and whether anyone contests the claim.

## UI and salience

### INV-030 — The UI may curate, not author

The UI can highlight interesting patterns. It cannot cause murders, attacks, discoveries, or coincidences because the player needs content.

### INV-031 — Debuggability is a design feature

The causal graph, event log, belief inspector, and institutional record viewer are not merely developer tools. They are prototypes of player-facing pleasures.

### INV-032 — No objective markers by default

The game should not point to the true bandit camp unless the actor has a reason to know it. Markers should represent beliefs, reports, rumors, maps, or records.

## LLM use

### INV-033 — LLMs may render only validated facts

LLMs can write natural-language dialogue or notices from structured data. They must not invent facts that become true without validation.

### INV-034 — LLM outputs need grounding

Every generated line that communicates actionable information should map back to structured propositions or known lies.

### INV-035 — Language generation must be lossy-safe

If a generated rumor is ambiguous, the ambiguity should not corrupt world state. Structured belief payloads remain authoritative.

## Scope

### INV-036 — Start small enough to inspect

If a bug occurs, developers should be able to replay and inspect it. A tiny village with deep causality is better than a huge opaque world.

### INV-037 — Do not simulate everything equally

The project is causality-first, not maximum-fidelity-first. Simulate details deeply only where they support belief, consequence, institution, and ordinary life.

### INV-038 — Interesting unfairness is allowed; opaque unfairness is not

A character may be punished unfairly by a corrupt or mistaken institution. The system should still be explainable.

### INV-039 — Emergence does not excuse nonsense

“It emerged” is not a defense if there is no intelligible causal chain. Emergence must be inspectable.

### INV-040 — The world should be able to continue without the human

Run the simulation without player input. If nothing interesting, meaningful, or socially consequential can happen, the foundation is not yet alive.
