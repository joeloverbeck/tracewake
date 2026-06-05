# Data Authoring, Validation, and Golden Fixtures

## Core rule

Author causal possibility space. Do not author outcome chains.

Data may define actors, places, objects, affordances, routines, beliefs, traces, norms, roles, reports, records, speech templates, pressures, and fixtures. Data may not force a plot.

Bad:

```yaml
script:
  - at 02:00 Mara steals Tomas coins
  - at 08:00 Tomas reports to Anna
  - at 10:00 Elias suspects Rafi
```

Good:

```yaml
scenario_seed:
  pressures:
    actor_mara:
      debt_pressure: high
      hunger: medium
  beliefs:
    actor_mara:
      - proposition: TomasLikelyStoresCoinsAtHome
        source: tavern_gossip_001
        confidence: 0.55
  expectations:
    actor_tomas:
      - proposition: ItemLocatedInContainer(coin_stack_01, strongbox_tomas)
        source: seed_evt_tomas_checked_strongbox
        confidence: 0.86
  affordances:
    strongbox_tomas:
      exposes: [open_container, search_container, remove_item, place_item]
  no_forced_outcomes: true
```

The simulation may produce theft, failed theft, refusal, confession, wrong suspicion, no report, or nothing. That is the point.

## File format posture

Do not freeze final syntax in the first proof.

Acceptable candidate formats:

- RON;
- TOML;
- YAML;
- JSON;
- custom reviewed format.

Execution cares about logical contracts:

- schema validation;
- stable IDs;
- deterministic loading order;
- referential integrity;
- readable errors;
- versioning;
- fixture reproducibility;
- migration support;
- no hidden scripting authority;
- TUI affordance coverage.

YAML-like examples in this document are contracts, not final syntax.

## Validation pipeline

Content must pass:

```text
syntax validation
 -> schema validation
 -> referential validation
 -> semantic validation
 -> action registry parity validation
 -> norm/procedure validation
 -> epistemic source validation
 -> no-script/no-outcome-chain validation
 -> no-player-privilege validation
 -> deterministic fixture validation
 -> TUI affordance coverage validation
 -> replay content-version compatibility validation
```

Validation failures must be understandable by writers and programmers.

## Package manifest

Every content package/fixture declares:

```yaml
ContentPackage:
  id: first_proof_missing_property_village
  version: 0.1.0
  compatible_kernel_contract: phase_1_to_phase_4
  deterministic_seed_policy: explicit
  source_documents:
    - foundation_authority
    - architecture_authority
    - execution_replacement
  fixtures:
    - strongbox_001
    - expectation_contradiction_001
  no_forced_outcomes: true
```

Replay must reject mismatched content versions unless an explicit migration path exists.

## Entity schema

```yaml
Entity:
  id: entity_id
  kind: actor | body | item | place | institution | household | record | trace | norm | role | controller_metadata
  display_name: string
  tags: [string]
  components: [component_ref]
```

Rules:

- IDs are stable.
- Generated IDs are deterministic.
- Display names do not define authority.
- `controller_metadata` is not world character identity.
- `PlayerCharacter` is forbidden.

## Place schema

```yaml
Place:
  id: room_tomas_bedroom
  kind: room
  parent: house_tomas
  privacy: household_private
  jurisdiction: reeves_office
  connects:
    - through: door_tomas_bedroom_hall
      to: hall_tomas_house
  visibility_profile: small_room_low_light
  sound_profile: muffled_by_closed_door
  affordances:
    - inspect_place
    - search_place
    - move_here_if_connected
```

Rules:

- Rooms and doors are epistemic architecture.
- Sound/visibility/privacy must be defined enough for first-proof observations.
- No map marker may point to hidden truth in embodied mode.

## Door schema

```yaml
Door:
  id: door_tomas_bedroom_hall
  connects: [room_tomas_bedroom, hall_tomas_house]
  open_state: closed
  lock_state: unlocked
  sound_occlusion: medium
  visibility_occlusion: high
  access_norms:
    - household_privacy
  affordances:
    - open_door
    - close_door
    - lock_door
    - unlock_door
    - listen_through
```

## Container schema

