# Starter Data Schemas

This document is deliberately provisional. Treat these as design sketches that establish vocabulary, not final engine formats.

## Entity

```yaml
Entity:
  id: entity_id
  kind: actor | item | location | institution | record | notice | trace | group
  display_name: string
  components:
    Position: optional
    Ownership: optional
    ActorBody: optional
    MindRef: optional
    Container: optional
    Institution: optional
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
  location: optional_location_id
  causes:
    - event_id | intention_id | process_id | exogenous_cause
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
  traces_created:
    - trace_id
  observations_created:
    - observation_id
  records_created:
    - record_id
  random_draws_debug:
    - stream: string
      value: any
```

## Action definition

```yaml
ActionDef:
  id: action_take_item
  display: Take item
  actor_requirements:
    - has_body
    - can_reach_target
  parameters:
    item: EntityRef<Item>
    source: EntityRef<Container|Location|Actor>
  preconditions:
    physical:
      - item_present_at_source
      - actor_can_carry
    social:
      - actor_has_permission OR actor_willing_to_violate_property_norm
    knowledge:
      - actor_perceives_item OR actor_searching_source
  duration:
    base: 5s
    modifiers: [darkness, stealth, injury]
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
```

## Belief

```yaml
Belief:
  id: belief_id
  holder: actor_id
  proposition:
    type: string
    args: map
  truth_value: believed_true | believed_false | uncertain
  confidence: 0.0-1.0
  source:
    kind: direct_observation | hearsay | rumor | record | inference | instruction | memory | divination
    source_id: event_id | actor_id | record_id | trace_id | null
  believed_event_time: optional_time
  acquired_at: sim_time
  last_rehearsed_at: optional_time
  emotional_weight: 0.0-1.0
  decay_profile: none | slow | normal | fast
  contradictions:
    - belief_id
```

## Observation

```yaml
Observation:
  id: observation_id
  observer: actor_id
  observed_target: event_id | trace_id | entity_id | absence_marker
  channel: sight | sound | smell | touch | reading | speech | inference
  location: location_id
  time: sim_time
  raw_payload: map
  confidence: 0.0-1.0
  interpreted_beliefs:
    - belief_id
```

## Trace

```yaml
Trace:
  id: trace_id
  kind: footprint | blood | disturbed_container | missing_item | broken_lock | rumor | record | absence
  created_by: event_id
  location: optional_location_id
  attached_to: optional_entity_id
  visibility: 0.0-1.0
  decay_rate: none | slow | normal | fast
  possible_interpretations:
    - proposition: map
      support: 0.0-1.0
  can_be_modified_by:
    - action_id
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
    kind: need | belief_contradiction | role_obligation | opportunity | order | emotion | routine
    source_id: optional
  commitment: weak | normal | strong | obsessive
  started_at: sim_time
  current_method: optional_htn_method
  plan_stack:
    - task_or_action_id
  suspend_conditions:
    - condition
  abandon_conditions:
    - condition
```

## Institution

```yaml
Institution:
  id: institution_id
  kind: local_authority | household | guild | temple | market | bandit_group
  members:
    - actor_id
  roles:
    actor_id: [role_id]
  jurisdiction:
    locations: [location_id]
    people_scope: string
  resources:
    money: integer
    staff_time: integer
    goods: map
  records:
    incident_ledger: [record_id]
    bounty_ledger: [record_id]
    warrants: [record_id]
  norms:
    - norm_id
  procedures:
    - procedure_id
```

## Norm

```yaml
Norm:
  id: norm_no_theft
  kind: prohibition | obligation | permission | constitutive | status
  applies_to: scope_expression
  action_pattern: string
  conditions:
    - expression
  violation_event_kind: optional
  detection_requirements:
    - observed_by_authority OR reported_by_credible_witness OR evidence_threshold_met
  sanctions:
    - sanction_id
```

## Procedure

```yaml
Procedure:
  id: procedure_post_bounty
  owner_institution: institution_id
  trigger:
    kind: report_received
    filters:
      incident_kind: road_threat
  preconditions:
    - within_jurisdiction
    - funds_available
    - threat_confidence_above_threshold
  steps:
    - create_or_update_case_record
    - reserve_funds
    - assign_clerk_write_notice
    - assign_runner_post_notice
  failure_modes:
    - insufficient_funds
    - no_available_clerk
    - report_not_credible
```

## Notice

```yaml
Notice:
  id: notice_id
  physical_entity: entity_id
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
  posted_at: location_id
  posted_time: sim_time
  last_verified: sim_time
  status: drafted | carried | posted | removed | destroyed | archived
```

## Lead card

```yaml
LeadCard:
  id: lead_id
  visible_to_controller: true
  known_to_actor: true | false
  source: observation_id | belief_id | notice_id | record_id | rumor_id
  title: string
  claims:
    - proposition
  uncertainty_notes:
    - string
  related_entities:
    - entity_id
  suggested_actions:
    - action_id # actions the current actor believes are plausible
```

## Schema principle

Every schema should support the question:

> What caused this, who knows it, who can act on it, and how could they be wrong?
