# LOD, Regional Processes, World History, and Scale

## Status

This document defines explicit level of detail, regional processes, long simulation, world history, boundary events, and scale staging.

Long simulation matters, but it must be staged. Do not generate a vast world before the village can explain a stolen coin.

## Core rule

LOD may reduce fidelity. It may not turn people into props, erase active causal ancestry, hide a drama director, or make the world wait for the player.

Human proximity, possession, viewport, or debug focus may influence rendering, inspection, scheduler attention, or detail promotion. They must not create events, guarantee outcomes, alter probabilities for drama, or pause ordinary causality.

## Staging order

Recommended staging:

1. hand-authored causal village;
2. hand-authored nearby route/site only if needed;
3. no-human simulation over days;
4. explicit LOD over settlement/regional processes;
5. longer history/prehistory;
6. procedural support only after ontology is proven.

The first proof remains missing property/theft. Road-threat/bounty/expedition is second proof.

## Simulation-scope tiers

These are simulation-scope tiers, separate from per-agent detail tiers.

```text
Tier 0: embodied/local detailed simulation
Tier 1: active settlement simulation
Tier 2: regional process simulation
Tier 3: historical/external abstraction
```

### Tier 0 — embodied/local detailed simulation

Used for current actor location, active scenes, high-salience investigations, immediate interactions, and debug-focused local replay.

Full detail:

- action pipeline;
- perception;
- traces;
- speech;
- local movement;
- reservations;
- beliefs;
- local planning;
- detailed event commit.

### Tier 1 — active settlement simulation

Used for village life outside current local scene.

Includes:

- routines;
- shops/workplaces;
- households;
- reports;
- notices;
- rumor spread;
- simplified movement;
- salient traces;
- institutional procedures;
- defeasible schedules;
- ordinary economy.

### Tier 2 — regional process simulation

Used for routes and nearby regional influences.

Includes:

- caravans/travelers;
- route risk;
- patrols;
- migration pressure;
- supply delay;
- disease later;
- message movement;
- trade/resource aggregates;
- population/resource changes;
- summary travel;
- regional rumors.

### Tier 3 — historical/external abstraction

Used for long spans and outside world.

Includes:

- seasonal summaries;
- boundary events;
- external famine/war/order/market pressure;
- coarse population/economic/ecological shifts;
- long history generation;
- public history/memory drift.

## Promotion triggers

Promote detail when:

- embodied actor enters or approaches;
- actor is possessed;
- debug focus requests inspection;
- significant event occurs;
- contradiction arises;
- institution opens or references a case;
- record/testimony cites an event;
- a lead points to a place or chain;
- trace persistence matters;
- route/site becomes active;
- low-LOD actor becomes socially or causally important;
- planned action requires detail.

Never promote because the player is bored.

## Demotion triggers

Demote when:

- scene is inactive;
- no unresolved local causal chains remain;
- no actor nearby needs detailed perception;
- no active trace requires local detail;
- institutional/lead references are summarized and preserved;
- routine spans can be batched;
- snapshot/projection checkpoints are safe;
- debug focus moves away.

Demotion creates summary events where state changes happened.

## LOD transition events

Promotion and demotion are events or replay-visible metadata when they affect simulation results.

```yaml
LODTransitionEvent:
  kind: LODPromoted
  target: actor_mara
  from_tier: AgentDetailC
  to_tier: AgentDetailA
  reason: debug_possession_or_active_investigation
  ancestry_sources:
    - summary_mara_week_routine_03
    - debt_project_mara_01
  restored_state:
    beliefs: sourced_summary
    needs: current_projection
    projects: active_project_refs
```

LOD transitions must be inspectable.

## Summary events

Summary events compress low-salience activity while preserving ancestry.

```yaml
SummaryEvent:
  id: summary_household_mara_day_142_08_11
  kind: HouseholdRoutineDay
  actors: [actor_mara, actor_iva]
  interval: 142-08-11
  effects:
    - food_consumed_small
    - fatigue_recovered
    - debt_pressure_increased
  causal_links:
    - debt_due_market_day
  retained_details:
    - argument_about_debt_became_salient
```

