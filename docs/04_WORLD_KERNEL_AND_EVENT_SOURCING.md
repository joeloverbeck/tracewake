# World Kernel and Event Sourcing

## Purpose

The world kernel is the authoritative machine that decides what is true. It should be small, deterministic where practical, event-sourced, and hostile to shortcuts.

Everything else — agent memory, rumor, records, UI cards, summaries, dialogue, save files, analytics — should be projected from world state and events.

## Kernel responsibilities

The kernel owns:

- entity identity;
- component state;
- primitive actions;
- preconditions;
- effects;
- event creation;
- event ordering;
- simulation time;
- random stream discipline;
- spatial occupancy and reservations;
- ownership and permissions;
- perception hooks;
- trace generation;
- causal links;
- replay and snapshot support.

The kernel does not own:

- prose;
- dramatic pacing;
- quest arcs;
- authorial intent;
- player importance;
- omniscient interpretation.

## Core entities

### Entity

A stable identity in the world.

Examples:

- actor;
- item;
- container;
- building;
- room;
- notice;
- record;
- institution;
- trail segment;
- corpse;
- fire;
- debt;
- contract.

### Component

A structured state slice attached to an entity.

Examples:

```yaml
Position:
  location: room_blacksmith_home

Ownership:
  legal_owner: actor_tomas
  possessor: actor_mara
  permitted_users: [actor_tomas, actor_elena]

Container:
  locked: true
  lock_id: lock_strongbox_01
  contents: [item_coin_stack_01]

ActorBody:
  health: 78
  fatigue: 42
  hunger: 20

Mind:
  beliefs_ref: belief_store_actor_tomas
  intentions_ref: intention_stack_actor_tomas
```

### Action

A proposed state transition with preconditions and effects.

Actions should be explicit enough to leave traces and support belief updates.

Example action:

```yaml
action_type: RemoveItemFromContainer
actor: actor_mara
parameters:
  container: item_strongbox_01
  item: item_coin_stack_01
preconditions:
  - actor is at same location as container
  - container is open or actor can open it
  - actor has free carrying capacity
  - actor believes taking item is acceptable or is willing to violate norm
primary_effects:
  - item possessor becomes actor_mara
  - item removed from container contents
trace_effects:
  - container last_opened_at changes
  - possible noise event
  - possible witness observation
  - possible disturbed-dust trace
```

### Event

The immutable record of something that happened or failed to happen.

Minimum event fields:

```yaml
event_id: evt_018239
world_tick: 81230
time: 12 Rainwane 142, 02:13
kind: ItemRemovedFromContainer
actor: actor_mara
participants:
  container: item_strongbox_01
  item: item_coin_stack_01
location: room_tomas_bedroom
causes:
  - evt_018232 # chest opened
  - intention_778 # mara intends to steal coins
preconditions_checked:
  - same_location: true
  - container_open: true
  - item_present: true
effects:
  - path: item_coin_stack_01.Possession.possessor
    old: item_strongbox_01
    new: actor_mara
traces_created:
  - trace_disturbed_dust_44
  - trace_faint_noise_12
observers:
  direct: []
  indirect:
    - actor_elena # heard faint noise, confidence low
visibility:
  public: false
```

## Event sourcing

The project should use an append-only event store for meaningful state changes.

Current state is a projection of event history plus snapshots. Agent beliefs, institutional records, and UI summaries are separate projections, not replacements for the event log.

### Why event sourcing fits this game

Event sourcing is not just an engineering pattern here. It is the game’s moral skeleton.

It gives the project:

- forensic replay;
- “why did this happen?” tools;
- debugging for emergent weirdness;
- belief generation from observation events;
- rumor generation from testimony events;
- institutional records from report events;
- historical chronicles from event streams;
- player-facing investigation surfaces.

### Event streams

Use multiple related streams:

```text
WorldStream              all globally ordered significant events
EntityStream(entity)     events involving one entity
ActorStream(actor)       events perceived, caused, or believed by an actor
InstitutionStream(inst)  reports, records, decisions, orders, sanctions
LocationStream(location) events and traces at a place
ItemStream(item)         ownership, possession, damage, custody, movement
```

The world stream provides total ordering. Derived streams provide efficient queries.

## Action pipeline

All meaningful actions should pass through the same pipeline:

