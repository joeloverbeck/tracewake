# Project Charter

## Name

**Tracewake**.

The name is doctrine. Actions leave wakes. Wakes become traces. Traces are observed, ignored, misunderstood, recorded, erased, remembered, forgotten, lied about, institutionalized, and acted upon by ordinary agents.

## One-sentence definition

Tracewake is a causality-first, epistemic, TUI-first ordinary-life simulation where ordinary agents live from needs, routines, beliefs, memory, records, institutions, households, traces, work, travel, weather, regional pressures, and long histories; the human temporarily controls one ordinary actor without becoming a privileged world entity.

## Product priority

Tracewake is built in this order:

1. Playable TUI simulation.
2. Replayable causal simulation engine.
3. Robust ordinary-life social substrate.
4. Larger regions, domain packs, and richer presentation.
5. Optional natural-language richness.
6. Future graphical clients.

A graphical shell without a playable causal village is failure. A research engine that cannot be played and inspected through the TUI is unfinished. An impressive language demo that lets prose invent facts is not Tracewake.

## First target

The first serious implementation target is a neutral medieval-ish ordinary-life village without magic.

The early village should contain roughly 10-30 high-detail agents, or at most a few dozen, with homes, rooms, doors, containers, food, sleep, hunger, fatigue, work, wages/payment, money custody, ownership/access claims, households, local norms, simple speech, reports, rumors, lying, search, records, a notice board, a clerk/reeve/local authority, and travel within or near the village.

The first target is not:

- a combat prototype;
- an expedition game;
- a road-threat bounty loop;
- a quest board;
- an omniscient detective UI;
- a radiant quest generator;
- a drama-directed colony sim;
- an LLM NPC chatbot;
- a graphics-first client;
- a large procedural region before the village works.

Ordinary life comes before adventure because adventure is only meaningful when ordinary life exists to be disrupted.

## Hard kernel identity

Tracewake's authoritative world is not a story manager, UI state, a prose transcript, or an LLM answer. It is a deterministic causal simulation that commits meaningful events, preserves traces and ancestry, derives projections, carries subjective beliefs through modeled channels, and can replay/debug significant outcomes.

The world must run coherently with no human controller attached.

## No sacred player

The authoritative simulation contains no metaphysically special player character.

Allowed:

```text
HumanController -> ActorId
```

Possession changes input binding, not reality. The previous body remains an ordinary agent with beliefs, needs, intentions, relationships, obligations, memories, possessions, suspicions, risks, and plans.

Every world-affecting player action must be possible for an ordinary non-player actor under equivalent physical, social, epistemic, resource, skill, risk, and institutional conditions. UI convenience may clarify. It may not grant authority.

## Product pillars

### Causality before drama

Events happen because prior state, actor intention, need pressure, belief, routine, affordance, environment, institution, household, regional process, authored initial condition, or declared boundary input made them possible.

Forbidden causes include boredom, pacing, quest acceptance, objective completion, LLM prose, player proximity, and dramatic need.

### Belief before truth

Agents act from subjective beliefs, memories, observations, testimony, rumors, records, inferences, expectations, and misinterpretations. They may be wrong, stale, confident, contradictory, biased, deceived, or deceiving.

Ground truth does not automatically correct minds, records, notices, or institutions.

### Ordinary life before adventure

Agents sleep, eat, work, travel, store things, lose things, search, talk, lie, remember, forget, report, cooperate, refuse, and pursue goals from their own beliefs. Survival, storage, routine, household membership, property, social obligation, work, and travel are the substrate, not flavor.

### Typed claims before prose

Typed claims/propositions are the core epistemic currency. The same substrate supports observations, beliefs, memories, testimony, lies, rumors, gossip, accusations, reports, notices, ledgers, ownership claims, debts, contracts, promises, institutional facts, household knowledge, actor-known leads, and source-bound notebook entries.

Raw prose may render claims. Raw prose must not be authoritative state.

### Institutions are fallible social machines

Institutions act through people, roles, records, procedures, norms, resources, delays, bias, jurisdiction, custody, authority, fear, corruption, absence, and failure. No clerk, reeve, guard, household, guild, court, office, ledger, notice board, or future institution reads ground truth.

### Records are artifacts, not UI flags

Records, notices, ledgers, letters, reports, warrants, debts, contracts, promises, accusations, rumors, and bounty letters are world artifacts or institutional/social state. After simulation start, they exist only because an actor, household, institution, or regional process had reason and means to create, carry, post, file, copy, alter, damage, lose, or destroy them.

A notice board is not a quest menu.

### Quests are not ontology

Tracewake stores incidents, requests, contracts, obligations, promises, records, rumors, notices, leads, debts, sanctions, opportunities, suspicions, reports, and institutional procedures.

A player-facing objective is a projection over source-bound actor-known world state. It is not an authoritative process. Embodied mode has no culprit labels, ground-truth targets, completion flags, or guaranteed rewards.

### Story is observed, not directed

Story sifting, salience scoring, recaps, notebooks, summaries, and debug explanations may help a human perceive what happened. They may not cause what happens.

### Symbolic, inspectable agents first

V1 agents are explicit symbolic agents with BDI-style separation: beliefs; desires/values/goals/needs/duties; and intentions. They use durable intentions/projects, HTN-style routines and procedures, bounded local planning for concrete means, bounded utility heuristics for choosing among explicit options, event-driven replanning at meaningful interruption points, and debug-inspectable decision traces.

LLMs are not agent brains.

### TUI-first, playable always

