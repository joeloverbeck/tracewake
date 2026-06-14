# Constitutional Invariants

These rules are hard. They are not preferences, milestone aspirations, or tone guidance. If Tracewake routinely violates them, it is no longer Tracewake.

## Identity and authority

### INV-001 — Causality comes before drama

Events must arise from modeled state, actor intention, belief, need, routine, affordance, environment, institution, household, regional process, authored initial condition, or declared boundary input. Runtime dramatic need is not a cause.

### INV-002 — Belief comes before truth

Actors, households, institutions, and groups act from their own beliefs, records, memories, observations, testimony, rumors, inferences, and procedures, not from hidden ground truth.

### INV-003 — Ordinary life comes before adventure

Sleep, food, work, homes, rooms, doors, containers, storage, travel, property, household access, routines, speech, belief, records, and TUI embodied play come before combat, expeditions, magic, large regions, graphics, or freeform LLM conversation.

### INV-004 — The authoritative world ignores human existence

The simulation must run coherently with no human controller. A human attached to an actor may submit inputs; the world must not store a sacred player entity.

### INV-005 — The only normal controller binding is ordinary possession

Allowed:

```text
HumanController -> ActorId
```

Possession changes input binding only.

### INV-006 — Possession transfers no world knowledge

The human may remember other bodies. The current actor may not report, accuse, testify, bargain, prove, threaten, navigate, or institutionally act from facts not acquired through modeled channels available to that actor.

### INV-007 — Every world-affecting player action must be NPC-possible

A committed world-affecting player action must correspond to an action a non-player actor could attempt under equivalent physical, social, epistemic, resource, skill, risk, and institutional conditions.

### INV-008 — UI assistance is not authority

Sorting, filters, maps, lead grouping, why-not explanations, summaries, command menus, and planning suggestions may clarify. They may not create facts, grant knowledge, bypass preconditions, reveal hidden truth in embodied mode, or make institutions recognize anything without modeled evidence/procedure.

## Events, traces, and replay

### INV-009 — Meaningful state changes require events

If a change can affect gameplay, history, belief, traces, records, reputation, law, economy, memory, UI leads, replay, debug explanation, or later reasoning, it must be represented by one or more events.

### INV-010 — Every event needs a cause model

A meaningful event must be explainable through preceding state, actor intention, need pressure, belief, routine, physical affordance, environmental process, institutional procedure, household procedure, regional process, authored initial condition, or declared boundary input.

### INV-011 — Current-state-only simulation is forbidden

Materialized state may exist for speed. It is not sufficient. The engine must preserve enough event history and causal ancestry to explain significant consequences.

### INV-012 — Events are not only physical

Mental, social, institutional, economic, environmental, procedural, regional, LOD, and erasure events are real when they affect later reasoning or consequence.

### INV-013 — Meaningful events leave traces

Every meaningful event leaves physical, mental, social, institutional, economic, environmental, spatial, procedural, regional, or erasure traces. A clean crime includes successful trace-removal events.

### INV-014 — Trace removal is eventful

Cleaning, hiding, forging, moving, burning, overwriting, losing, silencing, bribing, misfiling, intimidating, or forgetting evidence is eventful when it can matter.

### INV-015 — Failure is eventful when consequential

Refused reports, failed searches, invalid action attempts, blocked plans, missed meetings, failed payments, interrupted travel, rejected speech acts, abandoned intentions, and office failures produce events when they can affect future state.

### INV-016 — Absence becomes evidence only through a channel

A missing item, absent worker, closed shop, empty bed, missing record, uncollected debt, or quiet road can become evidence only through expectation, perception, instruction, or intentional search.

### INV-017 — Randomness must be seedable and auditable

Random draws that affect meaningful outcomes must be reproducible and visible in replay/debug. Hidden unlogged authoritative randomness is forbidden.

### INV-018 — Deterministic replay is foundational

Replay must reconstruct significant outcomes from initial seed/configuration, authored state, ordered events, meaningful random draw records, ruleset/data versions, and ancestry-preserving snapshots.

### INV-019 — Snapshots and compaction may not erase live ancestry

Snapshots, indexes, read models, and compaction are allowed only if significant causal ancestry, belief provenance, record provenance, trace history, active leads, institutional state, random draw auditability, and replay-critical data survive.

### INV-020 — Event schema evolution is mandatory

Event kinds and payloads must be versioned enough that replay can reject unsupported history rather than silently inventing repairs.

## Claims, belief, memory, and information

