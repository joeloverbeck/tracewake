# Phase 1: Kernel, TUI/View Models, Event Log, and Replay

## Purpose

Phase 1 builds the smallest playable and inspectable causal machine.

It proves that Tracewake can run a physical/action/event/TUI spine before advanced epistemics, routines, or institutions. The player can embody an ordinary actor, move, inspect, open, close, take, place, and see committed events. Debug can attach, inspect, replay, and explain basic item location.

Phase 1 does not prove belief. It prepares the substrate that belief will later observe.

## Entry requirements

Phase 1 may start only after Phase 0 exits.

Required inputs:

- approved primitive action vocabulary;
- approved primitive event vocabulary;
- first village minimal fixture;
- stable ID policy;
- TUI embodied/debug screen contracts;
- golden scenario sketches;
- no-scripting review;
- data validation requirements for minimal entities.

## Deliverables

### World identity and storage

Deliver:

- deterministic entity IDs;
- minimal component storage;
- stable entity kinds for actor, body, place, room, door, container, item, institution placeholder, record placeholder;
- deterministic fixture loading;
- component mutation only through event application.

Minimum components:

```yaml
Position:
  place: place_id
  contained_in: optional_entity_id

DoorState:
  connects: [place_id, place_id]
  open_state: open | closed
  lock_state: locked | unlocked

ContainerState:
  location: place_id
  open_state: open | closed
  lock_state: locked | unlocked
  contents: [item_id]
  access_profile: simple

OwnershipCustody:
  legal_owner: optional_actor_or_household
  possessor: entity_id
  custodian: optional_actor_or_institution
  expected_location_stub: optional_place_or_container
  permitted_users: [actor_id]
```

Expected location may be a stub in Phase 1. Full expectation contradiction belongs to Phase 2.

### Scheduler

Deliver:

- discrete simulation time;
- stable ordering;
- no wall-clock authority;
- deterministic tie breakers;
- simple scheduled action support;
- no-human advance command;
- controller-bound command handling without player privilege.

The scheduler must not treat a human-bound actor as more real.

### Action proposal framework

Deliver:

```text
command/proposal intake
 -> actor authority check
 -> action definition lookup
 -> parameter binding
 -> physical precondition check
 -> minimal knowledge/perception placeholder check
 -> social/norm placeholder check
 -> cost/duration check
 -> reservation/conflict check
 -> outcome resolution
 -> event commit
 -> projection update
 -> view-model update
```

Some checks may be minimal or vacuous in Phase 1, but they must exist as architectural slots so later phases do not bolt on parallel rules.

### Primitive action registry

Required Phase 1 actions:

- attach controller to actor in debug;
- look/inspect current place;
- move to adjacent place;
- open door;
- close door;
- open container;
- close container;
- inspect container;
- take item;
- place item;
- wait/advance time;
- debug inspect event log;
- debug rebuild projection.

Optional if cheap:

- lock/unlock with known key;
- search container as "inspect contents" without full epistemics;
- hide item as place item into a non-obvious container.

No action may be player-only in embodied mode.

### Event log

Deliver:

- append-only event log;
- versioned event envelope;
- global order;
- event type;
- actor/process;
- participants;
- place;
- causes/precondition report;
- effects summary;
- content version;
- replay checksum or equivalent diagnostic hook;
- debug validation report ID or equivalent.

Required Phase 1 event kinds:

```text
ControllerAttached
ActorMoved
DoorOpened
DoorClosed
ContainerOpened
ContainerClosed
ItemRemovedFromContainer
ItemPlacedInContainer
ActionRejected
ActionStarted
ActionFailed
NoHumanAdvanceStarted
ReplayProjectionRebuilt
```

A rejection may be non-event if nothing can later care. In tests and debug fixtures, meaningful rejections should be recordable.

### Places, doors, containers, items

Deliver a minimal fixture with:

- at least Tomas's house with two rooms;
- a workplace or public room;
- local authority office placeholder;
- at least one door;
- Tomas's strongbox;
- coin_stack_01;
- one food item or placeholder;
- at least two actors;
- enough path graph to move and inspect.

Do not use abstract inventory teleportation. Items are located, contained, possessed, or carried through eventful movement.

### Minimal embodied TUI

Required TUI capabilities:

- choose or attach to actor;
- display current actor;
- display current place;
- display visible immediate objects and exits according to Phase 1 placeholder visibility;
- show actor-available actions;
- submit semantic action commands;
- display why-not basics;
- display last actor-perceived event summaries if available;
- wait/advance time.

Phase 1 embodied view may be physically local rather than epistemically complete. It must not make future actor-known filtering impossible.

### Debug TUI

Required debug capabilities:

