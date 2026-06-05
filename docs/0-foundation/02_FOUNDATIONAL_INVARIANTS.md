# Foundational Invariants

These are hard rules. They are not preferences, moods, or phase-specific aspirations. If Tracewake violates them routinely, it stops being Tracewake.

## TUI and playability

### INV-001 — The TUI is a primary product interface

The TUI is not a throwaway debug shell. It must remain playable, legible, reusable, and architecturally protected.

### INV-002 — Every runnable phase has a playable TUI gate

A phase is not complete unless the new mechanic can be reached, used, inspected, and regression-tested through the TUI or through the same actor-filtered view models the TUI consumes.

### INV-003 — Mechanics hidden from play are unfinished

A system that exists only in headless tests is unfinished unless the phase is explicitly pre-runnable ontology work. Once the kernel can run, playability and inspectability advance together.

### INV-004 — Embodied mode preserves actor knowledge

Normal play shows the current actor’s perceived or believed world, not ground truth. Debug mode may reveal truth, event graphs, hidden beliefs, and planner state, but it must be visibly separate.

### INV-005 — The TUI must not implement simulation rules

The TUI consumes actor-filtered view models and submits typed action attempts. It must not query hidden truth in embodied mode, maintain quest state, invent leads, bypass preconditions, or mutate world state through player-only paths.

## Player ontology, possession, and action parity

### INV-006 — No sacred player entity

The authoritative simulation must not contain a metaphysically special player character.

Allowed:

```text
HumanController -> ActorId
```

Forbidden:

```text
GenerateQuestFor(PlayerCharacter)
Guard.KnowsCrimesOf(PlayerCharacter)
NPCs.IgnoreNeedsNearPlayer()
Threat.SpawnBecausePlayerIsBored()
Reward.SpawnForPlayerCompletion()
```

### INV-007 — Possession changes input, not reality

Switching bodies changes only controller binding. The previous body remains an ordinary agent with beliefs, needs, intentions, injuries, relationships, possessions, obligations, suspicions, memories, and plans.

### INV-008 — Body-switching is foundational and debug-first

Possession is currently a testing/debugging tool and architectural proof of actor parity. It may later become diegetic in a domain pack, but the kernel must not depend on possession metaphysics.

### INV-009 — Possession targets must be ordinary actor surfaces

Debug possession should generally allow high-detail ordinary actors. Abstract regional processes, institutions-as-systems, low-LOD aggregates, weather, markets, diseases, and non-agent processes are not directly possessable unless represented by a promoted ordinary actor/controller surface.

### INV-010 — Human knowledge is not actor knowledge

The human may remember what happened while controlling another actor. The current actor may not assert, accuse, testify, claim payment, persuade, report, threaten, bargain, or institutionally act from facts that have not reached that actor through modeled channels.

### INV-011 — Actor notebooks and human/debug notes are separate

Actor notebooks and actor-known leads may affect embodied play because they are actor knowledge. Human/debug notes are optional out-of-world UI conveniences and must never become sources for speech acts, testimony, accusations, reports, persuasion, institutional claims, or action preconditions.

### INV-012 — Every world-affecting player action must be NPC-possible

Every committed world-affecting player action must correspond to an action a non-player agent could attempt under equivalent physical, social, epistemic, resource, skill, risk, and institutional conditions.

### INV-013 — UI assistance is not world privilege

Sorting, filters, maps, notebooks, search, planning suggestions, why-not explanations, summaries, and action menus may clarify. They must not create facts, reveal hidden truth in embodied mode, grant unearned actor knowledge, bypass preconditions, or make institutions recognize anything without modeled evidence/procedure.

## Causality, events, and replay

### INV-014 — Meaningful state changes require events

If a change can affect gameplay, history, belief, traces, reputation, law, economy, memory, UI leads, replay, debug explanation, or later reasoning, it must be represented by one or more events.

### INV-015 — Every event needs a cause model

