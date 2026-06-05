# Agent Cognition

## Goal

Agents should appear alive not because they say clever lines, but because they persistently pursue goals from partial, fallible, socially situated beliefs.

The target mind model is:

```text
Needs + motives + traits + relationships + roles
      ↓
Beliefs and memories
      ↓
Desires / candidate goals
      ↓
Intentions / commitments
      ↓
HTN decomposition
      ↓
Local planner over believed world state
      ↓
Primitive actions
      ↓
Observation, surprise, interruption, replanning
```

## Agent state

Example high-level schema:

```yaml
AgentMind:
  actor: actor_tomas
  traits:
    risk_tolerance: low
    conscientiousness: high
    suspicion: medium
    greed: low
  needs:
    hunger: 22
    fatigue: 66
    safety: 80
    social_contact: 35
  motives:
    protect_family: 0.92
    maintain_status: 0.48
    recover_property: 0.0
    obey_law: 0.71
  relationships:
    actor_elena:
      kinship: spouse
      trust: 0.88
      affection: 0.81
    actor_mara:
      kinship: none
      trust: 0.31
      suspicion: 0.42
  roles:
    - household_head
    - miller
  beliefs_ref: belief_store_tomas
  memories_ref: memory_store_tomas
  current_intention: intention_go_to_work_991
  intention_stack: []
```

## Beliefs

A belief is a subjective proposition with confidence and provenance.

```yaml
Belief:
  id: belief_552
  holder: actor_tomas
  proposition:
    type: ItemLocatedAt
    item: item_coin_stack_01
    location: item_strongbox_01
  confidence: 0.86
  source:
    kind: direct_observation
    event: evt_checked_strongbox_yesterday
  believed_event_time: day_142_20_10
  acquired_at: day_142_20_10
  emotional_weight: 0.4
  decay: slow
  contradictions: []
```

Beliefs should support:

- direct observation;
- hearsay;
- rumor;
- institutional record;
- inference;
- instruction;
- memory degradation;
- contradiction;
- lies;
- stale confidence.

## Expectations and surprise

Discovery requires expectation.

When an agent observes something, compare perception to relevant beliefs:

```text
expected: coins in strongbox
observed: strongbox empty
result: contradiction
emotional response: alarm + anger
new beliefs:
  - coins missing
  - possible theft
  - last known present yesterday evening
candidate goals:
  - search room
  - question household
  - report theft
  - conceal loss
```

Surprise is not just flavor. It should trigger goal generation.

## Needs, motives, and goals

Separate urgent bodily needs from durable motives.

### Needs

- hunger;
- thirst;
- fatigue;
- health;
- safety;
- warmth;
- shelter;
- social contact.

### Motives

- protect child;
- preserve reputation;
- accumulate wealth;
- obey law;
- avoid shame;
- seek revenge;
- help friend;
- fulfill office;
- maintain routine;
- satisfy curiosity.

### Candidate goals

Candidate goals are generated from needs, beliefs, roles, opportunities, and interruptions.

Examples:

```yaml
CandidateGoal:
  type: RecoverMissingProperty
  source: belief_coins_missing
  urgency: 0.74
  emotional_force: anger
  social_constraints: [avoid_accusing_spouse_without_evidence]
```

## Intention persistence

A pure utility system can make agents fickle. Use intention persistence.

Once an agent commits to an intention, keep pursuing it until:

- completed;
- impossible;
- too costly;
- interrupted by emergency;
- contradicted by new belief;
- socially overridden;
- institutionally ordered away;
- fatigue/fear exceeds threshold.

Example:

```yaml
Intention:
  id: intention_774
  goal: RecoverMissingProperty(item_coin_stack_01)
  commitment: strong
  started_at: day_143_07_20
  current_phase: InspectImmediateArea
  abandon_conditions:
    - item_found
    - confidence_theft_false > 0.8
    - authority_accepts_case and actor_trusts_authority
    - physical_danger_high
```

## HTN decomposition

High-level activities should be decomposed using HTN-style methods.

Example: recover missing property.

```text
RecoverMissingProperty(item)
  Method A: SearchPrivately
    - go to last known location
    - inspect container
    - inspect nearby surfaces
    - ask household members
    - update suspicion
  Method B: ReportToAuthority
    - travel to office
    - wait for official
    - make report
    - provide last-known facts
    - receive case record
  Method C: AccuseSuspect
    - find suspect
    - confront
    - demand return or explanation
    - escalate if refused
```

Method selection depends on personality, social status, trust in law, evidence confidence, urgency, and risk.

