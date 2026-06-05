# State, Content Authoring, Domain Packs, and Schema Validation

## Status

This document defines Tracewake's state and authored-content architecture. It does not freeze a file format, editor, ECS, database, serialization library, or content compiler implementation.

The state model must support event sourcing, replay, actor knowledge, object affordances, institutions, ordinary life, domain packs, TUI view models, debug inspection, and validation.

## Core rule

Authored content creates **causal possibility space**. It may define actors, places, objects, actions, norms, records, routines, beliefs, relationships, starting conditions, and scenario pressures. It must not encode authored outcome chains.

Bad:

```yaml
script:
  - mara steals tomas_coins at midnight
  - tomas reports to clerk at 09:00
  - guard arrests wrong suspect at 11:00
```

Good:

```yaml
ScenarioSeed:
  actors:
    mara:
      debt_pressure: high
      believes_tomas_stores_coins: plausible_hearsay
      risk_tolerance: medium
    tomas:
      expected_property:
        coin_stack_01: strongbox_tomas
  affordances:
    strongbox_tomas:
      can_be_opened_by: [key_tomas, lockpick_attempt, force_attempt]
  norms:
    - household_privacy
    - property_theft
  no_forced_outcomes: true
```

The simulation may produce theft, failed theft, borrowing, avoidance, confession, rumor, wrong accusation, or nothing.

## Authority

This subsystem owns:

- entity and component template definitions;
- content package/domain pack manifests;
- scenario seeds and deterministic fixtures;
- action definitions and affordance sets;
- norm, role, institution, household, record, notice, and procedure definitions;
- routine and HTN method definitions;
- speech-act templates and deterministic language templates;
- trace, perception, memory, and LOD policy definitions;
- schema, semantic, no-script, parity, and replay-fixture validation reports.

It is denied:

- event commit;
- direct state mutation at scheduled times;
- player-only verbs;
- quest ontology;
- hidden outcome chains;
- ground-truth UI markers;
- LLM authority;
- setting/genre assumptions in the kernel.

## State is not one thing

Tracewake separates:

```text
content definitions       versioned authored possibility space
starting state            scenario seed projected into entities/components
current state projection  rebuildable from events/snapshot
belief state              actor-specific, source-bound, event-derived where meaningful
institutional records     world artifacts and projections with provenance
view models               actor-filtered or debug projections
save package              event log + content/schema manifest + snapshots + projection metadata
```

The authoritative answer to “what happened?” is the event log. The convenient answer to “what is currently true?” is a projection.

## Entity identity

Entities are stable identities for world things, agents, places, artifacts, and social/institutional objects.

Entity kinds include:

- actor and body;
- household;
- item, stack, tool, key, food, money/value token;
- container;
- door;
- room, building, workplace, settlement, route, region;
- bed and storage object;
- institution;
- role assignment;
- norm definition instance;
- record artifact, ledger page, notice, contract, debt, promise;
- trace;
- rumor packet;
- boundary process.

Entity IDs must be stable across replay. Generated IDs must be deterministic under the same seed, content version, and event history.

## Component contract

Components are typed state fragments. Components are not scripts. They must be serializable, validatable, replayable, and projection-friendly.

Example component shapes:

```yaml
Position:
  place: room_tomas_bedroom
  containment: null

Ownership:
  legal_owner: actor_tomas
  possessor: strongbox_tomas
  custodian: null
  expected_location_by:
    actor_tomas: strongbox_tomas
  permitted_users: [actor_tomas]
  disputed_claimants: []

Container:
  capacity: small
  contents: [coin_stack_01]
  lock_state: locked
  privacy_level: household_private
  trace_profile: ordinary_wooden_strongbox

ActorBody:
  hunger: 22
  fatigue: 63
  safety: 77
  injuries: []
  carrying_capacity: modest

MindRef:
  belief_store: belief_store_tomas
  intention_stack: intentions_tomas
  memory_profile: ordinary_adult

HouseholdMembership:
  household: household_tomas
  role: head_or_member
  domestic_permissions: [sleep, use_kitchen, access_private_storage]
```

Exact storage layout may evolve. Semantic meaning may not be weakened.

## Relationships as state

Relationships are not flavor tags. They affect trust, testimony, permissions, obligations, gossip uptake, suspicion, fear, work, help, conflict, and institutional treatment.

```yaml
Relationship:
  from: actor_elena
  to: actor_tomas
  kinship: spouse
  trust: 0.88
  affection: 0.81
  fear: 0.04
  debt: none
  recent_events:
    - evt_argument_about_money_17
  privacy_access:
    household: high
```

Relationship changes require causal events or summarized causal ancestry.

## Content families

Content/data includes:

- entity templates;
- component templates;
- scenario seeds;
- deterministic fixtures;
- domain packs;
- action definitions;
- affordance definitions;
- norm definitions;
- role definitions;
- institution and household definitions;
- routine templates;
- HTN methods and planner fragments;
- speech-act templates;
- deterministic surface templates;
- record/ledger/notice schemas;
- trace profiles;
- perception channels;
- memory decay profiles;
- LOD policies;
- validation rules.

## Domain packs

The kernel is genre-agnostic. Domain packs define setting-specific vocabulary and entities using core concepts.

Domain packs may define:

- species/body types;
- technology level;
- occupations;
- foods;
- tools;
- currencies/value stores;
- calendars;
- institutions and norms;
- item/place categories;
- speech/culture tags;
- special information channels;
- diseases/conditions;
- threat processes;
- scenario seeds.

Domain packs may not:

- introduce player-only verbs;
- mutate state without events;
- create facts from prose;
- bypass action validation;
- bypass belief provenance;
- create quest objects as engine ontology;
- assume fantasy, combat, magic, taverns, feudal law, coins, or humans in the kernel;
- make LLM output authoritative.

The first domain pack is neutral medieval-ish ordinary life without magic. It is a proof context, not the kernel identity.

## Actions and affordances as data

Action definitions should be data-driven where practical, but not by inventing untestable mini-programming languages.

```yaml
ActionDefinition:
  id: sleep_in_bed
  category: ordinary_life
  afforded_by: [Bed]
  actor_requirements:
    body_capabilities: [can_lie_down]
  parameters:
    bed: EntityRef<Bed>
  preconditions:
    physical:
      - actor_can_reach_bed
      - bed_not_occupied_or_actor_can_share_by_norm
    knowledge:
      - actor_knows_bed_location OR actor_can_search_home
    social_normative:
      - actor_has_domestic_access OR actor_willing_to_trespass
  duration:
    default: until_restored_or_interrupted
  traces:
    - bed_occupied
    - disturbed_bedding
  events:
    start: SleepStarted
    progress_summary: SleepPeriodCompleted
    interrupted: SleepInterrupted
```

Core actions may use handwritten kernel logic behind data-defined contracts. Data defines possibility and constraints; the kernel validates execution.

## Norm definitions

Norm data must preserve violation/detection/suspicion/report/proof/sanction separation.

```yaml
NormDefinition:
  id: property_theft
  kind: prohibition
  scope:
    jurisdiction: reeves_office
  action_pattern: take_property_without_permission
  conditions:
    - item_has_recognized_owner
    - actor_lacks_permission
    - no_emergency_exception
  possible_violation_event: PropertyViolationClassified
  detection_model:
    requires_observation_or_trace_or_report: true
  institutional_relevance:
    reportable: true
    thresholds:
      open_incident: low
      question_named_suspect: medium
      sanction: high
```

## Record schemas

Records are world artifacts with authors, issuers, locations, claims, and lifecycle events.

```yaml
RecordSchema:
  id: incident_ledger_entry
  institution: reeves_office
  fields:
    reporter: ActorRef
    receiver: ActorRef
    claims: ClaimList
    received_at: SimTime
    status: open_or_closed
    evidence_refs: [TraceOrRecordOrBeliefRef]
    physical_artifact: EntityRef<RecordPage>
  lifecycle_events:
    - ReportReceived
    - LedgerEntryCreated
    - EntryAmended
    - EntryContradicted
    - EntryDestroyed
    - CaseClosed
```

A record's existence is itself a cause for future beliefs and actions.

## Scenario seeds and fixtures

Scenario seeds define starting conditions and pressures.

```yaml
ScenarioSeed:
  id: first_village_missing_property
  domain_pack: neutral_village
  start_time: 142-08-12T05:30
  high_detail_agents: [actor_tomas, actor_elena, actor_mara, actor_anna_clerk]
  households: [household_tomas, household_mara]
  institutions: [reeves_office]
  initial_beliefs:
    actor_tomas:
      - proposition: ItemLocatedInContainer(coin_stack_01, strongbox_tomas)
        source: prior_direct_observation_fixture
        confidence: high
    actor_mara:
      - proposition: TomasLikelyStoresCoinsAtHome
        source: tavern_gossip
        confidence: medium
  pressures:
    actor_mara:
      debt: high
  no_forced_outcomes: true
```

Fixtures may set tight conditions for deterministic tests, but normal behavior fixtures should still use the action pipeline. A fixture that bypasses the pipeline must be marked as a validation/unit-test fixture, not normal simulation behavior.

## File format stance

Prefer human-reviewable content where practical. YAML-like, TOML-like, RON-like, JSON-like, or custom formats may be evaluated.

Architecture cares more about:

- schema validation;
- precise writer-facing errors;
- referential integrity;
- deterministic loading order;
- stable IDs;
- versioning and migration;
- reviewability;
- fixture ergonomics;
- no hidden scripting authority.

Do not freeze the format before content iteration proves the needs.

## Validation layers

Content must pass validation before use.

```text
syntax validation
 -> schema validation
 -> referential validation
 -> deterministic loading validation
 -> domain semantic validation
 -> action registry parity validation
 -> norm/procedure validation
 -> epistemic source validation
 -> record/artifact validation
 -> no-script/no-outcome-chain validation
 -> no-player-privilege validation
 -> TUI affordance coverage validation
 -> replay fixture validation
```

Validation failures should be readable by both writers and programmers.

## Versioning

Content packages must declare:

- package ID;
- semantic version and/or content hash;
- dependency versions;
- event schema compatibility;
- projection compatibility;
- migration notes;
- deterministic seed policy;
- fixture coverage;
- LLM-disabled behavior if language templates exist.

Replay must reject mismatched content versions unless an explicit migration path exists.

## Acceptance implications

A domain pack, scenario, or content feature is acceptable only if:

- it can run no-human simulation;
- it exposes TUI-playable affordances;
- it uses the shared action/event pipeline;
- important beliefs have sources;
- records have authors and artifacts;
- norms distinguish violation, detection, proof, and sanction;
- no event is created by prose alone;
- no player-only action exists;
- LLM-disabled operation works;
- projection rebuild succeeds.

## Anti-patterns

- Content file directly changes inventory at midnight.
- Domain pack defines `quest_target` as a kernel category.
- Record appears without author or writing event.
- Rumor has no origin or source chain.
- Container is an inventory list with no location, access, trace, or ownership model.
- Schedule teleports an actor.
- Data format is chosen because fashionable while validation is weak.
- Crate/framework dictates domain architecture.