Events must be explainable through preceding state, actor intention, need pressure, belief, routine, environmental process, institutional procedure, regional process, authored initial condition, or declared exogenous boundary input.

### INV-016 — Current-state-only simulation is forbidden

Materialized state may exist for performance, but it is not sufficient. The engine must preserve enough event history and causal ancestry to explain significant consequences.

### INV-017 — Events leave traces

Every meaningful event leaves physical, mental, social, institutional, economic, environmental, spatial, procedural, or erasure traces. A clean crime includes successful trace-removal events.

### INV-018 — Trace removal is eventful

Cleaning, hiding, forging, moving, overwriting, losing, burning, forgetting, misfiling, silencing, bribing, or otherwise destroying evidence must itself be represented by events when it can matter.

### INV-019 — Failure is an event

Failed attempts matter. Refused reports, interrupted routines, missed meetings, failed searches, botched accusations, failed payments, failed travel, rejected speech acts, invalid plans, and abandoned intentions produce events when they can affect future reasoning.

### INV-020 — Absence can be evidence

A missing item, absent worker, closed shop, unposted notice, quiet road, uncollected debt, empty bed, unlit hearth, unavailable witness, or missing record can become evidence if an agent expected otherwise or searched intentionally.

### INV-021 — Events can be mental, social, institutional, or regional

Physical events are not enough. Belief changes, memory changes, inferences, testimony, report filing, record revision, norm activation, payment promise, regional order arrival, and LOD promotion/demotion are eventful when meaningful.

### INV-022 — Replay is a design requirement

The engine must support reconstruction, audit, causal debugging, temporal query, and replay of significant outcomes. Determinism is a foundation requirement, not a convenience.

### INV-023 — Randomness must be seedable and auditable

Random draws that affect meaningful outcomes must be reproducible from seed and traceable in replay/debug output. Hidden unlogged randomness is forbidden for authoritative consequences.

### INV-024 — Snapshots and compaction are allowed only with surviving ancestry

Snapshots, indexes, read models, and compaction are implementation necessities. They may not erase significant causal ancestry, belief provenance, institutional records, surviving traces, active leads, or replay-critical random draws.

## Epistemics and memory

### INV-025 — Ground truth, subjective belief, and public/institutional record are separate

The engine must distinguish what happened, what each agent believes, and what an institution, household, rumor network, ledger, notice, or public artifact treats as known.

### INV-026 — Agents act from beliefs, not truth

An agent’s action selection, speech, search, suspicion, reporting, cooperation, avoidance, purchase, theft, work, and travel choices must be based on that agent’s beliefs and accessible records, not hidden authoritative truth.

### INV-027 — No telepathy

Information reaches agents only through modeled channels: perception, testimony, rumor, reading, records, inference, instruction, memory, observation, domain-defined special channels, or institutional notification.

### INV-028 — Wrong beliefs are first-class state

Mistakes, stale information, false rumors, lies, confabulations, misidentifications, biased inferences, contradictory hypotheses, and partial memories are required mechanics.

### INV-029 — Important beliefs need provenance

Important beliefs record holder, proposition, confidence, source, channel, acquisition time, believed event time when available, staleness, provenance chain, and contradiction links.

### INV-030 — Discovery requires expectation, perception, instruction, or search

An agent cannot notice coins are missing unless the agent expected coins to be present, perceived a relevant absence, was instructed to check, had a reason to inspect, or searched the relevant place.

### INV-031 — Staleness is not automatically corrected

Notices, maps, reports, rumors, memories, household knowledge, institutional records, and actor notebooks remain stale until updated through modeled channels.

### INV-032 — Identity uncertainty is mandatory

Recognition, witness testimony, rumor, records, footprints, handwriting, voices, clothing, tools, and social descriptions may identify the wrong person or support multiple hypotheses.

### INV-033 — Memory fallibility is foundation doctrine

