# Ordinary Life, Economy, Settlement, and Spatial Model

## Status

This document defines the first-slice ordinary-life, economy, settlement, and spatial architecture.

Tracewake begins with a small, deep, inspectable village. Adventure systems build on ordinary life; they do not replace it.

## Core rule

Ordinary life must be mechanically real before disruption matters.

A theft matters because homes, locks, storage, expectations, households, sleep, privacy, work, records, neighbors, traces, and fallible authority exist.

A road threat matters later because roads move people, food, work, rumors, fear, patrols, witnesses, and money. It is not the first proof.

## First-slice scale

Target:

- 10-30 high-detail agents;
- optional low-LOD background population only if causally honest and promotable;
- 4-8 households to start, not a huge town;
- homes with rooms, doors, beds, storage, food, and privacy;
- one or two workplaces;
- a small public area with notice board;
- a reeve/clerk office;
- simple routes only as needed for ordinary movement;
- no-human daily simulation;
- embodied TUI play.

The earlier temptation to start with dozens of agents, nearby threats, expedition sites, and bounties should be resisted until the missing-property miracle works.

## First domain

The first domain is neutral medieval-ish ordinary life without magic.

Use this domain because it supports:

- visible labor;
- slow information;
- local records;
- household privacy;
- public notice boards;
- simple tools;
- local authority;
- ownership disputes;
- food and storage;
- room/door/container causality.

Do not overbuild:

- magic;
- religion;
- species;
- detailed combat;
- large kingdoms;
- procedural terrain;
- full trade simulation;
- graphical presentation.

## Settlement graph

Start with a graph, not a terrain engine.

```text
Settlement
  public_square
    notice_board
    reeves_office
  house_tomas
    entry
    main_room
    bedroom
    pantry
  house_mara
    main_room
    sleeping_area
  workplace_mill
    workroom
    storage
  tavern_or_public_food_place_optional
  route_to_farms_optional
```

Places define affordances, visibility, sound, privacy, access, traces, and jurisdiction.

## Rooms, doors, and sound

Rooms and doors are epistemic architecture.

They answer:

- who can see what;
- who can hear what;
- who can enter;
- whether entry violates privacy/trespass;
- whether a door muffles sound;
- whether a container is visible;
- whether a bed is occupied;
- whether absence is noticeable;
- what traces persist.

Do not reduce space to pathfinding.

## Homes

Homes provide:

- sleep;
- storage;
- food access;
- privacy;
- household interaction;
- ownership boundaries;
- invitation/trespass logic;
- expectations about people and objects;
- hiding places;
- domestic conflict;
- evidence and traces.

A home is an institution-scale object, not a decorative building.

## Containers and storage

A container requires:

- location;
- contents;
- access rules;
- lock/open state;
- owner/custodian;
- expected contents by actor/household;
- privacy level;
- trace profile;
- affordances;
- damage state;
- sound/visibility behavior when used.

A strongbox is a causal object, not an inventory list.

## Ownership, possession, custody, expected location

Separate:

- legal owner;
- possessor;
- custodian;
- expected location;
- permitted users;
- disputed claimant;
- institutional custody;
- public record of ownership if any.

This supports theft, borrowing, found property, household use, hiding, evidence handling, debt, confiscation, inheritance later, and payment disputes.

## Beds, sleep, and fatigue

Beds expose sleep/rest affordances. Sleep is a scheduled action with interruption and observation possibilities.

Sleep creates:

- unavailability;
- vulnerability;
- expected bed occupancy;
- missed observations;
- fatigue recovery;
- household routine evidence;
- opportunities for theft;
- noise/absence contradictions.

NPCs must sleep in no-human simulation unless their state, role, or situation gives a reason not to.

## Food and hunger

Food is a physical/economic/social object.

Start simple:

- food items/stacks;
- household food storage;
- eating action;
- buying/trading food if a shop/tavern exists;
- hunger pressure;
- stealing food as desperate method;
- food absence as household/economic concern.

Do not build a detailed nutrition economy first.

## Work

Work produces consequences:

- goods or services;
- money or debt;
- absence from home;
- public visibility;
- relationships;
- records;
- routine expectations;
- opportunities for theft;
- fatigue and hunger changes;
- social interactions.

Example work routine:

```text
wake -> eat if possible -> travel to workplace -> open workplace if responsible
 -> perform work block -> interact with customer/coworker -> record debt/payment if any
 -> close or leave -> return home or social stop
```

Work can fail because of missing tools, illness, fear, locked door, office summons, food shortage, fatigue, or changed beliefs.

## Minimal economy

The first economy is about consequence, not realism.

Initial goods:

- food;
- coin or simple value token;
- grain/flour/bread if useful;
- tools;
- keys;
- paper/ink/ledger materials;
- ordinary valuables;
- simple weapons as objects only, not detailed combat system;
- bandages/medicine only if injury is modeled.

Initial flows:

```text
household food storage -> eating -> hunger changes
workplace activity -> goods/services -> payment/debt records
local authority -> staff time/records/notices -> public information
property movement -> expectation contradiction -> search/report/suspicion
```

Do not build market realism before causal play.

## Social ordinary life

Minimum social actions:

- greet;
- ask;
- answer;
- tell;
- gossip;
- report;
- refuse;
- lie;
- promise;
- accuse/suspect with actor-known basis;
- apologize if needed.

Social actions are speech acts and must use the speech pipeline.

## Roads and routes

The first village may include simple routes for local movement. Roads are ordinary-life infrastructure.

Routes move:

- people;
- food;
- goods;
- rumors;
- witnesses;
- institutional messages;
- fear;
- disease later.

Travel is an event chain, not teleportation:

```text
form destination intention
 -> choose route from belief
 -> depart
 -> traverse segment
 -> consume time/fatigue/safety
 -> observe or miss events/traces
 -> arrive, divert, turn back, or fail
```

Road-threat/bounty/expedition flow is second proof after ordinary life, institutions, speech, travel, and leads work.

## Canonical first proof in settlement terms

The missing-property miracle exercises:

- property stored in a household container;
- household privacy/access;
- actor motive and opportunity;
- door/container affordances;
- scheduled action and interruption windows;
- sound/visibility traces;
- expected location contradiction;
- search action;
- speech report;
- institutional record;
- rumor or wrong suspicion;
- stale/partial information;
- TUI possession and debug inspection.

The proof should work in no-human simulation before it becomes a player-facing scenario.

## First village content checklist

Required:

- named actors with homes, needs, relationships, roles, and beliefs;
- households with beds, food, storage, privacy;
- rooms and doors with access/sound/visibility;
- containers with ownership/expected contents/affordances;
- simple work routines;
- eating and sleeping;
- basic speech acts;
- one local authority office;
- incident ledger;
- notice board as artifact;
- simple money/custody;
- no-human scheduler run;
- TUI embodied view;
- debug event/causal/belief inspection.

Optional early:

- tavern/public food place;
- one local route;
- low-LOD background people;
- basic price/debt record;
- simple route rumor.

Deferred:

- bounties as central proof;
- expeditions;
- detailed combat;
- large market economy;
- procedural worldgen;
- magic;
- graphical map.

## Spatial data shape

```yaml
Place:
  id: room_tomas_bedroom
  kind: room
  parent: house_tomas
  privacy: household_private
  jurisdiction: reeves_office
  connected_by_doors:
    - door_tomas_bedroom_hall
  visibility_profile: small_room
  sound_profile: muffled_by_closed_door
  affordances:
    - inspect_room
    - search_room
    - enter_if_accessible
```

Map markers in embodied mode represent actor knowledge, not ground truth.

## Anti-patterns

- Ordinary life as static backdrop.
- Homes without storage/permissions/expectations.
- Shops that never close.
- Needs that never affect action.
- Schedules that ignore belief and interruption.
- Road threat becomes the first architecture gravity well.
- A huge village that cannot explain a stolen coin.
- Containers as abstract inventory lists.
- Combat-first prototype.
- Objective map markers from rumors.
- Low-LOD people as props.
