# Scale, LOD, Long Simulation, and Regional Processes

## Core claim

Tracewake has a long-term fantasy of worlds with history: aging, migration, marriage, death, inheritance, employment changes, abandoned houses, stale records, forgotten crimes, buried evidence, and institutions outliving individuals.

V1 must earn that fantasy by starting small and inspectable.

Scale is allowed only through honest abstraction. Low-detail simulation may reduce fidelity, but it may not hide a drama director, erase causal ancestry, turn people into props, or create player privilege.

## Start small enough to understand

The first serious vertical slice should be a small village, not a huge opaque region.

A good early target is approximately:

```text
10-30 high-detail agents
or at most a few dozen
```

This is not a permanent population cap. It is a quality constraint.

A tiny village with deep causality is better than a large region full of props.

The early village should be able to run for days without human input while producing ordinary routines, needs, mistakes, reports, stale information, institutional failures, and inspectable causal chains.

## LOD doctrine

Level of detail is part of the simulation contract, not an afterthought.

The system may represent different entities and processes at different fidelity levels:

```text
high-detail actor        full local beliefs, needs, intentions, action planning, perception
medium-detail actor      summarized routines, reduced perception, periodic decision events
low-detail actor         structured summary events, coarse needs/roles/relations
aggregate population     demographic/economic pressure events with ancestry
regional process         declared source, cadence, inputs, outputs, traces, scope
```

Lower detail must still produce structured events. It must not produce lore prose as a substitute for causality.

## Low-LOD agents are still people

Low-LOD is dormant or summarized detail. It is not permission to treat citizens as props.

A low-LOD agent may have fewer simulated moments, but the system must preserve enough ancestry to answer later:

- where did this person come from;
- what household, role, and relationships matter;
- what obligations or claims existed;
- what summary events shaped them;
- what records or rumors mention them;
- what beliefs should be initialized if promoted;
- what causal debts cannot be erased.

Promoting a low-LOD person into high detail must not create them from nothing.

## Summary events

Summary simulation must emit structured causal summary events.

Good:

```text
RegionalMigrationSummary
  source_region: North Road villages
  cause_inputs: winter food pressure, road safety, employment demand
  affected_households: H17, H22
  migrants: abstract persons promoted or referenced
  records_created: entry ledger, household letter
  belief_effects: rumors of famine
  random_draws: audited
```

Bad:

```text
Lore: many people came from the north because times were hard.
```

Summary events may be coarse. They must still have source, inputs, effects, provenance, and replay/debug visibility.

## Promotion and demotion

LOD promotion/demotion must be explicit and logged.

Promotion may be caused by:

- possession;
- local physical proximity;
- causal relevance to a high-detail event;
- social relevance to a high-detail actor;
- institutional relevance;
- record or evidence relevance;
- travel/path relevance;
- debug inspection;
- scheduled transition to a higher-fidelity region.

Demotion may be caused by:

- leaving the local detail area;
- reduced causal relevance;
- stable routine continuation;
- performance budget constraints;
- transition into a summarized regional process.

Promotion/demotion must not be caused by dramatic pacing or boredom.

A promotion event must preserve ancestry from summary state. A demotion event must summarize meaningful open threads, beliefs, obligations, property claims, records, and traces.

## Human focus and fairness

Human proximity, viewport, cursor focus, or possession may affect:

- rendering;
- UI inspection;
- scheduler interest;
- debug verbosity;
- which view models are prepared;
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

## Long prehistory

Long prehistory is a core long-term goal. It should eventually allow the player to enter a world that already has causes.

Prehistory may use multi-resolution simulation and summary events. It must not be generated as ungrounded lore prose.

Acceptable prehistory outputs include:

- settlement founding events;
- household formation and dissolution;
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
- buried or destroyed traces.

The early project should not attempt full prehistory before the small village works. The kernel must simply avoid choices that would block it.

## Demography and settlement history

Tracewake should eventually support:

- birth;
- childhood;
- aging;
- death;
- marriage or partnership;
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

V1 does not need full demographic realism. It needs ordinary life and household structure that can later become demographic simulation without being replaced.

## Regional and exogenous processes

Regional processes are allowed only when declared as causal machinery.

A regional process must define:

```text
source
cadence or trigger
input state
random model if any
domain authority
scope
delivery channel
local entry event
traces
affected beliefs
affected records
causal ancestry
replay/debug visibility
```

Examples:

- weather;
- caravan arrival;
- tax order;
- harvest pressure;
- road closure;
- migration wave;
- disease pressure;
- market price pressure;
- bandit movement;
- institutional order from a larger authority;
- distant war pressure in a future domain.

A regional process is never a disguised storyteller.

## Travel and route abstraction

Travel is foundational because ordinary life is spatial and evidence is local.

Early travel may be simple, but it must preserve:

- route choice;
- time cost;
- fatigue and hunger costs;
- risk events;
- observation opportunities;
- missed meetings;
- arrival/departure events;
- traces where relevant;
- actor beliefs about destination and route;
- stale route information.

Off-screen travel may be summarized, but it must not become teleportation that erases causality.

## Combat and injury staging

Detailed combat is deferred.

Early versions may contain injury or death only through abstract events, accidents, illness, travel risk, violence aftermath, fear, reports, traces, avoidance, and institutional response.

The foundation must leave room for later detailed combat with:

- wounds;
- broken bones;
- missing limbs;
- pain;
- disability;
- treatment;
- healing;
- death;
- evidence;
- social and institutional consequences.

Detailed combat must be its own serious future system. It must not dominate the first ordinary-life village.

## Acceptance implications

Scale, LOD, and regional features are not done unless they can answer:

```text
What fidelity level produced this state?
What event or summary event changed it?
What ancestry survived abstraction?
What random draws affected it?
What promoted or demoted the entity/process?
Could this happen without a human present?
Did human focus alter authoritative outcomes unfairly?
What traces, records, or beliefs crossed the LOD boundary?
Can debug mode reconstruct the causal path?
```