Forgetting, confidence decay, distortion, source confusion, misremembered time, misremembered identity, and contradiction handling are first-class mechanics. Implementation may stage them, but the design must not assume perfect memory.

## Agents, planning, speech, and language

### INV-034 — Agents are symbolic and inspectable in v1

V1 agents use explicit beliefs, values, needs, goals, projects, intentions, HTN methods, bounded local planning, budgets, interruption points, and failure events. Their decisions must be explainable.

### INV-035 — Agents need durable intentions

Agents need commitments that persist across small fluctuations and can be interrupted by stronger modeled causes. Avoid utility jitter.

### INV-036 — BDI separation is foundation doctrine

Beliefs, desires/values/goals, and intentions are distinct. Needs and motives are not truth access. Intentions are not guaranteed outcomes.

### INV-037 — HTN methods are causal procedures, not scripts

HTN methods may encode routines, jobs, household behavior, reporting procedures, search procedures, and institutional workflows only as state-dependent methods with failure, interruption, and alternatives.

### INV-038 — Bounded local planning is allowed for concrete means

GOAP-style planning may solve concrete action sequences such as reaching rooms, opening doors, retrieving items, eating, searching, reporting, fleeing, or delivering objects. It must be bounded and nested under explicit intentions or methods.

### INV-039 — Utility scoring is a bounded heuristic

Utility may help choose among explicit motives, methods, or planner options. It must not become the whole mind, flatten agents into meter machines, or produce jittery behavior.

### INV-040 — Routines are defeasible intentions

A worker works because she believes it is a workday, has obligations, needs income, can reach the workplace, has tools/access, and has no stronger interruption. Schedules are not teleports.

### INV-041 — Needs are pressures, not puppet strings

Hunger, fatigue, safety, warmth, shame, greed, loyalty, affection, duty, ambition, fear, debt, habit, and curiosity influence choice without erasing belief, value, risk, cost, relationship, skill, and circumstance.

### INV-042 — Competence is bounded

Agents should be resourceful enough to be interesting but limited by belief, skill, cost, time, risk, fatigue, social position, relationships, access, resources, and planner budget.

### INV-043 — Speech begins as typed speech acts

Initial conversation must use structured speech acts with templated surface text. No freeform parser may mutate authoritative state.

### INV-044 — Lying is causal, not magical

A lie is a speech act containing a claim the speaker does not believe or has reason to doubt, with modeled motive, risk, audience, credibility, provenance, memory effects, and possible future contradiction.

### INV-045 — LLMs are not agent brains

LLMs may not decide truth, plan authoritative agents, create hidden facts, grant knowledge, mutate state, generate quests, or make institutions recognize claims.

## Institutions, households, norms, records, and economy

### INV-046 — Institutions are not omniscient

Authorities, guilds, households, markets, workshops, gangs, councils, offices, and future domain-specific religious institutions act from reports, records, testimony, observations, procedures, jurisdiction, manpower, budgets, delay, fear, bias, corruption, and relationships.

### INV-047 — Law and norms are social machines

A violation becomes actionable through observation, trace, report, evidence, record, investigation, institutional belief, social belief, or procedure. Violation and detection are separate.

### INV-048 — Norms are explicit

Obligations, permissions, prohibitions, roles, sanctions, constitutive norms, institutional facts, access rules, and procedures must be represented explicitly enough to inspect and test.

### INV-049 — Records are world artifacts

Reports, ledgers, contracts, tax records, case files, orders, notices, warrants, receipts, promises, and household accounts have authors, issuers, custody, timestamps, claims, provenance, access rules, staleness, and contradiction potential.

### INV-050 — Procedure can fail

An office may ignore the poor, protect the powerful, lose a report, run out of funds, misclassify an incident, arrest the wrong person, fail to pay, refuse jurisdiction, or never learn the relevant fact.

### INV-051 — Households exist from the beginning

Households are domestic institutions with members, roles, property access, private spaces, shared storage, obligations, routine coordination, household knowledge, conflicts, and inheritance hooks.

