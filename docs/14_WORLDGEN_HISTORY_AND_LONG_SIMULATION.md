# World Generation, History, and Long Simulation

## Core claim

Tracewake eventually needs a region that has lived before the player enters it. Long simulation and world generation must exist, but not before the hand-authored causal model works.

Recommended order:

```text
hand-authored causal village
 -> hand-authored regional routes/sites
 -> long simulation over known content
 -> procedural population/history support
 -> procedural spatial generation
 -> larger dynamic region
```

## Long simulation fantasy

Before play begins, the region can simulate years:

- people age, die, marry, separate, migrate;
- shops open and close;
- wealth changes hands;
- roads become safer or more dangerous;
- institutions gain records and grudges;
- diseases spread or burn out;
- bandit groups form, move, splinter, or die;
- ruins are looted;
- rumors distort;
- families remember old wrongs;
- notices become historical artifacts;
- settlements grow, decline, or move.

## History layers

### Detailed events

Preserve causally important events: deaths, crimes, contracts, migrations, institution changes, relationships, property transfers, injuries, battles, disasters, public records, unresolved incidents.

### Summary events

Compress routine spans.

```yaml
SummaryEvent:
  kind: HouseholdRoutinePeriod
  actors: [family_aren]
  interval: year_12_spring
  effects:
    - food_consumed
    - savings_decreased
    - child_skill_increased
  causal_links:
    - harvest_shortfall
```

### Public histories

Represent what culture remembers, possibly wrongly.

```yaml
PublicHistory:
  claim: OldTowerCursed
  origin_events: [evt_disappearances_18, rumor_chain_44]
  accuracy: partial
```

Distinguish actual history from public memory.

## Worldgen principles

Procedural generation may create settlements, roads, households, occupations, institutions, relationships, property, rumors, historical incidents, ecology, sites, and threats. Generated content must satisfy the same rules as authored content: records have authors, rumors have origins, threats have needs/traces, institutions have resources/procedures, and no quest objects appear.

## WFC and constraint generation

WFC-like methods may later help with room layouts, settlement blocks, road networks, wilderness sites, ruins, ecological adjacency, and plausibility constraints. Use generation as authoring support, not simulation substitute.

## Population generation

A generated person needs home or homelessness reason, occupation or survival strategy, relationships, roles, needs, values, at least one project/concern, important beliefs, reputation hooks, routine template, possessions, and permissions. Low-LOD people may summarize these, but must be promotable.

## Historical causality

Bad:

```text
Mara hates Tomas.
```

Better:

```text
Mara distrusts Tomas because Tomas testified against her brother in tax court three years ago.
```

Even summarized causes should exist.

## Starting state

A playable start includes current date/season, settlement conditions, institutions, records, unresolved incidents, stale notices, rumors, ongoing projects, route risks, economic pressures, outside influences, and character histories.

## Time acceleration

During acceleration:

- embodied actor performs wait/sleep/travel/continue-routine action;
- simulation runs through LOD;
- summaries are generated from events;
- player receives only actor-accessible information in embodied mode;
- high-salience perceived interruptions may stop time;
- long-term changes preserve ancestry.

## Region size

The dream may feel Skyrim-sized, but implementation size should be causal: several settlements, key roads, wilderness sites, institutions, outside boundaries, and enough population to create social density. Better a smaller region that remembers than a larger one that lies.

## Anti-patterns

Generated lore that cannot affect play, towns with no institutions, NPCs with no home/work/relationships, history as prose only, black-box years, preserving every meal forever, and procedural generation before hand-authored ontology is validated.
