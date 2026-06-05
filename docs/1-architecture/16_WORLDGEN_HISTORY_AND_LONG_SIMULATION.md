# World Generation, History, and Long Simulation

## Core claim

Tracewake eventually needs a region that has lived before the player enters it. Long simulation and world generation matter, but they come after the hand-authored causal village works.

Recommended order:

```text
hand-authored causal village
 -> hand-authored regional routes/sites
 -> long simulation over known content
 -> procedural population/history support
 -> procedural spatial generation
 -> larger dynamic region
```

Do not generate a vast world before the ontology can explain a stolen coin.

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

Preserve causally important events:

- deaths;
- crimes;
- contracts;
- migrations;
- institutional changes;
- relationship changes;
- property transfers;
- injuries;
- disasters;
- public records;
- unresolved incidents;
- notable lies;
- major rumor mutations;
- sanctions;
- boundary events.

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
  retained_details:
    - debt_to_miller
    - child_illness_memory
```

### Public histories

Represent what culture remembers, possibly wrongly.

```yaml
PublicHistory:
  claim: OldTowerCursed
  origin_events: [evt_disappearances_18, rumor_chain_44]
  accuracy: partial
  believed_by:
    village_public: 0.71
```

Actual history and public memory must remain distinct.

## Procedural generation principles

Procedural generation may create:

- settlements;
- roads;
- households;
- occupations;
- institutions;
- relationships;
- property;
- rumors;
- historical incidents;
- ecology;
- sites;
- threats;
- records;
- debts.

Generated content must satisfy the same rules as authored content:

- records have authors;
- rumors have origins;
- threats have needs/traces/processes;
- institutions have resources/procedures;
- NPCs have homes or reasons for homelessness;
- no quest objects appear.

## Population generation

A generated person needs at least:

- body type;
- home or homelessness reason;
- occupation or survival strategy;
- household/relationship context;
- roles;
- needs;
- values;
- one current concern/project;
- important beliefs;
- possessions;
- permissions/access;
- routine template;
- reputation hooks.

Low-LOD people may summarize these, but must be promotable.

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

A playable start includes:

- current date/season;
- settlement conditions;
- institutions;
- records;
- unresolved incidents;
- stale notices;
- rumors;
- ongoing projects;
- route risks;
- economic pressures;
- outside influences;
- character histories;
- public memory;
- hidden truths if any;
- actor-accessible knowledge boundaries.

## Time acceleration

During acceleration:

- embodied actor performs wait/sleep/travel/continue-routine action;
- simulation runs through LOD;
- summaries are generated from events;
- player receives only actor-accessible information in embodied mode;
- high-salience perceived interruptions may stop time;
- long-term changes preserve ancestry.

## Procedural spatial generation

Constraint-based techniques such as WFC may later help generate:

- room layouts;
- settlement blocks;
- road networks;
- wilderness sites;
- ruins;
- ecological adjacency;
- plausible interiors.

Generation is authoring support. It does not replace simulation.

## Region size

The dream may feel large, but implementation size should be causal. Several settlements, key roads, wilderness sites, institutions, outside boundaries, and enough population for social density are better than a huge region that cannot explain itself.

## Anti-patterns

- generated lore that cannot affect play;
- towns with no institutions;
- NPCs with no home/work/relationships;
- history as prose only;
- black-box years;
- preserving every meal forever;
- procedural generation before hand-authored ontology;
- public history treated as truth;
- starting state with no causal ancestry.
