# State Model, Entities, Components, and Content Data

## Status

This document defines Tracewake's state and authored-data architecture. It is not a final file-format decision and does not freeze a particular ECS, database, serialization crate, or content editor.

The state model must support event sourcing, replay, actor knowledge, object affordances, institutions, ordinary life, domain packs, TUI view models, and validation.

## Core rule

Authored data creates **causal possibility space**. It may define actors, places, routines, actions, norms, records, templates, institutions, starting conditions, and scenario pressures. It must not encode authored outcome chains.

Bad:

```yaml
script:
  - mara steals tomas coins at midnight
  - tomas reports to clerk at 09:00
  - guard arrests wrong suspect at 11:00
```

Good:

```yaml
scenario_seed:
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
      can_be_opened_by: [key_tomas, lockpick_attempt]
  norms:
    - household_privacy
    - property_theft
```

The simulation may produce theft, failed theft, avoidance, confession, rumor, accusation, or nothing. Content authors define conditions, not guaranteed arcs.

## State is not one thing

Tracewake separates:

```text
content definitions       versioned authored possibility space
starting state            scenario seed projected into entities/components
current state projection  rebuildable from events/snapshot
belief state              actor-specific, source-bound, event-derived where meaningful
institutional records     world artifacts and projections with provenance
view models               actor-filtered or debug projections
save package              event log + snapshots + content manifest + projection metadata
```

The authoritative answer to "what happened?" lives in the event log. The convenient answer to "what is currently true?" lives in current state projections.

## Entity identity

Entities are stable identities for world things and social/institutional artifacts.

Entity kinds include:

- actor;
- body;
- household;
- item;
- container;
- door;
- room;
- building;
- workplace;
- bed;
- food item or stack;
- money/currency unit or account-like custody object;
- key/tool;
- road/route segment;
- settlement;
- region;
- institution;
- role assignment;
- norm definition instance;
- record artifact;
- ledger entry;
- notice;
- trace;
- rumor packet;
- contract/request/obligation;
- boundary process.

Entity IDs must be stable across replay. Generated IDs must be deterministic under the same seed, content version, and event history.

## Components

Components are typed state fragments. Components are not scripts. Components must be serializable, validateable, and replayable.

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
    household_tomas: strongbox_tomas
  permitted_users: [actor_tomas, actor_elena]
  disputed_claimants: []

Container:
  capacity: small
  contents: [coin_stack_01]
  lock_state: locked
  privacy_level: household_private
  trace_profile: ordinary_wooden_strongbox

Door:
  connects: [room_tomas_bedroom, hall_tomas_house]
  open_state: closed
  lock_state: unlocked
  sound_occlusion: medium
  visibility_occlusion: high
  access_norms: [household_privacy]

ActorBody:
  hunger: 22
  fatigue: 63
  safety: 77
  pain: 0
  injuries: []
  carrying_capacity: modest

MindRef:
  belief_store: belief_store_tomas
  intention_stack: intentions_tomas
  memory_profile: ordinary_adult

HouseholdMembership:
  household: household_tomas
  role: head
  domestic_permissions: [sleep, use_kitchen, access_private_storage]
```

Exact storage layout may evolve. The semantic boundaries should not.

## Relationships as state

Relationships are not decorative tags. They affect trust, testimony, permissions, obligations, household access, gossip uptake, suspicion, work, and conflict.

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

Relationship updates require causal events or summarized causal ancestry.

## Content definition families

Tracewake content/data includes:

- entity templates;
- component templates;
- scenario seeds;
- fixture worlds for tests;
- domain packs;
- action definitions;
- object affordance definitions;
- place affordance definitions;
- norm definitions;
- role definitions;
- institution definitions;
- household templates;
- routine templates;
- HTN method definitions;
- planner domain fragments;
- speech-act templates;
- language rendering templates;
- record and ledger schemas;
- notice templates;
- trace profiles;
- perception channel definitions;
- memory decay profiles;
- LOD policies;
- validation rules.

## Domain packs

The kernel is genre-agnostic. Domain packs define setting-specific vocabulary and entities using core concepts.

Domain packs may define:

- body/species types;
- technology level;
- occupations;
- foods;
- tools;
- currencies or value stores;
- local calendar labels;
- institutions and norms;
- item categories;
- place categories;
- speech/culture tags;
- special information channels;
- diseases/conditions;
- threat processes;
- scenario seeds.

Domain packs may not:

- introduce player-only verbs;
- mutate state without events;
- create hidden facts from prose;
- bypass action validation;
- bypass belief provenance;
- create quest objects as engine ontology;
- assume fantasy, combat, magic, taverns, coins, feudal law, or humans in the kernel;
- make LLM output authoritative.

The first domain pack is neutral medieval-ish ordinary life without magic. It is a first test domain, not the kernel's identity.

## Action definitions as data

Action definitions should be data-driven where practical, but not at the cost of untestable mini-programming languages.

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

Core actions may require hand-written kernel logic behind data-defined contracts. Data defines the contract; the kernel validates execution.

## Object affordances

Objects expose typed affordances. This makes ordinary life playable and agent planning tractable.

```yaml
AffordanceSet:
  target: strongbox_tomas
  exposes:
    - action: inspect_container
      visible_when: actor_can_perceive_container
    - action: open_container
      visible_when: actor_can_reach_container
      conditions_hint: may_require_key_or_force
    - action: remove_item_from_container
      visible_when: actor_knows_or_searches_contents
    - action: place_item_in_container
      visible_when: actor_can_reach_container