```yaml
Container:
  id: strongbox_tomas
  location: room_tomas_bedroom
  owner: actor_tomas
  custodian: actor_tomas
  contents: [coin_stack_01]
  open_state: closed
  lock_state: locked
  privacy: private_storage
  permitted_users: [actor_tomas]
  trace_profile:
    opened_carelessly: disturbed_container
    forced: lock_scratch
    noise: possible_metal_sound
  affordances:
    - inspect_container
    - open_container
    - close_container
    - search_container
    - remove_item
    - place_item
```

An affordance is not permission. Permission is evaluated through actor state, ownership, norms, willingness, and context.

## Item and custody schema

```yaml
Item:
  id: coin_stack_01
  kind: money_stack
  legal_owner: actor_tomas
  possessor: strongbox_tomas
  custodian: actor_tomas
  value: small_significant_sum
  portable: true
  visible_when_container_open: true
  expected_locations:
    actor_tomas:
      place_or_container: strongbox_tomas
      source: seed_evt_tomas_checked_strongbox
```

Separate:

- legal owner;
- possessor;
- custodian;
- expected location;
- permitted user;
- disputed claimant;
- institutional custody.

## Actor schema

```yaml
Actor:
  id: actor_mara
  body: body_mara
  household: household_mara
  home: house_mara
  roles:
    - part_time_worker
  needs:
    hunger: 41
    fatigue: 38
    safety: 64
  traits:
    risk_tolerance: medium
    property_respect: low_to_medium
    shame_sensitivity: high
  projects:
    - repay_debt_before_market_day
  relationships:
    actor_tomas:
      trust: low
      resentment: medium
  initial_beliefs:
    - belief_mara_tomas_likely_has_coins
  routines:
    - mara_workday
    - mara_evening_survival
```

Rules:

- major actors need at least one durable concern/project;
- actors must have homes or reasons for lacking homes;
- actors cannot know truth without source;
- actors cannot exist only as player-facing NPCs.

## Belief schema

```yaml
Belief:
  id: belief_tomas_coins_in_strongbox
  holder: actor_tomas
  proposition:
    type: ItemLocatedInContainer
    item: coin_stack_01
    container: strongbox_tomas
  stance: believed_true
  confidence: 0.86
  source:
    kind: direct_observation
    source_id: seed_evt_tomas_checked_strongbox
  acquired_at: seed_time
  last_verified_at: seed_time
  privacy: private_household
```

Rules:

- no important belief without source;
- `authored_initial` is allowed only with explicit source event or authored source artifact;
- belief confidence is not truth;
- records and institutions must also use source-backed beliefs/claims.

## Observation schema

```yaml
Observation:
  id: obs_elena_noise_001
  observer: actor_elena
  channel: sound
  observed_target: evt_container_noise_candidate
  place: hall_tomas_house
  time: 142-08-12T02:14
  raw_payload:
    intensity: low
    origin_estimate: near_tomas_bedroom
    material_hint: metal_or_wood
  confidence: 0.34
  interpreted_beliefs:
    - belief_elena_heard_possible_noise
```

Observation does not equal interpretation.

## Trace schema

```yaml
Trace:
  id: trace_disturbed_strongbox_001
  kind: disturbed_container
  created_by: evt_item_removed_from_container_001
  place: room_tomas_bedroom
  attached_to: strongbox_tomas
  visibility:
    sight: low
    touch: medium
  decay_rate: slow
  possible_interpretations:
    - proposition: ContainerOpenedRecently(strongbox_tomas)
      support: 0.55
    - proposition: OwnerOpenedNormally(strongbox_tomas)
      support: 0.20
    - proposition: ForcedEntry(strongbox_tomas)
      support: 0.10
  can_be_modified_by:
    - clean_container
    - rearrange_contents
```

Avoid one-meaning clues unless the cause is physically justified.

## Action definition schema

