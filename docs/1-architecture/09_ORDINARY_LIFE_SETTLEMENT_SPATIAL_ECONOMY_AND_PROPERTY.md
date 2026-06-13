# Ordinary Life, Settlement, Spatial, Economy, and Property

## Status

Authoritative architecture contract.

## Purpose / core rule

The canonical first proof is missing-property/no-human ordinary life in a small settlement. Tracewake must prove homes, rooms, containers, property expectations, food, fatigue, sleep, work, movement, search, actor knowledge, reports, and replay before road-threat, bounty, expedition, monster, party, or adventure systems can claim acceptance.

Ordinary life is not filler. It is the substrate that makes later incidents meaningful.

## Authority owned

This document owns first-slice settlement ontology, ordinary places, property/custody/access, food/sleep/work, small economy, route adjacency, household resources, missing-property proof constraints, and deferral boundaries for later adventure-scale content.

## Authority denied

This document does not grant domain packs mutation authority. It does not authorize quest ontology. It does not permit a road/adventure system to bypass the ordinary actor/institution pipeline.

## Contract

### Settlement spatial model

The first settlement needs a small but causal model:

- places, rooms, roads/paths, doors, thresholds, visibility defaults;
- containers, beds/sleep places, food supplies, workplaces, notice boards, records;
- actors with current places, carried items, capacities, needs, routines, beliefs;
- households and private/public spaces;
- simple travel edges with actor-known discovery/provenance;
- access constraints such as locked doors, closed containers, ownership norms, and social permissions.

Geometry may be abstract. Causality may not be abstracted away.

### Property and custody

Property is a bundle of typed claims and social facts:

```text
property_model:
  legal_owner_or_claimant
  custodian
  expected_location
  permitted_access
  access_norms
  source_records_or_memories
  current_physical_location_truth
  holder_beliefs_about_location
```

Current holder is physical truth. Ownership/custody/access/expectation are social and epistemic claims. They can diverge.

### Missing-property proof

The first proof requires:

1. a modeled expectation that property should be somewhere;
2. a physical world in which the property may be absent from that expected place;
3. a holder who checks/searches through ordinary action;
4. an observation or absence marker;
5. a belief/contradiction update;
6. actor-visible notebook/lead output that does not reveal hidden truth;
7. optional report/institution intake through modeled speech/procedure;
8. debug comparison that can show hidden truth without leaking it;
9. replay and deterministic no-human evidence.

It does not require identifying a culprit. It must not fabricate one.

### Food

Food is finite, located, accessible or inaccessible, visible or hidden, fresh or stale, owned or shared, and known or unknown. Hunger produces pressure to eat or search, but targets must come from actor-known context.

Actor behavior can include eating known food, searching known containers, asking, buying, waiting, failing, or replanning. It cannot use true nearest food unless the actor knows it or discovers it.

### Sleep

Sleep uses places/resources, body-exclusive reservations, fatigue pressure, interruption, completion, and actor-known sleep-place claims. A bed/home assignment is not actor knowledge unless modeled as memory/household record/routine source.

### Work

Work uses workplace facts, access, assignment, schedule/routine, output placeholders, fatigue/hunger effects, and body/time reservation. Workplace truth does not grant actor knowledge. Work can fail due to closed workplace, access, exhaustion, hunger, interruption, wrong place, unknown route, or missing role.

### Local economy

First-slice economy is small: property, custody, service/work placeholders, food supply, access norms, simple value tokens, and records. It must be event-sourced and actor-known. Do not introduce market abstractions that erase provenance before the missing-property proof is stable.

Food, sleep, work, fatigue, hunger, wages, and local-economy effects share the
same event-sourced accounting seam and duration lifecycle. Ordinary-life
systems may consume charged effects and lifecycle state, but they may not
double-charge the same tick/window, mint independent need-deltas, or silently
repair duplicate body-exclusive terminals. Byte-stable goldens do not prove the
ordinary-life contract if two consumers causally charge the same actor/time
window twice.

### Routines and ordinary schedules

Routines provide recurring opportunities and expectations. They are defeasible. They cannot dispatch primitive actions. A routine window triggers the actor decision transaction.

### Spatial knowledge

Route knowledge is holder-known. Actors may know current room, visible exits, public roads, remembered paths, and directions from testimony/records. They may be wrong or incomplete. Pathfinding for cognition must use known route surfaces; validation checks physical truth.

### Domain packs

Domain packs may define settlement types, object types, routine templates, institutions, norms, and fixture seeds. They define possibility space and validation rules; they do not run outcome scripts.

## Required diagnostics / replay / TUI hooks

- TUI embodied view must show local place, visible exits, visible containers/items/actors, actor-known notebook, needs/routine status, and actor-legible why-not.
- Debug view must show truth/belief mismatch and event ancestry separately.
- No-human ordinary-life reports must show actor decision order, ordinary action events, waits/failures, no player/controller conditioning, and replay match.
- Content validation must reject starting actor knowledge, routines, records, property expectations, and workplace/bed/food knowledge without source tags.

## Acceptance implications

No later system can be considered accepted until ordinary-life proof gates pass without hidden truth planning. Road threats, bounties, expeditions, monsters, parties, regional notices, and adventure pressures are second proof or later. Architecture may define their future constraints, but they cannot become shortcuts around first proof.

## Anti-patterns

- “Missing property automatically creates a theft quest.”
- “Actor knows home/work/bed/food because the fixture says so.”
- “Work succeeds because the true assigned workplace exists.”
- “The road encounter system can spawn danger to create drama.”
- “Economy can be abstracted as score deltas before custody/property facts work.”
- “A clue is spawned when the player has not progressed.”

## Cross-document obligations

This document uses holder-known context from document 03, action pipeline from document 04, actor decision from document 05, claims/property beliefs from document 06, institutions/households from document 08, TUI proof from document 10, incident/lead boundaries from document 11, LOD deferral from document 12, and acceptance gates from document 13.
