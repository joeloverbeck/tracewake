# Scale, LOD, Regional Boundaries, and Long Simulation

## Core claim

Tracewake has a long-term fantasy of worlds with history: aging, migration, marriage/partnership, birth, death, inheritance, employment changes, abandoned houses, stale records, forgotten crimes, buried evidence, regional disasters, institutions outliving individuals, and settlements shaped by decades of causes.

V1 must earn that fantasy by starting small and inspectable.

Scale is allowed only through honest abstraction. Low-detail simulation may reduce fidelity, but it may not hide a director, erase ancestry, turn people into props, or privilege the human.

## Start small

The first serious vertical slice should be a small village:

```text
10-30 high-detail agents
or at most a few dozen
```

A tiny village with deep causality is better than a large opaque region.

The early village should run for days without human input while producing ordinary routines, need satisfaction, mistakes, reports, stale information, wrong beliefs, institutional failures, and inspectable causal chains.

## LOD doctrine

Level of detail is part of the simulation contract, not just optimization.

Tracewake may represent entities and processes at different fidelity levels:

```text
high-detail actor
  local beliefs, needs, intentions, action planning, perception, speech, memory, traces

medium-detail actor
  summarized routines, reduced perception, periodic decision events, salient beliefs/projects

low-detail actor
  structured summary events, coarse needs/roles/relations/obligations, promotable ancestry

aggregate population
  demographic/economic/social pressure events with provenance

regional process
  declared source, cadence, inputs, outputs, traces, scope, affected claims/records
```

Lower detail still produces structured events. It is not lore prose.

## High-detail where it matters

High detail is required where local causality, TUI playability, actor knowledge, possession, records, physical traces, and ordinary actions matter.

Early high-detail focus includes:

- possessed actor;
- nearby ordinary actors;
- first-proof households and work sites;
- rooms/containers/property involved in missing-property proof;
- actors/institutions involved in reports, wrong suspicion, or records;
- local traces and searches;
- speech participants;
- debug inspection targets.

High detail is not “because the player is special.” It is because local causal explanation requires fidelity.

## Low-LOD people are still people

A low-LOD person may have fewer simulated moments, but the system must preserve enough ancestry to answer later:

- where did this person come from;
- what household, role, relationships, and obligations matter;
- what possessions/custody/claims exist;
- what salient beliefs, debts, rumors, records, or promises mention them;
- what summary events shaped them;
- what traces or claims they carry;
- what must initialize if promoted;
- what causal debts cannot be erased.

Promoting a low-LOD person into high detail must not create a prop from nothing.

## Summary events

Summary simulation emits structured causal events.

Good:

```yaml
RegionalMigrationSummary:
  source_region: north_road_villages
  interval: 142-07-01_to_142-08-01
  cause_inputs:
    - winter_food_pressure
    - road_safety_decline
    - mill_labor_demand
  affected_households:
    - household_h17
    - household_h22
  migrants:
    - person_ref_or_promotable_seed_mara_cousin
  records_created:
    - entry_ledger_ref
    - household_letter_ref
  belief_effects:
    - rumor_famine_north
  random_draws: audited
  fidelity_limits: no_individual_daily_perception
```

Bad:

```text
Lore: many people came from the north because times were hard.
```

Summary events may be coarse. They must still have source, inputs, outputs, provenance, fidelity limits, and replay/debug visibility.

## Promotion and demotion

LOD promotion/demotion must be explicit and logged.

Promotion may be caused by:

- possession;
- local physical proximity;
- causal relevance to a high-detail event;
- social relevance to a high-detail actor;
- institutional relevance;
- record/evidence relevance;
- travel/path relevance;
- debug inspection;
- scheduled transition into a higher-fidelity region;
- risk of future interaction with active claims/traces.

Demotion may be caused by:

- leaving local detail area;
- reduced causal relevance;
- stable routine continuation;
- performance budget constraints;
- transition into summarized regional process.

Promotion/demotion must not be caused by dramatic pacing or boredom.

A promotion event preserves ancestry from summary state. A demotion event summarizes open threads, beliefs, obligations, property claims, records, relationships, and traces.

## Human focus and fairness

Human proximity, viewport, cursor focus, or possession may affect:

- rendering;
- UI inspection;
- scheduler interest;
- prepared view models;
- debug verbosity;
- which actors are convenient to possess;
- performance allocation.

Human focus must not:

- create events;
- guarantee outcomes;
- alter probabilities for drama;
- make the world wait;
- protect the current actor from ordinary consequences;
- make nearby NPCs more real in ways that unfairly change authoritative high-level outcomes;
- erase off-screen causality.

If performance requires focus-sensitive fidelity, the policy must be explicit, deterministic, auditable, and forbidden from changing high-level outcomes unfairly.