```yaml
ActionDef:
  id: remove_item_from_container
  display: Remove item from container
  actor_requirements:
    - has_body
    - can_reach_target
  parameters:
    item: EntityRef<Item>
    container: EntityRef<Container>
  preconditions:
    physical:
      - actor_at_container_location
      - container_open_or_actor_can_attempt_open
      - item_present_or_actor_searching
      - actor_can_carry
    knowledge:
      - actor_perceives_item OR actor_has_search_intention
    social:
      - actor_has_permission OR actor_willing_to_violate_property_norm
  duration:
    base: short
  traces:
    - disturbed_container_if_careless
    - noise_if_failed_stealth
  norm_hooks:
    - property_theft
    - trespass_privacy
  failure_modes:
    - item_not_found
    - interrupted
    - caught_in_act
    - refuses_to_violate_norm
  tui:
    semantic_id: action.remove_item_from_container
    why_not_templates: source_bound
```

Action definitions create possibility, not outcome.

## Routine schema

```yaml
RoutineTemplate:
  id: miller_workday
  actor_role: miller
  applicability:
    - actor_believes_today_is_workday
    - actor_not_too_fatigued
    - actor_can_reach_workplace
    - no_stronger_interruption
  steps:
    - wake
    - eat_if_hungry_and_food_available
    - travel_to_mill
    - open_mill_if_responsible
    - work_block
    - handle_customer_or_record_payment_if_any
    - return_home_or_social_stop
  interruptions:
    - missing_property_contradiction
    - office_summons
    - hunger_too_high
    - fatigue_too_high
    - missing_tool
    - locked_workplace
  failure_modes:
    - no_food
    - route_blocked
    - work_refused
    - interrupted
```

A routine without failure modes is invalid.

## Institution schema

```yaml
Institution:
  id: reeves_office
  kind: local_authority
  jurisdiction:
    places: [village_core]
  roles:
    actor_anna: [clerk]
    actor_elias: [guard]
    actor_reeve: [reeve]
  resources:
    staff_time: limited
    paper: sufficient_for_first_fixture
  records:
    incident_ledger: ledger_reeves_incidents
  norms:
    - property_theft
    - trespass_privacy
  procedures:
    - receive_missing_property_report
    - open_incident_record
    - question_possible_witness
```

## Report and record schema

```yaml
ReportSpeechAct:
  id: speech_report_missing_property
  kind: report
  speaker: actor_tomas
  listeners: [actor_anna]
  propositions:
    - ItemMissing(coin_stack_01)
    - LastExpectedIn(coin_stack_01, strongbox_tomas)
  source_beliefs:
    - belief_tomas_strongbox_empty
    - belief_tomas_coins_in_strongbox
  asserted_confidence: mixed
  validation:
    requires_listener_attention: true
    requires_reporter_actor_known_basis: true

Record:
  id: ledger_entry_missing_coins_001
  kind: incident_ledger_entry
  institution: reeves_office
  physical_artifact: ledger_page_043
  author: actor_anna
  reporter: actor_tomas
  receiver: actor_anna
  claims:
    - proposition: ItemMissing(coin_stack_01)
      source: speech_report_missing_property
      confidence: claimant_high
    - proposition: LastExpectedIn(coin_stack_01, strongbox_tomas)
      source: speech_report_missing_property
      confidence: claimant_high
  status: open
```

Record claims are not truth.

## Norm schema

```yaml
Norm:
  id: property_theft
  kind: prohibition
  action_pattern: TakePropertyWithoutPermission
  conditions:
    - item_has_legal_owner
    - actor_lacks_permission
    - no_emergency_exception
  violation_event: PropertyViolationClassified
  detection_channels:
    - direct_observation
    - expectation_contradiction
    - trace_interpretation
    - testimony
    - possession_evidence
  thresholds:
    open_incident: low
    question_suspect: medium
    search_private_space: high
    sanction: high
```

Violation does not imply detection or sanction.

## Controller/view-model schema

```yaml
HumanController:
  id: human_01
  attached_actor: actor_tomas
  mode: embodied | debug
  possession_history: [actor_mara, actor_tomas]
  world_truth_effect: none

EmbodiedViewModel:
  viewer: actor_tomas
  perceived_place: room_tomas_bedroom
  visible_entities: [strongbox_tomas, bed_tomas]
  beliefs:
    - belief_tomas_coins_in_strongbox
  affordances:
    - search_container
    - ask_elena
    - report_missing_property_if_known
  forbidden_sources:
    - hidden_event_log_truth
    - debug_notes
    - previous_actor_knowledge

DebugViewModel:
  event_log: all
  causal_graph: all
  hidden_truth: allowed
  belief_stores: all
  possession_history: debug_metadata
```