### INV-052 — Ownership, custody, and access are distinct

Who owns an object, who physically holds it, who may access it, who is believed to own it, and who can prove it are separate states. Theft, borrowing, storing, hiding, buying, paying, promising, and confiscating depend on these distinctions.

### INV-053 — Ordinary survival is causal

Food, sleep, work, storage, money, and custody must be real enough to cause behavior. Fake meter refills that bypass ownership, storage, travel, institutions, or records are forbidden.

### INV-054 — V1 economy is simplified but not fake

V1 does not simulate full macroeconomics. It must still model food, wages, obligations, stock/service abstractions, money custody, ownership, buying, stealing, storing, hiding, paying, and promising payment as causal actions.

## Questlessness and story

### INV-055 — No quest as primary data type

The engine stores incidents, needs, requests, contracts, promises, obligations, notices, rumors, records, threats, opportunities, investigations, sanctions, and leads. It does not store quest as the authoritative world process.

### INV-056 — Tasks are projections

A player-facing objective, notebook card, lead, or reminder is a view over source-bound world state. It is not the process itself.

### INV-057 — Rewards require custody and verification

Payment depends on who promised it, whether funds exist, what proof is accepted, who verifies it, whether the payer remains able and willing, and whether anyone disputes the claim.

### INV-058 — No authored outcome chains

Forbidden: quest beats, scenario scripts, drama directors, boredom detectors, “when player does X, make Y happen,” NPC behavior that exists only to serve the player, content that waits indefinitely, objective markers to ground truth, and completion recognition without modeled custody/evidence/procedure.

### INV-059 — Public artifacts are not quests

A notice can be stale, forged, ignored, destroyed, contradicted, paid, unpaid, misread, solved by someone else, or based on partial information.

### INV-060 — Story sifting may curate only

Story recognition, salience, recaps, notebooks, and event summaries may highlight what happened. They may not cause events, repair pacing, generate convenient clues, or inject drama.

## Domain boundary and first scope

### INV-061 — Ordinary life comes before adventure

Sleep, eating, work, homes, storage, ownership, doors, rooms, containers, routines, needs, basic social interaction, belief, traces, and TUI embodied play come before expeditions, investigation depth, combat, magic, worldgen, or graphics.

### INV-062 — The first domain is neutral medieval-ish ordinary life without magic

The first domain may use villages, roads, workshops, taverns, storage, offices, local authority, notices, household property, travel, and public records. It must not require magic, monsters, divine authority, fantasy species, feudal assumptions, or combat-centric play.

### INV-063 — No fantasy leakage into the kernel

The core must not assume monsters, magic, species, feudal law, medieval technology, divine authority, adventurer classes, combat categories, supernatural senses, or religious metaphysics.

### INV-064 — Domain packs own flavor

Domain packs own technology, metaphysics, species/body types, magic, threats, domain institutions, occupations, items, affordances, unique traces, diseases, speech/culture tags, terrain/place types, scenario seeds, and domain-specific laws.

### INV-065 — Special powers are causal channels

Magic, sensors, dreams, psychic impressions, radios, rituals, alien technology, or divine messages, if ever added, are modeled as channels or actions with costs, reliability, distortion, provenance, traces, counters, and failure modes.

### INV-066 — No detailed combat in the first serious vertical slice

Early threats may appear through travel risk, injury/death events, aftermath traces, fear, reports, route avoidance, and institutional response. Detailed combat and granular injury are deferred.

### INV-067 — Start small enough to inspect

A tiny village with deep causality is better than a large opaque region. The earliest serious slice should target roughly 10-30 high-detail agents, or at most a few dozen.

### INV-068 — The world continues without the human

No-human simulations must produce ordinary life, reports, wrong beliefs, stale records, trade/work consequences, routine interruption, social propagation, and institutional action or failure.

## Scale, LOD, long simulation, and regional processes