## Regional and exogenous processes

Regional processes are allowed only as declared causal machinery.

A regional process must define:

```text
source
cadence or trigger
input state
random model if any
domain authority
scope
LOD tier/fidelity
delivery channel
local entry event
traces
affected actors/places/households/institutions/entities
affected claims/beliefs/records
causal ancestry
replay/debug visibility
```

Examples:

- weather fronts;
- caravans;
- tax orders;
- harvest/crop pressure;
- road closures;
- migration/refugee waves;
- disease pressure;
- animal migration;
- market price pressure;
- bandit or patrol movement;
- outside institutional orders;
- disasters;
- distant war pressure.

A regional process is never a disguised storyteller.

## Boundary imports

When a regional process affects the local village, it enters through local events:

```text
courier arrives with tax order
rain begins and changes road/trace decay
caravan reaches village square
rumor reaches tavern through traveler speech
refugee household requests shelter
disease sign observed by healer or household
road closure notice posted or reported
```

The local world learns through perception, records, speech, traces, and procedure. It does not receive omniscient regional updates.

## Travel and routes

Travel is a bridge between local detail and regional abstraction.

Travel must preserve:

- actor belief about destination and route;
- departure event;
- route choice;
- time cost;
- fatigue/hunger/safety costs;
- risk events;
- observations and missed observations;
- missed meetings;
- arrival event;
- route traces where relevant;
- stale route information;
- LOD transitions if travel leaves local detail.

Off-screen travel may be summarized. It must not become teleportation that erases causality.

## Long prehistory

Long prehistory is a core long-term goal. A player should eventually enter a world that already has causes.

Prehistory may use multi-resolution simulation and summary events. It must not be ungrounded lore prose when it affects play.

Acceptable prehistory outputs include:

- settlement founding events;
- household formation/dissolution;
- births, deaths, partnerships, inheritance, and migration summaries;
- work and institution succession;
- property transfers;
- abandoned houses;
- public records;
- stale notices;
- unresolved accusations;
- remembered and misremembered events;
- regional pressures;
- route changes;
- buried, damaged, or destroyed traces.

The early project should not attempt full prehistory before the village works. The kernel must avoid choices that would block it.

## Demography and settlement history

Tracewake should eventually support:

- birth;
- childhood;
- aging;
- death;
- marriage/partnership;
- reproduction;
- inheritance;
- migration;
- disease;
- employment change;
- household formation;
- household breakdown;
- settlement growth;
- settlement decline;
- institutional succession;
- memory and record decay.

V1 does not need full demographic realism. It needs ordinary life and household structure that can later become demographic simulation without replacement.

## Animals, disease, weather, and disasters

These long-term systems must be causal processes, not events for drama.

Animals need bodies, needs, territory/range, perception, fear, reproduction/migration hooks, traces, and human interaction rules appropriate to their domain fidelity.

Disease needs carriers, exposure, symptoms, belief/record effects, treatment/procedure, uncertainty, stigma, and spread/decay models.

Weather needs region process state, local entry events, route/perception/trace effects, forecasts/rumors/records if known, and replayable randomness.

Disasters need physical causes, warning channels, damage events, injuries/deaths, records, rumors, institutional response, migration/refugees, and long consequences.

## Combat and injury staging

Detailed combat is deferred.

Early versions may contain injury or death only through abstract events, accidents, illness, travel risk, violence aftermath, fear, reports, traces, avoidance, and institutional response.

The foundation leaves room for later combat with wounds, broken bones, pain, disability, treatment, healing, death, evidence, trauma, reputation, law, and institutional consequences.

Detailed combat must be its own serious system. It must not dominate the first ordinary-life village.

## Organizations and conflict at scale

Future large social processes must be built from lower-level machinery.

A caravan is route, stock, roles, contracts, risk, animal/vehicle state, weather, rumors, records, and arrival/departure.

A guild is membership, roles, work standards, dues, sanctions, records, reputation, apprenticeships, and politics.

An army is recruitment, pay, supply, morale, command, loyalty, fear, desertion, wounds, travel, orders, records, rumor, and logistics.

Conquering a settlement is not a special quest primitive. It is territorial control emerging from violence, authority, logistics, fear, negotiation, records, rumor, collaborators, resistance, and aftermath.

## Acceptance checks

Scale, LOD, and regional features are not done unless they answer:

```text
What fidelity level produced this state?
What event or summary event changed it?
What ancestry survived abstraction?
What random draws affected it?
What promoted or demoted the entity/process?
Could this happen without a human present?
Did human focus alter authoritative outcomes unfairly?
What traces, records, or beliefs crossed the LOD boundary?
What fidelity limits remain?
Can debug mode reconstruct the causal path?
```