## Golden fixtures

### strongbox_001

Purpose: prove the core missing-property chain.

Setup:

- Tomas expects coin_stack_01 in strongbox_tomas.
- Mara has debt/hunger or opportunity pressure and uncertain belief about coins.
- Elena may be in hearing range.
- Strongbox is physically reachable under some conditions.
- No outcome script forces theft.

Expected assertions:

- if Mara takes/moves coin, event log records action and causes;
- Tomas does not know until checking, being told, or inferring through valid channel;
- Elena's observation is uncertain if she hears anything;
- event log explains trace/observation chain;
- no player-only action used.

### expectation_contradiction_001

Purpose: prove absence-as-evidence.

Setup:

- Tomas has source-backed expectation.
- Coin is absent by prior event or variant fixture.
- Tomas searches/checks strongbox.

Expected:

- observation created;
- contradiction linked to expectation;
- belief update created;
- candidate goals arise;
- actor with no expectation does not receive missing-item contradiction.

### possession_parity_001

Purpose: prove no sacred player.

Setup:

- Human controls Mara and takes/moves coin.
- Human switches to Tomas or Elias.

Expected:

- new body lacks Mara/player-only knowledge;
- previous body remains ordinary actor;
- current body can search/report only from actor knowledge;
- debug shows possession history as non-diegetic.

### report_record_001

Purpose: prove report and record provenance.

Setup:

- Tomas knows item missing.
- Anna is available or unavailable depending variant.
- Office hours defined.

Expected:

- report is speech act;
- clerk receives only if reachable/available or explicit exception;
- ledger record created from claims;
- institution does not learn ground truth;
- report may be refused/delayed in failure variants.

### wrong_suspicion_001

Purpose: prove fallible reasoning.

Setup:

- partial testimony, rumor, access, bias, or trace points toward wrong actor.
- Correct culprit exists or may be unknown.
- Institution has limited knowledge.

Expected:

- suspicion attaches for visible source-backed reasons;
- correct culprit not required;
- suspicion threshold is actor/institution specific;
- debug explains wrongness without embodied leakage.

### no_human_day_001

Purpose: prove ordinary life without player.

Setup:

- 10–20 high-detail agents;
- homes, beds, food, work, office hours, routines;
- no controller bound.

Expected:

- agents sleep/eat/work/rest/speak minimally;
- interruptions occur for modeled reasons;
- no player references;
- debug can attach afterward.

### view_filtering_001

Purpose: prove same history supports different views.

Setup:

- hidden truth known to debug;
- Tomas, Mara, Elena, Anna have different beliefs.

Expected:

- embodied views differ by actor;
- debug view reveals truth;
- no embodied view contains hidden truth without source.

### replay_rebuild_001

Purpose: prove forensic backbone.

Setup:

- event log with item movement, observation, belief, report, record.
- content version manifest.

Expected:

- replay rebuilds state/projections;
- mismatch/migration issue fails loudly;
- causal graph remains inspectable.

## Schema anti-patterns

Forbidden:

- `Quest`;
- `PlayerCharacter`;
- `ObjectiveMarker.true_location`;
- `NPC.knows_truth`;
- `Reward.spawn_on_completion`;
- `LLM_mutated_state`;
- `scripted culprit`;
- `scripted accusation`;
- `scripted report acceptance`;
- `case_reads_ground_truth`;
- `FantasyKernelRole`;
- `CombatFirstActorState`;
- `notice_auto_updates_to_truth`;
- `bounty_from_incident_without_procedure`.

## Review checklist

Every authored element must answer:

```text
What process creates this?
Who knows it first?
How can others learn?
What traces exist?
How can it be misread?
What institution/norm cares?
Can an NPC do the same kind of thing?
Can the TUI expose it without leaking truth?
Can it run without a human?
Can replay rebuild it?
Does it require player privilege, hidden scripting, or LLM authority?
```

If not, reject the data.