### INV-021 — Typed claims/propositions are the epistemic currency

Important beliefs, memories, observations, testimony, lies, rumors, gossip, accusations, reports, notices, ledgers, ownership claims, debts, contracts, promises, institutional facts, household knowledge, actor-known leads, and notebook entries must be grounded in typed claims/propositions.

### INV-022 — Raw prose is not authoritative state

Prose may render claims, summarize records, and help humans read the world. It may not define hidden facts, belief content, proof, quest state, institutional recognition, or world mutation.

### INV-023 — Ground truth, subjective belief, and records are separate

The engine must distinguish what happened, what each belief-holder believes, and what public/institutional/household artifacts claim.

### INV-024 — No telepathy

Information reaches actors only through modeled perception, search, expectation contradiction, testimony, rumor, reading, records, instruction, memory, inference, institutional notification, household communication, or domain-defined special channels.

### INV-025 — Wrong beliefs are first-class state

False rumors, stale records, lies, misidentifications, biased inferences, contradictory hypotheses, source confusion, confabulations, and partial memories are required mechanics, not edge cases.

### INV-026 — Important beliefs need provenance

Important beliefs record holder, claim, stance, confidence, source, channel, acquisition time, believed event time when available, staleness, provenance chain, contradiction links, and scope.

### INV-027 — Memory is fallible by doctrine

Forgetting, decay, distortion, source confusion, temporal uncertainty, confidence drift, identity uncertainty, emotional salience, rehearsal, correction, and resistance to correction must remain possible even if staged gradually.

### INV-028 — Staleness is not automatically corrected

Notices, maps, reports, ledgers, rumors, household knowledge, institutional records, actor notebooks, memories, and public beliefs remain stale until updated through modeled channels.

### INV-029 — Identity uncertainty is mandatory

Recognition, witness testimony, rumor, handwriting, clothing, tools, footprints, voices, aliases, roles, and descriptions may support partial, wrong, or competing identity claims.

### INV-030 — Evidence is not truth

A trace, report, record, confession, accusation, rumor, or witness statement supports claims with provenance and confidence. It does not become ground truth for embodied actors or institutions.

### INV-031 — Human/debug notes are non-diegetic

Human/debug notes may help the human remember. They must never satisfy actor knowledge, speech-act preconditions, proof, institutional claims, reports, accusations, persuasion, or search authority.

## Agents and planning

### INV-032 — V1 agents are symbolic and inspectable

V1 agents use explicit beliefs, needs, values, duties, goals, projects, intentions, HTN-style methods, bounded local planning, budgets, interruption points, and decision traces.

### INV-033 — BDI separation is doctrine

Beliefs, desires/values/goals/needs/duties, and intentions are distinct. Needs do not grant truth. Intentions are commitments, not guaranteed outcomes.

### INV-034 — Agents need durable intentions

Agents require persistent commitments/projects that resist small score jitter and can be interrupted by stronger modeled causes.

### INV-035 — Routines are defeasible intentions

A worker works because they believe it is work time, have obligations, can reach work, have access/tools, and lack stronger interruptions. Schedules are not teleports or puppet strings.

### INV-036 — HTN methods are procedures, not story scripts

HTN-style methods may encode routines, jobs, household procedures, search procedures, report procedures, travel preparation, and institutional workflows only as state-dependent methods with failure, interruption, alternatives, costs, and traces.

### INV-037 — Bounded local planning is for concrete means

GOAP/STRIPS-style planning may sequence concrete actions such as reaching places, opening doors, retrieving food, searching, fleeing, delivering, buying, reporting, or speaking. It must be bounded and nested under explicit intentions/methods.

### INV-038 — Utility scoring is only a bounded heuristic

Utility may choose among explicit motives, methods, or options. It must not become the whole mind, a drama-weighted selector, player-proximity bias, global truth reader, or jitter machine.

### INV-039 — Needs are pressures, not puppet strings

Hunger, fatigue, safety, warmth, shame, greed, loyalty, affection, duty, ambition, fear, debt, habit, and curiosity influence choices without erasing belief, relationship, cost, risk, skill, access, or circumstance.

### INV-040 — Agents are bounded but competent

Agents should be resourceful enough to be interesting and limited by belief, memory, skill, time, distance, fatigue, hunger, fear, status, relationship, access, resources, and planner budget.

### INV-041 — Agent decisions need debug traces

Debug mode must be able to show beliefs used, needs/duties considered, active intention, selected method/plan, alternatives rejected, blocked preconditions, random draws, interruptions, and resulting events.