If a summarized event later becomes important, debug must show the summary's retained causal facts and limits.

## Low-LOD people are still people

A low-LOD person must have at least:

- identity;
- body/basic condition;
- home or reason for homelessness;
- occupation/survival strategy;
- household/relationship context;
- role if any;
- needs summary;
- salient beliefs;
- possessions/custody summary;
- current routine/project summary;
- promotable ancestry.

They are not crowd sprites or quest props.

## Boundary events

Outside the active region exists through boundary processes.

Examples:

- caravan arrives with goods and rumors;
- family migrates in because of famine elsewhere;
- tax order arrives from superior authority;
- disease enters through travelers;
- deserter group enters route network;
- market price pressure changes local work;
- relative dies elsewhere and inheritance claim arrives.

Boundary events require summarized causes. They are not variety injections for pacing.

```yaml
BoundaryEvent:
  kind: MigrantFamilyArrived
  source_region: outside_west
  summarized_causes:
    - harvest_failure
    - kinship_contact_in_village
  arrived_entities: [household_aren]
  information_carried:
    - rumor_western_famine
  event_time: 142-09-02
```

## World history

Future long simulation may create:

- births/deaths/marriages/separations;
- migrations;
- property transfers;
- old crimes;
- institutional grudges;
- closed shops;
- stale notices;
- public histories;
- rumor distortions;
- debts;
- road risk shifts;
- relationship histories.

History must remain causal. Public memory and actual history are distinct.

Bad:

```text
Mara hates Tomas.
```

Good:

```text
Mara distrusts Tomas because Tomas testified against her brother in a debt dispute three years ago.
```

Even summarized causes should exist.

## Procedural generation stance

Procedural generation is authoring support after ontology proof.

Generated content must obey the same rules as authored content:

- records have authors;
- rumors have origins;
- threats have processes/traces;
- institutions have roles/resources/procedures;
- people have homes or reasons;
- actions use the pipeline;
- beliefs have provenance;
- no quest objects appear;
- no hidden outcome chains.

Generate causal seeds, not story scripts.

## Scale policy

Prefer small and causal over vast and hollow.

A good first region:

- one deep village;
- one local route if needed;
- a few boundary process hooks;
- records and rumors with ancestry;
- enough people for social density;
- no-human days of ordinary life;
- replayable event history.

A bad first region:

- many settlements with no households;
- large maps with no epistemics;
- procedural lore that cannot affect play;
- hundreds of NPCs with no causal ancestry;
- road threats before domestic property works.

## No hidden drama director

LOD systems must not become pacing systems.

Forbidden:

- spawn incident because the player has seen nothing interesting;
- demote culprit to prevent discovery;
- promote threat because pacing is slow;
- pause NPC until player arrives;
- alter route risk because player accepted a lead;
- erase trace to make mystery harder;
- create clue to make mystery solvable.

Allowed:

- promote place because actor travels there;
- promote chain because institution opened a record;
- summarize routine day with causal links;
- run regional process that changes route risk for modeled reasons;
- observer sifter highlights interesting events after they occur.

## Acceptance implications

LOD/history features must test:

- promotion/demotion events or metadata are replay-visible;
- summary events preserve active causal ancestry;
- low-LOD actors are promotable;
- no-human simulation runs across LOD tiers;
- player proximity does not create events;
- boundary events have summarized causes;
- public history differs from truth when appropriate;
- debug can explain what detail was lost and retained;
- projections rebuild after compaction/snapshot;
- active leads/records/traces prevent unsafe compaction.

## Anti-patterns

- Every agent full-detail forever.
- Low-LOD actors become props.
- Summaries erase wrong-belief origins.
- Hidden director creates incidents.
- Player viewport changes probabilities.
- History is prose only.
- Procedural generation before hand-authored ontology.
- Boundary events injected for variety.
- Vast world cannot explain household theft.