```

Affordance visibility is actor-filtered. Debug may show hidden affordances and failed preconditions.

## Norm definitions

Norms are explicit data, but norm processing is kernel/institution architecture.

```yaml
NormDefinition:
  id: property_theft
  kind: prohibition
  scope:
    jurisdiction: reeves_office
  action_pattern: take_property_without_permission
  conditions:
    - item_has_legal_owner
    - actor_lacks_permission
    - actor_not_emergency_exception
  violation_event: PropertyViolationClassified
  detection_model:
    requires_observation_or_trace: true
  institutional_relevance:
    reportable: true
    evidence_thresholds:
      open_incident: low
      accuse_named_suspect: medium
      sanction: high
```

Norm data must preserve the distinction between violation, detection, suspicion, report, record, proof, and sanction.

## HTN and routine data

Routines are defeasible methods, not teleports.

```yaml
HTNMethod:
  id: ordinary_work_morning
  task: Workday(actor, workplace)
  applicability:
    - actor_believes_workday
    - actor_has_role_for_workplace
    - workplace_reachable
    - actor_need_fatigue_below_blocking_threshold
  steps:
    - satisfy_urgent_hunger_if_needed
    - travel_to_workplace
    - open_workplace_if_responsible
    - perform_work_blocks_until_interrupt_or_close
    - record_debts_or_payments_if_any
  failure_modes:
    - workplace_locked
    - tool_missing
    - summoned_by_authority
    - fear_prevents_travel
    - fatigue_too_high
```

A schedule creates expectations and decision points. It does not move actors by fiat.

## Speech-act templates

V1 uses structured speech-act menus and deterministic templates.

```yaml
SpeechActTemplate:
  id: report_missing_property_to_clerk
  kind: Report
  required_source_beliefs:
    - actor_believes_item_missing
    - actor_has_last_expected_location
  propositions:
    - ItemMissing(item)
    - LastExpectedIn(item, container_or_place)
  allowed_uncertainty: required
  surface_templates:
    - "I need to report missing property. {item} should have been in {place}."
```

Templates do not create facts. They surface structured claims from actor-held beliefs or deliberate lies/speculation.

## Record schemas

Records are world artifacts with authors, readers, physical/digital location, claims, and lifecycle.

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
    evidence_refs: [TraceOrRecordRef]
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

## Scenario seeds

A scenario seed defines starting conditions and pressures.

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

Fixtures for deterministic tests may set up tighter conditions, but they should still exercise the action pipeline unless explicitly testing validation failures.

## File format stance

Prefer human-editable files where practical. YAML-like, TOML-like, RON-like, JSON-like, or custom DSL formats may be evaluated.

Architecture cares more about:

- schema validation;
- precise error messages;
- versioning;
- referential integrity;
- deterministic loading order;
- stable IDs;
- reviewability;
- test fixture ergonomics;
- migration support;
- no hidden scripting authority.

Do not freeze the format before content iteration proves the needs.

## Validation layers

Content must pass validation before use.

```text
syntax validation
 -> schema validation
 -> referential validation
 -> domain semantic validation
 -> action registry parity validation
 -> norm/procedure validation
 -> epistemic source validation
 -> no-script/no-outcome-chain validation
 -> no-player-privilege validation
 -> determinism/replay fixture validation
 -> TUI affordance coverage validation
```

Validation failures should be readable by writers and programmers.

## Versioning

Content packages must declare:

- package ID;
- semantic version or content hash;
- dependency versions;
- event schema compatibility;
- projection compatibility;
- migration notes;
- deterministic seed policy;
- test fixture coverage.

Replay must reject mismatched content versions unless a migration path exists.

## Acceptance implications

A domain pack or scenario is acceptable only if:

- it can run no-human simulation;
- it exposes TUI-playable affordances;
- it uses the shared action pipeline;
- all important beliefs have sources;
- records have authors and artifacts;
- norms distinguish violation/detection/proof/sanction;
- no event is created by prose alone;
- no player-only action exists;
- LLM-disabled operation works;
- projection rebuild succeeds.

## Anti-patterns

- Content file directly changes inventory at a scheduled time.
- Domain pack defines `quest_target` as a kernel category.
- A record appears without author or writing event.
- A rumor has no origin or source chain.
- A container is an inventory list with no location, access, trace, or ownership model.
- A schedule teleports an actor.
- A data format is chosen because it is trendy while validation is weak.
- A crate/framework dictates domain architecture.
