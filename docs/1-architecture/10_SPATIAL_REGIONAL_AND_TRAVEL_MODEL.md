# Spatial, Regional, and Travel Model

## Core claim

Tracewake begins with a village but must be shaped for regional life. The spatial model must support rooms, homes, roads, wilderness sites, towns, borders, caravans, migration, disease, animal movement, and outside influences.

## Spatial hierarchy

Use a graph first:

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
```

A graphical frontend may later render continuous maps. The causal model begins as connected places with affordances, travel costs, visibility, jurisdiction, and event streams.

## Place types

Rooms support containers, beds, doors, witnesses, line of sight, sound, privacy.

Buildings support ownership, entry permissions, public/private status, household/workplace identity.

Settlements support residents, shops, rumors, offices, notices, routines.

Routes support caravans, patrols, ambushes, weather delays, disease movement, rumor flow.

Wilderness sites support resources, danger, hiding, corpses, animals, camps, and ruins.

Regions support long simulation, ecology, disease, institutions, and external boundaries.

## Travel as event chain

Travel is not teleportation.

```text
form intention -> prepare supplies -> recruit companions -> depart
-> traverse route segment -> encounter delays/events/traces
-> arrive, turn back, get lost, camp, die, or divert
```

Travel creates departure, route observations, arrival, missed-arrival evidence, fatigue/hunger changes, and information movement.

## Routes

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
```

Actors act on believed route risk, not actual risk.

## Location knowledge

Agents may know exact location, approximate location, route directions, rumored location, old map, landmark, jurisdiction, or “somewhere north.” Markers reflect belief precision.

## Outside-region influences

Outside the region exists as boundary processes:

- immigrant family arrives because of famine elsewhere;
- locals leave for city employment;
- caravan enters with goods and rumors;
- disease arrives through travelers;
- animal migration crosses boundary;
- tax order arrives from superior authority.

Represent these as exogenous boundary events with summarized causes. Do not spawn them because the player needs variety.

## Regional LOD

- Tier 0: embodied local rooms, perception, speech, traces.
- Tier 1: active settlement routines, shops, reports, notices, rumors.
- Tier 2: regional processes such as caravans, migration, disease, patrols.
- Tier 3: external boundary summaries.

Promotion occurs when an event becomes relevant, not when drama is needed.

## Sites of interest

A site is not a dungeon label. It has history, affordances, inhabitants, traces, and processes. It can be active, abandoned, looted, occupied, misidentified, falsely rumored, or stale in records.

## Expeditions

Flow:

```text
learn lead -> verify or ignore uncertainty -> recruit companions
-> negotiate payment/shares -> gather supplies -> travel
-> inspect target site -> adapt to stale or false information
-> return, continue, flee, or pursue new lead
```

Companions are agents, not tools.

## First implementation

The first village needs settlement, buildings, rooms, one route, one wilderness/threat site, travel action, route risk, arrival/departure events, and stale location belief. Schemas should be regional from day one.
