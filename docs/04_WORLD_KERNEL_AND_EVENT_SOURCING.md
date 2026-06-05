# World Kernel and Event Sourcing

## Purpose

The world kernel is the authoritative machine that decides what is true. It must be deterministic where practical, event-sourced, inspectable, and hostile to shortcuts.

Everything else is a projection: inventories, beliefs, memories, rumors, records, case files, UI views, generated prose, world history summaries, save files, and analytics.

## Kernel responsibilities

The kernel owns entity identity, component state, primitive action definitions, action proposals, precondition checks, effects, event creation, event ordering, simulation time, random stream discipline, spatial occupancy, ownership, permissions, norm hooks, perception hooks, trace generation, causal links, replay, snapshots, and LOD promotion events.

The kernel does not own prose, dramatic pacing, quest arcs, authorial intent, player importance, boredom detection, or omniscient interpretation.

## Core entities

Examples:

- actor;
- body;
- item;
- container;
- room;
- road segment;
- settlement;
- region;
- notice;
- record;
- institution;
- trace;
- corpse;
- fire;
- contract;
- disease outbreak;
- caravan;
- animal group;
- rumor packet;
- external influence process.

## Component examples

```yaml
Position:
  place: room_tomas_bedroom

Ownership:
  legal_owner: actor_tomas
  possessor: actor_mara
  expected_location: strongbox_tomas
  permitted_users: [actor_tomas, actor_elena]

Container:
  locked: true
  contents: [coin_stack_01]
  lock_state: intact

ActorBody:
  hunger: 22
  fatigue: 63
  pain: 0
  injuries: []

MindRef:
  belief_store: belief_store_tomas
  intention_stack: intention_stack_tomas
```

## Action model

An action is a proposed state transition with actor, parameters, physical preconditions, social preconditions, knowledge preconditions, institutional preconditions, cost, duration, outcomes, effects, traces, observation hooks, and norm implications.

Example:

```yaml
Action: RemoveItemFromContainer
actor: actor_mara
parameters:
  container: strongbox_tomas
  item: coin_stack_01
preconditions:
  physical:
    - actor_at_container_location
    - container_open_or_actor_can_open
    - item_present
    - actor_can_carry
  social:
    - actor_has_permission OR actor_willing_to_violate_property_norm
  knowledge:
    - actor_perceives_item OR actor_searching_container
effects:
  - possession_changes_to_actor
  - item_removed_from_container
traces:
  - disturbed_container
  - possible_noise
norm_checks:
  - theft
```

## Event model

An event is an immutable record of something that happened, was attempted, failed, was observed, was inferred, was recorded, or was communicated.

```yaml
Event:
  id: evt_018239
  kind: ItemRemovedFromContainer
  world_tick: 81230
  sim_time: 142-08-12T02:13
  actor: actor_mara
  participants:
    container: strongbox_tomas
    item: coin_stack_01
  place: room_tomas_bedroom
  causes:
    - intention_778
    - evt_018232
  effects:
    - path: coin_stack_01.Ownership.possessor
      old: strongbox_tomas
      new: actor_mara
  traces_created:
    - trace_disturbed_container_44
    - trace_faint_noise_12
  observations_created:
    - obs_elena_heard_noise_12
```

## Event streams

Use related streams:

```text
WorldStream              globally ordered significant events
EntityStream(entity)     events involving one entity
ActorStream(actor)       caused/perceived/believed events
PlaceStream(place)       events and traces at a place
InstitutionStream(inst)  records, reports, orders, sanctions
ItemStream(item)         possession, custody, damage, movement
RouteStream(route)       travel, attacks, closures, patrols
RegionStream(region)     migration, disease, trade, politics
RumorStream(topic)       rumor propagation and mutations
```

## Action pipeline

1. proposal;
2. authority check;
3. physical precondition check;
4. social/normative precondition check;
5. knowledge/belief precondition check;
6. reservation check;
7. cost/risk check;
8. execution;
9. outcome resolution;
10. state mutation;
11. trace generation;
12. observation pass;
13. belief update;
14. institutional hooks;
15. consequence scheduling;
16. event commit.

Player and NPC actions use the same pipeline.

## Causal graph

Cause types include enabling condition, actor intention, direct physical cause, information cause, institutional procedure, relationship motive, environmental process, regional process, exogenous boundary process, random seed branch, and prior event creating motive or opportunity.

Example:

```text
bandits_attack_traveler
 -> survivor_observes_attack
 -> survivor_reports_to_clerk
 -> clerk_records_report
 -> reeve_believes_road_threat
 -> reeve_reserves_bounty_funds
 -> clerk_writes_notice
 -> guard_posts_notice
 -> oren_reads_notice
 -> oren_forms_intention_to_recruit_help
```

## Snapshots and compaction

Long simulations need snapshots. Preserve detailed events for significant chains, traces, belief changes, records, and active leads. Compact low-salience routine spans into summary events with causal links.

A meal that created the last sighting of a murder victim is no longer routine.

## Determinism

Target deterministic replay:

- seeded random streams;
- stable iteration order;
- explicit event ordering;
- no wall-clock decisions;
- no uncontrolled LLM calls in authoritative simulation;
- random draws recorded in debug builds;
- versioned data files.

## Forbidden shortcuts

No inventory mutation without event. No guard knowledge from global truth. No notices without author, issuer, funds, and posting event. No objective markers to true locations from rumors. No player-only actions. No generated prose creating facts. No quest completion as truth.
