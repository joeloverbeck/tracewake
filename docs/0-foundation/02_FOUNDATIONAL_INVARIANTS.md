# Foundational Invariants

These are hard rules. If Tracewake violates them often, it becomes a conventional RPG, colony sim, procedural quest system, or LLM fiction toy.

## Player ontology

### INV-001 — No sacred player entity

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
```

### INV-002 — Possession changes input, not reality

Switching bodies changes only the controller binding. The previous body remains an agent with beliefs, needs, intentions, injuries, relationships, possessions, obligations, suspicions, and plans.

### INV-003 — Every player action must be NPC-possible

The UI may help the human understand options, but every world-affecting action must correspond to an action a non-player agent could attempt under equivalent physical, social, knowledge, and resource conditions.

### INV-004 — Player knowledge is not actor knowledge

The human may know facts the current actor does not. The actor may not assert, accuse, testify, claim reward, persuade, or institutionally act from facts that have not reached that actor through modeled channels.

### INV-005 — UI assistance is not world privilege

Action lists, why-not explanations, sorting, and notebooks are allowed if final actions still pass the same action pipeline used by NPCs.

## Causality

### INV-006 — Meaningful state changes require events

If a change can affect gameplay, history, belief, traces, reputation, law, economy, or later reasoning, it must be represented by events.

### INV-007 — Every event needs a cause model

Events must be explainable through preceding state, actor intention, environmental process, institutional procedure, regional process, or exogenous boundary input.

### INV-008 — Events leave traces

Every meaningful event leaves physical, mental, social, institutional, economic, environmental, spatial, procedural, or erased traces. A clean crime includes successful trace-removal events.

### INV-009 — Absence can be evidence

A missing item, absent guard, closed shop, unposted notice, quiet road, uncollected tax, or empty bed can be evidence if an agent expected otherwise.

### INV-010 — Failure is an event

Failed attempts often matter: refused reports, interrupted routines, missed meetings, botched accusations, failed searches, failed payments, or failed travel.

### INV-011 — Simulation first, narration second

No prose, dialogue, notice text, UI summary, or LLM output may define authoritative world reality.

## Belief and information

### INV-012 — Ground truth and belief are separate

Agents act from subjective beliefs, not omniscient state.

### INV-013 — No telepathy

Information reaches agents only through modeled channels: perception, testimony, rumor, reading, records, inference, instruction, memory, magic, sensors, or institutional notification.

### INV-014 — Wrong beliefs are first-class state

Mistakes, stale information, false rumors, lies, confabulations, misidentifications, biased inferences, and contradictory hypotheses are required features.

### INV-015 — Beliefs need provenance

Important beliefs record source, acquisition time, believed event time, confidence, and channel.

### INV-016 — Discovery requires expectations

An agent cannot notice coins are missing unless the agent expected coins to be present or had reason to check.

## Agents

### INV-017 — Agents need durable intentions

Agents need commitments that persist across small fluctuations and can be interrupted by stronger causes.

### INV-018 — Needs are not personality

Hunger, fatigue, safety, shame, greed, loyalty, affection, duty, and ambition influence choices but must not flatten characters into meter machines.

### INV-019 — Routines are defeasible intentions

A blacksmith works because she believes it is a workday, has obligations, needs income, has tools, can reach the forge, and has no stronger interruption.

### INV-020 — Competence must be bounded and inspectable

Agents should be resourceful and humanly competent, but every plan must be explainable through beliefs, goals, methods, resources, and time.

## Institutions

### INV-021 — Institutions are not omniscient

Authorities act from reports, records, testimony, procedures, jurisdiction, manpower, budgets, delay, politics, fear, bias, and corruption.

### INV-022 — Law is a social machine

A norm violation becomes actionable through observation, report, evidence, record, investigation, institutional belief, or procedure.

### INV-023 — Records are world artifacts

Bounties, warrants, ledgers, contracts, tax records, case files, and notices must have provenance.

### INV-024 — Procedure can fail

The law may ignore the poor, protect the powerful, lose a report, arrest the wrong person, or fail to pay because reserves were looted.

## Questlessness

### INV-025 — No quest as primary data type

Store incidents, needs, requests, contracts, obligations, notices, rumors, threats, opportunities, records, promises, and investigations.

### INV-026 — Tasks are projections

A player-facing objective is a view over a world process, not the process itself.

### INV-027 — Rewards require custody and verification

Payment depends on who promised it, whether funds exist, what proof is accepted, who verifies it, and whether someone contests the claim.

## UI and salience

### INV-028 — UI may curate, not author

The UI can highlight accessible patterns. It cannot cause events because the player needs content.

### INV-029 — No boredom detector

The simulation must not inject incidents because pacing feels low. Exogenous events must come from modeled weather, ecology, migration, disease, politics, trade, or outside-region processes.

### INV-030 — No objective markers to truth

Markers represent beliefs, reports, rumors, maps, records, or observations.

## LLM use

### INV-031 — LLMs may not mutate authoritative state directly

Any state change implied by language must be represented as a typed, validated action or speech act.

### INV-032 — LLM dialogue must be grounded

Outputs must map to structured propositions, questions, lies, refusals, promises, threats, or other speech acts before they affect beliefs.

## Scope

### INV-033 — Start small enough to inspect

A tiny village with deep causality is better than a huge opaque region.

### INV-034 — Scale through LOD, not dishonesty

Low-fidelity processes produce inspectable summary events and can be promoted when relevant.

### INV-035 — Long simulation must preserve causal ancestry

Simulating years may use abstraction, but important current-state facts must retain enough ancestry to interrogate.

### INV-036 — The world continues without the human

No-player simulations must produce ordinary life, incidents, reports, wrong beliefs, stale records, migrations, trades, and disruptions.
