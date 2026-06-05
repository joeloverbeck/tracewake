# World Kernel and Event Sourcing

## Purpose

The world kernel is the authoritative machine that decides what is true. It must be deterministic where practical, event-sourced, inspectable, and hostile to shortcuts.

Everything else is a projection: inventories, beliefs, memories, rumors, records, case files, notices, lead cards, UI views, generated prose, history summaries, analytics, and save/load convenience layers.

## Kernel responsibilities

The kernel owns:

- entity identity;
- component state;
- primitive action definitions;
- action proposals;
- precondition checks;
- reservation checks;
- outcome resolution;
- state mutation;
- event creation and ordering;
- simulation time;
- random stream discipline;
- spatial occupancy;
- ownership and custody;
- permissions and norm hooks;
- perception hooks;
- trace generation;
- belief update hooks;
- institutional hooks;
- causal links;
- replay;
- snapshots;
- LOD promotion/demotion events.

The kernel does not own:

- prose as truth;
- dramatic pacing;
- quest arcs;
- authorial outcome intent;
- player importance;
- boredom detection;
- omniscient interpretation;
- graphical presentation;
- LLM authority.

## Event-sourced doctrine

A meaningful world mutation is committed by event, not by hidden assignment. Current state is a projection of event history plus snapshots.

This is required for:

- replay;
- auditability;
- causal debugging;
- belief and record provenance;
- no-player simulation validation;
- institutional history;
- story sifting as observation;
- reconstructing past state;
- explaining “why could this happen?”;
- proving player and NPC action parity.

## Core entity kinds

Examples:

- actor;
- body;
- item;
- container;
- door;
- room;
- building;
- road segment;
- route;
- settlement;
- region;
- notice;
- record;
- institution;
- role;
- norm;
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
  custodian: null
  expected_location: strongbox_tomas
  permitted_users: [actor_tomas, actor_elena]

Container:
  locked: true
  contents: [coin_stack_01]
  lock_state: intact
  privacy_level: household_private

ActorBody:
  hunger: 22
  fatigue: 63
  safety: 77
  pain: 0
  injuries: []

MindRef:
  belief_store: belief_store_tomas
  intention_stack: intention_stack_tomas

Door:
  connects: [room_tomas_bedroom, hall_tomas_house]
  open_state: closed
  lock_state: unlocked
  sound_occlusion: medium
```

## Action model

An action is a proposed state transition with actor, parameters, physical preconditions, social/normative preconditions, knowledge preconditions, resource requirements, cost, duration, possible outcomes, effects, traces, observation hooks, and norm implications.

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
    - actor_perceives_item OR actor_is_searching_container
  resource:
    - enough_time_before_interruption
cost:
  time: 20s
  attention: medium
effects:
  - possession_changes_to_actor
  - item_removed_from_container
traces:
  - disturbed_container
  - possible_noise
norm_checks:
  - theft
failure_modes:
  - item_not_found
  - interrupted
  - caught_in_act
```

Player and NPC actions use the same definitions.

## Action pipeline

Every world-affecting action goes through the same pipeline:

1. proposal;
2. actor authority check;
3. physical precondition check;
4. social/normative precondition check;
5. knowledge/belief precondition check;
6. reservation and conflict check;
7. cost/risk check;
8. execution scheduling;
9. outcome resolution;
10. state mutation;
11. trace generation;
12. observation pass;
13. belief update hooks;
14. institutional hooks;
15. consequence scheduling;
16. event commit;
17. projection/view-model update.

The pipeline may reject or fail an action. Rejection/failure is itself event-worthy when future reasoning may care.

## Event model

An event is an immutable record of something that happened, was attempted, failed, was observed, was inferred, was recorded, was communicated, or was summarized.

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
    - belief_mara_tomas_stores_coins
    - need_mara_debt_pressure
    - evt_018232_door_opened
  preconditions:
    - actor_at_container_location: true
    - container_open_or_actor_can_open: true
    - item_present: true
    - actor_has_permission: false
    - actor_willing_to_violate_property_norm: true
  effects:
    - path: coin_stack_01.Ownership.possessor
      old: strongbox_tomas
      new: actor_mara
  traces_created:
    - trace_disturbed_container_44
    - trace_faint_noise_12
  observations_created:
    - obs_elena_heard_noise_12
  norm_implications:
    - possible_theft_violation
```

## Event streams

Use related streams to support efficient queries and forensic debugging:

```text
WorldStream              globally ordered significant events
EntityStream(entity)     events involving one entity
ActorStream(actor)       caused/perceived/believed/intended events
PlaceStream(place)       events and traces at a place
InstitutionStream(inst)  records, reports, orders, sanctions
ItemStream(item)         possession, custody, damage, movement
RouteStream(route)       travel, attacks, closures, patrols, rumors
RegionStream(region)     migration, disease, trade, politics
RumorStream(topic)       rumor propagation and mutations
ControllerStream(human)  debug binding history, not world truth
```

## Causal graph

Cause types include:

- enabling condition;
- actor intention;
- need pressure;
- value/motive;
- direct physical cause;
- information cause;
- institutional procedure;
- role obligation;
- relationship motive;
- environmental process;
- regional process;
- exogenous boundary process;
- random seed branch;
- prior event creating motive or opportunity;
- LOD summary ancestry.

Example:

```text
mara_debt_pressure
 -> mara_believes_tomas_has_coins
 -> mara_forms_steal_intention
 -> mara_enters_house
 -> mara_opens_strongbox
 -> mara_removes_coins
 -> elena_hears_noise
 -> tomas_later_checks_strongbox
 -> expectation_contradiction
 -> tomas_forms_recover_property_goal
 -> tomas_reports_to_clerk
 -> clerk_records_incident
 -> guard_considers_questioning
```

## Snapshots and compaction

Long simulations need snapshots. Snapshots are projections, not replacements for significant causal ancestry.

Preserve detailed events for:

- state-changing player/NPC actions;
- surviving traces;
- belief changes;
- contradiction events;
- institutional records;
- reports;
- notices;
- contracts;
- crimes and suspected crimes;
- injuries/deaths;
- ownership/custody changes;
- active leads;
- events referenced in testimony;
- events later used by story sifting;
- LOD promotion/demotion causes.

Compact low-salience routine spans into summary events with causal links.

A meal that created the last sighting of a missing person is no longer routine.

## Determinism

Target deterministic replay:

- seeded random streams;
- stable iteration order;
- explicit event ordering;
- no wall-clock decisions;
- deterministic scheduler;
- versioned data files;
- random draws recorded in debug builds;
- no uncontrolled LLM calls in authoritative simulation;
- LLM outputs stored and validated if used for surfaces;
- replay rejects mismatched data versions unless migrated.

## Authority and projections

Derived projections may cache:

- current component state;
- actor beliefs;
- inventories;
- institution records;
- lead notebooks;
- local map views;
- social reputation;
- event summaries;
- story sift results;
- UI view models.

Projections may be rebuilt. They are not authoritative if they conflict with event history.

## Forbidden shortcuts

- no inventory mutation without event;
- no guard knowledge from global truth;
- no notices without author, issuer, structured claims, and posting event;
- no bounty without institutional knowledge, decision, funds/custody, proof rules, and record;
- no objective marker to true location from rumor;
- no player-only actions;
- no generated prose creating facts;
- no quest completion flag as truth;
- no low-LOD summary that erases active causal ancestry;
- no actor belief without source;
- no institution sanction without procedure or exceptional authority event.