### INV-042 — No LLM agent brains

LLMs may not decide authoritative actions, truth, beliefs, motives, plans, success, guilt, proof, sanctions, or world mutation.

## Actions, affordances, and ordinary life

### INV-043 — Action validation is ordinary-agent validation

The action pipeline validates attempts against actor state, belief context, physical affordance, access, time, resource, norm, risk, skill, and institution conditions. It does not branch for player privilege.

### INV-044 — Affordances are conditional, not menu decoration

Objects, places, people, records, institutions, routes, and traces expose possible actions only through conditions: physical reach, knowledge, access, social permission or willingness to violate, tools, time, risk, and state.

### INV-045 — Ordinary survival is causal

Food, sleep, fatigue, work, money, storage, custody, travel time, and access must be real enough to cause behavior. Fake meter refills disconnected from world state are forbidden.

### INV-046 — V1 economy is simplified but not fake

V1 need not simulate macroeconomics. It must model food, wages/payment, promises, money custody, ownership/access claims, buying, stealing, storing, hiding, paying, refusing, and ordinary stock/service abstractions as causal actions.

### INV-047 — Search is intentional, costly, bounded, and fallible

Search is not a UI query over truth. It requires motive/question, area, method, time, access, perception, false negatives, possible irrelevant discoveries, and eventful failure when consequential.

### INV-048 — Travel is causal, not teleportation

Travel must preserve route choice, time, fatigue/hunger cost, risk, observations, missed meetings, departure/arrival events, route knowledge, and stale information where relevant.

## Institutions, households, norms, records

### INV-049 — Institutions are not omniscient

Authorities, households, workshops, offices, guilds, markets, gangs, courts, councils, companies, armies, religious institutions, and future organizations act from records, reports, testimony, procedures, roles, resources, jurisdiction, manpower, delay, bias, fear, corruption, and member beliefs.

### INV-050 — Households exist from the beginning

Households are domestic institutions with members, roles, private/shared spaces, storage, access rules, household knowledge, obligations, privacy, conflict, care, property ambiguity, and inheritance hooks.

### INV-051 — Norms are explicit enough to inspect

Obligations, permissions, prohibitions, powers, sanctions, roles, procedures, access rules, proof rules, and constitutive norms must be represented explicitly enough to test and debug.

### INV-052 — Violation, detection, proof, enforcement, and justice are separate

A norm violation is not automatically detected. Detection is not proof. Proof is not enforcement. Enforcement is not justice.

### INV-053 — Records are world artifacts or institutional state

Reports, ledgers, notices, contracts, debts, warrants, receipts, promises, letters, case files, orders, and household accounts have authors, issuers, custody, timestamps, claims, provenance, access rules, staleness, and contradiction potential.

### INV-054 — Runtime records need runtime causes

After simulation start, a letter, report, notice, accusation, order, rumor, ledger entry, bounty, or warrant exists only because a modeled actor, household, institution, or regional process had reason and means to create, carry, post, file, copy, alter, damage, lose, or destroy it.

### INV-055 — Ownership, custody, access, control, proof, and belief are distinct

Who owns an object, who holds it, who may access it, who can manipulate it, who can prove a claim, and who believes any of these are separate states.

### INV-056 — Procedure can fail

An office may be closed, absent, biased, corrupt, underfunded, afraid, misled, tired, out of jurisdiction, wrong, slow, or careless. Records can be lost, misfiled, forged, damaged, contradicted, or stale.

### INV-057 — Public artifacts are not quests

A notice may be stale, false, forged, removed, ignored, paid, unpaid, disputed, solved by someone else, or based on partial information. It does not imply a true target, guaranteed reward, player ownership, or world waiting.

## Questlessness and authoring

### INV-058 — Quest is not a primary data type

The engine stores incidents, needs, requests, contracts, promises, obligations, debts, notices, rumors, records, threats, opportunities, investigations, sanctions, and leads. It does not store quest as authoritative world process.

### INV-059 — Leads and tasks are projections

Actor-known leads, agendas, obligations, requests, notices, and reminders are source-bound projections over claims, beliefs, records, intentions, and artifacts. They are not ground truth objectives.

### INV-060 — No authored outcome chains

Forbidden: quest beats, scenario scripts, drama directors, boredom detectors, player-conditioned event injection, story-pacing mutation, guaranteed targets, guaranteed rewards, automatic culprit reveals, and completion recognition without modeled evidence/procedure.