## Local planning

Use GOAP-like planning for concrete action sequences under believed conditions.

Example:

```text
Goal: inspect strongbox
Believed preconditions:
  - strongbox in bedroom
  - key in Tomas's pouch
Plan:
  - go to bedroom
  - unlock strongbox
  - open strongbox
  - inspect contents
```

If the key is missing, the agent should form new beliefs and replan.

## Perception

Agents perceive through channels with range, reliability, and interpretation.

Channels:

- sight;
- sound;
- smell;
- touch;
- conversation;
- reading;
- institutional notification;
- inferred absence;
- magical/special sense if setting supports it.

Perception should often create observations, not immediate truth.

```yaml
Observation:
  observer: actor_elena
  event_or_trace: trace_faint_noise_12
  proposition:
    type: HeardNoiseAt
    location: room_tomas_bedroom
  confidence: 0.34
  interpretation_candidates:
    - thief
    - house settling
    - spouse awake
    - animal
```

## Memory

Use two layers:

### Episodic memory

Specific events the agent experienced.

```text
“I saw Mara near Tomas’s door after midnight.”
```

### Semantic/social memory

Generalized beliefs.

```text
“Mara is often near places she should not be.”
```

Memory operations:

- encode;
- retrieve by relevance;
- decay;
- strengthen by rehearsal;
- distort;
- merge;
- contradict;
- externalize into speech or records.

## Emotion

Emotion should modulate planning, not replace it.

Useful emotions:

- fear: increases avoidance and risk sensitivity;
- anger: increases confrontation and accusation;
- shame: increases concealment;
- grief: interrupts routine;
- affection: increases aid and belief charity;
- suspicion: increases inspection and surveillance;
- pride: increases reputation defense.

Emotion should have causes. “Anger because theft discovered” is better than random mood swings.

## Relationships

Relationships should alter affordances and belief revision.

Examples:

- A trusted spouse’s testimony is weighted more strongly.
- A hated rival’s accusation is discounted.
- A guard may ignore a poor vagrant’s report.
- A mother may lie for her child.
- A debtor may avoid the creditor’s street.

Relationship dimensions:

- kinship;
- affection;
- trust;
- fear;
- debt;
- obligation;
- rivalry;
- status difference;
- dependency;
- sexual/romantic interest;
- institutional authority.

## Roles

Roles add obligations, permissions, and goals.

Examples:

```yaml
Role: town_guard
obligations:
  - respond_to_report_if_on_duty
  - patrol_assigned_route
  - detain_suspects_when_warranted
permissions:
  - enter_property_with_warrant
  - carry_weapon_in_town
prohibitions:
  - accept_bribe
  - abandon_post_without_cause
```

A person can violate a role obligation. That creates institutional consequences if detected.

## Interruptions

Agents need interrupt logic.

Interruption sources:

- direct danger;
- pain/injury;
- fire;
- loud sound;
- social demand from high-status actor;
- contradictory observation;
- report from trusted source;
- urgent need;
- institutional order;
- opportunity too valuable to ignore.

Interruption policy:

```text
Compare current intention commitment against interruption urgency.
If interruption wins, suspend or abandon current intention.
Record the interruption event and reason.
```

## Decision loop pseudocode

```pseudo
for each agent at decision point:
    perceive_environment()
    update_beliefs_from_observations()
    detect_surprises_and_contradictions()
    update_needs_and_emotions()
    generate_candidate_goals()
    update_current_intention_or_select_new_one()
    decompose_intention_with_htn()
    plan_next_steps_over_believed_state()
    reserve_required_resources()
    execute_next_action_or_wait()
```

Do not run this at full fidelity for every actor every tick. Use event-driven wakeups, routines, and LOD.

## Anti-patterns

- Agents query global truth.
- Agents switch goals every few seconds.
- Schedules ignore theft, injury, fear, missing tools, or blocked paths.
- Dialogue invents facts not present in belief state.
- NPCs forgive player crimes because the player changed bodies.
- Guards know crimes without observation/report/evidence.
- “Suspicion” is a single global wanted meter.

## Minimal cognition for the vertical slice

Implement only:

- belief store with provenance and confidence;
- direct observation and hearsay;
- expectation contradiction;
- three needs: hunger, fatigue, safety;
- four motives: property, duty, affection, fear;
- intention persistence;
- HTN methods for workday, theft discovery, report theft, post notice, investigate threat;
- local planning for movement, containers, speech, reading, writing, posting;
- simple relationship trust;
- interruption on danger, contradiction, and authority order.

That is enough to create the first miracle.