- attach/detach/switch controller binding;
- view event log;
- inspect current state projection;
- inspect entity details;
- inspect action rejection report;
- rebuild projection from log;
- compare current projection to replayed projection.

Debug must be visibly non-diegetic.

## Explicit non-goals

Phase 1 does not include:

- full belief stores;
- direct/sound observation interpretation;
- expectation contradiction;
- actor-known notebook;
- no-transfer possession proof beyond command binding metadata;
- hunger/fatigue/safety;
- sleep/eat/work routines;
- report/record procedure;
- suspicion;
- rumor;
- notice lifecycle;
- travel beyond adjacent local movement;
- LLM surfaces.

## TUI/view-model gate

A user can:

1. attach to Tomas or Mara in debug;
2. move between connected rooms if allowed;
3. inspect current actor and place;
4. open and close a door;
5. inspect/open a container;
6. take an item if allowed;
7. place an item if allowed;
8. see a why-not when an action is rejected;
9. switch to debug and see the committed events;
10. rebuild projection and verify item location.

Automated acceptance should select actions by stable semantic IDs, not terminal coordinates.

## No-human simulation gate

Phase 1 no-human simulation is intentionally small.

Minimum:

```text
load fixture
run scheduler for N ticks with no HumanController bound
execute at least one neutral scheduled action or no-op advance policy
commit no player-referencing events
prove scheduler/event/projection code does not require a controller
```

If no agent cognition exists yet, no-human may only prove that time and scheduled processes advance. It must not contain `player`, `quest`, or `human-controlled actor` as world logic.

## Deterministic replay gate

Replay must:

- load same fixture;
- apply event log in order;
- rebuild current component state;
- reconstruct door/container/item location;
- reject unknown event type/version;
- detect changed content version when configured;
- produce stable checksum/report for tests.

Required first replay question:

```text
Why is coin_stack_01 in actor_mara's inventory?
```

Acceptable debug answer:

```text
evt_000014 ItemRemovedFromContainer by actor_mara from strongbox_tomas
preconditions: actor at room, container open, item present, carry allowed
effect: possessor changed from strongbox_tomas to actor_mara
```

## Test gate

Minimum tests:

### Unit

- stable ID generation;
- component insertion/update;
- action definition lookup;
- physical precondition pass/fail;
- event envelope creation;
- event application;
- projection rebuild;
- door open/close;
- container open/close;
- item remove/place;
- action rejection without mutation.

### Property

- no item has two possessors;
- every committed item movement has an event;
- event order is stable for same seed/input;
- TUI cannot call direct state mutation;
- no embodied action is marked player-only;
- replay of simple item movement matches live projection.

### Integration

- `container_item_move_001`;
- `door_access_001`;
- `debug_attach_001`;
- `no_human_advance_001`;
- `replay_item_location_001`;
- `view_model_local_actions_001`.

## Data/fixture gate

The Phase 1 fixture must validate:

- stable IDs;
- unique entity names/IDs;
- valid place graph references;
- door endpoints exist;
- container contents exist;
- item possessor/container consistency;
- ownership/custody references exist;
- action registry IDs match affordances;
- no `Quest`;
- no `PlayerCharacter`;
- no direct event scripts for normal play.

## Debug/inspection gate

Debug must answer:

- current place of actor;
- current contents of container;
- current possessor of item;
- last event that moved item;
- precondition that blocked an action;
- event order and causes for simple chain;
- projection rebuild status;
- controller binding history as non-world metadata.

## Failure cases to model

Required failure cases:

- actor cannot move because no connection;
- door closed blocks path if configured;
- door locked blocks open;
- actor cannot inspect unreachable container;
- container locked blocks open;
- item not in container;
- actor cannot carry item;
- item already reserved;
- action rejected before mutation;
- event application detects impossible old state;
- replay finds checksum mismatch;
- TUI command references invalid actor or target.

Failure cases are not polish. They prove the system is causal rather than scripted.

## Forbidden shortcuts

- UI button mutates inventory;
- content fixture moves item at scheduled time without action/event;
- player-only `take_all`;
- treating possession as world actor identity;
- saving only current state and calling it replay;
- using a storage schema as authority;
- adding belief/institution shortcuts before Phase 1 exits;
- relying on debug commands as embodied play.

## Phase exit checklist

Phase 1 exits when:

- minimal fixture loads;
- TUI can play local movement/object/container/item operations;
- actions pass through shared pipeline;
- meaningful changes commit events;
- replay rebuilds current state;
- debug can explain item location;
- no-human scheduler advance runs with no player references;
- tests cover success and failure;
- data validation rejects player/quest shortcuts.

Only then begin Phase 2.
