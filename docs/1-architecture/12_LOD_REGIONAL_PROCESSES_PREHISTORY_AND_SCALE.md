# LOD, Regional Processes, Prehistory, and Scale

## Status

Authoritative architecture contract.

## Purpose / core rule

LOD is ontology and replay-visible abstraction, not merely optimization. Low-detail people remain promotable people with ancestry. Regional and historical processes may maintain aggregate truth, but promoted actors/institutions receive only modeled knowledge derived from events, records, reports, memories, public artifacts, or summary ancestry.

## Authority owned

This document owns dynamic level-of-detail, regional processes, promotion/demotion, summary events, prehistory, long simulation scale, aggregate truth boundaries, and LOD contamination rules.

## Authority denied

LOD may not turn people into props, erase causality, inject knowledge into promoted holders, hide random drift, or create director logic. Regional abstraction may not become a quest/adventure generator.

## Contract

### Detail tiers

Tracewake may use tiers such as:

| Tier | Example | Authority |
|---|---|---|
| Detailed local | Room-scale actors, containers, doors, observations, speech, action proposals | Full holder-known/action pipeline |
| Active settlement | Daily routines, institutions, households, work, food, notices, incidents | Context packets may summarize but must preserve holder provenance |
| Regional process | Travel, trade, weather-like pressures, institutional backlog, rumors, road risk | Aggregate events with causal and epistemic ancestry |
| Historical/prehistory | Seeded past events, memories, records, public artifacts, scars/traces | Validated seed events/records; no hidden outcome scripts |
| External abstraction | Distant places and people not yet simulated | Summary truth only until promoted; no personal knowledge without ancestry |

Tier is not moral worth or narrative importance. It is simulation resolution.

### Summary truth

Low-LOD processes may track aggregate truth such as population movement, caravan arrival, market pressure, road danger, institutional caseload, seasonal food pressure, or distant family status. These facts may drive regional events and resource pressures.

They may not become actor/institution knowledge unless converted through modeled channels.

### Summary event minimum

```text
summary_event:
  event_id
  process_id
  region_or_scope
  tick_window
  aggregate_state_before_after
  participants_or_participant_sets
  random_stream_refs
  cause_event_ids
  public_artifacts_created
  records_or_rumors_created
  promotion_payload_policy
  confidence_or_abstraction_limit
```

### Promotion

Promotion turns a low-detail entity/process into a higher-detail actor/institution/place/incident.

Promotion must generate or reference:

- identity continuity;
- physical/social state seed;
- known personal memories or explicit unknowns;
- public records/artifacts visible to the promoted context;
- institutional records/procedure state;
- relationship/role claims with sources;
- event ancestry and random stream ancestry;
- facts deliberately not known to the promoted holder.

The promoted actor does not know aggregate truth merely because the aggregate process tracked it.

### Demotion

Demotion summarizes detailed entities into lower-detail state without erasing event history. It must preserve enough ancestry for future promotion, replay, and holder-known context reconstruction. Demotion cannot compress away a fact needed for property, memory, record, obligation, or institution procedure.

### Boundary crossings

Travel, communication, trade, rumor, notices, orders, weather-like pressures, migration, road threats, and institutional reach cross LOD boundaries through events. Boundary events must specify what became public, what remained private, and what holders can know.

### Prehistory

Prehistory may seed relationships, property expectations, household memory, institutions, public records, scars, debts, missing persons, road reputation, and artifacts. It must be represented as validated seeds or prehistory events with source/provenance. It cannot encode authored outcome chains.

### Regional incidents and second proof

Regional threats, bounties, expeditions, monsters, and travel parties are later. They must enter through notices, reports, travel events, institutional procedures, actor-known contexts, and LOD ancestry. A road threat cannot spawn because the story needs excitement.

## Required diagnostics / replay / TUI hooks

- LOD promotion reports must list known/provenance payload and withheld aggregate truth.
- Summary events must be replayable and random-stream scoped.
- Debug views may compare aggregate truth and promoted holder knowledge.
- Acceptance tests for future LOD must include adversarial aggregate truth absent from promoted actor/institution context.
- TUI embodied views of promoted actors must show only promoted holder-known facts.

## Acceptance implications

LOD is not required to complete the first missing-property proof, but the architecture must not introduce incompatible assumptions. Later LOD acceptance fails if promoted actors know summary truth without source, if regional processes spawn authored story beats, or if replay cannot rebuild promotion ancestry.

## Anti-patterns

- “Low-LOD NPCs are just counters until seen.”
- “On promotion, fill the actor's memory from regional truth.”
- “Distant institution knows every regional event.”
- “Summary event says thief escaped, so local guards know the thief.”
- “Road threat probability increases because the player is bored.”
- “Prehistory can define the guilty party for future content.”

## Cross-document obligations

LOD depends on event/replay from document 02, holder-known context from document 03, actor/institution transactions from documents 05 and 08, records and claims from document 06, notices/incidents from document 11, and acceptance artifacts from document 13.