### INV-061 — Authored causal machinery is required

Designers author actions, affordances, needs, routines, traits, HTN methods, norms, institution procedures, speech acts, trace types, claim vocabularies, regional processes, initial state, scenario seeds, and test fixtures. These create possibility space, not guaranteed arcs.

### INV-062 — Scenario seeds are tensions, not scripts

Seeds may create initial property, old records, unresolved accusations, debts, relationships, routines, rumors, weather, regional conditions, and long-past events. They may not condition runtime outcomes on player progress.

### INV-063 — Authored prehistory must be marked

Pre-simulation artifacts, old records, historical events, unresolved accusations, and boundary conditions must carry structured provenance marking them as authored prehistory, generated prehistory, simulated summary ancestry, or declared boundary input.

### INV-064 — Rewards require custody and recognition

Payment depends on promise/source, funds, custody, proof, verification, authority, willingness, dispute, corruption, delay, and procedure. A reward is not a completion flag.

## TUI, possession, and debug

### INV-065 — The TUI is a primary product interface

The TUI is not a throwaway debug shell. It must remain playable, legible, reusable, actor-filtered, testable, and architecturally protected.

### INV-066 — Every runnable phase has a TUI/view-model gate

A runnable phase is incomplete unless its mechanics can be reached, used, inspected, and regression-tested through the TUI or the same actor-filtered view models.

### INV-067 — Embodied mode shows actor-known reality

Normal play shows the current actor's perceived, believed, remembered, inferred, read, or told world. It hides hidden truth, other minds, true culprits, objective quest labels, and debug causal graphs.

### INV-068 — Debug mode is visibly non-diegetic

Debug mode may reveal truth, event graphs, hidden beliefs, planner state, random draws, LOD ancestry, and possession history. It must never be confused with embodied knowledge.

### INV-069 — The TUI must not implement simulation rules

The TUI consumes actor-filtered view models and submits typed action attempts. It must not query hidden truth in embodied mode, maintain quest state, bypass validators, or mutate world state through player-only paths.

### INV-070 — Why-not explanations are mandatory

Blocked actions should explain missing preconditions in actor-known terms, with optional debug-only truth clearly separated.

### INV-071 — Mechanics hidden from play are unfinished

Headless tests are necessary. Once the kernel is runnable, mechanics hidden from TUI/view-model reachability are not complete.

## LLM and language

### INV-072 — Speech begins as structured speech acts

Initial conversation uses typed speech acts and deterministic templates. Speech acts carry typed claims, source references, validation results, listener interpretation, and event provenance.

### INV-073 — Lying is causal, not magical

A lie is a validated speech act containing a claim the speaker does not believe or has reason to doubt, with motive, risk, audience, credibility, provenance, and possible consequences.

### INV-074 — LLMs may render or parse only behind validation

LLMs may render structured acts, paraphrase records, summarize actor-known state, or parse freeform text into candidate speech acts. The validator decides what can be committed.

### INV-075 — LLMs may not mutate authoritative state directly

Any state change implied by language must be represented as a typed, validated action or speech act committed through ordinary event pipelines.

### INV-076 — LLMs may not invent hidden facts or proof

No LLM output may decide truth, create evidence, grant knowledge, correct stale information, create quests, recognize completion, or make institutions accept claims.

### INV-077 — LLM-disabled operation is an acceptance gate

Core mechanics, tests, canonical scenarios, TUI play, records, speech acts, and agent planning must work with all LLM functionality disabled or replaced by deterministic mocks.

## Domain boundary

### INV-078 — The kernel is genre-agnostic

The kernel may know actors, bodies, places, objects, containers, doors, needs, intentions, events, traces, beliefs, perception, memory, speech acts, records, norms, institutions, households, custody, ownership/access claims, travel, LOD, regional processes, replay, and view filtering.

### INV-079 — No fantasy leakage into the kernel

The kernel must not assume magic, monsters, divine authority, feudal law, adventurer classes, species taxonomies, supernatural senses, combat categories, loot tiers, or graphical presentation.

### INV-080 — Domain packs own flavor

Domain packs own technology, metaphysics, species/body types, magic, threats, domain institutions, occupations, items, affordances, terrain, diseases, speech/culture tags, laws, traces, and scenario seeds. Domain packs may not bypass foundation doctrine.

### INV-081 — Special powers are causal channels

