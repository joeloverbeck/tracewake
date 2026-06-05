# Spatial, Regional, and Travel Model

## Core claim

Tracewake begins with a village but must be shaped for regional life. The spatial model must support rooms, homes, roads, wilderness sites, towns, borders, caravans, migration, disease, animal movement, and outside influences.

Start with a graph. Do not start with graphical terrain.

## Spatial hierarchy

```text
World
  Region
    Subregion / biome / jurisdiction
      Settlement
        District / block
          Building
            Room
      Route
        Route segment
      Wilderness site
      Resource site
      Threat site
      External boundary
```

A future graphical frontend may render continuous maps. The causal model begins as connected places with affordances, travel costs, visibility, jurisdiction, and event streams.

## Place types

### Rooms

Containers, beds, doors, witnesses, line of sight, sound, privacy, traces, possessions, local actions.

### Buildings

Ownership, access permissions, public/private status, household/workplace identity, opening hours, security, address.

### Settlements

Residents, shops, offices, records, notices, rumors, routines, public spaces, market, authority, social density.

### Routes

Caravans, patrols, ambush risk, weather delays, disease movement, rumor flow, supply chains, missed arrivals.

### Wilderness sites

Resources, danger, hiding, corpses, animals, camps, ruins, old traces, false rumors.

### Regions

Long simulation, ecology, disease, institutions, migration, trade, external boundaries, history.

## Travel as event chain

Travel is not teleportation.

```text
form intention
 -> decide destination from belief
 -> prepare supplies if relevant
 -> recruit companions if desired
 -> depart
 -> traverse route segment
 -> encounter delays/events/traces
 -> consume time/food/fatigue/safety
 -> arrive, divert, turn back, get lost, camp, die, or continue
```

Travel creates:

- departure events;
- route observations;
- arrival events;
- missed-arrival evidence;
- fatigue/hunger changes;
- route-risk updates;
- rumor movement;
- witness opportunities.

## Route model

```yaml
Route:
  id: north_road
  connects: [village, old_quarry, border_post]
  travel_time_base: 3h
  actual_risks:
    bandit_group_02: active
  public_beliefs:
    bandit_activity: medium
  jurisdiction:
    - reeves_office
  traffic_profile:
    market_days: high
    night: low
```

Actors act on believed route risk, not actual route risk.

## Location knowledge

Agents may know:

- exact location;
- approximate location;
- route directions;
- landmark;
- jurisdiction;
- rumored location;
- old map;
- “somewhere north”;
- false location;
- last known location;
- home/work address.

Markers represent belief precision and source. They do not point to ground truth unless the actor has ground-truth-quality knowledge through modeled channels.

## Doors, rooms, and sound

Spatial modeling must support ordinary life and epistemics:

- who can see;
- who can hear;
- who can enter;
- whether entry is trespass;
- whether a door muffles sound;
- whether a container is visible;
- whether a bed is occupied;
- whether an absence is noticeable.

Do not reduce space to pathfinding.

## Outside-region influences

Outside the region exists as boundary processes:

- immigrant family arrives because of famine elsewhere;
- locals leave for city employment;
- caravan enters with goods and rumors;
- disease arrives through travelers;
- animal migration crosses boundary;
- tax order arrives from superior authority;
- deserter group enters route network.

Represent these as exogenous boundary events with summarized causes. Do not spawn them because the player needs variety.

## Sites of interest

A site is not a dungeon label. It has:

- history;
- affordances;
- inhabitants or absence thereof;
- traces;
- processes;
- jurisdiction;
- public beliefs;
- access routes;
- resource/risk profile.

A site can be active, abandoned, looted, occupied, misidentified, falsely rumored, taboo, inaccessible, or stale in records.

## Expeditions

Expeditions are built after ordinary life, travel, and leads work.

Flow:

```text
learn source-bound lead
 -> verify or ignore uncertainty
 -> recruit companions
 -> negotiate payment/shares
 -> gather supplies
 -> travel
 -> inspect target site
 -> adapt to stale/false/moved information
 -> return, continue, flee, report, or pursue new lead
```

Companions are agents, not tools.

## First implementation

The first village needs:

- settlement graph;
- buildings and rooms;
- homes;
- workplace;
- tavern;
- authority office;
- notice board;
- one route;
- one wilderness/threat site;
- travel action;
- route risk;
- arrival/departure events;
- stale location belief;
- actor-filtered map view in TUI.

Schemas should be regional from day one even if content is tiny.

## Anti-patterns

- terrain engine before causal places;
- teleport travel;
- objective map markers;
- sites that exist only for quests;
- roads that do not move people/goods/rumors;
- wilderness threat frozen for player;
- graphical map as authoritative model;
- outside events injected for pacing.
