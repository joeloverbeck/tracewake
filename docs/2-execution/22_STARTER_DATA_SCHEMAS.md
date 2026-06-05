# Starter Data Schemas

These are provisional vocabulary, not final engine formats. They exist to keep doctrine concrete enough to resist wrong architecture.

Every schema should support:

```text
What caused this?
Who knows it?
Who can act on it?
How could they be wrong?
How can the TUI show it without leaking truth?
```

## Entity

```yaml
Entity:
  id: entity_id
  kind: actor | body | item | place | institution | record | notice | trace | group | route | process | norm | role
  display_name: string
  domain_tags: [string]
  components:
    Position: optional
    Ownership: optional
    ActorBody: optional
    MindRef: optional
    Container: optional
    Door: optional
    InstitutionMembership: optional
    RecordRef: optional
```

## Event

```yaml
Event:
  id: evt_000001
  kind: string
  world_tick: integer
  sim_time: string
  actor: optional_entity_id
  participants: map
  place: optional_place_id
  causes:
    - event_id | intention_id | belief_id | need_id | process_id | exogenous_cause
  preconditions:
    - name: string
      result: true | false
      details: optional
  effects:
    - target: entity_id
      component: string
      field: string
      old: any
      new: any
  traces_created: [trace_id]
  observations_created: [observation_id]
  records_created: [record_id]
  speech_acts: [speech_act_id]
  norm_implications: [norm_result_id]
  lod_context: optional
```

## ActionDef

```yaml
ActionDef:
  id: action_take_item
  display: Take item
  actor_requirements:
    - has_body
    - can_reach_target
  parameters:
    item: EntityRef<Item>
    source: EntityRef<Container|Place|Actor>
  preconditions:
    physical:
      - item_present_at_source
      - actor_can_carry
    social:
      - actor_has_permission OR actor_willing_to_violate_property_norm
    knowledge:
      - actor_perceives_item OR actor_searching_source
    institutional:
      - not_blocked_by_role_order
  cost:
    time: duration
    fatigue: optional
    risk: optional
  effects:
    - change_possessor
    - remove_from_source
    - add_to_actor_inventory
  traces:
    - disturbed_container_if_source_container
    - noise_if_failed_stealth
  norm_checks:
    - theft
  failure_modes:
    - interrupted
    - caught_in_act
    - item_not_found
  tui:
    why_not_templates: map
```

## Belief

```yaml
Belief:
  id: belief_id
  holder: actor_id | institution_id | group_id
  proposition:
    type: string
    args: map
  truth_value: believed_true | believed_false | uncertain
  confidence: 0.0-1.0
  source:
    kind: direct_observation | hearsay | rumor | record | inference | instruction | memory | domain_channel | authored_initial
    source_id: event_id | actor_id | record_id | trace_id | null
  believed_event_time: optional_time
  acquired_at: sim_time
  emotional_weight: 0.0-1.0
  decay_profile: none | slow | normal | fast
  contradictions: [belief_id]
  privacy:
    speakable: true | false
    shameful: true | false
    privileged: true | false
```

## Observation

```yaml
Observation:
  id: observation_id
  observer: actor_id
  observed_target: event_id | trace_id | entity_id | absence_marker | speech_act_id
  channel: sight | sound | smell | touch | reading | speech | inference | domain_channel
  place: place_id
  time: sim_time
  raw_payload: map
  confidence: 0.0-1.0
  interpreted_beliefs: [belief_id]
```

## SpeechAct

```yaml
SpeechAct:
  id: speech_act_id
  kind: tell | ask | answer | report | accuse | deny | lie | confess | promise | threaten | bargain | recruit | refuse | warn | gossip | instruct | command | testify | withhold | apologize
  speaker: actor_id
  listeners: [actor_id]
  propositions: [proposition]
  source_beliefs: [belief_id]
  asserted_confidence: 0.0-1.0
  intention: optional_intention_id
  surface_text_ref: optional_text_id
  validation:
    status: proposed | committed | rejected | downgraded
    reason: optional_string
```

## Trace

```yaml
Trace:
  id: trace_id
  kind: footprint | blood | disturbed_container | missing_item | broken_lock | rumor | record | absence | erased_trace | domain_trace
  created_by: event_id
  place: optional_place_id
  attached_to: optional_entity_id
  visibility: 0.0-1.0
  decay_rate: none | slow | normal | fast
  possible_interpretations:
    - proposition: map
      support: 0.0-1.0
      contradicts: optional
  can_be_modified_by: [action_id]
```

## Intention

```yaml
Intention:
  id: intention_id
  holder: actor_id
  goal:
    type: string
    args: map
  source:
    kind: need | belief_contradiction | role_obligation | opportunity | order | emotion | routine | project
    source_id: optional
  commitment: weak | normal | strong | obsessive
  started_at: sim_time
  current_method: optional_htn_method
  plan_stack: [task_or_action_id]
  interruption_policy: map
```