Magic, sensors, dreams, rituals, psychic impressions, radios, alien devices, or divine messages must be modeled as channels/actions with cost, reliability, distortion, provenance, traces, counters, and failure modes.

### INV-082 — Detailed combat is deferred from the first serious slice

Early versions may model injury/death aftermath, fear, reports, route avoidance, abstract danger, and institutional response. Granular combat and injury are future systems.

## Scale, LOD, regions, and long history

### INV-083 — Start small enough to inspect

A tiny village with deep causality is better than a large opaque region. The first serious target is roughly 10-30 high-detail agents, or at most a few dozen.

### INV-084 — Scale through honest abstraction

Medium/low detail and aggregate processes must emit structured events or summary events with provenance. They may not hide a director.

### INV-085 — Low-LOD people are still people

Lower detail is dormant or summarized detail, not permission to turn citizens into props. Promoted people need ancestry, identity, household/role context, relationships, obligations, possessions/custody, salient beliefs, and records.

### INV-086 — Promotion and demotion are explicit events

LOD changes log cause, tier, preserved ancestry, fidelity limits, affected claims/records/traces, and replay/debug visibility.

### INV-087 — Human focus is not player privilege

Human proximity, viewport, cursor focus, possession, or debug attention may affect rendering, scheduling priority, and prepared view models. It may not create events, guarantee outcomes, alter probabilities for drama, or make the world wait.

### INV-088 — Regional processes are declared causal processes

Weather fronts, caravans, disease, animal migration, crop pressure, refugees, tax orders, rumors, disasters, road closures, wars, and outside institutions require declared source, cadence/trigger, inputs, random model, scope, delivery channel, traces, affected beliefs/records, local entry events, ancestry, and replay/debug visibility.

### INV-089 — Long prehistory emits structured causes

Long-past events, generated history, and prehistory may use summary simulation. They must produce inspectable ancestry, public memory, stale information, records, traces, and present consequences.

### INV-090 — Demography is staged but protected

Birth, childhood, aging, death, partnership, reproduction, inheritance, migration, disease, employment changes, household formation, succession, settlement growth/decline, and memory/history decay are future domains. Early design must not block them.

## Validation

### INV-091 — No-human tests are mandatory

Every runnable phase must include no-human simulation tests. The world must continue without a human.

### INV-092 — Deterministic replay is tested

Replay and seed determinism must be verified by automated regression tests for significant causal scenarios.

### INV-093 — Actor-knowledge leakage is a high-severity defect

Embodied view models, speech options, notebooks, leads, maps, institutions, and planner queries must be tested against hidden truth leakage.

### INV-094 — Possession parity is tested

Tests must prove that controller binding changes input only and that beliefs, intentions, possessions, obligations, relationships, suspicion, and memory remain with actors/institutions.

### INV-095 — TUI/view-model tests are acceptance tests

Every runnable phase must test that a human can reach mechanics through embodied play or equivalent actor-filtered view models, and that debug can explain truth separately.

### INV-096 — Institutional fallibility is tested

Institution features must include refused, delayed, partial, wrong, stale, biased, misfiled, underfunded, or jurisdiction-blocked outcomes where relevant.

### INV-097 — No-script compliance is tested

Scenario content and systems must be inspected for hidden outcome chains, player-conditioned event injection, objective markers, guaranteed target existence, guaranteed reward payment, and storyteller causation.

### INV-098 — Feature acceptance is harsh

A runnable feature is done only when it is caused, agent-possible, eventful, trace-aware, epistemically bounded, TUI-playable, debug-inspectable, no-human runnable, replay-safe, LLM-independent, non-scripted, and regression-tested.

## 2026 hardening invariants: truth firewall and cognition authority

These invariants are added because implementation work on needs, routines, no-human simulation, and anti-contamination exposed a gap: earlier foundation doctrine correctly rejected omniscience, but did not make the actor decision boundary explicit enough to prevent convenient scheduler/planner shortcuts.

### Truth firewall

**INV-099 — Truth may validate actions, but truth may not plan them.** Authoritative world state may decide whether an attempted action succeeds, fails, or is rejected. Authoritative hidden truth may not select an actor's goal, plan, routine, speech interpretation, embodied affordance, or institutional conclusion unless it has entered the relevant holder's actor-known or institution-known context through a modeled causal information path.

**INV-100 — Hidden-truth cognition is forbidden.** No ordinary actor, possessed actor, autonomous agent, routine, HTN method, local planner, institution, notice process, speech parser, or LLM surface may read debug truth or hidden world state as cognition input.