The TUI is the first real client. Every runnable phase must expose new mechanics through the TUI or the same actor-filtered view models, provide why-not explanations, preserve embodied/debug separation, and support automated view-model or terminal tests.

### LLM-disabled operation is mandatory

LLMs may later enrich language. They may render utterances, paraphrase records, summarize actor-known state, or parse freeform text into candidate structured speech acts. They may not decide truth, mutate state, create hidden facts, grant knowledge, generate quests, recognize completion, plan authoritative agents, or make tests depend on live nondeterminism.

The simulation, first TUI, structured speech acts, records, reports, routines, agents, tests, and canonical scenarios must work with LLMs disabled or deterministic mocks.

### Rust-first implementation discipline

Tracewake is Rust-first for the authoritative simulation core, event/replay machinery, action validation, actor-knowledge filtering, agent decision core, no-human simulation harness, and first TUI client unless a later foundation-level decision explicitly replaces this doctrine.

This does not freeze crates, persistence engines, ECS/database architecture, terminal libraries, serialization formats, async runtimes, or module layout. Those are architecture decisions.

Non-Rust tooling may support docs, import/export, analysis, optional language surfaces, content pipelines, or visualization. It may not become the authoritative simulation brain.

## The required first miracle

The canonical early proof is missing property/theft, but it is only a proof, not Tracewake's whole identity.

The proof must show:

1. An actor stores or expects property somewhere.
2. Another actor takes, moves, hides, borrows, or steals it because of modeled need, belief, opportunity, risk, access, motive, routine, relationship, or obligation.
3. The victim discovers absence only through expectation contradiction, perception, instruction, or search.
4. Witnesses may perceive partial, uncertain, stale, or misinterpreted traces.
5. Testimony, lies, gossip, rumor, or report propagates typed claims.
6. A clerk, reeve, household, or local authority may open, refuse, delay, misclassify, misfile, or ignore a record from partial information.
7. Wrong suspicion arises for legible reasons.
8. Notices and records can become stale.
9. The same chain can occur with no human present.
10. Debug possession proves no knowledge transfer.
11. Deterministic replay and debug inspection explain what happened, what traces exist, who knows what, who is wrong, and which later events became possible.

This is not an authored mystery script. It is proof that physical state, belief state, social information, institutions, records, memory, and traces are one causal system.

## First playable loop

The first serious loop is a life-possession sandbox:

1. Enter a TUI village as one ordinary actor.
2. Inspect actor-known needs, location, possessions, beliefs, intentions, obligations, relationships, and available actions.
3. Sleep, eat, work, move, open doors, use containers, store, hide, take, steal, buy food, read notices, search, report, lie, gossip, accuse, refuse, and talk through structured speech acts.
4. Wait while ordinary routines continue without human attention.
5. Switch possession in debug mode and verify that knowledge, suspicion, obligations, consequences, and memories remain with bodies and institutions.
6. Inspect causal chains in debug mode.
7. Follow or ignore stale public leads only after ordinary life, belief, records, storage, travel, and action parity already work.

## Long-term posture

Tracewake should eventually support travel, expeditions, companions, parties, trade routes, disease, animals, weather, disasters, migration, corruption, inheritance, organizations, guilds, gangs, courts, armies, councils, religions, companies, politics, settlement conquest, detailed combat and injury, fantasy packs, and science-fiction packs.

These are not exceptions. They are future expansions built from the same machinery.

Gathering an army must eventually arise from recruitment, belief, loyalty, fear, pay, command, logistics, morale, food, travel, supply, wounds, desertion, records, rumor, authority, negotiation, territorial control, and consequence. It must not be a conquest quest.

## Strong decision

Build a village whose people can be wrong for the right reasons. Do not build a world that flatters the player, waits for the player, reads the player's mind, scripts outcomes for the player, or lets language invent reality.

## 2026 hardening: emergent richness requires a truth firewall

Tracewake's ambition is not merely to make agents perform recognizable routines. The project target is a simulation that can eventually produce long, surprising causal chains that other games would usually script: displaced threats, altered trade routes, reported incidents, institutional responses, notices, contracts, travel, investigation, mistaken conclusions, and stale aftermath.

That ambition requires a stricter charter rule:

```text
Truth may validate actions, but truth may not plan them.
```

The project must prefer a weaker-looking but honest actor over a clever-looking contaminated actor. An agent that finds food, work, danger, culprits, routes, witnesses, or opportunities by querying hidden physical truth has failed the project even when the resulting behavior looks plausible. Emergence is only real when the causal and epistemic route to the behavior is modeled.

## Foundation-level success standard

A Tracewake behavior is not good enough because it is entertaining, fluent, or superficially humanlike. It is good enough when it satisfies all of the following:

- the behavior arises from ordinary actor pressures, commitments, memories, relationships, roles, perceptions, and accessible affordances;
- the proposal can be explained from actor-known inputs before validation;
- validation may use authoritative state without leaking hidden truth into future cognition;
- the resulting events, observations, belief updates, records, traces, and projections are replayable;
- debug can explain both the truth and the actor's limited view of truth;
- the same machinery works with no human controller bound;
- no director, quest objective, LLM narration, UI marker, or script had to push the world toward the result.

## Genre-neutral implication

Fantasy, Lovecraftian, post-apocalyptic, detective, mundane, and other domain packs may add bodies, institutions, threats, rituals, monsters, hazards, technology, records, and speech styles. They must not add genre-specific exceptions to the truth firewall. A monster, disease, demon, caravan, patrol, notice board, court, radio tower, cult ledger, or village rumor network remains causal machinery, not a hidden director.