1. **Proposal** — an actor, institution, process, or environment proposes an action.
2. **Authority check** — is the proposer allowed to attempt this kind of action?
3. **Precondition check** — evaluate physical, social, knowledge, and resource preconditions.
4. **Reservation check** — prevent impossible simultaneous use of space or objects.
5. **Cost check** — time, fatigue, materials, risk, money, attention.
6. **Execution** — action may succeed, fail, partially succeed, or be interrupted.
7. **State mutation** — authoritative state changes.
8. **Trace generation** — physical/social/institutional traces are created or erased.
9. **Observation pass** — nearby agents, sensors, records, or institutions may perceive the event.
10. **Belief update** — observers update subjective knowledge.
11. **Trigger consequences** — plans may interrupt, institutions may receive reports, routines may change.
12. **Event commit** — event and linked subevents become durable.

## Preconditions should include social state

Many games treat action preconditions as physical only. This project needs richer preconditions.

Example: entering a home.

```yaml
physical:
  - door is open or actor can open it
  - actor can reach doorway
social:
  - actor owns home OR actor is invited OR actor is willing to trespass
knowledge:
  - actor believes this is the right home OR is exploring/searching
risk:
  - actor believes chance of being seen is acceptable
institutional:
  - actor has warrant OR local law allows entry OR actor is violating law
```

An action may remain physically possible while socially forbidden. That is how crimes happen.

## Causal graph

Events should link to causes where possible.

Cause types:

- enabling condition;
- actor intention;
- direct physical cause;
- institutional procedure;
- information cause;
- environmental process;
- random seed branch;
- prior event that created motive.

Example chain:

```text
bandits_attack_traveler
  -> survivor_observes_attack
  -> survivor_reports_to_reeve
  -> reeve_believes_road_threat
  -> reeve_allocates_bounty_funds
  -> clerk_writes_notice
  -> guard_posts_notice
  -> adventurer_reads_notice
  -> adventurer_forms_intention_to_hunt_bandits
```

## Traces

A trace is a stateful artifact produced by events and later observed by agents.

Trace examples:

```yaml
Trace:
  id: trace_blood_001
  kind: bloodstain
  created_by: evt_stabbing_44
  location: alley_west
  visibility: medium
  decay_rate: slow
  can_be_cleaned_by: [CleanSurface, Rainfall]
  evidential_links:
    possible_causes: [wound, butchery, animal_kill]
```

Important: traces are not always conclusive. A footprint suggests, a lock scratch implies, a missing item contradicts expectation.

## Snapshots

The event log is authoritative, but long simulations need snapshots.

Snapshots should be:

- deterministic products of events;
- versioned by schema;
- invalidatable;
- usable for replay from checkpoint;
- never a replacement for recent event history.

A useful policy:

```text
Full snapshot every simulated day.
Rolling event replay for current day.
Permanent archive for significant event chains.
Compressed summaries for low-salience routine events.
```

## Determinism

Perfect determinism may not be possible forever, but deterministic replay should be a goal for the headless core.

Guidelines:

- seeded random streams per subsystem;
- stable iteration order;
- explicit event ordering;
- no wall-clock-dependent decisions;
- no uncontrolled LLM calls in authoritative simulation;
- record random draws in debug builds.

## Level of detail

Do not simulate the entire world at full fidelity.

Use LOD tiers:

### Tier 0 — Active local simulation

Full action/event/perception/traces/planning.

### Tier 1 — Nearby social simulation

Routine actions batched, major events detailed, beliefs partially updated.

### Tier 2 — Regional institutional simulation

Trade, reports, incidents, travel, rumors, and threats summarized as events.

### Tier 3 — Historical abstraction

Civilization-level or region-level changes produce summary events and artifacts.

Rule: any process can be promoted to higher fidelity when the player, an investigation, or a causal chain brings it into focus.

## Forbidden kernel shortcuts

- Mutating inventory without an event.
- Guards learning crimes from global state.
- Notices appearing without author, funds, and posting event.
- Agents using exact locations of threats they only heard rumors about.
- Player-only actions.
- Text output that secretly creates facts.
- “Quest complete” as source of truth.
- Despawning inconvenient actors or evidence without cause.

## Minimal kernel for the vertical slice

The first build needs only:

- entities and components;
- rooms and paths;
- actors;
- containers and items;
- ownership/possession;
- lock/open/close/take/place actions;
- primitive speech/report actions;
- sleeping/working/eating routines;
- notices as physical objects;
- event log;
- perception hooks;
- belief store projection;
- trace creation for theft and violence;
- institution record projection;
- replay/debug viewer.

Everything else is secondary.