### INV-069 — Scale through honest abstraction

Low-fidelity processes produce inspectable summary events and preserve causal ancestry. They do not hide a director.

### INV-070 — Low-LOD agents are still people

Lower detail is dormant or summarized detail, not permission to turn citizens into props.

### INV-071 — Promotion and demotion are explicit events

LOD changes must be logged with cause, tier, preserved ancestry, lost fidelity if any, and replay/debug visibility.

### INV-072 — LOD relevance is causal, not dramatic

Promote detail when an entity becomes causally, spatially, social, institutional, epistemic, regional, or debug relevant. Never promote because the player is bored.

### INV-073 — Human focus is not player privilege

Human proximity, viewport, possession, or debug focus may affect rendering, inspection, and scheduler interest. It must not create events, guarantee outcomes, alter probabilities for drama, or make the world wait.

### INV-074 — Regional processes must be declared causal processes

A storm, caravan, tax order, plague pressure, crop failure, migration wave, bandit movement, road closure, or outside institutional order may enter local simulation only with declared source, cadence/trigger, input state, random model if any, domain authority, scope, delivery channel, traces, affected beliefs/records, causal ancestry, and replay/debug visibility.

### INV-075 — Long prehistory emits structured causes, not lore prose

Long simulation and prehistory may use multi-resolution simulation and summary events. They must create inspectable causal ancestry, public memory, stale information, records, and present consequences.

### INV-076 — Demography is staged but protected

Births, childhood, aging, death, marriage/partnership, reproduction, inheritance, migration, disease, employment changes, household formation, settlement growth/decline, institutional succession, and memory/history decay are long-term causal domains. V1 need not complete them, but v1 must not block them.

## LLM boundary

### INV-077 — LLMs may not mutate authoritative state directly

Any state change implied by language must be represented as a typed, validated action or speech act.

### INV-078 — LLMs may not invent hidden evidence or ground truth

No LLM output may determine guilt, create a clue, correct stale information, bypass preconditions, create a quest, or grant NPCs unearned knowledge.

### INV-079 — Core mechanics must work with LLMs disabled

Tests use deterministic mocks or no LLM calls. LLM calls are optional surfaces, not dependencies.

### INV-080 — LLM parsing is candidate generation only

An LLM may parse a proposed utterance into candidate speech acts only if the typed action validator can reject, narrow, or require clarification. The LLM’s interpretation is never authoritative.

## Implementation language

### INV-081 — Tracewake is Rust-first

The authoritative simulation core, event/replay machinery, action validation, actor-knowledge filters, agent decision core, no-human simulation harness, and first TUI client must be Rust-first.

### INV-082 — Rust doctrine does not freeze crates

Foundation may require Rust and safe, deterministic, maintainable core design. It must not prematurely freeze UI libraries, persistence engines, ECS frameworks, serialization formats, async runtimes, databases, or crate choices.

### INV-083 — Non-Rust code may not become authoritative simulation

Non-Rust tooling may support content, import/export, analysis, optional language rendering/parsing, documentation, or visualization. It may not decide authoritative simulation state.

## Validation

### INV-084 — Every feature must pass the foundation questions

Every feature must answer:

```text
What caused it?
Who knows it?
How can they be wrong?
What traces exist?
What institution, household, norm, or record cares?
Can an NPC do the same kind of thing?
Can it be played and inspected through the TUI?
Does it leak player privilege, ground truth, genre assumptions, scripting, or LLM authority?
Can it run in a no-human simulation?
Can debug mode explain it?
```

### INV-085 — No-human acceptance tests are mandatory

Every runnable phase must include no-human simulation tests. The world must continue without the human.

### INV-086 — Deterministic replay is tested, not hoped for

Replay and seed determinism must be verified by automated regression tests for significant causal scenarios.

### INV-087 — LLM-disabled operation is a gate

The simulation, TUI, tests, and canonical scenarios must function with all LLM functionality disabled.