**INV-101 — Actor-known context is sealed.** Action proposal generation must consume a sealed actor-known context or directly equivalent boundary. The context must identify what the actor currently perceives, remembers, believes, expects, intends, is obliged to do, has been assigned, has read, has heard, or can infer under modeled rules. It must not contain validator-only truth.

**INV-102 — Cognition inputs require provenance.** Every belief, memory, observation, claim, expectation, contradiction, actor-known affordance, routine premise, institutional assignment, rumor, notice, report, record, and action-relevant inference must carry source/provenance sufficient for replay and debug review. Missing provenance is a design smell and, for action-relevant cognition, a rejection condition.

**INV-103 — The scheduler is not a cognition authority.** A scheduler may choose the next actor/time window, apply due scheduled effects, and invoke the decision transaction. It must not construct action proposals from raw state, routine labels, need thresholds, true item locations, true workplace locations, true route availability, true danger, or true institutional outcomes.

**INV-104 — Routines and needs do not dispatch primitive actions directly.** Needs create pressures; routines provide defeasible method structure; intentions commit attention. None of them may bypass candidate generation, actor-known context, local planning, proposal construction, shared validation, and event commitment.

**INV-105 — Actor decision traces are authoritative diagnostic data.** Decision traces, stuck diagnostics, hidden-truth audits, why-not reports, routine/intention lifecycle changes, and planner outputs must be typed or structurally inspectable. Display strings may summarize them, but display strings must not be the authoritative diagnostic substrate.

**INV-106 — Validation failure feeds replanning without leakage.** When a proposal fails validation against authoritative truth, the actor may learn only modeled consequences: rejection reasons available to the actor, observations, failed attempts, social responses, or newly perceived blockers. Validator-only details remain debug-only unless a causal information channel exposes them.

**INV-107 — Debug omniscience is quarantined.** Debug mode may compare actor belief to truth, inspect provenance graphs, inspect hidden state, and flag leakage. Debug output must not create actor knowledge, institution knowledge, records, rumors, notices, speech acts, or future plans unless an explicit debug/test event is marked as non-ordinary.

**INV-108 — Human possession is cognition-neutral.** Binding a human controller to an actor changes input and viewpoint only. It does not grant hidden-truth planning, transfer knowledge from other possessions, reset intentions, improve routine competence, or create player-only candidate actions.

**INV-109 — LLM output is never cognition authority without validation.** LLMs may render, parse, summarize, or propose structured speech acts behind validation. They may not introduce hidden truth, choose world actions, mutate state, create records, decide institutional outcomes, or plan from information outside the actor-filtered prompt packet.

**INV-110 — LOD and summary processes must preserve the firewall.** Lower-detail simulation may summarize action and cognition, but summary events must retain enough causal and epistemic ancestry to explain what the actor, group, institution, or region could know and why later promoted simulation is not contaminated by hidden truth.

### Emergence acceptance

**INV-111 — Living-world acceptance requires observer-only emergence evidence.** Tracewake's living-world claim is not accepted merely by proving that ordinary-life phenomena are statically reachable. Acceptance must include replayable, observer-only evidence that unscripted ordinary-life phenomena actually arise from modeled causes in no-human or normal-controller runs. That evidence must be retrospective, explainable through event-log ancestry, and unable to feed simulation behavior, author outcomes, or set dramatic objectives.

### Temporal authority

**INV-112 — Time may validate, but holder-known time must plan.** This temporal authority invariant makes authoritative simulation time, event order, intervals, durations, and due effects available to validate replay, ordering, action legality, scheduled consequences, and causal explanation. Cognition, routine selection, institutional procedure, embodied view models, speech interpretation, leads, and LOD promotion may use temporal facts only when those facts are available to the relevant actor, household, institution, group, or region through modeled channels. Deadline, lateness, staleness, expected-by-now, yesterday, and office-closed states are claims, procedure states, or holder/institution-known interpretations with provenance, not free truth labels. The scheduler and replay clock may order and validate; they must not become cognition authority.

## Enforcement reading

Any implementation that produces plausible behavior by reading hidden truth has failed even if the test passes. Tests must be corrected when they reward:

- actor proposals derived from raw physical state;
- institutions reacting to true events rather than reports, observations, records, or procedures;
- embodied views generated from truth rather than actor-known context;
- routine labels that dispatch primitive actions without actor-known planning;
- debug strings parsed as proof;
- LLM prose treated as fact;
- no-human simulation that advances only because shortcut state is available.