## Project

```yaml
Project:
  id: project_id
  owner: actor_id | institution_id
  goal:
    type: string
    args: map
  horizon: days | weeks | months | years
  importance: low | medium | high
  state: active | suspended | completed | failed
  source_events: [event_id]
  current_concerns: [intention_id]
```

## Institution

```yaml
Institution:
  id: institution_id
  kind: local_authority | household | guild | temple | market | bandit_group | adventuring_company | domain_institution
  members: [actor_id]
  roles:
    actor_id: [role_id]
  jurisdiction:
    places: [place_id]
    people_scope: string
  resources:
    money: integer
    staff_time: integer
    goods: map
  records:
    incident_ledger: [record_id]
    bounty_ledger: [record_id]
    tax_ledger: [record_id]
  norms: [norm_id]
  procedures: [procedure_id]
```

## Role

```yaml
Role:
  id: role_id
  institution: institution_id
  obligations: [norm_id]
  permissions: [norm_id]
  prohibitions: [norm_id]
  access_rights: [place_or_record_id]
  credibility_modifiers: map
```

## Norm

```yaml
Norm:
  id: norm_id
  kind: prohibition | obligation | permission | constitutive | procedural | status | sanction
  applies_to:
    action_pattern: optional
    role: optional
    place: optional
    condition: optional
  conditions: [predicate]
  violation_event: optional_event_kind
  sanctions: [sanction_ref]
  detection_channels: [channel]
```

## Procedure

```yaml
Procedure:
  id: procedure_id
  institution: institution_id
  trigger: [event_or_belief_pattern]
  conditions: [predicate]
  steps: [task_or_action_ref]
  failure_modes: [failure_kind]
  records_created: [record_kind]
  authority_required: [role_id]
```

## Place and Route

```yaml
Place:
  id: place_id
  kind: room | building | settlement | route_segment | wilderness_site | region | external_boundary | domain_place
  parent: optional_place_id
  connects_to: [place_id]
  jurisdiction: [institution_id]
  affordances: [action_id]
  lod_tier: 0 | 1 | 2 | 3
  privacy: public | semi_public | private | restricted
  sound_profile: map
  visibility_profile: map

Route:
  id: route_id
  connects: [place_id]
  segments: [place_id]
  travel_time_base: duration
  actual_risks: map
  believed_risks_by_actor_or_public: map
  traffic_profile: map
  jurisdiction: [institution_id]
```

## Notice and Lead

```yaml
Notice:
  id: notice_id
  issuer: actor_id | institution_id
  author: actor_id
  authorized_by: actor_id | institution_id
  structured_claims:
    - proposition: map
      confidence: 0.0-1.0
      source_record: record_id
  offer:
    reward: optional
    proof_required: optional
    funds_reserved_ref: optional
  posted_at: place_id
  posted_time: sim_time
  last_verified: sim_time
  status: drafted | carried | posted | removed | destroyed | archived
  physical_artifact: entity_id

LeadCard:
  id: lead_id
  known_to_actor: actor_id
  source: observation_id | belief_id | notice_id | record_id | rumor_id
  title: string
  claims: [proposition]
  uncertainty_notes: [string]
  related_entities: [entity_id]
  suggested_actions: [action_id]
  stale_status: fresh | uncertain | stale | contradicted
```

## Controller and UI view models

```yaml
HumanController:
  id: controller_id
  attached_actor: actor_id
  mode: embodied | debug
  possession_history: [actor_id]

EmbodiedViewModel:
  actor: actor_id
  perceived_place: place_view
  actor_status: status_view
  nearby_affordances: [action_option]
  beliefs: [belief_view]
  leads: [lead_view]
  conversation: optional_conversation_view
  log: [actor_perceived_event_summary]

DebugViewModel:
  event_log: [event_id]
  causal_graph_ref: graph_id
  component_snapshot_ref: snapshot_id
  belief_stores: [belief_store_ref]
  institution_debug: [institution_id]
  planner_trace: [planner_trace_id]
  lod_state: [lod_debug]
```

## Domain pack manifest

```yaml
DomainPack:
  id: neutral_low_fantasy_village
  kernel_version: semver
  species: [species_def]
  body_types: [body_def]
  technology_tags: [string]
  institutions: [institution_def]
  norms: [norm_def]
  occupations: [occupation_def]
  items: [item_def]
  affordances: [action_def]
  traces: [trace_def]
  speech_tags: [speech_style_def]
  place_types: [place_type_def]
  scenario_seeds: [seed_def]
```

## Schema anti-patterns

- `PlayerCharacter` in authoritative world schema;
- `Quest` as primary process;
- `ObjectiveMarker.true_location`;
- `NPC.knows_truth`;
- `LLM_mutated_state`;
- `Reward.spawn_on_completion`;
- `MagicTruth` without channel/provenance;
- `MonsterQuestTarget`;
- `FantasyKernelRole`;
- `CombatFirstActorState`.
